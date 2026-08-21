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
//! **The third verb** (the loft section at the end) is the same audit
//! with the opposite verdict: loft's skinned chart follows the profile
//! traversal instead of an unconditional radial, so a concave arc and
//! a hole loop stay `sense: true` and are still honest. Its prism rows
//! loft THIS FILE'S fixtures — `notched_loops`, `holed_loops` — and
//! check the orientation against the extruded twin the rows above
//! verify, so the two chapters cannot drift apart.
//!
//! **Charts that turn** close that chapter. Those rows stack
//! IDENTICAL sections at `v_degree = 1`, which makes every wall a
//! generalized cylinder: its chart normal is the same vector at every
//! `v`, so no fixture among them can exercise the half of loft's
//! argument that is about `v`. (The criterion is identical sections
//! AND linear `v`, not the degree on its own — the flipping pair below
//! is a `v_degree = 1` section pair whose chart turns hard.) The rows
//! after them stack shapes that do turn: a pair whose convexity FLIPS
//! between the sections, a three-section curved-`v` dogleg, and
//! `sweep_body`'s quarter-turn elbow. None of those has an extruded
//! twin, so their oracle is the body's own LEVEL SETS — the walls'
//! iso-curves at one `v`, closed into a planar ring and decided by
//! crossing parity, which reads positions and no orientation bit at
//! all. That oracle is checked against the twin's door on the prisms,
//! where both are valid.
//!
//! **Tolerance shape**: structural (the S10 posture) — the bit is a
//! `bool` selected from stored `Sign`s; numeric assertions are against
//! analytic constants with generous absolute slack.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;
mod revolve_common;

use core::f64::consts::{FRAC_PI_2, FRAC_PI_8, PI};
use profile::RawLoop;

use common::orient::{
    along_v, assert_walls_face_out, loft_contains, wall_outward, wall_outward_at,
};
use common::quad;
use geom::Surface;
use geom_core::{Affine3, Band, Point3, Tolerance, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use revolve_common::{assert_all_tiers, axis_y, p2, validated};
use sweep::{
    Extrusion, Lofted, Revolution, Section, SketchSegment, extrude, loft_body, revolve,
    segment_curve, sweep_body,
};
use topo::boolean::{SolidContainment, point_in_solid};
use topo::{Body, FaceKey};
use geom_core::Tol;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn vol(body: &Body<f64>) -> f64 {
    topo::props::mass_properties(body, Tol::witness()).unwrap().volume
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

/// The S10 mixed convex/concave fixture, as PROFILE LOOPS: a square
/// whose bottom edge bows OUT (convex arc) and whose top edge bows IN
/// (concave arc). Both arcs are quarter circles of radius √2, so the
/// two circular segments have equal area and the region's area is
/// EXACTLY 3, with the concave notch floor at `y = 2.5 − √2`.
///
/// The single source of this region for the whole audit — extrude
/// rows, the loft rows below, and every extruded twin they compare
/// against all read it here.
fn notched_loops() -> Vec<ProfileLoop<f64>> {
    let b = FRAC_PI_8.tan();
    // Leaving bulges: the bottom arc bows out (+b), the top one bows
    // into the region (-b); the two sides are straight.
    vec![<ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), b),
        ProfileVertex::new(p2(2.0, 0.0), 0.0),
        ProfileVertex::new(p2(2.0, 1.5), -b),
        ProfileVertex::new(p2(0.0, 1.5), 0.0),
    ])]
}

/// The holed plate as PROFILE LOOPS: a 4×4 square with a unit-radius
/// circular hole cut by two half-arcs. The hole loop's canonical
/// winding is CLOCKWISE — the encoding that says the plate's material
/// lies OUTSIDE the hole's carrier.
fn holed_loops() -> Vec<ProfileLoop<f64>> {
    vec![
        ProfileLoop::polygon([p2(0.0, 0.0), p2(4.0, 0.0), p2(4.0, 4.0), p2(0.0, 4.0)]),
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(1.0, 2.0), 1.0),
            ProfileVertex::new(p2(3.0, 2.0), 1.0),
        ]),
    ]
}

fn notched() -> sweep::Extruded<f64> {
    extrude(&validated(notched_loops()), Extrusion::Distance(1.0), Tol::witness()).unwrap()
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
    assert_eq!(topo::validate::validate_geometric(&t.body, Tol::witness()), Ok(()));
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
    let t = extrude(
        &validated(notched_loops()),
        Extrusion::Vector(Vec3::new(0.0, 0.0, -1.0)),
        Tol::witness(),
    )
    .unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(topo::validate::validate_geometric(&t.body, Tol::witness()), Ok(()));
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
    let t = extrude(&validated(holed_loops()), Extrusion::Distance(1.0), Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(topo::validate::validate_geometric(&t.body, Tol::witness()), Ok(()));
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
        point_in_solid(&t.body, Point3::new(2.0, 2.0, 0.5), b, Tol::witness()).unwrap(),
        SolidContainment::Out,
        "the hole's center is void"
    );
    assert_eq!(
        point_in_solid(&t.body, Point3::new(2.9, 2.0, 0.5), b, Tol::witness()).unwrap(),
        SolidContainment::Out,
        "near the hole wall, still void"
    );
    assert_eq!(
        point_in_solid(&t.body, Point3::new(0.5, 0.5, 0.5), b, Tol::witness()).unwrap(),
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
    let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(topo::validate::validate_geometric(&t.body, Tol::witness()), Ok(()));
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
    match point_in_solid(&t.body, Point3::new(0.5, 0.5, 0.0), b, Tol::witness()) {
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
        Tol::witness(),
    )
    .unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(topo::validate::validate_geometric(&t.body, Tol::witness()), Ok(()));
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
        assert_eq!(point_in_solid(&t.body, p, b, Tol::witness()).unwrap(), want, "probe {p:?}");
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
    let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(topo::validate::validate_geometric(&t.body, Tol::witness()), Ok(()));
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
    // Leaving bulges: three straight legs, then the dimple arc (-b)
    // from (0.5, 2) to the axis; the closing leg is straight.
    let lp = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(1.0, 0.0), 0.0),
        ProfileVertex::new(p2(1.0, 2.0), 0.0),
        ProfileVertex::new(p2(0.5, 2.0), -b),
        ProfileVertex::new(p2(0.0, 1.5), 0.0),
    ]);
    let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(topo::validate::validate_geometric(&t.body, Tol::witness()), Ok(()));
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
    match point_in_solid(&t.body, Point3::new(0.0, 1.8, 0.0), b3, Tol::witness()) {
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
    // The notched profile shifted to x ∈ [1, 3]: same leaving bulges.
    let lp = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(p2(1.0, 0.0), b),
        ProfileVertex::new(p2(3.0, 0.0), 0.0),
        ProfileVertex::new(p2(3.0, 1.5), -b),
        ProfileVertex::new(p2(1.0, 1.5), 0.0),
    ]);
    let t = revolve(&validated(vec![lp]), axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(topo::validate::validate_geometric(&t.body, Tol::witness()), Ok(()));
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

// =====================================================================
// Loft: the third verb's chapter of the same audit.
//
// Extrude and revolve mint `sense: false` where the material lies
// against the chart normal. Loft mints `true` on EVERY wall and is
// still honest, because its chart is different: `u` follows the
// material-left profile traversal and `v` the stacking, so `S_u × S_v`
// is material-right whatever the segment's curvature — there is no
// unconditionally-radial frame to fight. That argument was pinned only
// on `loft_prism` (a prism: no concave arcs, no holes) until these
// rows, which run it on the two shapes that broke extrude.
//
// The oracle is this audit's own: each fixture is lofted as a SECTION
// PAIR of identical sections, so the body is the same solid `extrude`
// builds from the same loops, and the twin's analytic `point_in_solid`
// door — the one the extrude rows above verify — says which side is
// material.
// =====================================================================

/// The loft of `loops` stacked to height `h`: a section pair, `v`
/// linear, hence the extruded twin's solid.
fn loft_pair(loops: &[ProfileLoop<f64>], h: f64) -> Lofted<f64> {
    let sections: Vec<Section> = vec![loops.to_vec(), loops.to_vec()];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.0, 0.0, h)),
    ];
    loft_body::<f64>(&sections, &places, 1, Tol::witness()).expect("the section pair lofts")
}

fn extruded_twin(loops: &[ProfileLoop<f64>], h: f64) -> Body<f64> {
    extrude(&validated(loops.to_vec()), Extrusion::Distance(h), Tol::witness())
        .unwrap()
        .body
}

/// A rational body's volume, pinned across the ε schedule (the m8-3
/// posture, in the one form these rows need). The certified quadrature
/// runs a FIXED schedule against a `1024·ε` target, so the same body is
/// computable at the default ε and honestly out of budget at a tighter
/// one. Both are pinned and the target is never widened: a `Certified`
/// result must hit `want`, a `Budget` refusal must carry a width that
/// really missed a target that really is `1024·ε`, and only the
/// convergence predicate may escalate. Anything else is a real failure.
fn assert_rational_volume(row: &str, body: &Body<f64>, want: f64) {
    let target = 1024.0 * Tol::witness().get().eps;
    match topo::props::mass_properties(body, Tol::witness()) {
        Ok(m) => {
            // ACCURACY: the certified enclosure must CONTAIN the
            // independent oracle (`want` is the extruded twin's closed
            // form, pad exactly 0).
            assert!(
                (m.volume - want).abs() <= m.volume_pad,
                "{row}: the rational enclosure must contain the analytic \
                 volume: got {} ± {}, oracle {want}",
                m.volume,
                m.volume_pad
            );
            // PAD CEILING, pinned SEPARATELY: accuracy alone gets
            // easier as the enclosure degrades, so a loosening pad
            // cannot absorb it without tripping this. Keyed to ε
            // because the schedule is fixed and the pad is what the
            // schedule achieves.
            let ceiling = 2.0 * target;
            assert!(
                m.volume_pad.is_finite() && m.volume_pad < ceiling,
                "{row}: volume pad {} vs {ceiling:e}",
                m.volume_pad
            );
        }
        Err(topo::MassPropsError::Face {
            source:
                geom_brep::PropsError::QuadratureBudget {
                    width_len,
                    target_len,
                },
            ..
        }) => {
            assert!(
                width_len.is_finite() && width_len > target_len,
                "{row}: a budget refusal must carry a width that really \
                 missed: {width_len:e} vs {target_len:e}"
            );
            assert!(
                (target_len - target).abs() <= target * 1e-12,
                "{row}: the refused target must BE 1024·ε for this run: \
                 {target_len:e} vs {target:e}"
            );
        }
        Err(topo::MassPropsError::Face {
            source: geom_brep::PropsError::Escalated { cause },
            ..
        }) => assert_eq!(
            cause.predicate,
            Some("props_quad_converged"),
            "{row}: only the convergence predicate may escalate: {cause:?}"
        ),
        Err(other) => panic!("{row}: not an honest quadrature posture: {other}"),
    }
}

/// The extruded twin as an oracle — legitimate only where the loft IS
/// that prism: identical sections, `v` linear.
fn twin_oracle(twin: &Body<f64>) -> impl Fn(Point3<f64>) -> SolidContainment + '_ {
    |q| point_in_solid(twin, q, band(), Tol::witness()).expect("the twin's door decides")
}

/// The one sample a `v`-linear wall needs: `S_v` is constant along `v`
/// there, so mid-chart stands for every level set.
const MID_CHART: [(f64, f64); 1] = [(0.5, 0.5)];

/// **Loft's concave arc wall.** The shape that broke extrude, lofted:
/// each wall's sense-signed chart normal points out of the material as
/// the extruded twin's door reads it, the body meters its analytic
/// volume, and — the reason these rows carry the guard at all — an
/// inside-out wall is invisible to the tier ladder and to props.
///
/// One row, one build: the rational quadrature is this fixture's whole
/// cost and nextest is process-per-test, so the orientation, the volume
/// and the blindness residual are metered in one process rather than
/// paying that cost twice.
#[test]
fn loft_concave_arc_walls_face_out_and_a_flip_is_invisible_below() {
    let loops = notched_loops();
    let lofted = loft_pair(&loops, 1.0);
    let twin = extruded_twin(&loops, 1.0);
    assert_eq!(topo::validate(&lofted.body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&lofted.body), Ok(()), "tier 2");
    assert_walls_face_out(&lofted, &twin_oracle(&twin), &MID_CHART, 0.02, 4);
    assert!(
        sense_of(&lofted.body, lofted.top) && sense_of(&lofted.body, lofted.bottom),
        "the caps keep sense = true (the bottom cap's loop is reversed at mint)"
    );
    // The twin's flux is a closed form and holds at every ε.
    let twin_vol = vol(&twin);
    assert!(
        (twin_vol - 3.0).abs() < 1e-9,
        "the twin's analytic flux is the closed form: 3.0; got {twin_vol}"
    );
    // The loft's arc walls are RATIONAL, so its flux is the certified
    // quadrature against a FIXED schedule: honest at the default ε,
    // honestly out of budget at a tighter one (the m8-3 posture). Both
    // outcomes are pinned; neither target is ever widened.
    assert_rational_volume("the lofted concave prism", &lofted.body, 3.0);

    // The residual. Segment 2 is the concave arc — the wall whose
    // extruded twin mints `sense: false` — pinned by its mid-chart
    // sample so the row cannot drift onto a planar wall.
    let wall = lofted.side_faces[0][2];
    let (p, _) = wall_outward(&lofted.body, wall);
    assert!(
        (p.y - (2.5 - 2.0_f64.sqrt())).abs() < 1e-9,
        "the flipped wall is the concave arc, sampled on the notch floor \
         y = 2.5 − √2; got {p:?}"
    );
    let lied = lofted.body.flipped_face_sense_for_tests(wall).unwrap();
    assert_eq!(topo::validate(&lied), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&lied), Ok(()), "tier 2");
    // The recorded residual, stated as what it is: a CHANGE DETECTOR,
    // not a guarantee. Check 6's curved arm skips `Surface::Nurbs` by
    // name and its planar arm is line-bounded, so an orientation error
    // on a lofted wall is unreachable today and this assertion cannot
    // fail — which is the point. It fires the day that skip is
    // removed, i.e. exactly when the guard should move out of these
    // rows and into the validator where it belongs.
    //
    // Phrased over the whole report, not `== Ok(())`, so a tighter ε
    // that makes the rational flux honestly out of budget leaves the
    // claim intact.
    let report = topo::validate_geometric(&lied, Tol::witness());
    let orientation_errors = report
        .as_ref()
        .err()
        .into_iter()
        .flatten()
        .filter(|e| {
            matches!(
                e,
                topo::ValidationError::LoopRoleInverted { .. }
                    | topo::ValidationError::CurvedSenseInverted { .. }
            )
        })
        .count();
    assert_eq!(
        orientation_errors, 0,
        "no tier reads a lofted wall's sense, so an inside-out wall draws \
         no orientation complaint: {report:?}"
    );
}

/// **Loft's hole walls.** A hole loop's walls are concave walls — the
/// plate's material lies outside their carrier, which the clockwise
/// canonical winding encodes. Extrude mints `sense: false` there (the
/// row above); loft mints `true`, and this is the half that proves it.
///
/// Tiers 1 and 2 only: the rational hole walls miss `1024·ε` at this
/// scale, and tier 3 reads no lofted wall's sense anyway (the row
/// above), so a tier-3 assertion here would guard nothing.
#[test]
fn loft_hole_walls_face_out_of_the_plate() {
    let loops = holed_loops();
    let lofted = loft_pair(&loops, 1.0);
    assert_eq!(topo::validate(&lofted.body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&lofted.body), Ok(()), "tier 2");
    let twin = extruded_twin(&loops, 1.0);
    assert_walls_face_out(&lofted, &twin_oracle(&twin), &MID_CHART, 0.05, 6);
}

/// **A tapered loft agrees with the verified prism.** Every row above
/// stacks identical sections, which is what makes the extruded twin an
/// exact oracle — and is also the prism shape the finding objects to.
/// This row lofts a genuinely tapered concave-arc pair (top section
/// scaled 0.8) and requires each wall to point the same way.
///
/// It is a CONSISTENCY row, not a correctness one: its oracle is the
/// prism's own bits, so were those wrong the taper would agree with
/// them at `cos ≈ 1`. What it rules out is the taper introducing a
/// reversal the prism cannot show. Scaling about the sketch origin
/// displaces a wall at radius `d` by `0.2·d` over unit height and the
/// farthest wall sits at `d = 2.5`, so the honest tilt is at most
/// `atan(0.5) ≈ 26.6°`; the 45° threshold clears that and sits far from
/// the 180° a flip produces.
#[test]
fn a_tapered_concave_loft_keeps_the_prism_wall_directions() {
    let prism = loft_pair(&notched_loops(), 1.0);
    let b = FRAC_PI_8.tan();
    let scaled: Vec<ProfileLoop<f64>> = vec![<ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), b),
        ProfileVertex::new(p2(1.6, 0.0), 0.0),
        ProfileVertex::new(p2(1.6, 1.2), -b),
        ProfileVertex::new(p2(0.0, 1.2), 0.0),
    ])];
    let sections: Vec<Section> = vec![notched_loops(), scaled];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
    ];
    let tapered = loft_body::<f64>(&sections, &places, 1, Tol::witness()).expect("the tapered pair lofts");
    assert_eq!(topo::validate(&tapered.body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&tapered.body), Ok(()), "tier 2");
    for (li, walls) in tapered.side_faces.iter().enumerate() {
        for (si, &fk) in walls.iter().enumerate() {
            let (_, taper_n) = wall_outward(&tapered.body, fk);
            let (_, prism_n) = wall_outward(&prism.body, prism.side_faces[li][si]);
            let dot = taper_n.dot(prism_n);
            assert!(
                dot > core::f64::consts::FRAC_PI_4.cos(),
                "loop {li} segment {si}: the taper may tilt the wall, not \
                 reverse it; cos = {dot}"
            );
            assert!(sense_of(&tapered.body, fk), "loop {li} segment {si}");
        }
    }
}

/// **The S11 union check cannot reach a lofted operand.** S11's witness
/// was `union(notched, pellet)` on two disjoint solids: no surface
/// intersections, so the operation asks `point_in_solid`, and a wrong
/// `sense` on the concave wall made it answer `In` and swallow the
/// pellet. On a LOFTED operand both doors refuse instead — the boolean
/// layer rejects a rung-3 NURBS operand at admission, and
/// `point_in_solid` has no NURBS door at all.
///
/// So the swallow is closed for loft by REFUSAL, not by the bit, which
/// is why the material-side probes above carry the guard. The extrude
/// half of the witness is `m5_s10_face_sense`'s
/// `fixed_union_keeps_a_pellet_in_a_concave_notch`, in this binary; it
/// is not re-run here.
#[test]
fn a_lofted_operand_refuses_the_union_check_typed() {
    let lofted = loft_pair(&notched_loops(), 1.0);
    // The S11 pellet: strictly inside the concave notch (the floor at
    // x = 1 is y = 2.5 − √2 ≈ 1.0858), so the two solids are disjoint.
    let lp = <ProfileLoop<f64> as RawLoop<f64>>::polygon([
        p2(0.9, 1.25),
        p2(1.1, 1.25),
        p2(1.1, 1.35),
        p2(0.9, 1.35),
    ]);
    let plane = SketchPlane::from_frame(
        Point3::new(0.0, 0.0, 0.3),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    let vp = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let pellet = extrude(&vp, Extrusion::Distance(0.4), Tol::witness()).unwrap().body;

    let err = match topo::boolean::union(&lofted.body, &pellet, Tol::witness()) {
        Err(e) => e.to_string(),
        Ok(_) => panic!("a rung-3 NURBS operand has no boolean layer yet"),
    };
    assert!(
        err.contains("rung-3"),
        "the refusal names the operand: {err}"
    );
    let door = point_in_solid(&lofted.body, Point3::new(1.0, 1.3, 0.5), band(), Tol::witness());
    assert!(
        door.is_err(),
        "point_in_solid has no NURBS door: it must refuse, not guess: {door:?}"
    );
}

// =====================================================================
// The third verb, `v` direction: charts that can TWIST.
//
// Every loft row above stacks IDENTICAL sections at `v_degree = 1`.
// That combination — not the degree on its own — makes each wall a
// generalized cylinder, whose chart normal is one fixed vector for
// every `v`; and a chart that cannot turn cannot exhibit the failure
// loft's argument (the skinned chart's normal FOLLOWS the traversal)
// rules out. These rows loft shapes whose charts do turn: a pair whose
// convexity FLIPS between the sections — still a `v_degree = 1` pair,
// which is why the degree is not the criterion — and a three-section
// curved-`v` stack at `v_degree = 2`, the degree 27 of the 28 other
// `loft_body` call sites in the tree use.
//
// Neither shape has an extruded twin, because the sections differ, so
// the material side comes from the body's OWN level sets: the walls'
// iso-curves at one `v`, closed into a planar polyline per loop and
// decided by crossing parity. That reads positions only — no `sense`,
// no winding, nothing these rows are testing — and it is checked
// against #619's twin oracle on the prism, where both are valid.
// =====================================================================

/// The convexity-FLIPPING section: the audit's rectangle with both the
/// bottom and the top edge bowed, the bulge signs taken from `sign`.
/// `flipping_loops(1.0)` bows the bottom edge INTO the region and the
/// top edge out of it; `flipping_loops(-1.0)` does the reverse. The
/// bulges are the audit's own quarter circles, so either section's
/// area is exactly 3 — equal segments added and removed.
///
/// Lofting one onto the other gives S51's pointed shape: every wall's
/// convexity flips between the sections, so its chart passes through a
/// flat at mid-height. That is the one configuration where a
/// traversal-following chart could plausibly twist.
fn flipping_loops(sign: f64) -> Vec<ProfileLoop<f64>> {
    let b = FRAC_PI_8.tan() * sign;
    vec![<ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), -b),
        ProfileVertex::new(p2(2.0, 0.0), 0.0),
        ProfileVertex::new(p2(2.0, 1.5), b),
        ProfileVertex::new(p2(0.0, 1.5), 0.0),
    ])]
}

/// **A wall whose convexity flips along `v`.** The concave arc below,
/// the convex one above, and the flat in between — the chart S42's
/// objection reaches and #619's rows do not. Every wall's sense-signed
/// `S_u × S_v` must have material against it and void along it at
/// every sample of the chart, not merely at its centre.
///
/// The oracle is the body's own level sets rather than an extruded
/// twin, which does not exist for a shape whose sections differ.
#[test]
fn a_convexity_flipping_loft_faces_out_at_every_level() {
    let sections: Vec<Section> = vec![flipping_loops(1.0), flipping_loops(-1.0)];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
    ];
    let lofted =
        loft_body::<f64>(&sections, &places, 1, Tol::witness()).expect("the convexity-flipping pair lofts");
    assert_eq!(topo::validate(&lofted.body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&lofted.body), Ok(()), "tier 2");

    // ANTI-VACUITY: the fixture must really flip, or the row is the
    // prism again. Segment 0 is the bottom edge, whose corners are
    // fixed at y = 0, so its mid-`u` height IS its signed bow: the
    // quarter circle's sagitta √2 − 1 into the region below, out of it
    // above, and zero at the level where the wall is flat.
    let seg = lofted.side_faces[0][0];
    let bow = |t: f64| wall_outward_at(&lofted.body, seg, 0.5, t).0.y;
    let sag = 2.0_f64.sqrt() - 1.0;
    assert!(
        (bow(0.0) - sag).abs() < 1e-9 && (bow(1.0) + sag).abs() < 1e-9,
        "the wall must be concave below and convex above: {} then {}",
        bow(0.0),
        bow(1.0)
    );
    assert!(
        bow(0.5).abs() < 1e-9,
        "the flip passes through a flat at mid-height: {}",
        bow(0.5)
    );

    let samples = along_v();
    let oracle = |q| loft_contains(&lofted, q);
    assert_walls_face_out(&lofted, &oracle, &samples, 0.02, 4);
}

/// **A three-section curved-`v` loft.** The notched region stacked
/// through a dogleg — the middle section displaced sideways, skinned
/// at `v_degree = 2` — so `S_v` turns along `v` and the wall is a
/// genuinely curved chart rather than a ruling. Same claim, same
/// oracle, over the whole chart.
#[test]
fn a_three_section_curved_v_loft_faces_out_at_every_level() {
    let sections: Vec<Section> = vec![notched_loops(), notched_loops(), notched_loops()];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.35, 0.25, 0.5)),
        Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
    ];
    let lofted = loft_body::<f64>(&sections, &places, 2, Tol::witness()).expect("the dogleg lofts");
    assert_eq!(topo::validate(&lofted.body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&lofted.body), Ok(()), "tier 2");

    // ANTI-VACUITY: `v` must really turn. On a straight side wall the
    // chart normal is carried by the stacking, so the dogleg tilts it
    // one way over the lower half and the other way over the upper —
    // a ruled `v` would hold it fixed and this would read cos = 1.
    let side = lofted.side_faces[0][1];
    let n0 = wall_outward_at(&lofted.body, side, 0.5, 0.05).1;
    let n1 = wall_outward_at(&lofted.body, side, 0.5, 0.95).1;
    assert!(
        n0.dot(n1) < 0.9,
        "the dogleg must turn the chart normal along v: cos = {}",
        n0.dot(n1)
    );

    let samples = along_v();
    let oracle = |q| loft_contains(&lofted, q);
    assert_walls_face_out(&lofted, &oracle, &samples, 0.02, 4);
}

/// **The level-set oracle agrees with the extruded twin.** The two
/// rows above answer the material-side question from the loft's own
/// level sets; this one runs that oracle and #619's — the extruded
/// twin's `point_in_solid` door, which the extrude chapter above
/// verifies — over a grid on both prism fixtures, so the twisting rows
/// do not rest on machinery nothing checks. The holed plate is the
/// half that exercises crossing parity: its hole must read void.
///
/// The grid offsets are deliberately off every round coordinate, so no
/// sample sits on a boundary where the door answers `On` and parity
/// answers arbitrarily.
#[test]
fn the_level_set_oracle_agrees_with_the_extruded_twin() {
    for (loops, span) in [(notched_loops(), 3.0), (holed_loops(), 5.0)] {
        let lofted = loft_pair(&loops, 1.0);
        let twin = extruded_twin(&loops, 1.0);
        let door = twin_oracle(&twin);
        let (mut inside, mut outside) = (0usize, 0usize);
        for i in 0..7 {
            for j in 0..7 {
                for k in 0..3 {
                    #[allow(clippy::cast_precision_loss)]
                    let q = Point3::new(
                        -0.61 + span * f64::from(i) / 6.0,
                        -0.59 + span * f64::from(j) / 6.0,
                        0.13 + 0.37 * f64::from(k),
                    );
                    let want = door(q);
                    assert_eq!(
                        loft_contains(&lofted, q),
                        want,
                        "the level-set oracle and the twin's door must agree at {q:?}"
                    );
                    match want {
                        SolidContainment::In => inside += 1,
                        _ => outside += 1,
                    }
                }
            }
        }
        assert!(
            inside >= 20 && outside >= 20,
            "the grid must straddle the boundary to compare anything: \
             {inside} in, {outside} out"
        );
    }
}

/// **The same chart, swept along a curved path.** `sweep_body` is
/// `loft_body`'s assembly over `sweep_places`' frames, so it mints
/// wall senses by the same traversal argument, and its charts turn
/// where a loft's need not: the elbow's stations rotate a quarter turn
/// about the path's axis, so one wall's `S_v` sweeps 90°. Nothing
/// anywhere asked whether a swept wall faces out; this row asks it
/// with the probe and the oracle the loft rows above use.
///
/// The fixture is `m7_skin_integral`'s elbow — a square section of
/// half-width ¼ swept along a quarter circle of radius 3 through nine
/// stations at `v_degree = 3`. It is re-typed rather than shared: that
/// suite is a skin-integrality pin in another file, and `all.rs`
/// requires each aggregated file to behave as it did when it was its
/// own crate root.
///
/// # This does NOT close the class for `sweep_body`
///
/// The elbow is one curved-path caller of several, and it is the
/// EASIEST. `m8_14_long_turn_sweep.rs` sweeps helices at ½, 1 and 2
/// turns — a non-planar path with genuine frame roll, near-antipodal
/// by that suite's own header — and those are where "the chart normal
/// follows the traversal" carries the most weight. They stay unpinned,
/// and this oracle **cannot** reach them: its level planes fan, so a
/// stack that turns past a right angle stops being bisectable, and on
/// a half-turn helix `level_plane` refuses at both ends
/// (`cos ≈ 0.011`) rather than answering. Reaching a helix needs a
/// different oracle, not a wider bound here.
#[test]
fn a_curved_path_swept_body_faces_out_along_the_whole_turn() {
    let place = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        -FRAC_PI_2,
    );
    let path = segment_curve(
        0,
        SketchSegment::Arc {
            a: p2(0.0, 0.0),
            b: p2(3.0, 3.0),
            bulge: FRAC_PI_8.tan(),
        },
        place,
    )
    .expect("the elbow path is a well-formed quarter arc");
    let h = 0.25;
    let profile = quad([(-h, -h), (h, -h), (h, h), (-h, h)]);
    let swept =
        sweep_body::<f64>(&profile, Affine3::identity(), &path, 9, 3, Tol::witness()).expect("the elbow sweeps");
    assert_eq!(topo::validate(&swept.body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&swept.body), Ok(()), "tier 2");

    // ANTI-VACUITY: the turn has to reach the chart. One wall's normal
    // at the two ends of `v` is a quarter turn apart — a straight path
    // would hold it fixed and this would read cos = 1.
    let wall = swept.side_faces[0][0];
    let n0 = wall_outward_at(&swept.body, wall, 0.5, 0.0).1;
    let n1 = wall_outward_at(&swept.body, wall, 0.5, 1.0).1;
    assert!(
        n0.dot(n1).abs() < 0.05,
        "the elbow must turn the chart normal a quarter turn along v: cos = {}",
        n0.dot(n1)
    );

    let samples = along_v();
    let oracle = |q| loft_contains(&swept, q);
    assert_walls_face_out(&swept, &oracle, &samples, 0.05, 4);
}
