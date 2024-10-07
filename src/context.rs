use crate::{
    dense_map::DenseInternedInfoMap,
    expr::{Expr, ExprIdx},
    gen::Gen,
    real::Real,
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
                Expr::Coeff(Real(1.0), self.var_gen.next()),
                name.as_ref().to_string(),
            ),
            VarType::Real => {
                let pos = self.exprs.add(Expr::Coeff(Real(1.0), self.var_gen.next()));
                let neg = self.exprs.add(Expr::Coeff(Real(-1.0), self.var_gen.next()));
                self.exprs
                    .add_with_info(Expr::Sum(pos, neg), name.as_ref().to_string())
            }
        };
        self.vars.push(idx);
        idx
    }
}
