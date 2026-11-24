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
//! ([`in_session_box_at`]) all ask it, the gate and tier 3′ before
//! their checks: each refusal is reported alone, a face with no cover
//! with the cover's own refusal.
//!
//! **What is decided before the box admits.** Stage 1 decides nothing
//! but its own margin. Stage 3 admits nothing: it chooses which refusal.
//! Stage 2 admits on each face's own cover, whose proven constraints rest
//! on band decisions taken on the face's own data: lengths read as their
//! own margins (a vertex's distance from a cone's apex, a corner's
//! station along a cone's axis, a torus's radii), the chart walk's rows
//! levered at their carrier, and a sphere's seam levered at its ball.
//! Stage 3's placements read lengths too (a candidate's distance off the
//! carrier, off each edge). None of them is levered at the session box's
//! extent, which only this module's own decisions read. Every datum they
//! read lies in the face's stage-1 region, so a verdict among them is
//! sound wherever that region's reach is within the run's rounding reach
//! (`Tolerance::rounding_reach`, at least the box). A face whose carrier
//! datum stands farther (a cone's apex past the rounding reach, which is
//! 5 km at ε = 1e-12) is read past the premise; that residue is
//! `work/oneround/the-session-boxs-undecided-margins-admit.md`'s.

use geom::Surface;
use geom::cover::{BoxAxes, Cover};
use geom_core::{Band, Decide, Lever, Margin, Point3, Sign};
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
#[derive(Clone, Debug)]
struct Reads<T> {
    /// The largest `|coordinate|` of points known to lie on the body:
    /// its vertices and its certified edges' own points.
    points: T,
    /// The largest `|coordinate|` of its vertices and of each face's
    /// region bounded by its carrier and boundary alone; every face with
    /// no such cover where one has none.
    cover: Result<T, Vec<(FaceKey, crate::cover::FaceUncovered)>>,
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
/// affine; each certified edge's stored carrier through `map_carrier_stretched`,
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

/// **One edge's cover where it stands**: its carrier's cover over its
/// span hulled with its two ends as they stand (`cover::edge_cover_over`,
/// the image's `Body::edge_cover`).
fn edge_cover_where<T: Decide>(
    curve: &geom_brep::EdgeCurve<T>,
    ends: (Point3<T>, Point3<T>),
) -> Result<Cover<T>, EdgeUncovered> {
    crate::cover::edge_cover_over(curve.carrier(), curve.params(), ends)
}

/// The largest `|coordinate|` of a cover: of its box along the world
/// axes.
fn cover_reach<T: Decide>(cover: &Cover<T>) -> T {
    let origin = Point3::origin();
    let (lo, hi) = cover.box_in(&BoxAxes::World);
    (lo - origin).norm_inf().max((hi - origin).norm_inf())
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
    for view in body.edge_views() {
        let (e, data) = (view.key(), view.record());
        let Some(stored) = view.certified() else {
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
    let cover = faces_reach(body, vertices, |f| {
        let moved = surface_of(f.surface())?;
        let surface = moved.as_ref().unwrap_or(f.surface());
        Ok(crate::cover::face_cover_unproven(
            surface,
            crate::cover::outer_bounds(f),
            geom::cover::Apex::MayHold,
            || {
                crate::cover::boundary_cover_by(body, f, &place, |edge| {
                    covers
                        .get(edge.key())
                        .cloned()
                        .unwrap_or(Err(EdgeUncovered::Scaffold))
                })
            },
        ))
    })?;
    Ok(Reads { points, cover })
}

/// **The largest `|coordinate|` of every face's cover**, as `read`
/// reads each, from `from` up: the one fold over a body's faces. Where a
/// face has no cover, every such face, each with its cover's refusal.
fn faces_reach<T: Decide, E>(
    body: &Body<T>,
    from: T,
    mut read: impl FnMut(crate::live::FaceView<'_, T>) -> Result<crate::cover::FaceCovered<T>, E>,
) -> Result<Result<T, Vec<(FaceKey, crate::cover::FaceUncovered)>>, E> {
    let mut far = from;
    let mut uncovered = Vec::new();
    for (face, _) in body.faces() {
        match read(body.proven_face(face))? {
            Ok(cover) => far = far.max(cover_reach(&cover)),
            Err(why) => uncovered.push((face, why)),
        }
    }
    Ok(if uncovered.is_empty() {
        Ok(far)
    } else {
        Err(uncovered)
    })
}

/// The largest `|coordinate|` of `body`'s vertices as `place` sets them.
fn points_only<T: Decide>(body: &Body<T>, place: impl Fn(Point3<T>) -> Point3<T>) -> T {
    let origin = Point3::origin();
    body.vertices()
        .map(|(v, data)| (place(body.point_of(v, data)) - origin).norm_inf())
        .fold(T::zero(), T::max)
}

/// The session box's decision on a region's cover: its margin the box's
/// extent less the cover's largest `|coordinate|`.
pub(crate) const SESSION_BOX_REACH: &str = "session_box_reach";

/// The session box's decision on points known to lie on a body, asked
/// where its cover reaches past the box: its margin the box's extent
/// less their largest `|coordinate|`.
pub(crate) const SESSION_BOX_POINT: &str = "session_box_point";

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
pub(crate) fn in_session_box<T: Decide>(
    body: &Body<T>,
    band: Band,
) -> Result<(), Vec<ValidationError>> {
    let reads = unmoved_reads(body, edge_cover_where);
    if within(&reads.cover, band) {
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
    if within(&reads.cover, band) {
        return Ok(());
    }
    let image = crate::transform::map_isometry(body, &crate::Isometry::Proper(*motion))
        .map_err(crate::MoveError::Map)?;
    past_its_carriers(&image, reads.points, band)
        .map_err(|findings| crate::MoveError::NotFinished(crate::NotFinished::new(findings)))
}

/// Whether a cover's reach decides its region in the box: anything but a
/// definitely negative margin, and no face left uncovered.
fn within<T: Decide>(
    cover: &Result<T, Vec<(FaceKey, crate::cover::FaceUncovered)>>,
    band: Band,
) -> bool {
    cover.as_ref().is_ok_and(|&reach| {
        past(crate::validate::decide_reported(
            SESSION_BOX_REACH,
            box_margin(reach),
            band,
        ))
        .is_none()
    })
}

/// The box's margin on a reach: its extent less the reach.
fn box_margin<T: Decide>(reach: T) -> Margin<T> {
    Margin::of(Lever::<T>::session_box().length() - reach)
}

/// The margin of a box decision that is definitely past the box.
fn past(
    decided: Result<geom_core::Decided, geom_core::Indeterminate>,
) -> Option<geom_core::MarginDiag> {
    match decided {
        Ok(geom_core::Decided {
            sign: Sign::Negative,
            margin,
        }) => Some(margin),
        _ => None,
    }
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
) -> Result<(), Vec<ValidationError>> {
    let cover = faces_reach::<T, core::convert::Infallible>(body, points_only(body, |p| p), |f| {
        Ok(crate::cover::face_cover(body, f.key(), band))
    })
    .unwrap_or_else(|never| match never {});
    let unshown = match cover {
        Ok(reach) => match past(crate::validate::decide_reported(
            SESSION_BOX_REACH,
            box_margin(reach),
            band,
        )) {
            None => return Ok(()),
            Some(margin) => vec![ValidationError::SessionBoxUnshown { margin }],
        },
        Err(faces) => faces
            .into_iter()
            .map(|(face, why)| ValidationError::SessionBoxUncovered {
                face,
                cause: crate::validate::UncoveredCause::of(body, face, &why),
                why,
            })
            .collect(),
    };
    let origin = Point3::origin();
    let inside = body
        .faces()
        .flat_map(|(face, _)| crate::cover::face_extreme_points(body, face, band))
        .map(|q| (q - origin).norm_inf())
        .fold(points, T::max);
    // A point that is no finite number is no point past the box: moving
    // the geometry is no way through it, so the faces it leaves with no
    // cover say why instead.
    if !inside.is_bounded() {
        return Err(unshown);
    }
    match past(crate::validate::decide_reported(
        SESSION_BOX_POINT,
        box_margin(inside),
        band,
    )) {
        Some(margin) => Err(vec![ValidationError::OutsideSessionBox { margin }]),
        None => Err(unshown),
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
                    edge_cover_where(c, ends)
                        .ok()
                        .map(|c| c.box_in(&BoxAxes::World)),
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

    /// **A seamless frustum band does not hold its apex, wherever the
    /// apex stands**: a cone face bounded by its two full rims, each one
    /// circle winding the axis, with no seam edge (as a file states a
    /// frustum wall). A wall of radius 1 m tapered 1 mm over 10 m, at the
    /// origin, has its apex 10 km off; a band of the unit cone, its apex
    /// 0.2 m and 0.4 m past the box, every rim point inside it. Both are in
    /// the box, at the gate's ask and through a motion. *Breaks if* a face
    /// with two winding loops on one nappe is read as holding its apex
    /// (then the slender wall refuses as a limit 9 km past the box).
    #[test]
    fn a_seamless_frustum_band_does_not_hold_its_apex() {
        use crate::test_support_fixtures::{seamless_cone_band, seamless_cone_band_on};
        let tol = Tol::witness();
        let band = geom_core::Band::linear(tol).unwrap();
        let (slender, _) = seamless_cone_band_on((-1.0e4, 1.0e-4), 0.0, 10.0, tol);
        assert_eq!(
            in_session_box(&slender, band),
            Ok(()),
            "the slender wall at the origin"
        );
        let still =
            geom_core::RigidMotion::translation(geom_core::Vec3::new(0.0, 0.0, 0.0)).unwrap();
        assert!(
            in_session_box_at(&slender, &still, band).is_ok(),
            "placed where it stands"
        );
        let (unit, _) = seamless_cone_band(0.5, 1.5, tol);
        for dz in [-1000.2, -1000.4] {
            let at =
                geom_core::RigidMotion::translation(geom_core::Vec3::new(0.0, 0.0, dz)).unwrap();
            let image =
                crate::transform::map_isometry(&unit, &crate::Isometry::Proper(at)).unwrap();
            assert_eq!(
                in_session_box(&image, band),
                Ok(()),
                "the unit band, apex at {dz}"
            );
            assert!(
                in_session_box_at(&unit, &at, band).is_ok(),
                "placed, apex at {dz}"
            );
        }
    }

    /// **A cone cap whose apex alone is past the box is refused past it**:
    /// the cap of the cone of half-angle 0.5 bounded by its rim at height
    /// 1, which holds its apex with no vertex there ([`cone_cap`]), moved
    /// so the apex stands 0.5 m and 1 m past the box and the rim inside
    /// it. Refused past the box by exactly that, at the gate's ask and
    /// at the placing door; with the apex 0.5 m inside, it is in the box.
    /// *Breaks if* the cone's unproven region drops its apex (stage 1 then
    /// admits the cap), or the inside read drops the apex candidate (then
    /// it refuses as a limit).
    ///
    /// [`cone_cap`]: crate::test_support_fixtures::cone_cap
    #[test]
    fn a_cone_cap_whose_apex_alone_is_past_the_box_is_refused_past_it() {
        let tol = Tol::witness();
        let band = geom_core::Band::linear(tol).unwrap();
        let (cap, _) = crate::test_support_fixtures::cone_cap(0.5, 1.0, tol);
        for (dz, past) in [(-1000.5, Some(0.5)), (-1001.0, Some(1.0)), (-999.5, None)] {
            let at =
                geom_core::RigidMotion::translation(geom_core::Vec3::new(0.0, 0.0, dz)).unwrap();
            let image = crate::transform::map_isometry(&cap, &crate::Isometry::Proper(at)).unwrap();
            let gate = in_session_box(&image, band);
            let door = in_session_box_at(&cap, &at, band).err().map(|e| match e {
                crate::MoveError::NotFinished(found) => found.into_findings(),
                other => panic!("the door maps the cap: {other:?}"),
            });
            match past {
                Some(past) => {
                    for (what, got) in [
                        ("gate", gate.err().and_then(|f| f.into_iter().next())),
                        ("door", door.and_then(|f| f.into_iter().next())),
                    ] {
                        let Some(ValidationError::OutsideSessionBox { margin }) = got else {
                            panic!("{what}: the apex is {past} m past: {got:?}");
                        };
                        let said = margin.readable_below_zero().to_string();
                        assert!(
                            said.parse::<f64>().is_ok_and(|m| (m - past).abs() < 1e-9),
                            "{what}: {said}"
                        );
                    }
                }
                None => assert!(gate.is_ok() && door.is_none(), "the apex 0.5 m inside"),
            }
        }
    }

    /// **A face with no cover refuses at the gate, alone, every such face
    /// named, each in words true of it**, before any of tier 3's checks
    /// runs, at tier 3 and at tier 3′. One body per cause that reaches
    /// the gate:
    ///
    /// - a cube whose six faces are the seed's placeholder NURBS: six
    ///   findings, each "no surface described yet", ending in the
    ///   placeholder's recourse;
    /// - a brick with one face's surface a described bilinear net holding
    ///   a NaN: "its description holds invalid numbers", the defect
    ///   ending;
    /// - a sphere cap whose radius is NaN: "stores a radius that is not
    ///   a finite number", the defect ending;
    /// - a brick with one face's surface a cylinder whose axis is NaN
    ///   (a frame squared from a zero axis): "stores an axis that is not
    ///   a direction", the defect ending. Only geom's ruled rule refuses
    ///   this face (`Surface::face_region`'s axis guard): without it the
    ///   face reads its boundary's hull, in the box, and check 1 names it
    ///   instead;
    /// - a brick with one vertex at NaN: its three faces, each "an end
    ///   vertex of the edge is not a finite point, on the boundary of a
    ///   face", the defect ending.
    /// - the same brick with the vertex at `+∞`: the same three findings,
    ///   not "past the box": a point that is no finite number is no
    ///   point past it, and moving the geometry is no way through;
    /// - a brick with one face's surface a described net holding an
    ///   infinite control point: "stores a control point that is not a
    ///   finite number", the defect ending.
    ///
    /// A lone-vertex loop whose point is NaN never reaches the box: tier 2
    /// refuses it as scaffolding first, as it does a face no loop bounds
    /// and a null edge. *Breaks if* the box's refusal is deferred behind
    /// the checks or reported beside them, names fewer faces than have no
    /// cover, or says another cause's words.
    #[test]
    fn a_face_with_no_cover_refuses_at_the_gate_alone_by_its_cause() {
        let tol = Tol::witness();
        let swap = |surface: Surface<f64>| {
            let mut body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
            let face = body.faces().next().unwrap().0;
            let key = body.get_face(face).unwrap().surface;
            *body.surfaces.get_mut(key).unwrap() = surface;
            body
        };
        let net = crate::test_support_fixtures::declined_cube::<f64>(tol).body;
        let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let mut control: Vec<Point3<f64>> = (0..4)
            .map(|i| Point3::new(f64::from(i % 2), f64::from(i / 2), 0.0))
            .collect();
        control[1].x = f64::NAN;
        let described = swap(Surface::Nurbs(std::sync::Arc::new(
            geom::NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 4]).unwrap(),
        )));
        let (mut datum, cap) = crate::test_support_fixtures::sphere_cap(0.5, tol);
        let key = datum.get_face(cap).unwrap().surface;
        let Some(Surface::Sphere { radius, .. }) = datum.surfaces.get_mut(key) else {
            panic!("the cap's carrier is a sphere");
        };
        *radius = f64::NAN;
        let axis = swap(Surface::Cylinder {
            frame: geom::Frame::new(
                Point3::origin(),
                geom_core::Vec3::zero(),
                geom_core::Vec3::unit_x(),
            ),
            radius: 1.0,
        });
        let mut edge = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let corner = edge.vertices().next().unwrap().1.point;
        *edge.points.get_mut(corner).unwrap() = Point3::new(f64::NAN, 0.0, 0.0);
        let mut far = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let corner = far.vertices().next().unwrap().1.point;
        *far.points.get_mut(corner).unwrap() = Point3::new(f64::INFINITY, 0.0, 0.0);
        let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let mut control: Vec<Point3<f64>> = (0..4)
            .map(|i| Point3::new(f64::from(i % 2), f64::from(i / 2), 0.0))
            .collect();
        control[2].y = f64::INFINITY;
        let infinite = swap(Surface::Nurbs(std::sync::Arc::new(
            geom::NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 4]).unwrap(),
        )));
        let defect = geom_core::KERNEL_OR_FILE_DEFECT_ENDING;
        let records = Default::default();
        for (what, body, faces, says, ends) in [
            (
                "the placeholder cube",
                &net,
                6,
                "a face has no surface described yet, only the placeholder",
                geom_brep::recourse::PLACEHOLDER_RECOURSE,
            ),
            (
                "the described net holding a NaN",
                &described,
                1,
                "a face's surface description holds invalid numbers",
                defect,
            ),
            (
                "the NaN-radius cap",
                &datum,
                1,
                "a spherical face's surface stores a radius that is not a finite number",
                defect,
            ),
            (
                "the NaN-axis cylinder",
                &axis,
                1,
                "a cylindrical face's surface stores an axis that is",
                defect,
            ),
            (
                "the infinitely cornered brick",
                &far,
                3,
                "an end vertex of the edge is not a finite point, on the boundary of a face",
                defect,
            ),
            (
                "the described net holding an infinity",
                &infinite,
                1,
                "a face's surface description stores a control point that is not a finite number",
                defect,
            ),
            (
                "the NaN-cornered brick",
                &edge,
                3,
                "an end vertex of the edge is not a finite point, on the boundary of a face",
                defect,
            ),
        ] {
            for (tier, errors) in [
                ("tier 3", crate::validate::validate_geometric(body, tol)),
                (
                    "tier 3′",
                    crate::validate::validate_pseudomanifold(body, &records, tol),
                ),
            ] {
                let errors = errors.expect_err(what);
                assert!(
                    errors.len() == faces
                        && errors
                            .iter()
                            .all(|e| matches!(e, ValidationError::SessionBoxUncovered { .. })),
                    "{what}, {tier}: the box's refusal of {faces} faces, alone: {errors:?}"
                );
                for error in &errors {
                    let said = error.to_string();
                    assert!(
                        said.starts_with(says)
                            && said.contains("session box")
                            && said.ends_with(ends),
                        "{what}, {tier}: {said}"
                    );
                }
            }
        }
        let (mut ring, _, lone) = crate::test_support_fixtures::box_with_a_lone_ring(tol);
        let at = ring.get_vertex(lone).unwrap().point;
        *ring.points.get_mut(at).unwrap() = Point3::new(f64::NAN, 0.5, 1.0);
        let errors = crate::validate::validate_geometric(&ring, tol).unwrap_err();
        assert!(
            matches!(&errors[..], [ValidationError::ScaffoldingEmptyLoop { .. }]),
            "tier 2 refuses the lone ring before the box: {errors:?}"
        );
    }
}
