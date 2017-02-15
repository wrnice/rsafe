use rustc_serialize::json;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64::FromBase64;
use std;

use std::collections::HashMap;

use utilities::*;

#[derive(Debug, RustcEncodable)]
pub struct CreateDirData {
	pub directoryPath: String,
	pub isPrivate: bool,
	pub metadata: String,
	pub rootPath: String,
}

#[derive(Debug, RustcEncodable)]
pub struct CreateDirBody {
	pub isPrivate: bool,
	pub metadata: String,
}

#[derive(Debug, RustcEncodable)]
pub struct ReadDirData {
	pub directoryPath: String,
	pub rootPath: String,
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
	pub isPrivate: bool,
	pub createdOn: String,
	pub modifiedOn: String,
	pub metadata: String,
}

/*
info.name 	The name of the directory.
info.isPrivate 	Whether the directory is private or public.
info.createdOn 	Created timestamp (ISO 8601).
info.modifiedOn 	Last modified timestamp (ISO 8601).
info.metadata 	Metadata associated with the directory (encoded as a base64 string).
files 	List of metadata related to the files in the directory.
subDirectories 	List of metadata related to the subdirectories.
*/

#[derive(Debug, RustcDecodable)]
pub struct FileInfo {
	pub name: String,
	pub createdOn: String,
	pub modifiedOn: String,
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
pub struct MoveFileData {
	pub srcPath: String,
	pub destPath: String,
	pub retainSource: bool,
	pub isSrcPathShared: bool,
	pub isDestPathShared: bool,
}

#[derive(Debug, RustcEncodable)]
pub struct MoveDirData {
	pub srcPath: String,
	pub destPath: String,
	pub retainSource: bool,
	pub isSrcPathShared: bool,
	pub isDestPathShared: bool,
}

#[derive(Debug, RustcEncodable)]
pub struct WriteFileData {
	pub filePath: String,
	pub isPathShared: bool,
	pub fileContent: String,
	pub offset: i64,
}

#[derive(Debug, RustcEncodable)]
pub struct ReadFileData {
	pub filePath: String,
	pub isPathShared: bool,
	pub offset: i64,
	pub length: i64
}

#[derive(Debug, RustcEncodable)]
pub struct DeleteFileData {
	pub filePath: String,
	pub isPathShared: bool,
}

//fn get_base64_config() -> ::rustc_serialize::base64::Config {
	//::rustc_serialize::base64::Config {
		//char_set   : ::rustc_serialize::base64::CharacterSet::Standard,
		//newline    : ::rustc_serialize::base64::Newline::LF,
		//pad        : true,
		//line_length: None,
	//}
//}

#[derive(Debug, RustcDecodable)]
pub struct FileReadInfo {
	pub filename: String,
	pub filesize: i64,
	pub filecreatedtime: i64,
	pub filemodifiedtime: i64,
	pub filemetadata: String,
	pub filebody: String,
}


#[derive(Debug)]
pub enum ConnectionError { UnableToConnect , Unauthorized , FieldsAreMissing, BadRequest, UnknownError, InternalServerError, NotFound }

/* TODO
 * 
 * 	 read and write file with offset
 * 
 * 	 modify file info
 * 
 *   modify dir info
 *  
 *   move dir test    ----- 400
 * 
 */

// create a directory
pub fn create_dir ( create_dir_data : CreateDirData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< u16 , ConnectionError > {
	

		let token = &safe_register_resp.token ;
		
		let bearertoken = "Bearer ".to_string()+&token ;
		
		println!("App: Begin creating directory...");
		
		let dir_path = create_dir_data.directoryPath.to_string() ;
		let root_path = create_dir_data.rootPath.to_string();
		
		let url_nfs_mkd = "http://localhost:8100/nfs/directory/".to_string() +  &root_path + "/" + &dir_path ;
		println!("nfs dir = {}",&url_nfs_mkd);
		
		let create_dir_body =  CreateDirBody { 
			 isPrivate: create_dir_data.isPrivate,
			 metadata: create_dir_data.metadata
			  };
			
		// Encode the request as a JSON.
		let create_dir_json_str = ::rustc_serialize::json::encode(&create_dir_body).unwrap_or_else(|a| panic!("{:?}", a));
		println!("App: CreateDir encoded");		
		println!("jsonstr = {}",&create_dir_json_str);
		
		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Content-Type".to_string(), "application/json".to_string());
		headers.insert("Connection".to_string(), "close".to_string());
	
		//println!("sending request");
		//Send a request to launcher using "request" library	
		let res = ::request::post(&url_nfs_mkd, &mut headers, &create_dir_json_str.into_bytes() );
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
			println!("200 Ok"); { return Ok(res.status_code) }
			} else { return Err(ConnectionError::UnknownError) }
		}
};
}

/*
// move a directory
pub fn move_dir( move_dir_data : MoveDirData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< u16 , ConnectionError > {
	
		let token = &safe_register_resp.token ;
		let symm_key = &safe_register_resp.symm_key;
		let symm_nonce = &safe_register_resp.symm_nonce;
		
		let bearertoken = "Bearer ".to_string()+&token ;
		
		println!("App: Begin Moving Dir...");
		
		// Encode the request as a JSON.
		let move_dir_json_str = ::rustc_serialize::json::encode(&move_dir_data).unwrap_or_else(|a| panic!("{:?}", a));
		//println!("App: MoveDir encoded");

		// Get raw bytes to be encrypted.
		let move_dir_bytes = move_dir_json_str.into_bytes();

		// Encrypt the raw bytes using the Secret Key (Nonce and Symmetric Key).
		let move_dir_encrypted_bytes = ::sodiumoxide::crypto::secretbox::seal(&move_dir_bytes,
																				 &symm_nonce,
																				 &symm_key);

		let move_dir_json_encrypted_b64 = move_dir_encrypted_bytes.to_base64(get_base64_config());
		
		let url_nfs_dir = "http://localhost:8100/nfs/movedir".to_string();
		
		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Content-Type".to_string(), "application/json".to_string());
		headers.insert("Connection".to_string(), "close".to_string());
	
		//println!("sending request");
		//Send a request to launcher using "request" library
		let res = ::request::post(&url_nfs_dir, &mut headers, &move_dir_json_encrypted_b64.into_bytes() );
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
			} else if res.status_code == 404 {
			println!("404 Not Found"); return Err(ConnectionError::NotFound)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 200 {
			println!("200 Ok"); { return Ok(res.status_code) }
			} else { return Err(ConnectionError::UnknownError) }
		}
};
}
*/

// read a directory
pub fn read_dir ( read_dir_data : ReadDirData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< GetDirResponseData, ConnectionError > {

		println!("App: Begin reading directory...");	
				
		let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;	
		
		// path Parameters
		let requested_dir = read_dir_data.directoryPath ;
		let dir_path = requested_dir;
		let root_path = read_dir_data.rootPath;
		
		println!("dirPath = {}",&dir_path);
		
		// URL to send our 'ls' request to
		
		let url_nfs = "http://localhost:8100/nfs/directory".to_string();
		let url_nfs_ls = url_nfs + "/" + &root_path  + "/" + &dir_path ;			
		println!("url_nfs_ls = {}",&url_nfs_ls);
		
		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Connection".to_string(), "close".to_string());
		
		//println!("sending request");
		//Send a request to launcher using "request" library
		let res = ::request::get(&url_nfs_ls, &mut headers );
		println!("read request sent");
				
		//Error handling 
		match res {		
			Err(e) => { println!("{}", e); return Err(ConnectionError::UnableToConnect) }, // couldn't connect
			Ok(res) =>     
		{		
			
			println!("resp status = {:?}",&res.status_code);
				
			// Handle the response recieved from the launcher
			if res.status_code == 401 {
			println!("401 Unauthorized"); return Err(ConnectionError::Unauthorized)
			} else if res.status_code == 400 {
			println!("400 Bad Request"); return Err(ConnectionError::BadRequest)
			} else if res.status_code == 404 {
			println!("404 Not Found"); return Err(ConnectionError::NotFound)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 200 {
			println!("200 Ok"); { 
		
		println!("resp body = {:?}",&res.body);
		
		// Decode the received JSON into expected response structure - in this case a directory response as
		// stated in the RFC.
		let get_dir_response: GetDirResponseData = ::rustc_serialize::json::decode(&res.body)
																 .unwrap_or_else(|e| panic!("{:?}", e));
		//println!("App: GetDir Response decoded.");	
			
		return Ok(get_dir_response) }
		
		} else { return Err(ConnectionError::UnknownError) }
	}  

};	//match end
}	//fn end


// delete a directory
pub fn delete_dir ( delete_dir_data : ReadDirData, safe_register_resp : &super::auth::SafeRegisterResp  ) -> Result< u16 , ConnectionError > {
	
		println!("App: Begin deleting directory...");	
					
		let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;	
		
		// path Parameters
		let requested_dir = delete_dir_data.directoryPath ;
		let dir_path = ::url::percent_encoding::utf8_percent_encode ( &requested_dir, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
		let root_path = delete_dir_data.rootPath;
		
		println!("dirPath = {}",&dir_path);
		
		// URL to send our 'ls' request to
		let url_nfs = "http://localhost:8100/nfs/directory".to_string();
		let url_nfs_del = url_nfs  + "/" + &root_path + "/" + &dir_path;
		println!("url_nfs_del = {}",&url_nfs_del);
		
		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Connection".to_string(), "close".to_string());
		
		//println!("sending request");
		//Send a request to launcher using "request" library
		let res = ::request::delete(&url_nfs_del, &mut headers );
		
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
			println!("200 Ok Directory was deleted");		{ return Ok(res.status_code) }
			} else { return Err(ConnectionError::UnknownError) }
		}
};
	
} // fn end

/*

// create an empty file
pub fn create_file( create_file_data : CreateFileData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< u16 , ConnectionError > {
	
		let token = &safe_register_resp.token ;
		let symm_key = &safe_register_resp.symm_key;
		let symm_nonce = &safe_register_resp.symm_nonce;
		
		let bearertoken = "Bearer ".to_string()+&token ;
		
		println!("App: Begin creating file...");
		
		// Encode the request as a JSON.
		let create_file_json_str = ::rustc_serialize::json::encode(&create_file_data).unwrap_or_else(|a| panic!("{:?}", a));
		//println!("App: CreateFile encoded");

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
	
		//println!("sending request");
		//Send a request to launcher using "request" library
		let res = ::request::post(&url_nfs_file, &mut headers, &create_file_json_encrypted_b64.into_bytes() );
		
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
			} else if res.status_code == 404 {
			println!("404 Not Found"); return Err(ConnectionError::NotFound)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 200 {
			println!("200 Ok"); { return Ok(res.status_code) }
			} else { return Err(ConnectionError::UnknownError) }
		}
};
}

// move a file
pub fn move_file( move_file_data : MoveFileData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< u16 , ConnectionError > {
	
		let token = &safe_register_resp.token ;
		let symm_key = &safe_register_resp.symm_key;
		let symm_nonce = &safe_register_resp.symm_nonce;
		
		let bearertoken = "Bearer ".to_string()+&token ;
		
		println!("App: Begin moving file...");
		
		// Encode the request as a JSON.
		let move_file_json_str = ::rustc_serialize::json::encode(&move_file_data).unwrap_or_else(|a| panic!("{:?}", a));
		//println!("App: MoveFile encoded");

		// Get raw bytes to be encrypted.
		let move_file_bytes = move_file_json_str.into_bytes();

		// Encrypt the raw bytes using the Secret Key (Nonce and Symmetric Key).
		let move_file_encrypted_bytes = ::sodiumoxide::crypto::secretbox::seal(&move_file_bytes,
																				 &symm_nonce,
																				 &symm_key);

		let move_file_json_encrypted_b64 = move_file_encrypted_bytes.to_base64(get_base64_config());
		
		//println!( "encr = {}", &move_file_json_encrypted_b64 );
		
		let url_nfs_file = "http://localhost:8100/nfs/movefile".to_string();
		
		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Content-Type".to_string(), "application/json".to_string());
		headers.insert("Connection".to_string(), "close".to_string());
	
		//println!("sending request");
		//Send a request to launcher using "request" library	
		let res = ::request::post(&url_nfs_file, &mut headers, &move_file_json_encrypted_b64.into_bytes() );
		
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
			} else if res.status_code == 404 {
			println!("404 Not Found"); return Err(ConnectionError::NotFound)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 200 {
			println!("200 Ok"); { return Ok(res.status_code) }
			} else { return Err(ConnectionError::UnknownError) }
		}
};
}

// write to a file
pub fn write_file ( write_file_data : WriteFileData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< u16 , ConnectionError > {
	
		let token = &safe_register_resp.token ;
		let symm_key = &safe_register_resp.symm_key;
		let symm_nonce = &safe_register_resp.symm_nonce;
		
		let bearertoken = "Bearer ".to_string()+&token ;
		
		let fileContent = write_file_data.fileContent;
		
		println!("App: Begin writing to file...");
			
		// Encode the request as a JSON.
		let write_file_json_str = ::rustc_serialize::json::encode(&fileContent).unwrap_or_else(|a| panic!("{:?}", a));
		//println!("App: WriteFile encoded");

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
		let offset = write_file_data.offset ; // seems to be unsupported
		
		//println!("dirPath = {}",&dir_path);
		
		// URL to send our 'ls' request to
		
		let url_nfs = "http://localhost:8100/nfs/file".to_string();
		let url_nfs_write = url_nfs + "/" + &file_path + "/" + &is_path_shared.to_string() + "?offset:=" + &offset.to_string() ;
		//println!("url_nfs_ls = {}",&url_nfs_write);
	
		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Content-Type".to_string(), "application/json".to_string());
		headers.insert("Connection".to_string(), "close".to_string());
	
		//println!("sending request");
		//Send a request to launcher using "request" library	
		let res = ::request::put(&url_nfs_write, &mut headers, &write_file_json_encrypted_b64.into_bytes() );
		
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
			} else if res.status_code == 404 {
			println!("404 Not Found"); return Err(ConnectionError::NotFound)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 200 {
			println!("200 Ok"); { return Ok(res.status_code) }
			} else { return Err(ConnectionError::UnknownError) }
		}
};
}

// read a file
pub fn read_file ( read_file_data : ReadFileData , safe_register_resp : &super::auth::SafeRegisterResp ) -> Result< FileReadInfo , ConnectionError > {

		println!("App: Begin reading file...");		
		
		/*
		 * 
		 * 
		 *    TODO   PANIC on inexistant file
		 * 
		 * 
		 */	
			
		let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;	
		
		// path Parameters
		let requested_file = read_file_data.filePath ;
		let file_path = ::url::percent_encoding::utf8_percent_encode ( &requested_file, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
		let is_path_shared = read_file_data.isPathShared;
		
		let offset = read_file_data.offset ; //  seems to be unsupported ////// should be encoded b4 send request
		let length = read_file_data.length ; //  seems to be unsupported ///// should be encoded b4 send request
		
		// URL to send our 'ls' request to
		// http://localhost:8100/0.4/nfs/file/:filePath/:isPathShared?offset=:offset&length=:length
		
		let url_nfs = "http://localhost:8100/nfs/file".to_string();
		
		let url_nfs1 = url_nfs + "/" + &file_path + "/" + &is_path_shared.to_string() ; 
		
		let mut url_nfs_read = url_nfs1.clone() ;
		
		// append encrypted b64 length and offset if needed		
		if  length > 0 && offset > 0 {	
			let queryparams = "offset=".to_string() + &offset.to_string() + "&length=" + &length.to_string();
			let queryparamsencoded = launcher_string_encode ( queryparams, &safe_register_resp );
			url_nfs_read = url_nfs1 +  "?" + &queryparamsencoded; }
		else if  length == 0 && offset > 0  {
			let queryparams = "offset=".to_string() + &offset.to_string();
			let queryparamsencoded = launcher_string_encode ( queryparams, &safe_register_resp );
			url_nfs_read = url_nfs1 +  "?" + &queryparamsencoded; }
		else if  length > 0 && offset == 0  {
			let queryparams = "length=".to_string() + &length.to_string();
			let queryparamsencoded = launcher_string_encode ( queryparams, &safe_register_resp );
			url_nfs_read = url_nfs1 +  "?" + &queryparamsencoded; }			

		
		println!("url_nfs_read = {}",&url_nfs_read);
	
		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Content-Type".to_string(), "application/json".to_string());
		headers.insert("Connection".to_string(), "close".to_string());
	
		//println!("sending request");
		//Send a request to launcher using "request" library	
		let res = ::request::get( &url_nfs_read, &mut headers );
		
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
		
		// Decode the JSON into expected response structure
		
		/*
		 * 
		 * 		TODO
		 * 
		 *     panics on empty file  ----  EOF
		 * 
		 */
		
		let read_file_resp_body = ::rustc_serialize::json::decode(&decrypted_response_json_str)
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
		
		let file_info = FileReadInfo {
			filename: file_name.to_string(),
			filesize: file_size.parse().ok().expect("Wanted a number"),
			filecreatedtime: file_created_time.parse().ok().expect("Wanted a number"),
			filemodifiedtime: file_modified_time.parse().ok().expect("Wanted a number"),
			filemetadata: file_metadata.to_string(),
			filebody: read_file_resp_body,
		};

		return Ok( file_info ); }
	
		} else { return Err(ConnectionError::UnknownError) } // if end		
	} 
};//match end
} //fn end

// delete a file
pub fn delete_file ( delete_file_data : DeleteFileData, safe_register_resp : &super::auth::SafeRegisterResp  ) -> Result< u16 , ConnectionError > {
	
		println!("App: Begin deleting file...");			
		
		let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;	
		
		// path Parameters
		let requested_file = delete_file_data.filePath ;
		let file_path = ::url::percent_encoding::utf8_percent_encode ( &requested_file, ::url::percent_encoding::FORM_URLENCODED_ENCODE_SET );
		let is_path_shared = delete_file_data.isPathShared;
		
		//println!("filePath = {}",&file_path);
		
		// URL to send our 'ls' request to
		
		let url_nfs = "http://localhost:8100/nfs/file".to_string();
		let url_nfs_del = url_nfs + "/" + &file_path + "/" + &is_path_shared.to_string();
		//println!("url_nfs_ls = {}",&url_nfs_del);
	
		let mut headers: HashMap<String, String> = HashMap::new();
		headers.insert("Authorization".to_string(), bearertoken );
		headers.insert("Connection".to_string(), "close".to_string());
		
		//println!("sending request");
		//Send a request to launcher using "request" library	
		let res = ::request::delete(&url_nfs_del, &mut headers );
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
			} else if res.status_code == 404 {
			println!("404 Not Found"); return Err(ConnectionError::NotFound)
			} else if res.status_code == 500 {
			println!("500 Internal Server Error"); return Err(ConnectionError::InternalServerError)
			} else if res.status_code == 200 {
			println!("200 Ok"); { return Ok(res.status_code) }
			} else { return Err(ConnectionError::UnknownError) }
		}
};
}
*/
