extern crate rsafe;

use std::io;
use rsafe::*;

fn main() {

	//--------------------------------------------------------------------------------
	//         First we register with Safe Launcher
	//--------------------------------------------------------------------------------
			
	// change this with your own aaplication details		 
	let appdetails = auth::AppDetails {
		name: "appname".to_string(),
		version: "0.0.2".to_string(),
		vendor: "vendorname".to_string(),
		id: "myID".to_string(),
		permissions: vec! []
	};	

	// register our app with the safe launcher
	let safe_register_resp = auth::register( appdetails );
	
	//handle errors
	match safe_register_resp {
		Err(e) => { // something went wrong : launcher is not running , user didn't allow application in launcher ...
	        println!("{:?}\nUnable to Connect to Launcher \nMake sure Safe Launcher is running", e); 
	    },
	    Ok(credentials) => {
			// --------------------------------------------------------------------------------
			//                         We are Registered, let's play with Safe
			// --------------------------------------------------------------------------------
	        println!("Succesfully Registered");
	        
        	// --------------------------------------------------------------------------------
			//                         Test Token
			// --------------------------------------------------------------------------------
			print!("App: Testing Token... ");
	        
	        let auth_check = auth::check ( &credentials );
	        println! ( "Check results :{:?}", auth_check );
		
			// --------------------------------------------------------------------------------
			//                         Create a Directory - NFS operation
			// --------------------------------------------------------------------------------
	
				// use this to debug
				let createdir = false;		
				if createdir 
			{
		
			
			// Create base64 string for metadata
			//let meta = "ABCD";
			//let meta_b64 = encode(&meta).unwrap();
			//
			//we leave metadata empty for now
			let meta_b64 = String::new();
		
			
			let create_dir_data = nfs::CreateDirData {
			directoryPath: "testdir".to_string(),
			isPrivate: true,
			metadata: meta_b64,
			//isVersioned: false,
			rootPath: "app".to_string()
			};
			
			let nfs_create_dir = nfs::create_dir ( create_dir_data, &credentials );
			println! ( "{:?}", nfs_create_dir );
			}	
	
			// --------------------------------------------------------------------------------
			//                         Read a Directory - NFS operation
			// --------------------------------------------------------------------------------
				
				// use this to debug
				let readdir = true;		
				if readdir 
			{
				
			// populate the struct as per API doc: 
			let read_dir_data = nfs::ReadDirData {
			directoryPath: "testdir".to_string(),
			rootPath: "app".to_string(),
			};
			
			let nfs_read_dir = nfs::read_dir ( read_dir_data, &credentials );
			println!(" ls resp = {:?}", nfs_read_dir );
			
			}

/*
			
			/*   
			// --------------------------------------------------------------------------------
			//                         Move a Directory - NFS operation
			// --------------------------------------------------------------------------------
				
				// use this to debug
				let movedir = false;		
				if movedir 
			{
				
			// populate the struct as per API doc: 
			let move_dir_data = nfs::MoveDirData {
			srcPath: "/testdir".to_string(),
			destPath: "/destinationdir".to_string(),
			retainSource: false,
			isSrcPathShared: false,
			isDestPathShared: false
			};
			
			//for now, this returns 400, inconditionnally 
			let nfs_move_dir = nfs::move_dir ( move_dir_data, &credentials );
			println!(" ls resp = {:?}", nfs_move_dir );
			
			}
			*/			

			// --------------------------------------------------------------------------------
			//                         Create a File - NFS operation
			// --------------------------------------------------------------------------------
			
				// use this to debug
				let createfile = false;		
				if createfile
			{
			
			// Create base64 string for metadata
			//let meta = "ABCD";
			//let meta_b64 = encode(&meta).unwrap();
			//
			//we leave metadata empty for now
			let meta_b64 = String::new();
		
			// populate the struct as per API doc:
			let create_file_data = nfs::CreateFileData {
			filePath: "/testfile.txt".to_string(),
			isPrivate: true,
			metadata: meta_b64,
			isVersioned: false,
			isPathShared: false
			};
			
			let nfs_create_file = nfs::create_file ( create_file_data, &credentials );
			println!(" create file = {:?}", &nfs_create_file );
			}
			
			
			// --------------------------------------------------------------------------------
			//                         Move a File - NFS operation
			// --------------------------------------------------------------------------------
				
				// use this to debug
				let movefile = false;		
				if movefile 
			{
				
			// populate the struct as per API doc:
			let move_file_data = nfs::MoveFileData {
			srcPath: "/testfile.txt".to_string(),
			destPath: "/testdir".to_string(),
			retainSource: false,
			isSrcPathShared: false,
			isDestPathShared: false
			};
			
			let nfs_move_file = nfs::move_file ( move_file_data, &credentials );
			println!(" ls resp = {:?}", nfs_move_file );
			
			}
			
	
			// --------------------------------------------------------------------------------
			//                         Write to a File - NFS operation
			// --------------------------------------------------------------------------------	
			
				// use this to debug
				let writefile = false;		
				if writefile
			{
			
			// populate the struct as per API doc:
			let write_file_data = nfs::WriteFileData {
			filePath: "/testdir/testfile.txt".to_string(),
			isPathShared: false,
			fileContent : "This is just a sample text!!!".to_string(),
			offset : 0  // seems to be unsupported
			};
			
			let nfs_write_file = nfs::write_file ( write_file_data, &credentials );
			println!(" write file = {:?}", &nfs_write_file );
			
		}
	
			// --------------------------------------------------------------------------------
			//                         Read a File - NFS operation
			// --------------------------------------------------------------------------------
				
				// use this to debug
				let readfile = false;		
				if readfile
			{
				
			// are offset and length really supported ??
				
			// populate the struct as per API doc:
			let read_file_data = nfs::ReadFileData {
			filePath: "/testdir/testfile.txt".to_string(),
			isPathShared: false,
			offset: 0,  //  seems to be unsupported
			length: 0,  //  seems to be unsupported , use length: 0 to read the entire file
			};
			
			let nfs_read_file = nfs::read_file ( read_file_data, &credentials);		
			println!(" ls resp = {:?}", &nfs_read_file.unwrap() );
		
		}
		
			// --------------------------------------------------------------------------------
			//                         Delete a File - NFS operation
			// --------------------------------------------------------------------------------
				
		
			
				// use this to debug
				let deletefile = false;		
				if deletefile 
			{
				
			// populate the struct as per API doc:
			let delete_file_data = nfs::DeleteFileData {
			filePath: "/testdir/testfile.txt".to_string(),
			isPathShared: false
			};
			
			let nfs_delete_file = nfs::delete_file ( delete_file_data, &credentials );			
			println!(" delete resp = {:?}", &nfs_delete_file );
			
			}
			
			// --------------------------------------------------------------------------------
			//                         Delete a Directory - NFS operation
			// --------------------------------------------------------------------------------
				
				// use this to debug
				let deletedir = false;		
				if deletedir 
			{
				
			// populate the struct as per API doc:
			let delete_dir_data = nfs::ReadDirData {
			directoryPath: "/testdir".to_string(),
			rootPath: "app".to_string(),
			};
			
			let nfs_delete_dir = nfs::delete_dir ( delete_dir_data, &credentials );			
			println!(" delete resp = {:?}", &nfs_delete_dir );
			
			
		}

*/
		
			// --------------------------------------------------------------------------------	
			//                         Do Something
			// --------------------------------------------------------------------------------
			
			println! ("Hellooo!");
				
			// hit enter to quit
			let mut enter = String::new();
			println! ("Press enter to quit");
			io::stdin().read_line(&mut enter)
			.expect("Failed to read line");
			
			// --------------------------------------------------------------------------------
			//                         Clear Token
			// --------------------------------------------------------------------------------
			print!("App: Releasing Token... ");
			
			let deauth = auth::unregister ( &credentials );
			
			println! ( "quit {:?}", deauth );
			
			if deauth.unwrap() == 200 {
				println!( "Token released, bye" );
			}
	        
	        
	    } // end of Ok(credentials)

	} // end of match
	

		return

}


