
use crate::Testable;




pub trait  SequentialLinkedList<T> where T : Testable {

    fn push(&mut self, value : T);

    fn pop(&mut self) -> Result<T, String>;
}