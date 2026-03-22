# ZINW - 用 Rust 编写操作系统

本项目是一个基于 x86_64 架构的裸机操作系统，使用 Rust 语言编写。

## 运行所需软件

### 1. Rust（Nightly 版本）

本项目使用了一些仅在 Nightly 版本中可用的特性，需要安装 Rust Nightly 工具链。

安装 Rust（推荐使用 [rustup](https://rustup.rs/)）：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

切换到 Nightly 工具链：

```bash
rustup override set nightly
```

### 2. Rust 标准库源码组件

```bash
rustup component add rust-src
```

### 3. llvm-tools-preview 组件

`bootimage` 工具依赖此组件来生成可启动的磁盘镜像：

```bash
rustup component add llvm-tools-preview
```

### 4. bootimage 工具

用于将编译好的内核打包成可启动的磁盘镜像：

```bash
cargo install bootimage
```

### 5. QEMU（虚拟机模拟器）

用于在虚拟机中运行和测试操作系统镜像：

- **Linux（Debian/Ubuntu）**：
  ```bash
  sudo apt install qemu-system-x86
  ```
- **macOS**：
  ```bash
  brew install qemu
  ```
- **Windows**：
  从 [QEMU 官网](https://www.qemu.org/download/#windows) 下载安装包。

## 构建与运行

### 构建项目

```bash
cargo build
```

### 运行操作系统

```bash
cargo run
```

### 运行测试

```bash
cargo test
```

## 项目结构

```
ZINW/
├── src/
│   ├── main.rs          # 内核入口点
│   ├── lib.rs           # 公共库（测试框架等）
│   ├── vga_buffer.rs    # VGA 文本模式驱动
│   ├── serial.rs        # 串口输出驱动
│   ├── interrupts.rs    # 中断描述符表（IDT）
│   └── gdt.rs           # 全局描述符表（GDT）
├── tests/               # 集成测试
├── my_os.json           # 自定义目标平台配置
└── Cargo.toml           # 项目依赖配置
```
