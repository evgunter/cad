//! M4 PR 7 review ruling S1: an orientation-SENSITIVE oracle over the
//! committed fixtures.
//!
//! Why this exists: the other oracles are orientation-blind on the
//! emitted text — FreeCAD/OCC's ShapeHealing silently rectifies
//! inverted shells (the reviewer flipped all six ADVANCED_FACE flags
//! in cube.step and still got valid=True with POSITIVE volume), and
//! the truck-stepio reconstruction compares entity counts only. This
//! suite therefore parses the emitted Part 21 text itself
//! (CARTESIAN_POINT coordinates, VERTEX_POINT/EDGE_CURVE incidence,
//! ORIENTED_EDGE flags, EDGE_LOOP order, FACE_/FACE_OUTER_BOUND
//! orientation) and computes each CLOSED_SHELL's signed volume from
//! nothing but that boundary data. The boundary winding is exactly
//! the orientation axis the writer promises (interior-left ⇒ outward
//! normals ⇒ positive enclosed volume), so a flipped edge sense
//! anywhere makes these exact equalities fail.
//!
//! Exactness note: plain f64 summation is used, and the asserted
//! equalities are EXACT (`==`) — this relies on the committed
//! fixtures' coordinates being dyadic rationals (0, 1, 0.25, 0.5,
//! 0.75, 1.5, 2.0), for which every product and sum below is
//! representable without rounding. General inputs would need a
//! tolerance (or rational arithmetic); the fixtures are chosen so
//! they do not.
//!
//! `ADVANCED_FACE`'s own same_sense flag deliberately does not enter
//! the computation: the signed volume derives entirely from boundary
//! winding, which is the load-bearing orientation datum here (the
//! surface-normal side of the mapping is documented in lib.rs and
//! covered by the writer's fixed `.T.` emission).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::HashMap;

/// One parsed entity instance: keyword + raw argument text.
struct Record {
    keyword: String,
    args: String,
}

/// A minimal Part 21 line parser for OUR writer's output shape (one
/// `#id = KEYWORD(args);` record per line; complex instances get an
/// empty keyword and are ignored by the volume walk).
fn parse(text: &str) -> HashMap<u64, Record> {
    let mut records = HashMap::new();
    for line in text.lines() {
        let Some(rest) = line.strip_prefix('#') else {
            continue;
        };
        let Some((id, body)) = rest.split_once(" = ") else {
            continue;
        };
        let id: u64 = id.parse().expect("entity id");
        let body = body
            .trim_end()
            .strip_suffix(';')
            .expect("record ends with ;");
        let (keyword, args) = match body.split_once('(') {
            Some((kw, rest)) => (
                kw.trim().to_owned(),
                rest.strip_suffix(')')
                    .expect("record closes paren")
                    .to_owned(),
            ),
            None => (String::new(), body.to_owned()),
        };
        records.insert(id, Record { keyword, args });
    }
    records
}

/// All `#id` references in argument text, in order.
fn refs(args: &str) -> Vec<u64> {
    let mut out = Vec::new();
    let bytes = args.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'#' {
            let start = i + 1;
            let mut end = start;
            while end < bytes.len() && bytes[end].is_ascii_digit() {
                end += 1;
            }
            if end > start {
                out.push(args[start..end].parse().expect("entity ref"));
            }
            i = end;
        } else {
            i += 1;
        }
    }
    out
}

/// The trailing boolean flag of a record (`.T.` / `.F.`).
fn last_flag(args: &str) -> bool {
    let t = args.rfind(".T.");
    let f = args.rfind(".F.");
    match (t, f) {
        (Some(t), Some(f)) => t > f,
        (Some(_), None) => true,
        (None, Some(_)) => false,
        (None, None) => panic!("record has no boolean flag: {args}"),
    }
}

/// The three coordinates of a CARTESIAN_POINT record.
fn coords(rec: &Record) -> [f64; 3] {
    assert_eq!(rec.keyword, "CARTESIAN_POINT");
    let open = rec.args.find('(').expect("coordinate list");
    let close = rec.args.rfind(')').expect("coordinate list closes");
    let nums: Vec<f64> = rec.args[open + 1..close]
        .split(',')
        .map(|s| s.trim().parse().expect("coordinate"))
        .collect();
    assert_eq!(nums.len(), 3, "three coordinates");
    [nums[0], nums[1], nums[2]]
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// The position of a VERTEX_POINT's underlying CARTESIAN_POINT.
fn vertex_position(records: &HashMap<u64, Record>, vp: u64) -> [f64; 3] {
    let rec = &records[&vp];
    assert_eq!(rec.keyword, "VERTEX_POINT");
    coords(&records[&refs(&rec.args)[0]])
}

/// Signed volume of every CLOSED_SHELL in the file, in id order,
/// computed purely from boundary data: per planar face, pick a
/// boundary point `r`, accumulate `Σ (a−r)×(b−r)` over its directed
/// boundary segments (2·A⃗ by Stokes), and add `r·A⃗/3` (divergence
/// theorem with `p·n̂` constant on a plane).
fn shell_signed_volumes(text: &str) -> Vec<f64> {
    let records = parse(text);
    let mut shell_ids: Vec<u64> = records
        .iter()
        .filter(|(_, r)| r.keyword == "CLOSED_SHELL")
        .map(|(&id, _)| id)
        .collect();
    shell_ids.sort_unstable();
    shell_ids
        .iter()
        .map(|shell_id| {
            let mut six_v = 0.0_f64;
            for face_id in refs(&records[shell_id].args) {
                let face = &records[&face_id];
                assert_eq!(face.keyword, "ADVANCED_FACE");
                let face_refs = refs(&face.args);
                // Bounds are the parenthesized list; the surface is
                // the final reference.
                let (bounds, _surface) = face_refs.split_at(face_refs.len() - 1);
                let mut segments: Vec<([f64; 3], [f64; 3])> = Vec::new();
                for &bound_id in bounds {
                    let bound = &records[&bound_id];
                    assert!(
                        bound.keyword == "FACE_OUTER_BOUND" || bound.keyword == "FACE_BOUND",
                        "face bound keyword: {}",
                        bound.keyword
                    );
                    let bound_sense = last_flag(&bound.args);
                    let edge_loop = &records[&refs(&bound.args)[0]];
                    assert_eq!(edge_loop.keyword, "EDGE_LOOP");
                    for oe_id in refs(&edge_loop.args) {
                        let oe = &records[&oe_id];
                        assert_eq!(oe.keyword, "ORIENTED_EDGE");
                        let ec = &records[&refs(&oe.args)[0]];
                        assert_eq!(ec.keyword, "EDGE_CURVE");
                        let ec_refs = refs(&ec.args);
                        let mut a = vertex_position(&records, ec_refs[0]);
                        let mut b = vertex_position(&records, ec_refs[1]);
                        // Honor every orientation datum in the file:
                        // EDGE_CURVE same_sense, the ORIENTED_EDGE
                        // flag, and the bound's orientation.
                        if !last_flag(&ec.args) {
                            std::mem::swap(&mut a, &mut b);
                        }
                        if !last_flag(&oe.args) {
                            std::mem::swap(&mut a, &mut b);
                        }
                        if !bound_sense {
                            std::mem::swap(&mut a, &mut b);
                        }
                        segments.push((a, b));
                    }
                }
                let r = segments.first().expect("face has boundary segments").0;
                let mut area2 = [0.0_f64; 3];
                for &(a, b) in &segments {
                    let c = cross(sub(a, r), sub(b, r));
                    area2 = [area2[0] + c[0], area2[1] + c[1], area2[2] + c[2]];
                }
                six_v += dot(r, area2);
            }
            six_v / 6.0
        })
        .collect()
}

fn fixture(name: &str) -> String {
    let path = format!("{}/tests/fixtures/{name}.step", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("fixture {path} unreadable ({e})"))
}

#[test]
fn committed_fixtures_have_exact_positive_shell_volumes() {
    // Exact equalities: dyadic fixtures (module docs).
    assert_eq!(shell_signed_volumes(&fixture("cube")), vec![1.0]);
    assert_eq!(shell_signed_volumes(&fixture("die")), vec![0.875]);
    assert_eq!(
        shell_signed_volumes(&fixture("kiss_assembly")),
        vec![0.875, 0.875]
    );
}

#[test]
fn negative_control_flipped_winding_reads_negative() {
    // Prove the oracle would catch an inversion the external tools
    // silently heal: flip every ORIENTED_EDGE flag (reversing each
    // loop's winding) and the same computation must report exactly
    // the negated volume.
    let flipped: String = fixture("cube")
        .lines()
        .map(|line| {
            let mut line = line.to_owned();
            if line.contains("= ORIENTED_EDGE(") {
                line = if line.contains(".T.") {
                    line.replace(".T.", ".F.")
                } else {
                    line.replace(".F.", ".T.")
                };
            }
            line + "\n"
        })
        .collect();
    assert_eq!(shell_signed_volumes(&flipped), vec![-1.0]);
}
