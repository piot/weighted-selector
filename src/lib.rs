/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/weighted-selector
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

pub struct WeightedSelectorIndex {
    weights: Vec<usize>,
    total_size: usize,
}

impl WeightedSelectorIndex {
    pub fn new(weights: Vec<usize>) -> Self {
        let total_size = weights.iter().sum();
        Self {
            weights,
            total_size,
        }
    }

    pub fn total(&self) -> usize {
        self.total_size
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
    total_size: usize,
}

impl<T> WeightedSelector<T> {
    pub fn new(enums: Vec<(usize, T)>) -> Self {
        let total_size = enums.iter().map(|(weight, _)| weight).sum();
        Self { enums, total_size }
    }

    #[inline]
    pub fn total(&self) -> usize {
        self.total_size
    }

    #[inline]
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
