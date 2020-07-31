use crate::clause::AST::{Var, Iff};
use std::fmt::{Debug, Formatter, Display};

pub struct Variable {
    neg : bool,
    name : String
}

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
        AST::Var(_) => Vec::new(),
        AST::Not(e) => {
            let new_var = store.next_variable();
            let mut result =
                vec![Iff(Box::new(Var(new_var)), Box::new((*expr).clone()))];
            if let AST::Var(_) = **e {
                result
            } else {
                result.append(&mut tseytin_transform(e, store));
                result
            }
        },
        AST::Or(e1, e2) |
        AST::Iff(e1, e2) |
        AST::Implies(e1, e2) |
        AST::And(e1, e2) => {
            let new_var = store.next_variable();
            let mut result =
                vec![Iff(Box::new(Var(new_var)), Box::new((*expr).clone()))];
            result.append(&mut tseytin_transform(e1.as_ref(), store));
            result.append(&mut tseytin_transform(e2.as_ref(), store));
            result
        }
    }
}

pub fn to_cnf(expr : &AST) -> AST {
    let f = |e : &BoxAST| Box::new(to_cnf(e.as_ref()));
    match expr {
        AST::Var(_) => expr.clone(),
        AST::Not(e) => AST::Not(f(e)),
        AST::And(e1, e2) => AST::And(f(e1), f(e2)),
        AST::Or(e1, e2)  => AST::Or(f(e1), f(e2)),
        AST::Implies(e1, e2) => AST::Or(Box::new(AST::Not(f(e1))), f(e2)),
        AST::Iff(e1, e2) =>
            AST::And(
                Box::new(to_cnf(&(AST::Implies(e1.clone(), e2.clone())))),
                Box::new(to_cnf(&(AST::Implies(e2.clone(), e1.clone())))))
    }
}