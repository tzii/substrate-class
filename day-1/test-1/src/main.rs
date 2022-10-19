struct Solution;

impl Solution {
    pub fn sub_array(a: Vec<i32>, b: Vec<i32>) -> bool {
        if a.len() == 0 {
            return false;
        }
        if b.len() == 0 {
            return true;
        }
        'outer: for (i, x) in a.iter().enumerate() {
            if x != b.get(0).unwrap() {
                continue;
            }
            for (j, y) in b.iter().enumerate() {
                if a.get(i + j).unwrap() != y {
                    continue 'outer;
                }
            }
            return true;
        }
        return false;
    }
}

fn main() {
    let a = vec![1, 2, 3, 5, 6, 8, 10, 11];
    let b = vec![6, 8, 10, 1];
    print!("{ }", Solution::sub_array(a, b));
}
