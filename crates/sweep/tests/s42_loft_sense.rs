//! **S42: loft's `sense = true` derivation, pinned on the shapes that
//! broke extrude.**
//!
//! M5 S11 found the swept-wall orientation class: a constructor that
//! mints `sense` from a traversal argument gets it wrong exactly where
//! the material lies AGAINST the chart normal. Extrude shipped
//! `sense: true` on concave arc walls, so a public `union` swallowed a
//! disjoint body (volume 3.000 for 3.008, one shell for two, no
//! refusal); the remediating audit found the same defect already
//! shipped in revolve's bore cylinders, inward cones and under-side
//! plane annulus. The class recurred once per sweep verb.
//!
//! Loft is the third verb. It mints `sense = true` on EVERY wall and
//! argues the bit rather than deciding it: the skinned chart's `u`
//! follows the material-left profile traversal and `v` the stacking,
//! so `S_u × S_v` is material-right — outward — whatever the segment's
//! curvature (`loft.rs` module docs, "Orientation"). Unlike a
//! cylinder's unconditionally-radial frame, the chart normal FOLLOWS
//! the traversal, so concavity cannot flip it. The argument was pinned
//! only on `loft_prism`: no concave arcs, no holes — precisely the
//! shape that did not break extrude either.
//!
//! # What these rows do
//!
//! They run the S11 procedure on the two shapes that DO stress it: a
//! section pair with a concave arc, and one with a hole. The oracle is
//! the S11 one — **the same solid built by `extrude`**, whose walls are
//! analytic `Surface::Cylinder`/`Plane` with a `point_in_solid` door
//! that S11 itself corrected. For every lofted wall the rows evaluate
//! the SHIPPED surface's chart normal, fold in the stored `sense`, and
//! require the twin's door to read material on the inward side and void
//! on the outward one. A flipped bit swaps both answers, so the
//! assertion is two-sided by construction, and
//! `the_probe_inverts_when_a_wall_sense_flips` executes that.
//!
//! # Why this file has to exist
//!
//! Nothing else in the kernel reads a lofted wall's `sense`. Tier-3
//! check 6's curved arm skips `Surface::Nurbs` by name (a recorded
//! residual in `topo::validate`), the flux lanes are winding-derived
//! and bit-free, and `point_in_solid` and the booleans refuse a NURBS
//! operand outright. `a_flipped_loft_wall_sense_still_passes_tier_three`
//! pins that residual, so these rows are the only guard the bit has.

// Panicking is a test's failure mechanism (workspace lint policy).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_8, PI};

use geom_core::{Affine3, Band, Point2, Point3, Tolerance, Vec3};
use geom_surfaces::Surface;
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Lofted, Section, extrude, loft_body};
use topo::Body;
use topo::boolean::{SolidContainment, point_in_solid};

fn v(x: f64, y: f64, bulge: f64) -> ProfileVertex<f64> {
    ProfileVertex::new(Point2::new(x, y), bulge)
}

/// S11's `notched` region, scaled by `k`: a rectangle whose bottom edge
/// bows OUT (convex arc) and whose top edge bows IN (concave arc). The
/// two quarter-circle segments have equal area, so the region's area is
/// exactly `3k²` and the concave notch floor sits at `y = k(2.5 − √2)`.
fn notched_loops(k: f64) -> Vec<ProfileLoop<f64>> {
    let b = FRAC_PI_8.tan();
    vec![ProfileLoop::new(vec![
        v(0.0, 0.0, b),
        v(2.0 * k, 0.0, 0.0),
        v(2.0 * k, 1.5 * k, -b),
        v(0.0, 1.5 * k, 0.0),
    ])]
}

/// S11's holed plate: a 4×4 square with a unit-radius circular hole cut
/// by two half-arcs. The hole loop's canonical winding is CLOCKWISE, so
/// its walls are the hole-loop kin of a concave arc — the plate's
/// material lies OUTSIDE their carrier.
fn holed_loops() -> Vec<ProfileLoop<f64>> {
    vec![
        ProfileLoop::polygon([
            Point2::new(0.0, 0.0),
            Point2::new(4.0, 0.0),
            Point2::new(4.0, 4.0),
            Point2::new(0.0, 4.0),
        ]),
        ProfileLoop::new(vec![v(1.0, 2.0, 1.0), v(3.0, 2.0, 1.0)]),
    ]
}

/// The loft of `loops` stacked to height `h` — a SECTION PAIR, `v`
/// linear, so the body is the prism the extruded twin also builds.
fn loft_pair(loops: &[ProfileLoop<f64>], h: f64) -> Lofted<f64> {
    let sections: Vec<Section> = vec![loops.to_vec(), loops.to_vec()];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.0, 0.0, h)),
    ];
    loft_body::<f64>(&sections, &places, 1).expect("the section pair lofts")
}

/// The ORACLE: the same solid built by `extrude`, whose walls are
/// analytic cylinders and planes and whose containment door is the one
/// S11 corrected.
fn extruded_twin(loops: &[ProfileLoop<f64>], h: f64) -> Body<f64> {
    let vp = Profile::new(SketchPlane::xy(), loops.to_vec())
        .validate(Tolerance::get())
        .unwrap();
    extrude(&vp, Extrusion::Distance(h)).unwrap().body
}

/// One lofted wall's mid-chart point and its OUTWARD normal — the
/// shipped surface's `S_u × S_v` with the stored `sense` folded in.
fn wall_outward(body: &Body<f64>, face: topo::FaceKey) -> (Point3<f64>, Vec3<f64>) {
    let f = body.get_face(face).expect("wall face resolves");
    let Some(Surface::Nurbs(s)) = body.get_surface(f.surface) else {
        panic!("a lofted wall carries a skinned NURBS chart");
    };
    let (u0, u1) = s.knots_u().domain();
    let (v0, v1) = s.knots_v().domain();
    let (um, vm) = ((u0 + u1) * 0.5, (v0 + v1) * 0.5);
    let jet = s.ders(um, vm);
    let chart = jet.du.cross(jet.dv).normalize();
    (s.eval(um, vm), chart * f.sense_sign::<f64>())
}

/// The S11 probe, one wall: step `delta` off the wall along the stored
/// outward normal and against it, and ask the extruded twin's door.
/// Returns `(inward_side, outward_side)` — a flipped `sense` swaps them.
fn probe(
    oracle: &Body<f64>,
    p: Point3<f64>,
    outward: Vec3<f64>,
    delta: f64,
) -> (SolidContainment, SolidContainment) {
    let band = Band::linear().unwrap();
    (
        point_in_solid(oracle, p - outward * delta, band).expect("inward probe decides"),
        point_in_solid(oracle, p + outward * delta, band).expect("outward probe decides"),
    )
}

/// Every wall of `lofted`, probed against `oracle`. Returns the wall
/// count so a caller can pin that the sweep was not vacuous.
fn probe_all_walls(lofted: &Lofted<f64>, oracle: &Body<f64>, delta: f64) -> usize {
    let mut n = 0;
    for (li, walls) in lofted.side_faces.iter().enumerate() {
        for (si, &fk) in walls.iter().enumerate() {
            assert!(
                lofted.body.get_face(fk).unwrap().sense,
                "loop {li} segment {si}: loft mints sense = true on every wall"
            );
            let (p, outward) = wall_outward(&lofted.body, fk);
            let (inward_side, outward_side) = probe(oracle, p, outward, delta);
            assert_eq!(
                inward_side,
                SolidContainment::In,
                "loop {li} segment {si} at {p:?}: the stored orientation says \
                 {outward:?} is outward, so stepping AGAINST it must land in \
                 the material"
            );
            assert_eq!(
                outward_side,
                SolidContainment::Out,
                "loop {li} segment {si} at {p:?}: stepping ALONG the stored \
                 outward normal must leave the material"
            );
            n += 1;
        }
    }
    n
}

/// **Fixture 1 — the concave arc.** The shape that broke extrude,
/// lofted: tier-3 valid at rest, the exact analytic volume, every wall
/// `sense = true`, and every wall's sense-signed chart normal pointing
/// out of the material as the extruded twin's door reads it. The
/// concave wall's mid-chart sample sits on the notch floor
/// `y = 2.5 − √2 ≈ 1.0858` with outward normal `+ŷ` — INTO the notch,
/// away from the meat, which is the whole question.
#[test]
fn concave_arc_loft_walls_point_out_of_the_material() {
    let loops = notched_loops(1.0);
    let lofted = loft_pair(&loops, 1.0);
    let body = &lofted.body;
    assert_eq!(topo::validate(body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(body), Ok(()), "tier 2 (watertight)");
    assert_eq!(topo::validate_geometric(body), Ok(()), "tier 3 at rest");
    let vol = topo::props::mass_properties(body).unwrap().volume;
    // The arc walls are RATIONAL, so their patch flux is the certified
    // quadrature rather than a closed form: the accuracy target is
    // absolute and stated, never the returned pad (a pad-relative
    // bracket loosens exactly as the enclosure degrades).
    assert!(
        (vol - 3.0).abs() < 1e-7,
        "the equal-area segments make the region exactly 3, so the unit \
         prism meters 3.0 to the rational quadrature's accuracy; got {vol}"
    );
    assert_eq!(
        probe_all_walls(&lofted, &extruded_twin(&loops, 1.0), 0.02),
        4,
        "the fixture's four walls were all probed"
    );
    assert!(
        body.get_face(lofted.top).unwrap().sense && body.get_face(lofted.bottom).unwrap().sense,
        "the caps keep sense = true (the bottom cap's LOOP is reversed at \
         mint, so its plane derives normal-down)"
    );
}

/// **Fixture 2 — the hole.** A hole loop's walls are concave walls: the
/// plate's material lies outside their carrier, and the canonical
/// clockwise winding is what encodes that. Extrude mints `sense: false`
/// there (S11); loft mints `true`, and both are honest because the
/// charts differ — this row is what proves the loft half.
///
/// # The tier-3 posture
///
/// The hole walls are RATIONAL, so their patch flux runs the certified
/// quadrature, and at this fixture's scale the fixed schedule misses
/// `1024·ε`. That is the m8-3 ε posture, not an orientation verdict:
/// the row therefore requires tiers 1 and 2 green and pins that the
/// tier-3 report carries NOTHING but the volume refusal — an
/// orientation error appearing here would fail it.
#[test]
fn holed_loft_walls_point_out_of_the_material() {
    let loops = holed_loops();
    let lofted = loft_pair(&loops, 1.0);
    let body = &lofted.body;
    assert_eq!(topo::validate(body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(body), Ok(()), "tier 2 (watertight)");
    if let Err(errs) = topo::validate_geometric(body) {
        assert!(
            errs.iter()
                .all(|e| matches!(e, topo::ValidationError::VolumeUncomputable { .. })),
            "tier 3 may report the rational quadrature's ε budget and nothing \
             else; an orientation error here is the S11 defect: {errs:?}"
        );
    }
    assert_eq!(
        probe_all_walls(&lofted, &extruded_twin(&loops, 1.0), 0.05),
        6,
        "four plate walls and the hole's two half-cylinder walls"
    );
}

/// **The instrument goes red.** Flipping ONE wall's `sense` must make
/// the probe invert — material on the outward side, void on the inward
/// one. Run over every wall of both fixtures, so no row above can be
/// passing because the assertion is monotone in the wrong direction.
#[test]
fn the_probe_inverts_when_a_wall_sense_flips() {
    for (loops, delta) in [(notched_loops(1.0), 0.02), (holed_loops(), 0.05)] {
        let lofted = loft_pair(&loops, 1.0);
        let oracle = extruded_twin(&loops, 1.0);
        let mut flipped_walls = 0;
        for walls in &lofted.side_faces {
            for &fk in walls {
                let lied = lofted.body.flipped_face_sense_for_tests(fk).unwrap();
                let (p, outward) = wall_outward(&lied, fk);
                let (inward_side, outward_side) = probe(&oracle, p, outward, delta);
                assert_eq!(inward_side, SolidContainment::Out, "flipped {fk:?}");
                assert_eq!(outward_side, SolidContainment::In, "flipped {fk:?}");
                flipped_walls += 1;
            }
        }
        assert_eq!(flipped_walls, if loops.len() == 1 { 4 } else { 6 });
    }
}

/// **The residual these rows exist for.** A lone `sense` flip on a
/// lofted wall changes no winding, and tier-3 check 6's curved arm
/// skips `Surface::Nurbs` by name — so the tier ladder admits the lie.
/// The flux lanes are winding-derived and bit-free, so the volume is
/// bit-identical too. This row pins that state: if NURBS coverage ever
/// lands in check 6 it fails, and the guard moves from these rows into
/// the validator where it belongs.
///
/// # Cost
///
/// Only the LIED body is metered. The honest body's `3.0` is
/// `concave_arc_loft_walls_point_out_of_the_material`'s row, so
/// comparing the inside-out volume against that analytic constant says
/// exactly what a honest-vs-lied comparison would and pays one fewer
/// rational quadrature (the m8-3 test-cost discipline).
#[test]
fn a_flipped_loft_wall_sense_still_passes_tier_three() {
    let lofted = loft_pair(&notched_loops(1.0), 1.0);
    let wall = lofted.side_faces[0][2];
    // Segment order is the profile's: the third is the CONCAVE arc, the
    // one whose extruded twin mints `sense: false`. Pinned, so the row
    // cannot drift onto a planar wall and keep passing.
    let (p, _) = wall_outward(&lofted.body, wall);
    assert!(
        (p.y - (2.5 - 2.0_f64.sqrt())).abs() < 1e-9,
        "the flipped wall is the concave arc, whose mid-chart sample sits \
         on the notch floor y = 2.5 − √2; got {p:?}"
    );
    let lied = lofted.body.flipped_face_sense_for_tests(wall).unwrap();
    assert_eq!(topo::validate(&lied), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&lied), Ok(()), "tier 2");
    assert_eq!(
        topo::validate_geometric(&lied),
        Ok(()),
        "the recorded residual: check 6's curved arm exempts NURBS, so an \
         inside-out lofted wall is tier-3 green"
    );
    let lied_vol = topo::props::mass_properties(&lied).unwrap().volume;
    assert!(
        (lied_vol - 3.0).abs() < 1e-7,
        "the flux is winding-derived, so props meter the inside-out body at \
         the honest 3.0 — the bit is invisible to them; got {lied_vol}"
    );
}

/// **The S11 union check, run literally.** S11's witness was
/// `union(notched, pellet)` on two DISJOINT solids: with no surface
/// intersections the operation falls back to mutual containment, asks
/// `point_in_solid`, and — with a wrong `sense` on the concave wall —
/// answered `In` for the pellet and shipped a body silently missing a
/// solid.
///
/// On a LOFTED operand that path is closed by refusal, not by the bit:
/// the boolean layer rejects a rung-3 NURBS operand at admission, and
/// `point_in_solid` has no NURBS door at all. So loft cannot reproduce
/// the S11 swallow today for a reason independent of its orientation
/// bit — which is exactly why the probe rows above are needed, and why
/// this row pins the refusals: the day either door opens, an
/// unverified `sense` would become load-bearing without warning.
#[test]
fn the_union_check_refuses_typed_on_a_lofted_operand() {
    let loops = notched_loops(1.0);
    let lofted = loft_pair(&loops, 1.0);

    // The S11 pellet: strictly inside the concave notch, disjoint from
    // the solid (the notch floor at x = 1 is y = 2.5 − √2 ≈ 1.0858).
    let lp = ProfileLoop::polygon([
        Point2::new(0.9, 1.25),
        Point2::new(1.1, 1.25),
        Point2::new(1.1, 1.35),
        Point2::new(0.9, 1.35),
    ]);
    let plane = SketchPlane::from_frame(
        Point3::new(0.0, 0.0, 0.3),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    let vp = Profile::new(plane, vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    let pellet = extrude(&vp, Extrusion::Distance(0.4)).unwrap().body;

    // The extruded twin still runs the S11 check end to end: two shells,
    // volumes adding, no swallow. That is the behaviour a lofted operand
    // must eventually match.
    let twin = extruded_twin(&loops, 1.0);
    let joined = topo::boolean::union(&twin, &pellet).unwrap();
    let out = joined.body().expect("union of two non-empty solids");
    assert_eq!(out.body.shells().count(), 2, "the twin keeps both solids");
    let vol = topo::props::mass_properties(&out.body).unwrap().volume;
    assert!(
        (vol - 3.008).abs() < 1e-9,
        "3.000 meat + 0.008 pellet = 3.008; got {vol}"
    );

    // The lofted operand refuses, naming the missing layer.
    let err = match topo::boolean::union(&lofted.body, &pellet) {
        Err(e) => e.to_string(),
        Ok(_) => panic!("a rung-3 NURBS operand has no boolean layer yet"),
    };
    assert!(
        err.contains("rung-3"),
        "the refusal names the rung-3 NURBS operand: {err}"
    );

    // And the door the S11 swallow went through does not answer at all.
    let band = Band::linear().unwrap();
    let door = point_in_solid(&lofted.body, Point3::new(1.0, 1.3, 0.5), band);
    assert!(
        door.is_err(),
        "point_in_solid has no NURBS door: it must refuse, not guess: {door:?}"
    );
}

/// **Not a prism.** Every row above lofts identical sections, because
/// that is what makes the extruded twin an exact oracle — and it is
/// also the shape S42 objects to. This row lofts a GENUINELY tapered
/// concave-arc pair (the top section scaled to 0.8) and requires each
/// wall's outward normal to agree in direction with the verified
/// prism's. Scaling the top section by 0.8 about the sketch origin
/// displaces a wall at radius `d` by `0.2·d` over unit height, and the
/// fixture's farthest wall sits at `d = 2.5`, so the tilt is at most
/// `atan(0.5) ≈ 26.6°`. The row asserts `45°`: comfortably above that
/// geometric maximum and comfortably below the `180°` a flipped `sense`
/// produces, so it discriminates the bit and nothing else.
#[test]
fn a_tapered_concave_loft_keeps_the_verified_wall_directions() {
    let base = notched_loops(1.0);
    let prism = loft_pair(&base, 1.0);
    let sections: Vec<Section> = vec![base.clone(), notched_loops(0.8)];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
    ];
    let tapered = loft_body::<f64>(&sections, &places, 1).expect("the tapered pair lofts");
    assert_eq!(topo::validate(&tapered.body), Ok(()), "tier 1");
    assert_eq!(
        topo::validate_closed(&tapered.body),
        Ok(()),
        "tier 2 (watertight)"
    );
    for (li, walls) in tapered.side_faces.iter().enumerate() {
        for (si, &fk) in walls.iter().enumerate() {
            assert!(
                tapered.body.get_face(fk).unwrap().sense,
                "loop {li} segment {si}: sense = true"
            );
            let (_, taper_n) = wall_outward(&tapered.body, fk);
            let (_, prism_n) = wall_outward(&prism.body, prism.side_faces[li][si]);
            let dot = taper_n.dot(prism_n);
            assert!(
                dot > (PI / 4.0).cos(),
                "loop {li} segment {si}: the taper may tilt the wall, not \
                 reverse it; cos = {dot}"
            );
        }
    }
}
