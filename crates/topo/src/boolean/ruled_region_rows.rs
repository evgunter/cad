//! Rows for [`super::RuledFaceRegion`]: cylinder and cone faces built
//! edge by edge, read at lattices of points whose side is known in closed
//! form.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::*;
use crate::entity::EdgeKey;
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

fn edge(k: u64, curve: geom::Curve3<f64>, (t0, t1): (f64, f64), forward: bool) -> Edge<f64> {
    RegionEdge {
        edge: key(k),
        r#loop: LoopKey::default(),
        cycle: 0,
        carrier: curve,
        t0,
        t1,
        turn: crate::boolean::region::fixture_turn(t0, t1),
        forward,
    }
}

/// The parallel of radius `r` at height `z`, over `span`.
fn rim(k: u64, z: f64, r: f64, span: (f64, f64), forward: bool) -> Edge<f64> {
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
fn segment(k: u64, a: Point3<f64>, b: Point3<f64>) -> Edge<f64> {
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

fn cylinder(edges: Vec<Edge<f64>>) -> RuledFaceRegion<f64> {
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
fn window(k: u64, (u0, u1): (f64, f64), (z0, z1): (f64, f64), hole: bool) -> Vec<Edge<f64>> {
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

fn cone(edges: Vec<Edge<f64>>, scale: f64) -> RuledFaceRegion<f64> {
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
fn cone_rim(k: u64, v: f64, span: (f64, f64), forward: bool) -> Edge<f64> {
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

/// **A wall built edge by edge reads an edge inside it as no boundary,
/// and its null scaffolding as nothing.** The wall over `[0.2, 1.4] ×
/// [0, 1]` carries a strut up the ruling `u = 0.8` to `z = 0.5`, which
/// its loop runs both ways, and a null edge at the strut's tip
/// ([`crate::test_support_fixtures::kill_under_a_null_strut`]). The
/// strut has the face on both sides: a point on it is inside, and so is
/// one beside its tip.
#[test]
fn a_built_wall_reads_its_strut_as_inside_and_its_null_edge_as_nothing() {
    let (mut body, face, _, _, _) =
        crate::test_support_fixtures::kill_under_a_null_strut(geom_core::Tol::witness());
    // The fixture's loop bounds its window on the sense-false side.
    body.set_face_sense(face, false).unwrap();
    let region = ruled_face_region(&body, face).unwrap().expect("a region");
    for z in [0.1, 0.25, 0.4] {
        assert_eq!(
            region.contains(wall(0.8, z), b()),
            Ok(Some(true)),
            "on the strut at z {z}"
        );
    }
    reads(
        &region,
        lattice().map(|(u, z)| {
            let truth = both(
                within(u.rem_euclid(TAU), (0.2, 1.4), 0.01),
                within(z, (0.0, 1.0), 0.01),
            );
            (wall(u, z), truth)
        }),
        "the built wall",
    );
}

/// **The aim is a point of the surface, wherever the boundary's middles
/// lie.** A band whose two rims start at opposite azimuths has its
/// middles on opposite sides of the axis, whose mean is on the axis, and
/// is aimed on the reference azimuth; a cone face reaching both nappes
/// has its middles' mean height at the apex; a face with no edge offers
/// none.
#[test]
fn the_aim_is_a_point_of_the_surface() {
    let opposite = edge(
        2,
        geom::Curve3::Circle {
            center: Point3::new(0.0, 0.0, 1.0),
            axis: z_axis(),
            radius: 1.0,
            u_ref: -x_axis(),
        },
        (0.0, TAU),
        false,
    );
    let band = cylinder(vec![rim(1, 0.0, 1.0, (0.0, TAU), true), opposite]);
    let aim = band.aim().expect("an aim");
    assert!(
        (aim - Point3::new(1.0, 0.0, 0.5)).norm() < 1e-12,
        "on the reference azimuth: {aim:?}"
    );

    let hourglass = cone(
        vec![
            cone_rim(1, 1.0, (0.0, TAU), true),
            rim(2, -ALPHA.cos(), ALPHA.sin(), (0.0, TAU), true),
        ],
        1.0,
    );
    let aim = hourglass.aim().expect("an aim");
    assert!(
        aim.x.is_finite() && aim.y.is_finite() && aim.z.is_finite(),
        "{aim:?}"
    );

    assert!(cylinder(Vec::new()).aim().is_none());
}

/// **A cone face's scale is never under its boundary's reach.** An
/// ellipse arc whose farthest point from the apex lies between its ends
/// and its middle is reached past all three; the scale is bounded by the
/// arc's carrier, its centre's distance plus its larger semi-axis.
#[test]
fn a_cone_faces_scale_bounds_its_reach_from_the_apex() {
    let arc = edge(
        1,
        geom::Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 2.0),
            axis: Vec3::new(-0.3, 0.0, 1.0) / 1.09_f64.sqrt(),
            semi_u: 1.5,
            semi_v: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.3) / 1.09_f64.sqrt(),
        },
        (-0.6, 1.8),
        true,
    );
    let reach = |t: f64| (arc.at(t) - Point3::origin()).norm();
    let sampled = (0..=2400)
        .map(|i| reach(-0.6 + 2.4 * f64::from(i) / 2400.0))
        .fold(0.0, f64::max);
    let three = [-0.6, 0.6, 1.8].map(reach).into_iter().fold(0.0, f64::max);
    assert!(
        sampled > three + 1e-3,
        "the far point is between the three: {sampled} {three}"
    );
    let scale = reach_bound(&cone_surface(), &[arc]);
    assert!(scale >= sampled, "scale {scale} under the reach {sampled}");
}

/// **A generator edge through the apex reaches both nappes**, read at
/// its ends: its middle on the opening nappe does not keep it off the
/// mirror one.
#[test]
fn a_generator_through_the_apex_reaches_both_nappes() {
    let region = cone(vec![segment(1, on_cone(0.0, -1.0), on_cone(0.0, 2.0))], 2.0);
    assert_eq!(region.reaches_nappe(on_cone(1.0, -0.5), b()), Ok(true));
    assert_eq!(region.reaches_nappe(on_cone(1.0, 0.5), b()), Ok(true));
    let opening = cone(vec![segment(1, on_cone(0.0, 0.5), on_cone(0.0, 2.0))], 2.0);
    assert_eq!(opening.reaches_nappe(on_cone(1.0, -0.5), b()), Ok(false));
}

/// **A generator meets a conic only on its own nappe.** The meridian
/// plane of the generator at azimuth 0 meets the mirror nappe's rim at
/// two points, one of them on the leg's radial side — on the opposite
/// generator, which the leg never runs on.
#[test]
fn a_generator_meets_no_conic_on_the_mirror_nappe() {
    let mirror = rim(1, -ALPHA.cos(), ALPHA.sin(), (0.0, TAU), true);
    let own = cone_rim(2, 2.0, (0.0, TAU), true);
    let region = cone(vec![mirror.clone(), own.clone()], 2.0);
    let leg = region.ruling(on_cone(0.0, 1.0), true, true);
    assert!(
        region
            .landings(&mirror, &leg, b())
            .ok()
            .is_some_and(|l| l.is_empty())
    );
    assert_eq!(
        region.landings(&own, &leg, b()).ok().map(|l| l.len()),
        Some(1)
    );
}

/// **A leg running along an edge grazes**: a ruling along a line edge
/// it reaches, and a parallel along a rim it reaches, abandon the path
/// rather than read past the edge.
#[test]
fn a_leg_along_an_edge_grazes() {
    let line = cylinder(vec![segment(1, wall(0.0, 0.0), wall(0.0, 2.0))]);
    assert!(matches!(
        region::read_path(
            &line,
            wall(0.0, -0.5),
            region::Path::Single {
                parallel: false,
                forward: true
            },
            b()
        ),
        Err(RayFault::Graze)
    ));
    let arc = cylinder(vec![rim(1, 0.0, 1.0, (0.0, 1.0), true)]);
    assert!(matches!(
        region::read_path(
            &arc,
            wall(2.0, 0.0),
            region::Path::Single {
                parallel: true,
                forward: true
            },
            b()
        ),
        Err(RayFault::Graze)
    ));
}

/// **An aimed path's first leg reads nothing past its corner.** From
/// `(0.5, 0)` up the ruling to the corner at `z = 1`, then round the
/// parallel: the rim at `z = 1.5` lies on the first leg's line past the
/// corner, and the path's only crossing is the segment at `u = 1.2`,
/// which it leaves the face through. Read past the corner, the rim would
/// be the closest crossing and say the opposite.
#[test]
fn an_aimed_paths_first_leg_ends_at_its_corner() {
    let region = cylinder(vec![
        rim(1, 1.5, 1.0, (0.0, 1.0), true),
        segment(2, wall(1.2, 0.5), wall(1.2, 1.2)),
    ]);
    let spec = region::Path::Aimed {
        target: wall(1.5, 1.0),
        parallel_first: false,
        first_forward: true,
        then_forward: true,
    };
    assert!(matches!(
        region::read_path(&region, wall(0.5, 0.0), spec, b()),
        Ok(true)
    ));
}

fn cone_surface() -> Ruled<f64> {
    Ruled::Cone {
        apex: Point3::origin(),
        axis: z_axis(),
        half_angle: ALPHA,
        u_ref: x_axis(),
    }
}

/// **A generator ending at the apex stays on its nappe**: the apex is
/// on neither, so a quarter cone's two generators from the apex keep
/// the face off the mirror nappe, and a point there is outside.
#[test]
fn a_generator_from_the_apex_keeps_its_nappe() {
    let quarter = cone(
        vec![
            segment(1, Point3::origin(), on_cone(PI / 2.0, 2.0)),
            cone_rim(2, 2.0, (0.0, PI / 2.0), false),
            segment(3, on_cone(0.0, 2.0), Point3::origin()),
        ],
        2.0,
    );
    assert_eq!(quarter.reaches_nappe(on_cone(0.7, -1.0), b()), Ok(false));
    assert_eq!(quarter.contains(on_cone(0.7, -1.0), b()), Ok(Some(false)));
    assert_eq!(quarter.contains(on_cone(0.7, 1.0), b()), Ok(Some(true)));
}
