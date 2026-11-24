//! **A stored radius is positive by a decision, and a turn is a
//! decision or a refusal.** The doors a radius or a bulge's sign enters
//! by, each asked for the case its readers cannot hold: a table's
//! radius that is not positive, a bulge whose sign does not read, and an arc far enough
//! out that a reading of its squares would lose the difference.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/profile/src/seg.rs",
    "crates/profile/src/lib.rs",
    "crates/profile/src/path.rs",
    "crates/profile/src/validate.rs",
    "crates/geom-core/src/arc.rs",
    "crates/geom-core/src/real.rs",
];

use geom_core::{Arc2, Bounds, Interval, Point2, Real};
use profile::test_support::{arc_verb_loop, bulge_loop};
use profile::{
    Bulge, Open, PathError, Profile, ProfileLoop, RawLoop, Segment, SegmentDirections, SketchPlane,
    Start,
};

use crate::common::tol;

fn debug<E: core::fmt::Debug>(e: &E) -> String {
    format!("{e:?}")
}

/// Two half turns about the origin through `(±x, 0)` with the stored
/// `radius`, as a table.
fn halves<T: Real>(x: T, radius: T) -> ProfileLoop<T> {
    let half = Arc2 {
        centre: Point2::new(T::zero(), T::zero()),
        radius,
        sweep: T::from_f64(core::f64::consts::PI),
    };
    RawLoop::new([
        (Point2::new(x, T::zero()), Segment::Arc(half)),
        (Point2::new(-x, T::zero()), Segment::Arc(half)),
    ])
}

/// **A table arc whose radius is not positive is refused as an arc
/// that disagrees with its vertices, before any reader takes the
/// radius**: negative with its vertices exactly on the mirror circle
/// (where a gap over `‖w‖ + radius` is `0/0`), negative a rounding off
/// it, and zero. Over a box that holds zero the radius's own reading
/// escalates (`arc_radius`).
///
/// Red with the `arc_radius` reading removed (`seg::radius_positive`):
/// the on-mirror rows escalate on a poisoned `arc_start_on_carrier`
/// where they are refused here.
#[test]
fn a_table_arc_whose_radius_is_not_positive_is_refused_at_its_door() {
    for radius in [-1.0, -1.0 + 1e-13] {
        for x in [1.0, 1.0 + 2.0 * f64::EPSILON] {
            let got = Profile::new(SketchPlane::xy(), vec![halves(x, radius)])
                .validate(tol())
                .map(|_| ())
                .map_err(|e| debug(&e));
            let refusal = got.expect_err("a radius that is not positive validates nowhere");
            assert!(
                refusal.starts_with("InconsistentArc") && refusal.contains("OnCarrier"),
                "radius {radius:e}, vertex at {x:e}: {refusal}"
            );
        }
    }
    // Over a box that holds zero nothing decides the radius: the arc
    // length, read ahead of the carrier, escalates on it.
    let boxed = Interval::from_bounds(-0.5, 1.5);
    let got = Profile::new(
        SketchPlane::xy(),
        vec![halves(Interval::from_f64(1.0), boxed)],
    )
    .validate(tol())
    .map(|_| ())
    .map_err(|e| debug(&e));
    let refusal = got.expect_err("a radius boxed across zero is undecided");
    assert!(
        refusal.starts_with("Escalated"),
        "a radius boxed across zero: {refusal}"
    );
    // The same table at a positive radius is the control.
    Profile::new(SketchPlane::xy(), vec![halves(1.0, 1.0)])
        .validate(tol())
        .expect("the unit circle in two halves validates");
}

/// A bulge leg from `(4, 0)` to `(4, 4)` as a path's first leg, so no
/// junction is read ahead of it, closed by three lines.
fn first_leg_bulge(b: Interval) -> Result<ProfileLoop<Interval>, PathError<Interval>> {
    let l = Interval::from_f64;
    let p = |x: f64, y: f64| Point2::new(l(x), l(y));
    let w = tol();
    Ok(Open
        .at(p(4.0, 0.0))
        .arc_to(Bulge { p: p(4.0, 4.0), b }, w)?
        .line_to(p(0.0, 4.0), w)?
        .line_to(p(0.0, 0.0), w)?
        .line_to(Start, w)?
        .loop_
        .into_loop())
}

/// **A bulge whose sign does not read stores no arc.** Through the
/// path door a bulge boxed across zero refuses on its own reading
/// (`path_arc_bulge`): an escalation where the box is wider than the
/// band, a degenerate spec where it sits inside it. The fixture door,
/// which is total, stores a poisoned radius that validation refuses. A
/// bulge boxed clear of zero stores a positive radius and its radials.
///
/// Red with the lowering's tie arm put back (a measured radius, no
/// radial): the wide box stores an arc at the path door.
#[test]
fn a_bulge_whose_sign_does_not_read_stores_no_arc() {
    let wide = first_leg_bulge(Interval::from_bounds(-0.1, 0.3))
        .map(|_| ())
        .map_err(|e| debug(&e));
    let refusal = wide.expect_err("a bulge boxed across zero names no arc");
    assert!(
        refusal.contains("Escalated") && refusal.contains("PathArcBulge"),
        "a wide tie: {refusal}"
    );
    let narrow = first_leg_bulge(Interval::from_bounds(-1e-12, 1e-12))
        .map(|_| ())
        .map_err(|e| debug(&e));
    let refusal = narrow.expect_err("a bulge in band of zero names no arc");
    assert!(
        refusal.contains("DegenerateArcSpec") || refusal.contains("PathArcBulge"),
        "a tie inside the band: {refusal}"
    );
    let clear = first_leg_bulge(Interval::from_bounds(0.1, 0.3)).expect("a signed bulge builds");
    let arc = clear
        .segments()
        .iter()
        .find_map(|s| match s {
            Segment::Arc(a) => Some(*a),
            Segment::Line => None,
        })
        .expect("the leg is an arc");
    assert!(arc.radius.lo() > 0.0, "radius {:?}", arc.radius);
    assert!(
        clear
            .directions()
            .iter()
            .any(|d| matches!(d, Some(SegmentDirections::Arc { .. }))),
        "a signed bulge records its radials"
    );
    let l = Interval::from_f64;
    let fixture = bulge_loop(vec![
        (Point2::new(l(0.0), l(0.0)), l(0.0)),
        (
            Point2::new(l(4.0), l(0.0)),
            Interval::from_bounds(-0.1, 0.3),
        ),
        (Point2::new(l(4.0), l(4.0)), l(0.0)),
        (Point2::new(l(0.0), l(4.0)), l(0.0)),
    ]);
    assert!(
        Profile::new(SketchPlane::xy(), vec![fixture])
            .validate(tol())
            .is_err(),
        "the fixture door's tied bulge validates nowhere"
    );
}

/// **A far arc is read to its root's accuracy**: bulge and via arcs of
/// radius past `1e7` on a chord of four metres, whose start is on the
/// carrier to a rounding of the radius. The path door stores each and
/// validation accepts it, as when the reader was `rim − radius`.
///
/// Red with [`geom_core::radius_gap`] spelled `(N − r²)/(‖w‖ + r)`: a
/// rounding of the squares over twice the radius is past the band, and
/// the door's read-back refuses.
#[test]
fn a_far_arc_is_stored_by_the_path_door_at_the_default_tolerance() {
    // The documents are measured at the default row: at another ε the
    // band they sit against is another, and the lens rows hold it there
    // (`a_far_arcs_lens_reads_its_own_width`).
    if tol().eps().to_bits() != 1.0e-9_f64.to_bits() {
        return;
    }
    for (verb, q) in [
        ("bulge", -1.306_923_626_133_198e-8),
        ("bulge cw", -1.363_230_105_576_11e-8),
        ("via", 3.999_999_896_912_03),
        ("via cw", 3.999_999_987_985_185_5),
    ] {
        let lp = arc_verb_loop::<f64>(verb, q, tol());
        let radius = lp
            .segments()
            .iter()
            .find_map(|s| match s {
                Segment::Arc(a) => Some(a.radius),
                Segment::Line => None,
            })
            .expect("the verb stores an arc");
        assert!(radius > 1.0e7, "{verb}: radius {radius:e}");
        Profile::new(SketchPlane::xy(), vec![lp])
            .validate(tol())
            .unwrap_or_else(|e| panic!("{verb} at q = {q:e} (radius {radius:e}): {e:?}"));
    }
}
