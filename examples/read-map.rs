use std::{env, fs, path::PathBuf};

use sabers::schemas::{
	beatmap::{anyver_parse_str, AnyverBeatmap},
	mapinfo::v2::MapInfo
};

fn main() -> anyhow::Result<()> {
	let beatmap_path = PathBuf::from(env::args().nth(1).unwrap());

	let mapinfo_path = beatmap_path.join("Info.dat");
	let mut mapinfo = fs::read_to_string(mapinfo_path)?;
	let mapinfo = unsafe { MapInfo::from_str(&mut mapinfo) }.unwrap();

	println!("{} - {}\n\tmapped by {}", mapinfo.song_author_name, mapinfo.song_name, mapinfo.level_author_name);

	for set in mapinfo.beatmap_sets {
		for beatmap_desc in set.beatmaps {
			println!("{} ({}):", beatmap_desc.difficulty, set.characteristic);
			let mut beatmap = fs::read_to_string(beatmap_path.join(beatmap_desc.filename))?;
			let beatmap = unsafe { anyver_parse_str(&mut beatmap) }?;
			match beatmap {
				AnyverBeatmap::V3(beatmap) => {
					println!("\t{} notes", beatmap.color_notes.len());
				}
			}
		}
	}

	Ok(())
}
