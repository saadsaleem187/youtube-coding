struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut total = 0;
        let chars: Vec<char> = s.chars().collect();

        for i in 0..chars.len() {
            let current = Self::get_char_value(chars[i]);

            if i + 1 < chars.len() {
                let next = Self::get_char_value(chars[i + 1]);

                if current < next {
                    total -= current;
                } else {
                    total += current;
                }
            } else {
                total += current;
            }
        }

        total
    }

    fn get_char_value(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }
}

fn main() {
    println!("Example 1: {}", Solution::roman_to_int("III".to_string()));
    println!("Example 2: {}", Solution::roman_to_int("LVIII".to_string()));
    println!(
        "Example 3: {}",
        Solution::roman_to_int("MCMXCIV".to_string())
    );
}
