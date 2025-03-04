//! **`OneLauncher` Utilities**
//! Standard utilities for use within all Rust subprojects.
//!
//! - [`logging`]: Async utilities for log4j parsing with [`nom`].
//! - [`platform`]: Async utilities for managing OS-specific [`interpulse`] operations and rules.
//! - [`io`]: Async wrapper around [`tokio::fs`] and [`std::io::Error`] for our error system.

pub mod io;
pub mod logging;
pub mod platform;

/// Simple macro that takes a mutable reference and inserts it into a codeblock closure
/// as an owned reference.
///
/// mutable reference gets epically owned by free thinking macro!!!!! (not clickbait)
/// im going insane insane insane insane insane insane insane insane insane
#[macro_export]
macro_rules! ref_owned {
	($id:ident = $init:expr => $transform:block) => {{
		let mut it = $init;
		{
			let $id = &mut it;
			$transform;
		}
		it
	}};
}

/// Combines an iterator of `T` and an iterator of [`Option<T>`],
/// removing and ensuring the safety of any [`None`] values in the process.
pub fn chain_iterator<T>(
	required: impl IntoIterator<Item = T>,
	optional: impl IntoIterator<Item = Option<T>>,
) -> Vec<T> {
	required
		.into_iter()
		.map(Some)
		.chain(optional)
		.flatten()
		.collect()
}

#[inline]
#[must_use]
pub fn test_type_of<T>(_: T) -> &'static str {
	std::any::type_name::<T>()
}
