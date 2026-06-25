use std::rc::Rc; // reference counting, allows for mulltiple owners of a val by keeping track of how many vals point to it
use std::cell::RefCell; // moves borrowing rules to run time, allows mutibility


pub struct Node {
    // "'a" is explicit lifetime, meaning the struct can only live as long as the lifetime of the reference

    // rust doesnt have null vals so how will we say that a pointer is null? "Option" enum
    // we're saying prev can be either, Nothing(None) or Something(Some)

    pub val: Option<Box<i32>>,
    pub next: Option<Rc<RefCell<Node>>>, // pointer to some val on the heap via refrence counting so ownership isn't transfered
}
impl Node<>{
    // associated functions, all fn within an impl
    // fn with &self as a perameter are methods

    /// Makes a new node
    fn new (val:Option<Box<i32>>) -> Self{
        // Constructor
        Self{
            val, // value is a ppointer to a val on the heap, may or may not be null(NONE)
            next: None,
        }
    }

    // rust does not allow for function overloading :(
    /// Makes a new empty node
      fn create_empty() -> Self{
         Self{
             val: None,
             next: None,
         }
      }

}

impl Clone for Node {
    // creates a new copy of a node, it's a different node
    fn clone(&self) -> Self{
        Self{
            val: self.val.clone(), // deep copy of the val
            next: self.next.clone(), // shallow copy of the next pointer
        }
    }
}
pub struct LinkeddList {
    head: Node,
    tail: Node,
    size: u32,
    capacity: u32
}
impl LinkeddList<> {
     fn new(_head:Node, _tail:Node) -> Self{
         // constructor that takes in a head and a tail and returns a linked list
         Self {
             head: _head,
             tail: _tail,
             size: 1,
             capacity: 1,
         }
     }
}
// Rc handle ──► [ strong_count | weak_count | RefCell { borrow_flag | Node { val, next } } ]

fn main() {

    
    
    // println!("{:?}",mock_head.val);
    // println!("{:?}",node_pointer_2.clone().unwrap().borrow_mut().val);
}
