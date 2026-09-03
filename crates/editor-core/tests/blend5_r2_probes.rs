//! **BLEND-5 R2 review probes** — self-contained rows written against
//! PR #1301's claims, at review head `50fedb7d`.
//!
//! What each row establishes:
//!
//! - `the_corpus_band_trim_census_is_the_recorded_one`: the digest
//!   suite's prose numbers (4 and 84 band trimlines, nineteen rows with
//!   none), measured mechanically — and, as a corollary, that the
//!   LADDER rim phase already drove `BandTrim` emission through
//!   `names::emit_fillet` from registered documents before this unit.
//! - `the_base_rim_hosts_its_plane_too` /
//!   `a_bore_rim_hosts_its_plane_too`: the pinned host-is-planar
//!   invariant on two MORE plane-and-curved rims of the same lantern
//!   (the base and the bore lip), so the pin does not rest on one
//!   fixture's slot order.
//! - `which_wall_hosts_a_cone_on_cone_rim`: pins WHICH physical wall
//!   the structural fallback designates on the mouth rim, so a change
//!   to the fallback is visible.
//! - `the_role_moves_when_an_edit_crosses_the_planarity_boundary`: the
//!   measured limit of the PR's recipe-covariance claim ("the role does
//!   not move under any parameter edit"): an edit that flattens the
//!   MATE wall into a plane moves the host designation to it, renaming
//!   BOTH trim edges.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

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

/// The PR's lantern, with the profile taken as a parameter so edits
/// stay one-line diffs against the reviewed fixture.
fn lantern_with(profile_pts: Vec<(f64, f64)>) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("blend5_r2_probes", Tol::witness());
    let (doc, profile) = insert(
        doc,
        Node::Profile(desc(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![profile_pts],
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

/// The reviewed fixture's profile, verbatim.
fn pr_profile() -> Vec<(f64, f64)> {
    vec![
        (0.2, 0.0),
        (1.0, 0.0),
        (0.8, 0.6),
        (0.35, 0.75),
        (0.2, 0.75),
    ]
}

/// Fillet the latitude rim at profile vertex `v` of the given lantern.
fn filleted(profile_pts: Vec<(f64, f64)>, v: u32) -> (ProfileDoc, RecipeNodeId) {
    let (doc, revolve) = lantern_with(profile_pts);
    let rim = StableName {
        kind: EntityKind::Edge,
        node: revolve,
        path: vec![RoleSeg::BandRim(ProfileVertexRef {
            loop_index: 0,
            vertex: v,
        })],
    };
    insert(
        doc,
        Node::Fillet {
            target: revolve,
            radius: len(0.05),
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

/// The support surface under a trim arc: the edge's neighbour face
/// that is not the band's torus.
fn support_surface(body: &Body<f64>, e: EdgeKey) -> Surface<f64> {
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
    let (a, b) = (surf(edge.he_plus), surf(edge.he_minus));
    match (
        matches!(a, Surface::Torus { .. }),
        matches!(b, Surface::Torus { .. }),
    ) {
        (true, false) => b,
        (false, true) => a,
        _ => panic!("a trim arc separates the band's torus from one support, got {a:?} / {b:?}"),
    }
}

/// Every `BandTrim` name in the table, with the support role it
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

/// **The digest suite's prose, measured.** The comment beside the
/// pinned digests says `die_composed` and `die_composed_tour` are the
/// only registered documents whose tables carry rim-phase roles (four
/// and eighty-four band trimlines), and that every other row has none.
/// Nothing in the tree computes those numbers; this row does.
///
/// Corollary: both documents carve LADDER rims, so the rim-trim channel
/// of `BlendNaming` reached `names::emit_fillet` from registered
/// documents before BLEND-5 — the new suite's annulus rows are the
/// first ANNULUS-path coverage, not the first rim-phase coverage.
#[test]
fn the_corpus_band_trim_census_is_the_recorded_one() {
    let got: Vec<(String, usize)> = corpus::documents()
        .iter()
        .map(|d| {
            let ev = corpus::eval::<f64>(&d.doc);
            let n = ev
                .order
                .iter()
                .filter_map(|id| ev.value(*id))
                .flat_map(|v| v.name_table.iter())
                .filter(|(n, _)| n.path.iter().any(|s| matches!(s, RoleSeg::BandTrim { .. })))
                .count();
            (d.name.to_owned(), n)
        })
        .collect();
    for (name, n) in &got {
        let want = match name.as_str() {
            "die_composed" => 4,
            "die_composed_tour" => 84,
            _ => 0,
        };
        assert_eq!(
            *n, want,
            "{name}: {n} band-trim names, the digest suite's comment says {want}"
        );
    }
    assert_eq!(got.len(), 22, "the registry the comment counts over");
}

/// The pinned invariant on the lantern's BASE rim (profile vertex 1:
/// the flat base below, the lower cone above) — a second
/// plane-and-curved rim, with the plane on the other side of the
/// profile walk than the lip fixture's.
#[test]
fn the_base_rim_hosts_its_plane_too() {
    let (doc, fillet) = filleted(pr_profile(), 1);
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

/// The pinned invariant on the lantern's bore lip (profile vertex 4:
/// the lip plane meets the bore CYLINDER) — a plane-and-curved rim
/// whose curved side is not a cone, so the invariant is not
/// accidentally about cones.
#[test]
fn a_bore_rim_hosts_its_plane_too() {
    let (doc, fillet) = filleted(pr_profile(), 4);
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

/// The apex height (along the revolve axis, y) of a cone support —
/// the lower wall's generator through (1.0, 0)–(0.8, 0.6) apexes at
/// y = 3.0; the upper wall's through (0.8, 0.6)–(0.35, 0.75) at
/// y ≈ 0.867. So `apex.y > 1` says "the LOWER wall".
fn cone_apex_y(s: &Surface<f64>) -> f64 {
    match s {
        Surface::Cone { apex, .. } => apex.y,
        other => panic!("expected a cone, got {other:?}"),
    }
}

/// **Which wall the structural fallback designates**, pinned: on the
/// cone-on-cone mouth the host is the link's own `face_a` side, and
/// this row records which physical wall that is, so a change to the
/// fallback (or to half-edge orientation at minting) is visible here
/// rather than silent.
#[test]
fn which_wall_hosts_a_cone_on_cone_rim() {
    let (doc, fillet) = filleted(pr_profile(), 2);
    let ev = run(&doc);
    let rows = trims(&ev, fillet);
    assert_eq!(rows.len(), 2, "one trimline per support");
    let apex_of = |want: RimSupport| {
        let s = rows
            .iter()
            .find_map(|(r, s)| (*r == want).then_some(s))
            .expect("both roles emitted");
        cone_apex_y(s)
    };
    let (host_apex, mate_apex) = (apex_of(RimSupport::Host), apex_of(RimSupport::Mate));
    // Measured at 50fedb7d: the host is the UPPER wall (apex
    // y ≈ 0.867), the mate the lower (apex y = 3.0).
    assert!(
        host_apex < 1.0 && mate_apex > 1.0,
        "the mouth's host wall moved: host apex y = {host_apex}, mate apex y = {mate_apex}"
    );
}

/// **The measured limit of recipe covariance.** The PR argues the role
/// "does not move under any parameter edit"; this row executes the
/// edit for which that is false. Flattening the MATE wall of the
/// cone-on-cone mouth into a PLANE (one profile parameter) makes the
/// planar-preference branch designate IT the host — so the parameter
/// edit renames BOTH trim edges, exactly the strand-the-references
/// failure the role vocabulary is argued to prevent. The vocabulary is
/// covariant across kind changes AMONG curved supports (and among
/// rims that keep a planar side planar); it is not covariant across
/// the planarity boundary, where the host designation itself reads a
/// kind.
#[test]
fn the_role_moves_when_an_edit_crosses_the_planarity_boundary() {
    // The reviewed mouth: host is the UPPER cone (the row above), the
    // LOWER cone is the mate.
    let (doc, fillet) = filleted(pr_profile(), 2);
    let ev = run(&doc);
    let before = trims(&ev, fillet);
    let host_was_upper = before
        .iter()
        .any(|(r, s)| *r == RimSupport::Host && cone_apex_y(s) < 1.0);
    assert!(host_was_upper, "the mouth's host is the upper wall");

    // One parameter edit: vertex 1 moves so the LOWER wall — the MATE
    // — becomes a horizontal PLANE ring at the mouth's own height. The
    // upper wall and the mouth vertex are untouched; the rim is still
    // recipe vertex 2, and the rim stays convex.
    let edited = vec![
        (0.2, 0.0),
        (0.4, 0.6),
        (0.8, 0.6),
        (0.35, 0.75),
        (0.2, 0.75),
    ];
    let (doc, fillet) = filleted(edited, 2);
    let ev = run(&doc);
    let after = trims(&ev, fillet);
    assert_eq!(after.len(), 2, "one trimline per support");
    for (support, surface) in &after {
        let planar = matches!(surface, Surface::Plane { .. });
        assert_eq!(
            planar,
            *support == RimSupport::Host,
            "{support:?} lies on {surface:?}"
        );
    }
    // The wall that was the MATE before the edit — the lower one, the
    // one the edit flattened — is the HOST after it, and the upper
    // wall, host before, is now the mate: the roles crossed sides
    // under a parameter edit.
    let host_is_the_flattened_lower_wall = after
        .iter()
        .any(|(r, s)| *r == RimSupport::Host && matches!(s, Surface::Plane { .. }));
    let mate_is_the_upper_wall = after
        .iter()
        .any(|(r, s)| *r == RimSupport::Mate && cone_apex_y(s) < 1.0);
    assert!(host_is_the_flattened_lower_wall && mate_is_the_upper_wall);
}
