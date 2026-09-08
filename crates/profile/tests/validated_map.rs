//! **`ValidatedProfile::map` is total over the canonical form and
//! decides nothing.** Over the crate's own fixtures — a hole, arcs,
//! tangent joints, a fillet, the two-vertex lens, an outer loop the
//! author wound the wrong way, a rotated start — the lift under
//! `from_f64` answers every canonical-form accessor exactly as the
//! `f64` form does (outer first, holes in input order, the segment
//! kinds and turns, the joint set, the blend arcs, the canonical start
//! and winding) and carries, scalar for scalar, the value a validation
//! of the lifted RAW profile mints at the target scalar: the same bits
//! in every value channel (`Dual64`'s value, `Interval`'s bounds — the
//! re-derived arc carriers included), and a derivative channel that is
//! zero either way. `with_plane` replaces the plane and nothing else.
//! The `Interval` twin runs in the interval lane.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{annulus, bracket, chain, l_profile, lens, lift, profile, rounded_rect, tol};
use geom_core::{Affine3, Decide, Dual64, Point2, Real, Sign, Vec3};
use profile::{LoopRole, Profile, RawLoop, SegmentKind, SketchPlane, ValidatedProfile};

/// The fixtures, named: every canonical-form fact the door carries has
/// an instance here.
fn fixtures() -> Vec<(&'static str, Profile<f64>)> {
    // The rectangle wound clockwise: validation REVERSES it (outer
    // loops run counterclockwise), so the carried traversal sense is a
    // decision, not a copy of the input.
    let clockwise = chain(&[
        (0.0, 0.0, 0.0),
        (0.0, 1.0, 0.0),
        (3.0, 1.0, 0.0),
        (3.0, 0.0, 0.0),
    ]);
    // A rotated start: canonicalization starts each loop at its
    // lex-min vertex, so the carried start is a decision too.
    let rotated = chain(&[
        (2.0, 0.0, 0.0),
        (2.0, 1.0, 0.0),
        (0.0, 1.0, 0.0),
        (0.0, 0.0, 0.0),
    ]);
    vec![
        ("annulus (a hole, all arcs)", annulus()),
        (
            "rounded rectangle (every joint tangent)",
            profile(vec![rounded_rect(4.0, 2.0, 0.5)]),
        ),
        (
            "bracket (one fillet, mixed joints)",
            profile(vec![bracket()]),
        ),
        ("lens (two vertices, two arcs)", profile(vec![lens()])),
        ("L (concave polygon)", profile(vec![l_profile()])),
        (
            "clockwise rectangle (reversed by validation)",
            profile(vec![clockwise]),
        ),
        (
            "rotated rectangle (restarted by validation)",
            profile(vec![rotated]),
        ),
        (
            "L with a rounded hole (holes in input order)",
            profile(vec![
                chain(&[
                    (0.0, 0.0, 0.0),
                    (6.0, 0.0, 0.0),
                    (6.0, 6.0, 0.0),
                    (0.0, 6.0, 0.0),
                ]),
                rounded_hole(1.0, 1.0, 2.0, 1.0, 0.25),
                chain(&[
                    (3.0, 3.0, 0.0),
                    (4.0, 3.0, 0.0),
                    (4.0, 4.0, 0.0),
                    (3.0, 4.0, 0.0),
                ]),
            ]),
        ),
    ]
}

/// A rounded rectangle with its corner at (x0, y0): straight sides,
/// counterclockwise quarter-arc corners of radius `r`, every joint
/// declared tangent.
fn rounded_hole(x0: f64, y0: f64, w: f64, h: f64, r: f64) -> profile::ProfileLoop<f64> {
    let b = common::quarter_bulge();
    chain(&[
        (x0 + r, y0, 0.0),
        (x0 + w - r, y0, b),
        (x0 + w, y0 + r, 0.0),
        (x0 + w, y0 + h - r, b),
        (x0 + w - r, y0 + h, 0.0),
        (x0 + r, y0 + h, b),
        (x0, y0 + h - r, 0.0),
        (x0, y0 + r, b),
    ])
    .with_tangent_joints((0..8).collect())
}

/// Every scalar a validated profile stores, in one fixed order: the
/// plane's placement, then per loop each vertex's position and bulge,
/// then each segment's endpoints, bulge and (for an arc) center and
/// radius.
fn scalars<T: Real>(vp: &ValidatedProfile<T>) -> Vec<T> {
    let m = &vp.plane().placement;
    let mut out = vec![
        m.linear.c0.x,
        m.linear.c0.y,
        m.linear.c0.z,
        m.linear.c1.x,
        m.linear.c1.y,
        m.linear.c1.z,
        m.linear.c2.x,
        m.linear.c2.y,
        m.linear.c2.z,
        m.translation.x,
        m.translation.y,
        m.translation.z,
    ];
    for lp in vp.loops() {
        for v in lp.vertices() {
            out.extend([v.pos().x, v.pos().y, v.bulge()]);
        }
        for s in lp.segments() {
            out.extend([s.start.x, s.start.y, s.end.x, s.end.y, s.bulge]);
            if let SegmentKind::Arc { center, radius, .. } = s.kind {
                out.extend([center.x, center.y, radius]);
            }
        }
    }
    out
}

/// One value channel of a scalar, named, projected to `f64` for a bit
/// comparison.
type Channel<T> = (&'static str, fn(T) -> f64);

/// The lifted form against the form validation mints at `U` from the
/// lifted raw profile: every value channel the same bits (`channels`
/// projects each of them to `f64`), and every canonical-form accessor
/// answering as the `f64` form does.
fn lift_equals_revalidation<U: Real + Decide>(scalar: &str, channels: &[Channel<U>]) {
    for (name, raw) in fixtures() {
        let at_f64 = raw.validate(tol()).expect(name);
        let lifted: ValidatedProfile<U> = at_f64.clone().map(U::from_f64);
        let revalidated: ValidatedProfile<U> = lift::<U>(&raw).validate(tol()).expect(name);
        let (l, r) = (scalars(&lifted), scalars(&revalidated));
        assert_eq!(
            l.len(),
            r.len(),
            "{name} at {scalar}: the same scalar count"
        );
        for (channel, project) in channels {
            let bits = |xs: &[U]| xs.iter().map(|&x| project(x).to_bits()).collect::<Vec<_>>();
            assert!(
                bits(&l) == bits(&r),
                "{name} at {scalar}: the lift's {channel} channel differs from validating the \
                 lifted raw profile:\n lifted {l:?}\n revalidated {r:?}"
            );
        }
        // The structural claims, read through the accessors against
        // the f64 form's answers.
        assert_eq!(
            lifted.loops().len(),
            at_f64.loops().len(),
            "{name}: loop count"
        );
        for (li, (lu, lf)) in lifted.loops().iter().zip(at_f64.loops()).enumerate() {
            assert_eq!(lu.role(), lf.role(), "{name} loop {li}: role");
            assert_eq!(
                lu.tangent_joints(),
                lf.tangent_joints(),
                "{name} loop {li}: joints"
            );
            assert_eq!(
                lu.vertices().len(),
                lf.vertices().len(),
                "{name} loop {li}: arity"
            );
            assert_eq!(
                lu.segments().len(),
                lf.segments().len(),
                "{name} loop {li}: chain"
            );
            assert_eq!(
                lu.blend_arcs().len(),
                lf.blend_arcs().len(),
                "{name} loop {li}: blend arcs"
            );
            for (k, (su, sf)) in lu.segments().iter().zip(lf.segments()).enumerate() {
                match (su.kind, sf.kind) {
                    (SegmentKind::Line, SegmentKind::Line) => {}
                    (SegmentKind::Arc { turn: tu, .. }, SegmentKind::Arc { turn: tf, .. }) => {
                        assert_eq!(tu, tf, "{name} loop {li} segment {k}: turn");
                    }
                    (u, f) => panic!("{name} loop {li} segment {k}: kind {u:?} vs {f:?}"),
                }
            }
        }
    }
}

/// At `f64` the lift is the identity, bit for bit — the re-derived
/// carriers included.
#[test]
fn the_lift_to_f64_is_the_identity() {
    lift_equals_revalidation::<f64>("f64", &[("value", |x| x)]);
    for (name, raw) in fixtures() {
        let at_f64 = raw.validate(tol()).expect(name);
        let bits = |vp: &ValidatedProfile<f64>| {
            scalars(vp).iter().map(|x| x.to_bits()).collect::<Vec<_>>()
        };
        assert_eq!(bits(&at_f64.clone().map(|x| x)), bits(&at_f64), "{name}");
    }
}

/// At `Dual64` the value channel is the `f64` form's bits and the
/// derivative channel is zero — in the lift by `from_f64`'s contract,
/// in the re-validation because a constant's arithmetic has none (the
/// sign of that zero is the one bit the two can differ in, a negated
/// constant's derivative being `-0.0`; the door's doc states it).
#[test]
fn the_lift_to_dual_equals_validating_at_dual() {
    lift_equals_revalidation::<Dual64>("Dual64", &[("value", |d| d.value)]);
    for (name, raw) in fixtures() {
        let at_f64 = raw.validate(tol()).expect(name);
        let lifted = at_f64.clone().map(Dual64::from_f64);
        let revalidated = lift::<Dual64>(&raw).validate(tol()).expect(name);
        assert!(
            scalars(&lifted).iter().all(|d| d.deriv == 0.0)
                && scalars(&revalidated).iter().all(|d| d.deriv == 0.0),
            "{name}: a constant profile's derivative channel is zero"
        );
        assert_eq!(
            scalars(&lifted)
                .iter()
                .map(|d| d.value.to_bits())
                .collect::<Vec<_>>(),
            scalars(&at_f64)
                .iter()
                .map(|x| x.to_bits())
                .collect::<Vec<_>>(),
            "{name}: the value channel is the f64 form"
        );
    }
}

#[cfg(feature = "interval")]
#[test]
fn the_lift_to_interval_equals_validating_at_interval() {
    use geom_core::Bounds;
    lift_equals_revalidation::<geom_core::Interval>(
        "Interval",
        &[("lo", |i| i.lo()), ("hi", |i| i.hi())],
    );
}

/// The decided facts, read at `Dual64` on the fixtures whose input
/// contradicts them: the clockwise rectangle comes back
/// counterclockwise, the rotated one starts at its lex-min vertex, the
/// annulus's hole arcs turn clockwise behind its outer.
#[test]
fn the_carried_decisions_are_the_f64_ones() {
    // Twice the signed area of a polygonal loop (every segment a
    // line); an arc loop's winding is read from its turns instead.
    let shoelace = |vs: &[profile::ProfileVertex<Dual64>]| -> f64 {
        let n = vs.len();
        (0..n)
            .map(|i| {
                let (a, b) = (vs[i].pos(), vs[(i + 1) % n].pos());
                a.x.value * b.y.value - b.x.value * a.y.value
            })
            .sum()
    };
    for (name, raw) in fixtures() {
        let at_f64 = raw.validate(tol()).expect(name);
        let lifted = at_f64.clone().map(Dual64::from_f64);
        assert_eq!(
            lifted.loops()[0].role(),
            LoopRole::Outer,
            "{name}: outer first"
        );
        for (li, lu) in lifted.loops().iter().enumerate() {
            if lu
                .segments()
                .iter()
                .all(|s| matches!(s.kind, SegmentKind::Line))
            {
                let twice_area = shoelace(lu.vertices());
                match lu.role() {
                    LoopRole::Outer => {
                        assert!(twice_area > 0.0, "{name} loop {li}: outer runs CCW");
                    }
                    LoopRole::Hole => assert!(twice_area < 0.0, "{name} loop {li}: hole runs CW"),
                }
            }
            let start = lu.vertices()[0].pos();
            for v in lu.vertices() {
                let p = v.pos();
                assert!(
                    (start.x.value, start.y.value) <= (p.x.value, p.y.value),
                    "{name} loop {li}: the canonical start is the lex-min vertex"
                );
            }
        }
    }
    let ring = annulus().validate(tol()).unwrap().map(Dual64::from_f64);
    for s in ring.loops()[1].segments() {
        assert!(
            matches!(
                s.kind,
                SegmentKind::Arc {
                    turn: Sign::Negative,
                    ..
                }
            ),
            "the hole's arcs turn clockwise: {:?}",
            s.kind
        );
    }
}

/// `with_plane` replaces the plane and nothing else.
#[test]
fn with_plane_replaces_the_plane_and_nothing_else() {
    let vp = profile(vec![rounded_rect(4.0, 2.0, 0.5)])
        .validate(tol())
        .unwrap();
    let before = format!("{:?}", vp.loops());
    let plane = SketchPlane::new(Affine3::from_parts(
        geom_core::Mat3::from_cols(
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        Vec3::new(1.0, 2.0, 3.0),
    ));
    let moved = vp.with_plane(plane);
    assert_eq!(format!("{:?}", moved.plane()), format!("{plane:?}"));
    assert_eq!(format!("{:?}", moved.loops()), before);
    // The plane places; the 2-D data does not move with it.
    let origin = moved.plane().to_world(Point2::new(0.0, 0.0));
    assert_eq!((origin.x, origin.y, origin.z), (1.0, 2.0, 3.0));
}
