#![allow(non_snake_case)]

//		Tests

//§		AsStr																	
#[cfg(test)]
mod as_str {
	use super::super::*;
	use crate::sugar::s;
	
	//		as_str																
	#[test]
	fn as_str__string() {
		let foo = s!("Test");
		assert_eq!(foo.as_str(), "Test");
	}
	#[test]
	fn as_str__str() {
		let foo = "Test";
		assert_eq!(foo.as_str(), "Test");
	}
}

//§		IteratorExt																
#[cfg(test)]
mod iterator_ext {
	use super::super::*;
	
	//		limit																
	#[test]
	fn limit__empty() {
		let vec:    Vec<usize> = Vec::new();
		let result: Vec<_>     = vec.iter().limit(Some(10)).collect();
		assert_eq!(result.len(), 0);
	}
	#[test]
	fn limit__no_limit() {
		let vec            = vec![1, 2, 3, 4, 5];
		let result: Vec<_> = vec.iter().limit(None).cloned().collect();
		assert_eq!(result.len(), vec.len());
		assert_eq!(result,       vec);
	}
	#[test]
	fn limit__within_limit() {
		let vec            = vec![1, 2, 3, 4, 5];
		let result: Vec<_> = vec.iter().limit(Some(10)).cloned().collect();
		assert_eq!(result.len(), vec.len());
		assert_eq!(result,       vec);
	}
	#[test]
	fn limit__exceeds_limit() {
		let vec            = vec![1, 2, 3, 4, 5];
		let result: Vec<_> = vec.iter().limit(Some(3)).cloned().collect();
		assert_eq!(result.len(), 3);
		assert_eq!(result,       vec![1, 2, 3]);
	}
}
	
//§		PathExt																	
#[cfg(test)]
mod path_ext {
	use super::super::*;
	
	//		append																
	#[test]
	fn append() {
		let mut path: PathBuf;
		
		path = PathBuf::from("");
		assert_eq!(path.append(""), PathBuf::from(""));
		
		path = PathBuf::from("tests/std.rs");
		assert_eq!(path.append(".bak"),                 PathBuf::from("tests/std.rs.bak"));
		assert_eq!(path.append(".bak".to_owned()),      PathBuf::from("tests/std.rs.bak"));
		assert_eq!(path.append(OsString::from(".bak")), PathBuf::from("tests/std.rs.bak"));
		assert_eq!(path.append(PathBuf::from(".bak")),  PathBuf::from("tests/std.rs.bak"));
		assert_eq!(path.append(Path::new(".bak")),      PathBuf::from("tests/std.rs.bak"));
		
		let path: &Path;
		path = Path::new("tests/std.rs");
		assert_eq!(path.append(".bak"), PathBuf::from("tests/std.rs.bak"));
	}
	
	//		is_subjective														
	#[test]
	fn is_subjective() {
		assert_eq!(PathBuf::from(".").is_subjective(),      true);
		assert_eq!(PathBuf::from("./").is_subjective(),     true);
		assert_eq!(PathBuf::from("./foo").is_subjective(),  true);
		assert_eq!(PathBuf::from("..").is_subjective(),     true);
		assert_eq!(PathBuf::from("../").is_subjective(),    true);
		assert_eq!(PathBuf::from("../foo").is_subjective(), true);
		assert_eq!(PathBuf::from("foo").is_subjective(),    false);
		assert_eq!(PathBuf::from(".bak").is_subjective(),   false);
		assert_eq!(PathBuf::from("..bak").is_subjective(),  false);
		assert_eq!(PathBuf::from("/").is_subjective(),      false);
		assert_eq!(PathBuf::from("/.").is_subjective(),     false);
		assert_eq!(PathBuf::from("/..").is_subjective(),    false);
		assert_eq!(PathBuf::from("/foo").is_subjective(),   false);
		
		assert_eq!(Path::new(".").is_subjective(),          true);
	}
	
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
		assert_eq!(path.restrict(""),  cwd);
		
		path = PathBuf::from("");
		assert_eq!(path.restrict("."), cwd);
		
		path = PathBuf::from(".");
		assert_eq!(path.restrict(""),  cwd);
		
		path = PathBuf::from(".");
		assert_eq!(path.restrict("."), cwd);
		
		path = PathBuf::from("..");
		assert_eq!(path.restrict("."), cwd);
		
		path = PathBuf::from("/");
		assert_eq!(path.restrict("."), cwd);
		
		path = PathBuf::from("/tests/std.rs");
		assert_eq!(path.restrict("."), cwd);
		
		path = PathBuf::from("tests/std.rs");
		assert_eq!(path.restrict("."), cwd.join("tests/std.rs"));
		
		path = PathBuf::from("tests/../std.rs");
		assert_eq!(path.restrict("."), cwd.join("std.rs"));
		
		path = PathBuf::from("tests/../../std.rs");
		assert_eq!(path.restrict("."), cwd);
		
		path = PathBuf::from("tests/../../one/two/three/std.rs");
		assert_eq!(path.restrict("."), cwd);
		
		path = PathBuf::from("../tests/std.rs");
		assert_eq!(path.restrict("."), cwd);
		
		path = PathBuf::from("");
		assert_eq!(path.restrict(Path::new("/foo/bar")), PathBuf::from("/foo/bar"));

		path = PathBuf::from(".");
		assert_eq!(path.restrict(Path::new(".")), cwd);

		path = PathBuf::from("/tests/std.rs");
		assert_eq!(path.restrict(Path::new(".")), cwd);
		
		path = PathBuf::from("tests/std.rs");
		assert_eq!(path.restrict(Path::new(".")), cwd.join("tests/std.rs"));
		
		path = PathBuf::from("/foo/tests/std.rs");
		assert_eq!(path.restrict(Path::new("/foo/bar")), PathBuf::from("/foo/bar"));
		
		path = PathBuf::from("/foo/bar/tests/std.rs");
		assert_eq!(path.restrict(Path::new("/foo/bar")), PathBuf::from("/foo/bar/tests/std.rs"));
		
		let path: &Path;
		path = Path::new("/foo/bar/tests/std.rs");
		assert_eq!(path.restrict("/foo/bar"),                Path::new("/foo/bar/tests/std.rs"));
		assert_eq!(path.restrict("/foo/bar".to_owned()),     Path::new("/foo/bar/tests/std.rs"));
		assert_eq!(path.restrict(&Path::new("/foo/bar")),    Path::new("/foo/bar/tests/std.rs"));
		assert_eq!(path.restrict(&Path::new("/foo/bar")),    PathBuf::from("/foo/bar/tests/std.rs"));
		assert_eq!(path.restrict(PathBuf::from("/foo/bar")), Path::new("/foo/bar/tests/std.rs"));
		assert_eq!(path.restrict(PathBuf::from("/foo/bar")), PathBuf::from("/foo/bar/tests/std.rs"));
	}
	
	//		strip_parentdirs													
	#[test]
	fn strip_parentdirs() {
		let mut path: PathBuf;
		
		path = PathBuf::from("");
		assert_eq!(path.strip_parentdirs(true), path);
		
		path = PathBuf::from(".");
		assert_eq!(path.strip_parentdirs(true), path);
		
		path = PathBuf::from("..");
		assert_eq!(path.strip_parentdirs(true), PathBuf::from(""));
		
		path = PathBuf::from("/");
		assert_eq!(path.strip_parentdirs(true), path);
		
		path = PathBuf::from("/tests/std.rs");
		assert_eq!(path.strip_parentdirs(true), PathBuf::from("/tests/std.rs"));
		
		path = PathBuf::from("tests/std.rs");
		assert_eq!(path.strip_parentdirs(true), PathBuf::from("tests/std.rs"));
		
		path = PathBuf::from("../tests/std.rs");
		assert_eq!(path.strip_parentdirs(true), PathBuf::from("tests/std.rs"));
		
		path = PathBuf::from("/../tests/std.rs");
		assert_eq!(path.strip_parentdirs(true), PathBuf::from("/tests/std.rs"));
		
		path = PathBuf::from("../../../tests/std.rs");
		assert_eq!(path.strip_parentdirs(true), PathBuf::from("tests/std.rs"));
		
		path = PathBuf::from("/tests/../std.rs");
		assert_eq!(path.strip_parentdirs(true), PathBuf::from("/tests/std.rs"));
		
		path = PathBuf::from("tests/../std.rs");
		assert_eq!(path.strip_parentdirs(true), PathBuf::from("tests/std.rs"));
		
		path = PathBuf::from("../tests/../../std.rs");
		assert_eq!(path.strip_parentdirs(true), PathBuf::from("tests/std.rs"));
		
		path = PathBuf::from("");
		assert_eq!(path.strip_parentdirs(false), path);
		
		path = PathBuf::from(".");
		assert_eq!(path.strip_parentdirs(false), path);
		
		path = PathBuf::from("..");
		assert_eq!(path.strip_parentdirs(false), PathBuf::from(""));
		
		path = PathBuf::from("/");
		assert_eq!(path.strip_parentdirs(false), path);
		
		path = PathBuf::from("/tests/std.rs");
		assert_eq!(path.strip_parentdirs(false), PathBuf::from("/tests/std.rs"));
		
		path = PathBuf::from("tests/std.rs");
		assert_eq!(path.strip_parentdirs(false), PathBuf::from("tests/std.rs"));
		
		path = PathBuf::from("../tests/std.rs");
		assert_eq!(path.strip_parentdirs(false), PathBuf::from("tests/std.rs"));
		
		path = PathBuf::from("/../tests/std.rs");
		assert_eq!(path.strip_parentdirs(false), PathBuf::from("/../tests/std.rs"));
		
		path = PathBuf::from("../../../tests/std.rs");
		assert_eq!(path.strip_parentdirs(false), PathBuf::from("tests/std.rs"));
		
		path = PathBuf::from("/tests/../std.rs");
		assert_eq!(path.strip_parentdirs(false), PathBuf::from("/tests/../std.rs"));
		
		path = PathBuf::from("tests/../std.rs");
		assert_eq!(path.strip_parentdirs(false), PathBuf::from("tests/../std.rs"));
		
		path = PathBuf::from("../tests/../../std.rs");
		assert_eq!(path.strip_parentdirs(false), PathBuf::from("tests/../../std.rs"));
		
		let path: &Path;
		path = Path::new("tests/std.rs");
		assert_eq!(path.strip_parentdirs(false), Path::new("tests/std.rs"));
		assert_eq!(path.strip_parentdirs(false), PathBuf::from("tests/std.rs"));
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


