/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/weighted-selector
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

pub struct WeightedSelectorIndex {
    weights: Vec<usize>,
}

impl WeightedSelectorIndex {
    pub const fn new(weights: Vec<usize>) -> Self {
        Self { weights }
    }

    pub fn select(&self, value: usize) -> Option<usize> {
        let mut cumulative = 0;

        for (i, &weight) in self.weights.iter().enumerate() {
            cumulative += weight;
            if value < cumulative {
                return Some(i);
            }
        }

        None
    }
}

pub struct WeightedSelector<T> {
    enums: Vec<(usize, T)>,
}

impl<T> WeightedSelector<T> {
    pub const fn new(enums: Vec<(usize, T)>) -> Self {
        Self { enums }
    }

    pub fn select(&self, value: usize) -> Option<&T> {
        let mut cumulative = 0;

        for (weight, enum_value) in &self.enums {
            cumulative += weight;
            if value < cumulative {
                return Some(enum_value);
            }
        }

        None
    }
}
