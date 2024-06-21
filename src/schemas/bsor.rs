use std::{
	fs::File,
	io::{self, BufReader, Read},
	path::Path
};

use glam::{Quat, Vec3};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
	#[error("I/O error: {0}")]
	IoError(#[from] io::Error),
	#[error("Failed to parse string as UTF-8: {0}")]
	UTF8Error(#[from] std::string::FromUtf8Error)
}

fn read_byte<R: Read>(r: &mut R) -> Result<u8, ParseError> {
	let mut x = [0u8; 1];
	r.read_exact(&mut x)?;
	Ok(x[0])
}

fn read_bool<R: Read>(r: &mut R) -> Result<bool, ParseError> {
	let mut x = [0u8; 1];
	r.read_exact(&mut x)?;
	Ok(x[0] != 0)
}

fn read_i32<R: Read>(r: &mut R) -> Result<i32, ParseError> {
	let mut x = [0u8; 4];
	r.read_exact(&mut x)?;
	Ok(i32::from_le_bytes(x))
}

fn read_f32<R: Read>(r: &mut R) -> Result<f32, ParseError> {
	let mut x = [0u8; 4];
	r.read_exact(&mut x)?;
	Ok(f32::from_le_bytes(x))
}

fn read_str<R: Read>(r: &mut R) -> Result<String, ParseError> {
	let len = read_i32(r)?;
	let mut out = vec![0u8; len as usize];
	r.read_exact(&mut out)?;
	let s = String::from_utf8(out)?;
	Ok(s)
}

#[derive(Debug, Clone)]
pub struct ReplayInfo {
	pub version: String,
	pub game_version: String,
	pub timestamp: String,

	pub player_id: String,
	pub player_name: String,
	pub platform: String,

	pub tracking_system: String,
	pub hmd: String,
	pub controller: String,

	pub song_hash: String,
	pub song_name: String,
	pub mapper: String,
	pub difficulty: String,

	pub score: i32,
	pub mode: String,
	pub environment: String,
	pub modifiers: Vec<String>,
	pub jump_distance: f32,
	pub left_handed: bool,
	pub height: f32,

	pub start_time: f32,
	pub fail_time: f32,
	pub speed: f32
}

impl ReplayInfo {
	pub(crate) fn parse<R: Read>(r: &mut R) -> Result<Self, ParseError> {
		assert_eq!(read_byte(r)?, 0);
		Ok(Self {
			version: read_str(r)?,
			game_version: read_str(r)?,
			timestamp: read_str(r)?,

			player_id: read_str(r)?,
			player_name: read_str(r)?,
			platform: read_str(r)?,

			tracking_system: read_str(r)?,
			hmd: read_str(r)?,
			controller: read_str(r)?,

			song_hash: read_str(r)?,
			song_name: read_str(r)?,
			mapper: read_str(r)?,
			difficulty: read_str(r)?,

			score: read_i32(r)?,
			mode: read_str(r)?,
			environment: read_str(r)?,
			modifiers: read_str(r)?.split(',').map(String::from).collect(),
			jump_distance: read_f32(r)?,
			left_handed: read_bool(r)?,
			height: read_f32(r)?,

			start_time: read_f32(r)?,
			fail_time: read_f32(r)?,
			speed: read_f32(r)?
		})
	}
}

#[derive(Default, Debug, Clone)]
pub struct ReplayFrame {
	pub time: f32,
	pub fps: i32,
	pub head: (Vec3, Quat),
	pub left_hand: (Vec3, Quat),
	pub right_hand: (Vec3, Quat)
}

impl ReplayFrame {
	pub(crate) fn parse<R: Read>(r: &mut R) -> Result<Self, ParseError> {
		Ok(Self {
			time: read_f32(r)?,
			fps: read_i32(r)?,
			head: (Vec3::new(read_f32(r)?, read_f32(r)?, read_f32(r)?), Quat::from_xyzw(read_f32(r)?, read_f32(r)?, read_f32(r)?, read_f32(r)?)),
			left_hand: (Vec3::new(read_f32(r)?, read_f32(r)?, read_f32(r)?), Quat::from_xyzw(read_f32(r)?, read_f32(r)?, read_f32(r)?, read_f32(r)?)),
			right_hand: (Vec3::new(read_f32(r)?, read_f32(r)?, read_f32(r)?), Quat::from_xyzw(read_f32(r)?, read_f32(r)?, read_f32(r)?, read_f32(r)?))
		})
	}
}

#[derive(Debug, Clone)]
pub struct Replay {
	pub info: ReplayInfo,
	pub frames: Vec<ReplayFrame>
}

impl Replay {
	pub fn from_reader<R: Read>(r: &mut R) -> Result<Self, ParseError> {
		assert_eq!(read_i32(r)?, 0x442d3d69);
		assert_eq!(read_byte(r)?, 1);
		let info = ReplayInfo::parse(r)?;
		assert_eq!(read_byte(r)?, 1);
		let n_frames = read_i32(r)? as usize;
		let mut frames = vec![ReplayFrame::default(); n_frames];
		for frame in frames.iter_mut() {
			*frame = ReplayFrame::parse(r)?;
		}
		Ok(Self { info, frames })
	}

	pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ParseError> {
		Self::from_reader(&mut BufReader::new(File::open(path)?))
	}

	pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> Result<Self, ParseError> {
		Self::from_reader(&mut bytes.as_ref())
	}
}
