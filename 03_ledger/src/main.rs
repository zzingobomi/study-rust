use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version = "1.0")]
struct App {
    #[clap(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    /// 신규 계좌 작성
    New,
    /// 계좌 입금
    Deposit,
    /// 계좌 출금
    Withdraw,
    /// CSV에서 가져오기
    Import,
    /// 리포트 작성
    Report,
}

fn main() {
    let _args = App::parse();
}
