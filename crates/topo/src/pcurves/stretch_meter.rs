#![allow(clippy::unwrap_used, clippy::float_cmp)]

use super::{DescribedChart, Singular, chart_u_arm, singular_at, v_meter};
use geom::{NurbsSurface, Surface};
use geom_core::k_stats::decide;
use geom_core::spline::KnotVector;
use geom_core::{Band, Margin, Point3, Sign, SupSpeed, Vec3};
use std::sync::Arc;

/// The band every row here pins explicitly: `zero = 1e-9`,
/// `escalate = 1e-8`. Pinned rather than read from the run's
/// tolerance so the digits below mean the same thing at every ε
/// the sweep runs.
fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

/// A bilinear NURBS chart on `[0, 1]²` whose image is a flat
/// `span × span` metre square: `S(u, v) = (span·u, span·v, 0)`,
/// so `|S_u| = |S_v| = span` EVERYWHERE — the chart's metre
/// stretch is exactly `span`, not 1.
fn flat_chart(span: f64) -> Surface<f64> {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, span, 0.0),
        Point3::new(span, 0.0, 0.0),
        Point3::new(span, span, 0.0),
    ];
    Surface::Nurbs(Arc::new(
        NurbsSurface::new(
            geom_core::spline::Knots::from(kv.clone()).lift(geom_core::Real::from_f64),
            geom_core::spline::Knots::from(kv).lift(geom_core::Real::from_f64),
            control,
            vec![geom_core::Real::one(); 4],
        )
        .unwrap(),
    ))
}

fn chart(s: &Surface<f64>) -> DescribedChart<'_, f64> {
    DescribedChart::of(s).unwrap()
}

fn plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// **The red-first row, azimuth channel.** A 100× chart maps a
/// `1e-10` chart-unit u gap to `1e-8` metres — a DEFINITE
/// discontinuity at the pinned band. Metred by 1 it reads `1e-10`
/// and certifies `Zero`: the loop closes on a gap the kernel can
/// see. The arm is the whole of the defect.
#[test]
fn a_stretched_nurbs_chart_meters_its_azimuth_gap_in_metres() {
    let s = flat_chart(100.0);
    let arm = chart_u_arm(chart(&s));
    assert_eq!(arm.get(), 100.0, "the chart's own metre stretch, not 1");
    let gap = 1e-10;
    assert_eq!(
        decide(
            "pcurve_loop_continuity",
            Margin::metered_sup(gap, arm),
            band()
        ),
        Ok(Sign::Positive),
        "1e-10 chart units × 100 m/unit = 1e-8 m, at the escalate edge"
    );
    assert_eq!(
        decide(
            "pcurve_loop_continuity",
            Margin::metered_sup(gap, SupSpeed::new(1.0)),
            band()
        ),
        Ok(Sign::Zero),
        "the under-stated arm certifies the same loop closed"
    );
}

/// **The red-first row, second channel.** Same digits, `v_meter`.
#[test]
fn a_stretched_nurbs_chart_meters_its_second_channel_in_metres() {
    let s = flat_chart(100.0);
    let meter = v_meter(chart(&s));
    assert_eq!(meter.get(), 100.0);
    let gap = 1e-10;
    assert_eq!(
        decide(
            "pcurve_loop_continuity",
            Margin::metered_sup(gap, meter),
            band()
        ),
        Ok(Sign::Positive)
    );
    assert_eq!(
        decide(
            "pcurve_loop_continuity",
            Margin::metered_sup(gap, SupSpeed::new(1.0)),
            band()
        ),
        Ok(Sign::Zero)
    );
}

/// **The scale twin.** The same loop at a uniform 1e3 scale: the
/// arm scales exactly with the model, so a chart gap that was
/// `1e-8` m is `1e-5` m — the metering carries the scale rather
/// than fixing a metre size into the chart.
#[test]
fn the_stretch_arm_carries_a_uniform_scale() {
    let small = flat_chart(100.0);
    let large = flat_chart(100.0e3);
    assert_eq!(
        chart_u_arm(chart(&large)).get(),
        chart_u_arm(chart(&small)).get() * 1e3
    );
    assert_eq!(
        v_meter(chart(&large)).get(),
        v_meter(chart(&small)).get() * 1e3
    );
}

/// **The plane arm is 1 by construction, not by default**, and
/// stays bit-identical: a plane chart's u and v ARE metres.
#[test]
fn a_plane_chart_keeps_its_exact_unit_arms() {
    let p = plane();
    assert_eq!(chart_u_arm(chart(&p)).get(), 1.0);
    assert_eq!(v_meter(chart(&p)).get(), 1.0);
}

/// **Three-outcome posture on the newly-honest arm.** A chart gap
/// whose metred size lands INSIDE the band escalates typed rather
/// than picking a side: `5e-11 × 100 = 5e-9 ∈ (1e-9, 1e-8)`.
#[test]
fn an_in_band_metred_gap_escalates_rather_than_deciding() {
    let s = flat_chart(100.0);
    let arm = chart_u_arm(chart(&s));
    assert!(
        decide(
            "pcurve_loop_continuity",
            Margin::metered_sup(5e-11, arm),
            band()
        )
        .is_err(),
        "in-band residue is the third outcome, not a verdict"
    );
    assert_eq!(
        decide(
            "pcurve_loop_continuity",
            Margin::metered_sup(5e-12, arm),
            band()
        ),
        Ok(Sign::Zero),
        "5e-10 m is honestly closed"
    );
}

/// **The singular gate answers all three ways**, through
/// [`singular_at`], the one door the walk reads. On a spline chart
/// it is net-level (the metres the chart's `u` knot domain moves a
/// point, whatever the vertex); on a sphere it is the vertex's 3-D distance to the
/// nearer pole.
#[test]
fn the_singular_gate_answers_all_three_ways() {
    let anywhere = Point3::new(0.3, 0.4, 0.0);
    let read = |s: &Surface<f64>, at: Point3<f64>| singular_at(chart(s), at, band());
    // A chart whose whole u stretch is 1e-12 m per chart unit: no u
    // displacement on it moves a point past the band.
    assert!(
        matches!(read(&flat_chart(1e-12), anywhere), Singular::Collapsed),
        "a sub-band chart stretch is a collapsed lever"
    );
    for span in [5e-9_f64, 2e-9] {
        assert!(
            matches!(read(&flat_chart(span), anywhere), Singular::Undecided(_)),
            "an in-band stretch ({span:e}) escalates rather than deciding"
        );
    }
    assert!(matches!(read(&flat_chart(100.0), anywhere), Singular::Off));
    // The sphere: on a pole, within the band of one, and clear of both.
    let sphere: Surface<f64> = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 2.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    assert!(matches!(
        read(&sphere, Point3::new(0.0, 0.0, -2.0)),
        Singular::On(_)
    ));
    assert!(matches!(
        read(&sphere, Point3::new(5e-9, 0.0, 2.0)),
        Singular::Undecided(_)
    ));
    assert!(matches!(
        read(&sphere, Point3::new(2.0, 0.0, 0.0)),
        Singular::Off
    ));
    // A cylinder has no singular set, so it decides nothing, even on
    // its axis.
    let cylinder: Surface<f64> = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    assert!(matches!(
        read(&cylinder, Point3::new(0.0, 0.0, 0.0)),
        Singular::Off
    ));
}

/// **On a spline chart, a gate that reads collapsed decides no
/// period, and one that is undecided refuses**: a net whose `u` knot
/// domain moves a point under the band (`net_reach`) names no
/// azimuth integer, and the joint's chart-space gap is what decides
/// it (`spline_gap_closes`, here a zero gap).
/// Read off a net whose reach is under the band, in it, and past it,
/// on a chart given a period of 1: an identity with no period read
/// (every lift names the point), a refusal, and an ordinary
/// identity. The net's control points are all within the band of
/// each other below the band, so it also reads closed in `v`
/// (`chart_v_period`); its `v` reach gates the `v` periods the same
/// way (`pcurve_loop_v_stretch`).
#[test]
fn a_spline_gate_that_is_undecided_refuses_and_writes_nothing() {
    use super::{PinMiss, decide_plain_joint};
    use crate::joint::JointElement;
    use geom_brep::Pcurve;
    use geom_core::{Point2, Vec2};
    let zero = Vec2::new(0.0, 0.0);
    let at = Point2::new(0.4, 0.5);
    let image = Pcurve::free_harmonic(at, zero, zero, zero);
    for (span, decided) in [(1e-12, true), (5e-9, false), (100.0, true)] {
        let s = flat_chart(span);
        let vertex = Point3::new(0.4 * span, 0.5 * span, 0.0);
        let out = decide_plain_joint(
            chart(&s),
            &image,
            0.0,
            at,
            vertex,
            super::ChartPeriods::given_u(Some(geom_brep::Period::new(1.0)), &s, band()),
            band(),
        );
        if decided {
            assert!(
                matches!(out, Ok(JointElement::IDENTITY)),
                "span {span:e}: the identity, read {:?}",
                out.as_ref().ok()
            );
        } else {
            assert!(
                matches!(out, Err(PinMiss::NetStretchUndecided(_))),
                "span {span:e}: an undecided u gate refuses, read {:?}",
                out.as_ref().ok()
            );
        }
    }
}

/// **A joint clear of the apex, or within the band of a pole, takes
/// a lift of its own point at any gap on its orbit.** At `α = 0.01`
/// a vertex `d` from the apex is decided `Off` the singular set from
/// `d` past the band's escalate edge on, while its lever is `d·sin α`,
/// a hundredth of that: under the band until `d` is a hundred times
/// the edge. A continuous joint there (gap 0, a thousandth of a
/// radian, a whole turn) reads every mark Zero or in band, and takes
/// the orbit point its gap meets at that lever by the near-pole read
/// ([`super::lever_periods`]): the image itself, or the image a
/// period over. On a sphere, a vertex within the band of a pole reads
/// the same way in half periods, and a whole turn's gap meets the
/// image a period over, two half steps off.
#[test]
fn a_narrow_cone_loop_joint_clear_of_the_apex_takes_a_lift_of_its_point() {
    use super::{NearRead, Orbit, decide_plain_joint, near_pole_gap_closes};
    use crate::joint::{Deck, JointElement};
    use geom_brep::Pcurve;
    use geom_core::Point2;
    let tau = std::f64::consts::TAU;
    let at_gap = |surface: &Surface<f64>, vertex, v: f64, gap: f64| {
        let image = Pcurve::Harmonic {
            u: geom_brep::Channel::fixed_angle(0.0),
            v: geom_brep::Channel::constant(v),
        };
        let prev = Point2::new(gap, v);
        decide_plain_joint(
            chart(surface),
            &image,
            0.0,
            prev,
            vertex,
            super::ChartPeriods::given_u(Some(geom_brep::Period::new(tau)), surface, band()),
            band(),
        )
    };
    let read = |out: Result<JointElement, super::PinMiss>| match out {
        Ok(element) => format!("{element:?}"),
        Err(super::PinMiss::Discontinuity) => "Discontinuity".into(),
        Err(super::PinMiss::Escalated(_)) => "Escalated".into(),
        Err(super::PinMiss::NetStretchUndecided(_)) => "NetStretchUndecided".into(),
        Err(super::PinMiss::Singular(miss)) => format!("{miss:?}"),
    };
    let lift = |u| {
        format!(
            "{:?}",
            JointElement(Deck {
                u,
                v: 0,
                twin: false,
            })
        )
    };
    let alpha = 0.01f64;
    let cone: Surface<f64> = Surface::Cone {
        apex: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        half_angle: alpha,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    for d in [2.0, 5.0, 10.0, 100.0, 1000.0].map(|m| m * band().escalate()) {
        let vertex = Point3::new(d * alpha.sin(), 0.0, d * alpha.cos());
        assert!(
            matches!(singular_at(chart(&cone), vertex, band()), Singular::Off),
            "d = {d:e}: the vertex is decided clear of the apex"
        );
        for (gap, turns) in [(0.0, 0), (1e-3, 0), (tau, 1), (-tau, -1)] {
            assert_eq!(
                read(at_gap(&cone, vertex, d, gap)),
                lift(turns),
                "d = {d:e} (lever {:e}), gap {gap}: the image {turns} periods over",
                d * alpha.sin()
            );
        }
        // Half a turn round, the gap sits on a mark and the read's
        // nearest offsets sit `π·lever` off: within ε, the image is
        // taken; in band, the two chart points (`2·lever` apart in
        // metres) may be one point and the joint escalates; past the
        // band, every offset is decided off and the marks' decided
        // miss stands.
        let lever = d * alpha.sin();
        let pi_lever = core::f64::consts::PI * lever;
        let want = if pi_lever < band().zero() {
            lift(0)
        } else if pi_lever < band().escalate() {
            "Escalated".to_owned()
        } else {
            "Discontinuity".to_owned()
        };
        assert_eq!(
            read(at_gap(&cone, vertex, d, 0.5 * tau)),
            want,
            "d = {d:e} (lever {lever:e}), gap π"
        );
    }
    let sphere: Surface<f64> = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 2.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let near_pole = Point3::new(5e-9, 0.0, 2.0);
    assert!(
        matches!(
            singular_at(chart(&sphere), near_pole, band()),
            Singular::Undecided(_)
        ),
        "the vertex is within the band of the pole"
    );
    let v = (near_pole.z / 2.0).asin();
    for (gap, turns) in [(0.0, 0), (tau, 1), (-tau, -1)] {
        assert_eq!(
            read(at_gap(&sphere, near_pole, v, gap)),
            lift(turns),
            "sphere near its pole, gap {gap}: the image {turns} periods over"
        );
    }
    // Three half turns: the twin a period over, which the orbit's own
    // offsets do not reach and the branches astride the walk's mark do.
    assert_eq!(
        read(at_gap(&sphere, near_pole, v, 1.5 * tau)),
        format!(
            "{:?}",
            JointElement(Deck {
                u: 1,
                v: 0,
                twin: true,
            })
        ),
        "sphere near its pole, gap 3π: the twin a period over"
    );
    // Where a whole turn at the lever is itself within ε, the image
    // and the image a period over both close, and the image is taken
    // first. Through `decide_joint` a sphere never reaches that: its
    // read runs only where the incidence is undecided, so its lever
    // is past the band's zero edge and no two offsets are within ε.
    // So the order is pinned at the read itself.
    let both = SupSpeed::new(0.05 * band().zero());
    let one = SupSpeed::new(0.5 * band().zero());
    for (orbit, period_over) in [(Orbit::Periods, 1), (Orbit::HalfPeriodsWithTwin, 2)] {
        for (sign, gap) in [(1, tau), (-1, -tau)] {
            let first =
                near_pole_gap_closes(both, gap, geom_brep::Period::new(tau), orbit, None, band());
            assert!(
                matches!(first, NearRead::Closes(0)),
                "{orbit:?}, gap {gap}, a whole turn within ε: the image itself first: {first:?}"
            );
            let over =
                near_pole_gap_closes(one, gap, geom_brep::Period::new(tau), orbit, None, band());
            assert!(
                matches!(over, NearRead::Closes(m) if m == sign * period_over),
                "{orbit:?}, gap {gap}, only the image a period over within ε: {over:?}"
            );
        }
    }
}

/// **A gap on a mark answers by the band at the lever, however many
/// turns out.** On the `α = 0.01` cone, a gap of `k + ½` turns sits
/// on a mark, and the orbit points astride it are `π·lever` off. Within
/// ε the read takes one; in band it escalates; past the band the
/// marks' decided miss stands. The read tries the branches astride
/// the mark the walk stopped on, not only those about the image, so
/// the answer is the same at every `k`. Levers sit just either side
/// of each band edge, so a mutant moving an edge dies. A whole turn's
/// gap at those levers is a lift a whole number of periods over.
#[test]
fn a_half_turn_gap_answers_by_the_band_at_every_turn_count() {
    use super::decide_plain_joint;
    use crate::joint::JointElement;
    use geom_brep::Pcurve;
    use geom_core::Point2;
    let alpha = 0.01f64;
    let cone: Surface<f64> = Surface::Cone {
        apex: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        half_angle: alpha,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let (pi, tau) = (core::f64::consts::PI, core::f64::consts::TAU);
    let (zero, esc) = (band().zero(), band().escalate());
    let at = |lever: f64, gap: f64| {
        let d = lever / alpha.sin();
        let vertex = Point3::new(lever, 0.0, d * alpha.cos());
        let image = Pcurve::Harmonic {
            u: geom_brep::Channel::fixed_angle(0.0),
            v: geom_brep::Channel::constant(d),
        };
        let prev = Point2::new(gap, d);
        match decide_plain_joint(
            chart(&cone),
            &image,
            0.0,
            prev,
            vertex,
            super::ChartPeriods::given_u(Some(geom_brep::Period::new(tau)), &cone, band()),
            band(),
        ) {
            Ok(JointElement(deck)) if deck.v == 0 && !deck.twin => {
                format!("u = {}", deck.u)
            }
            Ok(other) => format!("{other:?}"),
            Err(super::PinMiss::Discontinuity) => "Discontinuity".into(),
            Err(super::PinMiss::Escalated(_)) => "Escalated".into(),
            Err(super::PinMiss::NetStretchUndecided(_)) => "NetStretchUndecided".into(),
            Err(super::PinMiss::Singular(miss)) => format!("{miss:?}"),
        }
    };
    // `π·lever` as a fraction of the escalate edge, in band: the
    // reviewers' levers, and a vertex `22.3·escalate` from the apex.
    let in_band = [0.55, 0.7, 0.75, 0.85, 0.95, 0.99]
        .map(|f| f * esc / pi)
        .into_iter()
        .chain([22.3 * esc * alpha.sin(), 1.01 * zero / pi]);
    for lever in in_band {
        for turns in [0.5, -0.5, 1.5, -1.5, 2.5, -2.5, 3.5, -3.5] {
            assert_eq!(
                at(lever, turns * tau),
                "Escalated",
                "π·lever {:e}, gap {turns}τ: the branches astride the mark are in band",
                pi * lever
            );
        }
    }
    // Just past the escalate edge every branch astride is decided off.
    let past = 1.01 * esc / pi;
    for turns in [0.5, -0.5, 2.5, -2.5, 3.5] {
        assert_eq!(
            at(past, turns * tau),
            "Discontinuity",
            "π·lever {:e}, gap {turns}τ: decided off every branch astride",
            pi * past
        );
    }
    // Just under the zero edge the image itself is within ε.
    assert_eq!(at(0.99 * zero / pi, 0.5 * tau), "u = 0", "gap π within ε");
    // A whole number of turns at levers with the marks in band.
    for lever in [0.55 * esc / pi, 0.95 * esc / pi, past] {
        for (turns, u) in [(1.0, 1), (2.0, 2), (-2.0, -2)] {
            assert_eq!(
                at(lever, turns * tau),
                format!("u = {u}"),
                "π·lever {:e}, gap {turns}τ: the image {u} periods over",
                pi * lever
            );
        }
    }
    // Two turns at a sub-band lever: the walk stops on its first
    // mark, half a turn from the image and undecided there, before it
    // reaches the gap, so no branch astride locates it; the marks'
    // own escalation answers, never a decided miss.
    for lever in [0.2 * zero, 0.5 * zero] {
        assert_eq!(
            at(lever, 2.0 * tau),
            "Escalated",
            "lever {lever:e}, gap 2τ: the walk's first mark is undecided"
        );
    }
}

/// **Each branch astride the walk's mark is tried.** A gap just off
/// a half-turn mark sits unequally far from the two branches astride
/// it: at `π·lever = escalate + ε/2`, a gap `(k + ½)·τ ± 0.8·ε/lever`
/// leaves the nearer branch in band and the farther one decided off.
/// The read must reach whichever is the nearer: the escalation is
/// its own, at either side of the mark and at every `k`.
#[test]
fn each_branch_astride_the_walks_mark_is_tried() {
    let (pi, tau) = (core::f64::consts::PI, core::f64::consts::TAU);
    let (zero, esc) = (band().zero(), band().escalate());
    let lever = (esc + 0.5 * zero) / pi;
    for (turns, side) in [
        (2.5, 1.0),
        (2.5, -1.0),
        (-2.5, 1.0),
        (-2.5, -1.0),
        (0.5, 1.0),
        (0.5, -1.0),
    ] {
        let gap = turns * tau + side * 0.8 * zero / lever;
        let out = super::lever_periods(
            "each_branch_row",
            SupSpeed::new(lever),
            gap,
            geom_brep::Period::new(tau),
            super::Orbit::Periods,
            band(),
        );
        assert!(
            matches!(&out, Err(geom_brep::BranchMiss::Undecided(cause))
                    if cause.predicate == Some("pcurve_loop_pole_gap")),
            "gap {turns}τ {side:+}·0.8ε/lever: the nearer branch is in band: {out:?}"
        );
    }
}

/// **The first in-band candidate's cause is the one answered.**
/// Two candidates in band, neither Zero: the escalation carries the
/// first one's margin, whichever is larger.
#[test]
fn first_within_answers_the_first_deferred_cause() {
    let margins = [3e-9, 7e-9];
    let out = geom_brep::first_within("first_within_row", margins, band(), |&m| Margin::of(m));
    let want = decide("first_within_row", Margin::of(margins[0]), band());
    let got = out.err().map(|cause| format!("{:?}", cause.margin));
    let first = want.err().map(|cause| format!("{:?}", cause.margin));
    assert!(first.is_some(), "the first margin is in band");
    assert_eq!(got, first, "the first in-band candidate's margin");
    assert!(
        matches!(
            geom_brep::first_within("first_within_row", [7e-9, 0.0], band(), |&m| Margin::of(m)),
            Ok(Some(m)) if m == 0.0
        ),
        "an in-band candidate does not stop a later Zero"
    );
}

/// **An undecided mark keeps its own cause.** At a lever of half ε a
/// gap of two turns stops the marks' walk on its first mark, in band,
/// and the near-pole read's offsets are in band too: the escalation
/// is the marks' (`name`), not the read's, so a caller that reads
/// the predicate sees the walk it asked for.
#[test]
fn an_undecided_mark_keeps_its_own_cause() {
    let arm = SupSpeed::new(0.5 * band().zero());
    let tau = std::f64::consts::TAU;
    let out = super::lever_periods(
        "undecided_mark_row",
        arm,
        2.0 * tau,
        geom_brep::Period::new(tau),
        super::Orbit::Periods,
        band(),
    );
    assert!(
        matches!(&out, Err(geom_brep::BranchMiss::Undecided(cause))
                if cause.predicate == Some("undecided_mark_row")),
        "the marks' own escalation: {out:?}"
    );
}

/// A placeholder payload has no net to bound, so it has no arms:
/// it is not a [`DescribedChart`], and these meters cannot be
/// asked about it.
#[test]
fn a_placeholder_chart_is_not_a_described_chart() {
    let s: Surface<f64> = Surface::Nurbs(Arc::new(NurbsSurface::placeholder()));
    assert!(DescribedChart::of(&s).is_none());
}
