use std::{
	fs::File,
	io::{BufReader, BufWriter, Read, Write},
	path::Path
};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MapInfo {
	#[serde(rename = "_version")]
	pub version: String,
	#[serde(rename = "_songName")]
	pub song_name: String,
	#[serde(rename = "_songSubName")]
	pub song_sub_name: String,
	#[serde(rename = "_songAuthorName")]
	pub song_author_name: String,
	#[serde(rename = "_levelAuthorName")]
	pub level_author_name: String,
	#[serde(rename = "_beatsPerMinute")]
	pub bpm: f32,
	#[serde(rename = "_shuffle")]
	pub shuffle: f32,
	#[serde(rename = "_shufflePeriod")]
	pub shuffle_period: f32,
	#[serde(rename = "_previewStartTime")]
	pub preview_start_time: f32,
	#[serde(rename = "_previewDuration")]
	pub preview_duration: f32,
	#[serde(rename = "_songFilename")]
	pub song_filename: String,
	#[serde(rename = "_coverImageFilename")]
	pub cover_image_filename: String,
	#[serde(rename = "_environmentName")]
	pub environment_name: String,
	#[serde(rename = "_songTimeOffset")]
	pub song_time_offset: f32,
	#[serde(rename = "_difficultyBeatmapSets")]
	pub beatmap_sets: Vec<BeatmapSet>
}

impl MapInfo {
	pub fn serialize_to_string(&self, readable: bool) -> simd_json::Result<String> {
		if readable { simd_json::to_string_pretty(self) } else { simd_json::to_string(self) }
	}

	pub fn serialize_to_writer<W: Write>(&self, writer: W, readable: bool) -> simd_json::Result<()> {
		if readable {
			simd_json::to_writer_pretty(writer, self)
		} else {
			simd_json::to_writer(writer, self)
		}
	}

	pub fn serialize_to_file<P: AsRef<Path>>(&self, path: P, readable: bool) -> simd_json::Result<()> {
		self.serialize_to_writer(&mut BufWriter::new(File::create(path)?), readable)
	}

	pub fn serialize_to_bytes(&self, readable: bool) -> simd_json::Result<Vec<u8>> {
		if readable { simd_json::to_vec_pretty(self) } else { simd_json::to_vec(self) }
	}

	pub fn from_string(s: impl Into<String>) -> simd_json::Result<Self> {
		unsafe { simd_json::from_str(&mut s.into()) }
	}

	pub fn from_reader<R: Read>(reader: R) -> simd_json::Result<Self> {
		simd_json::from_reader(reader)
	}

	pub fn from_file<P: AsRef<Path>>(path: P) -> simd_json::Result<Self> {
		Self::from_reader(BufReader::new(File::open(path)?))
	}
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BeatmapSet {
	#[serde(rename = "_beatmapCharacteristicName")]
	pub characteristic: String,
	#[serde(rename = "_difficultyBeatmaps")]
	pub beatmaps: Vec<Beatmap>
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone)]
#[repr(i32)]
pub enum DifficultyRank {
	Unknown = 0,
	Easy = 1,
	Normal = 3,
	Hard = 5,
	Expert = 7,
	ExpertPlus = 9
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Beatmap {
	#[serde(rename = "_difficulty")]
	pub difficulty: String,
	#[serde(rename = "_difficultyRank")]
	pub difficulty_rank: DifficultyRank,
	#[serde(rename = "_beatmapFilename")]
	pub filename: String,
	#[serde(rename = "_noteJumpMovementSpeed")]
	pub njs: f32,
	#[serde(rename = "_noteJumpStartBeatOffset")]
	pub njs_offset: f32
}
