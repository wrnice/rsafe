use rustc_serialize::json;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64::FromBase64;

use std::collections::HashMap;


#[derive(Debug)]
pub enum ConnectionError { UnableToConnect , Unauthorized , FieldsAreMissing, BadRequest, UnknownError, InternalServerError, NotFound }

#[derive(Debug, RustcEncodable)]
pub struct RegisterServiceData {
    pub longName: String,
    pub serviceName: String,
    pub serviceHomeDirPath: String,
    pub isPathShared: bool
}

fn get_base64_config() -> ::rustc_serialize::base64::Config {
	::rustc_serialize::base64::Config {
		char_set   : ::rustc_serialize::base64::CharacterSet::Standard,
		newline    : ::rustc_serialize::base64::Newline::LF,
		pad        : true,
		line_length: None,
	}
}

//Register a long name
pub fn register_long_name ( longname : String , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< u16 , ConnectionError > {
	
	println!("App: Begin Registering Long Name ...");

	let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;
	
	let url_dns = "http://localhost:8100/dns".to_string();	
	
	let longnameencoded = ::url::percent_encoding::utf8_percent_encode ( &longname, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
	
	let url_dns_ln = url_dns + "/" + &longnameencoded ;			
	println!("url_dns_ln = {}",&url_dns_ln);
	
	let mut headers: HashMap<String, String> = HashMap::new();
	headers.insert("Authorization".to_string(), bearertoken );
	headers.insert("Connection".to_string(), "close".to_string());
	
	let body = String::new();
	
	//println!("sending request");
	//Send a request to launcher using "request" library	
	let res = ::request::post(&url_dns_ln, &mut headers, &body.into_bytes() );
	//println!("request sent");
				
	//Error handling 
	match res {					
		Err(e) => { println!("{}", e); return Err(ConnectionError::UnableToConnect) }, // couldn't connect
		Ok(res) =>     
		{
			// Handle the response recieved from the launcher
			if res.status_code == 401 {
			println!("401 Unauthorized"); return Err(ConnectionError::Unauthorized)
			} else if res.status_code == 400 {
			println!("400 Bad request"); return Err(ConnectionError::BadRequest)
			}  else if res.status_code == 200 {
			println!("200 Ok"); { return Ok(res.status_code) }
			} else { return Err(ConnectionError::UnknownError) }
			
		} // end Ok
	}; // end match
	
} // end fn

//Register a service
pub fn register_service ( register_service_data : RegisterServiceData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< u16 , ConnectionError > {

	let token = &safe_register_resp.token ;
	let symm_key = &safe_register_resp.symm_key;
	let symm_nonce = &safe_register_resp.symm_nonce;
	
	let bearertoken = "bearer ".to_string()+&token ;
	
	println!("app: begin registering service...");
	
	// Encode the request as a JSON.
	let register_service_json_str = ::rustc_serialize::json::encode(&register_service_data).unwrap_or_else(|a| panic!("{:?}", a));
	//println!("App: RegisterService encoded");

	// Get raw bytes to be encrypted.
	let register_service_bytes = register_service_json_str.into_bytes();

	// Encrypt the raw bytes using the Secret Key (Nonce and Symmetric Key).
	let register_service_encrypted_bytes = ::sodiumoxide::crypto::secretbox::seal(&register_service_bytes,
																			 &symm_nonce,
																			 &symm_key);

	let register_service_json_encrypted_b64 = register_service_encrypted_bytes.to_base64(get_base64_config());	
	
	let url_dns = "http://localhost:8100/dns/";
		
		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Content-Type".to_string(), "application/json".to_string());
		headers.insert("Connection".to_string(), "close".to_string());
	
		//println!("sending request");
		//Send a request to launcher using "request" library	
		let res = ::request::post(&url_dns, &mut headers, &register_service_json_encrypted_b64.into_bytes() );
		//println!("request sent");
		
		//Error handling 
	match res {				
		Err(e) => { println!("{}", e); return Err(ConnectionError::UnableToConnect) }, // couldn't connect
		Ok(res) =>     
	{			
		// Handle the response recieved from the launcher
		if res.status_code == 200 {
		println!("200 Ok"); { return Ok(res.status_code) }
		} else
		{
			
		//this is the launcher's reply, in a b64 string
		let resp_enc_b64 = res.body;

		//we decode it from b64 
		let resp_enc = resp_enc_b64.from_base64().ok().unwrap();
		
		// Decrypt the raw bytes using the Secret Key (Nonce and Symmetric Key).
		let decrypted_response = ::sodiumoxide::crypto::secretbox::open(&resp_enc,
																	&safe_register_resp.symm_nonce,
																	&safe_register_resp.symm_key).ok().unwrap();
																	
		// Get it into a valid UTF-8 String - 
		let decrypted_response_str = String::from_utf8(decrypted_response).ok().unwrap();
																	
		println!( "decr = {}" , &decrypted_response_str );
		
		 if res.status_code == 401 {
		println!("401 Unauthorized"); return Err(ConnectionError::Unauthorized)
		} else if res.status_code == 400 {
		println!("400 Bad Request");
		
		
		return Err(ConnectionError::BadRequest)
		 
		}   else { return Err(ConnectionError::UnknownError) }
	}
	} // end ok
};  // end match
	
} // end fn
