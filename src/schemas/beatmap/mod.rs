use std::{
	fs::File,
	io::{BufReader, Read, Write},
	path::Path
};

use serde::Deserialize;
use simd_json::{
	OwnedValue,
	derived::{ValueObjectAccess, ValueTryAsScalar}
};
use thiserror::Error;

pub mod standard;
mod util;
pub mod v2;
pub mod v3;

#[derive(Debug, Clone)]
pub enum AnyverBeatmap {
	V2(v2::Beatmap),
	V3(v3::Beatmap)
}

impl From<v2::Beatmap> for AnyverBeatmap {
	fn from(value: v2::Beatmap) -> Self {
		Self::V2(value)
	}
}
impl From<v3::Beatmap> for AnyverBeatmap {
	fn from(value: v3::Beatmap) -> Self {
		Self::V3(value)
	}
}

impl TryInto<v2::Beatmap> for AnyverBeatmap {
	type Error = Self;

	fn try_into(self) -> Result<v2::Beatmap, Self::Error> {
		match self {
			AnyverBeatmap::V2(v2) => Ok(v2),
			v => Err(v)
		}
	}
}

impl TryInto<v3::Beatmap> for AnyverBeatmap {
	type Error = Self;

	fn try_into(self) -> Result<v3::Beatmap, Self::Error> {
		match self {
			AnyverBeatmap::V3(v3) => Ok(v3),
			v => Err(v)
		}
	}
}

#[derive(Debug, Error)]
pub enum AnyverParseError {
	#[error("Failed to read file: {0}")]
	IoError(#[from] std::io::Error),
	#[error("Failed to deserialize JSON: {0}")]
	SimdJson(#[from] simd_json::Error),
	#[error("Malformed JSON; expected field to be {}, got {}", .0.expected, .0.got)]
	ExpectedType(#[from] simd_json::TryTypeError),
	#[error("Unsupported map version type: {0}")]
	UnsupportedVersion(String)
}

impl AnyverBeatmap {
	pub fn serialize_to_string(&self, readable: bool) -> simd_json::Result<String> {
		match self {
			Self::V2(b) => b.serialize_to_string(readable),
			Self::V3(b) => b.serialize_to_string(readable)
		}
	}

	pub fn serialize_to_writer<W: Write>(&self, writer: W, readable: bool) -> simd_json::Result<()> {
		match self {
			Self::V2(b) => b.serialize_to_writer(writer, readable),
			Self::V3(b) => b.serialize_to_writer(writer, readable)
		}
	}

	pub fn serialize_to_file<P: AsRef<Path>>(&self, path: P, readable: bool) -> simd_json::Result<()> {
		match self {
			Self::V2(b) => b.serialize_to_file(path, readable),
			Self::V3(b) => b.serialize_to_file(path, readable)
		}
	}

	pub fn serialize_to_bytes(&self, readable: bool) -> simd_json::Result<Vec<u8>> {
		match self {
			Self::V2(b) => b.serialize_to_bytes(readable),
			Self::V3(b) => b.serialize_to_bytes(readable)
		}
	}

	pub fn from_string(s: impl Into<String>) -> Result<Self, AnyverParseError> {
		Self::inner_parse(unsafe { simd_json::from_str(&mut s.into())? })
	}

	pub fn from_reader<R: Read>(reader: R) -> Result<Self, AnyverParseError> {
		Self::inner_parse(simd_json::from_reader(reader)?)
	}

	pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, AnyverParseError> {
		Self::from_reader(BufReader::new(File::open(path)?))
	}

	fn inner_parse(value: OwnedValue) -> Result<Self, AnyverParseError> {
		if let Some(version) = value.get("_version") {
			let version = version.try_as_str()?;
			if version.starts_with("2.") {
				return Ok(AnyverBeatmap::V2(v2::Beatmap::deserialize(value)?));
			} else {
				return Err(AnyverParseError::UnsupportedVersion(version.to_string()));
			}
		} else if let Some(version) = value.get("version") {
			let version = version.try_as_str()?;
			if version.starts_with("3.") {
				return Ok(AnyverBeatmap::V3(v3::Beatmap::deserialize(value)?));
			} else {
				return Err(AnyverParseError::UnsupportedVersion(version.to_string()));
			}
		}
		Err(AnyverParseError::UnsupportedVersion(String::from("unknown")))
	}
}
