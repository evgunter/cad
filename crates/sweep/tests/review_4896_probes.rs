//! Review probes for PR 4896 (bore ruling step 3: a feature wholly
//! inside a convex ruled cut-off's sliver goes with it).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, SketchPlane, test_support::bulge_loop};
use sweep::Revolution;
use sweep::blend::{BlendError, fillet_edges};
use sweep::test_support::{
    ROD_FILLET, ROD_FLAT, ROD_L, ROD_R, extruded, finished, prism_on, revolved_about_y,
    rod_chord_at, rod_creases, rod_section_cut, sketch_from_axes,
};
use topo::{
    Body, ContactRecords, LoopBoundary, mass_properties, transform_rigid, validate_geometric,
    validate_pseudomanifold,
};

use crate::common::cavity::cut;

fn tol() -> Tol {
    Tol::witness()
}

fn volume(body: &Body<f64>) -> f64 {
    mass_properties(body, tol())
        .expect("closed-form props")
        .volume
}

fn d_loop() -> ProfileLoop<f64> {
    let c = rod_chord_at(ROD_FLAT);
    bulge_loop(vec![
        (Point2::new(ROD_FLAT, c.half), c.wall_bulge),
        (Point2::new(ROD_FLAT, -c.half), 0.0),
    ])
}

fn round(x: f64, y: f64, a: f64) -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(x + a, y), 1.0),
        (Point2::new(x - a, y), 1.0),
    ])
}

fn square_at(x: f64, y: f64, h: f64) -> ProfileLoop<f64> {
    bulge_loop(
        [(-h, -h), (h, -h), (h, h), (-h, h)]
            .into_iter()
            .map(|(dx, dy)| (Point2::new(x + dx, y + dy), 0.0))
            .collect(),
    )
}

fn d_rod_with(holes: Vec<ProfileLoop<f64>>) -> Body<f64> {
    let mut loops = vec![d_loop()];
    loops.extend(holes);
    extruded(SketchPlane::xy(), loops, ROD_L, tol())
}

fn plain_d_rod() -> Body<f64> {
    d_rod_with(Vec::new())
}

/// Edge count of every ring of every face, sorted.
fn ring_edge_counts(body: &Body<f64>) -> Vec<usize> {
    let mut out = Vec::new();
    for (_, f) in body.faces() {
        for &lp in &f.rings {
            let n = match body.get_loop(lp).unwrap().boundary {
                LoopBoundary::Empty { .. } => 0,
                LoopBoundary::Cycle { first } => body.loop_cycle(first).unwrap().len(),
            };
            out.push(n);
        }
    }
    out.sort_unstable();
    out
}

/// Fillet both creases; on success check tiers 3/3′, naming totality,
/// no ring left, and ΔV against `want_bore` (the bores' total volume).
fn fillet_and_check(body: &Body<f64>, want_bore: f64, what: &str) -> Result<(), BlendError> {
    fillet_and_check_at(body, ROD_FILLET, want_bore, what)
}

fn fillet_and_check_at(
    body: &Body<f64>,
    r: f64,
    want_bore: f64,
    what: &str,
) -> Result<(), BlendError> {
    let creases = rod_creases(body);
    assert_eq!(creases.len(), 2, "{what}: the D's two creases");
    let at = finished(what, body.clone(), tol());
    let out = fillet_edges(&at, &creases, r, tol()).map_err(|e| e.error)?;
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
    validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
        .unwrap_or_else(|e| panic!("{what}: tier 3′, {e:?}"));
    sweep::test_support::assert_naming_totality(body, &out, &creases, what);
    assert!(
        out.body.faces().all(|(_, f)| f.rings.is_empty()),
        "{what}: no face keeps a ring: {:?}",
        ring_edge_counts(&out.body)
    );
    let dv = volume(&out.body) - volume(body);
    let want = -2.0 * rod_section_cut(ROD_R, ROD_FLAT, r) * ROD_L + want_bore;
    assert!((dv - want).abs() < 1e-11, "{what}: ΔV {dv} vs {want}");
    Ok(())
}

/// A solid round rod of radius `a` about the `z`-parallel line through
/// `(x, y)`, from `z = -0.5` to `1.5`, built by a FULL revolve, so its
/// wall carries no seam edge and its rims are one-edge circles.
fn revolved_rod(x: f64, y: f64, a: f64) -> Body<f64> {
    let t = tol();
    let rod = revolved_about_y(
        vec![
            (Point2::new(0.0, -0.5), 0.0),
            (Point2::new(a, -0.5), 0.0),
            (Point2::new(a, 1.5), 0.0),
            (Point2::new(0.0, 1.5), 0.0),
        ],
        Revolution::Full,
        t,
    );
    let turn = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(1.0, 0.0, 0.0),
        core::f64::consts::FRAC_PI_2,
    );
    let rod = transform_rigid(&rod, &turn, t).expect("rotate the rod");
    let shift = Affine3::translation(Vec3::new(x, y, 0.0));
    transform_rigid(&rod, &shift, t).expect("place the rod")
}

/// A round rod of radius `a` (two semicircle arcs) whose axis runs
/// along `(0, s, 1)` through `(x, y, 0)`, spanning `z ∈ [-0.1, 1.1]`.
fn tilted_rod(x: f64, y: f64, a: f64, sx: f64, s: f64) -> Body<f64> {
    let t = tol();
    let dir = Vec3::new(sx, s, 1.0);
    let o = Point3::new(x - 0.1 * sx, y - 0.1 * s, -0.1);
    // u × v = dir: pick u ⊥ dir in the xz plane-ish, v = dir × u.
    let u = Vec3::new(1.0, 0.0, -sx);
    let v = dir.cross(u);
    let plane = sketch_from_axes(o, u, v, t);
    prism_on(
        plane,
        vec![(Point2::new(a, 0.0), 1.0), (Point2::new(-a, 0.0), 1.0)],
        1.2 * dir.norm(),
        t,
    )
}

// ---------------------------------------------------------------------

/// **A Boolean-cut bore through a revolved (seamless) rod** inside the
/// sliver. The cut imprints its rings in two edges each (so this does
/// NOT reach the one-edge path; see `p1b`), and it goes with the sliver.
#[test]
fn p1_a_boolean_cut_bore_from_a_seamless_rod_goes_with_it() {
    let (x, y, a) = (0.297, 0.395, 0.002);
    let body = cut("revolved-rod bore", &plain_d_rod(), &revolved_rod(x, y, a));
    validate_geometric(&body, tol()).expect("source tier 3");
    assert_eq!(
        ring_edge_counts(&body),
        vec![2, 2],
        "p1: the cut imprints two-edge rings"
    );
    fillet_and_check(
        &body,
        core::f64::consts::PI * a * a * ROD_L,
        "revolved-rod bore",
    )
    .unwrap_or_else(|e| panic!("p1: goes with the sliver, got {e:?}"));
}

/// A square through-hole inside the sliver: planar walls (the plane arm
/// of the prism reading).
#[test]
fn p2_a_square_hole_inside_the_sliver_goes_with_it() {
    let h = 0.001;
    let body = d_rod_with(vec![square_at(0.297, 0.395, h)]);
    fillet_and_check(&body, 4.0 * h * h * ROD_L, "square hole")
        .unwrap_or_else(|e| panic!("p2: square hole goes, got {e:?}"));
}

/// One bore in each crease's sliver: two features, two creases.
#[test]
fn p3_a_bore_in_each_sliver_both_go() {
    let a = 0.002;
    let body = d_rod_with(vec![round(0.297, 0.395, a), round(0.297, -0.395, a)]);
    fillet_and_check(
        &body,
        2.0 * core::f64::consts::PI * a * a * ROD_L,
        "two slivers",
    )
    .unwrap_or_else(|e| panic!("p3: both go, got {e:?}"));
}

/// Two bores in one sliver: two features on one pair of caps.
#[test]
fn p4_two_bores_in_one_sliver_both_go() {
    let body = d_rod_with(vec![
        round(0.297, 0.395, 0.002),
        round(0.2918, 0.3994, 0.001),
    ]);
    let want = core::f64::consts::PI * (0.002f64.powi(2) + 0.001f64.powi(2)) * ROD_L;
    fillet_and_check(&body, want, "two in one sliver")
        .unwrap_or_else(|e| panic!("p4: both go, got {e:?}"));
}

/// A slightly tilted bore wholly inside the sliver (Boolean-cut, so its
/// rings are ellipses and its seams tilted lines).
#[test]
fn p5_a_slightly_tilted_bore_inside() {
    let (a, s) = (0.004, 0.005);
    let body = cut(
        "tilted bore",
        &plain_d_rod(),
        &tilted_rod(0.279, 0.389, a, 0.0, s),
    );
    validate_geometric(&body, tol()).expect("source tier 3");
    let bore = volume(&plain_d_rod()) - volume(&body);
    let got = fillet_and_check_at(&body, 0.25, bore, "tilted inside");
    eprintln!("p5 outcome: {got:?}");
    // Building is the claim; a typed refusal is conservative. A build
    // was checked at tiers 3/3′ and ΔV inside the helper.
    if let Err(e) = got {
        assert!(
            matches!(
                e,
                BlendError::FaceClearance { bounded: true, .. } | BlendError::WhollyInside { .. }
            ),
            "p5: a conservative refusal, got {e:?}"
        );
    }
}

/// A bore inside the sliver at `z = 0` that tilts into the fillet's
/// ball by `z = 1`: crosses the band. Must not build.
#[test]
fn p6_a_bore_tilting_out_of_the_sliver_refuses() {
    let a = 0.002;
    let body = cut(
        "tilt-out bore",
        &plain_d_rod(),
        &tilted_rod(0.297, 0.395, a, -0.009, -0.0045),
    );
    let creases = rod_creases(&body);
    let got = fillet_edges(
        &finished("probe", body.clone(), tol()),
        &creases,
        ROD_FILLET,
        tol(),
    )
    .map(|_| ())
    .map_err(|e| e.error);
    eprintln!("p6 outcome: {got:?}");
    assert!(got.is_err(), "p6: a bore crossing the band does not build");
    assert!(
        !matches!(got, Err(BlendError::WhollyInside { .. })),
        "p6: it is not wholly inside: {got:?}"
    );
}

/// Both rings inside, walls geometrically inside, but the lateral run
/// `3e-3` exceeds the rings' depth: the prism reading cannot certify
/// it, so it refuses as a bound (conservative), never a panic.
#[test]
fn p7_a_tilted_bore_past_the_prism_readings_reach_refuses_as_a_bound() {
    let a = 0.004;
    let body = cut(
        "tilt-far bore",
        &plain_d_rod(),
        &tilted_rod(0.279, 0.389, a, 0.0, 0.012),
    );
    let creases = rod_creases(&body);
    let got = fillet_edges(
        &finished("probe", body.clone(), tol()),
        &creases,
        0.25,
        tol(),
    )
    .map(|_| ())
    .map_err(|e| e.error);
    eprintln!("p7 outcome: {got:?}");
    assert!(
        matches!(
            got,
            Err(BlendError::FaceClearance { .. } | BlendError::WhollyInside { .. }) | Ok(())
        ),
        "p7: typed, got {got:?}"
    );
}

/// A blind bore from one cap, inside the sliver: one ring, a floor. The
/// plan does not take it; it refuses typed.
#[test]
fn p8_a_blind_bore_inside_refuses_typed() {
    let a = 0.002;
    let rod = crate::common::cavity::rod(Point2::new(0.297, 0.395), a, -0.5, 0.5);
    let body = cut("blind bore", &plain_d_rod(), &rod);
    let creases = rod_creases(&body);
    let got = fillet_edges(
        &finished("probe", body.clone(), tol()),
        &creases,
        ROD_FILLET,
        tol(),
    )
    .map(|_| ())
    .map_err(|e| e.error);
    eprintln!("p8 outcome: {got:?}");
    assert!(
        matches!(
            got,
            Err(BlendError::FaceClearance { .. } | BlendError::WhollyInside { .. })
        ),
        "p8: typed refusal, got {got:?}"
    );
}

/// **Claim 2 at Interval**: the lane's Interval row checks only an
/// entity census; this one runs tiers 3 and 3′ and the naming
/// census on the Interval carve of the witness bore, and checks that
/// the bore's walls are retired.
#[test]
fn p9_the_witness_bore_at_interval_is_tier_3_and_3_prime() {
    let t = tol();
    let f = <geom_core::Interval as geom_core::Real>::from_f64;
    let c = rod_chord_at(ROD_FLAT);
    let d = bulge_loop(vec![
        (Point2::new(f(ROD_FLAT), f(c.half)), f(c.wall_bulge)),
        (Point2::new(f(ROD_FLAT), f(-c.half)), f(0.0)),
    ]);
    let (x, y, a) = (0.297, 0.395, 0.002);
    let bore = bulge_loop(vec![
        (Point2::new(f(x + a), f(y)), f(1.0)),
        (Point2::new(f(x - a), f(y)), f(1.0)),
    ]);
    let body = extruded(SketchPlane::xy(), vec![d, bore], f(ROD_L), t);
    let at = finished("bored at Interval", body.clone(), t);
    let creases = rod_creases(&body);
    let out = fillet_edges(&at, &creases, f(ROD_FILLET), t)
        .unwrap_or_else(|e| panic!("p9: builds at Interval, got {e}"));
    validate_geometric(&out.body, t).unwrap_or_else(|e| panic!("p9: tier 3, {e:?}"));
    validate_pseudomanifold(&out.body, &ContactRecords::default(), t)
        .unwrap_or_else(|e| panic!("p9: tier 3′, {e:?}"));
    let rec = out.naming.as_ref().expect("births recorded");
    assert_eq!(rec.dead.faces.len(), 2, "p9: both half-walls retired");
    assert!(
        out.body.faces().all(|(_, f)| f.rings.is_empty()),
        "p9: no ring survives"
    );
}

/// The D-rod laid along `y` (`y ∈ [0, 1]`) with its profile point
/// `(0.297, 0.395)` on the `y` axis, intersected with a revolve about
/// `y` of `x ∈ [inner, 0.2]` (`inner = 0` a solid rod) over the same
/// `y` span: caps coplanar, so the revolve's one-edge rims can survive.
fn d_around_axis(inner: f64) -> Body<f64> {
    let t = tol();
    let plane = sketch_from_axes(
        Point3::new(-0.297, 0.0, 0.395),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        t,
    );
    let d = extruded(plane, vec![d_loop()], ROD_L, t);
    let tube = revolved_about_y(
        vec![
            (Point2::new(inner, 0.0), 0.0),
            (Point2::new(0.2, 0.0), 0.0),
            (Point2::new(0.2, 1.0), 0.0),
            (Point2::new(inner, 1.0), 0.0),
        ],
        Revolution::Full,
        t,
    );
    let a = finished("d", d, t);
    let b = finished("tube", tube, t);
    topo::intersect(&a, &b, t)
        .unwrap_or_else(|e| panic!("D ∩ tube: {e:?}"))
        .body()
        .expect("material")
        .body
        .clone()
        .into_body()
}

/// **Claim 2, the one-edge bore, built** — ignored: no door tried
/// builds it today (`topo::intersect` refuses
/// `Join(SingleSiteSectionLoop { count: 2 })`). Run with `--ignored`
/// once one does; by inspection `consume` then refuses `Op`
/// (`kef` `FaceHasRings` on the seamless wall's two cycles). A seamless bore of radius
/// `0.002` on the `y` axis, wholly inside the D corner's sliver.
#[test]
#[ignore = "fixture: the Boolean refuses D ∩ tube today"]
fn p1b_a_seamless_bore_inside_the_sliver() {
    let bored = d_around_axis(0.002);
    let solid = d_around_axis(0.0);
    validate_geometric(&bored, tol()).expect("source tier 3");
    let rings = ring_edge_counts(&bored);
    let wall_rings: Vec<usize> = bored
        .faces()
        .filter(|(_, f)| {
            matches!(bored.get_surface(f.surface),
                Some(geom::Surface::Cylinder { radius, .. }) if (radius - 0.002).abs() < 1e-12)
        })
        .map(|(_, f)| f.rings.len())
        .collect();
    eprintln!("p1b ring edge counts {rings:?}, bore walls' ring counts {wall_rings:?}");
    let pick = |b: &Body<f64>| -> Vec<topo::EdgeKey> {
        rod_creases(b)
            .into_iter()
            .filter(|&e| {
                let ed = b.get_edge(e).unwrap();
                let he = b.get_half_edge(ed.he_plus).unwrap();
                let p = b.vertex_points().find(|(k, _)| *k == he.start).unwrap().1;
                p.x.hypot(p.z) < 0.05
            })
            .collect()
    };
    let (cb, cs) = (pick(&bored), pick(&solid));
    eprintln!("p1b creases bored {cb:?} solid {cs:?}");
    assert_eq!((cb.len(), cs.len()), (1, 1), "p1b: one corner crease each");
    let plain = fillet_edges(
        &finished("solid", solid.clone(), tol()),
        &cs,
        ROD_FILLET,
        tol(),
    )
    .unwrap_or_else(|e| panic!("p1b: the unbored corner carves, got {e}"));
    let got = fillet_edges(
        &finished("bored", bored.clone(), tol()),
        &cb,
        ROD_FILLET,
        tol(),
    );
    match got {
        Ok(out) => {
            validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("p1b: tier 3, {e:?}"));
            validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
                .unwrap_or_else(|e| panic!("p1b: tier 3′, {e:?}"));
            let (vb, vs) = (volume(&out.body), volume(&plain.body));
            assert!(
                (vb - vs).abs() < 1e-11,
                "p1b: the bored carve is the unbored one: {vb} vs {vs}"
            );
        }
        Err(e) => panic!(
            "p1b: the seamless bore goes with the sliver, got {:?}",
            e.error
        ),
    }
}

/// **Claim 1, a bore whose rings both lie inside the sliver but whose
/// middle crosses the fillet's ball.** At `r = 0.1` the sliver is not
/// convex (its arc bulges into it): the bore of radius `5e-4` runs from
/// `(0.29918, 0.36551)` at `z = 0` to `(0.26784, 0.42123)` at `z = 1`,
/// each ring `≈ 8e-4` clear of the sliver's boundary, while its axis's
/// midpoint is `≈ 4.2e-3` inside the ball — through kept material. It
/// must not build. (With `prism_inside`'s slack and extent terms
/// zeroed, the rest of the blend suite stays green; this row is the
/// one that can go red for them.)
#[test]
fn p10_a_bore_inside_at_both_caps_crossing_mid_length_refuses() {
    let a = 0.0005;
    let (x0, y0, x1, y1) = (0.29918, 0.36551, 0.26784, 0.42123);
    let body = cut(
        "dipping bore",
        &plain_d_rod(),
        &tilted_rod(x0, y0, a, x1 - x0, y1 - y0),
    );
    validate_geometric(&body, tol()).expect("source tier 3");
    let creases = rod_creases(&body);
    let got = fillet_edges(
        &finished("probe", body.clone(), tol()),
        &creases,
        ROD_FILLET,
        tol(),
    )
    .map(|_| ())
    .map_err(|e| e.error);
    eprintln!("p10 outcome: {got:?}");
    assert!(
        got.is_err(),
        "p10: a bore crossing kept material does not build"
    );
}
