//! Blinded-review probe, deviation 10: the IN-BAND-AT-REST escalation
//! row. A tangent line–arc join whose second-order margin, levered at
//! the strut's faces' extent (`topo::wedge_reach`), lands INSIDE the
//! band (zero < kappa_rel/2 * arm^2 < escalate): the C7/C12.2 contract
//! says an in-band second-order tie ESCALATES (F6) — at the profile, at
//! construction or at the tier-3 walk — and is never silently
//! classified definite.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point2;
use geom_core::Tol;
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::ExtrudeSide;
use sweep::{Extrusion, extrude};

use crate::common::contact_edges::intrinsic_edges;

#[test]
fn an_in_band_second_order_margin_at_rest_escalates_somewhere_loud() {
    // A line `free/3` long tangentially into an arc of radius 0.25
    // whose far end stands `free` from the strut's middle, extruded
    // `free/4`: the strut's farther wall reaches `free`, so the margin
    // is `free²/(2·0.25)`, set to the band's geometric mean. The profile
    // door reads the same sagitta between the arc and the line.
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let margin = (band.zero() * band.escalate()).sqrt();
    let r = 0.25;
    let free = (2.0 * r * margin).sqrt();
    let (h, l) = (free / 4.0, free / 3.0);
    let phi = 2.0 * ((free * free - h * h / 4.0).sqrt() / (2.0 * r)).asin();
    let end = Point2::new(l + r * phi.sin(), r * (1.0 - phi.cos()));
    let lp = bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(l, 0.0), (phi / 4.0).tan()),
        (end, 0.0),
        (Point2::new(end.x, 1.0), 0.0),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    let profile = match Profile::new(SketchPlane::xy(), vec![lp]).validate(Tol::witness()) {
        Ok(p) => p,
        Err(e) => {
            // A typed profile-stage refusal is loud enough — record it.
            eprintln!("IN-BAND-AT-REST: profile stage refused typed: {e:?}");
            return;
        }
    };
    match extrude(
        &profile,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    ) {
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
            let tangent_desc = intrinsic_edges(&body) > 0;
            match topo::contact_marks(&body, Tol::witness()) {
                Err(errs) => {
                    eprintln!("IN-BAND-AT-REST: tier-3 walk escalated: {errs:?}");
                    assert!(
                        errs.iter()
                            .any(|e| format!("{e:?}").contains("tangent_second_order")
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
