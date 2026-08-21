//! PR 3 adversarial review — consumer-side probes: DegenerateSection
//! honesty (target 3), degenerate/miss cases (target 10), the tier-3
//! description gap exploited as a consumer (target 9), carve leak
//! audit (target 7), and the single-solid gate (judgment call b).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::prism;
use geom_core::{Point3, Vec3};
use topo::{
    Body, SplitError, SplitFinishError, SplitJoinError, SplitPart, SplitPlane, mass_properties,
    plane_section, split, validate_closed, validate_geometric,
};
use geom_core::Tol;

fn plane_y(c: f64) -> SplitPlane<f64> {
    SplitPlane {
        origin: Point3::new(0.0, c, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
    }
}

fn body_of<T: geom_core::Real>(part: &SplitPart<T>) -> &Body<T> {
    part.body().expect("side has material")
}

/// Referential integrity both ways: every entity's geometry key
/// resolves (no geometry removed while referenced — the carve's
/// shared-key hazard), and every geometry arena entry is referenced
/// by a surviving entity (no orphan leaks).
fn audit_geometry(body: &Body<f64>) {
    for (_, v) in body.vertices() {
        assert!(body.get_point(v.point).is_some(), "dangling point key");
    }
    for (_, e) in body.edges() {
        assert!(body.get_curve_geom(e.curve).is_some(), "dangling curve key");
    }
    for (_, f) in body.faces() {
        assert!(
            body.get_surface(f.surface).is_some(),
            "dangling surface key"
        );
    }
    let mut live_p: Vec<_> = body.vertices().map(|(_, v)| v.point).collect();
    live_p.sort();
    for (k, _) in body.points() {
        assert!(live_p.binary_search(&k).is_ok(), "orphan point {k:?}");
    }
    let mut live_c: Vec<_> = body.edges().map(|(_, e)| e.curve).collect();
    live_c.sort();
    for (k, _) in body.curves() {
        assert!(live_c.binary_search(&k).is_ok(), "orphan curve {k:?}");
    }
    // Surfaces are kept alive by faces AND by edge descriptions (the
    // carve rule: an `Intersection`/`Seam` description on a surviving
    // edge must never dangle — with D6's native descriptions this lane
    // is now live on every split result).
    let mut live_s: Vec<_> = body.faces().map(|(_, f)| f.surface).collect();
    for (_, e) in body.edges() {
        if let Some(topo::CurveGeom::Certified(c)) = body.get_curve_geom(e.curve) {
            match *c.description() {
                geom_brep::EdgeGeometry::Intersection { s1, s2, .. }
                | geom_brep::EdgeGeometry::TangentIntersection { s1, s2, .. } => {
                    live_s.push(s1);
                    live_s.push(s2);
                }
                geom_brep::EdgeGeometry::Seam { surface }
                | geom_brep::EdgeGeometry::IsoCurve { surface, .. } => live_s.push(surface),
                geom_brep::EdgeGeometry::MappedCurve(_) => {}
            }
        }
    }
    live_s.sort();
    for (k, _) in body.surfaces() {
        assert!(live_s.binary_search(&k).is_ok(), "orphan surface {k:?}");
    }
    // Topological references resolve too (dangling shell/loop keys).
    for (_, f) in body.faces() {
        assert!(body.get_shell(f.shell).is_some());
        assert!(body.get_loop(f.outer).is_some());
        for &r in &f.rings {
            assert!(body.get_loop(r).is_some());
        }
    }
    for (_, he) in body.half_edges() {
        assert!(body.get_loop(he.parent_loop).is_some());
        assert!(body.get_edge(he.edge).is_some());
        assert!(body.get_vertex(he.start).is_some());
    }
}

/// Target 7: the carve must neither leak orphans nor drop shared-key
/// geometry still referenced by the kept side — audited on all four
/// result bodies of two different splits (generic + the notched
/// multi-shell landing).
#[test]
fn carve_leaves_no_orphans_and_no_dangling_keys() {
    let fx = prism::<f64>(
        &[(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (2.0, 3.0), (0.0, 2.0)],
        1.0,
    );
    let r = split(&fx.body, &plane_y(1.0)).unwrap();
    audit_geometry(body_of(&r.above));
    audit_geometry(body_of(&r.below));

    let notched = &[
        (0.0, 0.0),
        (8.0, 0.0),
        (8.0, 2.0),
        (7.0, 1.0),
        (6.0, 1.0),
        (5.0, 2.0),
        (4.0, 1.0),
        (3.0, 2.0),
        (0.0, 2.0),
    ];
    let fx = prism::<f64>(notched, 1.0);
    let r = split(&fx.body, &plane_y(1.0)).unwrap();
    audit_geometry(body_of(&r.above));
    audit_geometry(body_of(&r.below));
}

/// Target 3a: a genuinely tiny-but-real section (a needle apex only
/// 1e-4 above the plane) must NOT be swallowed by the zero-area net —
/// the split succeeds and the sliver's volume is right.
#[test]
fn tiny_real_sliver_not_wrongly_refused() {
    let h = 1.0e-4;
    let fx = prism::<f64>(&[(0.0, 0.0), (4.0, 0.0), (2.0, 1.0 + h)], 1.0);
    let r = split(&fx.body, &plane_y(1.0)).unwrap();
    let (above, below) = (body_of(&r.above), body_of(&r.below));
    assert_eq!(validate_closed(above), Ok(()));
    assert_eq!(validate_closed(below), Ok(()));
    // The sliver: a similar triangle scaled by h/(1+h) in both profile
    // axes, depth 1: V = (1/2)·4·(1+h)·(h/(1+h))² ≈ 2h²/(1+h).
    let va = mass_properties(above).unwrap().volume;
    let expect = 2.0 * h * h / (1.0 + h);
    assert!(
        (va - expect).abs() <= 1e-6 * expect,
        "sliver volume {va} vs {expect}"
    );
    // And the section query agrees: one tiny positive-area polygon.
    let s = plane_section(&fx.body, &plane_y(1.0)).unwrap();
    assert_eq!(s.polygons.len(), 1);
}

/// Target 3b: an in-band section (a wall whose thickness sits inside
/// the ambiguity band (ε, K·ε)) must escalate TYPED — never
/// misclassify as the zero-area DegenerateSection, never silently
/// succeed with an ill-conditioned sliver. (If the fixture itself
/// refuses to build at this ε, that is the same honest posture one
/// stage earlier — the test accepts either typed outcome.)
#[test]
fn in_band_section_escalates_typed_not_misclassified() {
    let eps = geom_core::Tol::witness().get().eps;
    let t = 5.0 * eps; // inside (ε, 10ε)
    let profile = [(2.0, 0.0), (2.0 + t, 0.0), (2.0 + t, 2.0), (2.0, 2.0)];
    let built = std::panic::catch_unwind(|| prism::<f64>(&profile, 1.0));
    let Ok(fx) = built else {
        eprintln!("in-band probe: fixture build refused at ε={eps}");
        return; // build-stage refusal: honest, earlier.
    };
    match split(&fx.body, &plane_y(1.0)) {
        Err(SplitError::Join(SplitJoinError::DegenerateSection { .. })) => {
            panic!("in-band sliver MISCLASSIFIED as zero-area tangency");
        }
        Err(e) => eprintln!("in-band probe: typed refusal {e}"),
        Ok(_) => panic!("in-band sliver silently split at ε={eps}"),
    }
}

/// Target 10: plane touching the body at exactly ONE vertex (a tilted
/// plane through a cube corner): typed Empty on the far side, the
/// whole body on the other, and `plane_section` reports zero polygons.
#[test]
fn vertex_only_contact_is_typed_empty() {
    let fx = prism::<f64>(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 1.0);
    let s3 = 3.0f64.sqrt();
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(-1.0 / s3, -1.0 / s3, -1.0 / s3),
    };
    let r = split(&fx.body, &plane).unwrap();
    assert!(matches!(r.above, SplitPart::Empty));
    let below = body_of(&r.below);
    assert_eq!(validate_closed(below), Ok(()));
    assert_eq!(below.vertices().count(), fx.body.vertices().count());
    let s = plane_section(&fx.body, &plane).unwrap();
    assert!(s.polygons.is_empty());
    assert!(s.u_ref.is_none());
}

/// Target 9, RESOLVED (M3 PR 6a, D6): split results carry honest
/// `Intersection` descriptions natively (minted at section-face
/// promotion from the two known parent surfaces), so tier 3
/// (`validate_geometric`) passes OUT OF THE BOX — the gap this test
/// originally witnessed ("the acceptance suite reaches tier 3 only
/// through a tests/common-only upgrade helper") is closed, and the
/// helper is retired. This test now pins the closure from the
/// consumer's seat.
#[test]
fn tier3_needs_upgrade_pass_consumers_lack() {
    let fx = prism::<f64>(
        &[(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (2.0, 3.0), (0.0, 2.0)],
        1.0,
    );
    let r = split(&fx.body, &plane_y(1.0)).unwrap();
    let above = body_of(&r.above);
    assert_eq!(validate_closed(above), Ok(()), "tier 2 at rest holds");
    assert!(mass_properties(above).is_ok(), "mass props work");
    assert_eq!(
        validate_geometric(above),
        Ok(()),
        "tier 3 accepts split output as shipped (D6: native descriptions)"
    );
}

/// Judgment call (b): the single-solid gate — `split` refuses a
/// two-solid body typed; `plane_section` (which never reaches the
/// finish stage) quietly accepts the same operand. Witnessed
/// inconsistency, for the writeup.
#[test]
fn single_solid_gate_split_vs_section() {
    let mut body = Body::<f64>::new();
    add_quad_prism(&mut body, 0.0);
    add_quad_prism(&mut body, 10.0);
    assert_eq!(body.solids().count(), 2);
    assert_eq!(validate_closed(&body), Ok(()));
    let err = split(&body, &plane_y(1.0)).unwrap_err();
    assert!(
        matches!(
            err,
            SplitError::Finish(SplitFinishError::NotSingleSolid { count: 2 })
        ),
        "got {err:?}"
    );
    // plane_section never reaches the finish gate: it quietly slices
    // BOTH solids (two polygons) — the gate asymmetry, witnessed.
    let s = plane_section(&body, &plane_y(1.0)).unwrap();
    assert_eq!(s.polygons.len(), 2);
}

/// `plane_section` ergonomics probe: is the (u, v) winding of the
/// returned polygons DETERMINED (a consumer computing signed areas or
/// offsets needs an orientation contract)? Executed answer, for the
/// writeup: whatever this asserts is what consumers can rely on
/// today; the acceptance suite only ever takes |area|.
#[test]
fn plane_section_winding_is_consistent() {
    let notched = &[
        (0.0, 0.0),
        (8.0, 0.0),
        (8.0, 2.0),
        (7.0, 1.0),
        (6.0, 1.0),
        (5.0, 2.0),
        (4.0, 1.0),
        (3.0, 2.0),
        (0.0, 2.0),
    ];
    let fx = prism::<f64>(notched, 1.0);
    let s = plane_section(&fx.body, &plane_y(1.0)).unwrap();
    assert_eq!(s.polygons.len(), 3);
    let mut signs = Vec::new();
    for poly in &s.polygons {
        let mut twice = 0.0;
        for i in 0..poly.uv.len() {
            let a = poly.uv[i];
            let b = poly.uv[(i + 1) % poly.uv.len()];
            twice += a.x * b.y - b.x * a.y;
        }
        signs.push(twice.signum());
    }
    assert!(
        signs.iter().all(|&s| s == signs[0]),
        "mixed winding across polygons of one section: {signs:?}"
    );
}

/// A unit 2×2×1-ish quad prism (x offset by `x0`, spanning y ∈ [0, 2])
/// added as a NEW solid of `body` — reassembly's builder,
/// body-parameterized (public ops only).
fn add_quad_prism(body: &mut Body<f64>, x0: f64) {
    use topo::{EdgeCurveSpec, FaceSurface, MefSite, MevSite};
    let profile = [(x0, 0.0), (x0 + 2.0, 0.0), (x0 + 2.0, 2.0), (x0, 2.0)];
    let c = |&(x, y): &(f64, f64), z: f64| Point3::new(x, y, z);
    let bot: Vec<Point3<f64>> = profile.iter().map(|p| c(p, 0.0)).collect();
    let top: Vec<Point3<f64>> = profile.iter().map(|p| c(p, 1.0)).collect();
    let n = 4;
    let line = EdgeCurveSpec::line_between;
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let plane = |corners: &[Point3<f64>]| geom_brep::newell_plane(corners, band).unwrap();
    let seed = body.mvfs(bot[0]).unwrap();
    let mut chain = vec![
        body.mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            bot[1],
            line(bot[0], bot[1]),
        )
        .unwrap(),
    ];
    for i in 2..n {
        let at = chain[i - 2].he_minus;
        chain.push(
            body.mev(
                MevSite::Fan { he1: at, he2: at },
                bot[i],
                line(bot[i - 1], bot[i]),
            )
            .unwrap(),
        );
    }
    let bottom: Vec<_> = core::iter::once(seed.vertex)
        .chain(chain.iter().map(|m| m.vertex))
        .collect();
    let he_last = body
        .find_half_edge(seed.face, bottom[n - 1], bottom[n - 2])
        .unwrap();
    let rev: Vec<Point3<f64>> = core::iter::once(bot[0])
        .chain(bot[1..].iter().rev().copied())
        .collect();
    let f_bottom = body
        .mef(
            MefSite::Chords {
                he1: he_last,
                he2: chain[0].he_plus,
            },
            line(bot[n - 1], bot[0]),
            FaceSurface::New(plane(&rev)),
        )
        .unwrap();
    let mut struts = Vec::new();
    for i in 0..n {
        let at = if i == 0 {
            chain[0].he_plus
        } else if i < n - 1 {
            chain[i].he_plus
        } else {
            f_bottom.he_plus
        };
        struts.push(
            body.mev(
                MevSite::Fan { he1: at, he2: at },
                top[i],
                line(bot[i], top[i]),
            )
            .unwrap(),
        );
    }
    let mut first_side_he_plus = None;
    for i in 0..n {
        let j = (i + 1) % n;
        let he2 = if i < n - 1 {
            struts[j].he_minus
        } else {
            first_side_he_plus.unwrap()
        };
        let f = body
            .mef(
                MefSite::Chords {
                    he1: struts[i].he_minus,
                    he2,
                },
                line(top[i], top[j]),
                FaceSurface::New(plane(&[bot[i], bot[j], top[j], top[i]])),
            )
            .unwrap();
        if i == 0 {
            first_side_he_plus = Some(f.he_plus);
        }
    }
    body.set_face_surface(seed.face, FaceSurface::New(plane(&top)))
        .unwrap();
}
