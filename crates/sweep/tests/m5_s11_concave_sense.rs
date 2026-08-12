//! M5 S11 acceptance: constructors mint the honest orientation bit.
//!
//! S10 made a face's outward normal `sense_sign · chart_normal` and
//! proved the consumers read the bit; S11 makes the CONSTRUCTORS write
//! it honestly. A swept wall whose material lies against its surface's
//! chart normal — extrude's concave arc walls and their hole-loop kin,
//! a revolve's bore cylinder, inward cone, under-side plane annulus,
//! concave sphere/torus band — now mints `sense: false`, decided from
//! the profile's STORED winding/turn structure (material-left under
//! the canonical winding), never from sampled normals.
//!
//! This file is the constructor audit, wall kind by wall kind: for
//! each fixture it pins the exact sense pattern, tier-3 validity, and
//! an ANALYTIC volume (the bit is Category B for props — flux comes
//! from windings — so an honest volume here proves the fixed bodies
//! stay coherent end-to-end); door rows re-check `point_in_solid`
//! where PR 9c doors exist (plane/cylinder/sphere). The e2e halves
//! that need other crates live next to their consumers:
//! tessellation in `crates/mesh/tests/m5_s11_concave_sense.rs`, STEP
//! `same_sense = .F.` in `crates/step-export/tests/m5_s11_same_sense.rs`.
//!
//! **Tolerance shape**: structural (the S10 posture) — the bit is a
//! `bool` selected from stored `Sign`s; numeric assertions are against
//! analytic constants with generous absolute slack.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod revolve_common;

use core::f64::consts::{FRAC_PI_8, PI};
use profile::RawLoop;

use geom_core::{Band, Point3, Tolerance, Vec3};
use geom_surfaces::Surface;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use revolve_common::{assert_all_tiers, axis_y, p2, validated};
use sweep::{Extrusion, Revolution, extrude, revolve};
use topo::boolean::{SolidContainment, point_in_solid};
use topo::{Body, FaceKey};

fn band() -> Band {
    Band::linear().unwrap()
}

fn vol(body: &Body<f64>) -> f64 {
    topo::props::mass_properties(body).unwrap().volume
}

fn sense_of(body: &Body<f64>, f: FaceKey) -> bool {
    body.get_face(f).unwrap().sense
}

/// The wall senses of one swept loop, in the constructor's own segment
/// order (`None` for on-axis segments).
fn wall_senses(body: &Body<f64>, walls: &[Option<FaceKey>]) -> Vec<Option<bool>> {
    walls.iter().map(|w| w.map(|f| sense_of(body, f))).collect()
}

// =====================================================================
// Extrude: concave arc walls and hole walls.
// =====================================================================

/// The S10 mixed convex/concave fixture: a square whose bottom edge
/// bows OUT (convex arc) and whose top edge bows IN (concave arc).
/// Both arcs are quarter circles of radius √2, so the two circular
/// segments have equal area and the region's area is EXACTLY 3.
fn notched() -> sweep::Extruded<f64> {
    let b = FRAC_PI_8.tan();
    let lp = ProfileLoop::builder(p2(0.0, 0.0))
        .arc_to(p2(2.0, 0.0), b)
        .line_to(p2(2.0, 1.5))
        .arc_to(p2(0.0, 1.5), -b)
        .close();
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    extrude(&vp, Extrusion::Distance(1.0)).unwrap()
}

/// **The concave-notched body, end-to-end (this crate's half).** The
/// first genuinely mixed-sense body a constructor mints: tier-3 valid
/// (check 6 reads winding AGAINST the sense-signed outward normal, so
/// an incoherent bit would refuse), exact analytic volume through the
/// winding-derived flux, and the expected sense pattern — exactly one
/// `false` wall, the concave one.
#[test]
fn notched_body_validates_meters_and_carries_one_reversed_wall() {
    let t = notched();
    assert_all_tiers(&t.body);
    assert_eq!(topo::validate::validate_geometric(&t.body), Ok(()));
    assert!(
        (vol(&t.body) - 3.0).abs() < 1e-9,
        "the notch region's area is exactly 3 (equal-area segments), \
         so the prism meters 3.0; got {}",
        vol(&t.body)
    );
    // Segment order: convex arc, line, concave arc, line.
    let senses: Vec<bool> = t.side_faces[0]
        .iter()
        .map(|&f| sense_of(&t.body, f))
        .collect();
    assert_eq!(
        senses,
        vec![true, true, false, true],
        "exactly the concave wall is reversed"
    );
    // Caps stay true.
    assert!(sense_of(&t.body, t.top) && sense_of(&t.body, t.bottom));
}

/// Extruding the SAME profile along `−n` (the reversed traversal)
/// mints the SAME senses: concavity is a property of the 2-D region
/// against the carrier circle, so the swept reversal must not leak
/// into the bit (the criterion reads the CANONICAL turn).
#[test]
fn downward_extrusion_keeps_the_same_senses() {
    let b = FRAC_PI_8.tan();
    let lp = ProfileLoop::builder(p2(0.0, 0.0))
        .arc_to(p2(2.0, 0.0), b)
        .line_to(p2(2.0, 1.5))
        .arc_to(p2(0.0, 1.5), -b)
        .close();
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    let t = extrude(&vp, Extrusion::Vector(Vec3::new(0.0, 0.0, -1.0))).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(topo::validate::validate_geometric(&t.body), Ok(()));
    assert!((vol(&t.body) - 3.0).abs() < 1e-9);
    // side_faces is per canonical segment? No — per swept segment; the
    // reversal retraces the canonical chain backwards, so the concave
    // wall sits at the mirrored index. Count, don't index.
    let senses: Vec<bool> = t.side_faces[0]
        .iter()
        .map(|&f| sense_of(&t.body, f))
        .collect();
    assert_eq!(
        senses.iter().filter(|s| !**s).count(),
        1,
        "still exactly one reversed wall: {senses:?}"
    );
    // And it is the concave one (material outside its carrier).
    for &fk in &t.side_faces[0] {
        let sk = t.body.get_face(fk).unwrap().surface;
        if let Surface::Cylinder { origin, radius, .. } = *t.body.get_surface(sk).unwrap() {
            let d = geom_core::Point2::new(1.0, 0.75) - geom_core::Point2::new(origin.x, origin.y);
            assert_eq!(
                sense_of(&t.body, fk),
                d.norm() < radius,
                "sense must be true iff the material is inside the carrier"
            );
        }
    }
}

/// A hole wall IS a concave wall: the plate's material lies outside
/// the hole's carrier cylinder, so the canonical (clockwise) hole
/// winding mints `sense: false` — and the cylinder door now reads the
/// hole interior as void.
#[test]
fn hole_walls_mint_sense_false_and_the_door_reads_the_hole_as_void() {
    let outer = ProfileLoop::polygon([p2(0.0, 0.0), p2(4.0, 0.0), p2(4.0, 4.0), p2(0.0, 4.0)]);
    let hole = ProfileLoop::new(vec![
        ProfileVertex {
            pos: p2(1.0, 2.0),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: p2(3.0, 2.0),
            bulge: 1.0,
        },
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![outer, hole])
        .validate(Tolerance::get())
        .unwrap();
    let t = extrude(&vp, Extrusion::Distance(1.0)).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(topo::validate::validate_geometric(&t.body), Ok(()));
    assert!(
        (vol(&t.body) - (16.0 - PI)).abs() < 1e-9,
        "plate minus unit-radius hole: 16 − π; got {}",
        vol(&t.body)
    );
    // Loop 0 (outer): all planar, all true. Loop 1 (hole): both
    // half-cylinder walls reversed.
    for &f in &t.side_faces[0] {
        assert!(sense_of(&t.body, f), "outer plate walls stay true");
    }
    for &f in &t.side_faces[1] {
        assert!(!sense_of(&t.body, f), "hole walls are concave: false");
    }
    // Doors: hole interior is void, meat is material.
    let b = band();
    assert_eq!(
        point_in_solid(&t.body, Point3::new(2.0, 2.0, 0.5), b).unwrap(),
        SolidContainment::Out,
        "the hole's center is void"
    );
    assert_eq!(
        point_in_solid(&t.body, Point3::new(2.9, 2.0, 0.5), b).unwrap(),
        SolidContainment::Out,
        "near the hole wall, still void"
    );
    assert_eq!(
        point_in_solid(&t.body, Point3::new(0.5, 0.5, 0.5), b).unwrap(),
        SolidContainment::In
    );
}

// =====================================================================
// Revolve: the same audit, wall kind by wall kind.
// =====================================================================

/// The washer (rectangle `r ∈ [1, 2]`, `z ∈ [0, 1]` revolved fully):
/// the bore cylinder and the UNDER-side plane annulus have material
/// against their chart normals (the outward radial; `+a₃`).
///
/// Canonical CCW traversal: bottom `(1,0)→(2,0)` (Δr > 0 ⇒ plane
/// `false`), outer `(2,0)→(2,1)` (Δz > 0 ⇒ cylinder `true`), top
/// `(2,1)→(1,1)` (Δr < 0 ⇒ plane `true`), bore `(1,1)→(1,0)`
/// (Δz < 0 ⇒ cylinder `false`).
#[test]
fn washer_bore_and_under_annulus_mint_sense_false() {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Full).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(topo::validate::validate_geometric(&t.body), Ok(()));
    assert!(
        (vol(&t.body) - 3.0 * PI).abs() < 1e-9,
        "π(2² − 1²)·1 = 3π; got {}",
        vol(&t.body)
    );
    assert_eq!(
        wall_senses(&t.body, &t.walls[0]),
        vec![Some(false), Some(true), Some(true), Some(false)],
        "bottom annulus and bore reversed; outer wall and top true"
    );
    // Door probes live on the WEDGE fixture below: a FULL revolve's
    // wall trim spans the whole period, which `point_in_solid`'s
    // cosine-window lane refuses typed (`bool_wall_trim_period` — a
    // pre-existing, sense-independent limitation, still refused
    // loudly on the mixed-sense body rather than answered wrong).
    let b = band();
    match point_in_solid(&t.body, Point3::new(0.5, 0.5, 0.0), b) {
        Err(topo::boolean::PointInSolidError::Escalated { .. }) => {}
        other => panic!("full-period wall trim must refuse typed, got {other:?}"),
    }
}

/// The partial-revolve washer wedge (θ = π/2): the SAME wall senses
/// through the partial path (whose wedge caps are Newell-outward
/// planes and stay `true` — every cap face of the wedge).
#[test]
fn washer_wedge_partial_revolve_mints_the_same_wall_senses() {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
    )
    .unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(topo::validate::validate_geometric(&t.body), Ok(()));
    assert!(
        (vol(&t.body) - 3.0 * PI / 4.0).abs() < 1e-9,
        "a quarter of the washer: 3π/4; got {}",
        vol(&t.body)
    );
    assert_eq!(
        wall_senses(&t.body, &t.walls[0]),
        vec![Some(false), Some(true), Some(true), Some(false)],
    );
    // Every remaining face (wedge caps) stays true.
    let wall_set: Vec<FaceKey> = t.walls[0].iter().flatten().copied().collect();
    for (fk, f) in t.body.faces() {
        if !wall_set.contains(&fk) {
            assert!(f.sense, "wedge caps are Newell-outward: sense true");
        }
    }
    // Doors (plane + cylinder, quarter-period trims): probes at the
    // wedge's mid azimuth (θ > 0 sweeps toward −n, i.e. −z). The bore
    // and the shell's far side are void, the meat material — the bore
    // cylinder's reversed sense is what turns its radial around.
    let b = band();
    let c = core::f64::consts::FRAC_PI_4.cos(); // = sin(π/4)
    for (p, want) in [
        (Point3::new(0.5 * c, 0.5, -0.5 * c), SolidContainment::Out),
        (Point3::new(1.5 * c, 0.5, -1.5 * c), SolidContainment::In),
        (Point3::new(1.5 * c, -0.5, -1.5 * c), SolidContainment::Out),
        (Point3::new(1.5 * c, 1.5, -1.5 * c), SolidContainment::Out),
        (Point3::new(2.5 * c, 0.5, -2.5 * c), SolidContainment::Out),
    ] {
        assert_eq!(point_in_solid(&t.body, p, b).unwrap(), want, "probe {p:?}");
    }
}

/// The countersunk bushing: trapezoid `(0.5,0) (2,0) (2,1) (1,1)`
/// revolved fully. The oblique closing edge `(1,1)→(0.5,0)` descends
/// (canonical Δz < 0), so its CONE wall has material outside the
/// carrier: `sense: false` — the nappe-independent axial-sign
/// criterion.
#[test]
fn countersink_cone_wall_mints_sense_false() {
    let lp = ProfileLoop::polygon([p2(0.5, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Full).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(topo::validate::validate_geometric(&t.body), Ok(()));
    // Pappus: A = 1.25, x̄ = 41/30 ⇒ V = 2π·x̄·A = 10.25π/3.
    assert!(
        (vol(&t.body) - 10.25 * PI / 3.0).abs() < 1e-9,
        "cylinder minus conical frustum bore: 10.25π/3; got {}",
        vol(&t.body)
    );
    assert_eq!(
        wall_senses(&t.body, &t.walls[0]),
        vec![Some(false), Some(true), Some(true), Some(false)],
        "bottom annulus false; outer cylinder true; top annulus true; \
         countersink cone false"
    );
}

/// The dimpled piston: a cylinder (r = 1, h = 2) with a hemispherical
/// dimple (r = ½) sunk into its top center — the WIRE case (an
/// on-axis run) with a CONCAVE sphere band: the dimple arc turns
/// clockwise in the canonical traversal, so its sphere wall's material
/// lies outside the carrier and the sense is `false`.
#[test]
fn dimple_sphere_wall_mints_sense_false() {
    let b = FRAC_PI_8.tan();
    let lp = ProfileLoop::builder(p2(0.0, 0.0))
        .line_to(p2(1.0, 0.0))
        .line_to(p2(1.0, 2.0))
        .line_to(p2(0.5, 2.0))
        .arc_to(p2(0.0, 1.5), -b)
        .close();
    let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Full).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(topo::validate::validate_geometric(&t.body), Ok(()));
    assert!(
        (vol(&t.body) - (2.0 * PI - PI / 12.0)).abs() < 1e-9,
        "cylinder 2π minus half-ball π/12; got {}",
        vol(&t.body)
    );
    // Canonical segments: bottom disc (Δr > 0 ⇒ false), outer cylinder
    // (Δz > 0 ⇒ true), top annulus (Δr < 0 ⇒ true), dimple sphere
    // (clockwise ⇒ false), axis segment (no wall).
    assert_eq!(
        wall_senses(&t.body, &t.walls[0]),
        vec![Some(false), Some(true), Some(true), Some(false), None],
    );
    // No door probes here: the dimple bowl is a PARTIAL sphere band
    // (rimmed against the top annulus), which `point_in_solid`
    // refuses typed (`PartialSphereFace`, PR 9c — the trimmed-sphere
    // chart is future recourse; pre-existing and sense-independent).
    // Pin that it still refuses loudly rather than answering wrong on
    // the mixed-sense body.
    let b3 = band();
    match point_in_solid(&t.body, Point3::new(0.0, 1.8, 0.0), b3) {
        Err(topo::boolean::PointInSolidError::PartialSphereFace { .. }) => {}
        other => panic!("partial sphere band must refuse typed, got {other:?}"),
    }
}

/// The notched ring: the notched profile shifted off-axis
/// (`x ∈ [1, 3]`) and revolved fully — the LAMINA case, whose concave
/// arc sweeps a TORUS band with material outside the tube:
/// `sense: false` by the same canonical turn sign as extrude.
#[test]
fn notched_ring_torus_band_mints_sense_false() {
    let b = FRAC_PI_8.tan();
    let lp = ProfileLoop::builder(p2(1.0, 0.0))
        .arc_to(p2(3.0, 0.0), b)
        .line_to(p2(3.0, 1.5))
        .arc_to(p2(1.0, 1.5), -b)
        .close();
    let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Full).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(topo::validate::validate_geometric(&t.body), Ok(()));
    // Pappus: area exactly 3 (equal-area segments), centroid x̄ = 2
    // (both segments are symmetric about x = 2), V = 2π·2·3 = 12π.
    assert!(
        (vol(&t.body) - 12.0 * PI).abs() < 1e-9,
        "Pappus: 12π; got {}",
        vol(&t.body)
    );
    // Canonical segments: convex torus (true), outer cylinder (Δz > 0,
    // true), concave torus (false), bore cylinder (Δz < 0, false).
    assert_eq!(
        wall_senses(&t.body, &t.walls[0]),
        vec![Some(true), Some(true), Some(false), Some(false)],
    );
}
