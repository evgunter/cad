//! SHELL-2 review probes (lane shell-2-r2, PR #1758 frozen at
//! `b58274d8`). Each row is named for the claim it attacks; the
//! prose says what it found. Not acceptance rows — a reviewer's
//! measurements, kept so the numbers are reproducible.

use std::sync::Arc;

use geom::{Curve3, NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use topo::{Body, EdgeCurveSpec, EdgeDescriptionSpec, FaceKey, FaceSurface, Pcurve};

use crate::common;
use common::approx::{band, box_with_approx_cap, planar_patch, pulled_back, top_face, unit_box};

fn rigid() -> Affine3<f64> {
    let mut map = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(0.3, -0.4, 0.8).normalize(),
        1.1,
    );
    map.translation = map.translation + Vec3::new(0.3, -0.2, 1.1);
    map
}

fn approx_of(body: &Body<f64>, face: FaceKey) -> Arc<geom::ApproxSurface<f64>> {
    match body.get_surface(body.get_face(face).unwrap().surface) {
        Some(Surface::Approx(a)) => Arc::clone(a),
        other => panic!("{face:?} must wear an approximating surface, got {other:?}"),
    }
}

/// Re-plant `face` with a surface built from the given parts and a
/// certifier that just hands back `cert` — the planted-claim shape.
fn plant(
    body: &mut Body<f64>,
    face: FaceKey,
    description: geom::SurfaceDescription<f64>,
    fit: NurbsSurface<f64>,
    window: geom::ApproxWindow,
    tolerance: f64,
    cert: geom::OffsetCertificate,
) {
    let planted = geom::ApproxSurface::certify(
        geom::SurfaceSpec {
            description,
            fit,
            window,
            tolerance,
        },
        |_, _, _, _| Ok::<_, geom_brep::OffsetFitError>(cert),
    )
    .unwrap();
    body.set_face_surface(face, FaceSurface::New(Surface::Approx(Arc::new(planted))))
        .unwrap();
}

fn kv2() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap()
}

/// A gently bowed polynomial patch over `[0,1]²` (geom-brep's
/// `approx_surface.rs::bowed`, copied so the base has real curvature).
fn bowed() -> NurbsSurface<f64> {
    let mut control = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            let (u, v) = (f64::from(i) * 0.5, f64::from(j) * 0.5);
            control.push(Point3::new(u, v, 0.15 * u * (1.0 - u) + 0.1 * v * v));
        }
    }
    NurbsSurface::new(kv2(), kv2(), control, vec![1.0; 9]).unwrap()
}

/// A rational quarter cylinder (weights `1, √½, 1` along u).
fn quarter_cylinder() -> NurbsSurface<f64> {
    let kv1 = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = vec![
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 1.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 1.0, 1.0),
    ];
    let w = core::f64::consts::FRAC_1_SQRT_2;
    NurbsSurface::new(kv2(), kv1, control, vec![1.0, 1.0, w, w, 1.0, 1.0]).unwrap()
}

// ---------------------------------------------------------------------
// C1 — the certificate is re-derived, never carried
// ---------------------------------------------------------------------

/// A bogus certificate (every limb zero, the wrong distance, `rounds`
/// = 7) planted behind a GOOD pair: the map must ship the re-derived
/// numbers, not the planted ones — and, per the PR, carry `rounds`.
#[test]
fn r2_c1_a_planted_bogus_certificate_is_replaced_by_the_re_derivation() {
    let d = 0.05;
    let (mut body, face) = box_with_approx_cap(d, 1e-9);
    let good = approx_of(&body, face);
    let bogus = geom::OffsetCertificate {
        distance: 42.0,
        cells: 999,
        samples: 1,
        on_locus_max: 0.0,
        hull_sup: 0.0,
        normal_floor: 0.0,
        curvature_reach: 0.0,
        rounds: 7,
    };
    plant(
        &mut body,
        face,
        good.description().clone(),
        good.fit().clone(),
        good.window(),
        good.tolerance(),
        bogus,
    );
    let moved = topo::transform_rigid(&body, &rigid(), Tol::witness()).expect("a good pair moves");
    let after = approx_of(&moved, face);
    let c = after.certificate();
    let geom::SurfaceDescription::Offset { base, .. } = after.description();
    let fresh = geom_brep::certify_offset(base, after.fit(), d, 1e-9, band()).unwrap();
    assert_eq!(c.distance, d, "distance is re-derived, not the planted 42");
    assert_eq!(c.cells, fresh.cells, "cells are the re-derivation's");
    assert_eq!(c.samples, fresh.samples);
    assert_eq!(c.normal_floor, fresh.normal_floor);
    assert_eq!(c.curvature_reach, fresh.curvature_reach);
    assert_eq!(c.hull_sup, fresh.hull_sup, "hull_sup is the re-derivation's, bit for bit");
    assert_eq!(c.on_locus_max, fresh.on_locus_max);
    assert_eq!(c.rounds, 7, "rounds is CARRIED across the map (deviation 1) — a planted count survives");
    eprintln!("[r2 c1] mapped cert: {c:?}\n[r2 c1] fresh cert:  {fresh:?}");
}

/// A window narrower than the base's rectangle planted behind an honest
/// certificate: the f64 arm refuses `WindowUnsupported` under
/// `ApproxRecertify`. (No PR row covers this refusal.)
#[test]
fn r2_c1_a_narrowed_window_refuses_across_the_map() {
    let d = 0.05;
    let (mut body, face) = box_with_approx_cap(d, 1e-9);
    let good = approx_of(&body, face);
    let mut window = good.window();
    window.u = (window.u.0, 0.5 * (window.u.0 + window.u.1));
    plant(
        &mut body,
        face,
        good.description().clone(),
        good.fit().clone(),
        window,
        good.tolerance(),
        *good.certificate(),
    );
    let e = topo::transform_rigid(&body, &rigid(), Tol::witness())
        .expect_err("a narrowed window is a bound the derivation never proved");
    assert!(
        matches!(
            e,
            topo::TransformError::ApproxRecertify {
                source: geom_brep::OffsetFitError::WindowUnsupported { .. }
            }
        ),
        "expected ApproxRecertify(WindowUnsupported), got {e}"
    );
    eprintln!("[r2 c1 window] {e}");
}

/// The BASE net edited (the description lies, the fit is honest, the
/// certificate is the old one): the map refuses on a limb.
#[test]
fn r2_c1_an_edited_base_net_refuses_across_the_map() {
    let d = 0.05;
    let (mut body, face) = box_with_approx_cap(d, 1e-9);
    let good = approx_of(&body, face);
    let geom::SurfaceDescription::Offset { base, .. } = good.description();
    let mut control = base.control().to_vec();
    control[3] = control[3] + Vec3::new(0.0, 0.0, 1e-3);
    let edited = NurbsSurface::new(
        base.knots_u().clone(),
        base.knots_v().clone(),
        control,
        base.weights().to_vec(),
    )
    .unwrap();
    plant(
        &mut body,
        face,
        geom::SurfaceDescription::Offset {
            base: Arc::new(edited),
            d,
        },
        good.fit().clone(),
        good.window(),
        good.tolerance(),
        *good.certificate(),
    );
    let e = topo::transform_rigid(&body, &rigid(), Tol::witness())
        .expect_err("an edited base must not move behind the old certificate");
    assert!(
        matches!(
            e,
            topo::TransformError::ApproxRecertify {
                source: geom_brep::OffsetFitError::Limb { .. }
            }
        ),
        "expected ApproxRecertify(Limb), got {e}"
    );
    eprintln!("[r2 c1 base] {e}");
}

/// A curved base fitted at 1e-6, re-planted claiming 1e-12 with the
/// 1e-6 certificate: the map classifies against the STORED tolerance
/// and refuses. (The one shape of stale claim where the fit and base
/// are both honest and only the tolerance lies.)
#[test]
fn r2_c1_a_tolerance_the_fit_does_not_honour_refuses_across_the_map() {
    let d = 0.02;
    let honest = geom_brep::approx_offset_surface(Arc::new(bowed()), d, 1e-6, band())
        .expect("the bowed base's offset fits at 1e-6");
    let Surface::Approx(good) = &honest else {
        panic!("the door mints the variant")
    };
    eprintln!("[r2 c1 tol] honest cert at 1e-6: {:?}", good.certificate());
    let mut body = unit_box();
    let face = top_face(&body);
    plant(
        &mut body,
        face,
        good.description().clone(),
        good.fit().clone(),
        good.window(),
        1e-12,
        *good.certificate(),
    );
    let e = topo::transform_rigid(&body, &rigid(), Tol::witness())
        .expect_err("a tolerance the fit does not honour must not move");
    assert!(
        matches!(
            e,
            topo::TransformError::ApproxRecertify {
                source: geom_brep::OffsetFitError::Limb { .. }
            }
        ),
        "expected ApproxRecertify(Limb), got {e}"
    );
    eprintln!("[r2 c1 tol] {e}");
}

// ---------------------------------------------------------------------
// C2 — map_affine is the map of the surface, rational included
// ---------------------------------------------------------------------

fn lcg(seed: &mut u64) -> f64 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    f64::from((*seed >> 33) as u32) / f64::from(u32::MAX)
}

#[test]
fn r2_c2_map_affine_of_a_rational_net_is_the_map_of_the_surface() {
    let s = quarter_cylinder();
    let map = rigid();
    let mapped = s.map_affine(&map);
    assert_eq!(mapped.weights(), s.weights());
    let mut seed = 0x5eed_u64;
    let (mut worst_p, mut worst_n) = (0.0_f64, 0.0_f64);
    for _ in 0..50 {
        let (u, v) = (lcg(&mut seed), lcg(&mut seed));
        let p = map.transform_point(s.eval(u, v));
        let q = mapped.eval(u, v);
        worst_p = worst_p.max(p.distance(q));
        let n0 = map.linear * Surface::Nurbs(Arc::new(s.clone())).normal(u, v);
        let n1 = Surface::Nurbs(Arc::new(mapped.clone())).normal(u, v);
        worst_n = worst_n.max((n0 - n1).norm());
    }
    eprintln!("[r2 c2] rigid: worst point gap {worst_p:e}, worst normal gap {worst_n:e}");
    assert!(worst_p <= 1e-14, "point gap {worst_p}");
    assert!(worst_n <= 1e-13, "normal gap {worst_n}");

    // The docs claim the identity for EVERY affine map: a shear.
    let shear = Affine3::from_parts(
        geom_core::Mat3::from_cols(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.7, 1.0, 0.0),
            Vec3::new(0.2, -0.3, 2.0),
        ),
        Vec3::new(1.0, 2.0, 3.0),
    );
    let sheared = s.map_affine(&shear);
    let mut worst = 0.0_f64;
    for _ in 0..50 {
        let (u, v) = (lcg(&mut seed), lcg(&mut seed));
        worst = worst.max(shear.transform_point(s.eval(u, v)).distance(sheared.eval(u, v)));
    }
    eprintln!("[r2 c2] shear: worst point gap {worst:e}");
    assert!(worst <= 1e-14, "shear point gap {worst}");
}

// ---------------------------------------------------------------------
// C3 — the f64 arm accepts everything the storage door mints
// ---------------------------------------------------------------------

#[test]
fn r2_c3_every_storage_door_surface_remaps_at_f64() {
    use geom_brep::PcurveFittedLane;
    let refined = bowed().refine_knots_u(&[0.5]).unwrap().refine_knots_v(&[0.25, 0.75]).unwrap();
    let cases: Vec<(&str, NurbsSurface<f64>, f64, f64)> = vec![
        ("planar +d", pulled_back(&planar_patch(1.0), 0.05), 0.05, 1e-9),
        ("planar -d", pulled_back(&planar_patch(1.0), -0.05), -0.05, 1e-9),
        ("bowed +d", bowed(), 0.02, 1e-6),
        ("bowed -d", bowed(), -0.02, 1e-6),
        ("bowed refined +d", refined.clone(), 0.02, 1e-6),
        ("bowed refined -d", refined, -0.02, 1e-6),
        ("rational quarter cylinder", quarter_cylinder(), 0.05, 1e-6),
    ];
    let map = rigid();
    for (name, base, d, tolerance) in cases {
        let minted = match geom_brep::approx_offset_surface(Arc::new(base), d, tolerance, band()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[r2 c3] {name}: the storage door itself refuses: {e}");
                continue;
            }
        };
        let Surface::Approx(a) = &minted else {
            panic!("the door mints the variant")
        };
        let geom::SurfaceDescription::Offset { base, d: dd } = a.description();
        let mapped_desc = geom::SurfaceDescription::Offset {
            base: Arc::new(base.map_affine(&map)),
            d: *dd,
        };
        let mapped_fit = a.fit().map_affine(&map);
        let re = f64::remap_certificate(&mapped_desc, &mapped_fit, a.window(), a.tolerance(), band());
        match re {
            Some(Ok(c)) => {
                let gap = (c.hull_sup - a.certificate().hull_sup).abs();
                eprintln!(
                    "[r2 c3] {name}: remaps; hull_sup {} vs stored {} (gap {gap:e}), rounds {} vs stored {}",
                    c.hull_sup, a.certificate().hull_sup, c.rounds, a.certificate().rounds
                );
                assert!(gap <= 1e-9, "{name}: hull_sup gap {gap}");
            }
            Some(Err(e)) => panic!("{name}: the f64 arm refuses a storage-door surface: {e}"),
            None => panic!("{name}: the f64 arm has no lane"),
        }
        // And through the body door.
        let mut body = unit_box();
        let face = top_face(&body);
        body.set_face_surface(face, FaceSurface::New(minted)).unwrap();
        topo::transform_rigid(&body, &map, Tol::witness())
            .unwrap_or_else(|e| panic!("{name}: the body does not move: {e}"));
    }
}

// ---------------------------------------------------------------------
// C4 — refusal order
// ---------------------------------------------------------------------

#[test]
fn r2_c4_non_finite_map_components_refuse_before_the_approx_arm() {
    let (body, _) = box_with_approx_cap(0.05, 1e-9);
    let mut nan_t = rigid();
    nan_t.translation.x = f64::NAN;
    let e = topo::transform_rigid(&body, &nan_t, Tol::witness()).expect_err("NaN translation");
    assert!(matches!(e, topo::TransformError::NonFiniteMap { .. }), "got {e}");
    let mut nan_l = rigid();
    nan_l.linear.c0.x = f64::NAN;
    let e = topo::transform_rigid(&body, &nan_l, Tol::witness()).expect_err("NaN linear");
    eprintln!("[r2 c4] NaN in the linear part refuses as: {e}");
    assert!(
        matches!(e, topo::TransformError::NotRigid { .. }),
        "a NaN linear entry refuses NotRigid (the module docs say so), got {e}"
    );
}

/// The Dual lane on a body a user could actually hold at `Dual64`.
#[test]
fn r2_c4_the_dual_lane_refuses_naming_itself() {
    use geom_core::{Dual, Dual64};
    use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
    let honest = geom_brep::approx_offset_surface(
        Arc::new(pulled_back(&planar_patch(1.0), 0.05)),
        0.05,
        1e-9,
        band(),
    )
    .unwrap();
    let Surface::Approx(a) = &honest else {
        panic!()
    };
    let lifted = a.map_scalar(Dual::constant);
    let c = Dual::constant;
    let v = |x: f64, y: f64| ProfileVertex::new(Point2::new(c(x), c(y)), c(0.0));
    let lp = ProfileLoop::new(vec![v(0.0, 0.0), v(2.0, 0.0), v(2.0, 2.0), v(0.0, 2.0)]);
    let profile = Profile::new(SketchPlane::<Dual64>::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let mut body = sweep::extrude(&profile, sweep::Extrusion::Distance(c(1.0)), Tol::witness())
        .unwrap()
        .body;
    let face = body
        .faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. })
                    if normal.z.value() > 0.5 && origin.z.value() > 0.5
            )
        })
        .map(|(k, _)| k)
        .unwrap();
    body.set_face_surface(face, FaceSurface::New(Surface::Approx(Arc::new(lifted))))
        .unwrap();
    let e = topo::transform_rigid(
        &body,
        &Affine3::translation(Vec3::new(c(1.0), c(0.0), c(0.0))),
        Tol::witness(),
    )
    .expect_err("the dual lane has no fit derivation");
    assert!(
        matches!(e, topo::TransformError::ApproxLaneUnsupported { lane: "dual" }),
        "got {e}"
    );
    eprintln!("[r2 c4 dual] {e}");
}

// ---------------------------------------------------------------------
// C6 — a fourth route: Chart-described cap edges, no caches
// ---------------------------------------------------------------------

/// The box cap re-described as Chart images on its OWN Approx chart
/// with its straight carriers kept. Tier 3 says nothing about a body
/// with no stored caches, so if the attach gate takes the description
/// the body is tier-3 clean AND movable — the premise the filed issue
/// says no body satisfies.
fn chart_described_cap(d: f64) -> (Body<f64>, FaceKey) {
    let (mut body, face) = box_with_approx_cap(d, 1e-9);
    let key = body.get_face(face).unwrap().surface;
    let fit = approx_of(&body, face).fit().clone();
    let outer = body.get_face(face).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("cycle")
    };
    let hes = body.loop_cycle(first).unwrap();
    for he in hes {
        let edge = body.half_edges().find(|(k, _)| *k == he).unwrap().1.edge;
        let curve_key = body.get_edge(edge).unwrap().curve;
        let Some(topo::CurveGeom::Certified(ec)) = body.get_curve_geom(curve_key) else {
            panic!("certified")
        };
        let carrier = ec.carrier().clone();
        let (t0, t1) = ec.params();
        let Curve3::Line { origin, dir } = carrier else {
            panic!("a box edge is a line")
        };
        // The fit's chart: S(u, v) = (2u, 2v, 1) — checked, not assumed.
        let img = |t: f64| Point2::new((origin.x + dir.x * t) / 2.0, (origin.y + dir.y * t) / 2.0);
        for t in [t0, 0.5 * (t0 + t1), t1] {
            let uv = img(t);
            let gap = fit.eval(uv.x, uv.y).distance(carrier.eval(t));
            assert!(gap <= 1e-12, "chart map gap {gap} at t = {t}");
        }
        let p0 = img(0.0);
        let pl = Vec2::new(dir.x / 2.0, dir.y / 2.0);
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::chart_image(key, Pcurve::IsoLine { p0, pl }),
            carrier,
            param_start: t0,
            param_end: t1,
        };
        body.set_edge_curve(edge, spec, Tol::witness())
            .unwrap_or_else(|e| panic!("the attach gate refuses a Chart image on the Approx chart for edge {edge:?}: {e}"));
    }
    (body, face)
}

#[test]
fn r2_c6_a_chart_described_approx_cap_is_movable_and_tier_three_clean() {
    for d in [0.05_f64, -0.05] {
        let (body, face) = chart_described_cap(d);
        let before = topo::validate_geometric(&body, Tol::witness());
        eprintln!("[r2 c6] d = {d}: tier 3 before the map: {before:?}");
        let moved = topo::transform_rigid(&body, &rigid(), Tol::witness())
            .unwrap_or_else(|e| panic!("d = {d}: the chart-described body does not move: {e}"));
        let after = topo::validate_geometric(&moved, Tol::witness());
        eprintln!("[r2 c6] d = {d}: tier 3 after the map: {after:?}");
        assert_eq!(before, Ok(()), "d = {d}: the chart-described cap is tier-3 clean");
        assert_eq!(after, Ok(()), "d = {d}: and so is its rigid image");
        assert!(matches!(
            moved.get_surface(moved.get_face(face).unwrap().surface),
            Some(Surface::Approx(_))
        ));
        // What still does not work on it, for the record.
        let props = topo::mass_properties(&moved, Tol::witness()).map(|m| (m.volume, m.volume_pad));
        eprintln!("[r2 c6] d = {d}: mass_properties on the moved body: {props:?}");
        let mut minted = moved.clone();
        let mint = topo::mint_pcurves(&mut minted, Tol::witness());
        eprintln!("[r2 c6] d = {d}: mint_pcurves on the moved body: {mint:?}");
        let mut m0 = body.clone();
        let mint0 = topo::mint_pcurves(&mut m0, Tol::witness());
        eprintln!("[r2 c6] d = {d}: mint_pcurves on the operand: {mint0:?}");
    }
}

/// The PR's §3.6 measurement says the fixture's cap refuses with the
/// SEAM-CLASS text. Read the actual refusal off the fixture.
#[test]
fn r2_c6_what_the_pcurve_pass_actually_says_on_the_fixture() {
    let (mut body, _) = box_with_approx_cap(0.05, 1e-9);
    let e = topo::mint_pcurves(&mut body, Tol::witness());
    eprintln!("[r2 c6 fixture] mint_pcurves: {e:?}");
    let e = e.expect_err("the fixture's cap has no cache route");
    let text = format!("{e}");
    eprintln!("[r2 c6 fixture] text: {text}");
    assert!(text.contains("Intersection carrier that is not a spline") || text.contains("seam-class"));
}

// ---------------------------------------------------------------------
// E2E — a user places an Approx-capped part
// ---------------------------------------------------------------------

#[test]
fn r2_e2e_a_user_places_an_approx_capped_part() {
    // 1. Mint through the storage door, the way a user would.
    let d = 0.05;
    let approx = geom_brep::approx_offset_surface(
        Arc::new(pulled_back(&planar_patch(1.0), d)),
        d,
        1e-9,
        band(),
    )
    .expect("the storage door mints");
    let mut part = unit_box();
    let cap = top_face(&part);
    part.set_face_surface(cap, FaceSurface::New(approx)).unwrap();

    // 2. Place it as the tour places parts: a rotation and a translation.
    let mut place = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::unit_z(),
        core::f64::consts::FRAC_PI_3,
    );
    place.translation = place.translation + Vec3::new(5.0, -2.0, 1.0);
    let placed = topo::transform_rigid(&part, &place, Tol::witness())
        .expect("an Approx-capped box places");
    let a = approx_of(&placed, cap);
    eprintln!("[r2 e2e] placed cap cert: {:?}", a.certificate());

    // 3. Measure it.
    let tier3 = topo::validate_geometric(&placed, Tol::witness());
    eprintln!("[r2 e2e] tier 3: {tier3:?}");
    let props = topo::mass_properties(&placed, Tol::witness());
    match &props {
        Ok(m) => eprintln!("[r2 e2e] mass properties: volume {} ± {}", m.volume, m.volume_pad),
        Err(e) => eprintln!("[r2 e2e] mass properties REFUSE: {e}"),
    }
    let mesh = mesh::tessellate(&placed, 0.05, Tol::witness());
    match &mesh {
        Ok(m) => eprintln!(
            "[r2 e2e] tessellates: {} patches, cap has {} triangles",
            m.patches.len(),
            m.patches.iter().find(|p| p.face == cap).map_or(0, |p| p.triangles.len())
        ),
        Err(e) => eprintln!("[r2 e2e] tessellation REFUSES: {e}"),
    }

    // 4. Export it.
    let step = step_export::step_string(&placed, &step_export::StepOptions::default(), Tol::witness());
    match &step {
        Ok(doc) => eprintln!("[r2 e2e] STEP: {} bytes", doc.len()),
        Err(e) => eprintln!("[r2 e2e] STEP REFUSES: {e}"),
    }
    assert!(
        matches!(step, Err(step_export::StepExportError::UnsupportedSurface { kind: "approximating surface", .. })),
        "the STEP writer refuses the kind"
    );
    let _ = mesh;
}
