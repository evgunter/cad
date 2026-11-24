//! **A gather of finished members finishes by carrying their verdicts**
//! ([`topo::gather`]): each member finished once in its own frame, then
//! placed by a rigid motion, the aggregate asking nothing its members'
//! finishes answered.
//!
//! The members are two bricks, one at rest and one turned about `z` by a
//! joint's box of `±0.03` rad, a few centimetres off the axis, so each
//! moved corner and each moved carrier is as wide as the box turns it.
//! Finishing that aggregate again from values asks whether a moved edge's
//! carrier starts at its moved vertex: two enclosures of one point, as
//! wide as the box, which plain `Interval` cannot decide.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::{Interval, ParamSymbol, RigidMotion, Sym, SymBudget, Tol, UnitVec3};
use topo::{AtRestPolicy, Body, Placed};

/// A brick a few centimetres off the `z` axis.
fn brick<T: AtRestPolicy + geom_core::Decide>(x0: f64) -> Body<T> {
    common::brick::<T>(
        (x0, x0 + 0.01),
        (-0.005, 0.005),
        (0.0, 0.01),
        Tol::witness(),
    )
}

/// The two members finished in their own frames, and the turn.
fn members<T: AtRestPolicy + geom_core::Decide>(
    angle: T,
) -> (T::Finished, T::Finished, RigidMotion<T>) {
    let tol = Tol::witness();
    let rest = topo::finish(brick::<T>(-0.02), tol).expect("the brick at rest finishes");
    let turned = topo::finish(brick::<T>(0.03), tol).expect("the brick to turn finishes");
    let turn = RigidMotion::rotation(UnitVec3::exact_z(), angle).expect("a finite angle");
    (rest, turned, turn)
}

/// A measurement's resolution, ε's.
fn resolution<T: geom_core::Real>() -> topo::Settle<T> {
    topo::Settle::Resolution(quantity::Length::from_meters(Tol::witness().get().eps))
}

/// The joint's box, at plain `Interval`.
fn joint() -> Interval {
    Interval::from_bounds(-0.03, 0.03)
}

/// **At plain `Interval` the gather finishes, where finishing the same
/// graft from values refuses.** The second half is the reason for the
/// first: the aggregate's own finish, the gather's end before it carried,
/// escalates on the moved edges' endpoint checks.
#[test]
fn a_gather_under_a_boxed_turn_finishes_at_interval_where_a_finish_from_values_refuses() {
    let tol = Tol::witness();
    let (rest, turned, turn) = members::<Interval>(joint());
    let (gathered, keys) = topo::gather(&[Placed::own(&rest), Placed::at(&turned, turn)], tol)
        .expect("a gather of finished members under a rigid motion finishes");
    assert_eq!(keys.len(), 2, "one key bridge per member");
    assert_eq!(gathered.solids().count(), 2, "one solid per member's solid");
    let derived = topo::finish(gathered.into_body(), tol)
        .expect_err("the same aggregate finished from values refuses at plain Interval");
    let text = format!("{derived:?}");
    assert!(
        text.contains("carrier_endpoint_start"),
        "the finish from values refuses on the moved endpoints: {text}"
    );
}

/// **At `Sym<Interval>`, under a symbolic joint, the gather finishes and
/// its volume is the members' own**: the aggregate's measurement reads
/// through the certificates the members' finishes kept, moved, so it
/// encloses the two bricks' volume without reading a moved face again.
#[test]
fn a_gather_under_a_symbolic_turn_finishes_and_measures_its_members_volume() {
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    let (volume, _) = geom_core::sym::with_session(budget, || {
        let angle = Sym::param_over(
            ParamSymbol::new(test_utils::symbol_id("gather_joint")),
            joint(),
            -0.03,
            0.03,
        );
        let tol = Tol::witness();
        let (rest, turned, turn) = members::<Sym<Interval>>(angle);
        let (gathered, _) = topo::gather(&[Placed::own(&rest), Placed::at(&turned, turn)], tol)
            .expect("a gather under a symbolic turn finishes");
        let measured = gathered
            .measure(resolution())
            .expect("the gathered certificate measures");
        measured.enclosure.volume
    });
    let want = 2.0 * 1e-6;
    let v = volume;
    assert!(
        geom_core::Bounds::lo(v) <= want && want <= geom_core::Bounds::hi(v),
        "the aggregate's volume encloses the two bricks' {want}: {v:?}"
    );
}

/// **At a point scalar the carried finish is the derived one**: the
/// aggregate a gather carries also finishes from values, and its
/// measured volume agrees with the derivation's. This is the kernel-bug
/// check of a carry (`[failure.taxonomy]` row 5), pinned as a row: a
/// gather whose carried verdict no derivation reproduces reds here.
#[test]
fn at_f64_a_carried_gather_is_what_a_finish_derives() {
    fn at<
        T: AtRestPolicy<Finished = topo::AtRestBody<T>> + geom_core::Decide + geom_core::Bounds,
    >(
        angle: T,
        what: &str,
    ) {
        let tol = Tol::witness();
        let (rest, turned, turn) = members::<T>(angle);
        let (gathered, _) = topo::gather(&[Placed::own(&rest), Placed::at(&turned, turn)], tol)
            .unwrap_or_else(|e| panic!("{what}: the gather finishes: {e:?}"));
        let carried = gathered
            .measure(resolution())
            .unwrap_or_else(|e| panic!("{what}: the carried certificate measures: {e:?}"))
            .enclosure
            .volume;
        let derived = topo::finish(gathered.into_body(), tol)
            .unwrap_or_else(|e| panic!("{what}: the aggregate also finishes from values: {e:?}"))
            .measure(resolution())
            .unwrap_or_else(|e| panic!("{what}: the derived certificate measures: {e:?}"))
            .enclosure
            .volume;
        let mid = |v: T| (geom_core::Bounds::lo(v) + geom_core::Bounds::hi(v)) / 2.0;
        let (c, d) = (mid(carried), mid(derived));
        assert!(
            (c - d).abs() <= 1e-15 && (c - 2e-6).abs() <= 1e-15,
            "{what}: the carried volume {c} is the derived {d}, the two bricks' 2e-6"
        );
    }
    at::<f64>(0.02, "f64");
}

/// **Every lane records the same gather**: the decisions a gather logs
/// (the session box asked of each moved image, nothing else) are the
/// same rows at `f64` and at `Interval`, so a drive's witness and its
/// leaf compare equal. A gather that derived at one lane and carried at
/// another would log a different vector here.
#[test]
fn a_gather_logs_the_same_verdicts_at_every_lane() {
    fn logged<T: AtRestPolicy + geom_core::Decide>(angle: T) -> Vec<geom_core::k_stats::Verdict> {
        let (rest, turned, turn) = members::<T>(angle);
        let bracket = geom_core::k_stats::Bracket::open();
        topo::gather(
            &[Placed::own(&rest), Placed::at(&turned, turn)],
            Tol::witness(),
        )
        .expect("the gather finishes");
        bracket.finish().verdicts
    }
    let point = logged::<f64>(0.02);
    let enclosed = logged::<Interval>(Interval::from_bounds(0.02, 0.02));
    assert!(
        !point.is_empty(),
        "the gather asks the session box of its moved image"
    );
    assert_eq!(
        point, enclosed,
        "the f64 and Interval gathers log one vector"
    );
}
