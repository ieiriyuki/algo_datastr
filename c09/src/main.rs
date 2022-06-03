use crate::List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

//http://www.nct9.ne.jp/m_hiroi/linux/rust04.html
struct Stack {
    head: List
}

impl Stack {
    fn new() -> Stack {
        Stack { head: Nil }
    }

    fn push(&mut self, x: u32) {
        let new_node = Cons(x, Box::new(std::mem::replace(&mut self.head, Nil)));
        self.head = new_node;
    }

    fn pop(&mut self) -> Option<u32> {
        match std::mem::replace(&mut self.head, Nil) {
            Nil => None,
            Cons(x, list) => {
                let list = *list;
                self.head = list;
                Some(x)
            }
        }
    }

    fn peek(&self) -> Option<&u32> {
        match self.head {
            Nil => None,
            Cons(ref x, _) => Some(x)
        }
    }

    fn is_empty(&self) -> bool {
        match self.head {
            Nil => true,
            _ => false
        }
    }
}

fn main() {
    try_linked_list();
    try_stack();
}

fn try_linked_list() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

fn try_stack() {
    let mut a = Stack::new();
    for i in 0..4 {
        a.push(i)
    }
    println!("{}", a.peek().unwrap());
    while !a.is_empty() {
        println!("{}", a.pop().unwrap());
    }
}
