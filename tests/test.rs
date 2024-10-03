/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/weighted-selector
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use weighted_selector::{WeightedSelector, WeightedSelectorIndex};

#[test]
fn weight() {
    let x = WeightedSelectorIndex::new([10, 5, 3].into());

    assert_eq!(x.total(), 18);
    assert_eq!(x.select(1), Some(0));
    assert_eq!(x.select(9), Some(0));
    assert_eq!(x.select(17), Some(2));
    assert_eq!(x.select(18), None);
}

#[derive(Debug, PartialEq)]
enum TestEnum {
    First,
    Second,
    Third,
}

#[test]
fn weight_enum() {
    let x = WeightedSelector::new(
        [
            (10, TestEnum::First),
            (33, TestEnum::Second),
            (4, TestEnum::Third),
        ]
        .into(),
    );

    assert_eq!(x.total(), 47);
    assert_eq!(x.select(1), Some(&TestEnum::First));
    assert_eq!(x.select(9), Some(&TestEnum::First));
    assert_eq!(x.select(10), Some(&TestEnum::Second));
    assert_eq!(x.select(42), Some(&TestEnum::Second));
    assert_eq!(x.select(44), Some(&TestEnum::Third));
    assert_eq!(x.select(47), None);
}
