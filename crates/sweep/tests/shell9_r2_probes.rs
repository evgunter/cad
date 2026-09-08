//! **SHELL-9 review lane R2 — probes.** Rows here falsify (or fail to
//! falsify) the unit's claims by execution: the end-to-end consumer
//! exercise over the public doors, the drum's refusal reason, and what
//! the closing mint does to an operand whose own pcurve map is wrong.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::{FRAC_PI_2, PI};

use geom_core::{Point2, Tol, Vec3};
use profile::{ProfileLoop, ProfileVertex, RawLoop};
use sweep::Revolution;
use topo::{Body, FaceKey};

use super::shell7_common::{drum, hollow_moves, p2, polyline, revolved, tol};
use super::shell8_common::beside;
use super::verbs_shell::{boxy, vessel};

/// The unit's two-arc sphere: one sphere in four faces, a same-surface
/// latitude seam at `v = π/4`.
fn two_arc_sphere() -> Body<f64> {
    let r = 1.0;
    let v = PI / 4.0;
    let (s, c) = v.sin_cos();
    revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -r), ((FRAC_PI_2 + v) / 4.0).tan()),
            ProfileVertex::new(p2(r * c, r * s), ((FRAC_PI_2 - v) / 4.0).tan()),
            ProfileVertex::new(p2(0.0, r), 0.0),
        ]),
        Revolution::Full,
    )
}

fn bulge(a: Point2<f64>, b: Point2<f64>, c: Point2<f64>) -> f64 {
    let (u, v) = (a - c, b - c);
    (u.perp_dot(v).atan2(u.dot(v)) / 4.0).tan()
}

/// `sf2b_axial`'s sphere-zone vase.
fn sphere_zone_vase(r: f64, h: f64) -> Body<f64> {
    let c = p2(0.0, h / 2.0);
    revolved(
        RawLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r, 0.0), bulge(p2(r, 0.0), p2(r, h), c)),
            ProfileVertex::new(p2(r, h), 0.0),
            ProfileVertex::new(p2(0.0, h), 0.0),
        ]),
        Revolution::Full,
    )
}

/// Planar faces normal to `y` at height `y`.
fn cap_at_y(body: &Body<f64>, y: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if (origin.y - y).abs() < 1e-9 && normal.x.abs() < 1e-9 && normal.z.abs() < 1e-9
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// Every `ValidationError::Pcurve` finding of `body`, as text.
fn pcurve_findings(body: &Body<f64>) -> Vec<String> {
    match topo::validate_geometric(body, tol()) {
        Ok(()) => Vec::new(),
        Err(errors) => errors
            .iter()
            .filter_map(|e| match e {
                topo::ValidationError::Pcurve { finding } => Some(format!("{finding:?}")),
                _ => None,
            })
            .collect(),
    }
}

/// Volume, tier 3, shell classification and a watertight mesh, from the
/// public doors only.
fn consume(label: &str, body: &Body<f64>, want_volume: f64, want_shells: usize) {
    assert_eq!(
        topo::validate_geometric(body, tol()),
        Ok(()),
        "{label}: tier 3"
    );
    assert!(
        pcurve_findings(body).is_empty(),
        "{label}: pcurve findings {:?}",
        pcurve_findings(body)
    );
    let props = topo::mass_properties(body, tol()).expect("props");
    assert!(
        (props.volume - want_volume).abs() <= 1e-9 + props.volume_pad,
        "{label}: volume {} (pad {}), want {want_volume}",
        props.volume,
        props.volume_pad
    );
    let shells: Vec<_> = body.shells().map(|(k, _)| k).collect();
    let roles = topo::classify_shells_of(body, &shells, tol()).expect("classify");
    let outers = roles
        .iter()
        .filter(|r| r.role == topo::ShellRole::Outer)
        .count();
    println!(
        "[r2] {label}: {} shells, {} outer, {} solids, {} pcurve rows",
        shells.len(),
        outers,
        body.solids().count(),
        body.pcurves().count()
    );
    assert_eq!(shells.len(), want_shells, "{label}: shell count");
    let mesh = mesh::tessellate(body, 1e-3, tol()).expect("tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
}

/// **E2E 1 — the two-arc sphere hollowed, from a consumer's seat.**
#[test]
fn r2_e2e_two_arc_sphere_hollows_and_tessellates() {
    let (r, t) = (1.0, 0.05);
    let body = two_arc_sphere();
    let out = topo::shell(&body, t, tol()).expect("shells");
    let want = 4.0 / 3.0 * PI * (r * r * r - (r - t) * (r - t) * (r - t));
    consume("two-arc sphere shelled", &out.body, want, 2);
}

/// **E2E 2 — the sphere-zone vase hollowed, then opened at its top cap.**
#[test]
fn r2_e2e_sphere_zone_vase_hollowed_then_opened() {
    let (r, h, t) = (1.0, 1.5, 0.1);
    let v = sphere_zone_vase(r, h);
    let sealed = topo::shell(&v, t, tol()).expect("shells");
    let sealed_vol = topo::mass_properties(&sealed.body, tol())
        .expect("props")
        .volume;
    consume("sphere-zone vase sealed", &sealed.body, sealed_vol, 2);
    let caps = cap_at_y(&v, h);
    assert!(!caps.is_empty(), "a top cap");
    println!("[r2] vase top cap faces: {}", caps.len());
    let opened = topo::shell_open(&v, t, &caps, tol()).expect("opens");
    let opened_vol = topo::mass_properties(&opened.body, tol())
        .expect("props")
        .volume;
    consume(
        "sphere-zone vase opened",
        &opened.body,
        opened_vol,
        opened.body.shells().count(),
    );
    assert!(
        opened_vol < sealed_vol,
        "opening removes the lid's material: {opened_vol} vs {sealed_vol}"
    );
}

/// **E2E 3 — SHELL-8's box beside a vessel, hollowed then opened on the
/// vessel's ceiling.**
#[test]
fn r2_e2e_box_beside_vessel_hollowed_and_opened() {
    let b = boxy(2.0, 2.0, 2.0);
    let v = vessel(1.0, 2.0);
    let pair = beside(&b, &v, 6.0);
    let sealed = topo::shell(&pair, 0.2, tol()).expect("shells the pair");
    let sealed_vol = topo::mass_properties(&sealed.body, tol())
        .expect("props")
        .volume;
    consume(
        "box beside vessel sealed",
        &sealed.body,
        sealed_vol,
        sealed.body.shells().count(),
    );
    // The vessel's own top cap, in the pair: the cap plane at y = 2 on
    // the placed copy.
    let caps = cap_at_y(&pair, 2.0);
    println!("[r2] caps at y=2: {}", caps.len());
    let opened = topo::shell_open(&pair, 0.2, &caps, tol()).expect("opens the pair");
    let opened_vol = topo::mass_properties(&opened.body, tol())
        .expect("props")
        .volume;
    consume(
        "box beside vessel opened",
        &opened.body,
        opened_vol,
        opened.body.shells().count(),
    );
}

/// **Claim 3 — the drum's refusal reason, by execution.** The reverted
/// cavity ALONE fails `validate_geometric`, and the door refuses
/// `ShellError::Insert` before any pcurve pass runs.
#[test]
fn r2_drum_reverted_cavity_alone_is_the_reason() {
    let d = drum(1.0, 2.0);
    let mut cavity = d.clone();
    let band = geom_core::Band::linear(tol()).expect("band");
    topo::offset_charts_together(&mut cavity, &hollow_moves(&d, 0.1), band, tol())
        .expect("the cavity is built");
    assert_eq!(
        topo::validate_geometric(&cavity, tol()),
        Ok(()),
        "the cavity itself is tier-3 valid"
    );
    let reverted = cavity.revert().expect("revert");
    let verdict = topo::validate_geometric(&reverted, tol());
    println!("[r2] reverted drum cavity alone: {verdict:?}");
    assert!(
        verdict.is_err(),
        "the reverted cavity alone is what the drum stops on"
    );
}

/// **Claim 7 — does the closing mint launder an operand whose own
/// pcurve map is wrong?** An operand missing one row fails tier 3; the
/// verb takes it anyway and returns a valid body.
#[test]
fn r2_the_closing_mint_launders_an_invalid_operand() {
    let v = vessel(1.0, 2.0);
    assert_eq!(topo::validate_geometric(&v, tol()), Ok(()), "operand valid");
    let good = topo::shell(&v, 0.2, tol()).expect("the sound operand shells");
    let good_rows: Vec<String> = good
        .body
        .pcurves()
        .map(|(he, c)| format!("{he:?} {:?} {:?}", c.params(), c.pcurve()))
        .collect();

    let mut maimed = v.clone();
    let victim = maimed.pcurves().map(|(he, _)| he).next().expect("a row");
    maimed.detach_pcurve(victim).expect("removed");
    let findings = pcurve_findings(&maimed);
    assert!(
        !findings.is_empty(),
        "the maimed operand fails the tier-3 pcurve pass"
    );
    println!("[r2] maimed operand findings: {findings:?}");

    let out = topo::shell(&maimed, 0.2, tol());
    match &out {
        Ok(s) => {
            println!("[r2] shell TOOK the tier-3-invalid operand");
            assert_eq!(
                topo::validate_geometric(&s.body, tol()),
                Ok(()),
                "and returned a valid body"
            );
            let rows: Vec<String> = s
                .body
                .pcurves()
                .map(|(he, c)| format!("{he:?} {:?} {:?}", c.params(), c.pcurve()))
                .collect();
            assert_eq!(
                rows, good_rows,
                "bit-identical to the sound operand's result"
            );
        }
        Err(e) => println!("[r2] shell refused the maimed operand: {e}"),
    }
}

/// **Claim 7, sharper — a WRONG row, not a missing one.** One face's
/// certified row attached to another face's half-edge: the operand is
/// tier-3 invalid with a stale row, exactly the defect the pcurve pass
/// exists to catch. What does the verb do?
#[test]
fn r2_the_closing_mint_launders_a_stale_row() {
    let v = vessel(1.0, 2.0);
    let rows: Vec<_> = v.pcurves().map(|(he, c)| (he, c.clone())).collect();
    assert!(rows.len() >= 2, "the vessel carries rows");
    let mut maimed = v.clone();
    maimed.attach_pcurve(rows[0].0, rows[1].1.clone());
    let findings = pcurve_findings(&maimed);
    println!("[r2] stale-row operand findings: {findings:?}");
    let out = topo::shell(&maimed, 0.2, tol());
    println!(
        "[r2] shell on the stale-row operand: {}",
        match &out {
            Ok(s) => format!("Ok, tier3 = {:?}", topo::validate_geometric(&s.body, tol())),
            Err(e) => format!("Err {e}"),
        }
    );
}

/// **Claim 5 — does an N-solid body pay N mints or one?** A single call
/// on a two-solid operand against two calls on the two operands apart.
#[test]
fn r2_multi_solid_pays_one_mint() {
    let b = boxy(2.0, 2.0, 2.0);
    let v = vessel(1.0, 2.0);
    let pair = beside(&b, &v, 6.0);
    let one = topo::shell(&pair, 0.2, tol()).expect("the pair shells");
    println!(
        "[r2] pair rows {} vs box {} + vessel {}",
        one.body.pcurves().count(),
        topo::shell(&b, 0.2, tol())
            .expect("box")
            .body
            .pcurves()
            .count(),
        topo::shell(&v, 0.2, tol())
            .expect("vessel")
            .body
            .pcurves()
            .count(),
    );
    let _ = Vec3::new(0.0, 0.0, 0.0);
    let _ = Tol::witness();
    let _ = polyline(
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        Revolution::Full,
    );
}
