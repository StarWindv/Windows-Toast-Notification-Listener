# Python-side API Documentation

This document introduces how to use the compiled binary library.

---

## Table of Contents

- [Python-side API Documentation](#python-side-api-documentation)
    - [Table of Contents](#table-of-contents)
    - [I. Introduction](#i-introduction)
        - [1.1 Import](#11-import)
        - [1.2 Classes](#12-classes)
    - [II. Listener](#ii-listener)
        - [2.1 Introduction](#21-introduction)
        - [2.2 Interfaces](#22-interfaces)
            - [2.2.1 request\_permission](#221-request_permission)
                - [2.2.1.1 Parameters](#2211-parameters)
                - [2.2.1.2 Type](#2212-type)
                - [2.2.1.3 Return Value](#2213-return-value)
                - [2.2.1.4 Example](#2214-example)
            - [2.2.2 get\_all\_notifications](#222-get_all_notifications)
                - [2.2.2.1 Parameters](#2221-parameters)
                - [2.2.2.2 Type](#2222-type)
                - [2.2.2.3 Return Value](#2223-return-value)
                - [2.2.2.4 Example](#2224-example)
            - [2.2.3 register\_toast\_handler](#223-register_toast_handler)
                - [2.2.3.1 Description](#2231-description)
            - [2.2.4 unregister\_toast\_handler](#224-unregister_toast_handler)
                - [2.2.4.1 Description](#2241-description)
    - [III. Toast](#iii-toast)
        - [3.1 Introduction](#31-introduction)
        - [3.2 Interfaces](#32-interfaces)
            - [3.2.1 `__init__`](#321-__init__)
                - [3.2.1.1 Parameter Description](#3211-parameter-description)
                - [3.2.1.2 Example](#3212-example)
        - [3.3 Properties](#33-properties)
        - [3.4 Fingerprint](#34-fingerprint)
        - [3.5 Empty Field Description](#35-empty-field-description)
    - [IV. MutableToast](#iv-mutabletoast)
        - [4.1 Introduction](#41-introduction)
    - [V. Diff](#v-diff)
        - [5.1 Introduction](#51-introduction)
        - [5.2 Properties](#52-properties)
        - [5.3 Example](#53-example)
    - [VI. DiffTool](#vi-difftool)
        - [6.1 Introduction](#61-introduction)
        - [6.2 Interfaces](#62-interfaces)
            - [6.2.1 diff\_full](#621-diff_full)
                - [6.2.1.1 Description](#6211-description)
                - [6.2.1.2 Parameters](#6212-parameters)
                - [6.2.1.3 Return Value](#6213-return-value)
                - [6.2.1.4 Example](#6214-example)
            - [6.2.2 diff\_by\_id](#622-diff_by_id)
                - [6.2.2.1 Description](#6221-description)
                - [6.2.2.2 Parameters](#6222-parameters)
                - [6.2.2.3 Return Value](#6223-return-value)
            - [6.2.3 diff\_without\_time](#623-diff_without_time)
                - [6.2.3.1 Description](#6231-description)
                - [6.2.3.2 Parameters](#6232-parameters)
                - [6.2.3.3 Return Value](#6233-return-value)
            - [6.2.4 to\_json\_str](#624-to_json_str)
                - [6.2.4.1 Description](#6241-description)
                - [6.2.4.2 Parameters](#6242-parameters)
                - [6.2.4.3 Return Value](#6243-return-value)
                - [6.2.4.4 Example](#6244-example)
            - [6.2.5 serialize\_to](#625-serialize_to)
                - [6.2.5.1 Description](#6251-description)
                - [6.2.5.2 Parameters](#6252-parameters)
                - [6.2.5.3 Return Value](#6253-return-value)
                - [6.2.5.4 Example](#6254-example)
    - [VII. SerializeFormat](#vii-serializeformat)
        - [7.1 Introduction](#71-introduction)
        - [7.2 Enum Values](#72-enum-values)

---

## I. Introduction

### 1.1 Import

You can import our library using the following code:

```python
import win_notice_lite as wnl
```

### 1.2 Classes

The library provides the following classes:
- [Listener](#ii-listener)
- [Toast](#iii-toast)
- [MutableToast](#iv-mutabletoast)
- [Diff](#v-diff)
- [DiffTool](#vi-difftool)
- [SerializeFormat](#vii-serializeformat)

---

## II. Listener

### 2.1 Introduction

`Listener` is the core class of this library, used to create notification listener instances, request permissions, and retrieve all current Toast notifications in the system.

### 2.2 Interfaces

This class provides the following interfaces (clickable):

- [`request_permission`](#221-request_permission)
- [`get_all_notifications`](#222-get_all_notifications)
- ~~[`register_toast_handler`](#223-register_toast_handler)~~
- ~~[`unregister_toast_handler`](#224-unregister_toast_handler)~~

---

#### 2.2.1 request_permission

##### 2.2.1.1 Parameters

| Parameter | Type | Description     |
|-----------|------|-----------------|
| `self`    | -    | Instance object |

##### 2.2.1.2 Type

- **Asynchronous method**: Requires `await` to obtain the result.

##### 2.2.1.3 Return Value

Returns a permission status of type `str`. Possible values are:

| Return Value    | Description                                                                 |
|-----------------|-----------------------------------------------------------------------------|
| `"Unspecified"` | User has neither allowed nor denied access                                  |
| `"Allowed"`     | User has granted access to `UserNotificationListener`                       |
| `"Denied"`      | User has denied access to `UserNotificationListener`                        |
| `"Unknown"`     | Unknown error, typically does not occur, reserved for Rust pattern matching |

##### 2.2.1.4 Example

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

##### 2.2.2.1 Parameters

| Parameter | Type | Description     |
|-----------|------|-----------------|
| `self`    | -    | Instance object |

##### 2.2.2.2 Type

- **Asynchronous method**: Requires `await` to obtain the result.

##### 2.2.2.3 Return Value

Returns a `list[Toast]` array containing all Toast notifications currently in the system.  
Returns an empty list `[]` if permission has not been granted.

##### 2.2.2.4 Example

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

##### 2.2.3.1 Description

This interface was intended to provide callback registration in the form of event notifications but has not been officially added. See [here](../About_Event_Notification/Explanation.md) for details.

---

#### 2.2.4 unregister_toast_handler

##### 2.2.4.1 Description

This interface was intended to provide a method for unregistering event notification callback functions but has not been officially added. See [here](../About_Event_Notification/Explanation.md) for details.

---

## III. Toast

### 3.1 Introduction

`Toast` is the core data structure in this library used to represent a single Windows notification.  
It contains all metadata fields of the notification, though some fields cannot be populated with actual content due to Windows API limitations (see field descriptions).

This class is implemented in Rust and exported to Python via PyO3. It supports direct construction and is typically returned by `get_all_notifications()`.

---

### 3.2 Interfaces

#### 3.2.1 `__init__`

##### 3.2.1.1 Parameter Description

| Parameter                  | Type        | Description                                                          |
|----------------------------|-------------|----------------------------------------------------------------------|
| `id`                       | `int`       | System-assigned notification identifier, unique ID                   |
| `name`                     | `str`       | Name of the source application                                       |
| `logo_uri`                 | `str`       | Source application icon URI (empty due to API limitations)           |
| `title`                    | `str`       | Notification title                                                   |
| `message`                  | `str`       | Notification content                                                 |
| `hero_image_uri`           | `str`       | Notification hero image URI (empty due to API limitations)           |
| `inline_images`            | `list[str]` | Array of inline image URIs (empty due to API limitations)            |
| `tag`                      | `str`       | Notification tag (empty due to API limitations)                      |
| `group`                    | `str`       | Notification group (empty due to API limitations)                    |
| `creation_time`            | `str`       | Notification creation time (Windows timestamp, epoch 1601/01/01 UTC) |
| `fingerprint`              | `str`       | Full notification fingerprint (SHA2-256)                             |
| `fingerprint_without_time` | `str`       | Full fingerprint excluding the timestamp                             |

##### 3.2.1.2 Example

```python
import win_notice_lite as wnl
toast = wnl.Toast(
    id=123,
    name="WeChat",
    logo_uri="",
    title="New Message",
    message="You have a new message",
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

### 3.3 Properties

All fields of this class are read-only properties and can be accessed using dot notation:

| Property                   | Type          | Description                        |
|----------------------------|---------------|------------------------------------|
| `id`                       | `int`         | Notification ID                    |
| `name`                     | `str`         | Application name                   |
| `logo_uri`                 | `str`         | Application icon (empty)           |
| `title`                    | `str`         | Title                              |
| `message`                  | `str`         | Content                            |
| `hero_image_uri`           | `str`         | Hero image (empty)                 |
| `inline_images`            | `list[str]`   | Inline images (empty)              |
| `tag`                      | `str`         | Tag (empty)                        |
| `group`                    | `str`         | Group (empty)                      |
| `creation_time`            | `str`         | Creation time                      |
| `fingerprint`              | `str`         | Full fingerprint                   |
| `fingerprint_without_time` | `str`         | Fingerprint excluding timestamp    |

---

### 3.4 Fingerprint

The fingerprint is generated as follows:

1. Concatenate all fields (except `fingerprint` and `fingerprint_without_time`) in order with spaces;
2. Compute the SHA2-256 hash of the concatenated string.

`fingerprint_without_time` is generated by **excluding the `creation_time` field** from the concatenation, allowing for deduplication while ignoring time differences.

---

### 3.5 Empty Field Description

The following fields are always empty strings or empty lists due to Windows API limitations:

- `logo_uri`
- `hero_image_uri`
- `inline_images`
- `tag`
- `group`

---

## IV. MutableToast

### 4.1 Introduction

This class is structurally identical to [Toast](#iii-toast), but every property is modifiable. Other details are not repeated here.

---

## V. Diff

### 5.1 Introduction

`Diff` is a data structure used to represent differences between notification lists, typically returned by the diff calculation methods of `DiffTool`. It contains two fields: a list of new notifications and a list of removed notifications.

---

### 5.2 Properties

| Property | Type          | Description                       |
|----------|---------------|-----------------------------------|
| `new`    | `list[Toast]` | List of newly added notifications |
| `remove` | `list[Toast]` | List of removed notifications     |

---

### 5.3 Example

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

## VI. DiffTool

### 6.1 Introduction

`DiffTool` is a utility class that provides multiple methods for calculating differences between two notification lists and supports serializing notification lists into various formats.

---

### 6.2 Interfaces

This class provides the following interfaces (clickable):
- [`diff_full`](#621-diff_full)
- [`diff_by_id`](#622-diff_by_id)
- [`diff_without_time`](#623-diff_without_time)
- [`to_json_str`](#624-to_json_str)
- [`serialize_to`](#625-serialize_to)

---

#### 6.2.1 diff_full

##### 6.2.1.1 Description

Compares the differences between two notification lists based on the full fingerprint (including timestamp).

##### 6.2.1.2 Parameters

| Parameter | Type            | Description            |
|-----------|-----------------|------------------------|
| `old`     | `list[Toast]`   | Old notification list  |
| `new`     | `list[Toast]`   | New notification list  |

##### 6.2.1.3 Return Value

Returns a [`Diff`](#v-diff) object containing new and removed notifications.

##### 6.2.1.4 Example

```python
diff = wnl.DiffTool.diff_full(old_toasts, new_toasts)
print(len(diff.new), len(diff.remove))
```

---

#### 6.2.2 diff_by_id

##### 6.2.2.1 Description

Compares the differences between two notification lists based on notification IDs.

##### 6.2.2.2 Parameters

| Parameter | Type            | Description            |
|-----------|-----------------|------------------------|
| `old`     | `list[Toast]`   | Old notification list  |
| `new`     | `list[Toast]`   | New notification list  |

##### 6.2.2.3 Return Value

Returns a [`Diff`](#v-diff) object.

---

#### 6.2.3 diff_without_time

##### 6.2.3.1 Description

Compares the differences between two notification lists based on the fingerprint excluding the timestamp, suitable for deduplication scenarios where time differences should be ignored.

##### 6.2.3.2 Parameters

| Parameter | Type            | Description            |
|-----------|-----------------|------------------------|
| `old`     | `list[Toast]`   | Old notification list  |
| `new`     | `list[Toast]`   | New notification list  |

##### 6.2.3.3 Return Value

Returns a [`Diff`](#v-diff) object.

---

#### 6.2.4 to_json_str

##### 6.2.4.1 Description

Serializes a notification list into a formatted JSON string. Returns `"[]"` if serialization fails.

##### 6.2.4.2 Parameters

| Parameter       | Type          | Description                        |
|-----------------|---------------|------------------------------------|
| `notifications` | `list[Toast]` | List of notifications to serialize |

##### 6.2.4.3 Return Value

`str`: Formatted JSON string.

##### 6.2.4.4 Example

```python
json_str = wnl.DiffTool.to_json_str(toasts)
print(json_str)
```

---

#### 6.2.5 serialize_to

##### 6.2.5.1 Description

Serializes a notification list into a string of the specified format.

##### 6.2.5.2 Parameters

| Parameter       | Type                                      | Description                        |
|-----------------|-------------------------------------------|------------------------------------|
| `notifications` | `list[Toast]`                             | List of notifications to serialize |
| `to`            | [`SerializeFormat`](#vii-serializeformat) | Target serialization format        |

##### 6.2.5.3 Return Value

`str`: Formatted string, returns `"[]"` on failure.

##### 6.2.5.4 Example

```python
yaml_str = wnl.DiffTool.serialize_to(toasts, wnl.SerializeFormat.Yaml)
print(yaml_str)
```

---

## VII. SerializeFormat

### 7.1 Introduction

`SerializeFormat` is an enumeration type used to specify serialization formats. This enumeration is **immutable** in Python and cannot be modified.

---

### 7.2 Enum Values

| Enum Value | Description     |
|------------|-----------------|
| `Json`     | JSON format     |
| `Yaml`     | YAML format     |
| `Toml`     | TOML format     |
| `XML`      | XML format      |

---

Last edited: March 16, 2026