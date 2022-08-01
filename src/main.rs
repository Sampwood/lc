// mod p1;
// mod p1480;
// mod p383;
// mod p412;
// mod p876;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn main() {
    println!("Hello, world!");

    // let a: Vec<i32> = p1480::Solution::running_sum(vec![3, 1, 2, 4]);
    // println!("{:?}", a);

    // let ransom_note = String::from("aa");
    // let magazine = String::from("ab");
    // let r = p383::Solution::can_construct(ransom_note, magazine);
    // println!("{}", r);

    // let node = p876::ListNode { val: 1, next: None };
    // let mid = p876::Solution::middle_node(Some(Box::new(node)));
    // println!("{:?}", mid);

    let a = Some(Box::new(ListNode::new(1)));
    println!("{:?}", middle_node(a));
}
