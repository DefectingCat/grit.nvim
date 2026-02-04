# Grit Neovim 插件开发文档

## 1. 项目概述

Grit 是一个用 Rust 编写的简单 Neovim 插件，使用 nvim-oxi 库与 Neovim API 交互。提供了一个 `:GritHello` 命令，在 Neovim 命令行中打印 "Hello World from Grit plugin!"。

## 2. 项目结构

```
/home/xfy/Developer/grit/
├── src/lib.rs              # Rust 实现（主要插件逻辑）
├── lua/                    # Lua 模块目录
│   └── grit.so            # 编译后的 Rust 共享库
├── plugin/grit.vim        # Vim 脚本初始化文件
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

### 4.1 使用 Cargo 直接编译

#### 4.1.1 开发模式编译（调试版本）

```bash
cd /home/xfy/Developer/grit
cargo build
```

生成文件：`target/debug/libgrit.so`

#### 4.1.2 发布模式编译（优化版本）

```bash
cd /home/xfy/Developer/grit
cargo build --release
```

生成文件：`target/release/libgrit.so`

### 4.2 使用 Makefile 编译

项目提供 Makefile 简化操作：

#### 4.2.1 编译发布版本（推荐）

```bash
cd /home/xfy/Developer/grit
make build  # 等同于 cargo build --release
```

#### 4.2.2 编译开发版本

```bash
cd /home/xfy/Developer/grit
make build-dev  # 等同于 cargo build
```

### 4.3 安装共享库

编译完成后，需要将共享库复制到 Lua 模块目录：

```bash
cd /home/xfy/Developer/grit
cp target/release/libgrit.so lua/grit.so  # 发布版本
# 或者使用开发版本
cp target/debug/libgrit.so lua/grit.so
```

### 4.4 验证编译结果

检查是否成功生成并复制了共享库：

```bash
cd /home/xfy/Developer/grit
ls -la lua/
```

## 5. 使用插件

### 5.1 直接加载（自动方式）

直接启动 Neovim，插件会通过 `plugin/grit.vim` 自动加载。

### 5.2 使用 lazy.nvim 包管理器加载（推荐用于开发）

在 Neovim 配置中（如 `~/.config/nvim/lua/plugins/grit.lua`），添加以下配置：

```lua
return {
  -- 使用本地路径
  dir = '/home/xfy/Developer/grit',
  build = 'make build',
  config = function()
    require('grit')
  end,
}
```

#### 关键点说明

- **`dir` 字段**：指向本地插件仓库的绝对路径
- **`build` 字段**：lazy.nvim 会在安装/更新时自动运行此命令来编译插件
- **`config` 字段**：插件加载后调用 `require('grit')` 来初始化

#### 使用流程

1. 确保已经编译过插件（如果没有，第一次加载时 lazy.nvim 会自动编译）
2. 在 Neovim 中运行 `:Lazy sync` 来安装插件
3. 使用 `:GritHello` 命令测试是否工作

### 5.3 执行命令

在 Neovim 中，输入以下命令：

```vim
:GritHello
```

会在命令行中显示：`Hello World from Grit plugin!`

### 5.4 检查 Lua 模块

在 Neovim 的 Lua 环境中，可以检查插件是否正确加载：

```vim
:lua print(vim.inspect(require('grit')))
```

会显示：

```
{ hello = "world" }
```

## 6. 测试插件

### 6.1 运行 Rust 单元测试

```bash
cd /home/xfy/Developer/grit
cargo test
```

### 6.2 运行插件集成测试

使用 Neovim 运行 Lua 测试脚本：

```bash
cd /home/xfy/Developer/grit
make test-plugin
```

或者直接运行：

```bash
cd /home/xfy/Developer/grit
nvim --headless -u NONE -c "luafile tests/test_plugin.lua"
```

### 6.3 手动测试

在 Neovim 中直接测试命令是否正常工作：

```bash
cd /home/xfy/Developer/grit
nvim -c "GritHello" -c "q"
```

## 7. 清理项目

### 7.1 清理编译产物

```bash
cd /home/xfy/Developer/grit
cargo clean  # 清理所有编译产物
# 或者使用 Makefile
make clean
```

### 7.2 删除共享库

```bash
cd /home/xfy/Developer/grit
rm lua/grit.so
```

## 8. 项目维护

### 8.1 更新依赖

```bash
cd /home/xfy/Developer/grit
cargo update
```

### 8.2 查看项目信息

```bash
cd /home/xfy/Developer/grit
cargo info
```

## 9. 技术细节

### 9.1 插件入口点

Rust 实现的入口点位于 `src/lib.rs:6`：

```rust
#[nvim::plugin]
fn grit() -> Dictionary {
    // 初始化逻辑
    // 注册 :GritHello 命令
    // 返回 Lua 可访问的字典
}
```

### 9.2 命令注册

命令注册使用 Neovim API：

```rust
api::create_user_command(
    "GritHello",
    "echo 'Hello World from Grit plugin!'",
    &opts,
);
```

### 9.3 与 Lua 的交互

插件通过返回 `Dictionary` 类型与 Lua 进行数据交换。

## 10. 常见问题

### 10.1 编译失败

**错误：** 找不到 Neovim 头文件
**解决：** 确保安装了 Neovim 0.9+ 版本，并且在系统路径中可以找到。

### 10.2 插件无法加载

**错误：** Neovim 启动时显示 "E5113: Error while calling lua chunk: cannot load module 'grit'"
**解决：** 检查 `lua/grit.so` 是否存在，以及编译是否成功。

### 10.3 命令无法使用

**错误：** 执行 :GritHello 显示 "Not an editor command"
**解决：** 检查插件是否正确加载，以及命令注册是否成功。

## 11. 许可证

项目使用 MIT 许可证（根据 Cargo.toml 配置）。

