//! The M1 PR 3 acceptance test: build a box with a square through-hole
//! (genus 1) by hand through the public Euler-operator API — 1 `mvfs` +
//! 15 `mev` + 10 `mef` + 1 `kemr` + 1 `kfmrh`, the provably minimal
//! sequence (Mäntylä §9.3 / §9.4.2 eq. 9.5 for v16 e24 f10 r2 h1 s1) —
//! validating after every operator, then checking counts, the
//! Euler–Poincaré ledger at genus 1, ring orientations, provenance
//! (live and killed), and lineage determinism.
//!
//! # The construction, geometrically
//!
//! Outer box 2×2×2; square hole 1×1 through the top and bottom faces:
//!
//! ```text
//!        D'-----------C'         bottom corners A B C D at z = 0,
//!       /|  S'---R'  /|          tops A' B' C' D' at z = 2;
//!      / | |     |  / |          hole rim P Q R S on the top face
//!     A'-----------B' |          (z = 2) over p q r s on the bottom
//!     |  D-|-----|-|--C          (z = 0):
//!     | /  s---r   | /             P=(.5,.5,2) Q=(1.5,.5,2)
//!     |/           |/              R=(1.5,1.5,2) S=(.5,1.5,2)
//!     A------------B               p,q,r,s directly below.
//! ```
//!
//! Sequence (§9.3 steps (a)–(l), mirrored to our CCW convention):
//!
//! - (a)–(e): the 2×2×2 box exactly as the cube acceptance test builds
//!   it (1 mvfs + 7 mev + 5 mef; the mvfs face ends as the TOP face).
//! - (f) one strut `mev` in the top face from corner A' to P;
//! - (g) `kemr` kills the strut: P is stranded as an **empty ring** of
//!   the top face — the planted hole anchor;
//! - (h) 3 `mev` grow the ring into the open chain P→Q→R→S;
//! - (i) `mef` closes the chain: the ring becomes the hole's top rim
//!   (walking P→S→R→Q, clockwise from above — rings run CW viewed from
//!   outside) and a new *membrane* face covers the opening (outer
//!   P→Q→R→S, CCW from above);
//! - (j) 4 `mev` drop the hole's verticals from P,Q,R,S to p,q,r,s
//!   inside the membrane;
//! - (k) 4 `mef` cut the four tube walls out of the membrane, leaving
//!   the membrane face as the square p→q→r→s at the bottom face's level
//!   (a zero-thickness floor — geometrically degenerate, tier-1-legal,
//!   and §9.3's exact intermediate);
//! - (l) `kfmrh` kills the membrane and demotes its loop to a ring of
//!   the bottom face: the through-hole is complete, genus 1.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point3;
use geom_core::Tol;
use topo::{
    Body, EntityId, KemrResult, KfmrhResult, MefCreated, MefSite, MevCreated, MevSite, MvfsCreated,
    Provenance, validate, validate_closed,
};

/// Every operator result of one holed-box construction, in call order.
/// Comparing two of these key-for-key is the lineage-determinism check.
#[derive(PartialEq, Eq, Debug)]
struct HoledBox {
    seed: MvfsCreated,
    box_mevs: [MevCreated; 7],
    box_mefs: [MefCreated; 5],
    strut: MevCreated,
    kill: KemrResult,
    rim_mevs: [MevCreated; 3],
    mef_top: MefCreated,
    tube_mevs: [MevCreated; 4],
    tube_mefs: [MefCreated; 4],
    plug: KfmrhResult,
}

/// Builds the holed box, asserting tier-1 validity after EVERY operator
/// (explicitly — not relying on the operators' debug postconditions).
fn build_holed_box(body: &mut Body<f64>) -> HoledBox {
    let pt = Point3::new;
    let ck = |body: &Body<f64>| assert_eq!(validate(body), Ok(()));

    // ---- (a)–(e): the 2×2×2 box, exactly the cube test's sequence. ----
    let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap(); // A
    ck(body);
    let mev = |body: &mut Body<f64>, site, x, y, z| {
        let created = body.mev_line(site, pt(x, y, z), Tol::witness()).unwrap();
        assert_eq!(validate(body), Ok(()));
        created
    };
    let mef = |body: &mut Body<f64>, he1, he2| {
        let created = body
            .mef_chord(MefSite::Chords { he1, he2 }, Tol::witness())
            .unwrap();
        assert_eq!(validate(body), Ok(()));
        created
    };
    let strut_at = |body: &mut Body<f64>, at, x, y, z| {
        let created = body
            .mev_line(
                MevSite::Fan { he1: at, he2: at },
                pt(x, y, z),
                Tol::witness(),
            )
            .unwrap();
        assert_eq!(validate(body), Ok(()));
        created
    };

    // Bottom chain A→B→C→D.
    let e_ab = mev(
        body,
        MevSite::Lone {
            r#loop: seed.r#loop,
        },
        2.0,
        0.0,
        0.0,
    ); // B
    let e_bc = strut_at(body, e_ab.he_minus, 2.0, 2.0, 0.0); // C
    let e_cd = strut_at(body, e_bc.he_minus, 0.0, 2.0, 0.0); // D
    // Close the bottom square (the lamina step): new bottom face walks
    // A→D→C→B, CCW seen from below.
    let he_dc = body
        .find_half_edge(seed.face, e_cd.vertex, e_bc.vertex)
        .unwrap();
    let f_bottom = mef(body, he_dc, e_ab.he_plus);
    // Verticals A→A' … D→D'.
    let e_aa = strut_at(body, e_ab.he_plus, 0.0, 0.0, 2.0);
    let e_bb = strut_at(body, e_bc.he_plus, 2.0, 0.0, 2.0);
    let e_cc = strut_at(body, e_cd.he_plus, 2.0, 2.0, 2.0);
    let e_dd = strut_at(body, f_bottom.he_plus, 0.0, 2.0, 2.0);
    // The four side faces; the seed face remains as the top square
    // A'→B'→C'→D' (CCW from above).
    let f_front = mef(body, e_aa.he_minus, e_bb.he_minus);
    let f_right = mef(body, e_bb.he_minus, e_cc.he_minus);
    let f_back = mef(body, e_cc.he_minus, e_dd.he_minus);
    let f_left = mef(body, e_dd.he_minus, f_front.he_plus);

    // ---- (f) strut A' → P in the top face. ----
    // f_front.he_plus is the top-face half A'→B'.
    let strut = strut_at(body, f_front.he_plus, 0.5, 0.5, 2.0); // P

    // ---- (g) kill it: P becomes an empty ring of the top face. ----
    let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
    ck(body);

    // ---- (h) grow the rim chain P→Q→R→S inside the ring. ----
    let s_pq = mev(body, MevSite::Lone { r#loop: kill.ring }, 1.5, 0.5, 2.0); // Q
    let s_qr = strut_at(body, s_pq.he_minus, 1.5, 1.5, 2.0); // R
    let s_rs = strut_at(body, s_qr.he_minus, 0.5, 1.5, 2.0); // S

    // ---- (i) close the rim: ring = hole's top opening (CW from
    // above), new membrane face covers it (CCW from above). ----
    let mef_top = mef(body, s_pq.he_plus, s_rs.he_minus);

    // ---- (j) hole verticals P→p … S→s inside the membrane. ----
    let e_pp = strut_at(body, s_pq.he_plus, 0.5, 0.5, 0.0);
    let e_qq = strut_at(body, s_qr.he_plus, 1.5, 0.5, 0.0);
    let e_rr = strut_at(body, s_rs.he_plus, 1.5, 1.5, 0.0);
    let e_ss = strut_at(body, mef_top.he_minus, 0.5, 1.5, 0.0);

    // ---- (k) the four tube walls, leaving the membrane as the square
    // p→q→r→s at the bottom level. ----
    let w_front = mef(body, e_pp.he_minus, e_qq.he_minus);
    let w_right = mef(body, e_qq.he_minus, e_rr.he_minus);
    let w_back = mef(body, e_rr.he_minus, e_ss.he_minus);
    // The last wall's he2 is the first wall's plus half p→q, still in
    // the membrane loop — addressed through the face, the way a hand
    // construction would.
    let he_pq_bottom = body
        .find_half_edge(mef_top.face, e_pp.vertex, e_qq.vertex)
        .unwrap();
    assert_eq!(
        he_pq_bottom, w_front.he_plus,
        "find_half_edge agrees with created keys"
    );
    let w_left = mef(body, e_ss.he_minus, he_pq_bottom);

    // ---- (l) the connected sum: kill the membrane into the bottom
    // face; its loop becomes the bottom face's ring. Genus 1. ----
    let plug = body.kfmrh(f_bottom.face, mef_top.face).unwrap();
    ck(body);

    HoledBox {
        seed,
        box_mevs: [e_ab, e_bc, e_cd, e_aa, e_bb, e_cc, e_dd],
        box_mefs: [f_bottom, f_front, f_right, f_back, f_left],
        strut,
        kill,
        rim_mevs: [s_pq, s_qr, s_rs],
        mef_top,
        tube_mevs: [e_pp, e_qq, e_rr, e_ss],
        tube_mefs: [w_front, w_right, w_back, w_left],
        plug,
    }
}

#[test]
fn holed_box_validates_with_minimal_counts_at_genus_one() {
    let mut body = Body::<f64>::new();
    let b = build_holed_box(&mut body);
    assert_eq!(validate(&body), Ok(()));
    // Genus 1 is a tier-2 closed solid too: the rings are attached
    // through the tube, so the shell is one component.
    assert_eq!(validate_closed(&body), Ok(()));

    // The provably minimal counts (eq. 9.5 with v=16, f=10, h=1, r=2:
    // 15 mev, 10 mef, 1 mvfs, 1 kfmrh, 1 kemr).
    assert_eq!(body.vertices().count(), 16);
    assert_eq!(body.edges().count(), 24);
    assert_eq!(body.half_edges().count(), 48); // e = 24 ⇒ 48 halves
    assert_eq!(body.faces().count(), 10);
    assert_eq!(body.loops().count(), 12); // 10 outer + 2 rings
    assert_eq!(body.shells().count(), 1);
    assert_eq!(body.solids().count(), 1);
    // Geometry policy: one point per vertex; one curve per LIVE edge
    // (the strut's curve died with it); one shared surface (the
    // membrane shared mvfs's surface, so kfmrh kept it).
    assert_eq!(body.points().count(), 16);
    assert_eq!(body.curves().count(), 24);
    assert_eq!(body.surfaces().count(), 1);
    assert_eq!(b.plug.killed_surface, None);

    // The Euler–Poincaré ledger at genus 1, by hand:
    //   v − e + f − r = 16 − 24 + 10 − 2 = 0 = 2(s − h) = 2(1 − 1).
    let (v, e, f) = (16_i64, 24_i64, 10_i64);
    let rings: usize = body.faces().map(|(_, face)| face.rings.len()).sum();
    assert_eq!(rings, 2);
    assert_eq!(v - e + f - 2, 0);

    // Exactly two faces carry rings: the top (the hole's rim ring from
    // kemr's lineage) and the bottom (the demoted membrane loop from
    // kfmrh).
    let top = body.get_face(b.seed.face).unwrap();
    assert_eq!(top.rings, vec![b.kill.ring]);
    let bottom = body.get_face(b.box_mefs[0].face).unwrap();
    assert_eq!(bottom.rings, vec![b.plug.ring]);
    assert_eq!(b.plug.ring, b.mef_top.r#loop);
    for (key, face) in body.faces() {
        if key != b.seed.face && key != b.box_mefs[0].face {
            assert!(face.rings.is_empty(), "unexpected ring on {key:?}");
        }
    }
    // The membrane face is dead; the ring loop survived it.
    assert!(body.get_face(b.mef_top.face).is_none());
    assert!(body.get_loop(b.mef_top.r#loop).is_some());

    // Every loop is a quad (outer and ring alike) and every vertex has
    // valence 3 — the rectilinear holed box's local structure.
    for (_, l) in body.loops() {
        let topo::LoopBoundary::Cycle { first } = l.boundary else {
            panic!("finished body has no empty loops");
        };
        assert_eq!(body.loop_cycle(first).unwrap().len(), 4);
    }
    for (_, vertex) in body.vertices() {
        let orbit = body.vertex_orbit(vertex.emanating.unwrap()).unwrap();
        assert_eq!(orbit.len(), 3);
    }

    // Orientation spot checks against the ratified convention.
    let starts = |he| {
        body.loop_cycle(he)
            .unwrap()
            .into_iter()
            .map(|member| body.get_half_edge(member).unwrap().start)
            .collect::<Vec<_>>()
    };
    let [s_pq, s_qr, s_rs] = &b.rim_mevs;
    // Top ring: P→S→R→Q — CLOCKWISE viewed from above (= from outside),
    // as rings must run under interior-left.
    assert_eq!(
        starts(b.mef_top.he_plus),
        vec![b.strut.vertex, s_rs.vertex, s_qr.vertex, s_pq.vertex],
        "top ring walks P → S → R → Q (CW from above)"
    );
    // Bottom ring: s→p→q→r — CCW from above = CW viewed from BELOW,
    // i.e. clockwise from outside the bottom face.
    let [e_pp, e_qq, e_rr, e_ss] = &b.tube_mevs;
    assert_eq!(
        starts(b.tube_mefs[3].he_plus),
        vec![e_ss.vertex, e_pp.vertex, e_qq.vertex, e_rr.vertex],
        "bottom ring walks s → p → q → r (CW viewed from below)"
    );
    // The top face's outer loop still walks A'→B'→C'→D' (CCW from
    // above); the strut episode left it intact.
    let [_, _, _, e_aa, e_bb, e_cc, e_dd] = &b.box_mevs;
    assert_eq!(
        starts(b.box_mefs[1].he_plus),
        vec![e_aa.vertex, e_bb.vertex, e_cc.vertex, e_dd.vertex],
        "top outer loop walks A' → B' → C' → D'"
    );
    // One tube wall in coordinates: the y = 0.5 wall's outer loop walks
    // q→p→P→Q. Hand-computed normal from consecutive edges (q→p) × (p→P)
    // = (−1,0,0) × (0,0,2) = (0,+2,0): outward normal +y — INTO the hole
    // void, away from the material at y < 0.5, per interior-left.
    assert_eq!(
        starts(b.tube_mefs[0].he_minus),
        vec![e_qq.vertex, e_pp.vertex, b.strut.vertex, s_pq.vertex],
        "the y=0.5 tube wall walks q → p → P → Q"
    );
}

#[test]
fn holed_box_provenance_covers_live_entities_and_forgets_killed_ones() {
    let mut body = Body::<f64>::new();
    let b = build_holed_box(&mut body);

    // Every live topology entity has a provenance record of the right
    // operator family.
    for (key, _) in body.vertices() {
        assert!(matches!(
            body.provenance(EntityId::Vertex(key)),
            Some(Provenance::Mvfs | Provenance::Mev { .. })
        ));
    }
    for (key, _) in body.edges() {
        assert!(matches!(
            body.provenance(EntityId::Edge(key)),
            Some(Provenance::Mev { .. } | Provenance::Mef { .. })
        ));
    }
    for (key, _) in body.half_edges() {
        assert!(matches!(
            body.provenance(EntityId::HalfEdge(key)),
            Some(Provenance::Mev { .. } | Provenance::Mef { .. })
        ));
    }
    for (key, _) in body.faces() {
        assert!(matches!(
            body.provenance(EntityId::Face(key)),
            Some(Provenance::Mvfs | Provenance::Mef { .. })
        ));
    }
    for (key, _) in body.loops() {
        assert!(matches!(
            body.provenance(EntityId::Loop(key)),
            Some(Provenance::Mvfs | Provenance::Mef { .. } | Provenance::Kemr { .. })
        ));
    }
    // The hole rim ring records its kemr birth — with the killed strut
    // halves as historical argument keys.
    assert_eq!(
        body.provenance(EntityId::Loop(b.kill.ring)),
        Some(&Provenance::Kemr {
            he1: b.strut.he_plus,
            he2: b.strut.he_minus,
        })
    );
    // The bottom ring keeps its mef birth record through kfmrh and the
    // demotion (birth records are permanent for survivors).
    assert!(matches!(
        body.provenance(EntityId::Loop(b.plug.ring)),
        Some(Provenance::Mef { .. })
    ));

    // Killed entities: dead keys AND dead provenance (no SecondaryMap
    // leaks — the named delhe hazard).
    assert!(body.get_edge(b.kill.killed_edge).is_none());
    assert!(body.get_half_edge(b.kill.killed_he_plus).is_none());
    assert!(body.get_half_edge(b.kill.killed_he_minus).is_none());
    assert!(body.get_face(b.plug.killed_face).is_none());
    assert_eq!(body.provenance(EntityId::Edge(b.kill.killed_edge)), None);
    assert_eq!(
        body.provenance(EntityId::HalfEdge(b.kill.killed_he_plus)),
        None
    );
    assert_eq!(
        body.provenance(EntityId::HalfEdge(b.kill.killed_he_minus)),
        None
    );
    assert_eq!(body.provenance(EntityId::Face(b.plug.killed_face)), None);
    // The strut's curve died with its edge (geometry hygiene).
    assert_eq!(b.kill.killed_curve, Some(b.strut.curve));
    assert!(body.get_curve_geom(b.strut.curve).is_none());
}

/// A deep, order-sensitive snapshot through the PUBLIC API only (the
/// in-crate `fixtures::deep_snapshot` is not visible here): every arena
/// entry with its full payload and provenance record.
fn snapshot(body: &Body<f64>) -> Vec<String> {
    let mut lines = Vec::new();
    for (k, e) in body.solids() {
        lines.push(format!(
            "{k:?}: {e:?} {:?}",
            body.provenance(EntityId::Solid(k))
        ));
    }
    for (k, e) in body.shells() {
        lines.push(format!(
            "{k:?}: {e:?} {:?}",
            body.provenance(EntityId::Shell(k))
        ));
    }
    for (k, e) in body.faces() {
        lines.push(format!(
            "{k:?}: {e:?} {:?}",
            body.provenance(EntityId::Face(k))
        ));
    }
    for (k, e) in body.loops() {
        lines.push(format!(
            "{k:?}: {e:?} {:?}",
            body.provenance(EntityId::Loop(k))
        ));
    }
    for (k, e) in body.half_edges() {
        lines.push(format!(
            "{k:?}: {e:?} {:?}",
            body.provenance(EntityId::HalfEdge(k))
        ));
    }
    for (k, e) in body.edges() {
        lines.push(format!(
            "{k:?}: {e:?} {:?}",
            body.provenance(EntityId::Edge(k))
        ));
    }
    for (k, e) in body.vertices() {
        lines.push(format!(
            "{k:?}: {e:?} {:?}",
            body.provenance(EntityId::Vertex(k))
        ));
    }
    for (k, e) in body.points() {
        lines.push(format!("{k:?}: {e:?}"));
    }
    for (k, e) in body.curves() {
        lines.push(format!("{k:?}: {e:?}"));
    }
    for (k, e) in body.surfaces() {
        lines.push(format!("{k:?}: {e:?}"));
    }
    lines
}

#[test]
fn holed_box_lineage_is_deterministic() {
    // D9 + lineage replay, kills included: the same construction in two
    // fresh bodies mints identical key sequences (every created- and
    // killed-keys struct compares equal key for key) and materializes
    // deeply identical bodies.
    let mut body_a = Body::<f64>::new();
    let mut body_b = Body::<f64>::new();
    let built_a = build_holed_box(&mut body_a);
    let built_b = build_holed_box(&mut body_b);
    assert_eq!(built_a, built_b);
    assert_eq!(snapshot(&body_a), snapshot(&body_b));
}
