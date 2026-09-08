//! **The void door's `Transfers` posture, stage by stage**, on the
//! two operands whose one surface carries a same-surface LATITUDE
//! seam: a drum whose top cap has a collinear profile vertex (one
//! plane in four faces, a latitude ring between them) and a sphere
//! authored as two cocircular arcs (one sphere in four faces, a seam
//! at `v = π/4`). Each row pins the stage at which the pipeline's
//! output first goes wrong. The drum goes wrong INSIDE `Body::revert`
//! — a plane's normal is negated, its `Chart` images are not mirrored
//! with it — which is TOPO's
//! `work/topo/revert-does-not-mirror-plane-chart-images.md` and stays
//! refusing here. The sphere goes wrong at the stored pcurve rows the
//! reversal leaves stale in content and the graft copies verbatim onto
//! the twins; `shell`'s closing mint re-derives them, and the last row
//! reads the verb's rows against a hand re-mint of the same graft.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom::Surface;
use profile::{ProfileLoop, ProfileVertex, RawLoop};
use sweep::Revolution;
use topo::{Body, EdgeKey, VoidContainment, VoidEvidence};

use super::shell7_common::*;

fn collinear_cap_drum() -> Body<f64> {
    let (r, h) = (1.0, 2.0);
    polyline(
        &[(0.0, 0.0), (r, 0.0), (r, h), (r / 2.0, h), (0.0, h)],
        Revolution::Full,
    )
}

fn two_arc_sphere() -> Body<f64> {
    let r = 1.0;
    let v = PI / 4.0;
    let (s, c) = v.sin_cos();
    revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -r), ((FRAC_PI_2 + v) / 4.0).tan()),
            ProfileVertex::new(p2(r * c, r * s), ((FRAC_PI_2 - v) / 4.0).tan()),
            ProfileVertex::new(p2(0.0, r), 0.0),
        ]),
        Revolution::Full,
    )
}

/// The door's cavity, tier-3 valid.
fn door_cavity(body: &Body<f64>, t: f64) -> Body<f64> {
    let mut cavity = body.clone();
    let band = geom_core::Band::linear(tol()).expect("band");
    topo::offset_charts_together(&mut cavity, &hollow_moves(body, t), band, tol())
        .expect("the door takes it");
    assert_eq!(
        topo::validate_geometric(&cavity, tol()),
        Ok(()),
        "cavity tier 3"
    );
    cavity
}

/// Re-certifies every edge of `body` exactly as the graft does
/// (`combine.rs`'s recertify arm: the description with its image
/// verbatim, carrier and params verbatim, endpoints from `he_plus`,
/// surfaces from the body itself). Returns the refusals.
fn recertify_like_the_graft(body: &Body<f64>) -> Vec<(EdgeKey, geom_brep::CertifyError)> {
    let band = geom_core::Band::linear(tol()).expect("band");
    let mut failures = Vec::new();
    for (ek, e) in body.edges() {
        let curve = body
            .get_curve_geom(e.curve)
            .and_then(|g| g.certified())
            .unwrap();
        let description = match *curve.description() {
            geom_brep::EdgeDescription::Intersection { s1, s2, witness } => {
                geom_brep::EdgeDescriptionSpec::Intersection { s1, s2, witness }
            }
            geom_brep::EdgeDescription::TangentIntersection { s1, s2, witness } => {
                geom_brep::EdgeDescriptionSpec::TangentIntersection { s1, s2, witness }
            }
            geom_brep::EdgeDescription::Chart(ref c) => geom_brep::EdgeDescriptionSpec::Chart {
                surface: c.surface,
                image: Some(c.pcurve.clone()),
                seam: c.seam,
                declared: match curve.authority() {
                    geom_brep::EdgeAuthority::Declared(mc) => Some(mc),
                    geom_brep::EdgeAuthority::Derived => None,
                },
            },
            geom_brep::EdgeDescription::Scaffold(_) => continue,
        };
        let start_v = body.get_half_edge(e.he_plus).unwrap().start;
        let end_v = body.half_edge_end(e.he_plus).unwrap();
        let spec = geom_brep::EdgeCurveSpec {
            description,
            carrier: curve.carrier().clone(),
            param_start: curve.params().0,
            param_end: curve.params().1,
        };
        let out = geom_brep::EdgeCurve::certify(
            spec,
            point(body, start_v),
            point(body, end_v),
            |sk| body.get_surface(sk).cloned(),
            band,
        );
        if let Err(err) = out {
            failures.push((ek, err));
        }
    }
    failures
}

fn evidence_for(cavity: &Body<f64>) -> VoidEvidence {
    VoidEvidence {
        shells: cavity
            .shells()
            .map(|(k, _)| {
                (
                    k,
                    VoidContainment::Carried {
                        sign: geom_core::Sign::Positive,
                    },
                )
            })
            .collect(),
    }
}

/// The edges described as `Chart` images on a PLANE.
fn plane_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    body.edges()
        .filter(|(_, e)| {
            let c = body
                .get_curve_geom(e.curve)
                .and_then(|g| g.certified())
                .unwrap();
            matches!(c.description(), geom_brep::EdgeDescription::Chart(c)
                if matches!(body.get_surface(c.surface), Some(Surface::Plane { .. })))
        })
        .map(|(k, _)| k)
        .collect()
}

/// Every stored pcurve row of `body`, in half-edge-slot order, as the
/// text a bit-for-bit comparison reads: key, parameter window, image.
fn rows(body: &Body<f64>) -> Vec<String> {
    body.pcurves()
        .map(|(he, cache)| format!("{he:?} {:?} {:?}", cache.params(), cache.pcurve()))
        .collect()
}

/// **Drum, stage by stage.** The door's cavity re-certifies edge for
/// edge; its `revert()` does not — the two half-circles of the
/// latitude ring, the only `Chart` images on the reverted PLANE with
/// a non-zero `v` channel, refuse — and the void door, which re-runs
/// the same meter on the graft, refuses `Recertify` before any pcurve
/// pass runs. TOPO's item; this verb's closing mint never reaches it.
#[test]
fn drum_reverted_cavity_fails_recertification_on_the_plane_chart_circle() {
    let t = 0.05;
    let body = collinear_cap_drum();
    let cavity = door_cavity(&body, t);
    assert!(
        recertify_like_the_graft(&cavity).is_empty(),
        "the cavity re-certifies edge for edge"
    );
    let reverted = cavity.revert().expect("revert");
    let failures = recertify_like_the_graft(&reverted);
    let on_plane = plane_edges(&reverted);
    let failing: Vec<EdgeKey> = failures.iter().map(|(k, _)| *k).collect();
    // The same insertion `shell` runs, by hand: refused at the graft's
    // re-certification.
    let mut out = body.clone();
    let solids: Vec<_> = body.solids().map(|(k, _)| k).collect();
    let err = topo::insert_voids(&mut out, &solids, cavity, &evidence_for(&reverted), tol())
        .expect_err("insert_voids refuses");
    assert!(
        matches!(err, topo::VoidInsertError::Recertify(_)),
        "got {err:?}"
    );
    assert!(
        !failing.is_empty() && failing.iter().all(|k| on_plane.contains(k)),
        "the failing edges are Chart images on the reverted PLANE"
    );
    for k in &failing {
        let e = reverted.get_edge(*k).unwrap();
        let c = reverted
            .get_curve_geom(e.curve)
            .and_then(|g| g.certified())
            .unwrap();
        assert!(
            matches!(c.carrier(), geom::Curve3::Circle { .. }),
            "{k:?} is a latitude circle"
        );
    }
    assert_eq!(
        failing.len(),
        2,
        "the two half-circles of the latitude ring"
    );
}

/// **Sphere, stage by stage.** Every edge re-certifies on the reverted
/// body (the sphere carries its reversal on `Face::sense`, so its
/// images stay right), yet the reversal alone already fails tier 3
/// with a pcurve `LoopDiscontinuity`: the stored rows are key for key
/// the cavity's, and the one-period azimuth wrap the forward walk
/// parked at a loop's closure sits mid-chain once the loop runs the
/// other way. The graft is taken and copies the rows verbatim, so the
/// grafted body reports the same finding.
#[test]
fn sphere_reverted_cavity_and_the_grafted_loop() {
    let t = 0.05;
    let body = two_arc_sphere();
    let cavity = door_cavity(&body, t);
    let reverted = cavity.revert().expect("revert");
    let v = topo::validate_geometric(&reverted, tol());
    assert!(
        matches!(&v, Err(errors) if errors.iter().any(|f| format!("{f:?}").contains("LoopDiscontinuity"))),
        "revert() alone breaks the stored pcurve loop, got {v:?}"
    );
    assert!(
        recertify_like_the_graft(&reverted).is_empty(),
        "every sphere edge re-certifies on the reverted body"
    );
    let mut out = body.clone();
    let solids: Vec<_> = body.solids().map(|(k, _)| k).collect();
    topo::insert_voids(&mut out, &solids, cavity, &evidence_for(&reverted), tol())
        .expect("the sphere's graft is taken");
    let v = topo::validate_geometric(&out, tol());
    assert!(
        matches!(&v, Err(errors) if errors.iter().any(|f| format!("{f:?}").contains("LoopDiscontinuity"))),
        "the grafted body carries the stale rows, got {v:?}"
    );
}

/// **Drum, the cause isolated.** On the reverted body the latitude
/// circle's carrier and endpoints are right; only the plane chart
/// IMAGE is stale (the reverted plane's `v_ref = normal × u_ref`
/// flipped under it). Re-deriving the image from the carrier against
/// the reverted plane certifies, and the re-derived image is the
/// stored one mirrored in `v`; carrying it verbatim does not.
#[test]
fn drum_reverted_plane_circle_certifies_once_its_image_is_rederived() {
    let body = collinear_cap_drum();
    let cavity = door_cavity(&body, 0.05);
    let reverted = cavity.revert().expect("revert");
    let band = geom_core::Band::linear(tol()).expect("band");
    let mut circles = 0;
    for (ek, e) in reverted.edges() {
        let curve = reverted
            .get_curve_geom(e.curve)
            .and_then(|g| g.certified())
            .unwrap();
        let geom_brep::EdgeDescription::Chart(c) = curve.description() else {
            continue;
        };
        if !matches!(reverted.get_surface(c.surface), Some(Surface::Plane { .. }))
            || !matches!(curve.carrier(), geom::Curve3::Circle { .. })
        {
            continue;
        }
        circles += 1;
        let start_v = reverted.get_half_edge(e.he_plus).unwrap().start;
        let end_v = reverted.half_edge_end(e.he_plus).unwrap();
        let spec_with = |image: Option<geom_brep::Pcurve<f64>>| geom_brep::EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::Chart {
                surface: c.surface,
                image,
                seam: c.seam,
                declared: None,
            },
            carrier: curve.carrier().clone(),
            param_start: curve.params().0,
            param_end: curve.params().1,
        };
        let certify = |spec| {
            geom_brep::EdgeCurve::certify(
                spec,
                point(&reverted, start_v),
                point(&reverted, end_v),
                |sk| reverted.get_surface(sk).cloned(),
                band,
            )
        };
        let verbatim = certify(spec_with(Some(c.pcurve.clone())));
        let rederived = certify(spec_with(None));
        assert!(verbatim.is_err(), "{ek:?}: the verbatim image refuses");
        let re = rederived.expect("the re-derived image certifies on the reverted plane");
        let geom_brep::EdgeDescription::Chart(rc) = re.description() else {
            panic!("a chart image")
        };
        for i in 0..9u32 {
            let t = curve.params().0 + (curve.params().1 - curve.params().0) * f64::from(i) / 8.0;
            let (a, b) = (c.pcurve.eval(t), rc.pcurve.eval(t));
            assert!(
                (a.x - b.x).abs() <= 1e-12 && (a.y + b.y).abs() <= 1e-12,
                "{ek:?} sample {i}: stored {a:?} vs re-derived {b:?}"
            );
        }
    }
    assert_eq!(circles, 2, "the latitude ring's two half-circles");
}

/// **Sphere, the closing mint.** The grafted body's only tier-3
/// finding is the stale rows; one `mint_pcurves` pass re-derives them
/// and the body is tier-3 valid at the thin solid's closed form. That
/// pass is what `shell` runs on the body it assembles: the verb's
/// result carries exactly the rows the hand re-mint of the same graft
/// carries, key for key and bit for bit.
#[test]
fn sphere_grafted_body_is_tier_3_valid_after_the_closing_mint_which_shell_runs() {
    let (r, t) = (1.0, 0.05);
    let body = two_arc_sphere();
    let cavity = door_cavity(&body, t);
    let mut out = body.clone();
    let solids: Vec<_> = body.solids().map(|(k, _)| k).collect();
    let evidence = evidence_for(&cavity);
    topo::insert_voids(&mut out, &solids, cavity, &evidence, tol()).expect("the graft is taken");
    assert!(
        topo::validate_geometric(&out, tol()).is_err(),
        "the grafted body carries stale rows"
    );
    topo::mint_pcurves(&mut out, tol()).expect("the re-mint takes the grafted body");
    assert_eq!(
        topo::validate_geometric(&out, tol()),
        Ok(()),
        "the re-mint restores tier 3"
    );
    let props = topo::mass_properties(&out, tol()).expect("props");
    let want = 4.0 / 3.0 * PI * (r * r * r - (r - t) * (r - t) * (r - t));
    assert!(
        (props.volume - want).abs() <= 1e-9 + props.volume_pad,
        "shell volume {} (pad {}), want {want}",
        props.volume,
        props.volume_pad
    );
    let shelled = topo::shell(&body, t, tol()).expect("shell runs the closing mint");
    assert!(!rows(&out).is_empty(), "the sphere's faces carry rows");
    assert_eq!(
        rows(&shelled.body),
        rows(&out),
        "the verb's rows are the re-mint's"
    );
}
