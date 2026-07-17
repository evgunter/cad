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
