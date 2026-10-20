//! A Split whose plane passes through a pinch: one half holds the
//! operand corner as several vertices, each `OnToolVertex { of }` +
//! `Fragment(Keeps)` over the corner's own edges it is an end of a
//! piece of in that half, read by descent (NAMES N2, *Vertices*). Each
//! copy keeps the rim of its own piece alone: the corner's edge lying
//! in the tool plane (the tip's lateral edge of a notched prism, the
//! circle a revolved apex sweeps) has no piece in that half, whose
//! edges along it are section chords.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};

use crate::fixture::{self, ang, insert, len, on_frame, scl};
use editor_core::{
    CancelToken, Datum, EntityKey, EntityRef, Entry, EvalOptions, Evaluation, ExtrudeSide, Formula,
    Node, ProfileDoc, Qualifier, RecipeNodeId, RoleSeg, SplitHalf, StableName, evaluate,
};
use geom_core::{Point3, Tol};

/// The notched block of `topo/tests/m3_pr3_split.rs`: the V-notch tip
/// `(4, 1)` has both neighbours above `y = 1`.
pub(crate) const NOTCHED: &[(f64, f64)] = &[
    (0.0, 0.0),
    (8.0, 0.0),
    (8.0, 2.0),
    (7.0, 1.0),
    (6.0, 1.0),
    (5.0, 2.0),
    (4.0, 1.0),
    (3.0, 2.0),
    (0.0, 2.0),
];

/// Its mirror image: the tip `(4, 1)` has both neighbours below.
pub(crate) const MIRRORED: &[(f64, f64)] = &[
    (0.0, 0.0),
    (3.0, 0.0),
    (4.0, 1.0),
    (5.0, 0.0),
    (6.0, 1.0),
    (7.0, 1.0),
    (8.0, 0.0),
    (8.0, 2.0),
    (0.0, 2.0),
];

/// The notched pentagon: the apex `(1, 1)` has both neighbours above.
pub(crate) const PENTAGON: &[(f64, f64)] =
    &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (1.0, 1.0), (0.0, 2.0)];

/// A pentagon whose apex `(2, 1)` has both neighbours above: revolved
/// a full turn about `y`, the apex sweeps a circle lying in `y = 1`.
pub(crate) const REVOLVED: &[(f64, f64)] =
    &[(1.0, 0.0), (3.0, 0.0), (3.0, 2.0), (2.0, 1.0), (1.0, 2.0)];

/// How the operand is made from its outline, drawn on the `xy` plane.
#[derive(Clone, Copy)]
enum Solid {
    /// Extruded 1 along `z`.
    Prism,
    /// Drawn with its `x` along `z`, and extruded 1 along `-x`.
    PrismAlongZ,
    /// Revolved a full turn about `y`.
    Turned,
}

/// A split of `outline` made `solid` by the plane `y = 1` with normal
/// `(0, ny, 0)`: the evaluation, the operand's node and the split's.
fn split_at_y1(
    outline: &[(f64, f64)],
    solid: Solid,
    ny: f64,
) -> (Evaluation<f64>, RecipeNodeId, RecipeNodeId) {
    let (doc, target, _, sp) = split_doc(outline, solid, [0.0, 1.0, 0.0], [0.0, ny, 0.0]);
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    assert!(
        ev.value(sp).is_some(),
        "the split names with no refusal: {:?}",
        ev.node_error(sp)
    );
    (ev, target, sp)
}

/// A split of `outline` made `solid` by the plane through `origin` with
/// `normal`: the document, the operand's node, the plane's and the
/// split's.
fn split_doc(
    outline: &[(f64, f64)],
    solid: Solid,
    origin: [f64; 3],
    normal: [f64; 3],
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("split_pinch_keeps", Tol::witness());
    let u = match solid {
        Solid::PrismAlongZ => [0.0, 0.0, 1.0],
        Solid::Prism | Solid::Turned => [1.0, 0.0, 0.0],
    };
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        u,
        [0.0, 1.0, 0.0],
        vec![outline.to_vec()],
    );
    let operand = match solid {
        Solid::Prism | Solid::PrismAlongZ => Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
        Solid::Turned => Node::Revolve {
            profile: p.into(),
            axis_through: [len(0.0), len(0.0)],
            axis_heading: Formula::turn_over(4).expect("a quarter turn"),
            angle: ang(std::f64::consts::TAU),
        },
    };
    let (doc, target) = insert(doc, operand);
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: origin.map(len),
            normal: normal.map(scl),
        }),
    );
    let (doc, sp) = insert(
        doc,
        Node::Split {
            target: editor_core::Operand::output(target, 0).into(),
            tool: plane.into(),
        },
    );
    (doc, target, plane, sp)
}

/// What each of `side`'s names denotes: the name, spelled with the
/// split's node read as `as_node` (two documents that differ only in
/// the plane's normal mint the split under different digests), and
/// where each of its entities sits, the mean of its vertices' points.
type Denoted = BTreeMap<String, Vec<[f64; 3]>>;

fn denoted(
    ev: &Evaluation<f64>,
    sp: RecipeNodeId,
    side: SplitHalf,
    as_node: RecipeNodeId,
) -> Denoted {
    let (from, to) = (format!("{sp:?}"), format!("{as_node:?}"));
    let body = ev
        .value(sp)
        .unwrap()
        .payload
        .output_body(side.output_body())
        .unwrap();
    let point = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let mean = |vs: Vec<topo::VertexKey>| {
        let n = vs.len() as f64;
        let s = vs
            .into_iter()
            .map(point)
            .fold([0.0; 3], |a, p: Point3<f64>| {
                [a[0] + p.x, a[1] + p.y, a[2] + p.z]
            });
        s.map(|c| c / n)
    };
    let at = |key: &EntityKey| match *key {
        EntityKey::Vertex(v) => mean(vec![v]),
        EntityKey::Edge(e) => {
            let he = body.get_edge(e).unwrap().he_plus;
            mean(vec![
                body.get_half_edge(he).unwrap().start,
                body.half_edge_end(he).unwrap(),
            ])
        }
        EntityKey::Face(f) => {
            let topo::LoopBoundary::Cycle { first } = body
                .get_loop(body.get_face(f).unwrap().outer)
                .unwrap()
                .boundary
            else {
                panic!("a face with an empty outer loop")
            };
            mean(
                body.loop_cycle(first)
                    .unwrap()
                    .into_iter()
                    .map(|he| body.get_half_edge(he).unwrap().start)
                    .collect(),
            )
        }
        _ => [0.0; 3],
    };
    fixture::half(ev, sp, side)
        .iter()
        .map(|(n, entry)| {
            let mut places: Vec<[f64; 3]> = match entry {
                Entry::Unique(r) => vec![at(&r.key)],
                Entry::Tied(c) => c.iter().map(|r| at(&r.key)).collect(),
            };
            places.sort_by(|a, b| a.partial_cmp(b).unwrap());
            (format!("{n:?}").replace(&from, &to), places)
        })
        .collect()
}

/// Whether two tables name alike and each name denotes entities within
/// `tol` of each other.
fn denote_alike(a: &Denoted, b: &Denoted, tol: f64) -> Result<(), String> {
    if a.keys().ne(b.keys()) {
        let only_a: Vec<_> = a.keys().filter(|k| !b.contains_key(*k)).collect();
        let only_b: Vec<_> = b.keys().filter(|k| !a.contains_key(*k)).collect();
        return Err(format!("the names differ: {only_a:?} against {only_b:?}"));
    }
    for (n, pa) in a {
        let pb = &b[n];
        let near = pa.len() == pb.len()
            && pa
                .iter()
                .zip(pb)
                .all(|(p, q)| (0..3).all(|i| (p[i] - q[i]).abs() <= tol));
        if !near {
            return Err(format!("{n} moved from {pa:?} to {pb:?}"));
        }
    }
    Ok(())
}

/// The name of the operand's edge from `a` to `b` (a closed edge where
/// they are one point).
fn operand_edge(
    ev: &Evaluation<f64>,
    target: RecipeNodeId,
    a: [f64; 3],
    b: [f64; 3],
) -> StableName {
    let body = ev.value(target).unwrap().payload.output_body(0).unwrap();
    let at = |v, q: [f64; 3]| {
        let p = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        (p - Point3::new(q[0], q[1], q[2])).norm() < 1e-9
    };
    ev.table(ev.output_var(target, 0).unwrap())
        .unwrap()
        .iter()
        .find_map(|(n, entry)| match entry {
            Entry::Unique(EntityRef {
                key: EntityKey::Edge(e),
                ..
            }) => {
                let he = body.get_edge(*e).unwrap().he_plus;
                let (s, t) = (
                    body.get_half_edge(he).unwrap().start,
                    body.half_edge_end(he).unwrap(),
                );
                ((at(s, a) && at(t, b)) || (at(s, b) && at(t, a))).then(|| n.clone())
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("an operand edge from {a:?} to {b:?}"))
}

/// Each qualified corner copy of `side`: its `Keeps` set, and the names
/// `side` gives the edges at it.
fn copies(
    ev: &Evaluation<f64>,
    sp: RecipeNodeId,
    side: SplitHalf,
) -> Vec<(Vec<StableName>, BTreeSet<StableName>)> {
    let table = fixture::half(ev, sp, side);
    let body = ev
        .value(sp)
        .unwrap()
        .payload
        .output_body(side.output_body())
        .unwrap();
    let mut out = Vec::new();
    for (n, entry) in table.iter() {
        let kept = match n.path.as_slice() {
            [
                RoleSeg::OnToolVertex { .. },
                RoleSeg::Fragment(Qualifier::Keeps(k)),
            ] => k.clone(),
            [RoleSeg::OnToolVertex { .. }] => continue,
            [RoleSeg::OnToolVertex { .. }, ..] => panic!("an unexpected copy name {n:?}"),
            _ => continue,
        };
        let Entry::Unique(EntityRef {
            key: EntityKey::Vertex(v),
            ..
        }) = entry
        else {
            panic!("a copy is one vertex: {n:?} {entry:?}")
        };
        let at = body
            .edges_of_vertex(*v)
            .unwrap()
            .into_iter()
            .filter_map(|e| {
                table
                    .name_of(&EntityRef {
                        body: 0,
                        key: EntityKey::Edge(e),
                    })
                    .cloned()
            })
            .collect();
        out.push((kept, at));
    }
    out
}

/// A pinched corner: its two rims' ends.
type Tip = [[[f64; 3]; 2]; 2];

/// A pinch at the corners `tips` of the operand made `solid` from
/// `outline`, each corner with its two `rims` (the operand edges off the
/// plane, ends given): the half `pinched` holds every corner twice, each
/// copy keeping the rim its piece holds (an edge `pinched` names at the
/// copy) and nothing else; the other half holds each once, bare.
/// `flips`: flipping the normal swaps the halves' tables and changes no
/// name.
struct Pinch {
    outline: &'static [(f64, f64)],
    solid: Solid,
    pinched: SplitHalf,
    tips: Vec<Tip>,
    flips: bool,
}

fn assert_pinch(pinch: &Pinch, what: &str) {
    let other = match pinch.pinched {
        SplitHalf::Above => SplitHalf::Below,
        SplitHalf::Below => SplitHalf::Above,
    };
    let (ev, target, sp) = split_at_y1(pinch.outline, pinch.solid, 1.0);
    let held = copies(&ev, sp, pinch.pinched);
    let mut expected: BTreeSet<Vec<StableName>> = BTreeSet::new();
    let mut rims: BTreeSet<StableName> = BTreeSet::new();
    for pair in &pinch.tips {
        for [ra, rb] in pair {
            let rim = operand_edge(&ev, target, *ra, *rb);
            rims.insert(rim.clone());
            expected.insert(vec![rim]);
        }
    }
    let got: BTreeSet<Vec<StableName>> = held.iter().map(|(k, _)| k.clone()).collect();
    assert_eq!(
        (held.len(), &got),
        (expected.len(), &expected),
        "{what}: each copy keeps its rim alone"
    );
    for (kept, at) in &held {
        let own: Vec<_> = at.intersection(&rims).collect();
        assert!(
            matches!(own.as_slice(), [rim] if kept.contains(rim)),
            "{what}: a copy keeps the rim its piece holds: keeps {kept:?}, rims at it {own:?}"
        );
    }
    let kept_other = copies(&ev, sp, other);
    assert!(
        kept_other.is_empty(),
        "{what}: a corner held once per half keeps its bare name: {kept_other:?}"
    );
    if pinch.flips {
        let (flipped, _, fsp) = split_at_y1(pinch.outline, pinch.solid, -1.0);
        for (here, there) in [(pinch.pinched, other), (other, pinch.pinched)] {
            if let Err(why) = denote_alike(
                &denoted(&ev, sp, here, sp),
                &denoted(&flipped, fsp, there, sp),
                1e-12,
            ) {
                panic!("{what}: flipping the normal swaps the halves' tables: {why}");
            }
        }
    }
}

/// A prism's tip at `(x, 1)` between rims to `left` and `right`, at both
/// caps.
fn prism_tips(x: f64, left: (f64, f64), right: (f64, f64)) -> Vec<Tip> {
    [0.0, 1.0]
        .into_iter()
        .map(|z| {
            let tip = [x, 1.0, z];
            [[tip, [left.0, left.1, z]], [tip, [right.0, right.1, z]]]
        })
        .collect()
}

#[test]
fn a_split_through_a_notch_tip_tells_its_copies_apart_by_keeps() {
    let pinch = Pinch {
        outline: NOTCHED,
        solid: Solid::Prism,
        pinched: SplitHalf::Above,
        tips: prism_tips(4.0, (3.0, 2.0), (5.0, 2.0)),
        flips: true,
    };
    assert_pinch(&pinch, "notched");
}

#[test]
fn a_mirrored_split_through_a_notch_tip_tells_its_copies_apart_by_keeps() {
    let pinch = Pinch {
        outline: MIRRORED,
        solid: Solid::Prism,
        pinched: SplitHalf::Below,
        tips: prism_tips(4.0, (3.0, 0.0), (5.0, 0.0)),
        flips: true,
    };
    assert_pinch(&pinch, "mirrored");
}

#[test]
fn a_split_at_a_notched_pentagons_apex_tells_its_copies_apart_by_keeps() {
    let pinch = Pinch {
        outline: PENTAGON,
        solid: Solid::Prism,
        pinched: SplitHalf::Above,
        tips: prism_tips(1.0, (0.0, 2.0), (2.0, 2.0)),
        flips: true,
    };
    assert_pinch(&pinch, "pentagon");
}

/// The revolved apex is one vertex on the seam, the circle it sweeps
/// lying in the plane: each copy keeps its own meridian rim, the
/// circle's copies being chords.
#[test]
fn a_split_through_a_revolved_apex_keeps_its_own_rim() {
    let apex = [2.0, 1.0, 0.0];
    let pinch = Pinch {
        outline: REVOLVED,
        solid: Solid::Turned,
        pinched: SplitHalf::Above,
        tips: vec![[[apex, [1.0, 2.0, 0.0]], [apex, [3.0, 2.0, 0.0]]]],
        flips: true,
    };
    assert_pinch(&pinch, "revolved");
}

/// The operand's face lying in `y = 1` with outward normal `+y`.
fn operand_floor(ev: &Evaluation<f64>, target: RecipeNodeId) -> StableName {
    let body = ev.value(target).unwrap().payload.output_body(0).unwrap();
    ev.table(ev.output_var(target, 0).unwrap())
        .unwrap()
        .iter()
        .find_map(|(n, entry)| match entry {
            Entry::Unique(EntityRef {
                key: EntityKey::Face(f),
                ..
            }) => match body.get_surface(body.get_face(*f).unwrap().surface) {
                Some(geom::Surface::Plane { origin, normal, .. })
                    if origin.y == 1.0 && normal.y == 1.0 =>
                {
                    Some(n.clone())
                }
                _ => None,
            },
            _ => None,
        })
        .expect("an operand face in y = 1")
}

/// The face the lower half of `outline` made `solid`, split at `y = 1`,
/// takes for its top: `Merged` of the plane, cited once and bare
/// whatever number of sections the join completed, and `floor` (the
/// operand's face in the plane, where there is one) (N3). No section is
/// published on its own,
/// and `seams` of the half's edges are named from the merged face and
/// the face across them, as a seam is.
fn assert_merged_top(outline: &[(f64, f64)], solid: Solid, floor: bool, seams: usize) {
    let (ev, target, sp) = split_at_y1(outline, solid, 1.0);
    let face = |seg| StableName {
        kind: editor_core::EntityKind::Face,
        node: sp,
        path: vec![seg],
    };
    let mut constituents = vec![face(RoleSeg::SectionFace)];
    if floor {
        constituents.push(operand_floor(&ev, target));
    }
    constituents.sort();
    // A `Merged` of one is that one (N3): two sections merged alone are
    // the plane, bare.
    let merged = match constituents.as_slice() {
        [one] => one.clone(),
        _ => face(RoleSeg::Merged(constituents)),
    };
    let below = fixture::half(&ev, sp, SplitHalf::Below);
    assert!(
        below.iter().any(|(n, _)| *n == merged),
        "the lower top is {merged:?}: {:?}",
        below.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );
    assert!(
        below
            .iter()
            .filter(|(n, _)| matches!(n.path.as_slice(), [RoleSeg::SectionFace, ..]))
            .all(|(n, _)| *n == merged),
        "no section of the lower half is published on its own"
    );
    let named_from_merged = below
        .iter()
        .filter(|(n, _)| match n.path.as_slice() {
            [RoleSeg::Seam { a, b }] => **a == merged || **b == merged,
            _ => false,
        })
        .count();
    assert_eq!(named_from_merged, seams, "edges named from the merged top");
}

/// The notched block's lower top: the floor and the three sections
/// beside it, one face; each cap's edge along it runs over the cap's
/// chords and the floor's rim, so it is named from its two faces.
#[test]
fn a_notched_blocks_lower_top_is_the_floor_and_its_sections_merged() {
    assert_merged_top(NOTCHED, Solid::Prism, true, 2);
}

/// The pentagon's lower top: its two sections either side of the apex,
/// one face; each cap's edge along it joins two chords, and stays one.
#[test]
fn a_pentagons_lower_top_is_its_two_sections_merged() {
    assert_merged_top(PENTAGON, Solid::Prism, false, 0);
}

/// The revolved pentagon's lower top: the disc's annulus inside the
/// apex circle and the one outside it, one face.
#[test]
fn a_revolved_pentagons_lower_top_is_its_two_sections_merged() {
    assert_merged_top(REVOLVED, Solid::Turned, false, 0);
}

/// **A split half on the plane is an operand a boolean accepts.** A
/// small box hole cut from each lower half, well away from the plane:
/// each half ends with maximal faces, so none is refused
/// `NonMaximalFaces`.
#[test]
fn a_lower_half_on_the_plane_takes_a_box_hole() {
    for (what, outline, solid, (x0, x1), (z0, dz)) in [
        ("notched", NOTCHED, Solid::Prism, (1.0, 2.0), (-1.0, 3.0)),
        ("pentagon", PENTAGON, Solid::Prism, (0.5, 1.0), (-1.0, 3.0)),
        ("revolved", REVOLVED, Solid::Turned, (1.5, 2.5), (-0.3, 0.6)),
    ] {
        let doc = ProfileDoc::empty_derived("split_half_hole", Tol::witness());
        let (doc, p) = on_frame(
            doc,
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![outline.to_vec()],
        );
        let operand = match solid {
            Solid::Prism | Solid::PrismAlongZ => Node::Extrude {
                profile: p.into(),
                distance: len(1.0),
                side: ExtrudeSide::Along,
            },
            Solid::Turned => Node::Revolve {
                profile: p.into(),
                axis_through: [len(0.0), len(0.0)],
                axis_heading: Formula::turn_over(4).expect("a quarter turn"),
                angle: ang(std::f64::consts::TAU),
            },
        };
        let (doc, target) = insert(doc, operand);
        let (doc, plane) = insert(
            doc,
            Node::Datum(Datum::Plane {
                origin: [len(0.0), len(1.0), len(0.0)],
                normal: [scl(0.0), scl(1.0), scl(0.0)],
            }),
        );
        let (doc, sp) = insert(
            doc,
            Node::Split {
                target: editor_core::Operand::output(target, 0).into(),
                tool: plane.into(),
            },
        );
        let (doc, q) = on_frame(
            doc,
            [0.0, 0.0, z0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(x0, 0.2), (x1, 0.2), (x1, 0.5), (x0, 0.5)]],
        );
        let (doc, hole) = insert(
            doc,
            Node::Extrude {
                profile: q.into(),
                distance: len(dz),
                side: ExtrudeSide::Along,
            },
        );
        let (doc, cut) = insert(
            doc,
            Node::Subtract {
                from: editor_core::Operand::output(sp, SplitHalf::Below.port()).into(),
                tool: hole.into(),
            },
        );
        let ev = evaluate::<f64>(
            &doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        assert!(
            ev.value(cut).is_some(),
            "{what}: the lower half takes the hole: {:?}",
            ev.node_error(cut)
        );
    }
}

// ------------------------------------------------------------------ //
// A Split's section faces: pieces of the plane (N2)
// ------------------------------------------------------------------ //

/// The one operand face of `target` whose plane holds `p` with normal
/// `±n` (`n` a unit vector), its outer loop's box holding `p`.
fn operand_face_on(
    ev: &Evaluation<f64>,
    target: RecipeNodeId,
    p: [f64; 3],
    n: [f64; 3],
) -> StableName {
    let body = ev.value(target).unwrap().payload.output_body(0).unwrap();
    let boxed = |f: topo::FaceKey| {
        let topo::LoopBoundary::Cycle { first } = body
            .get_loop(body.get_face(f).unwrap().outer)
            .unwrap()
            .boundary
        else {
            return false;
        };
        let pts: Vec<_> = body
            .loop_cycle(first)
            .unwrap()
            .into_iter()
            .map(|he| {
                let v = body.get_half_edge(he).unwrap().start;
                *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
            })
            .collect();
        let within = |q: f64, at: fn(&Point3<f64>) -> f64| {
            let lo = pts.iter().map(at).fold(f64::INFINITY, f64::min);
            let hi = pts.iter().map(at).fold(f64::NEG_INFINITY, f64::max);
            lo - 1e-9 <= q && q <= hi + 1e-9
        };
        within(p[0], |q| q.x) && within(p[1], |q| q.y) && within(p[2], |q| q.z)
    };
    let found: Vec<StableName> = ev
        .table(ev.output_var(target, 0).unwrap())
        .unwrap()
        .iter()
        .filter_map(|(name, entry)| match entry {
            Entry::Unique(EntityRef {
                key: EntityKey::Face(f),
                ..
            }) => match body.get_surface(body.get_face(*f).unwrap().surface) {
                Some(geom::Surface::Plane { origin, normal, .. }) => {
                    let off = (p[0] - origin.x) * normal.x
                        + (p[1] - origin.y) * normal.y
                        + (p[2] - origin.z) * normal.z;
                    let along = n[0] * normal.x + n[1] * normal.y + n[2] * normal.z;
                    (off.abs() < 1e-9 && (along.abs() - 1.0).abs() < 1e-9 && boxed(*f))
                        .then(|| name.clone())
                }
                _ => None,
            },
            _ => None,
        })
        .collect();
    match found.as_slice() {
        [one] => one.clone(),
        _ => panic!("one operand face through {p:?} normal to {n:?}: {found:?}"),
    }
}

/// `side`'s pieces of the plane, under the bare `SectionFace` or a
/// `Merged` holding it, by their qualifier: `None` for the parent's bare
/// name, else the `Borders` walls; each with its candidates.
fn plane_pieces(
    ev: &Evaluation<f64>,
    sp: RecipeNodeId,
    side: SplitHalf,
) -> Vec<(Option<Vec<StableName>>, usize)> {
    fixture::half(ev, sp, side)
        .iter()
        .filter_map(|(n, entry)| {
            let plane = match n.path.first() {
                Some(RoleSeg::SectionFace) => true,
                Some(RoleSeg::Merged(set)) => set
                    .iter()
                    .any(|c| matches!(c.path.as_slice(), [RoleSeg::SectionFace])),
                _ => false,
            };
            if !plane {
                return None;
            }
            let walls = match &n.path[1..] {
                [] => None,
                [RoleSeg::Fragment(Qualifier::Borders(walls))] => Some(walls.clone()),
                _ => panic!("an unexpected plane piece {n:?}"),
            };
            let candidates = match entry {
                Entry::Unique(_) => 1,
                Entry::Tied(c) => c.len(),
            };
            Some((walls, candidates))
        })
        .collect()
}

fn sorted(mut names: Vec<StableName>) -> Vec<StableName> {
    names.sort();
    names
}

/// **One plane piece is the bare `SectionFace`; two are told apart by
/// their walls, and a held bare name fails loudly.** Just below the
/// pentagon's apex the upper half holds one section; just above it, the
/// two prongs' sections, each citing the caps and its own prong's two
/// walls, since the exterior borders both. Moving the plane past the
/// apex vanishes the bare name, diagnosed `PredicateFlip` off the split's
/// own classification of the apex.
#[test]
fn a_pentagons_upper_section_is_bare_below_its_apex_and_two_borders_pieces_above() {
    let (doc, target, plane, sp) =
        split_doc(PENTAGON, Solid::Prism, [0.0, 0.9, 0.0], [0.0, 1.0, 0.0]);
    let before = fixture::run(&doc, &Default::default());
    assert_eq!(
        plane_pieces(&before, sp, SplitHalf::Above),
        vec![(None, 1)],
        "below the apex the upper half holds one section, bare"
    );
    let (moved, _) = fixture::step(
        doc.clone(),
        editor_core::DocEdit::SetParam {
            node: plane,
            slot: editor_core::SlotId::Origin(editor_core::Axis3::Y),
            value: len(1.1).into(),
            fresh: Vec::new(),
        },
    );
    let after = fixture::run(&moved, &Default::default());
    let face = |p, n| operand_face_on(&after, target, p, n);
    let r = std::f64::consts::FRAC_1_SQRT_2;
    let caps = [
        face([0.5, 0.5, 0.0], [0.0, 0.0, 1.0]),
        face([0.5, 0.5, 1.0], [0.0, 0.0, 1.0]),
    ];
    let prong = |wall: StableName, slant: StableName| {
        sorted(vec![caps[0].clone(), caps[1].clone(), wall, slant])
    };
    let left = prong(
        face([0.0, 1.5, 0.5], [1.0, 0.0, 0.0]),
        face([0.5, 1.5, 0.5], [r, r, 0.0]),
    );
    let right = prong(
        face([2.0, 1.5, 0.5], [1.0, 0.0, 0.0]),
        face([1.5, 1.5, 0.5], [r, -r, 0.0]),
    );
    let mut got = plane_pieces(&after, sp, SplitHalf::Above);
    got.sort();
    let mut want = vec![(Some(left), 1), (Some(right), 1)];
    want.sort();
    assert_eq!(
        got, want,
        "above the apex: two pieces, each by its prong's walls"
    );

    let bare = StableName {
        kind: editor_core::EntityKind::Face,
        node: sp,
        path: vec![RoleSeg::SectionFace],
    };
    let above = doc.output(sp, SplitHalf::Above.port()).unwrap();
    let new = editor_core::RunCtx {
        doc: &moved,
        eval: &after,
    };
    let prior = editor_core::RunCtx {
        doc: &doc,
        eval: &before,
    };
    match editor_core::resolve_with_prior(new, prior, above, &bare).unwrap() {
        editor_core::Resolution::Failed(f) => assert!(
            matches!(
                f.error,
                editor_core::ResolveError::Vanished {
                    diagnosis: editor_core::Diagnosis::PredicateFlip { .. },
                    ..
                }
            ),
            "the held bare section vanishes, a predicate flip: {:?}",
            f.error
        ),
        other => panic!("the bare section no longer resolves: {other:?}"),
    }
}

/// **A tilt within one topology moves no section's name.** The notched
/// block, drawn along `z`, cut at `y = 1.5` holds three sections in each
/// half, each by its own walls; turning the plane a little about `z` or
/// `x`, which reorders the join's sweep, moves no name to another
/// entity.
#[test]
fn a_notched_blocks_three_sections_keep_their_names_under_a_tilt() {
    let eval = |normal| {
        let (doc, _, _, sp) = split_doc(NOTCHED, Solid::PrismAlongZ, [0.0, 1.5, 0.0], normal);
        (fixture::run(&doc, &Default::default()), sp)
    };
    let (flat, sp) = eval([0.0, 1.0, 0.0]);
    let t = 0.06_f64.to_radians();
    for normal in [[t.sin(), t.cos(), 0.0], [0.0, t.cos(), t.sin()]] {
        let (tilted, tsp) = eval(normal);
        for side in [SplitHalf::Above, SplitHalf::Below] {
            if let Err(why) = denote_alike(
                &denoted(&flat, sp, side, sp),
                &denoted(&tilted, tsp, side, sp),
                0.05,
            ) {
                panic!("{side:?}: the plane turned to {normal:?} renames nothing: {why}");
            }
        }
    }
    for side in [SplitHalf::Above, SplitHalf::Below] {
        let pieces = plane_pieces(&flat, sp, side);
        assert!(
            pieces.len() == 3 && pieces.iter().all(|(w, c)| w.is_some() && *c == 1),
            "{side:?}: three sections, each by its walls: {pieces:?}"
        );
    }
}

/// **A hole that no other piece sits in is cited nowhere.** Two prongs
/// of a U, the left one bored along `y`, cut across both: each upper
/// section cites the caps and its own prong's outer walls, and the
/// bored one none of the bore's four faces.
#[test]
fn a_sections_hole_with_no_island_cites_nothing_of_the_hole() {
    const U: &[(f64, f64)] = &[
        (0.0, 0.0),
        (5.0, 0.0),
        (5.0, 2.0),
        (3.0, 2.0),
        (3.0, 1.0),
        (2.0, 1.0),
        (2.0, 2.0),
        (0.0, 2.0),
    ];
    let doc = ProfileDoc::empty_derived("split_section_hole", Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![U.to_vec()],
    );
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    // A bore along +y: drawn in z (u) and x (v), so its normal is +y.
    let (doc, q) = on_frame(
        doc,
        [0.0, -1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        vec![vec![(0.25, 0.5), (0.75, 0.5), (0.75, 1.5), (0.25, 1.5)]],
    );
    let (doc, bore) = insert(
        doc,
        Node::Extrude {
            profile: q.into(),
            distance: len(4.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: editor_core::Operand::output(ext, 0).into(),
            tool: bore.into(),
        },
    );
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(1.5), len(0.0)],
            normal: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    );
    let (doc, sp) = insert(
        doc,
        Node::Split {
            target: editor_core::Operand::output(cut, 0).into(),
            tool: plane.into(),
        },
    );
    let ev = fixture::run(&doc, &Default::default());
    assert!(ev.value(sp).is_some(), "{:?}", ev.node_error(sp));
    let face = |p, n| operand_face_on(&ev, cut, p, n);
    let caps = [
        face([1.0, 1.5, 0.0], [0.0, 0.0, 1.0]),
        face([1.0, 1.5, 1.0], [0.0, 0.0, 1.0]),
    ];
    let prong = |a: f64, b: f64| {
        sorted(vec![
            caps[0].clone(),
            caps[1].clone(),
            face([a, 1.5, 0.5], [1.0, 0.0, 0.0]),
            face([b, 1.5, 0.5], [1.0, 0.0, 0.0]),
        ])
    };
    let mut got = plane_pieces(&ev, sp, SplitHalf::Above);
    got.sort();
    let mut want = vec![(Some(prong(0.0, 2.0)), 1), (Some(prong(3.0, 5.0)), 1)];
    want.sort();
    assert_eq!(got, want, "each prong's section by its outer walls alone");
}

/// **A ring an island sits in divides the plane.** A block with a
/// square groove round a central pin, cut across the groove: the outer
/// wall's section holds the groove's outline as a ring and the pin's
/// section lies in it, so the ring's part of the plane borders both and
/// the exterior only the wall. The wall's section cites the groove's
/// outer faces, and the pin's its own, and neither cites the block's
/// outside.
#[test]
fn a_ring_holding_an_island_cites_the_ring_and_the_island() {
    let doc = ProfileDoc::empty_derived("split_section_island", Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (0.0, 3.0)]],
    );
    let (doc, block) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(4.0),
            side: ExtrudeSide::Along,
        },
    );
    // The groove along +y from y = 0.5: drawn in z (u) and x (v).
    let square = |a: f64, b: f64| vec![(a, a), (b, a), (b, b), (a, b)];
    let (doc, q) = on_frame(
        doc,
        [0.0, 0.5, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        vec![square(1.0, 3.0), square(1.5, 2.5)],
    );
    let (doc, groove) = insert(
        doc,
        Node::Extrude {
            profile: q.into(),
            distance: len(4.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: editor_core::Operand::output(block, 0).into(),
            tool: groove.into(),
        },
    );
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(2.0), len(0.0)],
            normal: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    );
    let (doc, sp) = insert(
        doc,
        Node::Split {
            target: editor_core::Operand::output(cut, 0).into(),
            tool: plane.into(),
        },
    );
    let ev = fixture::run(&doc, &Default::default());
    assert!(ev.value(sp).is_some(), "{:?}", ev.node_error(sp));
    let face = |p, n| operand_face_on(&ev, cut, p, n);
    let square_walls = |a: f64, b: f64| {
        let m = 0.5 * (a + b);
        sorted(vec![
            face([a, 2.0, m], [1.0, 0.0, 0.0]),
            face([b, 2.0, m], [1.0, 0.0, 0.0]),
            face([m, 2.0, a], [0.0, 0.0, 1.0]),
            face([m, 2.0, b], [0.0, 0.0, 1.0]),
        ])
    };
    let mut want = vec![
        (Some(square_walls(1.0, 3.0)), 1),
        (Some(square_walls(1.5, 2.5)), 1),
    ];
    want.sort();
    for side in [SplitHalf::Above, SplitHalf::Below] {
        let mut got = plane_pieces(&ev, sp, side);
        got.sort();
        assert_eq!(
            got, want,
            "{side:?}: the wall by the ring, the pin by its own"
        );
    }
}

/// **An island is read in the half as it ends.** A block with a pocket
/// down to `y = 1` round a pin, and a moat round the pin cut on down to
/// `y = 0.5`, split at `y = 1`. Below, the half's merge joins the rim's
/// section and the pocket's floor into one face, whose ring is the moat's
/// rim, and the pin's section lies in that ring: the pin cites the moat's
/// faces beside it, the merged face the moat's outer faces. Above, the
/// rim's section holds the pocket as a ring and the pin lies in it.
#[test]
fn an_island_in_a_ring_the_merge_made_cites_the_moat() {
    let doc = ProfileDoc::empty_derived("split_section_moat", Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (0.0, 3.0)]],
    );
    let (doc, block) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(4.0),
            side: ExtrudeSide::Along,
        },
    );
    // An annular tool along +y from `y = floor`: drawn in z (u) and x (v).
    let square = |a: f64, b: f64| vec![(a, a), (b, a), (b, b), (a, b)];
    let annulus = |doc, floor: f64, outer: f64| {
        let (doc, q) = on_frame(
            doc,
            [0.0, floor, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            vec![square(outer, 4.0 - outer), square(1.5, 2.5)],
        );
        insert(
            doc,
            Node::Extrude {
                profile: q.into(),
                distance: len(4.0),
                side: ExtrudeSide::Along,
            },
        )
    };
    let (doc, pocket) = annulus(doc, 1.0, 0.5);
    let (doc, moat) = annulus(doc, 0.5, 1.2);
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: editor_core::Operand::output(block, 0).into(),
            tool: pocket.into(),
        },
    );
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: editor_core::Operand::output(cut, 0).into(),
            tool: moat.into(),
        },
    );
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(1.0), len(0.0)],
            normal: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    );
    let (doc, sp) = insert(
        doc,
        Node::Split {
            target: editor_core::Operand::output(cut, 0).into(),
            tool: plane.into(),
        },
    );
    let ev = fixture::run(&doc, &Default::default());
    assert!(ev.value(sp).is_some(), "{:?}", ev.node_error(sp));
    let face = |p, n| operand_face_on(&ev, cut, p, n);
    let square_walls = |a: f64, b: f64, y: f64| {
        let m = 0.5 * (a + b);
        sorted(vec![
            face([a, y, m], [1.0, 0.0, 0.0]),
            face([b, y, m], [1.0, 0.0, 0.0]),
            face([m, y, a], [0.0, 0.0, 1.0]),
            face([m, y, b], [0.0, 0.0, 1.0]),
        ])
    };
    let pieces = |side| {
        let mut got: Vec<_> = plane_pieces(&ev, sp, side)
            .into_iter()
            .map(|(walls, c)| (walls.expect("two pieces, each qualified"), c))
            .collect();
        got.sort();
        got
    };
    let mut below = vec![
        (square_walls(1.2, 2.8, 0.75), 1),
        (square_walls(1.5, 2.5, 0.75), 1),
    ];
    below.sort();
    assert_eq!(
        pieces(SplitHalf::Below),
        below,
        "below: the merged face by the moat's outer faces, the pin by its own"
    );
    let mut above = vec![
        (square_walls(0.5, 3.5, 2.0), 1),
        (square_walls(1.5, 2.5, 2.0), 1),
    ];
    above.sort();
    assert_eq!(
        pieces(SplitHalf::Above),
        above,
        "above: the rim by the pocket's faces, the pin by its own"
    );
}

/// **Two sections bounded alike tie.** A square ring, its square
/// revolved a full turn about `y`, cut through its axis leaves each half
/// two squares, each bounded by the ring's same four faces: one `Tied`
/// row, never told apart by an order.
#[test]
fn a_ring_cut_through_its_axis_ties_its_two_sections() {
    const SQUARE: &[(f64, f64)] = &[(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)];
    let (doc, ring, _, sp) = split_doc(SQUARE, Solid::Turned, [0.0; 3], [1.0, 0.0, 0.0]);
    let ev = fixture::run(&doc, &Default::default());
    assert!(ev.value(sp).is_some(), "{:?}", ev.node_error(sp));
    let bands: Vec<StableName> = ev
        .table(ev.output_var(ring, 0).unwrap())
        .unwrap()
        .iter()
        .filter(|(n, _)| n.kind == editor_core::EntityKind::Face)
        .map(|(n, _)| n.clone())
        .collect();
    assert_eq!(bands.len(), 4, "the premise: a four-face ring: {bands:?}");
    // A wall is cited by its parent: an annulus the half's merge joined
    // across its own seam is the `Merged` of it alone.
    let parent_band = |w: &StableName| match w.path.as_slice() {
        [RoleSeg::Merged(set)] => match set.as_slice() {
            [one] => one.clone(),
            _ => panic!("a wall merged from several faces: {w:?}"),
        },
        _ => w.clone(),
    };
    for side in [SplitHalf::Above, SplitHalf::Below] {
        let pieces = plane_pieces(&ev, sp, side);
        let [(Some(walls), 2)] = pieces.as_slice() else {
            panic!("{side:?}: one tied row over the two squares: {pieces:?}");
        };
        assert_eq!(
            sorted(walls.iter().map(parent_band).collect()),
            sorted(bands.clone()),
            "{side:?}: the two squares are bounded by the ring's four faces"
        );
    }
}
