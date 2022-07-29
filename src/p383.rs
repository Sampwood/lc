use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut record: HashMap<char, i32> = HashMap::new();

        for c in ransom_note.chars() {
            let num = record.entry(c).or_insert(0);
            *num += 1;
        }

        for c in magazine.chars() {
            let num = record.get_mut(&c);
            match num {
                None => (),
                Some(x) => {
                    if *x == 1 {
                        record.remove(&c);
                    } else if *x > 1 {
                        *x -= 1;
                    }

                    ()
                }
            };
        }

        record.len() == 0
    }
}
