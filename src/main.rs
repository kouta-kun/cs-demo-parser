use clap::ArgAction;
use clap::Parser;
use source2_demo::DemoRunner;
use source2_demo::proto::{CDataGccStrike15V2MatchInfo, Message};
use observer::DemoObserver;

mod entities;
mod events;
mod utils;
mod observer;

#[derive(Parser)]
#[command(version = "1.0", name = "cs2-chat-reader")]
struct Cli {
    /// Demo file to read
    file_path: String,
    #[arg(short='c', long, action=ArgAction::SetTrue)]
    find_chat: bool,
    /// Replace chat filter
    #[arg(short, long, default_value="#CS2InterestingReplayTag")]
    filter: String,
    #[arg(short, long, action=ArgAction::SetTrue)]
    programmatic_output: bool,
    #[arg(short='k', long, action=ArgAction::SetTrue)]
    find_kills: bool,
    #[arg(long)]
    killer_name: Option<String>,
    #[arg(long)]
    killed_name: Option<String>,
    #[arg(short, long)]
    weapon: Option<String>,
    /// Filter by kill attribute:
    /// - headshot
    /// - noscope
    /// - thrusmoke
    /// - attackerblind
    /// - assistedflash
    #[arg(short='a', long, verbatim_doc_comment)]
    filter_attributes: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let replay = unsafe {
        memmap2::Mmap::map(
            &std::fs::File::open(&cli.file_path).unwrap()
        ).unwrap()
    };
    let infopath = cli.file_path + ".info";
    if std::fs::exists(&infopath).unwrap_or(false) {
        if let Ok(info_bytes) = std::fs::read(infopath) {
            if let Ok(info) = CDataGccStrike15V2MatchInfo::decode(
                &info_bytes[..]
            ) {
                if let Some(timestamp) = info.matchtime {
                    println!("Played at: {}",
                    chrono::DateTime::from_timestamp_secs(timestamp as u64 as i64).unwrap());
                }
            }
        }
    }
    let mut parser = source2_demo::Parser::new(&replay).unwrap();
    parser.register_observer::<DemoObserver>()
        .borrow_mut()
        .find_chat(cli.find_chat)
        .filter(cli.filter)
        .find_kills(cli.find_kills)
        .killer_name(cli.killer_name)
        .killed_name(cli.killed_name)
        .weapon(cli.weapon)
        .filter_attributes(cli.filter_attributes)
        .programmatic_output(cli.programmatic_output);
    parser.run_to_end().unwrap();
}
