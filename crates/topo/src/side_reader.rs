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
use crate::entity::{EntityId, FaceKey, LoopBoundary};
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
#[derive(Clone, Debug, PartialEq)]
pub struct AimedPair {
    /// The shell of the face the ray was aimed at.
    pub shell: crate::entity::ShellKey,
    /// The face the ray was aimed at.
    pub aimed: FaceKey,
    /// The point on it the ray was aimed at, as coordinates.
    pub aim: String,
    /// The face of the ray's closest crossing.
    pub near: FaceKey,
    /// The face of the crossing in band of it.
    pub far: FaceKey,
    /// Their gap along the ray (`bool_point_in_solid_order`), in band: a
    /// magnitude.
    pub gap: Indeterminate,
}

/// **An aimed ray the reader set aside**, and why
/// ([`side_at_infinity`]): the face it was aimed at, where it aimed, and
/// what kept it from reading the side.
#[derive(Clone, Debug, PartialEq)]
pub struct SetAside {
    /// The face the ray was aimed at.
    pub aimed: FaceKey,
    /// The point on it the ray was aimed at, as coordinates; `None`
    /// where no ray was cast at it (a face the ray test has no arm for).
    pub aim: Option<String>,
    /// Why it read nothing.
    pub why: SetAsideWhy,
}

/// Why an aimed ray read nothing ([`SetAside`]).
///
/// **NOT carried to the façade's curated list, and that is a
/// decision rather than an omission** (`scripts/payload-rung-sweep.py`
/// names this rung; the disposition table cites this paragraph as its
/// home). A Rust caller can already name and match it — `pncad`
/// re-exports `topo` whole — and the Python door says which refusal it
/// rides in (`orientation_undecided`, a shell's `corner`) and carries
/// its sentence, which names each ray set aside and why. Publishing it
/// owes the CUR3 property row
/// `carried_refusal_payloads_are_matchable_through_the_prelude` and a
/// Python word per arm, both the façade crate's to write.
///
/// **The falsifier is a Python caller who must tell a sharp corner from
/// a crossing in band of a face's boundary** without reading the
/// sentence. When the façade carries the reading, delete this paragraph
/// and the disposition row with it.
#[derive(Clone, Debug, PartialEq)]
pub enum SetAsideWhy {
    /// Its two closest crossings lie in band of each other on faces that
    /// meet: a sharp corner, whose gap closes at the cell the faces
    /// share, and no pair.
    Corner {
        /// The face of its closest crossing.
        near: FaceKey,
        /// The face of the crossing in band of it.
        far: FaceKey,
        /// Their gap along the ray, in band.
        gap: Indeterminate,
    },
    /// A crossing it could not place: in band of a face's boundary, or
    /// of a face it met tangentially.
    Crossing(PointInSolidError),
    /// It grazed a face, or crossed nothing it could read.
    Graze,
    /// A face it could not read, or a broken one: a limit, no tolerance
    /// moves.
    Limit(PointInSolidError),
}

/// What [`side_at_infinity`] could not read.
#[derive(Clone, Debug)]
pub(crate) enum SideUnread<V> {
    /// An aimed ray read in band on two faces that do not meet, and no
    /// other decided: the first such ray's pair.
    InBand(AimedPair),
    /// No aimed ray read the side, none read a pair in band, and the
    /// fallback refused with `volume`.
    Fallback {
        /// The fallback's refusal.
        volume: V,
        /// How many faces offered an aim.
        aimed: usize,
        /// The rays set aside, in face order.
        set_aside: Vec<SetAside>,
    },
}

/// **A pair's words**: the aimed face and point, the two faces met and
/// their gap, the one text every refusal naming an [`AimedPair`] renders.
pub(crate) struct PairPayload<'a>(pub(crate) &'a AimedPair);

impl core::fmt::Display for PairPayload<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let p = self.0;
        write!(
            f,
            "a ray aimed at {} at {} met {} and {} in band of each other: {}",
            EntityId::Face(p.aimed),
            p.aim,
            EntityId::Face(p.near),
            EntityId::Face(p.far),
            p.gap.payload()
        )
    }
}

/// **What no aimed ray read**, in words: that no face offered an aim,
/// or each ray set aside and why ([`SetAside`]).
pub(crate) struct UnreadRays<'a> {
    /// How many faces offered an aim.
    pub(crate) aimed: usize,
    /// The rays set aside.
    pub(crate) set_aside: &'a [SetAside],
}

impl core::fmt::Display for UnreadRays<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.set_aside.is_empty() {
            return f.write_str(
                "no face of it offers a ray to aim (a point more than the tolerance inside a \
                 planar face, or a curved face's chart window)",
            );
        }
        if self.aimed == 0 {
            f.write_str("no ray was aimed: ")?;
        } else {
            write!(
                f,
                "{} of the {} rays aimed at its faces read nothing: ",
                self.set_aside.len(),
                self.aimed
            )?;
        }
        for (k, s) in self.set_aside.iter().enumerate() {
            if k > 0 {
                f.write_str("; ")?;
            }
            write!(f, "at {}", EntityId::Face(s.aimed))?;
            if let Some(aim) = &s.aim {
                write!(f, " (aimed at {aim})")?;
            }
            match &s.why {
                SetAsideWhy::Corner { near, far, gap } => write!(
                    f,
                    ", {} and {}, which meet, crossed in band of each other at a sharp corner: {}",
                    EntityId::Face(*near),
                    EntityId::Face(*far),
                    gap.payload()
                )?,
                SetAsideWhy::Crossing(PointInSolidError::Escalated { face, diag }) => write!(
                    f,
                    ", its crossing of {} in band of that face's boundary: {}",
                    EntityId::Face(*face),
                    diag.payload()
                )?,
                SetAsideWhy::Crossing(e) => write!(f, ", a crossing in band: {}", e.summary())?,
                SetAsideWhy::Graze => {
                    f.write_str(", a graze or a tie, or no crossing it could place")?;
                }
                SetAsideWhy::Limit(PointInSolidError::KindUnsupported { face, kind }) => write!(
                    f,
                    ", {} is a {} surface, which no ray can cross yet",
                    EntityId::Face(*face),
                    crate::boolean::kind_word(*kind)
                )?,
                SetAsideWhy::Limit(e) => write!(f, ", {}", e.summary())?,
            }
        }
        Ok(())
    }
}

impl<V> SideUnread<V> {
    /// The first limit an aimed ray met, where one did: a face it could
    /// not read (a spline face), or a broken one.
    pub(crate) fn limit(&self) -> Option<&PointInSolidError> {
        match self {
            Self::Fallback { set_aside, .. } => set_aside.iter().find_map(|s| match &s.why {
                SetAsideWhy::Limit(e) => Some(e),
                _ => None,
            }),
            Self::InBand(_) => None,
        }
    }
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

/// **How far from `aim` any face of the selection reaches**: the loops'
/// reach ([`selection_reach`]), which holds a planar face (its outer
/// loop's hull) and a wall (its rulings' segments), and for a sphere or a
/// torus, which bulge past their loops' hull, the ball about `aim` that
/// holds the whole surface (its centre's distance plus its radius, or its
/// major plus its minor radius). An aimed ray starts twice this far from
/// its aim, so its start lies beyond every face of the selection,
/// whatever it is aimed at.
///
/// # Errors
///
/// A face or surface that does not resolve.
fn beyond<T: Decide>(
    body: &Body<T>,
    faces: &[FaceKey],
    aim: Point3<T>,
) -> Result<T, PointInSolidError> {
    let mut reach = selection_reach(body, faces, aim)?;
    for &face in faces {
        let surface = body
            .get_face(face)
            .and_then(|f| body.get_surface(f.surface))
            .ok_or(PointInSolidError::CorruptFace { face })?;
        let whole = match *surface {
            geom::Surface::Sphere { center, radius, .. } => (aim - center).norm() + radius.abs(),
            geom::Surface::Torus {
                center,
                major_radius,
                minor_radius,
                ..
            } => (aim - center).norm() + major_radius.abs() + minor_radius.abs(),
            _ => continue,
        };
        reach = reach.max(whole);
    }
    Ok(reach)
}

/// **Whether faces `a` and `b` meet**, so that a ray's two crossings of
/// them in band of each other are a sharp corner and no pair: the gap
/// between them closes at the cell they share.
/// - A face meets itself: a curved face crossed twice in band (a ray
///   grazing near its silhouette) is no pair either.
/// - Two faces meet where they share an edge, or only a vertex (a
///   pyramid's opposite flanks at its apex): some half-edge of one starts
///   where a half-edge of the other does.
///
/// Both faces' loops are walked in place; nothing is collected.
fn meet<T: geom_core::Real>(body: &Body<T>, a: FaceKey, b: FaceKey) -> bool {
    if a == b {
        return true;
    }
    let starts = |face: FaceKey| {
        body.get_face(face)
            .into_iter()
            .flat_map(|f| core::iter::once(f.outer).chain(f.rings.iter().copied()))
            .filter_map(|l| match body.get_loop(l)?.boundary {
                LoopBoundary::Cycle { first } => Some(body.loop_walk(first).closed("loop", first)),
                LoopBoundary::Empty { .. } => None,
            })
            .flatten()
            .filter_map(|h| body.get_half_edge(h).map(|he| he.start))
    };
    starts(a).any(|v| starts(b).any(|w| w == v))
}

/// The incidence an aimed ray meets its face at: `cos 60°` along the
/// face's outward normal and `sin 60°` along the face.
const AIM_INCIDENCE: (f64, f64) = (0.5, 0.866_025_403_784_438_6);

/// The turn about the face's normal its aimed ray's tangent component
/// makes off the normal's frame (`Vec3::orthonormal_basis`), as
/// `(sin, cos)` of the golden angle ([`geom_core::GOLDEN_ANGLE`], the
/// one the ray schedules step by): an axis frame's tangent would lay the
/// ray in a coordinate plane, parallel to every face of a box across it,
/// where a turn the model has no reason to share lays it parallel to
/// none.
fn aim_turn() -> (f64, f64) {
    geom_core::GOLDEN_ANGLE.sin_cos()
}

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
/// the coordinate planes ([`aim_turn`]). Its closest crossing's outward
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
/// **The face-boundary band is the other half, and the slack does not
/// cover it.** A crossing within `Kε` of a face's loop boundary
/// (`point_in_loop_boundary`, or a curved face's trim, in band) cannot be
/// placed in or out of the face, so its ray is set aside however
/// definite every pair is. A planar face's aim is certified more than
/// `Kε` inside it, so the aimed face's own crossing clears its boundary;
/// the faces the ray crosses on the way are not chosen, and on a body
/// thin in two directions every crossing lies near an edge. Such a body
/// offers no ray that reads (`work/flush/the-side-reader-refuses-a-body-thin-in-two-directions-whose-pairs-are-definite.md`).
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
    for &face in faces {
        if let Some(e) = uncrossable(body, face)? {
            return Ok(volume().map_err(|volume| SideUnread::Fallback {
                volume,
                aimed: 0,
                set_aside: vec![SetAside {
                    aimed: face,
                    aim: None,
                    why: SetAsideWhy::Limit(e),
                }],
            }));
        }
    }
    let (mut in_band, mut aimed, mut set_aside) = (None, 0, Vec::new());
    let (along, across) = AIM_INCIDENCE;
    for &face in faces {
        let Some((aim, normal)) = aim_at(body, sel, face, band)? else {
            continue;
        };
        aimed += 1;
        let at = crate::census::witness(aim);
        let mut set = |why| {
            set_aside.push(SetAside {
                aimed: face,
                aim: Some(at.clone()),
                why,
            });
        };
        let shell = body
            .get_face(face)
            .ok_or(PointInSolidError::CorruptFace { face })?
            .shell;
        let (b1, b2) = normal.orthonormal_basis();
        let (turn_sin, turn_cos) = aim_turn();
        let tangent = b1 * T::from_f64(turn_cos) + b2 * T::from_f64(turn_sin);
        let out = normal * T::from_f64(along) + tangent * T::from_f64(across);
        let from = aim + out * (beyond(body, faces, aim)? * T::from_f64(2.0));
        let d = (aim - from).normalize();
        let reach = selection_reach(body, faces, from)?;
        let crossings = match ray_crossings(body, sel, from, d, reach, band, AIMED_ROWS) {
            Ok(crossings) => crossings,
            Err(RayFault::Blocked(e) | RayFault::Fatal(e)) => {
                set(SetAsideWhy::Limit(e));
                continue;
            }
            Err(RayFault::InBand(e)) => {
                set(SetAsideWhy::Crossing(e));
                continue;
            }
            Err(RayFault::Graze | RayFault::Unread) => {
                set(SetAsideWhy::Graze);
                continue;
            }
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
                aim: at.clone(),
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
                set(SetAsideWhy::Corner {
                    near: pair.near,
                    far: pair.far,
                    gap: pair.gap,
                });
            }
            Err(RayFault::InBand(pair)) => {
                in_band.get_or_insert(pair);
            }
            // `closest` reads no limit: a tie, a crossing on an edge, or
            // none at all.
            Ok(None) | Err(_) => set(SetAsideWhy::Graze),
        }
    }
    if let Some(pair) = in_band {
        return Ok(Err(SideUnread::InBand(pair)));
    }
    Ok(volume().map_err(|volume| SideUnread::Fallback {
        volume,
        aimed,
        set_aside,
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
                    aimed: 0,
                    set_aside,
                })) if set_aside == &[SetAside {
                    aimed: spline,
                    aim: None,
                    why: SetAsideWhy::Limit(want.clone()),
                }]
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
                Ok(Err(unread @ SideUnread::Fallback {
                    volume: "the volume",
                    ..
                })) if matches!(
                    unread.limit(),
                    Some(PointInSolidError::KindUnsupported { face, .. }) if *face == spline
                )
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
