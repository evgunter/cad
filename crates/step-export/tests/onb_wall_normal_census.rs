//! **Wall-normal `z` census over the STEP-export fixture corpus**, and
//! the byte-movement list that follows from it.
//!
//! `Vec3::orthonormal_basis` hulls the frame at `Interval` whenever the
//! normal's `z` encloses zero. One proposed narrowing canonicalises the
//! zero at `f64` (`copysign(1, n.z + 0)`), which moves `f64` bits — and
//! therefore committed `DIRECTION` records — on exactly those planar
//! faces whose stored normal has `z = -0.0` today. This counts them,
//! and names the fixture and record for each one it finds.
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

use std::collections::HashMap;

use crate::common;

use geom::Surface;
use geom_core::{Tol, Vec3};
use step_export::{StepOptions, step_string};

/// The four classes the ruling asks for.
#[derive(Default, Clone, Copy)]
pub struct ZClasses {
    pub pos_zero: usize,
    pub neg_zero: usize,
    pub tiny: usize,
    pub other: usize,
}

impl ZClasses {
    pub fn add(&mut self, z: f64) {
        if z == 0.0 {
            if z.is_sign_negative() {
                self.neg_zero += 1;
            } else {
                self.pos_zero += 1;
            }
        } else if z.abs() < 1e-12 {
            self.tiny += 1;
        } else {
            self.other += 1;
        }
    }

    pub fn planes(&self) -> usize {
        self.pos_zero + self.neg_zero + self.tiny + self.other
    }
}

/// **Table 2 (step-export row)** — planar faces per fixture, by the
/// class of the stored normal's `z`.
#[test]
#[ignore = "wall-normal census instrument; run explicitly"]
fn wall_normal_z_census_over_the_fixture_corpus() {
    println!("| fixture | planes | z = +0.0 | z = -0.0 | 0 < |z| < 1e-12 | other |");
    println!("| --- | --- | --- | --- | --- | --- |");
    let mut total = ZClasses::default();
    for (name, body) in common::fixture_corpus() {
        let mut c = ZClasses::default();
        for (_, surface) in body.surfaces() {
            if let Surface::Plane { normal, .. } = surface {
                c.add(normal.z);
            }
        }
        total.pos_zero += c.pos_zero;
        total.neg_zero += c.neg_zero;
        total.tiny += c.tiny;
        total.other += c.other;
        println!(
            "| {name} | {} | {} | {} | {} | {} |",
            c.planes(),
            c.pos_zero,
            c.neg_zero,
            c.tiny,
            c.other
        );
    }
    println!(
        "| **step-export corpus** | {} | {} | {} | {} | {} |",
        total.planes(),
        total.pos_zero,
        total.neg_zero,
        total.tiny,
        total.other
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
        let v: Vec<f64> = inner.split(',').filter_map(|t| t.trim().parse().ok()).collect();
        (v.len() == 3).then(|| [v[0], v[1], v[2]])
    };
    let same = |a: [f64; 3], b: [f64; 3]| (0..3).all(|i| a[i].to_bits() == b[i].to_bits());
    let mut hits: Vec<String> = Vec::new();
    for (id, rec) in &by_id {
        if !rec.starts_with("PLANE('', #") {
            continue;
        }
        let Some(pid) = rec.split_once("#").map(|(_, r)| r.trim_end_matches(&[')', ';'][..])) else {
            continue;
        };
        let Some(place) = by_id.get(format!("#{pid}").as_str()) else {
            continue;
        };
        let refs: Vec<&str> = place.split('#').skip(1).map(|t| t.trim_end_matches(&[')', ';', ',', ' '][..])).collect();
        if refs.len() != 3 {
            continue;
        }
        let (Some(a), Some(r)) = (
            by_id.get(format!("#{}", refs[1]).as_str()).and_then(|l| reals(l)),
            by_id.get(format!("#{}", refs[2]).as_str()).and_then(|l| reals(l)),
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

/// The frame `Vec3::orthonormal_basis` mints today, respelled locally
/// so the counterfactual can sit beside it. `canonicalise` is option
/// (c'): `s = copysign(1, n.z + 0)`, which IEEE turns `-0.0` into
/// `+0.0` before the sign is read.
fn b1(n: Vec3<f64>, canonicalise: bool) -> Vec3<f64> {
    let s = 1.0f64.copysign(if canonicalise { n.z + 0.0 } else { n.z });
    let r = 1.0 / (1.0 + n.z.abs());
    Vec3::new(1.0 - n.x.powi(2) * r, -((n.x * n.y) * r), -(s * n.x))
}

fn same_bits(a: Vec3<f64>, b: Vec3<f64>) -> bool {
    a.x.to_bits() == b.x.to_bits()
        && a.y.to_bits() == b.y.to_bits()
        && a.z.to_bits() == b.z.to_bits()
}

/// **Table 3 (STEP half)** — which committed `DIRECTION` records move
/// under the canonicalising narrowing.
///
/// A plane's `u_ref` is written verbatim into the `AXIS2_PLACEMENT_3D`
/// of its `PLANE` record (`writer.rs`'s plane arm), so a `u_ref` that
/// moves moves committed bytes. Under (c') `s` changes on exactly the
/// planes whose normal has `z = -0.0`, and `b1.z = -(s * n.x)` is the
/// only component carrying `s` — so this walks those planes, checks the
/// stored frame really is the one `orthonormal_basis` mints (a frame
/// stored from elsewhere would not move at all), and prints the record
/// as written today beside the frame (c') would write.
#[test]
#[ignore = "byte-movement instrument; run explicitly"]
fn direction_records_that_move_under_the_canonicalised_sign() {
    let tol = Tol::witness();
    let (mut neg_zero, mut moved) = (0usize, 0usize);
    println!("| fixture | normal | stored u_ref | minted here? | u_ref under (c') | `u_ref` DIRECTION record(s) |");
    println!("| --- | --- | --- | --- | --- | --- |");
    for (name, body) in common::fixture_corpus() {
        let text = step_string(&body, &StepOptions::default(), tol).expect("fixture exports");
        for (_, surface) in body.surfaces() {
            let Surface::Plane { normal, u_ref, .. } = surface else {
                continue;
            };
            if !(normal.z == 0.0 && normal.z.is_sign_negative()) {
                continue;
            }
            neg_zero += 1;
            let today = b1(*normal, false);
            let minted = same_bits(*u_ref, today);
            let under = b1(*normal, true);
            let line = u_ref_records(
                &text,
                [normal.x, normal.y, normal.z],
                [u_ref.x, u_ref.y, u_ref.z],
            );
            if minted && !same_bits(today, under) {
                moved += 1;
            }
            println!(
                "| {name} | ({:?}, {:?}, {:?}) | ({:?}, {:?}, {:?}) | {minted} | ({:?}, {:?}, {:?}) | {line} |",
                normal.x, normal.y, normal.z,
                u_ref.x, u_ref.y, u_ref.z,
                under.x, under.y, under.z
            );
        }
    }
    if neg_zero == 0 {
        println!("| **(none)** | — | — | — | — | no planar face stores `n.z = -0.0` |");
    }
    println!("planes with n.z = -0.0: {neg_zero}; DIRECTION records that move under (c'): {moved}");
}
