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

// decode and decrypt an encoded b64 string into a string, using the symm and nonce returned when registering
pub fn launcher_resp_decode ( b65_resp : &String , safe_register_resp : &super::auth::SafeRegisterResp ) -> String {
	
	//decode the launcher's reply from b64 
		let resp_enc = b65_resp.from_base64().ok().unwrap();
		
		// Decrypt the raw bytes using the Secret Key (Nonce and Symmetric Key).
		let decrypted_response = ::sodiumoxide::crypto::secretbox::open(&resp_enc,
																	&safe_register_resp.symm_nonce,
																	&safe_register_resp.symm_key).ok().unwrap();
																	
		// Get it into a valid UTF-8 String - 
		let decrypted_response_str = String::from_utf8(decrypted_response).ok().unwrap();
		return decrypted_response_str;
}

// encode and encrypt a string as a b64 string, using the symm and nonce returned when registering
pub fn launcher_string_encode ( thestring : String , safe_register_resp : &super::auth::SafeRegisterResp ) -> String {
	
	// Get raw bytes to be encrypted.
	let bytes = thestring.into_bytes();

	// Encrypt the raw bytes using the Secret Key (Nonce and Symmetric Key).
	let encrypted_bytes = ::sodiumoxide::crypto::secretbox::seal(&bytes,
																 &safe_register_resp.symm_nonce,
																 &safe_register_resp.symm_key);

	let json_encrypted_b64 = encrypted_bytes.to_base64(get_base64_config());
	return json_encrypted_b64;
	
}


