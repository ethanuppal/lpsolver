// Copyright (C) 2024 Ethan Uppal. All rights reserved.nn

use crate::{
    dense_map::DenseInternedInfoMap,
    expr::{Expr, ExprIdx, Num},
    gen::Gen,
};

pub enum VarType {
    Nonnegative,
    Real,
}

#[derive(Default)]
pub struct Context {
    exprs: DenseInternedInfoMap<ExprIdx, Expr, String>,
    vars: Vec<ExprIdx>,
    var_gen: Gen,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn var<S: AsRef<str>>(&mut self, name: S, ty: VarType) -> ExprIdx {
        let idx = match ty {
            VarType::Nonnegative => self.exprs.add_with_info(
                Expr::Coeff(Num(1.0), self.var_gen.next()),
                name.as_ref().to_string(),
            ),
            VarType::Real => {
                let pos = self.exprs.add(Expr::Coeff(Num(1.0), self.var_gen.next()));
                let neg = self.exprs.add(Expr::Coeff(Num(-1.0), self.var_gen.next()));
                self.exprs
                    .add_with_info(Expr::Sum(pos, neg), name.as_ref().to_string())
            }
        };
        self.vars.push(idx);
        idx
    }
}
