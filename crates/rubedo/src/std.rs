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
	/// * [`restrict()`](PathExt::restrict())
	/// * [`std::fs::canonicalize()`]
	/// * [`std::path::Path::canonicalize()`]
	/// 
	fn normalize(&self) -> PathBuf;
	
	//		restrict															
	/// Restricts the path.
	/// 
	/// Computes the canonicalized, absolute path of a file or directory, but
	/// without allowing parent directory traversal to go beyond the base path.
	/// If no base path is specified, the current working directory will be
	/// used. If the path starts with `.` then this will be interpreted relative
	/// to the base path.
	/// 
	/// This function calls [`normalize()`](PathExt::normalize()), and so the
	/// fundamental behaviour of the resolution performed is the same as that
	/// function. The difference is that this function will not allow the path
	/// to go beyond the base path, and so any `..` segments will simply be
	/// removed from the path if they would otherwise go beyond the anchor
	/// point.
	/// 
	/// This does have the effect that if a path does try to traverse too far,
	/// it may lose additional components. For example, a path of `../foo` will
	/// end up losing the `foo` component, as the logic will be that `foo` is
	/// intended to be a sibling to the base path and not a child of it, and is
	/// therefore invalid. So if the base directory is `/home/user` then a path
	/// of `../foo` will be resolved to `/home/user` and not `/home/user/foo`.
	/// The effect of this continues further, in that all children of `foo` will
	/// also be deemed invalid. So `../foo/bar` will also be resolved to
	/// `/home/user`, and not `/home/user/foo/bar` or `/home/user/bar`. Care
	/// should therefore be taken when using this function to ensure that the
	/// path returned is valid for the intended use.
	/// 
	/// In the case of the path being absolute, it will be resolved and then
	/// compared against the base path. If the path is a child of the base path
	/// then it will be returned - otherwise the base path will be returned, as
	/// the path is invalid. For example, if the base directory is `/home/user`
	/// then a path of `/home/user/foo` will be returned, but a path of
	/// `/home/otheruser` will return `/home/user`.
	/// 
	/// Note that this function does not touch the filesystem, does not expand
	/// symlinks, and does not check that the path exists - including the
	/// base path. Hence when this documentation talks about base directory,
	/// it does so interchangeably with base path, as the valid intent would be
	/// for the base path to be a directory, but this is not actually checked.
	/// 
	/// # Parameters
	/// 
	/// * `base` - The base path to use. If this is `None` then the current
	///            working directory will be used.
	/// 
	/// # See Also
	/// 
	/// * [`normalize()`](PathExt::normalize())
	/// 
	fn restrict(&self, base: Option<&Path>) -> PathBuf;
	
	//		strip_root															
	/// Makes the path relative by removing the root and/or prefix components.
	/// 
	/// Removes any components from the path that are considered to be the root
	/// or prefix of the path. The prefix is this context is not the same as in
	/// [`strip_prefix()`](Path::strip_prefix()), which removes a specific
	/// string prefix from the path. Rather, the prefix here is a
	/// [`PrefixComponent`](std::path::PrefixComponent). A path is considered to
	/// be absolute if it has a root on Unix, or if it has both root and prefix
	/// on Windows. Therefore, in order to convert the path to be relative, both
	/// the root and prefix must be removed.
	/// 
	/// This function does not touch the filesystem, or check if the path is
	/// valid or exists. It will also not attempt to resolve special directory
	/// references such as `.` or `..`.
	/// 
	/// # See Also
	/// 
	/// * [`std::path::Path::components()`]
	/// * [`std::path::Path::has_root()`]
	/// * [`std::path::Path::is_absolute()`]
	/// * [`std::path::Path::strip_prefix()`]
	/// * [`std::path::Prefix`](std::path::Prefix)
	/// * [`std::path::PrefixComponent`](std::path::PrefixComponent)
	/// 
	fn strip_root(&self) -> PathBuf;
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
	
	//		restrict															
	fn restrict(&self, base: Option<&Path>) -> PathBuf {
		let basepath = match base {
			Some(base) => base.to_path_buf(),
			None       => env::current_dir().unwrap(),
		}.normalize();
		if self.as_os_str().is_empty() {
			return basepath;
		}
		let mut path = if self.is_absolute() {
			self.to_path_buf()
		} else {
			basepath.join(self)
		}.normalize();
		if !path.starts_with(&basepath) {
			path = basepath
		}
		path
	}
	
	//		strip_root															
	fn strip_root(&self) -> PathBuf {
		if self.as_os_str().is_empty() || self.is_relative() {
			return self.to_owned();
		}
		let mut segments: Vec<OsString> = vec!();
		for component in self.components() {
			match component {
				PathComponent::Prefix(_) |
				PathComponent::RootDir   => {},
				PathComponent::CurDir    |
				PathComponent::ParentDir |
				PathComponent::Normal(_) => {
					segments.push(component.as_os_str().to_os_string());
				},
			}
		}
		segments.iter().collect()
	}
}


