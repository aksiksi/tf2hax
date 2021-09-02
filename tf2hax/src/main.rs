#![cfg(windows)]

use tf2hax_lib::process::Process;
use tf2hax_lib::PlayerState;

fn main() {
    let process = Process::from_window("Team Fortress 2").expect("Failed to attach to process");
    dbg!(process.name());

    let player = PlayerState::new(process).expect("Failed to create PlayerState");
    println!("player HP: {}", player.get_hp());
}
