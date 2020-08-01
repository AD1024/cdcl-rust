use crate::model::Model;
use crate::AST::{self, *};
use crate::solver::SolverStatus::*;
use std::collections::HashMap;

#[derive(Eq, PartialEq)]
pub enum SolverStatus {
    TIMEOUT,
    SAT,
    UNSAT,
    IDLE
}

pub struct Solver {
    status : SolverStatus,
    model : Model,
    formula : Vec<AST>
}

struct VarPos {
    pub pos : usize,
    pub is_neg : bool
}

impl VarPos {
    pub fn new(pos : usize, is_neg : bool) -> Self {
        VarPos { pos, is_neg }
    }
}

impl Solver {
    pub fn new(formula : &Vec<AST>) -> Self {
        Solver {
            status : IDLE,
            model : Model::new(),
            formula : formula.clone()
        }
    }

    fn traverse_formula<'a>(&self, index : usize, expr : &'a AST, is_not : bool,
                        pos_map: &mut HashMap<&'a str, Vec<VarPos>>) {
        match expr {
            Var(name) => {
                if let Some(mut v) = pos_map.get_mut(name.as_str()) {
                    v.push(VarPos::new(index, is_not))
                } else {
                    pos_map.insert(name.as_str(), Vec::new());
                    pos_map.get_mut(name.as_str())
                            .unwrap()
                            .push(VarPos::new(index, is_not));
                }
            },
            Or(e1, e2) => {
                self.traverse_formula(index, e1.as_ref(), is_not, pos_map);
                self.traverse_formula(index, e2.as_ref(), is_not, pos_map);
            },
            Not(e) => {
                self.traverse_formula(index, e.as_ref(), !is_not, pos_map);
            },
            _ => panic!("{:#?} should not appear here!", expr)
        }
    }

    pub fn unite_propagation(&mut self) {
        if self.status != TIMEOUT {
            let mut appearance: HashMap<&str, Vec<VarPos>> = HashMap::new();
            for (i, expr) in self.formula.iter().enumerate() {
                self.traverse_formula(i, expr, false, &mut appearance);
            }
        }
    }
}