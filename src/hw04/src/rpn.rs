use std::io;
use std::result;
use rand::prelude::*;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
/// An element of the stack. May be either integer or boolean.
pub enum Elt {
    Int(i32),
    Bool(bool),
}

#[derive(Debug)]
/// An RPN calculator error.
pub enum Error {
    /// Tried to pop from an empty stack.
    Underflow,
    /// Tried to operate on invalid types (e.g. 4 + true)
    Type,
    /// Unable to parse the input.
    Syntax,
    /// Some IO error occurred.
    IO(io::Error),
    /// The user quit the program (with `quit`).
    Quit,
}

#[derive(Debug)]
/// Types of RPN calculator operations.
pub enum Op {
    /// Adds two numbers: pop x, pop y, push x + y.
    Add,
    /// Checks equality of two values: pop x, pop y, push x == y.
    Eq,
    /// Negates a value: pop x, push ~x.
    Neg,
    /// Swaps two values: pop x, pop y, push x, push y.
    Swap,
    /// Computes a random number: pop x, push random number in [0, x).
    Rand,
    /// Quit the calculator.
    Quit,
}

// Stack
// Simply a wrapper around a vector.
pub struct Stack {
    stack: Vec<Elt>,
}

// Result
// Alias to std::result::Result, using custom Elt and Error.
pub type Result<T> = result::Result<T, Error>;

impl Stack {
    /// Creates a new Stack
    pub fn new() -> Stack {
        return Stack { stack: vec![] };
    }

    /// Pushes a value onto the stack.
    pub fn push(&mut self, val: Elt) -> Result<()> {
        self.stack.push(val);
        return Ok(());
    }

    /// Tries to pop a value off of the stack.
    pub fn pop(&mut self) -> Result<Elt> {
        return match self.stack.pop() {
            Some(e) => Ok(e),
            None => Err(Error::Underflow),
        }
    }

    // Private helper functions

    fn eval_add(&mut self) -> Result<()> {
        let one: i32;
        match self.pop()? {
            Elt::Bool(_) => return Err(Error::Type),
            Elt::Int(i) => one = i,
        }
        let two: i32;
        match self.pop()? {
            Elt::Bool(_) => return Err(Error::Type),
            Elt::Int(i) => two = i,
        }
        return self.push(Elt::Int(one + two));
    }

    fn eval_eq(&mut self) -> Result<()> {
        let one = self.pop()?;
        let two = self.pop()?;
        return self.push(Elt::Bool(one == two));
    }

    fn eval_neg(&mut self) -> Result<()> {
        return match self.pop()? {
            Elt::Bool(b) => self.push(Elt::Bool(!b)),
            Elt::Int(i) => self.push(Elt::Int(-i)),
        };
    }

    fn eval_swap(&mut self) -> Result<()> {
        let one = self.pop()?;
        let two = self.pop()?;
        self.push(one)?;
        return self.push(two);
    }

    fn eval_rand(&mut self) -> Result<()> {
        return match self.pop()? {
            Elt::Bool(_) => Err(Error::Type),
            Elt::Int(i) => self.push(Elt::Int(rand::thread_rng().gen_range(0..=i)))
        }
    }

    /// Tries to evaluate an operator using values on the stack.
    pub fn eval(&mut self, op: Op) -> Result<()> {
        return match op {
            Op::Add => self.eval_add(),
            Op::Eq => self.eval_eq(),
            Op::Neg => self.eval_neg(),
            Op::Swap => self.eval_swap(),
            Op::Rand => self.eval_rand(),
            Op::Quit => Err(Error::Quit),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop_empty1() {
        let mut s = Stack::new();

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res {} else { assert!(false); }
    }

    #[test]
    fn test_pop_empty2() {
        let mut s = Stack::new();
        s.push(Elt::Int(0)).unwrap();

        let res = s.pop();
        assert!(res.is_ok());

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res {} else { assert!(false); }
    }

    #[test]
    fn test_eval_add1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Int(1)).unwrap();

        assert!(s.eval(Op::Add).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(2));
    }

    #[test]
    fn test_eval_add2() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Add);
        assert!(res.is_err());
        if let Err(Error::Type) = res {} else { assert!(false); }
    }

    #[test]
    fn test_eval_add3() {
        let mut s = Stack::new();
        s.push(Elt::Bool(true)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Add);
        assert!(res.is_err());
        if let Err(Error::Type) = res {} else { assert!(false); }
    }

    #[test]
    fn test_eval_eq1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Int(1)).unwrap();

        assert!(s.eval(Op::Eq).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Bool(true));
    }

    #[test]
    fn test_eval_eq2() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Add);
        assert!(res.is_err());
        if let Err(Error::Type) = res {} else { assert!(false); }
    }

    #[test]
    fn test_eval_neg1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        assert!(s.eval(Op::Neg).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(-1));
    }

    #[test]
    fn test_eval_neg2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();
        assert!(s.eval(Op::Neg).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Bool(true));
    }

    #[test]
    fn test_eval_swap1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        assert!(s.eval(Op::Swap).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(1));
        assert_eq!(s.pop().unwrap(), Elt::Bool(false));

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res {} else { assert!(false); }
    }

    #[test]
    fn test_eval_swap2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Swap);
        assert!(res.is_err());
        if let Err(Error::Underflow) = res {} else { assert!(false); }
    }

    #[test]
    fn test_eval_rand1() {
        let mut s = Stack::new();
        let i = 20;
        s.push(Elt::Int(i)).unwrap();

        assert!(s.eval(Op::Rand).is_ok());

        let rand_val = s.pop().unwrap();
        assert!(rand_val >= Elt::Int(0));
        assert!(rand_val < Elt::Int(i));
    }

    #[test]
    fn test_eval_rand2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Rand);
        assert!(res.is_err());
        if let Err(Error::Type) = res {} else { assert!(false); }
    }

    #[test]
    fn test_eval_quit() {
        let mut s = Stack::new();

        let res = s.eval(Op::Quit);
        assert!(res.is_err());
        if let Err(Error::Quit) = res {} else { assert!(false); }
    }
}