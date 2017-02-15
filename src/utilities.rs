//utils

use rustc_serialize::json;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64::FromBase64;

pub fn get_base64_config() -> ::rustc_serialize::base64::Config {
	::rustc_serialize::base64::Config {
		char_set   : ::rustc_serialize::base64::CharacterSet::Standard,
		newline    : ::rustc_serialize::base64::Newline::LF,
		pad        : true,
		line_length: None,
	}
}


