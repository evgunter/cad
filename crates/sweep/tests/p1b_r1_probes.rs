//! **PCURVE P-1b R1 review probes** (blinded reviewer R1, ordinal 201,
//! frozen head `0422043a`). Independent consumer rows attacking the
//! unit's claims from OUTSIDE its own test rewrites:
//!
//! 1. **Fence completeness** — every product verb this suite can reach
//!    is run and its body SCANNED for scaffold descriptions directly,
//!    cross-checked against tier 3's own verdict. A scan hit with a
//!    green tier 3 is a fence hole; a scan hit with a red tier 3 is a
//!    verb defect (the "eighth family" shape).
//! 2. **Declaration transport** — `EdgeAuthority::is_declared` must
//!    never flip silently across an offset or a rigid transform; where
//!    it cannot be carried it must refuse loudly.
//! 3. **The unmetered declaration** — a finding row: certification
//!    meters a SCAFFOLD's pushforward (`carrier_matches_mapped_source`)
//!    but never the `declared` record beside a chart image, so a
//!    corrupt declaration certifies clean and survives tier 3. The row
//!    documents the measured fact; if it ever goes red, the kernel
//!    grew a meter and the finding is closed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::Surface;
use geom_brep::{EdgeAuthority, EdgeCurveSpec, EdgeDescription, EdgeDescriptionSpec, MappedCurve};
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::fillet::build::fillet_edges;
use sweep::{
    Extrusion, Revolution, RevolveAxis, TubeWindow, extrude, loft_body, revolve, tube_along_arc,
    tube_along_arc_hollow,
};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations, CurveGeom, EdgeKey, ReplaceFaceError, ValidationError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn band() -> geom_core::Band {
    geom_core::Band::linear(Tol::witness()).unwrap()
}

/// Every edge at rest still described through the scaffolding door.
fn scaffold_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    body.edges()
        .filter(|(_, e)| {
            matches!(
                body.get_curve_geom(e.curve)
                    .and_then(CurveGeom::certified)
                    .map(topo::EdgeCurve::description),
                Some(EdgeDescription::Scaffold(_))
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// The body-wide authority census: every edge's `is_declared` bit, in
/// edge-arena order.
fn authority_census(body: &Body<f64>) -> Vec<(EdgeKey, bool)> {
    body.edges()
        .filter_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            Some((k, c.authority().is_declared()))
        })
        .collect()
}

/// The fence cross-check: the direct scan and tier 3's verdict must
/// agree, and both must be clean.
///
/// The four quadrants, spelled out because three of them are findings:
/// - scan clean, tier 3 green: the verb finished its own conversions.
/// - scan clean, tier 3 red: the body is broken for a NON-fence reason
///   (report verbatim).
/// - scan hit, tier 3 red naming the same edges: the verb is handing
///   back a body tier 3 refuses — the "eighth family" defect shape.
/// - scan hit, tier 3 GREEN: **a hole in the fence itself** — a
///   scaffold reached rest and validation cannot see it.
fn fence_crosscheck(body: &Body<f64>, ctx: &str) {
    let scaffolds = scaffold_edges(body);
    let verdict = topo::validate_geometric(body, Tol::witness());
    match (scaffolds.is_empty(), verdict) {
        (true, Ok(())) => {}
        (true, Err(errs)) => panic!("{ctx}: tier 3 refuses a scaffold-free body: {errs:?}"),
        (false, Err(errs)) => {
            let named: Vec<_> = errs
                .iter()
                .filter(|e| matches!(e, ValidationError::ScaffoldAtRest { .. }))
                .collect();
            panic!(
                "{ctx}: the verb handed back a body at rest with scaffold descriptions \
                 {scaffolds:?} (tier 3 names {named:?}) — an unconverted family"
            );
        }
        (false, Ok(())) => panic!(
            "{ctx}: FENCE HOLE — scaffold descriptions {scaffolds:?} survive at rest \
             and tier 3 passes the body anyway"
        ),
    }
}

fn validated(loops: Vec<ProfileLoop<f64>>) -> profile::ValidatedProfile<f64> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .expect("a valid probe profile")
}

fn extruded(loops: Vec<ProfileLoop<f64>>, h: f64) -> Body<f64> {
    extrude(
        &validated(loops),
        Extrusion::Distance(h),
        Tol::witness(),
    )
    .expect("the probe profile extrudes")
    .body
}

/// A two-vertex full circle (two semicircular arcs), counterclockwise.
fn circle_loop(cx: f64, cy: f64, r: f64) -> ProfileLoop<f64> {
    ProfileLoop::new(vec![
        ProfileVertex::new(p2(cx - r, cy), 1.0),
        ProfileVertex::new(p2(cx + r, cy), 1.0),
    ])
}

/// A rounded square: four lines and four quarter-circle corner arcs,
/// tangent-declared at every arc joint.
fn rounded_square(half: f64, r: f64) -> ProfileLoop<f64> {
    let b = (PI / 8.0).tan(); // quarter-turn bulge
    let v = |x, y, bulge| ProfileVertex::new(p2(x, y), bulge);
    ProfileLoop::new(vec![
        v(-half + r, -half, 0.0),
        v(half - r, -half, b),
        v(half, -half + r, 0.0),
        v(half, half - r, b),
        v(half - r, half, 0.0),
        v(-half + r, half, b),
        v(-half, half - r, 0.0),
        v(-half, -half + r, b),
    ])
    .with_tangent_joints(vec![1, 2, 3, 4, 5, 6, 7, 0])
}

fn revolved(points: &[(f64, f64, f64)], rev: Revolution<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(
        points
            .iter()
            .map(|(r, y, bulge)| ProfileVertex::new(p2(*r, *y), *bulge))
            .collect(),
    );
    revolve(
        &validated(vec![lp]),
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        rev,
        Tol::witness(),
    )
    .expect("the probe profile revolves")
    .body
}

// =====================================================================
// 1. Fence completeness: the verbs, scanned from outside.
// =====================================================================

#[test]
fn extrude_products_carry_no_scaffold_at_rest() {
    // The plain L (all-transverse corners).
    let l = ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(2.0, 0.0),
        p2(2.0, 1.0),
        p2(1.0, 1.0),
        p2(1.0, 2.0),
        p2(0.0, 2.0),
    ]);
    fence_crosscheck(&extruded(vec![l], 1.0), "extrude L");

    // Square with a circular hole: the hole's two half-walls share ONE
    // cylinder key, so its struts take the same-key under-determined
    // lane (declared images).
    let square = ProfileLoop::polygon([p2(-2.0, -2.0), p2(2.0, -2.0), p2(2.0, 2.0), p2(-2.0, 2.0)]);
    let holed = extruded(vec![square.clone(), circle_loop(0.0, 0.0, 0.8)], 1.0);
    fence_crosscheck(&holed, "extrude square + hole");

    // Rounded square with a circular hole — tangent joins on the outer
    // loop AND the ring path in one product.
    let rounded_holed = extruded(vec![rounded_square(2.0, 0.5), circle_loop(0.0, 0.0, 0.7)], 1.0);
    fence_crosscheck(&rounded_holed, "extrude rounded square + hole");
}

#[test]
fn revolve_products_carry_no_scaffold_at_rest() {
    // The full ring (rectangle off the axis).
    fence_crosscheck(
        &revolved(
            &[
                (0.4, 0.0, 0.0),
                (0.8, 0.0, 0.0),
                (0.8, 0.6, 0.0),
                (0.4, 0.6, 0.0),
            ],
            Revolution::Full,
        ),
        "revolve ring (full)",
    );
    // A partial wedge at an ordinary angle.
    fence_crosscheck(
        &revolved(
            &[
                (0.4, 0.0, 0.0),
                (0.8, 0.0, 0.0),
                (0.8, 0.6, 0.0),
                (0.4, 0.6, 0.0),
            ],
            Revolution::Partial(PI / 3.0),
        ),
        "revolve ring (partial pi/3)",
    );
    // Exactly pi: the two cap planes are coplanar and the meridian
    // copies land antipodally — the angle-pi lane the PR names.
    fence_crosscheck(
        &revolved(
            &[
                (0.4, 0.0, 0.0),
                (0.8, 0.0, 0.0),
                (0.8, 0.6, 0.0),
                (0.4, 0.6, 0.0),
            ],
            Revolution::Partial(PI),
        ),
        "revolve ring (partial pi)",
    );
    // The donut: a two-vertex bulge-1 circle profile revolved fully —
    // torus band walls, declared rim images.
    fence_crosscheck(
        &revolved(
            &[(1.5, 0.0, 1.0), (2.5, 0.0, 1.0)],
            Revolution::Full,
        ),
        "revolve donut (full)",
    );
    // The donut sector: same profile, partial sweep.
    fence_crosscheck(
        &revolved(
            &[(1.5, 0.0, 1.0), (2.5, 0.0, 1.0)],
            Revolution::Partial(2.0 * PI / 3.0),
        ),
        "revolve donut (partial)",
    );
}

#[test]
fn tube_products_carry_no_scaffold_at_rest() {
    let solid = tube_along_arc::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        2.0,
        TubeWindow::Arc { t0: 0.25, t1: 1.75 },
        0.5,
        Tol::witness(),
    )
    .expect("the solid elbow builds");
    fence_crosscheck(&solid.body, "tube_along_arc (arc window)");

    let hollow = tube_along_arc_hollow::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        2.0,
        TubeWindow::Full,
        0.5,
        0.125,
        Tol::witness(),
    )
    .expect("the hollow full tube builds");
    fence_crosscheck(&hollow.body, "tube_along_arc_hollow (full window)");
}

#[test]
fn loft_products_carry_no_scaffold_at_rest() {
    let quad = |pts: [(f64, f64); 4]| {
        vec![ProfileLoop::polygon(pts.map(|(x, y)| p2(x, y)))]
    };
    let square = quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]);
    let trapezoid = quad([(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)]);
    let places: Vec<Affine3<f64>> = [0.0, 1.0, 2.0]
        .iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect();
    let lofted = loft_body::<f64>(
        &[square.clone(), trapezoid, square],
        &places,
        2,
        Tol::witness(),
    )
    .expect("the loft prism builds");
    // The loft's NURBS walls keep tier 3 volume refused (the banked
    // rational-flux lane), so the cross-check here is the SCAN plus
    // the fence's own absence from the tier-3 report, not a green
    // tier 3.
    let scaffolds = scaffold_edges(&lofted.body);
    assert!(
        scaffolds.is_empty(),
        "loft prism: scaffold descriptions {scaffolds:?} survive at rest"
    );
    if let Err(errs) = topo::validate_geometric(&lofted.body, Tol::witness()) {
        assert!(
            !errs
                .iter()
                .any(|e| matches!(e, ValidationError::ScaffoldAtRest { .. })),
            "loft prism: tier 3 names a scaffold at rest: {errs:?}"
        );
    }
}

#[test]
fn boolean_products_carry_no_scaffold_at_rest() {
    let decls = BooleanDeclarations::none();
    // A through hole: plate minus a taller coaxial disc.
    let plate = extruded(
        vec![ProfileLoop::polygon([
            p2(-2.0, -2.0),
            p2(2.0, -2.0),
            p2(2.0, 2.0),
            p2(-2.0, 2.0),
        ])],
        1.0,
    );
    let disc = extruded(vec![circle_loop(0.0, 0.0, 0.6)], 1.0);
    let tall_disc = topo::transform_rigid(
        &disc,
        &Affine3::translation(Vec3::new(0.0, 0.0, -0.5)),
        Tol::witness(),
    )
    .unwrap();
    // The disc as minted spans z in [0, 1]; shifted to [-0.5, 0.5]
    // it still does not pierce the full plate — build a taller one.
    let tall = extruded(vec![circle_loop(0.0, 0.0, 0.6)], 2.0);
    let tall = topo::transform_rigid(
        &tall,
        &Affine3::translation(Vec3::new(0.0, 0.0, -0.5)),
        Tol::witness(),
    )
    .unwrap();
    drop(tall_disc);
    let holed = boolean_op_with(
        BooleanOp::Subtract,
        &plate,
        &tall,
        &decls,
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .expect("the subtract runs")
    .body()
    .expect("a body")
    .body
    .clone();
    fence_crosscheck(&holed, "boolean subtract (through hole)");

    // A union with a partially-embedded disc (blind boss).
    let boss = topo::transform_rigid(
        &disc,
        &Affine3::translation(Vec3::new(0.0, 0.0, 0.5)),
        Tol::witness(),
    )
    .unwrap();
    let united = boolean_op_with(
        BooleanOp::Union,
        &plate,
        &boss,
        &decls,
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .expect("the union runs")
    .body()
    .expect("a body")
    .body
    .clone();
    fence_crosscheck(&united, "boolean union (boss)");
}

#[test]
fn fillet_products_carry_no_scaffold_at_rest() {
    // The die blank: every edge of a cube filleted — the verb whose
    // strut conversion #1116 REVERTED. The in-source note at the
    // reverted site says the strut "reaches rest through the
    // scaffolding door and tier 3's transience fence names it"; this
    // row measures what the finished body actually carries.
    let body = sweep::test_support::cube(1.0, Tol::witness());
    let edges: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    let filleted = fillet_edges(&body, &edges, 0.125, band(), Tol::witness())
        .expect("the die blank fillets")
        .body;
    fence_crosscheck(&filleted, "fillet (die blank)");
}

// =====================================================================
// 2. Declaration transport: is_declared never flips silently.
// =====================================================================

/// The tube: outer wall r = 0.8, inner wall r = 0.4, annular caps at
/// y = 0 and y = 0.6 — the same fixture family `verbs_offd` uses.
fn tube() -> Body<f64> {
    revolved(
        &[
            (0.4, 0.0, 0.0),
            (0.8, 0.0, 0.0),
            (0.8, 0.6, 0.0),
            (0.4, 0.6, 0.0),
        ],
        Revolution::Full,
    )
}

fn plane_face_at(body: &Body<f64>, y: f64) -> topo::FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(body.get_surface(f.surface), Some(Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-9)
        })
        .map(|(k, _)| k)
        .unwrap_or_else(|| panic!("no planar face at y = {y}"))
}

fn cylinder_face_at(body: &Body<f64>, radius: f64) -> topo::FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(body.get_surface(f.surface), Some(Surface::Cylinder { radius: r, .. }) if (*r - radius).abs() < 1e-9)
        })
        .map(|(k, _)| k)
        .unwrap_or_else(|| panic!("no cylinder face at r = {radius}"))
}

/// A TRANSLATING offset (a cap) and a NON-translating offset (a wall)
/// both preserve the body-wide `is_declared` census: nothing flips
/// silently in either direction.
#[test]
fn offsets_preserve_the_authority_census() {
    // Cap offset: rigid translation, declarations carried bodily.
    let mut body = tube();
    let before = authority_census(&body);
    assert!(
        before.iter().any(|(_, d)| *d),
        "the tube must carry declared edges for this row to mean anything"
    );
    let cap = plane_face_at(&body, 0.6);
    topo::replace_face_offset(&mut body, cap, 0.05, 1e-6, band(), Tol::witness())
        .expect("the cap offsets");
    assert_eq!(
        authority_census(&body),
        before,
        "a translating offset must not flip any edge's is_declared"
    );
    fence_crosscheck(&body, "tube after cap offset");

    // Wall offset: the cylinder's offset is not a rigid translation,
    // but no DECLARED edge lies on the wall's own boundary (its rims
    // are intrinsic, its meridian is the chart's derived seam), so the
    // op succeeds and must still not flip anyone — in particular the
    // cap seams it re-anchors keep their declarations.
    let mut body = tube();
    let before = authority_census(&body);
    let wall = cylinder_face_at(&body, 0.4);
    topo::replace_face_offset(&mut body, wall, 0.05, 1e-6, band(), Tol::witness())
        .expect("the underived inner wall offsets");
    assert_eq!(
        authority_census(&body),
        before,
        "a non-translating offset of an underived wall must not flip any edge's is_declared"
    );
    fence_crosscheck(&body, "tube after inner wall offset");
}

/// A rigid transform preserves the authority census (and the mapped
/// declaration's placement moves with the body — certification of the
/// transformed body is the transform door's own gate; the census is
/// this row's).
#[test]
fn rigid_transform_preserves_the_authority_census() {
    let body = tube();
    let before: Vec<bool> = authority_census(&body).iter().map(|(_, d)| *d).collect();
    let map = Affine3::translation(Vec3::new(3.0, -1.0, 2.0))
        * Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::unit_z(), PI / 2.0);
    let moved = topo::transform_rigid(&body, &map, Tol::witness()).expect("the tube transforms");
    let after: Vec<bool> = authority_census(&moved).iter().map(|(_, d)| *d).collect();
    assert_eq!(after, before, "transform_rigid must not flip is_declared");
    fence_crosscheck(&moved, "tube after rigid transform");
}

/// Where a declaration CANNOT be carried, the door refuses loudly
/// instead of dropping it. Both refusal arms:
///
/// (a) a declared chart image on a face whose offset is not a rigid
///     translation (a circle rim on a cylinder wall — the carrier
///     transports, `delta` is `None`);
/// (b) a rotation-family declaring pushforward under a TRANSLATING
///     offset (the placement translates, the trajectory does not).
///
/// Both fixtures plant the declaration through `set_edge_curve`, which
/// accepts it — see `a_corrupt_declaration_certifies_clean_and_
/// survives_tier3` below for why that is possible at all.
#[test]
fn uncarriable_declarations_refuse_loudly_instead_of_flipping() {
    // (a) Re-describe the outer wall's top rim as a DECLARED image in
    // the wall's own chart, then offset the wall.
    let mut body = tube();
    let wall = cylinder_face_at(&body, 0.8);
    let wall_key = body.get_face(wall).unwrap().surface;
    let rim = body
        .edges()
        .find(|(k, e)| {
            let Some(c) = body.get_curve_geom(e.curve).and_then(CurveGeom::certified) else {
                return false;
            };
            let geom::Curve3::Circle { center, radius, .. } = c.carrier() else {
                return false;
            };
            ((*radius - 0.8).abs() < 1e-9 && (center.y - 0.6).abs() < 1e-9)
                && edge_touches_face(&body, *k, wall)
        })
        .map(|(k, _)| k)
        .expect("the outer wall's top rim");
    let curve = body
        .get_edge(rim)
        .and_then(|e| body.get_curve_geom(e.curve))
        .and_then(CurveGeom::certified)
        .unwrap()
        .clone();
    let mut spec = curve.restated_spec();
    spec.description = EdgeDescriptionSpec::chart(wall_key).declared_by(dummy_declaration());
    body.set_edge_curve(rim, spec, Tol::witness())
        .expect("the declared rim image certifies (the declaration is unmetered)");
    let err = topo::replace_face_offset(&mut body, wall, 0.05, 1e-6, band(), Tol::witness())
        .expect_err("a declared rim on a non-translating offset must refuse");
    assert!(
        matches!(err, ReplaceFaceError::CarrierLaneUnsupported { edge, .. } if edge == rim),
        "the refusal must name the declared rim, got {err:?}"
    );

    // (b) Give the top cap's seam a rotation-family declaration, then
    // offset the cap (a rigid translation — the placement would carry,
    // the trajectory cannot).
    let mut body = tube();
    let cap = plane_face_at(&body, 0.6);
    let seam = body
        .edges()
        .find(|(k, e)| {
            let Some(c) = body.get_curve_geom(e.curve).and_then(CurveGeom::certified) else {
                return false;
            };
            c.authority().is_declared()
                && matches!(c.carrier(), geom::Curve3::Line { .. })
                && edge_touches_face(&body, *k, cap)
        })
        .map(|(k, _)| k)
        .expect("the top cap's declared radial seam");
    let curve = body
        .get_edge(seam)
        .and_then(|e| body.get_curve_geom(e.curve))
        .and_then(CurveGeom::certified)
        .unwrap()
        .clone();
    let mut spec = curve.restated_spec();
    let EdgeDescriptionSpec::Chart {
        surface,
        image,
        seam: seam_flag,
        ..
    } = spec.description
    else {
        panic!("the cap seam is a chart image at rest");
    };
    spec.description = EdgeDescriptionSpec::Chart {
        surface,
        image,
        seam: seam_flag,
        declared: Some(MappedCurve::RevolvedPoint {
            point: Point2::new(0.0, 0.0),
            place: Affine3::translation(Vec3::new(0.4, 0.0, 0.0)),
            axis_origin: Point3::new(0.0, 0.0, 0.0),
            axis_dir: Vec3::unit_y(),
            angle: 0.5,
        }),
    };
    body.set_edge_curve(seam, spec, Tol::witness())
        .expect("the rotation-family declaration certifies (unmetered)");
    let err = topo::replace_face_offset(&mut body, cap, 0.05, 1e-6, band(), Tol::witness())
        .expect_err("a rotation-family declaration under a translating offset must refuse");
    assert!(
        matches!(err, ReplaceFaceError::CarrierLaneUnsupported { edge, .. } if edge == seam),
        "the refusal must name the declared seam, got {err:?}"
    );
}

fn edge_touches_face(body: &Body<f64>, edge: EdgeKey, face: topo::FaceKey) -> bool {
    let Some(e) = body.get_edge(edge) else {
        return false;
    };
    [e.he_plus, e.he_minus].iter().any(|&he| {
        body.get_half_edge(he)
            .and_then(|h| body.get_loop(h.parent_loop))
            .map(|l| l.face == face)
            .unwrap_or(false)
    })
}

fn dummy_declaration() -> MappedCurve<f64> {
    MappedCurve::ExtrudedPoint {
        point: Point2::new(0.0, 0.0),
        place: Affine3::translation(Vec3::new(123.0, -456.0, 789.0)),
        vec: Vec3::new(0.0, 0.0, 1.0),
    }
}

// =====================================================================
// 3. The unmetered declaration — a measured finding, kept as a row.
// =====================================================================

/// **FINDING (R1, P-1b):** certification meters a scaffold's
/// pushforward at every sample (`carrier_matches_mapped_source`), but
/// the `declared` record beside a chart image is metered NOWHERE — not
/// at `set_edge_curve`, not at tier 3. Pre-collapse, the pushforward
/// WAS the description on every conventional edge and the meter ran on
/// every certify; post-collapse a declaration can be arbitrarily false
/// and nothing says so. Tier 3's prefer-intrinsic rules and the offset
/// door's transport both READ this record.
///
/// This row pins the measured fact. If it goes red, the kernel grew a
/// meter for the declaration and the finding is closed — flip the
/// asserts, cite this doc.
#[test]
fn a_corrupt_declaration_certifies_clean_and_survives_tier3() {
    let mut body = tube();
    // The outer wall's derived seam meridian: give it a declaration
    // whose placement is ~1000 units away from the body.
    let victim = body
        .edges()
        .find(|(_, e)| {
            matches!(
                body.get_curve_geom(e.curve)
                    .and_then(CurveGeom::certified)
                    .map(topo::EdgeCurve::description),
                Some(EdgeDescription::Chart(c)) if c.seam
            )
        })
        .map(|(k, _)| k)
        .expect("the tube has a seam-described meridian");
    let curve = body
        .get_edge(victim)
        .and_then(|e| body.get_curve_geom(e.curve))
        .and_then(CurveGeom::certified)
        .unwrap()
        .clone();
    let mut spec = curve.restated_spec();
    spec.description = match spec.description {
        EdgeDescriptionSpec::Chart {
            surface,
            image,
            seam,
            ..
        } => EdgeDescriptionSpec::Chart {
            surface,
            image,
            seam,
            declared: Some(dummy_declaration()),
        },
        other => panic!("expected a chart image, got {other:?}"),
    };
    body.set_edge_curve(victim, spec, Tol::witness())
        .expect("MEASURED: a declaration 1000 units off the body certifies clean");
    assert_eq!(
        topo::validate_geometric(&body, Tol::witness()),
        Ok(()),
        "MEASURED: tier 3 passes the body carrying the corrupt declaration"
    );
    // And the record now steers tier-3 semantics: the edge reads as
    // modeler-declared.
    let after = body
        .get_edge(victim)
        .and_then(|e| body.get_curve_geom(e.curve))
        .and_then(CurveGeom::certified)
        .unwrap();
    assert!(
        matches!(after.authority(), EdgeAuthority::Declared(_)),
        "the corrupt record is now this edge's authority"
    );
}
