# Grit Neovim 插件开发文档

## 1. 项目概述

Grit 是一个用 Rust 编写的 Neovim 插件，使用 nvim-oxi 库与 Neovim API 交互。提供了一个 `:Grit` 命令，用于在 Neovim 中管理 Git 状态，包括暂存、取消暂存、丢弃更改等操作。

## 2. 项目结构

```
grit.nvim/
├── src/lib.rs              # Rust 实现（主要插件逻辑）
├── lua/                    # Lua 模块目录
│   └── grit.so            # 编译后的 Rust 共享库
├── plugin/grit.vim        # Vim 脚本初始化文件
├── syntax/grit.vim        # GritStatus 缓冲区语法高亮
├── Cargo.toml             # Cargo 项目配置
├── Cargo.lock             # Cargo 依赖锁定文件
├── Makefile               # 简化编译和测试的 Makefile
├── tests/test_plugin.lua  # 插件集成测试
└── README.md              # 项目说明文档
```

## 3. 依赖要求

- **Neovim 0.9+**：必须包含 LuaJIT 支持
- **Rust 工具链**：用于编译插件（2024 版）
- **nvim-oxi 库**：Rust 绑定的 Neovim API（自动通过 Cargo 管理）

## 4. 编译方法

### 4.1 macOS 系统特殊配置

在 macOS 系统上编译 nvim-oxi 插件时，需要特殊的 linker 配置，因为 Neovim 没有提供单独的动态库，所有符号都嵌入在 Neovim 可执行文件中。

项目已包含必要的配置文件 `.cargo/config.toml`，内容如下：

```toml
[target.x86_64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]

[target.aarch64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]
```

这些配置告诉 Rust 编译器：
- `-undefined dynamic_lookup`：未定义的符号将在运行时动态查找，而不是在编译时强制解析
- 这是因为 Neovim 的 API 符号只在 Neovim 进程运行时才会可用

### 4.2 使用 Makefile 编译

项目提供 Makefile 简化操作，已自动处理 macOS 系统的库命名问题：

#### 4.2.1 编译发布版本（推荐）

```bash
cd /path/to/grit.nvim
make build
```

#### 4.2.2 编译开发版本

```bash
cd /path/to/grit.nvim
make build-dev
```

### 4.3 验证编译结果

检查是否成功生成并复制了共享库：

```bash
cd /path/to/grit.nvim
ls -la lua/
```

## 5. 开发环境配置

### 5.1 使用 lazy.nvim 进行开发

在 Neovim 配置中（如 `~/.config/nvim/lua/plugins/grit.lua`），添加以下配置（注意：将 `/path/to/grit.nvim` 替换为你的实际项目路径）：

```lua
return {
  dir = '/path/to/grit.nvim',
  build = 'make build',
  config = function()
    require('grit')
  end,
}
```

### 5.2 开发工作流程

1. 确保已经编译过插件（如果没有，第一次加载时 lazy.nvim 会自动编译）
2. 在 Neovim 中运行 `:Lazy sync` 来安装插件
3. 使用 `:Grit` 命令测试是否工作
4. 对代码进行修改后，重新编译插件：`make build`
5. 在 Neovim 中重新加载插件：`:Lazy reload grit.nvim`

## 6. 测试插件

### 6.1 运行 Rust 单元测试

```bash
cd /path/to/grit.nvim
cargo test
```

### 6.2 运行插件集成测试

使用 Neovim 运行 Lua 测试脚本：

```bash
cd /path/to/grit.nvim
make test-plugin
```

或者直接运行：

```bash
cd /path/to/grit.nvim
nvim --headless -u NONE -c "luafile tests/test_plugin.lua"
```

### 6.3 手动测试

在 Neovim 中直接测试命令是否正常工作：

```bash
cd /path/to/grit.nvim
nvim -c "Grit" -c "q"
```

## 7. 清理项目

### 7.1 清理编译产物

```bash
cd /path/to/grit.nvim
make clean
```

### 7.2 删除共享库

```bash
cd /path/to/grit.nvim
rm lua/grit.so
```

## 8. 项目维护

### 8.1 更新依赖

```bash
cd /path/to/grit.nvim
cargo update
```

### 8.2 查看项目信息

```bash
cd /path/to/grit.nvim
cargo info
```

## 9. 技术细节

### 9.1 插件入口点

Rust 实现的入口点位于 `src/lib.rs:6`：

```rust
#[nvim::plugin]
fn grit() -> Dictionary {
    // 初始化逻辑
    // 注册 :Grit 命令
    // 返回 Lua 可访问的字典
}
```

### 9.2 命令注册

命令注册使用 Neovim API：

```rust
api::create_user_command(
    "Grit",
    grit_command,
    &opts,
);
```

### 9.3 功能实现

`grit_command` 函数实现了以下功能：
1. 打开新的标签页
2. 创建可列出但不可编辑的缓冲区
3. 设置缓冲区名称为 "GritStatus"
4. 在缓冲区中添加初始内容（包含命令提示）
5. 配置缓冲区属性（只读、无行号等）
6. 设置文件类型为 grit 以启用语法高亮

### 9.4 与 Lua 的交互

插件通过返回 `Dictionary` 类型与 Lua 进行数据交换。

## 10. 语法高亮

syntax/grit.vim 文件提供了 GritStatus 缓冲区的语法高亮：

- `GritHint`：高亮 "Hint: " 为注释
- `GritTab`：高亮 "<tab>" 为关键字
- `GritCommand`：高亮命令前缀（za, s, u, x, c, ?）为关键字

## 11. 常见问题

### 11.1 编译失败

#### 11.1.1 链接器错误（macOS）

**错误：** `Undefined symbols for architecture arm64` 或 `x86_64`，找不到 `_lua_*` 或 `_nvim_*` 符号

**解决：** 确保项目根目录下存在 `.cargo/config.toml` 文件，并且包含正确的 macOS 链接器配置。如果文件不存在，重新创建该文件即可。

#### 11.1.2 找不到 Neovim 头文件

**错误：** 找不到 Neovim 头文件
**解决：** 确保安装了 Neovim 0.9+ 版本，并且在系统路径中可以找到。

### 11.2 插件无法加载

**错误：** Neovim 启动时显示 "E5113: Error while calling lua chunk: cannot load module 'grit'"
**解决：** 检查 `lua/grit.so` 是否存在，以及编译是否成功。

### 11.3 命令无法使用

**错误：** 执行 :Grit 显示 "Not an editor command"
**解决：** 检查插件是否正确加载，以及命令注册是否成功。

## 12. 许可证

项目使用 MIT 许可证（根据 Cargo.toml 配置）。

