fn main() {
    let a: Vec<i32> = two_sum(vec![3,2,4], 6);
    println!("{:?}", a)
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();

    for (index, num) in nums.iter().enumerate() {
        for (index2, num2) in nums.iter().enumerate() {
            if num + num2 == target && index != index2 {
                res.push(index as i32);
                res.push(index2 as i32);
                
                return res;
            }
        }
    }

    res
}