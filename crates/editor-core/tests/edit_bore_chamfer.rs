//! **A hole's rims chamfered by ONE `Node::Chamfer`.** Each rim is a
//! circle where the hole's cylinder meets a cap square to its axis, so
//! its band is a cone frustum carved by the fillet's closed-rim walks,
//! and its births reach the document through the fillet's ring roles: a
//! `BandFace` per rim, a `BandFoot` and a `BandSlit` per band, a
//! `BandCross` per wall seam per band. Every name is unique, and a
//! replay — the document evaluated again, over its prior, and after an
//! edit of the setback — names the same entities the same way.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    EntityKey, Entry, EvalOptions, Evaluation, LoopProgram, NameTable, Node, NodeResult,
    ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg, StableName,
};

use crate::corpus;
use crate::fixture;
use fixture::{count, len, table, tol};
use topo::{Body, EdgeKey};

/// The names of `body`'s circle edges, read back through the node's
/// table: those at station `z`, or every edge when `z` is `None`.
fn edge_names(t: &NameTable, body: &Body<f64>, z: Option<f64>) -> Vec<StableName> {
    let keys: Vec<EdgeKey> = body
        .edges()
        .filter(|(_, e)| {
            let Some(z) = z else { return true };
            matches!(
                body.get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .map(|c| c.carrier()),
                Some(geom::Curve3::Circle(geom::Circle { frame: geom::Frame { origin: center, .. }, .. })) if (center.z - z).abs() < 1e-9
            )
        })
        .map(|(k, _)| k)
        .collect();
    let mut names: Vec<StableName> = t
        .iter()
        .filter_map(|(n, entry)| match entry {
            Entry::Unique(r) => match r.key {
                EntityKey::Edge(k) if keys.contains(&k) => Some(n.clone()),
                _ => None,
            },
            Entry::Tied(_) => None,
        })
        .collect();
    names.sort();
    assert_eq!(
        names.len(),
        keys.len(),
        "each selected edge is uniquely named"
    );
    names
}

/// A 2 × 2 plate one unit thick with a hole of radius 0.3 through its
/// middle, and a chamfer at `s` of the hole's two rims, or of every edge.
fn chamfered(s: f64, all: bool) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("edit_bore_chamfer", tol());
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![
                LoopProgram::polygon(fixture::square(0.0, 0.0, 1.0)).expect("a square"),
                LoopProgram::circle(0.0, 0.0, 0.3).expect("a finite hole"),
            ],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, plate) = fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let ev = fixture::run(&doc, &EvalOptions::default());
    let (t, body) = (table(&ev, plate), corpus::body_of(&ev, plate));
    let mut selection = if all {
        edge_names(t, &body, None)
    } else {
        [
            edge_names(t, &body, Some(0.0)),
            edge_names(t, &body, Some(1.0)),
        ]
        .concat()
    };
    selection.sort();
    fixture::insert(doc, Node::chamfer(plate, len(s), selection))
}

/// The chamfer's table, after asserting it evaluated and names every
/// entity uniquely: no tie, and no two names for one entity.
fn named(ev: &Evaluation<f64>, node: RecipeNodeId, what: &str) -> Vec<StableName> {
    assert!(
        matches!(ev.nodes.get(&node), Some(NodeResult::Ok(_))),
        "{what}: the chamfer evaluates, got {:?}",
        ev.nodes.get(&node)
    );
    let t = table(ev, node);
    let mut entities = Vec::new();
    for (n, entry) in t.iter() {
        match entry {
            Entry::Unique(r) => entities.push(r.key),
            Entry::Tied(c) => panic!("{what}: {n:?} is a tie over {c:?}"),
        }
    }
    let total = entities.len();
    entities.sort();
    entities.dedup();
    assert_eq!(entities.len(), total, "{what}: one name per entity");
    let mut names: Vec<StableName> = t.iter().map(|(n, _)| n.clone()).collect();
    names.sort();
    names
}

#[test]
fn a_holes_two_rims_chamfer_under_the_ring_roles_and_replay_alike() {
    let (doc, node) = chamfered(0.05, false);
    let ev = fixture::run(&doc, &EvalOptions::default());
    let names = named(&ev, node, "the two rims");
    let t = table(&ev, node);
    let n = |seg: fn(&RoleSeg) -> bool| count(t, seg);
    assert_eq!(
        n(|s| matches!(s, RoleSeg::BandFace(_))),
        2,
        "one band per rim"
    );
    assert_eq!(
        n(|s| matches!(s, RoleSeg::BandFoot(_))),
        2,
        "a host foot per band"
    );
    assert_eq!(
        n(|s| matches!(s, RoleSeg::BandSlit { .. })),
        2,
        "a slit per band"
    );
    assert_eq!(
        n(|s| matches!(s, RoleSeg::BandCross { .. })),
        4,
        "each wall seam is crossed once per band"
    );
    assert_eq!(
        n(|s| matches!(s, RoleSeg::BandCut(_))),
        2,
        "one surviving middle piece per wall seam"
    );

    let again = fixture::run(&doc, &EvalOptions::default());
    assert_eq!(
        named(&again, node, "evaluated again"),
        names,
        "a fresh evaluation"
    );
    let over = fixture::run_with_prior(&doc, &ev);
    assert_eq!(
        named(&over, node, "over its prior"),
        names,
        "an evaluation over its prior"
    );
    // The edited node mints under a new id, so its names are compared
    // with that id, wherever it is cited, read as the node's own.
    let roles = |names: &[StableName], own: RecipeNodeId| -> Vec<String> {
        let own = format!("{own:?}");
        let mut r: Vec<String> = names
            .iter()
            .map(|n| format!("{:?} {:?}", n.kind, n.path).replace(&own, "SELF"))
            .collect();
        r.sort();
        r
    };
    let (edited, edited_node) = chamfered(0.08, false);
    let ev = fixture::run(&edited, &EvalOptions::default());
    assert_eq!(
        roles(
            &named(&ev, edited_node, "after a setback edit"),
            edited_node
        ),
        roles(&names, node),
        "the setback edited"
    );
}

#[test]
fn a_holed_plate_chamfered_all_round_names_every_entity_once() {
    let (doc, node) = chamfered(0.05, true);
    let ev = fixture::run(&doc, &EvalOptions::default());
    named(&ev, node, "all round");
    let t = table(&ev, node);
    assert_eq!(
        count(t, |s| matches!(s, RoleSeg::BandFace(_))),
        2,
        "a frustum per rim among the strips"
    );
}
