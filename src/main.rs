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
pub struct LinkeddList {
    head: Node,
    tail: Node,
    size: u32,
    capacity: u32
}
impl LinkeddList<> {
    // fn new(_head:Node, _tail:Node) -> Self{
    //     // constructor that takes in a head and a tail and returns a linked list
    //     Self {
    //         head: _head,
    //         tail: _tail,
    //         size: 1,
    //         capacity: 1,
    //     }
    // }
}
fn main() {
    // the below is a mock manually created linked list
    let mut mock_head = Node::new(Option::from(Box::new(9)));
    let mut mock_tail = Node::new(Option::from(Box::new(19)));

    let node_pointer_1 = Option::from(Rc::new(RefCell::new(mock_head))); // moves the value of mock_head into the Rc
    let node_pointer_2 = Option::from(Rc::new(RefCell::new(mock_tail)));

    match &node_pointer_1 {
        Some(node) => {
            node.borrow_mut().next = node_pointer_2;
        },
        None => {
            println!("Node is None");
        }
    }
    
    println!("{:?}",node_pointer_1.clone().unwrap().borrow_mut().val);
    println!("{:?}",node_pointer_1.clone().unwrap().borrow_mut().val);
}
