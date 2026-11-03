//! Rows for the booleans' [`Holdings`](super::Holdings) and the
//! [`overlap`] door.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use geom_core::{Band, Tol};

use super::holdings::Held;
use super::overlap::{OverlapRefusal, overlap};
use super::{BooleanResult, MemberFace};
use crate::body::Body;
use crate::entity::FaceKey;
use crate::test_support::holdings::{rejoin_oracle, union_in};
use crate::test_support::meeting::orders;
use crate::test_support::{brick, finished, prism_z};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// The face of a prism over `profile`, z 0 to 1 (`up`) or 1 to 2, on
/// the plane z = 1: its top, facing +z, or its bottom, facing −z.
fn at_one(profile: &[(f64, f64)], up: bool) -> (Body<f64>, FaceKey) {
    if up {
        let p = prism_z::<f64>(profile, 0.0, 1.0, tol());
        (p.body, p.top_face)
    } else {
        let p = prism_z::<f64>(profile, 1.0, 2.0, tol());
        (p.body, p.bottom_face)
    }
}

fn rect(x: (f64, f64), y: (f64, f64)) -> Vec<(f64, f64)> {
    vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]
}

/// The corners of a regular polygon about the origin, radius 1, at
/// every `step`-th of `n` angles, counterclockwise.
fn regular(n: usize, step: usize) -> Vec<(f64, f64)> {
    (0..n)
        .step_by(step)
        .map(|i| {
            let a = core::f64::consts::TAU * i as f64 / n as f64;
            (a.cos(), a.sin())
        })
        .collect()
}

/// `overlap` of the tops over `x` and `y`, read both ways round, which
/// must agree.
fn overlaps(x: &[(f64, f64)], y: &[(f64, f64)]) -> Result<bool, OverlapRefusal> {
    let (bx, fx) = at_one(x, true);
    let (by, fy) = at_one(y, true);
    let there = overlap((&bx, fx), (&by, fy), band());
    let back = overlap((&by, fy), (&bx, fx), band());
    match (&there, &back) {
        (Ok(a), Ok(b)) => assert_eq!(a, b, "overlap reads alike in either order"),
        _ => assert_eq!(
            there.is_ok(),
            back.is_ok(),
            "overlap answers or refuses alike in either order: {there:?} / {back:?}"
        ),
    }
    there
}

/// **`overlap` finds each kind of witness, and none where the regions
/// share no area.**
#[test]
fn overlap_reads_each_witness_kind_and_no_area_as_false() {
    let yes = |what: &str, x: Vec<(f64, f64)>, y: Vec<(f64, f64)>| {
        assert!(overlaps(&x, &y).unwrap(), "{what}: the faces meet in area");
    };
    let no = |what: &str, x: Vec<(f64, f64)>, y: Vec<(f64, f64)>| {
        assert!(
            !overlaps(&x, &y).unwrap(),
            "{what}: the faces share no area"
        );
    };
    yes(
        "a vertex strictly inside",
        rect((0.0, 2.0), (0.0, 2.0)),
        rect((1.0, 3.0), (1.0, 3.0)),
    );
    yes(
        "crossings alone: a cross, no vertex of either inside the other",
        rect((0.0, 3.0), (1.0, 2.0)),
        rect((1.0, 2.0), (0.0, 3.0)),
    );
    yes(
        "crossings alone, every whole edge's middle outside the other face",
        rect((0.0, 10.0), (0.0, 1.0)),
        rect((3.0, 4.0), (-10.0, 1.5)),
    );
    yes(
        "a triangle inscribed in a hexagon: no vertex inside, no crossing",
        regular(6, 2),
        regular(6, 1),
    );
    yes(
        "a piece along its source face's edges from inside",
        rect((0.0, 0.8), (0.0, 2.0)),
        rect((0.0, 1.0), (0.0, 2.0)),
    );
    yes(
        "one face",
        rect((0.0, 1.0), (0.0, 1.0)),
        rect((0.0, 1.0), (0.0, 1.0)),
    );
    no(
        "apart",
        rect((0.0, 1.0), (0.0, 1.0)),
        rect((2.0, 3.0), (0.0, 1.0)),
    );
    no(
        "touching at a corner",
        rect((0.0, 1.0), (0.0, 1.0)),
        rect((1.0, 2.0), (1.0, 2.0)),
    );
    no(
        "sharing an edge from either side",
        rect((0.0, 1.0), (0.0, 1.0)),
        rect((1.0, 2.0), (0.0, 1.0)),
    );
    no(
        "a vertex on the other's edge, from outside",
        rect((0.0, 1.0), (0.0, 1.0)),
        vec![(1.0, 0.5), (2.0, 0.0), (2.0, 1.0)],
    );
}

/// **Two faces of opposite sense refuse**: the door reads both faces'
/// sides about one normal, which only one sense makes the same.
#[test]
fn overlap_refuses_faces_of_opposite_sense() {
    for (what, x, y) in [
        (
            "one square",
            rect((0.0, 1.0), (0.0, 1.0)),
            rect((0.0, 1.0), (0.0, 1.0)),
        ),
        (
            "a sub-square",
            rect((0.0, 2.0), (0.0, 2.0)),
            rect((0.5, 1.0), (0.5, 1.0)),
        ),
        (
            "an edge shared",
            rect((0.0, 1.0), (0.0, 1.0)),
            rect((1.0, 2.0), (0.0, 1.0)),
        ),
    ] {
        let (bx, fx) = at_one(&x, true);
        let (by, fy) = at_one(&y, false);
        for got in [
            overlap((&bx, fx), (&by, fy), band()),
            overlap((&by, fy), (&bx, fx), band()),
        ] {
            assert!(
                matches!(got, Err(OverlapRefusal::OppositeSenses)),
                "{what}: a top and a bottom on one plane refuse: {got:?}"
            );
        }
    }
}

/// **A margin in band refuses typed where no witness decides**: a face
/// whose edge lies in the band off the other's edge, neither on it nor
/// clear of it.
#[test]
fn overlap_refuses_a_face_in_band_of_the_others_edge() {
    let t = tol();
    let gap = t.eps() * (1.0 + t.k()) / 2.0;
    let got = overlaps(
        &rect((0.0, 1.0), (0.0, 1.0)),
        &rect((1.0 + gap, 2.0), (0.0, 1.0)),
    );
    assert!(
        matches!(
            got,
            Err(OverlapRefusal::Placement(_) | OverlapRefusal::Meeting(_))
        ),
        "an edge {gap} off the other face's edge is in band: {got:?}"
    );
}

/// **A reading in band does not hide a witness that decides**: a
/// triangle mostly inside a rectangle, one corner in the band outside
/// its edge, meets it in area whichever face is read first and wherever
/// its loop starts; so does a rectangle whose first edge runs in the
/// band beside the other's.
#[test]
fn overlap_answers_where_a_witness_decides_beside_a_reading_in_band() {
    let t = tol();
    let gap = t.eps() * (1.0 + t.k()) / 2.0;
    let r = rect((0.0, 2.0), (0.0, 1.0));
    let (p, q, s) = ((0.5, 0.2), (1.5, 0.2), (1.0, 1.0 + gap));
    for tri in [vec![p, q, s], vec![s, p, q]] {
        assert!(
            overlaps(&tri, &r).unwrap(),
            "the triangle {tri:?} meets the rectangle in area"
        );
    }
    // The first edge each face's loop reads runs in band beside the
    // other's, and refuses; a later piece lies inside.
    let low = rect((0.5, 1.5), (-gap, 0.5));
    assert!(
        overlaps(&low, &r).unwrap(),
        "a face whose first edge runs in band beside the other's, a later piece inside it"
    );
}

/// The faces of `body` on the plane z = `z` facing +z, read through the
/// face-normal door.
fn tops_at(body: &Body<f64>, z: f64) -> Vec<FaceKey> {
    facing_at(body, 2, z)
}

/// The faces of `body` on the plane `axis` = `at` facing up that axis.
fn facing_at(body: &Body<f64>, axis: usize, at: f64) -> Vec<FaceKey> {
    let coord = |p: geom_core::Point3<f64>| [p.x, p.y, p.z][axis];
    body.faces()
        .filter(|&(k, f)| {
            let Some(n) = crate::face_normal::face_outward_normal(body, k) else {
                return false;
            };
            let n = n.vec();
            [n.x, n.y, n.z][axis] > 0.5
                && matches!(body.get_surface(f.surface),
                    Some(crate::Surface::Plane { origin, .. }) if (coord(*origin) - at).abs() < 1e-9)
        })
        .map(|(k, _)| k)
        .collect()
}

/// **The pair door reads a sunk coplanar tile's top as held through the
/// substitution**: the union of a brick and a tile sunk under its top
/// has one top face, which holds both tops, A's and B's.
#[test]
fn the_pair_door_holds_a_sunk_tiles_top_through_its_substitution() {
    let t = tol();
    let p = finished("P", brick::<f64>((0.0, 1.0), (0.0, 2.0), (0.0, 1.0), t), t);
    let tile = finished("T", brick::<f64>((0.2, 0.6), (0.5, 1.5), (0.5, 1.0), t), t);
    let Ok(BooleanResult::Body(out)) = super::union(&p, &tile, t) else {
        panic!("the union builds");
    };
    let root_b = |f| out.naming.face_fragments_b.root(f);
    assert!(
        out.naming
            .substituted
            .iter()
            .any(|s| tops_at(&tile, 1.0).contains(&root_b(s.face))),
        "the tile's top reaches the result through a substitution row"
    );
    let finished_tops = tops_at(&out.body, 1.0);
    assert_eq!(finished_tops.len(), 1, "one top face");
    assert_eq!(
        out.holdings.of(finished_tops[0]),
        Some(&Held::Faces(BTreeSet::from([
            (0, tops_at(&p, 1.0)[0]),
            (1, tops_at(&tile, 1.0)[0]),
        ]))),
        "the top holds the brick's top and the tile's"
    );
    assert_eq!(
        out.holdings.lost_rows().len(),
        out.naming.discards.len(),
        "one lost region per discard row"
    );
}

/// **The pair door reads an uncut row's kept face as holding the dropped
/// face, as it reads a substitution's** (rows restated by hand: no
/// fixture of the corpus makes an uncut row), and keys the region the dropped
/// face lost outside its kept faces to that face: the sunk tile's union
/// with its substitution rows restated as the uncut row a containment
/// fallback records where it cannot divide the tile's top.
#[test]
fn the_pair_door_reads_an_uncut_row_as_a_substitution() {
    let t = tol();
    let p = finished("P", brick::<f64>((0.0, 1.0), (0.0, 2.0), (0.0, 1.0), t), t);
    let tile = finished("T", brick::<f64>((0.2, 0.6), (0.5, 1.5), (0.5, 1.0), t), t);
    let Ok(BooleanResult::Body(mut out)) = super::union(&p, &tile, t) else {
        panic!("the union builds");
    };
    let tile_top = tops_at(&tile, 1.0)[0];
    let n = &mut out.naming;
    let (rows, kept): (Vec<_>, Vec<_>) = n.substituted.drain(..).partition(|s| {
        s.operand == super::Operand::B && n.face_fragments_b.root(s.face) == tile_top
    });
    n.substituted = kept;
    assert!(!rows.is_empty(), "the tile's top is substituted");
    n.uncut = rows
        .iter()
        .map(|s| super::UncutFace {
            operand: s.operand,
            face: tile_top,
            by: s.by,
            kind: super::BooleanErrorKind::JoinDesync,
            what: "restated for the row".to_owned(),
        })
        .collect();
    n.uncut.dedup();
    let holdings = super::Holdings::of_records(
        super::BooleanOp::Union,
        &out.naming,
        &out.body,
        ([&p, &tile], [None, None]),
        band(),
    );
    let top = tops_at(&out.body, 1.0)[0];
    assert_eq!(
        holdings.of(top),
        Some(&Held::Faces(BTreeSet::from([
            (0, tops_at(&p, 1.0)[0]),
            (1, tile_top),
        ]))),
        "the top holds the tile's top through the uncut row"
    );
    assert_eq!(
        holdings.uncut_rows(),
        vec![Held::Faces(BTreeSet::from([(1, tile_top)])); out.naming.uncut.len()],
        "each uncut row keys the tile's top"
    );
}

/// A prism over the `(x, y)` `profile`, z 0 to 2: a tile sunk under the
/// plane y = 1, its top there, joined by a bridge to a keel whose tip
/// line lies in that plane at x = `tip`.
fn tile_and_keel(mirror: bool) -> Body<f64> {
    let profile = [
        (0.2, 0.5),
        (0.6, 0.5),
        (0.6, 1.0),
        (1.3, 1.5),
        (1.5, 1.0),
        (1.7, 1.5),
        (1.7, 2.0),
        (0.4, 2.0),
        (0.4, 1.0),
        (0.2, 1.0),
    ];
    let mut profile: Vec<(f64, f64)> = profile.to_vec();
    if mirror {
        profile = profile.iter().rev().map(|&(x, y)| (2.0 - x, y)).collect();
    }
    prism_z::<f64>(&profile, 0.0, 2.0, tol()).body
}

/// **A face the parting divides after a substitution holds the dropped
/// face only where its region meets it**: a brick under a body holding a
/// tile sunk under the brick's face y = 1 and a keel whose tip lies in
/// that face from edge to edge. The keel parts the face in two, and of
/// the two only the piece over the tile holds the tile's top, the tile
/// on either side of the keel.
#[test]
fn a_face_parted_after_a_substitution_holds_the_tile_only_where_it_lies() {
    let t = tol();
    let block = finished(
        "block",
        brick::<f64>((0.0, 2.0), (0.0, 1.0), (0.0, 2.0), t),
        t,
    );
    for mirror in [false, true] {
        let other = finished("tile and keel", tile_and_keel(mirror), t);
        let Ok(BooleanResult::Body(out)) = super::union(&block, &other, t) else {
            panic!("mirror {mirror}: the union builds");
        };
        let tile_top = facing_at(&other, 1, 1.0);
        assert_eq!(tile_top.len(), 1, "mirror {mirror}: the tile has one top");
        let faces_up = facing_at(&out.body, 1, 1.0);
        assert_eq!(
            faces_up.len(),
            2,
            "mirror {mirror}: the keel parts the block's face in two"
        );
        let tile_x = if mirror { 1.7 } else { 0.3 };
        let block_top = (0, facing_at(&block, 1, 1.0)[0]);
        for f in faces_up {
            let xs: Vec<f64> = out
                .body
                .vertex_points()
                .filter(|&(v, _)| out.body.faces_of_vertex(v).unwrap().contains(&f))
                .map(|(_, p)| p.x)
                .collect();
            let (lo, hi) = xs
                .iter()
                .fold((f64::MAX, f64::MIN), |(a, b), &x| (a.min(x), b.max(x)));
            let over_tile = lo < tile_x && tile_x < hi;
            let mut want = BTreeSet::from([block_top]);
            if over_tile {
                want.insert((1, tile_top[0]));
            }
            assert_eq!(
                out.holdings.of(f),
                Some(&Held::Faces(want)),
                "mirror {mirror}: the piece over x {lo}..{hi} holds the tile's top: {over_tile}"
            );
        }
    }
}

/// **A subtract's finished face holds the tool's face it lies under**,
/// read against that face turned as the result holds it: a block less a
/// T whose leg parts the block's face y = 1 in two and whose cap's
/// underside lies on it, each piece holding the block's face and the
/// cap's underside over it.
#[test]
fn a_subtracts_pieces_hold_the_tools_face_they_lie_under() {
    let t = tol();
    let block = finished(
        "block",
        brick::<f64>((0.0, 2.0), (0.0, 1.0), (0.0, 2.0), t),
        t,
    );
    let tee = [
        (0.9, 0.5),
        (1.1, 0.5),
        (1.1, 1.0),
        (1.5, 1.0),
        (1.5, 1.5),
        (0.5, 1.5),
        (0.5, 1.0),
        (0.9, 1.0),
    ];
    let tee = finished("tee", prism_z::<f64>(&tee, 0.0, 2.0, t).body, t);
    let Ok(BooleanResult::Body(out)) = super::subtract(&block, &tee, t) else {
        panic!("the subtract builds");
    };
    let block_top = (0, facing_at(&block, 1, 1.0)[0]);
    let x_span = |body: &Body<f64>, f: FaceKey| {
        body.vertex_points()
            .filter(|&(v, _)| body.faces_of_vertex(v).unwrap().contains(&f))
            .fold((f64::MAX, f64::MIN), |(a, b), (_, p)| {
                (a.min(p.x), b.max(p.x))
            })
    };
    // The cap's underside: the tee's faces on y = 1 facing −y.
    let under: Vec<FaceKey> = tee
        .faces()
        .map(|(k, _)| k)
        .filter(|&k| {
            crate::face_normal::face_outward_normal(&tee, k).is_some_and(|n| n.vec().y < -0.5)
                && x_span(&tee, k).1 <= 1.5 + 1e-9
                && tee
                    .vertex_points()
                    .filter(|&(v, _)| tee.faces_of_vertex(v).unwrap().contains(&k))
                    .all(|(_, p)| (p.y - 1.0).abs() < 1e-9)
        })
        .collect();
    assert_eq!(
        under.len(),
        2,
        "the cap's underside is two faces, either side of the leg"
    );
    let pieces = facing_at(&out.body, 1, 1.0);
    assert_eq!(pieces.len(), 2, "the leg parts the block's face in two");
    for f in pieces {
        let (lo, hi) = x_span(&out.body, f);
        let mut want = BTreeSet::from([block_top]);
        for &u in &under {
            let (ulo, uhi) = x_span(&tee, u);
            if ulo < hi && lo < uhi {
                want.insert((1, u));
            }
        }
        assert_eq!(
            want.len(),
            2,
            "the piece over x {lo}..{hi} lies under one cap face"
        );
        assert_eq!(
            out.holdings.of(f),
            Some(&Held::Faces(want)),
            "the piece over x {lo}..{hi} holds the block's face and the cap's underside over it"
        );
    }
}

/// **A union's holdings read alike in every member order**, and match
/// the finished body: the planar probe's left top face holds P's top
/// and the sunk tile's, the right Q's alone.
#[test]
fn a_unions_holdings_read_alike_in_every_member_order() {
    let t = tol();
    let b = |x, y, z| finished("b", brick::<f64>(x, y, z, t), t);
    let bodies = vec![
        b((0.0, 1.0), (0.0, 2.0), (0.0, 1.0)),
        b((1.0, 2.0), (0.0, 2.0), (0.0, 1.0)),
        b((0.8, 1.2), (-1.0, 3.0), (0.5, 2.0)),
        b((0.2, 0.6), (0.5, 1.5), (0.5, 1.0)),
    ];
    let top_of = |i: usize| (i, tops_at(&bodies[i], 1.0)[0]);
    let mut tables = BTreeSet::new();
    let mut losses = BTreeSet::new();
    for order in orders(bodies.len()) {
        let (result, held, lost) = union_in(&bodies, &order, t);
        assert!(
            lost.contains(&top_of(0)) && lost.contains(&top_of(1)),
            "order {order:?}: P's top and Q's lose the strip under the bar"
        );
        assert!(
            !lost.contains(&top_of(3)),
            "order {order:?}: the tile's top loses no area, though the bar cuts a face that \
             holds it"
        );
        losses.insert(lost);
        let mut tops: Vec<BTreeSet<MemberFace>> = tops_at(&result, 1.0)
            .into_iter()
            .map(|f| held[&f].clone())
            .collect();
        tops.sort();
        assert_eq!(
            tops,
            vec![
                BTreeSet::from([top_of(0), top_of(3)]),
                BTreeSet::from([top_of(1)]),
            ],
            "order {order:?}: the left top holds P's top and T's, the right Q's"
        );
        rejoin_oracle(&bodies, &order, &result, &held, (true, t));
        let mut table: Vec<BTreeSet<MemberFace>> = held.into_values().collect();
        table.sort();
        tables.insert(table);
    }
    assert_eq!(tables.len(), 1, "one holdings table in every member order");
    assert_eq!(
        losses.len(),
        1,
        "the same member faces lose area in every member order"
    );
}

/// The plane `face` of `body` lies on: a point of it and its unit normal.
fn plane_of(body: &Body<f64>, face: FaceKey) -> ([f64; 3], [f64; 3]) {
    let f = body.get_face(face).expect("the face resolves");
    match body.get_surface(f.surface) {
        Some(crate::Surface::Plane { origin, normal, .. }) => {
            let n = normal.normalize();
            ([origin.x, origin.y, origin.z], [n.x, n.y, n.z])
        }
        other => panic!("a brick's face lies on a plane, not {other:?}"),
    }
}

/// Whether member face `m` of `member` lies on the plane of finished face
/// `f` of `body`, in either sense: read off the planes' data, not the
/// carrier ladder the door reads them through.
fn coplanar((body, f): (&Body<f64>, FaceKey), (member, m): (&Body<f64>, FaceKey)) -> bool {
    let ((o, n), (p, k)) = (plane_of(body, f), plane_of(member, m));
    let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let off = [p[0] - o[0], p[1] - o[1], p[2] - o[2]];
    (dot(n, k).abs() - 1.0).abs() < 1e-9 && dot(off, n).abs() < 1e-9
}

/// **Every face a boolean of bricks finishes holds only member faces on
/// its own plane**, and every holding reads: a plate, the plate less a
/// slot, and each of those subtracted from, joined to and met with a
/// grid of brick tools. A substitution row naming a face across a
/// corner (a fate edge's half read through the null scaffolding the
/// join sets between two pieces of the dropped face) made the cut slot's
/// floor hold a tool's side wall; the door now reads every row against
/// the geometry, so such a record refuses rather than names.
#[test]
fn every_brick_booleans_face_holds_only_member_faces_on_its_plane() {
    let t = tol();
    let plate = finished(
        "plate",
        brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 1.0), t),
        t,
    );
    let slot = finished(
        "slot",
        brick::<f64>((1.0, 2.0), (-1.0, 4.0), (0.5, 1.5), t),
        t,
    );
    let Ok(BooleanResult::Body(cut)) = super::subtract(&plate, &slot, t) else {
        panic!("the slot cuts the plate");
    };
    let cut = finished("cut", cut.body.clone().into_body(), t);
    let xs = [-1.0, 0.0, 1.0, 2.0, 3.0];
    let ys = [-1.0, 0.0, 1.0, 2.0];
    let spans = |at: &[f64]| -> Vec<(f64, f64)> {
        let mut out = Vec::new();
        for (i, &lo) in at.iter().enumerate() {
            for &hi in &at[i + 1..] {
                out.push((lo, hi));
            }
        }
        out
    };
    let ops: [(&str, BoolFn); 3] = [
        ("subtract", super::subtract),
        ("union", super::union),
        ("intersect", super::intersect),
    ];
    let (mut read, mut wrong) = (0, Vec::new());
    for (base_name, base) in [("plate", &plate), ("plate less slot", &cut)] {
        for x in spans(&xs) {
            for y in spans(&ys) {
                for z in [(0.5, 1.5), (0.25, 1.5)] {
                    let tool = finished("tool", brick::<f64>(x, y, z, t), t);
                    for (op_name, op) in ops {
                        let Ok(BooleanResult::Body(out)) = op(base, &tool, t) else {
                            continue;
                        };
                        let at = format!("{base_name} {op_name} brick {x:?}x{y:?}x{z:?}");
                        for (f, held) in out.holdings.faces() {
                            let Held::Faces(members) = held else {
                                wrong.push(format!("{at}: face {f:?} refuses: {held:?}"));
                                continue;
                            };
                            read += 1;
                            for &(place, m) in members {
                                let member = if place == 0 { base } else { &tool };
                                if !coplanar((&out.body, f), (member, m)) {
                                    wrong.push(format!(
                                        "{at}: face {f:?} holds member face {:?} off its plane",
                                        (place, m)
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "{} faces misread:\n{}",
        wrong.len(),
        wrong.join("\n")
    );
    assert!(read > 6_000, "the sweep reads the holdings of {read} faces");
}

type BoolFn = fn(
    &crate::validate::AtRestBody<f64>,
    &crate::validate::AtRestBody<f64>,
    Tol,
) -> Result<BooleanResult<f64>, super::BooleanError>;
