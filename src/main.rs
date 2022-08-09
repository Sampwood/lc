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

    println!("{:?}", find_kth_largest(vec![1,2,3], 1));
}

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut res = nums;

    res.sort();

    res[res.len() - k as usize]
}
