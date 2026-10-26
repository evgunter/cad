//! **A curved operand's coincident faces are cut alike in either operand
//! order** (`topo::boolean::fate`): each substituted fragment is held by a
//! face the result keeps, never by a discard; the rows and the discards'
//! borders, read by the operand faces they name, do not depend on which
//! operand is A; and a hemisphere's base, a disc bounded by one closed
//! circle, is cut where a brick's coincident bottom holds it.
//!
//! Faces are named by their carrier (`topo::face_carrier`), rounded to
//! thousandths; a border by the live edge it settles on, by its two ends.
//! A stretch that settles on no live edge merged away with the faces
//! beside it and says nothing, so it is left out.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::common::cavity::rod;
use geom_core::{Point2, RigidMotion, Tol, UnitVec3};
use sweep::test_support::{brick, finished, hemisphere_on_flat_base};
use topo::{AtRestBody, BooleanBody, BooleanResult, BooleanResultKind, Cell, FaceKey, Operand};

/// `s` with each decimal number in it rounded to thousandths.
fn rounded(s: &str) -> String {
    let mut out = String::new();
    let mut num = String::new();
    let flush = |num: &mut String, out: &mut String| {
        if let Ok(x) = num.parse::<f64>()
            && num.contains('.')
        {
            let r = (x * 1000.0).round() / 1000.0;
            out.push_str(&format!("{}", if r == 0.0 { 0.0 } else { r }));
        } else {
            out.push_str(num);
        }
        num.clear();
    };
    for c in s.chars() {
        let exponent = c == 'e' && !num.is_empty();
        let sign = c == '-' && (num.is_empty() || num.ends_with('e'));
        if c.is_ascii_digit() || c == '.' || exponent || sign {
            num.push(c);
        } else {
            flush(&mut num, &mut out);
            out.push(c);
        }
    }
    flush(&mut num, &mut out);
    out
}

/// `f`, a face of a pristine operand, by its carrier.
fn seat(body: &topo::Body<f64>, f: FaceKey) -> String {
    rounded(&format!(
        "{:?}",
        topo::face_carrier(body, f).expect("a carrier")
    ))
}

/// The operand face `f`, a clone face of `operand`, is a fragment of.
fn root(out: &BooleanBody<f64>, operand: Operand, f: FaceKey) -> FaceKey {
    match operand {
        Operand::A => out.naming.face_fragments_a.root(f),
        Operand::B => out.naming.face_fragments_b.root(f),
    }
}

/// Each stretch's live edge, by its two ends in thousandths, sorted;
/// a stretch no live edge holds is left out.
fn live(out: &BooleanBody<f64>, rows: &[(topo::VertexKey, topo::VertexKey)]) -> Vec<String> {
    let body = &out.body;
    let settler = out.naming.settler(body);
    let at = |v| {
        let p = topo::readback::vertex_point(body, v).expect("a live end");
        rounded(&format!("({:.3},{:.3},{:.3})", p.x, p.y, p.z))
    };
    let ends = |e: topo::EdgeKey| {
        let d = body.get_edge(e).expect("a live edge");
        [d.he_plus, d.he_minus].map(|h| body.get_half_edge(h).expect("a live half").start)
    };
    let mut v: Vec<String> = rows
        .iter()
        .filter_map(|&(u, w)| {
            let (u, w) = (
                settler.settle(u).expect("settles"),
                settler.settle(w).expect("settles"),
            );
            let between = match (u, w) {
                (Cell::Vertex(u), Cell::Vertex(w)) => body.edges().map(|(k, _)| k).find(|&k| {
                    let [s, t] = ends(k);
                    (s, t) == (u, w) || (s, t) == (w, u)
                }),
                _ => None,
            };
            let e = between.or_else(|| out.naming.stretch_through_joins(body, (u, w)))?;
            let mut s = ends(e).map(at);
            s.sort();
            Some(format!("{} {}", s[0], s[1]))
        })
        .collect();
    v.sort();
    v
}

/// What one boolean says about its coincident region (as
/// `topo`'s `boolean_covered::OpFates`, by carrier): its path, volume,
/// rows as unordered pairs of operand faces and their count, discards
/// with their borders, and how many rows a discard holds.
#[derive(Debug, PartialEq, Eq)]
struct OpFates {
    kind: Option<BooleanResultKind>,
    volume: i64,
    rows: usize,
    held: BTreeSet<(String, String)>,
    lost: Vec<(String, Vec<String>)>,
    held_by_a_discard: usize,
}

fn op_fates(op: &str, x: &AtRestBody<f64>, y: &AtRestBody<f64>) -> OpFates {
    let tol = Tol::witness();
    let r = match op {
        "∪" => topo::union(x, y, tol),
        "∩" => topo::intersect(x, y, tol),
        _ => topo::subtract(x, y, tol),
    };
    let BooleanResult::Body(out) = r.unwrap_or_else(|e| panic!("{op}: {e:?}")) else {
        return OpFates {
            kind: None,
            volume: 0,
            rows: 0,
            held: BTreeSet::new(),
            lost: Vec::new(),
            held_by_a_discard: 0,
        };
    };
    let face = |o: Operand, f| match o {
        Operand::A => seat(x, root(&out, o, f)),
        Operand::B => seat(y, root(&out, o, f)),
    };
    let discarded: BTreeSet<(Operand, FaceKey)> = out
        .naming
        .discards
        .iter()
        .map(|r| (r.operand, r.face))
        .collect();
    let mut lost: Vec<(String, Vec<String>)> = out
        .naming
        .discards
        .iter()
        .map(|r| {
            let all: Vec<_> = r
                .bordered
                .iter()
                .chain(&r.bordered_through)
                .copied()
                .collect();
            (face(r.operand, r.face), live(&out, &all))
        })
        .collect();
    lost.sort();
    OpFates {
        kind: Some(out.kind),
        volume: (topo::test_support::pinned_volume("mass", &out.body, tol).volume * 1e6).round()
            as i64,
        rows: out.naming.substituted.len(),
        held: out
            .naming
            .substituted
            .iter()
            .map(|s| {
                let (p, q) = (face(s.operand, s.face), face(s.operand.other(), s.by));
                if p < q { (p, q) } else { (q, p) }
            })
            .collect(),
        lost,
        held_by_a_discard: out
            .naming
            .substituted
            .iter()
            .filter(|s| discarded.contains(&(s.operand.other(), s.by)))
            .count(),
    }
}

/// `x` and `y` in every op say alike in either order (`x ∖ y` against
/// itself across the orders), and no row is held by a discard, whichever
/// path each order takes. With `seam`, the operands' coincident curved
/// walls meet where operand A's ends, so the union's discards border
/// that circle in one order and the other's in the other
/// ([`coaxial_rods_border_the_seam_circle_only_where_the_result_holds_it`]),
/// and the union's discards are not compared.
fn assert_alike(label: &str, x: &AtRestBody<f64>, y: &AtRestBody<f64>, seam: bool) {
    for (op, p, q) in [
        ("∪", op_fates("∪", x, y), op_fates("∪", y, x)),
        ("∩", op_fates("∩", x, y), op_fates("∩", y, x)),
        ("x ∖ y", op_fates("∖", x, y), op_fates("∖", x, y)),
        ("y ∖ x", op_fates("∖", y, x), op_fates("∖", y, x)),
    ] {
        assert_eq!(p.volume, q.volume, "{label} {op}: the volume");
        assert_eq!(
            (p.rows, &p.held),
            (q.rows, &q.held),
            "{label} {op}: the rows"
        );
        assert_eq!(
            (p.held_by_a_discard, q.held_by_a_discard),
            (0, 0),
            "{label} {op}: rows held by a discard"
        );
        if !(seam && op == "∪") {
            assert_eq!(
                p.lost, q.lost,
                "{label} {op}: the discards and their borders"
            );
        }
    }
}

fn fin(label: &str, body: topo::Body<f64>) -> AtRestBody<f64> {
    finished(label, body, Tol::witness())
}

/// **A hemisphere's base and a brick's bottom are cut alike in either
/// order.** The hemisphere of radius 1 stands on its base disc in y 0,
/// bounded by one closed circle; each brick stands on y 0 too. Where the
/// brick crosses the circle, the ∩ keeps the hemisphere's copy of the
/// coincident region and holds the brick's bottom there through the part
/// of the disc inside the brick; the disc's corner at a brick corner is
/// a wide sector, and the piece over the brick's corner is read along
/// the brick's sector, inside it, not the disc's, whose bisector points
/// out of the brick. Red before that reading: the brick's bottom was held
/// by the disc's piece outside the brick, which the ∩ discards. The slab
/// covers the disc: with the hemisphere as A the ∩ cuts the slab's
/// bottom along the closed circle, whose chord closes a ring at the
/// circle's vertex. Red before that ring was read along the chord's own
/// curve: the ∩ refused ("a fate cut closes a ring of no area"), where
/// the merge base built it, volume 1.439897.
#[test]
fn a_hemisphere_base_and_a_brick_bottom_are_cut_alike_in_either_order() {
    let tol = Tol::witness();
    let hemi = fin("hemi", hemisphere_on_flat_base(1.0, tol));
    for (label, x, z) in [
        ("across the rim", (0.0, 2.0), (-0.3, 0.3)),
        ("through the disc", (-0.5, 0.5), (-2.0, 2.0)),
        ("over the disc", (-2.0, 2.0), (-2.0, 2.0)),
    ] {
        let brick = fin(label, brick(x, (0.0, 0.5), z, tol));
        assert_alike(label, &hemi, &brick, false);
    }
    let slab = fin("slab", brick((-2.0, 2.0), (0.0, 0.5), (-2.0, 2.0), tol));
    assert_eq!(op_fates("∩", &hemi, &slab).volume, 1_439_897, "hemi ∩ slab");
}

/// **A brick on a hemisphere's base, inside it, is cut alike by the
/// containment fallback.** The brick x, z −0.3..0.3, y 0..0.5 stands on
/// the base disc inside the hemisphere of radius 1, so `hemi ∪ brick` and
/// `brick ∩ hemi` keep operand A whole, by the fallback, and the other
/// order of each takes the section path. The fallback cuts the disc along
/// the brick's bottom in the ∩ and records each face it drops as a
/// discard, as the section path does. Red before the fallback cut: it
/// recorded no discard, and the ∩'s row held the whole disc.
#[test]
fn a_brick_inside_on_a_hemispheres_base_is_cut_alike_by_the_fallback() {
    let tol = Tol::witness();
    let hemi = fin("hemi", hemisphere_on_flat_base(1.0, tol));
    let brick = fin("brick", brick((-0.3, 0.3), (0.0, 0.5), (-0.3, 0.3), tol));
    for (op, x, y) in [("∪", &hemi, &brick), ("∩", &brick, &hemi)] {
        assert_eq!(
            op_fates(op, x, y).kind,
            Some(BooleanResultKind::OperandA),
            "{op}: operand A whole, by the fallback"
        );
    }
    assert_alike("inside, on the disc", &hemi, &brick, false);
}

/// **A slanted rod and a coaxial rod are cut alike in either order.**
/// The slanted rod is the rod r 1 over z 0..3 less the half-space above a
/// plane tilted 0.4 rad about x through z 2; the other is coaxial and
/// overlaps it from z 1. Their walls coincide over the overlap, and meet
/// where operand A's ends, as the coaxial rods' do. Red
/// before the kept piece was read along the dropped sector: with the
/// coaxial rod as A, the ∩ wrote three rows against two, one of them held
/// by the slanted rod's wall below z 1, which the ∩ discards.
#[test]
fn a_slanted_rod_and_a_coaxial_rod_are_cut_alike_in_either_order() {
    let tol = Tol::witness();
    let tilt = RigidMotion::rotation(UnitVec3::exact_x(), 0.4).expect("a rotation");
    let cutter = fin("cutter", brick((-3.0, 3.0), (-3.0, 3.0), (2.0, 6.0), tol));
    let cutter = fin(
        "cutter",
        topo::transform_rigid(&cutter, &tilt, tol).expect("moved"),
    );
    let whole = fin("rod", rod(Point2::new(0.0, 0.0), 1.0, 0.0, 3.0));
    let BooleanResult::Body(slanted) = topo::subtract(&whole, &cutter, tol).expect("the slant")
    else {
        panic!("the slanted rod is not empty");
    };
    for (label, z0, z1) in [("from z 1", 1.0, 3.0), ("past the slant", 1.0, 4.0)] {
        let coaxial = fin(label, rod(Point2::new(0.0, 0.0), 1.0, z0, z1));
        assert_alike(label, &slanted.body, &coaxial, true);
    }
}

/// **A rod at a box's edge borders alike in either order.** The rod
/// r 0.5 about (2, 0) over z 0..1 stands flush on the box
/// [−2,2]²×[0,1]'s bottom and top, across its x 2 wall. The union drops
/// the box's wall piece inside the rod, which borders the rod's wall on
/// its upright edges, and on its bottom and top edges the rod's bottom
/// and top through the box's fragments beside them where those are the
/// held ones (`DiscardRow::bordered_through`). Red before that reading:
/// with the box as A, the wall piece bordered its upright edges only.
/// The rod half out of the box's top (z 0..2) borders alike too.
#[test]
fn a_rod_at_a_box_edge_borders_alike_in_either_order() {
    let tol = Tol::witness();
    let boxed = fin("box", brick((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0), tol));
    let at_edge = fin("at the edge", rod(Point2::new(2.0, 0.0), 0.5, 0.0, 1.0));
    assert_alike("at the edge", &boxed, &at_edge, false);
    let union = op_fates("∪", &boxed, &at_edge);
    let wall = rounded(&format!(
        "{:?}",
        topo::CarrierDesc::Plane {
            origin: geom_core::Point3::new(2.0, 0.0, 0.5),
            normal: geom_core::Vec3::new(1.0, 0.0, 0.0),
        }
    ));
    let edges = [
        "(2,-0.5,0) (2,-0.5,1)",
        "(2,-0.5,0) (2,0.5,0)",
        "(2,-0.5,1) (2,0.5,1)",
        "(2,0.5,0) (2,0.5,1)",
    ]
    .map(String::from);
    assert!(
        union
            .lost
            .iter()
            .any(|(f, e)| *f == wall && edges.iter().all(|x| e.contains(x))),
        "the box's wall piece borders all four of its edges: {:?}",
        union.lost
    );
    let half_out = fin("half out", rod(Point2::new(2.0, 0.0), 0.5, 0.0, 2.0));
    assert_alike("half out", &boxed, &half_out, false);
}

/// **Coaxial rods border the seam circle only where the result holds
/// it.** `low` = r 1 over z 0..2, `high` over z 1..3: their walls coincide
/// over z 1..2 and the union keeps operand A's. Its wall ends at A's
/// circle, which the result keeps as an edge where the curved walls meet
/// (the merge does not join curved faces), so A's cap there borders it;
/// B's cap's circle lies inside A's wall, where the result holds no
/// vertex, so B's cap borders nothing. The cap that borders is the one
/// whose operand is A: the difference is the result's own geometry
/// between the orders, not the record's.
#[test]
fn coaxial_rods_border_the_seam_circle_only_where_the_result_holds_it() {
    let low = fin("low", rod(Point2::new(0.0, 0.0), 1.0, 0.0, 2.0));
    let high = fin("high", rod(Point2::new(0.0, 0.0), 1.0, 1.0, 3.0));
    for (x, y, at) in [(&low, &high, "2"), (&high, &low, "1")] {
        let union = op_fates("∪", x, y);
        let circle = format!("(-1,0,{at}) (1,0,{at})");
        let bordering: Vec<_> = union.lost.iter().filter(|(_, e)| !e.is_empty()).collect();
        assert!(
            matches!(bordering.as_slice(), [(_, e)] if e.iter().all(|s| *s == circle) && e.len() == 2),
            "z {at}: one cap borders the seam circle, the other nothing: {:?}",
            union.lost
        );
        assert_eq!(union.lost.len(), 2, "z {at}: the two caps");
        assert_eq!(union.held_by_a_discard, 0, "z {at}: rows held by a discard");
    }
}
