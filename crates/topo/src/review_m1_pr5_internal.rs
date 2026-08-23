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
use geom_core::Tol;

/// Pass 12, all seven arenas, MISSING direction: remove each kind's
/// record from a clean cube; expect exactly one MissingProvenance
/// naming that entity.
#[test]
fn missing_provenance_all_seven_arenas() {
    let tol = Tol::witness();
    let t = ops_cube(tol);

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
    let tol = Tol::witness();
    let t = ops_cube(tol);
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
    let tol = Tol::witness();
    let t = ops_cube(tol);
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
    let tol = Tol::witness();
    let mut t = pillow(tol);
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

/// `(door, why tier 1 survives it)` — the doors that do NOT declare
/// the tier-1 postcondition, each with the reason tier 1 survives it.
///
/// Module-scoped rather than local to the guard so that
/// [`the_two_door_tables_cover_the_same_surface`] can read it. That
/// row is the only other reader; this table stays this guard's.
pub(crate) const ALLOWED: &[(&str, &str)] = &[
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
///
/// **Where the door set comes from, and what it cannot see:**
/// [`crate::source_walk::mutation_doors`], shared with the
/// pcurve-posture guard in [`crate::pcurves`], which walks the same
/// population to ask a different question. That function's docs carry
/// the reason the two tables do not merge and the whole inherited
/// blind-spot list; this guard does not restate either.
///
/// **"Declares the postcondition" is a read of code, not of prose.**
/// The needle is `assert_euler_postcondition(`, with the paren, over a
/// body whose comments and literals are blanked — a bare name would be
/// satisfied by a `use` line. This guard used a raw `body.contains`,
/// and a planted door whose body only *mentioned* the call in a
/// comment was counted as asserting it, in both this guard and the
/// pcurve one, both green.

#[test]
fn every_public_mutation_path_preserves_tier1() {
    let mut asserting: Vec<String> = Vec::new();
    let mut listed: Vec<String> = Vec::new();
    let mut unlisted: Vec<String> = Vec::new();

    for door in crate::source_walk::mutation_doors() {
        if door.code_contains("assert_euler_postcondition(") {
            asserting.push(door.site());
        } else if ALLOWED.iter().any(|(n, _)| *n == door.name) {
            listed.push(door.name);
        } else {
            unlisted.push(door.site());
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
    // The walk's own floor is upstream, on `mutation_doors`. What is
    // this guard's is the needle: a lexing gap that erased the call
    // from a body would move that door from `asserting` to `unlisted`
    // and red loudly — except for a door that is ALSO allowlisted,
    // which cannot happen, and for the case where every door loses it
    // at once, which the pin below catches by name.
    assert!(
        asserting.iter().any(|s| s.ends_with("::mev")),
        "`mev` no longer reads as declaring the tier-1 postcondition. Either the operator \
         stopped asserting — a finding — or the source read lost the call.",
    );
    println!(
        "[mutation surface] {} public door(s): {} assert tier 1, {} allowlisted",
        asserting.len() + listed.len(),
        asserting.len(),
        listed.len(),
    );
}

/// **The co-domain claim under "two properties of one set", asserted
/// rather than assumed.**
///
/// [`crate::source_walk::mutation_doors`] argues that [`ALLOWED`] and
/// [`crate::pcurves::staleness_posture::DECLARED`] should stay
/// separate because they classify one population two ways. That
/// argument is only worth its line while the two tables together
/// still *cover* that population, and until this row nothing said so.
///
/// **The case it catches, which neither guard does.** A door that both
/// declares the tier-1 postcondition and re-mints the pcurve map is
/// classified by the walk on both sides and appears in **neither**
/// table — so it is a door with no prose anywhere, and both guards
/// stay green. Nothing in the crate forbids such a door; a compound
/// operator is the natural way to get one.
///
/// Deliberately not a merge of the tables: this reads both and asserts
/// one property of the pair. It lives here, in a test artifact, so the
/// import runs from a review module to a kernel one and not the
/// reverse.
#[test]
fn the_two_door_tables_cover_the_same_surface() {
    let doors = crate::source_walk::mutation_doors();
    let uncovered: Vec<String> = doors
        .iter()
        .filter(|d| {
            !ALLOWED.iter().any(|(n, _)| *n == d.name)
                && !crate::pcurves::staleness_posture::DECLARED
                    .iter()
                    .any(|(n, _, _)| *n == d.name)
        })
        .map(|d| d.site())
        .collect();
    assert!(
        uncovered.is_empty(),
        "mutation door(s) named by NEITHER table: {uncovered:?}. Each is a door the \
         tier-1 guard sorted by its own body and the pcurve guard sorted by its own \
         body, so both pass and no entry anywhere describes it. Give it an entry in \
         whichever table its posture is not self-evident from.",
    );
}
