//! **A closed rim's two trim arcs, named at the recipe layer** — the
//! annulus surgery's rim phase driven end to end through the public
//! doors: a revolve, a `Node::Fillet` naming one latitude rim, and the
//! emitted [`RoleSeg::BandTrim`] segments read off the node's name
//! table.
//!
//! Every other fillet-naming suite in this crate carves an OPEN chain
//! (the die's ladder edges), so the rim-phase channels of
//! `sweep::fillet::naming::FilletNaming` — `bands`, `rim_trims`,
//! `rim_feet`, `meridian_splits`, `meridian_remnants`, `slits` — reach
//! `names::emit_fillet` only from here.
//!
//! # What the fixture is for
//!
//! The lantern's mouth is a latitude rim between two CONES. The
//! surgery has no planar side to prefer there, so it designates one
//! structurally — the link's own `face_a` — and the persisted support
//! vocabulary must be able to SAY that. These rows pin the emitted
//! vocabulary against the surface each trim arc actually lies on, so a
//! variant naming a KIND the geometry contradicts fails here.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use editor_core::{
    CancelToken, Datum, EntityKey, EntityKind, Entry, EvalOptions, Evaluation, NameTable, Node,
    ProfileDoc, ProfileVertexRef, RecipeNodeId, RimSupport, RoleSeg, StableName, evaluate,
};
use fixture::{ang, desc, insert, len, scl};
use geom::Surface;
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

/// The mouth rim's profile vertex: the corner where the two conical
/// walls meet.
const MOUTH: u32 = 2;

/// **The lantern**, annular so that every latitude rim is CLOSED: a
/// bore at `0.2`, a flat base annulus, two conical walls of different
/// slope meeting at the mouth `(0.8, 0.6)`, and a lip disk closing it.
///
/// The mouth is the ARMS-2 curved-on-curved case with the strongest
/// property for a naming vocabulary: the two supports have the SAME
/// surface kind, so a kind cannot tell them apart at all.
fn lantern() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("blend5_rim_support", Tol::witness());
    let (doc, profile) = insert(
        doc,
        Node::Profile(desc(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![
                (0.2, 0.0),
                (1.0, 0.0),
                (0.8, 0.6),
                (0.35, 0.75),
                (0.2, 0.75),
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

/// The lantern with its mouth rim filleted, and the fillet node's id.
fn filleted_mouth() -> (ProfileDoc, RecipeNodeId) {
    let (doc, revolve) = lantern();
    let mouth = StableName {
        kind: EntityKind::Edge,
        node: revolve,
        path: vec![RoleSeg::BandRim(ProfileVertexRef {
            loop_index: 0,
            vertex: MOUTH,
        })],
    };
    insert(
        doc,
        Node::Fillet {
            target: revolve,
            radius: len(0.05),
            selection: vec![mouth],
        },
    )
}

fn table(ev: &Evaluation<f64>, id: RecipeNodeId) -> &NameTable {
    &ev.value(id)
        .unwrap_or_else(|| panic!("node {id:?} has no value: {:?}", ev.nodes.get(&id)))
        .name_table
}

/// The edge key a uniquely-named edge answers to.
fn edge_key(t: &NameTable, n: &StableName) -> EdgeKey {
    match t.lookup(n) {
        Some(Entry::Unique(r)) => match r.key {
            EntityKey::Edge(k) => k,
            other => panic!("{n:?} names {other:?}, not an edge"),
        },
        other => panic!("{n:?} is not uniquely named: {other:?}"),
    }
}

/// The surfaces of the two faces a trim edge separates: the band's
/// torus, and the SUPPORT the arc lies on.
fn neighbours(body: &Body<f64>, e: EdgeKey) -> (Surface<f64>, Surface<f64>) {
    let edge = body.get_edge(e).expect("a live edge");
    let surf = |he| {
        let l = body
            .get_half_edge(he)
            .expect("a live half-edge")
            .parent_loop;
        let f = body.get_loop(l).expect("a live loop").face;
        body.get_surface(body.get_face(f).expect("a live face").surface)
            .expect("a carrier")
            .clone()
    };
    (surf(edge.he_plus), surf(edge.he_minus))
}

/// The support surface under a trim arc: the neighbour that is not the
/// band's torus.
fn support_surface(body: &Body<f64>, e: EdgeKey) -> Surface<f64> {
    let (a, b) = neighbours(body, e);
    match (
        matches!(a, Surface::Torus { .. }),
        matches!(b, Surface::Torus { .. }),
    ) {
        (true, false) => b,
        (false, true) => a,
        _ => panic!("a trim arc separates the band's torus from one support, got {a:?} / {b:?}"),
    }
}

/// Every `BandTrim` name in the table, with the support variant it
/// carries and the surface its edge actually lies on.
fn trims(ev: &Evaluation<f64>, id: RecipeNodeId) -> Vec<(RimSupport, Surface<f64>)> {
    let t = table(ev, id);
    let body = corpus::body_of(ev, id);
    t.iter()
        .filter_map(|(n, _)| match n.path.first() {
            Some(RoleSeg::BandTrim { support, .. }) => {
                Some((*support, support_surface(body, edge_key(t, n))))
            }
            _ => None,
        })
        .collect()
}

/// **The rim phase reaches the emitter at all** — the annulus twin of
/// the die's open-chain rows. `check_total` runs inside `name_fillet`,
/// so a green table is already the statement that every entity the
/// annulus surgery minted or spared is named.
#[test]
fn a_closed_rim_carve_names_its_whole_output() {
    let (doc, fillet) = filleted_mouth();
    let ev = run(&doc);
    let t = table(&ev, fillet);
    let count = |f: fn(&RoleSeg) -> bool| t.iter().filter(|(n, _)| f(&n.path[0])).count();
    assert_eq!(
        count(|s| matches!(s, RoleSeg::BandFace(_))),
        1,
        "one band face rounds the rim"
    );
    assert_eq!(
        count(|s| matches!(s, RoleSeg::BandTrim { .. })),
        2,
        "one trimline per support"
    );
    assert_eq!(
        count(|s| matches!(s, RoleSeg::BandSlit(_))),
        1,
        "the band's slit keeps it ring-free"
    );
}

/// **The headline row.** Both of the mouth's supports are CONES, so no
/// name here can report a support's KIND: one of the two would have to
/// contradict the surface under it, and reporting the kind FAITHFULLY
/// would give the two arcs the same name. The vocabulary reports the
/// carve's two roles, which the surgery knows and the geometry cannot
/// take away.
#[test]
fn a_cone_on_cone_rim_names_its_supports_by_role() {
    let (doc, fillet) = filleted_mouth();
    let ev = run(&doc);
    let rows = trims(&ev, fillet);
    assert_eq!(rows.len(), 2, "one trimline per support");
    for (support, surface) in &rows {
        assert!(
            matches!(surface, Surface::Cone { .. }),
            "the mouth's supports are both cones; {support:?} lies on {surface:?}"
        );
    }
    let mut got: Vec<RimSupport> = rows.iter().map(|(s, _)| *s).collect();
    got.sort();
    assert_eq!(
        got,
        vec![RimSupport::Host, RimSupport::Mate],
        "the two arcs take the two roles of the carve"
    );
}

/// **The invariant that makes the re-spelling meaning-preserving**: on
/// a rim that HAS a planar support, the host is that support. A
/// selection that meant "the flat side" under the retired kind
/// vocabulary means it still.
#[test]
fn the_host_is_the_planar_support_wherever_the_rim_has_one() {
    let (doc, revolve) = lantern();
    // The lip disk meets the upper cone at profile vertex 3: a
    // plane–cone rim, one planar side.
    let lip = StableName {
        kind: EntityKind::Edge,
        node: revolve,
        path: vec![RoleSeg::BandRim(ProfileVertexRef {
            loop_index: 0,
            vertex: 3,
        })],
    };
    let (doc, fillet) = insert(
        doc,
        Node::Fillet {
            target: revolve,
            radius: len(0.05),
            selection: vec![lip],
        },
    );
    let ev = run(&doc);
    let rows = trims(&ev, fillet);
    assert_eq!(rows.len(), 2, "one trimline per support");
    for (support, surface) in &rows {
        let planar = matches!(surface, Surface::Plane { .. });
        assert_eq!(
            planar,
            *support == RimSupport::Host,
            "{support:?} lies on {surface:?}"
        );
    }
}
