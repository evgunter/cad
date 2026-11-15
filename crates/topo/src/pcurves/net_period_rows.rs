use super::{ChartPeriods, DescribedChart, PinMiss, chart_v_period, decide_plain_joint};
use crate::joint::{Deck, JointElement};
use geom::Surface;
use geom_brep::Pcurve;
use geom_core::spline::KnotVector;
use geom_core::{Band, Decide, Interval, Point2, Point3, Tol, Vec2};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn eps() -> f64 {
    Tol::witness().eps()
}

/// The prism's cross-section, a triangle of circumradius `size`.
fn tri(size: f64) -> [(f64, f64); 3] {
    [
        (size, 0.0),
        (-0.5 * size, 0.75_f64.sqrt() * size),
        (-0.5 * size, -(0.75_f64.sqrt()) * size),
    ]
}

/// The prism as one net: `u` the height over `[0, 1]`, `v` around
/// over `[a, a + s]` with interior knots at 0.3 and 0.7 of it, its
/// last row the point `last` (the first row's for a net closed in
/// `v`).
fn net(size: f64, (a, s): (f64, f64), last: (f64, f64)) -> Surface<f64> {
    let [c0, c1, c2] = tri(size);
    let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let kv = KnotVector::clamped(vec![a, a, a + 0.3 * s, a + 0.7 * s, a + s, a + s], 1).unwrap();
    let control = [0.0, 1.0]
        .into_iter()
        .flat_map(|z| [c0, c1, c2, last].map(|c| Point3::new(c.0, c.1, z)))
        .collect();
    Surface::Nurbs(std::sync::Arc::new(
        geom::NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap(),
    ))
}

fn closed(size: f64, domain: (f64, f64)) -> Surface<f64> {
    net(size, domain, tri(size)[0])
}

/// The joint into an iso line along `v` entering at `entry_v`, its
/// predecessor's exit at `prev_v`, both at height 0.5, at `T`.
fn joint<T: Decide>(s: &Surface<f64>, entry_v: f64, prev_v: f64) -> Result<JointElement, PinMiss> {
    let s: Surface<T> = s.map_scalar(T::from_f64);
    let f = T::from_f64;
    let image = Pcurve::IsoLine {
        p0: Point2::new(f(0.5), f(entry_v)),
        pl: Vec2::new(f(0.0), f(0.1)),
        ends: None,
    };
    decide_plain_joint(
        DescribedChart::of(&s).unwrap(),
        &image,
        f(0.0),
        Point2::new(f(0.5), f(prev_v)),
        Point3::new(f(1.0), f(0.0), f(0.5)),
        ChartPeriods::of(&s, band()),
        band(),
    )
}

fn shift(u: i32, v: i32) -> JointElement {
    JointElement(Deck { u, v, twin: false })
}

/// **`kv` reads the net's own domain**, offset and scaled, crossing
/// the seam either way, at `f64` and at `Interval`: entering at `a`
/// from `a + s` is one period up, the reverse one down, and a joint
/// that crosses nothing is the identity. Two periods out the marks
/// gallop to 2; exactly on a half-period mark no branch is named.
#[test]
fn kv_reads_an_offset_scaled_domain_both_ways_on_both_lanes() {
    for (a, s) in [(0.0, 1.0), (2.0, 3.0), (-7.5, 0.25), (100.0, 40.0)] {
        let n = closed(1.0, (a, s));
        assert_eq!(
            chart_v_period(&n, band()).map(|p| p.period().length()),
            Some(s),
            "({a}, {s})"
        );
        for (entry, prev, want) in [(a, a + s, 1), (a + s, a, -1), (a, a, 0), (a + s, a + s, 0)] {
            assert_eq!(
                joint::<f64>(&n, entry, prev).ok(),
                Some(shift(0, want)),
                "f64 ({a}, {s}) {entry} <- {prev}"
            );
            assert_eq!(
                joint::<Interval>(&n, entry, prev).ok(),
                Some(shift(0, want)),
                "Interval ({a}, {s}) {entry} <- {prev}"
            );
        }
        assert_eq!(
            joint::<f64>(&n, a, a + 2.0 * s).ok(),
            Some(shift(0, 2)),
            "two periods out"
        );
        assert!(
            joint::<f64>(&n, a, a + 0.5 * s).is_err(),
            "on a half-period mark"
        );
    }
}

/// **A net's `v` gate reads metres, not a rate.** A unit prism over a
/// `v` knot domain 10¹⁰ long stretches under ε per parameter unit
/// (`sup |S_v|` ≈ 6·10⁻¹⁰), yet a whole period moves a point metres
/// (`net_reach`), so the seam crossing is one period, not a gap of
/// metres refused as a discontinuity.
#[test]
fn a_long_v_domain_crosses_its_seam_by_one_period() {
    let n = closed(1.0, (0.0, 1e10));
    assert_eq!(joint::<f64>(&n, 0.0, 1e10).ok(), Some(shift(0, 1)));
    assert_eq!(joint::<Interval>(&n, 0.0, 1e10).ok(), Some(shift(0, 1)));
}

/// **The same of `u`** ([`super::singular_at`]'s net-level gate): the
/// prism closed in `u` over a `u` knot domain 10¹⁰ long crosses its
/// seam by one `u` period. Read as a rate, its stretch sits under
/// the band and the gate read no `u` integer, its gap of metres a
/// discontinuity.
#[test]
fn a_long_u_domain_crosses_its_seam_by_one_period() {
    let k = 1e10;
    let [c0, c1, c2] = tri(1.0);
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.3 * k, 0.7 * k, k, k], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [c0, c1, c2, c0]
        .into_iter()
        .flat_map(|c| [Point3::new(c.0, c.1, 0.0), Point3::new(c.0, c.1, 1.0)])
        .collect();
    let s = Surface::Nurbs(std::sync::Arc::new(
        geom::NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap(),
    ));
    let out = decide_plain_joint(
        DescribedChart::of(&s).unwrap(),
        &Pcurve::IsoLine {
            p0: Point2::new(0.0, 0.5),
            pl: Vec2::new(0.3 * k, 0.0),
            ends: None,
        },
        0.0,
        Point2::new(k, 0.5),
        Point3::new(1.0, 0.0, 0.5),
        ChartPeriods::of(&s, band()),
        band(),
    );
    assert_eq!(out.ok(), Some(shift(1, 0)));
}

/// **An undecided `v` gate refuses under its own name**
/// (`PinMiss::NetStretchUndecided`): a net closed in `v` whose `v`
/// knot domain moves a point within the band (`net_reach`) decides no
/// `v` integer, and the joint writes no element. Decided under the
/// band, the same joint is an ordinary identity
/// (`a_spline_gate_that_is_undecided_refuses_and_writes_nothing`'s
/// flat net); decided past it, the marks read.
#[test]
fn an_undecided_v_gate_refuses_under_its_own_name() {
    let mid = 0.5 * (1.0 + Tol::witness().k()) * eps();
    // The prism's `sup |S_v|` is its widest span: an edge `√3·size`
    // over 0.3 of the domain.
    let size = |reach: f64| reach * 0.3 / 3.0_f64.sqrt();
    let in_band = closed(size(mid), (0.0, 1.0));
    assert!(chart_v_period(&in_band, band()).is_some(), "closed in v");
    assert!(
        matches!(
            joint::<f64>(&in_band, 0.3, 0.3),
            Err(PinMiss::NetStretchUndecided(_))
        ),
        "f64"
    );
    assert!(
        matches!(
            joint::<Interval>(&in_band, 0.3, 0.3),
            Err(PinMiss::NetStretchUndecided(_))
        ),
        "Interval"
    );
    let under = closed(size(0.25 * eps()), (0.0, 1.0));
    assert_eq!(
        joint::<f64>(&under, 0.3, 0.3).ok(),
        Some(shift(0, 0)),
        "decided under"
    );
}

/// A net closed in BOTH channels: `u` around a unit triangle in the
/// `xy` plane, `v` around a triangle of circumradius `minor` in each
/// station's `(radial, z)` plane, both over `[0, 1]` with interior
/// knots at 0.3 and 0.7 (review C-delta's probe fixture).
fn tube(minor: f64) -> Surface<f64> {
    let ring = {
        let [a, b, c] = tri(1.0);
        [a, b, c, a]
    };
    let section = {
        let [a, b, c] = tri(minor);
        [a, b, c, a]
    };
    let k = || KnotVector::clamped(vec![0.0, 0.0, 0.3, 0.7, 1.0, 1.0], 1).unwrap();
    let mut control = Vec::new();
    for (x, y) in ring {
        let r = x.hypot(y);
        for (radial, z) in section {
            control.push(Point3::new(x + radial * x / r, y + radial * y / r, z));
        }
    }
    Surface::Nurbs(std::sync::Arc::new(
        geom::NurbsSurface::new(k(), k(), control, vec![1.0; 16]).unwrap(),
    ))
}

/// The joint into an iso line along `u` entering at `u = 0` from
/// its predecessor's exit at `u = 1`: a crossing of the `u` seam, at
/// `v = 0.5`, at `T`.
fn u_crossing<T: Decide>(s: &Surface<f64>) -> Result<JointElement, PinMiss> {
    let s: Surface<T> = s.map_scalar(T::from_f64);
    let f = T::from_f64;
    decide_plain_joint(
        DescribedChart::of(&s).unwrap(),
        &Pcurve::IsoLine {
            p0: Point2::new(f(0.0), f(0.5)),
            pl: Vec2::new(f(0.3), f(0.0)),
            ends: None,
        },
        f(0.0),
        Point2::new(f(1.0), f(0.5)),
        Point3::new(f(1.0), f(0.0), f(0.0)),
        ChartPeriods::of(&s, band()),
        band(),
    )
}

/// **An undecided `v` gate refuses even where the `u` element is
/// decided.** On a net closed in both channels whose `v` sweep sits
/// in the band, a crossing of the `u` seam decides `u = 1`, but the
/// joint refuses as `NetStretchUndecided`: an element is written
/// whole or not at all. With the `v` sweep decided
/// past the band the same crossing is `Shift(u: 1)`.
#[test]
fn an_undecided_v_gate_refuses_a_decided_u_crossing() {
    let mid = 0.5 * (1.0 + Tol::witness().k()) * eps();
    let in_band = tube(mid * 0.3 / 3.0_f64.sqrt());
    assert!(
        super::chart_u_period(&in_band, band()).is_some(),
        "closed in u"
    );
    assert!(chart_v_period(&in_band, band()).is_some(), "closed in v");
    assert!(
        matches!(
            u_crossing::<f64>(&in_band),
            Err(PinMiss::NetStretchUndecided(_))
        ),
        "f64"
    );
    assert!(
        matches!(
            u_crossing::<Interval>(&in_band),
            Err(PinMiss::NetStretchUndecided(_))
        ),
        "Interval"
    );
    assert_eq!(
        u_crossing::<f64>(&tube(0.1)).ok(),
        Some(shift(1, 0)),
        "v decided"
    );
}

/// **A seam a near-band offset apart** (the net's last row moved off
/// its first by `d`): under the band's zero the net is closed, and
/// the crossing is one period; in the band, the closure folds to
/// open (`work/pcert/chart-u-period-folds-an-in-band-net-closure-into-no-period.md`),
/// no `v` period is offered, and the crossing's whole-domain gap
/// refuses as a decided discontinuity where the honest answer would
/// escalate. This row pins that fold.
#[test]
fn a_seam_a_near_band_offset_apart() {
    let [c0, ..] = tri(1.0);
    let off = |d: f64| net(1.0, (2.0, 3.0), (c0.0 + d, c0.1));
    for d in [0.0, 0.1 * eps(), 0.5 * eps()] {
        let n = off(d);
        assert!(chart_v_period(&n, band()).is_some(), "d = {d:e}: closed");
        assert_eq!(
            joint::<f64>(&n, 2.0, 5.0).ok(),
            Some(shift(0, 1)),
            "d = {d:e}"
        );
    }
    let d = 0.5 * (1.0 + Tol::witness().k()) * eps();
    let n = off(d);
    assert!(
        chart_v_period(&n, band()).is_none(),
        "in band: folded to open"
    );
    assert!(
        matches!(joint::<f64>(&n, 2.0, 5.0), Err(PinMiss::Discontinuity)),
        "the fold refuses as decided"
    );
}
