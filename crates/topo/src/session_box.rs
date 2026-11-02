//! **The session box** (D4 ¶4; `crates/geom-core/README.md`, "Float
//! rounding"): one predicate over a body's consumed extent, its vertices
//! and its edges' spans (which bound its faces), never a carrier's stored
//! terms. The finished-body gate asks it before any check reads a margin
//! on the body's geometry, so every such check takes "in the box" as its
//! premise.
//!
//! The extent is read through [`geom_brep::Reach::Span`], the consumed
//! extent's span arm.

use geom_core::{Band, Decide, Margin, Point3, Sign, tolerance::SESSION_BOX_EXTENT};

use crate::body::Body;
use crate::validate::ValidationError;

/// **The farthest `body` reaches from the origin**: each vertex's point
/// and each certified edge's span ([`geom_brep::Reach::span_reach_from`]).
pub(crate) fn body_reach<T: Decide>(body: &Body<T>) -> T {
    let origin = Point3::origin();
    let vertices = body
        .vertices()
        .map(|(v, data)| (body.point_of(v, data) - origin).norm());
    let edges = body.edges().filter_map(|(e, data)| {
        let curve = body.edge_curve_linked(e, data).certified()?;
        let (t0, t1) = curve.params();
        let span = geom_brep::Reach::Span {
            carrier: curve.carrier().clone(),
            t0,
            t1,
        };
        Some(span.span_reach_from(origin))
    });
    vertices.chain(edges).fold(T::zero(), T::max)
}

/// The session box's decision: its margin the box's extent less the
/// body's reach.
pub(crate) const SESSION_BOX_REACH: &str = "session_box_reach";

/// **`session_box_reach`**: whether `body` lies in the session box, the
/// margin the box's extent less [`body_reach`]. Only a definitely negative
/// margin refuses: geometry within the band of the box's edge is in it.
///
/// # Errors
///
/// [`ValidationError::OutsideSessionBox`], with the margin.
pub(crate) fn in_session_box<T: Decide>(body: &Body<T>, band: Band) -> Result<(), ValidationError> {
    let margin = Margin::of(T::from_f64(SESSION_BOX_EXTENT) - body_reach(body));
    match crate::validate::decide_reported(SESSION_BOX_REACH, margin, band) {
        Ok(geom_core::Decided {
            sign: Sign::Negative,
            margin,
        }) => Err(ValidationError::OutsideSessionBox { margin }),
        _ => Ok(()),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::test_support_fixtures::brick;
    use geom_core::Tol;

    fn refusal(body: &Body<f64>) -> Option<ValidationError> {
        crate::validate::validate_geometric(body, Tol::witness())
            .err()
            .and_then(|errors| {
                errors
                    .into_iter()
                    .find(|e| matches!(e, ValidationError::OutsideSessionBox { .. }))
            })
    }

    /// **A body past the session box is refused at the finished-body
    /// gate, alone and by name; one inside it, up to its edge, is not.**
    /// A metre brick 2 km out reaches about 2001 m, 1001 m past the box;
    /// at 950 m out it reaches about 951 m. Red against the mutant "the gate asks nothing", and
    /// against one reading only the vertices of a body inside.
    #[test]
    fn a_body_past_the_session_box_is_refused_at_the_finished_body_gate() {
        let tol = Tol::witness();
        let far = brick::<f64>((2e3, 2e3 + 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let errors = crate::validate::validate_geometric(&far, tol).unwrap_err();
        let [ValidationError::OutsideSessionBox { margin }] = &errors[..] else {
            panic!("the box refusal, alone: {errors:?}");
        };
        let said = errors[0].to_string();
        assert!(
            said.contains("session box") && said.contains("1e3 m") && said.contains("1001."),
            "{said}"
        );
        assert!(
            margin
                .readable_below_zero()
                .to_string()
                .starts_with("1001."),
            "{said}"
        );
        for x in [0.0, 950.0, -950.0] {
            let near = brick::<f64>((x, x + 1.0), (0.0, 1.0), (0.0, 1.0), tol);
            assert!(refusal(&near).is_none(), "a brick at {x} m is in the box");
            assert!(body_reach(&near) < SESSION_BOX_EXTENT, "{x}");
        }
    }

    /// **The reach is never short of a vertex's distance**: a long thin
    /// brick whose eight corners lie at 1.2 km is past the box.
    #[test]
    fn the_reach_holds_every_vertex() {
        let tol = Tol::witness();
        let long = brick::<f64>((-1.2e3, 1.2e3), (0.0, 1.0), (0.0, 1.0), tol);
        assert!(body_reach(&long) >= 1.2e3, "{}", body_reach(&long));
        assert!(
            refusal(&long).is_some(),
            "corners at 1.2 km are past the box"
        );
    }
}
