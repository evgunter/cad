//! **SHELL-10 review, R2 probes** — the end-to-end exercise and the
//! claims the review tried to falsify by execution, all from a
//! consumer's seat (public doors only). Rows print `[r2-10]` lines.
//!
//! What is pinned here and why:
//!
//! - the box-beside-vessel assembly hollowed and then opened on the
//!   VESSEL's void ceiling, against closed forms, tier 3, and a
//!   watertight tessellation;
//! - a four-solid assembly hollowed in one call and then opened on one
//!   solid, the same way;
//! - the axial door naming one solid of two on a body whose OTHER solid
//!   wears a chart the whole-body mint refuses: the door builds, the
//!   bystander is deep-identical, and the moved solid comes out
//!   bit-identical to the same solid offset ALONE — the reading that
//!   says no reader reached the other solid;
//! - what `mint_pcurves_of` leaves behind on a half-edge a public kill
//!   door retired (the PR's stated reason for not delegating
//!   `mint_pcurves` through the subset entry), and that the verb's
//!   closing whole-body mint leaves none.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use core::f64::consts::PI;
use std::time::Instant;

use geom_core::{Point3, Vec3};
use topo::{Body, FaceKey, SolidKey};

use crate::shell8_common::{
    band, beside, cap, charts_of, deep_dump, face_of_he, faces_of, solid_of, tol, volume,
};
use crate::verbs_shell::{boxy, tube, v, vessel};

/// The stored rows of `solid`'s faces, in half-edge-slot order.
fn rows_of(body: &Body<f64>, solid: SolidKey) -> Vec<String> {
    let mine: Vec<FaceKey> = faces_of(body, solid);
    body.pcurves()
        .filter(|(he, _)| {
            let lp = body.get_half_edge(*he).unwrap().parent_loop;
            mine.contains(&body.get_loop(lp).unwrap().face)
        })
        .map(|(he, c)| format!("he {he:?} params {:?} pcurve {:?}", c.params(), c.pcurve()))
        .collect()
}

/// Rows keyed on a half-edge no arena holds.
fn dead_rows(body: &Body<f64>) -> usize {
    body.pcurves()
        .filter(|(he, _)| body.get_half_edge(*he).is_none())
        .count()
}

/// The whole chart `face` wears.
fn chart_of(body: &Body<f64>, face: FaceKey) -> Vec<FaceKey> {
    let key = body.get_face(face).unwrap().surface;
    body.faces()
        .filter(|(_, f)| f.surface == key)
        .map(|(k, _)| k)
        .collect()
}

/// The chart of the plane normal to `z` at `z = at`, over the whole
/// body — the caller guarantees only one such plane exists.
fn z_chart(body: &Body<f64>, at: f64) -> Vec<FaceKey> {
    let mut found: Vec<FaceKey> = Vec::new();
    for (k, f) in body.faces() {
        let Some(geom::Surface::Plane { origin, normal, .. }) = body.get_surface(f.surface) else {
            continue;
        };
        if normal.x.abs() < 1e-9 && normal.y.abs() < 1e-9 && (origin.z - at).abs() < 1e-9 {
            found.push(k);
        }
    }
    assert!(!found.is_empty(), "a plane at z = {at}");
    let chart = chart_of(body, found[0]);
    assert_eq!(found.len(), chart.len(), "exactly one chart at z = {at}");
    chart
}

/// `deep_dump` with every `SurfaceKey(..)` masked: the pair's surface
/// arena holds the box's keys too, so the vessel's descriptions name
/// different KEYS for the same surfaces; everything else is bits.
fn masked(rows: Vec<String>) -> Vec<String> {
    rows.into_iter()
        .map(|r| {
            let mut out = String::new();
            let mut rest = r.as_str();
            while let Some(i) = rest.find("SurfaceKey(") {
                out.push_str(&rest[..i]);
                out.push_str("SurfaceKey(_)");
                let j = rest[i..].find(')').map_or(rest.len(), |j| i + j + 1);
                rest = &rest[j..];
            }
            out.push_str(rest);
            out
        })
        .collect()
}

fn cyl(r: f64, h: f64) -> f64 {
    PI * r * r * h
}

fn annulus(ri: f64, ro: f64, h: f64) -> f64 {
    PI * (ro * ro - ri * ri) * h
}

/// Tier 3 and a watertight tessellation.
fn check(label: &str, body: &Body<f64>) {
    assert_eq!(
        topo::validate_geometric(body, tol()),
        Ok(()),
        "{label}: tier 3"
    );
    let mesh = mesh::tessellate(body, 5e-3, tol()).expect("tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
    println!(
        "[r2-10] {label}: solids={} shells={} volume={:.12} patches={} positions={} watertight",
        body.solids().count(),
        body.shells().count(),
        volume(body),
        mesh.patches.len(),
        mesh.positions.len()
    );
}

/// **E2E 1 — SHELL-8's box beside a vessel, hollowed, then opened on
/// the vessel's VOID ceiling.** Both closed forms, tier 3, watertight.
#[test]
fn r2_e2e_box_beside_vessel_opened_on_the_vessels_void_ceiling() {
    let (t, t2) = (0.05, 0.02);
    let pair = beside(&boxy(2.0, 3.0, 4.0), &vessel(1.0, 2.0), 10.0);
    let solids: Vec<SolidKey> = pair.solids().map(|(k, _)| k).collect();
    let ves = solids[1];
    let ves_shell = pair.get_solid(ves).unwrap().shells[0];
    let top = cap(&pair, ves_shell, Vec3::new(0.0, 1.0, 0.0), 2.0);

    let hollow = topo::shell(&pair, t, tol()).expect("hollow both");
    let want1 = (v(2.0, 3.0, 4.0) - v(1.9, 2.9, 3.9)) + (cyl(1.0, 2.0) - cyl(0.95, 1.9));
    println!(
        "[r2-10] e2e1 hollow: volume={:.12} closed form {:.12}",
        volume(&hollow.body),
        want1
    );
    assert!((volume(&hollow.body) - want1).abs() < 1e-9);

    // The vessel top's void twin, named through the record; the whole
    // chart it wears (a full revolve's cap is two half-discs).
    let ceiling = hollow
        .naming
        .inner_of(top[0])
        .expect("the vessel top's void twin");
    let chart = chart_of(&hollow.body, ceiling);
    let started = Instant::now();
    let opened = topo::shell_open(&hollow.body, t2, &chart, tol())
        .expect("open the vessel's void ceiling while the box stays sealed");
    let took = started.elapsed();
    let want_box =
        (v(2.0, 3.0, 4.0) - v(1.96, 2.96, 3.96)) + (v(1.94, 2.94, 3.94) - v(1.9, 2.9, 3.9));
    let want_ves = (cyl(1.0, 2.0) - cyl(0.98, 1.96)) + (cyl(0.97, 1.94) - cyl(0.95, 1.9));
    // The lid the opening removes: the designated face's TWIN's disc
    // (SHELL-8's e2e used the twin's rectangle for an outer lid).
    let lid_twin = cyl(0.97, t2);
    let lid_own = cyl(0.95, t2);
    let got = volume(&opened.body);
    println!(
        "[r2-10] e2e1 opened: volume={got:.12} twin-lid form {:.12} own-lid form {:.12} \
         took {took:?}",
        want_box + want_ves - lid_twin,
        want_box + want_ves - lid_own
    );
    assert!(
        (got - (want_box + want_ves - lid_twin)).abs() < 1e-9,
        "the twin-lid closed form"
    );
    assert_eq!(opened.body.solids().count(), 4);
    assert_eq!(opened.body.shells().count(), 7);
    assert_eq!(opened.naming.rims.len(), 1);
    check("e2e1 opened", &opened.body);
}

/// **E2E 2 — four parts grafted, hollowed in ONE call, then one of
/// them opened.** Box, vessel, tube, box; the second box's outer lid.
#[test]
fn r2_e2e_four_solids_hollowed_once_then_one_opened() {
    let (t, t2) = (0.05, 0.02);
    let mut four = beside(&boxy(2.0, 3.0, 4.0), &vessel(1.0, 2.0), 10.0);
    four = beside(&four, &tube(0.6, 1.0, 2.0), 20.0);
    four = beside(&four, &boxy(2.0, 2.0, 2.0), 30.0);
    assert_eq!(four.solids().count(), 4);

    let started = Instant::now();
    let hollow = topo::shell(&four, t, tol()).expect("four solids hollow in one call");
    let took_hollow = started.elapsed();
    let want1 = (v(2.0, 3.0, 4.0) - v(1.9, 2.9, 3.9))
        + (cyl(1.0, 2.0) - cyl(0.95, 1.9))
        + (annulus(0.6, 1.0, 2.0) - annulus(0.65, 0.95, 1.9))
        + (v(2.0, 2.0, 2.0) - v(1.9, 1.9, 1.9));
    println!(
        "[r2-10] e2e2 hollow: solids={} shells={} volume={:.12} closed form {:.12} took {took_hollow:?}",
        hollow.body.solids().count(),
        hollow.body.shells().count(),
        volume(&hollow.body),
        want1
    );
    assert!((volume(&hollow.body) - want1).abs() < 1e-9);
    assert_eq!(hollow.body.solids().count(), 4);
    assert_eq!(hollow.body.shells().count(), 8);

    // The second box's outer lid: the only plane at z = 2 in the body.
    let lid = z_chart(&hollow.body, 2.0);
    let started = Instant::now();
    let opened = topo::shell_open(&hollow.body, t2, &lid, tol())
        .expect("one lid opens while the other three solids shell sealed");
    let took_open = started.elapsed();
    let want2 = (v(2.0, 3.0, 4.0) - v(1.96, 2.96, 3.96))
        + (v(1.94, 2.94, 3.94) - v(1.9, 2.9, 3.9))
        + (cyl(1.0, 2.0) - cyl(0.98, 1.96))
        + (cyl(0.97, 1.94) - cyl(0.95, 1.9))
        + (annulus(0.6, 1.0, 2.0) - annulus(0.62, 0.98, 1.96))
        + (annulus(0.63, 0.97, 1.94) - annulus(0.65, 0.95, 1.9))
        + (v(2.0, 2.0, 2.0) - v(1.96, 1.96, 1.96))
        + (v(1.94, 1.94, 1.94) - v(1.9, 1.9, 1.9))
        - 1.96 * 1.96 * t2;
    println!(
        "[r2-10] e2e2 opened: volume={:.12} closed form {:.12} took {took_open:?}",
        volume(&opened.body),
        want2
    );
    assert!((volume(&opened.body) - want2).abs() < 1e-9);
    assert_eq!(opened.body.solids().count(), 8);
    assert_eq!(opened.body.shells().count(), 15);
    check("e2e2 opened", &opened.body);
}

/// **E2E 3 — the axial door naming one solid of two, the other wearing
/// a chart the whole-body mint refuses.** A consumer can put any
/// surface on a face through `set_face_surface`; the box's first face
/// becomes a cylinder it does not lie on. Tier 2 is clean, the base's
/// whole-body mint refuses the body, the head's door builds; the box
/// is deep-identical, and the vessel comes out BIT-identical to the
/// same vessel offset alone — no reader of the door reached the box.
#[test]
fn r2_e2e_axial_door_names_one_solid_while_the_other_is_unmintable() {
    let pair = beside(&vessel(1.0, 2.0), &boxy(2.0, 3.0, 4.0), 10.0);
    let solids: Vec<SolidKey> = pair.solids().map(|(k, _)| k).collect();
    let (ves, bx) = (solids[0], solids[1]);
    let victim = faces_of(&pair, bx)[0];
    let mut body = pair.clone();
    body.set_face_surface(
        victim,
        topo::FaceSurface::New(geom::Surface::Cylinder {
            origin: Point3::new(10.5, 1.5, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 0.5,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }),
    )
    .expect("a consumer can re-surface a face");
    assert_eq!(topo::validate_closed(&body), Ok(()), "tier 2 is clean");
    let mut whole = body.clone();
    let refused = topo::mint_pcurves(&mut whole, tol()).expect_err("the whole-body mint refuses");
    println!("[r2-10] e2e3 whole-body mint refuses: {refused}");

    // The operand's rows: the vessel's, through the subset entry — the
    // only public mint that can run on this body at all.
    let mut before = body.clone();
    let in_scope = faces_of(&before, ves);
    let minted =
        topo::mint_pcurves_of(&mut before, &in_scope, tol()).expect("the vessel's rows mint");
    let box_deep = deep_dump(&before, bx);
    let box_rows = rows_of(&before, bx);
    let moves: Vec<topo::ChartMove<f64>> = charts_of(&before, ves)
        .into_iter()
        .map(|faces| topo::ChartMove {
            faces,
            distance: -0.05,
        })
        .collect();
    let mut after = before.clone();
    topo::offset_charts_together(&mut after, &moves, band(), tol())
        .expect("the door reads its scope, and its scope charts");
    assert_eq!(deep_dump(&after, bx), box_deep, "the box is deep-identical");
    assert_eq!(
        rows_of(&after, bx),
        box_rows,
        "and its rows (none) are as found"
    );

    // The same vessel offset ALONE: its keys are the pair's (the pair
    // was built by cloning it first), so the readings compare directly.
    let mut alone = vessel(1.0, 2.0);
    assert_eq!(alone.solids().next().unwrap().0, ves);
    topo::mint_pcurves(&mut alone, tol()).expect("the vessel alone mints");
    let alone_moves: Vec<topo::ChartMove<f64>> = charts_of(&alone, ves)
        .into_iter()
        .map(|faces| topo::ChartMove {
            faces,
            distance: -0.05,
        })
        .collect();
    assert_eq!(
        moves.iter().map(|m| m.faces.clone()).collect::<Vec<_>>(),
        alone_moves
            .iter()
            .map(|m| m.faces.clone())
            .collect::<Vec<_>>()
    );
    topo::offset_charts_together(&mut alone, &alone_moves, band(), tol()).expect("alone");
    assert_eq!(
        masked(deep_dump(&after, ves)),
        masked(deep_dump(&alone, ves)),
        "the vessel in the pair is bit-identical to the vessel alone (surface keys masked)"
    );
    assert_eq!(rows_of(&after, ves), rows_of(&alone, ves), "rows too");
    println!(
        "[r2-10] e2e3 door built: minted={minted} vessel rows={} box rows={} tier3 on the pair: {:?}",
        rows_of(&after, ves).len(),
        box_rows.len(),
        topo::validate_geometric(&after, tol()).map_err(|e| e.len())
    );
    println!(
        "[r2-10] e2e3 tessellation of the pair: {:?}",
        mesh::tessellate(&after, 5e-3, tol()).map(|m| m.patches.len())
    );
}

/// **What the subset pass leaves on a half-edge a kill door retired.**
/// The vessel's rows are minted, then a public `kef` merges the two
/// wall faces across one seam and retires that edge's two half-edges.
/// Their rows outlive the keys; `mint_pcurves_of` over every face
/// cannot reach them, `mint_pcurves` drops them. The verb's own result
/// carries none.
#[test]
fn r2_subset_pass_leaves_a_retired_half_edges_row_and_the_whole_pass_drops_it() {
    let mut b = vessel(1.0, 2.0);
    topo::mint_pcurves(&mut b, tol()).expect("mints");
    assert_eq!(dead_rows(&b), 0);
    let total = b.pcurves().count();
    // A seam: an edge whose two faces are distinct and wear one
    // non-planar surface key.
    let seam = b
        .edges()
        .find(|(_, e)| {
            let (fa, fb) = (face_of_he(&b, e.he_plus), face_of_he(&b, e.he_minus));
            let (sa, sb) = (
                b.get_face(fa).unwrap().surface,
                b.get_face(fb).unwrap().surface,
            );
            fa != fb && sa == sb && !matches!(b.get_surface(sa), Some(geom::Surface::Plane { .. }))
        })
        .map(|(_, e)| e.he_plus)
        .expect("a wall seam");
    let had_row = b.pcurve(seam).is_some();
    match b.kef(seam) {
        Ok(_) => {}
        Err(e) => {
            println!("[r2-10] kef refused the seam: {e} — row not demonstrable this way");
            return;
        }
    }
    let dead_after_kill = dead_rows(&b);
    println!(
        "[r2-10] kill: seam had row={had_row} rows={} dead={dead_after_kill} of {total}",
        b.pcurves().count()
    );
    assert_eq!(
        dead_after_kill, 2,
        "the retired edge's two rows outlive their keys"
    );

    let faces: Vec<FaceKey> = b.faces().map(|(k, _)| k).collect();
    let mut sub = b.clone();
    let subset = topo::mint_pcurves_of(&mut sub, &faces, tol());
    println!(
        "[r2-10] subset pass over EVERY face: {:?} dead={} rows={}",
        subset.as_ref().map_err(|e| e.to_string()),
        dead_rows(&sub),
        sub.pcurves().count()
    );
    assert_eq!(
        dead_rows(&sub),
        2,
        "the subset pass cannot reach a retired key"
    );
    let mut whole = b.clone();
    let all = topo::mint_pcurves(&mut whole, tol());
    println!(
        "[r2-10] whole-body pass: {:?} dead={} rows={}",
        all.as_ref().map_err(|e| e.to_string()),
        dead_rows(&whole),
        whole.pcurves().count()
    );
    assert_eq!(dead_rows(&whole), 0, "the whole-body pass clears first");

    // The verb: its rim surgery retires half-edges, its closing mint is
    // whole-body, so the result carries no retired row.
    let ves = vessel(1.0, 2.0);
    let shell = ves.shells().next().unwrap().0;
    let top = cap(&ves, shell, Vec3::new(0.0, 1.0, 0.0), 2.0);
    let opened = topo::shell_open(&ves, 0.2, &top, tol())
        .expect("opens")
        .body;
    println!(
        "[r2-10] shell_open result: rows={} dead={}",
        opened.pcurves().count(),
        dead_rows(&opened)
    );
    assert_eq!(dead_rows(&opened), 0);
    let _ = solid_of;
}
