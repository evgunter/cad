//! **INTENT stage 4 PR H — a placed copy is the construction its bundle
//! states** (`docs/INTENT-STAGE4-SPEC.md` §9; D10
//! `[placement.placement]`). The coincidence door reads what a mate
//! states by the mate, and replays the fold at `Sym` where a row no mate
//! states stands on a placed copy, so the fold's arithmetic must keep a
//! form the symbolic tier can read: no value computed apart from the
//! poses in a map, and no guard the tier cannot see through in a pose's
//! frame.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::PoseValue;
use editor_core::mate::mate_coset;
use geom_core::linalg::{Mat3, UnitVec3, Vec3};
use geom_core::{Band, Decide, ParamSymbol, Real, Sym, SymBudget, Tol};

type S = Sym<f64>;

fn band() -> Band {
    Band::linear(Tol::witness()).expect("a band")
}

/// The door's own budget.
const BUDGET: SymBudget = SymBudget {
    max_terms: 256,
    max_degree: 128,
};

fn sym(key: u64, value: f64) -> S {
    Sym::param(ParamSymbol::new(key), value)
}

/// A unit direction over three symbols of its own, `key..key + 3`, at
/// `at`.
fn direction(key: u64, at: [f64; 3]) -> UnitVec3<S> {
    let v = Vec3::new(sym(key, at[0]), sym(key + 1, at[1]), sym(key + 2, at[2]));
    UnitVec3::new(v, "s4h_direction", band()).expect("a direction")
}

/// Whether `x` decides Zero as a theorem.
fn identically_zero(x: S) -> bool {
    let decided = x.sign_within(band()).expect("decides");
    decided.sign == geom_core::Sign::Zero && decided.margin.proven()
}

/// Each defect of `m`'s orthonormality, by name, that the tier does not
/// prove zero.
fn unproven_orthonormality(m: Mat3<S>) -> Vec<&'static str> {
    let one = S::one();
    [
        ("c0·c0 − 1", m.c0.dot(m.c0) - one),
        ("c1·c1 − 1", m.c1.dot(m.c1) - one),
        ("c2·c2 − 1", m.c2.dot(m.c2) - one),
        ("c0·c1", m.c0.dot(m.c1)),
        ("c1·c2", m.c1.dot(m.c2)),
        ("c0·c2", m.c0.dot(m.c2)),
    ]
    .into_iter()
    .filter(|&(_, x)| !identically_zero(x))
    .map(|(what, _)| what)
    .collect()
}

/// **A pose's frame is orthonormal as a polynomial identity.** A plane
/// over a symbolic normal on either side of the equator (aimed from `z`
/// above it, from `−z` below), mated onto from the plane through the
/// origin normal to `z`: the coset's representative is the plane's own
/// frame, and the tier proves its columns orthonormal at the door's own
/// budget, so a seat read through it keeps a form the door can read.
/// Mated onto that plane instead, the representative is the frame's
/// inverse, which is orthonormal by the same identities.
///
/// Red if the frame's in-plane basis is chosen by a guard the tier
/// keeps as an atom (a `select` on `|z|` against `|x|`, a `max` of
/// norms), or if one pole's frame is a reflection or carries `z` other
/// than to the normal. The lower normal is aimed from `−z` and `z` from
/// `+z`, so the coset compares the two branches; a defect both branches
/// share cancels in a coset and moves nothing.
#[test]
fn a_poses_frame_is_orthonormal_as_a_polynomial_identity() {
    for (key, at) in [(10, [0.2, 0.1, 0.9]), (20, [0.1, -0.3, -0.8])] {
        let (unproven, _) = geom_core::sym::with_session(BUDGET, || {
            let plane = PoseValue::Plane {
                normal: direction(key, at),
                offset: sym(key + 5, 0.0),
            };
            let z = PoseValue::Plane {
                normal: UnitVec3::exact_z(),
                offset: S::zero(),
            };
            let frame = mate_coset(&z, &plane).representative.linear;
            let d = direction(key, at).get();
            let mut defects = unproven_orthonormality(frame);
            defects.extend(
                [
                    ("c2 − d (x)", frame.c2.x - d.x),
                    ("c2 − d (y)", frame.c2.y - d.y),
                    ("c2 − d (z)", frame.c2.z - d.z),
                    ("det − 1", frame.determinant() - S::one()),
                ]
                .into_iter()
                .filter(|&(_, x)| !identically_zero(x))
                .map(|(what, _)| what),
            );
            [
                defects,
                unproven_orthonormality(mate_coset(&plane, &z).representative.linear),
            ]
        });
        assert_eq!(
            unproven,
            [Vec::<&str>::new(), Vec::new()],
            "the frame over {at:?} (a rotation carrying z to the normal), and its inverse: \
             defects not proven"
        );
    }
}
