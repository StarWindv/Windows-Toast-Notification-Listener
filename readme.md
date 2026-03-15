# Windows-Notice-Lite

[简体中文](./readme_cn.md)

## TOC

- [Windows-Notice-Lite](#windows-notice-lite)
    - [TOC](#TOC)
    - [I. Introduction](#i-introduction)
    - [II. Usage](#ii-usage)
        - [2.1 Pre-compiled Package](#21-pre-compiled-package)
        - [2.2 Building from Source](#22-building-from-source)
            - [2.2.1 Prerequisites](#221-prerequisites)
            - [2.2.2 Clone](#222-clone)
            - [2.2.3 Compile](#223-compile)
            - [2.2.4 Install](#224-install)
    - [III. Documentation](#iii-documentation)
    - [IV. Breaking Changes](#iv-breaking-changes)
    - [V. License](#v-license)

---

## I. Introduction

This project is secondary development based on the `windows crate`, primarily wrapping related methods for obtaining desktop toast notifications on Windows systems, and uses PyO3 for Python bindings to provide it as a Python library.

---

## II. Usage

### 2.1 Pre-compiled Package

You can use the following command to use our pre-compiled version for `windows-amd64` devices:

```shell
pip install win-notice-lite
```

### 2.2 Building from Source

#### 2.2.1 Prerequisites

- System: Windows 10 and above
- Environment: Rustup full toolchain, Python>=3.10, maturin>=1.9, git

#### 2.2.2 Clone

Execute the following command to clone the project locally:

```shell
git clone https://github.com/starwindv/windows-notice-lite.git
cd win-notice-lite
```

#### 2.2.3 Compile

```shell
maturin build # or python -m build
```

Depending on the command used, the output will be located in the following two locations:

**Using maturin**:<br>
`.\target\wheels\win_notice_lite-{proj_version}-{py_version}-{py_version}-win_{architecture}.whl`

**Using python**:<br>
`.\dist\win_notice_lite-{proj_version}.tar.gz`<br>
and<br>
`.\dist\win_notice_lite-{proj_version}-{py_version}-{py_version}-win_{architecture}.whl`

#### 2.2.4 Install

```shell
python -m pip install {path_to_wheel}
```

---

## III. Documentation

You can generate code documentation using `cargo doc` in the project root directory, or check our [organized documentation](https://github.com/starwindv/windows-notice-lite/blob/main/doc).

---

## IV. Breaking Changes

You can find descriptions of possible breaking changes in each version in [BREAKING.md](https://github.com/starwindv/windows-notice-lite/blob/main/BREAKING.md).

---

## V. License

This project follows the GPL-3.0 license, but please note: Developers listed in the [AUTHORS](https://github.com/starwindv/windows-notice-lite/blob/main/AUTHORS) file have a GPL-3.0 license exception, allowing them to use this project in the development of each `branch` project under `https://github.com/Python-island/Python-island` without being affected by the copyleft requirement.

All other users must still comply with the full terms of GPL-3.0.
