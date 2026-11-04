//! **ψ reads a chart point through the one reading** (D3, "One reading
//! per question"; C4): the projected image's channels
//! ([`ProjectedImage::raw`]) are [`geom::ChartInverse::point`]'s, named
//! on the piece's branch. The oracle reads the point world-side through
//! `ChartInverse::point` at a band, then moves each periodic channel by
//! whole turns onto the branch nearest the piece's centre (the test's
//! own `periodic_branch`, which no shipped code reads a turn off), and
//! compares with ψ evaluated on the frame-local point. Both are run at
//! `f64` and at `Interval`, whose enclosures must meet.
//!
//! Every chart the lane admits: the sphere, the cylinder, a narrow and
//! a wide cone on both nappes, and a ring and a near-horn torus. The
//! points are the surface's own at `(u, v)` sweeping more than a turn in
//! `u` (so across the angle window's cut at `−1` and `atan2`'s at `π`),
//! each moved along the normal by `0`, `±1e-6` and `1e-3` (a channel
//! re-spelled to agree on the surface alone parts from the reading off
//! it), read on the three branch centres within three eighths of a turn
//! of the point's azimuth (and of its tube angle on the torus).

use core::f64::consts::{FRAC_PI_2, TAU};

use geom::{ChartInverse, Surface};
use geom_core::{Band, Bounds, Decide, Interval, Point2, Point3, Real, Vec3};

/// The bracket a scalar carries, as `(lo, hi)`.
type Bracket<T> = fn(T) -> (f64, f64);

use super::{
    FramedCarrier, ProjectedChart, ProjectedImage, breaks_vector, chart_at, chart_frame, to_frame,
};

fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap_or_else(|e| panic!("a valid band: {e}"))
}

/// The charts the lane admits, with the frame tilted off the world's.
fn charts() -> Vec<(&'static str, Surface<f64>)> {
    let axis = Vec3::new(0.0, 0.6, 0.8);
    let u_ref = Vec3::new(1.0, 0.0, 0.0);
    let o = Point3::new(0.2, -0.1, 0.3);
    let cone = |half_angle| Surface::Cone {
        apex: o,
        axis,
        half_angle,
        u_ref,
    };
    let torus = |major_radius, minor_radius| Surface::Torus {
        center: o,
        axis,
        major_radius,
        minor_radius,
        u_ref,
    };
    vec![
        (
            "sphere",
            Surface::Sphere {
                center: o,
                radius: 0.7,
                axis,
                u_ref,
            },
        ),
        (
            "cylinder",
            Surface::Cylinder {
                origin: o,
                axis,
                radius: 0.7,
                u_ref,
            },
        ),
        ("narrow cone", cone(0.4)),
        ("wide cone", cone(1.3)),
        ("ring torus", torus(2.0, 0.5)),
        ("near-horn torus", torus(1.0, 0.999)),
    ]
}

/// The surface parameters swept: `u` past a turn, `v` per chart
/// (both nappes on a cone, clear of the poles on the sphere, all round
/// the tube on a torus).
fn samples(name: &str) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for i in 0..40 {
        let u = -3.3 + 0.17 * f64::from(i);
        for j in 0..12 {
            let s = f64::from(j) / 11.0;
            let v = match name {
                "sphere" => -1.4 + 2.8 * s,
                "cylinder" => -2.0 + 4.0 * s,
                // Both nappes, clear of the apex.
                _ if name.ends_with("cone") => {
                    if j < 6 {
                        -2.0 + 1.9 * s
                    } else {
                        0.1 + 1.9 * s
                    }
                }
                // A turn and a bit of the tube.
                _ => -3.3 + 6.8 * s,
            };
            out.push((u, v));
        }
    }
    out
}

/// The quarter turn nearest `angle`.
fn nearest_quarter(angle: f64) -> i32 {
    #[allow(clippy::cast_possible_truncation)]
    let m = (angle / FRAC_PI_2).round() as i32;
    m
}

/// `value` moved by whole turns onto the branch nearest `centre`: the
/// oracle's own branch step, in the scalar.
fn onto<T: Real>(value: T, centre: f64, bracket: Bracket<T>) -> T {
    let (lo, hi) = bracket(value);
    let mid = 0.5 * (lo + hi);
    let k = <f64 as Real>::periodic_branch(centre - mid, TAU);
    value + T::pi() * T::from_f64(2.0 * k)
}

/// Whether two enclosures meet within `tol`, each narrower than `tol`.
fn meet((a_lo, a_hi): (f64, f64), (b_lo, b_hi): (f64, f64), tol: f64) -> bool {
    a_hi - a_lo <= tol && b_hi - b_lo <= tol && a_lo <= b_hi + tol && b_lo <= a_hi + tol
}

/// The rows' one body at the scalar `T`, answering how many points it
/// compared.
fn compare<T: Decide>(lift: impl Fn(f64) -> T, bracket: Bracket<T>) -> usize {
    let mut compared = 0;
    for (name, surface) in charts() {
        let surface_t = surface.map_scalar(&lift);
        let inverse = ChartInverse::of(&surface_t)
            .unwrap_or_else(|e| panic!("{name}: an analytic chart: {e}"));
        let (origin, e) = chart_frame(&surface_t).unwrap_or_else(|| panic!("{name}: a frame"));
        for (u, v) in samples(name) {
            // The image's nappe is its carrier's: the sample's own.
            let chart = chart_at(&surface_t, || lift(v), band())
                .unwrap_or_else(|e| panic!("{name}: a chart at v = {v}: {e}"));
            let torus = matches!(chart, ProjectedChart::Torus { .. });
            let on = surface.eval(u, v);
            let normal = surface.normal(u, v);
            for d in [0.0, 1e-6, -1e-6, 1e-3] {
                let world = (on + normal * d).map(&lift);
                let framed = to_frame(world - origin, &e);
                let read = inverse.point(world, band());
                assert!(
                    !read.singular,
                    "{name} at ({u}, {v}) moved {d}: the oracle point is no singularity"
                );
                for dm in -1..=1 {
                    let m = nearest_quarter(u) + dm;
                    for dn in if torus { -1..=1 } else { 0..=0 } {
                        let n = nearest_quarter(v) + dn;
                        let image = ProjectedImage {
                            chart,
                            carrier: FramedCarrier::Line {
                                point: Vec3::zero(),
                                dir: Vec3::zero(),
                                origin: T::zero(),
                                span: T::one(),
                            },
                            breaks: breaks_vector(&[0.0, 1.0]),
                            azimuth: vec![m],
                            tube: if torus { vec![n] } else { Vec::new() },
                            u_off: T::zero(),
                            v_off: T::zero(),
                            v_sign: T::one(),
                        };
                        let psi = image.raw(framed, 0);
                        let theta = f64::from(m) * FRAC_PI_2;
                        let want = Point2::new(
                            onto(read.uv.x, theta, bracket),
                            if torus {
                                onto(read.uv.y, f64::from(n) * FRAC_PI_2, bracket)
                            } else {
                                read.uv.y
                            },
                        );
                        let tol = 1e-12;
                        assert!(
                            meet(bracket(psi.x), bracket(want.x), tol)
                                && meet(bracket(psi.y), bracket(want.y), tol),
                            "{name} at ({u}, {v}) moved {d}, centres ({m}, {n}), at {}: ψ reads \
                             ({}, {})..({}, {}), the reading on that branch ({}, {})..({}, {})",
                            T::NAME,
                            bracket(psi.x).0,
                            bracket(psi.y).0,
                            bracket(psi.x).1,
                            bracket(psi.y).1,
                            bracket(want.x).0,
                            bracket(want.y).0,
                            bracket(want.x).1,
                            bracket(want.y).1,
                        );
                        compared += 1;
                    }
                }
            }
        }
    }
    compared
}

/// **ψ's channels are the point reading's on the piece's branch**, at
/// `f64` (module docs).
#[test]
fn psi_reads_the_chart_point_on_its_branch_at_f64() {
    let compared = compare(|x| x, |x| (x, x));
    assert!(compared > 20_000, "the sweep compared {compared} reads");
}

/// The same at `Interval`: ψ's enclosure meets the reading's, both
/// within `1e-12` wide.
#[test]
fn psi_reads_the_chart_point_on_its_branch_at_interval() {
    let compared = compare(Interval::from_f64, |x| (x.lo(), x.hi()));
    assert!(compared > 20_000, "the sweep compared {compared} reads");
}
