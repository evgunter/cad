//! **The session box** (D4 ¶4; `crates/geom-core/README.md`, "Float
//! rounding"): one predicate over a body's largest `|coordinate|`, its
//! vertices' and its edges' own points (which bound its faces), never a
//! carrier's stored terms. The finished-body gate asks it before any check reads a
//! margin on the body's geometry, so every such check takes "in the box"
//! as its premise.
//!
//! An edge's reach is the edge's one reading, its cover
//! (`Body::edge_cover`), read from below ([`geom_brep::edge_reach_bounds`]),
//! so a body is refused only where points of it lie past the box.

use geom_core::{Band, Decide, Lever, Margin, Point3, Sign, Vec3};

use crate::body::Body;
use crate::validate::ValidationError;

/// **`body`'s largest `|coordinate|`, bounded from below** — the
/// session box's measure (`[tolerance.units]`: no coordinate of a model
/// is larger in magnitude than the box, so the box is a cube, not a
/// ball): each vertex's point and each certified edge's largest
/// `|coordinate|` over its own span, every one a coordinate of a point
/// of the body.
pub(crate) fn body_reach<T: Decide>(body: &Body<T>) -> T {
    let origin = Point3::origin();
    let vertices = body
        .vertices()
        .map(|(v, data)| (body.point_of(v, data) - origin).norm_inf());
    // The edge's one reading (`[one-way.places]`): its cover, through
    // the body's door (`Body::edge_cover`), read along each axis. On a
    // line or a circle arc that range is the edge's own
    // (`geom_brep::edge_coordinate_reach_below`), and every other kind
    // reads its own sampled points. An edge with no cover reads poison,
    // which no sign decides, so the box does not refuse on it.
    let axes = [
        Vec3::new(T::one(), T::zero(), T::zero()),
        Vec3::new(T::zero(), T::one(), T::zero()),
        Vec3::new(T::zero(), T::zero(), T::one()),
    ];
    let edges = body.edges().filter_map(|(e, data)| {
        let curve = body.edge_curve_linked(e, data).certified()?;
        let within = body
            .edge_cover(e)
            .ok()
            .map_or(T::from_f64(f64::NAN), |cover| {
                axes.iter().fold(T::zero(), |far, &axis| {
                    let (lo, hi) = cover.range_along(axis);
                    far.max(lo.abs()).max(hi.abs())
                })
            });
        Some(geom_brep::edge_coordinate_reach_below(
            curve.carrier(),
            curve.params(),
            within,
        ))
    });
    vertices.chain(edges).fold(T::zero(), T::max)
}

/// The session box's decision: its margin the box's extent less the
/// body's largest `|coordinate|`.
pub(crate) const SESSION_BOX_REACH: &str = "session_box_reach";

/// **`session_box_reach`**: whether `body` lies in the session box, the
/// margin the box's extent less [`body_reach`], its largest
/// `|coordinate|`. Only a definitely negative
/// margin refuses: geometry within the band of the box's edge is in it.
///
/// # Errors
///
/// [`ValidationError::OutsideSessionBox`], with the margin.
pub(crate) fn in_session_box<T: Decide>(body: &Body<T>, band: Band) -> Result<(), ValidationError> {
    let margin = Margin::of(Lever::<T>::session_box().length() - body_reach(body));
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
    /// A metre brick 2 km out has its largest coordinate at 2001 m, 1001 m
    /// past the box; at 950 m out, 951 m. Red against the mutant "the gate asks nothing", and
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
            said.contains("session box") && said.contains("1e3 m") && said.contains("1001 m"),
            "{said}"
        );
        assert!(
            margin.readable_below_zero().to_string().starts_with("1001"),
            "{said}"
        );
        for x in [0.0, 950.0, -950.0] {
            let near = brick::<f64>((x, x + 1.0), (0.0, 1.0), (0.0, 1.0), tol);
            assert!(refusal(&near).is_none(), "a brick at {x} m is in the box");
            assert!(
                body_reach(&near) < geom_core::tolerance::SESSION_BOX_EXTENT,
                "{x}"
            );
        }
    }

    /// **The box is a cube, not a ball**: a brick in the box's corner,
    /// every coordinate within `[900, 999]` m, lies `≈ 1.73` km from the
    /// origin and is in the box; pushed one metre past it along one axis,
    /// it is not. Red against the mutant reading the Euclidean distance.
    #[test]
    fn a_body_in_the_boxs_corner_is_in_it() {
        let tol = Tol::witness();
        let corner = brick::<f64>((900.0, 999.0), (900.0, 999.0), (900.0, 999.0), tol);
        assert!(
            refusal(&corner).is_none(),
            "every coordinate is within 1 km"
        );
        assert_eq!(body_reach(&corner), 999.0);
        let past = brick::<f64>((900.0, 1001.0), (900.0, 999.0), (900.0, 999.0), tol);
        assert!(refusal(&past).is_some(), "x reaches 1001 m");
    }

    /// **The reach is never short of a vertex's coordinate**: a long thin
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
