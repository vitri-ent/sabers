use std::sync::OnceLock;

use regex::Regex;

pub mod v3;

#[derive(Debug, Clone)]
pub enum AnyverBeatmap {
	V3(v3::Beatmap)
}

static VERSION_REGEX: OnceLock<Regex> = OnceLock::new();

pub unsafe fn anyver_parse_str(bytes: &mut str) -> simd_json::Result<AnyverBeatmap> {
	let caps = VERSION_REGEX
		.get_or_init(|| Regex::new(r#"\{\s*?"version"\s*?:\s*?"(\d\.\d\.\d)""#).unwrap())
		.captures(bytes)
		.unwrap();
	let version = caps.get(1).unwrap().as_str();
	if version.starts_with('3') {
		Ok(AnyverBeatmap::V3(simd_json::from_str(bytes)?))
	} else {
		unimplemented!("unimplemented beatmap version '{version}'")
	}
}
