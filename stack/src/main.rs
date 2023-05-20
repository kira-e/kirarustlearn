mod kirack {
    struct Node<T> {
        value: T,
        next: Option<Box<Node<T>>>,
    }

    pub struct Stack<T> {
        head: Option<Box<Node<T>>>,
        size: i32,
    }

    impl<T> Stack<T> {
        pub fn init() -> Stack<T> {
            Stack {
                head: None,
                size: 0,
            }
        }
    }

    pub trait Lifo<T> {
        fn push(&mut self, value: T);
        fn pop(&mut self) -> Option<T>;
        fn size(&self) -> i32;
    }

    impl<T> Lifo<T> for Stack<T> {
        fn push(&mut self, value: T) {
            self.head = Some(Box::new(Node {
                value,
                next: self.head.take(),
            }));
            self.size += 1;
        }

        fn pop(&mut self) -> Option<T> {
            match self.head.take() {
                None => None,
                Some(root_box) => {
                    self.head = root_box.next;
                    self.size -= 1;
                    Some(root_box.value)
                }
            }
        }

        fn size(&self) -> i32 {
            self.size
        }
    }
}

use crate::kirack::Lifo;

fn main() {
    let mut stack = kirack::Stack::init();
    assert!(stack.size() == 0);

    stack.push(33);
    assert!(stack.size() == 1);
    assert!(stack.pop() == Some(33));

    stack.push(1);
    stack.push(2);

    assert!(stack.size() == 2);
    assert!(stack.pop() == Some(2));
    assert!(stack.size() == 1);
    assert!(stack.pop() == Some(1));
    assert!(stack.size() == 0);

    println!("Nice!");
}
