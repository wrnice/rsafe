use curl::http;
use rustc_serialize::json;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64::FromBase64;
use std;

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
	publicKey: String,
	nonce: String,
	permissions: Vec<String>
}

#[derive(Debug, RustcDecodable)]
struct LauncherResponseData {
	token: String,
	encryptedKey: String,
	publicKey: String,
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
		pub code: u32,
		pub token: String,
		pub symm_key: ::sodiumoxide::crypto::secretbox::Key,
		pub symm_nonce: ::sodiumoxide::crypto::secretbox::Nonce
}

fn get_base64_config() -> ::rustc_serialize::base64::Config {
	::rustc_serialize::base64::Config {
		char_set   : ::rustc_serialize::base64::CharacterSet::Standard,
		newline    : ::rustc_serialize::base64::Newline::LF,
		pad        : true,
		line_length: None,
	}
}

pub fn register ( appdetails : AppDetails ) -> ( SafeRegisterResp ) {
	
	
	let mut safe_register_resp = SafeRegisterResp {
		code : 0,
		token : String::new(),
		symm_key : ::sodiumoxide::crypto::secretbox::gen_key(),
		symm_nonce : ::sodiumoxide::crypto::secretbox::gen_nonce()
	};
		
	let url = "http://localhost:8100/auth".to_string();
	
	// Generate Asymmetric Key-Pair using sodiumoxide.
	let (pub_key, priv_key) = ::sodiumoxide::crypto::box_::gen_keypair();
	// Generate Nonce using sodiumoxide.
	let asym_nonce = ::sodiumoxide::crypto::box_::gen_nonce();

	// Convert to Base64 encoded string so that we can put it into a JSON.
	let pub_key_b64 = pub_key.0.to_base64(get_base64_config());
	let asym_nonce_b64 = asym_nonce.0.to_base64(get_base64_config());

	// Prepare the request elements
	let appli = App {
		name: appdetails.name,
		version: appdetails.version,
		vendor: appdetails.vendor,
		id: appdetails.id,
		};

	let data = Data {
		app: appli,
		publicKey: pub_key_b64,
		nonce: asym_nonce_b64,
		//permissions: vec! ["SAFE_DRIVE_ACCESS".to_string()]
		permissions: appdetails.permissions
		};
		
	// Encode the data to a JSON
	let payload = ::rustc_serialize::json::encode(&data).unwrap();

	//println!("payload = {}", &payload );

	// Send a POST request to the SAFE Launcher using curl
	let resp = http::handle()
		.post( url, &payload )
		.header("Content-Type", "application/json")
		.exec().unwrap();

	// Handle the response recieved from the launcher

	  //println!("code={}; headers={:?}; body={:?}",
	 //resp.get_code(), resp.get_headers(), resp.get_body());
	
	if resp.get_code() == 401 {
	println!("Unauthorized"); 
	} else if resp.get_code() == 400 {
	println!("Fields are missing"); 
	} else

	{

		// In curl, resp.get_body() will return a reference to a slice of unsigned 8 bit intgers &[u8].
		// We need to turn those bytes into a unicode string slice.
		let resp_body = std::str::from_utf8(resp.get_body()).unwrap_or_else(|e| {
		panic!("Failed to parse response ");
		});

		//println!( "Body = {}", &resp_body );



		let launcher_response_data: LauncherResponseData = json::decode(&resp_body).ok().unwrap();

		//Our authorization token
		let ourtoken = launcher_response_data.token;

		println!("App: Auth Response decoded");

		// This is the encrypted symmetric key and nonce Launcher has passed us, duely encrypted
		// with the Asymmetric keys we gave it earlier so that no one can snoop on it. Convert from
		// base64 encoded String.
		let vec_encrypted_symm_key_nonce = launcher_response_data.encryptedKey.from_base64().ok().unwrap();

		// This is Launcher's Public Asymmetric Key - We will use this for decrypting the above.
		let vec_launcher_pub_key = launcher_response_data.publicKey.from_base64().ok().unwrap();
		let mut launcher_pub_key = ::sodiumoxide::crypto::box_::PublicKey([0; ::sodiumoxide::crypto::box_::PUBLICKEYBYTES]);

		assert_eq!(vec_launcher_pub_key.len(), ::sodiumoxide::crypto::box_::PUBLICKEYBYTES);

		for it in vec_launcher_pub_key.iter().enumerate() {
			launcher_pub_key.0[it.0] = *it.1;
		}

		// Finally decrypt using our Private Key, Nonce and Launcher's passed Public Key to get the
		// secret key - this is a combination of secret nonce and symmetric key.
		let vec_decrypted_symm_key_nonce = ::sodiumoxide::crypto::box_::open(&vec_encrypted_symm_key_nonce,
																		   &asym_nonce,
																		   &launcher_pub_key,
																		   &priv_key).ok().unwrap();

		assert_eq!(vec_decrypted_symm_key_nonce.len(), ::sodiumoxide::crypto::secretbox::NONCEBYTES + ::sodiumoxide::crypto::secretbox::KEYBYTES);

		let mut symm_key = ::sodiumoxide::crypto::secretbox::Key([0; ::sodiumoxide::crypto::secretbox::KEYBYTES]);
		let mut symm_nonce = ::sodiumoxide::crypto::secretbox::Nonce([0; ::sodiumoxide::crypto::secretbox::NONCEBYTES]);

		// Separate it into Secret Nonce and Symmetric Key - the secret key. Hence forth we will
		// encrypt all data we send to Launcher using these and decrypt all data from Launcher using
		// these.
		for it in vec_decrypted_symm_key_nonce.iter().take(::sodiumoxide::crypto::secretbox::KEYBYTES).enumerate() {
			symm_key.0[it.0] = *it.1;
		}
		for it in vec_decrypted_symm_key_nonce.iter().skip(::sodiumoxide::crypto::secretbox::KEYBYTES).enumerate() {
			symm_nonce.0[it.0] = *it.1;
		}
		
		
		//update struct
		
		
			safe_register_resp.code = resp.get_code();
			safe_register_resp.token = ourtoken;
			safe_register_resp.symm_key = symm_key;
			safe_register_resp.symm_nonce = symm_nonce;
		
		
	
	}	
	
	
		return safe_register_resp;
	
}

pub fn check ( safe_register_resp : &SafeRegisterResp ) -> ( u32 ) {
	
	let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;
	
	let url = "http://localhost:8100/auth".to_string();

	let resp_token = http::handle()
	.get( url )
	.header("Authorization", &bearertoken )
	.exec().unwrap();

	// Handle the response recieved from the launcher		
/*	
	if resp_token.get_code() == 200 {
		print!("200 OK ...Token is valid");
	} else if resp_token.get_code() == 400 {
		println!("401 Bad Request"); 
	} else if resp_token.get_code() == 401 {
		println!("401 Unauthorized"); 
	}
*/
	return resp_token.get_code();
}

pub fn unregister ( safe_register_resp : &SafeRegisterResp ) -> ( u32 ) {

		let bearertoken = "Bearer ".to_string()+&safe_register_resp.token ;
		
		let url2 = "http://localhost:8100/auth".to_string();

		let resp_token = http::handle()
		.delete( url2 )
		.header("Authorization", &bearertoken )
		.exec().unwrap();

		// Handle the response recieved from the launcher		
/*	
		if resp_token.get_code() == 200 {
			print!("200 OK ...Token was cleared");
		} else if resp_token.get_code() == 400 {
			println!("401 Bad Request"); 
		} else if resp_token.get_code() == 401 {
			println!("401 Unauthorized"); 
		}
*/
		return resp_token.get_code();
}



