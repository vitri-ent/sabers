use serde::{Deserialize, Deserializer};

pub fn deserialize_precision<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f32, D::Error> {
	let original = i32::deserialize(deserializer)?;
	if original <= -1000 || original >= 1000 {
		Ok(if original.is_negative() { original as f32 / 1000. + 1. } else { original as f32 / 1000. - 1. })
	} else {
		Ok(original as f32)
	}
}
