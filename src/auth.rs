use rustc_serialize::json;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64::FromBase64;
use std;

use std::collections::HashMap;

#[derive(Debug, RustcEncodable)]
struct App {
	name: String,
	version: String,
	vendor:  String,
	id: String
}

#[derive(Debug, RustcEncodable)]
struct Data {
	app: App,
	permissions: Vec<String>
}

#[derive(Debug, RustcDecodable)]
struct LauncherResponseData {
	token: String,
	permissions: Vec<String>
}

#[derive(Debug, RustcDecodable)]
pub struct AppDetails {
		pub name: String,
		pub version: String,
		pub vendor: String,
		pub id: String,
		pub permissions: Vec<String>
}

#[derive(Debug, RustcDecodable)]
pub struct SafeRegisterResp {
		pub code: u16,
		pub token: String,
}

fn get_base64_config() -> ::rustc_serialize::base64::Config {
	::rustc_serialize::base64::Config {
		char_set   : ::rustc_serialize::base64::CharacterSet::Standard,
		newline    : ::rustc_serialize::base64::Newline::LF,
		pad        : true,
		line_length: None,
	}
}

fn debugprintln ( strg: &str , level: i8 ) {
		
	match level {		
		1 => println!( "{:?}",strg ),
		_ => return
	};
	
}

#[derive(Debug)]
pub enum ConnectionError { UnableToConnect , Unauthorized , FieldsAreMissing, BadRequest, UnknownError }

pub fn register ( appdetails : AppDetails ) -> Result< SafeRegisterResp , ConnectionError > {
		
	let mut safe_register_resp = SafeRegisterResp {
		code : 0,
		token : String::new(),
	};	
	

	// Prepare the request details
	let appli = App {
		name: appdetails.name,
		version: appdetails.version,
		vendor: appdetails.vendor,
		id: appdetails.id,
		};

	let data = Data {
		app: appli,
		permissions: appdetails.permissions
		};
		
	// Encode the data into a JSON
	let payload = ::rustc_serialize::json::encode(&data).unwrap();		
	
	let url = "http://localhost:8100/auth";	

	let mut headers: HashMap<String, String> = HashMap::new();
	headers.insert("Content-Type".to_string(), "application/json".to_string());
	headers.insert("Connection".to_string(), "close".to_string());
	
	println!("sending request");
	//Send a request to launcher using "request" library	
	let res = ::request::post(&url, &mut headers, &payload.into_bytes() );
	
	println!("request sent");		

	//Error handling 
	match res {		
		Err(e) => { println!("{}", e); return Err(ConnectionError::UnableToConnect) }, // couldn't connect
		Ok(res) =>   // success  
	{
	// Handle the response recieved from the launcher
	if res.status_code == 401 {
	println!("401 Unauthorized"); return Err(ConnectionError::Unauthorized)
	} else if res.status_code == 400 {
	println!("400 Fields are missing"); return Err(ConnectionError::FieldsAreMissing)
	} else if res.status_code == 200
			

	{	println!("200 Ok");	
		
		println!("res.body : {:?}", &res.body); 
		
		let launcher_response_data: LauncherResponseData = json::decode(&res.body).ok().unwrap();

		//Our authorization token
		let ourtoken = launcher_response_data.token;

		println!("App: Auth Response decoded");
		
		//update result
		
		safe_register_resp.code = res.status_code;
		safe_register_resp.token = ourtoken;
		
		return Ok(safe_register_resp);	
	
	}  else { return Err(ConnectionError::UnknownError) }	
		
	}
};  //match end
}

pub fn check ( safe_register_resp : &SafeRegisterResp ) -> Result< u16 , ConnectionError > {
	
	let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;
	
	let url = "http://localhost:8100/auth";	

	let mut headers: HashMap<String, String> = HashMap::new();
	headers.insert("Authorization".to_string(), bearertoken );
	headers.insert("Connection".to_string(), "close".to_string());
	
	println!("sending request");
	//Send a request to launcher using "request" library	
	let res = ::request::get(&url, &mut headers );
	
	println!("request sent");	
	
	//Error handling 
	match res {		
		Err(e) => { println!("{}", e); return Err(ConnectionError::UnableToConnect) }, // couldn't connect
		Ok(res) =>   // success  
	{
		// Handle the response recieved from the launcher
		if res.status_code == 401 {
		println!("401 Unauthorized"); return Err(ConnectionError::Unauthorized)
		} else if res.status_code == 400 {
		println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
		} else if res.status_code == 200 {
		println!("200 Ok");		{ return Ok(res.status_code) }
		} else { return Err(ConnectionError::UnknownError) }
	}
};
}

pub fn unregister ( safe_register_resp : &SafeRegisterResp ) -> Result< u16 , ConnectionError > {

	let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;
	
	let url = "http://localhost:8100/auth".to_string();

	let mut headers: HashMap<String, String> = HashMap::new();
	headers.insert("Authorization".to_string(), bearertoken );
	headers.insert("Connection".to_string(), "close".to_string());
	
	
	/*
	 * 
	 * 
	 * 
	 *   TODO PANIC if launcher is killed before application quits
	 * 
	 * 
	 */
	println!("sending request");
	//Send a request to launcher using "request" library	
	let res_token = ::request::delete(&url, &mut headers );
	
	println!("request sent");
	
	//Error handling 
	match res_token {		
		Err(e) => { println!("{}", e); return Err(ConnectionError::UnableToConnect) }, // couldn't connect
		Ok(res) =>  // success    
	{
		// Handle the response recieved from the launcher
		if res.status_code == 401 {
		println!("401 Unauthorized"); return Err(ConnectionError::Unauthorized)
		} else if res.status_code == 400 {
		println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
		} else if res.status_code == 200 {
		println!("200 Ok");		{ return Ok(res.status_code) }
		} else { return Err(ConnectionError::UnknownError) }
	}
};


}



