//! **The wedge-end doors that need a curved body** (D1 tier 3): the
//! split's `SectionCusp` refusal and its counter-rows, the curved
//! boolean's refusals of an undeclared kiss, and the blend and shell
//! consumers of a declared cusp body. A cusp or slit is legal at rest
//! iff jet-determinate, so tier 3 no longer refuses one nobody
//! declared: the op that could mint it owns that refusal. The audit
//! that places every op's door, including those pinned elsewhere, is
//! `work/gather/every-op-that-can-mint-a-wedge-end-refuses-an-undeclared-one.md`.
//!
//! The split and boolean doors are `topo`'s; their rows live here
//! because they need a real curved body, which `sweep` builds. The
//! profile's doors are pinned in `profile`'s own suites
//! (`declared_tangency.rs`, `path_property.rs`, `rejections.rs`).
//!
//! The consumer rows at the end hold a DECLARED cusp body — one that
//! reaches a consumer legally — to the other half of the clause:
//! a consumer with no wedge-0/2π answer refuses typed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{Open, Profile, ProfileLoop, Start};
use sweep::blend::{BlendError, chamfer_edges, fillet_edges};
use sweep::test_support::{brick, sketch_at};
use sweep::{Extruded, Extrusion, extrude};
use topo::{
    Body, BooleanErrorKind, ContactMark, EdgeKey, ShellError, SplitError, SplitFinishError,
    SplitPlane,
};

fn tol() -> Tol {
    Tol::witness()
}

/// A full circle as two semicircular arcs, vertices at `(cx ± r, cy)`.
fn circle(cx: f64, cy: f64, r: f64) -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(cx + r, cy), 1.0),
        (Point2::new(cx - r, cy), 1.0),
    ])
}

fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(x0, y0), 0.0),
        (Point2::new(x1, y0), 0.0),
        (Point2::new(x1, y1), 0.0),
        (Point2::new(x0, y1), 0.0),
    ])
}

/// `loops` on the xy plane at height `z0`, extruded `h` along `+z`.
fn extruded(loops: Vec<ProfileLoop<f64>>, z0: f64, h: f64) -> Extruded<f64> {
    let profile = Profile::new(sketch_at(z0), loops)
        .validate(tol())
        .expect("the fixture's profile validates");
    extrude(&profile, Extrusion::Distance(h), tol()).expect("the fixture extrudes")
}

/// The 6 × 6 × 1 plate with a unit round hole on the z axis; the
/// hole's two arcs meet at `(±1, 0)`, so the plane `x = 1` is tangent
/// to the hole wall along the vertical line through a vertex.
fn plate_with_hole() -> Body<f64> {
    extruded(
        vec![rect(-3.0, -3.0, 3.0, 3.0), circle(0.0, 0.0, 1.0)],
        0.0,
        1.0,
    )
    .body
}

/// The lune between the internally tangent circles `(0,1) r 1` and
/// `(0,2) r 2`, `.cusp()` at the kiss — the DECLARED cusp.
fn lune() -> ProfileLoop<f64> {
    Open.at(Point2::new(0.0, 4.0))
        .angle(-std::f64::consts::FRAC_PI_2, tol())
        .unwrap()
        .line(2.0, tol())
        .unwrap()
        .turn(std::f64::consts::FRAC_PI_2, tol())
        .unwrap()
        .tangent_arc_to(Point2::new(0.0, 0.0), tol())
        .unwrap()
        .cusp()
        .tangent_arc_to(Start, tol())
        .unwrap()
        .into()
}

/// The edges check 4 marks `Tangent`, each by its two endpoints.
fn tangent_edges(body: &Body<f64>) -> Vec<(EdgeKey, [Point3<f64>; 2])> {
    let point = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    topo::contact_marks(body, tol())
        .expect("the body is tier-3 valid")
        .iter()
        .filter(|(_, m)| **m == ContactMark::Tangent)
        .map(|(e, _)| {
            let he = body.get_edge(e).unwrap().he_plus;
            let ends = [
                point(body.get_half_edge(he).unwrap().start),
                point(body.half_edge_end(he).unwrap()),
            ];
            (e, ends)
        })
        .collect()
}

fn on_the_kiss(p: &Point3<f64>) -> bool {
    p.x.abs() < 1e-9 && p.y.abs() < 1e-9
}

// ---------------------------------------------------------------------
// The split: no declaration channel, so every minted wedge end refuses.
// ---------------------------------------------------------------------

/// **A split plane tangent to a hole wall refuses at the split.** On
/// the hole's side of `x = 1` the material near the tangent line is two
/// crescents between the cut face and the wall, each vanishing to a
/// knife edge (a doubled cusp): jet-determinate, so tier 3 would pass
/// it, and nothing declared it. Both orientations of the plane, since
/// the side that carries the crescents is `below` in one and `above`
/// in the other.
#[test]
fn a_split_tangent_to_a_hole_wall_refuses_the_knife_edge_it_would_mint() {
    let body = plate_with_hole();
    for normal in [1.0, -1.0] {
        let plane = SplitPlane {
            origin: Point3::new(1.0, 0.0, 0.0),
            normal: Vec3::new(normal, 0.0, 0.0),
        };
        match topo::split(&body, &plane, tol()) {
            Err(SplitError::Finish(SplitFinishError::SectionCusp { .. })) => {}
            other => panic!("normal {normal}: expected SectionCusp, got {other:?}"),
        }
    }
}

/// The door's two counter-rows. A plane cutting THROUGH the hole meets
/// its wall transversally and cuts clean; a cut across a DECLARED cusp
/// (the lune's strut, at mid-height) cuts clean and keeps the cusp,
/// which is inherited, not minted.
#[test]
fn a_split_through_the_hole_or_across_a_declared_cusp_still_cuts() {
    let through = SplitPlane {
        origin: Point3::new(0.5, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
    };
    let halves = topo::split(&plate_with_hole(), &through, tol()).expect("a transverse cut");
    for (side, part) in [("above", &halves.above), ("below", &halves.below)] {
        let body = part.body().expect("material on both sides of x = 0.5");
        assert_eq!(
            topo::validate_geometric(body, tol()),
            Ok(()),
            "{side} is tier-3 valid"
        );
        assert!(
            tangent_edges(body).is_empty(),
            "{side}: no tangent edge on a transverse cut"
        );
    }

    let cusp = extruded(vec![lune()], 0.0, 1.0);
    let mid = SplitPlane {
        origin: Point3::new(0.0, 0.0, 0.5),
        normal: Vec3::new(0.0, 0.0, 1.0),
    };
    let halves = topo::split(&cusp.body, &mid, tol()).expect("a cut across the strut");
    for (side, part) in [("above", &halves.above), ("below", &halves.below)] {
        let body = part.body().expect("material on both sides of z = 0.5");
        let tangent = tangent_edges(body);
        assert_eq!(tangent.len(), 1, "{side}: the strut's half, {tangent:?}");
        assert!(
            tangent[0].1.iter().all(on_the_kiss),
            "{side}: the tangent edge is the declared strut, {tangent:?}"
        );
    }
}

/// **A π seam at the cut is not a wedge end, and cuts.** The rounded
/// shoulder: a quarter arc centred at the origin runs from the corner
/// `(0,1)` down to `(-1,0)`, so `y = 1` is tangent to its wall along
/// the vertex ruling — the hole-wall row's shape exactly — but here the
/// wall's material and the section's lie on the SAME side (aligned
/// normals): the piece below meets the cut in a smooth seam. Only the
/// material pairing tells the two apart, and this row fails a door
/// that decides by surface kind.
#[test]
fn a_split_tangent_to_a_rounded_shoulder_cuts_at_a_seam() {
    let bulge = (std::f64::consts::PI / 8.0).tan();
    let shoulder = bulge_loop(vec![
        (Point2::new(-1.0, 0.0), 0.0),
        (Point2::new(2.0, 0.0), 0.0),
        (Point2::new(2.0, 2.0), 0.0),
        (Point2::new(0.0, 2.0), 0.0),
        (Point2::new(0.0, 1.0), bulge),
    ]);
    let body = extruded(vec![shoulder], 0.0, 1.0).body;
    let on_the_ruling = |p: &Point3<f64>| p.x.abs() < 1e-9 && (p.y - 1.0).abs() < 1e-9;
    // Normal `+y` refuses earlier, at the reduction
    // (`ConsecutiveOnSectors`), for a reason of its own:
    // `work/hone/split-shoulder-refuses-one-orientation-at-the-reduction.md`.
    let plane = SplitPlane {
        origin: Point3::new(0.0, 1.0, 0.0),
        normal: Vec3::new(0.0, -1.0, 0.0),
    };
    let halves = topo::split(&body, &plane, tol())
        .unwrap_or_else(|e| panic!("a seam at the cut must cut, got {e:?}"));
    let mut seams = 0;
    for (side, part) in [("above", &halves.above), ("below", &halves.below)] {
        let piece = part
            .body()
            .unwrap_or_else(|| panic!("material {side} y = 1"));
        assert_eq!(
            topo::validate_geometric(piece, tol()),
            Ok(()),
            "{side} is tier-3 valid"
        );
        for (_, ends) in tangent_edges(piece) {
            assert!(
                ends.iter().all(on_the_ruling),
                "{side}: a tangent edge off the ruling, {ends:?}"
            );
            seams += 1;
        }
    }
    assert_eq!(seams, 1, "the one seam the cut mints");
}

// ---------------------------------------------------------------------
// The boolean: an undeclared tangent operand pair refuses at the op.
// ---------------------------------------------------------------------

/// **Each kiss the curved boolean could turn into a wedge end refuses
/// typed at the op** — internal (the crescent a subtract would leave),
/// external (the doubled slit a union would leave), and a plane cutter
/// tangent to a hole wall. None of these is a wedge-specific door yet:
/// the crossing layer's frontier refuses first, and the declared route
/// is `work/tang/declared-cusps-second-order-wedge-arm.md` item 3, which
/// has to land the undeclared refusal in the same change.
#[test]
fn a_boolean_that_would_kiss_a_curved_face_refuses_typed_at_the_op() {
    let big = extruded(vec![circle(0.0, 2.0, 2.0)], 0.0, 1.0).body;
    let small = extruded(vec![circle(0.0, 1.0, 1.0)], -1.0, 3.0).body;
    let left = extruded(vec![circle(0.0, 0.0, 1.0)], 0.0, 1.0).body;
    let right = extruded(vec![circle(2.0, 0.0, 1.0)], 0.0, 1.0).body;
    let cutter = brick::<f64>((1.0, 4.0), (-4.0, 4.0), (-1.0, 2.0), tol());
    let rows = [
        (
            "internal kiss, subtract",
            topo::subtract(&big, &small, tol()),
            BooleanErrorKind::CurvedPierceUnsupported,
        ),
        (
            "external kiss, union",
            topo::union(&left, &right, tol()),
            BooleanErrorKind::CurvedPierceUnsupported,
        ),
        (
            "plane tangent to a hole, subtract",
            topo::subtract(&plate_with_hole(), &cutter, tol()),
            BooleanErrorKind::CurvedBooleanUnsupported,
        ),
    ];
    for (name, got, want) in rows {
        match got {
            Err(e) => assert_eq!(e.kind(), want, "{name}: {e}"),
            Ok(r) => panic!("{name}: expected {want:?}, got a result {r:?}"),
        }
    }
}

// ---------------------------------------------------------------------
// The consumers: a declared cusp body refuses typed or comes out sane.
// ---------------------------------------------------------------------

/// **Chamfer and fillet refuse the cusp strut typed** — on the lune's
/// cusp (wedge 0) and on a lune-shaped hole's slit (wedge 2π) alike:
/// the supports meet tangentially, so the blend has no side to sit in.
#[test]
fn chamfer_and_fillet_refuse_a_cusp_or_slit_strut_typed() {
    let cusp = extruded(vec![lune()], 0.0, 1.0);
    let slit = extruded(vec![rect(-1.0, -1.0, 3.0, 5.0), lune()], 0.0, 1.0);
    for (name, built, loop_index) in [("cusp", &cusp, 0), ("slit", &slit, 1)] {
        let strut = built.strut_edges[loop_index]
            .iter()
            .copied()
            .find(|&e| tangent_edges(&built.body).iter().any(|(t, _)| *t == e))
            .expect("the strut the cusp joint swept is marked Tangent");
        for (verb, got) in [
            (
                "chamfer",
                chamfer_edges(&built.body, &[strut], 0.05, tol()).map(|_| ()),
            ),
            (
                "fillet",
                fillet_edges(&built.body, &[strut], 0.05, tol()).map(|_| ()),
            ),
        ] {
            match got {
                Err(refusal) => assert!(
                    matches!(refusal.error, BlendError::TangentialEdge { edge, .. } if edge == strut),
                    "{name} {verb}: expected TangentialEdge on the strut, got {refusal:?}"
                ),
                Ok(()) => panic!("{name} {verb}: blended a wedge-{name} edge"),
            }
        }
    }
}

/// **Shell refuses a cusp or slit body typed.** Neither refusal is a
/// wedge-specific door — an inward offset of the cusp's walls has no
/// place to meet near the kiss, and the slit's two unequal cylinders
/// have no closed-form pose — but both are typed at the op, at a thin
/// wall and a thick one, and no body ships.
#[test]
fn shell_refuses_a_cusp_or_slit_body_typed() {
    let cusp = extruded(vec![lune()], 0.0, 1.0);
    let slit = extruded(vec![rect(-1.0, -1.0, 3.0, 5.0), lune()], 0.0, 1.0);
    for (name, body) in [("cusp", &cusp.body), ("slit", &slit.body)] {
        for thickness in [1e-3, 0.05] {
            match topo::shell(body, thickness, tol()) {
                Err(ShellError::Face { .. }) => {}
                Err(other) => panic!("{name} at {thickness}: an unexpected refusal {other}"),
                Ok(_) => panic!("{name} at {thickness}: shelled a wedge-{name} body"),
            }
        }
    }
}
