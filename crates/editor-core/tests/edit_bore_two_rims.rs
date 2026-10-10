//! **Both rims of a through-hole, rounded by ONE `Node::Fillet`.** The
//! hole's two rims are ladder rims sharing the hole wall, whose seams
//! run from one rim to the other, so both bands split each seam: the
//! document evaluates, every minted entity is named, and the two
//! crossings on one wall seam are told apart by their band alone — on an
//! extruded plate with a hole and on a box with a cylinder subtracted.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::ExtrudeSide;
use editor_core::{
    EntityKey, Entry, EvalOptions, LoopProgram, NameTable, Node, NodeResult, ProfileDoc,
    ProfileProgram, RecipeNodeId, RoleSeg, StableName,
};

use crate::corpus;
use crate::fixture;
use fixture::{count, len, table, tol};
use topo::{Body, EdgeKey};

const R: f64 = 0.05;

/// A circular profile loop of radius `r` at `(cx, cy)` on the xy frame,
/// extruded `h` along `+z`.
fn disc(doc: ProfileDoc, cx: f64, cy: f64, r: f64, h: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, plane) = fixture::insert(doc, fixture::xy_frame());
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            frame: plane.into(),
            loops: vec![LoopProgram::circle(cx, cy, r).expect("a finite circle")],
            ids: Vec::new(),
        }),
    );
    fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(h),
            side: ExtrudeSide::Along,
        },
    )
}

/// The names of `body`'s circle edges at station `z`, read back through
/// the node's table.
fn rim_names(t: &NameTable, body: &Body<f64>, z: f64) -> Vec<StableName> {
    let keys: Vec<EdgeKey> = body
        .edges()
        .filter(|(_, e)| {
            matches!(
                body.get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .map(|c| c.carrier()),
                Some(geom::Curve3::Circle { center, .. }) if (center.z - z).abs() < 1e-9
            )
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(keys.len(), 2, "the rim at z = {z} is two arcs");
    let mut names: Vec<StableName> = t
        .iter()
        .filter_map(|(n, entry)| match entry {
            Entry::Unique(r) => match r.key {
                EntityKey::Edge(k) if keys.contains(&k) => Some(n.clone()),
                _ => None,
            },
            _ => None,
        })
        .collect();
    names.sort();
    assert_eq!(names.len(), 2, "each rim arc is uniquely named");
    names
}

fn fillet_both(
    doc: ProfileDoc,
    target: RecipeNodeId,
    lo: f64,
    hi: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let ev = fixture::run(&doc, &EvalOptions::default());
    let (t, body) = (table(&ev, target), corpus::body_of(&ev, target));
    let mut selection = [rim_names(t, body, lo), rim_names(t, body, hi)].concat();
    selection.sort();
    fixture::insert(
        doc,
        Node::Fillet {
            target: target.into(),
            radius: len(R),
            selection,
        },
    )
}

fn assert_two_bands(doc: &ProfileDoc, fillet: RecipeNodeId, what: &str) {
    let ev = fixture::run(doc, &EvalOptions::default());
    assert!(
        matches!(ev.nodes.get(&fillet), Some(NodeResult::Ok(_))),
        "{what}: the fillet evaluates, got {:?}",
        ev.nodes.get(&fillet)
    );
    let t = table(&ev, fillet);
    let n = |seg: fn(&RoleSeg) -> bool| count(t, seg);
    assert_eq!(
        n(|s| matches!(s, RoleSeg::BandFace(_))),
        2,
        "{what}: one band per rim"
    );
    assert_eq!(
        n(|s| matches!(s, RoleSeg::BandFoot(_))),
        2,
        "{what}: a host foot per band, the one its slit ends at (the other is joined away, \
         `docs/DESIGN.md`, maximal edges)"
    );
    assert_eq!(
        n(|s| matches!(s, RoleSeg::BandSlit { .. })),
        2,
        "{what}: a slit per band"
    );
    let crosses: Vec<&StableName> = t
        .iter()
        .map(|(n, _)| n)
        .filter(|n| matches!(n.path[0], RoleSeg::BandCross { .. }))
        .collect();
    assert_eq!(
        crosses.len(),
        4,
        "{what}: each wall seam is crossed once per band"
    );
    let mut seams: Vec<_> = crosses
        .iter()
        .map(|n| match &n.path[0] {
            RoleSeg::BandCross { edge, .. } => edge.clone(),
            _ => unreachable!(),
        })
        .collect();
    seams.sort();
    seams.dedup();
    assert_eq!(
        seams.len(),
        2,
        "{what}: two wall seams, each crossed by both bands"
    );
    assert_eq!(
        n(|s| matches!(s, RoleSeg::BandCut(_))),
        2,
        "{what}: one surviving middle piece per wall seam"
    );
}

#[test]
fn a_plate_holes_two_rims_fillet_in_one_node() {
    let doc = ProfileDoc::empty_derived("edit_bore_two_rims_plate", tol());
    let (doc, plane) = fixture::insert(doc, fixture::xy_frame());
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            frame: plane.into(),
            loops: vec![
                LoopProgram::polygon(fixture::square(0.0, 0.0, 1.0)).expect("a square"),
                LoopProgram::circle(0.0, 0.0, 0.3).expect("a finite hole"),
            ],
            ids: Vec::new(),
        }),
    );
    let (doc, block) = fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, fillet) = fillet_both(doc, block, 0.0, 1.0);
    assert_two_bands(&doc, fillet, "the plate");
}

#[test]
fn a_box_minus_a_cylinder_has_both_rims_filleted_in_one_node() {
    let doc = ProfileDoc::empty_derived("edit_bore_two_rims_box", tol());
    let (doc, block) = fixture::on_frame(
        doc,
        [0.0, 0.0, 0.5],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 1.0)],
    );
    let (doc, block) = fixture::insert(
        doc,
        Node::Extrude {
            profile: block.into(),
            distance: len(2.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, drill) = disc(doc, 0.0, 0.0, 0.3, 3.0);
    let (doc, holed) = fixture::insert(
        doc,
        Node::Subtract {
            from: block.into(),
            tool: drill.into(),
            declare: Vec::new(),
        },
    );
    let (doc, fillet) = fillet_both(doc, holed, 0.5, 2.5);
    assert_two_bands(&doc, fillet, "the box minus a cylinder");
}

/// Whether `edge`, read down its chain of operand wrappers, holds a piece
/// qualifier anywhere: an edge's line holds none (N2).
fn holds_a_piece_qualifier(edge: &StableName) -> bool {
    let mut n = edge;
    loop {
        if matches!(
            n.path.last(),
            Some(RoleSeg::Fragment(editor_core::Qualifier::Ends(_)))
        ) {
            return true;
        }
        match n.path.as_slice() {
            [RoleSeg::From { of: inner, .. }] => n = inner,
            _ => return false,
        }
    }
}

/// **A band's crossing cites the wall seam it lies on by its line** (N2,
/// *Vertices*). The drill is notched at mid-height across one of its
/// seams before it bores the box, so that hole-wall seam is two pieces;
/// each band crosses the piece at its own rim, and names the crossing
/// by the seam's line, not by the piece and its ends.
#[test]
fn a_band_crossing_a_wall_seams_piece_cites_the_seams_line() {
    let doc = ProfileDoc::empty_derived("edit_bore_two_rims_piece", tol());
    let (doc, block) = fixture::on_frame(
        doc,
        [0.0, 0.0, 0.5],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 1.0)],
    );
    let (doc, block) = fixture::insert(
        doc,
        Node::Extrude {
            profile: block.into(),
            distance: len(2.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, drill) = disc(doc, 0.0, 0.0, 0.3, 3.0);
    let (doc, notch) = fixture::on_frame(
        doc,
        [0.0, 0.0, 1.4],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.3, 0.0, 0.05)],
    );
    let (doc, notch) = fixture::insert(
        doc,
        Node::Extrude {
            profile: notch.into(),
            distance: len(0.2),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, notched) = fixture::insert(
        doc,
        Node::Subtract {
            from: drill.into(),
            tool: notch.into(),
            declare: Vec::new(),
        },
    );
    let (doc, holed) = fixture::insert(
        doc,
        Node::Subtract {
            from: block.into(),
            tool: notched.into(),
            declare: Vec::new(),
        },
    );
    let (doc, fillet) = fillet_both(doc, holed, 0.5, 2.5);
    let ev = fixture::run(&doc, &EvalOptions::default());
    assert!(
        matches!(ev.nodes.get(&fillet), Some(NodeResult::Ok(_))),
        "the fillet evaluates, got {:?}",
        ev.nodes.get(&fillet)
    );
    let crossed: Vec<StableName> = table(&ev, fillet)
        .iter()
        .filter_map(|(n, _)| match &n.path[0] {
            RoleSeg::BandCross { edge, .. } => Some((**edge).clone()),
            _ => None,
        })
        .collect();
    assert_eq!(
        crossed.len(),
        4,
        "each band crosses both wall seams: {crossed:?}"
    );
    let cited_pieces = table(&ev, holed)
        .iter()
        .filter(|(n, _)| {
            matches!(n.path.as_slice(), [RoleSeg::From { read: FOLD_B, of: inner }] if holds_a_piece_qualifier(inner))
        })
        .count();
    assert!(
        cited_pieces >= 2,
        "the notched seam is pieces in the bored box"
    );
    for edge in &crossed {
        assert!(
            !holds_a_piece_qualifier(edge),
            "a band crossing cites its seam's line, not a piece: {edge:?}"
        );
    }
}
