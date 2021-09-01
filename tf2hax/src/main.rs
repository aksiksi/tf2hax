#![cfg(windows)]

fn main() {
    let player = tf2hax_lib::PlayerState::new();
    assert!(player.is_some(), "failed to create PlayerState");
    let player = player.unwrap();
    println!("player HP: {}", player.get_hp());
}
