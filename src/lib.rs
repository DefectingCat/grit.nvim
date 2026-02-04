use nvim_oxi as nvim;
use nvim_oxi::Dictionary;
use nvim_oxi::api;
use nvim_oxi::api::opts::CreateCommandOpts;

#[nvim::plugin]
fn grit() -> Dictionary {
    nvim::print!("Grit plugin is initializing...");

    // 创建命令选项
    let opts = CreateCommandOpts::builder()
        .bang(true)
        .desc("Shows a hello world message from Grit plugin")
        .build();

    // 尝试注册 :GritHello 命令
    let result =
        api::create_user_command("GritHello", "echo 'Hello World from Grit plugin!'", &opts);

    match result {
        Ok(_) => nvim::print!("GritHello command registered successfully"),
        Err(e) => nvim::print!("Failed to register GritHello command: {:?}", e),
    }

    let mut dict = Dictionary::new();
    dict.insert("hello", "world");

    dict
}
