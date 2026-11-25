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
    unmoved_reach(body, edge_world_box)
}

/// [`reach_of`] over the body as it stands, each edge's world box read
/// by `world_box`: no map, so nothing refuses.
fn unmoved_reach<T: Decide>(
    body: &Body<T>,
    world_box: impl Fn(
        &geom_brep::EdgeCurve<T>,
        (Point3<T>, Point3<T>),
    ) -> Option<(Point3<T>, Point3<T>)>,
) -> T {
    match reach_of::<T, core::convert::Infallible>(body, |p| p, |_| Ok(None), world_box) {
        Ok(reach) => reach,
        Err(never) => match never {},
    }
}

/// **`body`'s largest `|coordinate|` where it stands under `motion`,
/// bounded from below** ([`body_reach`] of its image, the body unmoved):
/// the one fold [`body_reach`] reads, over the data
/// `transform::map_isometry` writes into the image. Each vertex's point
/// goes through the motion's affine; each certified edge's stored
/// carrier through `map_carrier_stretched`, its reading over its span re-taken on
/// the image (`EdgeCurve::carried`); its two ends through the affine. So
/// it reads the image's own quantity, from the image's own data, which a
/// placement holds without building the image.
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
    reach_of(
        body,
        |p| map.transform_point(p),
        |curve| placed_edge(&iso, curve).map(Some),
        edge_world_box,
    )
}

/// **An edge where it stands under `iso`, as the image holds it**: its
/// stored carrier mapped (`map_carrier_stretched`) and its reading over
/// its span, stretched as the map stretched the carrier's parameter,
/// re-taken on the image (`EdgeCurve::carried`), as
/// `transform::map_isometry` writes it. On a periodic cubic the reading
/// is a clamped view of the loop, and mapping that view instead rounds
/// differently. Only the carrier and its reading are read off it: the
/// description and authority are the source's, unmapped.
///
/// # Errors
///
/// A carrier the motion does not map.
fn placed_edge<T: Decide>(
    iso: &crate::Isometry<T>,
    curve: &geom_brep::EdgeCurve<T>,
) -> Result<geom_brep::EdgeCurve<T>, crate::TransformError> {
    let (carrier, stretch) = crate::transform::map_carrier_stretched(iso, curve.stored_carrier())?;
    Ok(curve.carried(
        carrier,
        stretch,
        curve.description().clone(),
        curve.authority(),
    ))
}

/// **One edge's world box**, its ends as they stand: its carrier's
/// cover over its span hulled with the ends (`cover::edge_cover_over`,
/// the image's `Body::edge_cover`), read along the world axes; `None`
/// where the cover refuses.
fn edge_world_box<T: Decide>(
    curve: &geom_brep::EdgeCurve<T>,
    ends: (Point3<T>, Point3<T>),
) -> Option<(Point3<T>, Point3<T>)> {
    crate::cover::edge_cover_over(curve.carrier(), curve.params(), ends)
        .ok()
        .map(|cover| cover.box_in(&BoxAxes::World))
}

/// **The one reach fold**, over the body's data as `place` and
/// `curve_of` carry it into the space it is read in: each vertex's
/// placed point, and each certified edge's largest `|coordinate|` over
/// its span (`geom_brep::edge_coordinate_reach_below`), its world box
/// `world_box` of the edge with its two placed ends ([`edge_world_box`] in
/// every caller but a row's). `curve_of` gives the edge as it stands
/// there, or `None` where it stands as stored. An edge with no world box
/// reads its own sampled points, so one missing reading never silences
/// the rest of the body.
fn reach_of<T: Decide, E>(
    body: &Body<T>,
    place: impl Fn(Point3<T>) -> Point3<T>,
    curve_of: impl Fn(&geom_brep::EdgeCurve<T>) -> Result<Option<geom_brep::EdgeCurve<T>>, E>,
    world_box: impl Fn(
        &geom_brep::EdgeCurve<T>,
        (Point3<T>, Point3<T>),
    ) -> Option<(Point3<T>, Point3<T>)>,
) -> Result<T, E> {
    let origin = Point3::origin();
    let mut reach = body
        .vertices()
        .map(|(v, data)| (place(body.point_of(v, data)) - origin).norm_inf())
        .fold(T::zero(), T::max);
    for view in body.edge_views() {
        let (e, data) = (view.key(), view.record());
        let Some(stored) = view.certified() else {
            continue;
        };
        let moved = curve_of(stored)?;
        let curve = moved.as_ref().unwrap_or(stored);
        let end = |he, which| place(crate::cover::edge_end_point(body, e, he, which));
        let ends = (end(data.he_plus, "he_plus"), end(data.he_minus, "he_minus"));
        reach = reach.max(geom_brep::edge_coordinate_reach_below(
            curve.carrier(),
            curve.params(),
            world_box(curve, ends),
        ));
    }
    Ok(reach)
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
        let reach = unmoved_reach(&far, |_, _| None);
        assert!(reach >= 1.1e3 + 1.0, "{reach}");
    }

    /// **A placement's reach is its image's, to the bit**
    /// ([`body_reach_at`] against [`body_reach`] of the moved body): a
    /// brick and a sphere cap (its rim a circle arc, read off its cover)
    /// under a motion off every axis, and a brick 900 m out moved back to
    /// the origin, where an edge's unmapped ends would hull its cover 900 m
    /// out. *Breaks if* the placed reading
    /// reads other data than the image's ask (a vertex alone, an edge's
    /// cover without its mapped ends, an unmapped carrier). A periodic
    /// cubic edge, whose stored loop and clamped reading differ, is
    /// pinned one level down
    /// ([`a_periodic_edges_placed_reach_is_its_images_to_the_bit`]): no
    /// body fixture has periodic cubic edges, and at the body both reads
    /// run through the one fold [`reach_of`], so the edge's mapping is
    /// the only place the two could differ.
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
        let motion =
            geom_core::RigidMotion::translation(geom_core::Vec3::new(637.0, -251.25, 82.5))
                .unwrap()
                .compose(geom_core::RigidMotion::rotation(axis, 0.7).unwrap())
                .unwrap();
        // A body far out moved in toward the origin: its unmapped ends,
        // 900 m out, would set an edge's hull past everything the image
        // reaches.
        let home =
            geom_core::RigidMotion::translation(geom_core::Vec3::new(-900.0, 0.0, 0.0)).unwrap();
        let cases = [
            (
                brick::<f64>((0.0, 1.0), (0.0, 2.0), (0.0, 0.5), tol),
                motion,
            ),
            (crate::test_support_fixtures::sphere_cap(0.3, tol).0, motion),
            (
                brick::<f64>((900.0, 901.0), (0.0, 2.0), (0.0, 0.5), tol),
                home,
            ),
        ];
        for (body, motion) in &cases {
            let image =
                crate::transform::map_isometry(body, &crate::Isometry::Proper(*motion)).unwrap();
            assert_eq!(
                body_reach_at(body, motion).unwrap().to_bits(),
                body_reach(&image).to_bits(),
                "the placement reads its image's reach"
            );
        }
        assert!(
            body_reach(
                &crate::transform::map_isometry(&cases[2].0, &crate::Isometry::Proper(home))
                    .unwrap()
            ) < 10.0,
            "the far brick moved home reaches a few metres"
        );
    }

    /// **A periodic cubic edge is placed as the image re-reads it, to the
    /// bit**: forty edges, each a span of one closed periodic cubic in a
    /// plane, the spans straddling its knots, under a motion off every
    /// axis. Each edge's reach as the placement reads it ([`placed_edge`]
    /// then its world box, [`edge_world_box`]) is its image's: the edge planted
    /// in a brick and the brick mapped by `transform::map_isometry`. The body-level row cannot hold
    /// one: no body fixture has periodic cubic edges. *Breaks if* the
    /// placement maps the edge's clamped view instead, which rounds off
    /// the image's on some of the spans; the row counts those, so the
    /// contrast is shown and not assumed.
    #[test]
    fn a_periodic_edges_placed_reach_is_its_images_to_the_bit() {
        let tol = Tol::witness();
        let band = geom_core::Band::linear(tol).unwrap();
        let axis = geom_core::UnitVec3::new(
            geom_core::Vec3::new(1.0, 2.0_f64.sqrt(), 3.0_f64.sqrt()),
            "a turn off every axis",
            band,
        )
        .unwrap();
        let motion =
            geom_core::RigidMotion::translation(geom_core::Vec3::new(637.0, -251.25, 82.5))
                .unwrap()
                .compose(geom_core::RigidMotion::rotation(axis, 0.7).unwrap())
                .unwrap();
        let iso = crate::Isometry::Proper(motion);
        let map = iso.to_affine();
        let points: Vec<Point3<f64>> = (0..40)
            .map(|i| {
                let a = core::f64::consts::TAU * f64::from(i) / 40.0;
                Point3::new(0.6 * a.cos() + 0.1 * (3.0 * a).cos(), 0.4 * a.sin(), 0.0)
            })
            .collect();
        let net = geom::PeriodicNurbs3::interpolate(&points).unwrap();
        let carrier = geom::Curve3::Periodic(std::sync::Arc::new(net));
        let mut view_differs = 0;
        for k in 0..40 {
            let t0 = (f64::from(k) + 0.37) / 40.0;
            let t1 = t0 + 1.0 / 40.0;
            // A certified edge over the span, its carrier then replaced
            // by the loop as a map replaces one (`EdgeCurve::carried`):
            // the loop stored, its clamped view over the span the
            // reading. No door certifies a periodic cubic on one plane.
            let arc = geom::Curve3::circle(
                Point3::origin(),
                geom_core::Vec3::unit_z(),
                1.0,
                geom_core::Vec3::unit_x(),
            );
            let spec = geom_brep::EdgeCurveSpec::arc_of_circle(arc.clone(), t0, t1).unwrap();
            let span = geom_brep::EdgeCurve::certify(
                spec,
                arc.eval(t0),
                arc.eval(t1),
                |_| None,
                geom_core::test_support::fixture_lever(1.0),
                band,
                geom_brep::NurbsLane::certified(),
            )
            .unwrap_or_else(|e| panic!("span {k} certifies: {e:?}"));
            let curve = span.carried(
                carrier.clone(),
                1.0,
                span.description().clone(),
                span.authority(),
            );
            assert!(
                !matches!(curve.carrier(), geom::Curve3::Periodic(_)),
                "the reading is a clamped view of the loop"
            );
            let ends = (
                map.transform_point(carrier.eval(t0)),
                map.transform_point(carrier.eval(t1)),
            );
            // The image as the transform writes it: the edge planted in a
            // brick's arena and the brick mapped (`map_isometry`).
            let mut host = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
            let edge = host.edges().map(|(e, _)| e).next().unwrap();
            let key = host.get_edge(edge).unwrap().curve;
            *host.curves.get_mut(key).unwrap() = crate::null::CurveGeom::Certified(curve.clone());
            let moved = crate::transform::map_isometry(&host, &iso).unwrap();
            let Some(crate::null::CurveGeom::Certified(image)) = moved.curves.get(key) else {
                panic!("span {k}: the image holds the edge's curve");
            };
            let reach = |c: &geom_brep::EdgeCurve<f64>| {
                geom_brep::edge_coordinate_reach_below(
                    c.carrier(),
                    c.params(),
                    edge_world_box(c, ends),
                )
            };
            let placed = reach(&placed_edge(&iso, &curve).unwrap());
            let imaged = reach(image);
            assert_eq!(placed.to_bits(), imaged.to_bits(), "span {k}");
            let viewed = crate::transform::map_carrier(&iso, curve.carrier()).unwrap();
            let viewed = curve.carried(viewed, 1.0, curve.description().clone(), curve.authority());
            if reach(&viewed).to_bits() != imaged.to_bits() {
                view_differs += 1;
            }
        }
        assert!(
            view_differs > 0,
            "the clamped view's map rounds off the image's on some span"
        );
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
