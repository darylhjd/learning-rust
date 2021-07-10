#![allow(dead_code)]

use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::io::BufRead;
use std::ops::{Add, Sub, Mul};
use std::panic;

use quick_xml::events::Event;
use quick_xml::Reader;
use serde::Deserialize;

// ROT-13 encryption. Basically character rotation
pub fn rot13(text: &str) -> String {
    text.chars().map(|c|
        match c {
            'A'..='M' | 'a'..='m' => (c as u8 + 13) as char,
            'N'..='Z' | 'n'..='z' => (c as u8 - 13) as char,
            _ => c,
        }).collect()
}

// Print all prime pairs that add to the given number.
pub fn prime_pairs(num: usize) -> String {
    let mut p_string = String::new();
    p_string.push_str(num.to_string().as_str());
    p_string.push_str(" = ");

    // Create a set to store already checked numbers
    let mut checked: HashSet<usize> = HashSet::new();

    // Loop to check the numbers.
    // We know that the first prime number is 2, so we start from there.
    let mut n = 2;
    let mut found = false;
    while n * 2 <= num { // We only check to half, since after that it will be repeated.
        if !checked.contains(&n) && check_prime(n) && check_prime(num - n) {
            found = true;
            checked.insert(n);
            p_string.push_str(format!("{} + {}, ", n, num - n).as_str());
        }
        n += 1;
    }

    if found {
        p_string.pop();
        p_string.pop();
    } else {
        p_string.push_str("None");
    }
    return p_string;
}

// Check if number is prime.
fn check_prime(num: usize) -> bool {
    // All integers have the form 6k + i, k int > 0 and i = [-1, 4].
    // 2 divides 6k + i, i = [0, 2, 4] and 3 divides 6k + i, i = [3].
    // 2 and 3 are primes, and all even numbers except 2 are non-primes.
    // Check if 2 or 3
    if num <= 3 {
        return num > 1;
    }
    // This leaves all remaining numbers unchecked in the form 6k +- 1.
    if (num % 2 == 0) || (num % 3 == 0) {
        return false;
    }
    // We can skip 4 since it is an even number that would have been checked earlier

    // From this point, all remaining numbers are of the form 6k +- 1.
    // We loop through all possible values then return accordingly.
    let mut k: usize = 1;
    loop {
        let p = (6 * k - 1).pow(2);
        if p > num { // Return true if all numbers exhausted.
            return true;
        } else if (num % p == 0) || (num % (p + 2) == 0) { // If num is divisible by the number.
            return false;
        }
        k += 1;
    }
}

// Stuff used for the quiz
// Struct for implementing the quiz.
#[derive(Debug, Deserialize)]
pub struct Quiz {
    questions: Vec<Question>,
}

// Struct for implementing a question.
#[derive(Debug, Deserialize)]
pub struct Question {
    question: String,
    answer: String,
}

impl Quiz {
    pub fn new(filename: &str) -> Result<Quiz, Box<dyn Error>> {
        let file_content = fs::read_to_string(filename)?;
        let questions: Vec<Question> = serde_json::from_str(&file_content)?;

        return Ok(Quiz { questions });
    }

    pub fn num_questions(&self) -> usize {
        return self.questions.len();
    }

    pub fn ask_questions(&self) -> usize {
        // Loop through the questions and ask them, then check for the answer and keep
        // track of the score.
        let mut score = 0;

        // Loop through the questions.
        let mut answer = String::new();
        for question in self.questions.iter() {
            // Print the question.
            println!("{}", question.question);
            println!("Your answer: ");
            // Read the line
            answer.clear();
            std::io::stdin().lock().read_line(&mut answer).expect("Unable to read from stdin.");
            let trimmed = answer.trim();
            // Check the answer
            if trimmed == question.answer {
                println!("Correct!");
                score += 1;
            } else {
                println!("Wrong! Answer was {}, but you answered {}.", question.answer, trimmed);
            }
            println!();
        }
        return score;
    }
}

// Parse a XML document and get all the text from it.
pub fn parse_xml(file: &str) -> Result<Vec<String>, Box<dyn Error>> {
    // Read the XML from the file.
    let mut reader = Reader::from_file(file)?;
    let mut buffer = Vec::new();
    let mut text = Vec::new();
    loop {
        match reader.read_event(&mut buffer) {
            Ok(Event::Text(bt)) => text.push(bt.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break,
            _ => (),
        }
        buffer.clear();
    }
    return Ok(text.into_iter().map(|s| s.trim().to_string()).filter(|s| s.len() > 0).collect());
}

// Stuff for matrix
#[derive(Debug, Eq, PartialEq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    matrix: Vec<Vec<isize>>,
}

impl Matrix {
    // Create a default matrix.
    pub fn new_default(rows: usize, cols: usize) -> Option<Self> {
        if rows == 0 || cols == 0 {
            return None;
        }
        return Some(Matrix {
            rows,
            cols,
            matrix: vec![vec![0; cols]; rows],
        });
    }

    // Create a new matrix.
    pub fn new_with_vector(m: Vec<Vec<isize>>) -> Option<Self> {
        // Check for matrix goodness.
        if m.len() == 0 { // If the number of rows is zero, return None.
            return None;
        }
        // Check that all rows have the same length.
        let length = m[0].len();
        if !m.iter().all(|r| r.len() == length) {
            return None;
        }
        if m[0].len() == 0 { // If the number of columns is zero, return None.
            return None;
        }

        return Some(Matrix {
            rows: m.len(),
            cols: m[0].len(),
            matrix: m,
        });
    }
}

impl Add for Matrix {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // Check dimension bounds
        if self.rows != other.rows || self.cols != other.cols {
            panic!("mismatched dimensions. expected {}x{} but got {}x{}", self.rows, self.cols,
                   other.rows, other.cols);
        }
        // Create the new_with_vector matrix to hold the data
        let new_matrix: Vec<Vec<isize>> = (0..self.rows)
            .map(|r| (0..self.cols).map(|c| self.matrix[r][c] + other.matrix[r][c]).collect())
            .collect();

        return Self::new_with_vector(new_matrix).unwrap();
    }
}

impl Sub for Matrix {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        // Check dimension bounds
        if self.rows != other.rows || self.cols != other.cols {
            panic!("mismatched dimensions. expected {}x{} but got {}x{}", self.rows, self.cols,
                   other.rows, other.cols);
        }
        // Create the new_with_vector matrix to hold the data
        let new_matrix: Vec<Vec<isize>> = (0..self.rows)
            .map(|r| (0..self.cols).map(|c| self.matrix[r][c] - other.matrix[r][c]).collect())
            .collect();

        return Self::new_with_vector(new_matrix).unwrap();
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        // Check dimension bounds
        // The cols of the first matrix must be equal to the rows of the second matrix.
        if self.cols != other.rows {
            panic!("mismatched dimensions. cannot multiply {}x{} with {}x{}", self.rows, self.cols,
                   other.rows, other.cols);
        }
        let mut new_vector: Vec<Vec<isize>> = vec![vec![0; other.cols]; self.rows];

        // Matrix multiplication
        for self_r in 0..self.rows {
            for other_c in 0..other.cols {
                // Calculate the total for the square
                let total = (0..other.rows)
                    .fold(0, |acc, other_row| acc + self.matrix[self_r][other_row] * other.matrix[other_row][other_c]);
                new_vector[self_r][other_c] = total;
            }
        }

        return Self::new_with_vector(new_vector).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rot13() {
        assert_eq!("Uryyb Jbeyq!", rot13("Hello World!"));
    }

    #[test]
    fn test_check_prime() {
        assert!(check_prime(59));
        assert!(check_prime(97));
        assert!(check_prime(271));
        assert_eq!(false, check_prime(68));
        assert_eq!(false, check_prime(267));
    }

    #[test]
    fn test_prime_pairs() {
        assert_eq!("10 = 3 + 7, 5 + 5", prime_pairs(10));
        assert_eq!("1 = None", prime_pairs(1));
        assert_eq!("11 = None", prime_pairs(11));
        assert_eq!("17 = None", prime_pairs(17));
        assert_eq!("21 = 2 + 19", prime_pairs(21));
        println!("{}", prime_pairs(5678));
    }

    #[test]
    fn test_quiz() {
        assert!(Quiz::new("non-existent file").is_err());
        assert!(Quiz::new("assets/quiz.json").is_ok());
    }

    #[test]
    fn test_parse_xml() {
        let result = vec!["Some <java> class",
                          "Another \"java\" class",
                          "Weird \'XML\' config",
                          "JavaScript & program",
                          "Cascading style sheet: © - \u{489}"];
        assert_eq!(result, parse_xml("assets/test1.xml").unwrap());

        let result = vec!["test",
                          "kkss\" = ddd' >"];
        assert_eq!(result, parse_xml("assets/test2.xml").unwrap());
    }

    #[test]
    fn test_matrix() {
        // Testing new default
        let m = Matrix::new_default(3, 3);
        let matrix = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(Matrix { rows: 3, cols: 3, matrix }, m.unwrap());

        // Testing new default fail
        let m = Matrix::new_default(0, 4);
        assert!(m.is_none());

        // Testing ok.
        let v = vec![vec![0, 1, 2], vec![3, 4, 5]];
        let m = Matrix::new_with_vector(v);
        assert!(m.is_some());
        assert_eq!(Matrix { rows: 2, cols: 3, matrix: vec![vec![0, 1, 2], vec![3, 4, 5]] }, m.unwrap());

        // Testing no rows.
        let v: Vec<Vec<isize>> = vec![];
        let m = Matrix::new_with_vector(v);
        assert!(m.is_none());

        // Testing no cols.
        let v: Vec<Vec<isize>> = vec![vec![]];
        let m = Matrix::new_with_vector(v);
        assert!(m.is_none());

        // Testing different col length.
        let v = vec![vec![1, 2, 3], vec![1, 2]];
        let m = Matrix::new_with_vector(v);
        assert!(m.is_none());

        // Testing addition
        let m1 = Matrix::new_with_vector(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let m2 = Matrix::new_with_vector(vec![vec![3, 2, 1], vec![6, 5, 4]]).unwrap();
        let r_matrix = vec![vec![4, 4, 4], vec![10, 10, 10]];
        assert_eq!(Matrix { rows: 2, cols: 3, matrix: r_matrix }, m1 + m2);

        // Testing panic addition
        let m1 = Matrix::new_with_vector(vec![vec![1, 2], vec![3, 4]]).unwrap();
        let m2 = Matrix::new_with_vector(vec![vec![1]]).unwrap();
        let result = panic::catch_unwind(|| m1 + m2);
        assert!(result.is_err());

        // Testing subtraction
        let m1 = Matrix::new_with_vector(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let m2 = Matrix::new_with_vector(vec![vec![3, 2, 1], vec![6, 5, 4]]).unwrap();
        let r_matrix = vec![vec![-2, 0, 2], vec![-2, 0, 2]];
        assert_eq!(Matrix { rows: 2, cols: 3, matrix: r_matrix}, m1 - m2);

        // Testing panic subtraction
        let m1 = Matrix::new_with_vector(vec![vec![1, 2], vec![3, 4]]).unwrap();
        let m2 = Matrix::new_with_vector(vec![vec![1]]).unwrap();
        let result = panic::catch_unwind(|| m1 - m2);
        assert!(result.is_err());

        // Testing multiplication
        let m1 = Matrix::new_with_vector(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]).unwrap();
        let m2 = Matrix::new_with_vector(vec![vec![3, 2, 1], vec![6, 5, 4], vec![9, 8, 7]]).unwrap();
        let r_matrix = vec![vec![42, 36, 30], vec![96, 81, 66], vec![150, 126, 102]];
        assert_eq!(Matrix { rows: 3, cols: 3, matrix: r_matrix }, m1 * m2);
    }
}