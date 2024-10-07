// Copyright (C) 2024 Ethan Uppal. All rights reserved.nn

#[derive(Default)]
pub struct Gen {
    id: usize,
}

impl Gen {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next<F: From<usize>>(&mut self) -> F {
        let next = self.id;
        self.id += 1;
        next.into()
    }
}
