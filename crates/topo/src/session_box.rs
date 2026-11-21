//! **The session box** (D4 ¶4; `crates/geom-core/README.md`, "Float
//! rounding"): one predicate over the region a body occupies, never a
//! carrier's stored terms. The box bounds each coordinate
//! (`[tolerance.units]`), so it is a cube, and the measure is the
//! largest `|coordinate|`.
//!
//! The region is read two ways, by the one fold [`reach_of`] over the
//! body's vertices, edges and faces as they stand:
//!
//! - **From outside, by its cover**: each face's region bounded by its
//!   carrier and its boundary alone (`cover::face_cover_unproven`, which
//!   reads no decision), and, only where that reaches past the box, each
//!   face's own cover (`cover::face_cover`, its proven constraints met
//!   in). A face with no cover is a region the box cannot bound.
//! - **From inside, by points known to lie on it**: each vertex, each
//!   certified edge's own points over its span
//!   (`geom_brep::edge_coordinate_reach_below`, exact on lines and circle
//!   arcs), and each face's points where its extremes need not be on its
//!   boundary (`cover::face_extreme_points`, each placed on the face).
//!
//! A body is in the box only where its cover is (`session_box_reach`).
//! Where the cover reaches past it, one with a point past it is refused
//! as past the box (`session_box_point`,
//! [`ValidationError::OutsideSessionBox`]); one whose cover alone
//! reaches past it is refused as a limit
//! ([`ValidationError::SessionBoxUnshown`],
//! [`ValidationError::SessionBoxUncovered`]). The finished-body gate,
//! tier 3′, every move's image and the door that places a body
//! ([`in_session_box_at`]) all ask it.

use geom::Surface;
use geom::cover::{BoxAxes, Cover};
use geom_brep::session_box::{BoxRefusal, cover_reach};
use geom_core::{Band, Decide, Point3};
use slotmap::SecondaryMap;

use crate::body::Body;
use crate::cover::EdgeUncovered;
use crate::entity::{EdgeKey, FaceKey};
use crate::validate::ValidationError;

/// **`body`'s largest `|coordinate|`, bounded from below**: each vertex's
/// point and each certified edge's largest `|coordinate|` over its own
/// span, every one a coordinate of a point of the body. The fold's
/// inside read of the vertices and edges ([`Reads::points`]).
#[cfg(test)]
fn body_reach<T: Decide>(body: &Body<T>) -> T {
    unmoved_reads(body, edge_cover_where).points
}

/// **What the one fold reads of a body as it stands.**
#[derive(Clone, Copy, Debug)]
struct Reads<T> {
    /// The largest `|coordinate|` of points known to lie on the body:
    /// its vertices and its certified edges' own points.
    points: T,
    /// The largest `|coordinate|` of its vertices and of each face's
    /// region bounded by its carrier and boundary alone; the first face
    /// with no such cover where one has none.
    cover: Result<T, FaceKey>,
}

/// [`reach_of`] over the body as it stands, each edge's cover read by
/// `cover_of_edge`: no map, so nothing refuses.
fn unmoved_reads<T: Decide>(
    body: &Body<T>,
    cover_of_edge: impl Fn(
        &geom_brep::EdgeCurve<T>,
        (Point3<T>, Point3<T>),
    ) -> Result<Cover<T>, EdgeUncovered>,
) -> Reads<T> {
    match reach_of::<T, core::convert::Infallible>(
        body,
        |p| p,
        |_| Ok(None),
        |_| Ok(None),
        cover_of_edge,
    ) {
        Ok(reads) => reads,
        Err(never) => match never {},
    }
}

/// **[`Reads`] of `body` where it stands under `motion`**, the body
/// unmoved: the one fold over the data `transform::map_isometry`
/// writes into the image. Each vertex's point goes through the motion's
/// affine; each certified edge's stored carrier through `map_carrier`,
/// its reading over its span re-taken on the image
/// (`EdgeCurve::carried`); its two ends through the affine; each face's
/// carrier through `transform::surface_at`. So it reads the image's own
/// quantities, from the image's own data, which a placement holds
/// without building the image.
///
/// # Errors
///
/// A carrier the motion does not map ([`crate::TransformError`]), which
/// the image's map refuses too.
fn reads_at<T: Decide>(
    body: &Body<T>,
    motion: &geom_core::RigidMotion<T>,
) -> Result<Reads<T>, crate::TransformError> {
    let iso = crate::Isometry::Proper(*motion);
    let map = iso.to_affine();
    reach_of(
        body,
        |p| map.transform_point(p),
        |curve| placed_edge(&iso, curve).map(Some),
        |surface| crate::transform::surface_at(motion, surface).map(Some),
        edge_cover_where,
    )
}

/// **An edge where it stands under `iso`, as the image holds it**: its
/// stored carrier mapped (`map_carrier`) and its reading over its span
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
    let carrier = crate::transform::map_carrier(iso, curve.stored_carrier())?;
    Ok(curve.carried(carrier, curve.description().clone(), curve.authority()))
}

/// **One edge's cover where it stands**: its carrier's cover over its
/// span hulled with its two ends as they stand (`cover::edge_cover_over`,
/// the image's `Body::edge_cover`).
fn edge_cover_where<T: Decide>(
    curve: &geom_brep::EdgeCurve<T>,
    ends: (Point3<T>, Point3<T>),
) -> Result<Cover<T>, EdgeUncovered> {
    crate::cover::edge_cover_over(curve.carrier(), curve.params(), ends)
}

/// **The one reach fold**, over the body's data as `place`, `curve_of`
/// and `surface_of` carry it into the space it is read in: each vertex's
/// placed point; each certified edge's cover `cover_of_edge` of the edge
/// with its two placed ends ([`edge_cover_where`] in every caller but a
/// row's), and its largest `|coordinate|` over its span read from below
/// off it (`geom_brep::edge_coordinate_reach_below`); and each face's
/// region bounded by its placed carrier and its boundary, the hull of
/// those edges' covers and its placed points
/// (`cover::face_cover_unproven`). `curve_of` and `surface_of` give the
/// carrier as it stands there, or `None` where it stands as stored. An
/// edge with no cover reads its own sampled points, so one missing
/// reading never silences the rest of the body.
fn reach_of<T: Decide, E>(
    body: &Body<T>,
    place: impl Fn(Point3<T>) -> Point3<T>,
    curve_of: impl Fn(&geom_brep::EdgeCurve<T>) -> Result<Option<geom_brep::EdgeCurve<T>>, E>,
    surface_of: impl Fn(&Surface<T>) -> Result<Option<Surface<T>>, E>,
    cover_of_edge: impl Fn(
        &geom_brep::EdgeCurve<T>,
        (Point3<T>, Point3<T>),
    ) -> Result<Cover<T>, EdgeUncovered>,
) -> Result<Reads<T>, E> {
    let vertices = points_only(body, &place);
    let mut points = vertices;
    let mut covers: SecondaryMap<EdgeKey, Result<Cover<T>, EdgeUncovered>> = SecondaryMap::new();
    for (e, data) in body.edges() {
        let Some(stored) = body.edge_curve_linked(e, data).certified() else {
            continue;
        };
        let moved = curve_of(stored)?;
        let curve = moved.as_ref().unwrap_or(stored);
        let end = |he, which| place(crate::cover::edge_end_point(body, e, he, which));
        let ends = (end(data.he_plus, "he_plus"), end(data.he_minus, "he_minus"));
        let cover = cover_of_edge(curve, ends);
        points = points.max(geom_brep::edge_coordinate_reach_below(
            curve.carrier(),
            curve.params(),
            cover.as_ref().ok().map(|c| c.box_in(&BoxAxes::World)),
        ));
        covers.insert(e, cover);
    }
    let mut cover = Ok(vertices);
    for (face, _) in body.faces() {
        let f = body.proven_face(face);
        let moved = surface_of(f.surface())?;
        let surface = moved.as_ref().unwrap_or(f.surface());
        let bounded = matches!(
            f.outer().1.boundary,
            crate::entity::LoopBoundary::Cycle { .. }
        );
        let read = crate::cover::face_cover_unproven(surface, bounded, || {
            crate::cover::boundary_cover_by(body, f, &place, |ek, _| {
                covers
                    .get(ek)
                    .cloned()
                    .unwrap_or(Err(EdgeUncovered::Scaffold))
            })
        });
        cover = match (cover, read) {
            (Ok(far), Ok(c)) => Ok(far.max(cover_reach(&c))),
            (Ok(_), Err(_)) => Err(face),
            (Err(first), _) => Err(first),
        };
    }
    Ok(Reads { points, cover })
}

/// The largest `|coordinate|` of `body`'s vertices as `place` sets them.
fn points_only<T: Decide>(body: &Body<T>, place: impl Fn(Point3<T>) -> Point3<T>) -> T {
    let origin = Point3::origin();
    body.vertices()
        .map(|(v, data)| (place(body.point_of(v, data)) - origin).norm_inf())
        .fold(T::zero(), T::max)
}

pub(crate) use geom_brep::session_box::{SESSION_BOX_POINT, SESSION_BOX_REACH};

/// **`session_box_reach`**: whether `body` lies in the session box
/// ([`reach_of`] unmoved, then [`past_its_carriers`]). Only a definitely
/// negative margin refuses: geometry within the band of the box's edge
/// is in it.
///
/// # Errors
///
/// [`ValidationError::OutsideSessionBox`], with the points' margin, or a
/// limit ([`ValidationError::SessionBoxUnshown`],
/// [`ValidationError::SessionBoxUncovered`]).
pub(crate) fn in_session_box<T: Decide>(body: &Body<T>, band: Band) -> Result<(), ValidationError> {
    let reads = unmoved_reads(body, edge_cover_where);
    if within(reads.cover, band) {
        return Ok(());
    }
    past_its_carriers(body, reads.points, band)
}

/// **`session_box_reach` of `body` where it stands under `motion`**:
/// [`in_session_box`] of its image, read off the body unmoved
/// ([`reads_at`]) where its carriers and boundaries bound it in the box,
/// and off the image (`transform::map_isometry`) only where they do not.
///
/// # Errors
///
/// [`crate::MoveError::Map`] where the motion does not map a carrier;
/// [`crate::MoveError::NotFinished`] holding the box's finding.
pub(crate) fn in_session_box_at<T: Decide>(
    body: &Body<T>,
    motion: &geom_core::RigidMotion<T>,
    band: Band,
) -> Result<(), crate::MoveError> {
    let reads = reads_at(body, motion).map_err(crate::MoveError::Map)?;
    if within(reads.cover, band) {
        return Ok(());
    }
    let image = crate::transform::map_isometry(body, &crate::Isometry::Proper(*motion))
        .map_err(crate::MoveError::Map)?;
    past_its_carriers(&image, reads.points, band)
        .map_err(|finding| crate::MoveError::NotFinished(crate::NotFinished::new(vec![finding])))
}

/// Whether a cover's reach decides its region in the box: anything but a
/// definitely negative margin, and no face left uncovered.
fn within<T: Decide>(cover: Result<T, FaceKey>, band: Band) -> bool {
    cover.is_ok_and(|reach| {
        geom_brep::session_box::past_the_box_as(SESSION_BOX_REACH, reach, band).is_none()
    })
}

/// **The box past `body`'s carriers and boundaries**, which reach past
/// it: each face's own cover ([`crate::cover::face_cover`]) bounds the
/// body from outside, and where that too reaches past the box, the
/// points known to lie on it decide which refusal it is. `points` is the
/// fold's inside read of the vertices and edges ([`Reads::points`]); each
/// face's extremes are read here ([`crate::cover::face_extreme_points`]).
fn past_its_carriers<T: Decide>(
    body: &Body<T>,
    points: T,
    band: Band,
) -> Result<(), ValidationError> {
    let mut cover = Ok(points_only(body, |p| p));
    for (face, _) in body.faces() {
        cover = match (cover, crate::cover::face_cover(body, face, band)) {
            (Ok(far), Ok(c)) => Ok(far.max(cover_reach(&c))),
            (Ok(_), Err(_)) => Err(face),
            (Err(first), _) => Err(first),
        };
    }
    let inside = || {
        let origin = Point3::origin();
        body.faces()
            .flat_map(|(face, _)| crate::cover::face_extreme_points(body, face, band))
            .map(|q| (q - origin).norm_inf())
            .fold(points, T::max)
    };
    geom_brep::session_box::region_in_box(cover, inside, band).map_err(|refusal| match refusal {
        BoxRefusal::Point { margin } => ValidationError::OutsideSessionBox { margin },
        BoxRefusal::Unshown { margin } => ValidationError::SessionBoxUnshown { margin },
        BoxRefusal::Uncovered(face) => ValidationError::SessionBoxUncovered { face },
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::test_support_fixtures::brick;
    use geom_core::Tol;

    /// A brick over `x`, `y`, `z` that reaches past the session box,
    /// which certification refuses at its first edge: the brick built at
    /// the origin and its image translated there by
    /// `transform::map_isometry`, which asks nothing, so the box's later
    /// asks meet it.
    fn beyond(x: (f64, f64), y: (f64, f64), z: (f64, f64), tol: Tol) -> Body<f64> {
        let home = brick::<f64>((0.0, x.1 - x.0), (0.0, y.1 - y.0), (0.0, z.1 - z.0), tol);
        let to = geom_core::RigidMotion::translation(geom_core::Vec3::new(x.0, y.0, z.0)).unwrap();
        crate::transform::map_isometry(&home, &crate::Isometry::Proper(to)).unwrap()
    }

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
        let far = beyond((2e3, 2e3 + 1.0), (0.0, 1.0), (0.0, 1.0), tol);
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
        let far = beyond((2e3, 2e3 + 1.0), (0.0, 1.0), (0.0, 1.0), tol);
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
        let past = beyond((900.0, 1001.0), (900.0, 999.0), (900.0, 999.0), tol);
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
        let far = beyond((1.1e3, 1.1e3 + 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let reach = unmoved_reads(&far, |_, _| Err(EdgeUncovered::Ends)).points;
        assert!(reach >= 1.1e3 + 1.0, "{reach}");
    }

    /// **A placement's reach is its image's, to the bit**
    /// ([`reads_at`] against [`unmoved_reads`] of the moved body, points and cover): a
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
            let (placed, imaged) = (
                reads_at(body, motion).unwrap(),
                unmoved_reads(&image, edge_cover_where),
            );
            assert_eq!(
                placed.points.to_bits(),
                imaged.points.to_bits(),
                "the placement reads its image's points"
            );
            assert_eq!(
                placed.cover.unwrap().to_bits(),
                imaged.cover.unwrap().to_bits(),
                "the placement reads its image's cover"
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
    /// then its cover's world box, [`edge_cover_where`]) is its image's: the edge planted
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
                    edge_cover_where(c, ends)
                        .ok()
                        .map(|c| c.box_in(&BoxAxes::World)),
                )
            };
            let placed = reach(&placed_edge(&iso, &curve).unwrap());
            let imaged = reach(image);
            assert_eq!(placed.to_bits(), imaged.to_bits(), "span {k}");
            let viewed = crate::transform::map_carrier(&iso, curve.carrier()).unwrap();
            let viewed = curve.carried(viewed, curve.description().clone(), curve.authority());
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
    /// bar along x whose eight corners lie at 1.2 km is past the box.
    #[test]
    fn the_reach_holds_every_vertex() {
        let tol = Tol::witness();
        // No 2.4 km bar along an axis stands in the box, so it is built
        // along the xy diagonal, its ends 849 m out, and its image turned
        // onto x by `transform::map_isometry`, which asks nothing.
        let band = geom_core::Band::linear(tol).unwrap();
        let z = geom_core::UnitVec3::new(geom_core::Vec3::unit_z(), "z", band).unwrap();
        let turn = |a: f64| geom_core::RigidMotion::rotation(z, a).unwrap();
        let diagonal = turn(core::f64::consts::FRAC_PI_4);
        let bar = crate::test_support_fixtures::mapped_cube::<f64>(
            |x, y, z| diagonal.transform_point(Point3::new(2.4e3 * x - 1.2e3, y, z)),
            tol,
        );
        let long = crate::transform::map_isometry(
            &bar,
            &crate::Isometry::Proper(turn(-core::f64::consts::FRAC_PI_4)),
        )
        .unwrap();
        assert!(body_reach(&long) > 1.2e3 - 1e-9, "{}", body_reach(&long));
        assert!(
            refusal(&long).is_some(),
            "corners at 1.2 km are past the box"
        );
    }
}
