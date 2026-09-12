//! **The axis-order tie census over the STEP-export fixture corpus**,
//! and the `DIRECTION` records the frames are written to.
//!
//! `Vec3::orthonormal_basis` crosses the normal with the world axis of
//! its smallest-magnitude component, and its one discontinuity is the
//! set where the two smallest magnitudes TIE — which every axis-aligned
//! normal sits exactly on. A plane's `u_ref` is written verbatim into
//! the `AXIS2_PLACEMENT_3D` of its `PLANE` record, so the frames these
//! fixtures store are committed bytes; the second table locates the
//! record each one is written to, which is the receipt a re-bless of a
//! byte-golden fixture is owed.
//!
//! `#[ignore]`d: asserts nothing, gates nothing, prints. The corpus is
//! `common::fixture_corpus()` — the same bodies the byte-golden
//! fixtures are written from, so a body added there is censused here
//! without editing this file.
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

/// The world-axis choice, at `f64`: which axis the normal is crossed
/// with, and whether it sits exactly ON the comparison's seam.
///
/// The seam `|n.z| = max(|n.x|, |n.y|)` is the construction's one
/// discontinuity — the 45° cone — and it is the only class an enclosure
/// of positive width can fail to decide. No axis direction and no
/// axis-aligned face is on it, which is the property the census is here
/// to measure rather than assert.
#[derive(Default, Clone, Copy)]
struct TieClasses {
    on_seam: usize,
    off_seam: usize,
    e_z_arm: usize,
    e_y_arm: usize,
    /// The same count for the rule this construction did NOT take: an
    /// order over all THREE components, whose tie set is where the two
    /// smallest magnitudes are equal. Every axis-aligned normal is on
    /// that one, which is why it is not the rule.
    three_way_tie: usize,
}

impl TieClasses {
    fn add(&mut self, n: Vec3<f64>) {
        let other = n.x.abs().max(n.y.abs());
        if n.z.abs() <= other {
            self.e_z_arm += 1;
        } else {
            self.e_y_arm += 1;
        }
        if n.z.abs() == other {
            self.on_seam += 1;
        } else {
            self.off_seam += 1;
        }
        let mut m = [n.x.abs(), n.y.abs(), n.z.abs()];
        m.sort_by(f64::total_cmp);
        if m[0] == m[1] {
            self.three_way_tie += 1;
        }
    }

    fn merge(&mut self, o: TieClasses) {
        self.on_seam += o.on_seam;
        self.off_seam += o.off_seam;
        self.e_z_arm += o.e_z_arm;
        self.e_y_arm += o.e_y_arm;
        self.three_way_tie += o.three_way_tie;
    }

    fn planes(&self) -> usize {
        self.on_seam + self.off_seam
    }
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
    let mut total = TieClasses::default();
    for (name, body) in common::fixture_corpus() {
        let mut c = TieClasses::default();
        for (_, surface) in body.surfaces() {
            if let Surface::Plane { normal, .. } = surface {
                c.add(*normal);
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
