//! **Ordinary unions over merged faces, and what they name.**
//!
//! `NamingError::Emission` means a mint-time fact disagreed with the
//! result body: a kernel bug. A legal recipe that reaches a missing
//! naming rule must say so, not send the author to file a bug against
//! the kernel. The first rows are cuts across a sunk boss whose chords
//! cross or run along the line where two glued wall faces met, which
//! name as pieces of the seam their merged face's parent makes (N3), the
//! same wherever the cut crosses it; then a union whose end pass still
//! refuses under the bug's words, the one known exception, pinned with
//! the row that owns it. The next three are constructions that once
//! refused and now build: a merged wall whose edges are pieces of member
//! edges, a block sunk flush inside another, and a rim in several
//! pieces. The last three run members whose
//! coincident faces overlap in area, under a covering block, in every
//! member order: no fold step may refuse a contact the pairwise
//! judgement passed.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{Spans, block, block_on, block_on_of, failure, run};
use crate::corpus::body_of;
use crate::docm8_flat_merged::split_fixture;

use crate::fixture::{insert, len, point, step, table, value_slot};
use editor_core::{
    Bodies, Coord, DocEdit, EntityKey, Entry, NamingError, Node, NodeErrorKind, ProfileDoc,
    Qualifier, RecipeNodeId, RoleSeg, StableName, TargetAxis,
};
use geom_core::{Point3, Tol};
use std::collections::BTreeSet;

/// The cut's slab, across the line the boss's wall pieces meet on.
pub(crate) const SLAB: Spans = ((1.4, 1.6), (-1.0, 4.0), 0.8, 1.2);

/// A step beside [`SLAB`] whose chord crosses the glued line.
pub(crate) const STEP_ACROSS: Spans = ((0.9, 1.4), (1.4, 4.0), 1.3, 0.7);

/// [`STEP_ACROSS`] the other way along y.
pub(crate) const STEP_THE_OTHER_WAY: Spans = ((0.9, 1.4), (-1.0, 1.7), 1.3, 0.7);

/// A slab whose chord runs along the glued line.
pub(crate) const ALONG: Spans = ((1.4, 2.0), (1.5, 4.0), 0.8, 1.2);

/// The plate `[0,3]² × [0,1]` and the cylinder boss sunk into it, its
/// wall two semicircular pieces either side of y = 1.5.
pub(crate) fn plate_and_boss(doc: ProfileDoc) -> (ProfileDoc, [RecipeNodeId; 2]) {
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 3.0), 1.0);
    let (doc, boss) = crate::emit_union_borders::cylinder(
        doc,
        plate,
        [1.5, 1.5, 0.5],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        1.0,
    );
    (doc, [plate, boss])
}

/// A tool block over `spans`, a copy slid onto `plate` by values an
/// edit can move: the placement, whose output is the copy.
pub(crate) fn tool_block(
    doc: ProfileDoc,
    plate: RecipeNodeId,
    spans: Spans,
) -> (ProfileDoc, RecipeNodeId) {
    block_on_of(doc, plate, spans)
}

/// A plate `[0,3]² × [0,1]` and a cylinder boss sunk into it, its wall
/// drawn as two semicircular pieces either side of y = 1.5, joined by a
/// two-member union, minus the union of `tool`'s blocks (or the one
/// block), each block a copy slid onto the plate by values an edit can
/// move.
struct SunkBossCut {
    doc: ProfileDoc,
    boss: RecipeNodeId,
    joined: RecipeNodeId,
    tool: RecipeNodeId,
    cut: RecipeNodeId,
    placements: Vec<RecipeNodeId>,
}

impl SunkBossCut {
    fn new(tool: &[Spans]) -> Self {
        let doc = ProfileDoc::empty_derived("wire_merged_chord", Tol::witness());
        let (mut doc, [plate, boss]) = plate_and_boss(doc);
        let mut blocks = Vec::new();
        for &spans in tool {
            let (d, b) = tool_block(doc, plate, spans);
            doc = d;
            blocks.push(b);
        }
        let (doc, tool) = match blocks.as_slice() {
            [one] => (doc, *one),
            several => insert(
                doc,
                Node::Union {
                    members: Bodies::Spelled(several.iter().map(|&b| b.into()).collect()),
                },
            ),
        };
        let (doc, joined) = insert(
            doc,
            Node::Union {
                members: Bodies::Spelled(vec![plate.into(), boss.into()]),
            },
        );
        let (doc, cut) = insert(
            doc,
            Node::Subtract {
                from: joined.into(),
                tool: tool.into(),
            },
        );
        Self {
            doc,
            boss,
            joined,
            tool,
            cut,
            placements: blocks,
        }
    }

    /// The same document with block `k`'s copy slid to `y` along y: an
    /// edit of its Slide Y value, so every node keeps its id. The value
    /// is charted on the plane its copy's start cap is mated to, its `y`
    /// the world's.
    fn moved(&self, k: usize, y: f64) -> Self {
        let slot = value_slot(&self.doc, self.placements[k], Coord::Slide(TargetAxis::Y));
        let (doc, _) = step(
            self.doc.clone(),
            DocEdit::SetParam {
                node: self.placements[k],
                slot,
                value: len(y).into(),
                fresh: Vec::new(),
            },
        );
        Self {
            doc,
            placements: self.placements.clone(),
            ..*self
        }
    }

    /// The names of the cut's edges whose two ends `ends` accepts, the
    /// cut having named, each a seam whose one side is the boss wall's
    /// merged face, cited by its `Merged` parent over the two wall pieces
    /// ([`assert_cites_the_merged_wall`]).
    fn edges(
        &self,
        label: &str,
        ends: impl Fn(Point3<f64>, Point3<f64>) -> bool,
    ) -> Vec<StableName> {
        let ev = run(&self.doc);
        assert!(
            failure(&ev, self.joined).is_none(),
            "{label}: the boss joins the plate"
        );
        assert!(
            failure(&ev, self.tool).is_none(),
            "{label}: the tool builds"
        );
        assert!(
            failure(&ev, self.cut).is_none(),
            "{label}: the cut names: {:?}",
            failure(&ev, self.cut)
        );
        let boss_body = body_of(&ev, self.boss);
        let pieces: BTreeSet<StableName> = table(&ev, self.boss)
            .iter()
            .filter_map(|(name, entry)| match entry {
                Entry::Unique(ent) => match ent.key {
                    EntityKey::Face(f)
                        if topo::query::face_surface_kind(&boss_body, f)
                            == Some(geom::SurfaceKind::Cylinder) =>
                    {
                        Some(name.clone())
                    }
                    _ => None,
                },
                _ => None,
            })
            .collect();
        assert_eq!(pieces.len(), 2, "{label}: the boss wall is two pieces");
        let body = body_of(&ev, self.cut);
        let mut named = Vec::new();
        for (name, entry) in table(&ev, self.cut).iter() {
            let Entry::Unique(ent) = entry else { continue };
            let EntityKey::Edge(e) = ent.key else {
                continue;
            };
            let [p0, p1] = crate::fixture::ends(&body, e).map(|v| point(&body, v));
            if ends(p0, p1) || ends(p1, p0) {
                assert_cites_the_merged_wall(label, name, &pieces);
                named.push(name.clone());
            }
        }
        named
    }

    /// The name of the cut's one edge from `chord[0]` to `chord[1]`
    /// (either way round): a seam between the boss wall's merged face,
    /// cited by its `Merged` parent holding both wall pieces, and a face
    /// of the tool — never by one wall piece alone.
    fn chord(&self, label: &str, chord: [[f64; 3]; 2]) -> StableName {
        let named = self.edges(label, |p0, p1| near(p0, chord[0]) && near(p1, chord[1]));
        let [name] = named.as_slice() else {
            panic!("{label}: one named edge runs {chord:?}: {named:?}");
        };
        name.clone()
    }
}

fn near(p: Point3<f64>, q: [f64; 3]) -> bool {
    (p.x - q[0]).abs() < 1e-3 && (p.y - q[1]).abs() < 1e-3 && (p.z - q[2]).abs() < 1e-3
}

/// `name` is a seam, one of whose sides is the boss wall's `Merged`
/// parent: a set whose members, read through the reads that carried
/// them, are the two wall pieces `pieces`, and no other face.
fn assert_cites_the_merged_wall(label: &str, name: &StableName, pieces: &BTreeSet<StableName>) {
    let Some(RoleSeg::Seam { a, b }) = name.path.first() else {
        panic!("{label}: the chord is a seam: {name}");
    };
    let through = |mut n: &StableName| {
        while let [RoleSeg::From { of, .. }] = n.path.as_slice() {
            n = of;
        }
        n.clone()
    };
    let merged: Vec<BTreeSet<StableName>> = [a, b]
        .into_iter()
        .filter_map(|side| match side.path.as_slice() {
            [RoleSeg::Merged(set)] => Some(set.iter().map(through).collect()),
            _ => None,
        })
        .collect();
    assert_eq!(
        merged,
        std::slice::from_ref(pieces),
        "{label}: one side of the seam is the merged wall's parent, over the two wall \
         pieces and nothing else: {name}"
    );
}

/// **A chord across the glued pieces' common line names, the same
/// wherever the cut crosses it.**
///
/// A slab along y through the boss, and a box on the boss's left wall
/// beside it whose floor at z = 1.3 cuts the left merged wall in one
/// chord, from (1.217, 1.4) to (1.4, 1.783): across y = 1.5, where the
/// two boss pieces met before the merge glued them. Moving the box 0.2
/// along y leaves its chord on the upper piece alone, under the same
/// name; the box running the other way along y, whose chord crosses
/// y = 1.5 from above, names the same way.
#[test]
fn a_chord_across_the_glued_pieces_line_names_wherever_the_cut_crosses() {
    let fixture = SunkBossCut::new(&[SLAB, STEP_ACROSS]);
    let across = fixture.chord("across", [[1.2172, 1.4, 1.3], [1.4, 1.7828, 1.3]]);
    let upper = fixture
        .moved(1, 0.2)
        .chord("upper", [[1.2172, 1.6, 1.3], [1.4, 1.7828, 1.3]]);
    assert_eq!(
        across, upper,
        "a chord across the glued line and one clear of it are one name"
    );
    SunkBossCut::new(&[SLAB, STEP_THE_OTHER_WAY])
        .chord("the other way", [[1.2764, 1.7, 1.3], [1.4, 1.2172, 1.3]]);
}

/// **A chord along the glued pieces' common line names as one beside
/// it.** A block whose wall at y = 1.5 is the boss's split plane cuts
/// the boss's right wall in a chord along the line the two pieces met
/// on; the same block moved 0.1 along y cuts a chord on the upper piece
/// alone, under the same name.
#[test]
fn a_chord_along_the_glued_pieces_line_names_as_one_beside_it() {
    let fixture = SunkBossCut::new(&[ALONG]);
    let along = fixture.chord("along", [[1.8, 1.5, 1.0], [1.8, 1.5, 1.5]]);
    let beside = fixture
        .moved(0, 0.1)
        .chord("beside", [[1.7828, 1.6, 1.0], [1.7828, 1.6, 1.5]]);
    assert_eq!(
        along, beside,
        "the chord along the glued line and one beside it"
    );
}

/// **Two chords of one seam are told apart by their ends.** The slab's
/// wall at x = 1.4 cuts the boss's left merged wall twice, once on each
/// wall piece: two edges between the same two faces, so two pieces of
/// one seam that cites the merged wall's parent, each qualified by its
/// `Ends` (N2), never by the piece it lies on.
#[test]
fn two_chords_of_one_seam_are_told_apart_by_their_ends() {
    let fixture = SunkBossCut::new(&[SLAB]);
    let on_wall = |p: Point3<f64>| ((p.x - 1.5).hypot(p.y - 1.5) - 0.3).abs() < 1e-9;
    let chords = fixture.edges("x = 1.4", |p0, p1| {
        on_wall(p0)
            && on_wall(p1)
            && (p0.x - 1.4).abs() < 1e-9
            && (p1.x - 1.4).abs() < 1e-9
            && (p0.y - p1.y).abs() < 1e-9
    });
    let [c0, c1] = chords.as_slice() else {
        panic!("two chords run up the left wall at x = 1.4: {chords:?}");
    };
    for c in [c0, c1] {
        assert!(
            matches!(
                c.path.as_slice(),
                [RoleSeg::Seam { .. }, RoleSeg::Fragment(Qualifier::Ends(_))]
            ),
            "a chord is a piece of the seam, told apart by its ends: {c}"
        );
    }
    assert_eq!(
        c0.path[0], c1.path[0],
        "the two chords are pieces of one seam"
    );
    assert_ne!(c0, c1, "and their ends tell them apart");
}

fn volume(body: &topo::Body<f64>) -> f64 {
    topo::test_support::pinned_volume(
        "mass",
        &topo::test_support::finished("the measured body", topo::Body::clone(body), Tol::witness()),
        Tol::witness(),
    )
    .volume
}

/// **A bar through a sunk boss passes the chord rule and refuses at the
/// union's parents.**
///
/// A plate and a cylinder boss sunk into it, its wall drawn as two
/// semicircular pieces, joined by a union; a bar along y through the
/// boss, joined to them by a second union. The bar's walls cut each wall
/// piece in two, and the judgement of the two members merges the two
/// cosurface pieces on each side of the bar, so each merged wall face
/// holds both boss pieces. The chord where a bar wall meets it cites the
/// merged wall's parent, as the sunk-boss slot's does
/// (`emit_union_borders::a_slot_across_a_sunk_boss_divides_its_merged_wall_by_the_slot_walls`).
/// The union's end pass then links member faces only across the two
/// members a judgement holds, so the two faces of the first member it
/// merged are two parents in one finished face, and that refuses as an
/// emission bug: the one legal recipe here told the kernel is broken,
/// pinned as the known exception. Owned by
/// `work/refs/a-union-judgement-that-merges-two-faces-of-one-member-links-neither.md`,
/// whose fix flips this row.
#[test]
fn a_bar_through_a_sunk_boss_refuses_at_the_unions_parents() {
    let doc = ProfileDoc::empty_derived("wire_merged_chord", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 3.0), 1.0);
    let (doc, boss) = crate::emit_union_borders::cylinder(
        doc,
        plate,
        [1.5, 1.5, 0.5],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        1.0,
    );
    let (doc, bar) = block_on(doc, plate, (1.4, 1.6), (0.5, 2.5), 0.8, 1.2);
    let (doc, joined) = crate::fixture::insert(
        doc,
        editor_core::Node::Union {
            members: editor_core::Bodies::Spelled(vec![plate.into(), boss.into()]),
        },
    );
    let (doc, barred) = crate::fixture::insert(
        doc,
        editor_core::Node::Union {
            members: editor_core::Bodies::Spelled(vec![joined.into(), bar.into()]),
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, joined).is_none(), "the boss joins the plate");
    assert!(
        matches!(
            failure(&ev, barred),
            Some(NodeErrorKind::Naming(NamingError::Emission {
                what: "a union's face descends from member faces no judged pair links"
            }))
        ),
        "the union's parents do not link the boss's two pieces: {:?}",
        failure(&ev, barred)
    );
}

/// **A split piece the fold's kills cut off is a piece, not a chord.**
///
/// `a` and `c` meet flush along x on all four families; `s` rises
/// through `a`'s top cap across its depth, flush with `a`'s two
/// y-walls. Folding `a` in last glues a face of `c` and of `s` into one
/// merged wall with `a`'s. The edges along that wall are pieces of
/// `a`'s and `s`'s own edges whose split lineage runs through the
/// stretches the fold killed, so they are named as those pieces, and
/// every order fuses to the volume the geometry says with every member
/// edge it names lying on its source.
#[test]
fn a_split_fixture_with_a_merged_wall_fuses_in_every_order() {
    let doc = ProfileDoc::empty_derived("wire_seam_vertex", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, c) = block_on(doc, a, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, s) = block_on(doc, a, (0.2, 0.4), (0.0, 1.0), 0.5, 1.0);
    for order in [
        vec![a, c, s],
        vec![a, s, c],
        vec![c, a, s],
        vec![c, s, a],
        vec![s, a, c],
        vec![s, c, a],
    ] {
        let (docx, union) = crate::fixture::union_over(doc.clone(), &order);
        let ev = run(&docx);
        assert!(
            failure(&ev, union).is_none(),
            "{order:?}: the union fuses: {:?}",
            failure(&ev, union)
        );
        let v = volume(&(body_of(&ev, union)));
        assert!((v - 1.6).abs() < 1e-9, "{order:?}: volume {v}");
        crate::emit_shared_rim_several::every_member_edge_lies_on_its_source(
            &docx,
            &ev,
            union,
            &order,
            &format!("{order:?}"),
        );
    }
}

/// **The split fixture with `s` sunk inside `a` fuses in every order.**
/// `s` spans `z ∈ [0.5, 0.75]`, wholly inside `a` and flush with its two
/// y-walls, so the union is `a ∪ c`: every order decides the flush
/// containment and fuses to `1.5`.
#[test]
fn a_split_fixture_with_its_block_sunk_flush_fuses_in_every_order() {
    let doc = ProfileDoc::empty_derived("wire_sunk_flush", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, c) = block_on(doc, a, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, s) = block_on(doc, a, (0.2, 0.4), (0.0, 1.0), 0.5, 0.25);
    for order in [
        vec![a, c, s],
        vec![a, s, c],
        vec![c, a, s],
        vec![c, s, a],
        vec![s, a, c],
        vec![s, c, a],
    ] {
        let (docx, union) = crate::fixture::union_over(doc.clone(), &order);
        let ev = run(&docx);
        assert!(
            failure(&ev, union).is_none(),
            "{order:?}: the union fuses: {:?}",
            failure(&ev, union)
        );
        let v = volume(&(body_of(&ev, union)));
        assert!((v - 1.5).abs() < 1e-9, "{order:?}: volume {v}");
    }
}

/// **A rim that comes in several pieces no longer refuses.**
///
/// `a` and `b` meet flush along x, so the fold's first step merges
/// their caps; `g` is a slab that rises through the
/// merged top cap and out through both y-walls. At step 2 an edge along
/// the merged cap's rim is a piece of one of the pieces step 1 left on
/// that line, split off around a stretch `g` killed, and is named as
/// that piece through its lineage. This document refused
/// `SharedRim { found: Several }` while the lineage stopped at the
/// killed stretch and the edge was read as a chord between two faces
/// sharing several pieces; the refusal's sentence stays pinned in
/// `display_contract`.
///
/// The recipe is legal: the same two members fuse on their own, `g` is
/// an ordinary overlapping solid, and the three together fuse to the
/// volume the geometry says.
#[test]
fn a_rim_in_several_pieces_is_named_not_refused() {
    let doc = ProfileDoc::empty_derived("wire_shared_rim", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block_on(doc, a, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, g) = block_on(doc, a, (0.7, 0.8), (-1.0, 2.0), 0.5, 3.0);

    let (docx, union) = crate::fixture::union_over(doc, &[a, b, g]);
    let ev = run(&docx);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    // a ∪ b is 1.5; g (z = 0.5..3.5) adds 0.1 × 3 × 3 less its
    // 0.1 × 1 × 0.5 inside.
    let v = volume(&(body_of(&ev, union)));
    assert!((v - 2.35).abs() < 1e-9, "volume {v}");
}

/// What one member order of a union came to. Every refusal here is a
/// missing rule another row owns; none is the document's fault.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Seen {
    /// One body, at the volume the geometry says.
    Fused,
}

/// Every ordering of `0..n`.
fn orders(n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![vec![]];
    }
    let mut out = Vec::new();
    for first in 0..n {
        for rest in orders(n - 1) {
            let mut o = vec![first];
            o.extend(rest.into_iter().map(|k| k + usize::from(k >= first)));
            out.push(o);
        }
    }
    out
}

/// Runs the union of `members` in every order and checks the
/// outcomes against `want`, one line per order: an order that fuses
/// must reach `fused_volume`, and an order that refuses must refuse with a
/// missing rule some other row owns.
///
/// The refusal that must never appear is the fold's own contact verdict:
/// the n-ary door (`topo::nary_boolean`) judged every member pair before the fold,
/// so a fold step that refuses a contact would be judging one no pair
/// has (`fold_step_refusal`).
fn every_order(
    doc: &ProfileDoc,
    members: &[(&str, RecipeNodeId)],
    fused_volume: f64,
    want: &[(&str, Seen)],
) {
    let mut seen: Vec<(String, Seen)> = orders(members.len())
        .into_iter()
        .map(|o| {
            let label = o
                .iter()
                .map(|&k| members[k].0)
                .collect::<Vec<_>>()
                .join(",");
            let order: Vec<RecipeNodeId> = o.iter().map(|&k| members[k].1).collect();
            let (docx, union) = crate::fixture::union_over(doc.clone(), &order);
            let ev = run(&docx);
            let outcome = match failure(&ev, union) {
                None => {
                    let v = volume(&(body_of(&ev, union)));
                    assert!(
                        (v - fused_volume).abs() < 1e-9,
                        "[{label}]: volume {v}, want {fused_volume}"
                    );
                    Seen::Fused
                }
                Some(other) => panic!("[{label}]: a legal union refused {other:?}"),
            };
            (label, outcome)
        })
        .collect();
    let mut want: Vec<(String, Seen)> = want.iter().map(|&(o, s)| (o.to_owned(), s)).collect();
    seen.sort();
    want.sort();
    assert_eq!(seen, want, "the outcome of some member order moved");
}

/// `a` is a unit block; `s` rises through `a`'s top cap across its
/// whole depth, flush with `a`'s two y-walls, which it overlaps in
/// area; `big` swallows `a`'s top cap and the part of `s` above
/// z = 0.8. Folding `a` into an accumulation that holds `s` glues each
/// coincident wall pair into one merged face, so the step after it meets
/// no coplanar pair inside one operand, and no fold step judges a
/// contact the pairwise pre-pass did not.
fn area_overlap_fixture(doc: ProfileDoc) -> (ProfileDoc, [RecipeNodeId; 3]) {
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, s) = block_on(doc, a, (0.2, 0.4), (0.0, 1.0), 0.5, 1.0);
    let (doc, big) = block_on(doc, a, (-1.0, 2.0), (-1.0, 2.0), 0.8, 1.4);
    (doc, [a, s, big])
}

/// `big` (12.6) plus `a` below z = 0.8 (0.8); the rest of `a` and all
/// of `s` lie inside the two.
const AREA_OVERLAP_VOLUME: f64 = 13.4;

#[test]
fn no_order_of_an_area_overlap_under_a_covering_block_refuses_a_fold_contact() {
    use Seen::Fused;
    let doc = ProfileDoc::empty_derived("wire_fold_contact_3", Tol::witness());
    let (doc, [a, s, big]) = area_overlap_fixture(doc);
    // Every order fuses: the vertex whose half-decided parentage refused
    // two of them was a cut vertex the output stage now joins away.
    every_order(
        &doc,
        &[("a", a), ("s", s), ("big", big)],
        AREA_OVERLAP_VOLUME,
        &[
            ("a,s,big", Fused),
            ("a,big,s", Fused),
            ("s,a,big", Fused),
            ("s,big,a", Fused),
            ("big,a,s", Fused),
            ("big,s,a", Fused),
        ],
    );
}

/// The same three, plus `p` resting on `a`'s top cap inside `big`.
#[test]
fn no_order_of_the_area_overlap_union_with_a_fourth_member_refuses_a_fold_contact() {
    use Seen::Fused;
    let doc = ProfileDoc::empty_derived("wire_fold_contact_4", Tol::witness());
    let (doc, [a, s, big]) = area_overlap_fixture(doc);
    let (doc, p) = block_on(doc, a, (0.6, 0.9), (0.2, 0.8), 1.0, 0.2);
    every_order(
        &doc,
        &[("a", a), ("s", s), ("big", big), ("p", p)],
        AREA_OVERLAP_VOLUME,
        &[
            ("a,s,big,p", Fused),
            ("a,s,p,big", Fused),
            ("a,big,s,p", Fused),
            ("a,big,p,s", Fused),
            ("a,p,s,big", Fused),
            ("a,p,big,s", Fused),
            ("s,a,big,p", Fused),
            ("s,a,p,big", Fused),
            ("s,big,a,p", Fused),
            ("s,big,p,a", Fused),
            ("s,p,a,big", Fused),
            ("s,p,big,a", Fused),
            ("big,a,s,p", Fused),
            ("big,a,p,s", Fused),
            ("big,s,a,p", Fused),
            ("big,s,p,a", Fused),
            ("big,p,a,s", Fused),
            ("big,p,s,a", Fused),
            ("p,a,s,big", Fused),
            ("p,a,big,s", Fused),
            ("p,s,a,big", Fused),
            ("p,s,big,a", Fused),
            ("p,big,a,s", Fused),
            ("p,big,s,a", Fused),
        ],
    );
}

/// R1's split fixture (`a` and `c` flush along x, `s` flush with `a`'s
/// y-walls) under a block covering `a`'s top cap: a fourth member over
/// the same area overlap.
#[test]
fn no_order_of_the_split_fixture_under_a_covering_block_refuses_a_fold_contact() {
    use Seen::Fused;
    let doc = ProfileDoc::empty_derived("wire_fold_contact_split", Tol::witness());
    let (doc, [a, c, s]) = split_fixture(doc);
    let (doc, big) = block_on(doc, a, (-0.5, 1.2), (-0.5, 1.5), 0.8, 1.2);
    // `big` (4.08) plus `a ∪ c` (1.5) less their overlap (1.2 × 1 × 0.2).
    every_order(
        &doc,
        &[("a", a), ("c", c), ("s", s), ("big", big)],
        5.34,
        &[
            ("a,c,s,big", Fused),
            ("a,c,big,s", Fused),
            ("a,s,c,big", Fused),
            ("a,s,big,c", Fused),
            ("a,big,c,s", Fused),
            ("a,big,s,c", Fused),
            ("c,a,s,big", Fused),
            ("c,a,big,s", Fused),
            ("c,s,a,big", Fused),
            ("c,s,big,a", Fused),
            ("c,big,a,s", Fused),
            ("c,big,s,a", Fused),
            ("s,a,c,big", Fused),
            ("s,a,big,c", Fused),
            ("s,c,a,big", Fused),
            ("s,c,big,a", Fused),
            ("s,big,a,c", Fused),
            ("s,big,c,a", Fused),
            ("big,a,c,s", Fused),
            ("big,a,s,c", Fused),
            ("big,c,a,s", Fused),
            ("big,c,s,a", Fused),
            ("big,s,a,c", Fused),
            ("big,s,c,a", Fused),
        ],
    );
}
