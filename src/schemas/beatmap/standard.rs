use std::{io::Read, path::Path};

use super::{AnyverBeatmap, AnyverParseError, v2, v3};

#[derive(Debug, Clone)]
pub struct Beatmap {
	pub beats: Vec<Beat>,
	pub bombs: Vec<Bomb>,
	pub obstacles: Vec<Obstacle>,
	pub chains: Vec<Chain>
}

impl Beatmap {
	pub fn from_any(beatmap: AnyverBeatmap, bpm: f32) -> Self {
		match beatmap {
			AnyverBeatmap::V2(v2) => Self::from_v2(v2, bpm),
			AnyverBeatmap::V3(v3) => Self::from_v3(v3, bpm)
		}
	}

	pub fn from_file<P: AsRef<Path>>(path: P, bpm: f32) -> Result<Self, AnyverParseError> {
		Ok(Self::from_any(AnyverBeatmap::from_file(path)?, bpm))
	}

	pub fn from_string(s: impl Into<String>, bpm: f32) -> Result<Self, AnyverParseError> {
		Ok(Self::from_any(AnyverBeatmap::from_string(s)?, bpm))
	}

	pub fn from_reader<R: Read>(reader: R, bpm: f32) -> Result<Self, AnyverParseError> {
		Ok(Self::from_any(AnyverBeatmap::from_reader(reader)?, bpm))
	}

	pub fn from_v2(beatmap: v2::Beatmap, bpm: f32) -> Self {
		let bpm_events = beatmap.bpm_events.into_iter().map(BpmEvent::from).collect();
		let bpm_tracker = BpmTracker::new(bpm, bpm_events);

		let mut beats = Vec::new();
		let mut bombs = Vec::new();
		for note in beatmap.notes {
			if note.note_type == v2::NoteType::Bomb {
				let mut bomb: Bomb = note.try_into().unwrap();
				bomb.time = bpm_tracker.beat_to_song_time(bomb.beat);
				bombs.push(bomb);
			} else {
				let mut beat: Beat = note.try_into().unwrap();
				beat.time = bpm_tracker.beat_to_song_time(beat.beat);
				beats.push(beat);
			}
		}

		let obstacles = beatmap
			.obstacles
			.into_iter()
			.map(Obstacle::from)
			.map(|mut x| {
				let start_time = bpm_tracker.beat_to_song_time(x.beat);
				let end_time = bpm_tracker.beat_to_song_time(x.beat + x.duration_beats);
				x.time = start_time;
				x.end_time = end_time;
				x.duration = end_time - start_time;
				x
			})
			.collect();

		Self {
			beats,
			bombs,
			obstacles,
			chains: Vec::new()
		}
	}

	pub fn from_v3(beatmap: v3::Beatmap, bpm: f32) -> Self {
		let bpm_events = beatmap.bpm_events.into_iter().map(BpmEvent::from).collect();
		let bpm_tracker = BpmTracker::new(bpm, bpm_events);

		let bombs = beatmap
			.bomb_notes
			.into_iter()
			.map(Bomb::from)
			.map(|mut x| {
				x.time = bpm_tracker.beat_to_song_time(x.beat);
				x
			})
			.collect();
		let beats = beatmap
			.color_notes
			.into_iter()
			.map(Beat::from)
			.map(|mut x| {
				x.time = bpm_tracker.beat_to_song_time(x.beat);
				x
			})
			.collect();
		let obstacles = beatmap
			.obstacles
			.into_iter()
			.map(Obstacle::from)
			.map(|mut x| {
				let start_time = bpm_tracker.beat_to_song_time(x.beat);
				let end_time = bpm_tracker.beat_to_song_time(x.beat + x.duration_beats);
				x.time = start_time;
				x.end_time = end_time;
				x.duration = end_time - start_time;
				x
			})
			.collect();
		let chains = beatmap
			.burst_sliders
			.into_iter()
			.map(Chain::from)
			.map(|mut x| {
				x.time = bpm_tracker.beat_to_song_time(x.beat);
				x.tail_time = bpm_tracker.beat_to_song_time(x.tail_beat);
				x
			})
			.collect();

		Self { beats, bombs, obstacles, chains }
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NoteColor {
	Red = 0,
	Blue = 1
}

impl TryFrom<v2::NoteType> for NoteColor {
	type Error = v2::NoteType;

	fn try_from(value: v2::NoteType) -> Result<Self, Self::Error> {
		match value {
			v2::NoteType::Red => Ok(NoteColor::Red),
			v2::NoteType::Blue => Ok(NoteColor::Blue),
			v => Err(v)
		}
	}
}

impl From<v3::NoteColor> for NoteColor {
	fn from(value: v3::NoteColor) -> Self {
		match value {
			v3::NoteColor::Red => NoteColor::Red,
			v3::NoteColor::Blue => NoteColor::Blue
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl From<v2::NoteDirection> for NoteDirection {
	fn from(value: v2::NoteDirection) -> Self {
		match value {
			v2::NoteDirection::Up => NoteDirection::Up,
			v2::NoteDirection::Down => NoteDirection::Down,
			v2::NoteDirection::Left => NoteDirection::Left,
			v2::NoteDirection::Right => NoteDirection::Right,
			v2::NoteDirection::UpLeft => NoteDirection::UpLeft,
			v2::NoteDirection::UpRight => NoteDirection::UpRight,
			v2::NoteDirection::DownLeft => NoteDirection::DownLeft,
			v2::NoteDirection::DownRight => NoteDirection::DownRight,
			v2::NoteDirection::Any => NoteDirection::Any
		}
	}
}

impl From<v3::NoteDirection> for NoteDirection {
	fn from(value: v3::NoteDirection) -> Self {
		match value {
			v3::NoteDirection::Up => NoteDirection::Up,
			v3::NoteDirection::Down => NoteDirection::Down,
			v3::NoteDirection::Left => NoteDirection::Left,
			v3::NoteDirection::Right => NoteDirection::Right,
			v3::NoteDirection::UpLeft => NoteDirection::UpLeft,
			v3::NoteDirection::UpRight => NoteDirection::UpRight,
			v3::NoteDirection::DownLeft => NoteDirection::DownLeft,
			v3::NoteDirection::DownRight => NoteDirection::DownRight,
			v3::NoteDirection::Any => NoteDirection::Any
		}
	}
}

#[derive(Debug, Clone)]
pub struct Beat {
	beat: f32,
	pub time: f32,
	pub x: f32,
	pub y: f32,
	pub angle_offset: Option<f32>,
	pub color: NoteColor,
	pub direction: NoteDirection
}

impl TryFrom<v2::Note> for Beat {
	type Error = v2::Note;

	fn try_from(value: v2::Note) -> Result<Self, Self::Error> {
		if value.note_type == v2::NoteType::Bomb {
			return Err(value);
		}

		Ok(Self {
			beat: value.beat,
			time: 0.0,
			x: value.x,
			y: value.y,
			angle_offset: value.angle_offset,
			color: value.note_type.try_into().unwrap(),
			direction: value.direction.into()
		})
	}
}

impl From<v3::ColorNote> for Beat {
	fn from(value: v3::ColorNote) -> Self {
		Self {
			beat: value.beat,
			time: 0.0,
			x: value.x,
			y: value.y,
			angle_offset: value.angle_offset,
			color: value.color.into(),
			direction: value.direction.into()
		}
	}
}

#[derive(Debug, Clone)]
pub struct Bomb {
	beat: f32,
	pub time: f32,
	pub x: f32,
	pub y: f32
}

impl TryFrom<v2::Note> for Bomb {
	type Error = v2::Note;

	fn try_from(value: v2::Note) -> Result<Self, Self::Error> {
		if value.note_type != v2::NoteType::Bomb {
			return Err(value);
		}
		Ok(Self {
			beat: value.beat,
			time: 0.0,
			x: value.x,
			y: value.y
		})
	}
}

impl From<v3::BombNote> for Bomb {
	fn from(value: v3::BombNote) -> Self {
		Self {
			beat: value.beat,
			time: 0.0,
			x: value.x,
			y: value.y
		}
	}
}

#[derive(Debug, Clone)]
pub struct Obstacle {
	beat: f32,
	pub time: f32,
	pub x: f32,
	pub y: f32,
	duration_beats: f32,
	pub duration: f32,
	pub end_time: f32,
	pub width: f32,
	pub height: f32
}

impl From<v2::Obstacle> for Obstacle {
	fn from(value: v2::Obstacle) -> Self {
		let (y, height) = match value.wall_type {
			0 => (0., 5.),
			1 => (2., 3.),
			t => {
				let mut value = t;
				let h = if t >= 4001 && t <= 410000 {
					value -= 4001;
					value / 1000
				} else {
					value - 1000
				};
				let h = ((h as f32 / 1000.) * 5.) * 1000. + 1000.;

				let mut sh = 0.0;
				let mut v1 = t;
				if t >= 4001 && t <= 410000 {
					v1 -= 4001;
					sh = v1 as f32 % 1000.;
				}

				let l = ((sh / 750.) * 5.) * 1000. + 1334.;
				(l / 1000. - 2., (h - 1000.) / 1000.)
			}
		};
		Self {
			beat: value.beat,
			time: 0.,
			x: value.x,
			y,
			duration_beats: value.duration,
			duration: 0.0,
			end_time: 0.0,
			height,
			width: value.width
		}
	}
}

impl From<v3::Obstacle> for Obstacle {
	fn from(value: v3::Obstacle) -> Self {
		Self {
			beat: value.beat,
			time: 0.0,
			x: value.x,
			y: value.y,
			duration_beats: value.duration,
			duration: 0.0,
			end_time: 0.0,
			height: value.height,
			width: value.width
		}
	}
}

#[derive(Debug, Clone)]
pub struct Chain {
	beat: f32,
	pub time: f32,
	pub x: f32,
	pub y: f32,
	pub color: NoteColor,
	pub direction: NoteDirection,
	tail_beat: f32,
	pub tail_time: f32,
	pub tail_x: f32,
	pub tail_y: f32,
	pub num_slices: u8,
	pub squish_factor: f32
}

impl From<v3::BurstSlider> for Chain {
	fn from(value: v3::BurstSlider) -> Self {
		Self {
			beat: value.beat,
			time: 0.0,
			x: value.x,
			y: value.y,
			color: value.color.into(),
			direction: value.direction.into(),
			tail_beat: value.tail_beat,
			tail_time: 0.0,
			tail_x: value.tail_x,
			tail_y: value.tail_y,
			num_slices: value.num_slices,
			squish_factor: value.squish_amount
		}
	}
}

struct BpmEvent {
	song_time: f32,
	beats: f32
}

impl From<v2::BpmEvent> for BpmEvent {
	fn from(value: v2::BpmEvent) -> Self {
		Self {
			song_time: value.song_time,
			beats: value.beats
		}
	}
}

impl From<v3::BpmEvent> for BpmEvent {
	fn from(value: v3::BpmEvent) -> Self {
		Self {
			song_time: value.song_time,
			beats: value.beats
		}
	}
}

#[derive(Clone)]
struct BpmChangeEvent {
	bpm: f32,
	start_time: f32,
	start_bpm_time: f32
}

struct BpmTracker {
	base_bpm: f32,
	changes: Vec<BpmChangeEvent>
}

impl BpmTracker {
	pub fn new(start_bpm: f32, events: Vec<BpmEvent>) -> Self {
		let mut base_bpm = start_bpm;
		let mut changes = Vec::new();
		if !events.is_empty() {
			let mut n_base = 0;
			if events[0].song_time == 0. {
				n_base = 1;
				base_bpm = events[0].beats;
				changes.push(BpmChangeEvent {
					bpm: base_bpm,
					start_time: 0.,
					start_bpm_time: 0.
				});
			}

			for event in events.iter().skip(n_base) {
				let last_change = changes.get(changes.len() - 1).cloned().unwrap_or(BpmChangeEvent {
					bpm: base_bpm,
					start_time: 0.,
					start_bpm_time: 0.
				});
				changes.push(BpmChangeEvent {
					bpm: event.beats,
					start_bpm_time: event.song_time,
					start_time: last_change.start_time + ((event.song_time - last_change.start_bpm_time) / last_change.bpm) * 60.0
				});
			}
		}
		Self { base_bpm, changes }
	}

	pub fn beat_to_song_time(&self, time: f32) -> f32 {
		if self.changes.is_empty() {
			return time * (60.0 / self.base_bpm);
		}

		let mut i = 0;
		while i < self.changes.len() - 1 && self.changes[i + 1].start_bpm_time < time {
			i += 1;
		}
		let prev_bpm_change = &self.changes[i];
		prev_bpm_change.start_time + ((time - prev_bpm_change.start_bpm_time) / prev_bpm_change.bpm) * 60.0
	}
}

#[cfg(test)]
mod tests {
	use super::Beatmap;

	#[test]
	fn test_mapping_extensions_ok() {
		assert!(Beatmap::from_file("tests/data/maps/1579c_ExpertPlusStandard.dat", 222.0).is_ok());
	}
}
