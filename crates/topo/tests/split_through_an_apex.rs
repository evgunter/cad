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
            let v = topo::mass_properties(b, t()).unwrap().volume;
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
/// with its walls rising either side. The volume above the top is the
/// closed form [`crown_above`].
#[test]
fn a_crown_whose_valleys_lie_in_the_plane_splits_sound() {
    let rings: [(&str, &[f64], [usize; 2]); 3] = [
        ("three valleys", &[0.3, 0.0, 0.3, 0.0, 0.3, 0.0], [3, 1]),
        (
            "a V-groove floor bent at the vertex",
            &[0.3, 0.0, 0.25, 0.0, 0.2],
            [2, 1],
        ),
        (
            "three valleys, uneven",
            &[0.3, 0.0, 0.25, 0.0, 0.2, 0.0],
            [3, 1],
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
            let whole = topo::mass_properties(&u, t()).unwrap().volume;
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

/// **A corner `0.9 ε` off the plane can leave a re-charted edge's
/// residual in the band, and the split refuses it typed.** A base the
/// review's fuzz drew (seed `0x9E3779B97F4A7C15`, its 123rd), split at
/// the pose turned in general with the plane's normal down: the finish
/// re-charts an edge at the vertex read On, and its residual lands in
/// the band (`pcurve_map_residual`, about `1.1 ε`), so the split refuses
/// `ChartResidual` rather than ship it. The other cells of this base
/// split sound. The class is filed as
/// `work/topo/a-split-recharts-an-edge-at-a-vertex-read-on-off-the-plane-into-the-band.md`.
#[test]
fn a_recharted_residual_in_the_band_refuses_typed() {
    let base = base_of(&[
        (-0.3709, 0.1491, false),
        (-0.0536, -0.2641, false),
        (0.3537, -0.2767, false),
        (-0.0022, 0.0927, false),
        (-0.0422, 0.9, true),
    ]);
    let pose = poses().into_iter().find(|p| p.label == "turned").unwrap();
    let u = apex_pyramid(&base, &pose, t());
    match split(&u, &plane_at(&pose, false), t()) {
        Err(topo::SplitError::Finish(topo::SplitFinishError::Euler(
            topo::EulerOpError::RechartFalsifies { error, .. }
            | topo::EulerOpError::Certification { error },
        ))) => assert!(
            matches!(
                error,
                topo::CertifyError::Escalated {
                    check: geom_brep::CertCheck::ChartResidual,
                    ..
                }
            ),
            "{error:?}"
        ),
        other => panic!("{:?}", other.map(|_| "a body")),
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
