// Copyright (C) 2024 Ethan Uppal. All rights reserved.nn

use std::hash::Hash;

use crate::{dense_map::DenseInternedInfoMapKey, real::BoundedPrecisionReal};

pub type Num = BoundedPrecisionReal<16>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Var(pub usize);

impl From<usize> for Var {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct ExprIdx(usize);

impl DenseInternedInfoMapKey for ExprIdx {
    fn from(index: usize) -> Self {
        Self(index)
    }

    fn index(&self) -> usize {
        let Self(index) = self;
        *index
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Expr {
    Coeff(Num, Var),
    Sum(ExprIdx, ExprIdx),
}
