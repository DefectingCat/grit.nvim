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
    api::command("tabnew")?;

    // 创建新的可列出但不可编辑的 buffer
    let mut buffer = api::create_buf(false, false)?;

    // 设置 buffer 名称
    if let Err(e) = buffer.set_name("GritStatus") {
        nvim::print!("Failed to set buffer name: {:?}", e);
    }

    // 将新 buffer 设置为当前窗口的 buffer
    api::set_current_buf(&buffer)?;

    // 在 buffer 中添加初始内容
    buffer.set_lines(
        0..0,
        false,
        [
            "Hint: <tab> za toggle | s stage | u unstage | x discard | c commit | ? help",
            "",
        ],
    )?;

    // 明确设置 buffer 为不可修改（只读）
    api::command("setlocal nomodifiable")?;
    api::command("setlocal readonly")?;
    // 关闭当前窗口的行号显示（只影响这个 buffer）
    api::command("setlocal nonumber")?;
    api::command("setlocal norelativenumber")?;
    // 设置 buffer 类型为 nofile，防止保存提示
    api::command("setlocal buftype=nofile")?;
    // 设置文件类型为 grit，以便加载对应的语法高亮
    api::command("setlocal filetype=grit")?;

    // 设置退出快捷键 (q)：关闭当前 tab（无命令显示）
    api::command("nnoremap <silent> <buffer> q :tabclose<CR>")?;

    Ok(())
}
