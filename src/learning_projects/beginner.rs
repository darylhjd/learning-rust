#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::HashMap;
use std::f64::consts::PI;

use rand;
use rand::Rng;

use super::io_utils;

// Find factorial of number entered by user.
pub fn factorial(num: &isize) -> Result<usize, &'static str> {
    if *num < 0 {
        return Err("Number must be non-negative!");
    } else if *num == 0 {
        return Ok(1);
    }

    let sum: Option<usize> = (1..=*num).into_iter().try_fold(1, |acc: usize, i| acc.checked_mul(i as usize));
    match sum {
        Some(n) => Ok(n),
        None => Err("Overflow!")
    }
}

// Convert from SGD to USD.
pub fn convert_currency(a: &f64) -> f64 {
    let rate = 0.75517108;
    return a * rate;
}

// Print triangles
pub fn triangles(n: &u32) {
    // Print from most to least
    (1..=*n).rev().into_iter().for_each(|i| println!("{}", "*".repeat(i as usize)));
    println!();

    // Print from least to most
    (1..=*n).into_iter().for_each(|i| println!("{}", "*".repeat(i as usize)));
    println!();

    // Print least to most skipping even rows
    (1..=*n).filter(|i| *i % 2 != 0).for_each(|i| println!("{}", "*".repeat(i as usize)));
}

// Execute a random function
pub fn random_fn() {
    fn func_one() {
        println!("Function 1!");
    }

    fn func_two() {
        println!("Function 2!");
    }

    fn func_three() {
        println!("Function 3!");
    }

    // Create a hashmap for the functions
    let mut map: HashMap<i32, fn()> = HashMap::new();
    map.insert(0, func_one);
    map.insert(1, func_two);
    map.insert(2, func_three);

    // Create a random integer from 0 to 2.
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..=2);

    // Call the random function.
    map.get(&n).unwrap()(); // Ignore errors.
}

// Guessing game between 1 and 100.
pub fn guessing_game() {
    // Choose a random number between 1 and 100 inclusive.
    // I'm changing to 10 since it takes very long.
    let correct_num = rand::thread_rng().gen_range(1..=10);
    loop {
        let guess = io_utils::read_number("Enter guess: ") as i32;
        if !(1..=10).contains(&guess) {
            println!("Please input a number between 1 and 10!");
            continue;
        }

        match guess.cmp(&correct_num) {
            Ordering::Equal => {
                println!("You won!");
                break;
            }
            Ordering::Less => println!("Go higher!"),
            Ordering::Greater => println!("Go lower!")
        }
    }
}

// Print fibonacci sequence up till `n`.
pub fn fib_until(n: usize) {
    let mut x = 1;
    let mut y = 1;
    let mut temp;
    while x <= n {
        print!("{} ", x);
        temp = y;
        y = x + y;
        x = temp;
    }
    println!();
}

// Count vowels and consonants
pub fn count_vow_cons(s: &str) -> (usize, usize) {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut v_count = 0;
    let mut c_count = 0;
    for c in s.chars() {
        if !c.is_alphabetic() {
            break;
        }
        match vowels.contains(&c) {
            true => v_count += 1,
            _ => c_count += 1
        }
    }
    return (v_count, c_count);
}

// Find fifth root of sum of squares of first 100 odd numbers.
pub fn find_fifth_root() -> f64 {
    let mut sum = 0;

    let mut num = 0;
    let mut current_number: usize = 0;
    while num < 100 {
        while current_number % 2 == 0 {
            current_number += 1;
        }
        sum += current_number.pow(2); // Square number.
        num += 1;
        current_number += 1;
    }

    return (sum as f64).powf(1.0 / 5.0);
}

// Get all possible permutations for a word.
pub fn word_scramble(w: String) -> Vec<String> {
    // Base case.
    // If the string is empty, then return.
    if w.len() == 0 {
        return vec![w.to_string()];
    }

    let mut char_iter = w.chars();
    // Get the first letter
    let first_letter = char_iter.next().unwrap();
    // Permute the remaining letters.
    let combinations = word_scramble(char_iter.collect());

    // Insert the first character at every possible position for each combination.
    let mut new_combinations: Vec<String> = vec![];
    for comb in combinations {
        // Insert at every possible index
        for i in 0..=comb.len() {
            let start = comb.chars().take(i).collect::<String>();
            let end = comb.chars().skip(i).collect::<String>();
            new_combinations.push(format!("{}{}{}", start, first_letter, end));
        }
    }

    return new_combinations;
}

// Stuff used for Circle Params
#[derive(Debug)]
pub struct Circle {
    radius: f64,
    diameter: f64,
    area: f64,
}

// Circle methods/functions
impl Circle {
    // New with radius
    pub fn new_with_radius(radius: f64) -> Self {
        let area = PI * radius.powi(2);
        return Circle { radius, diameter: radius * 2.0, area };
    }

    // New with diameter
    pub fn new_with_diameter(diameter: f64) -> Self {
        let radius = diameter / 2.0;
        let area = PI * radius.powi(2);
        return Circle { radius, diameter, area };
    }

    // New with area
    pub fn new_with_area(area: f64) -> Self {
        let radius = (area / PI).powf(1.0 / 2.0); // Square root.
        return Circle { radius, diameter: radius * 2.0, area };
    }
}

// Calculate circle params
pub fn circle_params() -> Circle {
    // Take either radius, diameter or area and calculate the rest.
    println!("Enter an option:");
    println!("1. New Circle with Area");
    println!("2. New Circle with Diameter");
    println!("3. New Circle with Radius");

    let mut m;
    loop {
        m = io_utils::read_number("Enter your choice: ") as usize;
        match m {
            1..=3 => break,
            _ => {
                println!("Please enter a choice from 1 to 3 only.");
                continue;
            }
        };
    }

    // Enter the number.
    let mut num;
    loop {
        num = io_utils::read_number("Enter value: ");
        if num < 0 as f64 {
            println!("Number must be non-negative.");
            continue;
        }
        break;
    }

    match m {
        1 => Circle::new_with_area(num),
        2 => Circle::new_with_diameter(num),
        3 => Circle::new_with_radius(num),
        _ => panic!("Not possible!"),
    }
}

// Read line and write it out backwards using a recursive function.
pub fn backwards_string(s: String) -> String {
    // Base case. If length of the string is zero, then return.
    if s.len() == 0 {
        return s;
    }

    // Get the first character
    let first = s.chars().nth(0).unwrap();
    // Reverse the rest of the string
    let reversed = backwards_string(s.chars().skip(1).collect());

    // Append the first character to the end of the string
    return format!("{}{}", reversed, first);
}

// Empty struct for implementing the calculator
pub struct Calc;

pub enum CalcOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Calc {
    // Addition
    pub fn add(x: f64, y: f64) -> f64 {
        return x + y;
    }

    // Subtract
    pub fn subtract(x: f64, y: f64) -> f64 {
        return x - y;
    }

    // Multiply
    pub fn multiply(x: f64, y: f64) -> f64 {
        return x * y;
    }

    // Divide
    pub fn divide(x: f64, y: f64) -> f64 {
        return x / y; // No panic for floats, instead return inf.
    }

    // Any operation
    pub fn operate(x: f64, y: f64, op: CalcOperation) -> f64 {
        match op {
            CalcOperation::Add => Self::add(x, y),
            CalcOperation::Subtract => Self::subtract(x, y),
            CalcOperation::Multiply => Self::multiply(x, y),
            CalcOperation::Divide => Self::divide(x, y),
        }
    }
}

// Simple calculator
pub fn calculator(x: f64, y: f64, op: CalcOperation) -> f64 {
    return Calc::operate(x, y, op);
}

// Stuff used for PiggyBank
pub struct PiggyBank {
    coins: HashMap<CoinType, usize>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum CoinType {
    TwoDollar = 200,
    OneDollar = 100,
    FiftyCents = 50,
    TwentyCents = 20,
    TenCents = 10,
    FiveCents = 5,
}

impl PiggyBank {
    pub fn new() -> Self {
        return PiggyBank {
            coins: HashMap::new(),
        };
    }

    pub fn push_coin(&mut self, t: CoinType, amt: usize) {
        let num = self.coins.entry(t).or_default();
        *num = num.saturating_add(amt);
    }

    pub fn get_worth(&self) -> f64 {
        // I need help here.
        // How can I use
        let total: usize = self.coins.iter().
            fold(0, |acc, (key, value)|
                acc.saturating_add(value.saturating_mul(*key as usize)));
        return total as f64 / 100.0;
    }
}

// Check whether a string is a palindrome or not.
pub fn check_palindrome(s: &str) -> bool {
    let check_str = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>();

    for (c, r) in check_str.chars().zip(check_str.chars().rev()) {
        if !c.eq(&r) {
            return false;
        }
    }
    return true;
}

// Stuff for Weighted Score
pub struct Scores {
    mid_term: usize,
    homework: usize,
    finals: usize,
}

impl Scores {
    pub fn new(mid_term: usize, homework: usize, finals: usize) -> Self {
        return Scores { mid_term, homework, finals };
    }

    pub fn calculate_weighted_score(&self) -> f64 {
        return 0.2 * self.mid_term as f64 + 0.4 * self.finals as f64 + 0.4 * self.homework as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(Ok(362_880), factorial(&9));
        assert_eq!(Err("Overflow!"), factorial(&999));
    }

    #[test]
    fn test_convert_currency() {
        assert_eq!("25.93", format!("{:.2}", convert_currency(&34.34)))
    }

    #[test]
    fn test_count_vow_cons() {
        let cases = ["this", "what", "no1qw"];
        let results = [(1, 3), (1, 3), (1, 1)];
        for it in cases.iter().zip(results.iter()) {
            let (case, result) = it;
            assert_eq!(*result, count_vow_cons(*case));
        }
    }

    #[test]
    fn test_find_fifth_root() {
        assert_eq!(16.787482611035614, find_fifth_root());
    }

    #[test]
    fn test_word_scramble() {
        assert_eq!(24, word_scramble(String::from("test")).len()); // 4!
        assert_eq!(5040, word_scramble(String::from("amazing")).len()); // 7!
    }

    #[test]
    fn test_backwards_string() {
        assert_eq!(String::from("tset"), backwards_string(String::from("test")));
        assert_eq!(String::from("gnizama"), backwards_string(String::from("amazing")));
        assert_eq!(String::new(), backwards_string(String::new()));
    }

    #[test]
    fn test_calculator() {
        assert!((7.69 - calculator(5.0, 2.69, CalcOperation::Add)).abs() < 0.00000001);
        assert!((-9.54 - calculator(4.2, 13.74, CalcOperation::Subtract)).abs() < 0.00000001);
        assert!((5.0 - calculator(2.5, 2.0, CalcOperation::Multiply)).abs() < 0.00000001);
        assert!(calculator(6.0, 0.0, CalcOperation::Divide).is_infinite());
        assert!((2.5 - calculator(5.0, 2.0, CalcOperation::Divide)).abs() < 0.00000001);
    }

    #[test]
    fn test_piggy_bank() {
        let mut bank = PiggyBank::new();
        bank.push_coin(CoinType::FiftyCents, 2);
        bank.push_coin(CoinType::TwoDollar, 3);
        assert!((7.0 - bank.get_worth()).abs() < 0.00000001);

        bank.push_coin(CoinType::TenCents, 5);
        assert!((7.5 - bank.get_worth()).abs() < 0.00000001);
    }

    #[test]
    fn test_check_palindrome() {
        assert!(check_palindrome("racecar"));
        assert!(check_palindrome(""));
        assert!(check_palindrome("2002"));
        assert_eq!(check_palindrome("2004"), false);
        assert_eq!(check_palindrome("not a palindrome"), false);
    }
}
