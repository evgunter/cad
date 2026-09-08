//! **SHELL-8's acceptance: `shell` / `shell_open` apply to EVERY solid.**
//!
//! Shelling is a per-solid verb, so on a body of `N` solids each solid
//! becomes its own thin solids and a designation names faces on any of
//! them. Every row is a closed form on a fixture built through the
//! public doors.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_lines,
    clippy::float_cmp
)]

use core::f64::consts::PI;

use geom_core::{Affine3, Band, Point3, Tol, Vec3};
use sweep::{TubeWindow, tube_along_arc};
use topo::{Body, FaceKey, ShellError, SolidKey};

use crate::verbs_shell::{boxy, hollow_box, v, vessel};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).expect("a band")
}

/// `body` with `other` placed `dx` along `+x` beside it, as a second
/// solid of one body — the public disjoint-graft door, which is what a
/// user assembling two parts reaches for.
fn beside(body: &Body<f64>, other: &Body<f64>, dx: f64) -> Body<f64> {
    let mut out = body.clone();
    let placed =
        topo::transform_rigid(other, &Affine3::translation(Vec3::new(dx, 0.0, 0.0)), tol())
            .expect("a rigid map");
    topo::graft_disjoint(&mut out, &placed, tol()).expect("the placed copy grafts");
    assert_eq!(
        topo::validate_geometric(&out, tol()),
        Ok(()),
        "the two-solid operand is valid"
    );
    out
}

fn volume(body: &Body<f64>) -> f64 {
    topo::mass_properties(body, tol()).expect("props").volume
}

/// Every vertex point of `body`, in arena order — the bitwise reading
/// the untouched-solid rows compare.
fn points(body: &Body<f64>) -> Vec<(topo::VertexKey, Point3<f64>)> {
    body.vertices()
        .map(|(k, v)| (k, *body.get_point(v.point).unwrap()))
        .collect()
}

/// The solid a face belongs to.
fn solid_of(body: &Body<f64>, face: FaceKey) -> SolidKey {
    let shell = body.get_face(face).unwrap().shell;
    body.get_shell(shell).unwrap().solid
}

/// Every face of `solid`, in arena order.
fn faces_of(body: &Body<f64>, solid: SolidKey) -> Vec<FaceKey> {
    body.faces()
        .filter(|(k, _)| solid_of(body, *k) == solid)
        .map(|(k, _)| k)
        .collect()
}

/// The chart groups of `solid`: faces by surface key, in arena order.
fn charts_of(body: &Body<f64>, solid: SolidKey) -> Vec<Vec<FaceKey>> {
    let mut out: Vec<(topo::SurfaceKey, Vec<FaceKey>)> = Vec::new();
    for face in faces_of(body, solid) {
        let key = body.get_face(face).unwrap().surface;
        match out.iter_mut().find(|(k, _)| *k == key) {
            Some((_, v)) => v.push(face),
            None => out.push((key, vec![face])),
        }
    }
    out.into_iter().map(|(_, v)| v).collect()
}

/// The planar face of `body` whose plane is normal to `+z` and sits at
/// `z`, restricted to `solid` — the whole chart wearing it.
fn top_chart(body: &Body<f64>, solid: SolidKey, z: f64) -> Vec<FaceKey> {
    for face in faces_of(body, solid) {
        let f = body.get_face(face).unwrap();
        let Some(geom::Surface::Plane { origin, normal, .. }) = body.get_surface(f.surface) else {
            continue;
        };
        if normal.x.abs() > 1e-9 || normal.y.abs() > 1e-9 || (origin.z - z).abs() > 1e-9 {
            continue;
        }
        let chart = f.surface;
        return body
            .faces()
            .filter(|(_, g)| g.surface == chart)
            .map(|(k, _)| k)
            .collect();
    }
    panic!("no z = {z} cap on {solid:?}")
}

// ---------------------------------------------------------------------
// Row 2 — two disjoint boxes in one body
// ---------------------------------------------------------------------

/// **Two boxes, one body, one `shell` call.** Each solid becomes its own
/// thin solid, and the wall is the sum of the two closed forms. The
/// separation is irrelevant: two parts facing each other across space
/// are not a wall, so the clearance gate never fires across solids —
/// the same pair at `0.05 < 2t` builds the same body.
#[test]
fn two_disjoint_boxes_each_shell_and_the_gap_between_them_never_gates() {
    let t = 0.05;
    let one_wall = v(2.0, 3.0, 4.0) - v(2.0 - 2.0 * t, 3.0 - 2.0 * t, 4.0 - 2.0 * t);

    for dx in [10.0, 2.0 + 0.05] {
        let pair = beside(&boxy(2.0, 3.0, 4.0), &boxy(2.0, 3.0, 4.0), dx);
        assert_eq!(pair.solids().count(), 2);
        let s = topo::shell(&pair, t, tol()).expect("both solids shell");
        let body = &s.body;
        println!(
            "[8] two boxes at dx={dx}: solids={} shells={} volume={} want={}",
            body.solids().count(),
            body.shells().count(),
            volume(body),
            2.0 * one_wall
        );
        assert_eq!(topo::validate_geometric(body, tol()), Ok(()), "tier 3");
        assert_eq!(body.solids().count(), 2, "one thin solid per operand solid");
        assert_eq!(body.shells().count(), 4, "each thin solid has two");
        assert!((volume(body) - 2.0 * one_wall).abs() < 1e-12);
        assert_eq!(s.naming.thickened.len(), 2, "a row per operand shell");
    }
}

// ---------------------------------------------------------------------
// Row 3 — the door is decided per solid
// ---------------------------------------------------------------------

/// **A box beside a vessel.** The body is neither all-planar nor a body
/// of revolution, so a whole-body door reading would put both on the
/// per-chart door and refuse the vessel's corners it solves alone.
/// Read per solid, the box takes the planar simultaneous door and the
/// vessel the axial one, and both closed forms come out.
#[test]
fn a_box_beside_a_vessel_takes_one_door_each() {
    let t = 0.05;
    let (r, h) = (1.0, 2.0);
    let pair = beside(&boxy(2.0, 3.0, 4.0), &vessel(r, h), 10.0);
    // The whole-body reading, measured: not all-planar, not axial.
    assert!(
        !topo::is_axial(&pair, band()).expect("the axis gate decides"),
        "the pair is not one body of revolution"
    );
    let s = topo::shell(&pair, t, tol()).expect("each solid takes its own door");
    let body = &s.body;
    let want = (v(2.0, 3.0, 4.0) - v(2.0 - 2.0 * t, 3.0 - 2.0 * t, 4.0 - 2.0 * t))
        + PI * (r * r * h - (r - t) * (r - t) * (h - 2.0 * t));
    println!(
        "[8] box+vessel: solids={} shells={} volume={} want={want}",
        body.solids().count(),
        body.shells().count(),
        volume(body)
    );
    assert_eq!(topo::validate_geometric(body, tol()), Ok(()), "tier 3");
    assert_eq!(body.solids().count(), 2);
    assert_eq!(body.shells().count(), 4);
    let props = topo::mass_properties(body, tol()).expect("props");
    assert!((props.volume - want).abs() <= 1e-9 + props.volume_pad);
}

/// **A box beside a full torus** — SHELL-7's door, on one solid of two.
#[test]
fn a_box_beside_a_full_torus_takes_one_door_each() {
    let t = 0.05;
    let (big_r, r) = (2.0, 0.5);
    let torus = tube_along_arc::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        big_r,
        TubeWindow::Full,
        r,
        tol(),
    )
    .expect("the solid torus builds")
    .body;
    let pair = beside(&boxy(2.0, 3.0, 4.0), &torus, 20.0);
    let s = topo::shell(&pair, t, tol()).expect("each solid takes its own door");
    let body = &s.body;
    let want = (v(2.0, 3.0, 4.0) - v(2.0 - 2.0 * t, 3.0 - 2.0 * t, 4.0 - 2.0 * t))
        + 2.0 * PI * PI * big_r * (r * r - (r - t) * (r - t));
    println!(
        "[8] box+torus: solids={} shells={} volume={} want={want}",
        body.solids().count(),
        body.shells().count(),
        volume(body)
    );
    assert_eq!(topo::validate_geometric(body, tol()), Ok(()), "tier 3");
    assert_eq!(body.solids().count(), 2);
    let props = topo::mass_properties(body, tol()).expect("props");
    assert!((props.volume - want).abs() <= 1e-9 + props.volume_pad);
}

// ---------------------------------------------------------------------
// Row 4 — a hollow solid beside a solid one
// ---------------------------------------------------------------------

/// **A hollow solid beside a plain one.** The hollow solid thickens
/// every boundary into two thin solids and the plain one into a third;
/// the roles are decided per solid, over that solid's own shells.
#[test]
fn a_hollow_solid_beside_a_plain_one_gives_three_thin_solids() {
    let t = 0.05;
    let pair = beside(&hollow_box(), &boxy(2.0, 3.0, 4.0), 10.0);
    assert_eq!(pair.solids().count(), 2);
    assert_eq!(
        pair.shells().count(),
        3,
        "outer, its void, and the plain box"
    );
    let s = topo::shell(&pair, t, tol()).expect("both solids shell");
    let body = &s.body;
    let want = (v(2.0, 3.0, 4.0) - v(1.9, 2.9, 3.9))
        + (v(1.6, 2.6, 3.6) - v(1.5, 2.5, 3.5))
        + (v(2.0, 3.0, 4.0) - v(1.9, 2.9, 3.9));
    println!(
        "[8] hollow+plain: solids={} shells={} volume={} want={want} thickened={:?}",
        body.solids().count(),
        body.shells().count(),
        volume(body),
        s.naming.thickened
    );
    assert_eq!(topo::validate_geometric(body, tol()), Ok(()), "tier 3");
    assert_eq!(body.solids().count(), 3);
    assert_eq!(body.shells().count(), 6);
    assert_eq!(s.naming.thickened.len(), 3, "a row per operand shell");
    assert!((volume(body) - want).abs() < 1e-12);
}

// ---------------------------------------------------------------------
// Row 5 — designations on any solid
// ---------------------------------------------------------------------

/// **A designation on each solid, in one call**, and a designation on
/// one solid only. Two rims, two cups; and with one named, the other
/// solid stays sealed with its two shells.
#[test]
fn designations_land_on_whichever_solid_carries_them() {
    let t = 0.05;
    let pair = beside(&boxy(2.0, 3.0, 4.0), &boxy(2.0, 3.0, 4.0), 10.0);
    let solids: Vec<SolidKey> = pair.solids().map(|(k, _)| k).collect();
    let lid = |i: usize| top_chart(&pair, solids[i], 4.0);
    let one_wall = v(2.0, 3.0, 4.0) - v(2.0 - 2.0 * t, 3.0 - 2.0 * t, 4.0 - 2.0 * t);
    // An OUTER designation removes the lid over the CAVITY and leaves
    // the rim standing — the wall thickness is exactly what shows.
    let one_lid = (2.0 - 2.0 * t) * (3.0 - 2.0 * t) * t;

    // Both lids, one call.
    let mut both = lid(0);
    both.extend(lid(1));
    let s = topo::shell_open(&pair, t, &both, tol()).expect("both lids open");
    println!(
        "[8] both lids: solids={} shells={} rims={} volume={} want={}",
        s.body.solids().count(),
        s.body.shells().count(),
        s.naming.rims.len(),
        volume(&s.body),
        2.0 * (one_wall - one_lid)
    );
    assert_eq!(topo::validate_geometric(&s.body, tol()), Ok(()), "tier 3");
    assert_eq!(s.body.solids().count(), 2);
    assert_eq!(s.body.shells().count(), 2, "each cup fused to one shell");
    assert_eq!(s.naming.rims.len(), 2, "one rim per designated chart");
    assert!((volume(&s.body) - 2.0 * (one_wall - one_lid)).abs() < 1e-12);

    // One lid: the other solid stays sealed.
    let s = topo::shell_open(&pair, t, &lid(0), tol()).expect("one lid opens");
    println!(
        "[8] one lid: solids={} shells={} volume={} want={}",
        s.body.solids().count(),
        s.body.shells().count(),
        volume(&s.body),
        2.0 * one_wall - one_lid
    );
    assert_eq!(topo::validate_geometric(&s.body, tol()), Ok(()), "tier 3");
    assert_eq!(s.body.solids().count(), 2);
    assert_eq!(s.body.shells().count(), 3, "one cup, one sealed thin solid");
    assert_eq!(s.naming.rims.len(), 1);
    assert!((volume(&s.body) - (2.0 * one_wall - one_lid)).abs() < 1e-12);
}

// ---------------------------------------------------------------------
// Row 7 and the STOP — the doors' relaxed precondition
// ---------------------------------------------------------------------

/// **The simultaneous doors' coverage is the SOLID, not the body.** A
/// move set naming some faces of a solid refuses typed as it always
/// did; a set naming every face of ONE solid of a two-solid body
/// builds, and the other solid is BITWISE untouched — which is the
/// measurement that says the corner walk, the axis frame and the edge
/// re-author all stay inside the named solid.
#[test]
fn a_simultaneous_door_moves_one_solid_and_leaves_the_other_bitwise() {
    let pair = beside(&boxy(2.0, 3.0, 4.0), &boxy(2.0, 3.0, 4.0), 10.0);
    let solids: Vec<SolidKey> = pair.solids().map(|(k, _)| k).collect();
    let moves = |body: &Body<f64>, solid: SolidKey, d: f64| -> Vec<topo::ChartMove<f64>> {
        charts_of(body, solid)
            .into_iter()
            .map(|faces| topo::ChartMove { faces, distance: d })
            .collect()
    };

    // A solid named in PART still refuses, naming the face nothing moved.
    let mut partial = moves(&pair, solids[0], -0.1);
    partial.pop();
    let mut work = pair.clone();
    let e = topo::offset_planes_together(&mut work, &partial, band(), tol())
        .expect_err("a partially named solid has corners this door cannot solve");
    println!("[8] partial solid: {e}");
    assert!(
        matches!(e, topo::ReplaceFaceError::TogetherPartialSet { .. }),
        "{e}"
    );

    // Every face of ONE solid: builds, and the other is bitwise.
    let before = points(&pair);
    let mut work = pair.clone();
    topo::offset_planes_together(&mut work, &moves(&pair, solids[0], -0.1), band(), tol())
        .expect("one solid's charts move together");
    let after = points(&work);
    assert_eq!(before.len(), after.len(), "no vertex minted or killed");
    let bits = |p: &Point3<f64>| (p.x.to_bits(), p.y.to_bits(), p.z.to_bits());
    let mut moved = 0usize;
    for ((k, b), (k2, a)) in before.iter().zip(after.iter()) {
        assert_eq!(k, k2, "the vertex arena kept its order");
        if solid_of_vertex(&pair, *k) == solids[1] {
            assert_eq!(
                bits(b),
                bits(a),
                "the untouched solid's {k:?} moved: {b:?} -> {a:?}"
            );
        } else if bits(b) != bits(a) {
            moved += 1;
        }
    }
    println!("[8] one solid moved: {moved} of {} vertices", before.len());
    assert_eq!(moved, 8, "the named solid's eight corners moved");
}

/// The solid a vertex belongs to, through its emanating half-edge.
fn solid_of_vertex(body: &Body<f64>, vertex: topo::VertexKey) -> SolidKey {
    let he = body.get_vertex(vertex).unwrap().emanating.unwrap();
    let lp = body.get_half_edge(he).unwrap().parent_loop;
    let face = body.get_loop(lp).unwrap().face;
    solid_of(body, face)
}

// ---------------------------------------------------------------------
// The empty operand
// ---------------------------------------------------------------------

/// **A body with NO solid is what still refuses.** It is an EMPTY
/// operand, not an unsupported arity: any positive solid count builds.
#[test]
fn a_body_with_no_solid_refuses_typed() {
    let empty: Body<f64> = Body::new();
    let e = topo::shell(&empty, 0.05, tol()).expect_err("nothing to thicken");
    println!("[8] empty operand: {e}");
    assert!(matches!(e, ShellError::NoSolid), "{e}");
}
