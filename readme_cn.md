# Windows-Notice-Lite

[English](./readme.md)

## 目录

- [Windows-Notice-Lite](#windows-notice-lite)
  - [目录](#目录)
  - [一、简介](#一简介)
  - [二、使用方法](#二使用方法)
    - [2.1 预编译包](#21-预编译包)
    - [2.2 从源构建](#22-从源构建)
      - [2.2.1 准备条件](#221-准备条件)
      - [2.2.2 克隆](#222-克隆)
      - [2.2.3 编译](#223-编译)
      - [2.2.4 安装](#224-安装)
  - [三、文档](#三文档)
  - [四、破坏性更新](#四破坏性更新)
  - [五、许可证](#五许可证)

---

## 一、简介

本项目基于`windows crate`二次开发, 主要包装了在 Windows 系统上获取桌面 toast 通知的相关方法, 并使用 PyO3 进行 Python 绑定以作为 Python 库提供

---

## 二、使用方法

### 2.1 预编译包

你可以使用如下命令使用我们为`windows-amd64`设备制作的预编译版本
```shell
pip install win-notice-lite
```

### 2.2 从源构建

#### 2.2.1 准备条件

 - 系统: Windows 10 及以上
 - 环境: Rustup 全套工具链, Python>=3.10, 2>maturin>=1.9, git

#### 2.2.2 克隆

执行以下命令将项目克隆到本地:
```shell
git clone https://github.com/starwindv/windows-notice-lite.git
cd win-notice-lite
```

#### 2.2.3 编译

```shell
maturin build # 或者 python -m build
```

这一步根据使用命令的不同, 产物分布在以下两个位置:

**使用 maturin**: `.\target\wheels\win_notice_lite-{proj_version}-{py_version}-{py_version}-win_{architecture}.whl`

**使用 python**:
`.\dist\win_notice_lite-{proj_version}.tar.gz` 
和 
`.\dist\win_notice_lite-{proj_version}-{py_version}-{py_version}-win_{architecture}.whl`


#### 2.2.4 安装

```shell
python -m pip install {path_to_wheel}
```

---
## 三、文档

你可以在本项目的根目录下使用`cargo doc`生成文档

---

## 四、破坏性更新

你可以在[BREAKING.md](https://github.com/starwindv/windows-notice-lite/blob/main/BREAKING.md)中找到各版本可能存在的破坏性更新说明

---

## 五、许可证

本项目遵循 GPL-3.0 许可证, 但请注意: 对于列出在[文件](https://github.com/starwindv/windows-notice-lite/blob/main/AUTHORS)中的开发者, 享有 GPL-3.0 许可证例外, 允许他们将本项目用于`https://github.com/Python-island/Python-island`下的每个`branch`项目的开发中, 并且不需要受到传染性影响.

除此之外的所有使用者, 仍需遵守 GPL-3.0 的完整条款.
