//! **Reviewer probes for PR #1222 (BLEND-1, the multi-link closed-rim
//! door)** — bodies authored HERE, not the PR's fixtures, driven
//! through the public API as an outside consumer would.
//!
//! What each row goes red on:
//!
//! - **`the_lip_bands_removed_volume_matches_a_hand_quadrature`** — an
//!   INDEPENDENT closed form for the material a seam-split carve
//!   removes: Pappus over the exact corner region (kite minus circular
//!   sector), integrated by hand here. The PR's own volume oracle is a
//!   DIFFERENTIAL against the bored one-edge twin, so both sides of
//!   that comparison could share a defect; this row shares nothing
//!   with either. Red if the carve removes the wrong material even
//!   when the twin agrees.
//! - **`the_same_hand_quadrature_holds_for_the_one_edge_twin`** — the
//!   same hand form against the bored twin, so the differential's
//!   trusted side is measured too rather than assumed.
//! - **`a_strict_subset_of_a_seam_split_rims_arcs_refuses_typed`** —
//!   asking for ONE arc of a two-arc rim must still refuse at the seam
//!   vertex, not carve half a band.
//! - **`two_rims_sharing_a_wall_in_one_call_carve_both_bands`** — the
//!   lantern's neck and shoulder share the sphere wall; all four arcs
//!   in one request carve both bands (FLIPPED at #935 — the reviewer
//!   pinned the refusal this request met at BLEND-1's merge).
//! - **`arcs_of_two_different_rims_refuse_typed`** — a closed-looking
//!   request whose arcs are not one rim.
//! - **`a_seam_split_bands_birth_rows_key_uniquely`** — no two rows of
//!   one naming field key on the same source entity, which is what the
//!   PR's "every name stays unique" claim rests on.
//! - **`the_one_edge_annulus_fingerprint_is_stable`** — a canonical
//!   census + volume-bit fingerprint of a ONE-EDGE rim carve, for the
//!   merge-base differential (claim 1). Printed, and pinned to the
//!   values measured at the merge base.
//! - **`a_cylinder_capped_both_ends_carves_both_seam_split_rims`** — a
//!   second pole-touching body the PR never built, two rims, sequential
//!   calls.
//! - **`the_seam_vertex_recourse_names_a_door_that_answers`** — at a
//!   site where the tag actually fires, the request its recourse names
//!   is built and must succeed (the A3-2 correction's own standard).
//! - **`the_waisted_bodys_convex_rims_carve_so_its_concave_row_is_not_vacuous`**
//!   — the PR's concave fixture reaches this unit's door on its other
//!   rims, so its concave refusal is about convexity and not about the
//!   body.
//! - **`the_seam_vertex_recourse_is_true_at_every_site_the_tag_fires`**
//!   — the review's MAJOR, converted into its own regression pin: the
//!   tag fires at a CONCAVE rim's seam vertex as readily as a convex
//!   one, so the sentence must be true there too. The row now composes
//!   the recourse and the whole-rim answer on BOTH material sides, so
//!   neither half can drift alone.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, SQRT_2};

use geom_core::{Band, Point2, Tol};
use profile::ProfileVertex;
use sweep::Revolution;
use sweep::fillet::build::fillet_edges;
use sweep::fillet::{CornerConfig, FILLET3_SEAM_VERTEX_RECOURSE, FilletError};
use sweep::test_support::{revolved_about_y, rim_arcs_at};
use topo::{Body, EdgeKey, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::new(tol().eps(), tol().k() * tol().eps()).unwrap()
}

fn v(x: f64, y: f64, bulge: f64) -> ProfileVertex<f64> {
    ProfileVertex::new(Point2::new(x, y), bulge)
}

const SHOULDER: (f64, f64) = (0.8, 0.6);
const TOP: f64 = 1.2;
const LIP_R: f64 = 0.2;
const BORE: f64 = 0.1;

fn sphere_bulge() -> f64 {
    (0.6f64.asin() / 4.0).tan()
}

/// The PR's lantern, re-authored here from the same profile.
fn lantern() -> Body<f64> {
    revolved_about_y(
        vec![
            v(0.0, 0.0, 0.0),
            v(1.0, 0.0, sphere_bulge()),
            v(SHOULDER.0, SHOULDER.1, 0.0),
            v(LIP_R, TOP, 0.0),
            v(0.0, TOP, 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// The bored twin — every rim ONE closed edge.
fn bored_lantern() -> Body<f64> {
    revolved_about_y(
        vec![
            v(BORE, 0.0, 0.0),
            v(1.0, 0.0, sphere_bulge()),
            v(SHOULDER.0, SHOULDER.1, 0.0),
            v(LIP_R, TOP, 0.0),
            v(BORE, TOP, 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

fn volume(body: &Body<f64>) -> f64 {
    let props = mass_properties(body, tol()).expect("mass properties must compute");
    assert_eq!(props.volume_pad, 0.0, "closed-form inventory");
    props.volume
}

// ------------------------------------------------------------------
// An INDEPENDENT closed form for the removed material.
// ------------------------------------------------------------------

/// The volume a convex fillet of radius `r` removes at a corner of a
/// solid of revolution whose TWO supports are straight in the meridian
/// (here: the cone×plane lip).
///
/// The removed 2D region is bounded by the two tangent segments and the
/// fillet arc, and the solid it sweeps is that region revolved about
/// the axis, so by Pappus `V = 2π ∫∫ x dA`. The region is the KITE
/// `P, T1, C, T2` MINUS the circular sector of the ball between the two
/// tangent points, and both pieces have exact first moments:
///
/// - a triangle's `∫∫ x dA` is its area times the mean of its three
///   vertices' `x`;
/// - a sector of radius `r` about `C` spanning `[α1, α2]` has
///   `∫∫ x dA = A·C.x + (r³/3)(sin α2 − sin α1)`.
///
/// Nothing here is read from the kernel: `P`, the two tangent points and
/// `C` are all written from the profile's own geometry.
fn removed_volume_two_straight(
    corner: (f64, f64),
    center: (f64, f64),
    t1: (f64, f64),
    t2: (f64, f64),
    r: f64,
) -> f64 {
    let tri = |a: (f64, f64), b: (f64, f64), c: (f64, f64)| {
        let area = ((b.0 - a.0) * (c.1 - a.1) - (c.0 - a.0) * (b.1 - a.1)).abs() / 2.0;
        (area, area * (a.0 + b.0 + c.0) / 3.0)
    };
    let (a1, m1) = tri(corner, t1, center);
    let (a2, m2) = tri(corner, center, t2);
    let _kite_area = a1 + a2;
    let kite_moment = m1 + m2;

    // The sector between the two tangent points, on the side facing the
    // corner.
    let ang = |t: (f64, f64)| (t.1 - center.1).atan2(t.0 - center.0);
    let (mut lo, mut hi) = (ang(t1), ang(t2));
    if hi < lo {
        core::mem::swap(&mut lo, &mut hi);
    }
    // The corner must lie inside the swept wedge; if it does not, the
    // sector is the complementary one.
    let ac = ang(corner);
    let (lo, hi) = if ac > lo && ac < hi {
        (lo, hi)
    } else {
        (hi, lo + 2.0 * PI)
    };
    let theta = hi - lo;
    let sector_area = 0.5 * r * r * theta;
    let sector_moment = sector_area * center.0 + (r * r * r / 3.0) * (hi.sin() - lo.sin());

    2.0 * PI * (kite_moment - sector_moment)
}

/// The lip corner's ball centre, from its own two linear equations.
fn lip_center(r: f64) -> (f64, f64) {
    let y = TOP - r;
    (SHOULDER.0 + SHOULDER.1 - y - r * SQRT_2, y)
}

/// A point in the meridian half-plane: `(radial, axial)`.
type Meridian2 = (f64, f64);

/// The lip carve's four defining points: the corner, the ball centre,
/// the tangent point on the top plane, and the one on the cone.
fn lip_geometry(r: f64) -> (Meridian2, Meridian2, Meridian2, Meridian2) {
    let c = lip_center(r);
    let corner = (LIP_R, TOP);
    let t_top = (c.0, TOP);
    let n = (1.0 / SQRT_2, 1.0 / SQRT_2);
    let t_cone = (c.0 + r * n.0, c.1 + r * n.1);
    (corner, c, t_top, t_cone)
}

/// **The seam-split carve removes what a hand quadrature says it does.**
///
/// This is the oracle the PR's suite does NOT have: its volume row is a
/// differential against the bored one-edge twin, whose carve is a
/// sibling of this one and could share a defect. This row compares the
/// seam-split lantern's lip carve against a closed form written from
/// the profile alone.
#[test]
fn the_lip_bands_removed_volume_matches_a_hand_quadrature() {
    let source = lantern();
    let before = volume(&source);
    for r in [0.02, 0.05, 0.08] {
        let arcs = rim_arcs_at(&source, LIP_R, TOP);
        assert_eq!(arcs.len(), 2, "the lip rim is seam-split");
        let out = fillet_edges(&source, &arcs, r, band(), tol())
            .unwrap_or_else(|e| panic!("the lip fillets at r={r}, got {e:?}"));
        let removed = before - volume(&out.body);
        let (corner, c, t_top, t_cone) = lip_geometry(r);
        let want = removed_volume_two_straight(corner, c, t_top, t_cone, r);
        assert!(
            want > 0.0,
            "the hand form must itself be a positive volume at r={r}, got {want}"
        );
        assert!(
            (removed - want).abs() < 1e-12,
            "r={r}: the carve removed {removed}, the hand quadrature says {want} \
             (difference {})",
            removed - want
        );
    }
}

/// **The same hand form holds for the ONE-EDGE twin.** So the
/// differential's trusted side is measured rather than assumed.
#[test]
fn the_same_hand_quadrature_holds_for_the_one_edge_twin() {
    let source = bored_lantern();
    let before = volume(&source);
    for r in [0.02, 0.05, 0.08] {
        let arcs = rim_arcs_at(&source, LIP_R, TOP);
        assert_eq!(arcs.len(), 1, "the twin's lip rim is one closed edge");
        let out = fillet_edges(&source, &arcs, r, band(), tol())
            .unwrap_or_else(|e| panic!("the twin's lip fillets at r={r}, got {e:?}"));
        let removed = before - volume(&out.body);
        let (corner, c, t_top, t_cone) = lip_geometry(r);
        let want = removed_volume_two_straight(corner, c, t_top, t_cone, r);
        assert!(
            (removed - want).abs() < 1e-12,
            "r={r}: the twin removed {removed}, the hand quadrature says {want}"
        );
    }
}

// ------------------------------------------------------------------
// Planted violations of the seam-split resolution checks.
// ------------------------------------------------------------------

/// **A STRICT SUBSET of a seam-split rim's arcs must refuse.** One arc
/// of a two-arc rim is an OPEN chain terminating at two seam vertices,
/// which is exactly the shape ARMS-3 refuses — the door this PR builds
/// must not have widened that into half a band.
#[test]
fn a_strict_subset_of_a_seam_split_rims_arcs_refuses_typed() {
    let source = lantern();
    for (name, rr, ry) in [
        ("neck", 1.0, 0.0),
        ("shoulder", SHOULDER.0, SHOULDER.1),
        ("lip", LIP_R, TOP),
    ] {
        let arcs = rim_arcs_at(&source, rr, ry);
        assert_eq!(arcs.len(), 2, "{name} is seam-split");
        for one in &arcs {
            match fillet_edges(&source, &[*one], 0.05, band(), tol()) {
                Err(FilletError::FilletCornerUnsupported { .. }) => {}
                Err(other) => {
                    panic!("{name}: one arc alone should refuse at the seam vertex, got {other:?}")
                }
                Ok(_) => panic!("{name}: one arc alone must NOT carve"),
            }
        }
    }
}

/// **Two rims sharing a wall, in ONE call, carve** — FLIPPED
/// DELIBERATELY at #935 (BLEND-2). The reviewer pinned the upfront
/// refusal this request met at BLEND-1's merge; the seam-key refresh
/// now serves it, and what the row keeps of the reviewer's claim is
/// the half that is still true: nothing is half-built — the request
/// carves BOTH bands to a tier-3-valid solid. The widened door's own
/// rows (equality with the sequential composition to the bit, naming
/// totality, the colliding-band boundary) live in `blend_tworims.rs`.
#[test]
fn two_rims_sharing_a_wall_in_one_call_carve_both_bands() {
    let source = lantern();
    let mut both = rim_arcs_at(&source, 1.0, 0.0);
    both.extend(rim_arcs_at(&source, SHOULDER.0, SHOULDER.1));
    assert_eq!(both.len(), 4, "two seam-split rims are four arcs");
    let out = fillet_edges(&source, &both, 0.05, band(), tol())
        .unwrap_or_else(|e| panic!("the shared-wall pair carves in one call (#935), got {e:?}"));
    assert_eq!(out.band_faces.len(), 2, "one band per rim");
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("tier 3, got {e:?}"));
}

/// **Arcs from two DIFFERENT rims are not one rim.** One arc of the
/// neck and one of the shoulder: a request the resolver must not read
/// as a closed chain of one rim.
#[test]
fn arcs_of_two_different_rims_refuse_typed() {
    let source = lantern();
    let neck = rim_arcs_at(&source, 1.0, 0.0);
    let shoulder = rim_arcs_at(&source, SHOULDER.0, SHOULDER.1);
    let mixed = [neck[0], shoulder[0]];
    match fillet_edges(&source, &mixed, 0.05, band(), tol()) {
        Err(e) => println!("mixed-rim refusal: {e:?}"),
        Ok(_) => panic!("one arc of each of two rims must not carve"),
    }
}

// ------------------------------------------------------------------
// Naming.
// ------------------------------------------------------------------

/// **No two birth rows of one field key on the same source entity.**
/// The PR's claim is "every name stays unique because each row keys on
/// a distinct source entity"; this measures it on every rim of the
/// lantern rather than on one.
#[test]
fn a_seam_split_bands_birth_rows_key_uniquely() {
    let source = lantern();
    for (name, rr, ry) in [
        ("neck", 1.0, 0.0),
        ("shoulder", SHOULDER.0, SHOULDER.1),
        ("lip", LIP_R, TOP),
    ] {
        let arcs = rim_arcs_at(&source, rr, ry);
        let out = fillet_edges(&source, &arcs, 0.05, band(), tol())
            .unwrap_or_else(|e| panic!("{name} fillets, got {e:?}"));
        let rec = out.naming.as_ref().expect("birth records");

        let uniq = |what: &str, mut keys: Vec<String>| {
            let n = keys.len();
            keys.sort();
            keys.dedup();
            assert_eq!(keys.len(), n, "{name}: {what} rows collide on a source key");
        };
        uniq(
            "rim_trims",
            rec.rim_trims
                .iter()
                .map(|(_, e, s)| format!("{e:?}/{s:?}"))
                .collect(),
        );
        uniq(
            "rim_feet",
            rec.rim_feet.iter().map(|(_, v)| format!("{v:?}")).collect(),
        );
        uniq(
            "meridian_splits",
            rec.meridian_splits
                .iter()
                .map(|(_, e)| format!("{e:?}"))
                .collect(),
        );
        uniq(
            "meridian_remnants",
            rec.meridian_remnants
                .iter()
                .map(|(_, e)| format!("{e:?}"))
                .collect(),
        );
        uniq(
            "slits",
            rec.slits.iter().map(|(_, e)| format!("{e:?}")).collect(),
        );
        // The MINTED side must be injective too: one key, one row.
        let mut minted: Vec<String> = rec
            .rim_trims
            .iter()
            .map(|(k, _, _)| format!("e{k:?}"))
            .chain(rec.meridian_remnants.iter().map(|(k, _)| format!("e{k:?}")))
            .chain(rec.slits.iter().map(|(k, _)| format!("e{k:?}")))
            .chain(rec.rim_feet.iter().map(|(k, _)| format!("v{k:?}")))
            .chain(rec.meridian_splits.iter().map(|(k, _)| format!("v{k:?}")))
            .collect();
        let n = minted.len();
        minted.sort();
        minted.dedup();
        assert_eq!(minted.len(), n, "{name}: two rows mint the same key");

        // And the retirement list names each key once.
        let mut dead: Vec<String> = rec.dead.edges.iter().map(|e| format!("{e:?}")).collect();
        let dn = dead.len();
        dead.sort();
        dead.dedup();
        assert_eq!(dead.len(), dn, "{name}: an edge is retired twice");
    }
}

// ------------------------------------------------------------------
// The merge-base differential (claim 1).
// ------------------------------------------------------------------

/// A canonical fingerprint of a carve: entity census plus the volume's
/// exact bits plus the shape of the name table.
fn fingerprint(source: &Body<f64>, arcs: &[EdgeKey], r: f64) -> String {
    let out = fillet_edges(source, arcs, r, band(), tol()).expect("carves");
    let rec = out.naming.as_ref().expect("names");
    let props = mass_properties(&out.body, tol()).expect("mass properties");
    format!(
        "f={} e={} v={} l={} bands={} vol={:#018x} pad={} \
         trims={} feet={} msplit={} mrem={} slits={} dead_e={} dead_v={}",
        out.body.faces().count(),
        out.body.edges().count(),
        out.body.vertices().count(),
        out.body.loops().count(),
        out.band_faces.len(),
        props.volume.to_bits(),
        props.volume_pad,
        rec.rim_trims.len(),
        rec.rim_feet.len(),
        rec.meridian_splits.len(),
        rec.meridian_remnants.len(),
        rec.slits.len(),
        rec.dead.edges.len(),
        rec.dead.vertices.len(),
    )
}

/// **The ONE-EDGE annulus sequence is unchanged, move for move.** The
/// PR claims the one-entry case reproduces the old surgery exactly.
/// This row prints a full census + volume-BIT fingerprint of all three
/// one-edge rims of the bored twin; the same file run at the merge base
/// (`26c9e19c`) must print the same three lines.
///
/// The expected strings below were measured at the MERGE BASE and
/// pasted here, so this row goes red if the head moved any of them.
#[test]
fn the_one_edge_annulus_fingerprint_is_stable() {
    let source = bored_lantern();
    let mut lines = Vec::new();
    for (name, rr, ry) in [
        ("neck", 1.0, 0.0),
        ("shoulder", SHOULDER.0, SHOULDER.1),
        ("lip", LIP_R, TOP),
    ] {
        let arcs = rim_arcs_at(&source, rr, ry);
        assert_eq!(arcs.len(), 1, "{name} is one closed edge on the twin");
        let fp = fingerprint(&source, &arcs, 0.05);
        println!("FINGERPRINT {name}: {fp}");
        lines.push(format!("{name}: {fp}"));
    }
    // Pinned from the merge base run. See the probe branch's log.
    let want = [
        "neck: f=6 e=12 v=6 l=6 bands=1 vol=0x400129725c9a0b3f pad=0 trims=2 feet=1 msplit=1 mrem=2 slits=1 dead_e=2 dead_v=1",
        "shoulder: f=6 e=12 v=6 l=6 bands=1 vol=0x400130d71d2ca44c pad=0 trims=2 feet=1 msplit=1 mrem=2 slits=1 dead_e=1 dead_v=1",
        "lip: f=6 e=12 v=6 l=6 bands=1 vol=0x400130b4b9b69dac pad=0 trims=2 feet=1 msplit=1 mrem=2 slits=1 dead_e=1 dead_v=1",
    ];
    for (got, want) in lines.iter().zip(want.iter()) {
        assert_eq!(got, want, "the one-edge sequence moved");
    }
}

// ------------------------------------------------------------------
// A second pole-touching body the PR never built.
// ------------------------------------------------------------------

/// **A capped cylinder — two seam-split rims, sequential calls.**
/// A profile touching the axis at BOTH ends: base disk, cylinder wall,
/// top disk. Each rim is two arcs over two half-disc supports and two
/// half-cylinder supports, and the two rims do share the cylinder wall,
/// so they go in separate calls.
#[test]
fn a_cylinder_capped_both_ends_carves_both_seam_split_rims() {
    let source = revolved_about_y(
        vec![
            v(0.0, 0.0, 0.0),
            v(1.0, 0.0, 0.0),
            v(1.0, 1.0, 0.0),
            v(0.0, 1.0, 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    let before = volume(&source);
    let r = 0.1;
    let mut body = source.clone();
    let mut bands = 0;
    for (name, ry) in [("base", 0.0), ("top", 1.0)] {
        let arcs = rim_arcs_at(&body, 1.0, ry);
        assert_eq!(arcs.len(), 2, "{name} rim is seam-split");
        let out = fillet_edges(&body, &arcs, r, band(), tol())
            .unwrap_or_else(|e| panic!("{name} rim fillets, got {e:?}"));
        bands += out.band_faces.len();
        body = out.body;
    }
    assert_eq!(bands, 2, "one band per rim");
    validate_geometric(&body, tol()).expect("tier-3 valid");

    // Each rim is a plane×cylinder corner, both supports straight in
    // the meridian, so the SAME hand quadrature applies — twice.
    let corner = (1.0, 0.0);
    let c = (1.0 - r, r);
    let one = removed_volume_two_straight(corner, c, (1.0, r), (1.0 - r, 0.0), r);
    let removed = before - volume(&body);
    assert!(
        (removed - 2.0 * one).abs() < 1e-12,
        "two capped-cylinder fillets remove {removed}, the hand form says {}",
        2.0 * one
    );
}

/// **The `SeamVertex` recourse names a door that answers.** At a site
/// where the tag fires — one arc of a capped cylinder's base rim — the
/// request the recourse names ("the rim whole") must then CARVE, which
/// is the standard the A3-2 correction set and the thing this PR
/// claims to have made true.
#[test]
fn the_seam_vertex_recourse_names_a_door_that_answers() {
    let source = revolved_about_y(
        vec![
            v(0.0, 0.0, 0.0),
            v(1.0, 0.0, 0.0),
            v(1.0, 1.0, 0.0),
            v(0.0, 1.0, 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    let arcs = rim_arcs_at(&source, 1.0, 0.0);
    assert_eq!(arcs.len(), 2);
    // The refusal fires...
    let refused = fillet_edges(&source, &arcs[..1], 0.1, band(), tol());
    match refused {
        Err(e @ FilletError::FilletCornerUnsupported { .. }) => {
            let msg = format!("{e}");
            println!("recourse: {msg}");
            assert!(
                msg.contains("rim whole"),
                "the recourse names the whole-rim request, got {msg}"
            );
        }
        other => panic!("one arc alone refuses at the seam vertex, got {other:?}"),
    }
    // ...and the door it names answers.
    let out = fillet_edges(&source, &arcs, 0.1, band(), tol())
        .expect("the recourse's own request must carve");
    assert_eq!(out.band_faces.len(), 1);
    validate_geometric(&out.body, tol()).expect("tier-3 valid");
}

// ------------------------------------------------------------------
// Claim 8: is the rewritten recourse true at EVERY site the tag fires?
// ------------------------------------------------------------------

/// A waisted pole-touching revolve — the PR's own concave fixture. Two
/// cones meeting at radius 0.5, so the waist rim is CONCAVE and
/// seam-split; the base and top rims are convex and seam-split.
fn waisted() -> Body<f64> {
    revolved_about_y(
        vec![
            v(0.0, 0.0, 0.0),
            v(1.0, 0.0, 0.0),
            v(0.5, 0.5, 0.0),
            v(1.0, 1.0, 0.0),
            v(0.0, 1.0, 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// **Claim 7 support: the concave row's fixture DOES reach the door.**
/// The waisted body's other rims carve through this unit's door, so the
/// concave refusal at its waist is about convexity and not about the
/// body being unreachable for some upstream reason.
#[test]
fn the_waisted_bodys_convex_rims_carve_so_its_concave_row_is_not_vacuous() {
    let source = waisted();
    for (name, ry) in [("base", 0.0), ("top", 1.0)] {
        let arcs = rim_arcs_at(&source, 1.0, ry);
        assert_eq!(arcs.len(), 2, "{name} rim is seam-split");
        let out = fillet_edges(&source, &arcs, 0.05, band(), tol())
            .unwrap_or_else(|e| panic!("{name} rim of the waisted body carves, got {e:?}"));
        assert_eq!(out.band_faces.len(), 1);
        validate_geometric(&out.body, tol()).expect("tier-3 valid");
    }
}

/// **The composed honesty pin: the recourse is TRUE at every site the
/// tag fires.**
///
/// This row began as the r2 review's MAJOR — the rewritten recourse
/// promised "carves as one annulus" unconditionally, while
/// `is_seam_vertex` classifies purely on INCIDENCE (two rim arcs on one
/// support pair plus two co-surface seams) and never reads convexity.
/// The tag therefore fires at a CONCAVE rim's seam vertex too, where
/// the promised whole-rim request refuses `"a concave chain adds
/// material"` — a recourse naming a door that cannot serve the caller
/// who was just refused, which is the exact A3-2 defect.
///
/// The fix conditioned the SENTENCE rather than widening the carve
/// (the concave closed-rim band is unbuilt and filed as issue 1244;
/// widening a material gate is not the seam-split door's business).
/// So this row now composes the two halves that must never drift
/// apart, on BOTH material sides:
///
/// - the sentence still names the REQUEST unconditionally, and states
///   the carve only for the convex side — asserted on the rendered
///   refusal, not on the constant alone;
/// - the whole-rim request then does what the sentence says it does:
///   it CARVES at the convex rim, and answers with the material-side
///   refusal at the concave one.
///
/// Red if the hedge is dropped, red if the tag narrows to convex rims
/// only without the sentence following, and red if issue 1244 lands
/// without this sentence being re-widened.
#[test]
fn the_seam_vertex_recourse_is_true_at_every_site_the_tag_fires() {
    let source = waisted();
    // The sentence conditions its carve half. Spelled against the
    // constant so the assertion names what the hedge IS, not a
    // substring of one phrasing of it.
    assert!(
        FILLET3_SEAM_VERTEX_RECOURSE.contains("CONVEX"),
        "the carve half is conditioned on the side the door serves: \
         {FILLET3_SEAM_VERTEX_RECOURSE}"
    );

    for (name, rim_r, rim_y, convex) in [
        ("the concave waist", 0.5, 0.5, false),
        ("the convex base", 1.0, 0.0, true),
        ("the convex top", 1.0, 1.0, true),
    ] {
        let arcs = rim_arcs_at(&source, rim_r, rim_y);
        assert_eq!(arcs.len(), 2, "{name} is seam-split");

        // Half one: the tag fires, and shows the conditioned sentence.
        let Err(one) = fillet_edges(&source, &arcs[..1], 0.05, band(), tol()) else {
            panic!("{name}: one arc stops at a seam vertex and must refuse")
        };
        assert!(
            matches!(
                one,
                FilletError::FilletCornerUnsupported {
                    corner: CornerConfig::SeamVertex,
                    policy: None,
                    ..
                }
            ),
            "{name}: the incidence-only tag fires on both material sides, got {one:?}"
        );
        let shown = one.to_string();
        assert!(
            shown.contains(FILLET3_SEAM_VERTEX_RECOURSE),
            "{name}: the seam recourse is the one appended: {shown}"
        );

        // Half two: the request that sentence names, answered.
        let whole = fillet_edges(&source, &arcs, 0.05, band(), tol());
        if convex {
            let out =
                whole.unwrap_or_else(|e| panic!("{name}: the promised carve happens, got {e:?}"));
            assert_eq!(out.band_faces.len(), 1, "{name}: one annulus band");
            validate_geometric(&out.body, tol())
                .unwrap_or_else(|e| panic!("{name}: tier-3 valid, got {e:?}"));
        } else {
            let Err(FilletError::UnsupportedChain { detail, .. }) = whole else {
                panic!("{name}: the whole-rim request meets the material-side refusal")
            };
            assert!(
                detail.contains("concave"),
                "{name}: and it is the material side that refuses, got {detail}"
            );
        }
    }
}
