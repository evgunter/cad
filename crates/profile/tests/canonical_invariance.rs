//! The canonical-form key property (D9-load-bearing): validation's
//! output is invariant — byte-level on `Debug` — under starting-vertex
//! rotation and traversal reversal of every input loop.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{annulus, bracket, circle_h, l_profile, lens, profile, rect, rounded_rect, tol};
use profile::{Profile, ProfileLoop, RawLoop};
use proptest::prelude::*;

/// Rotates a loop's starting vertex by `r` (a pure reindexing — the
/// same closed chain).
fn rotated(lp: &ProfileLoop<f64>, r: usize) -> ProfileLoop<f64> {
    let n = lp.vertices().len();
    ProfileLoop::new((0..n).map(|k| lp.vertices()[(r + k) % n]).collect())
        // Declared joints follow their vertex through the reindexing.
        .with_tangent_joints(
            lp.tangent_joints()
                .iter()
                .map(|&j| (j + n - r) % n)
                .collect(),
        )
}

/// Translates a loop rigidly (fixture plumbing).
fn translated(lp: &ProfileLoop<f64>, dx: f64, dy: f64) -> ProfileLoop<f64> {
    ProfileLoop::new(
        lp.vertices()
            .iter()
            .map(|v| {
                profile::ProfileVertex::new(
                    geom_core::Point2::new(v.pos().x + dx, v.pos().y + dy),
                    v.bulge(),
                )
            })
            .collect(),
    )
    .with_tangent_joints(lp.tangent_joints().to_vec())
}

/// The named fixture set (every accepting fixture with ≥ 1 loop).
fn fixtures() -> Vec<(&'static str, Profile<f64>)> {
    vec![
        ("rect", profile(vec![rect(0.0, 0.0, 2.0, 1.0)])),
        ("l_profile", profile(vec![l_profile()])),
        ("rounded_rect", profile(vec![rounded_rect(4.0, 3.0, 0.5)])),
        ("circle", profile(vec![circle_h(0.0, 0.0, 2.0)])),
        // Mixed declared/undeclared joints (2 tangent of 7): the
        // partial declaration set discriminates rotation/reversal
        // remapping bugs the fully-declared fixtures cannot.
        ("bracket", profile(vec![bracket()])),
        ("annulus", annulus()),
        ("lens", profile(vec![lens()])),
        (
            "plate_two_holes",
            profile(vec![
                rect(0.0, 0.0, 10.0, 4.0),
                circle_h(2.5, 2.0, 1.0),
                translated(&rounded_rect(4.0, 3.0, 0.5), 5.0, 0.5),
            ]),
        ),
    ]
}

proptest! {
    /// Rotating and/or reversing every loop of every fixture leaves the
    /// canonical form byte-identical.
    #[test]
    fn canonical_form_is_rotation_and_reversal_invariant(
        rot in prop::collection::vec(0usize..64, 3),
        rev in prop::collection::vec(any::<bool>(), 3),
    ) {
        for (name, base) in fixtures() {
            let canon = base.validate(tol()).expect(name);
            let transformed = Profile::new(
                base.plane,
                base.loops
                    .iter()
                    .enumerate()
                    .map(|(i, lp)| {
                        let r = rot[i % rot.len()] % lp.vertices().len();
                        let turned = rotated(lp, r);
                        if rev[i % rev.len()] {
                            turned.reversed()
                        } else {
                            turned
                        }
                    })
                    .collect(),
            );
            let canon2 = transformed.validate(tol()).expect(name);
            prop_assert_eq!(
                format!("{canon:?}"),
                format!("{canon2:?}"),
                "canonical form of {} not invariant",
                name
            );
        }
    }

    /// Reversal is a bit-exact involution on arbitrary chains (garbage
    /// included — this is pure reindexing plus exact negation).
    #[test]
    fn reversal_involution_on_random_chains(
        verts in prop::collection::vec(
            (-1.0e3..1.0e3f64, -1.0e3..1.0e3f64, -10.0..10.0f64),
            1..12,
        ),
    ) {
        let lp = common::chain(
            &verts.iter().map(|&(x, y, b)| (x, y, b)).collect::<Vec<_>>(),
        );
        let back = lp.reversed().reversed();
        for (a, b) in lp.vertices().iter().zip(back.vertices().iter()) {
            prop_assert_eq!(a.pos().x.to_bits(), b.pos().x.to_bits());
            prop_assert_eq!(a.pos().y.to_bits(), b.pos().y.to_bits());
            prop_assert_eq!(a.bulge().to_bits(), b.bulge().to_bits());
        }
    }
}
