



use crate::linked_list::SequentialLinkedList;
use crate::BasicLinkedList;
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}



#[cfg(test)]
mod tests {


    use super::*;

    type ToTest = usize;

    fn get_vec_of_defaults() -> Vec<Box<dyn SequentialLinkedList<ToTest>>>{
        vec![
            Box::new(BasicLinkedList::<ToTest>::default())
        ]
    }

    #[test]
    fn it_works() {
        let mut defaults = get_vec_of_defaults();


        for mut linked_list in defaults{

            assert!(linked_list.pop().is_err());
            linked_list.push(0);
            let v = linked_list.pop();
            assert!(v.is_ok());
            assert_eq!(v.unwrap(), 0);
        }

    }
}