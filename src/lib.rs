use std::fs;
use std::path::Path;

extern crate gcc;

#[warn(dead_code)]
fn main() {
	
	//let lib_include_path = Path::new("/");
	let include_path = Path::new("lua/src/");
	let path = fs::read_dir(include_path).unwrap();
	
    let mut cfg = gcc::Build::new();    

	cfg.flag("-O2").define("LUA_USE_APICHECK", Some("1"));
	//cfg.flag_if_supported();
	//cfg.include(lib_path)
	//.define("FOO","BAR")
	  
    for ifile in path {
		    match ifile {
				Ok(p) => {  let y = p.path().display().to_string();
							if (y.contains(".c")) {
								cfg.file(<String as AsRef<Path>>::as_ref(&y));
						    }
						  },
				Err(e) => {println!("Error in PathBuf");}
			} 
	}
    
    cfg.include(include_path);

      //cfg.target("arm-linux-gnueabihf");
        cfg.compile("liblua.a");
      //.shared_flag(true)
      //.static_flag(true)
      //.compile("liblua.so")
}
