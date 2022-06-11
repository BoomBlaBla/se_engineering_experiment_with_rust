use std::cell::Ref;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct LinkedList<T> {
	head: Link<T>,
	tail: Link<T>,
}

//RefCell使得Rc的内容可以被修改
type Link<T> = Option<Rc<RefCell<Node<T>>>>;
 
#[derive(Debug)]
struct Node<T> {
	elem: T,
	next: Link<T>,
}

impl<T> LinkedList<T>{
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, elem: T) {
		let new_point = Rc::new(RefCell::new(Node {
			elem,
			next: self.head.take()
		}));
		if self.tail.is_none() {
			self.tail = Some(Rc::clone(&new_point));
		}
		self.head = Some(new_point);
	}

    fn pop_link(&mut self) -> Link<T> {
		let first_link = self.head.take();
		let next_link = match first_link {
			Some(ref x) => x.borrow_mut().next.take(),
			None => None
		};
		self.head = next_link;
		if self.head.is_none() {
			self.tail.take();
		}
		first_link
	}
 
	fn pop(&mut self) -> Option<T> {
		let first_link = self.head.take();
		let next_link = match first_link {
			Some(ref x) => x.borrow_mut().next.take(),
			None => None
		};
		self.head = next_link;
		if self.head.is_none() {
			self.tail.take();
		}
		first_link.map(|r| Rc::try_unwrap(r).ok().unwrap().into_inner().elem)
	}

	fn peek_node(&self) -> Option<Ref<Node<T>>> {
		self.head.as_ref().map(|r| r.borrow())
	}
 
	fn peek(&self) -> Option<Ref<T>> {
		self.head.as_ref().map(|r| Ref::map(
			r.borrow(), |node| &node.elem
		))
	}

    
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

struct IntoIter<T>(LinkedList<T>);
impl<T> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		self.0.pop()
	}
}

fn main(){
	let mut list: LinkedList<u32> = LinkedList::new();
	list.push(101);
	list.push(202);
	list.push(303);
	list.push(404);

	let mut iter = list.into_iter();
	println!("next: {:?}\n", iter.next());
	println!("next: {:?}\n", iter.next());
	println!("next: {:?}\n", iter.next());
	println!("next: {:?}\n", iter.next());
}


 