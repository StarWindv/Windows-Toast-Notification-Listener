# Python 侧 API 文档

本文档用于介绍编译后二进制库的使用

---

## 目录

- [Python 侧 API 文档](#python-侧-api-文档)
  - [目录](#目录)
  - [一、介绍](#一介绍)
    - [1.1 导入](#11-导入)
    - [1.2 类](#12-类)
  - [二、Listener](#二listener)
    - [2.1 介绍](#21-介绍)
    - [2.2 接口](#22-接口)
      - [2.2.1 request\_permission](#221-request_permission)
        - [2.2.1.1 参数](#2211-参数)
        - [2.2.1.2 类型](#2212-类型)
        - [2.2.1.3 返回值](#2213-返回值)
        - [2.2.1.4 示例](#2214-示例)
      - [2.2.2 get\_all\_notifications](#222-get_all_notifications)
        - [2.2.2.1 参数](#2221-参数)
        - [2.2.2.2 类型](#2222-类型)
        - [2.2.2.3 返回值](#2223-返回值)
        - [2.2.2.4 示例](#2224-示例)
      - [2.2.3 register\_toast\_handler](#223-register_toast_handler)
        - [2.2.3.1 说明](#2231-说明)
      - [2.2.4 unregister\_toast\_handler](#224-unregister_toast_handler)
        - [2.2.4.1 说明](#2241-说明)
  - [三、Toast](#三toast)
    - [3.1 介绍](#31-介绍)
    - [3.2 接口](#32-接口)
      - [3.2.1 `__init__`](#321-__init__)
        - [3.2.1.1 参数说明](#3211-参数说明)
        - [3.2.1.2 示例](#3212-示例)
    - [3.3 属性](#33-属性)
    - [3.4 指纹](#34-指纹)
    - [3.5 空字段说明](#35-空字段说明)
  - [四、MutableToast](#四mutabletoast)
    - [4.1 介绍](#41-介绍)
  - [五、Diff](#五diff)
    - [5.1 介绍](#51-介绍)
    - [5.2 属性](#52-属性)
    - [5.3 示例](#53-示例)
  - [六、DiffTool](#六difftool)
    - [6.1 介绍](#61-介绍)
    - [6.2 接口](#62-接口)
      - [6.2.1 diff\_full](#621-diff_full)
        - [6.2.1.1 说明](#6211-说明)
        - [6.2.1.2 参数](#6212-参数)
        - [6.2.1.3 返回值](#6213-返回值)
        - [6.2.1.4 示例](#6214-示例)
      - [6.2.2 diff\_by\_id](#622-diff_by_id)
        - [6.2.2.1 说明](#6221-说明)
        - [6.2.2.2 参数](#6222-参数)
        - [6.2.2.3 返回值](#6223-返回值)
      - [6.2.3 diff\_without\_time](#623-diff_without_time)
        - [6.2.3.1 说明](#6231-说明)
        - [6.2.3.2 参数](#6232-参数)
        - [6.2.3.3 返回值](#6233-返回值)
      - [6.2.4 to\_json\_str](#624-to_json_str)
        - [6.2.4.1 说明](#6241-说明)
        - [6.2.4.2 参数](#6242-参数)
        - [6.2.4.3 返回值](#6243-返回值)
        - [6.2.4.4 示例](#6244-示例)
      - [6.2.5 serialize\_to](#625-serialize_to)
        - [6.2.5.1 说明](#6251-说明)
        - [6.2.5.2 参数](#6252-参数)
        - [6.2.5.3 返回值](#6253-返回值)
        - [6.2.5.4 示例](#6254-示例)
  - [七、SerializeFormat](#七serializeformat)
    - [7.1 介绍](#71-介绍)
    - [7.2 枚举值](#72-枚举值)

---

## 一、介绍

### 1.1 导入

你可以使用以下代码来导入我们的库

```python
import win_notice_lite as wnl
```

### 1.2 类

库提供如下几个类:
- [Listener](#二Listener)
- [Toast](#三Toast)
- [MutableToast](#四MutableToast)
- [Diff](#五Diff)
- [DiffTool](#六DiffTool)
- [SerializeFormat](#七Serializeformat)

---

## 二、Listener

### 2.1 介绍

`Listener` 是本库的核心类, 用于创建通知监听器实例、申请权限以及获取当前系统所有 Toast 通知.

### 2.2 接口

此类提供如下接口(可点击):

- [`request_permission`](#221-request_permission)
- [`get_all_notifications`](#222-get_all_notifications)
- ~~[`register_toast_handler`](#223-register_toast_handler)~~
- ~~[`unregister_toast_handler`](#224-unregister_toast_handler)~~

---

#### 2.2.1 request_permission

##### 2.2.1.1 参数

| 参数名    | 类型 | 说明   |
|--------|----|------|
| `self` | -  | 实例对象 |

##### 2.2.1.2 类型

- **异步方法**: 需要使用 `await` 来获取结果.

##### 2.2.1.3 返回值

返回一个 `str` 类型的权限状态, 可能的值如下: 

| 返回值             | 描述                                      |
|-----------------|-----------------------------------------|
| `"Unspecified"` | 用户尚未允许或拒绝访问                             |
| `"Allowed"`     | 用户已授予对 `UserNotificationListener` 的访问权限 |
| `"Denied"`      | 用户拒绝访问 `UserNotificationListener`       |
| `"Unknown"`     | 未知错误, 通常不会出现, 仅为配合 Rust 模式匹配而保留         |

##### 2.2.1.4 示例

```python
import win_notice_lite as wnl
import asyncio

async def main():
    listener = wnl.Listener()
    permission = await listener.request_permission()
    print(permission)

asyncio.run(main())
```

---

#### 2.2.2 get_all_notifications

##### 2.2.2.1 参数

| 参数名    | 类型 | 说明   |
|--------|----|------|
| `self` | -  | 实例对象 |

##### 2.2.2.2 类型

- **异步方法**: 需要使用 `await` 来获取结果.

##### 2.2.2.3 返回值

返回一个 `list[Toast]` 类型的数组, 包含当前系统中所有 Toast 通知.  
若未获得权限, 则返回空列表 `[]`.

##### 2.2.2.4 示例

```python
import win_notice_lite as wnl
import asyncio

async def main():
    listener = wnl.Listener()
    status = await listener.request_permission()
    match status:
        case x if x != "Allowed": return
    toasts = await listener.get_all_notifications()
    for toast in toasts:
        print(toast)

asyncio.run(main())
```

---

#### 2.2.3 register_toast_handler

##### 2.2.3.1 说明

该接口旨在提供事件通知形式的回调注册, 但是并未正式加入, 详见[此处](../About_Event_Notification/Explanation.md)

---

#### 2.2.4 unregister_toast_handler

##### 2.2.4.1 说明

该接口旨在提供事件通知回调函数的注销方法, 但是并未正式加入, 详见[此处](../About_Event_Notification/Explanation.md)

---

## 三、Toast

### 3.1 介绍

`Toast` 是本库中用于表示单条 Windows 通知的核心数据结构.   
它包含了通知的所有元数据字段, 部分字段由于 Windows API 的限制无法获取实际内容 (详见字段说明) . 

该类由 Rust 实现并通过 PyO3 导出到 Python, 支持直接构造, 通常由 `get_all_notifications()` 返回. 

---

### 3.2 接口

#### 3.2.1 `__init__`

##### 3.2.1.1 参数说明

| 参数名                        | 类型          | 说明                                       |
|----------------------------|-------------|------------------------------------------|
| `id`                       | `int`       | 系统分配的通知标识符, 唯一 ID                        |
| `name`                     | `str`       | 通知来源程序名称                                 |
| `logo_uri`                 | `str`       | 通知来源程序图标 URI (由于 API 限制, 实际为空)           |
| `title`                    | `str`       | 通知标题                                     |
| `message`                  | `str`       | 通知内容                                     |
| `hero_image_uri`           | `str`       | 通知主图 URI (由于 API 限制, 实际为空)               |
| `inline_images`            | `list[str]` | 通知内联图片 URI 数组 (由于 API 限制, 实际为空)          |
| `tag`                      | `str`       | 通知标签 (由于 API 限制, 实际为空)                   |
| `group`                    | `str`       | 通知分组 (由于 API 限制, 实际为空)                   |
| `creation_time`            | `str`       | 通知创建时间 (Windows 时间戳, 起点为 1601/01/01 UTC) |
| `fingerprint`              | `str`       | 通知的完整指纹 (SHA2-256)                       |
| `fingerprint_without_time` | `str`       | 不考虑时间戳的完整指纹                              |

##### 3.2.1.2 示例

```python
import win_notice_lite as wnl
toast = wnl.Toast(
    id=123,
    name="微信",
    logo_uri="",
    title="新消息",
    message="你有一条新消息",
    hero_image_uri="",
    inline_images=[],
    tag="",
    group="",
    creation_time="2026-01-01 12:00:00",
    fingerprint="a1b2c3...",
    fingerprint_without_time="d4e5f6..."
)
```

---

### 3.3 属性

该类所有字段均为只读属性, 可通过点号访问:

| 属性名                        | 类型          | 说明       |
|----------------------------|-------------|----------|
| `id`                       | `int`       | 通知 ID    |
| `name`                     | `str`       | 应用名称     |
| `logo_uri`                 | `str`       | 应用图标 (空) |
| `title`                    | `str`       | 标题       |
| `message`                  | `str`       | 内容       |
| `hero_image_uri`           | `str`       | 主图 (空)   |
| `inline_images`            | `list[str]` | 内联图片 (空) |
| `tag`                      | `str`       | 标签 (空)   |
| `group`                    | `str`       | 分组 (空)   |
| `creation_time`            | `str`       | 创建时间     |
| `fingerprint`              | `str`       | 完整指纹     |
| `fingerprint_without_time` | `str`       | 不含时间戳的指纹 |

---

### 3.4 指纹

指纹生成方式如下: 

1. 将所有字段 (除 `fingerprint` 和 `fingerprint_without_time` 外) 按顺序用**空格**拼接; 
2. 对拼接后的字符串计算 SHA2-256 哈希值. 

`fingerprint_without_time` 则在拼接时**不包含 `creation_time` 字段**, 用于忽略时间影响进行去重. 

---

### 3.5 空字段说明

以下字段由于 Windows API 限制, 始终为空字符串或空列表:

- `logo_uri`
- `hero_image_uri`
- `inline_images`
- `tag`
- `group`

---

## 四、MutableToast

### 4.1 介绍

此类与[Toast](#三Toast)结构上完全相同, 但是每个属性都可以修改, 其余信息不再赘述.

---

## 五、Diff

### 5.1 介绍

`Diff` 是一个用于表示通知列表差异的数据结构, 通常由 `DiffTool` 的差异计算方法返回. 它包含两个字段: 新增的通知列表和移除的通知列表. 

---

### 5.2 属性

| 属性名      | 类型            | 说明      |
|----------|---------------|---------|
| `new`    | `list[Toast]` | 新增的通知列表 |
| `remove` | `list[Toast]` | 移除的通知列表 |

---

### 5.3 示例

```python
import win_notice_lite as wnl

diff = wnl.Diff(
    new=[toast1, toast2],
    remove=[toast3]
)

for toast in diff.new:
    print(toast.title)
```

---

## 六、DiffTool

### 6.1 介绍

`DiffTool` 是一个工具类, 提供多种方式计算两个通知列表之间的差异, 并支持将通知列表序列化为多种格式. 

---

### 6.2 接口

此类提供如下接口(可点击):
 - [`diff_full`](#621-diff_full)
 - [`diff_by_id`](#622-diff_by_id)
 - [`diff_without_time`](#623-diff_without_time)
 - [`to_json_str`](#624-to_json_str)
 - [`serialize_to`](#625-serialize_to)

---

#### 6.2.1 diff_full

##### 6.2.1.1 说明

基于完整指纹 (包含时间戳) 对比两个通知列表的差异. 

##### 6.2.1.2 参数

| 参数名   | 类型            | 说明    |
|-------|---------------|-------|
| `old` | `list[Toast]` | 旧通知列表 |
| `new` | `list[Toast]` | 新通知列表 |

##### 6.2.1.3 返回值

返回一个 [`Diff`](#五Diff) 对象, 包含新增和移除的通知. 

##### 6.2.1.4 示例

```python
diff = wnl.DiffTool.diff_full(old_toasts, new_toasts)
print(len(diff.new), len(diff.remove))
```

---

#### 6.2.2 diff_by_id

##### 6.2.2.1 说明

基于通知 ID 对比两个通知列表的差异. 

##### 6.2.2.2 参数

| 参数名   | 类型            | 说明    |
|-------|---------------|-------|
| `old` | `list[Toast]` | 旧通知列表 |
| `new` | `list[Toast]` | 新通知列表 |

##### 6.2.2.3 返回值

返回一个 [`Diff`](#五Diff) 对象. 

---

#### 6.2.3 diff_without_time

##### 6.2.3.1 说明

基于不含时间戳的指纹对比两个通知列表的差异, 适用于忽略时间影响的去重场景. 

##### 6.2.3.2 参数

| 参数名   | 类型            | 说明    |
|-------|---------------|-------|
| `old` | `list[Toast]` | 旧通知列表 |
| `new` | `list[Toast]` | 新通知列表 |

##### 6.2.3.3 返回值

返回一个 [`Diff`](#五Diff) 对象. 

---

#### 6.2.4 to_json_str

##### 6.2.4.1 说明

将通知列表序列化为格式化的 JSON 字符串. 若序列化失败, 返回 `"[]"`. 

##### 6.2.4.2 参数

| 参数名             | 类型            | 说明        |
|-----------------|---------------|-----------|
| `notifications` | `list[Toast]` | 待序列化的通知列表 |

##### 6.2.4.3 返回值

`str`: 格式化的 JSON 字符串. 

##### 6.2.4.4 示例

```python
json_str = wnl.DiffTool.to_json_str(toasts)
print(json_str)
```

---

#### 6.2.5 serialize_to

##### 6.2.5.1 说明

将通知列表序列化为指定格式的字符串. 

##### 6.2.5.2 参数

| 参数名             | 类型                                     | 说明        |
|-----------------|----------------------------------------|-----------|
| `notifications` | `list[Toast]`                          | 待序列化的通知列表 |
| `to`            | [`SerializeFormat`](#七Serializeformat) | 目标序列化格式   |

##### 6.2.5.3 返回值

`str`: 格式化的字符串, 失败时返回 `"[]"`. 

##### 6.2.5.4 示例

```python
yaml_str = wnl.DiffTool.serialize_to(toasts, wnl.SerializeFormat.Yaml)
print(yaml_str)
```

---

## 七、SerializeFormat

### 7.1 介绍

`SerializeFormat` 是一个枚举类型, 用于指定序列化格式. 该枚举在 Python 中为**不可变类型**, 不可修改. 

---

### 7.2 枚举值

| 枚举值    | 说明      |
|--------|---------|
| `Json` | JSON 格式 |
| `Yaml` | YAML 格式 |
| `Toml` | TOML 格式 |
| `XML`  | XML 格式  |

---

末次编辑日期: 2026年3月16日
