use core::fmt::Display;
use std::{default, ops::Deref};
use crate::{Testable, linked_list::SequentialLinkedList};



impl <T> SequentialLinkedList<T> for BasicLinkedList<T> where T : Testable{
    fn push(&mut self, value : T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Result<T, String> {
        let v = self.head.take().map(|node| {
            self.head = node.next;
            node.value
        });
        v.ok_or("Failed".to_string())
    }
}

pub struct BasicLinkedList<T> where T : Testable {

    head : Link<T>

}

impl<T> Display for BasicLinkedList<T> where T : Testable{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.head{
            None => write!(f, "Nil"),
            Some(x) => {
                let other = x.next.as_ref().take().map(|x| {
                    x.as_ref()
                });
                write!(f, "({})", x.value)
            }
        }
    }
}

impl<T> Default for BasicLinkedList<T> where T : Testable{
    fn default() -> Self {
        Self { head: Default::default() }
    }
}


impl<T> Drop for BasicLinkedList<T> where T : Testable {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> where T: Testable {
    pub value : T,
    pub next : Link<T>
}

impl<T> Default for Node<T> where T : Testable{
    fn default() -> Self {
        Self { value: Default::default(), next : None }
    }
}


