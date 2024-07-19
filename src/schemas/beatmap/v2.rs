use std::{
	fs::File,
	io::{BufReader, BufWriter, Read, Write},
	path::Path
};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Beatmap {
	#[serde(rename = "_version")]
	pub version: String,
	#[serde(rename = "_notes")]
	pub notes: Vec<Note>,
	#[serde(rename = "_obstacles")]
	pub obstacles: Vec<Obstacle>,
	#[serde(rename = "_bpmEvents", default = "Vec::new")]
	pub bpm_events: Vec<BpmEvent>
}

impl Beatmap {
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

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NoteType {
	Red = 0,
	Blue = 1,
	Bomb = 3
}

#[derive(Serialize_repr, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NoteDirection {
	Up = 0,
	Down = 1,
	Left = 2,
	Right = 3,
	UpLeft = 4,
	UpRight = 5,
	DownLeft = 6,
	DownRight = 7,
	Any = 8
}

impl<'de> Deserialize<'de> for NoteDirection {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>
	{
		let value = u32::deserialize(deserializer)?;
		match value {
			0 => Ok(NoteDirection::Up),
			1 => Ok(NoteDirection::Down),
			2 => Ok(NoteDirection::Left),
			3 => Ok(NoteDirection::Right),
			4 => Ok(NoteDirection::UpLeft),
			5 => Ok(NoteDirection::UpRight),
			6 => Ok(NoteDirection::DownLeft),
			7 => Ok(NoteDirection::DownRight),
			8 => Ok(NoteDirection::Any),

			// close enough approximation for mapping extensions' 360 degree note rotation
			1000..1023 => Ok(NoteDirection::Down),
			1023..1068 => Ok(NoteDirection::DownLeft),
			1068..1113 => Ok(NoteDirection::Left),
			1113..1158 => Ok(NoteDirection::UpLeft),
			1158..1203 => Ok(NoteDirection::Up),
			1203..1248 => Ok(NoteDirection::UpRight),
			1248..1293 => Ok(NoteDirection::Right),
			1293..1338 => Ok(NoteDirection::DownRight),
			1338..=1360 => Ok(NoteDirection::Down),

			other => Err(serde::de::Error::custom(format!("invalid value: {other}")))
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
	#[serde(rename = "_time")]
	pub beat: f32,
	#[serde(rename = "_lineIndex", deserialize_with = "super::util::deserialize_precision")]
	pub x: f32,
	#[serde(rename = "_lineLayer", deserialize_with = "super::util::deserialize_precision")]
	pub y: f32,
	#[serde(rename = "_type")]
	pub note_type: NoteType,
	#[serde(rename = "_cutDirection")]
	pub direction: NoteDirection,
	#[serde(rename = "_angleOffset")]
	pub angle_offset: Option<f32>,
	#[serde(rename = "_customData")]
	pub custom_data: Option<simd_json::OwnedValue>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Obstacle {
	#[serde(rename = "_time")]
	pub beat: f32,
	#[serde(rename = "_type")]
	pub wall_type: u32,
	#[serde(rename = "_lineIndex", deserialize_with = "super::util::deserialize_precision")]
	pub x: f32,
	#[serde(rename = "_duration")]
	pub duration: f32,
	#[serde(rename = "_width", deserialize_with = "super::util::deserialize_precision")]
	pub width: f32,
	#[serde(rename = "_customData")]
	pub custom_data: Option<simd_json::OwnedValue>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BpmEvent {
	#[serde(rename = "b")]
	pub song_time: f32,
	#[serde(rename = "m")]
	pub beats: f32
}
