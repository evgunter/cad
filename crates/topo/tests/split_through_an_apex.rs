//! **A split through a vertex hangs one null edge per corner of the
//! section there, as the tree the corners make.**
//!
//! A pyramid with its apex at `MEET` ([`apex_pyramid`]) is split by the
//! plane through the apex with the top's normal, either way up, at every
//! pose. Round the apex the plane meets the solid in corners, the
//! material between two germs, and each side keeps one vertex there per
//! piece of the solid it holds round the apex. Where the runs above the
//! plane lie apart, each corner pairs one run's germs; where they nest
//! (`meeting::comb`, `meeting::arch_cone`, `meeting::branching_cone`), a
//! corner pairs germs of two runs, and a null edge per run would close
//! each section loop across the corners of others.
//!
//! Each row asserts both sides' closed-form volumes, tier 3, that every
//! face's corners at one point are angularly disjoint, and how many
//! vertices each side keeps at the apex.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{
    MEET, Pose, apex_pyramid, arch_cone, at, branching_cone, comb, corners_disjoint, crown, poses,
    pyramid_volumes, single_arch, zigzag,
};
use common::split_plane;
use geom_core::{Tol, Vec3};
use topo::{SplitPart, split, validate_geometric};

fn t() -> Tol {
    Tol::witness()
}

/// The bases, labelled, with how many pieces of the pyramid lie round
/// the apex above the top and below it: the connected parts of the base
/// polygon on each side of `z = 0`.
type Base = (&'static str, Vec<[f64; 3]>, [usize; 2]);

fn bases() -> [Base; 6] {
    [
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
    ]
}

/// The plane through the apex with the top's normal at `pose`, turned
/// over where `up` is false.
fn plane_at(pose: &Pose, up: bool) -> topo::SplitPlane<f64> {
    let o = pose.at(MEET);
    let n = pose.at([MEET[0], MEET[1], MEET[2] + 1.0]) - o;
    let s = if up { 1.0 } else { -1.0 };
    split_plane(o, Vec3::new(s * n.x, s * n.y, s * n.z), t())
}

/// **Every base, pose and plane orientation splits into two sound
/// sides**: closed-form volumes, tier 3, corners disjoint, and one
/// vertex at the apex per piece round it on that side.
#[test]
fn a_split_through_an_apex_closes_each_corner_of_its_section_on_its_own() {
    for (name, base, [pieces_above, pieces_below]) in bases() {
        let (whole, below) = pyramid_volumes(&base);
        for pose in poses() {
            let apex = at(pose.at(MEET));
            let u = apex_pyramid(&base, &pose, t());
            for up in [true, false] {
                let what = format!(
                    "{name}, {}, the plane's normal {}",
                    pose.label,
                    if up { "up" } else { "down" }
                );
                let r = split(&u, &plane_at(&pose, up), t())
                    .unwrap_or_else(|e| panic!("{what}: refused: {e:?}"));
                let sides = [
                    (&r.above, whole - below, pieces_above),
                    (&r.below, below, pieces_below),
                ];
                let sides = if up {
                    sides
                } else {
                    [
                        (&r.above, below, pieces_below),
                        (&r.below, whole - below, pieces_above),
                    ]
                };
                for (k, (part, volume, pieces)) in sides.into_iter().enumerate() {
                    let what = format!("{what}, side {}", ["above", "below"][k]);
                    let SplitPart::Body(b) = part else {
                        panic!("{what}: empty");
                    };
                    let v = topo::mass_properties(b, t()).unwrap().volume;
                    assert!(
                        (v - volume).abs() < 1e-9,
                        "{what}: volume {v}, closed form {volume}"
                    );
                    assert_eq!(validate_geometric(b, t()), Ok(()), "{what}: tier 3");
                    assert_eq!(corners_disjoint(b), Ok(()), "{what}: corners");
                    let at_apex = b
                        .vertices()
                        .filter(|&(k, _)| at(topo::readback::vertex_point(b, k).unwrap()) == apex)
                        .count();
                    assert_eq!(at_apex, pieces, "{what}: vertices at the apex");
                }
            }
        }
    }
}

/// **Germs within the band of one another refuse typed.** A W-shaped
/// base whose valley dips `1000 ε` below the top between peaks so
/// narrow that the plane leaves the valley's two faces along germs
/// about `3 ε` apart at the apex's reach: whether the two corners there
/// pair those germs with each other's neighbours is undecided, and the
/// split refuses `SectionGermsEscalated` rather than guess, at every
/// pose and either way up.
#[test]
fn germs_within_the_band_of_one_another_refuse_typed() {
    let (eps, depth) = (t().eps(), 1000.0 * t().eps());
    // The crossings beside the valley lie `w·depth/(0.1 + depth)` either
    // side of it, about `20·w·depth` apart.
    let w = 10f64.sqrt() * eps / (20.0 * depth);
    let base = [
        [0.5, -0.3, -0.1],
        [0.5, 0.3, -0.1],
        [0.5, w, 0.1],
        [0.5, 0.0, -depth],
        [0.5, -w, 0.1],
    ];
    for pose in poses() {
        let u = apex_pyramid(&base, &pose, t());
        for up in [true, false] {
            let what = format!("{}, up {up}", pose.label);
            match split(&u, &plane_at(&pose, up), t()) {
                Err(topo::SplitError::Reduce(topo::SplitReduceError::SectionGermsEscalated {
                    ..
                })) => {}
                other => panic!("{what}: {:?}", other.map(|_| "a body")),
            }
        }
    }
}
