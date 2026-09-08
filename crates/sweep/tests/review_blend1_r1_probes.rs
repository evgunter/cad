//! Reviewer probes for BLEND unit 1 (PR 2123): the coaxiality arm in
//! `BlendError::Escalated`'s recourse dispatch, and the fixture the
//! trio row reaches it through.
//!
//! Each row states what it measures about the tree; none proposes a
//! fix.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::approx::band;
use geom_core::{Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::blend::battery::{BlendRequest, convexity_at, run_battery};
use sweep::blend::{BlendError, BlendSite};
use sweep::{Extrusion, extrude};
use topo::{Body, EdgeKey, FaceSurface};

fn tol() -> Tol {
    Tol::witness()
}

fn in_band() -> f64 {
    5.0 * tol().eps()
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The same three-arc cylinder `m5_pr12_refusals` builds.
fn cylinder() -> Body<f64> {
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th: f64 = deg.to_radians();
        p2(0.5 * th.cos(), 0.5 * th.sin())
    };
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(at(0.0), b120),
        ProfileVertex::new(at(120.0), b120),
        ProfileVertex::new(at(240.0), b120),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body
}

/// A cylinder whose rim is TWO semicircles — the shape every closed-rim
/// suite in the tree builds.
fn two_arc_cylinder() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.5, 0.0), 1.0),
        ProfileVertex::new(p2(-0.5, 0.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0), tol())
        .unwrap()
        .body
}

/// The unit's fixture, restated here so the probes read the same
/// body the trio row does. Returns the rim, the cap face, and the
/// rim circle's (axis, radius) as stored.
fn tilted_cap(departure: f64) -> (Body<f64>, EdgeKey, topo::FaceKey, Vec3<f64>, f64) {
    tilt_raised_cap(cylinder(), departure)
}

/// The unit's tilt, on any extruded body whose raised rim is circular.
fn tilt_raised_cap(
    mut body: Body<f64>,
    departure: f64,
) -> (Body<f64>, EdgeKey, topo::FaceKey, Vec3<f64>, f64) {
    let (rim, axis, lever) = body
        .edges()
        .find_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            match c.carrier() {
                geom::Curve3::Circle {
                    center,
                    radius,
                    axis,
                    ..
                } if center.z > 0.5 => Some((k, *axis, *radius)),
                _ => None,
            }
        })
        .expect("the raised rim of an extruded circle");
    let sides = {
        let e = body.get_edge(rim).expect("the rim resolves");
        [e.he_plus, e.he_minus]
    };
    let cap = sides
        .into_iter()
        .find_map(|he| {
            let h = body.get_half_edge(he)?;
            let f = body.get_loop(h.parent_loop)?.face;
            matches!(
                body.get_surface(body.get_face(f)?.surface)?,
                geom::Surface::Plane { .. }
            )
            .then_some(f)
        })
        .expect("the cap side of the rim");
    let geom::Surface::Plane {
        origin,
        normal,
        u_ref,
    } = *body
        .get_surface(body.get_face(cap).expect("the cap resolves").surface)
        .expect("the cap's plane")
    else {
        panic!("the cap side is a plane")
    };
    let theta = (departure / lever).asin();
    let tilted = geom::Surface::Plane {
        origin,
        normal: normal * theta.cos() + u_ref.cross(normal) * theta.sin(),
        u_ref,
    };
    body.set_face_surface(cap, FaceSurface::New(tilted))
        .expect("a plane for a planar cap");
    (body, rim, cap, axis, lever)
}

fn verdict(body: &Body<f64>, rim: EdgeKey) -> Result<(), BlendError> {
    run_battery(
        &BlendRequest {
            body,
            edges: vec![rim],
            size: 0.05,
        },
        band(),
    )
    .map(|_| ())
}

/// **The fixture's departure IS the meridian reading.** The cap's new
/// normal parts from the rim circle's stored axis by exactly
/// `departure` at the rim's lever, in the same arithmetic
/// `arms::Meridian::trace` uses for a plane (`|n × k| · lever`), so
/// the trio row's `5ε` lands where the row says it does and not
/// somewhere a nearby number happens to be classified the same way.
#[test]
fn r1_tilted_cap_departure_is_the_meridian_reading() {
    for departure in [1e-3, in_band(), 2.0 * tol().eps(), 0.5 * tol().eps()] {
        let (body, _, cap, axis, lever) = tilted_cap(departure);
        let geom::Surface::Plane { normal, .. } = *body
            .get_surface(body.get_face(cap).unwrap().surface)
            .unwrap()
        else {
            panic!("a plane")
        };
        let read = normal.cross(axis).norm() * lever;
        assert!(
            (read - departure).abs() <= 1e-9 * departure.max(1e-300),
            "departure {departure:e} reads back as {read:e}"
        );
        assert!(
            (normal.norm() - 1.0).abs() < 1e-15,
            "the tilted normal stays unit"
        );
    }
}

/// **What the certified door certifies, measured.** `set_face_surface`
/// runs tier 1 only; the tilted body still passes tier 2 (closed
/// solid) at every departure, and tier 3 — the planar-face residual
/// and the rim's own certification — is the layer that sees the
/// tilt. The row records which tier-3 checks fire at the definite
/// tilt and at the in-band one, so "a real body reached through a
/// public door" is stated with its exact validity.
#[test]
fn r1_tilted_cap_is_tier2_valid_and_tier3_names_the_tilt() {
    for departure in [0.0, in_band(), 1e-3] {
        let (body, _, _, _, _) = tilted_cap(departure);
        assert_eq!(
            topo::validate_closed(&body),
            Ok(()),
            "tier 2 at departure {departure:e}"
        );
        let errs = topo::validate_geometric(&body, tol())
            .expect_err("a re-minted cap surface key is what tier 3 sees, at every departure");
        let names: Vec<String> = errs
            .iter()
            .map(|e| format!("{e:?}").split(' ').next().unwrap().to_string())
            .collect();
        eprintln!("departure {departure:e}: tier 3 names {names:?}");
        // FaceSurface::New mints a fresh key and the three rim arcs'
        // Intersection descriptions still name the old one — so even
        // the ZERO leg is not the untouched extrusion.
        assert!(
            names
                .iter()
                .filter(|n| n.contains("DescriptionNotAdjacent"))
                .count()
                == 3,
            "{names:?}"
        );
        // The tilt itself is what the planar checks see: in band they
        // ESCALATE (PlanarFaceEscalated / PlanarBoundaryEscalated), at
        // the definite tilt they refuse.
        let planar: Vec<&String> = names.iter().filter(|n| n.contains("Planar")).collect();
        assert_eq!(planar.is_empty(), departure == 0.0, "{names:?}");
        if departure > 0.0 {
            let escalated = planar.iter().all(|n| n.contains("Escalated"));
            assert_eq!(escalated, departure < 1e-3, "{names:?}");
        }
    }
}

/// **The battery reaches the coaxiality question before tier 3
/// would refuse the body**, which is the premise the trio row rests
/// on: the departure the fixture writes is read by
/// `fillet3_support_coaxiality`, not swallowed by an earlier
/// predicate. Both non-zero legs classify on that predicate; the
/// zero leg builds.
#[test]
fn r1_the_coaxiality_predicate_is_the_first_to_speak_on_the_tilted_cap() {
    // ONE arc of the rim at departure 0: the pair is coaxial, and the
    // refusal that follows is the open chain's termination at the
    // rim's seam vertices — not a build. The trio row's zero leg is
    // satisfied by this refusal.
    let (body, rim, _, _, _) = tilted_cap(0.0);
    let zero = verdict(&body, rim).unwrap_err();
    eprintln!("one arc at departure 0: {zero:?}");
    assert!(
        matches!(zero, BlendError::UnsupportedCorner { .. }),
        "{zero:?}"
    );
    let (body, rim, _, _, _) = tilted_cap(in_band());
    match verdict(&body, rim) {
        Err(BlendError::Escalated {
            site: BlendSite::Chain,
            source,
        }) => {
            assert_eq!(source.predicate, Some("fillet3_support_coaxiality"));
            let geom_core::MarginDiag::Value(v) = source.margin else {
                panic!("an f64 reading")
            };
            assert!(
                (v - in_band()).abs() < 1e-24,
                "the margin is the departure: {v:e}"
            );
        }
        other => panic!("{other:?}"),
    }
    // Just under the zero threshold: Sign::Zero, coaxial, and the
    // same corner refusal as the zero leg.
    let (body, rim, _, _, _) = tilted_cap(0.5 * tol().eps());
    let sub = verdict(&body, rim).unwrap_err();
    assert!(
        matches!(sub, BlendError::UnsupportedCorner { .. }),
        "{sub:?}"
    );
}

/// **A CHARACTERIZATION row: it pins a defect, and goes red when the
/// defect is fixed.** The whole raised rim of a THREE-arc cylinder is
/// refused `ChainNotG1` at a junction with a 120° reading (margin
/// 0.75 at arm 0.866), while the same rim built from TWO semicircles
/// carves. Nothing about the body differs: `walk_chains` lists a
/// closed chain's junctions as `[closing vertex, j01, j12]` against
/// links `[0, 1, 2]`, so the junction check pairs each vertex with
/// one link that does not touch it and reads that link's FAR-end
/// tangent. A two-arc rim is immune because both links touch both
/// vertices — which is why every closed-rim suite in the tree happens
/// to build one and the defect has gone unseen.
///
/// The item is `work/blend/closed-chain-junctions-pair-with-a-rotated-link`.
/// When it lands this row's `ChainNotG1` assertions fail, and that is
/// the intended signal: delete the row, or turn it into the row that
/// pins the fix.
#[test]
fn r1_a_three_arc_rim_refuses_chain_g1_at_a_junction_where_a_two_arc_rim_builds() {
    // `topo::query::rim_of` refuses the re-keyed body (`NotOneRim`:
    // the arcs' descriptions name the old cap key), so the arcs are
    // gathered by their carriers.
    let raised_arcs = |body: &Body<f64>| -> Vec<EdgeKey> {
        body.edges()
            .filter_map(|(k, e)| {
                let c = body.get_curve_geom(e.curve)?.certified()?;
                match c.carrier() {
                    geom::Curve3::Circle { center, .. } if center.z > 0.5 => Some(k),
                    _ => None,
                }
            })
            .collect()
    };
    let whole = |body: &Body<f64>, arcs: Vec<EdgeKey>| -> Result<(), BlendError> {
        run_battery(
            &BlendRequest {
                body,
                edges: arcs,
                size: 0.05,
            },
            band(),
        )
        .map(|_| ())
    };
    // Control: a TWO-semicircle cylinder's raised rim, both arcs. It
    // passes the battery and carves through the public door.
    let two = two_arc_cylinder();
    let arcs2 = raised_arcs(&two);
    assert_eq!(arcs2.len(), 2);
    whole(&two, arcs2.clone()).expect("the two-arc rim resolves");
    assert!(
        sweep::blend::fillet_edges(&two, &arcs2, 0.05, tol()).is_ok(),
        "the two-arc rim carves"
    );
    // The defect: the PRISTINE three-arc extrusion, whole rim.
    let pristine = cylinder();
    let arcs3 = raised_arcs(&pristine);
    assert_eq!(arcs3.len(), 3);
    let chain_g1_at_a_junction = |r: &Result<(), BlendError>| match r {
        Err(BlendError::ChainNotG1 { margin, arm, .. }) => {
            // 120° between the two carriers' tangents, folded against
            // the smaller link's extent: sin(120°) · 0.866 = 0.75.
            assert!(
                margin.value().is_some_and(|v| (v - 0.75).abs() < 1e-12),
                "{margin:?}"
            );
            let geom_core::MarginDiag::Value(a) = arm else {
                panic!("an f64 arm")
            };
            assert!((a - 3f64.sqrt() / 2.0).abs() < 1e-12, "{arm:?}");
        }
        other => panic!("the three-arc rim refuses at a junction, got {other:?}"),
    };
    chain_g1_at_a_junction(&whole(&pristine, arcs3.clone()));
    assert!(
        matches!(
            sweep::blend::fillet_edges(&pristine, &arcs3, 0.05, tol()),
            Err(sweep::blend::BlendRefusal {
                error: BlendError::ChainNotG1 { .. },
                ..
            })
        ),
        "the public door carries the same refusal"
    );
    // The unit's re-keyed body reads the same at departure 0 — the
    // junction check runs after the coaxiality legs, so the tilt does
    // not reach this.
    let (body, _, _, _, _) = tilted_cap(0.0);
    chain_g1_at_a_junction(&whole(&body, raised_arcs(&body)));
}

/// **The in-band convexity-sign escalation renders the TANGENTIAL
/// sentence, not the chain-flip one.** A 5ε wedge at a 1 m arm
/// escalates `fillet3_convexity_sign` at the link's own lever; that
/// is the same site and the same user situation as the wedge decided
/// `Zero`, whose refusal is `TangentialEdge`, so the two arms carry
/// one recourse (D4 ¶1 addendum). "Split the chain at the convexity
/// flip" belongs to `ConvexitySignFlip` — a chain whose links all
/// resolved definitely and disagree — and is advice about a flip no
/// escalation here decided; this row pins its absence.
#[test]
fn r1_in_band_convexity_sign_renders_the_tangential_sentence() {
    let body = cylinder();
    let e = body.edges().next().unwrap().0;
    let tau = Vec3::new(0.0, 0.0, 1.0);
    let escalated = convexity_at(
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, in_band(), 0.0).normalize(),
        tau,
        1.0,
        e,
        band(),
    )
    .unwrap_err();
    let text = format!("{escalated}");
    assert!(
        matches!(&escalated, BlendError::Escalated { source, .. }
            if source.predicate == Some("fillet3_convexity_sign")),
        "{escalated:?}"
    );
    assert!(
        text.contains(sweep::blend::FILLET3_TANGENTIAL_RECOURSE),
        "the decided-Zero sibling's sentence: {text}"
    );
    assert!(
        !text.contains(sweep::blend::FILLET3_CONVEXITY_RECOURSE),
        "no flip was decided, so the flip sentence is absent: {text}"
    );
    // The definite neighbour, rendered from the same site: a wedge
    // decided Zero. Both arms, one sentence.
    let flat = convexity_at(
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        tau,
        1.0,
        e,
        band(),
    )
    .unwrap_err();
    assert!(
        matches!(flat, BlendError::TangentialEdge { .. }),
        "{flat:?}"
    );
    assert!(
        format!("{flat}").contains(sweep::blend::FILLET3_TANGENTIAL_RECOURSE),
        "{flat}"
    );
}

/// **Can a BODY reach the in-band convexity arm?** A profile whose
/// vertex turns by an in-band angle is the natural fixture; this row
/// records what the profile validator and the battery say about it.
#[test]
fn r1_a_near_collinear_profile_vertex_and_the_convexity_arm() {
    // Two levers. The profile's `chord_side` meters the vertex's
    // perpendicular miss from the previous chord (d), the battery's
    // `fillet3_convexity_sign` meters sin(turn) · the edge's own
    // extent (≈ d / L · h). Separating them by L and h puts the
    // first definitely off zero and the second in band.
    for (d, l, h) in [(in_band(), 1.0, 1.0), (1e-6, 10.0, 0.05), (1e-6, 10.0, 0.5)] {
        let lp = ProfileLoop::new(
            [
                (0.0, 0.0),
                (1.0, 0.0),
                (1.0 + l, d),
                (1.0 + l, 1.0),
                (0.0, 1.0),
            ]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
        );
        let profile = match Profile::new(SketchPlane::xy(), vec![lp]).validate(tol()) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("(d={d:e}, L={l}, h={h}) the profile door refuses: {e}");
                continue;
            }
        };
        let body = match extrude(&profile, Extrusion::Distance(h), tol()) {
            Ok(b) => b.body,
            Err(e) => {
                eprintln!("(d={d:e}, L={l}, h={h}) the extrude door refuses: {e}");
                continue;
            }
        };
        let edge = body
            .edges()
            .find_map(|(k, e)| {
                let c = body.get_curve_geom(e.curve)?.certified()?;
                match c.carrier() {
                    geom::Curve3::Line { origin, dir } => ((origin.x - 1.0).abs() < 1e-12
                        && origin.y.abs() < 1e-6
                        && dir.z.abs() > 0.5)
                        .then_some(k),
                    _ => None,
                }
            })
            .expect("the vertical edge at the near-collinear vertex");
        let r = verdict(&body, edge);
        eprintln!("(d={d:e}, L={l}, h={h}) battery on the wedge: {r:?}");
        if let Err(e @ BlendError::Escalated { .. }) = &r {
            eprintln!("  renders: {e}");
        }
    }
}

/// **The trio on a rim whose zero leg BUILDS.** On the two-semicircle
/// cylinder the whole raised rim is a two-link closed chain the
/// battery admits, so the exact leg is a pass rather than a
/// downstream refusal, and the three legs read build / escalate /
/// refuse on one body — the shape `trio_chain_g1` gives its
/// coincidence predicate.
#[test]
fn r1_two_arc_tilted_rim_builds_at_zero_escalates_in_band_and_refuses_definitely() {
    let arcs_of = |body: &Body<f64>| -> Vec<EdgeKey> {
        body.edges()
            .filter_map(|(k, e)| {
                let c = body.get_curve_geom(e.curve)?.certified()?;
                match c.carrier() {
                    geom::Curve3::Circle { center, .. } if center.z > 0.5 => Some(k),
                    _ => None,
                }
            })
            .collect()
    };
    let whole = |departure: f64| -> Result<(), BlendError> {
        let (body, _, _, _, _) = tilt_raised_cap(two_arc_cylinder(), departure);
        let arcs = arcs_of(&body);
        assert_eq!(arcs.len(), 2);
        run_battery(
            &BlendRequest {
                body: &body,
                edges: arcs,
                size: 0.05,
            },
            band(),
        )
        .map(|_| ())
    };
    whole(0.0).expect("the exactly coaxial rim resolves");
    let escalated = whole(in_band()).unwrap_err();
    assert!(
        matches!(&escalated, BlendError::Escalated { site: BlendSite::Chain, source }
            if source.predicate == Some("fillet3_support_coaxiality")),
        "{escalated:?}"
    );
    let definite = whole(1e-3).unwrap_err();
    assert!(
        matches!(definite, BlendError::SpineUnsupported { .. }),
        "{definite:?}"
    );
    let (d, e) = (format!("{definite}"), format!("{escalated}"));
    assert!(d.contains(sweep::blend::FILLET3_SPINE_KIND_RECOURSE), "{d}");
    assert!(e.contains(sweep::blend::FILLET3_SPINE_KIND_RECOURSE), "{e}");
}

/// **A tilted sketch plane through the boolean.** The one public
/// route to a cylinder whose stored axis is NOT the world normal is a
/// tilted `SketchPlane`; a boss extruded from a plane tilted by an
/// in-band angle and unioned onto a flat cap would mint a
/// cylinder–plane rim with an in-band coaxiality departure — if the
/// boolean admits it. This row records which door answers first.
#[test]
fn r1_a_boss_on_an_in_band_tilted_sketch_plane_through_the_union() {
    let base = {
        let lp = ProfileLoop::new(
            [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
                .into_iter()
                .map(|(x, y)| ProfileVertex::new(p2(x, y), 0.0))
                .collect(),
        );
        let profile = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(tol())
            .unwrap();
        extrude(&profile, Extrusion::Distance(1.0), tol())
            .unwrap()
            .body
    };
    // The boss: a 0.5-radius cylinder standing on z = 1, its sketch
    // plane turned about x by an angle whose departure at the rim's
    // lever (0.25 m) is in band.
    for (departure, z0) in [
        (0.0, 1.0),
        (0.0, 0.5),
        (in_band(), 1.0),
        (1e-3, 1.0),
        (5e-2, 1.0),
        (5e-2, 0.5),
    ] {
        eprintln!("-- departure {departure:e}, boss sketch plane at z = {z0}");
        let theta = (departure / 0.25).asin();
        let u = Vec3::new(1.0, 0.0, 0.0);
        let v = Vec3::new(0.0, theta.cos(), theta.sin());
        let plane = SketchPlane::from_frame(geom_core::Point3::new(0.0, 0.0, z0), u, v);
        let b120 = (core::f64::consts::PI / 6.0).tan();
        let at = |deg: f64| {
            let th: f64 = deg.to_radians();
            p2(0.25 * th.cos(), 0.25 * th.sin())
        };
        let lp = ProfileLoop::new(vec![
            ProfileVertex::new(at(0.0), b120),
            ProfileVertex::new(at(120.0), b120),
            ProfileVertex::new(at(240.0), b120),
        ]);
        let profile = Profile::new(plane, vec![lp]).validate(tol()).unwrap();
        let boss = extrude(&profile, Extrusion::Distance(1.0), tol())
            .expect("a boss on a tilted plane")
            .body;
        let r = topo::boolean::union(&base, &boss, tol());
        match r {
            Ok(out) => {
                eprintln!("the union built; walking its cylinder–plane rims");
                let body = &out.body().expect("a non-empty union").body;
                let mut seen = 0;
                for (k, e) in body.edges() {
                    let Some(c) = body.get_curve_geom(e.curve).and_then(|c| c.certified()) else {
                        continue;
                    };
                    if !matches!(c.carrier(), geom::Curve3::Circle { .. }) {
                        continue;
                    }
                    seen += 1;
                    let v = verdict(body, k);
                    eprintln!("rim {k:?}: {v:?}");
                }
                assert!(seen > 0, "the union has a circular rim");
            }
            Err(e) => {
                eprintln!("departure {departure:e}: the boolean answers first: {e}");
            }
        }
    }
}
