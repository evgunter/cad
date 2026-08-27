//! **R2 review probes for VERBS-SHELLFIX PR-1 (#1099 / #1082).**
//!
//! Instruments, not acceptance. Each row prints what it measures so a
//! reader can see the shape rather than only a pass/fail bit. Rows that
//! assert do so only where the PR makes a load-bearing claim.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, FaceKey, ShellError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}
fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}
const FIT_TOL: f64 = 1e-6;

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

/// Faces whose plane origin sits at height `y` (the revolve fixtures
/// sketch on xy and revolve about +y, so caps are planes at origin.y).
fn plane_chart_at_y(body: &Body<f64>, y: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect()
}

/// Faces whose plane origin sits at height `z` (the extrude fixtures).
fn plane_chart_at_z(body: &Body<f64>, z: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if (origin.z - z).abs() < 1e-12 && normal.x.abs() < 1e-9 && normal.y.abs() < 1e-9)
        })
        .map(|(k, _)| k)
        .collect()
}

fn revolved(loops: Vec<ProfileLoop<f64>>, revolution: Revolution<f64>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .expect("a valid meridian");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        revolution,
        Tol::witness(),
    )
    .expect("the meridian revolves")
    .body
}

fn poly(pts: &[(f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::new(
        pts.iter()
            .map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    )
}

/// A circle as a two-vertex bulge loop, CCW.
fn circle_loop(cx: f64, cy: f64, r: f64) -> ProfileLoop<f64> {
    RawLoop::new(vec![
        ProfileVertex::new(p2(cx - r, cy), 1.0),
        ProfileVertex::new(p2(cx + r, cy), 1.0),
    ])
}

fn extruded(loops: Vec<ProfileLoop<f64>>, h: f64) -> Option<Body<f64>> {
    let profile = Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .ok()?;
    Some(extrude(&profile, Extrusion::Distance(h), Tol::witness()).ok()?.body)
}

/// Report an opened body's shape, or the typed refusal.
fn report(what: &str, opened: &Result<Body<f64>, ShellError<f64>>) {
    match opened {
        Ok(cup) => {
            let tier3 = topo::validate_geometric(cup, Tol::witness());
            let mesh_ok: Vec<String> = [1e-2, 1e-3, 2e-4]
                .iter()
                .map(|&d| match mesh::tessellate(cup, d, Tol::witness()) {
                    Ok(_) => format!("{d}:ok"),
                    Err(e) => format!("{d}:REFUSED({e:?})"),
                })
                .collect();
            println!(
                "  [{what}] OPENED shells={} faces={} rings={} genus={} tier3={} mesh=[{}]",
                cup.shells().count(),
                cup.faces().count(),
                rings_of(cup),
                genus_of(cup),
                if tier3.is_ok() {
                    "ok".to_string()
                } else {
                    format!("{tier3:?}")
                },
                mesh_ok.join(" ")
            );
        }
        Err(e) => println!("  [{what}] REFUSED: {e}"),
    }
}

// =====================================================================
// Claim 1 — the central inversion: an OPERAND artefact, not a surgery
// limit. Attacked with revolved shapes outside the PR's fixtures.
// =====================================================================

/// **A PARTIAL revolve's cap** — the seam is NOT a full-period slit:
/// the wedge's cap is one face bounded by two radial legs and an arc,
/// with no duplicate edge for `canonicalize_chart` to find. Either it
/// opens correctly or it refuses typed; never a validated wrong body.
#[test]
fn r2_partial_revolve_axis_touching_cap() {
    let tol = Tol::witness();
    let (r, h, t) = (0.5, 0.4, 0.05);
    for theta in [core::f64::consts::FRAC_PI_2, 2.4] {
        let body = revolved(
            vec![poly(&[(0.0, 0.0), (r, 0.0), (r, h), (0.0, h)])],
            Revolution::Partial(theta),
        );
        let chart = plane_chart_at_y(&body, h);
        println!("theta={theta}: cap chart has {} face(s)", chart.len());
        let opened = topo::shell_open(&body, t, &chart, FIT_TOL, band(), tol);
        report(&format!("partial axis-touching theta={theta}"), &opened);
        if let Ok(cup) = &opened {
            assert_eq!(
                topo::validate_geometric(cup, tol),
                Ok(()),
                "a partial revolve's cup must be valid or refuse"
            );
            for delta in [1e-2, 1e-3, 2e-4] {
                mesh::tessellate(cup, delta, tol).unwrap_or_else(|e| {
                    panic!("VALIDATED WRONG BODY: valid but no mesh at {delta}: {e:?}")
                });
            }
            let props = topo::mass_properties(cup, tol).expect("props");
            let want = 0.5 * theta * (r * r * h - (r - t) * (r - t) * (h - t));
            println!(
                "    volume got {} want (naive wedge closed form) {want}",
                props.volume
            );
        } else {
            assert!(
                matches!(
                    opened,
                    Err(ShellError::OpenFaceRimNotExpressible { .. })
                        | Err(ShellError::Rim { .. })
                        | Err(ShellError::Lift { .. })
                ),
                "the refusal must be typed"
            );
        }
    }
}

/// **A PARTIAL revolve of an ANNULAR meridian** — one wedge-annulus cap
/// face, no slit, a hole that is not a closed ring.
#[test]
fn r2_partial_revolve_annular_cap() {
    let tol = Tol::witness();
    let (ri, ro, h, t) = (0.30, 0.50, 0.40, 0.05);
    let body = revolved(
        vec![poly(&[(ri, 0.0), (ro, 0.0), (ro, h), (ri, h)])],
        Revolution::Partial(2.0),
    );
    let chart = plane_chart_at_y(&body, h);
    println!("partial annular: cap chart has {} face(s)", chart.len());
    let opened = topo::shell_open(&body, t, &chart, FIT_TOL, band(), tol);
    report("partial annular cap", &opened);
    if let Ok(cup) = &opened {
        assert_eq!(topo::validate_geometric(cup, tol), Ok(()));
        for delta in [1e-2, 1e-3, 2e-4] {
            mesh::tessellate(cup, delta, tol).unwrap_or_else(|e| {
                panic!("VALIDATED WRONG BODY: valid but no mesh at {delta}: {e:?}")
            });
        }
    }
}

/// **A profile touching the axis at ONE end only.** The bottom cap is a
/// disc that owns the axis apex; the top cap is a slit annulus. Both
/// designations are probed.
#[test]
fn r2_axis_at_one_end_only() {
    let tol = Tol::witness();
    let (r, a, h, t) = (0.5, 0.2, 0.4, 0.05);
    let body = revolved(
        vec![poly(&[(0.0, 0.0), (r, 0.0), (r, h), (a, h)])],
        Revolution::Full,
    );
    for (name, y) in [("the annular TOP", h), ("the axis-touching BOTTOM", 0.0)] {
        let chart = plane_chart_at_y(&body, y);
        println!("{name}: chart has {} face(s)", chart.len());
        if chart.is_empty() {
            continue;
        }
        let opened = topo::shell_open(&body, t, &chart, FIT_TOL, band(), tol);
        report(name, &opened);
        if let Ok(cup) = &opened {
            assert_eq!(topo::validate_geometric(cup, tol), Ok(()), "{name}");
            for delta in [1e-2, 1e-3, 2e-4] {
                mesh::tessellate(cup, delta, tol).unwrap_or_else(|e| {
                    panic!("{name}: VALIDATED WRONG BODY at {delta}: {e:?}")
                });
            }
        }
    }
}

/// **A multi-segment (stepped) meridian's cap** — my own vase, five
/// segments, axis-touching at both ends. Claim 3's MINT on a fixture
/// the PR does not use.
#[test]
fn r2_stepped_meridian_vase_mints_one_annular_rim() {
    let tol = Tol::witness();
    let t = 0.04;
    let h = 0.62;
    let body = revolved(
        vec![poly(&[
            (0.0, 0.0),
            (0.44, 0.0),
            (0.44, 0.18),
            (0.31, 0.35),
            (0.37, h),
            (0.0, h),
        ])],
        Revolution::Full,
    );
    let chart = plane_chart_at_y(&body, h);
    println!("vase: mouth chart has {} face(s)", chart.len());
    let opened = topo::shell_open(&body, t, &chart, FIT_TOL, band(), tol);
    report("stepped vase", &opened);
    let cup = opened.expect("the vase's stepped meridian must open (claim 3's MINT)");
    assert_eq!(topo::validate_geometric(&cup, tol), Ok(()), "tier 3");
    assert_eq!(cup.shells().count(), 1);
    assert_eq!(
        (rings_of(&cup), genus_of(&cup)),
        (1, 0),
        "one annular rim, one ring, genus 0"
    );
    assert_eq!(plane_chart_at_y(&cup, h).len(), 1, "ONE rim face");
    for delta in [1e-2, 1e-3, 2e-4] {
        mesh::tessellate(&cup, delta, tol)
            .unwrap_or_else(|e| panic!("the vase rim must mesh at {delta}: {e:?}"));
    }
}

// =====================================================================
// Claim 2 — the annular SPLIT, on my own radii, volume re-derived.
// =====================================================================

/// My own tube: different radii and thickness from the PR's fixture.
/// Closed form re-derived here rather than copied:
/// V = π[(ro²−ri²)h − ((ro−t)²−(ri+t)²)(h−t)].
#[test]
fn r2_my_own_annulus_splits_into_two_rims() {
    let tol = Tol::witness();
    let (ri, ro, h, t) = (0.12, 0.37, 0.55, 0.031);
    let body = revolved(
        vec![poly(&[(ri, 0.0), (ro, 0.0), (ro, h), (ri, h)])],
        Revolution::Full,
    );
    let chart = plane_chart_at_y(&body, h);
    assert_eq!(chart.len(), 1, "a closed off-axis meridian closes its seam");
    let cup = topo::shell_open(&body, t, &chart, FIT_TOL, band(), tol).expect("my tube opens");
    assert_eq!(topo::validate_geometric(&cup, tol), Ok(()), "tier 3");
    assert_eq!(cup.shells().count(), 1);
    assert_eq!((rings_of(&cup), genus_of(&cup)), (2, 1));
    let mouth = plane_chart_at_y(&cup, h);
    assert_eq!(mouth.len(), 2, "two disjoint rim annuli");
    for delta in [1e-2, 1e-3, 2e-4] {
        mesh::tessellate(&cup, delta, tol)
            .unwrap_or_else(|e| panic!("the two rims must mesh at {delta}: {e:?}"));
    }
    let props = topo::mass_properties(&cup, tol).expect("props");
    let want = core::f64::consts::PI
        * ((ro * ro - ri * ri) * h - ((ro - t).powi(2) - (ri + t).powi(2)) * (h - t));
    println!(
        "  my tube: V={} pad={} want={want} delta={}",
        props.volume,
        props.volume_pad,
        (props.volume - want).abs()
    );
    assert!(
        (props.volume - want).abs() <= 1e-9 + props.volume_pad,
        "my tube cup volume: got {} want {want}",
        props.volume
    );
}

/// **The deliberate single-hole limitation.** A designated face with
/// TWO holes must refuse typed, not guess.
#[test]
fn r2_two_holed_designation_refuses_typed() {
    let tol = Tol::witness();
    let outer = poly(&[(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)]);
    let body = extruded(
        vec![
            outer,
            circle_loop(0.5, 0.5, 0.18),
            circle_loop(1.5, 0.5, 0.18),
        ],
        0.6,
    )
    .expect("a twice-holed rectangle extrudes");
    let top = plane_chart_at_z(&body, 0.6);
    println!("two-holed: top chart {} face(s)", top.len());
    let opened = topo::shell_open(&body, 0.05, &top, FIT_TOL, band(), tol);
    report("two-holed designation", &opened);
    match opened {
        Err(ShellError::OpenFaceRimNotExpressible { what, .. }) => {
            println!("  typed refusal: {what}");
            assert!(
                what.contains("single hole"),
                "the refusal must name the single-hole limitation, got {what}"
            );
        }
        other => panic!("a two-holed designation must refuse typed; got {other:?}"),
    }
}

/// **The ONE-holed control** (extruded, not revolved): the door's
/// single-hole pairing on a shape with no seam at all.
#[test]
fn r2_one_holed_extrusion_opens() {
    let tol = Tol::witness();
    let outer = poly(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
    let body = extruded(vec![outer, circle_loop(0.5, 0.5, 0.2)], 0.6)
        .expect("a holed rectangle extrudes");
    let top = plane_chart_at_z(&body, 0.6);
    let opened = topo::shell_open(&body, 0.05, &top, FIT_TOL, band(), tol);
    report("one-holed extrusion", &opened);
    if let Ok(cup) = &opened {
        assert_eq!(topo::validate_geometric(cup, tol), Ok(()));
        for delta in [1e-2, 1e-3, 2e-4] {
            mesh::tessellate(cup, delta, tol)
                .unwrap_or_else(|e| panic!("VALIDATED WRONG BODY at {delta}: {e:?}"));
        }
    }
}

// =====================================================================
// Claim 5 — the tier-3 net (check 9, RingMeetsOuter). Attacks.
// =====================================================================

/// **A legitimate ring NEAR the outer loop must NOT refuse**, and the
/// ladder through the ambiguity band is printed rather than assumed.
/// Band at the default lane: zero = 1e-9, escalate = 1e-8. A gap in
/// `(zero, escalate)` is `Err(Indeterminate)` at the decide seam —
/// this row records what check 9 does with it.
#[test]
fn r2_check9_gap_ladder_on_a_near_ring() {
    let tol = Tol::witness();
    println!(
        "band: zero={:?} escalate={:?}",
        band().zero(),
        band().escalate()
    );
    for gap in [1e-2, 1e-4, 1e-7, 5e-9, 1e-11, 0.0] {
        // A square hole whose corner sits `gap` from the outer corner
        // along the diagonal.
        let g = gap / core::f64::consts::SQRT_2;
        let outer = poly(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
        let hole = poly(&[(g, g), (0.6, 0.2), (0.6, 0.6), (0.2, 0.6)]);
        // Holes wind the other way in a profile; try both windings.
        let hole_cw = poly(&[(g, g), (0.2, 0.6), (0.6, 0.6), (0.6, 0.2)]);
        let mut built = None;
        for h in [hole, hole_cw] {
            if let Some(b) = extruded(vec![outer.clone(), h], 0.4) {
                built = Some(b);
                break;
            }
        }
        let Some(body) = built else {
            println!("  gap={gap}: the PROFILE gate refused it (no body)");
            continue;
        };
        let v = topo::validate_geometric(&body, tol);
        let ring_meets = match &v {
            Ok(()) => "none".to_string(),
            Err(es) => {
                let hits: Vec<String> = es
                    .iter()
                    .filter(|e| matches!(e, topo::ValidationError::RingMeetsOuter { .. }))
                    .map(|e| format!("{e}"))
                    .collect();
                if hits.is_empty() {
                    format!("none (other errors: {} of them)", es.len())
                } else {
                    hits.join("; ")
                }
            }
        };
        let meshes = mesh::tessellate(&body, 1e-3, tol).is_ok();
        println!("  gap={gap}: check9={ring_meets} | meshes={meshes}");
        if gap >= 1e-7 {
            assert!(
                !matches!(&v, Err(es) if es.iter().any(|e| matches!(e, topo::ValidationError::RingMeetsOuter { .. }))),
                "gap={gap} is far above the band: check 9 must NOT refuse a legitimate near ring"
            );
        }
    }
}

/// **A ring VERTEX standing on the INTERIOR of an outer edge.** Not a
/// shared vertex position, not an edge running along an edge — the two
/// shapes check 9 matches. This row records whether the net sees it.
#[test]
fn r2_check9_vertex_on_outer_edge_interior() {
    let tol = Tol::witness();
    for gap in [1e-3, 1e-7, 1e-11, 0.0] {
        let outer = poly(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
        // A triangular hole with one vertex reaching down to y = gap,
        // i.e. `gap` above the middle of the outer loop's bottom edge.
        let hole = poly(&[(0.5, gap), (0.75, 0.6), (0.25, 0.6)]);
        let hole_r = poly(&[(0.5, gap), (0.25, 0.6), (0.75, 0.6)]);
        let mut built = None;
        for h in [hole, hole_r] {
            if let Some(b) = extruded(vec![outer.clone(), h], 0.4) {
                built = Some(b);
                break;
            }
        }
        let Some(body) = built else {
            println!("  gap={gap}: the PROFILE gate refused it");
            continue;
        };
        let v = topo::validate_geometric(&body, tol);
        let saw_check9 = matches!(&v, Err(es) if es.iter().any(|e| matches!(e, topo::ValidationError::RingMeetsOuter { .. })));
        let meshes = mesh::tessellate(&body, 1e-3, tol).is_ok();
        println!(
            "  vertex-on-edge gap={gap}: check9_fired={saw_check9} tier3_ok={} meshes={meshes}",
            v.is_ok()
        );
    }
}

/// **A ring TANGENT to the outer loop at one point** — a circular hole
/// inside a square, touching one side. The overlap arm needs three
/// shared samples; a tangency shares one.
#[test]
fn r2_check9_tangent_ring() {
    let tol = Tol::witness();
    for gap in [1e-3, 1e-7, 1e-11, 0.0] {
        let outer = poly(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
        let hole = circle_loop(0.5, 0.5, 0.5 - gap);
        let Some(body) = extruded(vec![outer, hole], 0.4) else {
            println!("  tangent gap={gap}: the PROFILE gate refused it");
            continue;
        };
        let v = topo::validate_geometric(&body, tol);
        let saw_check9 = matches!(&v, Err(es) if es.iter().any(|e| matches!(e, topo::ValidationError::RingMeetsOuter { .. })));
        let meshes = mesh::tessellate(&body, 1e-3, tol).is_ok();
        println!(
            "  tangent gap={gap}: check9_fired={saw_check9} tier3_ok={} meshes={meshes}",
            v.is_ok()
        );
    }
}

/// **A collinear ring edge overlapping an outer edge OFF the ring
/// edge's midpoint.** The `Line` trim step tests only `samples[1]`, the
/// ring edge's MIDDLE sample, against the outer edge's endpoints — so
/// an overlap that does not contain the ring edge's midpoint is not
/// matched by the trim step. Recorded as an instrument.
#[test]
fn r2_check9_offcentre_collinear_overlap() {
    let tol = Tol::witness();
    // A ring edge from x=0.0..0.9 on y=0 overlapping an outer edge
    // trimmed to x=0.7..1.0: the ring's midpoint x=0.45 is OUTSIDE the
    // outer edge's trim, but the two share a positive-length arc.
    // Built by hand is impossible through the profile door (the loops
    // touch), so this row only records that the shape is unreachable
    // here and names the code path.
    let outer = poly(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
    let hole = poly(&[(0.05, 1e-12), (0.9, 1e-12), (0.9, 0.5), (0.05, 0.5)]);
    match extruded(vec![outer, hole], 0.4) {
        Some(body) => {
            let v = topo::validate_geometric(&body, tol);
            let saw = matches!(&v, Err(es) if es.iter().any(|e| matches!(e, topo::ValidationError::RingMeetsOuter { .. })));
            println!(
                "  offcentre collinear: check9_fired={saw} tier3_ok={} meshes={}",
                v.is_ok(),
                mesh::tessellate(&body, 1e-3, tol).is_ok()
            );
        }
        None => println!("  offcentre collinear: the PROFILE gate refused it"),
    }
}

// =====================================================================
// Claim 4 — the box control. A one-face unslit chart canonicalises to
// itself: dumped in canonical form so the R2 lane can diff it against
// the merge-base build.
// =====================================================================

/// Prints a canonical structural + metric fingerprint of the opened box
/// and of every other #1048-corpus shape that opens. Run at head and at
/// the merge base and diff the two outputs.
#[test]
fn r2_box_control_fingerprint() {
    let tol = Tol::witness();
    let cases: Vec<(&str, Body<f64>, Vec<FaceKey>)> = {
        let mut v = Vec::new();
        let bx = extruded(
            vec![poly(&[(0.0, 0.0), (1.0, 0.0), (1.0, 0.7), (0.0, 0.7)])],
            0.5,
        )
        .expect("box");
        let top = plane_chart_at_z(&bx, 0.5);
        v.push(("box top", bx, top));
        let bx2 = extruded(
            vec![poly(&[
                (0.0, 0.0),
                (1.0, 0.0),
                (1.0, 0.4),
                (0.4, 0.4),
                (0.4, 0.9),
                (0.0, 0.9),
            ])],
            0.5,
        )
        .expect("L");
        let top2 = plane_chart_at_z(&bx2, 0.5);
        v.push(("L top", bx2, top2));
        v
    };
    for (name, body, chart) in cases {
        let cup = topo::shell_open(&body, 0.05, &chart, FIT_TOL, band(), tol);
        match cup {
            Ok(cup) => {
                let props = topo::mass_properties(&cup, tol).expect("props");
                println!(
                    "FINGERPRINT {name}: v={} e={} f={} l={} r={} s={} genus={} V={:.17e} A={:.17e} tier3={}",
                    cup.vertices().count(),
                    cup.edges().count(),
                    cup.faces().count(),
                    cup.loops().count(),
                    rings_of(&cup),
                    cup.shells().count(),
                    genus_of(&cup),
                    props.volume,
                    props.area,
                    topo::validate_geometric(&cup, tol).is_ok()
                );
            }
            Err(e) => println!("FINGERPRINT {name}: REFUSED {e}"),
        }
    }
}
