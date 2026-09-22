//! **The void door's `Transfers` posture, stage by stage**, on the
//! two operands whose one surface carries a same-surface LATITUDE
//! seam: a drum whose top cap has a collinear profile vertex (one
//! plane in four faces, a latitude ring between them) and a sphere
//! authored as two cocircular arcs (one sphere in four faces, a seam
//! at `v = π/4`). Each row pins the stage at which the pipeline's
//! output first goes wrong. The drum used to go wrong INSIDE
//! `Body::revert` — a plane's normal was negated and its `Chart`
//! images were not mirrored with it; the reversal now re-states every
//! plane image under the frame reflection, and the drum rows pin that
//! the reverted cavity re-certifies edge for edge and the void door
//! takes it (the reversal's own rows: `revert_plane_charts`). The
//! sphere used to go wrong inside `Body::revert` too — the one-period
//! wrap the forward loop walk parks at a loop's closure sat mid-chain
//! once the loop ran the other way; the reversal now moves each curved
//! loop's anchor with the direction (its own rows:
//! `revert_periodic_wrap`), so the stored rows travel verbatim through
//! the graft and stay continuous on the twins, and the last row reads
//! the verb's rows against a hand re-mint of the same graft.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::Surface;

use super::common::latitude_seam::{
    collinear_cap_drum, door_cavity, graft_recertify_failures, plane_images, two_arc_sphere,
    void_evidence,
};
use super::shell7_common::*;
use super::shell9_rows::rows;

/// **Drum, stage by stage.** The door's cavity re-certifies edge for
/// edge, and so does its `revert()`: the two half-circles of the
/// latitude ring — the only `Chart` images on the PLANE with a
/// non-zero `v` channel, and the two edges that refused `ChartResidual`
/// at sample 1 while the reversal left them unmirrored — carry the
/// mirrored image now, so the void door, which re-runs the same meter
/// on the graft, takes the cavity. The plane edges are eight: the six
/// radial lines and the ring's two halves.
#[test]
fn drum_reverted_cavity_re_certifies_and_the_void_door_takes_it() {
    let t = 0.05;
    let body = collinear_cap_drum();
    let cavity = door_cavity(&body, t);
    assert!(
        graft_recertify_failures(&cavity).is_empty(),
        "the cavity re-certifies edge for edge"
    );
    let reverted = cavity.revert().expect("revert");
    let failures = graft_recertify_failures(&reverted);
    assert!(
        failures.is_empty(),
        "the reverted cavity re-certifies edge for edge: {failures:?}"
    );
    let on_plane = plane_images(&reverted);
    let circles = on_plane
        .iter()
        .filter(|(k, _)| {
            let e = reverted.get_edge(*k).unwrap();
            let c = reverted
                .get_curve_geom(e.curve)
                .and_then(|g| g.certified())
                .unwrap();
            matches!(c.carrier(), geom::Curve3::Circle { .. })
        })
        .count();
    assert_eq!(circles, 2, "the two half-circles of the latitude ring");
    assert_eq!(
        on_plane.len(),
        8,
        "six radial lines and the ring's two halves"
    );
    // The same insertion `shell` runs, by hand: taken.
    let mut out = body.clone();
    let solids: Vec<_> = body.solids().map(|(k, _)| k).collect();
    topo::insert_voids(&mut out, &solids, cavity, &void_evidence(&reverted), tol())
        .expect("insert_voids takes the reverted cavity");
    assert_eq!(out.shells().count(), 2, "outer + cavity");
}

/// **Sphere, stage by stage.** Every edge re-certifies on the reverted
/// body (the sphere carries its reversal on `Face::sense`, so its
/// images stay right), and the reversal alone reports exactly the
/// complement: the stored rows are key for key the cavity's, and the
/// one-period azimuth wrap the forward walk parked at each lune's
/// closure sits at the reversed closure because the loop's anchor
/// moved with the direction. The graft is taken, copies the rows
/// verbatim onto the twins and carries each loop's anchor through its
/// key map, so the grafted body is tier-3 valid on the carried rows
/// alone.
#[test]
fn sphere_reverted_cavity_re_certifies_and_the_grafted_loop_is_continuous() {
    let t = 0.05;
    let body = two_arc_sphere();
    let cavity = door_cavity(&body, t);
    let reverted = cavity.revert().expect("revert");
    assert_eq!(
        topo::validate_geometric(&reverted, tol()),
        Err(vec![topo::ValidationError::NegativeVolume {
            solid: reverted.solids().next().expect("one solid").0
        }]),
        "revert() alone leaves every stored loop continuous"
    );
    assert!(
        graft_recertify_failures(&reverted).is_empty(),
        "every sphere edge re-certifies on the reverted body"
    );
    let mut out = body.clone();
    let solids: Vec<_> = body.solids().map(|(k, _)| k).collect();
    topo::insert_voids(&mut out, &solids, cavity, &void_evidence(&reverted), tol())
        .expect("the sphere's graft is taken");
    assert_eq!(
        topo::validate_geometric(&out, tol()),
        Ok(()),
        "the grafted body is tier-3 valid on the carried rows"
    );
}

/// **Drum, the cause isolated — and gone.** On the reverted body the
/// latitude circle's carrier and endpoints are right, and so is the
/// plane chart IMAGE: the reverted plane's `v_ref = normal × u_ref`
/// flipped, and the reversal mirrored the image with it. The stored
/// image certifies verbatim against the reverted plane, and it is the
/// image a fresh derivation from the carrier against that plane
/// produces, sample for sample. (Before the reversal mirrored, the
/// verbatim image refused and the re-derived one was its `v`
/// negation — the measurement that located the defect.)
#[test]
fn drum_reverted_plane_circle_image_is_the_one_a_fresh_derivation_gives() {
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
        assert!(
            verbatim.is_ok(),
            "{ek:?}: the stored image certifies verbatim on the reverted plane"
        );
        let re = rederived.expect("the re-derived image certifies on the reverted plane");
        let geom_brep::EdgeDescription::Chart(rc) = re.description() else {
            panic!("a chart image")
        };
        for i in 0..9u32 {
            let t = curve.params().0 + (curve.params().1 - curve.params().0) * f64::from(i) / 8.0;
            let (a, b) = (c.pcurve.eval(t), rc.pcurve.eval(t));
            assert!(
                (a.x - b.x).abs() <= 1e-12 && (a.y - b.y).abs() <= 1e-12,
                "{ek:?} sample {i}: stored {a:?} vs re-derived {b:?}"
            );
        }
    }
    assert_eq!(circles, 2, "the latitude ring's two half-circles");
}

/// **Sphere, the closing mint.** The grafted body is tier-3 valid on
/// the rows the graft carried, and the closing `mint_pcurves` pass —
/// which `shell` runs on the body it assembles, the `Transfers` row's
/// contract for a producer — clears and re-derives them, leaving the
/// body tier-3 valid at the thin solid's closed form: the verb's
/// result carries exactly the rows the hand re-mint of the same graft
/// carries, key for key and bit for bit.
#[test]
fn sphere_grafted_body_is_tier_3_valid_before_and_after_the_closing_mint_which_shell_runs() {
    let (r, t) = (1.0, 0.05);
    let body = two_arc_sphere();
    let cavity = door_cavity(&body, t);
    let mut out = body.clone();
    let solids: Vec<_> = body.solids().map(|(k, _)| k).collect();
    let evidence = void_evidence(&cavity);
    topo::insert_voids(&mut out, &solids, cavity, &evidence, tol()).expect("the graft is taken");
    assert_eq!(
        topo::validate_geometric(&out, tol()),
        Ok(()),
        "the grafted body is tier-3 valid on the carried rows"
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
