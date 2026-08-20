//! Adversarial e2e review artifact for M1 PR 5 (2026-07-16) — the
//! **in-crate** half, promoted per the standing convention alongside
//! the public-API half (`tests/review_m1_pr5.rs`). These probes need
//! pub(crate) access (raw corruption, provenance-map surgery), which is
//! exactly why they live in `src/` under cfg(test).
//!
//! Coverage: pass 12 across **all seven arenas in both directions**
//! (missing records exact-vector, leaked records present-in-report —
//! completing the shipped suite's 2/14 to 14/14), pass 10/11 interplay
//! at scale (a cube shredded across three shells: termination,
//! determinism, coherent per-shell reporting), and the tier-2
//! strut-scan echo pin (a dangling `start` deflates a derived valence —
//! the documented aggregate-scan echo in `validate`'s cascade docs;
//! unreachable through the public API).
//!
//! Promoted verbatim except this header.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::entity::EntityId;
use crate::fixtures::{ops_cube, pillow, prov};
use crate::validate::{ValidationError, validate, validate_closed};

/// Pass 12, all seven arenas, MISSING direction: remove each kind's
/// record from a clean cube; expect exactly one MissingProvenance
/// naming that entity.
#[test]
fn missing_provenance_all_seven_arenas() {
    let t = ops_cube();

    // Solids.
    let mut b = t.body.clone();
    let k = b.solids.keys().next().unwrap();
    b.solid_provenance.remove(k);
    assert_eq!(
        validate(&b),
        Err(vec![ValidationError::MissingProvenance {
            entity: EntityId::Solid(k)
        }])
    );

    let mut b = t.body.clone();
    let k = b.shells.keys().next().unwrap();
    b.shell_provenance.remove(k);
    assert_eq!(
        validate(&b),
        Err(vec![ValidationError::MissingProvenance {
            entity: EntityId::Shell(k)
        }])
    );

    let mut b = t.body.clone();
    let k = b.faces.keys().next().unwrap();
    b.face_provenance.remove(k);
    assert_eq!(
        validate(&b),
        Err(vec![ValidationError::MissingProvenance {
            entity: EntityId::Face(k)
        }])
    );

    let mut b = t.body.clone();
    let k = b.loops.keys().next().unwrap();
    b.loop_provenance.remove(k);
    assert_eq!(
        validate(&b),
        Err(vec![ValidationError::MissingProvenance {
            entity: EntityId::Loop(k)
        }])
    );

    let mut b = t.body.clone();
    let k = b.half_edges.keys().next().unwrap();
    b.half_edge_provenance.remove(k);
    assert_eq!(
        validate(&b),
        Err(vec![ValidationError::MissingProvenance {
            entity: EntityId::HalfEdge(k)
        }])
    );

    let mut b = t.body.clone();
    let k = b.edges.keys().next().unwrap();
    b.edge_provenance.remove(k);
    assert_eq!(
        validate(&b),
        Err(vec![ValidationError::MissingProvenance {
            entity: EntityId::Edge(k)
        }])
    );

    let mut b = t.body.clone();
    let k = b.vertices.keys().next().unwrap();
    b.vertex_provenance.remove(k);
    assert_eq!(
        validate(&b),
        Err(vec![ValidationError::MissingProvenance {
            entity: EntityId::Vertex(k)
        }])
    );
}

/// Pass 12, LEAK direction for all seven arenas: kill each entity kind
/// raw (record left behind); the LeakedProvenance for that key must be
/// among the errors (dangling/orphan echoes from the raw removal are
/// expected and NOT gated away — provenance is pass 12, ungated).
#[test]
fn leaked_provenance_all_seven_arenas() {
    let t = ops_cube();
    macro_rules! leak_probe {
        ($arena:ident, $variant:ident) => {{
            let mut b = t.body.clone();
            let k = b.$arena.keys().next().unwrap();
            b.$arena.remove(k);
            let errs = validate(&b).unwrap_err();
            assert!(
                errs.contains(&ValidationError::LeakedProvenance {
                    entity: EntityId::$variant(k)
                }),
                "no {} leak reported: {errs:?}",
                stringify!($variant)
            );
        }};
    }
    leak_probe!(solids, Solid);
    leak_probe!(shells, Shell);
    leak_probe!(faces, Face);
    leak_probe!(loops, Loop);
    leak_probe!(half_edges, HalfEdge);
    leak_probe!(edges, Edge);
    leak_probe!(vertices, Vertex);
}

/// Pass 10 + 11 at scale: an ops cube shredded across three shells
/// (faces round-robined). Pass 10 must fire for every edge whose halves
/// land in different shells; pass 11 must run (not hang, not skip),
/// reporting per-shell component violations; the report must be finite
/// and deterministic across repeated validation.
#[test]
fn cross_shell_shredding_terminates_and_reports_coherently() {
    let t = ops_cube();
    let mut b = t.body;
    let faces: Vec<_> = b.faces.keys().collect();
    let solid = b.solids.keys().next().unwrap();
    let shell0 = b.shells.keys().next().unwrap();
    let shell1 = b.add_shell(
        crate::entity::Shell {
            faces: vec![],
            solid,
        },
        prov(),
    );
    let shell2 = b.add_shell(
        crate::entity::Shell {
            faces: vec![],
            solid,
        },
        prov(),
    );
    b.get_solid_mut(solid).unwrap().shells.push(shell1);
    b.get_solid_mut(solid).unwrap().shells.push(shell2);
    let shells = [shell0, shell1, shell2];
    // Round-robin the six faces across the three shells.
    for (i, &f) in faces.iter().enumerate() {
        let target = shells[i % 3];
        if target == shell0 {
            continue;
        }
        b.get_shell_mut(shell0).unwrap().faces.retain(|&x| x != f);
        b.get_shell_mut(target).unwrap().faces.push(f);
        b.get_face_mut(f).unwrap().shell = target;
    }
    let errs1 = validate(&b).unwrap_err();
    let errs2 = validate(&b).unwrap_err();
    assert_eq!(errs1, errs2, "deterministic report");
    let crossings = errs1
        .iter()
        .filter(|e| matches!(e, ValidationError::EdgeAcrossShells { .. }))
        .count();
    // Every cube edge separates two adjacent faces; adjacent faces
    // land in the same shell only when i % 3 collides. Just sanity:
    // plenty of crossings, and pass 11 ran (some component violations).
    assert!(crossings >= 8, "expected many crossings, got {crossings}");
    let violations = errs1
        .iter()
        .filter(|e| matches!(e, ValidationError::ComponentEulerViolation { .. }))
        .count();
    assert!(violations >= 1, "pass 11 ran through the corruption");
    // Tier 2 shares the enumeration and must also terminate.
    let _ = validate_closed(&b).unwrap_err();
}

/// Tier-2 echo probe: a dangling `start` reference (pass-1 error)
/// deflates a vertex's derived valence to 1, so validate_closed emits a
/// ScaffoldingStrutVertex ECHO for a vertex that is not a strut. This
/// documents actual behavior (the tier-2 scans are ungated by design);
/// if it ever changes, re-check the module docs' cascade wording.
#[test]
fn tier2_strut_scan_echoes_on_dangling_start() {
    let mut t = pillow();
    // Mint a dead vertex key.
    let dead = t.body.add_vertex(
        crate::entity::Vertex {
            point: t.points[0],
            emanating: None,
        },
        prov(),
    );
    t.body.vertices.remove(dead);
    t.body.vertex_provenance.remove(dead);
    // b1 starts at v0; repoint its start at the dead key. v0's real
    // incidence drops to 1 in the derived count.
    t.body.get_half_edge_mut(t.hes_b[1]).unwrap().start = dead;
    let errs = validate_closed(&t.body).unwrap_err();
    let has_strut_echo = errs
        .iter()
        .any(|e| matches!(e, ValidationError::ScaffoldingStrutVertex { .. }));
    // Record the actual behavior either way; the assert documents it.
    assert!(
        has_strut_echo,
        "expected the ungated tier-2 strut scan to echo: {errs:?}"
    );
}

/// **The closure property the module docs of [`crate::euler`] and D9's
/// footnote both rest on, checked instead of asserted.** Every public
/// mutation path into a [`crate::Body`] — `pub fn` taking `&mut self`,
/// plus the free functions taking `&mut Body<T>` — either declares the
/// shared tier-1 debug postcondition (`assert_euler_postcondition`) or
/// appears below with the reason it does not need one.
///
/// **Why a test and not a sentence.** The claim these documents make
/// used to be an enumeration ("the eleven public mutators"), and it
/// rotted: five more doors landed and the count stayed. Replacing the
/// count with prose about a closure property fixes the arithmetic and
/// keeps the failure mode — the next door added is still a door
/// nothing checks. This goes red the day one lands, which is the whole
/// point, and it is why neither document carries a number.
///
/// **What the allowlist is, and what it is not.** Not a waiver list.
/// Each entry states why tier 1 survives that door, and the entries
/// divide into four kinds: sugar delegating to an asserting operator;
/// pipelines composed of asserting operators; setters carrying their
/// own tier-1 `debug_assert`; and setters writing fields tier 1 does
/// not constrain. The fifth kind has exactly one member and is the
/// finding that produced this test — `instance`'s grafts do NOT
/// preserve tier 1 on their failure path, which their own docs
/// concede, and which is open as S14.
///
/// Stale entries are caught in both directions: an entry naming a door
/// that no longer exists, or one that has since started asserting,
/// fails as loudly as an unlisted door.
#[test]
fn every_public_mutation_path_preserves_tier1() {
    // `(door, why tier 1 survives it)`.
    const ALLOWED: &[(&str, &str)] = &[
        // ---- Sugar: delegates to an asserting operator. ----
        ("mev_line", "derives the spec, then calls `mev`"),
        ("mef_chord", "derives the spec, then calls `mef`"),
        ("mekr_chord", "derives the spec, then calls `mekr`"),
        ("mfkrh_plug", "calls `mfkrh` with a placeholder surface"),
        (
            "set_edge_curve_nurbs_lane",
            "`set_edge_curve` with the NURBS certifier injected — same body, same assertion",
        ),
        // ---- Pipelines composed of asserting operators. ----
        (
            "merge_coplanar_faces",
            "calls `merge_coplanar_faces_declared` with no declarations",
        ),
        (
            "merge_coplanar_faces_declared",
            "gates on `validate_closed` at entry and mutates only through `ring_move`/`kef`",
        ),
        // ---- Setters carrying their own tier-1 debug_assert. ----
        (
            "set_face_surface",
            "asserts tier 1 directly (a surface swap can orphan a key)",
        ),
        (
            "set_edge_curve",
            "asserts tier 1 directly (a curve swap can orphan a key)",
        ),
        // ---- Writes fields tier 1 does not constrain. ----
        ("set_face_sense", "writes one `bool`; sense is tier 3's"),
        ("set_surface_source", "GeomSource metadata, no arena key"),
        ("set_curve_source", "GeomSource metadata, no arena key"),
        ("set_point_source", "GeomSource metadata, no arena key"),
        ("clear_geom_sources", "GeomSource metadata, no arena key"),
        ("attach_pcurve", "pcurve cache; coherence is tier 3's"),
        ("detach_pcurve", "pcurve cache; coherence is tier 3's"),
        ("mint_pcurves", "pcurve caches only; no topology touched"),
        (
            "set_null_face_pair",
            "null-face annotation; tier 2 bans it at rest, tier 1 does not see it",
        ),
        ("clear_null_face_pair", "removes that annotation"),
        // ---- The exception. Not a waiver: a recorded hole. ----
        (
            "graft_disjoint",
            "RAW TRANSPLANT — see `graft_disjoint_all_keyed`",
        ),
        (
            "graft_disjoint_all",
            "RAW TRANSPLANT — see `graft_disjoint_all_keyed`",
        ),
        (
            "graft_disjoint_all_keyed",
            "RAW TRANSPLANT, and the one door that does NOT preserve tier 1: it mints an \
             empty destination solid per source solid before transplanting, and a refusal \
             raised mid-transplant leaves `dst` partially written (its own docs: spent, \
             never resumable). An empty solid IS `SolidWithoutShells`, a tier-1 error. A \
             caller that discards the `Err` can fire a later operator's postcondition from \
             API MISUSE rather than a kernel bug — the state class D9's footnote says \
             cannot occur. Open as S14; this entry records it, it does not excuse it.",
        ),
        (
            "graft_disjoint_all_onto_keyed",
            "RAW TRANSPLANT — see `graft_disjoint_all_keyed`",
        ),
    ];

    let files = crate::fixtures::crate_sources();
    let mut asserting: Vec<String> = Vec::new();
    let mut listed: Vec<String> = Vec::new();
    let mut unlisted: Vec<String> = Vec::new();

    for path in &files {
        let text = std::fs::read_to_string(path).expect("a readable source file");
        for (name, params, body) in public_fns(&text) {
            if !params.contains("&mut self") && !params.contains("&mut Body") {
                continue;
            }
            let where_ = format!("{}::{name}", path.display());
            if body.contains("assert_euler_postcondition") {
                asserting.push(where_);
            } else if ALLOWED.iter().any(|(n, _)| *n == name) {
                listed.push(name.to_string());
            } else {
                unlisted.push(where_);
            }
        }
    }

    assert!(
        unlisted.is_empty(),
        "public mutation path(s) that neither declare the tier-1 debug postcondition nor \
         appear on this test's allowlist: {unlisted:?}. Either call \
         `assert_euler_postcondition` at the end of the door, or add it above WITH the \
         reason tier 1 survives it — and if the reason is that it does not, that is a \
         finding, not an entry.",
    );
    // The allowlist rots in the other direction too.
    for (name, _) in ALLOWED {
        assert!(
            listed.iter().any(|n| n == name),
            "the allowlist names `{name}`, which is no longer a non-asserting public \
             mutation path — it was renamed, deleted, or has started asserting. Drop the \
             entry.",
        );
    }
    // A walk that found nothing would pass every assertion above.
    assert!(
        asserting.len() >= 10 && listed.len() >= 10,
        "the walk found {} asserting and {} allowlisted door(s) — it is not reading the \
         real surface",
        asserting.len(),
        listed.len(),
    );
    println!(
        "[mutation surface] {} public door(s): {} assert tier 1, {} allowlisted",
        asserting.len() + listed.len(),
        asserting.len(),
        listed.len(),
    );
}

/// Every `pub fn` in `text`, as `(name, parameter list, body)`.
///
/// Deliberately a source read: Rust offers no way to enumerate a
/// type's methods at runtime, and the property being checked is about
/// the SOURCE surface — what a future door will look like when someone
/// adds one.
fn public_fns(text: &str) -> Vec<(&str, &str, &str)> {
    let mut out = Vec::new();
    let bytes = text.as_bytes();
    let mut from = 0usize;
    while let Some(rel) = text[from..].find("pub fn ") {
        let at = from + rel;
        from = at + "pub fn ".len();
        // Item position only: a `pub fn` glued to another identifier is
        // not a definition.
        if at > 0 && !matches!(bytes[at - 1], b'\n' | b' ') {
            continue;
        }
        let rest = &text[from..];
        let Some(name_end) = rest.find(|c: char| !c.is_alphanumeric() && c != '_') else {
            continue;
        };
        let name = &rest[..name_end];
        let Some(open) = text[from..].find('(') else {
            continue;
        };
        let Some(close) = matching(text, from + open, b'(', b')') else {
            continue;
        };
        let params = &text[from + open..close];
        let Some(brace) = text[close..].find('{') else {
            continue;
        };
        let Some(end) = matching(text, close + brace, b'{', b'}') else {
            continue;
        };
        out.push((name, params, &text[close + brace..end]));
        from = end;
    }
    out
}

/// The index of the delimiter closing the one at `open`, skipping
/// string and comment content.
fn matching(text: &str, open: usize, l: u8, r: u8) -> Option<usize> {
    let b = text.as_bytes();
    let (mut depth, mut i) = (0usize, open);
    while i < b.len() {
        match b[i] {
            b'"' => {
                i += 1;
                while i < b.len() && b[i] != b'"' {
                    i += usize::from(b[i] == b'\\') + 1;
                }
            }
            b'/' if b.get(i + 1) == Some(&b'/') => {
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
            }
            b'/' if b.get(i + 1) == Some(&b'*') => {
                i += 2;
                while i + 1 < b.len() && !(b[i] == b'*' && b[i + 1] == b'/') {
                    i += 1;
                }
                i += 1;
            }
            c if c == l => depth += 1,
            c if c == r => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}
