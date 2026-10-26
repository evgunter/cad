//! **A split through a vertex hangs one null edge per corner of the
//! section there, as the tree the corners make.**
//!
//! A solid with a vertex at `MEET` is split by the plane through it with
//! the top's normal, either way up, at every pose. Round the vertex the
//! plane meets the solid in corners, the material between two germs,
//! and each side keeps one vertex there per piece of the solid it holds
//! round it. Where the runs above the plane lie apart, each corner pairs
//! one run's germs; where they nest (`meeting::comb`,
//! `meeting::arch_cone`, `meeting::branching_cone`), a corner pairs
//! germs of two runs, and a null edge per run would close each section
//! loop across the corners of others. Where an edge at the vertex lies
//! in the plane, exactly or within the tolerance, the germs either side
//! of it are ordered by the topology alone.
//!
//! Each sound row asserts both sides' closed-form volumes, tier 3, that
//! every face's corners at one point are angularly disjoint, and how
//! many vertices each side keeps at the vertex.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{
    MEET, Pose, apex_pyramid, arch_cone, at, branching_cone, comb, corners_disjoint, crown,
    posed_crown, poses, pyramid_volumes, single_arch, zigzag,
};
use common::split_plane;
use geom_core::{Tol, Vec3};
use topo::{AtRestBody, SplitPart, SplitReduceError, split, validate_geometric};

fn t() -> Tol {
    Tol::witness()
}

/// The plane through `MEET` with the top's normal at `pose`, turned
/// over where `up` is false.
fn plane_at(pose: &Pose, up: bool) -> topo::SplitPlane<f64> {
    let o = pose.at(MEET);
    let n = pose.at([MEET[0], MEET[1], MEET[2] + 1.0]) - o;
    let s = if up { 1.0 } else { -1.0 };
    split_plane(o, Vec3::new(s * n.x, s * n.y, s * n.z), t())
}

/// Splits `u` at `pose` either way up and asserts both sides sound: the
/// volume above the top and below it (`volumes`), tier 3, corners
/// disjoint, and, where given, the pieces round `MEET` above and below
/// (`pieces`) as vertices there.
fn splits_sound(
    what: &str,
    u: &AtRestBody<f64>,
    pose: &Pose,
    volumes: [f64; 2],
    pieces: Option<[usize; 2]>,
) {
    let apex = at(pose.at(MEET));
    for up in [true, false] {
        let what = format!(
            "{what}, {}, the plane's normal {}",
            pose.label,
            if up { "up" } else { "down" }
        );
        let r =
            split(u, &plane_at(pose, up), t()).unwrap_or_else(|e| panic!("{what}: refused: {e:?}"));
        let (top, bottom) = if up { (0, 1) } else { (1, 0) };
        for (part, k, side) in [(&r.above, top, "above"), (&r.below, bottom, "below")] {
            let what = format!("{what}, side {side}");
            let SplitPart::Body(b) = part else {
                panic!("{what}: empty");
            };
            let v = topo::test_support::pinned_volume("the measured body", b, t()).volume;
            // A vertex read in the plane within `ε` of it moves the cut
            // by as much.
            assert!(
                (v - volumes[k]).abs() < 1e-9 + t().eps(),
                "{what}: volume {v}, closed form {}",
                volumes[k]
            );
            assert_eq!(validate_geometric(b, t()), Ok(()), "{what}: tier 3");
            assert_eq!(corners_disjoint(b, t()), Ok(()), "{what}: corners");
            let at_apex = b
                .vertices()
                .filter(|&(k, _)| at(topo::readback::vertex_point(b, k).unwrap()) == apex)
                .count();
            if let Some(pieces) = pieces {
                assert_eq!(at_apex, pieces[k], "{what}: vertices at MEET");
            }
        }
    }
}

/// [`apex_pyramid`] over `base` at every pose, sound
/// ([`splits_sound`]) against [`pyramid_volumes`].
fn pyramid_splits_sound(what: &str, base: &[[f64; 3]], pieces: Option<[usize; 2]>) {
    let (whole, below) = pyramid_volumes(base);
    for pose in poses() {
        let u = apex_pyramid(base, &pose, t());
        splits_sound(what, &u, &pose, [whole - below, below], pieces);
    }
}

/// **Every base, pose and plane orientation splits into two sound
/// sides**: closed-form volumes, tier 3, corners disjoint, and one
/// vertex at the apex per piece round it on that side (the connected
/// parts of the base polygon on each side of `z = 0`).
#[test]
fn a_split_through_an_apex_closes_each_corner_of_its_section_on_its_own() {
    let bases = [
        ("a single arch", single_arch(), [1, 1]),
        ("a zigzag, two arches apart", zigzag(), [2, 1]),
        ("a crown, three arches apart", crown(), [3, 1]),
        ("the comb, two runs nested under one", comb(), [1, 3]),
        (
            "the arch cone, a run inside one inside one",
            arch_cone(),
            [2, 2],
        ),
        ("the branching cone", branching_cone(), [3, 3]),
    ];
    for (name, base, pieces) in bases {
        pyramid_splits_sound(name, &base, Some(pieces));
    }
}

/// **An edge at the apex in the plane, exactly or within the tolerance,
/// splits sound.** A zigzag whose valley corner lies `δ` off the top,
/// `δ ∈ {0, ±0.3 ε, ±0.6 ε, ±0.9 ε}`: the valley edge is read in the
/// plane, rule (b) sends it Below, and the two germs either side of it
/// are ordered by the topology, so no reading of their `δ`-tilted
/// directions decides anything. The two arches are two pieces above,
/// meeting along the valley edge.
#[test]
fn an_edge_at_the_apex_in_the_plane_splits_sound_exact_and_near() {
    let eps = t().eps();
    for s in [0.0, 0.3, -0.3, 0.6, -0.6, 0.9, -0.9] {
        let base = [
            [0.5, -0.375, -0.2],
            [0.5, 0.375, -0.2],
            [0.5, 0.1875, 0.2],
            [0.5, 0.0, s * eps],
            [0.5, -0.1875, 0.2],
        ];
        pyramid_splits_sound(&format!("a zigzag's valley at {s} ε"), &base, Some([2, 1]));
    }
}

/// **A crown whose valleys lie in the plane splits sound**: the
/// [`posed_crown`] over a ring whose valleys sit at the height of
/// `MEET`, so every germ there is one of a pair either side of an
/// in-plane valley edge, and the pairs alone fix the corners. Five
/// corners, two valleys: the floor of a V-groove bent at the vertex,
/// with its walls rising either side. Below, the sections either side
/// of each valley edge merge into one face, so no vertex is left at
/// `MEET`. The volume above the top is the closed form [`crown_above`].
#[test]
fn a_crown_whose_valleys_lie_in_the_plane_splits_sound() {
    let rings: [(&str, &[f64], [usize; 2]); 3] = [
        ("three valleys", &[0.3, 0.0, 0.3, 0.0, 0.3, 0.0], [3, 0]),
        (
            "a V-groove floor bent at the vertex",
            &[0.3, 0.0, 0.25, 0.0, 0.2],
            [2, 0],
        ),
        (
            "three valleys, uneven",
            &[0.3, 0.0, 0.25, 0.0, 0.2, 0.0],
            [3, 0],
        ),
    ];
    for (name, heights, pieces) in rings {
        let n = heights.len();
        let ring: Vec<[f64; 3]> = (0..n)
            .map(|i| {
                let a = std::f64::consts::TAU * (i as f64 + 0.13 * (i % 2) as f64) / n as f64;
                let r = 1.0 + 0.1 * (i % 3) as f64;
                [r * a.cos(), r * a.sin(), heights[i]]
            })
            .collect();
        let above = crown_above(&ring, -1.0);
        for pose in poses() {
            let u = posed_crown(&ring, [0.0, 0.0, -1.0], &pose, t());
            let whole = topo::test_support::pinned_volume("the measured body", &u, t()).volume;
            splits_sound(name, &u, &pose, [above, whole - above], Some(pieces));
        }
    }
}

/// The volume of a [`posed_crown`] at rest above `z = 0`, in closed form,
/// over a ring at heights `h ≥ 0` round `MEET` with its bottom at
/// `bottom` below `MEET`: over each sector `(MEET, p, q)` of the ring,
/// the top fan's height, `area·(h_p + h_q)/3`, less the part of the
/// bottom fan above `z = 0`, a linear height integrated over the sector
/// clipped to where it is positive.
fn crown_above(ring: &[[f64; 3]], bottom: f64) -> f64 {
    let n = ring.len();
    // ∫ of a linear height over a polygon `(x, y, height)`, fanned.
    let integral = |poly: &[[f64; 3]]| -> f64 {
        (1..poly.len().saturating_sub(1))
            .map(|i| {
                let (a, b, c) = (poly[0], poly[i], poly[i + 1]);
                let area = ((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])) / 2.0;
                area * (a[2] + b[2] + c[2]) / 3.0
            })
            .sum()
    };
    (0..n)
        .map(|i| {
            let (p, q) = (ring[i], ring[(i + 1) % n]);
            let top = integral(&[[0.0, 0.0, 0.0], p, q]);
            // The bottom fan's sector, clipped to its height above zero.
            let sector = [[0.0, 0.0, bottom], p, q];
            let mut clipped = Vec::new();
            for k in 0..3 {
                let (a, b) = (sector[k], sector[(k + 1) % 3]);
                if a[2] >= 0.0 {
                    clipped.push(a);
                }
                if (a[2] >= 0.0) != (b[2] >= 0.0) {
                    let s = a[2] / (a[2] - b[2]);
                    clipped.push([a[0] + s * (b[0] - a[0]), a[1] + s * (b[1] - a[1]), 0.0]);
                }
            }
            top - integral(&clipped)
        })
        .sum()
}

/// **A crown with one ring corner in the plane splits sound**: the
/// [`posed_crown`] over a ring with one corner at the height of `MEET`,
/// exactly or `0.6 ε` or `0.9 ε` off it, and the rest above. The edge from `MEET`
/// to that corner is a valley in the plane, and the material above
/// runs round `MEET` from one side of it to the other, so the side
/// above touches itself along the valley: its section face runs out
/// along it and back, two chords from two copies of the corner to one
/// vertex at `MEET`. The side below keeps no vertex at `MEET`: the
/// valley lies inside its section face. Volumes against the closed
/// form [`crown_above`], and the outline `plane_section` reads against
/// the bottom fan's section ([`section_is`]); the ring families vary
/// its size and which corner lies in the plane, the seed corner the
/// crown is built from among them.
#[test]
fn a_crown_with_one_corner_in_the_plane_splits_sound() {
    let c2 = vec![
        [0.4760, 1.3181, 0.3518],
        [-0.7080, 0.8592, 0.1597],
        [-1.1890, 0.5653, 0.1028],
        [-1.1262, -0.2072, 0.0],
        [0.4166, -1.4394, 0.2963],
    ];
    let around = |heights: &[f64]| -> Vec<[f64; 3]> {
        let n = heights.len();
        (0..n)
            .map(|i| {
                let a = std::f64::consts::TAU * (i as f64 + 0.13 * (i % 2) as f64) / n as f64;
                let r = 1.0 + 0.1 * (i % 3) as f64;
                [r * a.cos(), r * a.sin(), heights[i]]
            })
            .collect()
    };
    let rings = [
        ("the fuzz's witness C2", c2),
        ("three corners", around(&[0.2, 0.0, 0.3])),
        (
            "four corners, the seed in the plane",
            around(&[0.0, 0.25, 0.3, 0.15]),
        ),
        (
            "five corners, the last in the plane",
            around(&[0.3, 0.2, 0.25, 0.1, 0.0]),
        ),
        ("six corners", around(&[0.3, 0.05, 0.0, 0.4, 0.2, 0.25])),
    ];
    let eps = t().eps();
    for (name, ring) in rings {
        let low = ring
            .iter()
            .position(|c| c[2] == 0.0)
            .expect("one corner in the plane");
        for s in [0.0, 0.6, -0.6, 0.9, -0.9] {
            let mut ring = ring.clone();
            ring[low][2] = s * eps;
            let above = crown_above(&ring, -1.0);
            // The section is the bottom fan's: the corners where its
            // edges cross the plane, one the corner in it.
            let cut: Vec<[f64; 2]> = ring
                .iter()
                .map(|c| {
                    let s = 1.0 / (c[2] + 1.0);
                    [s * c[0], s * c[1]]
                })
                .collect();
            let section = (0..cut.len())
                .map(|i| {
                    let (p, q) = (cut[i], cut[(i + 1) % cut.len()]);
                    p[0] * q[1] - p[1] * q[0]
                })
                .sum::<f64>()
                / 2.0;
            let what = format!("{name}, its corner at {s} ε");
            for pose in poses() {
                let u = posed_crown(&ring, [0.0, 0.0, -1.0], &pose, t());
                let whole = topo::test_support::pinned_volume("the measured body", &u, t()).volume;
                splits_sound(&what, &u, &pose, [above, whole - above], Some([1, 0]));
                section_is(&what, &u, &pose, section, ring.len());
            }
        }
    }
}

/// `plane_section` through `MEET` at `pose`, either way up, reads one
/// region of `area` (closed form) whose outline has `corners` corners,
/// no two within the escalation distance `K·ε` of one another: no spur
/// runs out from it and back.
fn section_is(what: &str, u: &AtRestBody<f64>, pose: &Pose, area: f64, corners: usize) {
    for up in [true, false] {
        let what = format!("{what}, {}, the plane's normal up {up}", pose.label);
        let cut = topo::plane_section(u, &plane_at(pose, up), t())
            .unwrap_or_else(|e| panic!("{what}: the section refused: {e:?}"));
        let [region] = cut.regions.as_slice() else {
            panic!("{what}: {} regions, not one", cut.regions.len());
        };
        assert!(
            (region.area() - area).abs() < 1e-9 + t().eps(),
            "{what}: section area {}, closed form {area}",
            region.area()
        );
        let points = region.outline.points();
        // Two corners closer than the escalation distance are one.
        let near = t().eps() * t().k();
        let repeated = (0..points.len())
            .any(|i| (i + 1..points.len()).any(|j| points[i].distance(points[j]) <= near));
        assert!(
            points.len() == corners && !repeated,
            "{what}: the outline {:?}, not {corners} distinct corners",
            points.iter().map(|&p| at(p)).collect::<Vec<_>>()
        );
    }
}

/// **A trench whose keel lies in the plane, closed at one end and open
/// at the other, splits sound**: a box less a prism of triangular
/// section along `+x`, its keel from `MEET` out through the box's side
/// at `x = 4`, built by `topo::subtract` at every pose. Above holds the
/// box's top less the trench, touching itself along the keel, with one
/// vertex at `MEET`; below holds the box's bottom, and the keel lies
/// inside its section face. The section is the box's, a rectangle.
#[test]
fn a_trench_whose_keel_lies_in_the_plane_splits_sound() {
    use topo::test_support::{FaceGeometry, describe_as_intersections, finished, prism_ops};
    let tol = t();
    for pose in poses() {
        let solid = |what: &str, profile: &[(f64, f64)], span: (f64, f64), axis_x: bool| {
            let mut b = topo::Body::<f64>::new();
            if axis_x {
                prism_ops(
                    &mut b,
                    profile,
                    span,
                    |a, c, h| pose.at([h, a, c]),
                    FaceGeometry::Certified,
                    tol,
                );
            } else {
                prism_ops(
                    &mut b,
                    profile,
                    span,
                    |x, y, z| pose.at([x, y, z]),
                    FaceGeometry::Certified,
                    tol,
                );
            }
            describe_as_intersections(&mut b, tol);
            finished(what, b, tol)
        };
        let block = solid(
            "the box",
            &[(0.0, -1.0), (4.0, -1.0), (4.0, 3.0), (0.0, 3.0)],
            (0.0, 2.0),
            false,
        );
        let trench = solid(
            "the trench",
            &[(1.0, 1.0), (1.6, 3.0), (0.4, 3.0)],
            (1.5, 4.5),
            true,
        );
        let topo::BooleanResult::Body(cut) = topo::subtract(&block, &trench, tol)
            .unwrap_or_else(|e| panic!("{}: the box less the trench: {e:?}", pose.label))
        else {
            panic!("{}: the box less the trench is empty", pose.label);
        };
        // Above: the box's top half less the trench's part below its
        // top, a triangle 0.6 wide at z = 2 over the 2.5 inside the box.
        let above = 16.0 - 0.5 * 0.6 * 1.0 * 2.5;
        splits_sound("a trench", &cut.body, &pose, [above, 16.0], Some([1, 0]));
        section_is("a trench", &cut.body, &pose, 16.0, 5);
    }
}

/// A pyramid base from `(y, z)` corners in the plane half a unit along
/// `+x`, `z` scaled by `ε` where `in_eps`.
fn base_of(corners: &[(f64, f64, bool)]) -> Vec<[f64; 3]> {
    let eps = t().eps();
    corners
        .iter()
        .map(|&(y, z, in_eps)| [0.5, y, if in_eps { z * eps } else { z }])
        .collect()
}

/// **Untangled random bases with corners within the tolerance of the
/// plane split sound**, at every pose either way up, against the closed
/// form: four bases drawn by the review's fuzz (seed
/// `0x9E3779B97F4A7C15`), each with two or more corners read in the
/// plane at `δ ∈ {0, ±0.3 ε, ±0.6 ε, ±0.9 ε}`. Their germs mix pairs
/// beside in-plane edges with germs of other sectors, so the order read
/// between them is pinned.
#[test]
fn random_bases_with_corners_in_the_band_split_sound() {
    let bases = [
        vec![
            (0.1624, 0.2349, false),
            (-0.3483, 0.0198, false),
            (-0.3115, 0.0, true),
            (-0.3049, -0.1276, false),
            (-0.1191, -0.2310, false),
            (0.3140, -0.2955, false),
            (0.3173, -0.9, true),
            (0.1422, -0.0457, false),
            (0.1595, 0.0750, false),
            (0.3689, 0.0, true),
        ],
        vec![
            (0.0069, -0.1705, false),
            (0.1677, 0.0167, false),
            (0.2073, 0.0697, false),
            (-0.0899, 0.2899, false),
            (0.0107, 0.0, true),
            (-0.1137, 0.1222, false),
            (-0.3990, 0.6, true),
            (-0.2155, 0.6, true),
            (-0.2004, -0.2192, false),
            (-0.1011, -0.0868, false),
            (-0.1799, -0.2115, false),
        ],
        vec![
            (0.1472, -0.0231, false),
            (-0.0031, 0.9, true),
            (-0.0582, -0.1681, false),
            (-0.2239, 0.9, true),
            (-0.1410, 0.9, true),
            (-0.1251, 0.0459, false),
            (0.3563, 0.0908, false),
        ],
        vec![
            (0.1798, -0.1702, false),
            (0.1596, 0.1138, false),
            (0.0818, 0.0, true),
            (0.1762, 0.1967, false),
            (-0.2215, 0.2067, false),
            (-0.0670, -0.9, true),
            (-0.1700, -0.0148, false),
            (-0.3876, 0.3, true),
            (-0.3589, -0.0302, false),
            (-0.1389, -0.0299, false),
        ],
    ];
    for (k, corners) in bases.iter().enumerate() {
        pyramid_splits_sound(&format!("random base {k}"), &base_of(corners), None);
    }
}

/// **A notch whose floor lies `0.9 ε` above the plane splits sound,
/// the two lobes above it apart round the apex**, at every pose either
/// way up, against the closed form: a base the review's fuzz drew
/// (seed `0x9E3779B97F4A7C15`, its 123rd). The floor is read On, so the
/// edge from the apex to it lies in the plane. Each piece passes tier
/// 3′ with no contact records: the two edges continuing each other
/// through the floor, kinked by `0.9 ε`, overlap nowhere.
#[test]
fn a_notch_floor_in_the_band_splits_sound() {
    let base = base_of(&[
        (-0.3709, 0.1491, false),
        (-0.0536, -0.2641, false),
        (0.3537, -0.2767, false),
        (-0.0022, 0.0927, false),
        (-0.0422, 0.9, true),
    ]);
    pyramid_splits_sound("a notch floor 0.9 ε above", &base, Some([2, 1]));
    for pose in poses() {
        let u = apex_pyramid(&base, &pose, t());
        for up in [true, false] {
            let r = split(&u, &plane_at(&pose, up), t()).unwrap();
            for (part, side) in [(&r.above, "above"), (&r.below, "below")] {
                let SplitPart::Body(b) = part else {
                    panic!("{}: side {side} empty", pose.label);
                };
                assert_eq!(
                    topo::validate_pseudomanifold(b, &topo::ContactRecords::default(), t()),
                    Ok(()),
                    "{}, up {up}, side {side}: tier 3′",
                    pose.label
                );
            }
        }
    }
}

/// A W-shaped base whose valley dips `1000 ε` below the top between
/// peaks so close that the plane leaves the valley's two faces along
/// germs about `gap·ε` apart at the apex's reach. The crossings beside
/// the valley lie `w·depth/(0.1 + depth)` either side of it, about
/// `20·w·depth` apart.
fn w_base(gap: f64) -> [[f64; 3]; 5] {
    let (eps, depth) = (t().eps(), 1000.0 * t().eps());
    let w = gap * eps / (20.0 * depth);
    [
        [0.5, -0.3, -0.1],
        [0.5, 0.3, -0.1],
        [0.5, w, 0.1],
        [0.5, 0.0, -depth],
        [0.5, -w, 0.1],
    ]
}

/// **Germs within the band of one another refuse typed, and germs on one
/// line to the tolerance refuse as a coincidence.** [`w_base`] at a gap
/// of `√10 ε` reads the order of the valley's two germs in the band
/// (`SectionGermsEscalated`, an order reading); at `0.3 ε` it decides
/// them on one line, two vertices of the section within the tolerance
/// of each other (`SectionGermsUnordered`, `Coincident`). Every pose,
/// either way up.
#[test]
fn germs_within_the_band_or_on_one_line_refuse_typed() {
    for pose in poses() {
        let near = apex_pyramid(&w_base(10f64.sqrt()), &pose, t());
        let on = apex_pyramid(&w_base(0.3), &pose, t());
        for up in [true, false] {
            let what = format!("{}, up {up}", pose.label);
            match split(&near, &plane_at(&pose, up), t()) {
                Err(topo::SplitError::Reduce(SplitReduceError::SectionGermsEscalated {
                    reading: topo::SectionGermReading::Order,
                    ..
                })) => {}
                other => panic!("{what}, √10 ε: {:?}", other.map(|_| "a body")),
            }
            match split(&on, &plane_at(&pose, up), t()) {
                Err(topo::SplitError::Reduce(SplitReduceError::SectionGermsUnordered {
                    fault: topo::SectionGermFault::Coincident,
                    ..
                })) => {}
                other => panic!("{what}, 0.3 ε: {:?}", other.map(|_| "a body")),
            }
        }
    }
}

/// **A split leaves no edge between a section and a base face it lies
/// flush with.** Random base 2 of
/// [`random_bases_with_corners_in_the_band_split_sound`] cuts a section
/// lying on a base face's plane within the tolerance, which the merge's
/// plane ladder decides one carrier (Zero); each half ends with maximal
/// faces, so the merge takes the pair into one face, at least one group
/// per half, and no conventional edge (`OnPair`, or an image in one
/// plane's chart) is left between two distinct planes.
#[test]
fn a_split_merges_a_section_flush_with_a_base_face() {
    let corners = [
        (0.0069, -0.1705, false),
        (0.1677, 0.0167, false),
        (0.2073, 0.0697, false),
        (-0.0899, 0.2899, false),
        (0.0107, 0.0, true),
        (-0.1137, 0.1222, false),
        (-0.3990, 0.6, true),
        (-0.2155, 0.6, true),
        (-0.2004, -0.2192, false),
        (-0.1011, -0.0868, false),
        (-0.1799, -0.2115, false),
    ];
    let base = base_of(&corners);
    for pose in poses() {
        let u = apex_pyramid(&base, &pose, t());
        for up in [true, false] {
            let r = split(&u, &plane_at(&pose, up), t()).expect("random base 2 splits");
            for (side, part) in [
                (topo::splitting::PlaneSide::Above, &r.above),
                (topo::splitting::PlaneSide::Below, &r.below),
            ] {
                let SplitPart::Body(b) = part else { continue };
                assert!(
                    r.naming.merge_groups.iter().any(|(s, _)| *s == side),
                    "{}, up {up}: the {side:?} half merges the flush pair",
                    pose.label
                );
                let plane = |f: topo::FaceKey| {
                    matches!(
                        b.get_surface(b.get_face(f).unwrap().surface),
                        Some(geom::Surface::Plane { .. })
                    )
                };
                for (edge, e) in b.edges() {
                    let Some(c) = b
                        .get_curve_geom(e.curve)
                        .and_then(topo::CurveGeom::certified)
                    else {
                        continue;
                    };
                    let sides = topo::readback::edge_sides(b, edge).unwrap();
                    let (sa, sb) = sides.surfaces();
                    let conventional = matches!(
                        c.description(),
                        geom_brep::EdgeDescription::OnPair { .. }
                            | geom_brep::EdgeDescription::Chart(_)
                    );
                    let (fa, fb) = sides.faces();
                    assert!(
                        !conventional || sa == sb || !(plane(fa) && plane(fb)),
                        "{}, up {up}: {edge:?} is a conventional edge between two planes",
                        pose.label
                    );
                }
            }
        }
    }
}
