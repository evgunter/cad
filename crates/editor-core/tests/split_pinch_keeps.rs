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

use crate::fixture::{self, against, ang, axes_at, desc, insert, len, on_frame};
use editor_core::{
    CancelToken, EntityKey, EntityRef, Entry, EvalOptions, Evaluation, Formula, Node, ProfileDoc,
    Qualifier, RecipeNodeId, RoleSeg, SplitHalf, StableName, evaluate,
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

/// How the operand is made from its outline, drawn on its own `xy`.
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
    let (doc, target, sp) = split_doc(outline, solid, [0.0, 1.0, 0.0], [0.0, ny, 0.0]);
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
/// `normal`, both in the world: the document, the operand's node and
/// the split's.
fn split_doc(
    outline: &[(f64, f64)],
    solid: Solid,
    origin: [f64; 3],
    normal: [f64; 3],
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, target) = operand_doc(outline, solid);
    let tool = plane_tool(&doc, target, solid, origin, normal);
    let (doc, sp) = insert(
        doc,
        Node::Split {
            target: editor_core::Operand::output(target, 0).into(),
            tool,
        },
    );
    (doc, target, sp)
}

/// `outline` made `solid`, in a document of its own: the document and
/// the operand's node — for [`Solid::PrismAlongZ`], the placement that
/// stands the prism in the world, spun `−turn/4` about `y`.
fn operand_doc(outline: &[(f64, f64)], solid: Solid) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("split_pinch_keeps", Tol::witness());
    let (doc, p) = insert(doc, Node::Profile(desc(vec![outline.to_vec()])));
    let prism = |doc| {
        insert(
            doc,
            Node::Extrude {
                profile: p.into(),
                distance: len(1.0),
            },
        )
    };
    match solid {
        Solid::Prism => prism(doc),
        Solid::PrismAlongZ => {
            let (doc, body) = prism(doc);
            let quarter = Formula::turn_over(4).expect("a quarter turn");
            let back = Formula::neg(quarter).expect("an angle negates");
            fixture::in_world_by(
                doc,
                body,
                fixture::turned_to([0.0; 3], &[(editor_core::TargetAxis::Y, back)]),
            )
        }
        Solid::Turned => insert(
            doc,
            Node::Revolve {
                profile: p.into(),
                axis_through: [len(0.0), len(0.0)],
                axis_heading: Formula::turn_over(4).expect("a quarter turn"),
                angle: ang(std::f64::consts::TAU),
            },
        ),
    }
}

/// **The plane through `origin` with `normal`, both in the world, read
/// off `target`** ([`operand_doc`]'s): exactly, off a face it is
/// parallel to ([`fixture::plane_on`]), where the operand has one — every
/// axis plane but one through a turned operand's axis — and otherwise
/// off a frame on the operand ([`fixture::plane_on_frame`]). A plane on
/// a [`Solid::PrismAlongZ`] operand is given in the prism's own
/// coordinates, the world's turned back.
fn plane_tool(
    doc: &ProfileDoc,
    target: RecipeNodeId,
    solid: Solid,
    origin: [f64; 3],
    normal: [f64; 3],
) -> editor_core::pose::RoleRead<editor_core::Operand> {
    let own = |p: [f64; 3]| match solid {
        Solid::PrismAlongZ => [p[2], p[1], -p[0]],
        Solid::Prism | Solid::Turned => p,
    };
    let (origin, normal) = (own(origin), own(normal));
    let axis = normal.iter().filter(|c| **c != 0.0).count() == 1;
    let body = editor_core::Operand::output(target, 0);
    if axis && !(matches!(solid, Solid::Turned) && normal[1] == 0.0) {
        fixture::plane_on(doc, body, origin, normal)
    } else {
        fixture::plane_on_frame(doc, body, origin, normal)
    }
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
    let point = |v| {
        *body
            .body()
            .get_point(body.body().get_vertex(v).unwrap().point)
            .unwrap()
    };
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
            let he = body.body().get_edge(e).unwrap().he_plus;
            mean(vec![
                body.body().get_half_edge(he).unwrap().start,
                body.body().half_edge_end(he).unwrap(),
            ])
        }
        EntityKey::Face(f) => {
            let topo::LoopBoundary::Cycle { first } = body
                .body()
                .get_loop(body.body().get_face(f).unwrap().outer)
                .unwrap()
                .boundary
            else {
                panic!("a face with an empty outer loop")
            };
            mean(
                body.body()
                    .loop_cycle(first)
                    .unwrap()
                    .into_iter()
                    .map(|he| body.body().get_half_edge(he).unwrap().start)
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
        let p = *body
            .body()
            .get_point(body.body().get_vertex(v).unwrap().point)
            .unwrap();
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
                let he = body.body().get_edge(*e).unwrap().he_plus;
                let (s, t) = (
                    body.body().get_half_edge(he).unwrap().start,
                    body.body().half_edge_end(he).unwrap(),
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
            .body()
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
            }) => match body
                .body()
                .get_surface(body.body().get_face(*f).unwrap().surface)
            {
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
            },
            Solid::Turned => Node::Revolve {
                profile: p.into(),
                axis_through: [len(0.0), len(0.0)],
                axis_heading: Formula::turn_over(4).expect("a quarter turn"),
                angle: ang(std::f64::consts::TAU),
            },
        };
        let (doc, target) = insert(doc, operand);
        let body = editor_core::Operand::output(target, 0);
        let tool = crate::fixture::plane_on(&doc, body.clone(), [0.0, 1.0, 0.0], [0.0, 1.0, 0.0]);
        let (doc, sp) = insert(
            doc,
            Node::Split {
                target: body.clone().into(),
                tool,
            },
        );
        let (doc, q) = on_frame(
            doc,
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(x0, 0.2), (x1, 0.2), (x1, 0.5), (x0, 0.5)]],
        );
        let (doc, hole) = insert(
            doc,
            Node::Extrude {
                profile: q.into(),
                distance: len(dz),
            },
        );
        let (doc, hole) = crate::fixture::slid(doc, body, hole, [0.0, 0.0, z0]);
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
            .body()
            .get_loop(body.body().get_face(f).unwrap().outer)
            .unwrap()
            .boundary
        else {
            return false;
        };
        let pts: Vec<_> = body
            .body()
            .loop_cycle(first)
            .unwrap()
            .into_iter()
            .map(|he| {
                let v = body.body().get_half_edge(he).unwrap().start;
                *body
                    .body()
                    .get_point(body.body().get_vertex(v).unwrap().point)
                    .unwrap()
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
            }) => match body
                .body()
                .get_surface(body.body().get_face(*f).unwrap().surface)
            {
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
    // The plane `y = 0.9`, its slide off the floor the variable `cut`
    // an edit can move.
    let (doc, target) = operand_doc(PENTAGON, Solid::Prism);
    let (doc, _) = fixture::step(
        doc,
        editor_core::DocEdit::DeclareVar {
            name: editor_core::VarName::from_static("cut"),
            def: editor_core::VarDecl::Free(editor_core::FreeVar::continuous(
                editor_core::Dimension::Length,
                0.9,
            )),
        },
    );
    let cut = doc.var_named("cut").expect("declared");
    let tool = match plane_tool(&doc, target, Solid::Prism, [0.0, 0.9, 0.0], [0.0, 1.0, 0.0]) {
        editor_core::pose::RoleRead {
            read: editor_core::Operand::Pose(def),
            role: None,
        } => match *def {
            editor_core::pose::PoseDef::Slide { pose, along, .. } => {
                editor_core::pose::RoleRead::plain(editor_core::Operand::Pose(Box::new(
                    editor_core::pose::PoseDef::Slide {
                        pose,
                        along,
                        by: Formula::var(cut, editor_core::Dimension::Length),
                    },
                )))
            }
            other => panic!("the plane is slid off the floor: {other:?}"),
        },
        other => panic!("the plane is a pose: {other:?}"),
    };
    let (doc, sp) = insert(
        doc,
        Node::Split {
            target: editor_core::Operand::output(target, 0).into(),
            tool,
        },
    );
    let before = fixture::run(&doc, &Default::default());
    assert_eq!(
        plane_pieces(&before, sp, SplitHalf::Above),
        vec![(None, 1)],
        "below the apex the upper half holds one section, bare"
    );
    let (moved, _) = fixture::step(
        doc.clone(),
        editor_core::DocEdit::SetVarValue {
            var: cut.into(),
            value: editor_core::FreeValue::Continuous(1.1),
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
        let (doc, _, sp) = split_doc(NOTCHED, Solid::PrismAlongZ, [0.0, 1.5, 0.0], normal);
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
        },
    );
    // A bore along +y: drawn on its own xy, its copy placed against the
    // U with its x along z and its y along x, so its normal is +y.
    let (doc, q) = insert(
        doc,
        Node::Profile(desc(vec![vec![
            (0.25, 0.5),
            (0.75, 0.5),
            (0.75, 1.5),
            (0.25, 1.5),
        ]])),
    );
    let (doc, bore) = insert(
        doc,
        Node::Extrude {
            profile: q.into(),
            distance: len(4.0),
        },
    );
    let (doc, bore) = against(
        doc,
        ext,
        bore,
        &axes_at([0.0, -1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
    );
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: editor_core::Operand::output(ext, 0).into(),
            tool: bore.into(),
        },
    );
    let plane = fixture::plane_on(
        &doc,
        editor_core::Operand::output(cut, 0),
        [0.0, 1.5, 0.0],
        [0.0, 1.0, 0.0],
    );
    let (doc, sp) = insert(
        doc,
        Node::Split {
            target: editor_core::Operand::output(cut, 0).into(),
            tool: plane,
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
        },
    );
    // The groove along +y from y = 0.5: drawn on its own xy, its copy
    // placed against the block with its x along z and its y along x.
    let (doc, q) = insert(
        doc,
        Node::Profile(desc(vec![square(1.0, 3.0), square(1.5, 2.5)])),
    );
    let (doc, groove) = insert(
        doc,
        Node::Extrude {
            profile: q.into(),
            distance: len(4.0),
        },
    );
    let (doc, groove) = against(
        doc,
        block,
        groove,
        &axes_at([0.0, 0.5, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
    );
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: editor_core::Operand::output(block, 0).into(),
            tool: groove.into(),
        },
    );
    let plane = fixture::plane_on(
        &doc,
        editor_core::Operand::output(cut, 0),
        [0.0, 2.0, 0.0],
        [0.0, 1.0, 0.0],
    );
    let (doc, sp) = insert(
        doc,
        Node::Split {
            target: editor_core::Operand::output(cut, 0).into(),
            tool: plane,
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
    assert_moat_split(square(1.5, 2.5), square(1.2, 2.8), &[], &[], None);
}

/// **Two sections bounded alike tie.** A square ring, its square
/// revolved a full turn about `y`, cut through its axis leaves each half
/// two squares, each bounded by the ring's same four faces: one `Tied`
/// row, never told apart by an order.
#[test]
fn a_ring_cut_through_its_axis_ties_its_two_sections() {
    const SQUARE: &[(f64, f64)] = &[(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)];
    let (doc, ring, sp) = split_doc(SQUARE, Solid::Turned, [0.0; 3], [1.0, 0.0, 0.0]);
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

/// The square `[a, b]²`, counter-clockwise.
fn square(a: f64, b: f64) -> Vec<(f64, f64)> {
    vec![(a, a), (b, a), (b, b), (a, b)]
}

/// The pocket-and-moat block of
/// [`an_island_in_a_ring_the_merge_made_cites_the_moat`], split at
/// `y = 1`: a 4 × 3 × 4 block, the pocket `[0.5, 3.5]²` down to `y = 1`
/// round the pin of outline `pin`, and a moat of outline `moat` round the
/// pin cut on down to `y = 0.5`, each outline drawn on its own `xy` and
/// placed against the block with its `x` along `z` and its `y` along `x`.
/// The evaluation, the cut's node and the split's.
fn moat_split(
    pin: Vec<(f64, f64)>,
    moat: Vec<(f64, f64)>,
) -> (Evaluation<f64>, RecipeNodeId, RecipeNodeId) {
    moat_split_in(square(0.5, 3.5), pin, moat)
}

/// [`moat_split`] with the pocket of outline `pocket`.
fn moat_split_in(
    pocket: Vec<(f64, f64)>,
    pin: Vec<(f64, f64)>,
    moat: Vec<(f64, f64)>,
) -> (Evaluation<f64>, RecipeNodeId, RecipeNodeId) {
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
        },
    );
    // A tool along +y from `y = floor`, round the pin.
    let tool = |doc, floor: f64, outer: Vec<(f64, f64)>| {
        let (doc, q) = insert(doc, Node::Profile(desc(vec![outer, pin.clone()])));
        let (doc, tool) = insert(
            doc,
            Node::Extrude {
                profile: q.into(),
                distance: len(4.0),
            },
        );
        against(
            doc,
            block,
            tool,
            &axes_at([0.0, floor, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
        )
    };
    let (doc, pocket) = tool(doc, 1.0, pocket);
    let (doc, moat) = tool(doc, 0.5, moat);
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
    let plane = fixture::plane_on(
        &doc,
        editor_core::Operand::output(cut, 0),
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
    );
    let (doc, sp) = insert(
        doc,
        Node::Split {
            target: editor_core::Operand::output(cut, 0).into(),
            tool: plane,
        },
    );
    (fixture::run(&doc, &Default::default()), cut, sp)
}

/// The names in `table` of `body`'s vertices at `p`: every row, a tie
/// among them, that holds one.
fn named_vertices_at<'t>(
    table: &'t editor_core::NameTable,
    body: &topo::Body<f64>,
    p: [f64; 3],
) -> Vec<&'t StableName> {
    let at = |e: &EntityRef| {
        let EntityKey::Vertex(v) = e.key else {
            return false;
        };
        let q = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        (q.x - p[0]).abs() < 1e-9 && (q.y - p[1]).abs() < 1e-9 && (q.z - p[2]).abs() < 1e-9
    };
    table
        .iter()
        .filter(|(_, entry)| {
            let held = match entry {
                Entry::Unique(e) => core::slice::from_ref(e),
                Entry::Tied(es) => es.as_slice(),
            };
            held.iter().any(at)
        })
        .map(|(n, _)| n)
        .collect()
}

/// An operand face, by a point of it and its normal ([`operand_face_on`]).
type Wall = ([f64; 3], [f64; 3]);

/// The split of [`moat_split`]'s block round the pin `pin`, with the moat
/// `moat`, whose corners touch the pocket's wall at `touches`: below, the
/// merged face is bounded by the moat's outer faces and the faces
/// `outside`, the pin by its own; above, the rim by `rim` (by default the
/// pocket's faces), the pin by its own. Each touch is the cut's corner, held once below as
/// `OnToolVertex` and joined away above, where the wall's foot runs
/// through it.
fn assert_moat_split(
    pin: Vec<(f64, f64)>,
    moat: Vec<(f64, f64)>,
    touches: &[[f64; 3]],
    outside: &[Wall],
    rim: Option<&[Wall]>,
) {
    let (ev, cut, sp) = moat_split(pin.clone(), moat.clone());
    assert!(ev.value(sp).is_some(), "{:?}", ev.node_error(sp));
    let cut_body = ev.value(cut).unwrap().payload.output_body(0).unwrap();
    let cut_table = ev.table(ev.output_var(cut, 0).unwrap()).unwrap();
    let half = |side: SplitHalf| {
        let body = ev
            .value(sp)
            .unwrap()
            .payload
            .output_body(side.output_body())
            .unwrap();
        (fixture::half(&ev, sp, side), body)
    };
    let (below_table, below_body) = half(SplitHalf::Below);
    let (above_table, above_body) = half(SplitHalf::Above);
    for &p in touches {
        let [corner] = named_vertices_at(cut_table, cut_body.body(), p)[..] else {
            panic!("the cut holds the touch at {p:?} as one vertex");
        };
        let want = StableName {
            kind: editor_core::EntityKind::Vertex,
            node: sp,
            path: vec![RoleSeg::OnToolVertex {
                of: editor_core::NameRef::new(corner.clone()),
            }],
        };
        assert_eq!(
            named_vertices_at(below_table, below_body.body(), p),
            vec![&want],
            "below: the touch at {p:?} is the cut's corner, held once"
        );
        assert!(
            named_vertices_at(above_table, above_body.body(), p).is_empty(),
            "above: the pocket wall's foot runs through {p:?} unbroken"
        );
    }
    let face = |p, n| operand_face_on(&ev, cut, p, n);
    // A tool's faces round `outline` at height `y`, each through its
    // edge's midpoint, an outline point `(u, v)` placed at `x = v`, `z = u`.
    let walls = |outline: &[(f64, f64)], y: f64| {
        sorted(
            (0..outline.len())
                .map(|i| {
                    let (a, b) = (outline[i], outline[(i + 1) % outline.len()]);
                    let (du, dv) = (b.0 - a.0, b.1 - a.1);
                    let l = du.hypot(dv);
                    face(
                        [0.5 * (a.1 + b.1), y, 0.5 * (a.0 + b.0)],
                        [du / l, 0.0, -dv / l],
                    )
                })
                .collect(),
        )
    };
    let pieces = |side| {
        let mut got: Vec<_> = plane_pieces(&ev, sp, side)
            .into_iter()
            .map(|(walls, c)| (walls.expect("two pieces, each qualified"), c))
            .collect();
        got.sort();
        got
    };
    let mut merged = walls(&moat, 0.75);
    merged.extend(outside.iter().map(|&(p, n)| face(p, n)));
    let mut below = vec![(sorted(merged), 1), (walls(&pin, 0.75), 1)];
    below.sort();
    assert_eq!(
        pieces(SplitHalf::Below),
        below,
        "below: the merged face by the moat's outer faces, the pin by its own"
    );
    let rim = match rim {
        Some(rim) => sorted(rim.iter().map(|&(p, n)| face(p, n)).collect()),
        None => walls(&square(0.5, 3.5), 2.0),
    };
    let mut above = vec![(rim, 1), (walls(&pin, 2.0), 1)];
    above.sort();
    assert_eq!(
        pieces(SplitHalf::Above),
        above,
        "above: the rim by the pocket's faces, the pin by its own"
    );
}

/// **An operand corner on the plane is a corner, whatever made it.** A
/// diamond moat touching the pocket's wall at its four corners: the
/// subtract makes each touch by splitting the pocket floor's edge along
/// the wall, and the split, whose plane holds that edge, holds the
/// vertex as the corner it is, never as a crossing of the edge.
#[test]
fn a_diamond_moat_holds_its_touches_on_the_pocket_wall_as_corners() {
    assert_moat_split(
        square(1.5, 2.5),
        vec![(2.0, 0.5), (3.5, 2.0), (2.0, 3.5), (0.5, 2.0)],
        &[
            [0.5, 1.0, 2.0],
            [2.0, 1.0, 3.5],
            [3.5, 1.0, 2.0],
            [2.0, 1.0, 0.5],
        ],
        &[],
        None,
    );
}

/// A kite moat reaching the pocket's wall at one corner, its others
/// clear of it.
#[test]
fn a_kite_moat_holds_its_one_touch_on_the_pocket_wall_as_a_corner() {
    assert_moat_split(
        square(1.5, 2.5),
        vec![(2.0, 0.8), (3.2, 2.0), (2.0, 3.5), (0.8, 2.0)],
        &[[3.5, 1.0, 2.0]],
        &[],
        None,
    );
}

/// **Pieces that meet only at a point border one obstacle.** A diamond
/// moat reaching the block's walls at its corners divides both halves'
/// sections into four corner pieces round the pin, each pair meeting
/// where a corner meets a wall. Below, each corner piece cites the two
/// block walls it lies against and the moat's face across it; above,
/// also the pocket's two walls it lies against; the pin cites its own
/// faces.
#[test]
fn a_moat_reaching_the_block_walls_at_its_corners_quarters_both_sections() {
    let moat = vec![(2.0, 0.0), (4.0, 2.0), (2.0, 4.0), (0.0, 2.0)];
    let (ev, cut, sp) = moat_split(square(1.5, 2.5), moat);
    assert!(ev.value(sp).is_some(), "{:?}", ev.node_error(sp));
    let face = |p, n| operand_face_on(&ev, cut, p, n);
    let h = std::f64::consts::FRAC_1_SQRT_2;
    let pin = |y| {
        sorted(vec![
            face([1.5, y, 2.0], [1.0, 0.0, 0.0]),
            face([2.5, y, 2.0], [1.0, 0.0, 0.0]),
            face([2.0, y, 1.5], [0.0, 0.0, 1.0]),
            face([2.0, y, 2.5], [0.0, 0.0, 1.0]),
        ])
    };
    let pieces = |side| {
        let mut got: Vec<_> = plane_pieces(&ev, sp, side)
            .into_iter()
            .map(|(walls, c)| (walls.expect("five pieces, each qualified"), c))
            .collect();
        got.sort();
        got
    };
    for (side, y, pocket) in [
        (SplitHalf::Below, 0.75, false),
        (SplitHalf::Above, 2.0, true),
    ] {
        let mut want = vec![(pin(y), 1)];
        for (cx, cz) in [(0.0, 0.0), (0.0, 4.0), (4.0, 0.0), (4.0, 4.0)] {
            // Toward the corner `(cx, cz)` from the centre: +1 or -1.
            let (sx, sz): (f64, f64) = (
                if cx > 2.0 { 1.0 } else { -1.0 },
                if cz > 2.0 { 1.0 } else { -1.0 },
            );
            let mut walls = vec![
                face([cx, y, 2.0 + 1.75 * sz], [1.0, 0.0, 0.0]),
                face([2.0 + 1.75 * sx, y, cz], [0.0, 0.0, 1.0]),
                face([2.0 + sx, 0.75, 2.0 + sz], [h, 0.0, h * sx * sz]),
            ];
            if pocket {
                walls.push(face([2.0 + 1.5 * sx, y, 2.0 + sz], [1.0, 0.0, 0.0]));
                walls.push(face([2.0 + sx, y, 2.0 + 1.5 * sz], [0.0, 0.0, 1.0]));
            }
            want.push((sorted(walls), 1));
        }
        want.sort();
        assert_eq!(
            pieces(side),
            want,
            "{side:?}: each corner piece by the walls it lies against and the moat's face, \
             the pin by its own"
        );
    }
}

/// The moat of the row's second document: a diamond reaching the block's
/// wall at one corner and the pocket's wall at two, round a pin off the
/// pocket's centre. The two touches on the pocket's wall are the cut's
/// corners; the corner on the block's wall lies along the wall, an edge
/// the plane crosses. The moat meets the block's outside at that corner,
/// so in each half the two are one obstacle: below, the merged face cites
/// the block's walls too; above, the rim cites the block's walls, the
/// pocket's (its wall at `x = 0.5` in two, the moat's corner between) and
/// the moat's two faces across the rim.
#[test]
fn a_diamond_moat_reaching_the_block_wall_holds_its_touches_on_the_pocket_wall_as_corners() {
    let h = std::f64::consts::FRAC_1_SQRT_2;
    let block = [
        ([0.0, 0.75, 0.5], [1.0, 0.0, 0.0]),
        ([4.0, 0.75, 0.5], [1.0, 0.0, 0.0]),
        ([2.0, 0.75, 0.0], [0.0, 0.0, 1.0]),
        ([2.0, 0.75, 4.0], [0.0, 0.0, 1.0]),
    ];
    let mut rim = block.to_vec();
    rim.extend([
        ([0.5, 2.0, 1.0], [1.0, 0.0, 0.0]),
        ([0.5, 2.0, 3.0], [1.0, 0.0, 0.0]),
        ([3.5, 2.0, 2.0], [1.0, 0.0, 0.0]),
        ([2.5, 2.0, 0.5], [0.0, 0.0, 1.0]),
        ([2.5, 2.0, 3.5], [0.0, 0.0, 1.0]),
        ([0.75, 0.75, 1.25], [h, 0.0, h]),
        ([0.75, 0.75, 2.75], [h, 0.0, -h]),
    ]);
    assert_moat_split(
        vec![(1.5, 1.0), (2.5, 1.0), (2.5, 2.0), (1.5, 2.0)],
        vec![(2.0, 0.0), (3.5, 1.5), (2.0, 3.0), (0.5, 1.5)],
        &[[1.5, 1.0, 0.5], [1.5, 1.0, 3.5]],
        &block,
        Some(&rim),
    );
}

/// The diamond of half-diagonal `r` about `(2, 2)`, counter-clockwise.
fn diamond(r: f64) -> Vec<(f64, f64)> {
    vec![
        (2.0, 2.0 - r),
        (2.0 + r, 2.0),
        (2.0, 2.0 + r),
        (2.0 - r, 2.0),
    ]
}

/// [`moat_split_in`] with the pocket the diamond whose corners touch
/// the block's four walls, round the pin `[1.5, 2.5]²`, and the moat
/// `moat` of area `area`: each half builds at its closed-form volume and
/// passes tier 3′ with no contact records. At each pocket corner the rim
/// above the floor is two wedges meeting along a line, two coincident
/// edges in the block's wall; the wall wraps round their foot on the
/// plane, a full turn.
fn assert_diamond_pocket_split(moat: Vec<(f64, f64)>, union_area: f64, moat_area: f64) {
    let (ev, _, sp) = moat_split_in(diamond(2.0), square(1.5, 2.5), moat.clone());
    assert!(ev.value(sp).is_some(), "{moat:?}: {:?}", ev.node_error(sp));
    // Above, the block's section less the pocket and moat round the
    // pin, over `y ∈ [1, 3]`; below, the block less the moat round the
    // pin, over `y ∈ [0.5, 1]`.
    for (side, want) in [
        (SplitHalf::Above, 2.0 * (16.0 - (union_area - 1.0))),
        (SplitHalf::Below, 16.0 - 0.5 * (moat_area - 1.0)),
    ] {
        let body = ev
            .value(sp)
            .unwrap()
            .payload
            .output_body(side.output_body())
            .unwrap()
            .body();
        let finished = topo::test_support::finished(
            "the split's half",
            topo::Body::clone(body),
            Tol::witness(),
        );
        let got =
            topo::test_support::pinned_volume("the split's half", &finished, Tol::witness()).volume;
        assert!(
            (got - want).abs() < 1e-9,
            "{moat:?} {side:?}: volume {got}, want {want}"
        );
        assert_eq!(
            topo::validate_pseudomanifold(body, &topo::ContactRecords::default(), Tol::witness()),
            Ok(()),
            "{moat:?} {side:?}: tier 3′"
        );
    }
}

/// **A split along a pocket floor holds the wall's full turn round a
/// pinch line's foot.** The issue's first moat, the square
/// `[1.2, 2.8]²` inside the pocket.
#[test]
fn a_split_along_a_diamond_pocket_floor_touching_the_block_walls_builds() {
    assert_diamond_pocket_split(square(1.2, 2.8), 8.0, 2.56);
}

/// [`a_split_along_a_diamond_pocket_floor_touching_the_block_walls_builds`]
/// over the other moats: a square crossing the pocket's walls, diamonds
/// inside it, and the square that takes the whole rim.
#[test]
fn every_moat_round_a_diamond_pocket_touching_the_block_walls_splits_along_its_floor() {
    // The pocket and `[0.5, 3.5]²` overlap in the pocket less its four
    // tips beyond the square, each a quarter: 8 + 9 − 7.
    assert_diamond_pocket_split(square(0.5, 3.5), 10.0, 9.0);
    for r in [1.2, 1.5, 1.75] {
        assert_diamond_pocket_split(diamond(r), 8.0, 2.0 * r * r);
    }
    assert_diamond_pocket_split(square(0.0, 4.0), 16.0, 16.0);
}
