//! This module provides extensions to the Rust standard library.



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



//		Structs

//		LimitIterator															
/// This struct provides an iterator that limits the number of items returned.
/// 
/// This will be returned from the [`limit()`](IteratorExt::limit()) method, and
/// will generally not be used directly.
/// 
/// # See also
/// 
/// * [`IteratorExt::limit()`]
/// 
pub struct LimitIterator<I> {
	//		Private properties													
	/// The iterator to limit.
	iter:  I,
	
	/// The maximum number of items to return.
	limit: Option<usize>,
	
	/// The number of items returned so far.
	count: usize,
}

impl<I: Iterator> Iterator for LimitIterator<I> {
	type Item = I::Item;
	
	//		next																
	fn next(&mut self) -> Option<Self::Item> {
		if let Some(limit) = self.limit {
			if self.count >= limit {
				return None;
			}
			self.count += 1;
		}
		self.iter.next()
	}
}



//		Traits

//§		AsStr																	
/// This trait provides an [`as_str()`](AsStr::as_str()) method.
/// 
/// This trait requires the presence of an [`as_str()`](AsStr::as_str()) method.
/// It's not possible to apply this trait purely as a marker to the existing
/// types such as [`String`] that already have an [`as_str()`](AsStr::as_str())
/// method and have it recognised that they already have it, due to Rust's
/// implementation determination allowing multiple methods of the same name,
/// differentiated by trait. In other words, our trait could define a method
/// with the same name and signature as another trait, but an implementation of
/// the function would not be considered to satisfy both. Both traits would have
/// to have their methods specifically implemented, even if identical, and then
/// the conflict would be resolved at call-time by specifying which trait's
/// method is being called.
/// 
/// However, it is possible to apply this trait and call the underlying method
/// on the type, for such cases as this may be required. This trait should
/// therefore be applied to any types of interest, for which the [`as_str()`](crate::serde::as_str())
/// serialisation function provided by the [`serde`](crate::serde) module is
/// intended to be specified. Suitable standard and common types such as
/// [`String`] and [`str`] have already had this trait implemented, and those
/// implementations will be brought into scope when this trait is used.
/// 
/// In reality, implementations onto standard types should not really be
/// necessary, as this trait exists primarily for use with the
/// [`serde::as_str()`](crate::serde::as_str()) method, and Serde already knows
/// how to handle such types so there is no real advantage to be gained by
/// implementing this trait for such types. The intent and purpose of this trait
/// is to provide a way to specify a string representation for types that do not
/// already have one, such as dual-nature enums, i.e. where they can be
/// represented as either a string or a number. Still, the trait has been
/// applied to some common types for consistency and completeness.
/// 
/// The only current drawback is that trait functions cannot currently be
/// declared as `const`, and the scope of the [`as_str()`](AsStr::as_str())
/// method is usually such that it could be declared as `const` otherwise.
/// 
pub trait AsStr {
	//		as_str																
	/// Provides a string slice representation of the type.
	#[must_use]
	fn as_str(&self) -> &str;
}

impl AsStr for String {
	//		as_str																
	fn as_str(&self) -> &str {
		//	This simply calls the existing method, i.e. String.as_str(), but is
		//	required to allow the trait to be applied to the type.
		self.as_str()
	}
}

impl AsStr for str {
	//		as_str																
	fn as_str(&self) -> &str {
		//	This simply returns the existing value, i.e. self, but is required
		//	to allow the trait to be applied to the type.
		self
	}
}

//§		IteratorExt																
/// This trait provides additional functionality to [`Iterator`].
pub trait IteratorExt: Iterator {
	//		limit																
	/// Limits the number of items returned by an iterator.
	/// 
	/// This is the same as [`Iterator::take()`], but accepts an [`Option`], so
	/// that the limit does not have to be specified. It allows a match such as
	/// `foo.iter().take(match limit { Some(n) => n, None => foo.len() })`
	/// to be simplified to `foo.iter().limit(limit)`, and is especially useful
	/// when `foo` is of unknown or infinite length.
	/// 
	/// # Parameters
	/// 
	/// * `limit` - The maximum number of items to return. If [`None`], no limit
	///             will be applied.
	/// 
	fn limit(self, limit: Option<usize>) -> LimitIterator<Self> where Self: Sized {
		LimitIterator { iter: self, limit, count: 0 }
	}
}

impl<I: Iterator> IteratorExt for I {}

//§		PathExt																	
/// This trait provides additional functionality to [`Path`].
pub trait PathExt {
	//		append																
	/// Appends a string to a path.
	/// 
	/// Adds a string to the end of a path, and returns the result as a new
	/// path. This is specifically different to both [`push()`](PathBuf::push())
	/// and [`join()`](Path::join()), as it simply appends the string without
	/// having any further effect on the path. By contrast, [`push()`](PathBuf::push())
	/// and [`join()`](Path::join()) will append a new string as a new path
	/// component, which will then be normalized, and will also replace the path
	/// entirely if the string is an absolute path.
	/// 
	/// # Parameters
	/// 
	/// * `suffix` - The string to append to the path.
	/// 
	/// # See also
	/// 
	/// * [`std::path::Path::join()`]
	/// * [`std::path::PathBuf::push()`]
	/// 
	fn append<P: AsRef<Path>>(&self, suffix: P) -> PathBuf;
	
	//		is_subjective														
	/// Checks if the path is specifically relative to the current directory.
	/// 
	/// Returns `true` if the path starts with a reference to the current
	/// directory, i.e. `.` or `..` (as `..` is the parent of the current
	/// directory and therefore related to it), making it specifically and
	/// explicitly related to the current working directory. This can be
	/// described as a subjective relative path, as opposed to an objective
	/// relative path which is generically relative because it lacks a root
	/// component.
	/// 
	/// A path that is subjective is also always relative. It is not possible to
	/// have a subjective absolute path, as that would be a contradiction in
	/// terms. However, objective paths may be either absolute or relative.
	/// There is therefore no method `is_objective()`, as it does not currently
	/// appear to have a useful purpose.
	/// 
	/// # See also
	/// 
	/// * [`std::path::Path::is_absolute()`]
	/// * [`std::path::Path::is_relative()`]
	/// 
	fn is_subjective(&self) -> bool;
	
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
	/// Key differences are that [`canonicalize()`](Path::canonicalize()) will
	/// return an error if the path does not exist, and will resolve symlinks.
	/// This function will remove `.` segments, and will remove the parent
	/// segment along with the current segment for `..` segments.
	/// 
	/// # See also
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
	/// * `base` - The base path to use. If this is [`None`] then the current
	///            working directory will be used.
	/// 
	/// # See also
	/// 
	/// * [`normalize()`](PathExt::normalize())
	/// 
	fn restrict<P: AsRef<Path>>(&self, base: P) -> PathBuf;
	
	//		strip_parentdirs													
	/// Removes references to parent directories, i.e. `..`.
	/// 
	/// Removes any [`ParentDir`](std::path::Component::ParentDir) components
	/// from either the beginning of the path or anywhere in the path.
	/// 
	/// This function does not touch the filesystem, or check if the path is
	/// valid or exists. It will also not attempt to resolve the parent
	/// directory references that it removes, so they will be taken out with no
	/// effect on the rest of the path.
	/// 
	/// # Parameters
	/// 
	/// * `remove_all` - If `true` then all parent directory references will be
	///                  removed, otherwise only those at the beginning of the
	///                  path will be removed.
	/// 
	/// # See also
	/// 
	/// * [`std::path::Component`]
	/// * [`std::path::Path::components()`]
	/// 
	fn strip_parentdirs(&self, remove_all: bool) -> PathBuf;
	
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
	/// # See also
	/// 
	/// * [`std::path::Path::components()`]
	/// * [`std::path::Path::has_root()`]
	/// * [`std::path::Path::is_absolute()`]
	/// * [`std::path::Path::strip_prefix()`]
	/// * [`std::path::Prefix`]
	/// * [`std::path::PrefixComponent`]
	/// 
	fn strip_root(&self) -> PathBuf;
}

impl PathExt for Path {
	//		append																
	fn append<P: AsRef<Path>>(&self, suffix: P) -> PathBuf {
		PathBuf::from([
			self.as_os_str().to_os_string(),
			OsString::from(suffix.as_ref()),
		].into_iter().collect::<OsString>())
	}
	
	//		is_subjective														
	#[allow(clippy::iter_nth_zero)]
	fn is_subjective(&self) -> bool {
			self.is_relative()
		&&	self.components().count() > 0
		&&	(
				self.components().nth(0).unwrap() == PathComponent::CurDir
			||	self.components().nth(0).unwrap() == PathComponent::ParentDir
			)
	}
	
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
	fn restrict<P: AsRef<Path>>(&self, base: P) -> PathBuf {
		let basepath = base.as_ref().normalize();
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
	
	//		strip_parentdirs													
	fn strip_parentdirs(&self, remove_all: bool) -> PathBuf {
		if self.as_os_str().is_empty() || (!remove_all && self.is_absolute()) {
			return self.to_owned();
		}
		let mut at_start = true;
		let mut segments: Vec<OsString> = vec!();
		for component in self.components() {
			match component {
				PathComponent::Prefix(_) |
				PathComponent::RootDir   |
				PathComponent::CurDir    |
				PathComponent::Normal(_) => {
					segments.push(component.as_os_str().to_os_string());
					at_start = false;
				},
				PathComponent::ParentDir => {
					if !remove_all && !at_start {
						segments.push(component.as_os_str().to_os_string());
					}
				},
			}
		}
		segments.iter().collect()
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


