#![allow(clippy::unwrap_used)]

use super::RowSpace;
use geom_core::spline::KnotVector;

fn kv(knots: &[f64]) -> KnotVector {
    KnotVector::clamped(knots.to_vec(), 1).unwrap()
}

/// The row run back is any EXACT reflection of the row: on
/// `[0.1, 0.3]` the knots `0.25` and `0.15` sum to `0.1 + 0.3` in ℝ
/// while `fl(0.1 + 0.3 − 0.25)` is not `0.15`; on `[0, 1]`,
/// `fl(1 − 0.1)` is `0.9` while `0.1 + 0.9` is not 1; the reflection
/// through 0, `−k`, never rounds, while one about 0.7 rebuilt as
/// `fl(0.7 − k)` does. Forward is the row's own knots and nothing
/// else: a row symmetric only in decimal is not run back on them.
#[test]
fn each_direction_admits_its_own_relation() {
    let row = kv(&[0.1, 0.1, 0.25, 0.3, 0.3]);
    let only_back = RowSpace {
        forward: false,
        backward: true,
    };
    assert_eq!(
        RowSpace::of(&kv(&[0.1, 0.1, 0.15, 0.3, 0.3]), &row),
        only_back,
        "an exact mirror the rounded reflection misses runs the row back"
    );
    assert_eq!(
        RowSpace::of(&row.negated(), &row),
        only_back,
        "the row reflected through 0, which rounds nowhere, runs it back"
    );
    let about_0_7: Vec<f64> = row.knots().iter().rev().map(|k| 0.7 - k).collect();
    assert_eq!(
        RowSpace::of(&kv(&about_0_7), &row),
        RowSpace {
            forward: false,
            backward: false
        },
        "a reflection about 0.7 that holds only after rounding is refused"
    );
    assert_eq!(
        RowSpace::of(
            &kv(&[0.0, 0.0, 0.9, 1.0, 1.0]),
            &kv(&[0.0, 0.0, 0.1, 1.0, 1.0])
        ),
        RowSpace {
            forward: false,
            backward: false
        },
        "a vector the rounded reflection reproduces, a 2Sum residual off the exact \
             one, is in neither direction"
    );
    let decimal = kv(&[0.1, 0.1, 0.2, 0.3, 0.3]);
    assert_eq!(
        RowSpace::of(&decimal, &decimal),
        RowSpace {
            forward: true,
            backward: false
        },
        "a row symmetric only in decimal runs forward on its own knots, not back"
    );
    let dyadic = kv(&[0.0, 0.0, 0.5, 1.0, 1.0]);
    assert_eq!(
        RowSpace::of(&dyadic, &dyadic),
        RowSpace {
            forward: true,
            backward: true
        },
        "an exactly symmetric row runs either way on its own knots"
    );
}
