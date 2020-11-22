mod configure;
mod assets;
mod systemd;
mod api;

use crate::api::HttpApi;
use crate::configure::Command;

#[tokio::main]
async fn main() {
    let opt = configure::parse_and_configure().await;

    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt()
            .finish()).expect("set gloabl tracing subsriber");

    if opt.auto_update {
        todo!("--auto-update");
    }

    match opt.command {
        Some(Command::Run) | None => {
            // dbg!(assets::Assets::prepare(dbg!(assets::Cpu::detect())));
        }
        Some(Command::Systemd) => systemd::systemd_system(opt),
        Some(Command::SystemdUser) => systemd::systemd_user(opt),
        Some(Command::Configure) => (),
    }
}
