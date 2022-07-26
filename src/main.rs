mod p1480;

fn main() {
    println!("Hello, world!");

    let a: Vec<i32> = p1480::Solution::running_sum(vec![3,1,2,4]);

    println!("{:?}", a);
}
