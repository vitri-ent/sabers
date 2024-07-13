use std::{
	fs::File,
	io::{self, BufReader, BufWriter, Read, Write},
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

fn write_str<W: Write>(w: &mut W, s: &str) -> Result<(), io::Error> {
	w.write_all(&(s.len() as i32).to_le_bytes())?;
	w.write_all(s.as_bytes())?;
	Ok(())
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
	pub fn from_reader<R: Read>(r: &mut R) -> Result<Self, ParseError> {
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
			modifiers: match read_str(r)?.as_str() {
				"" => Vec::new(),
				v => v.split(',').map(String::from).collect()
			},
			jump_distance: read_f32(r)?,
			left_handed: read_bool(r)?,
			height: read_f32(r)?,

			start_time: read_f32(r)?,
			fail_time: read_f32(r)?,
			speed: read_f32(r)?
		})
	}

	pub fn is_same_map(&self, other: &Self) -> bool {
		self.song_hash == other.song_hash && self.mode == other.mode && self.difficulty == other.difficulty
	}

	pub fn serialize_to_writer<W: Write>(&self, w: &mut W) -> Result<(), io::Error> {
		w.write_all(&[0])?;

		write_str(w, &self.version)?;
		write_str(w, &self.game_version)?;
		write_str(w, &self.timestamp)?;

		write_str(w, &self.player_id)?;
		write_str(w, &self.player_name)?;
		write_str(w, &self.platform)?;

		write_str(w, &self.tracking_system)?;
		write_str(w, &self.hmd)?;
		write_str(w, &self.controller)?;

		write_str(w, &self.song_hash)?;
		write_str(w, &self.song_name)?;
		write_str(w, &self.mapper)?;
		write_str(w, &self.difficulty)?;

		w.write_all(&self.score.to_le_bytes())?;
		write_str(w, &self.mode)?;
		write_str(w, &self.environment)?;
		write_str(w, &self.modifiers.join(","))?;
		w.write_all(&self.jump_distance.to_le_bytes())?;
		w.write_all(&(self.left_handed as u8).to_le_bytes())?;
		w.write_all(&self.height.to_le_bytes())?;

		w.write_all(&self.start_time.to_le_bytes())?;
		w.write_all(&self.fail_time.to_le_bytes())?;
		w.write_all(&self.speed.to_le_bytes())?;

		Ok(())
	}

	pub fn serialize_to_vector(&self) -> Vec<u8> {
		let mut out = Vec::new();
		self.serialize_to_writer(&mut out).unwrap();
		out
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
	pub fn from_reader<R: Read>(r: &mut R) -> Result<Self, ParseError> {
		Ok(Self {
			time: read_f32(r)?,
			fps: read_i32(r)?,
			head: (Vec3::new(read_f32(r)?, read_f32(r)?, read_f32(r)?), Quat::from_xyzw(read_f32(r)?, read_f32(r)?, read_f32(r)?, read_f32(r)?)),
			left_hand: (Vec3::new(read_f32(r)?, read_f32(r)?, read_f32(r)?), Quat::from_xyzw(read_f32(r)?, read_f32(r)?, read_f32(r)?, read_f32(r)?)),
			right_hand: (Vec3::new(read_f32(r)?, read_f32(r)?, read_f32(r)?), Quat::from_xyzw(read_f32(r)?, read_f32(r)?, read_f32(r)?, read_f32(r)?))
		})
	}

	pub fn serialize_to_writer<W: Write>(&self, w: &mut W) -> Result<(), io::Error> {
		w.write_all(&self.time.to_le_bytes())?;
		w.write_all(&self.fps.to_le_bytes())?;
		for (pos, rot) in [self.head, self.left_hand, self.right_hand] {
			w.write_all(&pos.x.to_le_bytes())?;
			w.write_all(&pos.y.to_le_bytes())?;
			w.write_all(&pos.z.to_le_bytes())?;
			w.write_all(&rot.x.to_le_bytes())?;
			w.write_all(&rot.y.to_le_bytes())?;
			w.write_all(&rot.z.to_le_bytes())?;
			w.write_all(&rot.w.to_le_bytes())?;
		}
		Ok(())
	}

	pub fn serialize_to_vector(&self) -> Vec<u8> {
		let mut out = Vec::with_capacity(4 + 4 + ((3 + 4) * 4 * 3));
		self.serialize_to_writer(&mut out).unwrap();
		out
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
		let info = ReplayInfo::from_reader(r)?;
		assert_eq!(read_byte(r)?, 1);
		let n_frames = read_i32(r)? as usize;
		let mut frames = vec![ReplayFrame::default(); n_frames];
		for frame in frames.iter_mut() {
			*frame = ReplayFrame::from_reader(r)?;
		}
		Ok(Self { info, frames })
	}

	pub fn serialize_to_writer<W: Write>(&self, w: &mut W) -> Result<(), io::Error> {
		w.write_all(&[0x69, 0x3d, 0x2d, 0x44, 1])?;
		self.info.serialize_to_writer(w)?;
		w.write_all(&[1])?;
		w.write_all(&(self.frames.len() as i32).to_le_bytes())?;
		for frame in &self.frames {
			frame.serialize_to_writer(w)?;
		}
		Ok(())
	}

	pub fn serialize_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error> {
		self.serialize_to_writer(&mut BufWriter::new(File::create(path)?))
	}

	pub fn serialize_to_bytes(&self) -> Vec<u8> {
		let mut out = Vec::new();
		self.serialize_to_writer(&mut out).unwrap();
		out
	}

	pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ParseError> {
		Self::from_reader(&mut BufReader::new(File::open(path)?))
	}

	pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> Result<Self, ParseError> {
		Self::from_reader(&mut bytes.as_ref())
	}
}

#[cfg(test)]
mod tests {
	use std::f32::consts::PI;

	use super::*;

	#[test]
	fn test_replay_frame_ser() {
		let frame = ReplayFrame {
			time: 0.3,
			fps: 60,
			head: (Vec3::new(0.0, 1.8, 0.0), Quat::IDENTITY),
			left_hand: (Vec3::new(-1.0, 1.6, 0.25), Quat::from_rotation_y(PI / 2.)),
			right_hand: (Vec3::new(1.0, 1.6, 0.25), Quat::from_rotation_z(PI / 2.))
		};
		let serialized_frame = frame.serialize_to_vector();
		let deserialized_frame = ReplayFrame::from_reader(&mut serialized_frame.as_slice()).unwrap();
		assert_eq!(deserialized_frame.time, frame.time);
		assert_eq!(deserialized_frame.fps, frame.fps);
		assert_eq!(deserialized_frame.head, frame.head);
		assert_eq!(deserialized_frame.left_hand, frame.left_hand);
		assert_eq!(deserialized_frame.right_hand, frame.right_hand);
	}

	#[test]
	fn test_replay_info_ser() {
		let Replay { info, .. } = Replay::from_file("tests/data/replays/replay1.bsor").unwrap();

		let serialized_info = info.serialize_to_vector();
		let deserialized_info = ReplayInfo::from_reader(&mut serialized_info.as_slice()).unwrap();
		assert_eq!(deserialized_info.mapper, info.mapper);
	}

	#[test]
	fn test_replay_parse() {
		let replay = Replay::from_file("tests/data/replays/replay1.bsor").unwrap();

		assert_eq!(replay.info.version, "0.7.1");
		assert_eq!(replay.info.player_name, "Reddek");
		assert_eq!(replay.info.height, 1.71 - f32::EPSILON);
		assert_eq!(replay.info.start_time, 0.0);
		assert_eq!(replay.info.modifiers.len(), 0);
		assert_eq!(replay.info.difficulty, "ExpertPlus");
	}

	#[test]
	fn test_replay_ser() {
		let replay = std::fs::read("tests/data/replays/replay1.bsor").unwrap();
		let parsed_replay = Replay::from_bytes(&replay).unwrap();
		let serialized_replay = parsed_replay.serialize_to_bytes();
		assert_eq!(serialized_replay, replay[..serialized_replay.len()]); // slice is temporary until the other fields are finished
	}
}
