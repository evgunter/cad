//! **A sphere face slit to its pole: the interval lane's azimuth walk
//! never excludes the `f64` replay's branch, and containment answers the
//! same.**
//!
//! The dome is a unit hemisphere revolved about `y`: a flat base and two
//! π-band sphere faces meeting along two meridians at the pole. Merging
//! the base and killing one meridian leaves ONE sphere face whose loop
//! runs the surviving meridian up into the pole and straight back down
//! it — a slit, the pole a valence-1 strut tip (tier 2 names it; tier 1
//! holds). Killed from its minus half, the surviving face's walk crosses
//! the slit mid-walk: it enters the pole along the meridian and leaves
//! along the same one, so the face wraps the pole and the strictly-next
//! branch is the previous one a whole turn on, a literal at every scalar.
//!
//! The window row holds the interval walk against the `f64` window on
//! every slit: the interval window encloses the replay's, each end a
//! point-width enclosure, where the walk closes before the slit and where
//! it crosses it mid-walk alike. A pick that read the slit's coincident
//! azimuths as numbers would straddle the jump there, spanning both
//! branches, and escalate at the joint after it. The containment row
//! asks points of the dome at both scalars; the slit face's rims wrap
//! the axis, so the door reads its latitude window and not the azimuth
//! one, and a pick that refused the slit instead would turn every
//! verdict into a partial-sphere refusal. A point outside the dome whose
//! rays meet nothing is answered off a ray aimed at its base: the slit's
//! cusp leaves the at-infinity volume uncertified.
//!
//! A third row hands the slit to the Boolean, which serves finished
//! bodies only: a slit operand — the dome against a brick, and a slit
//! ball on the cube's top face — refuses at the at-rest gate that would
//! finish it, with tier 2's findings, naming its poles, while the unslit
//! bodies finish and reach a result. The split serves finished bodies
//! too, so the same gate keeps the slit from it; a fourth row hands the
//! slit to the split at a dual, whose gate runs nothing, and the split's
//! door reads tier 2 itself. The fifth does the same at both blend
//! doors, edge by edge: each refuses the slit as unfinished, never with
//! a reason about the edge it was asked to blend.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Bounds, Decide, Dual64, Interval, Point2, Point3, Real, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y_at};
use topo::construction::Construct;
use topo::{
    AtRestPolicy, Body, BooleanError, BooleanOp, BooleanResult, SolidContainment, SplitError,
    SplitJoinError, SplitReduceError, SweepStrategy, ValidationError, face_azimuth_window_traces,
    point_in_solid, validate, validate_closed,
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
            // A point outside is read off a ray aimed at the base: the
            // slit's strut tip is a cusp of the sphere face's loop, which
            // the sphere arm of the volume refuses
            // (`props_sphere_loop_cusp`), so the volume reads no side.
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

/// The slit face's azimuth window walk: the one sphere face the slit
/// leaves.
fn window<T: Decide>(slit: &Body<T>) -> Result<Option<(T, T)>, SplitJoinError> {
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
}

#[test]
fn the_interval_walk_encloses_the_f64_window_on_every_slit() {
    let replay = slits::<f64>();
    let certified = slits::<Interval>();
    assert_eq!(
        replay.len(),
        certified.len(),
        "one slit per kill in each lane"
    );
    for (i, (r, c)) in replay.iter().zip(&certified).enumerate() {
        let (r_lo, r_hi) = window(r)
            .expect("the f64 walk")
            .expect("the slit face's loop is a cycle");
        let (c_lo, c_hi) = window(c)
            .unwrap_or_else(|e| panic!("slit {i}: the interval walk: {e:?}"))
            .expect("the slit face's loop is a cycle");
        for (end, want, got) in [("lo", r_lo, c_lo), ("hi", r_hi, c_hi)] {
            assert!(
                got.lo() <= want && want <= got.hi() && got.hi() - got.lo() < 1e-9,
                "slit {i}: the interval window's {end} [{:e}, {:e}] is not a tight \
                 enclosure of the f64 window's {want} (f64 window ({r_lo}, {r_hi}))",
                got.lo(),
                got.hi()
            );
        }
    }
}

/// `slit` with a null strut at its pole: a zero-length scaffold edge
/// spliced into the slit face's loop between the meridian's climb and
/// its descent, so the walk skips two halves there.
fn strutted<T: Decide + topo::AtRestPolicy + Bounds>(slit: &Body<T>) -> Body<T> {
    let pole = slit
        .vertex_points()
        .find(|(_, p)| (p.y.lo() - 1.0).abs() < 1e-9 && p.x.hi().abs() < 1e-9)
        .map(|(v, _)| v)
        .expect("the slit's pole");
    let (down, _) = slit
        .half_edges()
        .find(|(_, he)| he.start == pole)
        .expect("the meridian leaves the pole");
    let mut body = slit.clone();
    body.mev_null(
        topo::construction::MevSite::Fan {
            he1: down,
            he2: down,
        },
        topo::construction::NewVertexSide::Above,
    )
    .expect("a null strut at the pole");
    body
}

/// **The slit face covers one turn, read through the walk's own
/// wiring**: every slit's window is exactly a turn wide, at `f64` and at
/// `Interval`, with and without a null strut between the meridian's two
/// passes through the pole. The face wraps the pole once, so where the
/// walk crosses the slit mid-walk its entry is the previous branch one
/// turn on in the face's own sense: a branch the other way, or a pick
/// that missed the same edge across the skipped strut, opens the window
/// to three turns or refuses it.
fn one_turn<T: Decide + topo::AtRestPolicy + Bounds>(lane: &str) {
    for (i, slit) in slits::<T>().iter().enumerate() {
        for (what, body) in [("bare", slit.clone()), ("strutted", strutted(slit))] {
            let (lo, hi) = window(&body)
                .unwrap_or_else(|e| panic!("[{lane}] slit {i} ({what}): {e:?}"))
                .expect("the slit face's loop is a cycle");
            let width = hi - lo;
            let tau = core::f64::consts::TAU;
            assert!(
                (width.lo() - tau).abs() < 1e-9 && (width.hi() - tau).abs() < 1e-9,
                "[{lane}] slit {i} ({what}): the window is one turn wide: [{:e}, {:e}]",
                width.lo(),
                width.hi()
            );
        }
    }
}

#[test]
fn every_slit_window_is_one_turn_at_f64() {
    one_turn::<f64>("f64");
}

#[test]
fn every_slit_window_is_one_turn_at_interval() {
    one_turn::<Interval>("Interval");
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
        &topo::BooleanInputs::default(),
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

/// **The split's door reads tier 2 where no verdict rides the slit.** At
/// a dual the at-rest gate runs nothing, so the slit reaches `split`
/// carrying no verdict, and the door refuses it before any stage with
/// tier 2's findings, the strut tip at the pole — whether the plane
/// meets the dome or clears it. Red without that read: a plane through
/// the dome refused for its sphere face, and a plane clear of it carried
/// the slit whole to its side and refused that side at the result gate,
/// as a kernel defect.
#[test]
fn a_slit_operand_refuses_at_the_split_door_at_a_dual() {
    let tol = Tol::witness();
    for (i, slit) in slits::<Dual64>().iter().enumerate() {
        let operand =
            Dual64::gate_at_rest_kept(slit.clone(), tol).expect("a dual gate runs nothing");
        for (y, cut) in [(0.5, "through the dome"), (2.0, "clear of the dome")] {
            let plane = topo::test_support::split_plane(
                Point3::new(f(0.0), f(y), f(0.0)),
                Vec3::new(f(0.0), f(1.0), f(0.0)),
                tol,
            );
            match topo::split(&operand, &plane, tol) {
                Err(SplitError::Reduce(SplitReduceError::ScaffoldingOperand { errors })) => {
                    assert_eq!(
                        Err(errors),
                        validate_closed(slit),
                        "slit {i}, {cut}: the payload is tier 2's verdict on the operand"
                    );
                }
                other => panic!(
                    "slit {i}, {cut}: want the door's tier-2 refusal, got {:?}",
                    other.map(|_| ())
                ),
            }
        }
    }
}

/// **The blend doors read tier 2 at a dual too.** Every edge of every
/// slit, filleted and chamfered: each refusal is the door's own tier-2
/// read, carrying `validate_closed`'s verdict. Red without that read:
/// the doors refused with reasons about the edge (a valence-2 corner,
/// tangential supports, a chart-seam vertex). The control follows the
/// refusal's recourse: the dome with its edit not left half-way (no
/// slit) passes the same read, and the fillet blends its base rim.
#[test]
fn a_slit_operand_refuses_at_both_blend_doors_at_a_dual() {
    let tol = Tol::witness();
    let whole = dome::<Dual64>();
    let rim = sweep::test_support::rim_arcs_at(&whole, 1.0, 0.0);
    let whole = Dual64::gate_at_rest_kept(whole, tol).expect("a dual gate runs nothing");
    let filleted = sweep::blend::fillet_edges(&whole, &rim, f(0.05), tol).map(|_| ());
    assert!(
        filleted.is_ok(),
        "the unslit dome's rim fillets: {filleted:?}"
    );
    for (i, slit) in slits::<Dual64>().iter().enumerate() {
        let operand =
            Dual64::gate_at_rest_kept(slit.clone(), tol).expect("a dual gate runs nothing");
        let want = validate_closed(slit);
        for (edge, _) in slit.edges() {
            for (verb, refused) in [
                (
                    "fillet",
                    sweep::blend::fillet_edges(&operand, &[edge], f(0.05), tol).map(|_| ()),
                ),
                (
                    "chamfer",
                    sweep::chamfer::chamfer_edges(&operand, &[edge], f(0.05), tol).map(|_| ()),
                ),
            ] {
                match refused.map_err(|r| r.error) {
                    Err(sweep::blend::BlendError::ScaffoldingOperand { errors }) => assert_eq!(
                        Err(errors),
                        want,
                        "slit {i}, {verb} {edge:?}: the payload is tier 2's verdict"
                    ),
                    other => {
                        panic!("slit {i}, {verb} {edge:?}: want the tier-2 refusal, got {other:?}")
                    }
                }
            }
        }
    }
}
