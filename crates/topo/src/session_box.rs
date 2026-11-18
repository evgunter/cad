//! **The session box** (D4 ¶4; `crates/geom-core/README.md`, "Float
//! rounding"): one predicate over a body's largest `|coordinate|`, read
//! from below off its vertices' and its edges' own points, never a
//! carrier's stored terms. The finished-body gate asks it first, so a
//! body it refuses has a point past the box. What it does not certify is
//! the converse: a curved face's interior extreme, bounded by no edge,
//! is not read, so a body it admits may still reach past the box there
//! (`work/oneround/the-session-box-reach-misses-a-curved-faces-bulge.md`).
//!
//! An edge's reach is the edge's one reading, its cover
//! (`Body::edge_cover`), read along the world axes from below
//! ([`geom_brep::edge_coordinate_reach_below`]), so a body is refused
//! only where points of it lie past the box.

use geom::cover::BoxAxes;
use geom_core::{Band, Decide, Lever, Margin, Point3, Sign};

use crate::body::Body;
use crate::validate::ValidationError;

/// **`body`'s largest `|coordinate|`, bounded from below** — the
/// session box's measure (`[tolerance.units]`: no coordinate of a model
/// is larger in magnitude than the box, so the box is a cube, not a
/// ball): each vertex's point and each certified edge's largest
/// `|coordinate|` over its own span, every one a coordinate of a point
/// of the body.
pub(crate) fn body_reach<T: Decide>(body: &Body<T>) -> T {
    body_reach_by(body, |e| {
        body.edge_cover(e)
            .ok()
            .map(|cover| cover.box_in(&BoxAxes::World))
    })
}

/// **`body`'s largest `|coordinate|` where it stands under `motion`,
/// bounded from below** ([`body_reach`] of its image, the body unmoved):
/// each vertex's point and each certified edge's carrier sent through
/// the motion as the image's are (`transform::map_isometry`: the
/// motion's affine on a point, `map_carrier` on a carrier), each edge's
/// world box its cover over its span hulled with its two mapped ends, as
/// the image's cover is (`cover::edge_cover_over`). So it reads the
/// image's own quantity, from the image's own data, which a placement
/// holds without building the image.
///
/// # Errors
///
/// A carrier the motion does not map ([`crate::TransformError`]), which
/// the image's map refuses too.
pub(crate) fn body_reach_at<T: Decide>(
    body: &Body<T>,
    motion: &geom_core::RigidMotion<T>,
) -> Result<T, crate::TransformError> {
    let iso = crate::Isometry::Proper(*motion);
    let map = iso.to_affine();
    let origin = Point3::origin();
    let vertices = body
        .vertices()
        .map(|(v, data)| (map.transform_point(body.point_of(v, data)) - origin).norm_inf());
    let mut edges = Vec::new();
    for (e, data) in body.edges() {
        let Some(curve) = body.edge_curve_linked(e, data).certified() else {
            continue;
        };
        let carrier = crate::transform::map_carrier(&iso, curve.carrier())?;
        let ends = (
            map.transform_point(crate::cover::edge_end_point(body, e, data.he_plus, "he_plus")),
            map.transform_point(crate::cover::edge_end_point(body, e, data.he_minus, "he_minus")),
        );
        let world_box = crate::cover::edge_cover_over(&carrier, curve.params(), ends)
            .ok()
            .map(|cover| cover.box_in(&BoxAxes::World));
        edges.push(geom_brep::edge_coordinate_reach_below(
            &carrier,
            curve.params(),
            world_box,
        ));
    }
    Ok(vertices.chain(edges).fold(T::zero(), T::max))
}

/// [`body_reach`] with each edge's world box read by `world_box`; an
/// edge it gives none reads its own sampled points instead
/// (`geom_brep::edge_coordinate_reach_below`), so one missing reading
/// never silences the rest of the body.
fn body_reach_by<T: Decide>(
    body: &Body<T>,
    world_box: impl Fn(crate::entity::EdgeKey) -> Option<(Point3<T>, Point3<T>)>,
) -> T {
    let origin = Point3::origin();
    let vertices = body
        .vertices()
        .map(|(v, data)| (body.point_of(v, data) - origin).norm_inf());
    // The edge's one reading (`[one-way.places]`): its cover's box along
    // the world axes, through the body's door (`Body::edge_cover`).
    let edges = body.edges().filter_map(|(e, data)| {
        let curve = body.edge_curve_linked(e, data).certified()?;
        Some(geom_brep::edge_coordinate_reach_below(
            curve.carrier(),
            curve.params(),
            world_box(e),
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
    box_decided(body_reach(body), band)
}

/// **`session_box_reach` of `body` where it stands under `motion`**:
/// [`in_session_box`] of its image, read off the body unmoved
/// ([`body_reach_at`]).
///
/// # Errors
///
/// [`crate::MoveError::Map`] where the motion does not map a carrier;
/// [`crate::MoveError::NotFinished`] holding
/// [`ValidationError::OutsideSessionBox`], with the margin.
pub(crate) fn in_session_box_at<T: Decide>(
    body: &Body<T>,
    motion: &geom_core::RigidMotion<T>,
    band: Band,
) -> Result<(), crate::MoveError> {
    let reach = body_reach_at(body, motion).map_err(crate::MoveError::Map)?;
    box_decided(reach, band)
        .map_err(|finding| crate::MoveError::NotFinished(crate::NotFinished::new(vec![finding])))
}

/// The box's decision on a reach: its margin the box's extent less it.
fn box_decided<T: Decide>(reach: T, band: Band) -> Result<(), ValidationError> {
    let margin = Margin::of(Lever::<T>::session_box().length() - reach);
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

    /// **Tier 3′ asks the box as tier 3 does**: with no contact record
    /// tier 3′ is tier 3 exactly, so the same brick 2 km out is refused
    /// by tier 3′'s own path, alone and by name, and one at the origin is
    /// not. *Breaks if* tier 3′'s battery skips tier 3's gate, which
    /// passes the far brick.
    #[test]
    fn tier_3_prime_refuses_a_body_past_the_session_box_as_tier_3_does() {
        let tol = Tol::witness();
        // No contact record: the door's own signature names the type.
        let records = Default::default();
        let far = brick::<f64>((2e3, 2e3 + 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let errors = crate::validate::validate_pseudomanifold(&far, &records, tol).unwrap_err();
        assert!(
            matches!(&errors[..], [ValidationError::OutsideSessionBox { .. }]),
            "the box refusal, alone: {errors:?}"
        );
        let near = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        assert!(crate::validate::validate_pseudomanifold(&near, &records, tol).is_ok());
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

    /// **A missing edge reading silences nothing**: with every edge's
    /// cover withheld, the edges read their own samples, and a body with
    /// a vertex past the box still reads past it. Red against an edge
    /// with no cover reading poison, which `max` carries through the whole
    /// fold so that no sign decides and nothing is refused.
    #[test]
    fn an_uncovered_edge_leaves_the_rest_of_the_reach() {
        let tol = Tol::witness();
        let far = brick::<f64>((1.1e3, 1.1e3 + 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let reach = body_reach_by(&far, |_| None);
        assert!(reach >= 1.1e3 + 1.0, "{reach}");
    }

    /// **A placement's reach is its image's, to the bit**
    /// ([`body_reach_at`] against [`body_reach`] of the moved body): a
    /// brick and a sphere cap (its rim a circle arc, read off its cover)
    /// under a motion off every axis. *Breaks if* the placed reading
    /// reads other data than the image's ask (a vertex alone, an edge's
    /// cover without its mapped ends, an unmapped carrier).
    #[test]
    fn a_placements_reach_is_its_images_to_the_bit() {
        let tol = Tol::witness();
        let band = geom_core::Band::linear(tol).unwrap();
        let axis = geom_core::UnitVec3::new(
            geom_core::Vec3::new(1.0, 2.0_f64.sqrt(), 3.0_f64.sqrt()),
            "a turn off every axis",
            band,
        )
        .unwrap();
        let motion = geom_core::RigidMotion::translation(geom_core::Vec3::new(637.0, -251.25, 82.5))
            .unwrap()
            .compose(geom_core::RigidMotion::rotation(axis, 0.7).unwrap())
            .unwrap();
        let bodies = [
            brick::<f64>((0.0, 1.0), (0.0, 2.0), (0.0, 0.5), tol),
            crate::test_support_fixtures::sphere_cap(0.3, tol).0,
        ];
        for body in &bodies {
            let image =
                crate::transform::map_isometry(body, &crate::Isometry::Proper(motion)).unwrap();
            assert_eq!(
                body_reach_at(body, &motion).unwrap().to_bits(),
                body_reach(&image).to_bits(),
                "the placement reads its image's reach"
            );
        }
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
