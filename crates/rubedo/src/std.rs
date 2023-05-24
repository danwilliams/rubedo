//		Modules

#[cfg(test)]
#[path = "tests/std.rs"]
mod tests;



//		Packages

use std::{
	env,
	ffi::OsString,
	path::{Component as PathComponent, Path, PathBuf},
};



//		Traits

//§		PathExt																	
/// This trait provides additional functionality to [`Path`].
pub trait PathExt {
	//		normalize															
	/// Normalizes the path.
	/// 
	/// Computes the canonicalized, absolute path of a file or directory, but
	/// without expanding symlinks or checking existence. A path that starts
	/// with `.` or without an initial separator will be interpreted relative to
	/// the current working directory. Empty paths and paths of `.` alone will
	/// result in the current working directory being returned.
	/// 
	/// This function will normalize the path by removing any `.` and `..`
	/// segments and returning the "real" path. It does this without touching
	/// the filesystem, and so is an abstract but also simpler version of
	/// [`canonicalize()`](Path::canonicalize()), which does a number of
	/// filesystem checks.
	/// 
	/// Key differences are that `canonicalize()` will return an error if the
	/// path does not exist, and will resolve symlinks. This function will
	/// remove `.` segments, and will remove the parent segment along with the
	/// current segment for `..` segments.
	/// 
	/// # See Also
	/// 
	/// * [`std::fs::canonicalize()`]
	/// * [`std::path::Path::canonicalize()`]
	/// 
	fn normalize(&self) -> PathBuf;
}

impl PathExt for Path {
	//		normalize															
	fn normalize(&self) -> PathBuf {
		let cwd = env::current_dir().unwrap();
		if self.as_os_str().is_empty() {
			return cwd;
		}
		let mut segments: Vec<OsString> = vec!();
		for (i, component) in self.components().enumerate() {
			match component {
				PathComponent::Prefix(_) |
				PathComponent::RootDir   => {
					if i == 0 {
						segments.push(component.as_os_str().to_os_string());
					}
				},
				PathComponent::CurDir    |
				PathComponent::ParentDir => {
					if i == 0 {
						segments.append(
							cwd.components()
								.map(|c| c.as_os_str().to_os_string())
								.collect::<Vec<OsString>>()
								.as_mut()
						);
					}
					if component == PathComponent::ParentDir && segments.len() > 1 {
						segments.pop();
					}
				},
				PathComponent::Normal(_) => {
					if i == 0 {
						segments.push(cwd.as_os_str().to_os_string());
					}
					segments.push(component.as_os_str().to_os_string());
				},
			}
		}
		segments.iter().collect()
	}
}


