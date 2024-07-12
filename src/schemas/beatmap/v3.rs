use std::{
	fs::File,
	io::{BufReader, BufWriter, Read, Write},
	path::Path
};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Beatmap {
	pub version: String,
	pub color_notes: Vec<ColorNote>,
	pub bomb_notes: Vec<BombNote>,
	pub obstacles: Vec<Obstacle>,
	pub burst_sliders: Vec<BurstSlider>,
	pub bpm_events: Vec<BpmEvent>,
	pub fake_color_notes: Option<Vec<ColorNote>>,
	pub fake_bomb_notes: Option<Vec<ColorNote>>,
	pub fake_obstacles: Option<Vec<ColorNote>>,
	pub fake_burst_sliders: Option<Vec<ColorNote>>
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

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NoteColor {
	Red = 0,
	Blue = 1
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorNote {
	#[serde(rename = "b")]
	pub beat: f32,
	pub x: i8,
	pub y: i8,
	#[serde(rename = "a")]
	pub angle_offset: Option<f32>,
	#[serde(rename = "c")]
	pub color: NoteColor,
	#[serde(rename = "d")]
	pub direction: NoteDirection
}

impl ColorNote {
	/// Returns the event time of this note based on the current BPM.
	pub fn time(&self, bpm: f32) -> f32 {
		self.beat * (60. / bpm)
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BombNote {
	#[serde(rename = "b")]
	pub beat: f32,
	pub x: i8,
	pub y: i8
}

impl BombNote {
	/// Returns the event time of this note based on the current BPM.
	pub fn time(&self, bpm: f32) -> f32 {
		self.beat * (60. / bpm)
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Obstacle {
	#[serde(rename = "b")]
	pub beat: f32,
	pub x: i8,
	pub y: i8,
	#[serde(rename = "d")]
	pub duration: f32,
	#[serde(rename = "w")]
	pub width: u8,
	#[serde(rename = "h")]
	pub height: u8
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BurstSlider {
	#[serde(rename = "b")]
	pub beat: f32,
	pub x: i8,
	pub y: i8,
	#[serde(rename = "c")]
	pub color: NoteColor,
	#[serde(rename = "d")]
	pub direction: NoteDirection,
	#[serde(rename = "tb")]
	pub tail_beat: f32,
	#[serde(rename = "tx")]
	pub tail_x: i8,
	#[serde(rename = "ty")]
	pub tail_y: i8,
	#[serde(rename = "sc")]
	pub num_slices: u8,
	#[serde(rename = "s")]
	pub squish_amount: f32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BpmEvent {
	#[serde(rename = "b")]
	pub song_time: f32,
	#[serde(rename = "m")]
	pub beats: f32
}
