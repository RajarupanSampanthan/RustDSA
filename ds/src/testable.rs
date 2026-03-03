use std::fmt::Display;


pub trait Testable : Display + Default{}


impl<T> Testable for T where T: std::fmt::Display + Default {}