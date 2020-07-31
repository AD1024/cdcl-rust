use crate::model::Model;
use crate::AST;
use crate::solver::SolverStatus::*;

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

impl Solver {
    fn new(formula : &Vec<AST>) -> Self {
        Solver {
            status : IDLE,
            model : Model::new(),
            formula : formula.clone()
        }
    }

    fn unite_propagation(&mut self) {

    }
}