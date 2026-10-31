//! **A cut elsewhere on an edge leaves its other pieces' names as they
//! were** (N2's `Ends`, the locality the ruled rule buys). A piece is
//! named by its two ends, so a cut that does not touch it neither moves
//! its ends nor renames it; a rank over the pieces would have written
//! the group's new size into every one of them. A crossing is named by
//! its sense, so a second crossing by a face that already crosses the
//! edge leaves the first crossing's name as it was.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::blocks::{block, block_on, failure, run};
use crate::corpus::body_of;
use crate::fixture::{self, insert, len, point, step, table};
use editor_core::{
    Coord, DocEdit, EntityKey, Entry, Node, ProfileDoc, Qualifier, RoleSeg, SlotId, StableName,
    TargetAxis,
};
use geom_core::Tol;

/// The pieces of the seam the cut leaves along y = 1, z = 1, by their
/// x-span (to 1e-6).
fn seam_pieces(
    ev: &editor_core::Evaluation<f64>,
    cut: editor_core::RecipeNodeId,
) -> BTreeMap<(i64, i64), StableName> {
    let body = body_of(ev, cut);
    let micro = |x: f64| (x * 1e6).round() as i64;
    let mut out = BTreeMap::new();
    for (name, entry) in table(ev, cut).iter() {
        let (Entry::Unique(r), Some(RoleSeg::Fragment(Qualifier::Ends(_)))) =
            (entry, name.path.last())
        else {
            continue;
        };
        let EntityKey::Edge(e) = r.key else { continue };
        let edge = body.get_edge(e).unwrap();
        let ps = [edge.he_plus, edge.he_minus]
            .map(|he| point(&body, body.get_half_edge(he).unwrap().start));
        if ps
            .iter()
            .all(|p| (p.y - 1.0).abs() < 1e-9 && (p.z - 1.0).abs() < 1e-9)
        {
            let (x0, x1) = (ps[0].x.min(ps[1].x), ps[0].x.max(ps[1].x));
            out.insert((micro(x0), micro(x1)), name.clone());
        }
    }
    out
}

#[test]
fn a_second_cut_on_a_seam_leaves_the_other_pieces_names() {
    let doc = ProfileDoc::empty_derived("edge-piece-locality", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 3.0), 1.0);
    let (doc, rib) = block_on(doc, plate, (-1.0, 4.0), (1.0, 2.0), 0.5, 1.5);
    let (doc, joined) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![plate.into(), rib.into()]),
        },
    );
    // Two notches across the rib's y = 1 seam: the first in place, the
    // second parked away from everything until the edit slides it on.
    let (doc, n1) = block_on(doc, plate, (0.5, 0.7), (0.5, 1.5), 0.8, 0.7);
    let (doc, tr) = block_on(doc, plate, (2.0, 2.2), (4.5, 5.5), 0.8, 0.7);
    let (doc, notches) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![n1.into(), tr.into()]),
        },
    );
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: joined.into(),
            tool: notches.into(),
        },
    );
    let (doc2, _) = step(
        doc.clone(),
        DocEdit::SetParam {
            node: tr,
            // The value is a coordinate on the plate's own `y` (its start
            // cap is where the identity orientation puts the notch's):
            // this slides the notch 4 toward `−y`.
            slot: crate::fixture::value_slot(&doc, tr, Coord::Slide(TargetAxis::Y)),
            value: len(-4.0).into(),
            fresh: Vec::new(),
        },
    );
    let (ev, ev2) = (run(&doc), run(&doc2));
    for (label, ev) in [("one notch", &ev), ("two notches", &ev2)] {
        assert!(
            failure(ev, cut).is_none(),
            "{label}: {:?}",
            failure(ev, cut)
        );
    }
    let x = |a: f64, b: f64| ((a * 1e6).round() as i64, (b * 1e6).round() as i64);
    let (one, two) = (seam_pieces(&ev, cut), seam_pieces(&ev2, cut));
    assert_eq!(
        one.keys().copied().collect::<Vec<_>>(),
        vec![x(0.0, 0.5), x(0.7, 3.0)],
        "one notch cuts the seam in two"
    );
    assert_eq!(
        two.keys().copied().collect::<Vec<_>>(),
        vec![x(0.0, 0.5), x(0.7, 2.0), x(2.2, 3.0)],
        "the second cuts it in three"
    );
    assert_eq!(
        one[&x(0.0, 0.5)],
        two[&x(0.0, 0.5)],
        "the piece the second notch does not touch keeps its name"
    );
    assert_ne!(
        one[&x(0.7, 3.0)],
        two[&x(0.7, 2.0)],
        "the piece it cuts is renamed: one of its ends moved"
    );
}

/// **A second crossing by a face that already crosses the edge renames
/// neither the first crossing nor the pieces ending at it.** A crossing
/// is named by its sense (N2), a fact of the one vertex, and a second
/// crossing of the other sense leaves it as it was; a piece is named by
/// its ends, a lone one too, so the piece from P to 180° keeps its name
/// when a second Below piece of the rim appears.
///
/// One document, edited in place: a D-shaped rod (its profile an arc of
/// radius 0.3 from 0° round to 225° and the chord back, `Piece(0)` the
/// arc) split by a vertical plane through the rim point P at 135°. Its
/// other crossing is on the chord (toward 300°) until the plane's
/// normal is edited to put it at 30° (on the arc), so the plane crosses
/// the arc once and then twice, at P both times. The arc runs from 0°
/// to 225°, and the plane's normal puts 180° Below, so the rim enters
/// the Below half at P and, once crossed at 30°, leaves it there. (The
/// rod's wall is one curved face and one flat one, so no merge joins
/// the arc's Below pieces: a two-piece cylinder's wall pieces merge in
/// each half, and its rim is joined across the profile's seam.)
#[test]
fn a_second_crossing_by_the_same_face_keeps_the_first_crossing_and_its_pieces_names() {
    use editor_core::{Sense, SplitHalf};

    let at = |deg: f64| {
        let t = deg.to_radians();
        (0.3 * t.cos(), 0.3 * t.sin())
    };
    let p = at(135.0);
    // The plane through P and the rim point at `other`, its normal's
    // sign putting the rim point at 180° Below.
    let normal = |other: f64| {
        let q = at(other);
        let (dx, dy) = (q.0 - p.0, q.1 - p.1);
        let k = (dx * dx + dy * dy).sqrt();
        let (nx, ny) = (dy / k, -dx / k);
        if nx * (-0.3 - p.0) + ny * (0.0 - p.1) > 0.0 {
            (-nx, -ny)
        } else {
            (nx, ny)
        }
    };
    let doc = ProfileDoc::empty_derived("edge-piece-locality-twice", Tol::witness());
    let (doc, rod) = d_rod(doc, at(225.0));
    // The plane through P whose normal puts the rim point at `other`,
    // read off the rod's own geometry.
    let plane = |doc: &ProfileDoc, other: f64| {
        let (nx, ny) = normal(other);
        plane_read(doc, rod, [p.0, p.1, 0.0], [nx, ny, 0.0])
    };
    let tool = plane(&doc, 300.0);
    let (doc, cut) = insert(
        doc,
        Node::Split {
            target: rod.into(),
            tool: tool.into(),
        },
    );
    let (doc2, _) = step(
        doc.clone(),
        DocEdit::SetParam {
            node: cut,
            slot: SlotId::Operand(editor_core::OperandSlot::Tool),
            value: plane(&doc, 30.0).into(),
            fresh: Vec::new(),
        },
    );
    let (ev, ev2) = (run(&doc), run(&doc2));
    for (label, ev) in [("crossed once", &ev), ("crossed twice", &ev2)] {
        assert!(
            failure(ev, cut).is_none(),
            "{label}: {:?}",
            failure(ev, cut)
        );
    }
    // The Below crossings of the start rim (z = 0) on `Piece(0)`, by
    // where they lie.
    let below_crossings = |ev: &editor_core::Evaluation<f64>| {
        let below = match &ev.value(cut).unwrap().payload {
            editor_core::ValuePayload::Split { below, .. } => match below {
                editor_core::SplitSide::Body(b) => std::sync::Arc::clone(b.shared()),
                editor_core::SplitSide::Empty => panic!("a Below half"),
            },
            _ => panic!("a split value"),
        };
        let mut out = Vec::new();
        for (n, e) in crate::fixture::half(ev, cut, SplitHalf::Below).iter() {
            let (Entry::Unique(r), Some(RoleSeg::CrossingVertex { .. })) = (e, n.path.first())
            else {
                continue;
            };
            let EntityKey::Vertex(v) = r.key else {
                continue;
            };
            let q = point(&below, v);
            if q.z.abs() < 1e-9 && q.y > -1e-9 {
                out.push(((q.x, q.y), n.clone()));
            }
        }
        out
    };
    let near = |q: (f64, f64), r: (f64, f64)| (q.0 - r.0).abs() < 1e-9 && (q.1 - r.1).abs() < 1e-9;
    let (once, twice) = (below_crossings(&ev), below_crossings(&ev2));
    assert_eq!(once.len(), 1, "crossed once: {once:?}");
    assert_eq!(twice.len(), 2, "crossed twice: {twice:?}");
    let at_p = |crossings: &[((f64, f64), StableName)]| {
        crossings
            .iter()
            .find(|(q, _)| near(*q, p))
            .map(|(_, n)| n.clone())
            .expect("a Below crossing at P")
    };
    let (p_once, p_twice) = (at_p(&once), at_p(&twice));
    assert_eq!(p_once, p_twice, "the crossing at P keeps its name");
    // The rim's pieces ending at P, by their `Ends`: one Below piece,
    // P to 180°, once the rim is crossed once and once it is crossed
    // twice. (The cap's section chord ends at P too, and at the other
    // crossing, which the edit moved.)
    let ending_at = |ev: &editor_core::Evaluation<f64>, vertex: &StableName| {
        crate::fixture::half(ev, cut, SplitHalf::Below)
            .iter()
            .filter(|(n, _)| {
                matches!(n.path.first(), Some(RoleSeg::SplitFragment { .. }))
                    && matches!(n.path.last(), Some(RoleSeg::Fragment(Qualifier::Ends(ends))) if ends.contains(vertex))
            })
            .map(|(n, _)| n.clone())
            .collect::<Vec<_>>()
    };
    let (ending_once, ending_twice) = (ending_at(&ev, &p_once), ending_at(&ev2, &p_twice));
    assert_eq!(
        ending_once.len(),
        1,
        "one rim piece ends at P: {ending_once:?}"
    );
    assert_eq!(
        ending_once, ending_twice,
        "the rim piece ending at P keeps its name"
    );
    assert!(
        matches!(
            p_once.path.as_slice(),
            [RoleSeg::CrossingVertex {
                sense: Sense::Enters,
                ..
            }]
        ),
        "the rim enters the Below half at P, and nothing ranks it: {p_once:?}"
    );
    let second = twice
        .iter()
        .find(|(q, _)| near(*q, at(30.0)))
        .map(|(_, n)| n)
        .expect("a Below crossing at 30°");
    assert!(
        matches!(
            second.path.as_slice(),
            [RoleSeg::CrossingVertex {
                sense: Sense::Leaves,
                ..
            }]
        ),
        "the rim leaves the Below half at 30°: {second:?}"
    );
}

/// **The plane through `origin` normal to `normal`, read off `body`**, as
/// one `Plane` read: the frame [`fixture::plane_on_frame`] reads (the
/// normal is tilted off every face), through its `xy`, slid by the
/// constant zero along its `z` — the spelling a split's tool slot is
/// re-pointed at.
fn plane_read(
    doc: &ProfileDoc,
    body: editor_core::RecipeNodeId,
    origin: [f64; 3],
    normal: [f64; 3],
) -> editor_core::Operand {
    use editor_core::pose::{PoseDef, PoseRole, RoleRead};
    let frame = fixture::plane_on_frame(doc, body, origin, normal).read;
    editor_core::Operand::Pose(Box::new(PoseDef::Slide {
        pose: RoleRead::through(frame.clone(), PoseRole::Xy),
        along: RoleRead::through(frame, PoseRole::Z),
        by: editor_core::Formula::zero(editor_core::Dimension::Length),
    }))
}

/// A rod of length 1 along `z` whose profile is the arc of radius 0.3
/// about the origin from 0° counter-clockwise to `end`, closed by the
/// chord back.
pub(crate) fn d_rod(doc: ProfileDoc, end: (f64, f64)) -> (ProfileDoc, editor_core::RecipeNodeId) {
    use editor_core::{LoopProgram, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget};
    let (doc, d) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Chain(vec![
                ProgramStep::At([len(0.3), len(0.0)]),
                ProgramStep::ArcTo(ProgramArcData::Center {
                    c: [len(0.0), len(0.0)],
                    winding: profile::ArcSweep::Ccw,
                    target: ProgramTarget::Point([len(end.0), len(end.1)]),
                }),
                ProgramStep::LineTo(ProgramTarget::Start),
            ])],
            ids: Vec::new(),
            on: None,
        }),
    );
    insert(
        doc,
        Node::Extrude {
            profile: d.into(),
            distance: len(1.0),
        },
    )
}
