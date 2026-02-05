use nvim_oxi as nvim;
use nvim_oxi::Dictionary;
use nvim_oxi::Result;
use nvim_oxi::api;
use nvim_oxi::api::opts::CreateCommandOpts;
use nvim_oxi::api::types::CommandArgs;

#[nvim::plugin]
fn grit() -> Dictionary {
    let grit_opts = CreateCommandOpts::builder()
        .bang(true)
        .desc("Opens a new tab with GritStatus buffer")
        .build();

    if let Err(e) = api::create_user_command("Grit", grit_command, &grit_opts) {
        nvim::print!("Failed to register Grit command: {:?}", e);
    }

    Dictionary::new()
}

fn grit_command(_args: CommandArgs) -> Result<()> {
    // 打开新 tab 页
    match api::command("tabnew") {
        Ok(_) => (),
        Err(e) => {
            nvim::print!("Failed to open new tab: {:?}", e);
            return Ok(());
        }
    }

    // 创建新的可列出但不可编辑的 buffer
    let mut buffer = match api::create_buf(true, false) {
        Ok(buf) => buf,
        Err(e) => {
            nvim::print!("Failed to create buffer: {:?}", e);
            return Ok(());
        }
    };

    // 设置 buffer 名称
    if let Err(e) = buffer.set_name("GritStatus") {
        nvim::print!("Failed to set buffer name: {:?}", e);
    }

    // 将新 buffer 设置为当前窗口的 buffer
    if let Err(e) = api::set_current_buf(&buffer) {
        nvim::print!("Failed to set current buffer: {:?}", e);
        return Ok(());
    }

    // 在 buffer 中添加初始内容
    if let Err(e) = buffer.set_lines(
        0..0,
        false,
        [
            "Hint: <tab> za toggle | s stage | u unstage | x discard | c commit | ? help",
            "",
        ],
    ) {
        nvim::print!("Failed to set buffer lines: {:?}", e);
    }

    // 明确设置 buffer 为不可修改（只读）
    if let Err(e) = api::command("setlocal nomodifiable") {
        nvim::print!("Failed to set nomodifiable: {:?}", e);
    }
    if let Err(e) = api::command("setlocal readonly") {
        nvim::print!("Failed to set readonly: {:?}", e);
    }
    // 关闭当前窗口的行号显示（只影响这个 buffer）
    if let Err(e) = api::command("setlocal nonumber") {
        nvim::print!("Failed to set nonumber: {:?}", e);
    }
    if let Err(e) = api::command("setlocal norelativenumber") {
        nvim::print!("Failed to set norelativenumber: {:?}", e);
    }
    // 设置 buffer 类型为 nofile，防止保存提示
    if let Err(e) = api::command("setlocal buftype=nofile") {
        nvim::print!("Failed to set buftype=nofile: {:?}", e);
    }
    // 设置文件类型为 grit，以便加载对应的语法高亮
    if let Err(e) = api::command("setlocal filetype=grit") {
        nvim::print!("Failed to set filetype=grit: {:?}", e);
    }

    Ok(())
}
