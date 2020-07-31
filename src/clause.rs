use crate::clause::AST::*;
use std::fmt::{Debug, Formatter, Display};

#[derive(Clone)]
#[derive(Debug)]
pub enum AST {
    Var(String),
    Not(BoxAST),
    And(BoxAST, BoxAST),
    Or(BoxAST, BoxAST),
    Implies(BoxAST, BoxAST),
    Iff(BoxAST, BoxAST)
}

type BoxAST = Box<AST>;

pub struct VariableStore {
    count : i32
}

impl VariableStore {
    pub fn new() -> Self {
        VariableStore { count: 0 }
    }

    fn next_variable(&mut self) -> String {
        let result = format!("$x_{}", self.count);
        self.count += 1;
        result
    }
}

pub fn tseytin_transform(expr : &AST, store : &mut VariableStore) -> Vec<AST> {
    match expr {
        Var(_) => Vec::new(),
        Not(e) => {
            let new_var = store.next_variable();
            let mut result =
                vec![Iff(Box::new(Var(new_var)), Box::new((*expr).clone()))];
            if let Var(_) = **e {
                result
            } else {
                result.append(&mut tseytin_transform(e, store));
                result
            }
        },
        Or(e1, e2) |
        Iff(e1, e2) |
        Implies(e1, e2) |
        And(e1, e2) => {
            let new_var = store.next_variable();
            let mut result =
                vec![Iff(Box::new(Var(new_var)), Box::new((*expr).clone()))];
            result.append(&mut tseytin_transform(e1.as_ref(), store));
            result.append(&mut tseytin_transform(e2.as_ref(), store));
            result
        }
    }
}

pub fn push_neg(expr : &AST) -> AST {
    let f = |e : &BoxAST| Box::new(push_neg(e.as_ref()));
    match expr {
        Not(e) => {
            match *(e.to_owned()) {
                And(e1, e2) =>
                    Or(
                        Box::new(push_neg(&Not(e1))),
                        Box::new(push_neg(&Not(e2)))
                    ),
                Or(e1, e2) =>
                    And(
                        Box::new(push_neg(&Not(e1))),
                        Box::new(push_neg(&Not(e2)))
                    ),
                _ => panic!("{:#?} should not be here!", e)
            }
        },
        And(e1, e2) => And(f(e1), f(e2)),
        Or(e1, e2)  => Or(f(e1), f(e2)),
        _ => panic!("{:#?} should not be here!", expr)
    }
}

pub fn to_cnf(expr : &AST) -> AST {
    let f = |e : &BoxAST| Box::new(to_cnf(e.as_ref()));
    match expr {
        Var(_) => expr.clone(),
        Not(e) => AST::Not(f(e)),
        And(e1, e2) => And(f(e1), f(e2)),
        Or(e1, e2)  => Or(f(e1), f(e2)),
        Implies(e1, e2) => Or(Box::new(Not(f(e1))), f(e2)),
        Iff(e1, e2) =>
            And(
                Box::new(to_cnf(&(Implies(e1.clone(), e2.clone())))),
                Box::new(to_cnf(&(Implies(e2.clone(), e1.clone())))))
    }
}

pub fn vec_to_cnf(exprs : &Vec<AST>) -> Vec<AST> {
    exprs.iter().map(to_cnf).collect()
}

pub fn break_and(exprs : &Vec<AST>) -> Vec<AST> {
    let mut result : Vec<AST> = Vec::new();
    for formula in exprs.iter() {
        match formula.to_owned() {
            AST::And(e1, e2) => {
                result.push(*e1);
                result.push(*e2);
            },
            _ => result.push(formula.to_owned())
        }
    }
    result
}