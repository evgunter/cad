//! **The seam-split closed rim's band: ONE annulus over several arcs.**
//!
//! A full revolve of a POLE-TOUCHING profile splits every wall into two
//! half-bands, so every latitude rim arrives as two arcs meeting at two
//! chart-seam vertices. A chain that STOPS at such a vertex is refused
//! `SeamVertex`, whose recourse names the request that describes what
//! the caller wants: ask for the rim WHOLE. These rows are what makes
//! that recourse true.
//!
//! The fixture is a LANTERN: a pole-touching solid of revolution whose
//! three latitude rims are the three support pairs the seam splits —
//! plane×sphere at its base (issue 319's own case), sphere×cone at its
//! shoulder, cone×plane at its lip. Every number below is derived from
//! the profile by the rolling ball's own two equations, never read back
//! from the arm.
//!
//! What makes each row go red:
//!
//! - **Each rim carves to its own closed form** — the band's torus is
//!   centred where `|c − O| = R − r` and `dist(c, support) = r` put the
//!   ball, to 1e-12, and the result is tier-3 valid (which re-derives
//!   both trim circles' tangential-contact descriptions and the slit's
//!   seam description at rest).
//! - **The band is ONE annulus wall** over both arcs: one boundary
//!   cycle, no ring, two trim arcs per side and one doubly-traversed
//!   slit — red if the carve ever leaves a sector wall behind.
//! - **The supports are several FACES of one SURFACE** per side, which
//!   is the resolution this door rests on and the half-face shape #319
//!   met.
//! - **The seam-split carve removes exactly what a ONE-EDGE carve of
//!   the same rim removes**: the same profile bored on-axis has the
//!   same three rims as single closed edges, and both doors take the
//!   same material out of the same solid region.
//! - **The three rims compose in sequence** — one call each, each on
//!   the last one's result, to a tier-3-valid solid carrying three
//!   bands.
//! - **Both material configurations**, at the arms' own level: each of
//!   the three support pairs puts the ball where its two equations say
//!   with the stored sense bits set either way. The CARVE takes one of
//!   them — a concave chain adds material — and the concave gate is
//!   rowed beside it, on a seam-split rim, so this door cannot be read
//!   as having widened that.
//! - **The naming totality**: every output entity of a seam-split band
//!   is a recorded mint or a survivor, and every retirement names a
//!   SOURCE key.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::SQRT_2;

use geom::{Curve3, Surface};
use geom_core::{Band, Point2, Point3, Tol, Vec3};
use profile::ProfileVertex;
use sweep::Revolution;
use sweep::fillet::FilletError;
use sweep::fillet::blend::{Meridian, SupportTrace, sheet_center};
use sweep::fillet::build::fillet_edges;
use sweep::test_support::revolved_about_y;
use topo::{Body, EdgeKey, FaceKey, SurfaceKey, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::new(tol().eps(), tol().k() * tol().eps()).unwrap()
}

fn v(x: f64, y: f64, bulge: f64) -> ProfileVertex<f64> {
    ProfileVertex::new(Point2::new(x, y), bulge)
}

// ------------------------------------------------------------------
// The lantern, and the numbers its profile fixes.
// ------------------------------------------------------------------

/// The sphere the lantern's shoulder is cut from: the UNIT sphere on
/// the origin, so `(1, 0)` and `(0.8, 0.6)` are both exact on it.
const SPHERE_R: f64 = 1.0;
/// Where the sphere gives way to the cone — a 3-4-5 point.
const SHOULDER: (f64, f64) = (0.8, 0.6);
/// The lantern's top plane, and the lip radius there. The cone runs
/// from the shoulder to `(LIP_R, TOP)`, i.e. along `(−1, 1)/√2`.
const TOP: f64 = 1.2;
const LIP_R: f64 = 0.2;
/// The bore of the ANNULAR twin: inside every trim circle below, so the
/// twin's rims are the lantern's rims as single closed edges.
const BORE: f64 = 0.1;

/// The lantern's profile, from the bottom pole outward.
///
/// `(0,0) → (1,0)` is radial, so it revolves to the base DISK;
/// `(1,0) → (0.8,0.6)` is the unit-sphere arc; `(0.8,0.6) → (0.2,1.2)`
/// is straight, so it revolves to a CONE; `(0.2,1.2) → (0,1.2)` is
/// radial again, the top DISK. Both ends touch the axis, which is what
/// makes every wall a pair of half-bands and every rim a pair of arcs.
fn lantern() -> Body<f64> {
    // A profile arc's bulge is the tangent of a QUARTER of its sweep,
    // and this arc's sweep is the angle between two exact unit vectors,
    // `(1, 0)` and `(0.8, 0.6)`.
    let bulge = (0.6f64.asin() / 4.0).tan();
    revolved_about_y(
        vec![
            v(0.0, 0.0, 0.0),
            v(1.0, 0.0, bulge),
            v(SHOULDER.0, SHOULDER.1, 0.0),
            v(LIP_R, TOP, 0.0),
            v(0.0, TOP, 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// The same solid bored on-axis at [`BORE`], so the profile is ANNULAR
/// and each of the three rims is ONE closed edge — the shape the
/// annulus door already served. Outside the bore the two bodies are the
/// same solid, so a fillet of the same rim removes the same material.
fn bored_lantern() -> Body<f64> {
    let bulge = (0.6f64.asin() / 4.0).tan();
    revolved_about_y(
        vec![
            v(BORE, 0.0, 0.0),
            v(1.0, 0.0, bulge),
            v(SHOULDER.0, SHOULDER.1, 0.0),
            v(LIP_R, TOP, 0.0),
            v(BORE, TOP, 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// The cone's outward unit normal in the meridian sheet: the cone runs
/// along `(−1, 1)/√2`, so its normal is `(1, 1)/√2`.
fn cone_normal() -> (f64, f64) {
    (1.0 / SQRT_2, 1.0 / SQRT_2)
}

/// **The NECK rim's ball centre**, from its own two equations: the ball
/// sits `r` above the base plane and `R − r` from the sphere's centre.
fn neck_center(r: f64) -> (f64, f64) {
    (((SPHERE_R - r).powi(2) - r * r).sqrt(), r)
}

/// **The SHOULDER rim's ball centre**: on the cone's inward offset
/// line, at distance `R − r` from the sphere's centre. The offset line
/// meets the offset sphere twice; the RIM branch is the crossing that
/// returns the rim point as `r → 0`, which is the root nearer zero.
fn shoulder_center(r: f64) -> (f64, f64) {
    let (nx, ny) = cone_normal();
    let (ux, uy) = (-1.0 / SQRT_2, 1.0 / SQRT_2);
    let q = (SHOULDER.0 - r * nx, SHOULDER.1 - r * ny);
    let b = q.0 * ux + q.1 * uy;
    let dd = q.0 * q.0 + q.1 * q.1 - (SPHERE_R - r).powi(2);
    let s = (b * b - dd).sqrt();
    let (t1, t2) = (-b - s, -b + s);
    let t = if t1.abs() <= t2.abs() { t1 } else { t2 };
    (q.0 + t * ux, q.1 + t * uy)
}

/// **The LIP rim's ball centre**: two LINEAR equations — `r` below the
/// top plane, and `r` inside the cone through the shoulder.
fn lip_center(r: f64) -> (f64, f64) {
    // (c − shoulder)·n = −r with n = (1,1)/√2 gives
    // c.x = (0.8 + 0.6) − c.y − r√2.
    let y = TOP - r;
    (SHOULDER.0 + SHOULDER.1 - y - r * SQRT_2, y)
}

/// One rim of the lantern: its name, its circle's radius and station,
/// and the ball centre its two supports put the ball at.
type Rim = (&'static str, f64, f64, fn(f64) -> (f64, f64));

/// The three rims a pole-touching revolve of this profile splits.
fn rims() -> [Rim; 3] {
    [
        (
            "the plane×sphere neck",
            1.0,
            0.0,
            neck_center as fn(f64) -> (f64, f64),
        ),
        (
            "the sphere×cone shoulder",
            SHOULDER.0,
            SHOULDER.1,
            shoulder_center,
        ),
        ("the cone×plane lip", LIP_R, TOP, lip_center),
    ]
}

/// One support pair, as the sense-bit row reads it: its name, the rim
/// point its sheet is taken at, the two traces as functions of their
/// stored sense bit, the two signed distances in the supports' own
/// closed forms, and whether a ball rests there at all.
type TraceOf<'a> = Box<dyn Fn(f64) -> SupportTrace<f64> + 'a>;
type DistOf<'a> = Box<dyn Fn(Point3<f64>) -> f64 + 'a>;
type ArmRow<'a> = (
    &'static str,
    Point3<f64>,
    TraceOf<'a>,
    TraceOf<'a>,
    DistOf<'a>,
    DistOf<'a>,
    Box<dyn Fn(f64, f64) -> bool + 'a>,
);

// ------------------------------------------------------------------
// Reading the fixture.
// ------------------------------------------------------------------

fn surface_of(body: &Body<f64>, f: FaceKey) -> SurfaceKey {
    body.get_face(f).unwrap().surface
}

/// The two faces an edge separates, in `(he_plus, he_minus)` order.
fn faces_of(body: &Body<f64>, e: EdgeKey) -> (FaceKey, FaceKey) {
    let ed = body.get_edge(e).unwrap();
    let f = |he| {
        body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
            .unwrap()
            .face
    };
    (f(ed.he_plus), f(ed.he_minus))
}

/// Every circular edge at radius `r` and station `y` whose two supports
/// are DIFFERENT surfaces — which excludes a chart seam, whose carrier
/// can share a rim's radius and centre exactly (a sphere's seam
/// meridian is a great circle).
fn rim_arcs(body: &Body<f64>, r: f64, y: f64) -> Vec<EdgeKey> {
    body.edges()
        .filter_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            match *c.carrier() {
                Curve3::Circle { radius, center, .. }
                    if (radius - r).abs() < 1e-9 && (center.y - y).abs() < 1e-9 =>
                {
                    Some(k)
                }
                _ => None,
            }
        })
        .filter(|k| {
            let (a, b) = faces_of(body, *k);
            surface_of(body, a) != surface_of(body, b)
        })
        .collect()
}

fn band_torus(body: &Body<f64>, face: FaceKey) -> (Point3<f64>, f64, f64) {
    match body
        .get_surface(body.get_face(face).unwrap().surface)
        .unwrap()
    {
        &Surface::Torus {
            center,
            major_radius,
            minor_radius,
            ..
        } => (center, major_radius, minor_radius),
        other => panic!("the band face is a torus, got {other:?}"),
    }
}

fn volume(body: &Body<f64>) -> f64 {
    let props = mass_properties(body, tol()).expect("mass properties must compute");
    assert_eq!(
        props.volume_pad, 0.0,
        "every face of a lantern carve is closed-form"
    );
    props.volume
}

// ------------------------------------------------------------------
// The rows.
// ------------------------------------------------------------------

/// **Every rim of the lantern carves whole, to its own closed form.**
///
/// Each rim arrives as TWO arcs meeting at two seam vertices, and each
/// leaves as ONE torus band whose spine is where the rolling ball's two
/// equations put it. The band's `major_radius` is the ball centre's own
/// radial coordinate and its `minor_radius` is `r`; the spine's station
/// is the centre's own. None of the three is the arm's algebra.
#[test]
fn every_lantern_rim_carves_whole_to_its_closed_form() {
    let r = 0.05;
    let source = lantern();
    for (name, rim_r, rim_y, center) in rims() {
        let arcs = rim_arcs(&source, rim_r, rim_y);
        assert_eq!(arcs.len(), 2, "{name} arrives as two arcs");
        let out = fillet_edges(&source, &arcs, r, band(), tol())
            .unwrap_or_else(|e| panic!("{name} fillets whole, got {e:?}"));
        validate_geometric(&out.body, tol())
            .unwrap_or_else(|e| panic!("{name} must carve tier-3 valid, got {e:?}"));
        assert_eq!(out.band_faces.len(), 1, "{name} leaves ONE band");
        let (tc, major, minor) = band_torus(&out.body, out.band_faces[0]);
        let (want_x, want_y) = center(r);
        assert!(
            (major - want_x).abs() < 1e-12,
            "{name}: the spine radius is {want_x}, got {major}"
        );
        assert!(
            (tc.y - want_y).abs() < 1e-12,
            "{name}: the spine station is {want_y}, got {}",
            tc.y
        );
        assert!(
            (minor - r).abs() < 1e-15,
            "{name}: the tube radius is {r}, got {minor}"
        );
        assert!(
            tc.x.abs() < 1e-15 && tc.z.abs() < 1e-15,
            "{name}: the spine is on the axis, got {tc:?}"
        );
    }
}

/// **The band is one more revolution wall, over BOTH arcs.**
///
/// One boundary cycle, no ring (a curved face must be ring-free), and
/// the cycle carries two trim arcs on each support plus ONE slit walked
/// twice. Two trim arcs per side rather than one closed circle is the
/// whole structural difference from a one-edge rim's band: the trim
/// circle is cut where the supports' seam meridians reach it.
#[test]
fn the_band_over_two_arcs_is_one_annulus_wall() {
    let source = lantern();
    let arcs = rim_arcs(&source, SHOULDER.0, SHOULDER.1);
    let out = fillet_edges(&source, &arcs, 0.05, band(), tol())
        .unwrap_or_else(|e| panic!("the shoulder fillets, got {e:?}"));
    let face = out.band_faces[0];
    let fd = out.body.get_face(face).unwrap();
    assert!(fd.rings.is_empty(), "a curved face carries no ring");
    let topo::LoopBoundary::Cycle { first } = out.body.get_loop(fd.outer).unwrap().boundary else {
        panic!("the band's boundary is a cycle")
    };
    let cycle = out.body.loop_cycle(first).unwrap();
    let mut edges: Vec<EdgeKey> = cycle
        .iter()
        .map(|he| out.body.get_half_edge(*he).unwrap().edge)
        .collect();
    assert_eq!(edges.len(), 6, "four trim arcs and one slit walked twice");
    edges.sort_unstable();
    edges.dedup();
    assert_eq!(edges.len(), 5, "five distinct edges, one of them the slit");
    // The slit is the one the cycle walks twice, and it is the only
    // edge of the band whose two sides are the band itself.
    let slits: Vec<EdgeKey> = edges
        .iter()
        .copied()
        .filter(|e| {
            let (a, b) = faces_of(&out.body, *e);
            a == face && b == face
        })
        .collect();
    assert_eq!(slits.len(), 1, "exactly one slit");
}

/// **Both supports are several FACES of one SURFACE.** That is the
/// resolution the door rests on, and #319's own finding: a pole-touching
/// revolve splits the base DISK into two half-faces, so the planar
/// support of the neck rim's two arcs is two different faces of one
/// plane.
#[test]
fn each_side_of_a_seam_split_rim_is_two_faces_of_one_surface() {
    let source = lantern();
    for (name, rim_r, rim_y, _) in rims() {
        let arcs = rim_arcs(&source, rim_r, rim_y);
        assert_eq!(arcs.len(), 2, "{name} arrives as two arcs");
        let (a0, b0) = faces_of(&source, arcs[0]);
        let (a1, b1) = faces_of(&source, arcs[1]);
        assert_eq!(
            [surface_of(&source, a0), surface_of(&source, b0)],
            [surface_of(&source, a1), surface_of(&source, b1)],
            "{name}: the rim arrives and leaves on ONE support pair"
        );
        assert!(
            a0 != a1 && b0 != b1,
            "{name}: each side of the rim is TWO faces"
        );
    }
}

/// **The seam-split carve removes exactly what the one-edge carve
/// removes.** The bored twin is the same solid outside its bore, and
/// every trim circle below sits well outside it, so filleting the same
/// rim takes the same material out of the same region — through the
/// door that was already there in the twin's case, and through this
/// unit's in the lantern's.
#[test]
fn a_seam_split_rim_removes_what_its_one_edge_twin_removes() {
    let r = 0.05;
    let (lantern, bored) = (lantern(), bored_lantern());
    let (v_lantern, v_bored) = (volume(&lantern), volume(&bored));
    for (name, rim_r, rim_y, _) in rims() {
        let split = rim_arcs(&lantern, rim_r, rim_y);
        assert_eq!(split.len(), 2, "{name} is seam-split on the lantern");
        let whole = rim_arcs(&bored, rim_r, rim_y);
        assert_eq!(whole.len(), 1, "{name} is ONE edge on the bored twin");

        let cut_split = v_lantern
            - volume(
                &fillet_edges(&lantern, &split, r, band(), tol())
                    .unwrap_or_else(|e| panic!("{name} fillets on the lantern, got {e:?}"))
                    .body,
            );
        let cut_whole = v_bored
            - volume(
                &fillet_edges(&bored, &whole, r, band(), tol())
                    .unwrap_or_else(|e| panic!("{name} fillets on the twin, got {e:?}"))
                    .body,
            );
        assert!(
            cut_split > 0.0,
            "{name}: a convex fillet removes material, got {cut_split}"
        );
        assert!(
            (cut_split - cut_whole).abs() < 1e-12,
            "{name}: the seam-split carve removes {cut_split}, the one-edge carve \
             {cut_whole}"
        );
    }
}

/// **The three rims compose.** Two rims of one request share a wall, so
/// they are filleted in SEQUENTIAL calls — each on the last one's
/// result, which is the recourse the shared-support gate names — and
/// the lantern comes out with three bands and tier-3 valid.
#[test]
fn the_three_rims_fillet_in_sequence_to_one_valid_solid() {
    let r = 0.05;
    let mut body = lantern();
    let mut bands = 0;
    for (name, rim_r, rim_y, _) in rims() {
        let arcs = rim_arcs(&body, rim_r, rim_y);
        assert_eq!(arcs.len(), 2, "{name} is still two arcs before its carve");
        let out = fillet_edges(&body, &arcs, r, band(), tol())
            .unwrap_or_else(|e| panic!("{name} fillets on the running result, got {e:?}"));
        bands += out.band_faces.len();
        body = out.body;
    }
    assert_eq!(bands, 3, "one band per rim");
    validate_geometric(&body, tol())
        .unwrap_or_else(|e| panic!("the thrice-filleted lantern is tier-3 valid, got {e:?}"));
    let props = mass_properties(&body, tol()).expect("mass properties must compute");
    assert_eq!(props.volume_pad, 0.0, "every face stays closed-form");
    assert!(
        props.volume < volume(&lantern()),
        "three convex fillets remove material"
    );
}

/// **Both material configurations, at the level the arms have both.**
///
/// Each of the lantern's three support pairs puts the rolling ball
/// where its own two equations say — `dist(c, support) = −r·σ` in each
/// support's own closed form — with each support's stored sense bit set
/// either way. A combination for which NO such ball exists must POISON
/// rather than answer, and which combinations those are is decided here
/// independently: a straight-and-round pair has a ball exactly when the
/// offset line meets the offset circle.
///
/// The CARVE takes the convex configuration only (a concave chain adds
/// material, which no closed-rim carve in the module builds), which is
/// why the row below this one exists.
#[test]
fn the_lanterns_arms_fold_both_sense_bits() {
    let r = 0.05;
    let (nx, ny) = cone_normal();
    let origin = Point3::new(0.0, 0.0, 0.0);
    let cone_n = Vec3::new(nx, ny, 0.0);
    let base_n = Vec3::new(0.0, -1.0, 0.0);
    let top_n = Vec3::new(0.0, 1.0, 0.0);
    let shoulder = Point3::new(SHOULDER.0, SHOULDER.1, 0.0);
    let sheet_at = |p: Point3<f64>| Meridian {
        origin,
        axis: Vec3::new(0.0, 1.0, 0.0),
        rim: p,
    };
    let sphere = |side: f64| SupportTrace::Round {
        center: origin,
        radius: SPHERE_R,
        side,
    };
    let flat = |normal: Vec3<f64>| move |side: f64| SupportTrace::Straight { normal, side };
    // Each support's own signed distance, positive on its chart
    // normal's side — written here, not read from the kernel.
    let plane_dist = move |p: Point3<f64>, n: Vec3<f64>, o: Point3<f64>| (p - o).dot(n);
    let sphere_dist = move |p: Point3<f64>| (p - origin).norm() - SPHERE_R;
    // A straight-and-round pair has a rest centre exactly when the line
    // offset by `−r·σ_straight` meets the circle of radius `R − r·σ_round`.
    let meets = move |rim: Point3<f64>, n: Vec3<f64>, straight: f64, round: f64| {
        let offset_radius = SPHERE_R - r * round;
        offset_radius > 0.0 && ((origin - rim).dot(n) + r * straight).abs() <= offset_radius
    };

    let rows: [ArmRow<'_>; 3] = [
        (
            "plane×sphere",
            Point3::new(1.0, 0.0, 0.0),
            Box::new(flat(base_n)),
            Box::new(sphere),
            Box::new(move |p| plane_dist(p, base_n, origin)),
            Box::new(sphere_dist),
            Box::new(move |sa, sb| meets(Point3::new(1.0, 0.0, 0.0), base_n, sa, sb)),
        ),
        (
            "sphere×cone",
            shoulder,
            Box::new(sphere),
            Box::new(flat(cone_n)),
            Box::new(sphere_dist),
            Box::new(move |p| plane_dist(p, cone_n, shoulder)),
            Box::new(move |sa, sb| meets(shoulder, cone_n, sb, sa)),
        ),
        (
            "cone×plane",
            Point3::new(LIP_R, TOP, 0.0),
            Box::new(flat(cone_n)),
            Box::new(flat(top_n)),
            Box::new(move |p| plane_dist(p, cone_n, shoulder)),
            Box::new(move |p| plane_dist(p, top_n, Point3::new(0.0, TOP, 0.0))),
            // Two straight traces meet unless they are parallel, and
            // the cone is at 45° to the top plane.
            Box::new(|_, _| true),
        ),
    ];
    for (name, rim, ta, tb, da, db, feasible) in rows {
        let sheet = sheet_at(rim);
        let mut folded = 0;
        for (sa, sb) in [(1.0, 1.0), (-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0)] {
            let c = sheet_center(sheet.rim, sheet.sheet_normal(), ta(sa), tb(sb), r);
            if !feasible(sa, sb) {
                assert!(
                    !(c.x.is_finite() && c.y.is_finite() && c.z.is_finite()),
                    "{name} at senses ({sa}, {sb}): no ball rests there, so the arm must \
                     poison rather than answer {c:?}"
                );
                continue;
            }
            folded += 1;
            for (d, side) in [(da(c), sa), (db(c), sb)] {
                assert!(
                    (d + r * side).abs() < 1e-12,
                    "{name} at senses ({sa}, {sb}): the centre is {d} from a support, \
                     wanted {}",
                    -r * side
                );
            }
        }
        assert!(
            folded >= 2,
            "{name}: at least two configurations have a ball, so the row is not vacuous"
        );
    }
}

/// **The concave gate stands on a seam-split rim too.** The door this
/// unit builds widens which CHAIN SHAPES the annulus takes, not which
/// material configuration it carves: a concave rim adds material, and
/// the surgery still refuses it — whole, and with the same detail a
/// one-edge concave rim gets.
#[test]
fn a_concave_seam_split_rim_still_refuses() {
    // A waisted lantern: two cones meeting at radius 0.5, so the waist
    // rim is CONCAVE, and pole-touching, so it is seam-split.
    let body = revolved_about_y(
        vec![
            v(0.0, 0.0, 0.0),
            v(1.0, 0.0, 0.0),
            v(0.5, 0.5, 0.0),
            v(1.0, 1.0, 0.0),
            v(0.0, 1.0, 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    let arcs = rim_arcs(&body, 0.5, 0.5);
    assert_eq!(arcs.len(), 2, "the waist rim is seam-split too");
    match fillet_edges(&body, &arcs, 0.05, band(), tol()) {
        Err(FilletError::UnsupportedChain { detail, .. }) => assert!(
            detail.contains("concave"),
            "a concave seam-split rim refuses as concave, got {detail}"
        ),
        other => panic!("a concave seam-split rim refuses, got {other:?}"),
    }
}

/// **The naming totality, on a seam-split band.** Every output entity
/// is a recorded mint or a survivor of the source, and every retirement
/// names a SOURCE key — checked in both directions, which is what makes
/// the conditional dead-edge pushes evidence rather than hope.
#[test]
fn a_seam_split_band_records_every_birth_and_every_death() {
    let source = lantern();
    let arcs = rim_arcs(&source, SHOULDER.0, SHOULDER.1);
    let out = fillet_edges(&source, &arcs, 0.05, band(), tol())
        .unwrap_or_else(|e| panic!("the shoulder fillets, got {e:?}"));
    let rec = out
        .naming
        .as_ref()
        .expect("the rim phase records its births");

    let minted_edges: Vec<EdgeKey> = rec
        .rim_trims
        .iter()
        .map(|(e, _, _)| *e)
        .chain(rec.meridian_remnants.iter().map(|(e, _)| *e))
        .chain(rec.slits.iter().map(|(e, _)| *e))
        .collect();
    for (k, _) in out.body.edges() {
        assert!(
            minted_edges.contains(&k) || source.get_edge(k).is_some(),
            "output edge {k:?} is neither minted nor a survivor"
        );
    }
    for e in &rec.dead.edges {
        assert!(
            source.get_edge(*e).is_some(),
            "a retirement names a source edge, got {e:?}"
        );
        assert!(
            !out.body.edges().any(|(k, _)| k == *e) || minted_edges.contains(e),
            "a retired edge does not survive: {e:?}"
        );
    }
    for v in &rec.dead.vertices {
        assert!(
            source.get_vertex(*v).is_some(),
            "a retirement names a source vertex, got {v:?}"
        );
        assert!(
            !out.body.vertices().any(|(k, _)| k == *v),
            "a retired vertex does not survive: {v:?}"
        );
    }
    // The two seam vertices are exactly what this rim retired, beside
    // its two arcs.
    assert_eq!(
        rec.dead.vertices.len(),
        2,
        "both seam vertices are retired, and nothing else is"
    );
    let mut banded: Vec<EdgeKey> = rec
        .bands
        .iter()
        .flat_map(|(_, edges)| edges.iter().copied())
        .collect();
    banded.sort_unstable();
    let mut want = arcs.clone();
    want.sort_unstable();
    assert_eq!(banded, want, "the band names both arcs it replaces");
}
