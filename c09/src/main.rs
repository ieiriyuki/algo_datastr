use crate::List::*;
use crate::CalcElem::*;

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

#[allow(dead_code)]
struct Queue {
    head: List,
    tail: List,
}

impl Queue {
    // VecDeque 使え
}

fn main() {
    try_linked_list();
    try_stack();

    println!("question 9-2");
    test_q09_2();
    let eq: Vec<&str> = vec!["3", "4", "+", "1", "2", "-", "*"];
    println!("{}", q09_2(eq));
    let eq: Vec<&str> = vec!["0.3", "0.4", "+", "0.1", "0.2", "-", "*"];
    println!("{}", q09_2(eq));

    println!("question 9-3");
    test_q09_3();
    println!("{}", q09_3("()"));
    println!("{}", q09_3("()()(()())"));
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

fn q09_2(eq: Vec<&str>) -> f64 {
    let mut stack: Vec<f64> = Vec::new();
    for i in eq.into_iter() {
        stack = parse_eq(stack, i)
    }
    let res: f64 = stack.pop().unwrap();
    return res
}

#[derive(Debug, PartialEq)]
enum CalcElem<'a> {
    Opr(&'a str),
    Flt(f64),
}

fn is_operator(s: &str) -> CalcElem {
    match s {
        "+" => CalcElem::Opr("+"),
        "-" => CalcElem::Opr("-"),
        "*" => CalcElem::Opr("*"),
        "/" => CalcElem::Opr("/"),
        _ => CalcElem::Flt(s.parse::<f64>().unwrap()),
    }
}

fn parse_eq(mut stack: Vec<f64>, op: &str) -> Vec<f64> {
    match is_operator(op){
        Flt(x) => {
            stack.push(x)
        },
        Opr("+") => {
            let y: f64 = stack.pop().unwrap();
            let x: f64 = stack.pop().unwrap();
            stack.push(x + y)
        },
        Opr("-") => {
            let y: f64 = stack.pop().unwrap();
            let x: f64 = stack.pop().unwrap();
            stack.push(x - y)
        },
        Opr("*") => {
            let y: f64 = stack.pop().unwrap();
            let x: f64 = stack.pop().unwrap();
            stack.push(x * y)
        },
        Opr("/") => {
            let y: f64 = stack.pop().unwrap();
            let x: f64 = stack.pop().unwrap();
            stack.push(x / y)
        },
        _ => {},
    }
    return stack
}

fn test_q09_2() {
    assert_eq!(is_operator("+"), Opr("+"));
    assert_eq!(is_operator("-"), Opr("-"));
    assert_eq!(is_operator("*"), Opr("*"));
    assert_eq!(is_operator("/"), Opr("/"));
    assert_eq!(is_operator("0.9"), Flt(0.9));

    assert_eq!(
        parse_eq(Vec::new(), "0.9"),
        vec![0.9]
    );
    println!("{}, {}", "0.1".parse::<f64>().unwrap(), "0.2".parse::<f64>().unwrap());
    println!("{:?}", parse_eq(vec![0.1, 0.2], "+"));
}

fn q09_3(x: &str) -> i32 {
    let mut stack: Vec<char> = Vec::new();
    let mut counter = 0;
    for i in x.chars() {
        match i {
            '(' => stack.push(i),
            ')' => {
                match stack.pop() {
                    Some('(') => counter += 1,
                    Some(_) => return -1,
                    None => return -1,
                }
            },
            _ => return -1,
        }
    }
    if stack.len() > 0 {
        return -1
    }
    return counter
}

fn test_q09_3() {
    assert_eq!(q09_3("a"), -1);
    assert_eq!(q09_3(")"), -1);
    assert_eq!(q09_3("("), -1);
}
