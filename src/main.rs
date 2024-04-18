
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    println!("Hello, world!");

    build_dag();

    let arr = vec![1];
    // let safe_arr = Arc::new(RwLock::new(arr.clone()));
    let arr2 = arr.clone();
    // let arr2 = &arr;
    let t = std::thread::spawn(move || {
        println!("arr: {:?}", arr2);
    });
    let _ = t.join();
    println!("{:?}", arr);

    let hello = Arc::new("hello");
    let hello2 = hello.clone();
    let hello_thread = std::thread::spawn(move || {
        println!("hello in thread: {:?}", hello2);
    });
    println!("hello in main: {:?}", hello);
    let _ = hello_thread.join();

    let aaa: &'static str = "aaa...";
    let ptr = aaa.as_ptr();
    let len = aaa.len();
    println!("ptr: {:?}, len: {:?}", ptr, len);

    let num = 33i32;
    let num2 = 10u64;
    println!("num: {:?}, num2: {:?}", num, num2);
}

fn build_dag() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let node4 = Node::new(4);

    node3.update_downstream(Rc::new(node4));
    node1.update_downstream(Rc::new(node3));
    node2.update_downstream(node1.get_downstream().unwrap());

    println!("node1: {:?}, node2: {:?}", node1, node2);
    node1.get_id();

    // let node3 = node1.get_downstream().unwrap();
    // node3.deref().update_downstream(Rc::new(Node::new(5)));
    // let node5 = Node::new(5);
    // cannot borrow as mutable
    // node3.update_downstream(Rc::new(node5));
}

// DAG
#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<Node>>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstream: Rc<Node>) {
        self.downstream = Some(downstream);
    }

    pub fn get_downstream(&self) -> Option<Rc<Node>> {
        self.downstream.as_ref().map(|node| Rc::clone(node))
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}
