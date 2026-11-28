//! Reviewer probes of `topo::boolean::gather_finished` and
//! `FinishedBody::gathered` (PR 7723's head): what the public door
//! admits, and whether a carried finish is the derived one beyond two
//! bricks.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::{Interval, RigidMotion, Tol, UnitVec3, Vec3};
use topo::{AtRestBody, AtRestPolicy, Body, BooleanResult, FinishedBody, Placed};

fn brick<T: AtRestPolicy + geom_core::Decide>(
    x: (f64, f64),
    y: (f64, f64),
    z: (f64, f64),
) -> Body<T> {
    common::brick::<T>(x, y, z, Tol::witness())
}

fn fin<T: AtRestPolicy + geom_core::Decide>(body: Body<T>) -> T::Finished {
    topo::finish(body, Tol::witness()).expect("the fixture finishes")
}

fn resolution<T: geom_core::Real>() -> topo::Settle<T> {
    topo::Settle::Resolution(quantity::Length::from_meters(Tol::witness().get().eps))
}

/// **The trait method is public and gates nothing**: a finished brick,
/// and beside it in one body a brick the finish refuses (past the
/// session box). `FinishedBody::gathered`, handed that body, the one
/// finished part and the keys of its graft, returns a finished body of
/// two solids, one of which no gate read.
#[test]
fn the_public_trait_method_finishes_a_body_holding_a_solid_no_gate_read() {
    let tol = Tol::witness();
    let good = fin(brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)));
    let far = || brick::<f64>((2.0e3, 2.0e3 + 1.0), (0.0, 1.0), (0.0, 1.0));
    assert!(
        topo::finish(far(), tol).is_err(),
        "the far brick is not a finished body"
    );
    let mut aggregate = Body::new();
    let keys = topo::graft_disjoint_all_keyed(&mut aggregate, &good).unwrap();
    topo::graft_disjoint_all_keyed(&mut aggregate, &far()).unwrap();
    let minted = <AtRestBody<f64> as FinishedBody>::gathered(aggregate, &[&good], &[keys])
        .expect("gathered gates nothing at a certifying scalar");
    assert_eq!(minted.solids().count(), 2);
    let derived = topo::finish(minted.into_body(), tol);
    assert!(
        derived.is_err(),
        "the body gathered() finished is one the finish refuses"
    );
}

/// The same with a body that is no graft of the part at all.
#[test]
fn the_public_trait_method_finishes_a_body_unrelated_to_its_parts() {
    let tol = Tol::witness();
    let good = fin(brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)));
    let mut scratch = Body::new();
    let keys = topo::graft_disjoint_all_keyed(&mut scratch, &good).unwrap();
    let far = brick::<f64>((2.0e3, 2.0e3 + 1.0), (0.0, 1.0), (0.0, 1.0));
    let minted = <AtRestBody<f64> as FinishedBody>::gathered(far, &[&good], &[keys])
        .expect("gathered gates nothing");
    // It is taken wherever a finished body is: a Boolean operand here.
    let other = fin(brick::<f64>((5.0, 6.0), (0.0, 1.0), (0.0, 1.0)));
    let united = topo::union(&minted, &other, tol);
    eprintln!("union with the minted body: {:?}", united.map(|_| ()));
    assert!(topo::finish(minted.into_body(), tol).is_err());
}

fn union_of(a: Body<f64>, b: Body<f64>) -> AtRestBody<f64> {
    match topo::union(&fin(a), &fin(b), Tol::witness()) {
        Ok(BooleanResult::Body(bb)) => bb.body,
        other => panic!("the union builds: {:?}", other.map(|_| ())),
    }
}

fn subtract_of(a: Body<f64>, b: Body<f64>) -> AtRestBody<f64> {
    match topo::subtract(&fin(a), &fin(b), Tol::witness()) {
        Ok(BooleanResult::Body(bb)) => bb.body,
        other => panic!("the difference builds: {:?}", other.map(|_| ())),
    }
}

fn members() -> Vec<(&'static str, AtRestBody<f64>)> {
    vec![
        ("brick", fin(brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)))),
        (
            "union",
            union_of(
                brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
                brick::<f64>((0.5, 1.5), (0.25, 0.75), (0.25, 1.5)),
            ),
        ),
        (
            "cavity",
            subtract_of(
                brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
                brick::<f64>((0.25, 0.75), (0.25, 0.75), (0.25, 0.75)),
            ),
        ),
        (
            "two solids",
            subtract_of(
                brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
                brick::<f64>((0.4, 0.6), (-1.0, 2.0), (-1.0, 2.0)),
            ),
        ),
    ]
}

fn motions() -> Vec<(&'static str, Option<RigidMotion<f64>>)> {
    let shift = |x: f64| RigidMotion::translation(Vec3::new(x, 0.0, 0.0)).unwrap();
    vec![
        ("same place", None),
        ("overlapping", Some(shift(0.3))),
        ("touching", Some(shift(1.0))),
        ("apart", Some(shift(3.0))),
        (
            "turned apart",
            Some(
                RigidMotion::rotation(UnitVec3::exact_z(), 0.3)
                    .unwrap()
                    .translated(Vec3::new(3.0, 0.5, 0.2))
                    .unwrap(),
            ),
        ),
        ("half turn", Some(RigidMotion::half_turn_x().translated(Vec3::new(4.0, 0.0, 0.0)).unwrap())),
    ]
}

/// **The differential**: for each member kind and each placement of a
/// second copy, the gather's outcome beside the outcome of finishing the
/// same aggregate from values, and the two volumes' bits.
#[test]
fn a_carried_gather_beside_the_derived_finish_over_member_kinds_and_placements() {
    let tol = Tol::witness();
    let mut differing = Vec::new();
    for (kind, member) in members() {
        eprintln!("member {kind}: {} solids, {} faces", member.solids().count(), member.faces().count());
        for (place, motion) in motions() {
            let what = format!("{kind} / {place}");
            let gathered = topo::boolean::gather_finished(
                &[Placed::own(&member), Placed::new(&member, motion)],
                tol,
            );
            let (gathered, _) = match gathered {
                Ok(g) => g,
                Err(e) => {
                    eprintln!("{what}: the gather refuses: {e:?}");
                    continue;
                }
            };
            let carried = gathered.measure(resolution()).map(|m| m.enclosure.volume);
            let derived = topo::finish(gathered.into_body(), tol);
            match (&carried, &derived) {
                (Ok(c), Ok(d)) => {
                    let d = d.measure(resolution()).map(|m| m.enclosure.volume);
                    // A carried run keeps the flux its member's finish read; a
                    // derived one reads the moved face again, so the two may
                    // differ by rounding (measured: 1 to 3 ulp on the
                    // two-solid member). Held to 1e-12 relative.
                    let same = d.as_ref().is_ok_and(|d| d.to_bits() == c.to_bits());
                    let close = d.as_ref().is_ok_and(|d| ((d - c) / c).abs() <= 1e-12);
                    eprintln!("{what}: carried {c:e}, derived {d:?}, same bits {same}");
                    if !close {
                        differing.push(what);
                    }
                }
                (c, Err(e)) => {
                    let e = format!("{e:?}");
                    eprintln!("{what}: carried {c:?}; THE DERIVED FINISH REFUSES: {}", &e[..e.len().min(600)]);
                    differing.push(what);
                }
                (Err(c), Ok(_)) => {
                    eprintln!("{what}: the carried certificate does not measure ({c:?}); derived finishes");
                    differing.push(what);
                }
            }
        }
    }
    assert!(differing.is_empty(), "carried differs from derived at: {differing:#?}");
}

/// **Lane logs over the same members**, at `f64` and at a zero-width
/// `Interval`.
#[test]
fn the_gather_logs_one_vector_at_f64_and_interval_over_boolean_members() {
    fn logged<T: AtRestPolicy + geom_core::Decide + geom_core::Bounds>(
        angle: T,
    ) -> Vec<geom_core::k_stats::Verdict> {
        let tol = Tol::witness();
        let a = fin(brick::<T>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)));
        let b = fin(brick::<T>((0.5, 1.5), (0.25, 0.75), (0.25, 1.5)));
        let member = match topo::union(&a, &b, tol) {
            Ok(BooleanResult::Body(bb)) => bb.body,
            other => panic!("the union builds: {:?}", other.map(|_| ())),
        };
        let turn = RigidMotion::rotation(UnitVec3::exact_z(), angle)
            .unwrap()
            .translated(Vec3::new(T::from_f64(3.0), T::from_f64(0.5), T::from_f64(0.2)))
            .unwrap();
        let bracket = geom_core::k_stats::Bracket::open();
        topo::boolean::gather_finished(&[Placed::own(&member), Placed::at(&member, turn)], tol)
            .expect("the gather finishes");
        bracket.finish().verdicts
    }
    let point = logged::<f64>(0.3);
    let boxed = logged::<Interval>(Interval::from_bounds(0.3, 0.3));
    eprintln!("f64 logs {} verdicts, Interval {}", point.len(), boxed.len());
    assert_eq!(point, boxed);
}

/// **`GatherError`'s arms a row can reach**: a member whose image is past
/// the session box (`Moved`, naming it), and a gather of none (`Graft`).
#[test]
fn the_gathers_typed_refusals_are_reachable() {
    let tol = Tol::witness();
    let a = fin(brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)));
    let far = RigidMotion::translation(Vec3::new(2.0e3, 0.0, 0.0)).unwrap();
    let moved = topo::boolean::gather_finished(&[Placed::own(&a), Placed::at(&a, far)], tol);
    assert!(
        matches!(moved, Err(topo::boolean::GatherError::Moved { member: 1, .. })),
        "{:?}",
        moved.map(|_| ())
    );
    let none = topo::boolean::gather_finished::<AtRestBody<f64>>(&[], tol);
    assert!(
        matches!(none, Err(topo::boolean::GatherError::Graft { member: 0, .. })),
        "{:?}",
        none.map(|_| ())
    );
}

/// **A member in another's cavity, and a gather of a gather**: carried,
/// and what the derived finish says of the same aggregates.
#[test]
fn a_member_in_anothers_cavity_and_a_gather_of_a_gather() {
    let tol = Tol::witness();
    let shell = subtract_of(
        brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
        brick::<f64>((0.25, 0.75), (0.25, 0.75), (0.25, 0.75)),
    );
    let pip = fin(brick::<f64>((0.4, 0.6), (0.4, 0.6), (0.4, 0.6)));
    let (nested, _) = topo::boolean::gather_finished(&[Placed::own(&shell), Placed::own(&pip)], tol)
        .expect("the gather carries");
    let carried = nested.measure(resolution()).unwrap().enclosure.volume;
    let derived = topo::finish(nested.clone().into_body(), tol);
    eprintln!(
        "a brick in a cavity: carried {carried:e}; derived finish {:?}",
        derived.as_ref().map(|d| d.measure(resolution()).map(|m| m.enclosure.volume))
    );
    assert!(derived.is_ok(), "the derived finish of a member in a cavity");
    let turn = RigidMotion::rotation(UnitVec3::exact_z(), 0.7)
        .unwrap()
        .translated(Vec3::new(4.0, 1.0, -2.0))
        .unwrap();
    let (twice, _) =
        topo::boolean::gather_finished(&[Placed::at(&nested, turn), Placed::own(&nested)], tol)
            .expect("a gather of a gather carries");
    assert_eq!(twice.solids().count(), 4);
    let c = twice.measure(resolution()).unwrap().enclosure.volume;
    let d = topo::finish(twice.into_body(), tol)
        .expect("derives")
        .measure(resolution())
        .unwrap()
        .enclosure
        .volume;
    eprintln!("a gather of a gather: carried {c:e}, derived {d:e}");
    assert!(((c - d) / d).abs() <= 1e-12 && (c - 2.0 * (0.875 + 0.008)).abs() <= 1e-12);
}

/// **At plain `Interval`, unmoved**: the carried certificate's volume
/// beside the derived finish's, on a Boolean member gathered twice where
/// it stands (no motion, so the derived finish has no moved endpoint to
/// refuse on).
#[test]
fn at_interval_an_unmoved_gather_carried_beside_derived() {
    let tol = Tol::witness();
    let a = fin(brick::<Interval>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)));
    let b = fin(brick::<Interval>((0.5, 1.5), (0.25, 0.75), (0.25, 1.5)));
    let member = match topo::union(&a, &b, tol) {
        Ok(BooleanResult::Body(bb)) => bb.body,
        other => panic!("the union builds at Interval: {:?}", other.map(|_| ())),
    };
    let lone = fin(brick::<Interval>((3.0, 4.0), (0.0, 1.0), (0.0, 1.0)));
    let (g, _) = topo::boolean::gather_finished(&[Placed::own(&member), Placed::own(&lone)], tol)
        .expect("carries");
    let c = g.measure(resolution()).unwrap().enclosure.volume;
    let derived = topo::finish(g.into_body(), tol);
    match derived {
        Ok(d) => {
            let d = d.measure(resolution()).unwrap().enclosure.volume;
            eprintln!("Interval unmoved: carried {c:?}, derived {d:?}");
            use geom_core::Bounds;
            assert!(c.lo() <= d.hi() && d.lo() <= c.hi(), "the enclosures meet");
            assert!(c.lo() <= 2.4375 && 2.4375 <= c.hi());
        }
        Err(e) => {
            let e = format!("{e:?}");
            eprintln!("Interval unmoved: carried {c:?}; derived refuses: {}", &e[..e.len().min(500)]);
        }
    }
}

/// **Can a mirrored map reach `Placed`?** `Placed` takes a
/// `RigidMotion`, whose fields are private, but `RigidMotion::map_scalar`
/// is public and carries each entry through any closure between two
/// scalars of one witness kind: negating every entry of the identity is
/// the point inversion, an improper map, typed as a rigid motion. This
/// prints what the gather and the derived finish make of it.
#[test]
fn a_point_inversion_typed_as_a_rigid_motion_reaches_the_gather() {
    let tol = Tol::witness();
    let a = fin(brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0)));
    let inversion = RigidMotion::<f64>::identity().map_scalar(|x| -x);
    let p = inversion.transform_point(geom_core::Point3::new(1.0, 2.0, 3.0));
    eprintln!("the map sends (1, 2, 3) to {p:?}");
    let gathered = topo::boolean::gather_finished(&[Placed::own(&a), Placed::at(&a, inversion)], tol);
    match gathered {
        Err(e) => eprintln!("the gather refuses the inversion: {e:?}"),
        Ok((g, _)) => {
            let carried = g.measure(resolution()).map(|m| m.enclosure.volume);
            let derived = topo::finish(g.into_body(), tol).map(|_| ());
            let derived = format!("{derived:?}");
            eprintln!(
                "the gather CARRIES under the inversion: carried volume {carried:?}; the derived finish: {}",
                &derived[..derived.len().min(500)]
            );
        }
    }
}
