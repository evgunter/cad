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

use geom_core::{Point3, Vec3};
use sweep::{TubeWindow, tube_along_arc};
use topo::{Body, ShellError, SolidKey};

use crate::shell8_common::{
    band, beside, bits, charts_of, deep_dump, edge_rows, outer_and_void_of, points, solid_of,
    solid_of_vertex, tol, top_chart, volume,
};
use crate::verbs_shell::{boxy, hollow_box, v, vessel};

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

// ---------------------------------------------------------------------
// The lift's scope — the row the §3 STOP owes
// ---------------------------------------------------------------------

/// **The rim lift re-authors the designated face's solid, and no
/// other.** The lift's simultaneous door names every chart of the solid
/// it is lifting in, the untouched ones at distance zero; read over the
/// whole RESULT body it named the other thin solids' charts too, and
/// re-derived their edges — same geometry, but a re-authored
/// description, which on the vessel's axis-parallel rim edges flips the
/// sign of the `Harmonic` pcurve's `1.2246467991473532e-16` term.
///
/// So: shell the hollow vessel SEALED, and shell it OPENED on one wall.
/// Every solid the designation is not in must carry, edge for edge and
/// key for key, exactly the rows the sealed arm wrote — the sealed body
/// is the oracle, since both are the same construction up to the rim
/// surgery.
///
/// **This is the row that goes red under `Scope::whole` at the lift.**
/// It is the whole content of the §3 STOP: the lift now touches only
/// its own solid.
#[test]
fn the_lift_re_authors_only_the_designated_faces_solid() {
    let t = 0.02;
    let hollow = topo::shell(&vessel(1.0, 2.0), 0.1, tol())
        .expect("the vessel hollows")
        .body;
    let (hollow_solid, _) = hollow.solids().next().expect("one solid");
    let (outer, void) = outer_and_void_of(&hollow, hollow_solid);
    let y = Vec3::new(0.0, 1.0, 0.0);

    let sealed = topo::shell(&hollow, t, tol()).expect("sealed");
    for (label, designation) in [
        (
            "outer lid",
            crate::shell8_common::cap(&hollow, outer, y, 2.0),
        ),
        (
            "void ceiling",
            crate::shell8_common::cap(&hollow, void, y, 1.9),
        ),
    ] {
        let opened = topo::shell_open(&hollow, t, &designation, tol()).expect("opens");
        // The solid the surgery ran in, read off the rim the record
        // reports rather than guessed from geometry.
        let rim = opened.naming.rims[0].rim;
        let rim_solid = solid_of(&opened.body, rim);

        let sealed_rows: Vec<(topo::EdgeKey, String)> = edge_rows(&sealed.body);
        let mut compared = 0usize;
        for (key, row) in edge_rows(&opened.body) {
            // An edge of the designated solid is expected to differ —
            // that solid is what the surgery re-authored.
            if solid_of(&opened.body, face_of_he_pub(&opened.body, key)) == rim_solid {
                continue;
            }
            let Some((_, want)) = sealed_rows.iter().find(|(k, _)| *k == key) else {
                panic!("[8] {label}: {key:?} has no sealed counterpart");
            };
            assert_eq!(
                &row, want,
                "[8] {label}: {key:?} is outside the designated solid and was re-authored"
            );
            compared += 1;
        }
        println!(
            "[8] lift scope, {label}: {compared} edges outside the rim's solid, all identical"
        );
        assert!(compared >= 8, "[8] {label}: only {compared} edges compared");

        // And deeper: every solid but the rim's is deep-identical to
        // the sealed body's same solid — faces, surfaces, carriers,
        // parameters, descriptions and vertex BITS.
        for (solid, _) in opened.body.solids() {
            if solid == rim_solid {
                continue;
            }
            assert_eq!(
                deep_dump(&opened.body, solid),
                deep_dump(&sealed.body, solid),
                "[8] {label}: {solid:?} is not the sealed arm's own"
            );
        }
    }
}

/// The face an edge's positive half belongs to.
fn face_of_he_pub(body: &Body<f64>, edge: topo::EdgeKey) -> topo::FaceKey {
    crate::shell8_common::face_of_he(body, body.get_edge(edge).unwrap().he_plus)
}
