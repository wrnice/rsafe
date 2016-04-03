extern crate rsafe;

use std::io;
use rsafe::*;

fn main() {

	// --------------------------------------------------------------------------------
	//                               Authentication
	// --------------------------------------------------------------------------------
		 
	let appdetails = auth::AppDetails {
		name: "rustsafe".to_string(),
		version: "0.1.0".to_string(),
		vendor: "nice".to_string(),
		id: "myID".to_string(),
		permissions: vec! []
	};	

	// register our app with the safe launcher
	let safe_register_resp = auth::register( appdetails );

	// --------------------------------------------------------------------------------
	//                         Test Token
	// --------------------------------------------------------------------------------
	print!("App: Testing Token... ");
	
	let auth_check = auth::check ( &safe_register_resp );
	
	println! ( "{}", auth_check );

	// --------------------------------------------------------------------------------
	//                         Create a Directory - NFS operation
	// --------------------------------------------------------------------------------
	
	// Create base64 string for metadata
	//let meta = "ABCD";
	//let meta_b64 = encode(&meta).unwrap();
	//
	//we leave metadata empty for now
	let meta_b64 = String::new();

	// Fill in the details as in the RFC.
	let create_dir_data = nfs::CreateDirData {
	dirPath: "/dirtest".to_string(),
	isPrivate: true,
	metadata: meta_b64,
	isVersioned: false,
	isPathShared: false
	};
	
		// just so we don't mess during debugging
		let createdir = false;		
		if createdir 
	{
	
	let nfs_create_dir = nfs::create_dir ( create_dir_data, &safe_register_resp );
	
	}	

	// --------------------------------------------------------------------------------
	//                         Read a Directory - NFS operation
	// --------------------------------------------------------------------------------
		
				// just so we don't mess during debugging
		let readdir = true;		
		if readdir 
	{
		
	// Fill in the details 
	let read_dir_data = nfs::ReadDirData {
	dirPath: "/".to_string(),
	isPathShared: false
	};
	
	let nfs_read_dir = nfs::read_dir ( read_dir_data, &safe_register_resp );

	println!(" ls resp = {:?}", &nfs_read_dir );
	
}
	
	// --------------------------------------------------------------------------------
	//                         Delete a Directory - NFS operation
	// --------------------------------------------------------------------------------
		

	
		// just so we don't mess during debugging
		let deletedir = false;		
		if deletedir 
	{
		
	// Fill in the details 
	let delete_dir_data = nfs::ReadDirData {
	dirPath: "/dirtestpub".to_string(),
	isPathShared: false
	};
	
	let nfs_delete_dir = nfs::delete_dir ( delete_dir_data, &safe_register_resp );
	
	println!(" delete resp = {:?}", &nfs_delete_dir );
	
	}	

	// --------------------------------------------------------------------------------
	//                         Create a File - NFS operation
	// --------------------------------------------------------------------------------
	
	// Create base64 string for metadata
	//let meta = "ABCD";
	//let meta_b64 = encode(&meta).unwrap();
	//
	//we leave metadata empty for now
	let meta_b64 = String::new();

	// Fill in the details as in the RFC.
	let create_file_data = nfs::CreateFileData {
	filePath: "/dirtest/testfile.txt".to_string(),
	isPrivate: true,
	metadata: meta_b64,
	isVersioned: false,
	isPathShared: false
	};
	
		// just so we don't mess during debugging
		let createfile = false;		
		if createfile
	{
	
	let nfs_create_file = nfs::create_file ( create_file_data, &safe_register_resp );
	
	}
	
	// --------------------------------------------------------------------------------
	//                         Write to a File - NFS operation
	// --------------------------------------------------------------------------------	
	
			// just so we don't mess during debugging
		let writefile = false;		
		if writefile
	{
	
		// Fill in the details 
	let write_file_data = nfs::WriteFileData {
	filePath: "/dirtest/testfile.txt".to_string(),
	isPathShared: false,
	fileContent : "This is just a sample text!!!".to_string()
	};
	
	let nfs_write_file = nfs::write_file ( write_file_data, &safe_register_resp );
}

	// --------------------------------------------------------------------------------
	//                         Read a File - NFS operation
	// --------------------------------------------------------------------------------
		
		// just so we don't mess during debugging
		let readfile = false;		
		if readfile
	{
		
	// Fill in the details 
	let read_file_data = nfs::ReadFileData {
	filePath: "/dirtest/testfile.txt".to_string(),
	isPathShared: false,
	};
	
	let nfs_read_file = nfs::read_file ( read_file_data, &safe_register_resp );

	println!(" ls resp = {:?}", &nfs_read_file );

}
	
	// --------------------------------------------------------------------------------
	//                         Delete a File - NFS operation
	// --------------------------------------------------------------------------------
		

	
		// just so we don't mess during debugging
		let deletefile = false;		
		if deletefile 
	{
		
	// Fill in the details 
	let delete_file_data = nfs::ReadFileData {
	filePath: "/dirtest/testfile.txt".to_string(),
	isPathShared: false
	};
	
	let nfs_delete_file = nfs::delete_file ( delete_file_data, &safe_register_resp );
	
	println!(" delete resp = {:?}", &nfs_delete_file );
	
	}
	
	// --------------------------------------------------------------------------------	
	//                         Do Something
	// --------------------------------------------------------------------------------
	
	println! ("Hellooo!");
		
	// hit a key to quit
	let mut enter = String::new();
	println! ("Press enter to quit");
	io::stdin().read_line(&mut enter)
	.expect("Failed to read line");
	
	// --------------------------------------------------------------------------------
	//                         Clear Token
	// --------------------------------------------------------------------------------
	print!("App: Releasing Token... ");
	
	if auth::unregister ( &safe_register_resp ) == 200 {
		println!( "Token released, bye" );
	}
		return

}


