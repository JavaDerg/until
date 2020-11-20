use std::time::{Instant, Duration};

/// Iter that will continue unless the inner iter is empty or a previously specified time has
/// passed
pub struct Until<I> {
	inner: I,
	until: Instant,
}

pub trait UntilExt<T: Iterator> {
	/// Using this the iter will continue till the specified time has passed or the internal
    /// iter is empty
    fn until(self, until: Instant) -> Until<Self>
		where
			Self: Sized,
	{
		Until::new(self, until)
	}

    /// Using this the iter will continue for the specified time or until the internal iter is
    /// empty
	fn do_for(self, until: Duration) -> Until<Self>
		where
			Self: Sized,
	{
		Until::new(self, Instant::now() + until)
	}
}

impl<I> Until<I> {
	pub fn new(iter: I, until: Instant) -> Self {
		Self { inner: iter, until }
	}
}

impl<I: Iterator> Iterator for Until<I> {
	type Item = I::Item;

	fn next(&mut self) -> Option<Self::Item> {
		if Instant::now() > self.until {
			None
		} else {
			self.inner.next()
		}
	}
}

impl<I: Iterator> UntilExt<I> for I {}
