//! **The frame-seam census over the STEP-export fixture corpus**, and
//! the `DIRECTION` records the frames are written to.
//!
//! `Vec3::orthonormal_basis` crosses the normal with the world axis its
//! own components choose — `e_z` when `|n.z| ≤ max(|n.x|, |n.y|)/2` —
//! and the equality is its one discontinuity. The classification lives
//! in `test_utils::seam_census`, shared with the other two corpus
//! instruments; its docs say what a count of normals on the seam does
//! and does not measure. A plane's `u_ref` is written verbatim into the
//! `AXIS2_PLACEMENT_3D` of its `PLANE` record, so the frames these
//! fixtures store are committed bytes; the second table locates the
//! record each one is written to, which is the receipt a re-bless of a
//! byte-golden fixture is owed.
//!
//! [`no_fixture_face_sits_on_the_frame_seam`] ASSERTS this corpus's
//! zero; the two instruments below it print the tables and are
//! `#[ignore]`d.
//!
//! ```text
//! cargo test -p step-export --test all \
//!     -- --ignored --nocapture onb_wall_normal_census
//! ```

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::HashMap;

use crate::common;

use geom::Surface;
use geom_core::{Tol, Vec3};
use step_export::{StepOptions, step_string};
use test_utils::seam_census::SeamClasses;

/// **This corpus's half of the measurement that decided the
/// comparison, asserted**: no fixture body has a planar face on the
/// seam `|n.z| = max(|n.x|, |n.y|)/2`, so no `DIRECTION` record these
/// byte-golden fixtures commit can be a hulled frame's. The instrument
/// below prints the per-fixture table.
#[test]
fn no_fixture_face_sits_on_the_frame_seam() {
    let mut total = SeamClasses::default();
    let mut fixtures = 0usize;
    for (name, body) in common::fixture_corpus() {
        let mut c = SeamClasses::default();
        for (_, surface) in body.surfaces() {
            if let Surface::Plane { normal, .. } = surface {
                c.add((normal.x, normal.y, normal.z));
            }
        }
        assert_eq!(
            c.on_seam,
            0,
            "{name}: {} of its {} planar faces are on the frame seam",
            c.on_seam,
            c.planes()
        );
        total.merge(c);
        fixtures += 1;
    }
    println!(
        "step-export corpus ({fixtures} fixtures): {} planar faces, {} on the seam, \
         {} on the three-component order's tie set",
        total.planes(),
        total.on_seam,
        total.three_way_tie
    );
    // The floor is below the measured 81 planar faces over 17 fixture
    // bodies because which bodies build is ε-dependent; the zero above
    // is not.
    assert!(
        fixtures >= 17 && total.planes() >= 60,
        "the fixture corpus shrank: {fixtures} fixtures, {} planar faces",
        total.planes()
    );
    assert!(
        total.three_way_tie * 2 > total.planes(),
        "the three-component order's tie set no longer covers most of the corpus: \
         {} of {}",
        total.three_way_tie,
        total.planes()
    );
}

/// Planar faces per fixture, by whether the normal sits on the axis
/// order's tie set and by which axis wins.
#[test]
#[ignore = "tie census instrument; run explicitly"]
fn axis_tie_census_over_the_fixture_corpus() {
    println!(
        "| fixture | planes | on the seam | off it | e_z arm | e_y arm | on a three-way tie |"
    );
    println!("| --- | --- | --- | --- | --- | --- | --- | --- |");
    let mut total = SeamClasses::default();
    for (name, body) in common::fixture_corpus() {
        let mut c = SeamClasses::default();
        for (_, surface) in body.surfaces() {
            if let Surface::Plane { normal, .. } = surface {
                c.add((normal.x, normal.y, normal.z));
            }
        }
        total.merge(c);
        println!(
            "| {name} | {} | {} | {} | {} | {} | {} |",
            c.planes(),
            c.on_seam,
            c.off_seam,
            c.e_z_arm,
            c.e_y_arm,
            c.three_way_tie
        );
    }
    println!(
        "| **step-export corpus** | {} | {} | {} | {} | {} | {} |",
        total.planes(),
        total.on_seam,
        total.off_seam,
        total.e_z_arm,
        total.e_y_arm,
        total.three_way_tie
    );
}

/// The `DIRECTION` record a plane's `u_ref` is actually written to.
///
/// Followed, not guessed: `PLANE('', #p)` names an
/// `AXIS2_PLACEMENT_3D('', #cp, #a, #r)` whose third reference is the
/// `u_ref` direction, so a plane whose axis record carries `normal`'s
/// bits and whose ref record carries `u_ref`'s bits identifies `#r`
/// exactly. Every matching plane is reported (an axis-aligned corpus
/// repeats frames across faces and across bodies).
fn u_ref_records(text: &str, normal: [f64; 3], u_ref: [f64; 3]) -> String {
    let mut by_id: HashMap<&str, &str> = HashMap::new();
    for line in text.lines().map(str::trim) {
        if let Some((id, rest)) = line.split_once(" = ") {
            by_id.insert(id, rest);
        }
    }
    let reals = |rec: &str| -> Option<[f64; 3]> {
        let inner = rec.split_once("('', (")?.1.split_once("))")?.0;
        let v: Vec<f64> = inner
            .split(',')
            .filter_map(|t| t.trim().parse().ok())
            .collect();
        (v.len() == 3).then(|| [v[0], v[1], v[2]])
    };
    let same = |a: [f64; 3], b: [f64; 3]| (0..3).all(|i| a[i].to_bits() == b[i].to_bits());
    let mut hits: Vec<String> = Vec::new();
    for (id, rec) in &by_id {
        if !rec.starts_with("PLANE('', #") {
            continue;
        }
        let Some(pid) = rec
            .split_once("#")
            .map(|(_, r)| r.trim_end_matches(&[')', ';'][..]))
        else {
            continue;
        };
        let Some(place) = by_id.get(format!("#{pid}").as_str()) else {
            continue;
        };
        let refs: Vec<&str> = place
            .split('#')
            .skip(1)
            .map(|t| t.trim_end_matches(&[')', ';', ',', ' '][..]))
            .collect();
        if refs.len() != 3 {
            continue;
        }
        let (Some(a), Some(r)) = (
            by_id
                .get(format!("#{}", refs[1]).as_str())
                .and_then(|l| reals(l)),
            by_id
                .get(format!("#{}", refs[2]).as_str())
                .and_then(|l| reals(l)),
        ) else {
            continue;
        };
        if same(a, normal) && same(r, u_ref) {
            hits.push(format!("#{} via {id}", refs[2]));
        }
    }
    hits.sort();
    if hits.is_empty() {
        "(not located)".to_string()
    } else {
        hits.join(", ")
    }
}

/// Every planar face's stored frame, with the `DIRECTION` record it is
/// written to — the receipt a re-bless of these byte-golden fixtures is
/// owed, and the check that the stored frame really is the one
/// `orthonormal_basis` mints (a frame stored from elsewhere would not
/// move when the constructor does).
///
/// The plane's LOCUS is the invariant across a frame change: the origin
/// and the normal are printed beside the frame so a reader can see they
/// did not move.
#[test]
#[ignore = "frame-record instrument; run explicitly"]
fn the_direction_records_every_stored_frame_is_written_to() {
    let tol = Tol::witness();
    let (mut planes, mut minted_here) = (0usize, 0usize);
    println!(
        "| fixture | normal | stored u_ref | minted by the constructor? | on a tie? | `u_ref` DIRECTION record(s) |"
    );
    println!("| --- | --- | --- | --- | --- | --- | --- |");
    for (name, body) in common::fixture_corpus() {
        let text = step_string(&body, &StepOptions::default(), tol).expect("fixture exports");
        for (_, surface) in body.surfaces() {
            let Surface::Plane { normal, u_ref, .. } = surface else {
                continue;
            };
            planes += 1;
            let minted = same_bits(*u_ref, normal.orthonormal_basis().0);
            if minted {
                minted_here += 1;
            }
            let mut m = [normal.x.abs(), normal.y.abs(), normal.z.abs()];
            m.sort_by(f64::total_cmp);
            let line = u_ref_records(
                &text,
                [normal.x, normal.y, normal.z],
                [u_ref.x, u_ref.y, u_ref.z],
            );
            println!(
                "| {name} | ({:?}, {:?}, {:?}) | ({:?}, {:?}, {:?}) | {minted} | {} | {line} |",
                normal.x,
                normal.y,
                normal.z,
                u_ref.x,
                u_ref.y,
                u_ref.z,
                m[0] == m[1]
            );
        }
    }
    println!("planar faces: {planes}; frames the constructor mints: {minted_here}");
}

fn same_bits(a: Vec3<f64>, b: Vec3<f64>) -> bool {
    a.x.to_bits() == b.x.to_bits()
        && a.y.to_bits() == b.y.to_bits()
        && a.z.to_bits() == b.z.to_bits()
}
