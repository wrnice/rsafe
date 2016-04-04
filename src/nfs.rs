use rustc_serialize::json;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64::FromBase64;
use std;

use std::collections::HashMap;

#[derive(Debug, RustcEncodable)]
pub struct CreateDirData {
	pub dirPath: String,
	pub isPrivate: bool,
	pub metadata: String,
	pub isVersioned: bool,
	pub isPathShared: bool,
}

#[derive(Debug, RustcEncodable)]
pub struct ReadDirData {
	pub dirPath: String,
	pub isPathShared: bool,
}

#[derive(Debug, RustcDecodable)]
struct GetDirResponse {
	id: String,
	data: GetDirResponseData,
}

#[derive(Debug, RustcDecodable)]
pub struct GetDirResponseData {
	pub info: DirInfo,
	pub files: Vec<FileInfo>,
	pub subDirectories: Vec<DirInfo>,
}

#[derive(Debug, RustcDecodable)]
pub struct DirInfo {
	pub name: String,
	pub createdOn: i64,
	pub modifiedOn: i64,
	pub isPrivate: bool,
	pub isVersioned: bool,
	pub metadata: String,
}

#[derive(Debug, RustcDecodable)]
pub struct FileInfo {
	pub name: String,
	pub createdOn: i64,
	pub modifiedOn: i64,
	pub metadata: String,

}

#[derive(Debug, RustcEncodable)]
pub struct CreateFileData {
	pub filePath: String,
	pub isPrivate: bool,
	pub metadata: String,	
	pub isVersioned: bool,
	pub isPathShared: bool,
}

#[derive(Debug, RustcEncodable)]
pub struct WriteFileData {
	pub filePath: String,
	pub isPathShared: bool,
	pub fileContent: String,
}

#[derive(Debug, RustcEncodable)]
pub struct ReadFileData {
	pub filePath: String,
	pub isPathShared: bool,
}


fn get_base64_config() -> ::rustc_serialize::base64::Config {
	::rustc_serialize::base64::Config {
		char_set   : ::rustc_serialize::base64::CharacterSet::Standard,
		newline    : ::rustc_serialize::base64::Newline::LF,
		pad        : true,
		line_length: None,
	}
}

/*
#[derive(Debug, RustcDecodable)]
pub struct GetFileResponseHeaders {
	pub filename: String,
	pub filesize: i64,
	pub filecreatedtime: i64,
	pub filemodifiedtime: i64,
	pub filemetadata: String,
}
*/

#[derive(Debug)]
pub enum ConnectionError { UnableToConnect , Unauthorized , FieldsAreMissing, BadRequest, UnknownError, InternalServerError, NotFound }

pub fn create_dir ( create_dir_data : CreateDirData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< u16 , ConnectionError > {
	
		let token = &safe_register_resp.token ;
		let symm_key = &safe_register_resp.symm_key;
		let symm_nonce = &safe_register_resp.symm_nonce;
		
		let bearertoken = "Bearer ".to_string()+&token ;
		
		println!("App: Begin creating directory...");
			
		// Encode the request as a JSON.
		let create_dir_json_str = ::rustc_serialize::json::encode(&create_dir_data).unwrap_or_else(|a| panic!("{:?}", a));
		println!("App: CreateDir encoded");

		// Get raw bytes to be encrypted.
		let create_dir_bytes = create_dir_json_str.into_bytes();

		// Encrypt the raw bytes using the Secret Key (Nonce and Symmetric Key).
		let create_dir_encrypted_bytes = ::sodiumoxide::crypto::secretbox::seal(&create_dir_bytes,
																				 &symm_nonce,
																				 &symm_key);

		let create_dir_json_encrypted_b64 = create_dir_encrypted_bytes.to_base64(get_base64_config());
		
		//println!( "encr = {}", &create_dir_json_encrypted_b64 );
	
		let url_nfs = "http://localhost:8100/nfs/directory";
		
		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Content-Type".to_string(), "application/json".to_string());
		headers.insert("Connection".to_string(), "close".to_string());
	
		println!("sending request");
		//Send a request to launcher using the "request" extern crate	
		let res = ::request::post(&url_nfs, &mut headers, &create_dir_json_encrypted_b64.into_bytes() );
		
		println!("request sent");
				
		//Error handling 
		match res {		
			// couldn't connect
			Err(e) => { println!("{}", e); return Err(ConnectionError::UnableToConnect) },
			Ok(res) =>     
		{
			
			println!("code = {:?} " , res.status_code );
			
			// Handle the response recieved from the launcher
			if res.status_code == 401 {
			println!("401 Unauthorized"); return Err(ConnectionError::Unauthorized)
			} else if res.status_code == 400 {
			println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 202 {
			println!("202 Accepted"); { return Ok(res.status_code) }
			} else { return Err(ConnectionError::UnknownError) }
		}
};
}

pub fn read_dir ( read_dir_data : ReadDirData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< GetDirResponseData, ConnectionError > {

		println!("App: Begin reading directory...");	
		
		
		
		let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;	
		
		// path Parameters
		let requested_dir = read_dir_data.dirPath ;
		let dir_path = ::url::percent_encoding::utf8_percent_encode ( &requested_dir, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
		let is_path_shared = read_dir_data.isPathShared;
		
		//println!("dirPath = {}",&dir_path);
		
		// URL to send our 'ls' request to
		
		let url_nfs = "http://localhost:8100/nfs/directory".to_string();
		let url_nfs_ls = url_nfs + "/" + &dir_path + "/" + &is_path_shared.to_string();			
		//println!("url_nfs_ls = {}",&url_nfs_ls);
		
		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Connection".to_string(), "close".to_string());
		
		println!("sending request");
		//Send a request to launcher using the "request" extern crate	
		let res = ::request::get(&url_nfs_ls, &mut headers );
		
		println!("request sent");
				
		//Error handling 
		match res {		
			// couldn't connect
			Err(e) => { println!("{}", e); return Err(ConnectionError::UnableToConnect) },
			Ok(res) =>     
		{
			
		//println!("code = {:?} " , res.status_code );
			
		// Handle the response recieved from the launcher
		if res.status_code == 401 {
		println!("401 Unauthorized"); return Err(ConnectionError::Unauthorized)
		} else if res.status_code == 400 {
		println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
		} else if res.status_code == 500 {
		println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
		} else if res.status_code == 200 {
		println!("200 Ok"); { 
	
		let resp_ls_dir_enc_b64 = res.body;
		
		//println!( "enc_b64 = {}", &resp_ls_dir_enc_b64 );

		let resp_ls_dir_enc = resp_ls_dir_enc_b64.from_base64().ok().unwrap();
		
		//println!( "enc = {:?}" , &resp_ls_dir_enc );
		
		// Decrypt the raw bytes using the Secret Key (Nonce and Symmetric Key).
		let decrypted_response = ::sodiumoxide::crypto::secretbox::open(&resp_ls_dir_enc,
																	&safe_register_resp.symm_nonce,
																	&safe_register_resp.symm_key).ok().unwrap();
																	
		//println!( "decr = {:?}" , &decrypted_response );
																  
		// Get it into a valid UTF-8 String - this will be the JSON response.
		let decrypted_response_json_str = String::from_utf8(decrypted_response).ok().unwrap();
		
		//println!("App: GetDir Response JSON: {:?}", decrypted_response_json_str);
		
		// Decode the JSON into expected response structure - in this case a directory response as
		// stated in the RFC.
		let get_dir_response: GetDirResponseData = ::rustc_serialize::json::decode(&decrypted_response_json_str)
																 .unwrap_or_else(|e| panic!("{:?}", e));
		println!("App: GetDir Response decoded.");	
			
		return Ok(get_dir_response) }
		
		} else { return Err(ConnectionError::UnknownError) }
	}  

};	//match end
}	//fn end

pub fn delete_dir ( delete_dir_data : ReadDirData, safe_register_resp : &super::auth::SafeRegisterResp  ) -> Result< u16 , ConnectionError > {
	
		println!("App: Begin deleting directory...");	
					
		let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;	
		
		// path Parameters
		let requested_dir = delete_dir_data.dirPath ;
		let dir_path = ::url::percent_encoding::utf8_percent_encode ( &requested_dir, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
		let is_path_shared = delete_dir_data.isPathShared;
		
		println!("dirPath = {}",&dir_path);
		
		// URL to send our 'ls' request to
		let url_nfs = "http://localhost:8100/nfs/directory".to_string();
		let url_nfs_del = url_nfs + "/" + &dir_path + "/" + &is_path_shared.to_string();
		//println!("url_nfs_ls = {}",&url_nfs_del);
		
		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Connection".to_string(), "close".to_string());
		
		println!("sending request");
		//Send a request to launcher using the "request" extern crate	
		let res = ::request::delete(&url_nfs_del, &mut headers );
		
		println!("request sent");
					
		//Error handling 
		match res {		
		//request couldn't connect
			Err(e) => { println!("{}", e); return Err(ConnectionError::UnableToConnect) },
			Ok(res) =>     
		{
			
			//println!("code = {:?} " , res.status_code );
			
			// Handle the response recieved from the launcher
			if res.status_code == 401 {
			println!("401 Unauthorized"); return Err(ConnectionError::Unauthorized)
			} else if res.status_code == 400 {
			println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 202 {
			println!("202 Accepted Directory was deleted");		{ return Ok(res.status_code) }
			} else { return Err(ConnectionError::UnknownError) }
		}
};
	
} // fn end

pub fn create_file( create_file_data : CreateFileData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< u16 , ConnectionError > {
	
		let token = &safe_register_resp.token ;
		let symm_key = &safe_register_resp.symm_key;
		let symm_nonce = &safe_register_resp.symm_nonce;
		
		let bearertoken = "Bearer ".to_string()+&token ;
		
		println!("App: Begin creating file...");
		
		// Encode the request as a JSON.
		let create_file_json_str = ::rustc_serialize::json::encode(&create_file_data).unwrap_or_else(|a| panic!("{:?}", a));
		println!("App: CreateFile encoded");

		// Get raw bytes to be encrypted.
		let create_file_bytes = create_file_json_str.into_bytes();

		// Encrypt the raw bytes using the Secret Key (Nonce and Symmetric Key).
		let create_file_encrypted_bytes = ::sodiumoxide::crypto::secretbox::seal(&create_file_bytes,
																				 &symm_nonce,
																				 &symm_key);

		let create_file_json_encrypted_b64 = create_file_encrypted_bytes.to_base64(get_base64_config());
		
		//println!( "encr = {}", &create_file_json_encrypted_b64 );
		
		let url_nfs_file = "http://localhost:8100/nfs/file".to_string();
		
		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Content-Type".to_string(), "application/json".to_string());
		headers.insert("Connection".to_string(), "close".to_string());
	
		println!("sending request");
		//Send a request to launcher using the "request" extern crate	
		let res = ::request::post(&url_nfs_file, &mut headers, &create_file_json_encrypted_b64.into_bytes() );
		
		println!("request sent");
		
		//Error handling 
		match res {		
			// couldn't connect
			Err(e) => { println!("{}", e); return Err(ConnectionError::UnableToConnect) },
			Ok(res) =>     
		{
			
			println!("code = {:?} " , res.status_code );
			
			// Handle the response recieved from the launcher
			if res.status_code == 401 {
			println!("401 Unauthorized"); return Err(ConnectionError::Unauthorized)
			} else if res.status_code == 400 {
			println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
			} else if res.status_code == 404 {
			println!("404 Not Found"); return Err(ConnectionError::NotFound)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 202 {
			println!("202 Accepted"); { return Ok(res.status_code) }
			} else { return Err(ConnectionError::UnknownError) }
		}
};
}

pub fn write_file ( write_file_data : WriteFileData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< u16 , ConnectionError > {
	
		let token = &safe_register_resp.token ;
		let symm_key = &safe_register_resp.symm_key;
		let symm_nonce = &safe_register_resp.symm_nonce;
		
		let bearertoken = "Bearer ".to_string()+&token ;
		
		let fileContent = write_file_data.fileContent;
		
		println!("App: Begin writing to file...");
			
		// Encode the request as a JSON.
		let write_file_json_str = ::rustc_serialize::json::encode(&fileContent).unwrap_or_else(|a| panic!("{:?}", a));
		println!("App: WriteFile encoded");

		// Get raw bytes to be encrypted.
		let write_file_bytes = write_file_json_str.into_bytes();

		// Encrypt the raw bytes using the Secret Key (Nonce and Symmetric Key).
		let write_file_encrypted_bytes = ::sodiumoxide::crypto::secretbox::seal(&write_file_bytes,
																				 &symm_nonce,
																				 &symm_key);

		let write_file_json_encrypted_b64 = write_file_encrypted_bytes.to_base64(get_base64_config());
		
		//println!( "encr = {}", &create_dir_json_encrypted_b64 );
		
		// path Parameters
		let requested_file = write_file_data.filePath ;
		let file_path = ::url::percent_encoding::utf8_percent_encode ( &requested_file, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
		let is_path_shared = write_file_data.isPathShared;
		
		//println!("dirPath = {}",&dir_path);
		
		// URL to send our 'ls' request to
		
		let url_nfs = "http://localhost:8100/nfs/file".to_string();
		let url_nfs_write = url_nfs + "/" + &file_path + "/" + &is_path_shared.to_string();
		//println!("url_nfs_ls = {}",&url_nfs_write);
	
		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Content-Type".to_string(), "application/json".to_string());
		headers.insert("Connection".to_string(), "close".to_string());
	
		println!("sending request");
		//Send a request to launcher using the "request" extern crate	
		let res = ::request::put(&url_nfs_write, &mut headers, &write_file_json_encrypted_b64.into_bytes() );
		
		println!("request sent");

		//Error handling 
		match res {		
			// couldn't connect
			Err(e) => { println!("{}", e); return Err(ConnectionError::UnableToConnect) },
			Ok(res) =>     
		{
			
			println!("code = {:?} " , res.status_code );
			
			// Handle the response recieved from the launcher
			if res.status_code == 401 {
			println!("401 Unauthorized"); return Err(ConnectionError::Unauthorized)
			} else if res.status_code == 400 {
			println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
			} else if res.status_code == 404 {
			println!("404 Not Found"); return Err(ConnectionError::NotFound)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 202 {
			println!("202 Accepted"); { return Ok(res.status_code) }
			} else { return Err(ConnectionError::UnknownError) }
		}
};
}

pub fn read_file ( read_file_data : ReadFileData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result<  String , ConnectionError > {

		println!("App: Begin reading file...");			
			
		let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;	
		
		// path Parameters
		let requested_file = read_file_data.filePath ;
		let file_path = ::url::percent_encoding::utf8_percent_encode ( &requested_file, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
		let is_path_shared = read_file_data.isPathShared;
		
		/*
		 *   TODO
		 * 
		 *   query params
		 * 
		 *   --> offset
		 *   --> length
		 * 
		 * 
		 */
		
		// URL to send our 'ls' request to
		
		let url_nfs = "http://localhost:8100/nfs/file".to_string();
		let url_nfs_read = url_nfs + "/" + &file_path + "/" + &is_path_shared.to_string();
		//println!("url_nfs_read = {}",&url_nfs_ls);
	
		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Content-Type".to_string(), "application/json".to_string());
		headers.insert("Connection".to_string(), "close".to_string());
	
		println!("sending request");
		//Send a request to launcher using the "request" extern crate	
		let res = ::request::get( &url_nfs_read, &mut headers );
		
		println!("request sent");
	
		//Error handling 
		match res {		
			// couldn't connect
			Err(e) => { println!("{}", e); return Err(ConnectionError::UnableToConnect) },
			Ok(res) =>     
		{
			
		//println!("code = {:?} " , res.status_code );
			
		// Handle the response recieved from the launcher
		if res.status_code == 401 {
		println!("401 Unauthorized"); return Err(ConnectionError::Unauthorized)
		} else if res.status_code == 400 {
		println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
		} else if res.status_code == 500 {
		println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
		} else if res.status_code == 200 {
		println!("200 Ok"); { 	

		let resp_read_file_enc_b64 = res.body;
		
		//println!( "enc_b64 = {}", &resp_ls_dir_enc_b64 );

		let resp_read_file_enc = resp_read_file_enc_b64.from_base64().ok().unwrap();
		
		//println!( "enc = {:?}" , &resp_ls_dfile_enc );
		
		// Decrypt the raw bytes using the Secret Key (Nonce and Symmetric Key).
		let decrypted_response = ::sodiumoxide::crypto::secretbox::open(&resp_read_file_enc,
																	&safe_register_resp.symm_nonce,
																	&safe_register_resp.symm_key).ok().unwrap();
																	
		//println!( "decr = {:?}" , &decrypted_response );
																  
		// Get it into a valid UTF-8 String - this will be the JSON response.
		let decrypted_response_json_str = String::from_utf8(decrypted_response).ok().unwrap();
		
		//println!("App: GetFile Response JSON: {:?}", decrypted_response_json_str);
		
		// Decode the JSON into expected response structure - in this case a directory response as
		// stated in the RFC.
		let read_file_resp_body = ::rustc_serialize::json::decode(&decrypted_response_json_str)
																 .unwrap_or_else(|e| panic!("{:?}", e));
		println!("App: GetFile Response decoded.");
		
		/*
		 * 
		 *   TODO
		 * 
		 * 
		//get headers
		let resp_read_file_headers = resp_read_file.get_headers();
		
		println!( "get file headers = {:?}", resp_read_file_headers);
		
		let get_file_response_headers : GetFileResponseHeaders = rustc_serialize::json::decode(&resp_read_file_headers)
																 .unwrap_or_else(|e| panic!("{:?}", e));
		*/
		
		return Ok(read_file_resp_body); }
	
		} else { return Err(ConnectionError::UnknownError) } // if end		
	} 
};//match end
} //fn end

pub fn delete_file ( delete_file_data : ReadFileData, safe_register_resp : &super::auth::SafeRegisterResp  ) -> Result< u16 , ConnectionError > {
	
		println!("App: Begin deleting file...");			
		
		let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;	
		
		// path Parameters
		let requested_file = delete_file_data.filePath ;
		let file_path = ::url::percent_encoding::utf8_percent_encode ( &requested_file, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
		let is_path_shared = delete_file_data.isPathShared;
		
		println!("filePath = {}",&file_path);
		
		// URL to send our 'ls' request to
		
		let url_nfs = "http://localhost:8100/nfs/file".to_string();
		let url_nfs_del = url_nfs + "/" + &file_path + "/" + &is_path_shared.to_string();
		//println!("url_nfs_ls = {}",&url_nfs_del);
	
		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Connection".to_string(), "close".to_string());
		
		println!("sending request");
		//Send a request to launcher using the "request" extern crate	
		let res = ::request::delete(&url_nfs_del, &mut headers );
	
		println!("request sent");
				
		//Error handling 
		match res {		
			// couldn't connect
			Err(e) => { println!("{}", e); return Err(ConnectionError::UnableToConnect) },
			Ok(res) =>     
		{
			
			println!("code = {:?} " , res.status_code );
			
			// Handle the response recieved from the launcher
			if res.status_code == 401 {
			println!("401 Unauthorized"); return Err(ConnectionError::Unauthorized)
			} else if res.status_code == 400 {
			println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
			} else if res.status_code == 404 {
			println!("404 Not Found"); return Err(ConnectionError::NotFound)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 202 {
			println!("202 Accepted"); { return Ok(res.status_code) }
			} else { return Err(ConnectionError::UnknownError) }
		}
};
}

