//! **Where a plane normal that cannot certify is stopped.**
//!
//! `ssi::certify`'s chart tube reads the three components of a
//! `Surface::Plane` normal into the C9 ring, and that crossing now takes
//! the certified door (`certify.rs`'s `normal_crossing_tests`). This
//! suite asks the other half of the question: can a normal that fails
//! `CertifiedEnclosure` be *minted* by the kernel in the first place?
//!
//! `newell_plane` is the kernel's one derived-plane mint at a generic
//! scalar, and the answer there is no — but by a **guard, not a type**.
//! The mint normalizes an unconstrained cross-sum and then certifies
//! every vertex against the result, and a decoration that cannot certify
//! poisons that decision rather than the endpoints, so the mint refuses
//! with `Escalated`. The row below is what makes that a fact instead of
//! a reading: nothing in the signature says it, and a future mint that
//! skips the residual pass would hand `certify` exactly the operand its
//! own crossing now refuses.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::interval::iv;
use geom_brep::{NewellError, newell_plane};
use geom_core::predicate::Band;
use geom_core::{CertifiedEnclosure, Interval, Point3, Real};

/// A domain violation with a finite, strictly positive bracket:
/// `sqrt([−1, 4]) + 1` is `[1, 3]` at decoration `Trv`. Endpoints alone
/// cannot tell it from a healthy `[1, 3]`.
fn trv_pos() -> Interval {
    Interval::from_bounds(-1.0, 4.0).sqrt() + iv(1.0)
}

/// **Deliberately not `shared::tol::band`.** These rows measure the
/// mint's own arithmetic against a FIXED coincidence scale, so the
/// band must not follow the ε the run drew — a row about the mint
/// would otherwise become a row about the matrix point it ran at.
fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

/// A planar triangle in `z = 0` whose first vertex carries `c` in `x`.
fn loop_with(c: Interval) -> [Point3<Interval>; 3] {
    [
        Point3::new(c, iv(0.0), iv(0.0)),
        Point3::new(iv(4.0), iv(0.0), iv(0.0)),
        Point3::new(iv(2.0), iv(3.0), iv(0.0)),
    ]
}

/// The control: the same loop with a certified coordinate mints a plane,
/// so the refusal below is the decoration's doing and not the fixture's
/// geometry.
#[test]
fn a_certified_loop_mints_its_plane() {
    let healthy = Interval::from_bounds(1.0, 1.0).sqrt();
    assert_eq!(healthy.certified_bracket(), Some((1.0, 1.0)));
    newell_plane(&loop_with(healthy), band()).expect("a certified planar loop must mint");
}

#[test]
fn a_violated_vertex_cannot_mint_a_plane() {
    assert!(
        trv_pos().certified_bracket().is_none(),
        "fixture drifted: it certifies"
    );
    match newell_plane(&loop_with(trv_pos()), band()) {
        Err(NewellError::Escalated { .. }) => {}
        Err(other) => panic!("refused, but not by the residual pass: {other}"),
        Ok(_) => panic!(
            "the kernel's derived-plane mint emitted a plane from a \
             domain-violated vertex — its normal would reach the SSI \
             chart tube's ring crossing"
        ),
    }
}
