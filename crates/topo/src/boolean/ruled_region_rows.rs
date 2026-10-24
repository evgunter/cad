//! Rows for [`super::RuledFaceRegion`]: cylinder and cone faces built
//! edge by edge, read at lattices of points whose side is known in closed
//! form.

#![allow(clippy::unwrap_used, clippy::panic)]

use super::*;
use core::f64::consts::{PI, TAU};
use slotmap::KeyData;

fn b() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

fn z_axis() -> Vec3<f64> {
    Vec3::new(0.0, 0.0, 1.0)
}

fn x_axis() -> Vec3<f64> {
    Vec3::new(1.0, 0.0, 0.0)
}

fn key(k: u64) -> EdgeKey {
    EdgeKey::from(KeyData::from_ffi(k))
}

fn edge(k: u64, curve: geom::Curve3<f64>, (t0, t1): (f64, f64), forward: bool) -> RegionEdge<f64> {
    RegionEdge {
        edge: key(k),
        curve,
        t0,
        t1,
        forward,
    }
}

/// The parallel of radius `r` at height `z`, over `span`.
fn rim(k: u64, z: f64, r: f64, span: (f64, f64), forward: bool) -> RegionEdge<f64> {
    edge(
        k,
        geom::Curve3::Circle {
            center: Point3::new(0.0, 0.0, z),
            axis: z_axis(),
            radius: r,
            u_ref: x_axis(),
        },
        span,
        forward,
    )
}

/// The segment from `a` to `b`, run from `a`.
fn segment(k: u64, a: Point3<f64>, b: Point3<f64>) -> RegionEdge<f64> {
    let d = b - a;
    let len = d.norm();
    edge(
        k,
        geom::Curve3::Line {
            origin: a,
            dir: d / len,
        },
        (0.0, len),
        true,
    )
}

fn cylinder(edges: Vec<RegionEdge<f64>>) -> RuledFaceRegion<f64> {
    RuledFaceRegion {
        surface: Ruled::Cylinder {
            origin: Point3::origin(),
            axis: z_axis(),
            radius: 1.0,
            u_ref: x_axis(),
        },
        sense: true,
        outer: LoopKey::default(),
        scale: 1.0,
        edges,
    }
}

/// The unit cylinder's point at azimuth `u`, height `z`.
fn wall(u: f64, z: f64) -> Point3<f64> {
    Point3::new(u.cos(), u.sin(), z)
}

/// `region` read at every point of `points` whose side `truth` gives,
/// skipping those it calls too near the boundary (`None`).
fn reads(
    region: &RuledFaceRegion<f64>,
    points: impl IntoIterator<Item = (Point3<f64>, Option<bool>)>,
    what: &str,
) {
    let (mut right, mut refused, mut wrong) = (0, Vec::new(), Vec::new());
    for (p, truth) in points {
        let Some(truth) = truth else { continue };
        match region.contains(p, b()) {
            Ok(Some(got)) if got == truth => right += 1,
            Ok(got) => wrong.push((p, got, truth)),
            Err(e) => refused.push((p, e)),
        }
    }
    assert!(
        wrong.is_empty() && refused.is_empty() && right > 0,
        "{what}: {right} right, wrong at {wrong:?}, refused at {refused:?}"
    );
}

/// Whether `x` lies in `(lo, hi)` clear of its ends by `clear`, `None`
/// within `clear` of either.
fn within(x: f64, (lo, hi): (f64, f64), clear: f64) -> Option<bool> {
    if (x - lo).abs() < clear || (x - hi).abs() < clear {
        None
    } else {
        Some(x > lo && x < hi)
    }
}

fn both(a: Option<bool>, c: Option<bool>) -> Option<bool> {
    match (a, c) {
        (Some(false), _) | (_, Some(false)) => Some(false),
        (Some(true), Some(true)) => Some(true),
        _ => None,
    }
}

fn lattice() -> impl Iterator<Item = (f64, f64)> {
    (0..24).flat_map(|i| {
        let u = f64::from(i) * TAU / 24.0 - PI + 0.07;
        (0..13).map(move |j| (u, f64::from(j) * 0.25 - 0.9))
    })
}

/// **A full-turn band reads its height window at every azimuth.** Two
/// whole rims bound the wall `0 < z < 1`; its rims are the boundary.
#[test]
fn a_band_reads_its_height_window() {
    let band = cylinder(vec![
        rim(1, 0.0, 1.0, (0.0, TAU), true),
        rim(2, 1.0, 1.0, (0.0, TAU), false),
    ]);
    reads(
        &band,
        lattice().map(|(u, z)| (wall(u, z), within(z, (0.0, 1.0), 0.01))),
        "band",
    );
    for u in [0.3, 2.0, 5.0] {
        assert_eq!(band.contains(wall(u, 1.0), b()), Ok(None), "on the rim");
        assert_eq!(band.contains(wall(u, 1.0 - 1e-6), b()), Ok(Some(true)));
        assert_eq!(band.contains(wall(u, 1.0 + 1e-6), b()), Ok(Some(false)));
    }
}

/// The loop of the window `us × zs` on the unit cylinder, the face to
/// its left (`hole: false`) or the window a hole in it.
fn window(k: u64, (u0, u1): (f64, f64), (z0, z1): (f64, f64), hole: bool) -> Vec<RegionEdge<f64>> {
    if hole {
        vec![
            rim(k, z0, 1.0, (u0, u1), false),
            segment(k + 1, wall(u0, z0), wall(u0, z1)),
            rim(k + 2, z1, 1.0, (u0, u1), true),
            segment(k + 3, wall(u1, z1), wall(u1, z0)),
        ]
    } else {
        vec![
            rim(k, z0, 1.0, (u0, u1), true),
            segment(k + 1, wall(u1, z0), wall(u1, z1)),
            rim(k + 2, z1, 1.0, (u0, u1), false),
            segment(k + 3, wall(u0, z1), wall(u0, z0)),
        ]
    }
}

/// **A windowed wall reads its rectangle**, and a hole in it reads as
/// outside: the ring is read in the same stride as the outer loop.
#[test]
fn a_windowed_wall_with_a_ring_reads_its_region() {
    let rect = cylinder(window(1, (0.0, 3.0), (0.0, 2.0), false));
    let az = |u: f64| u.rem_euclid(TAU);
    reads(
        &rect,
        lattice().map(|(u, z)| {
            let truth = both(within(az(u), (0.0, 3.0), 0.01), within(z, (0.0, 2.0), 0.01));
            (wall(u, z), truth)
        }),
        "rectangle",
    );
    let mut edges = window(1, (0.0, 3.0), (0.0, 2.0), false);
    edges.extend(window(11, (1.0, 2.0), (0.5, 1.5), true));
    let ringed = cylinder(edges);
    reads(
        &ringed,
        lattice().map(|(u, z)| {
            let outer = both(within(az(u), (0.0, 3.0), 0.01), within(z, (0.0, 2.0), 0.01));
            let hole = both(within(az(u), (1.0, 2.0), 0.01), within(z, (0.5, 1.5), 0.01));
            let truth = match (outer, hole) {
                (Some(true), Some(false)) => Some(true),
                (Some(false), _) => Some(false),
                (_, Some(true)) => Some(false),
                _ => None,
            };
            (wall(u, z), truth)
        }),
        "ringed",
    );
}

/// **A wall closed by a tilted section reads its height extreme inside
/// the edge.** The section `z = 1 + 0.4x` is an ellipse whose major
/// semi-axis is `√1.16`; the wall below it and above the rim `z = 0` is
/// the face.
#[test]
fn a_wall_under_a_tilted_section_reads_its_region() {
    let s = 1.16_f64.sqrt();
    let section = edge(
        2,
        geom::Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 1.0),
            axis: Vec3::new(-0.4, 0.0, 1.0) / s,
            semi_u: s,
            semi_v: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.4) / s,
        },
        (0.0, TAU),
        false,
    );
    let region = cylinder(vec![rim(1, 0.0, 1.0, (0.0, TAU), true), section]);
    reads(
        &region,
        lattice().map(|(u, z)| {
            let top = 1.0 + 0.4 * u.cos();
            (wall(u, z), within(z, (0.0, top), 0.01))
        }),
        "tilted",
    );
}

fn cone(edges: Vec<RegionEdge<f64>>, scale: f64) -> RuledFaceRegion<f64> {
    RuledFaceRegion {
        surface: Ruled::Cone {
            apex: Point3::origin(),
            axis: z_axis(),
            half_angle: ALPHA,
            u_ref: x_axis(),
        },
        sense: true,
        outer: LoopKey::default(),
        scale,
        edges,
    }
}

const ALPHA: f64 = 0.5;

/// The cone's point at azimuth `u`, signed slant `v`.
fn on_cone(u: f64, v: f64) -> Point3<f64> {
    let (s, c) = ALPHA.sin_cos();
    Point3::new(v * s * u.cos(), v * s * u.sin(), v * c)
}

/// The cone's parallel at slant `v > 0`, over `span`.
fn cone_rim(k: u64, v: f64, span: (f64, f64), forward: bool) -> RegionEdge<f64> {
    rim(k, v * ALPHA.cos(), v * ALPHA.sin(), span, forward)
}

fn cone_lattice() -> impl Iterator<Item = (f64, f64)> {
    (0..24).flat_map(|i| {
        let u = f64::from(i) * TAU / 24.0 - PI + 0.07;
        (0..17).map(move |j| (u, f64::from(j) * 0.25 - 1.9))
    })
}

/// **A frustum, an apex-closed cone and a window read their regions, and
/// the apex is a point like any other.** The frustum `1 < v < 2` has
/// its two rims; the apex-closed face is the disc `0 ≤ v < 2` under its
/// one rim, which holds the apex and nothing of the mirror nappe; the
/// window `0 < u < 2, 1 < v < 2` adds two generators.
#[test]
fn cone_faces_read_their_regions_through_the_apex() {
    let frustum = cone(
        vec![
            cone_rim(1, 1.0, (0.0, TAU), true),
            cone_rim(2, 2.0, (0.0, TAU), false),
        ],
        2.0,
    );
    reads(
        &frustum,
        cone_lattice()
            .filter(|&(_, v)| v.abs() > 0.01)
            .map(|(u, v)| (on_cone(u, v), within(v, (1.0, 2.0), 0.01))),
        "frustum",
    );
    assert_eq!(
        frustum.contains(Point3::origin(), b()),
        Ok(Some(false)),
        "the frustum does not hold the apex"
    );

    let closed = cone(vec![cone_rim(1, 2.0, (0.0, TAU), false)], 2.0);
    reads(
        &closed,
        cone_lattice()
            .filter(|&(_, v)| v.abs() > 0.01)
            .map(|(u, v)| (on_cone(u, v), within(v, (0.0, 2.0), 0.01))),
        "apex-closed",
    );
    assert_eq!(
        closed.contains(Point3::origin(), b()),
        Ok(Some(true)),
        "the apex-closed face holds its apex"
    );
    assert_eq!(
        closed.contains(on_cone(0.4, 1e-6), b()),
        Ok(Some(true)),
        "beside the apex"
    );

    let window = cone(
        vec![
            cone_rim(1, 1.0, (0.0, 2.0), true),
            segment(2, on_cone(2.0, 1.0), on_cone(2.0, 2.0)),
            cone_rim(3, 2.0, (0.0, 2.0), false),
            segment(4, on_cone(0.0, 2.0), on_cone(0.0, 1.0)),
        ],
        2.0,
    );
    let az = |u: f64| u.rem_euclid(TAU);
    reads(
        &window,
        cone_lattice()
            .filter(|&(_, v)| v.abs() > 0.01)
            .map(|(u, v)| {
                (
                    on_cone(u, v),
                    both(within(az(u), (0.0, 2.0), 0.01), within(v, (1.0, 2.0), 0.01)),
                )
            }),
        "window",
    );
}

/// **A face whose outward normal points at the axis reads the same
/// region from the same boundary run the other way.** The band's loops
/// reversed and its sense cleared describe the one point set.
#[test]
fn a_reversed_face_reads_the_same_region() {
    let band = RuledFaceRegion {
        sense: false,
        ..cylinder(vec![
            rim(1, 0.0, 1.0, (0.0, TAU), false),
            rim(2, 1.0, 1.0, (0.0, TAU), true),
        ])
    };
    reads(
        &band,
        lattice().map(|(u, z)| (wall(u, z), within(z, (0.0, 1.0), 0.01))),
        "reversed band",
    );
}

/// **A point near an edge is read by its distance from it, before any
/// path**: within the band's zero of a meridian it is on the boundary,
/// in the band it refuses.
#[test]
fn a_point_near_an_edge_is_read_before_any_path() {
    let rect = cylinder(window(1, (0.0, 3.0), (0.0, 2.0), false));
    for z in [0.3, 1.0, 1.7] {
        let on = wall(3.0, z);
        let out = Vec3::new(-(3.0_f64.sin()), 3.0_f64.cos(), 0.0);
        assert_eq!(rect.contains(on + out * 5e-10, b()), Ok(None), "z {z}");
        assert!(
            matches!(
                rect.contains(on + out * 5e-9, b()),
                Err(RegionRefusal::Escalated(_))
            ),
            "z {z}"
        );
    }
}
