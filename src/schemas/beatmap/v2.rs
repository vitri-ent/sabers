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
	#[serde(rename = "_bpmEvents")]
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

	pub fn from_path<P: AsRef<Path>>(&self, path: P) -> simd_json::Result<Self> {
		Self::from_reader(BufReader::new(File::open(path)?))
	}
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum NoteType {
	Red = 0,
	Blue = 1,
	Bomb = 3
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
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
	Any = 8,
	None = 9
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
	#[serde(rename = "_time")]
	pub beat: f32,
	#[serde(rename = "_lineIndex")]
	pub x: i8,
	#[serde(rename = "_lineLayer")]
	pub y: i8,
	#[serde(rename = "_type")]
	pub note_type: NoteType,
	#[serde(rename = "_cutDirection")]
	pub direction: NoteDirection,
	#[serde(rename = "_angleOffset")]
	pub angle_offset: f32,
	#[serde(rename = "_customData")]
	pub custom_data: Option<simd_json::OwnedValue>
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum WallType {
	Wall = 0,
	Ceiling = 1
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Obstacle {
	#[serde(rename = "_time")]
	pub beat: f32,
	#[serde(rename = "_type")]
	pub wall_type: WallType,
	#[serde(rename = "_lineIndex")]
	pub x: i8,
	#[serde(rename = "_duration")]
	pub duration: f32,
	#[serde(rename = "_width")]
	pub width: u8,
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
