use std::{env, path::PathBuf};

use sabers::schemas::bsor::Replay;

fn main() -> anyhow::Result<()> {
	let replay_path = PathBuf::from(env::args().nth(1).unwrap());

	let replay = Replay::from_file(replay_path)?;

	println!("{} ({})\n\tmapped by {}\n\tplayed by {}", replay.info.song_name, replay.info.difficulty, replay.info.mapper, replay.info.player_name);

	Ok(())
}
