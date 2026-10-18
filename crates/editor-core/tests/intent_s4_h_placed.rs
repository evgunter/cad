//! **INTENT stage 4 PR H — a placed copy is the construction its bundle
//! states** (`docs/INTENT-STAGE4-SPEC.md` §9; D10
//! `[placement.placement]`), so the coincidence door reads a copy's
//! carriers off the fold as the replay at `Sym` runs it. The fold's
//! arithmetic must keep a form the symbolic tier can read: no value
//! computed apart from the poses in a map, and no guard the tier
//! cannot see through in a pose's frame.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::PoseValue;
use editor_core::mate::coset::{Arm, Subgroup, intersect_subgroups};
use editor_core::mate::mate_coset;
use geom_core::linalg::{Mat3, Point3, UnitVec3, Vec3};
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
/// norms).
#[test]
fn a_poses_frame_is_orthonormal_as_a_polynomial_identity() {
    for (key, at) in [(10, [0.2, 0.1, 0.9]), (20, [0.1, -0.3, -0.8])] {
        let (unproven, _) = geom_core::sym::with_session(BUDGET, || {
            let plane = PoseValue::Plane {
                origin: Point3::new(sym(key + 5, 0.0), sym(key + 6, 0.0), sym(key + 7, 0.0)),
                normal: direction(key, at),
            };
            let z = PoseValue::Plane {
                origin: Point3::origin(),
                normal: UnitVec3::exact_z(),
            };
            [
                unproven_orthonormality(mate_coset(&z, &plane).representative.linear),
                unproven_orthonormality(mate_coset(&plane, &z).representative.linear),
            ]
        });
        assert_eq!(
            unproven,
            [Vec::<&str>::new(), Vec::new()],
            "the frame over {at:?}, and its inverse: defects not proven"
        );
    }
}

/// **The line two planes meet in carries no lever arm.** Two planar
/// residuals over symbolic normals meet in a prismatic one; its
/// direction is the normalised cross product of the two normals and
/// nothing else, so a fold's map built along it is a function of its
/// poses alone, and the arm stays inside the decision it levers.
///
/// Red if the line is minted from the cross product levered by the arm
/// (opaque at `Sym`): the difference then keeps the arm's atom.
#[test]
fn the_line_two_planes_meet_in_carries_no_lever_arm() {
    let (residual, _) = geom_core::sym::with_session(BUDGET, || {
        let (n1, n2) = (
            direction(30, [0.0, 0.1, 1.0]),
            direction(40, [1.0, 0.2, 0.1]),
        );
        let arm = Arm::of(1.0, 2.0).expect("an arm in range");
        let Ok(Subgroup::Prismatic { direction }) = intersect_subgroups(
            Subgroup::Planar { normal: n1 },
            Subgroup::Planar { normal: n2 },
            band(),
            arm,
        ) else {
            panic!("two planes at a definite angle meet in a line");
        };
        let line = n1.get().cross(n2.get()).normalize();
        let d = direction.get() - line;
        [d.x, d.y, d.z].map(identically_zero)
    });
    assert_eq!(
        residual, [true; 3],
        "the line is the normals' cross product normalised"
    );
}
