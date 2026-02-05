use nvim_oxi as nvim;
use nvim_oxi::Dictionary;
use nvim_oxi::api;
use nvim_oxi::api::opts::CreateCommandOpts;
use nvim_oxi::api::types::CommandArgs;
use nvim_oxi::Result;

#[nvim::plugin]
fn grit() -> Dictionary {
    nvim::print!("Grit plugin is initializing...");

    // 创建 GritHello 命令选项
    let hello_opts = CreateCommandOpts::builder()
        .bang(true)
        .desc("Prints hello from Grit plugin")
        .build();

    // 注册 GritHello 命令
    let hello_result = api::create_user_command("GritHello", "echo 'Hello World from Grit plugin!'", &hello_opts);

    match hello_result {
        Ok(_) => nvim::print!("GritHello command registered successfully"),
        Err(e) => nvim::print!("Failed to register GritHello command: {:?}", e),
    }

    let grit_opts = CreateCommandOpts::builder()
        .bang(true)
        .desc("Opens a new tab with GritStatus buffer")
        .build();

    let grit_result = api::create_user_command("Grit", grit_command, &grit_opts);

    match grit_result {
        Ok(_) => nvim::print!("Grit command registered successfully"),
        Err(e) => nvim::print!("Failed to register Grit command: {:?}", e),
    }

    Dictionary::new()
}

fn grit_command(_args: CommandArgs) -> Result<()> {
    // 打开新 tab 页
    api::command("tabnew")?;

    // 创建新的可列出但不可编辑的 buffer
    let mut buffer = api::create_buf(true, false)?;

    // 设置 buffer 名称
    buffer.set_name("GritStatus")?;

    // 将新 buffer 设置为当前窗口的 buffer
    api::set_current_buf(&buffer)?;

    // 在 buffer 中添加初始内容
    buffer.set_lines(0..0, false, ["Grit Status Buffer", ""])?;

    // 明确设置 buffer 为不可修改（只读）
    api::command("setlocal nomodifiable")?;
    api::command("setlocal readonly")?;

    nvim::print!("GritStatus buffer created successfully");

    Ok(())
}
