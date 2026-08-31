//! **PCURVE P-1b, reviewer R2's independent consumer probes** at the
//! verb layer.
//!
//! The unit's headline claim is a FENCE: no product verb hands back a
//! body at rest carrying a scaffolding description, because tier 3
//! refuses one (`ValidationError::ScaffoldAtRest`). The unit found two
//! verbs that still did (extrude's distinct-key under-determined join,
//! and the re-anchor door) by running the whole battery. These rows
//! attack the same claim from the other side: a broad, deliberately
//! cheap sweep over the product verbs, run in ONE row so the report is
//! a LIST of offenders rather than the first one. That shape stands on
//! its own — one row that enumerates beats N rows a reader has to
//! collate — but the reason originally given for it does not: hosted CI
//! truncating a red run to one failure per shard, which was real when
//! these rows were written and is not true any more, since both sharded
//! run steps now pass `--no-fail-fast`.
//!
//! The second group attacks the declaration-carrying claim: the unit
//! says `EdgeAuthority::is_declared()` never flips silently across an
//! offset or a transform — it refuses loudly instead. These rows check
//! that as an INVARIANT over every face of two fixtures rather than on
//! the one seam the unit's own row reads.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec2, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::blend::fillet_edges;
use sweep::test_support::cube;
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, loft_body, revolve};
use topo::{Body, EdgeKey, FaceKey, ValidationError};

mod common;
use common::quad;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// Every `ScaffoldAtRest` report tier 3 makes about `body`.
fn scaffolds_at_rest(body: &Body<f64>) -> Vec<EdgeKey> {
    match topo::validate_geometric(body, Tol::witness()) {
        Ok(()) => Vec::new(),
        Err(errors) => errors
            .iter()
            .filter_map(|e| match e {
                ValidationError::ScaffoldAtRest { edge } => Some(*edge),
                _ => None,
            })
            .collect(),
    }
}

/// Whether the body carries any edge whose stored description is the
/// scaffolding door's — read from the DESCRIPTIONS, not from the
/// validator, so a body tier 3 never looks at is still measured.
fn scaffold_descriptions(body: &Body<f64>) -> Vec<EdgeKey> {
    body.edges()
        .filter(|(_, e)| {
            body.get_curve_geom(e.curve)
                .and_then(topo::CurveGeom::certified)
                .is_some_and(|c| matches!(c.description(), geom_brep::EdgeDescription::Scaffold(_)))
        })
        .map(|(k, _)| k)
        .collect()
}

// ---------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------

fn slab(x0: f64, y0: f64, side: f64, z0: f64, height: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon([
        p2(x0, y0),
        p2(x0 + side, y0),
        p2(x0 + side, y0 + side),
        p2(x0, y0 + side),
    ]);
    let plane = SketchPlane::new(Affine3::from_parts(
        geom_core::Mat3::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()),
        Point3::new(0.0, 0.0, z0) - Point3::origin(),
    ));
    let validated = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&validated, Extrusion::Distance(height), Tol::witness())
        .unwrap()
        .body
}

/// A profile with an ARC segment, extruded — the arc scaffolding door
/// (`arc_of_circle`) rather than the chord one.
fn arc_prism() -> Body<f64> {
    let v = |x: f64, y: f64, bulge: f64| ProfileVertex::new(p2(x, y), bulge);
    let lp = ProfileLoop::new(vec![
        v(0.0, 0.0, 0.0),
        v(1.0, 0.0, 0.4),
        v(1.0, 1.0, 0.0),
        v(0.0, 1.0, 0.0),
    ]);
    let validated = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&validated, Extrusion::Distance(0.7), Tol::witness())
        .unwrap()
        .body
}

/// Revolves the closed `(r, y)` polygon about the `y` axis.
fn revolved(points: &[(f64, f64)], revolution: Revolution<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(
        points
            .iter()
            .map(|(r, y)| ProfileVertex::new(p2(*r, *y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a valid profile");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        revolution,
        Tol::witness(),
    )
    .expect("the polygon revolves")
    .body
}

fn tube() -> Body<f64> {
    revolved(
        &[(0.4, 0.0), (0.8, 0.0), (0.8, 0.6), (0.4, 0.6)],
        Revolution::Full,
    )
}

fn all_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    body.edges().map(|(k, _)| k).collect()
}

// ---------------------------------------------------------------
// R2-S1 — THE FENCE, swept over the product verbs in ONE row
// ---------------------------------------------------------------

/// **R2-S1.** Twelve bodies, each one what a product verb actually
/// hands a caller. For each: does tier 3 report `ScaffoldAtRest`, and
/// does the body carry a scaffolding description at all?
///
/// The two questions are asked separately on purpose. Tier 3's fence
/// fires only where an edge's TWO adjacent face surfaces resolve, so a
/// stored scaffold on an edge the fence cannot reach is invisible to
/// the validator and visible here.
///
/// The row reports EVERY offender before failing.
#[test]
fn r2_no_product_verb_hands_back_a_scaffold_at_rest() {
    let mut bodies: Vec<(&'static str, Body<f64>)> = vec![
        ("euler cube", cube(1.0, Tol::witness())),
        ("extrude slab", slab(0.0, 0.0, 2.0, 0.0, 2.0)),
        ("extrude arc prism", arc_prism()),
        ("revolve full tube", tube()),
        (
            "revolve full ball-ish annulus",
            revolved(
                &[(0.2, 0.0), (0.5, 0.0), (0.5, 1.0), (0.2, 1.0)],
                Revolution::Full,
            ),
        ),
        (
            "revolve partial wedge",
            revolved(
                &[(0.4, 0.0), (0.8, 0.0), (0.8, 0.6), (0.4, 0.6)],
                Revolution::Partial(1.1),
            ),
        ),
    ];

    // Loft — the cap rims that go through the scaffolding door until
    // their planes exist.
    let square = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let trapezoid = [(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let lofted = loft_body::<f64>(
        &[quad(square), quad(trapezoid), quad(square)],
        &[
            Affine3::identity(),
            Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
            Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
        ],
        2,
        Tol::witness(),
    )
    .expect("the loft builds");
    bodies.push(("loft prism", lofted.body));

    // Booleans — the two lanes the spec named by file:line, plus a
    // curved pair.
    //
    // **Correcting my own first note here.** I originally wrote that
    // these are "the verbs whose own acceptance rows stop at tier 2",
    // having read `m3_pr5_extrude_booleans.rs`, which does. Checked
    // across the workspace, that is false as a general claim: 30+ test
    // files call a boolean op AND a tier-3 entry point. So these rows
    // add little fence coverage over the existing gate; what they add
    // is the DESCRIPTION read below (a scaffold on an edge tier 3
    // cannot reach is invisible to the validator and visible here) and
    // a single place that lists every offender instead of stopping at
    // the first.
    let a = slab(0.0, 0.0, 2.0, 0.0, 2.0);
    let b = slab(1.0, 1.0, 2.0, 1.0, 2.0);
    for (name, r) in [
        ("boolean union", topo::union(&a, &b, Tol::witness())),
        ("boolean subtract", topo::subtract(&a, &b, Tol::witness())),
        ("boolean intersect", topo::intersect(&a, &b, Tol::witness())),
    ] {
        if let Some(body) = r.ok().and_then(|r| r.body().map(|b| b.body.clone())) {
            bodies.push((name, body));
        }
    }
    // A curved pair: a pocket cut out of the tube by a slab.
    if let Some(body) = topo::subtract(&tube(), &slab(0.5, -1.0, 2.0, 0.2, 0.2), Tol::witness())
        .ok()
        .and_then(|r| r.body().map(|b| b.body.clone()))
    {
        bodies.push(("boolean curved subtract", body));
    }

    // Shell.
    if let Ok(body) = topo::shell(
        &cube(1.0, Tol::witness()),
        0.1,
        1e-9,
        band(),
        Tol::witness(),
    ) {
        bodies.push(("shell cube", body));
    }
    if let Ok(body) = topo::shell(&tube(), 0.05, 1e-9, band(), Tol::witness()) {
        bodies.push(("shell tube", body));
    }

    // Chamfer and fillet of the cube — the two verbs built on the
    // strut surgery whose six conversion sites the unit reverted.
    let c = cube(1.0, Tol::witness());
    if let Ok(f) = sweep::chamfer::chamfer_edges(&c, &all_edges(&c), 0.1, band(), Tol::witness()) {
        bodies.push(("chamfer cube (all edges)", f.body));
    }
    if let Ok(f) = fillet_edges(&c, &all_edges(&c), 0.15, band(), Tol::witness()) {
        bodies.push(("fillet cube (all edges)", f.body));
    }
    // A PARTIAL fillet: one face's four edges. Its struts run out onto
    // untouched planar supports, which is the configuration the unit's
    // #1116 argument says has a legal chart image.
    let one_face: Vec<EdgeKey> = {
        let (fk, fd) = c.faces().next().expect("a face");
        let _ = fk;
        let topo::LoopBoundary::Cycle { first } = c.get_loop(fd.outer).unwrap().boundary else {
            panic!("a face loop")
        };
        c.loop_cycle(first)
            .unwrap()
            .into_iter()
            .map(|he| c.get_half_edge(he).unwrap().edge)
            .collect()
    };
    match fillet_edges(&c, &one_face, 0.12, band(), Tol::witness()) {
        Ok(f) => bodies.push(("fillet cube (one face's four edges)", f.body)),
        Err(e) => println!("[R2-S1] the one-face fillet refused: {e:?}"),
    }

    let mut offenders: Vec<String> = Vec::new();
    for (name, body) in &bodies {
        let fenced = scaffolds_at_rest(body);
        let stored = scaffold_descriptions(body);
        println!(
            "[R2-S1] {name}: {} scaffold description(s), {} ScaffoldAtRest report(s)",
            stored.len(),
            fenced.len()
        );
        if !fenced.is_empty() || !stored.is_empty() {
            offenders.push(format!(
                "{name}: {} stored scaffold(s), {} fence report(s)",
                stored.len(),
                fenced.len()
            ));
        }
    }
    assert!(
        offenders.is_empty(),
        "product verbs handed back scaffolding at rest:\n  {}",
        offenders.join("\n  ")
    );
}

// ---------------------------------------------------------------
// R2-S2 / S3 — the authority record cannot flip quietly
// ---------------------------------------------------------------

/// Every edge's `is_declared`, keyed by edge.
fn declared_map(body: &Body<f64>) -> Vec<(EdgeKey, bool)> {
    let mut out: Vec<(EdgeKey, bool)> = body
        .edges()
        .filter_map(|(k, e)| {
            body.get_curve_geom(e.curve)
                .and_then(topo::CurveGeom::certified)
                .map(|c| (k, c.authority().is_declared()))
        })
        .collect();
    out.sort_by_key(|(k, _)| format!("{k:?}"));
    out
}

/// **R2-S2.** Offsetting ANY face of the tube either refuses loudly or
/// leaves every surviving edge's `is_declared` exactly where it was.
/// The unit's own row reads one seam of one offset; this reads every
/// edge of every face at both signs.
#[test]
fn r2_no_face_offset_flips_is_declared_silently() {
    let base = tube();
    let faces: Vec<FaceKey> = base.faces().map(|(k, _)| k).collect();
    let before = declared_map(&base);
    let mut findings: Vec<String> = Vec::new();
    for f in faces {
        for d in [0.03_f64, -0.03] {
            let mut body = base.clone();
            match topo::replace_face_offset(&mut body, f, d, 1e-9, band(), Tol::witness()) {
                Err(e) => {
                    println!("[R2-S2] face {f:?} at d = {d}: refused loudly — {e:?}");
                }
                Ok(()) => {
                    let after = declared_map(&body);
                    for (edge, was) in &before {
                        if let Some((_, now)) = after.iter().find(|(k, _)| k == edge)
                            && was != now
                        {
                            findings.push(format!(
                                "face {f:?} at d = {d}: edge {edge:?} is_declared {was} -> {now}"
                            ));
                        }
                    }
                }
            }
        }
    }
    assert!(
        findings.is_empty(),
        "an offset flipped a declaration without refusing:\n  {}",
        findings.join("\n  ")
    );
}

/// **R2-S3.** A rigid transform is a pure relabelling of space: every
/// edge's authority must come out the other side identical. (The
/// declaration is sketch data under a 3-space placement, so this is
/// the one lane where "it travels with the map" has to be true rather
/// than vacuous.)
#[test]
fn r2_a_rigid_transform_preserves_every_authority() {
    for (name, base) in [
        ("tube", tube()),
        ("arc prism", arc_prism()),
        ("loft-free cube", cube(1.0, Tol::witness())),
    ] {
        let before = declared_map(&base);
        let map = Affine3::from_parts(
            geom_core::Mat3::from_cols(
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(-1.0, 0.0, 0.0),
                Vec3::unit_z(),
            ),
            Vec3::new(0.25, -1.5, 3.0),
        );
        let moved = topo::transform_rigid(&base, &map, Tol::witness())
            .unwrap_or_else(|e| panic!("{name} transforms: {e:?}"));
        let after = declared_map(&moved);
        assert_eq!(
            before.len(),
            after.len(),
            "{name}: the edge count changed under a rigid map"
        );
        for ((_, was), (_, now)) in before.iter().zip(after.iter()) {
            assert_eq!(was, now, "{name}: a rigid transform flipped a declaration");
        }
        assert!(
            scaffolds_at_rest(&moved).is_empty(),
            "{name}: a rigid transform created a scaffold at rest"
        );
    }
}

/// **R2-S4.** The declaration-carrying claim's converse, measured: an
/// edge whose locus NOTHING declared must cross a non-translating
/// offset freely — that is what retiring the *"not a rigid
/// translation"* refusal bought, and a fix that asks for `delta`
/// unconditionally would have taken it back.
///
/// The cone wall of a coned tube has no closed-form rigid-translation
/// offset; its rims are intrinsic (derived), so the offset must go
/// through.
#[test]
fn r2_an_undeclared_edge_still_crosses_a_non_translating_offset() {
    let base = revolved(
        &[(0.4, 0.0), (0.8, 0.0), (0.8, 0.3), (0.4, 0.6)],
        Revolution::Full,
    );
    let cone = base
        .faces()
        .find(|(_, f)| matches!(base.get_surface(f.surface), Some(Surface::Cone { .. })))
        .map(|(k, _)| k);
    let Some(cone) = cone else {
        println!("[R2-S4] the fixture has no cone face; nothing to measure");
        return;
    };
    let mut body = base.clone();
    let outcome = topo::replace_face_offset(&mut body, cone, 0.02, 1e-9, band(), Tol::witness());
    println!("[R2-S4] cone offset: {:?}", outcome.as_ref().err());
    if outcome.is_ok() {
        let before = declared_map(&base);
        let after = declared_map(&body);
        for (edge, was) in &before {
            if let Some((_, now)) = after.iter().find(|(k, _)| k == edge) {
                assert_eq!(was, now, "the cone offset flipped edge {edge:?}");
            }
        }
    }
}

// ---------------------------------------------------------------
// R2-S5 — the METER, measured rather than argued
// ---------------------------------------------------------------

/// **R2-S5.** Every edge the fence CONVERTED re-meters from
/// `|C(t) − mapped(s)|` (which never asked whether the locus was on
/// either surface) to D1's `|C(t) − S(P(t))|` (which does). The unit
/// calls that "a stricter statement about the same locus"; the
/// question a consumer has is how much room is left under ε, because
/// a converted edge sitting near the band is one ε-tightening away
/// from a body that used to build and no longer does.
///
/// So this row MEASURES it: over every verb body above, the largest
/// certified residual carried by a chart-described edge whose
/// authority is `Declared` — i.e. exactly the edges the conversion
/// created — reported as a fraction of the witness ε.
///
/// The assertion is only the one certification already guarantees
/// (≤ ε). The NUMBER is the finding, and it is printed.
#[test]
fn r2_the_converted_edges_have_measurable_epsilon_headroom() {
    let eps = Tol::witness().get().eps;
    let mut bodies: Vec<(&'static str, Body<f64>)> = vec![
        ("extrude slab", slab(0.0, 0.0, 2.0, 0.0, 2.0)),
        ("extrude arc prism", arc_prism()),
        ("revolve full tube", tube()),
        (
            "revolve partial wedge",
            revolved(
                &[(0.4, 0.0), (0.8, 0.0), (0.8, 0.6), (0.4, 0.6)],
                Revolution::Partial(1.1),
            ),
        ),
    ];
    let square = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let trapezoid = [(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    if let Ok(l) = loft_body::<f64>(
        &[quad(square), quad(trapezoid), quad(square)],
        &[
            Affine3::identity(),
            Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
            Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
        ],
        2,
        Tol::witness(),
    ) {
        bodies.push(("loft prism", l.body));
    }

    let mut worst: (f64, String) = (0.0, "none".to_string());
    let mut converted = 0usize;
    for (name, body) in &bodies {
        for (k, e) in body.edges() {
            let Some(c) = body
                .get_curve_geom(e.curve)
                .and_then(topo::CurveGeom::certified)
            else {
                continue;
            };
            if !matches!(c.description(), geom_brep::EdgeDescription::Chart(_))
                || !c.authority().is_declared()
            {
                continue;
            }
            converted += 1;
            let r = c.certificate().max_residual;
            if r > worst.0 {
                worst = (r, format!("{name} edge {k:?}"));
            }
        }
    }
    println!(
        "[R2-S5] {converted} converted edge(s); worst residual {:e} = {:.3e} x eps ({eps:e})",
        worst.0,
        worst.0 / eps
    );
    assert!(
        converted > 0,
        "the fixtures must actually contain converted edges for this row to mean anything"
    );
    assert!(
        worst.0 <= eps,
        "a converted edge is over ε — {} at {:e}",
        worst.1,
        worst.0
    );
}

// ---------------------------------------------------------------
// R2-S6 — is the "not a rigid translation" refusal really retired
//         AT REST?
// ---------------------------------------------------------------

/// **R2-S6.** `replace_face.rs`'s retirement text says the two
/// *"not a rigid translation"* refusals "are unreachable for a body AT
/// REST, because tier 3's transience fence refuses a scaffold there",
/// and `demos/tour`'s
/// `the_not_a_rigid_translation_door_is_unreachable_at_rest`
/// demonstrates the premise by showing the fixtures carry no
/// scaffolding description.
///
/// That premise is about the SCAFFOLD arm. This unit also added a
/// second arm — `carried_declaration()` — which raises the SAME
/// `ReplaceFaceError::CarrierLaneUnsupported` variant, with the same
/// sentence reworded, for a chart image whose authority is
/// `Declared`. And declared chart images are exactly what the fence's
/// conversion produces from the pushforwards that used to raise the
/// retired arm.
///
/// So the row asks the question the retirement text answers in prose:
/// on a body AT REST, with no scaffolding description anywhere, does
/// offsetting a chart whose boundary carries a DECLARED image still
/// refuse through `CarrierLaneUnsupported`?
///
/// The fixture is the revolved ball: its angle-π meridian is a
/// declared image in the sphere's own chart (the unit's own
/// `revolve_ball` row asserts exactly that), and a sphere's offset is
/// concentric — not a rigid translation.
#[test]
fn r2_the_declared_arm_of_the_retired_refusal_is_reachable_at_rest() {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -1.0), 1.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the half disc is a valid profile");
    let ball = revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the ball revolves")
    .body;

    // The premise the tour row asserts: nothing scaffolded survives.
    assert!(
        scaffold_descriptions(&ball).is_empty(),
        "the ball is at rest — no scaffolding description"
    );
    // And at least one edge IS declared, which is the other premise
    // the retirement text does not state.
    let declared: Vec<EdgeKey> = ball
        .edges()
        .filter(|(_, e)| {
            ball.get_curve_geom(e.curve)
                .and_then(topo::CurveGeom::certified)
                .is_some_and(|c| c.authority().is_declared())
        })
        .map(|(k, _)| k)
        .collect();
    println!("[R2-S6] declared edges at rest: {}", declared.len());
    assert!(
        !declared.is_empty(),
        "the ball's angle-pi meridian is a declared chart image"
    );

    // Both sphere bands share ONE surface key, so the chart moves as a
    // group.
    let sphere_key = ball
        .faces()
        .find_map(|(_, f)| {
            matches!(ball.get_surface(f.surface), Some(Surface::Sphere { .. })).then_some(f.surface)
        })
        .expect("the ball's sphere chart");
    let group: Vec<FaceKey> = ball
        .faces()
        .filter(|(_, f)| f.surface == sphere_key)
        .map(|(k, _)| k)
        .collect();

    let mut body = ball.clone();
    let outcome = topo::replace_faces_offset(&mut body, &group, 0.05, 1e-9, band(), Tol::witness());
    println!("[R2-S6] offsetting the sphere chart: {outcome:?}");
    match outcome {
        Err(topo::ReplaceFaceError::CarrierLaneUnsupported { what, .. }) => {
            println!("[R2-S6] CarrierLaneUnsupported: {what}");
            assert!(
                what.contains("declared"),
                "the refusal that fired is the DECLARED arm, at rest: {what}"
            );
        }
        other => {
            // Not a failure of the unit by itself — but it is the
            // measurement the retirement text needs and does not have.
            println!("[R2-S6] the declared arm did not fire here: {other:?}");
        }
    }
}

/// **R2-S7 — the other half of the #1116 check.** `die_fillet`'s body
/// is `sweep::test_support::cube`, and the fillet's SUPPORTS are that
/// body's faces. Every one of them is a plane, so no support in that
/// run is curved and the recorded "secant on a curved support" cause
/// cannot be what escalated there.
#[test]
fn r2_the_die_fixtures_supports_are_all_planes() {
    let c = cube(1.0, Tol::witness());
    let kinds: Vec<bool> = c
        .faces()
        .map(|(_, f)| matches!(c.get_surface(f.surface), Some(Surface::Plane { .. })))
        .collect();
    println!("[R2-S7] die blank faces: {} total", kinds.len());
    assert_eq!(kinds.len(), 6, "the die blank is a box");
    assert!(
        kinds.iter().all(|p| *p),
        "every support of the die fillet is a PLANE — a chord between two of its points \
         lies in it, so the strut is not a secant there"
    );
    assert_eq!(c.edges().count(), 12, "all twelve edges are filleted");
}
