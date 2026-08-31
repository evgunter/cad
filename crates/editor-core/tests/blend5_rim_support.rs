//! **A closed rim's two trim arcs, named at the recipe layer** — the
//! annulus surgery's rim phase driven end to end through the public
//! doors: a revolve, a `Node::Fillet` naming one latitude rim, and the
//! emitted [`RoleSeg::BandTrim`] segments read off the node's name
//! table.
//!
//! **What is first here is the ANNULUS, not the rim phase.** The rim
//! phase already reached `names::emit_fillet` from the registry: the
//! corpus's `die_composed` and `die_composed_tour` carve LADDER rims
//! (their pip cavities), which is why those two documents' name-table
//! digests are the ones the rim vocabulary moves — pinned by
//! `blend5_r1_probes::the_ladder_rim_phase_already_reached_the_emitter_from_the_corpus`
//! and by the census in `blend5_r2_probes`. No registered document
//! carves an ANNULUS rim, so this suite is the first to drive that
//! surgery's output through the emitter, which is what issue #1294
//! asks for.
//!
//! Of the rim-phase channels of `sweep::fillet::naming::BlendNaming`,
//! these rows ASSERT on three — `bands`, `rim_trims` and `slits`. The
//! other three (`rim_feet`, `meridian_splits`, `meridian_remnants`)
//! ride `check_total`: the emitter refuses a table that does not name
//! every output entity, so reaching a green table at all is the
//! statement that they emitted, but no row here reads their names.
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
use topo::{Body, EdgeKey, SurfaceKey};

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
///
/// **Two things this row does NOT establish, both measured by the
/// review probes rather than here.** Its `iff` over-asserts in one
/// direction: the mate-is-never-planar half is UNREACHABLE, because a
/// plane-on-plane rim is refused upstream as one surface, so no
/// fixture can make that half fail. And the row is fixture-lucky —
/// with the planar preference deleted from `second_support_is_host`
/// it stays GREEN, because this rim's `face_a` happens to be the
/// plane. `blend5_r2_probes::{the_base_rim_hosts_its_plane_too,
/// a_bore_rim_hosts_its_plane_too}` are the rows that go red under
/// that mutation (opposite slot order, and a non-cone curved side);
/// they are the real guard and this row is the readable statement.
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

/// The SURFACE KEY of the support a trim arc lies on — identity, not
/// shape, so two arcs of one seam-split rim can be asked whether they
/// sit on the SAME support face's surface.
fn support_surface_key(body: &Body<f64>, e: EdgeKey) -> SurfaceKey {
    let edge = body.get_edge(e).expect("a live edge");
    let of = |he| {
        let l = body
            .get_half_edge(he)
            .expect("a live half-edge")
            .parent_loop;
        let f = body.get_loop(l).expect("a live loop").face;
        body.get_face(f).expect("a live face").surface
    };
    let (a, b) = (of(edge.he_plus), of(edge.he_minus));
    let torus = |k: SurfaceKey| {
        matches!(
            body.get_surface(k).expect("a carrier"),
            Surface::Torus { .. }
        )
    };
    match (torus(a), torus(b)) {
        (true, false) => b,
        (false, true) => a,
        _ => panic!("a trim arc separates the band's torus from one support"),
    }
}

/// **The seam-split rim: several arcs, ONE pair of roles.** A
/// profile touching the axis at BOTH ends splits every wall into
/// half-bands, so this rim reaches the surgery as TWO arcs meeting at
/// chart-seam vertices and must be requested whole (the ARMS-3
/// recourse). It is carved by `resolve_seam_split_rim`, the THIRD site
/// that decides the host — the one the other rows here do not reach.
///
/// **The rim is chosen so its two links DISAGREE on slot order**,
/// which is what gives this row its discrimination. That site picks a
/// host SURFACE once for the whole chain and then re-tests, per link,
/// which slot carries it, precisely because the links need not agree.
/// On this rim they do not: the base disk is `face_a` of one link and
/// `face_b` of the other. So the claim is not "each arc has two roles"
/// (trivially true) but that the two arcs AGREE ON GEOMETRY — both
/// `Host` trims on one support surface, both `Mate` trims on the
/// other.
///
/// Verified by mutation, both ways, because the first draft of this
/// row used a rim whose links happened to agree and discriminated
/// nothing:
///
/// * delete the per-link slot re-test (`a_is_host = true` always) and
///   this row goes RED — on the earlier fixture all 21 rows stayed
///   green;
/// * delete the planar preference from
///   `naming::second_support_is_host` and it goes RED too, because
///   this rim HAS a planar support (the base disk) and the surgery
///   must prefer it. That is the seam-split arm's own spelling of the
///   host rule, which no other row here reaches.
#[test]
fn a_seam_split_rim_gives_all_its_arcs_one_pair_of_roles() {
    let doc = ProfileDoc::empty_derived("blend5_seam_split", Tol::witness());
    // Both ends on the axis, so every wall is a pair of half-bands and
    // every rim a pair of arcs (BLEND-1's lantern shape).
    let (doc, profile) = insert(
        doc,
        Node::Profile(desc(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![
                (0.0, 0.0),
                (1.0, 0.0),
                (0.8, 0.6),
                (0.2, 1.2),
                (0.0, 1.2),
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
    let arc = |seg: RoleSeg| StableName {
        kind: EntityKind::Edge,
        node: revolve,
        path: vec![seg],
    };
    // The BASE rim (disk meets the lower cone): the one whose two
    // links disagree on slot order, and which has a planar support.
    let pv = ProfileVertexRef {
        loop_index: 0,
        vertex: 1,
    };
    // The rim WHOLE: both of its arcs, which is the only request the
    // surgery accepts here (one alone terminates at a seam vertex).
    let mut selection = vec![arc(RoleSeg::BandRim(pv)), arc(RoleSeg::BandRimPi(pv))];
    selection.sort();
    let (doc, fillet) = insert(
        doc,
        Node::Fillet {
            target: revolve,
            radius: len(0.05),
            selection,
        },
    );
    let ev = run(&doc);
    let t = table(&ev, fillet);
    let body = corpus::body_of(&ev, fillet);
    let mut by_role: Vec<(RimSupport, SurfaceKey)> = t
        .iter()
        .filter_map(|(n, _)| match n.path.first() {
            Some(RoleSeg::BandTrim { support, .. }) => {
                Some((*support, support_surface_key(body, edge_key(t, n))))
            }
            _ => None,
        })
        .collect();
    by_role.sort();
    assert_eq!(
        by_role.len(),
        4,
        "two arcs, one trimline per support each: {by_role:?}"
    );
    let hosts: Vec<SurfaceKey> = by_role
        .iter()
        .filter(|(r, _)| *r == RimSupport::Host)
        .map(|(_, k)| *k)
        .collect();
    let mates: Vec<SurfaceKey> = by_role
        .iter()
        .filter(|(r, _)| *r == RimSupport::Mate)
        .map(|(_, k)| *k)
        .collect();
    assert_eq!(hosts.len(), 2, "one host trim per arc: {by_role:?}");
    assert_eq!(mates.len(), 2, "one mate trim per arc: {by_role:?}");
    assert_eq!(
        hosts[0], hosts[1],
        "both arcs' HOST trims lie on one support surface: {by_role:?}"
    );
    assert_eq!(
        mates[0], mates[1],
        "both arcs' MATE trims lie on one support surface: {by_role:?}"
    );
    assert_ne!(hosts[0], mates[0], "the two roles are two supports");
    // This rim HAS a planar support, so the host rule's third spelling
    // must prefer it — the half the slot re-test alone cannot give.
    assert!(
        matches!(
            body.get_surface(hosts[0]).expect("a carrier"),
            Surface::Plane { .. }
        ),
        "the host is the base disk"
    );
    assert!(
        matches!(
            body.get_surface(mates[0]).expect("a carrier"),
            Surface::Cone { .. }
        ),
        "the mate is the cone"
    );
}
