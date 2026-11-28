//! Reviewer probes for `topo::boolean::gather_finished` (PR 7723): the
//! carried certificate against the one a finish derives on the same
//! aggregate, over member shapes the unit's own rows do not draw.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::{Interval, RigidMotion, Tol, UnitVec3, Vec3};
use topo::{AtRestBody, Body, BooleanResult, Placed};

fn tol() -> Tol {
    Tol::witness()
}

fn res<T: geom_core::Real>() -> topo::Settle<T> {
    topo::Settle::Resolution(quantity::Length::from_meters(tol().get().eps))
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    topo::finish(common::brick::<f64>(x, y, z, tol()), tol()).expect("brick finishes")
}

fn hollow() -> AtRestBody<f64> {
    let outer = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let cavity = brick((0.2, 0.8), (0.2, 0.8), (0.2, 0.8));
    match topo::subtract(&outer, &cavity, tol()).expect("subtract runs") {
        BooleanResult::Body(r) => r.body,
        other => panic!("{other:?}"),
    }
}

fn drum() -> AtRestBody<f64> {
    topo::finish(common::seamless_drum::<f64>(0.3, 0.5, tol()), tol()).expect("drum finishes")
}

fn shift(x: f64) -> RigidMotion<f64> {
    RigidMotion::translation(Vec3::new(x, 0.0, 0.0)).unwrap()
}

fn turn(a: f64) -> RigidMotion<f64> {
    RigidMotion::rotation(UnitVec3::exact_z(), a).unwrap()
}

/// Carried against derived, at f64: every measured field, as bits.
fn carried_is_derived(what: &str, members: &[Placed<'_, AtRestBody<f64>>]) {
    let (gathered, keys) = topo::boolean::gather_finished(members, tol())
        .unwrap_or_else(|e| panic!("{what}: gather: {e:?}"));
    assert_eq!(keys.len(), members.len());
    let carried = gathered.measure(res()).expect("carried measures").enclosure;
    let derived = topo::finish(gathered.into_body(), tol());
    let derived = match derived {
        Ok(d) => d.measure(res()).expect("derived measures").enclosure,
        Err(e) => panic!("{what}: the aggregate does not finish from values at f64: {e:?}"),
    };
    eprintln!("{what}: carried {carried:?}\n{what}: derived {derived:?}");
    // Measured: NOT bit for bit (the carried runs were summed in each
    // member's own frame; e.g. 2.0000000000000008e-6 against 2e-6 for
    // the turned bricks), but within a few ulps relative.
    for (c, d) in [
        (carried.volume, derived.volume),
        (carried.surface_area, derived.surface_area),
    ] {
        assert!(
            (c - d).abs() <= 1e-14 * d.abs(),
            "{what}: carried {c} is derived {d} to ulps"
        );
    }
}

#[test]
fn probe_carried_is_derived_two_bricks_one_turned() {
    let a = brick((-0.02, -0.01), (-0.005, 0.005), (0.0, 0.01));
    let b = brick((0.03, 0.04), (-0.005, 0.005), (0.0, 0.01));
    carried_is_derived("bricks", &[Placed::own(&a), Placed::at(&b, turn(0.02))]);
}

#[test]
fn probe_carried_is_derived_with_a_cavity_and_a_curved_member() {
    let h = hollow();
    let d = drum();
    carried_is_derived(
        "hollow+drum",
        &[
            Placed::at(&d, shift(3.0)),
            Placed::own(&h),
            Placed::at(&d, turn(1.1).compose(shift(-3.0)).unwrap()),
        ],
    );
}

#[test]
fn probe_carried_is_derived_for_a_multi_solid_member_carried_twice() {
    let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let d = drum();
    let (two, _) =
        topo::boolean::gather_finished(&[Placed::own(&a), Placed::at(&d, shift(3.0))], tol())
            .expect("inner gather");
    assert_eq!(two.solids().count(), 2);
    carried_is_derived(
        "multi-solid twice",
        &[
            Placed::own(&two),
            Placed::at(&two, shift(10.0)),
            Placed::at(&two, turn(0.7).compose(shift(-10.0)).unwrap()),
        ],
    );
}

/// Claim 1: members standing in one another. What does a finish of the
/// aggregate from values say, which the gather no longer asks?
#[test]
fn probe_nested_and_overlapping_members_old_finish_vs_gather() {
    let big = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let small = brick((0.4, 0.6), (0.4, 0.6), (0.4, 0.6));
    let h = hollow();
    let cases: Vec<(&str, Vec<Placed<'_, AtRestBody<f64>>>)> = vec![
        (
            "small inside big",
            vec![Placed::own(&big), Placed::own(&small)],
        ),
        (
            "small in the cavity",
            vec![Placed::own(&h), Placed::own(&small)],
        ),
        (
            "overlap half",
            vec![Placed::own(&big), Placed::at(&big, shift(0.5))],
        ),
        (
            "one place twice",
            vec![Placed::own(&big), Placed::own(&big)],
        ),
        (
            "touching faces",
            vec![Placed::own(&big), Placed::at(&big, shift(1.0))],
        ),
    ];
    for (what, members) in cases {
        let (g, _) = topo::boolean::gather_finished(&members, tol())
            .unwrap_or_else(|e| panic!("{what}: {e:?}"));
        let v = g.measure(res()).unwrap().enclosure.volume;
        let derived = topo::finish(g.into_body(), tol());
        eprintln!(
            "PROBE {what}: gather Ok (volume {v}); finish from values: {}",
            match &derived {
                Ok(d) => format!("Ok (volume {})", d.measure(res()).unwrap().enclosure.volume),
                Err(e) => format!("Err {e:?}"),
            }
        );
    }
}

/// At plain Interval with a zero-width turn the aggregate also finishes
/// from values: carried and derived compared there.
#[test]
fn probe_carried_is_derived_at_interval_point_turn() {
    let mk = |x: (f64, f64)| {
        topo::finish(
            common::brick::<Interval>(x, (-0.005, 0.005), (0.0, 0.01), tol()),
            tol(),
        )
        .unwrap()
    };
    let (a, b) = (mk((-0.02, -0.01)), mk((0.03, 0.04)));
    let t = RigidMotion::rotation(UnitVec3::exact_z(), Interval::from_bounds(0.02, 0.02)).unwrap();
    let (g, _) =
        topo::boolean::gather_finished(&[Placed::own(&a), Placed::at(&b, t)], tol()).unwrap();
    let c = g.measure(res()).unwrap().enclosure;
    let d = topo::finish(g.into_body(), tol()).expect("derived at a point Interval");
    let d = d.measure(res()).unwrap().enclosure;
    eprintln!("interval carried {c:?}\ninterval derived {d:?}");
    // The carried runs were read in the member's own frame, so they are
    // the narrower: the carried enclosure lies inside the derived one.
    use geom_core::Bounds;
    for (cv, dv) in [(c.volume, d.volume), (c.surface_area, d.surface_area)] {
        assert!(
            dv.lo() <= cv.lo() && cv.hi() <= dv.hi(),
            "carried {cv:?} inside derived {dv:?}"
        );
    }
}

/// Claim 3: `FinishedBody::gathered` is a public trait method, so a
/// caller outside the gather can hand it ANY body beside a finished
/// part's certificate and keys, and get a finished body back with no
/// check run. Here: a 2 m brick handed the certificate of a 1 cm one.
#[test]
fn probe_the_trait_method_finishes_a_body_it_was_not_the_graft_of() {
    use topo::FinishedBody;
    let small = brick((0.0, 0.01), (0.0, 0.01), (0.0, 0.01));
    let (_, keys) = topo::boolean::gather_finished(&[Placed::own(&small)], tol()).unwrap();
    let other = common::brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0), tol());
    let forged = <AtRestBody<f64> as FinishedBody>::gathered(other, &[&small], &keys)
        .expect("the trait method checks nothing at a certifying scalar");
    let v = forged.measure(res()).unwrap().enclosure.volume;
    eprintln!("PROBE forged: a finished 2 m brick measures volume {v}");
    assert!(
        (v - 1e-6).abs() < 1e-15,
        "the forged body reports the small brick's volume: {v}"
    );
}

/// Claim 3/8: the same public trait method panics on input a caller
/// can write: no part at all (`unreachable!` in `KeptSign::gathered`).
#[test]
#[should_panic(expected = "a gather grafts at least one part")]
fn probe_the_trait_method_panics_on_no_part() {
    use topo::FinishedBody;
    let any = common::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol());
    let _ = <AtRestBody<f64> as FinishedBody>::gathered(any, &[], &[]);
}

/// ... and on keys that are not the part's graft's
/// (`unreachable!` in `KeptSign::rekeyed`).
#[test]
#[should_panic(expected = "a graft maps every face of its source")]
fn probe_the_trait_method_panics_on_keys_of_another_graft() {
    use topo::FinishedBody;
    let d = drum();
    let (_, keys) = topo::boolean::gather_finished(&[Placed::own(&d)], tol()).unwrap();
    let b = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let any = common::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol());
    let _ = <AtRestBody<f64> as FinishedBody>::gathered(any, &[&b], &keys);
}
