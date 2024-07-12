use std::{env, path::PathBuf};

use sabers::schemas::mapinfo::standard::MapInfo;

fn main() -> anyhow::Result<()> {
	let beatmap_path = PathBuf::from(env::args().nth(1).unwrap());

	let mapinfo = MapInfo::from_dir(beatmap_path)?;

	println!("{} - {}\n\t", mapinfo.song.author, mapinfo.song.title);

	for beatmap in mapinfo.maps {
		println!("{} ({}):", beatmap.difficulty, beatmap.characteristic);
		println!("\t{} notes", beatmap.map.beats.len());
		for beat in beatmap.map.beats {
			println!("\t{:?}", beat);
		}
	}

	Ok(())
}
