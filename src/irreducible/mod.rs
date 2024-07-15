pub mod utils;
mod conversions;

use std::rc::Rc;

///represents a type that is only composed of integers
///if an operation would result in a fractional, instead it will transform this type to either a division (dividend divisor pair) or a root (ex: 2f32.sqrt() -> root(int, 2))
pub enum Irreducible {
    Integer(i128),
    Division { dividend: Rc<Irreducible>, divisor: Rc<Irreducible> },
    Root { negative: bool, value: Rc<Irreducible>, power: Rc<Irreducible> },
    Complex { real: Rc<Irreducible>, imaginary: Rc<Irreducible> },
    Combination(Vec<Irreducible>), //this irreducible is composed of multiple irreducible summed together (ex top part of a division that is the sum of an int and a sqrt)
}