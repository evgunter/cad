//! VERBS-SHELLFIX PR-1 — R1 blinded review probes (ordinal 101).
//!
//! Probe branch only; NOT part of the PR under review. Every fixture
//! here is outside the PR's own enumeration, so each green/red row is
//! independent signal:
//!
//! - P1: a PARTIAL revolve's cap (seam that is NOT a full-period slit);
//! - P2: an annular mouth worn by TWO HALF-ANNULI (axis-touching body
//!   with a counterbore — kef across TWO shared seam legs, then kemr);
//! - P3: a multi-segment vase opened at its BOTTOM (lift sign);
//! - P4: the annular split on my own radii/thickness;
//! - P5: a TWO-holed designation (the stated single-hole limitation);
//! - P6: a ONE-square-holed slab (the split on LINE carriers, which no
//!   PR fixture exercises — their splits are all circles);
//! - P7: check 9's margin band at the door (the thickness gate shields
//!   the band, measured);
//! - P8: the sealed arm's #1056 hollow-operand gate still fires.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, FaceKey, LoopBoundary, ShellError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

const FIT_TOL: f64 = 1e-6;

fn polygon(pts: &[(f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::new(
        pts.iter()
            .map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    )
}

fn revolved_full(pts: &[(f64, f64)]) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![polygon(pts)])
        .validate(Tol::witness())
        .expect("meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("meridian revolves")
    .body
}

fn extruded(loops: Vec<ProfileLoop<f64>>, h: f64) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .expect("profile validates");
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .expect("profile extrudes")
        .body
}

/// Every planar face whose plane origin sits at height `y` about the
/// revolve axis (+y fixtures).
fn plane_chart_at_y(body: &Body<f64>, y: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect()
}

/// Planar faces at extrusion height `z` (xy-sketch extrudes along z).
fn plane_chart_at_z(body: &Body<f64>, z: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if (origin.z - z).abs() < 1e-12
                        && normal.x.abs() < 1e-9 && normal.y.abs() < 1e-9)
        })
        .map(|(k, _)| k)
        .collect()
}

/// **One of NINE copies of this helper across five crates (#1123).**
/// `demos/tour` is a separate workspace and an integration test cannot
/// import a binary's module, so no existing home covers them all; the
/// issue carries the list and the shared-test-support fix.
fn rings_of(body: &Body<f64>) -> usize {
    body.faces().map(|(_, f)| f.rings.len()).sum()
}

fn genus_of(body: &Body<f64>) -> i64 {
    let (v, e, f) = (
        body.vertices().count() as i64,
        body.edges().count() as i64,
        body.faces().count() as i64,
    );
    let chi = v - e + f - rings_of(body) as i64;
    assert!(chi % 2 == 0, "v - e + f - r = {chi} is ODD");
    body.shells().count() as i64 - chi / 2
}

/// The whole-body coherence bar: tier 3, meshes at two budgets, and a
/// closed-form volume when one is given. "Never a validated wrong
/// body" is exactly this set.
fn assert_coherent(what: &str, cup: &Body<f64>, want_volume: Option<f64>) {
    assert_eq!(
        topo::validate_geometric(cup, Tol::witness()),
        Ok(()),
        "{what}: tier 3"
    );
    for delta in [1e-2, 1e-3] {
        mesh::tessellate(cup, delta, Tol::witness())
            .unwrap_or_else(|e| panic!("{what}: must triangulate at delta = {delta}, got {e:?}"));
    }
    if let Some(want) = want_volume {
        let props = topo::mass_properties(cup, Tol::witness()).expect("props");
        assert!(
            (props.volume - want).abs() <= 1e-9 + props.volume_pad,
            "{what}: volume {} (pad {}), want {want}",
            props.volume,
            props.volume_pad
        );
    }
}

// ---------------------------------------------------------------------
// P1 — the partial revolve's cap: a chart whose seam is NOT a
// full-period slit. Whatever the outcome, it must not be a validated
// wrong body.
// ---------------------------------------------------------------------

#[test]
fn p1_partial_revolve_cap_is_never_a_validated_wrong_body() {
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![polygon(&[(0.0, 0.0), (0.5, 0.0), (0.5, 0.4), (0.0, 0.4)])],
    )
    .validate(Tol::witness())
    .expect("meridian validates");
    let body = revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        Tol::witness(),
    )
    .expect("the wedge revolves")
    .body;
    let chart = plane_chart_at_y(&body, 0.4);
    println!("[p1] wedge cap chart: {} face(s)", chart.len());
    assert!(!chart.is_empty(), "the wedge has a top cap");
    match topo::shell_open(&body, 0.05, &chart, FIT_TOL, Tol::witness()) {
        Err(e) => println!("[p1] REFUSED typed: {e}"),
        Ok(cup) => {
            println!(
                "[p1] BUILT: shells {}, rings {}, genus {}",
                cup.shells().count(),
                rings_of(&cup),
                genus_of(&cup)
            );
            assert_coherent("p1 wedge cup", &cup, None);
        }
    }
}

// ---------------------------------------------------------------------
// P2 — the counterbored drum: the mouth is an ANNULUS worn by TWO
// half-annuli (the body touches the axis below, so the full revolve
// halves every face). Neither PR fixture has this chart shape: the
// axis-touching cap merges into a disc (spur kill), the annular cap is
// ONE slit face (no merge at all). Here canonicalize must run kef
// across one seam leg AND kemr on the other.
// ---------------------------------------------------------------------

#[test]
fn p2_counterbore_mouth_two_half_annuli_split_into_two_rims() {
    let (ro, rb, h, depth, t) = (0.5, 0.2, 0.4, 0.2, 0.05);
    let body = revolved_full(&[
        (0.0, 0.0),
        (ro, 0.0),
        (ro, h),
        (rb, h),
        (rb, h - depth),
        (0.0, h - depth),
    ]);
    let chart = plane_chart_at_y(&body, h);
    println!("[p2] mouth chart: {} face(s)", chart.len());
    // MEASURED at first run: this designation refuses at the standing
    // OpenFacesDisconnect gate — the mouth annulus is the ONLY bridge
    // between {outer wall, base} and {bore wall, recess floor}, so the
    // remainder falls into two components. The two-half-annuli chart
    // shape therefore never reaches `canonicalize_chart` through this
    // door on this operand class; the kef-then-kemr composition (a
    // merge whose leftover duplicate is anchored at BOTH ends) stays
    // door-unreachable here. Pinned as the typed refusal it is: never
    // a validated wrong body.
    match topo::shell_open(&body, t, &chart, FIT_TOL, Tol::witness()) {
        Err(ShellError::OpenFacesDisconnect { components, .. }) => {
            println!("[p2] refused typed at the designation gate: {components} components");
            assert_eq!(components, 2);
        }
        Err(e) => panic!("[p2] expected the disconnect gate or a build, got {e}"),
        Ok(cup) => {
            let mouth = plane_chart_at_y(&cup, h);
            println!(
                "[p2] BUILT: shells {}, rings {}, genus {}, mouth faces {}",
                cup.shells().count(),
                rings_of(&cup),
                genus_of(&cup),
                mouth.len()
            );
            assert_eq!(cup.shells().count(), 1, "the cavity fuses in");
            assert_eq!(mouth.len(), 2, "two disjoint rim annuli on the mouth plane");
            // The rims' radii: [rb, rb+t] and [ro-t, ro].
            let mut radii: Vec<(f64, f64)> = mouth
                .iter()
                .map(|&k| {
                    let f = cup.get_face(k).expect("rim face");
                    assert_eq!(f.rings.len(), 1, "each rim is an annulus");
                    let radius = |lk| {
                        let LoopBoundary::Cycle { first } =
                            cup.get_loop(lk).expect("loop").boundary
                        else {
                            panic!("empty rim loop")
                        };
                        let he = cup.loop_cycle(first).expect("cycle")[0];
                        let e = cup
                            .get_edge(cup.get_half_edge(he).expect("he").edge)
                            .expect("edge");
                        match cup
                            .get_curve_geom(e.curve)
                            .and_then(|g| g.certified())
                            .expect("carrier")
                            .carrier()
                        {
                            geom::Curve3::Circle { radius, .. } => *radius,
                            other => panic!("rim bounded by {other:?}"),
                        }
                    };
                    (radius(f.rings[0]), radius(f.outer))
                })
                .collect();
            radii.sort_by(|a, b| a.0.total_cmp(&b.0));
            for (got, want) in radii.iter().zip([(rb, rb + t), (ro - t, ro)]) {
                assert!(
                    (got.0 - want.0).abs() < 1e-12 && (got.1 - want.1).abs() < 1e-12,
                    "[p2] rim between {got:?}, want {want:?}"
                );
            }
            // Closed form: solid minus (cavity plus the lifted column).
            let pi = core::f64::consts::PI;
            let v_solid = pi * (ro * ro * h - rb * rb * depth);
            let v_cav = pi
                * ((ro - t) * (ro - t) * (h - 2.0 * t)
                    - (rb + t) * (rb + t) * ((h - t) - (h - depth - t)));
            let v_lift = pi * ((ro - t) * (ro - t) - (rb + t) * (rb + t)) * t;
            assert_coherent("p2 counterbore cup", &cup, Some(v_solid - v_cav - v_lift));
        }
    }
}

// ---------------------------------------------------------------------
// P3 — a three-tier vase opened at its BOTTOM: the multi-segment
// meridian MINT, with the lift running the other way.
// ---------------------------------------------------------------------

#[test]
fn p3_vase_opened_at_its_bottom_mints_one_annular_rim() {
    let t = 0.02;
    let body = revolved_full(&[
        (0.0, 0.0),
        (0.21, 0.0),
        (0.21, 0.07),
        (0.34, 0.07),
        (0.34, 0.19),
        (0.11, 0.19),
        (0.11, 0.31),
        (0.0, 0.31),
    ]);
    let chart = plane_chart_at_y(&body, 0.0);
    assert_eq!(chart.len(), 2, "the base is two half-discs");
    let cup = topo::shell_open(&body, t, &chart, FIT_TOL, Tol::witness())
        .unwrap_or_else(|e| panic!("[p3] the vase opens at its base, got {e}"));
    assert_eq!(cup.shells().count(), 1);
    assert_eq!(
        (rings_of(&cup), genus_of(&cup)),
        (1, 0),
        "one rim annulus, genus 0"
    );
    assert_eq!(
        plane_chart_at_y(&cup, 0.0).len(),
        1,
        "the two half-discs became ONE rim face"
    );
    let pi = core::f64::consts::PI;
    let v_solid = pi * (0.21f64.powi(2) * 0.07 + 0.34f64.powi(2) * 0.12 + 0.11f64.powi(2) * 0.12);
    let v_cav = pi
        * ((0.21 - t) * (0.21 - t) * (0.09 - t)
            + (0.34 - t) * (0.34 - t) * (0.17 - 0.09)
            + (0.11 - t) * (0.11 - t) * (0.29 - 0.17));
    let v_lift = pi * (0.21 - t) * (0.21 - t) * t;
    assert_coherent("p3 vase", &cup, Some(v_solid - v_cav - v_lift));
}

// ---------------------------------------------------------------------
// P4 — the annular SPLIT on my own radii and thickness.
// ---------------------------------------------------------------------

#[test]
fn p4_annular_split_holds_on_fresh_radii() {
    let (ri, ro, h, t) = (0.7, 1.1, 0.6, 0.08);
    let body = revolved_full(&[(ri, 0.0), (ro, 0.0), (ro, h), (ri, h)]);
    let chart = plane_chart_at_y(&body, h);
    assert_eq!(chart.len(), 1, "a closed off-axis meridian closes its seam");
    let cup = topo::shell_open(&body, t, &chart, FIT_TOL, Tol::witness())
        .unwrap_or_else(|e| panic!("[p4] the tube opens, got {e}"));
    assert_eq!(cup.shells().count(), 1);
    assert_eq!(
        (rings_of(&cup), genus_of(&cup)),
        (2, 1),
        "two rim annuli; the bore keeps genus 1"
    );
    assert_eq!(plane_chart_at_y(&cup, h).len(), 2);
    let pi = core::f64::consts::PI;
    let want = pi * ((ro * ro - ri * ri) * h - ((ro - t).powi(2) - (ri + t).powi(2)) * (h - t));
    assert_coherent("p4 fresh-radii tube", &cup, Some(want));
}

// ---------------------------------------------------------------------
// P5 — TWO holes in the designated face: the stated single-hole
// limitation must refuse typed, never guess a pairing.
// ---------------------------------------------------------------------

#[test]
fn p5_two_holed_designation_refuses_typed() {
    let (w, d, h, t) = (1.2, 0.9, 0.3, 0.04);
    let s = 0.1; // hole half-side
    let outer = polygon(&[(0.0, 0.0), (w, 0.0), (w, d), (0.0, d)]);
    let hole = |cx: f64, cy: f64| {
        polygon(&[
            (cx - s, cy - s),
            (cx + s, cy - s),
            (cx + s, cy + s),
            (cx - s, cy + s),
        ])
    };
    let body = extruded(vec![outer, hole(0.3, 0.45), hole(0.9, 0.45)], h);
    let top = plane_chart_at_z(&body, h);
    assert_eq!(top.len(), 1, "one top face");
    assert_eq!(
        body.get_face(top[0]).unwrap().rings.len(),
        2,
        "carrying two holes"
    );
    match topo::shell_open(&body, t, &top, FIT_TOL, Tol::witness()) {
        Err(ShellError::OpenFaceRimNotExpressible { what, .. }) => {
            println!("[p5] refused: {what}");
            assert!(
                what.contains("single"),
                "[p5] the refusal must name the single-hole scope, got: {what}"
            );
        }
        Err(e) => panic!("[p5] expected OpenFaceRimNotExpressible, got {e}"),
        Ok(cup) => {
            // If it builds, it must at least be coherent — but the PR
            // says this door refuses, so a build is a finding either way.
            assert_coherent("p5 two-holed cup", &cup, None);
            panic!("[p5] the PR says a two-holed designation refuses; it built");
        }
    }
}

// ---------------------------------------------------------------------
// P6 — ONE square hole: the split path on LINE carriers. Every split
// fixture in the PR is circles; `encloses` and the mfkrh/ring_move
// promotion have never been shown on straight edges.
// ---------------------------------------------------------------------

#[test]
fn p6_single_square_hole_splits_on_line_carriers() {
    let (w, d, h, t) = (1.0, 0.8, 0.3, 0.04);
    let s = 0.15; // hole half-side
    let (cx, cy) = (0.5, 0.4);
    let outer = polygon(&[(0.0, 0.0), (w, 0.0), (w, d), (0.0, d)]);
    let hole = polygon(&[
        (cx - s, cy - s),
        (cx + s, cy - s),
        (cx + s, cy + s),
        (cx - s, cy + s),
    ]);
    let body = extruded(vec![outer, hole], h);
    let top = plane_chart_at_z(&body, h);
    assert_eq!(top.len(), 1);
    match topo::shell_open(&body, t, &top, FIT_TOL, Tol::witness()) {
        Err(e) => {
            // A refusal here is a FINDING (an undisclosed narrowing to
            // circular splits) — record it loudly.
            panic!("[p6] a one-holed slab is inside the stated scope and refused: {e}");
        }
        Ok(cup) => {
            assert_eq!(cup.shells().count(), 1);
            // FOUR rings total, not two: the operand's through-hole
            // already puts one ring on the bottom cap and one on the
            // cavity's bottom counterpart, and the split adds one per
            // rim band (first run: wanted (2,1), measured (4,1) — the
            // fixture's own holes were the miscount, not the rim).
            assert_eq!(
                (rings_of(&cup), genus_of(&cup)),
                (4, 1),
                "two rim bands plus the holed bottom cap and its cavity counterpart; genus 1"
            );
            let rims = plane_chart_at_z(&cup, h);
            assert_eq!(rims.len(), 2, "two rim faces");
            for k in &rims {
                assert_eq!(
                    cup.get_face(*k).unwrap().rings.len(),
                    1,
                    "each rim is a band"
                );
            }
            let want = w * d * h
                - (2.0 * s).powi(2) * h
                - ((w - 2.0 * t) * (d - 2.0 * t) - (2.0 * s + 2.0 * t).powi(2)) * (h - t);
            assert_coherent("p6 square-holed cup", &cup, Some(want));
        }
    }
}

// ---------------------------------------------------------------------
// P7 — check 9's margins at the door: sub-ambiguity walls are refused
// by the THICKNESS gate before any ring can land near the outer loop,
// and a wall at the escalation floor (K·eps) still builds a valid,
// meshable rim. Measured, because the check's own band is unreachable
// through this verb.
// ---------------------------------------------------------------------

#[test]
fn p7_thickness_gate_shields_check_9s_band_at_this_door() {
    let body = extruded(
        vec![polygon(&[(0.0, 0.0), (2.0, 0.0), (2.0, 3.0), (0.0, 3.0)])],
        4.0,
    );
    let top = plane_chart_at_z(&body, 4.0);
    // In the band or below: the thickness gate refuses first.
    for t in [3e-7, 5e-6] {
        match topo::shell_open(&body, t, &top, FIT_TOL, Tol::witness()) {
            Err(ShellError::Thickness { .. }) => println!("[p7] t = {t}: thickness gate"),
            Err(e) => println!("[p7] t = {t}: refused elsewhere: {e}"),
            Ok(cup) => {
                assert_coherent("p7 thin cup", &cup, None);
                println!("[p7] t = {t}: BUILT and coherent (note: ring sits {t} from outer)");
            }
        }
    }
    // At the escalation floor: certifiably positive, must build, and
    // the ring at 1e-5 from the outer loop must NOT trip check 9.
    let t = 2e-5;
    let cup = topo::shell_open(&body, t, &top, FIT_TOL, Tol::witness())
        .unwrap_or_else(|e| panic!("[p7] t = {t} is certifiably positive, got {e}"));
    assert_eq!(
        topo::validate_geometric(&cup, Tol::witness()),
        Ok(()),
        "[p7] a legitimate near ring (2e-5 gap) must not refuse at rest"
    );
}

// ---------------------------------------------------------------------
// P8 — the #1056 hollow-operand gate still fires after the rewrite.
// ---------------------------------------------------------------------

#[test]
fn p8_hollow_operand_still_refuses_typed() {
    let body = extruded(
        vec![polygon(&[(0.0, 0.0), (2.0, 0.0), (2.0, 3.0), (0.0, 3.0)])],
        4.0,
    );
    let sealed = topo::shell(&body, 0.25, FIT_TOL, Tol::witness()).expect("seals");
    let e =
        topo::shell(&sealed, 0.05, FIT_TOL, Tol::witness()).expect_err("a hollow operand refuses");
    assert!(
        matches!(e, ShellError::OperandAlreadyHollow { shells: 2 }),
        "got {e}"
    );
}
