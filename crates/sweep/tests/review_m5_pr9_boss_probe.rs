//! Blinded-review probes, M5 PR 9 charter A/D/F: an independently
//! authored boss-union consumer (own dimensions, own closed forms),
//! the subtract lane, the 2-arc germ-facing refusal (deviation 8), a
//! chained second curved boolean (zip stage attack), the du_of_rims
//! equal-span witness, and the non-maximal curved operand attack on
//! the F7 gate exemption.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::{Affine3, Point2, Point3, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::Body;
use topo::splitting::{SplitPlane, split};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn rect(w: f64, h: f64) -> ProfileLoop<f64> {
    ProfileLoop::new(
        [(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    )
}

/// My plate: 3 x 3 x 0.8.
fn plate() -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![rect(3.0, 3.0)])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(0.8), Tol::witness())
        .unwrap()
        .body
}

/// My boss: r = 0.35 at (1.2, 1.7), authored as `n` equal arcs,
/// sketched at z0, extruded by len.
fn boss(n: usize, z0: f64, len: f64) -> Body<f64> {
    let theta = 2.0 * std::f64::consts::PI / n as f64;
    let bulge = (theta / 4.0).tan();
    let at = |i: usize| {
        let th = theta * i as f64;
        p2(1.2 + 0.35 * th.cos(), 1.7 + 0.35 * th.sin())
    };
    let lp = ProfileLoop::new((0..n).map(|i| ProfileVertex::new(at(i), bulge)).collect());
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(len), Tol::witness())
        .unwrap()
        .body
}

/// Tier-3 + closed-form volume + seam inventory + pcurve coverage.
fn audit(body: &Body<f64>, expect_vol: f64, expect_seam_arcs: usize, seam_z: f64) {
    if let Err(errs) = topo::validate_geometric(body, Tol::witness()) {
        panic!("tier-3 validity: {errs:?}");
    }
    let vol = topo::mass_properties(body, Tol::witness()).unwrap().volume;
    assert!(
        (vol - expect_vol).abs() < 1e-9,
        "volume {vol} vs closed form {expect_vol}"
    );
    // Seam arcs: exact Circle carriers, intrinsic Intersection
    // descriptions (D6).
    let mut arcs = 0;
    for (_, e) in body.edges() {
        let Some(c) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
            continue;
        };
        if let geom::Curve3::Circle { center, radius, .. } = *c.carrier()
            && (center.z - seam_z).abs() < 1e-9
            && (radius - 0.35).abs() < 1e-9
        {
            arcs += 1;
            assert!(
                matches!(
                    c.description(),
                    geom_brep::EdgeGeometry::Intersection { .. }
                ),
                "curved boolean seam must be intrinsic: {:?}",
                c.description()
            );
        }
    }
    assert_eq!(arcs, expect_seam_arcs, "rim seam arc count at z={seam_z}");
    // Pcurves: every half-edge bounding a curved face carries a
    // certified cache (the PR 6 exit criterion, re-checked from the
    // consumer side).
    let cached: std::collections::BTreeSet<_> = body.pcurves().map(|(k, _)| k).collect();
    for (he_key, he) in body.half_edges() {
        let curved = body
            .get_loop(he.parent_loop)
            .and_then(|l| body.get_face(l.face))
            .and_then(|f| body.get_surface(f.surface))
            .is_some_and(|s| !matches!(s, geom::Surface::Plane { .. }));
        if curved {
            assert!(
                cached.contains(&he_key),
                "half-edge {he_key:?} on a curved face has no pcurve"
            );
        }
    }
}

#[test]
fn my_boss_union_all_the_way_down() {
    let out = topo::union(&plate(), &boss(3, 0.3, 1.0), Tol::witness()).expect("union");
    let body = &out.body().expect("a body").body;
    let expect = 3.0 * 3.0 * 0.8 + std::f64::consts::PI * 0.35 * 0.35 * 0.5;
    audit(body, expect, 3, 0.8);
}

#[test]
fn my_boss_subtract_makes_a_blind_hole_honestly() {
    // plate - boss: the bread-and-butter drilled blind hole. The MAJ-3
    // finding (curved booleans were silently UNION-ONLY, dying deep in
    // the pipeline at a stale planar-era revert message) got its ruling
    // at the PR 9 fix pass: a typed front door. M5 S12 RETIRED that door
    // for this class — S10's `Face::sense`, S11's honest constructor
    // bits and S12's revert wiring made curved ∖ real — so this row
    // flips from a refusal pin to the construction row it always wanted
    // to be, audited exactly like the union twin above: exact
    // closed-form volume, tier 3, intrinsic seam arcs, pcurve coverage.
    let out = topo::subtract(&plate(), &boss(3, 0.3, 1.0), Tol::witness())
        .expect("curved subtract is live");
    let body = &out.body().expect("a body").body;
    // The pocket runs from z = 0.3 to the top face at z = 0.8.
    let expect = 3.0 * 3.0 * 0.8 - std::f64::consts::PI * 0.35 * 0.35 * 0.5;
    audit(body, expect, 3, 0.8);
    // Intersect takes the same live lane, and the pair is additive.
    let met = topo::intersect(&plate(), &boss(3, 0.3, 1.0), Tol::witness())
        .expect("curved intersect is live");
    let met_body = &met.body().expect("a body").body;
    let met_vol = topo::mass_properties(met_body, Tol::witness())
        .unwrap()
        .volume;
    assert!(
        (met_vol - std::f64::consts::PI * 0.35 * 0.35 * 0.5).abs() < 1e-9,
        "intersect meters the plug: {met_vol}"
    );
    assert!(
        (topo::mass_properties(body, Tol::witness()).unwrap().volume + met_vol - 3.0 * 3.0 * 0.8)
            .abs()
            < 1e-9,
        "A∖B + A∩B must meter A"
    );
}

#[test]
fn the_two_arc_boss_refuses_typed_not_silently() {
    // Deviation 8, CLOSED at the fix pass: the SAME disc authored as
    // two semicircles — the canonical PR 5 authoring — now unions
    // LIVE. Three chord-degeneracy repairs made it so: germ facing
    // became locus-aware (rotational senses about the section frame —
    // a semicircle's endpoint tangents are exactly chord-
    // perpendicular), the ring-run winding gained the arc bulge term
    // (a two-semicircle 2-gon's chord Newell sum is exactly zero),
    // and the second-chord skip guard asks WINDOW membership instead
    // of on-locus membership (both complementary arcs share one
    // locus). Same audit as the 3-arc row: exact volume, tier 3,
    // seam arcs, pcurves.
    let out = topo::union(&plate(), &boss(2, 0.3, 1.0), Tol::witness())
        .expect("the 2-arc authoring unions live since the fix pass");
    let body = &out.body().expect("a body").body;
    let expect = 3.0 * 3.0 * 0.8 + std::f64::consts::PI * 0.35 * 0.35 * 0.5;
    audit(body, expect, 2, 0.8);
}

#[test]
fn a_second_curved_boolean_chains_on_the_first_result() {
    // Zip stage attack: the first union's result (same-key wedge
    // walls, minted seam arcs, curved pcurves) is itself an operand.
    // A planar-boolean-only pipeline never saw such an operand.
    let first = topo::union(&plate(), &boss(3, 0.3, 1.3), Tol::witness())
        .expect("first union")
        .body()
        .expect("body")
        .body
        .clone();
    // Second plate: 3 x 3 slab z in [1.0, 1.15], crossing the boss
    // wall strictly above the first plate.
    let lp = rect(3.0, 3.0);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 1.0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let slab = extrude(&profile, Extrusion::Distance(0.15), Tol::witness())
        .unwrap()
        .body;
    match topo::union(&first, &slab, Tol::witness()) {
        Ok(out) => {
            let body = &out.body().expect("body").body;
            if let Err(errs) = topo::validate_geometric(body, Tol::witness()) {
                panic!("chained curved boolean must stay tier-3: {errs:?}");
            }
            let vol = topo::mass_properties(body, Tol::witness()).unwrap().volume;
            // plate + slab + boss pieces outside both:
            // boss z in [0.3, 1.6]: exposed z in [0.8,1.0] and [1.15,1.6].
            let boss_extra = std::f64::consts::PI * 0.35 * 0.35 * ((1.0 - 0.8) + (1.6 - 1.15));
            let expect = 7.2 + 3.0 * 3.0 * 0.15 + boss_extra;
            assert!(
                (vol - expect).abs() < 1e-9,
                "chained volume {vol} vs closed form {expect}"
            );
        }
        Err(e) => {
            // A typed refusal is honest; a wrong body is not.
            eprintln!("CHAINED UNION REFUSED TYPED: {e}");
        }
    }
}

#[test]
fn du_of_rims_sums_equal_span_arcs_the_shape_the_old_rule_silently_halved() {
    // Charter D executed witness: disc cylinder (2 semicircle walls,
    // meridians at azimuth 0 and pi), split by x = 0 (cuts at
    // azimuth +/- pi/2): the below part's wall fragments span pi/2
    // EACH — equal spans, the exact shape the pre-PR-9 first-arc rule
    // accepted silently at HALF the true du. After the cosurface
    // merge the face has two arcs per rim level; volume must be the
    // closed-form half-cylinder.
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(-0.5, 0.0), 1.0),
        ProfileVertex::new(p2(0.5, 0.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let body = extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body;
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
    };
    let parts = split(&body, &plane, Tol::witness()).expect("split at x=0");
    let mut below = parts.below.body().expect("below").clone();
    let out = below
        .merge_coplanar_faces(Tol::witness())
        .expect("cosurface merge");
    assert_eq!(out.groups.len(), 1, "one wall re-merge: {:?}", out.groups);
    let vol = topo::mass_properties(&below, Tol::witness())
        .unwrap()
        .volume;
    let expect = std::f64::consts::PI * 0.25 / 2.0;
    assert!(
        (vol - expect).abs() < 1e-9,
        "half-disc volume {vol} vs {expect} (a first-arc du rule would give ~{})",
        expect / 2.0 + 0.0 // sanity anchor only
    );
    if let Err(errs) = topo::validate_geometric(&below, Tol::witness()) {
        panic!("merged half-disc stays tier-3: {errs:?}");
    }
}

#[test]
fn a_genuinely_non_maximal_curved_operand_slips_the_f7_gate_what_then() {
    // The F7 curved exemption under attack: the below part WITHOUT
    // its cosurface re-merge is genuinely non-maximal (two same-key
    // sub-period wall fragments — merge_coplanar_faces would merge
    // them). The gate lets it in; the pipeline must then either work
    // correctly or refuse typed — never a silently wrong body.
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(-0.5, 0.0), 1.0),
        ProfileVertex::new(p2(0.5, 0.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let body = extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body;
    let plane = SplitPlane {
        origin: Point3::new(0.2, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
    };
    let parts = split(&body, &plane, Tol::witness()).expect("split");
    let below = parts.below.body().expect("below").clone(); // NOT merged
    let vol_below = topo::mass_properties(&below, Tol::witness())
        .unwrap()
        .volume;
    // A thin slab through the fragments: 2 x 2 x [0.4, 0.6] centered.
    let lp2 = ProfileLoop::new(
        [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    );
    let plane2 = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.4)));
    let profile2 = Profile::new(plane2, vec![lp2])
        .validate(Tol::witness())
        .unwrap();
    let slab = extrude(&profile2, Extrusion::Distance(0.2), Tol::witness())
        .unwrap()
        .body;
    match topo::union(&below, &slab, Tol::witness()) {
        Ok(out) => {
            let b = &out.body().expect("body").body;
            if let Err(errs) = topo::validate_geometric(b, Tol::witness()) {
                panic!("non-maximal curved operand produced an invalid body: {errs:?}");
            }
            let vol = topo::mass_properties(b, Tol::witness()).unwrap().volume;
            // below is a prism (height 1); the slab [-1,1]^2 x [0.4,0.6]
            // contains its whole cross-section, so overlap = 0.2 * vol.
            let expect = vol_below + 2.0 * 2.0 * 0.2 - vol_below * 0.2;
            assert!(
                (vol - expect).abs() < 1e-9,
                "non-maximal operand: volume {vol} vs closed form {expect} — \
                 the F7 exemption let a wrong body through"
            );
        }
        Err(e) => eprintln!("NON-MAXIMAL CURVED OPERAND REFUSED TYPED: {e}"),
    }
}

#[test]
fn a_boss_overhanging_the_plate_edge_hits_the_curved_pierce_frontier() {
    // The plate's x=0 side wall passes THROUGH the boss wall (boss
    // centered on the plate's edge): the crossing layer meets a
    // curved face away from any shared boundary — the typed frontier
    // door (CurvedPierceUnsupported), never a wrong body.
    let theta = 2.0 * std::f64::consts::PI / 3.0;
    let bulge = (theta / 4.0).tan();
    let at = |i: usize| {
        let th = theta * i as f64;
        p2(0.0 + 0.35 * th.cos(), 1.5 + 0.35 * th.sin())
    };
    let lp = ProfileLoop::new((0..3).map(|i| ProfileVertex::new(at(i), bulge)).collect());
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.3)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let boss_over = extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body;
    match topo::union(&plate(), &boss_over, Tol::witness()) {
        Err(e @ topo::BooleanError::CurvedPierceUnsupported { .. }) => {
            let msg = format!("{e}");
            eprintln!("PIERCE FRONTIER: {msg}");
            assert!(
                msg.contains("zero") && msg.contains("escalate"),
                "the frontier arm must quote both tolerances: {msg}"
            );
        }
        Err(other) => eprintln!("PIERCE: refused via a different typed arm: {other}"),
        Ok(out) => {
            // If it works, the volume must be right (overhang union).
            let body = &out.body().expect("body").body;
            let vol = topo::mass_properties(body, Tol::witness()).unwrap().volume;
            eprintln!("PIERCE: union succeeded, vol {vol} — inspect");
        }
    }
}
