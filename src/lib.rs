mod clause;

pub use clause::{tseytin_transform, VariableStore, AST};

#[cfg(test)]
mod tests {
    use crate::{tseytin_transform, VariableStore, AST};
    use AST::*;
    use crate::clause::to_cnf;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_t_transform() {
        let mut store = VariableStore::new();
        println!("{:#?}", tseytin_transform(
            &(Implies(
                        Box::new(And(
                                Box::new(Or(Box::new(Var("p".into())), Box::new(Var("q".into())))),
                                Box::new(Var("r".into())))),
                        Box::new(Not(Box::new(Var("s".into()))))
            )), &mut store
        ))
    }

    #[test]
    fn test_cnf() {
        let mut store = VariableStore::new();
        let mut formula = tseytin_transform(
            &(Implies(
                Box::new(And(
                    Box::new(Or(Box::new(Var("p".into())), Box::new(Var("q".into())))),
                    Box::new(Var("r".into())))),
                Box::new(Not(Box::new(Var("s".into()))))
            )), &mut store
        );
        let cnf_formula : Vec<AST> = formula.iter().map(to_cnf).collect();
        println!("{:#?}", cnf_formula)
    }
}
