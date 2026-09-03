//! MATE-7a R1 review probes (blinded lane) — independent re-measurement
//! of PR #1477's load-bearing claims, above all the HEADLINE deviation:
//! lily wall 1 did NOT retire, on a measurement claim about the lily
//! scene's own geometry. Each probe either confirms that measurement
//! from scratch (rebuilding the scene's stem and arch from the turtle
//! math in `demos/tour/src/lily.rs`, outside the demo crate) or
//! attacks a claim the PR's own rows cannot already witness.
//!
//! Review-lane only; not part of the PR under review.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::SurfaceKind;
use geom_core::{Point3, Tol, Vec3};
use sweep::{TubeWindow, tube_along_arc};
use topo::query::{self, SurfaceKindSet};
use topo::{Body, BooleanDeclarations, BooleanError, ContactClass, FaceKey, FacePairDeclaration};

const STEM_TUBE: f64 = 0.060;
const ARCH_TUBE: f64 = 0.052;
const STEM_RING: f64 = 5.0;
const ARCH_RING: f64 = 1.1;

fn deg(d: f64) -> f64 {
    d.to_radians()
}

/// The lily's lower stem arc, rebuilt from the scene's own turtle
/// numbers: root at the origin heading +z (in the xz-plane), left turn
/// of 22 degrees on a 5 m ring — so the ring centre is (-5, 0, 0), the
/// start radial +x, the axis -y (`tube_arc`'s left-turn sense).
fn stem() -> Body<f64> {
    tube_along_arc(
        Point3::new(-STEM_RING, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        STEM_RING,
        TubeWindow::Arc {
            t0: 0.0,
            t1: deg(22.0),
        },
        STEM_TUBE,
        Tol::witness(),
    )
    .expect("stem builds")
    .body
}

/// The fork point (the stem's end / the arch's start), its tangent,
/// the arch's ring centre and start radial — the turtle math of
/// `lily.rs` (`Turtle::arc`), reproduced independently.
struct ArchFrame {
    fork: Point3<f64>,
    center: Point3<f64>,
    radial: Vec3<f64>,
    /// The arch's END point (the far cap's centre).
    far: Point3<f64>,
}

fn arch_frame() -> ArchFrame {
    let a = deg(22.0);
    // at_fork (xz): p = stem centre + 5·rot((1,0), 22°); t = rot((0,1), 22°).
    let fork2 = (-STEM_RING + STEM_RING * a.cos(), STEM_RING * a.sin());
    let t2 = (-a.sin(), a.cos());
    // Left turn: n = (-t.z, t.x); centre = fork + 1.1 n; radial = -n.
    let n2 = (-t2.1, t2.0);
    let c2 = (fork2.0 + ARCH_RING * n2.0, fork2.1 + ARCH_RING * n2.1);
    let r2 = (-n2.0, -n2.1);
    // The arch's end: centre + 1.1·rot(radial, 170°).
    let turn = deg(170.0);
    let adv = (
        r2.0 * turn.cos() - r2.1 * turn.sin(),
        r2.0 * turn.sin() + r2.1 * turn.cos(),
    );
    ArchFrame {
        fork: Point3::new(fork2.0, 0.0, fork2.1),
        center: Point3::new(c2.0, 0.0, c2.1),
        radial: Vec3::new(r2.0, 0.0, r2.1),
        far: Point3::new(c2.0 + ARCH_RING * adv.0, 0.0, c2.1 + ARCH_RING * adv.1),
    }
}

/// The lily's arch, continuing G1 from the stem's end: 170 degrees on
/// a 1.1 m ring, tube 0.052.
fn arch() -> Body<f64> {
    let f = arch_frame();
    tube_along_arc(
        f.center,
        Vec3::new(0.0, -1.0, 0.0),
        f.radial,
        ARCH_RING,
        TubeWindow::Arc {
            t0: 0.0,
            t1: deg(170.0),
        },
        ARCH_TUBE,
        Tol::witness(),
    )
    .expect("arch builds")
    .body
}

fn plane_faces(body: &Body<f64>) -> Vec<(FaceKey, Point3<f64>, Vec3<f64>)> {
    body.faces()
        .filter_map(|(k, f)| match body.get_surface(f.surface) {
            Some(&geom::Surface::Plane { origin, normal, .. }) => Some((k, origin, normal)),
            _ => None,
        })
        .collect()
}

fn torus_faces(body: &Body<f64>) -> Vec<FaceKey> {
    query::all_faces(body)
        .into_iter()
        .filter(|&f| query::face_surface_matches(body, f, SurfaceKindSet::just(SurfaceKind::Torus)))
        .collect()
}

/// The scene's flush plane declaration, reproduced: the ONE coplanar
/// cross pair (stem end cap against arch start cap), declared `Rest` —
/// the weld's whole declared contact, the pair
/// `topo::flush::find_flush_candidates` reports there (which is what
/// the scene's own helper now runs).
///
/// The selection here is by POSITION, not by flushness: it picks the
/// caps at the fork and lets the op verify them, which is why it is
/// not a second spelling of the detector's decisions.
fn weld_declarations(stem: &Body<f64>, arch: &Body<f64>) -> (BooleanDeclarations, usize) {
    let fork = arch_frame().fork;
    let mut decls = BooleanDeclarations::none();
    let mut pairs = 0;
    for &(fa, oa, _) in &plane_faces(stem) {
        for &(fb, ob, _) in &plane_faces(arch) {
            if (oa - fork).norm() < 1e-9 && (ob - fork).norm() < 1e-9 {
                decls
                    .coincident_faces
                    .push(FacePairDeclaration::rest(fa, fb));
                pairs += 1;
            }
        }
    }
    (decls, pairs)
}

/// Exact distance from a point to the stem's 22-degree tube-centre
/// arc (closest point on an arc is the azimuth projection when it
/// lands in the window, else the nearer endpoint).
fn dist_to_stem_center_arc(p: Point3<f64>) -> f64 {
    let c = Point3::new(-STEM_RING, 0.0, 0.0);
    let rel = p - c;
    let theta = rel.z.atan2(rel.x).clamp(0.0, deg(22.0));
    let on = Point3::new(
        c.x + STEM_RING * theta.cos(),
        0.0,
        c.z + STEM_RING * theta.sin(),
    );
    (p - on).norm()
}

/// P1 — the HEADLINE measurement, re-taken from scratch. PR #1477's
/// deviation 1 claims: (a) the wall-1 refusal names the stem's tube
/// wall against the arch's FAR cap; (b) those two exact loci never
/// come within ~2 m of each other; (c) what overlaps is the stem
/// wall's WHOLE-TORUS box (a 22° arc boxed as the full ring); (d) the
/// weld has no torus×torus contact to declare (tube 0.060 vs 0.052 —
/// the walls share only the weld plane). All four are re-derived here
/// with no `if let` skip-hazard: the far cap MUST exist and MUST be
/// the named face.
#[test]
fn p1_wall1_named_pair_is_a_whole_torus_box_artifact_two_metres_from_contact() {
    let (s, a) = (stem(), arch());
    let (decls, pairs) = weld_declarations(&s, &a);
    assert_eq!(pairs, 1, "exactly one coplanar cross cap pair at the fork");

    let err = topo::union_with(&s, &a, &decls, Tol::witness())
        .expect_err("wall 1 must still refuse (the PR's own headline deviation)");
    let (face, other_face) = match err {
        BooleanError::CurvedPairUnsupported {
            op: None,
            kind: geom_brep::SurfaceKind::Torus,
            other_kind: geom_brep::SurfaceKind::Plane,
            face,
            other_face,
            ..
        } => (face, other_face),
        other => panic!("wall 1 must be the gate's torus-against-plane pair, got {other:?}"),
    };
    assert!(
        torus_faces(&s).contains(&face),
        "the named A face is the stem's tube wall"
    );

    // (a) The named plane is the arch's FAR cap — no `if let` escape:
    // the far cap must exist, and must be the named face.
    let frame = arch_frame();
    let far_cap = plane_faces(&a)
        .into_iter()
        .find(|&(_, o, _)| (o - Point3::new(0.0, 0.0, 0.0)).norm() > 2.0)
        .expect("the arch has a cap more than 2 m from the world origin");
    assert!(
        (far_cap.1 - frame.far).norm() < 1e-9,
        "that cap sits at the turtle math's arch end point"
    );
    assert_eq!(
        other_face, far_cap.0,
        "the gate names the stem wall against the arch's FAR cap"
    );

    // (b) The exact loci never approach: every point of the far cap's
    // disc is more than 1.9 m from the stem's windowed tube wall.
    // Lower bound: distance to the tube-CENTRE arc minus the tube
    // radius, taken over a dense disc sampling.
    let (e1, e2) = {
        let n = (frame.far - frame.center).cross(Vec3::new(0.0, 1.0, 0.0));
        // The cap plane contains ±y and the in-plane direction
        // perpendicular to the end tangent; a basis from the world's
        // own frame is enough for sampling.
        let n = n.normalize();
        (Vec3::new(0.0, 1.0, 0.0), n)
    };
    let mut min_lb = f64::INFINITY;
    for ir in 0..=8 {
        let r = ARCH_TUBE * f64::from(ir) / 8.0;
        for ip in 0..48 {
            let psi = core::f64::consts::TAU * f64::from(ip) / 48.0;
            let p = frame.far + (e1 * psi.cos() + e2 * psi.sin()) * r;
            min_lb = min_lb.min(dist_to_stem_center_arc(p) - STEM_TUBE);
        }
    }
    assert!(
        min_lb > 1.9,
        "the far cap must stand ~2 m clear of the stem wall's exact locus; \
         measured lower bound {min_lb:.3} m"
    );

    // (c) The whole-torus box artifact: the far cap's CENTRE sits
    // inside the stem wall's whole-ring box, while a boundary-tight
    // box of the 22° arc tube would clear it by more than 1.5 m in x.
    let whole = (
        (-2.0 * STEM_RING - STEM_TUBE, STEM_TUBE),
        (-STEM_TUBE, STEM_TUBE),
        (-STEM_RING - STEM_TUBE, STEM_RING + STEM_TUBE),
    );
    let q = frame.far;
    assert!(
        q.x > whole.0.0
            && q.x < whole.0.1
            && q.y > whole.1.0
            && q.y < whole.1.1
            && q.z > whole.2.0
            && q.z < whole.2.1,
        "the far cap centre lies inside the stem wall's whole-torus box"
    );
    let tight_min_x = -STEM_RING + STEM_RING * deg(22.0).cos() - STEM_TUBE;
    assert!(
        q.x < tight_min_x - 1.5,
        "a boundary-tight box of the 22° arc (min x {tight_min_x:.3}) would clear the \
         far cap (x {:.3}) by more than 1.5 m",
        q.x
    );

    // (d) The weld has no torus×torus contact: the two tube walls'
    // minor radii differ (0.060 vs 0.052), and near the weld plane the
    // walls stay ~8 mm apart (concentric cross-sections). Sampled on
    // both walls within 1° of the weld.
    let assert_minor = |body: &Body<f64>, want: f64| {
        for (_, f) in body.faces() {
            if let Some(&geom::Surface::Torus { minor_radius, .. }) = body.get_surface(f.surface) {
                assert!((minor_radius - want).abs() < 1e-12, "minor radius {want}");
            }
        }
    };
    assert_minor(&s, STEM_TUBE);
    assert_minor(&a, ARCH_TUBE);
    let stem_pts: Vec<Point3<f64>> = {
        let mut v = Vec::new();
        for it in 0..=30 {
            let theta = deg(21.0) + deg(1.0) * f64::from(it) / 30.0;
            let cs = Point3::new(
                -STEM_RING + STEM_RING * theta.cos(),
                0.0,
                STEM_RING * theta.sin(),
            );
            let radial = Vec3::new(theta.cos(), 0.0, theta.sin());
            for ip in 0..180 {
                let phi = core::f64::consts::TAU * f64::from(ip) / 180.0;
                v.push(
                    cs + (radial * phi.cos() + Vec3::new(0.0, 1.0, 0.0) * phi.sin()) * STEM_TUBE,
                );
            }
        }
        v
    };
    let arch_pts: Vec<Point3<f64>> = {
        let mut v = Vec::new();
        for it in 0..=30 {
            let theta = deg(1.0) * f64::from(it) / 30.0;
            let (rc, rs) = (theta.cos(), theta.sin());
            // Rotate the arch's start radial by theta about -y (the
            // same sense the window advances).
            let r0 = frame.radial;
            let radial = Vec3::new(r0.x * rc - r0.z * rs, 0.0, r0.x * rs + r0.z * rc);
            let cs = frame.center + radial * ARCH_RING;
            for ip in 0..180 {
                let phi = core::f64::consts::TAU * f64::from(ip) / 180.0;
                v.push(
                    cs + (radial * phi.cos() + Vec3::new(0.0, 1.0, 0.0) * phi.sin()) * ARCH_TUBE,
                );
            }
        }
        v
    };
    let mut min_wall = f64::INFINITY;
    for p in &stem_pts {
        for q in &arch_pts {
            min_wall = min_wall.min((*p - *q).norm());
        }
    }
    assert!(
        (0.006..0.010).contains(&min_wall),
        "the two tube walls share only the weld plane: nearest approach must be the \
         8 mm annular gap, measured {min_wall:.4} m"
    );
}

/// P2 — the π-arm PRICE claim, re-measured through the verdict log on
/// the PR's own G1 chain fixture, counting definite verdicts per
/// predicate.
///
/// **RE-MEASURED at the fix pass, and the number moved: 34 → 53.** This
/// probe was written against the pre-fix routing, which reached the
/// material arm with no first-order screen in front of it — the MAJOR
/// defect. Importing the screen that arm is only defined behind costs
/// one `classify_dihedral` per station, and that call meters two rows
/// (`dihedral_arm`, `dihedral_wedge`), so nine stations add 18. The
/// screen is what makes the answer TRUE; 18 rows is what truth costs
/// here. No baseline is a target to preserve — the number moved and the
/// question is whether the new behaviour is right, not how to get the
/// old number back.
///
/// The whole current price:
///
/// - **18 first-order screen** — 9 × (`dihedral_arm`, `dihedral_wedge`);
/// - **27 material arm** — 9 × (`material_wedge_side`,
///   `tangent_second_order`, `material_cusp_side`);
/// - **6 rim identification** — `rim_circle_radius` ×3,
///   `rim_circle_center` ×2, `rim_circle_axis_parallel` ×1 (the fix
///   pass put the two LENGTH data ahead of the angular one, so radius
///   now leads and short-circuits more pairs);
/// - **2 conformal screen** — `carrier_torus_axis_parallel`,
///   `carrier_torus_center`.
///
/// **The counts are FIXTURE-SPECIFIC, and the split is the point.** The
/// per-station blocks are structural — `CERT_SAMPLES` times the
/// predicate set, true of any rim this door classifies — and are
/// asserted against `CERT_SAMPLES` itself rather than against the
/// literal 9, so they state the invariant instead of a coincidence. The
/// rim-identification counts are not structural: they depend on how
/// many boundary circles each face carries and on the order the scan
/// meets them, so the same claim on the kissing fixture counts
/// differently. That is why the PR body reports the price per fixture.
#[test]
fn p2_the_g1_chain_price_is_the_measured_53_rows() {
    // The PR's fixtures, verbatim from `mate7a_torus_rest.rs`.
    let seg_a = stem();
    let seg_b = {
        let turn = deg(22.0);
        let end = Point3::new(
            -STEM_RING + STEM_RING * turn.cos(),
            0.0,
            STEM_RING * turn.sin(),
        );
        let tangent = Vec3::new(-turn.sin(), 0.0, turn.cos());
        let inward = Vec3::new(-tangent.z, 0.0, tangent.x);
        let center = end + inward * 1.1;
        tube_along_arc(
            center,
            Vec3::new(0.0, -1.0, 0.0),
            (end - center).normalize(),
            1.1,
            TubeWindow::Arc {
                t0: 0.0,
                t1: deg(170.0),
            },
            STEM_TUBE,
            Tol::witness(),
        )
        .expect("segment B builds")
        .body
    };
    let mut decls = BooleanDeclarations::none();
    for &fa in &torus_faces(&seg_a) {
        for &fb in &torus_faces(&seg_b) {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Tangent));
        }
    }
    geom_core::k_stats::start_verdict_log();
    let err = topo::union_with(&seg_a, &seg_b, &decls, Tol::witness())
        .expect_err("the chain refuses at the routing");
    let log = geom_core::k_stats::take_verdict_log();
    // The variant SPLIT after this probe was written (fix-pass MIN-6):
    // the pi arm is built, so the seam case no longer borrows a name
    // that calls its arm unbuilt. Same claim, current spelling.
    assert!(
        matches!(&err, BooleanError::RimSeamNotDeclarable { .. }),
        "the chain's rim is the wedge-π seam: {err:?}"
    );
    let count = |name: &str| log.iter().filter(|v| v.predicate == name).count();
    let mut histogram: std::collections::BTreeMap<&'static str, usize> = Default::default();
    for v in &log {
        *histogram.entry(v.predicate).or_default() += 1;
    }
    println!("verdict histogram for the G1 chain fixture: {histogram:#?}");
    let n = usize::try_from(geom_brep::CERT_SAMPLES).expect("the sample schedule fits usize");
    for (name, want) in [
        // Structural: the per-station predicate set, once per station.
        ("dihedral_arm", n),
        ("dihedral_wedge", n),
        ("material_wedge_side", n),
        ("tangent_second_order", n),
        ("material_cusp_side", n),
        // Fixture-specific: how many boundary circles this face pair
        // carries, and the order the scan meets them.
        ("rim_circle_radius", 3),
        ("rim_circle_center", 2),
        ("rim_circle_axis_parallel", 1),
        ("carrier_torus_axis_parallel", 1),
        ("carrier_torus_center", 1),
    ] {
        assert_eq!(
            count(name),
            want,
            "PR #1477's price table says {want} definite {name} rows; the log says \
             {} — full histogram above",
            count(name)
        );
    }
    // The other two torus-rung margins must NOT have fired on this
    // fixture (the PR's table stops at the two-screen), and no other
    // mate7a-new predicate name exists to fire.
    assert_eq!(count("carrier_torus_major_radius"), 0);
    assert_eq!(count("carrier_torus_minor_radius"), 0);
}

/// P3 — attack on the covered-pair rung: the wall-1 pair CANNOT be
/// bought. If an author declares the stem's tube wall against the
/// arch's far cap `Rest` (the exact pair the gate names), the
/// verification runs BEFORE the gate and contradicts the cross-kind
/// carrier claim structurally — the declaration never becomes
/// coverage. This is the "kind-generic, not kind-blind" claim
/// exercised at its sharpest point: both kinds ARE in the widened
/// inventory (plane and torus), and the pair is still refused, on the
/// carrier-kind rung.
#[test]
fn p3_a_false_cross_kind_declaration_cannot_cover_the_wall1_pair() {
    let (s, a) = (stem(), arch());
    let (mut decls, _) = weld_declarations(&s, &a);
    let frame = arch_frame();
    let far_cap = plane_faces(&a)
        .into_iter()
        .find(|&(_, o, _)| (o - frame.far).norm() < 1e-9)
        .expect("far cap");
    for &wall in &torus_faces(&s) {
        decls
            .coincident_faces
            .push(FacePairDeclaration::rest(wall, far_cap.0));
    }
    let err = topo::union_with(&s, &a, &decls, Tol::witness())
        .expect_err("a torus-against-plane Rest declaration is a false statement");
    match err {
        BooleanError::ContactContradicted { margin, .. } => {
            assert_eq!(
                margin.predicate,
                Some("carrier_kind"),
                "the refusal is the structural kind rung's: {margin:?}"
            );
        }
        other => panic!(
            "the false declaration must be contradicted before it can cover anything, \
             got {other:?}"
        ),
    }
}

/// P4 — the routing's rim identification is decided at the run's band
/// on the carrier's own data: two near-miss circles (radius off by a
/// definite margin) yield NO shared rim, so the bare class refusal
/// stands. Guards the `shared_rim` margins against a
/// value-equality-only reading (the claim that the identification is
/// METERED, not bit-compared).
#[test]
fn p4_a_definitely_different_rim_radius_keeps_the_class_refusal() {
    let seg_a = stem();
    // Same frame as the chain's segment B, but tube 0.055: its
    // terminal circle rides a different-radius carrier, so no rim is
    // shared and the routing must stay silent.
    let turn = deg(22.0);
    let end = Point3::new(
        -STEM_RING + STEM_RING * turn.cos(),
        0.0,
        STEM_RING * turn.sin(),
    );
    let tangent = Vec3::new(-turn.sin(), 0.0, turn.cos());
    let inward = Vec3::new(-tangent.z, 0.0, tangent.x);
    let center = end + inward * 1.1;
    let seg_b = tube_along_arc(
        center,
        Vec3::new(0.0, -1.0, 0.0),
        (end - center).normalize(),
        1.1,
        TubeWindow::Arc {
            t0: 0.0,
            t1: deg(170.0),
        },
        0.055,
        Tol::witness(),
    )
    .expect("segment B' builds")
    .body;
    let mut decls = BooleanDeclarations::none();
    for &fa in &torus_faces(&seg_a) {
        for &fb in &torus_faces(&seg_b) {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Tangent));
        }
    }
    let err = topo::union_with(&seg_a, &seg_b, &decls, Tol::witness())
        .expect_err("no shared rim, no routing");
    assert!(
        matches!(
            err,
            BooleanError::UnsupportedDeclarationClass {
                class: ContactClass::Tangent
            }
        ),
        "different-radius terminal circles are not one rim: {err:?}"
    );
}
