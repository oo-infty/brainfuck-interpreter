#![allow(unused)]

#[derive(Clone)]
pub struct Config {
    pub len: usize,
    pub addr: Addr,
    pub cell: Cell,
    pub overflow: Overflow,
    pub eof: Eof,
}

#[derive(Clone)]
pub enum Addr {
    Unsigned,
    Signed,
}

#[derive(Clone)]
pub enum Cell {
    I8,
    I32,
}

#[derive(Clone)]
pub enum Overflow {
    Error,
    Wrap,
}

#[derive(Clone)]
pub enum Eof {
    Zero,
    Keep,
    Ignore,
}