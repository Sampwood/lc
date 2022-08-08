// mod p1;
// mod p1480;
// mod p383;
// mod p412;
// mod p876;

use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
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

    println!("{:?}", merge_two_lists(None, None));
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if list1.is_none() {
        return list2;
    } else if list2.is_none() {
        return list1;
    }

    let mut head = ListNode::new(0);
    let mut cur = &mut head;
    let mut l1 = list1;
    let mut l2 = list2;

    while let (Some(i1), Some(i2)) = (l1.as_ref(), l2.as_ref()) {
        if i1.val <= i2.val {
            cur.next = l1;
            cur = cur.next.as_mut().unwrap();
            l1 = cur.next.take();
        } else {
            cur.next = l2;
            cur = cur.next.as_mut().unwrap();
            l2 = cur.next.take();
        }
    }

    if l1.is_none() {
        cur.next = l2;
    } else {
        cur.next = l1;
    }


    head.next
}
