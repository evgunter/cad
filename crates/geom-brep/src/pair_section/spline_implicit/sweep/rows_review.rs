//! Review 6150's counterexample rows against the sweep's join argument
//! and against C5's stages.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol, Vec3};

use super::Sweep;

fn band() -> Band {
    Band::linear(Tol::witness()).expect("band")
}

fn level(h: f64) -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.5, 0.5, h),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

/// A tensor wall over `[0,1]²`: `x` and `y` controls at the Greville
/// abscissae of `ku` and `kv`, height `z(i, j)`, weight `w(i, j)`.
fn tensor(
    ku: (Vec<f64>, usize),
    kv: (Vec<f64>, usize),
    z: &dyn Fn(usize, usize) -> f64,
    w: &dyn Fn(usize, usize) -> f64,
) -> Surface<f64> {
    let greville = |k: &[f64], p: usize| -> Vec<f64> {
        (0..k.len() - p - 1)
            .map(|i| k[i + 1..=i + p].iter().sum::<f64>() / p as f64)
            .collect()
    };
    let (gu, gv) = (greville(&ku.0, ku.1), greville(&kv.0, kv.1));
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for (i, &x) in gu.iter().enumerate() {
        for (j, &y) in gv.iter().enumerate() {
            let wt = w(i, j);
            control.push(Point3::new(x, y, z(i, j)));
            weights.push(wt);
        }
    }
    let ku = KnotVector::clamped(ku.0, ku.1).unwrap();
    let kv = KnotVector::clamped(kv.0, kv.1).unwrap();
    Surface::Nurbs(std::sync::Arc::new(
        NurbsSurface::new(ku, kv, control, weights).unwrap(),
    ))
}

fn q() -> (Vec<f64>, usize) {
    (vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2)
}

fn swept(p: &Surface<f64>, w: &Surface<f64>) -> Sweep {
    super::swept(p, w, band())
}

fn never_loop_free(what: &str, p: &Surface<f64>, w: &Surface<f64>) {
    let got = swept(p, w);
    assert!(
        !matches!(got, Sweep::LoopFree),
        "{what}: a loop of the section is proved away: {got:?}"
    );
}

/// The dome `−(x−½)² − (y−½)²` on two spans each way, its top on both
/// knot lines: a loop about the top straddles four span cells.
fn split_dome() -> Surface<f64> {
    let k = || (vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2);
    // Blossom of −(t−½)² at (0,0), (0,½), (½,1), (1,1).
    let g = [-0.25, 0.0, 0.0, -0.25];
    tensor(k(), k(), &|i, j| g[i] + g[j], &|_, _| 1.0)
}

/// A shallow dome whose top `≈ 0.00278` stands at `(0.05, ½)`, near the
/// side `u = 0`.
fn edge_dome() -> Surface<f64> {
    let gx = [0.0, 0.05 / 0.9, -1.0];
    let gy = [-0.25, 0.25, -0.25];
    tensor(q(), q(), &|i, j| gx[i] + gy[j], &|_, _| 1.0)
}

/// `g(x) + h(y)`, `g` the cubic `x³/3 − x²/2 + 0.21x` (a top at 0.3 of
/// 0.027, a dip at 0.7 of 0.01633), `h = −(y − ½)²`: a dome beside a
/// saddle.
fn dome_and_saddle() -> Surface<f64> {
    let gx = [
        0.0,
        0.07,
        -0.16 / 0.6 * 0.1 + 0.14 - 0.16667 + 0.16667 - 0.0,
        0.04333,
    ];
    // P2 = b/3 + 2c/3 = −1/6 + 0.14.
    let gx = [gx[0], gx[1], -1.0 / 6.0 + 0.14, gx[3]];
    let gy = [-0.25, 0.25, -0.25];
    tensor(
        (vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3),
        q(),
        &|i, j| gx[i] + gy[j],
        &|_, _| 1.0,
    )
}

#[test]
fn review_loops_are_never_proved_away() {
    let b = band();
    let dome = || {
        let g = [-0.25, 0.25, -0.25];
        tensor(q(), q(), &|i, j| g[i] + g[j], &|_, _| 1.0)
    };
    // The dome cut just past the escalation under its top: a tiny loop.
    for k in [1.01, 2.0, 10.0] {
        never_loop_free(
            &format!("dome cut {k} escalations under its top"),
            &level(-k * b.escalate()),
            &dome(),
        );
    }
    // Loops near the floor's scale: radius 2^-8 .. 2^-12 of the span.
    for r in [1.0 / 256.0, 1.0 / 1024.0, 1.0 / 4096.0] {
        let d = r * r;
        if d > b.escalate() {
            never_loop_free(&format!("dome loop of radius {r:e}"), &level(-d), &dome());
        }
    }
    // A loop hugging the knot lines through the top.
    for d in [1e-2, 1e-4, 1e-6] {
        if d > b.escalate() {
            never_loop_free(
                &format!("split dome cut {d:e} under"),
                &level(-d),
                &split_dome(),
            );
        }
    }
    // A loop beside the side u = 0.
    for d in [1e-3, 1e-4, 2e-3, 2.5e-3] {
        never_loop_free(
            &format!("edge dome cut {d:e} under"),
            &level(0.002_777 - d),
            &edge_dome(),
        );
    }
    // A dome beside a saddle cut above the saddle: a loop about the top.
    for h in [0.02, 0.0175] {
        never_loop_free(
            &format!("dome and saddle at {h}"),
            &level(h),
            &dome_and_saddle(),
        );
    }
    // A rational dome: the middle weight 4.
    let g = [-0.25, 0.25, -0.25];
    let rational = tensor(q(), q(), &|i, j| g[i] + g[j], &|i, j| {
        if (i, j) == (1, 1) { 4.0 } else { 1.0 }
    });
    for d in [0.05, 1e-4] {
        never_loop_free(
            &format!("rational dome cut {d:e} under"),
            &level(-d),
            &rational,
        );
    }
}

/// Below its saddle the dome-and-saddle's section is open (no loop):
/// shown for what the sweep says, not asserted. Run with `cargo nextest
/// run -p geom-brep --run-ignored only review_dome_and_saddle --no-capture`.
#[test]
#[ignore = "a print for review: asserts nothing"]
fn review_dome_and_saddle_below_the_saddle() {
    eprintln!(
        "below the saddle: {:?}",
        swept(&level(0.012), &dome_and_saddle())
    );
}

/// C5: "the hull of φ decides apart and coincident" and "anything in
/// band escalates". A planar wall tilted so that its every point lies
/// within the zero band of the plane is not a transverse, loop-free
/// pose: read through the classification (hull, door, pass, sweep), in
/// either operand order, it is coincident.
#[test]
fn review_a_wall_within_the_band_everywhere_is_coincident() {
    let t = 0.1 * band().zero();
    let tilted = tensor(
        (vec![0.0, 0.0, 1.0, 1.0], 1),
        (vec![0.0, 0.0, 1.0, 1.0], 1),
        &|i, _| if i == 0 { -t } else { t },
        &|_, _| 1.0,
    );
    let p = level(0.0);
    for (order, got) in [
        (
            "plane first",
            crate::pair_section::pair_section(
                &p,
                &tilted,
                None,
                Some(crate::pair_section::SsiLane::marched()),
                band(),
            ),
        ),
        (
            "wall first",
            crate::pair_section::pair_section(
                &tilted,
                &p,
                None,
                Some(crate::pair_section::SsiLane::marched()),
                band(),
            ),
        ),
    ] {
        assert!(
            matches!(got, Err(crate::intersect::SectionError::CoincidentSurfaces)),
            "a wall within {t:e} of the plane everywhere, {order}: {got:?}"
        );
    }
}

/// The edge and split domes' sweeps, printed. Run with `cargo nextest
/// run -p geom-brep --run-ignored only review_print_edge_dome
/// --no-capture`.
#[test]
#[ignore = "a print for review: asserts nothing"]
fn review_print_edge_dome() {
    for d in [1e-3, 1e-4, 2e-3, 2.5e-3] {
        eprintln!(
            "edge dome {d:e}: {:?}",
            swept(&level(0.002_777 - d), &edge_dome())
        );
    }
    for d in [1e-2, 1e-4] {
        eprintln!("split dome {d:e}: {:?}", swept(&level(-d), &split_dome()));
    }
}

/// Re-review 6150: a long, thin wall (1 m across `u`, 100 m along `v`)
/// tilted across its short side by `rise` in all, its middle row of
/// controls raised `lift` with weight `w` so that the net's hull reaches
/// past the escalation while the wall itself stays inside the zero band.
fn thin_wall(rise: f64, lift: f64, w: f64) -> Surface<f64> {
    let kv = vec![0.0, 0.0, 0.0, 0.2, 0.4, 0.6, 0.8, 1.0, 1.0, 1.0];
    let rows = kv.len() - 3;
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for i in 0..2 {
        for j in 0..rows {
            #[allow(clippy::cast_precision_loss)]
            let y = 100.0 * j as f64 / (rows - 1) as f64;
            #[allow(clippy::cast_precision_loss)]
            let x = i as f64;
            let mid = j == rows / 2;
            control.push(Point3::new(
                x,
                y,
                rise * (x - 0.5) + if mid { lift } else { 0.0 },
            ));
            weights.push(if mid { w } else { 1.0 });
        }
    }
    let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let kv = KnotVector::clamped(kv, 2).unwrap();
    Surface::Nurbs(std::sync::Arc::new(
        NurbsSurface::new(ku, kv, control, weights).unwrap(),
    ))
}

/// The largest height of `w` over `z = 0`, sampled.
fn sampled_reach(w: &Surface<f64>) -> f64 {
    let Surface::Nurbs(n) = w else { unreachable!() };
    let mut m: f64 = 0.0;
    for a in 0..=200 {
        for b in 0..=2000 {
            #[allow(clippy::cast_precision_loss)]
            let p = n.eval(f64::from(a) / 200.0, f64::from(b) / 2000.0);
            m = m.max(p.z.abs());
        }
    }
    m
}

/// Prints, for thin walls inside the zero band everywhere whose hull is
/// not: the wall's sampled reach, the sweep, the classification through
/// the march door, and the whole reading. Run with `cargo nextest run -p
/// geom-brep --run-ignored only rereview_thin --no-capture`.
#[test]
#[ignore = "a print for re-review"]
fn rereview_thin_wall_inside_the_band() {
    let b = band();
    for (rise, lift, w) in [
        (0.4, 0.0, 1.0),
        (0.4, 12.0, 1e-3),
        (0.4, 12.0, 1e-2),
        (0.6, 20.0, 1e-3),
        (1.2, 12.0, 1e-3),
    ] {
        let wall = thin_wall(rise * b.zero(), lift * b.zero(), w);
        let p = level(0.0);
        let pose = super::super::SplineImplicitPose::of(&p, &wall).unwrap();
        eprintln!(
            "rise {rise} lift {lift} w {w:e}: reach/zero {:.3}; sweep {:?}",
            sampled_reach(&wall) / b.zero(),
            swept(&p, &wall),
        );
        eprintln!(
            "   classified {:?}",
            crate::pair_section::pair_section(
                &p,
                &wall,
                None,
                Some(crate::pair_section::SsiLane::marched()),
                b
            )
            .map(|r| match r {
                crate::pair_section::SectionRead::Built(s) => format!("{:?}", s.components()),
                other => format!("{other:?}"),
            })
        );
        eprintln!(
            "   whole {:?}",
            super::super::whole(&pose, b).map(|s| format!("{:?}", s.components()))
        );
    }
}

/// **A thin wall within the zero band everywhere is never cleared**
/// (re-review 6150). The sweep alone reads each of these walls
/// loop-free: across their 1 m side the slope, levered by the 100 m
/// extent, is decided off zero, though no point of the wall stands a
/// zero band off the plane. So `review_a_wall_within_the_band_everywhere_is_not_loop_free`,
/// on a unit square where the lever is short, is green on the sweep's
/// lever and not on C5's order. The classification must refuse every
/// one: the hull (coincident) where the net lies in the band, the
/// boundary pass where a lifted, light row carries the net past it.
/// Red against the hull's coincidence dropped.
#[test]
fn rereview_a_thin_wall_within_the_band_is_never_cleared() {
    let b = band();
    for (rise, lift, w) in [(0.4, 0.0, 1.0), (0.4, 12.0, 1e-3), (1.2, 12.0, 1e-3)] {
        let wall = thin_wall(rise * b.zero(), lift * b.zero(), w);
        assert!(
            sampled_reach(&wall) <= b.zero(),
            "the wall stands within the band"
        );
        let p = level(0.0);
        for (order, got) in [
            (
                "plane first",
                crate::pair_section::pair_section(
                    &p,
                    &wall,
                    None,
                    Some(crate::pair_section::SsiLane::marched()),
                    b,
                ),
            ),
            (
                "wall first",
                crate::pair_section::pair_section(
                    &wall,
                    &p,
                    None,
                    Some(crate::pair_section::SsiLane::marched()),
                    b,
                ),
            ),
        ] {
            assert!(
                got.is_err(),
                "rise {rise} lift {lift} w {w:e}, {order}: a wall within the band is \
                 classified {:?}",
                got.map(|r| match r {
                    crate::pair_section::SectionRead::Built(s) => format!("{:?}", s.components()),
                    other => format!("{other:?}"),
                })
            );
        }
    }
}
