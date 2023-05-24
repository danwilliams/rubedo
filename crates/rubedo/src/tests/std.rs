#![allow(non_snake_case)]

//		Tests

//§		PathExt																	
#[cfg(test)]
mod path_ext {
	use super::super::*;
	
	//		normalize															
	#[test]
	fn normalize() {
		let cwd = env::current_dir().unwrap();
		let mut path: PathBuf;
		
		path = PathBuf::from("");
		assert_eq!(path.normalize(), cwd);
		
		path = PathBuf::from(".");
		assert_eq!(path.normalize(), cwd);
		
		path = PathBuf::from("..");
		assert_eq!(path.normalize(), cwd.parent().unwrap());
		
		path = PathBuf::from("./");
		assert_eq!(path.normalize(), cwd);
		
		path = PathBuf::from("./.");
		assert_eq!(path.normalize(), cwd);
		
		path = PathBuf::from("./..");
		assert_eq!(path.normalize(), cwd.parent().unwrap());
		
		path = PathBuf::from("././/.");
		assert_eq!(path.normalize(), cwd);
		
		path = PathBuf::from("/");
		assert_eq!(path.normalize(), PathBuf::from("/"));
		
		path = PathBuf::from("//");
		assert_eq!(path.normalize(), PathBuf::from("/"));
		
		path = PathBuf::from("/.");
		assert_eq!(path.normalize(), PathBuf::from("/"));
		
		path = PathBuf::from("/./");
		assert_eq!(path.normalize(), PathBuf::from("/"));
		
		path = PathBuf::from("/tests/std.rs");
		assert_eq!(path.normalize(), PathBuf::from("/tests/std.rs"));
		
		path = PathBuf::from("/tests//std.rs");
		assert_eq!(path.normalize(), PathBuf::from("/tests/std.rs"));
		
		path = PathBuf::from("/tests/./std.rs");
		assert_eq!(path.normalize(), PathBuf::from("/tests/std.rs"));
		
		path = PathBuf::from("/tests/../std.rs");
		assert_eq!(path.normalize(), PathBuf::from("/std.rs"));
		
		path = PathBuf::from("/tests/../../std.rs");
		assert_eq!(path.normalize(), PathBuf::from("/std.rs"));
		
		path = PathBuf::from("tests/std.rs");
		assert_eq!(path.normalize(), cwd.join("tests/std.rs"));
		
		path = PathBuf::from("tests//std.rs");
		assert_eq!(path.normalize(), cwd.join("tests/std.rs"));
		
		path = PathBuf::from("tests/./std.rs");
		assert_eq!(path.normalize(), cwd.join("tests/std.rs"));
		
		path = PathBuf::from("tests/one/two/three/std.rs");
		assert_eq!(path.normalize(), cwd.join("tests/one/two/three/std.rs"));
		
		path = PathBuf::from("tests/one/two/three/../../../std.rs");
		assert_eq!(path.normalize(), cwd.join("tests/std.rs"));
		
		path = PathBuf::from("tests//one/./two/../three/.//std.rs");
		assert_eq!(path.normalize(), cwd.join("tests/one/three/std.rs"));
		
		path = PathBuf::from("tests/.rs");
		assert_eq!(path.normalize(), cwd.join("tests/.rs"));
		
		path = PathBuf::from("tests/🥳.rs");
		assert_eq!(path.normalize(), cwd.join("tests/🥳.rs"));
		
		let path: &Path;
		path = Path::new("/tests/std.rs");
		assert_eq!(path.normalize(), Path::new("/tests/std.rs"));
		assert_eq!(path.normalize(), PathBuf::from("/tests/std.rs"));
	}
}


