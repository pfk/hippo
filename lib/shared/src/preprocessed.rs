//
// Hippo
// (C) 2021 Brave Monday
//

use std::fmt::Display;

/// An entity which has been subjected to preprocessing.
pub trait Preprocessed<T>: Display {
	fn preprocessed_data() -> &'static T;
}
