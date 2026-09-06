//! **Review probes, FILLET-H6 lane r1** — the cap-rim arm's argument
//! read at the EDGE of the admitted set, not at its comfortable
//! interior.
//!
//! The arm's sentence says the wedge margin `sin θ · arm` is "definite
//! wherever the arm is". These rows build the worst admitted extrusion
//! (in-plane ε against height K·ε, so the tilted wall's normal is
//! `(K, 0, −1)/√(K²+1)` and `sin θ = K/√(K²+1)`) over a rectangle
//! whose SHORT rim is the lever arm, and walk that arm across the
//! band. The prediction is derived from the run's K alone, so each row
//! asserts a K-dependent outcome and can fail in every regime:
//!
//! - `margin ≤ ε`      ⇒ the `Smooth` arm IS reached (K ≲ 1.27 only);
//! - `ε < margin < Kε` ⇒ `SliverRim` under `dihedral_wedge` with a
//!   DEFINITE arm — the outcome the sentence says cannot happen;
//! - `margin ≥ Kε`     ⇒ `Transverse`, every rim `Intersection`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::EdgeDescription;
use geom_core::{Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane, ValidatedProfile};
use sweep::{ExtrudeError, Extruded, Extrusion, extrude};
use topo::{Body, EdgeKey, FaceKey, LoopBoundary, validate_geometric};

fn rect(sx: f64, sy: f64) -> ValidatedProfile<f64> {
    let p2 = Point2::<f64>::new;
    Profile::new(
        SketchPlane::xy(),
        vec![ProfileLoop::polygon([
            p2(0.0, 0.0),
            p2(sx, 0.0),
            p2(sx, sy),
            p2(0.0, sy),
        ])],
    )
    .validate(Tol::witness())
    .unwrap()
}

/// Every edge of a face, over its outer loop and every ring.
fn face_edges(body: &Body<f64>, face: FaceKey) -> Vec<EdgeKey> {
    let fd = body.get_face(face).unwrap();
    let mut edges = Vec::new();
    for lk in core::iter::once(fd.outer).chain(fd.rings.iter().copied()) {
        let LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
            continue;
        };
        for he in body.loop_cycle(first).unwrap() {
            edges.push(body.get_half_edge(he).unwrap().edge);
        }
    }
    edges
}

fn description(body: &Body<f64>, edge: EdgeKey) -> EdgeDescription<f64> {
    body.get_curve_geom(body.get_edge(edge).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap()
        .description()
        .clone()
}

fn cap_rim_descriptions(built: &Extruded<f64>) -> Vec<EdgeDescription<f64>> {
    cap_rims(built).into_iter().map(|(_, d)| d).collect()
}

/// Every cap rim's description beside the surface key of the cap it
/// borders.
fn cap_rims(built: &Extruded<f64>) -> Vec<(topo::SurfaceKey, EdgeDescription<f64>)> {
    [built.bottom, built.top]
        .into_iter()
        .flat_map(|cap| {
            let surface = built.body.get_face(cap).unwrap().surface;
            face_edges(&built.body, cap)
                .into_iter()
                .map(move |e| (surface, description(&built.body, e)))
        })
        .collect()
}

#[derive(Debug, PartialEq)]
enum Predicted {
    Smooth,
    InBand,
    Transverse,
}

/// The rim verdict the linear band must give on the short rim of
/// `rect(2, s)` with `s = f·K·ε` under the vector `(ε, 0, K·ε)`,
/// from K alone: the arm is `s` (both walls are planes, so the fold is
/// the extent), the wedge margin `s · K/√(K²+1)`.
fn predict(k: f64, f: f64) -> Predicted {
    let sin_theta = k / (k * k + 1.0).sqrt();
    let margin_over_eps = f * k * sin_theta;
    if margin_over_eps <= 1.0 {
        Predicted::Smooth
    } else if margin_over_eps < k {
        Predicted::InBand
    } else {
        Predicted::Transverse
    }
}

/// The short rim's lever arm is `f·K·ε`: definite by exactly the same
/// band the arm gate reads, so `dihedral_arm` passes and the wedge
/// margin alone decides. At K = 10 the row at `f = 1.002` lands in the
/// band (9.97ε), refuting "definite wherever the arm is"; `f = 1.1`
/// clears it (10.95ε). Run with `CAD_AMBIGUITY_K=1.1` the same rows
/// predict — and must find — the `Smooth` arm reached.
#[test]
fn worst_admitted_obliquity_on_a_tight_rim_reads_the_band() {
    let tol = Tol::witness();
    let (eps, k) = (tol.eps(), tol.k());
    let w = Vec3::new(eps, 0.0, k * eps);
    for f in [1.002, 1.1, 1.5] {
        let s = f * k * eps;
        let predicted = predict(k, f);
        let built = extrude(&rect(2.0, s), Extrusion::Vector(w), tol);
        match predicted {
            Predicted::InBand => {
                let err = built.err().unwrap_or_else(|| {
                    panic!("f={f} K={k}: predicted an in-band wedge, but the body built")
                });
                assert!(
                    matches!(
                        &err,
                        ExtrudeError::SliverRim { source, .. }
                            if source.predicate == Some("dihedral_wedge")
                    ),
                    "f={f} K={k}: predicted SliverRim under dihedral_wedge, got {err:?}",
                );
            }
            Predicted::Transverse => {
                let built = built.unwrap_or_else(|e| {
                    panic!("f={f} K={k}: predicted transverse rims, refused: {e:?}")
                });
                for d in cap_rim_descriptions(&built) {
                    assert!(
                        matches!(d, EdgeDescription::Intersection { .. }),
                        "f={f} K={k}: predicted transverse, a cap rim carries {d:?}",
                    );
                }
            }
            Predicted::Smooth => {
                let built = built.unwrap_or_else(|e| {
                    panic!("f={f} K={k}: predicted the Smooth arm, refused: {e:?}")
                });
                let rims = cap_rims(&built);
                // Two short rims per cap, two caps. Plane against
                // plane has an exactly-zero jet, so the arm's
                // under-determined branch is the one that runs: the
                // rim is restated as an image in ITS cap's chart — not
                // left as the scaffold (the no-op the unit retired),
                // not the other cap's, not intrinsic.
                let smooth_reached = rims
                    .iter()
                    .filter(|(_, d)| !matches!(d, EdgeDescription::Intersection { .. }))
                    .count();
                assert_eq!(
                    smooth_reached, 4,
                    "f={f} K={k}: predicted the Smooth arm on the four short rims, got {rims:?}",
                );
                for (cap, d) in &rims {
                    if matches!(d, EdgeDescription::Intersection { .. }) {
                        continue;
                    }
                    assert!(
                        matches!(d, EdgeDescription::Chart(c) if !c.seam && c.surface == *cap),
                        "f={f} K={k}: a smooth rim must be an image in its own cap's chart, got {d:?}",
                    );
                }
                let descs: Vec<_> = rims.iter().map(|(_, d)| d).collect();
                // What the arm minted, and whether the at-rest gate
                // accepts it — measured, not assumed.
                let at_rest = validate_geometric(&built.body, tol);
                eprintln!("f={f} K={k}: Smooth arm reached; rims {descs:?}; tier 3: {at_rest:?}");
            }
        }
    }
}

/// Bodies whose every cap rim is transverse validate at rest. Under a
/// planted `Transverse → Smooth` verdict at the rim this row is the
/// tier-3 witness: it says whether the mis-description is caught by
/// the at-rest gate (the cylinder wall's `TangentIntersection` is
/// refused at certification; the plane wall's chart image is the case
/// worth measuring).
#[test]
fn transverse_cap_rims_validate_at_rest() {
    let tol = Tol::witness();
    let p2 = Point2::<f64>::new;
    let circle = Profile::new(
        SketchPlane::xy(),
        vec![ProfileLoop::new(vec![
            ProfileVertex::new(p2(-1.5, 0.0), 1.0),
            ProfileVertex::new(p2(1.5, 0.0), 1.0),
        ])],
    )
    .validate(tol)
    .unwrap();
    for (name, profile) in [("square", rect(2.0, 2.0)), ("cylinder", circle)] {
        let built = extrude(&profile, Extrusion::Distance(1.0), tol)
            .unwrap_or_else(|e| panic!("{name}: {e:?}"));
        for d in cap_rim_descriptions(&built) {
            assert!(
                matches!(d, EdgeDescription::Intersection { .. }),
                "{name}: cap rim carries {d:?}",
            );
        }
        validate_geometric(&built.body, tol).unwrap_or_else(|e| panic!("{name}: tier 3 {e:?}"));
    }
}

/// **Deviation 3, measured.** `revolve::upgrade::jet_determinate` folds
/// an in-band `tangent_second_order` verdict into `false` and keeps the
/// conventional description, where extrude's strut arm escalates the
/// same verdict as `SliverJoin`. This row builds the body that tells
/// the two policies apart: a bore cylinder of radius `r` meeting a
/// unit-minor-radius torus tangentially at a latitude circle of radius
/// `r`, with `r² / 2 = (1 + K)·ε / 2` — the second-order sagitta
/// squarely inside the band at every ε. The door hands the body back
/// `Ok`, the join carries a chart image, and the at-rest gate refuses
/// that body under the very predicate the fold swallowed.
#[test]
fn revolve_folds_an_in_band_second_order_into_a_body_tier_3_refuses() {
    use geom::Surface;
    use sweep::{Revolution, RevolveAxis, revolve};
    use topo::ValidationError;

    let tol = Tol::witness();
    let (eps, k) = (tol.eps(), tol.k());
    let r = ((1.0 + k) * eps).sqrt();
    let p2 = Point2::<f64>::new;
    let b = (core::f64::consts::FRAC_PI_8).tan();
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![
            ProfileLoop::new(vec![
                ProfileVertex::new(p2(r, 0.0), 0.0),
                ProfileVertex::new(p2(1.5, 0.0), 0.0),
                ProfileVertex::new(p2(1.5, 2.0), 0.0),
                ProfileVertex::new(p2(1.0 + r, 2.0), b),
                ProfileVertex::new(p2(r, 1.0), 0.0),
            ])
            .with_tangent_joints(vec![3, 4]),
        ],
    )
    .validate(tol)
    .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    let built = revolve(&profile, axis, Revolution::Full, tol)
        .unwrap_or_else(|e| panic!("the fold means the door does not refuse; got {e:?}"));
    let body = &built.body;

    // The torus–bore join: the one edge between a torus and a
    // cylinder. Its description is what the fold left it.
    let surface_of = |he| {
        let face = body
            .get_loop(body.get_half_edge(he).unwrap().parent_loop)
            .unwrap()
            .face;
        body.get_surface(body.get_face(face).unwrap().surface)
            .unwrap()
            .clone()
    };
    let mut joins = 0usize;
    for (ek, e) in body.edges() {
        let (a, c) = (surface_of(e.he_plus), surface_of(e.he_minus));
        let torus_cyl = matches!(
            (&a, &c),
            (Surface::Torus { .. }, Surface::Cylinder { .. })
                | (Surface::Cylinder { .. }, Surface::Torus { .. })
        );
        if !torus_cyl {
            continue;
        }
        joins += 1;
        let d = description(body, ek);
        assert!(
            matches!(&d, EdgeDescription::Chart(ch) if !ch.seam),
            "the folded join must carry the conventional chart image, got {d:?}",
        );
    }
    assert_eq!(joins, 1, "one torus–bore latitude join expected");

    let refused = validate_geometric(body, tol).expect_err("tier 3 must refuse the folded join");
    assert!(
        refused.iter().any(|e| matches!(
            e,
            ValidationError::SliverDihedral { cause, .. }
                if cause.predicate == Some("tangent_second_order")
        )),
        "expected a SliverDihedral under tangent_second_order, got {refused:?}",
    );
}
