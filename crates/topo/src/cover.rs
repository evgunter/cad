//! **The region a face or an edge can occupy** (DESIGN `[one-way.places]`,
//! "The region a carrier, an edge or a face can occupy", and
//! `[one-way.readings]`): topo's one reading of each, [`face_cover`] and
//! [`edge_cover`]. A face's cover is its carrier's (`geom`'s
//! `Surface::cover`) met with every trim constraint its one arm per kind
//! proves; an edge's is its carrier's over its span (`geom`'s
//! `Curve3::cover_over`) hulled with its ends. Those two readings in
//! `geom` hold the only kind matches; every bound a door reads (a box in
//! any frame, a support, a distance range, a ball) is a method on the
//! cover.
//!
//! Its home follows `[topo.homes]` (`crates/topo/README.md`): a reading
//! over a `Body`.

use geom::Surface;
use geom::cover::{Cover, ParamWindow, Uncovered};
use geom_core::spline::SpanLocate;
use geom_core::{Band, Decide, Point3, Real, UnitVec3, Vec3};

use crate::body::Body;
use crate::entity::{Edge, EdgeKey, EntityId, FaceKey, HalfEdgeKey, VertexKey};
use crate::live::{BoundaryMember, FaceView, linked, proven};

/// **Why a face has no cover** ([`face_cover`]): its carrier's own
/// refusal (a plane, a cylinder and a cone are unbounded whole; a net
/// can carry poison), and each boundary entity whose refusal left the
/// boundary unread where the arm reads it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FaceUncovered {
    carrier: Uncovered,
    boundary: Vec<BoundaryUnknown>,
}

/// **A boundary entity a face reading could not cover**.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoundaryUnknown {
    /// A boundary edge with no cover, and why.
    Edge(EdgeKey, EdgeUncovered),
    /// An isolated-vertex loop whose point is not a finite number.
    Vertex(VertexKey),
}

impl FaceUncovered {
    /// The carrier's own refusal.
    #[must_use]
    pub fn carrier(&self) -> &Uncovered {
        &self.carrier
    }

    /// Each boundary entity that left the boundary unread, in boundary
    /// order; empty where the arm reads no boundary or it read whole.
    #[must_use]
    pub fn boundary(&self) -> &[BoundaryUnknown] {
        &self.boundary
    }

    /// The refusal of a face whose carrier refused and whose arm read no
    /// boundary.
    pub(crate) fn of_carrier(carrier: Uncovered) -> Self {
        Self {
            carrier,
            boundary: Vec::new(),
        }
    }
}

impl core::fmt::Display for FaceUncovered {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "the face's carrier has {}", self.carrier)?;
        for unknown in &self.boundary {
            match unknown {
                BoundaryUnknown::Edge(ek, why) => write!(f, " and its edge {ek:?}: {why}")?,
                BoundaryUnknown::Vertex(v) => {
                    write!(f, " and its lone vertex {v:?} is not a finite point")?;
                }
            }
        }
        Ok(())
    }
}

impl std::error::Error for FaceUncovered {}

/// A face's cover, or why it has none.
pub type FaceCovered<T> = Result<Cover<T>, FaceUncovered>;

/// A boundary's cover, or every boundary entity that has none.
pub(crate) type BoundaryCovered<T> = Result<Cover<T>, Vec<BoundaryUnknown>>;

/// The carrier's cover met with a trim constraint the boundary bounds:
/// a refusal on either side is the meet's identity, and a refusal on
/// both keeps both.
fn and_trim<T: Real>(carrier: FaceCovered<T>, trim: BoundaryCovered<T>) -> FaceCovered<T> {
    match (carrier, trim) {
        (Ok(c), Ok(t)) => Ok(c.meet(t)),
        (Ok(c), Err(_)) => Ok(c),
        (Err(_), Ok(t)) => Ok(t),
        (Err(mut why), Err(unknowns)) => {
            why.boundary.extend(unknowns);
            Err(why)
        }
    }
}

/// The cover met with a constraint the reading proved, where it proved
/// one; an unproven one adds nothing.
fn and_proven<T: Real>(cover: FaceCovered<T>, constraint: Option<Cover<T>>) -> FaceCovered<T> {
    match (cover, constraint) {
        (Ok(c), Some(t)) => Ok(c.meet(t)),
        (Err(_), Some(t)) => Ok(t),
        (cover, None) => cover,
    }
}

/// **A constraint's read**: `Some` where it decides, `None` where it
/// does not. Its decisions are spliced onto the open log where the
/// constraint stands, and spliced superseded where it is absent, since
/// nothing the cover answers rests on them.
fn constraint<X, E>(read: impl FnOnce() -> Result<X, E>) -> Option<X> {
    let (read, log) = geom_core::k_stats::detached(read);
    match read {
        Ok(x) => {
            geom_core::k_stats::splice(log);
            Some(x)
        }
        Err(_) => {
            geom_core::k_stats::splice_superseded(log);
            None
        }
    }
}

/// K name: the point where a plane face's distance from a site is least
/// (a line's pierce, a point's foot) against the face's plane, the
/// point placement's on-plane read before it places the point
/// ([`face_nearest`]).
pub(crate) const NEAREST_FOOT: &str = "cover_face_nearest_foot";

/// **The nearest a face stands from `site`**: a lower bound on the
/// distance of any of its points, topo's reading of it.
///
/// The cover's ([`Cover::distance_range`]) holds on every face. On a
/// plane face the reading proves one more constraint: the distance from
/// a point or a line, restricted to a plane, is strictly convex with its
/// least at one point (the point's foot, the line's pierce), so where
/// the face places that point definitely `Out` the face's least is its
/// boundary's ([`boundary_cover`]), which a convex hull does not bound.
/// A placement that is not a definite `Out` adds nothing.
///
/// # Panics
///
/// As [`face_cover`]: `cover` is `face`'s.
pub fn face_nearest<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    cover: &Cover<T>,
    site: &geom::cover::Site<T>,
    band: Band,
) -> T {
    let near = cover.distance_range(site).0;
    let f = body.proven_face(face);
    let Surface::Plane { origin, normal, .. } = *f.surface() else {
        return near;
    };
    let least = match *site {
        geom::cover::Site::Point(q) => geom::Line {
            origin,
            dir: normal,
        }
        .square_foot(q),
        geom::cover::Site::Line(geom::Line { origin: o, dir }) => {
            o + dir * ((origin - o).dot(normal) / dir.dot(normal))
        }
    };
    let finite = [least.x, least.y, least.z]
        .into_iter()
        .all(geom_core::is_finite_length);
    let out = finite
        && constraint(|| {
            match crate::boolean::face_placement(body, face, least, band, NEAREST_FOOT) {
                Ok(crate::boolean::CurvedPlacement::Trim(Some(
                    crate::boolean::FaceContainment::Out,
                ))) => Ok(()),
                _ => Err(()),
            }
        })
        .is_some();
    match boundary_cover(body, f) {
        Ok(boundary) if out => near.max(boundary.distance_range(site).0),
        _ => near,
    }
}

/// **The ball where two faces' covers meet, within `tolerance`**: the
/// ball of their meet ([`geom::cover::Cover::ball`], which reads the
/// meet's box) widened by a geometric tolerance, the band a section
/// point of the two is read within: such a point lies within the band
/// of both faces, not on their covers' meet. A refusal is the meet's
/// identity, so one side with no cover leaves the other's ball; with
/// neither, the ball is poison, which every lever reading it escalates.
/// The one construction of where two regions meet: a face pair's
/// section reach, and an operation's region over its operands' covers.
pub(crate) fn meet_within<T: Real>(
    a: &FaceCovered<T>,
    b: &FaceCovered<T>,
    tolerance: f64,
) -> geom_brep::ExtentBall<T> {
    let within = |c: &Cover<T>| geom_brep::ExtentBall::of_cover_within(c, T::from_f64(tolerance));
    match (a, b) {
        (Ok(a), Ok(b)) => within(&a.clone().meet(b.clone())),
        (Ok(one), Err(_)) | (Err(_), Ok(one)) => within(one),
        (Err(_), Err(_)) => {
            let poison = geom_core::poison();
            geom_brep::ExtentBall::of_cover_from(None, Point3::new(poison, poison, poison))
        }
    }
}

/// The region any of `covers` occupies: their hull, a refusal
/// absorbing it. `None` for no cover.
pub(crate) fn hull_of_covers<T: Real>(
    covers: impl IntoIterator<Item = FaceCovered<T>>,
) -> Option<FaceCovered<T>> {
    covers.into_iter().reduce(|h, c| match (h, c) {
        (Ok(h), Ok(c)) => Ok(h.hull(c)),
        (Err(e), _) | (_, Err(e)) => Err(e),
    })
}

/// **Topo's reading of the region a face can occupy** (DESIGN
/// `[one-way.places]`, "The region a carrier, an edge or a face can
/// occupy", and `[one-way.readings]`): its carrier's cover
/// ([`Surface::cover`]) met with every trim constraint the reading
/// proves, one arm per kind, each constraint a stage of that one rule.
///
/// - **Plane**: the convex hull of its boundary (each edge's cover,
///   [`edge_cover`], and each isolated vertex).
/// - **Cylinder**: the boundary's convex hull, met with the carrier's
///   wall over the heights the hull spans ([`Cover::wall`]), which holds
///   the face's level about its axis. The carrier is ruled along its
///   axis, so the ruling through any point of the face leaves it at the
///   boundary both ways; that needs only that the axis be a direction.
/// - **Cone**: the convex hull of the boundary and, where the face may
///   hold the apex, the apex: the rulings run through it. A face holds
///   the apex only where a loop winds round the axis
///   ([`cone_may_hold_apex`]). Met with the carrier's frustum over the
///   heights the hull spans ([`Cover::cone`]), which is measured about
///   the axis ([`Cover::about`]).
/// - **Sphere**: the ball, met with the chart rectangle or latitude zone
///   its boundary spans ([`sphere_window`]), and with the pole-side cut
///   along an aim where one is read ([`face_cover_toward`]).
/// - **Torus**: the tube, met with the chart rectangle its boundary's
///   certified pcurves span ([`torus_chart_window`]) on a ring torus.
/// - **Spindle**: its tube; the chart window's closed form reads a ring
///   torus, so a spindle reads none.
/// - **Spline**: its control net (the fit's, for an approximation).
///
/// **A constraint the reading does not prove adds nothing**: a window
/// the walk gives up on, a pole it does not place `Out`. No arm then tries a second construction. A
/// face whose meet is still the whole carrier refuses.
///
/// # Errors
///
/// [`FaceUncovered`]: the carrier's refusal, where no trim constraint
/// bounds the face (a plane or a cylinder whose boundary has an edge
/// with no cover, a cylinder whose axis is no direction, a net carrying
/// poison).
///
/// # Panics
///
/// Where `face`, a face of `body`, or a record on the walk from it does
/// not resolve, or a loop walk does not close (D2 row 4): a torn
/// boundary is not one with no cover. The bodies are at rest or an
/// operation's working copies, whose links hold by
/// [`crate::live::OPERATORS_KEEP_LINKS`].
pub fn face_cover<T: Decide>(body: &Body<T>, face: FaceKey, band: Band) -> FaceCovered<T> {
    face_cover_in(body, face, band, None)
}

/// [`face_cover`] read along `aim`, which a sphere face's arm reads one
/// more constraint off: at each end of the aim where the face places
/// the sphere's pole `c ± r·â` definitely `Out`
/// ([`crate::boolean::curved_face_placement`]), the face reaches no
/// further along it than its boundary. `â·x` on a sphere has its only
/// critical points at those poles, so with one not a point of the face
/// its extreme that way is on the boundary. Only a definite `Out`
/// narrows; `In`, a boundary verdict, no verdict and a refusal say the
/// pole may be a point of the face. The cut is the solid cylinder of
/// the sphere's radius about the aim through the centre, between the
/// two reaches: it holds the ball's part on the face's side.
///
/// # Errors
///
/// As [`face_cover`].
///
/// # Panics
///
/// As [`face_cover`].
pub fn face_cover_toward<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
    aim: UnitVec3<T>,
) -> FaceCovered<T> {
    face_cover_in(body, face, band, Some(aim))
}

fn face_cover_in<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
    aim: Option<UnitVec3<T>>,
) -> FaceCovered<T> {
    let f = body.proven_face(face);
    let surface = f.surface();
    let carrier = surface.cover().map_err(FaceUncovered::of_carrier);
    match surface {
        Surface::Plane { .. } => and_trim(carrier, boundary_cover(body, f).map(Cover::convex)),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => {
            if !crate::validate::is_direction(axis) {
                return carrier;
            }
            let frame = geom::Frame::new(*origin, *axis, *u_ref);
            let hull = boundary_cover(body, f).map(Cover::convex);
            let wall = hull
                .as_ref()
                .ok()
                .and_then(|h| Cover::wall(&frame, *radius, h));
            and_proven(and_trim(carrier, hull), wall)
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => {
            let Some(at_apex) = Cover::points(vec![*apex]) else {
                return carrier;
            };
            let boundary = boundary_cover(body, f);
            let hull = if cone_may_hold_apex(body, face, surface, band) {
                boundary.map(|b| b.hull(at_apex))
            } else {
                boundary
            }
            .map(Cover::convex);
            let frustum = hull.as_ref().ok().and_then(|h| {
                Cover::cone(&geom::Frame::new(*apex, *axis, *u_ref), *half_angle, h)
            });
            and_proven(and_trim(carrier, hull), frustum)
        }
        Surface::Sphere { center, radius, .. } => {
            let zone = sphere_window(body, f, surface, band);
            let cut = aim.and_then(|aim| pole_side(body, f, (*center, *radius), aim, band));
            and_proven(and_proven(carrier, zone), cut)
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => {
            let ring = constraint(|| {
                geom::is_ring_torus(*major_radius, *minor_radius, band)
                    .then_some(())
                    .ok_or(())
            });
            let rect = ring
                .and_then(|()| {
                    torus_chart_window(&face_window_steps(body, face), *major_radius, *minor_radius)
                })
                .and_then(|(u, v)| {
                    Cover::patch(
                        &geom::Frame::new(*center, *axis, *u_ref),
                        (*major_radius, *minor_radius),
                        Some(u),
                        v,
                    )
                });
            and_proven(carrier, rect)
        }
        // A spindle's tube, with no chart window: the window's closed
        // form reads a ring torus.
        Surface::Spindle { .. } | Surface::Nurbs(_) | Surface::Approx(_) => carrier,
    }
}

/// **Whether a face on `cone` may hold its apex**: unless every loop is
/// decided to close in the chart ([`closes_in_chart`], over
/// `chart_window::face_loop_azimuth_travels`). The apex is
/// the chart's pole, and a bounded face holding it has a loop that
/// winds round it a whole turn, where a loop that closes in the chart
/// leaves it outside. Every loop, not the outer alone: a file's
/// designated outer bound can be a hole, and the loop enclosing the
/// apex then a ring. A loop not read, or a refusal reading one, may
/// hold it.
pub(crate) fn cone_may_hold_apex<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    cone: &Surface<T>,
    band: Band,
) -> bool {
    let travels = constraint(|| {
        crate::chart_window::face_loop_azimuth_travels(body, cone, face, band).map_err(|_| ())
    });
    travels.is_none_or(|loops| !loops.into_iter().all(|t| t.is_some_and(closes_in_chart)))
}

/// **Whether a loop whose net travel round a chart's period is `travel`
/// closes in the chart**: decided within half a turn of zero for every
/// value the scalar stands for. A loop that closes returns to where it
/// entered (net zero); one that winds round the period travels a whole
/// turn, so half a turn separates the two and the test needs no band.
/// The one reading of it for the face cover's walks: the torus chart
/// window's wrap guard and the cone's apex rule. (`pcurves::loop_windings`
/// asks it of a loop's stored certified rows, for W2's certificate.)
pub(crate) fn closes_in_chart<T: SpanLocate>(travel: T) -> bool {
    travel.is_bounded() && (T::pi() - travel.abs()).structure_positive()
}

/// **The boundary's cover**: each boundary edge's cover
/// ([`edge_cover_linked`]), hulled with each isolated vertex. A face's
/// region is not in it; a ruled one's is in its convex hull
/// ([`Cover::convex`]).
///
/// # Errors
///
/// Every boundary entity that has no cover.
///
/// # Panics
///
/// As [`face_cover`]: `f` is `face`'s record.
pub(crate) fn boundary_cover<T: Decide>(body: &Body<T>, f: FaceView<'_, T>) -> BoundaryCovered<T> {
    let mut acc: Option<BoundaryCovered<T>> = None;
    for member in f.boundary() {
        let one = match member {
            BoundaryMember::Isolated { vertex, point } => {
                Cover::points(vec![point]).ok_or_else(|| vec![BoundaryUnknown::Vertex(vertex)])
            }
            BoundaryMember::Edge { ek, edge, .. } => edge_cover_linked(body, ek, edge)
                .map_err(|why| vec![BoundaryUnknown::Edge(ek, why)]),
        };
        acc = Some(match (acc, one) {
            (None, one) => one,
            (Some(Ok(a)), Ok(b)) => Ok(a.hull(b)),
            (Some(Err(why)), Ok(_)) | (Some(Ok(_)), Err(why)) => Err(why),
            (Some(Err(mut a)), Err(b)) => {
                a.extend(b);
                Err(a)
            }
        });
    }
    let Some(acc) = acc else {
        unreachable!(
            "{} has an outer loop, so a boundary",
            EntityId::Face(f.key())
        )
    };
    acc
}

/// The pole-side cut of a sphere face along `aim` ([`face_cover_toward`]).
fn pole_side<T: Decide>(
    body: &Body<T>,
    f: FaceView<'_, T>,
    (center, radius): (Point3<T>, T),
    aim: UnitVec3<T>,
    band: Band,
) -> Option<Cover<T>> {
    let boundary = boundary_cover(body, f).ok()?;
    let (d, r) = (aim.get(), radius.abs());
    let c = (center - Point3::origin()).dot(d);
    let (below, above) = boundary.range_along(d);
    let out = |s: T| {
        constraint(|| {
            match crate::boolean::curved_face_placement(body, f.key(), center + d * (r * s), band) {
                Ok(crate::boolean::CurvedPlacement::Trim(Some(
                    crate::boolean::FaceContainment::Out,
                ))) => Ok(()),
                _ => Err(()),
            }
        })
        .is_some()
    };
    let lo = out(T::zero() - T::one()).then(|| below - c);
    let hi = out(T::one()).then(|| above - c);
    if lo.is_none() && hi.is_none() {
        return None;
    }
    let window = ParamWindow::new(lo.unwrap_or_else(|| T::zero() - r), hi.unwrap_or(r));
    Cover::slab(center, aim, r, window)
}

/// The K funnel name of a sphere's polar axis read as a unit direction,
/// before its chart window is read ([`sphere_window`]).
pub(crate) const SPHERE_AXIS: &str = "cover_sphere_axis";

/// The K funnel name of a sphere's seam direction against its polar
/// axis: their cosine, levered by the radius, decided zero before the
/// azimuth window is read in the frame they span ([`sphere_window`]).
pub(crate) const SPHERE_SEAM: &str = "cover_sphere_seam";

/// The K funnel name of a sphere's seam direction read as a unit
/// direction, before the azimuth window is read in the frame it spans
/// ([`sphere_window`]).
pub(crate) const SPHERE_SEAM_UNIT: &str = "cover_sphere_seam_unit";

/// **A sphere face's chart window**, from the window
/// `solid_contain::sphere_chart_trim` pins: its latitude extremes and
/// the azimuth window of its boundary walk, as the chart patch
/// ([`Cover::patch`], `R = 0`) of the rectangle or the latitude zone.
///
/// The trim reads the window off the boundary, and the boundary of a
/// rectangle is also the boundary of its complement. The rectangle is
/// therefore taken only when the side the outer loop encodes
/// (`boundary_material_sign`: the side of the loop that holds no chart
/// pole, the reading tier 3's curved sense check runs) agrees with the
/// face's `sense`, which places the face on the rectangle's side of its
/// boundary. The trim's class then holds no pole inside the face, so
/// the face lies in the rectangle its boundary spans. The side is read
/// without the tips the chart image drops: a wrap-tip cap holds the
/// pole at its tip, outside that class, and its rim alone encodes no
/// side. A face whose side is unencoded, refused, or in disagreement,
/// one whose outer loop is a lone vertex or carries null scaffolding,
/// one outside the trim's class, one whose trim escalates, one whose
/// axis is not decided unit, and one whose radius is not positive (the
/// patch's premise) read no window. A face that wraps the azimuth
/// alone, or whose seam direction is not decided a unit normal of the
/// axis, reads its latitude zone.
///
/// `face` resolved to `f` in the caller, so its outer loop and that
/// loop's members are links ([`Body::loop_members_linked`]), and a
/// record the flattening of that loop names is one too: a flattening
/// that refuses past a cycle loop is a torn body and panics
/// ([`crate::live::OPERATORS_KEEP_LINKS`]). The trim's `CorruptFace` is
/// a record miss past `face` too, and panics the same way.
fn sphere_window<T: Decide>(
    body: &Body<T>,
    f: FaceView<'_, T>,
    surface: &Surface<T>,
    band: Band,
) -> Option<Cover<T>> {
    use crate::boolean::solid_contain::PointInSolidError;
    use crate::props::LoopEdgesError;
    use geom_core::{Margin, Sign};
    let Surface::Sphere {
        center,
        radius,
        axis,
        u_ref,
    } = surface
    else {
        unreachable!("a sphere's window is read of a sphere")
    };
    if !radius.structure_positive() {
        return None;
    }
    let (outer, outer_loop) = f.outer();
    if matches!(
        body.loop_members_linked(outer, outer_loop).as_slice(),
        [BoundaryMember::Isolated { .. }]
    ) {
        return None;
    }
    let outer = match crate::props::chart_image_edges(body, outer) {
        Ok((outer, _)) if outer.is_empty() => return None,
        Ok((outer, _)) => outer,
        Err(LoopEdgesError::NullScaffoldEdge { .. }) => return None,
        Err(refusal @ LoopEdgesError::Corrupt { .. }) => torn_outer_loop(body, f, refusal),
    };
    let side = if f.sense() {
        Sign::Positive
    } else {
        Sign::Negative
    };
    let (trim, unit) = constraint(|| {
        if geom_brep::props::boundary_material_sign(surface, &outer, band).ok()
            != Some(geom_brep::props::MaterialSign::Encoded(side))
        {
            return Err(());
        }
        let trim = match crate::boolean::solid_contain::sphere_chart_trim(
            body,
            f.key(),
            *center,
            *radius,
            *axis,
            band,
        ) {
            Ok(Some(trim)) => trim,
            Ok(None) | Err(PointInSolidError::Escalated { .. }) => return Err(()),
            Err(refusal) => torn_outer_loop(body, f, refusal),
        };
        let unit = UnitVec3::new(*axis, SPHERE_AXIS, band).map_err(|_| ())?;
        Ok((trim, unit))
    })?;
    let pole = |sign: T| (sign * *radius, T::zero());
    let latitude = |(h, rho): (T, T)| h.atan2(rho);
    let lat = ParamWindow::new(
        latitude(trim.south.unwrap_or_else(|| pole(T::zero() - T::one()))),
        latitude(trim.north.unwrap_or_else(|| pole(T::one()))),
    );
    let seam = constraint(|| {
        let seam = UnitVec3::new(*u_ref, SPHERE_SEAM_UNIT, band).map_err(|_| ())?;
        let square = crate::validate::decide(
            SPHERE_SEAM,
            Margin::levered(
                seam.get().dot(unit.get()),
                geom_brep::ExtentBall::of_sphere(*center, *radius).lever(),
            ),
            band,
        );
        matches!(square, Ok(Sign::Zero)).then_some(seam).ok_or(())
    });
    let patch = |u_ref: Vec3<T>, u: Option<ParamWindow<T>>| {
        Cover::patch(
            &geom::Frame::new(*center, unit.get(), u_ref),
            (T::zero(), *radius),
            u,
            lat,
        )
    };
    match trim.az.zip(seam) {
        Some(((lo, hi), seam)) => patch(seam.get(), Some(ParamWindow::new(lo, hi))),
        None => patch(unit.orthonormal_basis().0, None),
    }
}

/// The torn hop under `refusal`, a record-miss refusal of a reading of
/// `face`'s outer loop ([`crate::props::loop_edges`],
/// `sphere_chart_trim`) past `face`, which resolved to `f`. Each hop
/// those readings take is re-read as a link, so the one that does not
/// resolve panics naming its holder, its field and both premises
/// ([`crate::live::dangling_link`]).
#[track_caller]
fn torn_outer_loop<T: Decide>(
    body: &Body<T>,
    f: FaceView<'_, T>,
    refusal: impl core::fmt::Debug,
) -> ! {
    f.surface();
    let (outer, outer_loop) = f.outer();
    for member in body.loop_members_linked(outer, outer_loop) {
        if let BoundaryMember::Edge { he, half, ek, edge } = member {
            body.edge_curve_linked(ek, edge);
            body.linked_vertex_point(half.start, EntityId::HalfEdge(he), "start");
            body.proven_half_edge_end(he);
        }
    }
    unreachable!(
        "the outer loop of {} refused {refusal:?} with every record on it resolved",
        EntityId::Face(f.key())
    )
}

/// A torus face's chart window: the `u` (major azimuth) and `v`
/// (minor angle) channels' windows, in that order.
pub(crate) type TorusWindowPair<T> = (ParamWindow<T>, ParamWindow<T>);

/// Two ranges' hull, least end first.
fn hull_of<T: Real>((a, b): (T, T), (c, d): (T, T)) -> (T, T) {
    (a.min(c), b.max(d))
}

/// One HALF-EDGE, as the window walk reads it: its stored certified
/// pcurve cache, its image as the loop's lift places it
/// ([`crate::Body::loop_lift`]), and whether the loop traverses it
/// FORWARD (the `he_plus` side, so the certified span runs `t₀ → t₁`).
/// `None` for a half-edge of a loop with no lift.
pub(crate) type WindowStep<'a, T> =
    Option<(&'a geom_brep::PcurveCache<T>, geom_brep::Pcurve<T>, bool)>;

/// **A torus face's chart window, from its boundary's stored certified
/// pcurves — the ONE walk, for every lane.**
///
/// The caller hands over its own arena traversal as loops of
/// [`WindowStep`]s and nothing else: the face cover's torus arm and the
/// construction rows both enter here.
///
/// # Why the window is a superset of the face's chart region
///
/// The face's region is the planar region its loops' chart images
/// bound, in the lift the pcurve mint certified closed, and a region
/// lies inside the bounding rectangle of its own boundary.
///
/// **That argument needs the boundary to SAY which side is material,
/// and two guards are what make it hold:
///
/// - **A face with a RING gets no window.** Two loops can be pinned on
///   different branches, and their hull is then a rectangle neither
///   bounds.
/// - **A loop whose walk WRAPS a channel gets no window.** A loop of
///   lone full circles closes by going once round a channel rather
///   than by returning to where it started, and the two complementary
///   annuli it can denote have the SAME pcurves and the same hull —
///   so the hull cannot contain both, and nothing in the caches says
///   which one this face is. Measured: a face bounded by two lone
///   full meridians at `u = 0` and `u = 22°` windows to
///   `[0, 22°] × [0, 2π]`, and in the orientation whose material is
///   the 338° complement, 1591 of 1681 sampled surface points lie
///   OUTSIDE that box. No kernel door mints such a face today — every
///   torus constructor seams its walls — so this is a latent hole
///   rather than a live one, and it is closed by refusing the class.
///
/// The wrap test is the loop's NET travel per channel: a loop that
/// closes in the chart returns to its entry point, so the signed sum
/// of `P(exit) − P(entry)` over its half-edges is zero, while a loop
/// that wraps sums to `± τ`. Half a period separates the two, so the
/// test needs no `Band` and no `decide`.
///
/// # The slack, and the bound it holds under
///
/// A cache certifies `sup |S(P(t)) − C(t)| ≤ envelope` in METRES
/// ([`geom_brep::PcurveCertificate::envelope`]), so the carrier can
/// sit up to `envelope` off the chart image's surface points. Pulling
/// that back through the chart costs the inverse's Lipschitz
/// constants: one radian of `u` moves a surface point by at least
/// `R − r` (the tube's nearest approach to the axis) and one radian of
/// `v` by exactly `r`.
///
/// **The factor of two is not "one for each end"**: the slack widens
/// both ends already. It is the price of the segment between
/// `S(P(t))` and `C(t)` LEAVING the surface: the chart inverse is
/// Lipschitz on the surface, and a point `envelope` off it is pulled
/// back only after being projected there, which can cost as much
/// again. That argument holds while the displacement stays inside the
/// tube's own scale — `envelope ≤ (R − r)/2` and `envelope ≤ r/2` —
/// so [`torus_window_slack`] GUARDS both, and a cache whose envelope
/// exceeds either gets no window at all.
pub(crate) struct TorusChartWindow<T: Real> {
    u: Option<(T, T)>,
    v: Option<(T, T)>,
    envelope: T,
    /// The current loop's signed travel per channel; `None` once a
    /// loop has been abandoned or has wrapped.
    net: Option<(T, T)>,
    loops: usize,
    ok: bool,
}

/// The certified slack a torus chart window owes an envelope, per
/// channel `(u, v)` and on each end, or `None` when the envelope is too
/// large for the argument that derives it ([`TorusChartWindow`], "the
/// slack").
///
/// `envelope` is in metres and both radii are lengths, so the guard is
/// a comparison of lengths and reads no angle.
pub(crate) fn torus_window_slack<T: Real>(envelope: T, major: T, minor: T) -> Option<(T, T)> {
    let half = T::from_f64(0.5);
    let (tube, ring) = (minor, major - minor);
    // **The comparison, in the terms [`Real`] has.** There is no
    // ordering on a generic scalar here — an interval scalar cannot
    // answer `<=` — so the guard is spelled through the totality
    // policy the module already builds [`geom_core::poison`] with:
    // `√(bound − envelope)` is poison exactly when the envelope
    // exceeds the bound, and at a bracketed scalar it is poison as
    // soon as it MIGHT, which refuses in the direction that costs the
    // whole tube rather than a wrong window.
    let exceeded = |bound: T| (bound * half - envelope).sqrt().is_poison();
    if exceeded(tube) || exceeded(ring) {
        return None;
    }
    let two = T::from_f64(2.0);
    Some((two * envelope / ring, two * envelope / tube))
}

/// A value that is not a bounded number for every value it stands for
/// ([`Real::is_bounded`]): poison, `±∞`, or an enclosure reaching
/// either. The window ends are tested with this rather than with
/// [`Real::is_poison`] alone, which an infinity walks straight past.
fn not_finite<T: Real>(x: T) -> bool {
    !x.is_bounded()
}

impl<T: SpanLocate> TorusChartWindow<T> {
    /// An empty accumulator.
    pub(crate) fn new() -> Self {
        Self {
            u: None,
            v: None,
            envelope: T::zero(),
            net: Some((T::zero(), T::zero())),
            loops: 0,
            ok: true,
        }
    }

    /// Open a loop. The FIRST loop is the face's outer one; every
    /// later one is a ring, and a ring abandons the window.
    pub(crate) fn open_loop(&mut self) {
        self.loops += 1;
        if self.loops > 1 {
            self.ok = false;
        }
        self.net = Some((T::zero(), T::zero()));
    }

    /// One half-edge of the open loop. The window reads a torus chart's
    /// harmonic images; any other image (a Villarceau circle's focal
    /// section among them) abandons it — which widens the box to the
    /// whole tube, never narrows it.
    pub(crate) fn step(&mut self, step: &WindowStep<'_, T>) {
        let Some((cache, image, forward)) = step else {
            self.ok = false;
            return;
        };
        let (t0, t1) = cache.params();
        let Some(b) = image.closed_form_span_box(t0, t1) else {
            self.ok = false;
            return;
        };
        let (u, v) = ((b.u_min, b.u_max), (b.v_min, b.v_max));
        let Some(travel) = harmonic_travel(image, t0, t1, *forward) else {
            self.ok = false;
            return;
        };
        self.u = Some(self.u.map_or(u, |a| hull_of(a, u)));
        self.v = Some(self.v.map_or(v, |a| hull_of(a, v)));
        self.envelope = self.envelope.max(cache.certificate().envelope);
        self.net = self.net.map(|(a, b)| (a + travel.0, b + travel.1));
    }

    /// Close the open loop: it must return to where it entered the
    /// chart rather than wrap a channel (the type docs).
    pub(crate) fn close_loop(&mut self) {
        let Some((nu, nv)) = self.net else {
            self.ok = false;
            return;
        };
        if !(closes_in_chart(nu) && closes_in_chart(nv)) {
            self.ok = false;
        }
        self.net = None;
    }

    /// Abandon the window outright — the caller's own arm for a loop
    /// it cannot read at all (a lone vertex carries no chart image).
    pub(crate) fn abandon(&mut self) {
        self.ok = false;
    }

    /// The window, widened by the certificate slack — see the type
    /// docs. `None` whenever any guard fired, and the arm then keeps
    /// the whole tube: no window widens, never narrows.
    pub(crate) fn finish(self, major: T, minor: T) -> Option<TorusWindowPair<T>> {
        if !self.ok {
            return None;
        }
        let (u, v) = (self.u?, self.v?);
        let (su, sv) = torus_window_slack(self.envelope, major, minor)?;
        let (u, v) = ((u.0 - su, u.1 + su), (v.0 - sv, v.1 + sv));
        if [u.0, u.1, v.0, v.1].into_iter().any(not_finite) {
            return None;
        }
        Some((ParamWindow::new(u.0, u.1), ParamWindow::new(v.0, v.1)))
    }
}

/// One half-edge's SIGNED travel in each chart channel, in the
/// direction the loop traverses it — `P(exit) − P(entry)`, the datum
/// [`TorusChartWindow`]'s wrap guard sums. `None` for a pcurve outside
/// the harmonic family.
pub(crate) fn harmonic_travel<T: Real>(
    pcurve: &geom_brep::Pcurve<T>,
    t0: T,
    t1: T,
    forward: bool,
) -> Option<(T, T)> {
    let (p0, pa, pb, pl) = pcurve.harmonic_form()?;
    let at = |t: T| {
        let (s, c) = t.sin_cos();
        (
            p0.x + pa.x * c + pb.x * s + pl.x * t,
            p0.y + pa.y * c + pb.y * s + pl.y * t,
        )
    };
    let (entry, exit) = if forward {
        (at(t0), at(t1))
    } else {
        (at(t1), at(t0))
    };
    Some((exit.0 - entry.0, exit.1 - entry.1))
}

/// **The face's loops, as [`WindowStep`]s**, read a loop at a time
/// through [`crate::body::Body::face_boundary_by_loop`]: the window
/// steps the face cover's torus arm and the construction rows read.
///
/// A lone-vertex loop yields an EMPTY loop, which [`torus_chart_window`]
/// abandons the window on: it carries no chart image.
///
/// # Panics
///
/// Where `face`, which every caller resolved in `body`, or a record on
/// the walk from it does not resolve, or a loop walk does not close
/// (D2 row 4), and where an edge does not claim the half-edge its loop
/// names. The bodies are at rest (the census's) or the boolean's
/// working copies, whose links hold by
/// [`crate::live::OPERATORS_KEEP_LINKS`].
pub(crate) fn face_window_steps<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Vec<Vec<WindowStep<'_, T>>> {
    body.proven_face(face)
        .boundary_by_loop()
        .map(|(_, members, _)| {
            let cycle: Vec<(HalfEdgeKey, &Edge)> = members
                .into_iter()
                .filter_map(|member| match member {
                    BoundaryMember::Edge { he, edge, .. } => Some((he, edge)),
                    BoundaryMember::Isolated { .. } => None,
                })
                .collect();
            let halves: Vec<HalfEdgeKey> = cycle.iter().map(|&(he, _)| he).collect();
            let lifted = crate::pcurves::lifted_images(body, &halves);
            cycle
                .into_iter()
                .zip(lifted)
                .map(|((he, edge), image)| {
                    let Some(claim) = edge.claim(he) else {
                        unreachable!(
                            "{} names {} on its loop, and its edge does not claim it",
                            EntityId::Face(face),
                            EntityId::HalfEdge(he)
                        )
                    };
                    body.pcurve(he)
                        .zip(image)
                        .map(|(c, image)| (c, image, claim.plus))
                })
                .collect()
        })
        .collect()
}

/// **The ONE walk**, over a face's loops of [`WindowStep`]s — see
/// [`TorusChartWindow`]. A loop that yields NO half-edge (a lone
/// vertex) abandons the window: it carries no chart image, so the walk
/// cannot see what bounds the face's chart region.
pub(crate) fn torus_chart_window<T: SpanLocate>(
    loops: &[Vec<WindowStep<'_, T>>],
    major: T,
    minor: T,
) -> Option<TorusWindowPair<T>> {
    let mut acc = TorusChartWindow::new();
    for lp in loops {
        acc.open_loop();
        for step in lp {
            acc.step(step);
        }
        if lp.is_empty() {
            acc.abandon();
        } else {
            acc.close_loop();
        }
    }
    acc.finish(major, minor)
}

/// **Topo's reading of the region an edge can occupy** (DESIGN
/// `[one-way.places]`, "The region a carrier, an edge or a face can
/// occupy"): its certified carrier's cover over its span
/// ([`geom::Curve3::cover_over`], which holds the one kind match),
/// hulled with its two end vertices, which sit on the carrier only to
/// its certification residual. Every bound on where an edge is reads
/// this ([`edge_box`], the census's and the narrow phase's reads in a
/// frame, the containment, shell and reduction doors) through the cover's methods, and none
/// matches on the carrier's kind.
///
/// # Errors
///
/// [`EdgeUncovered`]: null scaffolding, whose chord is not a bound; a
/// carrier its own reading cannot cover; an end vertex that is not a
/// finite point.
pub(crate) fn edge_cover_of<T: SpanLocate>(
    certified: Option<&geom_brep::EdgeCurve<T>>,
    ends: (Point3<T>, Point3<T>),
) -> Result<Cover<T>, EdgeUncovered> {
    let curve = certified.ok_or(EdgeUncovered::Scaffold)?;
    edge_cover_over(curve.carrier(), curve.params(), ends)
}

/// **[`edge_cover_of`]'s construction**, from a carrier, its span and
/// the edge's two ends: the carrier's cover over the span hulled with
/// the ends. Read alone where there is no certified edge record to read
/// it from: rows whose near-fold sector `mev`'s witness certification
/// refuses.
///
/// # Errors
///
/// As [`edge_cover_of`], save [`EdgeUncovered::Scaffold`].
pub(crate) fn edge_cover_over<T: SpanLocate>(
    carrier: &geom::Curve3<T>,
    span: (T, T),
    (a, b): (Point3<T>, Point3<T>),
) -> Result<Cover<T>, EdgeUncovered> {
    let arc = arc_cover(carrier, span)?;
    let ends = Cover::points(vec![a, b]).ok_or(EdgeUncovered::Ends)?;
    Ok(arc.hull(ends))
}

/// **The carrier's half of an edge's cover**: its cover over the span
/// ([`geom::Curve3::cover_over`]), with no ends. The one place topo
/// reads a carrier over an edge's span; [`edge_cover_over`] hulls it
/// with the ends. Read alone by a reader that holds a lifted carrier and
/// span with no edge record behind them (`cell_region`'s boundary arcs,
/// through `splitting::containment::arc_distance_floor`), which reads
/// its ends as vertices of its own.
///
/// # Errors
///
/// [`EdgeUncovered::Carrier`].
pub(crate) fn arc_cover<T: SpanLocate>(
    carrier: &geom::Curve3<T>,
    (t0, t1): (T, T),
) -> Result<Cover<T>, EdgeUncovered> {
    carrier
        .cover_over(ParamWindow::new(t0, t1))
        .map_err(EdgeUncovered::Carrier)
}

/// **How far `edge` reaches from `pivot`, bounded from both sides**
/// ([`geom_brep::edge_reach_bounds`]), its bound from above the far
/// distance of the edge's cover ([`edge_cover_linked`]) from `pivot`,
/// poison where the edge has no cover, which the reading levered at it
/// escalates. `None` for null scaffolding.
pub(crate) fn edge_reach_bounds<T: SpanLocate>(
    body: &Body<T>,
    edge: EdgeKey,
    e: &Edge,
    pivot: Point3<T>,
) -> Option<geom_brep::LeverBounds<T>> {
    let curve = body.edge_curve_linked(edge, e).certified()?;
    let cover = edge_cover_linked(body, edge, e).ok();
    Some(reach_bounds_in(
        curve.carrier(),
        curve.params(),
        cover.as_ref(),
        pivot,
    ))
}

/// **How far a piece of an edge reaches from `pivot`, bounded from both
/// sides**: the piece of `carrier` over `span` from a point inside the
/// edge to one of its vertices (`overlap`'s half-turns), its region the
/// edge cover's construction over the piece ([`edge_cover_over`]), read
/// as [`edge_reach_bounds`] reads the whole edge's.
pub(crate) fn piece_reach_bounds<T: SpanLocate>(
    carrier: &geom::Curve3<T>,
    span: (T, T),
    ends: (Point3<T>, Point3<T>),
    pivot: Point3<T>,
) -> geom_brep::LeverBounds<T> {
    let cover = edge_cover_over(carrier, span, ends).ok();
    reach_bounds_in(carrier, span, cover.as_ref(), pivot)
}

/// The reach bounds of an arc whose region is `cover`: from above its
/// far distance from `pivot` ([`geom_brep::ExtentBall::of_cover_from`]).
fn reach_bounds_in<T: SpanLocate>(
    carrier: &geom::Curve3<T>,
    span: (T, T),
    cover: Option<&Cover<T>>,
    pivot: Point3<T>,
) -> geom_brep::LeverBounds<T> {
    let above = geom_brep::ExtentBall::of_cover_from(cover, pivot).lever_from(pivot);
    geom_brep::edge_reach_bounds(carrier, span, pivot, above)
}

/// [`edge_cover_of`] for `edge`, its ends read through its half-edges.
///
/// # Errors
///
/// As [`edge_cover_of`].
///
/// # Panics
///
/// Where `edge`, which the caller read out of `body`, its curve or a
/// record on the way to its ends does not resolve (D2 row 4): a torn
/// curve is not an uncertified one.
pub fn edge_cover<T: SpanLocate>(body: &Body<T>, edge: EdgeKey) -> Result<Cover<T>, EdgeUncovered> {
    edge_cover_linked(body, edge, proven(&body.edges, edge, EntityId::Edge))
}

impl<T: SpanLocate> Body<T> {
    /// **The region `edge` can occupy** (`[one-way.places]`): its
    /// certified carrier's cover over its span hulled with its two end
    /// vertices, topo's one reading of an edge ([`edge_cover_of`]), for
    /// a reader outside topo.
    ///
    /// # Errors
    ///
    /// [`EdgeUncovered`], as [`edge_cover_of`].
    ///
    /// # Panics
    ///
    /// Where `edge`, a live edge of `self`, or a record on the way to its
    /// ends does not resolve (D2 row 4).
    pub fn edge_cover(&self, edge: EdgeKey) -> Result<Cover<T>, EdgeUncovered> {
        edge_cover(self, edge)
    }
}

impl<T: Decide> Body<T> {
    /// **The region `face` can occupy** (`[one-way.places]`): topo's one
    /// reading of a face ([`face_cover`]), for a reader outside topo.
    ///
    /// # Errors
    ///
    /// [`FaceUncovered`], as [`face_cover`].
    ///
    /// # Panics
    ///
    /// As [`face_cover`].
    pub fn face_cover(&self, face: FaceKey, band: Band) -> FaceCovered<T> {
        face_cover(self, face, band)
    }
}

/// [`edge_cover`] past its edge's lookup: `e` is `edge`'s record.
///
/// # Errors
///
/// As [`edge_cover_of`].
pub(crate) fn edge_cover_linked<T: SpanLocate>(
    body: &Body<T>,
    edge: EdgeKey,
    e: &Edge,
) -> Result<Cover<T>, EdgeUncovered> {
    let ends = (
        edge_end_point(body, edge, e.he_plus, "he_plus"),
        edge_end_point(body, edge, e.he_minus, "he_minus"),
    );
    edge_cover_of(body.edge_curve_linked(edge, e).certified(), ends)
}

/// **Why an edge has no cover** ([`edge_cover_of`]). A reader states its
/// own direction on it: a pruning reader takes the box that overlaps
/// everything, a granting reader refuses.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EdgeUncovered {
    /// Null scaffolding: nothing is certified about the locus, and the
    /// chord between its ends is not a bound.
    Scaffold,
    /// The certified carrier's own reading refused.
    Carrier(geom::cover::Uncovered),
    /// An end vertex's point is not a finite number.
    Ends,
}

impl core::fmt::Display for EdgeUncovered {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Scaffold => f.write_str("the edge is null scaffolding"),
            Self::Carrier(why) => write!(f, "the edge's carrier has {why}"),
            Self::Ends => f.write_str("an end vertex of the edge is not a finite point"),
        }
    }
}

/// The start point of `edge`'s half `he`, which `edge`'s field `field`
/// names: every hop is a link.
#[track_caller]
pub(crate) fn edge_end_point<T: Real>(
    body: &Body<T>,
    edge: EdgeKey,
    he: HalfEdgeKey,
    field: &'static str,
) -> Point3<T> {
    let start = linked(
        &body.half_edges,
        he,
        EntityId::HalfEdge,
        EntityId::Edge(edge),
        field,
    )
    .start;
    body.linked_vertex_point(start, EntityId::HalfEdge(he), "start")
}
