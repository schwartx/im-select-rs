# im-select Windows 实现

这是一个用 Rust 实现的 Windows 版本 im-select 工具。该工具用于在 Windows 系统中切换输入法。

## 说明

* 该项目仅支持 Windows 系统。
* 基于 [im-select](https://github.com/daipeihust/im-select) 项目进行实现，感谢其开源贡献。

## 安装

1. 下载最新的 [release](#) 或通过源码构建。
2. 将 `im-select.exe` 放置在系统路径中，或直接放置于需要执行的目录。

## 使用说明

### 切换输入法

```bash
im-select <输入法标识符>
````

* `<输入法标识符>`：十进制的语言标识符（Locale ID），例如切换到简体中文输入法可能传入 `2052`。
* 示例：

  ```bash
  im-select 2052
  ```

### 查看当前输入法

不带参数运行，会输出当前激活的输入法语言标识符：

```bash
im-select
```

### 查看程序版本

```bash
im-select version
```

### 列出已加载的输入法列表

```bash
im-select hkls
```

---

所有错误信息和调试信息只在 Debug 模式下打印，正常运行时保持简洁输出。

## 开源协议

本项目遵循 [MIT License](LICENSE)。

感谢原作者 [im-select](https://github.com/daipeihust/im-select) 的开源工作！

