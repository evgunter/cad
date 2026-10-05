//! **Blinded-review probes for M5 PR 7b** — the SSI side: deviation 2's
//! independent reproduction (the inflected-wall fit deviation is REAL
//! geometry), the domain-mismatch typed-refusal shape (deviations 1
//! and 3), and the retirement's practical breadth on a multi-cell
//! (interior-knot) wall.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::{band, eps};
use geom::{NurbsSurface, Surface};
use geom_brep::ssi::{self, SsiDomain, SsiError, SsiOperand, TubeScale};
use geom_core::spline::KnotVector;
use geom_core::spline::compose::ComposeError;
use geom_core::{Point3, Vec3};

/// PR 7's inflected wall, verbatim from the acceptance suite.
fn nurbs_wall() -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let cols = [(0.0, 0.0), (0.35, 0.18), (0.70, -0.12), (1.05, 0.04)];
    let mut control = Vec::with_capacity(8);
    for (x, y) in cols {
        control.push(Point3::new(x, y, 0.0));
        control.push(Point3::new(x, y, 0.8));
    }
    NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap()
}

fn cutting_plane() -> Surface<f64> {
    let n = Vec3::new(0.0, 0.25, 1.0);
    let n = n / n.norm();
    let u = Vec3::new(1.0, 0.0, 0.0);
    let u = (u - n * u.dot(n)) / (u - n * u.dot(n)).norm();
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.4),
        normal: n,
        u_ref: u,
    }
}

/// **Deliberately not shared with `m5_pr7_ssi.rs`'s**, which is this
/// box: this suite reviews that one, so the region it marches in has
/// to be its own. See that file's copy for the same note.
fn wall_domain() -> SsiDomain {
    SsiDomain {
        center: Point3::new(0.5, 0.0, 0.4),
        half_extent: 2.0,
        extent: 1.5,
        floor_scale: 1.0,
    }
}

/// The dense-scan max of |S(P(t)) − C(t)| for a traced pair, plus the
/// argmax parameter.
/// `e` is the **generator's** step tolerance, not the run's: this door
/// returns no certificate, which is the whole reason it takes one at
/// all. It is the only door in the module that does.
fn trace_deviation(w: &NurbsSurface<f64>, e: f64, samples: u32) -> (f64, f64) {
    let p = cutting_plane();
    let (carrier, _pa, pb) =
        match ssi::trace_plane_nurbs_uncertified(&p, w, (0.5, 0.5), wall_domain(), e, band()) {
            Ok(t) => t,
            Err(err) => panic!("trace: {err}"),
        };
    let (t0, t1) = carrier.domain();
    let mut max = 0.0f64;
    let mut arg = t0;
    for i in 0..=samples {
        let t = t0 + (t1 - t0) * (f64::from(i) / f64::from(samples));
        let q = pb.eval(t);
        let r = (w.eval(q.x, q.y) - carrier.eval(t)).norm();
        if r > max {
            max = r;
            arg = t;
        }
    }
    (max, pb.eval(arg).x)
}

#[test]
fn deviation2a_the_inflected_wall_deviation_is_real_geometry() {
    // Reproduce the fit-pair deviation with NO ring code in the loop:
    // two independently fitted objects (carrier, pcurve∘surface)
    // evaluated directly, 200k samples. If the number were an artifact
    // of the composite or the certificate, this scan could not see it.
    //
    // The march runs at ε = 1e-9, handed to `trace_deviation`
    // explicitly. The door then refines the trace where the certificate,
    // at the ambient band, refuses it, as the certifying door does. On a
    // band whose zero is above the march's own deviation nothing is
    // refined, and the scan reads the march's: below the march's ε,
    // since each step is kept only where its predicted state lies on
    // the locus, so the section's curvature zero, where the
    // `h_fit ∝ (ε/κ³)^¼` rung unbinds, is stepped as finely as the
    // locus asks (measured 1.31e-10 m). On a finer band the gap with
    // the deviation is halved until limb 2 is answered.
    let w = nurbs_wall();
    let (max, u_at_max) = trace_deviation(&w, 1e-9, 200_000);
    eprintln!("[review] inflected-wall fit deviation: {max:.3e} m at u = {u_at_max:.4}");
    if band().zero() > 4.8e-9 {
        assert!(
            max <= 1e-9,
            "the march's deviation exceeds its ε: {max:e} at u = {u_at_max:.4}"
        );
    } else {
        assert!(
            max < 4.2e-9,
            "the march's deviation is not refined at ε {:e}: {max:e}",
            band().zero()
        );
    }
}

#[test]
fn deviation1_and_3_domain_mismatch_refuses_typed_with_the_recourse() {
    // Deviation 3: the ComposeError::DomainMismatch Display carries the
    // S6/S9 message shape — one situation, one recourse, both domains
    // as payload data.
    let e = ComposeError::DomainMismatch {
        a: (0.0, 1.0),
        b: (0.0, 2.0),
    };
    let msg = format!("{e}");
    assert!(msg.contains("one knot domain"), "{msg}");
    assert!(
        msg.contains("[0, 1]") && msg.contains("[0, 2]"),
        "both domains as data: {msg}"
    );
    assert!(
        msg.contains("refit the pair on one parameterization"),
        "the recourse sentence: {msg}"
    );
    // Deviation 1: at the certify entry the mismatch is a TYPED refusal
    // (UnsupportedCertificate naming the OQ4 identity), never a silent
    // bound. The merge-base path never certified this shape either (the
    // midpoint term evaluates the pcurve at foreign parameters and
    // reports geometry-scale error), so nothing regressed from
    // certifying to refusing.
    let w = nurbs_wall();
    let p = cutting_plane();
    let (carrier, _pa, pb) = match ssi::trace_plane_nurbs_uncertified(
        &p,
        &w,
        (0.5, 0.5),
        wall_domain(),
        band().zero(),
        band(),
    ) {
        Ok(t) => t,
        Err(err) => panic!("trace: {err}"),
    };
    let knots: Vec<f64> = pb.knots().knots().iter().map(|k| k * 2.0).collect();
    let stretched = KnotVector::clamped(knots, pb.knots().degree()).unwrap();
    let bad = geom::NurbsCurve2::new(stretched, pb.control().to_vec(), pb.weights().to_vec())
        .expect("structure fine");
    let err = ssi::certify_rung3(
        &carrier,
        Some(&bad),
        &SsiOperand::Analytic(&p),
        &SsiOperand::nurbs(&w).expect("the wall's chart speeds mint"),
        TubeScale::uniform(1.5),
        band(),
    )
    .expect_err("a domain-mismatched pcurve cannot certify");
    match err {
        SsiError::UnsupportedCertificate { what } => {
            assert!(what.contains("knot domains"), "{what}");
        }
        other => panic!("expected the typed OQ4 refusal, got {other}"),
    }
}

#[test]
fn retirement_breadth_a_multicell_wall_is_served_or_refuses_loudly() {
    // The retired arm now claims route(Plane, Nurbs).implemented for
    // ALL NURBS walls. A wall with an INTERIOR knot makes the pcurve's
    // span windows straddle a knot line, where the composite hulls the
    // neighbor cell's polynomial extension — the bound inflates by the
    // C²-join mismatch, orders above ε. Record what actually happens:
    // certification (great) or a typed/in-band refusal (honest, but the
    // arm's practical breadth is single-cell — worth the record).
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let cols = [
        (0.0, 0.0),
        (0.26, 0.10),
        (0.52, 0.18),
        (0.78, 0.24),
        (1.05, 0.28),
    ];
    let mut control = Vec::with_capacity(10);
    for (x, y) in cols {
        control.push(Point3::new(x, y, 0.0));
        control.push(Point3::new(x, y, 0.8));
    }
    let w = NurbsSurface::new(ku, kv, control, vec![1.0; 10]).unwrap();
    // Refined where limb 2 refuses, the wall certifies at each ε the
    // suite runs (measured 37, 162 and 827 samples at 1e-6, 1e-9 and
    // 1e-12); elsewhere a refusal must still be the typed kind.
    let certifies_here = [1.0e-6, 1.0e-9, 1.0e-12].contains(&eps());
    match ssi::plane_nurbs_ssi(&cutting_plane(), &w, wall_domain(), band()) {
        Ok(out) => {
            let sup = out.branches[0].certificate.hull_sup;
            eprintln!("[review] multi-cell wall CERTIFIED, hull_sup {sup:.3e}");
            assert!(sup <= eps());
        }
        Err(e) => {
            assert!(
                !certifies_here,
                "the multicell wall certifies once refined at ε {:e}: {e}",
                eps()
            );
            // A refusal must be the loud, typed kind — never a panic
            // (reaching here at all proves that much); pin that it is
            // the hull limb or an in-band escalation, i.e. the bound
            // stayed honest rather than lying under ε.
            match e {
                SsiError::CertificateLimb { .. }
                | SsiError::Escalated { .. }
                | SsiError::CertificateEscalated { .. }
                | SsiError::RefinementExhausted { .. }
                | SsiError::ExhaustivenessInconclusive(_) => {}
                other => panic!("unexpected refusal shape: {other}"),
            }
        }
    }
}
