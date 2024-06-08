use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Beatmap {
	pub color_notes: Vec<ColorNote>
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum NoteColor {
	Red = 0,
	Blue = 1
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
	// wtf is none if its different from any?
	None = 9
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorNote {
	#[serde(rename = "b")]
	pub beat: f32,
	pub x: u8,
	pub y: u8,
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
