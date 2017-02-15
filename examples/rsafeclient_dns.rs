extern crate rsafe;

use std::io;
use rsafe::*;

fn main() {

	//--------------------------------------------------------------------------------
	//         First we register with Safe Launcher
	//--------------------------------------------------------------------------------
			
	// change this with your own aaplication details		 
	let appdetails = auth::AppDetails {
		name: "safeshare".to_string(),
		version: "0.0.2".to_string(),
		vendor: "nice".to_string(),
		id: "share".to_string(),
		permissions: vec! ["SAFE_DRIVE_ACCESS".to_string()]
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
	        println! ( "{:?}", auth_check );
		
/*

			// --------------------------------------------------------------------------------
			//                         Register a DNS Name - DNS operation
			// --------------------------------------------------------------------------------
							
				// use this to debug
				let registerlongname = false;		
				if registerlongname 
			{
				
			let longName = "testlongname22".to_string();
			
			let dns_register_long_name = dns::register_long_name ( longName, &credentials );			
			println!(" register long name resp = {:?}", &dns_register_long_name );
			
			}
			
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
		
			// populate the struct as per API doc:
			let create_dir_data = nfs::CreateDirData {
			dirPath: "/blog".to_string(),
			isPrivate: true,
			metadata: meta_b64,
			isVersioned: false,
			isPathShared: false
			};
			
			let nfs_create_dir = nfs::create_dir ( create_dir_data, &credentials );
			println! ( "{:?}", nfs_create_dir );
			}
			
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
			filePath: "/blog/testfile.txt".to_string(),
			isPrivate: true,
			metadata: meta_b64,
			isVersioned: false,
			isPathShared: false
			};
			
			let nfs_create_file = nfs::create_file ( create_file_data, &credentials );
			println!(" create file = {:?}", &nfs_create_file );
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
			filePath: "/blog/testfile.txt".to_string(),
			isPathShared: false,
			fileContent : "This is just a sample text!!!".to_string(),
			offset : 0  // seems to be unsupported
			};
			
			let nfs_write_file = nfs::write_file ( write_file_data, &credentials );
			println!(" write file = {:?}", &nfs_write_file );
			
		}
			
			// --------------------------------------------------------------------------------
			//                 Register a DNS name, a service for that name
			//                 and a directory for that service - DNS operation		
			// --------------------------------------------------------------------------------
							
				// use this to debug
				let registerservice = false;		
				if registerservice 
			{
			
			let register_service_data = dns::RegisterServiceData	
			{
			    longName: "testlongname22".to_string(),
			    serviceName: "serv".to_string(),
			    serviceHomeDirPath: "/serv".to_string(),
			    isPathShared: false
			  };
			
			let dns_register_service = dns::register_service ( register_service_data, &credentials );			
			println!(" register service resp = {:?}", &dns_register_service );
			
			}
			
			// --------------------------------------------------------------------------------
			//                 Add a service for an existing DNS name. - DNS operation
			// --------------------------------------------------------------------------------
							
				// use this to debug
				let addservice = false;		
				if addservice 
			{
			
			let add_service_data = dns::AddServiceData	
			{
			    longName: "testlongname22".to_string(),
			    serviceName: "blog".to_string(),
			    serviceHomeDirPath: "/blog".to_string(),
			    isPathShared: false
			  };
			
			let dns_add_service = dns::add_service ( add_service_data, &credentials );			
			println!(" add service resp = {:?}", &dns_add_service );
			
			}
			
			// --------------------------------------------------------------------------------
			//                 List registered DNS names. - DNS operation
			// --------------------------------------------------------------------------------
							
				// use this to debug
				let listnames = false;		
				if listnames 
			{
							
			let list_names = dns::list_names ( &credentials );			
			println!(" list DNS names resp = {:?}", &list_names );
			
			}
			
			// --------------------------------------------------------------------------------
			//                 List registered services for a DNS name - DNS operation
			// --------------------------------------------------------------------------------
							
				// use this to debug
				let listservices = false;		
				if listservices
			{
				
			let longname = "testlongname22".to_string();			
			let list_services = dns::list_services ( &longname, &credentials );			
			println!(" list services resp = {:?}", &list_services );
			
			}
			
			// --------------------------------------------------------------------------------
			//                Get the home directory associated to a service. - DNS operation
			// --------------------------------------------------------------------------------
							
				// use this to debug
				let getservicedir = true;		
				if getservicedir 
			{
				
			let longname = "share".to_string();	
			let service = "safe".to_string();		
			let get_public_dir = dns::get_public_dir ( longname, service, &credentials );			
			println!(" get service dir = {:?}", &get_public_dir );
			
			}
			
			// --------------------------------------------------------------------------------
			//                Get a file from a service. - DNS operation
			// --------------------------------------------------------------------------------
							
				// use this to debug
				let getservicefile = false;		
				if getservicefile
			{
				
					/*#[derive(Debug, RustcEncodable)]*/
			let read_public_file_data = dns::ReadPublicFileData {
				longName : "projectdecorum".to_string(),
				serviceName : "www".to_string(),
				filePath: "style.css".to_string(),				
				offset: 0, //  seems to be unsupported ??
				length: 0 //  seems to be unsupported , use length: 0 to read the entire file
		};
			
			let get_public_file = dns::get_public_file ( read_public_file_data , &credentials );			
			println!(" get service file = {:?}", &get_public_file );
			
			}
		
			// --------------------------------------------------------------------------------
			//                         Delete a service from a DNS name - NFS operation
			// --------------------------------------------------------------------------------
				
			/*
			 * 
			 * 
			 * 
			 *                ISSUE :   THIS ALSO DELETES THE DNS NAME  
			 * 
			 * 				https://maidsafe.atlassian.net/browse/CS-63
			 * 
			 */
				
				
				// use this to debug
				let deleteserv = false;		
				if deleteserv 
			{
				
			let longname = "testlongname22".to_string();
			let servicename = "blog".to_string();
			
			let nfs_delete_serv = dns::delete_service ( longname, servicename, &credentials );			
			println!(" delete resp = {:?}", &nfs_delete_serv );
			
			
		}
		
			// --------------------------------------------------------------------------------
			//                         Delete a DNS name - NFS operation
			// --------------------------------------------------------------------------------
				
				// use this to debug
				let deletedns = false;		
				if deletedns 
			{
				
			let longname = "testlongname22".to_string();
			
			let nfs_delete_dns = dns::delete_long_name ( longname, &credentials );			
			println!(" delete resp = {:?}", &nfs_delete_dns );
			
			
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
			dirPath: "/blog".to_string(),
			isPathShared: false
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


