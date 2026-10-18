//! **INTENT stage 4 PR H — a placed copy is the construction its bundle
//! states** (`docs/INTENT-STAGE4-SPEC.md` §9; D10
//! `[placement.placement]`), so the coincidence door reads a copy's
//! carriers off the fold as the replay at `Sym` runs it. The fold's
//! arithmetic must keep a form the symbolic tier can read: no value
//! computed apart from the poses in a map, and no guard the tier
//! cannot see through in a pose's frame.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::PoseValue;
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
/// norms), or if one pole's frame is a reflection or carries `z` other
/// than to the normal. The lower normal is aimed from `−z` and `z` from
/// `+z`, so the coset compares the two branches; a defect both branches
/// share cancels in a coset and moves nothing.
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

/// **Measurement, not a gate**: whether the identities a mate-placed
/// seat stands on prove at a budget, for cap normals built as an
/// extrude builds them (`normalize(u × v)` over a frame's six component
/// symbols). It prints, per identity, whether the tier proves it; the
/// seat item (`work/intent/a-mate-placed-seat-is-unproven-at-the-doors-budget.md`)
/// records what it printed and is the reason it exists. Run with
/// `S4H_TERMS=<n> cargo nextest run -p editor-core --test all
/// measure_a_mated_seat_at_sym --run-ignored all --no-capture`
/// (256, the door's budget, by default).
#[test]
#[ignore = "measurement: prints which seat identities prove at S4H_TERMS"]
fn measure_a_mated_seat_at_sym() {
    let terms = std::env::var("S4H_TERMS")
        .ok()
        .and_then(|t| t.parse().ok())
        .unwrap_or(BUDGET.max_terms);
    let budget = SymBudget {
        max_terms: terms,
        ..BUDGET
    };
    let (lines, _) = geom_core::sym::with_session(budget, || {
        let cap = |key: u64, flip: bool| {
            let u = Vec3::new(sym(key, 1.0), sym(key + 1, 0.0), sym(key + 2, 0.0));
            let v = Vec3::new(sym(key + 3, 0.0), sym(key + 4, 1.0), sym(key + 5, 0.0));
            let n = u.cross(v).normalize();
            if flip { -n } else { n }
        };
        let unit = |v: Vec3<S>| UnitVec3::new(v, "s4h_direction", band()).expect("a direction");
        let (n_on, n_to) = (cap(100, true), cap(200, false));
        let at_origin = |n| PoseValue::Plane {
            origin: Point3::origin(),
            normal: n,
        };
        let z = at_origin(UnitVec3::exact_z());
        let (on, to) = (at_origin(unit(n_on)), at_origin(-unit(n_to)));
        let undo_on = mate_coset(&on, &z).representative.linear;
        let take_to = mate_coset(&z, &to).representative.linear;
        let composed = mate_coset(&on, &to).representative.linear;
        let chained = take_to * (undo_on * n_on);
        let turn = Mat3::rotation_about(n_to, sym(300, 0.3));
        let want = -n_to;
        vec![
            (
                "the copy's own frame undone: F_onᵀ·n_on − z (x)",
                identically_zero((undo_on * n_on).x),
            ),
            (
                "chained: F_to·(F_onᵀ·n_on) + n_to (x)",
                identically_zero(chained.x - want.x),
            ),
            (
                "composed: (F_to·F_onᵀ)·n_on + n_to (x)",
                identically_zero((composed * n_on).x - want.x),
            ),
            (
                "the value's turn: R(n_to, θ)·n_to − n_to (x)",
                identically_zero((turn * n_to).x - n_to.x),
            ),
        ]
    });
    for (what, proven) in lines {
        eprintln!("at {terms} terms: {what}: proven {proven}");
    }
}
