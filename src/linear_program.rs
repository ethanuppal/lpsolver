// Copyright (C) 2024 Ethan Uppal. All rights reserved.nn

use crate::expr::Num;

pub struct Constraint {
    coefficients: Vec<Num>,
    bound: Num,
}

pub struct LinearProgram {
    objective: Vec<Num>,
    constraints: Vec<Constraint>,
}
