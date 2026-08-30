//! **BLEND-5 R1 review probes** — three measurements the unit's own
//! suites do not take, each written to go RED if the claim it probes
//! stops holding.
//!
//! 1. The per-document `BandTrim` counts the PR records as an
//!    invariant (`die_composed` = 4, `die_composed_tour` = 84, the
//!    other nineteen registered documents = 0) are recorded only as
//!    PROSE beside the digest pins. Nothing computes with them, so
//!    nothing goes red when they stop being true. This row executes
//!    them.
//!
//! 2. That same count contradicts `blend5_rim_support.rs`'s module
//!    doc, which says the rim-phase channels of `FilletNaming` reach
//!    `names::emit_fillet` "only from here". `die_composed` is a
//!    registered corpus document that already drove `rim_phase` — the
//!    LADDER arm, which writes the same `rim_trims` / `rim_feet` /
//!    `slits` channels — through the emitter before this unit existed.
//!    The narrower claim (the ANNULUS arm) is the true one.
//!
//! 3. The role is claimed recipe-covariant: "the role does not move
//!    under any parameter edit". It does. `Host` is DEFINED as the
//!    planar support wherever the rim has one, so a parameter edit
//!    that makes a curved support planar can hand the host role to the
//!    other side of the same rim — renaming both trim arcs and
//!    stranding exactly the downstream references the role was chosen
//!    to protect. This row measures that flip on a rim whose stable
//!    name never moves.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use editor_core::{
    CancelToken, Datum, EntityKey, EntityKind, Entry, EvalOptions, Evaluation, NameTable, Node,
    ProfileDoc, ProfileVertexRef, RecipeNodeId, RimSupport, RoleSeg, StableName, evaluate,
};
use fixture::{ang, desc, insert, len, scl};
use geom_core::Tol;
use topo::{Body, EdgeKey};

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// Every `BandTrim` name in every value of an evaluation.
fn band_trims(ev: &Evaluation<f64>) -> usize {
    ev.order
        .iter()
        .filter_map(|id| ev.value(*id))
        .map(|v| {
            v.name_table
                .iter()
                .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::BandTrim { .. })))
                .count()
        })
        .sum()
}

/// **Probe 1 — the recorded count invariant, executed.** The PR states
/// "exactly two corpus documents carry band-trim names (4 and 84), the
/// other nineteen byte-identical" and records it beside the digest
/// pins as prose. The digests guard the NAMES; nothing guards the
/// COUNT, so a document added to the registry that carves a closed rim
/// would move a third number with only a doc comment to notice. This
/// row goes red when it does.
#[test]
fn the_recorded_band_trim_counts_are_executable() {
    let got: Vec<(String, usize)> = corpus::documents()
        .iter()
        .map(|d| {
            let ev = corpus::eval::<f64>(&d.doc);
            (d.name.to_owned(), band_trims(&ev))
        })
        .collect();
    let nonzero: Vec<(String, usize)> = got.iter().filter(|(_, c)| *c > 0).cloned().collect();
    assert_eq!(
        nonzero,
        vec![
            ("die_composed".to_owned(), 4),
            ("die_composed_tour".to_owned(), 84),
        ],
        "the rim-vocabulary count invariant moved; the whole tally is {got:?}"
    );
    assert_eq!(
        got.len() - nonzero.len(),
        19,
        "nineteen registered documents carry no band trimline at all: {got:?}"
    );
}

/// **Probe 2 — the "only from here" premise is false.** The new
/// suite's module doc claims the rim-phase channels reach
/// `names::emit_fillet` only from `blend5_rim_support.rs`. A
/// registered corpus document already drives them: `die_composed`'s
/// pip-rim fillet is a CLOSED chain resolved onto the ladder arm,
/// which writes `rim_trims` (and `rim_feet`, `meridian_splits`,
/// `meridian_remnants`, `slits`) into the same record the emitter
/// reads. Only the ANNULUS arm was previously unreached from
/// editor-core.
#[test]
fn the_ladder_rim_phase_already_reached_the_emitter_from_the_corpus() {
    let d = corpus::documents()
        .into_iter()
        .find(|d| d.name == "die_composed")
        .expect("die_composed is registered");
    let ev = corpus::eval::<f64>(&d.doc);
    assert!(
        band_trims(&ev) > 0,
        "die_composed drives the rim phase through emit_fillet, so the rim-phase \
         channels did not first reach the emitter from blend5_rim_support.rs"
    );
}

// ---- Probe 3: the role is not covariant under every parameter edit ----

/// A stepped lantern, annular so every latitude rim is closed. Only
/// two profile vertices move between the variants below; the loop's
/// vertex COUNT and the mouth's index never do, so
/// `RoleSeg::BandRim(vertex 3)` names the same rim in all three — the
/// stable name is exactly what a parameter edit is supposed to
/// preserve.
///
/// The mouth rim's two supports are the wall BELOW it (vertex 2 →
/// `mouth`) and the wall ABOVE it (`mouth` → `top`).
fn lantern(mouth: (f64, f64), top: (f64, f64)) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("blend5_r1_probe_lantern", Tol::witness());
    let (doc, profile) = insert(
        doc,
        Node::Profile(desc(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![
                (0.2, 0.0),
                (0.6, 0.0),
                (0.6, 0.3),
                mouth,
                top,
                (0.35, 1.2),
                (0.2, 1.2),
            ]],
        )),
    );
    let (doc, axis) = insert(
        doc,
        Node::Datum(Datum::Axis {
            origin: [len(0.0), len(0.0), len(0.0)],
            direction: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    );
    let (doc, revolve) = insert(
        doc,
        Node::Revolve {
            profile,
            axis,
            angle: ang(std::f64::consts::TAU),
        },
    );
    (doc, revolve)
}

/// The mouth rim filleted, and the fillet node's id.
fn filleted(mouth: (f64, f64), top: (f64, f64)) -> (ProfileDoc, RecipeNodeId) {
    let (doc, revolve) = lantern(mouth, top);
    let rim = StableName {
        kind: EntityKind::Edge,
        node: revolve,
        path: vec![RoleSeg::BandRim(ProfileVertexRef {
            loop_index: 0,
            vertex: 3,
        })],
    };
    insert(
        doc,
        Node::Fillet {
            target: revolve,
            radius: len(0.04),
            selection: vec![rim],
        },
    )
}

fn table(ev: &Evaluation<f64>, id: RecipeNodeId) -> &NameTable {
    &ev.value(id)
        .unwrap_or_else(|| panic!("node {id:?} has no value: {:?}", ev.nodes.get(&id)))
        .name_table
}

fn edge_key(t: &NameTable, n: &StableName) -> EdgeKey {
    match t.lookup(n) {
        Some(Entry::Unique(r)) => match r.key {
            EntityKey::Edge(k) => k,
            other => panic!("{n:?} names {other:?}, not an edge"),
        },
        other => panic!("{n:?} is not uniquely named: {other:?}"),
    }
}

/// The height along the revolve axis (`y`) at which a trim arc sits.
/// Each trim arc is a latitude circle, so one of its vertices fixes
/// it.
fn arc_height(body: &Body<f64>, e: EdgeKey) -> f64 {
    let edge = body.get_edge(e).expect("a live edge");
    let he = body.get_half_edge(edge.he_plus).expect("a live half-edge");
    let v = body.get_vertex(he.start).expect("a live vertex");
    body.get_point(v.point).expect("a live point").y
}

/// The role carried by the trim arc on the wall BELOW the mouth.
fn role_below_the_mouth(mouth: (f64, f64), top: (f64, f64)) -> RimSupport {
    let (doc, fillet) = filleted(mouth, top);
    let ev = run(&doc);
    let t = table(&ev, fillet);
    let body = corpus::body_of(&ev, fillet);
    let mut rows: Vec<(f64, RimSupport)> = t
        .iter()
        .filter_map(|(n, _)| match n.path.first() {
            Some(RoleSeg::BandTrim { support, .. }) => {
                Some((arc_height(body, edge_key(t, n)), *support))
            }
            _ => None,
        })
        .collect();
    assert_eq!(rows.len(), 2, "one trimline per support: {rows:?}");
    rows.sort_by(|a, b| a.0.partial_cmp(&b.0).expect("finite heights"));
    // The lower arc lies on the wall below the mouth. (Where that wall
    // is a PLANE the arc lies in it, so the bound is not strict.)
    assert!(
        rows[0].0 <= mouth.1 && rows[1].0 >= mouth.1,
        "the arcs sit on the two walls of the mouth corner at y={}: {rows:?}",
        mouth.1
    );
    rows[0].1
}

/// **Probe 3 — the role MOVES under a parameter edit.** The PR's third
/// reason for choosing roles over kinds is that "the role does not
/// move under any parameter edit", where a kind "renames the trim edge
/// and strands every downstream reference to it".
///
/// `Host` is not a pure slot. It is DEFINED as the planar support
/// wherever the rim has one, and falls back to the link's own `face_a`
/// only when NEITHER side is planar — `sweep/src/fillet/surgery.rs`
/// decides it that way at all three sites (the one-link arm's
/// `is_plane(link0.face_a)` test, the ladder discriminant, and
/// `resolve_seam_split_rim`'s `is_plane_surface`). So an edit that
/// carries either support across planarity re-decides the role of BOTH
/// arcs.
///
/// Three variants of one recipe, differing in two profile vertices:
///
/// * `CONE_CONE` — both walls conical; the role is the link's slot.
/// * `PLANE_BELOW` — the lower wall is horizontal, so it is the host;
///   the UPPER wall is geometrically untouched.
/// * `PLANE_ABOVE` — the upper wall is horizontal, so it is the host;
///   the LOWER wall is geometrically untouched.
///
/// Measured: the lower wall's arc is `Mate` on the cone-on-cone mouth
/// and `Mate` when the UPPER wall is flattened, but `Host` when the
/// LOWER wall is flattened. So the role of a rim's two arcs is decided
/// by the PLANARITY of the supports, and a parameter edit that carries
/// a support across planarity swaps which arc `Host` addresses — while
/// `BandRim(vertex 3)` and the fillet's selection stay word-for-word
/// the same.
///
/// That is strictly worse than the kind vocabulary it replaces for
/// this failure mode, not better: a kind rename makes a stored
/// selection STOP RESOLVING (loud), whereas the role SILENTLY
/// retargets it to the other arc of the same rim.
///
/// This row is green on the reviewed tree and goes red if the roles
/// are ever made independent of the supports' kinds.
#[test]
fn the_host_role_moves_when_a_parameter_edit_makes_a_support_planar() {
    let cone_cone = role_below_the_mouth((1.0, 0.6), (0.8, 0.9));
    let plane_below = role_below_the_mouth((1.0, 0.3), (0.8, 0.9));
    let plane_above = role_below_the_mouth((1.0, 0.6), (0.8, 0.6));
    assert_ne!(
        plane_below, plane_above,
        "the lower wall's trim arc takes the SAME role ({plane_below:?}) whichever \
         support is made planar, so the role would be independent of the supports' \
         kinds after all"
    );
    assert_eq!(
        (cone_cone, plane_below, plane_above),
        (RimSupport::Mate, RimSupport::Host, RimSupport::Mate),
        "the measured roles of the lower wall's arc across the three variants"
    );
}
