use crate::types::{Relay, TransportError};
use crate::element::{Finder, WebElement};

pub struct Waiter<T: Relay> {
	f: Box<dyn Finder<T>>,
	d: std::time::Duration // TODO: use chrono
}


impl<T:Relay> Waiter<T> {

	pub fn from(&self, finder: impl Finder<T>) -> &Waiter<T> {
		todo!();
	}

	pub fn duration(&self, d: std::time::Duration) -> &Waiter<T>{
		todo!()
	}

	pub fn until<F>(self, f:F) ->Result<(bool,WebElement<T>),TransportError>
	where
	F: FnMut(Box<dyn Finder<T>>)-> Result<(bool,WebElement<T>),TransportError>{
		todo!()
	}
}
