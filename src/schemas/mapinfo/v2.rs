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
	pub shuffle: i32,
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
	pub unsafe fn from_str(s: &mut str) -> simd_json::Result<Self> {
		simd_json::from_str(s)
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
