//! **The side at infinity of a selection of faces**: which side of its
//! boundary lies outside every face, read off rays aimed at its faces
//! from beyond them, and off its certified volume only where no aim
//! reads ([`side_at_infinity`]). It is the one reader of that fact,
//! and every question that is that fact asks it:
//!
//! - a shell's role, outer or cavity ([`crate::props`]'s
//!   `shell_role`), which check 10 ([`crate::validate`]), the pieces
//!   sort ([`crate::pieces`]), the cavity read of `shell`, the census's
//!   role gate ([`crate::census`]) and `classify_shells` read;
//! - a solid's orientation, check 7 ([`crate::validate`]);
//! - the side of a point no schedule ray reaches, point containment's
//!   missed-ray side ([`crate::boolean::solid_contain`]).
//!
//! The rays it casts are point containment's
//! ([`crate::boolean::solid_contain`]'s crossing walk), logged under their
//! own rows ([`AIMED_ROWS`]) so the two readers' verdicts stay apart.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use crate::body::Body;
use crate::boolean::SolidContainment;
use crate::boolean::solid_contain::{
    FaceGeo, PointInSolidError, RayRows, SolidFaces, face_geo, face_plane, ray_crossings,
    selection_reach, side_before, uncrossable,
};
use crate::entity::{FaceKey, LoopBoundary};
use crate::ray_walk::RayFault;

/// An aimed ray's rows ([`side_at_infinity`]).
pub(crate) const AIMED_ROWS: RayRows = RayRows {
    advance: "side_at_infinity_advance",
    denom: "side_at_infinity_denom",
    clearance: "side_at_infinity_clearance",
    beside: "side_at_infinity_beside",
    order: "side_at_infinity_order",
};

/// **Two crossings of an aimed ray in band of each other along it** —
/// what a side-at-infinity reading that lies in band found
/// ([`side_at_infinity`]). Two faces that do not meet, whose crossings
/// of one ray lie in band, are a pair of cells at most that far apart, so
/// a ray reads in band only where a pair does.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AimedPair {
    /// The shell of the face the ray was aimed at.
    pub shell: crate::entity::ShellKey,
    /// The face the ray was aimed at.
    pub aimed: FaceKey,
    /// The face of the ray's closest crossing.
    pub near: FaceKey,
    /// The face of the crossing in band of it.
    pub far: FaceKey,
    /// Their gap along the ray (`bool_point_in_solid_order`), in band: a
    /// magnitude.
    pub gap: Indeterminate,
}

/// What [`side_at_infinity`] could not read.
#[derive(Clone, Debug)]
pub(crate) enum SideUnread<V> {
    /// An aimed ray read in band, and no other decided: the first such
    /// ray's pair on two faces that do not meet, or, where the fallback
    /// did not read either, the first sharp corner's.
    InBand(AimedPair),
    /// No aimed ray read the side, and the fallback refused with
    /// `volume`. `limit` is the first limit an aimed ray met: a face it
    /// could not read (a spline face), or a broken one.
    Fallback {
        /// The fallback's refusal.
        volume: V,
        /// The first limit an aimed ray met, where one did.
        limit: Option<PointInSolidError>,
    },
}

/// An aim: a point on a face and the face's outward normal there.
type Aim<T> = (Point3<T>, Vec3<T>);

/// **Where a ray aimed at `face` aims**, and the face's outward normal
/// there. A planar face aims at a point certified strictly inside it
/// ([`crate::stands::planar_interior_point`]), and offers none where no
/// candidate certifies. A cylinder, cone or torus face aims at its
/// chart's point at the middle of its window (a full turn's middle where
/// it wraps), and a closed sphere at a fixed point of its chart; neither
/// is certified inside the face, nor needs to be: the ray's start lies
/// beyond every face whatever it aims at, so its closest crossing reads
/// the side wherever it lands, and an aim off the face only steers it.
/// A trimmed sphere, a cone this door cannot trim and a spline offer
/// none.
///
/// # Errors
///
/// A planar face that does not walk, or whose plane or in-face walk
/// refuses other than in band; a face that does not resolve.
fn aim_at<T: Decide>(
    body: &Body<T>,
    sel: &SolidFaces,
    face: FaceKey,
    band: Band,
) -> Result<Option<Aim<T>>, PointInSolidError> {
    match face_plane(body, face) {
        Ok((_, normal)) => {
            return Ok(
                crate::stands::planar_interior_point(body, face, band)?.map(|aim| (aim, normal))
            );
        }
        Err(PointInSolidError::KindUnsupported { .. }) => {}
        Err(e) => return Err(e),
    }
    let mid = |(lo, hi): (T, T)| (lo + hi) * T::from_f64(0.5);
    let (u, v) = match face_geo(body, face, &sel.charts, band) {
        Ok(FaceGeo::Cylinder { az, h, .. }) => (mid(az), mid(h)),
        Ok(FaceGeo::Cone { az, v, .. }) => (az.map_or_else(T::pi, mid), mid(v)),
        Ok(FaceGeo::Torus { u, v, .. }) => (u.map_or_else(T::pi, mid), v.map_or_else(T::pi, mid)),
        Ok(FaceGeo::Sphere { .. }) => (T::from_f64(1.0), T::from_f64(0.3)),
        Ok(_) => return Ok(None),
        Err(PointInSolidError::CorruptFace { face }) => {
            return Err(PointInSolidError::CorruptFace { face });
        }
        Err(_) => return Ok(None),
    };
    let aim = body
        .get_face(face)
        .and_then(|f| body.get_surface(f.surface))
        .ok_or(PointInSolidError::CorruptFace { face })?
        .eval(u, v);
    // A normal the face's own reader cannot give (an in-band residual,
    // an apex) leaves no incidence to aim at: the face offers none.
    Ok(
        crate::face_normal::face_outward_normal_at(body, face, aim, band)
            .ok()
            .flatten()
            .map(|normal| (aim, normal.vec())),
    )
}

/// Whether faces `a` and `b` meet: some vertex of one's loops is a vertex
/// of the other's.
fn meet<T: geom_core::Real>(body: &Body<T>, a: FaceKey, b: FaceKey) -> bool {
    let vertices = |face: FaceKey| -> Vec<crate::entity::VertexKey> {
        let Some(f) = body.get_face(face) else {
            return Vec::new();
        };
        core::iter::once(f.outer)
            .chain(f.rings.iter().copied())
            .filter_map(|l| match body.get_loop(l)?.boundary {
                LoopBoundary::Cycle { first } => Some(body.loop_walk(first).closed("loop", first)),
                LoopBoundary::Empty { .. } => None,
            })
            .flatten()
            .filter_map(|h| body.get_half_edge(h).map(|he| he.start))
            .collect()
    };
    let of_a = vertices(a);
    vertices(b).iter().any(|v| of_a.contains(v))
}

/// The incidence an aimed ray meets its face at: `cos 60°` along the
/// face's outward normal and `sin 60°` along the face.
const AIM_INCIDENCE: (f64, f64) = (0.5, 0.866_025_403_784_438_6);

/// The turn about the face's normal its aimed ray's tangent component
/// makes off the normal's frame (`Vec3::orthonormal_basis`), as
/// `(sin, cos)` of the golden angle: an axis frame's tangent would lay
/// the ray in a coordinate plane, parallel to every face of a box
/// across it, where a turn the model has no reason to share lays it
/// parallel to none.
const AIM_TURN: (f64, f64) = (0.675_490_294_261_523_8, -0.737_368_878_078_319_7);

/// **The side at infinity of `sel`'s faces** — the one reader of a
/// shell's role (D10, Booleans: `Out` is an outer boundary, `In` a
/// cavity's) and of a solid's orientation (D1 tier 3, check 7: `In` is
/// inside out).
///
/// **Aimed rays first.** Each face of the selection that offers an aim
/// ([`aim_at`]: a planar face's certified interior point, a curved
/// face's chart window's middle) is aimed at, in face order: the ray
/// leaves from twice the selection's reach from that point, beyond every
/// face, so its start lies on the side at infinity, and meets the face at
/// 60° from its outward normal ([`AIM_INCIDENCE`]), turned about it off
/// the coordinate planes ([`AIM_TURN`]). Its closest crossing's outward
/// sign is the side, as a schedule ray's is (point containment's), its decisions
/// logged under the aimed rows ([`AIMED_ROWS`]). The first ray that
/// decides answers.
///
/// The incidence keeps a ray's reading in band only where a pair is:
/// the ray cannot graze the face it aims at (`d·n = −½`), and between
/// two faces `t` apart its two crossings lie at least `t` apart, `2t`
/// across two parallel ones, so a pair the census reads definite at
/// `Kε` leaves the ray `2Kε`, a full `Kε` of slack against the rounding
/// of advances measured from a start a few reaches out. A ray that
/// grazes another face, whose hit lands in band of a face's boundary, or
/// whose two closest crossings lie in band on faces that meet (a sharp
/// corner, whose gap closes at the cell the faces share, and no pair) is
/// set aside, and the next face is aimed at.
///
/// **The volume only where no face offers an aim that reads.**
/// Where no aimed ray decided and none read in band, `volume` reads
/// the side: a selection all of whose faces are trimmed spheres, untrimmable
/// cones or splines, or whose rays all grazed, met a limit or crossed a
/// sharp corner in band. A selection holding a face the ray test has no
/// arm for ([`crate::boolean::solid_contain::ray_crosses`]: a spline) casts no ray at all: every ray
/// would fault on that face, whatever it aimed at, so the volume reads
/// with it as the limit.
/// An aimed ray read in band on two faces that do not meet refuses on
/// its pair ([`SideUnread::InBand`]) without the volume: a pair in band
/// is the answer, and the volume's margin, a mean half-thickness, would
/// only judge the selection thin. Where the volume does not read either,
/// a sharp corner's pair in band is the refusal: the selection is thin
/// wherever an aimed ray crossed it.
///
/// # Errors
///
/// A refusal about the body: a face that does not walk, or whose plane
/// or in-face walk refuses other than in band.
pub(crate) fn side_at_infinity<T: Decide, V>(
    body: &Body<T>,
    sel: &SolidFaces,
    band: Band,
    volume: impl FnOnce() -> Result<SolidContainment, V>,
) -> Result<Result<SolidContainment, SideUnread<V>>, PointInSolidError> {
    let faces = sel.faces();
    // A face the ray test has no arm for faults EVERY ray at
    // `face_geo`, whatever it aims at: no aim can read, so none is cast,
    // and the volume reads with that face as the limit.
    if let Some(e) = faces
        .iter()
        .find_map(|&face| uncrossable(body, face).transpose())
    {
        let e = e?;
        return Ok(volume().map_err(|volume| SideUnread::Fallback {
            volume,
            limit: Some(e),
        }));
    }
    let (mut in_band, mut at_a_corner, mut limit) = (None, None, None);
    let (along, across) = AIM_INCIDENCE;
    for &face in faces {
        let Some((aim, normal)) = aim_at(body, sel, face, band)? else {
            continue;
        };
        let shell = body
            .get_face(face)
            .ok_or(PointInSolidError::CorruptFace { face })?
            .shell;
        let (b1, b2) = normal.orthonormal_basis();
        let (turn_sin, turn_cos) = AIM_TURN;
        let tangent = b1 * T::from_f64(turn_cos) + b2 * T::from_f64(turn_sin);
        let out = normal * T::from_f64(along) + tangent * T::from_f64(across);
        let from = aim + out * (selection_reach(body, faces, aim)? * T::from_f64(2.0));
        let d = (aim - from).normalize();
        let reach = selection_reach(body, faces, from)?;
        let crossings = match ray_crossings(body, sel, from, d, reach, band, AIMED_ROWS) {
            Ok(crossings) => crossings,
            Err(RayFault::Blocked(e) | RayFault::Fatal(e)) => {
                limit.get_or_insert(e);
                continue;
            }
            Err(RayFault::InBand(_) | RayFault::Graze | RayFault::Unread) => continue,
        };
        // The pair's gap is read as a magnitude, the distance between the
        // two crossings whichever the scan met first.
        let pair = |signed, &(_, far, at_far): &(Sign, FaceKey, T), &(_, near, at_near): &_| {
            let gap = geom_core::k_stats::decide_magnitude(
                AIMED_ROWS.order,
                Margin::of((at_far - at_near).abs()),
                band,
            )
            .err()
            .unwrap_or(signed);
            AimedPair {
                shell,
                aimed: face,
                near,
                far,
                gap,
            }
        };
        match crossings.closest(AIMED_ROWS.order, band, pair) {
            Ok(Some((outward, _, _))) => return Ok(Ok(side_before(outward))),
            // Two faces that meet at a vertex are no pair: their gap closes
            // at the cell they share, and a ray crossing a sharp corner
            // near it reads in band however definite every pair is.
            Err(RayFault::InBand(pair)) if meet(body, pair.near, pair.far) => {
                at_a_corner.get_or_insert(pair);
            }
            Err(RayFault::InBand(pair)) => {
                in_band.get_or_insert(pair);
            }
            Ok(None) | Err(_) => {}
        }
    }
    if let Some(pair) = in_band {
        return Ok(Err(SideUnread::InBand(pair)));
    }
    Ok(volume().map_err(|volume| match at_a_corner {
        Some(pair) => SideUnread::InBand(pair),
        None => SideUnread::Fallback { volume, limit },
    }))
}
#[cfg(test)]
mod pre_filter_tests {
    #![allow(clippy::expect_used, clippy::panic)]
    use super::*;
    use crate::test_support_fixtures::brick;
    use geom::Surface;
    use geom_core::Tol;
    use geom_core::k_stats::Bracket;

    /// A unit brick, and its faces.
    fn brick_and_faces(tol: Tol) -> (Body<f64>, Vec<FaceKey>) {
        let body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let faces = body.faces().map(|(k, _)| k).collect();
        (body, faces)
    }

    /// The predicates an aimed ray logs.
    fn aimed(log: &geom_core::k_stats::Recorded) -> Vec<&'static str> {
        log.verdicts
            .iter()
            .map(|v| v.predicate)
            .filter(|p| p.starts_with("side_at_infinity"))
            .collect()
    }

    /// **A selection holding a spline face casts no aimed ray**, and
    /// reads the volume with that face's refusal as its limit — the
    /// refusal every ray would have met at it, whatever it aimed at.
    #[test]
    fn a_spline_face_casts_no_ray_and_limits_the_volume_as_a_ray_would() {
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the witness band");
        let (mut body, faces) = brick_and_faces(tol);
        let spline = *faces.last().expect("a brick has faces");
        let key = body.get_face(spline).expect("the face resolves").surface;
        *body.surfaces.get_mut(key).expect("its surface resolves") = Surface::nurbs_placeholder();
        let sel = SolidFaces::select(&body, faces).expect("the faces select");

        let bracket = Bracket::open();
        let read = side_at_infinity(&body, &sel, band, || {
            Err::<SolidContainment, _>("the volume")
        });
        let log = bracket.finish();

        let want = PointInSolidError::KindUnsupported {
            face: spline,
            kind: geom::SurfaceKind::Nurbs,
        };
        assert!(
            matches!(
                &read,
                Ok(Err(SideUnread::Fallback {
                    volume: "the volume",
                    limit: Some(e),
                })) if *e == want
            ),
            "the volume reads, limited by the spline face"
        );
        assert_eq!(aimed(&log), Vec::<&str>::new(), "no aimed ray is cast");
        // The limit is the one a ray meets at that face, from anywhere.
        let ray = ray_crossings(
            &body,
            &sel,
            Point3::new(5.0, 4.0, 3.0),
            Vec3::new(-1.0, -0.8, -0.6).normalize(),
            10.0,
            band,
            AIMED_ROWS,
        );
        assert!(
            matches!(&ray, Err(RayFault::Fatal(e)) if *e == want),
            "a ray meets the same refusal"
        );
    }

    /// The brick with a point of `faces[0]` removed: that planar face's
    /// in-face walk refuses (`Desync`), and its plane still reads.
    fn with_unwalkable_first_face(body: &mut Body<f64>, faces: &[FaceKey]) {
        let point = body
            .half_edges
            .values()
            .find(|he| {
                body.loops
                    .get(he.parent_loop)
                    .is_some_and(|l| l.face == faces[0])
            })
            .and_then(|he| body.vertices.get(he.start))
            .map(|v| v.point)
            .expect("the first face has a vertex");
        body.points.remove(point);
    }

    /// **The walk refusal a spline face now answers instead of.** On a
    /// selection whose first planar face's in-face walk refuses, the
    /// first aim used to return that refusal; with a spline face in the
    /// selection no aim is taken, so the volume reads, limited by the
    /// spline face. Without the spline face the walk's refusal stands.
    #[test]
    fn a_spline_face_reads_the_volume_where_a_planar_aims_walk_refuses() {
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the witness band");
        let (mut body, faces) = brick_and_faces(tol);
        with_unwalkable_first_face(&mut body, &faces);
        let sel = SolidFaces::select(&body, faces.clone()).expect("the faces select");
        let read = side_at_infinity(&body, &sel, band, || {
            Err::<SolidContainment, _>("the volume")
        });
        assert!(
            read.is_err(),
            "without a spline face the first aim's walk refusal is the answer"
        );

        let spline = *faces.last().expect("a brick has faces");
        let key = body.get_face(spline).expect("the face resolves").surface;
        *body.surfaces.get_mut(key).expect("its surface resolves") = Surface::nurbs_placeholder();
        let sel = SolidFaces::select(&body, faces).expect("the faces select");
        let read = side_at_infinity(&body, &sel, band, || {
            Err::<SolidContainment, _>("the volume")
        });
        assert!(
            matches!(
                &read,
                Ok(Err(SideUnread::Fallback {
                    volume: "the volume",
                    limit: Some(PointInSolidError::KindUnsupported { face, .. }),
                })) if *face == spline
            ),
            "with one, the volume reads, limited by the spline face"
        );
    }

    /// **A selection with no spline face still casts its rays**, and the
    /// volume is not read where one decides.
    #[test]
    fn a_selection_with_no_spline_face_still_aims() {
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the witness band");
        let (body, faces) = brick_and_faces(tol);
        let sel = SolidFaces::select(&body, faces).expect("the faces select");

        let bracket = Bracket::open();
        let read = side_at_infinity(&body, &sel, band, || -> Result<SolidContainment, ()> {
            panic!("the volume is read only where no aim decides")
        });
        let log = bracket.finish();

        assert!(
            matches!(read, Ok(Ok(SolidContainment::Out))),
            "an aim reads the side"
        );
        assert!(!aimed(&log).is_empty(), "an aimed ray is cast");
    }
}
