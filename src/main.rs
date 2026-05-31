use std::io;
use std::cmp::Ordering;
use std::collections::LinkedList;
use rand::Rng;

fn main()
{
    pub struct Node<'a> {
        // "'a" is explicit lifetime
        pub prev: Option<&'a Box<i32>>, // pointer to some val on the heap via a refrence so ownership isn't transfered
        // rust doesnt have null vals so how will we say that a pointer is null? option enum
        // we're saying prev can be either, Nothing(None) or Something(Some)
        pub val: Box<i32>,
        pub next: Option<&'a Box<i32>>,

    }
    impl Node<'_>{
        // associated functions, all fn within an impl
        // fn with &self as a perameter are methods

        /// Makes a new node
        fn new (val:Box<i32>) -> Self{
            Self{
                prev: None,
                val,
                next: None,
            }
        }

    }
    pub struct LinkedList<'a> {
        head: Node<'a>,
        tail: Node<'a>,
        size: u32,
        capacity: u32
    }


    fn main(){
        let mut mock_head = Node::new(Box::new(89));

        let mut mock_tail = Node::new(Box::new(19));

        mock_head.prev = None;
        mock_head.next = Option::from(&mock_tail.val);


        mock_tail.prev = Option::from(&mock_tail.val);
        mock_tail.next = Option::None;

    }

}
