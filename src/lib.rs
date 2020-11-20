use std::time::{Instant, Duration};

pub struct Until<I> {
	inner: I,
	until: Instant,
}

pub trait UntilExt<T: Iterator> {
	fn until(self, until: Instant) -> Until<Self>
		where
			Self: Sized,
	{
		Until::new(self, until)
	}

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