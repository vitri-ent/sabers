#[cfg(feature = "zip")]
use std::io::{Read, Seek};
use std::{
	error::Error,
	fs::{self},
	io::{self},
	path::{Path, PathBuf}
};

#[cfg(feature = "zip")]
use zip::{result::ZipError, ZipArchive};

pub trait FileSystem {
	type Err: Error;

	fn list(&self) -> Result<Vec<PathBuf>, Self::Err>;
	fn read_bytes(&mut self, path: &Path) -> Result<Vec<u8>, Self::Err>;
}

pub struct NativeFileSystem {
	root: PathBuf
}

impl NativeFileSystem {
	pub fn new(root: impl Into<PathBuf>) -> Self {
		Self { root: root.into() }
	}
}

impl FileSystem for NativeFileSystem {
	type Err = io::Error;

	fn list(&self) -> Result<Vec<PathBuf>, Self::Err> {
		fs::read_dir(&self.root).and_then(|c| c.map(|e| e.map(|e| PathBuf::from(e.file_name()))).collect())
	}

	fn read_bytes(&mut self, path: &Path) -> Result<Vec<u8>, Self::Err> {
		fs::read(self.root.join(path))
	}
}

#[cfg(feature = "zip")]
pub struct ZipFileSystem<R: Read + Seek> {
	archive: ZipArchive<R>
}

#[cfg(feature = "zip")]
impl<R: Read + Seek> ZipFileSystem<R> {
	pub fn new(reader: R) -> Result<Self, ZipError> {
		Ok(Self { archive: ZipArchive::new(reader)? })
	}
}

#[cfg(feature = "zip")]
impl<R: Read + Seek> FileSystem for ZipFileSystem<R> {
	type Err = ZipError;

	fn list(&self) -> Result<Vec<PathBuf>, Self::Err> {
		Ok(self.archive.file_names().map(PathBuf::from).collect())
	}

	fn read_bytes(&mut self, path: &Path) -> Result<Vec<u8>, Self::Err> {
		self.archive.by_name(&path.as_os_str().to_string_lossy()).and_then(|mut f| {
			let mut vec = Vec::new();
			f.read_to_end(&mut vec)?;
			Ok(vec)
		})
	}
}
