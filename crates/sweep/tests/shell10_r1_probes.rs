//! **SHELL-10 review lane R1 — probes.**
//!
//! Rows written to falsify the PR's claims by EXECUTION rather than by
//! reading, plus the lane's end-to-end consumer exercise. Each row's
//! doc says what would make it red.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Vec3};
use topo::{Body, FaceKey, HalfEdgeKey, SolidKey};

use crate::shell8_common::{
    band, beside, cap, deep_dump, faces_of, outer_and_void_of, tol, volume,
};
use crate::verbs_shell::{boxy, vessel};

fn y() -> Vec3<f64> {
    Vec3::new(0.0, 1.0, 0.0)
}

/// Every stored row whose half-edge key no longer resolves.
fn dead_rows(body: &Body<f64>) -> Vec<HalfEdgeKey> {
    body.pcurves()
        .map(|(he, _)| he)
        .filter(|he| body.get_half_edge(*he).is_none())
        .collect()
}

fn all_faces(body: &Body<f64>) -> Vec<FaceKey> {
    body.faces().map(|(k, _)| k).collect()
}

/// Every chart of `solid` as a move of `distance`.
fn moves_of(body: &Body<f64>, solid: SolidKey, distance: f64) -> Vec<topo::ChartMove<f64>> {
    crate::shell8_common::charts_of(body, solid)
        .into_iter()
        .map(|faces| topo::ChartMove { faces, distance })
        .collect()
}

// ---------------------------------------------------------------
// Claim 3: the subset pass is NOT the whole-body pass restricted.
// ---------------------------------------------------------------

/// **`mint_pcurves_of` cannot hold the `Maintains` posture its new
/// `DECLARED` row claims: it leaves rows behind on half-edges its
/// caller killed.**
///
/// `mint_pcurves` opens with `body.pcurves.clear()` precisely so a row
/// on a dead half-edge key cannot survive the pass — its own comment
/// says a face walk cannot reach one, and the PR's "shared `mint_faces`
/// rather than delegation" deviation is argued from exactly that. The
/// subset entry clears "through the faces it is given", so a row whose
/// half-edge was killed by the very producer that then calls it
/// survives — which makes `mint_pcurves_of`'s doc sentence ("it
/// re-derives the rows it may have staled and asserts nothing about the
/// rest") false of precisely those rows.
///
/// The row runs each entry over EVERY face of the body, so the
/// difference measured is the clearing rule and not the subset.
///
/// Red the day the subset entry drops rows a face walk cannot reach.
#[test]
fn r1_the_subset_pass_leaves_rows_on_half_edges_its_caller_killed() {
    let mut body = vessel(1.0, 2.0);
    topo::mint_pcurves(&mut body, tol()).expect("the vessel's rows mint");
    assert!(body.pcurves().count() > 0, "the vessel carries rows");
    assert!(dead_rows(&body).is_empty(), "a fresh mint leaves none dead");

    // Kill one edge that carries a row, through a public Euler door
    // that touches no pcurve row — the shape a scoped producer has.
    let victim = body
        .pcurves()
        .map(|(he, _)| he)
        .find(|he| body.clone().kef(*he).is_ok())
        .expect("some half-edge with a row admits a kef");
    body.kef(victim).expect("the kef lands");
    let dead = dead_rows(&body);
    println!("[r1p1] dead rows straight after the kef: {}", dead.len());
    assert!(!dead.is_empty(), "the kef stranded at least one row");

    let mut subset = body.clone();
    // Friction, recorded: the natural spelling
    // `mint_pcurves_of(&mut b, &all_faces(&b), tol)` does not
    // borrow-check; every caller has to bind the face list first.
    let every = all_faces(&subset);
    topo::mint_pcurves_of(&mut subset, &every, tol()).expect("the subset pass mints");
    let after_subset = dead_rows(&subset);

    let mut whole = body.clone();
    topo::mint_pcurves(&mut whole, tol()).expect("the whole-body pass mints");
    let after_whole = dead_rows(&whole);

    println!(
        "[r1p1] dead rows after mint_pcurves_of over EVERY face: {} | after mint_pcurves: {}",
        after_subset.len(),
        after_whole.len()
    );
    assert!(
        after_whole.is_empty(),
        "the whole-body entry clears dead rows — its stated reason for clear()"
    );
    assert!(
        !after_subset.is_empty(),
        "FINDING: the subset entry leaves them, so `Maintains` is not what it holds"
    );
}

// ---------------------------------------------------------------
// The end-to-end exercise, from a consumer's seat.
// ---------------------------------------------------------------

/// **A box beside a vessel, hollowed, then opened on the VESSEL's void
/// ceiling.** Volumes against closed forms on the operand, tier-3
/// validation, a watertight tessellation, and the untouched box
/// compared deep against the sealed arm — the SHELL-8 oracle.
#[test]
fn r1_e2e_box_beside_a_hollow_vessel_opened_on_its_void_ceiling() {
    let pair = beside(&boxy(2.0, 3.0, 4.0), &vessel(1.0, 2.0), 10.0);
    let pi = core::f64::consts::PI;
    assert!(
        (volume(&pair) - (24.0 + pi * 2.0)).abs() < 1e-9,
        "the operand's volume"
    );

    let t0 = 0.1;
    let hollow = topo::shell(&pair, t0, tol())
        .expect("both solids hollow")
        .body;
    let want = (24.0 - 1.8 * 2.8 * 3.8) + (pi * 2.0 - pi * 0.9 * 0.9 * 1.8);
    println!("[r1e2e-a] hollow volume {} want {want}", volume(&hollow));
    assert!(
        (volume(&hollow) - want).abs() < 1e-9,
        "closed-form volume of the hollowed pair"
    );

    let solids: Vec<SolidKey> = hollow.solids().map(|(k, _)| k).collect();
    assert_eq!(solids.len(), 2);
    let ves = solids
        .iter()
        .copied()
        .find(|s| {
            faces_of(&hollow, *s).iter().any(|f| {
                !matches!(
                    hollow.get_surface(hollow.get_face(*f).unwrap().surface),
                    Some(geom::Surface::Plane { .. })
                )
            })
        })
        .expect("the vessel is the curved solid");
    let bx = *solids.iter().find(|s| **s != ves).expect("the box");
    let (_, void) = outer_and_void_of(&hollow, ves);

    let t = 0.02;
    let sealed = topo::shell(&hollow, t, tol()).expect("the sealed arm");
    let opened = topo::shell_open(&hollow, t, &cap(&hollow, void, y(), 1.9), tol())
        .expect("the void ceiling opens");

    assert_eq!(
        topo::validate_geometric(&opened.body, tol()),
        Ok(()),
        "the opened result is tier-3 valid"
    );
    let m = mesh::tessellate(&opened.body, 5e-3, tol()).expect("tessellates");
    mesh::validate::check_mesh(&m).expect("watertight");

    // The box is not the designated face's solid; it must read exactly
    // as the sealed arm left it.
    let bx_sealed = deep_dump(&sealed.body, bx);
    let bx_opened = deep_dump(&opened.body, bx);
    println!(
        "[r1e2e-a] box deep rows sealed={} opened={} volumes sealed={} opened={}",
        bx_sealed.len(),
        bx_opened.len(),
        volume(&sealed.body),
        volume(&opened.body)
    );
    assert_eq!(bx_opened, bx_sealed, "the untouched solid, deep");
    assert!(
        volume(&opened.body) < volume(&sealed.body),
        "opening the ceiling removes material"
    );
}

/// **Four solids grafted, hollowed in ONE call, then one of them
/// opened.** Closed-form volume on the hollow, tier 3, a watertight
/// mesh, and the three untouched solids deep-identical to the sealed
/// arm.
#[test]
fn r1_e2e_four_solids_hollowed_then_one_opened() {
    let unit = boxy(2.0, 2.0, 2.0);
    let mut four = unit.clone();
    for i in 1..4 {
        four = beside(&four, &unit, 10.0 * f64::from(i));
    }
    let solids: Vec<SolidKey> = four.solids().map(|(k, _)| k).collect();
    assert_eq!(solids.len(), 4, "four solids in one body");
    assert!((volume(&four) - 32.0).abs() < 1e-9);

    let t = 0.2;
    let hollow = topo::shell(&four, t, tol()).expect("all four hollow").body;
    let want = 4.0 * (8.0 - 1.6 * 1.6 * 1.6);
    println!(
        "[r1e2e-b] four-solid hollow {} want {want}",
        volume(&hollow)
    );
    assert!((volume(&hollow) - want).abs() < 1e-9, "closed form, x4");
    assert_eq!(topo::validate_geometric(&hollow, tol()), Ok(()));

    // Open the third solid's top lid, sealing the other three.
    let target = solids[2];
    let lid = crate::shell8_common::top_chart(&four, target, 2.0);
    let opened = topo::shell_open(&four, t, &lid, tol()).expect("one lid opens");
    let sealed = topo::shell(&four, t, tol()).expect("the sealed arm");
    assert_eq!(topo::validate_geometric(&opened.body, tol()), Ok(()));
    let m = mesh::tessellate(&opened.body, 5e-3, tol()).expect("tessellates");
    mesh::validate::check_mesh(&m).expect("watertight");

    let rim_solid = crate::shell8_common::solid_of(&opened.body, opened.naming.rims[0].rim);
    let mut compared = 0usize;
    for s in opened.body.solids().map(|(k, _)| k) {
        if s == rim_solid {
            continue;
        }
        assert_eq!(
            deep_dump(&opened.body, s),
            deep_dump(&sealed.body, s),
            "an undesignated solid was re-authored"
        );
        compared += 1;
    }
    println!("[r1e2e-b] {compared} undesignated solids deep-identical to the sealed arm");
    assert_eq!(compared, 3);
}

/// **A direct `offset_charts_together` naming ONE solid of two, on a
/// body whose OTHER solid carries a chart the whole-body mint refuses**
/// — the acceptance-row shape, from a consumer's seat in `sweep` rather
/// than through the crate's internals.
///
/// Red at the merge base (the door's closing `mint_pcurves` refuses the
/// neighbour's chart) and green at the head.
#[test]
fn r1_e2e_direct_door_over_one_of_two_with_an_unmintable_neighbour() {
    let mut pair = beside(&vessel(1.0, 2.0), &boxy(2.0, 3.0, 4.0), 10.0);
    let solids: Vec<SolidKey> = pair.solids().map(|(k, _)| k).collect();
    let (ves, bx) = (solids[0], solids[1]);

    // A cylinder the box's face cannot be charted on: structurally
    // sound, tier-2 clean, and the whole-body mint refuses it.
    let victim = faces_of(&pair, bx)[0];
    pair.set_face_surface(
        victim,
        topo::FaceSurface::New(geom::Surface::Cylinder {
            origin: Point3::new(10.5, 0.5, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 0.5,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }),
    )
    .expect("the chart attaches");
    let mut probe = pair.clone();
    let refusal = topo::mint_pcurves(&mut probe, tol())
        .expect_err("a whole-body mint refuses the neighbour's chart");
    println!("[r1e2e-c] the base's refusal: {refusal:?}");

    let before = deep_dump(&pair, bx);
    let mut work = pair.clone();
    topo::offset_charts_together(&mut work, &moves_of(&pair, ves, -0.05), band(), tol())
        .expect("the door reads its scope, and its scope charts");
    println!(
        "[r1e2e-c] the door built; out-of-scope deep rows {} unchanged",
        before.len()
    );
    assert_eq!(deep_dump(&work, bx), before, "the neighbour, deep");
}

/// **The doors no longer launder an out-of-scope solid's HALF-MINTED
/// face, and tier 3 sees the difference.**
///
/// At the merge base both doors closed with `mint_pcurves`, which
/// cleared and re-derived every row of the clone: a door handed an
/// operand whose out-of-scope solid was missing one row returned a body
/// tier 3 accepted. At the head the row is outside the scope and stays
/// missing, so the same call returns a body tier 3 REFUSES
/// (`StoredPcurve { MissingCache }`).
///
/// That is the intended semantics — the defect is the operand's — but
/// it is a postcondition a direct caller of the door had and no longer
/// has, and the PR's docs state it only as "as fresh as they were
/// found". `shell` is unaffected: its own closing whole-body mint
/// (SHELL-9) still launders.
///
/// Red the day the door re-derives out-of-scope rows again.
#[test]
fn r1_the_door_no_longer_launders_a_half_minted_out_of_scope_face() {
    let mut pair = beside(&vessel(1.0, 2.0), &vessel(1.0, 2.0), 10.0);
    topo::mint_pcurves(&mut pair, tol()).expect("both vessels mint");
    let solids: Vec<SolidKey> = pair.solids().map(|(k, _)| k).collect();
    let (ves, other) = (solids[0], solids[1]);

    // Drop ONE stored row of the out-of-scope solid: a half-minted
    // face, which is exactly what tier 3's check 8 looks for.
    let strip = pair
        .pcurves()
        .map(|(he, _)| he)
        .find(|he| {
            let lp = pair.get_half_edge(*he).unwrap().parent_loop;
            let f = pair.get_loop(lp).unwrap().face;
            crate::shell8_common::solid_of(&pair, f) == other
        })
        .expect("the other solid carries rows");
    pair.detach_pcurve(strip).expect("the row was there");
    assert!(
        topo::validate_geometric(&pair, tol()).is_err(),
        "the operand is already tier-3 bad — that is the premise"
    );

    let mut work = pair.clone();
    topo::offset_charts_together(&mut work, &moves_of(&pair, ves, -0.05), band(), tol())
        .expect("the door builds");
    let after = topo::validate_geometric(&work, tol());
    println!("[r1p3] tier 3 on the door's result: {after:?}");
    assert!(
        after.is_err(),
        "FINDING: at the base the closing whole-body mint re-derived this row \
         and the result was tier-3 clean; at the head it is not"
    );
}

// ---------------------------------------------------------------
// Cost, reproduced. Gated on CAD_R1_BENCH=1 so the suite is unaffected.
// ---------------------------------------------------------------

/// **Claim 7: reproduce one row of the PR's cost table, and measure the
/// direct door's scaling with the scope on a FOUR-solid body naming
/// one.** Release only; 20 warm-up calls, then 5 timed runs of 200,
/// median ms/call.
#[test]
fn r1_cost() {
    if std::env::var("CAD_R1_BENCH").as_deref() != Ok("1") {
        return;
    }
    println!(
        "[r1cost] debug_assertions compiled in: {}",
        cfg!(debug_assertions)
    );
    let bench = |label: &str, f: &dyn Fn()| {
        for _ in 0..20 {
            f();
        }
        let mut ms: Vec<f64> = Vec::new();
        for _ in 0..5 {
            let t = std::time::Instant::now();
            for _ in 0..200 {
                f();
            }
            ms.push(t.elapsed().as_secs_f64() * 1000.0 / 200.0);
        }
        ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("[r1cost] {label}: median {:.4} ms/call  all {ms:?}", ms[2]);
    };

    // Row 4 of the PR's table: the axial door on 1 of 2 vessels.
    let pair = beside(&vessel(1.0, 2.0), &vessel(1.0, 2.0), 10.0);
    let ves = pair.solids().next().unwrap().0;
    let mv2 = moves_of(&pair, ves, -0.05);
    bench("offset_charts_together, 1 of 2 vessels", &|| {
        let mut w = pair.clone();
        topo::offset_charts_together(&mut w, &mv2, band(), tol()).expect("builds");
    });

    // The same door on a FOUR-solid body naming one: if the door still
    // scaled with the BODY the median would rise with the neighbours.
    let mut four = vessel(1.0, 2.0);
    for i in 1..4 {
        four = beside(&four, &vessel(1.0, 2.0), 10.0 * f64::from(i));
    }
    let ves4 = four.solids().next().unwrap().0;
    let mv4 = moves_of(&four, ves4, -0.05);
    bench("offset_charts_together, 1 of 4 vessels", &|| {
        let mut w = four.clone();
        topo::offset_charts_together(&mut w, &mv4, band(), tol()).expect("builds");
    });

    // And the planar door, 1 of 4 boxes.
    let mut fourb = boxy(2.0, 3.0, 4.0);
    for i in 1..4 {
        fourb = beside(&fourb, &boxy(2.0, 3.0, 4.0), 10.0 * f64::from(i));
    }
    let bx4 = fourb.solids().next().unwrap().0;
    let mvb = moves_of(&fourb, bx4, -0.05);
    bench("offset_planes_together, 1 of 4 boxes", &|| {
        let mut w = fourb.clone();
        topo::offset_planes_together(&mut w, &mvb, band(), tol()).expect("builds");
    });

    // Baseline: the `Body::clone` each timed closure pays, so the
    // door's own cost can be read net of the operand's size.
    bench("clone only, 2 vessels", &|| {
        let _ = pair.clone();
    });
    bench("clone only, 4 vessels", &|| {
        let _ = four.clone();
    });
    bench("clone only, 4 boxes", &|| {
        let _ = fourb.clone();
    });
}
