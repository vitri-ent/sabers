use std::{
	convert::Infallible,
	fmt::Display,
	io,
	path::{Path, PathBuf},
	str::FromStr
};

use thiserror::Error;

use super::v2;
use crate::schemas::beatmap::{self, AnyverBeatmap, AnyverParseError};

#[derive(Debug, Clone, Hash)]
pub enum BeatmapCharacteristic {
	Standard,
	NoArrows,
	OneSaber,
	Degree360,
	Degree90,
	Legacy,
	Other(String)
}

impl Display for BeatmapCharacteristic {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Standard => f.write_str("Standard"),
			Self::NoArrows => f.write_str("NoArrows"),
			Self::OneSaber => f.write_str("OneSaber"),
			Self::Degree360 => f.write_str("360Degree"),
			Self::Degree90 => f.write_str("90Degree"),
			Self::Legacy => f.write_str("Legacy"),
			Self::Other(s) => f.write_str(&s)
		}
	}
}

impl FromStr for BeatmapCharacteristic {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"Standard" => Self::Standard,
			"NoArrows" => Self::NoArrows,
			"OneSaber" => Self::OneSaber,
			"360Degree" => Self::Degree360,
			"90Degree" => Self::Degree90,
			"Legacy" => Self::Legacy,
			s => Self::Other(s.to_string())
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Difficulty {
	Easy = 1,
	Normal = 3,
	Hard = 5,
	Expert = 7,
	ExpertPlus = 9
}

impl Display for Difficulty {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Easy => f.write_str("Easy"),
			Self::Normal => f.write_str("Normal"),
			Self::Hard => f.write_str("Hard"),
			Self::Expert => f.write_str("Expert"),
			Self::ExpertPlus => f.write_str("ExpertPlus")
		}
	}
}

impl FromStr for Difficulty {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"Easy" => Ok(Self::Easy),
			"Normal" => Ok(Self::Normal),
			"Hard" => Ok(Self::Hard),
			"Expert" => Ok(Self::Expert),
			"Expert+" | "ExpertPlus" => Ok(Self::ExpertPlus),
			other => Err(other.to_string())
		}
	}
}

impl Difficulty {
	pub fn rank(&self) -> u8 {
		*self as u8
	}

	pub fn from_rank(rank: u8) -> Option<Self> {
		match rank {
			1 => Some(Self::Easy),
			3 => Some(Self::Normal),
			5 => Some(Self::Hard),
			7 => Some(Self::Expert),
			9 => Some(Self::ExpertPlus),
			_ => None
		}
	}
}

#[derive(Debug, Clone)]
pub struct SongMeta {
	pub title: String,
	pub subtitle: Option<String>,
	pub author: String,
	pub cover_image_path: PathBuf
}

#[derive(Debug, Clone)]
pub struct AudioMeta {
	pub bpm: f32,
	pub song_time_offset: f32,
	pub audio_path: PathBuf
}

#[derive(Debug, Clone)]
pub struct Beatmap {
	pub difficulty: Difficulty,
	pub characteristic: BeatmapCharacteristic,
	pub map: beatmap::standard::Beatmap,
	pub njs: f32,
	pub njs_offset: f32
}

#[derive(Debug, Error)]
pub enum MapReadError {
	#[error("Failed to parse map info: {0}")]
	InfoParseError(#[from] simd_json::Error),
	#[error("Failed to parse beatmap: {0}")]
	MapParseError(#[from] AnyverParseError),
	#[error("Failed to read file: {0}")]
	IoError(#[from] io::Error),
	#[error("Unexepcted beatmap difficulty '{0}'")]
	BadDifficulty(String)
}

#[derive(Debug)]
pub struct MapInfo {
	pub song: SongMeta,
	pub audio: AudioMeta,
	pub maps: Vec<Beatmap>
}

impl MapInfo {
	pub fn from_dir<P: AsRef<Path>>(path: P) -> Result<Self, MapReadError> {
		let path = path.as_ref();
		let mut info_dat = path.join("Info.dat");
		if !info_dat.exists() {
			info_dat = path.join("info.dat");
		}
		let info = v2::MapInfo::from_file(info_dat)?;

		let mut maps = Vec::new();
		for set in info.beatmap_sets {
			let characteristic = BeatmapCharacteristic::from_str(&set.characteristic).unwrap();
			for map in set.beatmaps {
				let beatmap = beatmap::standard::Beatmap::from_any(AnyverBeatmap::from_file(path.join(map.filename))?, info.bpm);
				maps.push(Beatmap {
					difficulty: Difficulty::from_str(&map.difficulty).map_err(MapReadError::BadDifficulty)?,
					characteristic: characteristic.clone(),
					map: beatmap,
					njs: map.njs,
					njs_offset: map.njs_offset
				});
			}
		}
		Ok(Self {
			audio: AudioMeta {
				bpm: info.bpm,
				audio_path: path.join(info.song_filename),
				song_time_offset: info.song_time_offset
			},
			song: SongMeta {
				title: info.song_name,
				subtitle: (!info.song_sub_name.is_empty()).then_some(info.song_sub_name),
				author: info.song_author_name,
				cover_image_path: path.join(info.cover_image_filename)
			},
			maps
		})
	}
}
