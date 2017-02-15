use rustc_serialize::json;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64::FromBase64;

use std::collections::HashMap;

use utilities::*;


#[derive(Debug)]
pub enum ConnectionError { UnableToConnect , Unauthorized , FieldsAreMissing, BadRequest, UnknownError, InternalServerError, NotFound }

#[derive(Debug, RustcEncodable)]
pub struct RegisterServiceData {
    pub longName: String,
    pub serviceName: String,
    pub serviceHomeDirPath: String,
    pub isPathShared: bool
}

#[derive(Debug, RustcEncodable)]
pub struct AddServiceData {
    pub longName: String,
    pub serviceName: String,
    pub serviceHomeDirPath: String,
    pub isPathShared: bool
}

#[derive(Debug, RustcEncodable)]
pub struct ReadPublicFileData {
	pub filePath: String,
	pub serviceName : String,
	pub longName : String,
	pub offset: i64,
	pub length: i64
}

//fn get_base64_config() -> ::rustc_serialize::base64::Config {
	//::rustc_serialize::base64::Config {
		//char_set   : ::rustc_serialize::base64::CharacterSet::Standard,
		//newline    : ::rustc_serialize::base64::Newline::LF,
		//pad        : true,
		//line_length: None,
	//}
//}



//Register a DNS long name
pub fn register_long_name ( longname : String , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< u16 , ConnectionError > {
	
	println!("App: Begin Registering Long Name ...");

	let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;
	
	let url_dns = "http://localhost:8100/dns".to_string();	
	
	let longname_url_encoded = ::url::percent_encoding::utf8_percent_encode ( &longname, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
	
	let url = url_dns + "/" + &longname_url_encoded ;			
	println!("url = {}",&url);
	
	let mut headers: HashMap<String, String> = HashMap::new();
	headers.insert("Authorization".to_string(), bearertoken );
	headers.insert("Connection".to_string(), "close".to_string());
	
	let body = String::new();
	
	//println!("sending request");
	//Send a request to launcher using "request" library	
	let res = ::request::post(&url, &mut headers, &body.into_bytes() );
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

//Delete a DNS long name
pub fn delete_long_name ( longname : String , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< u16 , ConnectionError > {
	
	println!("App: Begin deleting Long Name ...");

	let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;
	
	let url_dns = "http://localhost:8100/dns".to_string();	
	
	let longname_url_encoded = ::url::percent_encoding::utf8_percent_encode ( &longname, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
	
	let url_dns_del = url_dns + "/" + &longname_url_encoded ;			
	//println!("url_dns_del = {}",&url_dns_del);
	
	let mut headers: HashMap<String, String> = HashMap::new();
	headers.insert("Authorization".to_string(), bearertoken );
	headers.insert("Connection".to_string(), "close".to_string());
	
	let body = String::new();
	
	//println!("sending request");
	//Send a request to launcher using "request" library	
	let res = ::request::delete(&url_dns_del, &mut headers );
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

//Delete a service from a DNS long name
/*
 * 
 * *****************************************************************
 * 
 *                ISSUE :   THIS ALSO DELETES THE DNS NAME  
 * 
 * 					https://maidsafe.atlassian.net/browse/CS-63
 * 
 * *****************************************************************
 * 
 */
pub fn delete_service ( longname : String , servicename : String , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< u16 , ConnectionError > {
	
	println!("App: Begin deleting service...");

	let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;
	
	let url_dns = "http://localhost:8100/dns".to_string();	
	
	let longname_url_encoded = ::url::percent_encoding::utf8_percent_encode ( &longname, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
	let servicename_url_encoded = ::url::percent_encoding::utf8_percent_encode ( &servicename, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
	
	let url_dns_del = url_dns + "/" + &servicename_url_encoded + "/" + &longname_url_encoded ;			
	//println!("url_dns_del = {}",&url_dns_del);
	
	let mut headers: HashMap<String, String> = HashMap::new();
	headers.insert("Authorization".to_string(), bearertoken );
	headers.insert("Connection".to_string(), "close".to_string());
	
	let body = String::new();
	
	//println!("sending request");
	//Send a request to launcher using "request" library	
	let res = ::request::delete(&url_dns_del, &mut headers );
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
	
	let bearertoken = "bearer ".to_string()+&token ;
	
	println!("app: begin registering service...");
	
	// Encode the request as a JSON.
	let register_service_json_str = ::rustc_serialize::json::encode(&register_service_data).unwrap_or_else(|a| panic!("{:?}", a));
	//println!("App: RegisterService encoded");

	// encode the JSON as a b64 string, using the symm and nonce returned when registering
	let register_service_json_encrypted_b64 = launcher_string_encode ( register_service_json_str , &safe_register_resp );
	
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
			
		//decode the launcher's reply		
		let decrypted_response_str = launcher_resp_decode ( &res.body, &safe_register_resp ) ;
																	
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

//Add a service to an existing DNS name
pub fn add_service ( add_service_data : AddServiceData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< u16 , ConnectionError > {

	let token = &safe_register_resp.token ;
	
	let bearertoken = "bearer ".to_string()+&token ;
	
	println!("app: begin adding service...");
	
	// Encode the request as a JSON.
	let add_service_json_str = ::rustc_serialize::json::encode(&add_service_data).unwrap_or_else(|a| panic!("{:?}", a));
	//println!("App: RegisterService encoded");

	// encode the JSON as a b64 string, using the symm and nonce returned when registering
	let add_service_json_encrypted_b64 = launcher_string_encode ( add_service_json_str , &safe_register_resp );
	
	let url_dns = "http://localhost:8100/dns/";
		
	let mut headers: HashMap<String, String> = HashMap::new();
	headers.insert("Authorization".to_string(), bearertoken );
	headers.insert("Content-Type".to_string(), "application/json".to_string());
	headers.insert("Connection".to_string(), "close".to_string());

	//println!("sending request");
	//Send a request to launcher using "request" library	
	let res = ::request::put(&url_dns, &mut headers, &add_service_json_encrypted_b64.into_bytes() );
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
			
		//decode the launcher's reply		
		let decrypted_response_str = launcher_resp_decode ( &res.body, &safe_register_resp ) ;
															
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

// List the DNS names registered by the user.
pub fn list_names ( safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< String , ConnectionError > {

		println!("App: Begin listing DNS names...");	
				
		let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;	
				
		// URL to send our 'ls' request to
		
		let url_dns = "http://localhost:8100/dns".to_string();
				
		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Connection".to_string(), "close".to_string());
		
		//println!("sending request");
		//Send a request to launcher using "request" library
		let res = ::request::get(&url_dns, &mut headers );
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
			println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 200 {
			println!("200 Ok"); { 
			
		//decode the launcher's reply		
		let decrypted_response_str = launcher_resp_decode ( &res.body, &safe_register_resp ) ;
				
		return Ok(decrypted_response_str) }
		
		} else { return Err(ConnectionError::UnknownError) }
	}  

};	//match end
}	//fn end

// List the services registered for a DNS name.
pub fn list_services ( longname : &String , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< String , ConnectionError > {

		println!("App: Begin listing services...");	
				
		let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;	
		
		let longname_url_encoded = ::url::percent_encoding::utf8_percent_encode ( &longname, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
		
		// URL to send our 'ls' request to
		let url_dns = "http://localhost:8100/dns".to_string();
		let url = url_dns + "/" + &longname_url_encoded;
		//println!("url_dns_serv = {}",&url);
				
		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Connection".to_string(), "close".to_string());
		
		//println!("sending request");
		//Send a request to launcher using "request" library
		let res = ::request::get(&url, &mut headers );
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
			println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 200 {
			println!("200 Ok"); { 
			
		//decode the launcher's reply		
		let decrypted_response_str = launcher_resp_decode ( &res.body, &safe_register_resp ) ;
				
		return Ok(decrypted_response_str) }
		
		} else { return Err(ConnectionError::UnknownError) }
	}  

};	//match end
}	//fn end

//Get the home directory associated to a service.
pub fn get_public_dir ( longname : String, servicename : String , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< super::nfs::GetDirResponseData , ConnectionError > {

		println!("App: Begin reading public directory...");	
				
		let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;	
		
		let longname_url_encoded = ::url::percent_encoding::utf8_percent_encode ( &longname, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
		let service_url_encoded = ::url::percent_encoding::utf8_percent_encode ( &servicename, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
		
		// URL to send our 'ls' request to
		let url_dns = "http://localhost:8100/dns".to_string();
		let url = url_dns + "/" + &service_url_encoded + "/" + &longname_url_encoded ;
		println!("url_dns_get_dir = {}",&url);
		
		let mut headers: HashMap<String, String> = HashMap::new();
		
		//only send auth for private data  - for now, leaving this commented for testing		
		//headers.insert("Authorization".to_string(), bearertoken );
		
		headers.insert("Connection".to_string(), "close".to_string());
		
		//println!("sending request");
		//Send a request to launcher using "request" library
		let res = ::request::get(&url, &mut headers );
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
			println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 200 {
			println!("200 Ok"); { 
				
		//println!("App: GetDir Response : {:?}", res.body );
		
		// Decode the JSON into expected response structure - in this case a directory response as
		// stated in the RFC.
		let get_dir_response: super::nfs::GetDirResponseData = ::rustc_serialize::json::decode(&res.body)
																 .unwrap_or_else(|e| panic!("{:?}", e));
		//println!("App: GetDir Response decoded.");	
			
		return Ok(get_dir_response) }
		
		} else { return Err(ConnectionError::UnknownError) }
	}  

};	//match end
}	//fn end

//Get a public file from to a service.
pub fn get_public_file( read_public_file_data : ReadPublicFileData , safe_register_resp : &super::auth::SafeRegisterResp )
						-> Result< super::nfs::FileReadInfo , ConnectionError > {

		println!("App: Begin reading public file...");	
				
		let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;	
		
		let longname_url_encoded = ::url::percent_encoding::utf8_percent_encode ( &read_public_file_data.longName, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
		let service_url_encoded = ::url::percent_encoding::utf8_percent_encode ( &read_public_file_data.serviceName, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
		let filepath_url_encoded = ::url::percent_encoding::utf8_percent_encode ( &read_public_file_data.filePath, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
		
		let offset = read_public_file_data.offset ;
		let length = read_public_file_data.length ;
		
		// URL to send our 'ls' request to
		let url_dns = "http://localhost:8100/dns".to_string();
		let url1 = url_dns + "/" + &service_url_encoded + "/" + &longname_url_encoded + "/" + &filepath_url_encoded  ;
		println!("url_dns_get_dir = {}",&url1);
		
		let mut url_dns_read = url1.clone() ;
		
		// append encrypted b64 length and offset if needed		
		if  length > 0 && offset > 0 {	
			let queryparams = "offset=".to_string() + &offset.to_string() + "&length=" + &length.to_string();
			let queryparamsencoded = launcher_string_encode ( queryparams, &safe_register_resp );
			url_dns_read = url1 +  "?" + &queryparamsencoded; }
		else if  length == 0 && offset > 0  {
			let queryparams = "offset=".to_string() + &offset.to_string();
			let queryparamsencoded = launcher_string_encode ( queryparams, &safe_register_resp );
			url_dns_read = url1 +  "?" + &queryparamsencoded; }
		else if  length > 0 && offset == 0  {
			let queryparams = "length=".to_string() + &length.to_string();
			let queryparamsencoded = launcher_string_encode ( queryparams, &safe_register_resp );
			url_dns_read = url1 +  "?" + &queryparamsencoded; }	

		
		let mut headers: HashMap<String, String> = HashMap::new();
		
		//only send auth for private data  - for now, leaving this commented for testing		
		//headers.insert("Authorization".to_string(), bearertoken );
		
		headers.insert("Connection".to_string(), "close".to_string());
		
		//println!("sending request");
		//Send a request to launcher using "request" library
		let res = ::request::get(&url_dns_read, &mut headers );
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
			println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 200 {
			println!("200 Ok"); { 
				
		//println!("App: GetDir Response : {:?}", res.body );
		
				//decode the launcher's reply		
		let decrypted_response_str = launcher_resp_decode ( &res.body, &safe_register_resp ) ;
		
		let read_file_resp_body = ::rustc_serialize::json::decode(&decrypted_response_str)
																 .unwrap_or_else(|e| panic!("{:?}", e));
		//println!("App: GetFile Response decoded.");

		//get headers
		let headers = res.headers;
			
		//println!( "get file headers = {:?}", headers);
		
		let mut file_size = "";
		let mut file_name = "";
		let mut file_created_time = "";
		let mut file_modified_time = "";
		let mut file_metadata = "";

	
		match headers.get("file-size") {
			Some ( val ) => { file_size = val; },
			_ => { file_size = "0"; }
		}
		
		match headers.get("file-name") {
			Some ( val ) => { file_name = val; },
			_ => { file_name = "None"; }
		}
		
		match headers.get("file-created-time") {
			Some ( val ) => { file_created_time = val; },
			_ => { file_created_time = "0"; }
		}
		
		match headers.get("file-modified-time") {
			Some ( val ) => { file_modified_time = val; },
			_ => { file_modified_time = "0"; }
		}
		
		match headers.get("file-metadata") {
			Some ( val ) => { file_metadata = val; },
			_ => { file_metadata = "None"; }
		}
		
		let file_info = ::nfs::FileReadInfo {
			filename: file_name.to_string(),
			filesize: file_size.parse().ok().expect("Wanted a number"),
			filecreatedtime: file_created_time.parse().ok().expect("Wanted a number"),
			filemodifiedtime: file_modified_time.parse().ok().expect("Wanted a number"),
			filemetadata: file_metadata.to_string(),
			filebody: read_file_resp_body,
		};

		return Ok( file_info ); }
		
		} else { return Err(ConnectionError::UnknownError) }
	}  

};	//match end
}	//fn end


