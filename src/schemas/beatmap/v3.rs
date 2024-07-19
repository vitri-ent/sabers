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

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NoteColor {
	Red = 0,
	Blue = 1
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
pub struct ColorNote {
	#[serde(rename = "b")]
	pub beat: f32,
	#[serde(deserialize_with = "super::util::deserialize_precision")]
	pub x: f32,
	#[serde(deserialize_with = "super::util::deserialize_precision")]
	pub y: f32,
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
	#[serde(deserialize_with = "super::util::deserialize_precision")]
	pub x: f32,
	#[serde(deserialize_with = "super::util::deserialize_precision")]
	pub y: f32
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
	#[serde(deserialize_with = "super::util::deserialize_precision")]
	pub x: f32,
	#[serde(deserialize_with = "super::util::deserialize_precision")]
	pub y: f32,
	#[serde(rename = "d")]
	pub duration: f32,
	#[serde(rename = "w", deserialize_with = "super::util::deserialize_precision")]
	pub width: f32,
	#[serde(rename = "h", deserialize_with = "super::util::deserialize_precision")]
	pub height: f32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BurstSlider {
	#[serde(rename = "b")]
	pub beat: f32,
	#[serde(deserialize_with = "super::util::deserialize_precision")]
	pub x: f32,
	#[serde(deserialize_with = "super::util::deserialize_precision")]
	pub y: f32,
	#[serde(rename = "c")]
	pub color: NoteColor,
	#[serde(rename = "d")]
	pub direction: NoteDirection,
	#[serde(rename = "tb")]
	pub tail_beat: f32,
	#[serde(rename = "tx", deserialize_with = "super::util::deserialize_precision")]
	pub tail_x: f32,
	#[serde(rename = "ty", deserialize_with = "super::util::deserialize_precision")]
	pub tail_y: f32,
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
