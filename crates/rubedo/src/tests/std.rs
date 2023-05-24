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

	//		restrict															
	#[test]
	fn restrict() {
		let cwd = env::current_dir().unwrap();
		let mut path: PathBuf;
		
		path = PathBuf::from("");
		assert_eq!(path.restrict(None), cwd);
		
		path = PathBuf::from(".");
		assert_eq!(path.restrict(None), cwd);
		
		path = PathBuf::from("..");
		assert_eq!(path.restrict(None), cwd);
		
		path = PathBuf::from("/");
		assert_eq!(path.restrict(None), cwd);
		
		path = PathBuf::from("/tests/std.rs");
		assert_eq!(path.restrict(None), cwd);
		
		path = PathBuf::from("tests/std.rs");
		assert_eq!(path.restrict(None), cwd.join("tests/std.rs"));
		
		path = PathBuf::from("tests/../std.rs");
		assert_eq!(path.restrict(None), cwd.join("std.rs"));
		
		path = PathBuf::from("tests/../../std.rs");
		assert_eq!(path.restrict(None), cwd);
		
		path = PathBuf::from("tests/../../one/two/three/std.rs");
		assert_eq!(path.restrict(None), cwd);
		
		path = PathBuf::from("../tests/std.rs");
		assert_eq!(path.restrict(None), cwd);
		
		path = PathBuf::from("");
		assert_eq!(path.restrict(Some(Path::new("/foo/bar"))), PathBuf::from("/foo/bar"));

		path = PathBuf::from(".");
		assert_eq!(path.restrict(Some(Path::new("."))), cwd);

		path = PathBuf::from("/tests/std.rs");
		assert_eq!(path.restrict(Some(Path::new("."))), cwd);
		
		path = PathBuf::from("tests/std.rs");
		assert_eq!(path.restrict(Some(Path::new("."))), cwd.join("tests/std.rs"));
		
		path = PathBuf::from("/foo/tests/std.rs");
		assert_eq!(path.restrict(Some(Path::new("/foo/bar"))), PathBuf::from("/foo/bar"));
		
		path = PathBuf::from("/foo/bar/tests/std.rs");
		assert_eq!(path.restrict(Some(Path::new("/foo/bar"))), PathBuf::from("/foo/bar/tests/std.rs"));
		
		let path: &Path;
		path = Path::new("/foo/bar/tests/std.rs");
	//	assert_eq!(path.restrict(Some("/foo/bar")),                 Path::new("/foo/bar/tests/std.rs"));
	//	assert_eq!(path.restrict(Some("/foo/bar".to_owned())),      Path::new("/foo/bar/tests/std.rs"));
		assert_eq!(path.restrict(Some(Path::new("/foo/bar"))),      Path::new("/foo/bar/tests/std.rs"));
		assert_eq!(path.restrict(Some(Path::new("/foo/bar"))),      PathBuf::from("/foo/bar/tests/std.rs"));
		assert_eq!(path.restrict(Some(&PathBuf::from("/foo/bar"))), Path::new("/foo/bar/tests/std.rs"));
		assert_eq!(path.restrict(Some(&PathBuf::from("/foo/bar"))), PathBuf::from("/foo/bar/tests/std.rs"));
	}
	
	//		strip_root															
	#[test]
	fn strip_root() {
		let mut path: PathBuf;
		
		path = PathBuf::from("");
		assert_eq!(path.strip_root(), path);
		
		path = PathBuf::from(".");
		assert_eq!(path.strip_root(), path);
		
		path = PathBuf::from("..");
		assert_eq!(path.strip_root(), path);
		
		path = PathBuf::from("/");
		assert_eq!(path.strip_root(), PathBuf::from(""));
		
		path = PathBuf::from("/tests/std.rs");
		assert_eq!(path.strip_root(), PathBuf::from("tests/std.rs"));
		
		path = PathBuf::from("//tests/std.rs");
		assert_eq!(path.strip_root(), PathBuf::from("tests/std.rs"));
		
		if cfg!(windows) {
			path = PathBuf::from(r"C:\tests\std.rs");
			assert_eq!(path.strip_root(), PathBuf::from(r"tests\std.rs"));
			
			path = PathBuf::from(r"C:tests\std.rs");
			assert_eq!(path.strip_root(), PathBuf::from(r"tests\std.rs"));
			
			path = PathBuf::from(r"\tests\std.rs");
			assert_eq!(path.strip_root(), PathBuf::from(r"tests\std.rs"));
			
			path = PathBuf::from(r"\\tests\std.rs");
			assert_eq!(path.strip_root(), PathBuf::from(r"tests\std.rs"));
		}
		
		let path: &Path;
		path = Path::new("tests/std.rs");
		assert_eq!(path.strip_root(), Path::new("tests/std.rs"));
		assert_eq!(path.strip_root(), PathBuf::from("tests/std.rs"));
	}
}


