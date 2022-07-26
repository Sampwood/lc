pub struct Solution {}

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
      let mut results = Vec::new();
      let mut sum = 0;

      for i in &nums {
        sum += i;
        results.push(sum);
      }
      
      results
    }
}
