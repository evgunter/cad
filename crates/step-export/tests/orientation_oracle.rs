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
//! winding, which is the load-bearing orientation datum here.
//!
//! That is a scoping decision, and since M5 S10 it is also the only
//! honest phrasing. The writer no longer emits a FIXED `.T.`:
//! same_sense is `topo::Face::sense` verbatim, while the surface's
//! `AXIS2_PLACEMENT_3D` carries the unmodified CHART normal. The two
//! together are STEP's encoding of "which way is out" — a
//! sense-reversed face keeps its chart axis and flips the flag rather
//! than negating the direction. So the signed-volume oracle covers the
//! winding half of the mapping and says so; the flag half is pinned
//! where it is actually discriminating, by the S10/S11 acceptance rows
//! and by `m5_pr13_curved.rs`.
//!
//! Since M5 PR 13, reversed faces DO reach real output: the notched
//! prism's concave wall, the washer's bore and under-side annulus, and
//! the cone's two BASE-DISC halves — which are `PLANE` faces, not the
//! conical bands (both `CONICAL_SURFACE` faces of the cone write
//! `.T.`). That is the general shape of S11's rule: the reversed face
//! is the one whose material lies against the chart normal, and on a
//! revolved solid the under-side cap is exactly such a face even
//! though it is flat. The PLANAR fixtures pinned below are unchanged —
//! no planar-only body at rest mints a reversed face — and the curved
//! fixtures get the curved-agnostic oracle appended at the bottom of
//! this file.
//!
//! **FreeCAD cannot arbitrate the flag, measured not assumed.** The M4
//! finding (flipping all six `ADVANCED_FACE` flags in `cube.step` and
//! still getting `valid=True` with positive volume) was re-run at PR 13
//! on curved geometry: `revert(ball)` and `revert(washer)` — genuinely
//! inside-out solids, every face `same_sense = .F.` — import as
//! `valid: True` with the SAME positive volumes as the un-reverted
//! bodies (4188790204.79 and 9424777960.77 mm³). OCC's ShapeHealing
//! rectifies the shell silently. So the reversed-face acceptance is
//! text-level here by necessity, exactly as the plan's fallback
//! anticipated, and the double-composition negative control below is
//! the row that would otherwise have been FreeCAD's job.

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

// ==================================================================
// M5 PR 13: the curved fixtures — an orientation oracle that does not
// need planarity
// ==================================================================
//
// The signed-volume oracle above reduces each face's surface integral
// to a boundary sum using `p·n̂ = const on a plane`. That identity has
// no curved counterpart in closed form, so it cannot be pointed at
// `ball.step` or `donut.step`. What CAN be checked on any shell,
// exactly and from the emitted text alone, is **edge-use coherence**:
// in a coherently oriented closed shell every edge is traversed once in
// each direction, counting the whole shell's loops. That is the same
// orientation datum the volume oracle reads (the boundary winding),
// stated locally instead of globally — and it is precisely what a
// DOUBLE-COMPOSED face breaks, because composing the reversal twice
// flips one face's loops relative to all of its neighbours.
//
// Two negative controls below prove the check has teeth, one of them
// the double-composition bug itself.

/// The composed spatial traversal direction of one `ORIENTED_EDGE`
/// inside one bound: the `ORIENTED_EDGE` flag, reversed if the bound's
/// own orientation is `.F.`.
///
/// `EDGE_CURVE`'s `same_sense` deliberately does not enter: it relates
/// the CURVE's parameterization to the edge's vertex order, not the
/// loop's direction along the edge. `ADVANCED_FACE`'s `same_sense`
/// does not enter either — that is the whole point. It says which side
/// of the surface is material, and the writer must state the reversal
/// there and ONLY there; a writer that also flipped the bound would
/// show up here as an incoherent shell.
fn composed_direction(bound_orientation: bool, oriented_edge_flag: bool) -> bool {
    oriented_edge_flag == bound_orientation
}

/// `Ok(edge uses)` per `CLOSED_SHELL` in id order, or `Err(reason)` on
/// the first incoherence: every `EDGE_CURVE` referenced by a shell must
/// be used exactly twice, once in each composed direction.
fn edge_use_coherence(text: &str) -> Result<Vec<usize>, String> {
    let records = parse(text);
    let mut shell_ids: Vec<u64> = records
        .iter()
        .filter(|(_, r)| r.keyword == "CLOSED_SHELL")
        .map(|(&id, _)| id)
        .collect();
    shell_ids.sort_unstable();
    let mut per_shell = Vec::new();
    for shell_id in shell_ids {
        // edge id → (forward uses, backward uses)
        let mut uses: HashMap<u64, (usize, usize)> = HashMap::new();
        for face_id in refs(&records[&shell_id].args) {
            let face = &records[&face_id];
            assert_eq!(face.keyword, "ADVANCED_FACE");
            let face_refs = refs(&face.args);
            let (bounds, _surface) = face_refs.split_at(face_refs.len() - 1);
            for &bound_id in bounds {
                let bound = &records[&bound_id];
                let bound_orientation = last_flag(&bound.args);
                let edge_loop = &records[&refs(&bound.args)[0]];
                assert_eq!(edge_loop.keyword, "EDGE_LOOP");
                for oe_id in refs(&edge_loop.args) {
                    let oe = &records[&oe_id];
                    assert_eq!(oe.keyword, "ORIENTED_EDGE");
                    let edge = refs(&oe.args)[0];
                    assert_eq!(records[&edge].keyword, "EDGE_CURVE");
                    let slot = uses.entry(edge).or_insert((0, 0));
                    if composed_direction(bound_orientation, last_flag(&oe.args)) {
                        slot.0 += 1;
                    } else {
                        slot.1 += 1;
                    }
                }
            }
        }
        for (edge, (forward, backward)) in &uses {
            if (*forward, *backward) != (1, 1) {
                return Err(format!(
                    "shell #{shell_id}: edge #{edge} used {forward} forward / \
                     {backward} backward (a coherent closed shell uses each edge \
                     exactly once each way)"
                ));
            }
        }
        per_shell.push(uses.len());
    }
    Ok(per_shell)
}

/// Every committed fixture — planar AND curved, including the ones
/// with `same_sense = .F.` faces and the ones whose full-2π faces use
/// a seam edge twice within a single loop — is edge-use coherent.
///
/// The per-shell edge counts are pinned as well, so the row also says
/// how much geometry it walked (and would catch a shell silently
/// losing a face).
#[test]
fn every_fixture_shell_is_edge_use_coherent() {
    let expect: &[(&str, &[usize])] = &[
        ("cube", &[12]),
        ("die", &[24]),
        ("kiss_assembly", &[24, 24]),
        ("cut_cylinder", &[6]),
        ("boss_union", &[21]),
        ("notched", &[12]),
        // The washer's two full-2π walls each use their seam edge
        // twice inside ONE loop — the coherence rule is blind to which
        // loop the two uses come from, which is exactly right.
        ("washer", &[8]),
        ("ball", &[2]),
        ("cone", &[6]),
        ("donut", &[4]),
    ];
    for (name, counts) in expect {
        let text = fixture(name);
        match edge_use_coherence(&text) {
            Ok(seen) => assert_eq!(&seen, counts, "{name}: per-shell edge counts"),
            Err(why) => panic!("{name} is not edge-use coherent: {why}"),
        }
    }
}

/// **Negative control 1: the double-composition bug.** Take the
/// fixture with a reversed face (`notched.step`, whose concave
/// cylinder wall writes `same_sense = .F.`) and ALSO flip that face's
/// bound orientation — the "be consistent, flip it everywhere" mistake
/// the writer docs warn about. ISO 10303-42 composes the two, so the
/// face's loops now run backwards relative to every neighbour, and the
/// coherence check must catch it.
///
/// This is the row that makes the `.T.` bound flag load-bearing rather
/// than incidental.
#[test]
fn negative_control_double_composed_face_is_incoherent() {
    let text = fixture("notched");
    // Locate the one `.F.` ADVANCED_FACE and its bound reference.
    let reversed = text
        .lines()
        .find(|l| l.contains("= ADVANCED_FACE(") && l.contains(".F."))
        .expect("notched has a reversed face");
    let bound_id: u64 = refs(reversed.split_once(" = ").expect("record").1)[0];
    let doubled: String = text
        .lines()
        .map(|line| {
            if line.starts_with(&format!("#{bound_id} = FACE_OUTER_BOUND(")) {
                line.replace(".T.", ".F.") + "\n"
            } else {
                line.to_owned() + "\n"
            }
        })
        .collect();
    assert_ne!(doubled, text, "the control actually changed a flag");
    let why = edge_use_coherence(&doubled).expect_err("double composition must be caught");
    assert!(why.contains("used"), "{why}");
}

/// **Negative control 2: a single inverted face.** Flip every
/// `ORIENTED_EDGE` flag of one face of the ball — the curved analogue
/// of the planar whole-shell inversion control above, but scoped to
/// ONE face so the result is not merely a globally reversed (and
/// therefore still coherent) shell.
#[test]
fn negative_control_one_inverted_curved_face_is_incoherent() {
    let text = fixture("ball");
    // The ball's second face is the one whose EDGE_LOOP comes last.
    let loop_line = text
        .lines()
        .rfind(|l| l.contains("= EDGE_LOOP("))
        .expect("two edge loops");
    let oriented: Vec<u64> = refs(loop_line.split_once(" = ").expect("record").1);
    let flipped: String = text
        .lines()
        .map(|line| {
            let is_target = oriented
                .iter()
                .any(|id| line.starts_with(&format!("#{id} = ORIENTED_EDGE(")));
            if is_target {
                if line.contains(".T.") {
                    line.replace(".T.", ".F.") + "\n"
                } else {
                    line.replace(".F.", ".T.") + "\n"
                }
            } else {
                line.to_owned() + "\n"
            }
        })
        .collect();
    assert_ne!(flipped, text, "the control actually changed flags");
    let why = edge_use_coherence(&flipped).expect_err("an inverted face must be caught");
    assert!(why.contains("used"), "{why}");
}

/// The complement of the two controls: a WHOLE shell reversed (every
/// oriented edge flipped) stays coherent, because coherence is a
/// relative property. Said out loud so the oracle's scope is not
/// overclaimed — it catches a face inconsistent with its neighbours,
/// not a uniformly inside-out solid. That case is covered by the
/// signed-volume oracle on the planar fixtures and by FreeCAD's volume
/// on the curved ones.
#[test]
fn a_uniformly_reversed_shell_stays_coherent_by_design() {
    let flipped: String = fixture("donut")
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
    assert!(edge_use_coherence(&flipped).is_ok());
}

/// **Known blind spot, stated rather than discovered later (review
/// probe A3).** Corrupt `notched.step` by flipping ONLY the
/// `ADVANCED_FACE` `same_sense` flags — leaving every winding, every
/// `ORIENTED_EDGE` and every bound orientation untouched — and this
/// oracle reports the file GREEN. That is correct and not a defect:
/// edge-use coherence is a statement about traversal directions, and
/// the corruption changes none of them. It is recorded here because
/// the same corruption is ALSO invisible to FreeCAD — measured, not
/// assumed: the flipped file imports as `valid: True`, 6 faces / 12
/// edges / 8 vertices, volume 3000000000.0 mm³, every figure identical
/// to the honest fixture's. So between the two oracles this particular
/// lie has no external witness at all.
///
/// What does catch it is the kernel-side identity in
/// `m5_pr13_curved.rs`: `same_sense` IS `topo::Face::sense`, asserted
/// per face against the body the file came from. That is the ONLY
/// guard on this axis, which is exactly why it is asserted per face
/// rather than by counting `.F.`s.
#[test]
fn known_blind_spot_same_sense_only_corruption_reads_green() {
    let text = fixture("notched");
    let flipped: String = text
        .lines()
        .map(|line| {
            if line.contains("= ADVANCED_FACE(") {
                // Only the trailing same_sense flag: the bound list
                // references are untouched, so no winding moves.
                if line.ends_with(".F.);") {
                    line.trim_end_matches(".F.);").to_owned() + ".T.);\n"
                } else {
                    line.trim_end_matches(".T.);").to_owned() + ".F.);\n"
                }
            } else {
                line.to_owned() + "\n"
            }
        })
        .collect();
    assert_ne!(flipped, text, "the corruption actually changed flags");
    // Every flag really did move (6 faces, 1 was .F. and is now .T.).
    assert_eq!(
        flipped
            .lines()
            .filter(|l| l.contains("= ADVANCED_FACE(") && l.ends_with(".F.);"))
            .count(),
        5,
        "five faces flipped to .F., the sixth to .T."
    );
    assert!(
        edge_use_coherence(&flipped).is_ok(),
        "the winding half is genuinely untouched — this row documents the gap"
    );
}
