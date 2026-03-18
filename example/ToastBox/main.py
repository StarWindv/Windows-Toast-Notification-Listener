import asyncio
import json
import sqlite3
from collections import deque
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime, timedelta, timezone
from pathlib import Path
from typing import Dict, Optional, List

from textual import on
from textual.app import App, ComposeResult
from textual.binding import Binding
from textual.css.query import NoMatches
from textual.messages import Message
from textual.widgets import Header, Footer, DataTable, Static

import win_notice_lite as wnl


class NotificationAdded(Message):
    def __init__(self, toast: wnl.Toast) -> None:
        self.toast = toast
        super().__init__()


class NotificationRemoved(Message):
    def __init__(self, toast: wnl.Toast) -> None:
        self.toast = toast
        super().__init__()


# noinspection PyBroadException
def time_converter(ft_str: str) -> str:
    try:
        ft = int(ft_str)
        unix_seconds = (ft / 10_000_000) - 11644473600
        dt_utc8 = datetime.fromtimestamp(unix_seconds, tz=timezone.utc) + timedelta(hours=8)
        return dt_utc8.strftime("%Y-%m-%d %H:%M:%S")
    except Exception:
        return ft_str


# noinspection SqlNoDataSourceInspection
class ToastDatabase:
    DB_PATH = Path.home() / "ToastBox" / "toast.sqlite"

    @classmethod
    def init_db(cls) -> None:
        cls.DB_PATH.parent.mkdir(parents=True, exist_ok=True)
        with sqlite3.connect(cls.DB_PATH) as conn:
            conn.execute("""
                         CREATE TABLE IF NOT EXISTS toasts (
                                                               id INTEGER,
                                                               name TEXT,
                                                               logo_uri TEXT,
                                                               title TEXT,
                                                               message TEXT,
                                                               hero_image_uri TEXT,
                                                               inline_images TEXT,
                                                               tag TEXT,
                                                               "group" TEXT,
                                                               creation_time TEXT,
                                                               fingerprint TEXT,
                                                               fingerprint_without_time TEXT,
                                                               received_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                         )
                         """)
            conn.execute("""
                         CREATE INDEX IF NOT EXISTS idx_creation_time_id
                             ON toasts(creation_time ASC, id ASC)
                         """)

    @classmethod
    def save_toast(cls, toast: wnl.Toast) -> None:
        with sqlite3.connect(cls.DB_PATH) as conn:
            conn.execute("""
                         INSERT INTO toasts (
                             id, name, logo_uri, title, message,
                             hero_image_uri, inline_images, tag, "group",
                             creation_time, fingerprint, fingerprint_without_time
                         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                         """, (
                             toast.id, toast.name, toast.logo_uri, toast.title, toast.message,
                             toast.hero_image_uri, json.dumps(toast.inline_images, ensure_ascii=False),
                             toast.tag, toast.group, toast.creation_time,
                             toast.fingerprint, toast.fingerprint_without_time
                         ))

    @classmethod
    def fetch_latest(cls, limit: int) -> List[Dict]:
        with sqlite3.connect(cls.DB_PATH) as conn:
            conn.row_factory = sqlite3.Row
            cursor = conn.execute("""
                                  SELECT * FROM toasts
                                  ORDER BY creation_time ASC, id ASC
                                      LIMIT ?
                                  """, (limit,))
            return [dict(row) for row in cursor.fetchall()]

    @classmethod
    def fetch_older_than(cls, cursor_time: str, cursor_id: int, limit: int) -> List[Dict]:
        with sqlite3.connect(cls.DB_PATH) as conn:
            conn.row_factory = sqlite3.Row
            cursor = conn.execute("""
                                  SELECT * FROM toasts
                                  WHERE (creation_time < ?) OR (creation_time = ? AND id < ?)
                                  ORDER BY creation_time DESC, id DESC
                                      LIMIT ?
                                  """, (cursor_time, cursor_time, cursor_id, limit))
            rows = list(reversed([dict(row) for row in cursor.fetchall()]))
            return rows

    @classmethod
    def fetch_newer_than(cls, cursor_time: str, cursor_id: int, limit: int) -> List[Dict]:
        with sqlite3.connect(cls.DB_PATH) as conn:
            conn.row_factory = sqlite3.Row
            cursor = conn.execute("""
                                  SELECT * FROM toasts
                                  WHERE (creation_time > ?) OR (creation_time = ? AND id > ?)
                                  ORDER BY creation_time ASC, id ASC
                                      LIMIT ?
                                  """, (cursor_time, cursor_time, cursor_id, limit))
            return [dict(row) for row in cursor.fetchall()]


# noinspection PyUnresolvedReferences
class ToastBox(App):
    CSS = """
    $base03: #002b36;
    $base02: #073642;
    $base01: #586e75;
    $base00: #657b83;
    $base0: #839496;
    $base1: #93a1a1;
    $base2: #eee8d5;
    $base3: #fdf6e3;
    $yellow: #b58900;
    $orange: #cb4b16;
    $red: #dc322f;
    $magenta: #d33682;
    $violet: #6c71c4;
    $blue: #268bd2;
    $cyan: #2aa198;
    $green: #859900;

    Screen {
        background: $base03;
    }

    DataTable {
        background: $base02;
        color: $base0;
        height: 1fr;
        border: solid $base01;
        & > .datatable--header {
            background: $base01;
            color: $base2;
            text-style: bold;
        }
        & > .datatable--cursor {
            background: $blue;
            color: $base3;
        }
    }

    #detail {
        height: 3;
        background: $base02;
        border-top: solid $base01;
        color: $base0;
        padding: 0 1;
        overflow-y: auto;
    }

    Footer {
        background: $base01;
        color: $base2;
    }

    Header {
        background: $base02;
        color: $base1;
    }
    """

    BINDINGS = [
        Binding("space", "toggle_polling", "启动/停止"),
        Binding("+", "increase_interval", "增加间隔"),
        Binding("-", "decrease_interval", "减少间隔"),
        Binding("c", "clear_list", "清空列表"),
        Binding("q", "quit", "退出"),
        Binding("g", "scroll_top", "顶部"),
        Binding("G", "scroll_bottom", "底部"),
    ]
    MAX_TOASTS = 60
    PAGE_SIZE = 20

    def __init__(self) -> None:
        super().__init__()
        self.listener = wnl.Listener()
        self.polling: Optional[wnl.features.Polling] = None
        self.callback_token: Optional[wnl.features.CallbackToken] = None
        self.running = False
        self.interval = 1000

        self._window: deque[wnl.Toast] = deque(maxlen=self.MAX_TOASTS)
        self._toasts_by_id: Dict[int, wnl.Toast] = {}
        self._active_status: Dict[int, bool] = {}

        self._db_executor = ThreadPoolExecutor(max_workers=1)
        self._db_queue: asyncio.Queue[wnl.Toast] = asyncio.Queue()
        self._db_task: Optional[asyncio.Task] = None

        self._loading = False

        self._last_cursor_row = -1

        self._initialized = False

    def compose(self) -> ComposeResult:
        yield Header()

        table = DataTable()

        table.add_column("状态", key="status")
        table.add_column("ID", key="id")
        table.add_column("应用", key="app")
        table.add_column("标题", key="title")
        table.add_column("时间", key="time")
        table.add_column("消息", key="message")
        yield table

        yield Static(id="detail", markup=False)
        yield Footer()

    async def on_mount(self) -> None:

        await asyncio.to_thread(ToastDatabase.init_db)

        self._db_task = asyncio.create_task(self._db_writer())

        status = await self.listener.request_permission()
        if status != "Allowed":
            self.sub_title = f"[red]权限未授予: {status}[/]"
            return

        self.sub_title = "[green]权限已授予，加载历史通知...[/]"

        await self._load_initial_from_db()

        current = await self.listener.get_all_notifications()
        current_ids = {t.id for t in current}
        for toast in self._window:
            self._active_status[toast.id] = toast.id in current_ids

        self._initialized = True

        self._refresh_table()

        self.polling = wnl.features.Polling(self.listener, self.interval)
        self.callback_token = self.polling.register_polling_event_callback(
            self._on_polling_event
        )
        self.polling.start_all()
        self.running = True
        self.sub_title = f"[green]监听中 (间隔 {self.interval}ms)[/]"

    async def on_unmount(self) -> None:
        if self.polling and self.callback_token:
            self.polling.stop_all()
            self.polling.unregister(self.callback_token)
        if self._db_task:
            self._db_task.cancel()
        self._db_executor.shutdown(wait=False)

    def _get_table(self) -> Optional[DataTable]:
        try:
            return self.query_one(DataTable)
        except NoMatches:
            return None

    async def _db_writer(self) -> None:
        while True:
            try:
                toast = await self._db_queue.get()
                await asyncio.to_thread(ToastDatabase.save_toast, toast)
            except asyncio.CancelledError:
                break
            except Exception as e:

                print(f"DB write error: {e}")

    async def _load_initial_from_db(self) -> None:
        rows = await asyncio.to_thread(ToastDatabase.fetch_latest, self.MAX_TOASTS)
        toasts = []
        for r in rows:

            toast = wnl.Toast(
                id=r["id"],
                name=r["name"],
                logo_uri=r["logo_uri"],
                title=r["title"],
                message=r["message"],
                hero_image_uri=r["hero_image_uri"],
                inline_images=json.loads(r["inline_images"]),
                tag=r["tag"],
                group=r["group"],
                creation_time=r["creation_time"],
                fingerprint=r["fingerprint"],
                fingerprint_without_time=r["fingerprint_without_time"]
            )
            toasts.append(toast)
        self._window.clear()
        self._window.extend(toasts)
        self._rebuild_index()

    def _rebuild_index(self) -> None:
        self._toasts_by_id.clear()
        for t in self._window:
            self._toasts_by_id[t.id] = t

    async def _load_older(self) -> bool:
        if self._loading or not self._window:
            return False
        first = self._window[0]
        self._loading = True
        try:
            rows = await asyncio.to_thread(
                ToastDatabase.fetch_older_than,
                first.creation_time, first.id,
                self.PAGE_SIZE
            )
            if not rows:
                return False

            older_toasts = []
            for r in rows:
                t = wnl.Toast(
                    id=r["id"],
                    name=r["name"],
                    logo_uri=r["logo_uri"],
                    title=r["title"],
                    message=r["message"],
                    hero_image_uri=r["hero_image_uri"],
                    inline_images=json.loads(r["inline_images"]),
                    tag=r["tag"],
                    group=r["group"],
                    creation_time=r["creation_time"],
                    fingerprint=r["fingerprint"],
                    fingerprint_without_time=r["fingerprint_without_time"]
                )
                older_toasts.append(t)

            self._window.extendleft(reversed(older_toasts))
            self._rebuild_index()
            self._refresh_table()

            table = self._get_table()
            if table and table.row_count > 0 and len(older_toasts) < table.row_count:
                table.move_cursor(row=len(older_toasts))
            return True
        finally:
            self._loading = False

    async def _load_newer(self) -> bool:
        if self._loading or not self._window:
            return False
        last = self._window[-1]
        self._loading = True
        try:
            rows = await asyncio.to_thread(
                ToastDatabase.fetch_newer_than,
                last.creation_time, last.id,
                self.PAGE_SIZE
            )
            if not rows:
                return False
            newer_toasts = []
            for r in rows:
                t = wnl.Toast(
                    id=r["id"],
                    name=r["name"],
                    logo_uri=r["logo_uri"],
                    title=r["title"],
                    message=r["message"],
                    hero_image_uri=r["hero_image_uri"],
                    inline_images=json.loads(r["inline_images"]),
                    tag=r["tag"],
                    group=r["group"],
                    creation_time=r["creation_time"],
                    fingerprint=r["fingerprint"],
                    fingerprint_without_time=r["fingerprint_without_time"]
                )
                newer_toasts.append(t)

            self._window.extend(newer_toasts)
            self._rebuild_index()
            self._refresh_table()
            return True
        finally:
            self._loading = False

    def _refresh_table(self) -> None:

        if not self._initialized:
            return

        table = self._get_table()
        if table is None:
            return

        selected_row_key = table.cursor_row
        selected_id = None
        if selected_row_key is not None and 0 <= selected_row_key < len(self._window):
            selected_id = self._window[selected_row_key].id

        table.clear()

        for toast in self._window:
            time_str = time_converter(toast.creation_time)
            msg_preview = (toast.message[:50] + "…") if len(toast.message) > 50 else toast.message
            status = "活跃" if self._active_status.get(toast.id, False) else "过期"
            try:
                table.add_row(
                    status,
                    str(toast.id),
                    toast.name,
                    toast.title,
                    time_str,
                    msg_preview,
                    key=str(toast.id),
                )
            except Exception as e:

                print(f"Error adding row for toast {toast.id}: {e}")

        if selected_id is not None:

            for idx, t in enumerate(self._window):
                if t.id == selected_id:
                    table.move_cursor(row=idx)
                    break
        else:

            if table.row_count > 0:
                table.move_cursor(row=table.row_count - 1)

        self._update_count()

    def _update_count(self) -> None:
        total = len(self._window)
        active = sum(1 for v in self._active_status.values() if v)

        try:
            detail = self.query_one("#detail")
            if detail:
                detail.update(f"通知总数(内存): {total} (活跃: {active})")
        except NoMatches:
            pass

    def _on_polling_event(self, diff: wnl.Diff) -> None:
        for t in diff.new:
            self.post_message(NotificationAdded(t))
        for t in diff.remove:
            self.post_message(NotificationRemoved(t))

    async def on_notification_added(self, msg: NotificationAdded) -> None:
        toast = msg.toast

        await self._db_queue.put(toast)

        if toast.id not in self._toasts_by_id:

            self._window.append(toast)
            self._toasts_by_id[toast.id] = toast
        self._active_status[toast.id] = True

        self._rebuild_index()

        self._refresh_table()

    async def on_notification_removed(self, msg: NotificationRemoved) -> None:
        toast_id = msg.toast.id
        if toast_id in self._active_status:
            self._active_status[toast_id] = False

            table = self._get_table()
            if table:
                row_key = str(toast_id)
                if row_key in table.rows:
                    try:
                        table.update_cell(row_key, "status", "过期")
                    except Exception as e:
                        print(f"Error updating cell for toast {toast_id}: {e}")
            self._update_count()

    async def on_data_table_row_selected(self, event: DataTable.RowSelected) -> None:
        toast_id = int(event.row_key.value)
        toast = self._toasts_by_id.get(toast_id)
        if toast:
            time_str = time_converter(toast.creation_time)
            detail = (
                f"[bold]应用:[/] {toast.name}  "
                f"[bold]标题:[/] {toast.title}  "
                f"[bold]时间:[/] {time_str}\n"
                f"[bold]消息:[/] {toast.message}"
            )
        else:
            detail = "[dim]未找到通知详情[/]"

        try:
            detail_widget = self.query_one("#detail")
            if detail_widget:
                detail_widget.update(detail)
        except NoMatches:
            pass

    @on(DataTable.RowHighlighted)
    async def on_data_table_row_highlighted(self, event: DataTable.RowHighlighted) -> None:
        table = self._get_table()
        if table is None or table.row_count == 0:
            return

        cursor_row = event.cursor_row
        if cursor_row is None:
            return

        if cursor_row == self._last_cursor_row:
            return
        self._last_cursor_row = cursor_row

        if cursor_row == 0 and not self._loading:
            await self._load_older()

        elif cursor_row == table.row_count - 1 and not self._loading:
            await self._load_newer()

    def action_toggle_polling(self) -> None:
        if not self.polling:
            return
        if self.running:
            self.polling.stop_all()
            self.running = False
            self.sub_title = "[yellow]已暂停[/]"
        else:
            self.polling.start_all()
            self.running = True
            self.sub_title = f"[green]监听中 (间隔 {self.interval}ms)[/]"

    def action_increase_interval(self) -> None:
        if self.polling:
            self.interval = min(10000, self.interval + 500)
            self.polling.change_interval(self.interval)
            if self.running:
                self.sub_title = f"[green]监听中 (间隔 {self.interval}ms)[/]"

    def action_decrease_interval(self) -> None:
        if self.polling:
            self.interval = max(500, self.interval - 500)
            self.polling.change_interval(self.interval)
            if self.running:
                self.sub_title = f"[green]监听中 (间隔 {self.interval}ms)[/]"

    def action_clear_list(self) -> None:
        self._window.clear()
        self._toasts_by_id.clear()
        self._active_status.clear()
        self._refresh_table()

    def action_scroll_top(self) -> None:
        table = self._get_table()
        if table and table.row_count > 0:
            table.move_cursor(row=0)

            asyncio.create_task(self._load_older())

    def action_scroll_bottom(self) -> None:
        table = self._get_table()
        if table and table.row_count > 0:
            table.move_cursor(row=table.row_count - 1)

            asyncio.create_task(self._load_newer())


if __name__ == "__main__":
    app = ToastBox()
    app.run()
