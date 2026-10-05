//! **A sphere face slit to its pole: the interval lane's azimuth window
//! encloses the `f64` replay's, and containment answers the same.**
//!
//! The dome is a unit hemisphere revolved about `y`: a flat base and two
//! π-band sphere faces meeting along two meridians at the pole. Merging
//! the base and killing one meridian leaves ONE sphere face whose loop
//! runs the surviving meridian up into the pole and straight back down
//! it — a slit, the pole a valence-1 strut tip (tier 2 names it; tier 1
//! holds). Killed from its minus half, the surviving face's walk crosses
//! the slit mid-walk, where the azimuth walk's strictly-next pole branch
//! sits exactly on its jump: at `f64` it lands a whole period on, and at
//! `Interval` the enclosure straddles it and spans both branches.
//!
//! The window row holds the interval window against the `f64`
//! window on every slit: a pick that kept one branch of the straddle
//! would hand back a window excluding the replay's. The containment row
//! asks points of the dome at both scalars; the slit face's rims wrap
//! the axis, so the door reads its latitude window and not the azimuth
//! one, and a pick that refused the straddle instead would turn every
//! verdict into a partial-sphere refusal.
//!
//! A third row hands the slit to the Boolean, which serves finished
//! bodies only: a slit operand — the dome against a brick, and a slit
//! ball on the cube's top face — refuses at the at-rest gate that would
//! finish it, with tier 2's findings, naming its poles, while the unslit
//! bodies finish and reach a result.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Bounds, Decide, Interval, Point2, Point3, Real, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y_at};
use topo::{
    Body, BooleanDeclarations, BooleanError, BooleanOp, BooleanResult, SolidContainment,
    SweepStrategy, ValidationError, face_azimuth_window_traces, point_in_solid, validate,
    validate_closed,
};

fn f<T: Real>(x: f64) -> T {
    T::from_f64(x)
}

/// The unit dome, its base merged to one disc.
fn dome<T: Decide + topo::AtRestPolicy>() -> Body<T> {
    let bulge = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    let mut body = revolved_about_y_at(
        vec![
            (Point2::new(f(0.0), f(0.0)), f(0.0)),
            (Point2::new(f(1.0), f(0.0)), f(bulge)),
            (Point2::new(f(0.0), f(1.0)), f(0.0)),
        ],
        Revolution::Full,
        Tol::witness(),
    );
    body.merge_coplanar_faces(Tol::witness())
        .expect("the pole-split base merges");
    body
}

/// Every slit of the dome: each meridian between the two sphere bands,
/// killed from each of its halves.
fn slits<T: Decide + topo::AtRestPolicy>() -> Vec<Body<T>> {
    slits_of(&dome::<T>(), 1)
}

/// Every slit of `body`: each meridian between its two sphere bands,
/// killed from each of its halves.
fn slits_of<T: Decide + topo::AtRestPolicy>(body: &Body<T>, struts: usize) -> Vec<Body<T>> {
    let on_sphere = |he| {
        body.face_of_half_edge(he)
            .and_then(|face| body.get_face(face))
            .and_then(|face| body.get_surface(face.surface))
            .is_some_and(|s| matches!(s, geom::Surface::Sphere { .. }))
    };
    let meridians: Vec<_> = body
        .edges()
        .filter(|(_, e)| on_sphere(e.he_plus) && on_sphere(e.he_minus))
        .map(|(_, e)| (e.he_plus, e.he_minus))
        .collect();
    assert_eq!(meridians.len(), 2, "the dome's two band meridians");
    meridians
        .into_iter()
        .flat_map(|(plus, minus)| [plus, minus])
        .map(|he| {
            let mut slit = body.clone();
            slit.kef(he).expect("the meridian kills");
            assert_eq!(validate(&slit), Ok(()), "tier 1 holds on the slit");
            let tier2 = validate_closed(&slit).expect_err("the slit is scaffolding");
            assert!(
                tier2.len() == struts
                    && tier2
                        .iter()
                        .all(|e| matches!(e, ValidationError::ScaffoldingStrutVertex { .. })),
                "the slit's poles are its {struts} strut tips: {tier2:?}"
            );
            slit
        })
        .collect()
}

/// Points against the unit dome `{ y ≥ 0, |p| ≤ 1 }`, with the answer.
const TABLE: [((f64, f64, f64), SolidContainment); 6] = [
    ((0.1, 0.5, 0.1), SolidContainment::In),
    ((0.1, 1.5, 0.1), SolidContainment::Out),
    ((0.5, 0.2, -0.3), SolidContainment::In),
    ((0.0, 0.99, 0.0), SolidContainment::In),
    ((0.7, 0.7, 0.0), SolidContainment::In),
    ((0.0, 0.5, 0.86), SolidContainment::In),
];

fn verdicts<T: Decide + topo::AtRestPolicy + Bounds>(lane: &str) {
    let band = Band::linear(Tol::witness()).expect("the witness band");
    for (i, slit) in slits::<T>().iter().enumerate() {
        for ((x, y, z), want) in TABLE {
            let got = point_in_solid(slit, Point3::new(f(x), f(y), f(z)), band, Tol::witness());
            assert!(
                matches!(got, Ok(v) if v == want),
                "[{lane}] slit {i}: point ({x}, {y}, {z}) against the slit dome: \
                 {got:?}, want {want:?}"
            );
        }
    }
}

#[test]
fn a_slit_dome_answers_containment_at_f64() {
    verdicts::<f64>("f64");
}

#[test]
fn a_slit_dome_answers_containment_at_interval() {
    verdicts::<Interval>("Interval");
}

/// The slit face's azimuth window: the one sphere face the slit leaves.
fn window<T: Decide + Bounds>(slit: &Body<T>) -> (T, T) {
    let band = Band::linear(Tol::witness()).expect("the witness band");
    let mut spheres = slit.faces().filter(|(_, face)| {
        matches!(
            slit.get_surface(face.surface),
            Some(geom::Surface::Sphere { .. })
        )
    });
    let (face, _) = spheres.next().expect("the slit sphere face");
    assert!(spheres.next().is_none(), "one sphere face after the kill");
    face_azimuth_window_traces(slit, face, band)
        .expect("the slit face's walk")
        .expect("the slit face's loop is a cycle")
}

#[test]
fn the_interval_window_of_a_slit_dome_encloses_the_f64_window() {
    let replay = slits::<f64>();
    let certified = slits::<Interval>();
    assert_eq!(
        replay.len(),
        certified.len(),
        "one slit per kill in each lane"
    );
    for (i, (r, c)) in replay.iter().zip(&certified).enumerate() {
        let (r_lo, r_hi) = window(r);
        let (c_lo, c_hi) = window(c);
        for (end, want, got) in [("lo", r_lo, c_lo), ("hi", r_hi, c_hi)] {
            assert!(
                got.lo() <= want && want <= got.hi(),
                "slit {i}: the interval window's {end} [{:e}, {:e}] does not enclose \
                 the f64 window's {want} (f64 window ({r_lo}, {r_hi}))",
                got.lo(),
                got.hi()
            );
        }
    }
}

/// The unit ball about `(0.5, 1, 0.5)`'s `y` axis at radius 0.3, whose
/// slits each leave both poles strut tips: the die-pip placement on the
/// unit cube's top face.
fn ball<T: Decide + topo::AtRestPolicy>() -> Body<T> {
    sweep::test_support::ball_poled_y::<T>(
        f(0.3),
        Vec3::new(f(0.5), f(1.0), f(0.5)),
        Tol::witness(),
    )
}

fn subtract<T: Decide + topo::AtRestPolicy + Bounds>(
    a: &Body<T>,
    b: &Body<T>,
) -> Result<BooleanResult<T>, BooleanError> {
    topo::boolean_op_with(
        BooleanOp::Subtract,
        &finished("operand A", a.clone(), Tol::witness()),
        &finished("operand B", b.clone(), Tol::witness()),
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
}

/// The refusal a slit operand owes: typed at the at-rest gate that
/// would finish it for the Boolean, carrying tier 2's own findings — the
/// poles, each named as the strut tip it is.
fn assert_refused_at_the_gate<T: Decide + topo::AtRestPolicy + Bounds>(
    what: &str,
    slit: &Body<T>,
    poles: &[(f64, f64, f64)],
) {
    let Err(errors) = T::gate_at_rest_kept(slit.clone(), Tol::witness()) else {
        panic!("{what}: a slit body is not a finished body");
    };
    assert_eq!(
        Err(errors.clone()),
        validate_closed(slit),
        "{what}: the payload is tier 2's verdict on the operand"
    );
    let mut tips: Vec<(f64, f64, f64)> = errors
        .iter()
        .map(|e| {
            let ValidationError::ScaffoldingStrutVertex { vertex } = e else {
                panic!("{what}: a finding other than a strut tip: {e:?}");
            };
            let p = slit
                .get_vertex(*vertex)
                .and_then(|v| slit.get_point(v.point))
                .expect("the named vertex resolves");
            let mid = |v: T| 0.5 * (v.lo() + v.hi());
            (mid(p.x), mid(p.y), mid(p.z))
        })
        .collect();
    tips.sort_by(|a, b| a.partial_cmp(b).expect("finite poles"));
    assert_eq!(
        tips.len(),
        poles.len(),
        "{what}: one finding per pole: {tips:?}"
    );
    for (tip, pole) in tips.iter().zip(poles) {
        let off = (tip.0 - pole.0).hypot(tip.1 - pole.1).hypot(tip.2 - pole.2);
        assert!(
            off < 1e-9,
            "{what}: strut tip {tip:?} is not the pole {pole:?}"
        );
    }
}

fn slit_operands_refuse_at_the_gate<T: Decide + topo::AtRestPolicy + Bounds>(lane: &str) {
    let brick =
        sweep::test_support::brick::<T>((-2.0, 2.0), (0.5, 2.0), (-2.0, 2.0), Tol::witness());
    for (i, slit) in slits::<T>().iter().enumerate() {
        assert_refused_at_the_gate(&format!("[{lane}] slit dome {i}"), slit, &[(0.0, 1.0, 0.0)]);
    }
    let cube = sweep::test_support::cube::<T>(1.0, Tol::witness());
    for (i, slit) in slits_of(&ball::<T>(), 2).iter().enumerate() {
        assert_refused_at_the_gate(
            &format!("[{lane}] slit ball {i}"),
            slit,
            &[(0.5, 0.7, 0.5), (0.5, 1.3, 0.5)],
        );
    }
    // What is at rest finishes: the same placements unslit reach a
    // result.
    for (what, got) in [
        ("dome ∖ brick", subtract(&dome::<T>(), &brick)),
        ("cube ∖ ball", subtract(&cube, &ball::<T>())),
    ] {
        assert!(
            matches!(got, Ok(BooleanResult::Body(_))),
            "[{lane}] {what}: an at-rest operand passes the gate to a result: {got:?}"
        );
    }
}

#[test]
fn a_slit_operand_refuses_at_the_boolean_gate_at_f64() {
    slit_operands_refuse_at_the_gate::<f64>("f64");
}

#[test]
fn a_slit_operand_refuses_at_the_boolean_gate_at_interval() {
    slit_operands_refuse_at_the_gate::<Interval>("Interval");
}
