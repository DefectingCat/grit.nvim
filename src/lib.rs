use nvim_oxi as nvim;
use nvim_oxi::Dictionary;
use nvim_oxi::api;
use nvim_oxi::api::opts::CreateCommandOpts;

#[nvim::plugin]
fn grit() -> Dictionary {
    nvim::print!("Grit plugin is initializing...");

    // 创建命令选项
    let hello_opts = CreateCommandOpts::builder()
        .bang(true)
        .desc("Shows a hello world message from Grit plugin")
        .build();

    // 尝试注册 :GritHello 命令
    let hello_result = api::create_user_command(
        "GritHello",
        "echo 'Hello World from Grit plugin!'",
        &hello_opts,
    );

    match hello_result {
        Ok(_) => nvim::print!("GritHello command registered successfully"),
        Err(e) => nvim::print!("Failed to register GritHello command: {:?}", e),
    }

    // 创建 Grit 命令选项
    let grit_opts = CreateCommandOpts::builder()
        .bang(true)
        .desc("Opens a new tab for Grit operations")
        .build();

    // 尝试注册 :Grit 命令，执行时打开新 tab
    let grit_result = api::create_user_command("Grit", "tabnew", &grit_opts);

    match grit_result {
        Ok(_) => nvim::print!("Grit command registered successfully"),
        Err(e) => nvim::print!("Failed to register Grit command: {:?}", e),
    }

    Dictionary::new()
}
