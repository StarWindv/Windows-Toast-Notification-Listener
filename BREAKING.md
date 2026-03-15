# 破坏性更新与更新说明

此文件用于说明各版本中 Python 侧 API 的变动

---

## 目录

- [破坏性更新与更新说明](#破坏性更新与更新说明)
  - [目录](#目录)
  - [详细信息](#详细信息)
    - [v0.0.1](#v001)
    - [v0.0.1a0](#v001a0)
    - [v0.0.2](#v002)
    - [v0.0.3](#v003)

---

## 详细信息

### v0.0.1

此版本是项目的初始版本, 不存在破坏性更新

---

### v0.0.1a0

此版本实际上是初始版本的补丁, 处理了 readme 中的部分问题, 只是版本号写错了

---

### v0.0.2

此版本仅做了 API 的位置迁移

| Old API                      | New API                    |
|------------------------------|----------------------------|
| `Listener.diff_full`         | `Differ.diff_full`         |
| `Listener.diff_without_time` | `Differ.diff_without_time` |
| `Listener.diff_by_id`        | `Differ.diff_by_id`        |
| `Listener.serialize`         | `Differ.serialize`         |

---

### v0.0.3

已有接口变化:

|          | Old API                                            | New API                                        |
|----------|----------------------------------------------------|------------------------------------------------|
| API Name | `Listener.elevate_privilege`                       | `Listener.request_permission`                  |
| Args     | `No Args`                                          | `No Args`                                      |
| Returns  | `UserNotificationListenerAccessStatus(0/1/2)`, str | `Unspecified/Allowed/Denied/UnknownError`, str |

名称迁移:

| Old API                     | New API                                                 | Explanation |
|-----------------------------|---------------------------------------------------------|-------------|
| `Differ.diff_full`          | `DiffTool.diff_full`                                    | 名称迁移        |
| `Differ.diff_without_time`  | `DiffTool.diff_without_time`                            | 名称迁移        |
| `Differ.diff_by_id`         | `DiffTool.diff_by_id`                                   | 名称迁移        |
| `Differ.serialize([Toast])` | `DiffTool.serialize_to([Toast], Type: SerializeFormat)` | 参数改变, 名称迁移  |
|                             | `DiffTool.to_json_str([Toast])`                         | 新 API       |

新的类:

|                 | 类型  | 成员                    | 接口 | 是否可实例化 |
|-----------------|-----|-----------------------|----|--------|
| SerializeFormat | 枚举类 | Json, Yaml, Toml, XML | 无  | 否      |
