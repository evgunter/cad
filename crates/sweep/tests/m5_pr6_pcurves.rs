//! **M5 PR 6 acceptance**: pcurves as per-half-edge certified caches,
//! at body level (spec §6). The value-level rows live in `geom-brep`
//! (`pcurve_cache`'s module tests, `tests/pcurve_parameter_finding.rs`);
//! these are the rows that need real bodies — which is why they live in
//! `sweep`, the lowest crate that can build a cylinder.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;
use profile::RawLoop;

use geom::Surface;
use geom_core::Tol;
use geom_core::{Band, Point2, Point3, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::splitting::{SplitPart, SplitPlane, split};
use topo::{Body, Pcurve, validate_geometric};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The corpus shape (i) profile: a radius-0.5 disc as two half-circle
/// arcs — extrudes to a cylinder whose two wall faces share one
/// cylinder surface.
fn disc() -> ValidatedProfile<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(-0.5, 0.0), 1.0),
        ProfileVertex::new(p2(0.5, 0.0), 1.0),
    ]);
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap()
}

/// A square profile in the xz-ish sketch plane, revolved a full turn
/// about the y axis: the outer wall is ONE cylinder face closed by its
/// own seam meridian — the edge whose two half-edges lie in one loop of
/// one face, on one surface.
fn revolved_tube() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.4, 0.0), 0.0),
        ProfileVertex::new(p2(0.8, 0.0), 0.0),
        ProfileVertex::new(p2(0.8, 0.6), 0.0),
        ProfileVertex::new(p2(0.4, 0.6), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    revolve(&profile, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

fn cylinder_body() -> Body<f64> {
    extrude(&disc(), Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body
}

/// The corpus shape (i) cut: a tilted plane through a cylinder.
fn tilted_cut() -> (Body<f64>, Body<f64>) {
    let body = cylinder_body();
    let phi = 0.3f64;
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, 0.5),
        normal: Vec3::new(phi.sin(), 0.0, phi.cos()),
    };
    let result = split(&body, &plane).unwrap();
    let (SplitPart::Body(a), SplitPart::Body(b)) = (result.above, result.below) else {
        panic!("both sides carry material");
    };
    (a, b)
}

// ---------------------------------------------------------------------
// Spec §6: shape (i) corpus bodies gain stored pcurves
// ---------------------------------------------------------------------

/// The C5 splitting lane mints caches where it mints curved faces: both
/// sides of the tilted cut come back with a stored, certified pcurve on
/// every half-edge of every cylinder face — and on nothing else.
#[test]
fn the_tilted_cut_mints_certified_caches_on_its_curved_faces() {
    let (above, below) = tilted_cut();
    for part in [&above, &below] {
        let mut cylinder_halves = 0usize;
        for (_, face) in part.faces() {
            let surface = part.get_surface(face.surface).unwrap();
            let is_cylinder = matches!(surface, Surface::Cylinder { .. });
            for lp in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
                let loop_data = part.get_loop(lp).unwrap();
                let topo::LoopBoundary::Cycle { first } = loop_data.boundary else {
                    continue;
                };
                for he in part.loop_cycle(first).unwrap() {
                    if is_cylinder {
                        cylinder_halves += 1;
                        let cache = part
                            .pcurve(he)
                            .expect("a cylinder face's half-edge carries a cache");
                        // Certified in METRES, both limbs.
                        let c = cache.certificate();
                        assert!(c.max_residual < 1e-12, "{:e}", c.max_residual);
                        assert!(c.envelope < 1e-12, "{:e}", c.envelope);
                        assert_eq!(c.samples, geom_brep::CERT_SAMPLES);
                    } else {
                        assert!(
                            part.pcurve(he).is_none(),
                            "a planar face's half-edge stores nothing (C4)"
                        );
                    }
                }
            }
        }
        assert!(cylinder_halves > 0, "the cut body has curved faces");
        // The stored set is exactly the cylinder-face half-edges.
        assert_eq!(part.pcurves().count(), cylinder_halves);
    }
}

/// The section edges — the elliptical ones — carry the sinusoid-graph
/// chart image on the cylinder side and nothing on the plane side.
#[test]
fn section_edges_carry_a_cylinder_chart_cache_and_no_plane_chart_one() {
    let (above, _) = tilted_cut();
    let mut sections = 0usize;
    for (_, edge) in above.edges() {
        let Some(curve) = above.get_curve_geom(edge.curve).and_then(|g| g.certified()) else {
            continue;
        };
        if !matches!(curve.carrier(), geom::Curve3::Ellipse { .. }) {
            continue;
        }
        sections += 1;
        let stored: Vec<_> = [edge.he_plus, edge.he_minus]
            .into_iter()
            .filter(|he| above.pcurve(*he).is_some())
            .collect();
        assert_eq!(
            stored.len(),
            1,
            "exactly one side of a section edge is a cylinder face"
        );
        let Pcurve::Harmonic { pa, pb, pl, .. } = *above.pcurve(stored[0]).unwrap().pcurve() else {
            panic!("the minting lane stores closed-form images")
        };
        // The sinusoid graph: azimuth affine with unit winding, height
        // harmonic. This is the tilted-section chart image, exactly.
        assert!((pl.x.abs() - 1.0).abs() < 1e-12);
        assert!(pa.x.abs() < 1e-12 && pb.x.abs() < 1e-12 && pl.y.abs() < 1e-12);
        assert!(
            pa.y.abs() + pb.y.abs() > 1e-3,
            "the graph really oscillates"
        );
    }
    assert!(sections > 0);
}

// ---------------------------------------------------------------------
// Spec §6: the seam-edge under-keying counterexample
// ---------------------------------------------------------------------

/// **One seam edge, two half-edges, the same surface, two DIFFERENT
/// certified pcurves.** The revolved tube's outer wall is a single
/// cylinder face closed by its seam meridian, so both half-edges of
/// that edge sit in one loop of one face — a per-edge key cannot hold
/// two pcurves, and neither can a per-(edge, face) key. The two
/// branches differ by exactly one period.
#[test]
fn a_seam_edge_carries_two_different_pcurves_on_one_surface() {
    let mut body = revolved_tube();
    topo::mint_pcurves(&mut body, Tol::witness()).unwrap();
    let mut found = 0usize;
    for (_, edge) in body.edges() {
        let (Some(a), Some(b)) = (body.pcurve(edge.he_plus), body.pcurve(edge.he_minus)) else {
            continue;
        };
        // Same face?
        let fa = body
            .get_loop(body.get_half_edge(edge.he_plus).unwrap().parent_loop)
            .unwrap()
            .face;
        let fb = body
            .get_loop(body.get_half_edge(edge.he_minus).unwrap().parent_loop)
            .unwrap()
            .face;
        if fa != fb {
            continue;
        }
        found += 1;
        let Pcurve::Harmonic { p0: pa, .. } = *a.pcurve() else {
            panic!("the minting lane stores closed-form images")
        };
        let Pcurve::Harmonic { p0: pb, .. } = *b.pcurve() else {
            panic!("the minting lane stores closed-form images")
        };
        let gap = (pa.x - pb.x).abs();
        assert!(
            (gap - TAU).abs() < 1e-9,
            "the two branches differ by one period, measured {gap}"
        );
        // Both are genuinely certified, against the same surface.
        assert!(a.certificate().envelope < 1e-12);
        assert!(b.certificate().envelope < 1e-12);
    }
    // Two: the tube has an inner and an outer cylinder wall, each a
    // single face closed by its own seam meridian.
    assert_eq!(found, 2, "both tube walls are seam-closed cylinder faces");
}

// ---------------------------------------------------------------------
// Spec §6: the planar-zero-caches pin
// ---------------------------------------------------------------------

/// A planar body carries ZERO stored pcurves — before and after a
/// split, and before and after an explicit minting pass. M2's
/// derive-on-demand status is untouched (C4 verbatim), and the pcurve
/// of such an edge is still available, derived exactly on demand.
#[test]
fn planar_bodies_carry_zero_stored_pcurves() {
    let square = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(1.0, 0.0), 0.0),
        ProfileVertex::new(p2(1.0, 1.0), 0.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![square])
        .validate(Tol::witness())
        .unwrap();
    let mut prism = extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body;
    assert_eq!(prism.pcurves().count(), 0);
    topo::mint_pcurves(&mut prism, Tol::witness()).unwrap();
    assert_eq!(prism.pcurves().count(), 0, "no speculative planar caches");

    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, 0.5),
        normal: Vec3::unit_z(),
    };
    let result = split(&prism, &plane).unwrap();
    for part in [result.above.body(), result.below.body()]
        .into_iter()
        .flatten()
    {
        assert_eq!(part.pcurves().count(), 0);
        assert_eq!(validate_geometric(part, Tol::witness()), Ok(()));
    }

    // Derive-on-demand still answers, exactly (the chart is affine).
    let band = Band::linear(Tol::witness()).unwrap();
    let (he, _) = prism.half_edges().next().unwrap();
    let derived = topo::pcurve_of(&prism, he, band).unwrap();
    let Pcurve::Harmonic { pa, pb, .. } = derived else {
        panic!("chart_pcurve derives closed-form images")
    };
    assert!(
        pa.x.abs() + pa.y.abs() + pb.x.abs() + pb.y.abs() < 1e-15,
        "a line in a plane chart is a chart line"
    );
}

// ---------------------------------------------------------------------
// Spec §6: the wrong unwrap, and the tier gate
// ---------------------------------------------------------------------

/// The wrong (nearest-previous, per-sample) unwrap is **unrepresentable**
/// here: a [`Pcurve::Harmonic`] azimuth channel is `α + β·t` with ONE
/// stored `α` and `β ∈ {−1, 0, +1}`, so a τ jump partway along an edge
/// has nowhere to live. What remains — which branch a half-edge takes —
/// is decided once per loop and certified; corrupting that decision on
/// a single half-edge is caught at rest.
#[test]
fn a_tampered_branch_is_refused_at_rest() {
    let (mut above, _) = tilted_cut();
    // Shift ONE half-edge's cache by a whole period: the pcurve is
    // still a perfectly valid chart image of the same locus (periodic
    // chart!) and still certifies in isolation — what breaks is the
    // face's one-branch continuity, which is exactly the property the
    // per-sample unwrap bug destroys.
    let victim = above
        .pcurves()
        .map(|(he, _)| he)
        .next()
        .expect("the cut body carries caches");
    let (t0, t1, shifted) = {
        let cache = above.pcurve(victim).unwrap();
        let (t0, t1) = cache.params();
        (
            t0,
            t1,
            cache
                .pcurve()
                .shift_branch(1.0, TAU)
                .expect("a harmonic image always shifts"),
        )
    };
    let surface = {
        let lp = above.get_half_edge(victim).unwrap().parent_loop;
        let face = above.get_loop(lp).unwrap().face;
        above
            .get_surface(above.get_face(face).unwrap().surface)
            .unwrap()
            .clone()
    };
    let carrier = {
        let edge = above
            .get_edge(above.get_half_edge(victim).unwrap().edge)
            .unwrap();
        above
            .get_curve_geom(edge.curve)
            .and_then(|g| g.certified())
            .unwrap()
            .carrier()
            .clone()
    };
    let band = Band::linear(Tol::witness()).unwrap();
    // A window wide enough that trim containment is not what fires —
    // the finding must be the BRANCH, not the box.
    let window = topo::ChartWindow {
        u_min: -100.0,
        u_max: 100.0,
        v_min: -100.0,
        v_max: 100.0,
    };
    let tampered =
        topo::PcurveCache::certify(shifted, t0, t1, &carrier, &surface, window, band).unwrap();
    above.attach_pcurve(victim, tampered);
    let findings = topo::pcurves::validate_pcurves(&above, band);
    assert!(
        findings.iter().any(|f| matches!(
            f,
            topo::PcurveMintError::LoopDiscontinuity { .. }
                | topo::PcurveMintError::LoopNotClosed { .. }
        )),
        "{findings:?}"
    );
    // And the ladder surfaces it typed.
    let errs = validate_geometric(&above, Tol::witness()).unwrap_err();
    assert!(
        errs.iter().any(|e| format!("{e}").contains("pcurve")),
        "{errs:?}"
    );
}

/// A corrupted stored cache fails the tier gate typed.
///
/// Note WHICH corruptions are even expressible: a [`topo::PcurveCache`]
/// cannot be built except through certification, so "a pcurve that does
/// not map onto its carrier" cannot be put into a body at all — the
/// value-level refusal rows live in `geom-brep`. What a body CAN carry
/// is a cache that is perfectly certified against a *different*
/// half-edge, and the at-rest pass — which re-derives the carrier, the
/// surface and the window rather than trusting the stored certificate —
/// is what catches that.
#[test]
fn a_cache_certified_against_another_edge_fails_the_tier_gate() {
    let (mut above, _) = tilted_cut();
    let halves: Vec<_> = above.pcurves().map(|(he, _)| he).collect();
    let a = halves[0];
    let params_a = above.pcurve(a).unwrap().params();
    let b = halves
        .iter()
        .copied()
        .find(|he| {
            let cb = above.pcurve(*he).unwrap().params();
            (params_a.0 - cb.0).abs() + (params_a.1 - cb.1).abs() > 1e-3
        })
        .expect("two half-edges with different carrier intervals");
    let donor = above.pcurve(a).unwrap().clone();
    above.attach_pcurve(b, donor);
    let findings = topo::pcurves::validate_pcurves(&above, Band::linear(Tol::witness()).unwrap());
    assert!(!findings.is_empty(), "the swap must be caught");
    assert!(
        findings.iter().any(|f| matches!(
            f,
            topo::PcurveMintError::Certify { .. }
                | topo::PcurveMintError::LoopDiscontinuity { .. }
                | topo::PcurveMintError::LoopNotClosed { .. }
        )),
        "{findings:?}"
    );
    assert!(validate_geometric(&above, Tol::witness()).is_err());
}

/// A body carrying caches survives a rigid transform: the caches are
/// re-derived construction-fresh against the mapped geometry (never
/// mapped), and the body re-validates.
#[test]
fn a_rigid_transform_re_derives_the_caches() {
    let (above, _) = tilted_cut();
    let before = above.pcurves().count();
    let map = geom_core::Affine3::translation(Vec3::new(1.0, -2.0, 0.5));
    let moved = topo::transform::transform_rigid(&above, &map, Tol::witness()).unwrap();
    assert_eq!(moved.pcurves().count(), before);
    assert!(
        topo::pcurves::validate_pcurves(&moved, Band::linear(Tol::witness()).unwrap()).is_empty()
    );
}

/// Determinism (D9): the same cut replayed produces byte-identical
/// pcurve caches.
#[test]
fn caches_replay_bit_identically() {
    let (a1, _) = tilted_cut();
    let (a2, _) = tilted_cut();
    let dump = |b: &Body<f64>| -> Vec<String> {
        b.pcurves()
            .map(|(k, c)| format!("{k:?}|{:?}|{:?}", c.pcurve(), c.certificate()))
            .collect()
    };
    assert_eq!(dump(&a1), dump(&a2));
}

// ---------------------------------------------------------------------
// Spec §3/§6: scalar-generic certification (C6) — the interval lane
// ---------------------------------------------------------------------

/// The same cut replayed at `T = Interval`: the caches mint and certify
/// generically, and every certificate's limbs are rigorous enclosures
/// rather than `f64` readings. The 3ε row is the CI matrix's
/// (`CAD_TOLERANCE_EPS`), not a separate test — nothing here depends on
/// the exact ε beyond "the residuals are rounding-scale".
#[cfg(feature = "interval")]
#[test]
fn caches_certify_on_the_interval_lane() {
    use geom_core::{Interval, Real};

    let ip2 = |x: f64, y: f64| Point2::new(Interval::from_f64(x), Interval::from_f64(y));
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(ip2(-0.5, 0.0), Interval::from_f64(1.0)),
        ProfileVertex::new(ip2(0.5, 0.0), Interval::from_f64(1.0)),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let body = extrude(&profile, Extrusion::Distance(Interval::from_f64(1.0)))
        .unwrap()
        .body;
    let phi = 0.3f64;
    let plane = SplitPlane {
        origin: Point3::new(
            Interval::from_f64(0.0),
            Interval::from_f64(0.0),
            Interval::from_f64(0.5),
        ),
        normal: Vec3::new(
            Interval::from_f64(phi.sin()),
            Interval::from_f64(0.0),
            Interval::from_f64(phi.cos()),
        ),
    };
    let result = split(&body, &plane).unwrap();
    let mut seen = 0usize;
    for part in [result.above.body(), result.below.body()]
        .into_iter()
        .flatten()
    {
        assert!(part.pcurves().count() > 0, "curved faces mint on any T");
        for (_, cache) in part.pcurves() {
            seen += 1;
            // Rigorous on this lane: the certified limbs are enclosures,
            // and they are still rounding-scale in METRES.
            let c = cache.certificate();
            assert!(geom_core::Bounds::hi(c.max_residual) < 1e-11);
            assert!(geom_core::Bounds::hi(c.envelope) < 1e-11);
        }
        assert!(
            topo::pcurves::validate_pcurves(part, Band::linear(Tol::witness()).unwrap()).is_empty()
        );
    }
    assert!(seen > 0);
}

// ---------------------------------------------------------------------
// Adopted from the adversarial review's seam probe (F6 attack): shapes
// beyond the committed fixture, where the branch walk has the most to
// get wrong.
// ---------------------------------------------------------------------

/// The seam-closed tube split by a tilted plane: its wall pieces' loops
/// contain seam-meridian fragments AND section arcs, so the walk must
/// keep one branch across a boundary that crosses the chart seam.
/// Whatever the outcome, it must be TYPED — mint success with clean
/// validation, or a typed refusal; never a wrong branch shipped.
#[test]
fn a_seam_closed_tube_split_is_typed_either_way() {
    let tube = revolved_tube();
    let phi = 0.25f64;
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.3, 0.0),
        normal: Vec3::new(phi.sin(), phi.cos(), 0.0),
    };
    match split(&tube, &plane) {
        Ok(result) => {
            let band = Band::linear(Tol::witness()).unwrap();
            for part in [result.above.body(), result.below.body()]
                .into_iter()
                .flatten()
            {
                let findings = topo::pcurves::validate_pcurves(part, band);
                assert!(findings.is_empty(), "{findings:?}");
            }
        }
        Err(e) => {
            // A typed refusal is an acceptable outcome for a frontier
            // configuration; a panic or a silently wrong body is not.
            // `split`'s signature is what makes the refusal typed, so
            // what is left to check at runtime is that it reaches a
            // human as prose: every arm of `SplitError` names the split
            // — three through their own stage (`split_reduce`, `split
            // join`, `split finish`), one through the door's name —
            // and none of them renders a payload's `Debug`.
            let msg = format!("{e}");
            assert!(
                msg.contains("split"),
                "the refusal must name its door: {msg}"
            );
            assert!(!msg.contains('{'), "Debug guts leaked: {msg}");
        }
    }
}

/// The tilted cut ROTATED 0.5 rad about the axis, so the section
/// ellipse's major direction — which anchors the pcurve azimuth — sits
/// at a non-seam azimuth and the arcs cross `u = 0` in their interior.
/// The walk must still be continuous and closed on both wall pieces.
#[test]
fn a_rotated_tilted_cut_mints_branch_consistent_caches() {
    let body = cylinder_body();
    let phi = 0.3f64;
    let rot = 0.5f64;
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, 0.5),
        normal: Vec3::new(phi.sin() * rot.cos(), phi.sin() * rot.sin(), phi.cos()),
    };
    let result = split(&body, &plane).unwrap();
    let band = Band::linear(Tol::witness()).unwrap();
    let mut caches = 0usize;
    for part in [result.above.body(), result.below.body()]
        .into_iter()
        .flatten()
    {
        caches += part.pcurves().count();
        for (_, c) in part.pcurves() {
            assert!(c.certificate().max_residual < 1e-12);
            assert!(c.certificate().envelope < 1e-12);
        }
        assert!(topo::pcurves::validate_pcurves(part, band).is_empty());
    }
    assert!(caches > 0, "the rotated cut minted caches");
}
