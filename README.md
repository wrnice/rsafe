# rsafe - a Safe client library written in Rust

**This project is mainly an exercise to learn Rust and the Safe Network internals**

This is in early testing and debugging phase. Some features may not work as expected, or may not be implemented at all.
The Safe API is subject to change often and without notice, and this code may be behind or outdated.
Use at your own risks and without any sort of warranty.

rsafe is a client library written in rust, to be used as an external crate in your rust projects so that they can communicate with the Safe Network Launcher.
It comes with a minimal example client called rsafeclient, for testing and debugging purposes.

## Installation

To install the library and the example client locally :


	cd <path to the directory where you want to install rsafe>
	git clone https://github.com/wrnice/rsafe.git
	cd rsafe
	
To build the library :

	cargo build 
	
To build the example client :

	cargo build --example rsafeclient
	

## Running and tinkering with the example client

	
To run the example client :

	cargo run --example rsafeclient
	
The code in the example client is commented and should hopefully be relatively self explanatory.
The example demonstrates calls to all the functions in the library, and can be easily tweaked for testing, by changing the conditional statements and various file names.

First it registers with the launcher, then tests the returned token.
Then it creates , reads , and deletes directories.
Then it creates, moves, writes into, and deletes a file.
Finally it unregisters from the launcher and exits.	
	
## Usage as an external crate

If you installed the library locally as described above , you can call the library by including the following lines in your cargo.toml :

```rust
[dependencies]
rsafe= { path = "<path_to_local_installation>/rsafe"  }
```	

and in your rust source :

```rust
extern crate rsafe;
use rsafe::*;
```

You can then call the various rsafe functions in your project.

## Usage

TODO - rewrite this with proper documentation

For now, please refer to comments in examples/rsafeclient.rs , hopefully this should get you sorted out...
