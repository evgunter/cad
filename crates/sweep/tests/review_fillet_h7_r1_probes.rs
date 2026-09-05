//! **FILLET-H7 review probes (lane r1)** — rows the unit's own suite
//! does not hold, each an ordinary test the fix pass may adopt.
//!
//! What they draw, beyond `fillet_h7_transverse_cap.rs`:
//!
//! - the ruled band on the OTHER sense of the cylinder — a groove milled
//!   along a block: the lip creases are convex, the cylinder's material
//!   is OUTSIDE it, so the arm's sheet crossing takes the outer offset
//!   trace; the closed form is `quad − sector − segment` (the rod's has
//!   `+ segment`: the support arc bulges the other way);
//! - the CONCAVE ruled band — a rod sunk into a block, whose two creases
//!   along the ruling add material: the fold the unit states and pins
//!   nowhere. Both bodies come through the EXTRUDE door as one profile
//!   loop; the boolean refuses both (`block ∖ cylinder`,
//!   `block ∪ cylinder`), which is pinned beside them;
//! - a cap carrying a RING (the bored D-rod): the plan checks the
//!   supports for rings, not the cap;
//! - a cap rim requested beside its crease: a typed refusal (the
//!   battery's, not the surgery's);
//! - a flat past the axis (`ROD_FLAT < 0`): the section's fillet arc
//!   sweeps 139°, so the cut-off arcs are long and `rod_section_cut`'s
//!   closed form is exercised off the unit's one fixture;
//! - the `seam_split_param` generalisation on an EXISTING door: a
//!   cylinder–cone rim of a revolve whose cylinder wall's seam meridian
//!   is a LINE longer than 2π — the period guard that refused it at the
//!   merge base no longer applies to a line carrier.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::blend::{BlendError, fillet_edges};
use sweep::test_support::{
    ROD_FILLET, ROD_L, ROD_R, assert_naming_totality, rod_creases, rod_section_cut,
};
use sweep::{Extrusion, extrude};
use topo::{Body, EdgeKey, mass_properties, validate_geometric};

const R: f64 = ROD_FILLET;
/// The groove / sunk-rod cylinder's radius and its axis depth below the
/// block's top plane.
const RC: f64 = 0.5;
const S: f64 = 0.3;
/// The block's length along the ruling.
const L: f64 = 1.0;

fn tol() -> Tol {
    Tol::witness()
}

fn census(body: &Body<f64>) -> (usize, usize, usize) {
    (
        body.vertices().count(),
        body.edges().count(),
        body.faces().count(),
    )
}

fn volume(body: &Body<f64>) -> f64 {
    let p = mass_properties(body, tol()).expect("closed-form props");
    assert_eq!(p.volume_pad, 0.0, "the inventory is closed-form");
    p.volume
}

fn rect(x0: f64, x1: f64, y0: f64, y1: f64) -> ProfileLoop<f64> {
    ProfileLoop::new(
        [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(Point2::new(x, y), 0.0))
            .collect(),
    )
}

fn extruded(plane: SketchPlane<f64>, loops: Vec<ProfileLoop<f64>>, len: f64) -> Body<f64> {
    let p = Profile::new(plane, loops)
        .validate(tol())
        .expect("the profile validates");
    extrude(&p, Extrusion::Distance(len), tol())
        .expect("the profile extrudes")
        .body
}

/// The block: `x ∈ [−1, 1]`, `y ∈ [−1, 0]`, `z ∈ [0, L]` — its top
/// plane is `y = 0`, its two end faces `z = 0` and `z = L` are the
/// transverse caps.
fn block() -> Body<f64> {
    extruded(SketchPlane::xy(), vec![rect(-1.0, 1.0, -1.0, 0.0)], L)
}

/// A cylinder of radius [`RC`] about the line `(0, −S, z)`, over
/// `z ∈ [z0, z0 + len]`.
fn cylinder(z0: f64, len: f64) -> Body<f64> {
    let disc = profile::circle(Point2::new(0.0, -S), RC, tol()).expect("a disc");
    let plane = SketchPlane::new(geom_core::Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    extruded(plane, vec![disc.into()], len)
}

/// **The cross-section a ruled band moves, in the general shape**: the
/// straight-sided hull `c → f_b → V → f_a` (the ball centre, the foot
/// on the plane, the crease, the foot on the cylinder), minus the ball's
/// sector at `c`, minus or plus the cylinder's circular segment between
/// the chord `V–f_a` and its arc — `+` where the arc bulges AWAY from
/// the hull (the rod: material inside the cylinder), `−` where it
/// bulges INTO it (the groove and the sunk rod: the ball on the outer
/// offset). Independent of `rod_section_cut`'s spelling — checked
/// against a 9-million-cell grid at each fixture before it was written.
fn section_area(
    c: (f64, f64),
    axis: (f64, f64),
    big_r: f64,
    r: f64,
    v: (f64, f64),
    segment_sign: f64,
) -> f64 {
    let f_b = (c.0, 0.0);
    let d = (c.0 - axis.0, c.1 - axis.1);
    let n = d.0.hypot(d.1);
    let f_a = (axis.0 + d.0 * big_r / n, axis.1 + d.1 * big_r / n);
    let quad = [c, f_b, v, f_a];
    let mut twice = 0.0;
    for i in 0..4 {
        let (p, q) = (quad[i], quad[(i + 1) % 4]);
        twice += p.0 * q.1 - q.0 * p.1;
    }
    let theta = (((f_a.0 - c.0) * (f_b.0 - c.0) + (f_a.1 - c.1) * (f_b.1 - c.1)) / (r * r)).acos();
    let phi = ((v.1 - axis.1).atan2(v.0 - axis.0) - (f_a.1 - axis.1).atan2(f_a.0 - axis.0)).abs();
    0.5 * twice.abs() - 0.5 * r * r * theta + segment_sign * 0.5 * big_r * big_r * (phi - phi.sin())
}

/// Carve every cylinder–plane crease and check the walk's invariants:
/// two bands, no corner patch, the census delta, tier 3, naming
/// totality. Returns the signed volume change `V₁ − V₀`.
fn carve_ruled(source: &Body<f64>, what: &str) -> f64 {
    let creases = rod_creases(source);
    assert_eq!(
        creases.len(),
        2,
        "{what}: two ruling creases, got {creases:?}"
    );
    let (v0, e0, f0) = census(source);
    let vol0 = volume(source);
    let out = fillet_edges(source, &creases, R, tol())
        .unwrap_or_else(|e| panic!("{what}: both creases carve, got {e}"));
    assert_eq!(out.blend_faces.len(), 2, "{what}: one band per crease");
    assert!(out.corner_faces.is_empty() && out.band_faces.is_empty());
    assert_eq!(
        census(&out.body),
        (v0 + 4, e0 + 6, f0 + 2),
        "{what}: the census delta of two cut-off bands"
    );
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, got {e:?}"));
    assert_naming_totality(source, &out, &creases, what);
    volume(&out.body) - vol0
}

/// The block's top edge with the rod's section cut into it (the major
/// arc dipping below `y = 0`) or standing on it (the minor arc rising
/// above): one profile loop, extruded along the ruling, so the block's
/// two end faces are the transverse caps.
fn block_with_section(sunk: bool) -> Body<f64> {
    let xv = (RC * RC - S * S).sqrt();
    // The arc from (xv, 0) to (−xv, 0) about (0, −S). Travelling the
    // loop counter-clockwise the arc runs from +xv to −xv: counter-
    // clockwise about the centre for the rising minor arc (positive
    // bulge), clockwise for the dipping major arc (negative bulge).
    let minor = 2.0 * (xv / RC).asin();
    let sweep = if sunk {
        minor
    } else {
        core::f64::consts::TAU - minor
    };
    let bulge = (sweep / 4.0).tan() * if sunk { 1.0 } else { -1.0 };
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(-1.0, -1.0), 0.0),
        ProfileVertex::new(Point2::new(1.0, -1.0), 0.0),
        ProfileVertex::new(Point2::new(1.0, 0.0), 0.0),
        ProfileVertex::new(Point2::new(xv, 0.0), bulge),
        ProfileVertex::new(Point2::new(-xv, 0.0), 0.0),
        ProfileVertex::new(Point2::new(-1.0, 0.0), 0.0),
    ]);
    extruded(SketchPlane::xy(), vec![lp], L)
}

/// The area of the rod's section above the block's top plane.
fn cap_above() -> f64 {
    let minor = 2.0 * ((RC * RC - S * S).sqrt() / RC).asin();
    0.5 * RC * RC * (minor - minor.sin())
}

/// **The boolean builds neither ruled fixture on a block** — the groove
/// (`block ∖ cylinder`) refuses `CurvedSectorSideUnsupported`, the sunk
/// rod (`block ∪ cylinder`) refuses at the all-planar join lane. Pinned
/// so the Phase-1 table's "no body" for the CONCAVE band is read as the
/// boolean's, not the kernel's: the extrude door builds both (below).
#[test]
fn the_boolean_builds_neither_the_groove_nor_the_sunk_rod() {
    let groove = topo::subtract(&block(), &cylinder(-0.5, L + 1.0), tol());
    assert!(
        matches!(
            groove,
            Err(topo::BooleanError::CurvedSectorSideUnsupported { .. })
        ),
        "the groove refuses at the boolean, got {groove:?}"
    );
    let sunk = topo::union(&block(), &cylinder(0.2, 0.6), tol());
    assert!(
        matches!(sunk, Err(topo::BooleanError::Join(_))),
        "the sunk rod refuses at the boolean, got {sunk:?}"
    );
}

/// **A groove milled along a block** (extrude door): the lip creases
/// are convex with the cylinder's material OUTSIDE the cylinder — the
/// sense bit the rod does not exercise. Both lips carve at the prism
/// closed form.
#[test]
fn a_groove_lip_carves_with_the_cylinder_material_outside() {
    let source = block_with_section(false);
    validate_geometric(&source, tol()).expect("the grooved block is tier-3 valid");
    let expect = (2.0 - (core::f64::consts::PI * RC * RC - cap_above())) * L;
    assert!(
        (volume(&source) - expect).abs() < 1e-12,
        "the groove's own volume: {} vs {expect}",
        volume(&source)
    );
    let dv = carve_ruled(&source, "groove");
    // The ball rests inside the material: under the plane by r, outside
    // the cylinder by r.
    let c = (((RC + R).powi(2) - (S - R).powi(2)).sqrt(), -R);
    let v = ((RC * RC - S * S).sqrt(), 0.0);
    let a = section_area(c, (0.0, -S), RC, R, v, -1.0);
    assert!(
        (a - 0.005625893891207202).abs() < 1e-12,
        "the oracle's own value at this fixture: {a}"
    );
    assert!(
        (-dv - 2.0 * a * L).abs() < 1e-12,
        "groove: ΔV = −2·A·L, measured {dv} vs {}",
        -2.0 * a * L
    );
}

/// **A rod sunk into a block** (extrude door): the two creases along
/// the ruling are CONCAVE, and the band ADDS material — the fold's
/// other arm, which the unit states and pins nowhere because the
/// parallel-cylinder union has no body. This body does.
#[test]
fn a_sunk_rod_has_concave_ruled_creases_that_add_material() {
    let source = block_with_section(true);
    validate_geometric(&source, tol()).expect("the sunk rod is tier-3 valid");
    let expect = (2.0 + cap_above()) * L;
    assert!(
        (volume(&source) - expect).abs() < 1e-12,
        "the sunk rod's own volume: {} vs {expect}",
        volume(&source)
    );
    let dv = carve_ruled(&source, "sunk rod");
    // The ball rests in the void: above the plane by r, outside the
    // cylinder by r.
    let c = (((RC + R).powi(2) - (S + R).powi(2)).sqrt(), R);
    let v = ((RC * RC - S * S).sqrt(), 0.0);
    let a = section_area(c, (0.0, -S), RC, R, v, -1.0);
    assert!(
        (a - 0.0002949786679543043).abs() < 1e-12,
        "the oracle's own value at this fixture: {a}"
    );
    assert!(dv > 0.0, "a concave band adds material, got {dv}");
    assert!(
        (dv - 2.0 * a * L).abs() < 1e-12,
        "sunk rod: ΔV = +2·A·L, measured {dv} vs {}",
        2.0 * a * L
    );
}

/// The D-profile rod at an arbitrary flat offset (the unit's helper is
/// pinned to `ROD_FLAT`), optionally with a coaxial bore.
fn d_rod(flat: f64, bore: Option<f64>) -> Body<f64> {
    let y = (ROD_R * ROD_R - flat * flat).sqrt();
    let theta = 2.0 * (core::f64::consts::PI - y.atan2(flat));
    let bulge = (theta / 4.0).tan();
    let mut loops = vec![ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(flat, y), bulge),
        ProfileVertex::new(Point2::new(flat, -y), 0.0),
    ])];
    if let Some(b) = bore {
        loops.push(
            profile::circle(Point2::new(0.0, 0.0), b, tol())
                .expect("the bore's disc")
                .into(),
        );
    }
    extruded(SketchPlane::xy(), loops, ROD_L)
}

/// **A cap with a ring**: the bored D-rod's caps each carry the bore as
/// a ring. `RuledPlan::plan` checks the SUPPORTS for rings and not the
/// cap; the cut-off `mef` runs on the cap's outer cycle and the cap
/// keeps its rings. Both creases carve at the same closed form as the
/// unbored rod, and the rings survive on the caps.
#[test]
fn a_cap_carrying_a_ring_keeps_it_through_the_cut_off() {
    let source = d_rod(0.3, Some(0.15));
    let dv = carve_ruled(&source, "bored D-rod");
    let a = rod_section_cut(ROD_R, 0.3, R);
    assert!(
        (-dv - 2.0 * a * ROD_L).abs() < 1e-12,
        "bored: ΔV = −2·A·L, measured {dv}"
    );
    let creases = rod_creases(&source);
    let out = fillet_edges(&source, &creases, R, tol()).unwrap();
    let ringed = out
        .body
        .faces()
        .filter(|(k, _)| !out.body.get_face(*k).unwrap().rings.is_empty())
        .count();
    assert_eq!(ringed, 2, "both caps keep their ring");
}

/// **A flat past the axis**: `flat = −0.2`, so the crease's dihedral is
/// obtuse on the material side and the fillet arc sweeps 139° — long
/// cut-off arcs (still under π) and `rod_section_cut` off the unit's one
/// fixture.
#[test]
fn a_flat_past_the_axis_carves_at_the_closed_form() {
    let source = d_rod(-0.2, None);
    let dv = carve_ruled(&source, "flat past the axis");
    let a = rod_section_cut(ROD_R, -0.2, R);
    assert!(
        (-dv - 2.0 * a * ROD_L).abs() < 1e-12,
        "flat past the axis: ΔV = −2·A·L, measured {dv} vs {}",
        -2.0 * a * ROD_L
    );
}

/// **A cap rim requested beside its crease** refuses typed — whichever
/// door raises it — and does not panic: the D-rod's flat chord on one
/// cap is a plane–plane edge whose ends are the crease ends.
#[test]
fn requesting_a_cap_rim_beside_the_crease_refuses_typed() {
    let source = d_rod(0.3, None);
    let creases = rod_creases(&source);
    let chord = topo::query::all_edges(&source)
        .into_iter()
        .find(|&k| {
            topo::query::edge_carrier_matches(
                &source,
                k,
                topo::query::CurveKindSet::just(topo::query::CurveKind::Line),
            ) && !creases.contains(&k)
        })
        .expect("a cap's chord");
    let err = fillet_edges(&source, &[creases[0], chord], R, tol())
        .expect_err("a cap rim requested beside its crease is not carved");
    // The battery gets there first: the chord and the crease meet at an
    // angle, so the two-link chain is not G1 — a typed refusal, and the
    // surgery's own "cap rim is itself requested" clause is never
    // reached from the front door by this request.
    assert!(
        matches!(err.error, BlendError::ChainNotG1 { .. }),
        "a typed refusal, got {:?}",
        err.error
    );
}

/// **The `seam_split_param` generalisation on an EXISTING door.** A
/// cylinder–cone rim of a solid of revolution carves through the annulus
/// surgery, whose trim splits each wall's seam meridian — the cylinder's
/// is a LINE whose stored window is the wall's height. Before H7 the
/// period guard read that length against 2π and refused a wall taller
/// than 2π as "not under one period"; a line has no period, so the
/// guard now applies to circle carriers only. This row pins that a
/// 7-tall wall's rim carves (at the merge base it refuses).
#[test]
fn a_tall_cylinder_wall_rim_carves_past_a_two_pi_meridian() {
    let body = sweep::test_support::revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 7.0), 0.0),
            ProfileVertex::new(Point2::new(0.5, 7.5), 0.0),
            ProfileVertex::new(Point2::new(0.0, 7.5), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    );
    validate_geometric(&body, tol()).expect("the capped tall cylinder is tier-3 valid");
    let rim: Vec<EdgeKey> = topo::query::all_edges(&body)
        .into_iter()
        .filter(|&k| {
            topo::query::edge_adjacent_matches(
                &body,
                k,
                topo::query::SurfaceKindSet::just(geom_brep::SurfaceKind::Cylinder),
                topo::query::SurfaceKindSet::just(geom_brep::SurfaceKind::Cone),
            )
        })
        .collect();
    assert!(!rim.is_empty(), "the cylinder–cone rim's arcs");
    let out = fillet_edges(&body, &rim, R, tol())
        .unwrap_or_else(|e| panic!("the tall wall's rim carves, got {e}"));
    validate_geometric(&out.body, tol()).expect("tier 3");
}
