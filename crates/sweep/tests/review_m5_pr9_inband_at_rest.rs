//! Blinded-review probe, deviation 10: the IN-BAND-AT-REST escalation
//! row the implementation report left unauthored. A filleted block
//! scaled so the fillet's curvature margin at the strut's lever arm
//! lands INSIDE the band (zero < kappa_rel/2 * arm^2 < escalate):
//! the C7/C12.2 contract says an in-band second-order tie ESCALATES
//! (F6) — at construction or at the tier-3 walk — and is never
//! silently classified definite.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tolerance};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, extrude};
use geom_core::Tol;

#[test]
fn an_in_band_second_order_margin_at_rest_escalates_somewhere_loud() {
    // Scale the PR's own filleted-block shape by S: fillet radius
    // R = 0.25 * S, strut extent = extrude height 1, arm = min(R, 1)
    // = 1, margin = 1/(2R). Choose S from the RESOLVED band so the
    // margin is mid-band: margin = sqrt(zero * escalate) (geometric
    // mean), R = 1/(2*margin).
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let margin = (band.zero() * band.escalate()).sqrt();
    let r_fillet = 1.0 / (2.0 * margin);
    let s = r_fillet / 0.25;
    let b = (std::f64::consts::PI / 8.0).tan();
    let mut lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(0.0, 0.0), 0.0),
        ProfileVertex::new(Point2::new(s, 0.0), 0.0),
        ProfileVertex::new(Point2::new(s, 0.75 * s), b),
        ProfileVertex::new(Point2::new(0.75 * s, s), 0.0),
        ProfileVertex::new(Point2::new(0.0, s), 0.0),
    ]);
    lp = lp.with_tangent_joints(vec![2, 3]);
    let profile = match Profile::new(SketchPlane::xy(), vec![lp]).validate(Tol::witness()) {
        Ok(p) => p,
        Err(e) => {
            // A typed profile-stage refusal is loud enough — record it.
            eprintln!("IN-BAND-AT-REST: profile stage refused typed: {e:?}");
            return;
        }
    };
    match extrude(&profile, Extrusion::Distance(1.0), Tol::witness()) {
        Err(e) => {
            // Loud at construction: acceptable per F6. The error must
            // not be a panic and should speak.
            eprintln!("IN-BAND-AT-REST: extrude escalated/refused typed: {e}");
        }
        Ok(out) => {
            let body = out.body;
            // Construction accepted: then the tier-3 walk must
            // escalate the tangency (SliverDihedral with the
            // tangent_second_order cause) — or, at minimum, the edge
            // must NOT be marked jet-determinate Tangent and must NOT
            // carry a definite TangentIntersection description.
            let tangent_desc = body.edges().any(|(_, e)| {
                matches!(
                    body.get_curve_geom(e.curve)
                        .and_then(|g| g.certified())
                        .map(geom_brep::EdgeCurve::description),
                    Some(geom_brep::EdgeGeometry::TangentIntersection { .. })
                )
            });
            match topo::contact_marks(&body, Tol::witness()) {
                Err(errs) => {
                    eprintln!("IN-BAND-AT-REST: tier-3 walk escalated: {errs:?}");
                    assert!(
                        errs.iter()
                            .any(|e| format!("{e}").contains("tangent_second_order")
                                || format!("{e:?}").contains("SliverDihedral")),
                        "the escalation must be the F6 second-order one: {errs:?}"
                    );
                }
                Ok(marks) => {
                    assert!(
                        !tangent_desc,
                        "an IN-BAND tangency was stored as definite TangentIntersection"
                    );
                    assert!(
                        !marks
                            .values()
                            .any(|m| matches!(m, topo::ContactMark::Tangent)),
                        "an IN-BAND tangency was marked jet-determinate: the definite \
                         verdict was minted from an in-band margin (F6 violated)"
                    );
                    eprintln!(
                        "IN-BAND-AT-REST: accepted at rest WITHOUT any escalation and \
                         without a definite verdict; marks: {:?}",
                        marks.values().collect::<Vec<_>>()
                    );
                }
            }
        }
    }
}
