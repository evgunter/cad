//! **R1 review probes for MESH-10** (issue 1562) — body-level rows
//! against the torus meridian fold, written to falsify.
//!
//! The unit pins the split-seam donut through every consumer and pins
//! the partition premise on `split_edge`. The rows here ask what those
//! leave open:
//!
//! * the premise the fold rests on — "the pieces of a split edge
//!   partition the parent's own parametrisation" — is a property of
//!   `split_edge`, not an invariant the fold checks. `set_edge_curve`
//!   is public and does not clear an edge's split lineage. What does
//!   the fold answer for a split child whose interval has been moved
//!   through that door?
//! * the same, on a body where the moved interval is congruent mod 2π,
//!   so the folded span's SIN/COS still place the rims at the extremes
//!   while the span itself is wrong: does any consumer notice?
//! * the matrix the unit reports (every donut edge × both split
//!   patterns), measured here independently, including the second
//!   seam-side copy of the meridian and a three-piece chain.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

mod common;

use common::*;
use geom_core::Tol;
use topo::Body;

fn tris(m: &mesh::Mesh) -> usize {
    m.patches.iter().map(|p| p.triangles.len()).sum()
}

fn band() -> geom_core::Band {
    geom_core::Band::linear(Tol::witness()).unwrap()
}

/// Every face's `(door, flux bits, area bits, side)` on a body.
type Receipt = (
    String,
    Result<(), String>,
    Result<(u64, u64), String>,
    Result<String, String>,
);
fn receipts(body: &Body<f64>) -> Vec<Receipt> {
    body.faces()
        .map(|(fk, f)| {
            let (outer, _) = topo::props::loop_edges(body, f.outer).unwrap();
            let s = body.get_surface(f.surface).unwrap();
            let sense = if f.sense { 1.0 } else { -1.0 };
            (
                format!("{fk:?}"),
                geom_brep::props::require_iso_rectangle(s, &outer, band()).map_err(|e| format!("{e:?}")),
                geom_brep::props::curved_face(s, &outer, sense, band())
                    .map(|c| (c.flux.to_bits(), c.area.to_bits()))
                    .map_err(|e| format!("{e:?}")),
                geom_brep::props::boundary_material_sign(s, &outer, band())
                    .map(|m| format!("{m:?}"))
                    .map_err(|e| format!("{e:?}")),
            )
        })
        .collect()
}

/// The donut with edge `i` split at the given fractions of its stored
/// interval.
fn split_donut(i: usize, fracs: &[f64]) -> Body<f64> {
    let tol = Tol::witness();
    let mut body = donut();
    let ek = body.edges().nth(i).unwrap().0;
    let c = body
        .get_curve_geom(body.get_edge(ek).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap();
    let (t0, t1) = c.params();
    for f in fracs {
        body.split_edge(ek, t0 + f * (t1 - t0), tol).unwrap();
    }
    body
}

/// **The matrix, re-measured.** Every edge of the tour donut at both
/// of the issue-653 sweep's patterns: the door, the flux lane, the
/// side and `mass_properties`, each compared BITWISE against the
/// unsplit donut. Independent of the unit's own row.
#[test]
fn r1_every_donut_split_is_the_donut_bitwise() {
    let tol = Tol::witness();
    let base = donut();
    let r0 = receipts(&base);
    let mp0 = topo::mass_properties(&base, tol).unwrap();
    let n = base.edges().count();
    println!("donut edges: {n}");
    let mut bad = Vec::new();
    for i in 0..n {
        for fracs in [&[0.5][..], &[0.3129, 0.15645][..]] {
            let body = split_donut(i, fracs);
            let r = receipts(&body);
            let mp = topo::mass_properties(&body, tol);
            let vol_ok = mp
                .as_ref()
                .map(|m| {
                    (m.volume.to_bits(), m.surface_area.to_bits())
                        == (mp0.volume.to_bits(), mp0.surface_area.to_bits())
                })
                .unwrap_or(false);
            let faces_ok = r == r0;
            println!(
                "edge {i} @{fracs:?}: faces_bitwise={faces_ok} mp_bitwise={vol_ok} mp={:?}",
                mp.as_ref().map(|m| m.volume)
            );
            if !(vol_ok && faces_ok) {
                bad.push(format!("edge {i} @{fracs:?}: faces={r:?} mp={mp:?}"));
            }
        }
    }
    assert!(bad.is_empty(), "not bitwise the donut: {bad:#?}");
}

/// **The mesh of the split-seam donut, measured here.** Positions,
/// triangles, watertightness, and where the two meshes differ.
#[test]
fn r1_split_seam_donut_mesh_measured() {
    let tol = Tol::witness();
    for (i, fracs) in [
        (0usize, &[0.5][..]),
        (0, &[0.3129, 0.15645][..]),
        (1, &[0.5][..]),
    ] {
        let m0 = mesh::tessellate(&donut(), 0.1, tol).unwrap();
        let m = mesh::tessellate(&split_donut(i, fracs), 0.1, tol).unwrap();
        let wt = mesh::validate::check_mesh(&m).map_err(|e| format!("{e:?}"));
        let key = |mm: &mesh::Mesh| {
            let mut v: Vec<[u64; 3]> = mm
                .positions
                .iter()
                .map(|p| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()])
                .collect();
            v.sort_unstable();
            v.dedup();
            v
        };
        let (a, b) = (key(&m0), key(&m));
        let only_a = a.iter().filter(|x| b.binary_search(x).is_err()).count();
        let only_b = b.iter().filter(|x| a.binary_search(x).is_err()).count();
        // Worst distance of any position from the torus surface.
        let worst = m
            .positions
            .iter()
            .map(|p| {
                let rho = (p.x * p.x + p.z * p.z).sqrt();
                (((rho - 2.0).powi(2) + p.y * p.y).sqrt() - 0.5).abs()
            })
            .fold(0.0_f64, f64::max);
        println!(
            "edge {i} @{fracs:?}: positions {} (unsplit {}), tris {} (unsplit {}), \
             watertight={wt:?}, only-unsplit {only_a}, only-split {only_b}, off-torus {worst:e}"
        , m.positions.len(), m0.positions.len(), tris(&m), tris(&m0));
        assert!(wt.is_ok());
    }
}

/// **THE FINDING (R1 MAJOR): a silently wrong volume through the
/// public API.** The fold's premise — the pieces of a split edge
/// partition the parent's own parametrisation — is `split_edge`'s
/// guarantee, and nothing re-checks it at the fold.
/// `Body::set_edge_curve` is public API, keeps an edge's split
/// lineage, and takes the stored interval from the caller. Move ONE
/// child's interval by +2π on its own carrier (the identical
/// geometric arc; both endpoints unmoved; certification passes) and
/// the fold reads `[lowest t0, highest t1] = 3π` for a meridian that
/// spans π. `sin`/`cos` of 3π place the rims at the extremes, so the
/// shape door answers `Ok(())`, `boundary_material_sign` answers, the
/// mesh is the correct 7311-position mesh — and `mass_properties`
/// returns **twice the donut's volume and area**, with no refusal
/// anywhere. Before this unit the same body refused typed
/// (`props_rim_level`), because each piece was its own meridian.
/// The per-edge gate that would have caught the same interval on ONE
/// edge is certification's `WindingExceeded`
/// (`r1_a_reparametrised_unsplit_edge_control` below); the fold
/// reconstructs a span across edges that no single edge could
/// certify, and has no equivalent gate.
///
/// **This row asserts the DEFECT.** Invert it when the premise is
/// enforced at the fold.
#[test]
fn r1_a_reparametrised_split_child_still_folds() {
    let tol = Tol::witness();
    let base = donut();
    let mp0 = topo::mass_properties(&base, tol).unwrap();
    println!("unsplit: V={} A={}", mp0.volume, mp0.surface_area);
    let mut body = split_donut(0, &[0.5]);
    // The second child is the edge the split minted: the last edge.
    let child = body.edges().last().unwrap().0;
    let cert = body
        .get_curve_geom(body.get_edge(child).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap()
        .clone();
    let (t0, t1) = cert.params();
    println!("child stored interval before: [{t0}, {t1}]");
    let mut spec = cert.restated_spec();
    let shift = core::f64::consts::TAU;
    spec.param_start = t0 + shift;
    spec.param_end = t1 + shift;
    match body.set_edge_curve(child, spec, tol) {
        Err(e) => println!("set_edge_curve REFUSED: {e:?}"),
        Ok(_) => {
            let c2 = body
                .get_curve_geom(body.get_edge(child).unwrap().curve)
                .unwrap()
                .certified()
                .unwrap();
            println!("child stored interval after: {:?}", c2.params());
            let mp = topo::mass_properties(&body, tol);
            println!("mass_properties: {:?}", mp.as_ref().map(|m| (m.volume, m.surface_area)));
            println!("tessellate: {:?}", mesh::tessellate(&body, 0.1, tol).map(|m| m.positions.len()).map_err(|e| format!("{e:?}")));
            for r in receipts(&body) {
                println!("  face {}: door={:?} flux={:?} side={:?}", r.0, r.1, r.2, r.3);
            }
            let m = mp.expect("mass_properties answers - it does not refuse");
            println!("V ratio to the unsplit donut: {}", m.volume / mp0.volume);
            assert_eq!(
                m.volume / mp0.volume,
                2.0,
                "the folded span is 3pi, not pi: the volume is exactly doubled"
            );
        }
    }
}

/// The same door, on an UNSPLIT edge given the same shifted interval —
/// the control that says whether the fold widened the reach of a
/// caller-stated interval or merely inherited it.
#[test]
fn r1_a_reparametrised_unsplit_edge_control() {
    let tol = Tol::witness();
    let mut body = donut();
    let ek = body.edges().next().unwrap().0;
    let cert = body
        .get_curve_geom(body.get_edge(ek).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap()
        .clone();
    let (t0, t1) = cert.params();
    let mut spec = cert.restated_spec();
    spec.param_start = t0;
    spec.param_end = t1 + core::f64::consts::TAU;
    match body.set_edge_curve(ek, spec, tol) {
        Err(e) => println!("unsplit +2π span: set_edge_curve REFUSED: {e:?}"),
        Ok(_) => {
            let mp = topo::mass_properties(&body, tol);
            println!("unsplit +2π span accepted; mass_properties {:?}", mp.map(|m| m.volume));
        }
    }
}
