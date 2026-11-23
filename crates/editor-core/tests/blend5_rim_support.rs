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
//! Of the rim-phase channels of `sweep::blend::naming::BlendNaming`,
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

use crate::corpus;
use crate::fixture;

use editor_core::{
    CancelToken, EvalOptions, Evaluation, Node, ProfileDoc, RecipeNodeId, RimSupport, RoleSeg,
    evaluate,
};
use fixture::{ang, edge_of, insert, len, on_frame, revolve_about, table};
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
    let (doc, profile) = on_frame(
        doc,
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
    );
    let (doc, revolve) = insert(
        doc,
        revolve_about(
            profile,
            (0.0, 0.0),
            crate::fixture::along_y(),
            ang(std::f64::consts::TAU),
        ),
    );
    (doc, revolve)
}

/// The lantern with its mouth rim filleted, and the fillet node's id.
fn filleted_mouth() -> (ProfileDoc, RecipeNodeId) {
    let (doc, revolve) = lantern();
    let mouth = editor_core::band_rim(revolve, fixture::vpiece(&doc, revolve, 0, MOUTH as usize));
    insert(
        doc,
        Node::Fillet {
            radius: len(0.05),
            selection: editor_core::Operand::select(
                editor_core::Operand::output(revolve, 0),
                vec![mouth],
            ),
        },
    )
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
        body.face_surface(f).expect("a carrier").clone()
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
            Some(RoleSeg::BandTrim { support, .. }) => Some((
                *support,
                support_surface(&body, edge_of(t, "a trim arc", n)),
            )),
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
    assert_eq!(
        fixture::count(t, |s| matches!(s, RoleSeg::BandFace(_))),
        1,
        "one band face rounds the rim"
    );
    assert_eq!(
        fixture::count(t, |s| matches!(s, RoleSeg::BandTrim { .. })),
        2,
        "one trimline per support"
    );
    assert_eq!(
        fixture::count(t, |s| matches!(s, RoleSeg::BandSlit { .. })),
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
    let lip = editor_core::band_rim(revolve, fixture::vpiece(&doc, revolve, 0, 3));
    let (doc, fillet) = insert(
        doc,
        Node::Fillet {
            radius: len(0.05),
            selection: editor_core::Operand::select(
                editor_core::Operand::output(revolve, 0),
                vec![lip],
            ),
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

/// **The pole-to-pole revolve's base rim: ONE pair of roles.** A
/// profile touching the axis at BOTH ends sweeps every wall as one face
/// and every rim as one closed edge, so this rim reaches the surgery
/// whole: one trimline per support, `Host` on one surface and `Mate` on
/// the other.
///
/// **The rim has a planar support** (the base disk), and the surgery
/// must prefer it: delete the planar preference from
/// `naming::second_support_is_host` and this row goes RED. That is the
/// host rule's spelling no other row here reaches.
#[test]
fn a_seam_split_rim_gives_all_its_arcs_one_pair_of_roles() {
    let doc = ProfileDoc::empty_derived("blend5_seam_split", Tol::witness());
    // Both ends on the axis (BLEND-1's lantern shape): every wall is one
    // face and every rim one closed edge. The seam-split rim this row
    // was written for (two arcs whose links disagree on slot order) has
    // no revolve fixture now
    // (`work/band/the-seam-split-rim-arms-have-no-native-fixture-once-a-full-revolve-is-one-band.md`);
    // the host rule's planar preference is what it still discriminates.
    let (doc, profile) = on_frame(
        doc,
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
    );
    let (doc, revolve) = insert(
        doc,
        revolve_about(
            profile,
            (0.0, 0.0),
            crate::fixture::along_y(),
            ang(std::f64::consts::TAU),
        ),
    );
    // The BASE rim (disk meets the lower cone): the one with a planar
    // support.
    let pv = crate::fixture::vpiece(&doc, revolve, 0, 1);
    // The rim WHOLE: one full-period circle on the one-band revolve.
    let selection = vec![editor_core::band_rim(revolve, pv)];
    let (doc, fillet) = insert(
        doc,
        Node::Fillet {
            radius: len(0.05),
            selection: editor_core::Operand::select(
                editor_core::Operand::output(revolve, 0),
                selection,
            ),
        },
    );
    let ev = run(&doc);
    let t = table(&ev, fillet);
    let body = corpus::body_of(&ev, fillet);
    // A trimline is named for its (arc, support) whether it stands
    // alone or the blend's closing join made it one edge with the next
    // arc's (`docs/DESIGN.md`, maximal edges): then the joined edge is
    // the set of their names, and each member still reads its support.
    let mut by_role: Vec<(RimSupport, SurfaceKey)> = t
        .iter()
        .flat_map(|(n, _)| {
            let members: Vec<&editor_core::StableName> = match n.path.first() {
                Some(RoleSeg::Merged(cs)) => cs.iter().collect(),
                _ => vec![n],
            };
            members
                .into_iter()
                .filter_map(|m| match m.path.first() {
                    Some(RoleSeg::BandTrim { support, .. }) => Some((
                        *support,
                        support_surface_key(&body, edge_of(t, "a trim arc", n)),
                    )),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect();
    by_role.sort();
    assert_eq!(
        by_role.len(),
        2,
        "one closed rim, one trimline per support: {by_role:?}"
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
    assert_eq!(hosts.len(), 1, "one host trim: {by_role:?}");
    assert_eq!(mates.len(), 1, "one mate trim: {by_role:?}");
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
