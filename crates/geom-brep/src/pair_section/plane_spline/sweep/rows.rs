//! **The regularity sweep's rows**: the probe's pairs it proves
//! loop-free, which the guard clears with no whole reading, and the
//! poses it declines, which fall to C3's whole reading. Every row names
//! the mutant that turns it red; each runs at the run's ε (the suite
//! runs 1e-9, 1e-6 and 1e-12).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol, Vec3};

use super::super::PlaneSplinePose;
use super::{Declined, Sweep, sweep};
use crate::pair_section::{ChartReach, Components, SectionRead, SsiLane, pair_section};
use crate::ssi::{ChartAxis, ChartEnd, ChartSide};

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the run's tolerance forms a band")
}

fn pt(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

/// The plane through `origin` square to `normal`.
fn plane(origin: Point3<f64>, normal: Vec3<f64>) -> Surface<f64> {
    let n = normal * (1.0 / normal.norm());
    let pick = if n.x.abs() < 0.9 {
        Vec3::unit_x()
    } else {
        Vec3::unit_y()
    };
    let u = pick - n * pick.dot(n);
    Surface::Plane {
        origin,
        normal: n,
        u_ref: u * (1.0 / u.norm()),
    }
}

fn level(h: f64) -> Surface<f64> {
    plane(pt(0.5, 0.5, h), Vec3::unit_z())
}

fn wall(n: NurbsSurface<f64>) -> Surface<f64> {
    Surface::Nurbs(std::sync::Arc::new(n))
}

/// A wall over `[0, 1]²`, of degree one less than its rows in `u` and
/// ruled in `v` from the `bottom` row (`v = 0`) to the `top`.
fn ruled(bottom: &[Point3<f64>], top: &[Point3<f64>]) -> Surface<f64> {
    let p = bottom.len() - 1;
    let mut ku = vec![0.0; p + 1];
    ku.extend(vec![1.0; p + 1]);
    let ku = KnotVector::clamped(ku, p).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control: Vec<_> = bottom.iter().zip(top).flat_map(|(b, t)| [*b, *t]).collect();
    let count = control.len();
    wall(NurbsSurface::new(ku, kv, control, vec![1.0; count]).unwrap())
}

/// The planar wall in `y = 0` whose bottom side is the quadratic with
/// heights `z` at its three controls, its top side at `z = 1`.
fn bowed(z: [f64; 3]) -> Surface<f64> {
    ruled(
        &[pt(0.0, 0.0, z[0]), pt(0.5, 0.0, z[1]), pt(1.0, 0.0, z[2])],
        &[pt(0.0, 0.0, 1.0), pt(0.5, 0.0, 1.0), pt(1.0, 0.0, 1.0)],
    )
}

/// The S-shaped wall of the probe, `s` metres across about `o`: a cubic
/// side inflected twice, ruled upward; `twisted` mirrors its top row's
/// S and tilts its rows.
fn inflected(o: Point3<f64>, s: f64, twisted: bool) -> Surface<f64> {
    let at = |x: f64, y: f64, z: f64| pt(o.x + s * x, o.y + s * y, o.z + s * z);
    let (lift, flip) = if twisted { (0.1, -1.0) } else { (0.0, 1.0) };
    ruled(
        &[
            at(0.0, 0.0, 0.0),
            at(0.33, 0.6, lift),
            at(0.66, -0.6, -lift),
            at(1.0, 0.0, 0.0),
        ],
        &[
            at(0.0, 0.0, 1.0),
            at(0.33, 0.6 * flip, 1.0 - lift),
            at(0.66, -0.6 * flip, 1.0 + lift),
            at(1.0, 0.0, 1.0),
        ],
    )
}

/// A biquadratic over `[0, 1]²` whose height is `z(i, j)` at control
/// point `(i, j)`.
fn net(z: &dyn Fn(usize, usize) -> f64) -> Surface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = (0..9)
        .map(|n| {
            let (i, j) = (n / 3, n % 3);
            #[allow(clippy::cast_precision_loss)]
            pt(0.5 * i as f64, 0.5 * j as f64, z(i, j))
        })
        .collect();
    wall(NurbsSurface::new(k.clone(), k, control, vec![1.0; 9]).unwrap())
}

/// The dome `z = −(x − ½)² − (y − ½)²`, its top `z = 0` at the centre.
fn dome() -> Surface<f64> {
    let g = [-0.25, 0.25, -0.25];
    net(&|i, j| g[i] + g[j])
}

/// A dome over `[0, 1]²` whose top stands off the rectangle's middle,
/// near `(0.3, 0.3)`: its height `z(x) + z(y)` with `z` the quadratic
/// through `(0, −0.2)`, `(½, 0.2)`, `(1, −0.6)`.
fn skewed_dome() -> Surface<f64> {
    let g = [-0.2, 0.2, -0.6];
    net(&|i, j| g[i] + g[j])
}

/// The sweep of the pair's wall against its plane.
fn swept(p: &Surface<f64>, w: &Surface<f64>) -> Sweep {
    let pose = PlaneSplinePose::of(p, w).expect("a plane and a spline");
    sweep(pose.net(), pose.pivot().heights, band())
}

/// The pair's classification through the march door.
fn classified(p: &Surface<f64>, w: &Surface<f64>) -> Components<f64> {
    match pair_section(p, w, None, Some(SsiLane::marched()), band()) {
        Ok(SectionRead::Built(s)) => s.components().clone(),
        other => panic!("the pair is classified: {other:?}"),
    }
}

fn side(fixed: ChartAxis, end: ChartEnd) -> ChartReach {
    ChartReach::side(ChartSide { fixed, end })
}

/// **The probe's pairs are proved loop-free, with no whole reading.**
/// Every pair the probe measured W5 to clear, through the pair's
/// classification: its answer is the sweep's (`Reaching`), every
/// component open. Among them are the poses the whole reading refuses
/// alone: the exact graze (its bottom side touches the plane at one
/// point), a near-graze 100ε out, the walls at 1 km, and a side lying in
/// the plane with the wall leaving it at 10⁻⁶ rad or less. The exact
/// graze reaches only the side it touches, every other side excluded.
///
/// Red against the sweep removed (each pose is read whole: a list, or
/// a refusal).
#[test]
fn the_probes_pairs_are_proved_loop_free() {
    let eps = band().zero();
    let mut pairs: Vec<(String, Surface<f64>, Surface<f64>)> = vec![
        ("exterior graze".into(), level(0.0), bowed([0.1, -0.1, 0.1])),
        (
            "interior graze".into(),
            level(0.0),
            bowed([-0.1, 0.1, -0.1]),
        ),
    ];
    for d in [100.0 * eps, -100.0 * eps, 1e-3] {
        for (what, z) in [
            ("exterior", [0.1, -0.1, 0.1]),
            ("interior", [-0.1, 0.1, -0.1]),
        ] {
            pairs.push((format!("{what} near-graze at {d:e}"), level(d), bowed(z)));
        }
    }
    for th in [std::f64::consts::FRAC_PI_2, 0.3, 1e-3, 1e-6, 1e-8] {
        let (s, c) = th.sin_cos();
        pairs.push((
            format!("a side in the plane, the wall at {th:e} rad"),
            level(0.0),
            ruled(
                &[pt(0.0, 0.0, 0.0), pt(1.0, 0.0, 0.0)],
                &[pt(0.0, c, s), pt(1.0, c, s)],
            ),
        ));
    }
    pairs.push((
        "a bowed side lying in the plane".into(),
        level(0.0),
        ruled(
            &[pt(0.0, 0.0, 0.0), pt(0.5, 0.3, 0.0), pt(1.0, 0.0, 0.0)],
            &[pt(0.0, 0.0, 1.0), pt(0.5, 0.3, 1.0), pt(1.0, 0.0, 1.0)],
        ),
    ));
    let flat = || {
        ruled(
            &[pt(0.0, 0.0, 0.0), pt(1.0, 0.0, 0.0)],
            &[pt(0.0, 0.0, 1.0), pt(1.0, 0.0, 1.0)],
        )
    };
    for (what, n) in [
        ("a plane touching one corner", Vec3::new(1.0, 0.0, 1.0)),
        ("the diagonal", Vec3::new(1.0, 0.0, -1.0)),
        ("a plane near a side", Vec3::new(1.0, 0.0, 1e-3)),
    ] {
        pairs.push((what.into(), plane(pt(0.0, 0.0, 0.0), n), flat()));
    }
    let far = pt(1000.0, 1000.0, 1000.0);
    let at = |x: f64, y: f64, z: f64| pt(far.x + 50.0 * x, far.y + 50.0 * y, far.z + 50.0 * z);
    for (what, w, n) in [
        (
            "the S wall at 1 km",
            inflected(far, 50.0, false),
            Vec3::unit_z(),
        ),
        (
            "the S wall at 1 km, the plane tilted",
            inflected(far, 50.0, false),
            Vec3::new(0.3, 0.0, 1.0),
        ),
        (
            "the twisted wall at 1 km",
            inflected(far, 50.0, true),
            Vec3::unit_z(),
        ),
        (
            "the S wall at the origin, the plane tilted",
            inflected(Point3::origin(), 1.0, false),
            Vec3::new(0.3, 0.0, 1.0),
        ),
    ] {
        let o = if what.contains("origin") {
            pt(0.5, 0.0, 0.5)
        } else {
            at(0.5, 0.0, 0.5)
        };
        pairs.push((what.into(), plane(o, n), w));
    }
    pairs.push((
        "the exterior graze at 1 km".into(),
        plane(at(0.5, 0.0, 0.0), Vec3::unit_z()),
        ruled(
            &[at(0.0, 0.0, 0.1), at(0.5, 0.0, -0.1), at(1.0, 0.0, 0.1)],
            &[at(0.0, 0.0, 1.0), at(0.5, 0.0, 1.0), at(1.0, 0.0, 1.0)],
        ),
    ));
    for (what, p, w) in &pairs {
        for (order, got) in [
            ("plane first", classified(p, w)),
            ("wall first", classified(w, p)),
        ] {
            assert!(
                matches!(got, Components::Reaching(_)),
                "{what}, {order}: the sweep's verdict: {got:?}"
            );
        }
    }
    assert_eq!(
        swept(&level(0.0), &bowed([0.1, -0.1, 0.1])),
        Sweep::LoopFree(side(ChartAxis::V, ChartEnd::Low)),
        "the exact graze reaches the side it touches alone"
    );
    assert!(
        matches!(
            classified(&level(0.0), &bowed([0.1, -0.1, 0.1])),
            Components::Reaching([plane_side, wall_side])
                if !plane_side.any() && wall_side == side(ChartAxis::V, ChartEnd::Low)
        ),
        "the reach is the wall's, in the order asked"
    );
}

/// **A loop inside the rectangle is never proved away.** The dome cut at
/// `z = −0.1` meets the plane in a circle about its top. The top's cells
/// are excluded above the plane, but no chain of cells above it joins
/// them to the rectangle's sides, so the sweep declines there, and the
/// pair is read whole: one loop.
///
/// Red against the join to the boundary dropped (a cell excluded is read
/// as harmless however it is enclosed: the sweep proves the loop away),
/// and against the sweep forced to clear.
#[test]
fn a_loop_about_an_excluded_top_declines() {
    assert!(
        matches!(
            swept(&level(-0.1), &dome()),
            Sweep::Declined(Declined::Enclosed(_))
        ),
        "{:?}",
        swept(&level(-0.1), &dome())
    );
    let got = classified(&level(-0.1), &dome());
    assert!(
        matches!(&got, Components::Counted(c) if c.len() == 1
            && c[0].shape == crate::pair_section::ComponentShape::Loop),
        "read whole, one loop: {got:?}"
    );
}

/// **The gradient is bounded over the whole cell, not read at a point.**
/// The skewed dome cut at `z = −0.2` meets the plane in a loop about its
/// top near `(0.3, 0.3)`; the gradient at the rectangle's middle is not
/// zero. Red against the gradient read at a cell's middle rather than
/// bounded over it (the one span cell reads regular and the loop is
/// proved away).
#[test]
fn the_gradient_is_bounded_over_the_cell() {
    assert!(
        matches!(swept(&level(-0.2), &skewed_dome()), Sweep::Declined(_)),
        "{:?}",
        swept(&level(-0.2), &skewed_dome())
    );
}

/// **A critical point within the band of the plane declines at the
/// floor.** The dome's top within the band of the plane: the cells about
/// the top are neither regular nor past the band, down to the floor,
/// where an isolated critical point is decided (a touch). The dome
/// scaled to `10⁻⁷` m deep has its top half an escalation below the
/// plane, its floor cells' boxes far narrower than that. Red against a
/// declined cell read as regular, and against the exclusion read past
/// zero rather than past the band (the shallow top's cells read below
/// the plane, joined to the sides, and the in-band touch is proved
/// away).
#[test]
fn a_critical_point_in_band_declines_at_the_floor() {
    let g = [-0.25e-7, 0.25e-7, -0.25e-7];
    let shallow = net(&|i, j| g[i] + g[j]);
    for (what, h, w) in [
        ("above", 0.5 * band().zero(), dome()),
        ("through", 0.0, dome()),
        ("below", -0.5 * band().zero(), dome()),
        ("above the shallow dome", 0.5 * band().escalate(), shallow),
    ] {
        let got = swept(&level(h), &w);
        assert!(
            matches!(got, Sweep::Declined(Declined::Floor(_))),
            "{what}, h {h:e}: {got:?}"
        );
    }
}

/// A pyramid of four bilinear patches over `[0, 1]²`, creased along
/// `x = ½` and `y = ½`, its apex `z = 1` at the centre and its rim at
/// `z = 0`.
fn pyramid() -> Surface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
    let control = (0..9)
        .map(|n| {
            let (i, j) = (n / 3, n % 3);
            #[allow(clippy::cast_precision_loss)]
            pt(
                0.5 * i as f64,
                0.5 * j as f64,
                if n == 4 { 1.0 } else { 0.0 },
            )
        })
        .collect();
    wall(NurbsSurface::new(k.clone(), k, control, vec![1.0; 9]).unwrap())
}

/// **A crease inside the rectangle declines.** The pyramid cut at
/// `z = ½` meets the plane in a loop about its apex, which lies on its
/// creases: each patch's gradient is off zero near the apex, from its
/// own side, and `φ` peaks where they meet. Red against the crease left
/// unread (every cell regular or joined below the plane: the loop is
/// proved away).
#[test]
fn a_crease_inside_the_rectangle_declines() {
    assert_eq!(
        swept(&level(0.5), &pyramid()),
        Sweep::Declined(Declined::Crease)
    );
}

/// **A side reached only where a cell along it is not excluded.** The
/// ramp `z = x` cut at `z = 0.4`: the line `x = 0.4` runs from the side
/// `v = 0` to `v = 1`; the sides `u = 0` and `u = 1` stand 0.4 and 0.6
/// off the plane. Red against every side read as reached (the guard
/// would ask a side the section never meets to bound).
#[test]
fn the_reach_is_the_sides_the_section_can_meet() {
    let ramp = net(&|i, _| 0.5 * f64::from(u8::try_from(i).unwrap()));
    assert_eq!(
        swept(&level(0.4), &ramp),
        Sweep::LoopFree(side(ChartAxis::V, ChartEnd::Low).or(side(ChartAxis::V, ChartEnd::High)))
    );
}
