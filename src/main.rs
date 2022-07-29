mod p1;
mod p1480;
mod p383;
mod p412;

fn main() {
    println!("Hello, world!");

    let a: Vec<i32> = p1480::Solution::running_sum(vec![3, 1, 2, 4]);
    println!("{:?}", a);

    let ransom_note = String::from("aa");
    let magazine = String::from("ab");
    let r = p383::Solution::can_construct(ransom_note, magazine);
    println!("{}", r);
}
