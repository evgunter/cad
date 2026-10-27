//! **The side at infinity of a selection of faces**: which side of its
//! boundary lies outside every face, read off rays aimed at its faces
//! from beyond them, and off its certified volume only where no aim
//! reads ([`side_at_infinity`]). It is the one reader of that fact,
//! and every question that is that fact asks it:
//!
//! - a shell's role, outer or cavity ([`crate::props`]'s
//!   `shell_role`), which check 10 ([`fn@crate::validate`]), the pieces
//!   sort ([`crate::pieces`]), the cavity read of `shell`, the census's
//!   role gate ([`crate::census`]) and `classify_shells` read;
//! - a solid's orientation, check 7 ([`fn@crate::validate`]);
//! - the side of a point no schedule ray reaches, point containment's
//!   missed-ray side ([`crate::boolean::solid_contain`]).
//!
//! The rays it casts are point containment's
//! ([`crate::boolean::solid_contain`]'s crossing walk), logged under their
//! own rows ([`AIMED_ROWS`]) so the two readers' verdicts stay apart.

use geom_core::{Band, Decide, Indeterminate, Point3, Vec3};

use crate::body::Body;
use crate::boolean::SolidContainment;
use crate::boolean::solid_contain::{
    FaceGeo, PointInSolidError, RayHit, RayRows, SolidFaces, face_geo, face_plane, ray_crossings,
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
    vertex: "side_at_infinity_vertex",
    edge: "side_at_infinity_edge",
    edge_span: "side_at_infinity_edge_span",
    winding: "side_at_infinity_winding",
    represents: "side_at_infinity_met_edge_represents",
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
    /// The point on it the ray was aimed at, as coordinates: a witness
    /// rendered once, never a value to compare. The point exists only at
    /// the run's scalar, and a refusal payload carries no `f64` out of a
    /// `T: Decide` (#990, `geom_core::real`'s "two shapes that are not
    /// decisions"), so it takes the witness posture
    /// (`census::witness`).
    pub aim: String,
    /// The face of one crossing.
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
    /// A witness, as [`AimedPair::aim`] is, by #990.
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
/// **The falsifier is a Python caller who must tell a corner from
/// a crossing in band of a face's boundary** without reading the
/// sentence. When the façade carries the reading, delete this paragraph
/// and the disposition row with it.
#[derive(Clone, Debug, PartialEq)]
pub enum SetAsideWhy {
    /// It passes within the band of a vertex, and reads nothing (D10,
    /// Booleans): a corner, never a pair.
    Corner {
        /// Two faces at the corner.
        faces: [FaceKey; 2],
        /// The ray's distance from the vertex, in band.
        gap: Indeterminate,
    },
    /// It passes within the band of two edges of one face, narrower than
    /// the band there, and reads nothing (D10, Booleans): a wedge, never
    /// a pair.
    Wedge {
        /// The face and the other face of one of its edges.
        faces: [FaceKey; 2],
        /// The ray's distance from the nearer edge met in band.
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
                 planar face, or a point of a curved face)",
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
                SetAsideWhy::Corner { faces, gap } => write!(
                    f,
                    ", it passed in band of a corner of {} and {}: {}",
                    EntityId::Face(faces[0]),
                    EntityId::Face(faces[1]),
                    gap.payload()
                )?,
                SetAsideWhy::Wedge { faces, gap } => write!(
                    f,
                    ", it passed in band of two edges of {}, narrower than the band there \
                     (one of them shared with {}): {}",
                    EntityId::Face(faces[0]),
                    EntityId::Face(faces[1]),
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
/// there. A planar face whose every edge is a line or a circle arc aims at
/// a point stepped in from its first edge ([`loop_aim`]), which asks
/// nothing: the ray meets the edges it passes, however near, and crosses
/// the face by the turn of its loops ([`ray_crossings`]). Another planar
/// face aims at a point certified strictly inside it
/// ([`crate::stands::planar_interior_point`]), and offers none where no
/// candidate certifies. A cylinder or cone face aims at its region's own
/// aim (its boundary's middle points carried onto the surface), a
/// trimmed sphere at its region's own (its first arc's golden-section
/// point stepped toward the face), a torus and a closed sphere at a
/// fixed point of their charts; none of these needs to lie inside the
/// face: the ray's start lies beyond every face whatever it aims at, so
/// its count reads the side wherever it lands, and an aim off the face
/// only steers it. A cylinder's or a sphere's aim lies on its surface by
/// construction, so its normal is the chart's, read without certifying
/// the aim onto the surface; a torus's and a cone's are their face's
/// reader's (below). A cylinder or cone face whose
/// region cannot be read ([`FaceGeo::PartialWall`],
/// [`FaceGeo::PartialCone`]) has no window to aim in and offers none. A
/// spline offers none: a selection holding one casts no ray
/// ([`side_at_infinity`]).
///
/// # Errors
///
/// A planar face that does not walk, or whose plane or in-face walk
/// refuses other than in band; a face that does not resolve.
fn aim_at<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    geo: &FaceGeo<T>,
    band: Band,
) -> Result<Option<Aim<T>>, PointInSolidError> {
    match face_plane(body, face) {
        Ok((_, normal)) => {
            if let Some(aim) = loop_aim(body, face, normal) {
                return Ok(Some((aim, normal)));
            }
            return Ok(
                crate::stands::planar_interior_point(body, face, band)?.map(|aim| (aim, normal))
            );
        }
        Err(PointInSolidError::KindUnsupported { .. }) => {}
        Err(e) => return Err(e),
    }
    let f = body
        .get_face(face)
        .ok_or(PointInSolidError::CorruptFace { face })?;
    let surface = body
        .get_surface(f.surface)
        .ok_or(PointInSolidError::CorruptFace { face })?;
    let aim = match *geo {
        FaceGeo::Cylinder { ref region, .. } | FaceGeo::Cone { ref region, .. } => {
            match region.aim() {
                Some(aim) => aim,
                None => return Ok(None),
            }
        }
        FaceGeo::SpherePatch { ref region, .. } => match region.aim() {
            Some(aim) => aim,
            None => return Ok(None),
        },
        // A torus face, whole or trimmed, is aimed at one point of its
        // surface: the ray starts beyond every face whatever it aims at.
        FaceGeo::Torus { .. } | FaceGeo::TorusPatch { .. } => surface.eval(T::pi(), T::pi()),
        FaceGeo::Sphere { .. } => surface.eval(T::from_f64(1.0), T::from_f64(0.3)),
        _ => return Ok(None),
    };
    // A cylinder's or a sphere's aim lies on its surface by construction,
    // where the chart's gradient is its unit normal: the aim only steers,
    // so nothing certifies it onto the surface. A torus's chart is singular
    // outside the ring convention, which nothing here has asked of it, and
    // a cone's gradient is unit everywhere off its axis and undefined at
    // its apex, so for those two the face's own reader gives the normal,
    // and a normal it cannot give (an in-band residual, a horn or spindle
    // torus, an apex) leaves no incidence to aim at: the face offers none.
    if matches!(
        *geo,
        FaceGeo::Cone { .. } | FaceGeo::Torus { .. } | FaceGeo::TorusPatch { .. }
    ) {
        return Ok(
            crate::face_normal::face_outward_normal_at(body, face, aim, band)
                .ok()
                .flatten()
                .map(|normal| (aim, normal.vec())),
        );
    }
    let chart = geom_brep::implicit_gradient(surface, aim);
    Ok(Some((
        aim,
        geom_brep::OutwardNormal::from_chart(chart, f.sense).vec(),
    )))
}

/// **Where a ray aimed at planar `face` aims**, where every edge of it is
/// a line or a circle arc, found by evaluation alone: the point `m` of the
/// outer loop's first edge (in walk order, so a body rebuilt with another
/// first half-edge aims elsewhere) at the golden section of its span
/// ([`geom_core::GOLDEN_SECTION`]), stepped into the face along
/// `normal × τ` ([`crate::stands::inward`]) by half a lower bound on `m`'s
/// distance from every other edge of every loop
/// ([`crate::boolean::solid_contain::edge_clearance`]), and
/// by at most half its own arc's radius. The step's path meets no edge,
/// so the aim lies in the face, off its middles and its boundary; a face
/// narrower than the rounding there steps nowhere, and its aim only
/// steers. It asks nothing, and the ray crosses the face edge by edge or
/// by the turn of its loops however near an edge it lands
/// ([`ray_crossings`]). `None` for a face with another edge, or one whose
/// outer loop has none.
fn loop_aim<T: Decide>(body: &Body<T>, face: FaceKey, normal: Vec3<T>) -> Option<Point3<T>> {
    let edges = crate::boolean::solid_contain::face_edges(body, face).ok()?;
    let outer = body.get_face(face)?.outer;
    let first = *edges.first()?;
    if body.get_half_edge(first.half)?.parent_loop != outer {
        return None;
    }
    let curve = |e: crate::entity::EdgeKey| {
        body.get_edge(e)
            .and_then(|ed| body.get_curve_geom(ed.curve))
            .and_then(crate::null::CurveGeom::certified)
    };
    let (m, tau) = crate::boolean::solid_contain::along_half(
        body,
        first.half,
        1.0 - geom_core::GOLDEN_SECTION,
    )?;
    let half = T::from_f64(0.5);
    let mut step = match *curve(first.edge)?.carrier() {
        geom::Curve3::Line { .. } => None,
        geom::Curve3::Circle { radius, .. } => Some(radius.abs() * half),
        _ => return None,
    };
    for e in &edges {
        if e.edge == first.edge || e.across == face {
            continue;
        }
        let room = crate::boolean::solid_contain::edge_clearance(body, e, m)? * half;
        step = Some(step.map_or(room, |s: T| s.min(room)));
    }
    let into = crate::stands::inward(normal, tau);
    Some(m + into / into.norm() * step.unwrap_or(T::zero()).max(T::zero()))
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
/// them in band of each other are no pair: the gap between them closes
/// at the cell they share, and the ray sums them.
/// - A face meets itself: a curved face crossed twice in band (a ray
///   grazing near its silhouette) is no pair either.
/// - Two faces meet where they share an edge, or only a vertex (a
///   pyramid's opposite flanks at its apex): some half-edge of one starts
///   where a half-edge of the other does.
///
/// Both faces' loops are walked in place; nothing is collected.
pub(crate) fn meet<T: geom_core::Real>(body: &Body<T>, a: FaceKey, b: FaceKey) -> bool {
    if a == b {
        return true;
    }
    // Not `face_edges`, which collects and resolves every mate: this reads
    // only the starts, in place, once per pair of faces.
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

/// The incidence an aimed ray meets its face at: `cos 10°` along the
/// face's outward normal and `sin 10°` along the face. Near the normal,
/// a face is as wide to the ray as it is, so a ray through a narrow face
/// passes its two edges as far apart as they are; the tilt keeps the ray
/// off the planes of a box's faces square to the aimed one.
pub(crate) const AIM_INCIDENCE: (f64, f64) = (0.984_807_753_012_208, 0.173_648_177_666_930_33);

/// The turn about the face's normal its aimed ray's tangent component
/// makes off the normal's frame (`Vec3::orthonormal_basis`), as
/// `(sin, cos)` of the golden angle ([`geom_core::GOLDEN_ANGLE`], which
/// profile's planar ray schedule also steps by; topo's own ray schedules,
/// `SCHEDULE` and `SCHEDULE_2D`, are dyadic tables and no reader of
/// it): an axis frame's tangent would lay the ray in a coordinate plane,
/// parallel to every face of a box across it, where a turn the model has
/// no reason to share lays it parallel to none.
fn aim_turn() -> (f64, f64) {
    geom_core::GOLDEN_ANGLE.sin_cos()
}

/// **The side at infinity of `sel`'s faces** — the one reader of a
/// shell's role (D10, Booleans: `Out` is an outer boundary, `In` a
/// cavity's) and of a solid's orientation (D1 tier 3, check 7: `In` is
/// inside out).
///
/// **Aimed rays first.** Each face of the selection that offers an aim
/// ([`aim_at`]: a point stepped into a planar face from its first edge
/// where its edges are lines and circle arcs, another planar face's certified interior
/// point, a point of a curved face's surface)
/// is aimed at, in face order: the ray leaves from twice the
/// selection's reach from that point, beyond every face, so its start
/// lies on the side at infinity, and meets the face at 10° from its
/// outward normal ([`AIM_INCIDENCE`]), turned about it off the coordinate
/// planes ([`aim_turn`]). It reads the side as the sign of its count of
/// crossings (D10, Booleans; [`ray_crossings`]): a ray within the band
/// of an edge meets it, half a crossing for each face the edge bounds,
/// crossings in band of each other are summed, and the first nonzero
/// running count's sign is the side, as a schedule ray's is (point
/// containment's), its decisions logged under the aimed rows
/// ([`AIMED_ROWS`]). The first ray that decides answers.
///
/// **A pair is two crossings of faces that do not meet** (`meet`), in
/// band of each other along the ray, each read where the ray meets its
/// faces ([`RayHit::pair`]): a face's crossing, or the point where the
/// ray passes an edge, in band of both its faces; an edge the ray passes
/// without crossing is no party. Two faces `t` apart are met about `t`
/// apart along the ray, less what the band lets an edge's passage slide
/// along it, so the ray refuses only where its parties' faces lie about
/// in band. Where the ray crosses their carriers is no such read: on a
/// ray shallow to a face, its carrier crossing lands far off the face,
/// beside another face.
///
/// **A corner or a wedge reads nothing, and is read before a pair** in
/// the same cluster ([`crate::ray_walk::Crossings::first_count`]): D10
/// refuses only on a pair, which restricts refusal and does not demand
/// it, and a ray that meets a corner or a wedge reads nothing. A ray
/// that grazes a face, meets a corner (a vertex in band of it) or a
/// wedge (a face it passes within the band of at two of its edges,
/// narrower than the band there), or crosses a curved face in band of
/// its trim, reads nothing and is set aside, and the next face is aimed
/// at. A needle narrower than about `2Kε` across offers no ray that
/// reads: every ray through it meets a wedge.
///
/// **The volume only where no face offers an aim that reads.**
/// Where no aimed ray decided and none read in band, `volume` reads
/// the side: a selection all of whose faces are cylinders or cones whose
/// region cannot be read, or whose rays all grazed, met a limit, a
/// corner or a wedge. A selection holding a face the ray test has no
/// arm for ([`crate::boolean::solid_contain::ray_crosses`]: a spline) casts no ray at all: every ray
/// would fault on that face, whatever it aimed at, so the volume reads
/// with it as the limit.
/// An aimed ray read in band on two faces that do not meet refuses on
/// its pair ([`SideUnread::InBand`]) without the volume: a pair in band
/// is the answer. The volume's sign is certified by its enclosure alone,
/// never held against the band (D10), so it reads a thin selection as
/// surely as a thick one. A ray that meets a corner or a wedge is set
/// aside ([`SetAsideWhy::Corner`], [`SetAsideWhy::Wedge`]): it reads
/// nothing and is never the refusal,
/// and where the volume does not read either, the volume's refusal is,
/// carrying the rays set aside ([`SideUnread::Fallback`]).
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
    match geom_core::k_stats::in_family(FAMILY, || cast_aims(body, sel, band))? {
        Rays::Side(side) => Ok(Ok(side)),
        Rays::InBand(pair) => Ok(Err(SideUnread::InBand(pair))),
        Rays::Unread { aimed, set_aside } => Ok(volume().map_err(|volume| SideUnread::Fallback {
            volume,
            aimed,
            set_aside,
        })),
    }
}

/// **The family the ray caster's decisions are recorded under**
/// ([`geom_core::k_stats::in_family`]): every verdict [`cast_aims`]
/// decides — the aimed rays' crossings, their pair's gap
/// (`side_at_infinity_order`) and the in-face and region walks it reads
/// them through. The volume the reader falls back on lies outside it,
/// under its own names: it is the document's measurement, not the rays'.
///
/// **What a flip among them can and cannot do.** The side a ray reads
/// is the same for every ray from outside the body, so where one ray
/// reads and another reads differently is a tie-break the band forbids
/// the instrument to represent (`[verdicts]`). One verdict is no
/// tie-break: `side_at_infinity_order` is a pair's gap, geometry, and its
/// flip moves a ray between reading and refusing on that pair. Either
/// way a flip in this family changes whether the side is read, never
/// which side: it surfaces as a refusal, never as a piece that moved,
/// so N5's flip lane passes over the family (`editor_core::resolve`).
pub const FAMILY: &str = "side_at_infinity_";

/// What one aimed ray's count reads in band: a pair, or a corner or a
/// wedge it meets ([`cast_aims`]).
enum Unread {
    Pair(AimedPair),
    Mark(SetAsideWhy),
}

/// What the aimed rays read ([`cast_aims`]).
#[derive(Debug)]
enum Rays {
    /// A ray decided the side.
    Side(SolidContainment),
    /// A ray read in band on two faces that do not meet.
    InBand(AimedPair),
    /// No ray read: how many were aimed, and each set aside.
    Unread {
        aimed: usize,
        set_aside: Vec<SetAside>,
    },
}

/// The aimed rays of [`side_at_infinity`], before the volume.
fn cast_aims<T: Decide>(
    body: &Body<T>,
    sel: &SolidFaces,
    band: Band,
) -> Result<Rays, PointInSolidError> {
    let faces = sel.faces();
    // A face the ray test has no arm for faults EVERY ray at
    // `face_geo`, whatever it aims at: no aim can read, so none is cast,
    // and the volume reads with that face as the limit.
    for &face in faces {
        if let Some(e) = uncrossable(body, face)? {
            return Ok(Rays::Unread {
                aimed: 0,
                set_aside: vec![SetAside {
                    aimed: face,
                    aim: None,
                    why: SetAsideWhy::Limit(e),
                }],
            });
        }
    }
    // Each face's geometry, resolved once for every aimed ray. A face
    // whose geometry refuses refuses every ray that reaches it, so none
    // is cast, and the volume reads with that face as the limit.
    let mut geos = Vec::with_capacity(faces.len());
    for &face in faces {
        match face_geo(body, face, &sel.charts) {
            Ok(geo) => geos.push((face, geo)),
            Err(e) => {
                return Ok(Rays::Unread {
                    aimed: 0,
                    set_aside: vec![SetAside {
                        aimed: face,
                        aim: None,
                        why: SetAsideWhy::Limit(e),
                    }],
                });
            }
        }
    }
    let (mut in_band, mut aimed, mut set_aside) = (None, 0, Vec::new());
    for (face, geo) in &geos {
        let face = *face;
        let Some((aim, normal)) = aim_at(body, face, geo, band)? else {
            continue;
        };
        aimed += 1;
        match aimed_ray(body, faces, &geos, face, aim, normal, band)? {
            Aimed::Side(side) => return Ok(Rays::Side(side)),
            Aimed::Pair(pair) => {
                in_band.get_or_insert(pair);
            }
            Aimed::SetAside(why) => set_aside.push(SetAside {
                aimed: face,
                aim: Some(crate::census::witness(aim)),
                why,
            }),
        }
    }
    if let Some(pair) = in_band {
        return Ok(Rays::InBand(pair));
    }
    Ok(Rays::Unread { aimed, set_aside })
}

/// What one aimed ray reads ([`aimed_ray`]).
#[derive(Debug)]
enum Aimed {
    /// The side it decided.
    Side(SolidContainment),
    /// A pair in band.
    Pair(AimedPair),
    /// Why it read nothing.
    SetAside(SetAsideWhy),
}

/// **One aimed ray**: from twice the selection's reach off `aim` on
/// `face`, at [`AIM_INCIDENCE`] to `normal`, turned off the coordinate
/// planes ([`aim_turn`]), read off the sign of its count.
#[allow(clippy::too_many_arguments)] // the selection, the aim and the band
fn aimed_ray<T: Decide>(
    body: &Body<T>,
    faces: &[FaceKey],
    geos: &[(FaceKey, FaceGeo<T>)],
    face: FaceKey,
    aim: Point3<T>,
    normal: Vec3<T>,
    band: Band,
) -> Result<Aimed, PointInSolidError> {
    let (along, across) = AIM_INCIDENCE;
    let at = crate::census::witness(aim);
    let shell = body
        .get_face(face)
        .ok_or(PointInSolidError::CorruptFace { face })?
        .shell;
    // The ray is scratch: no output keeps its aim, and only the
    // crossings it reads decide.
    let (b1, b2) = normal.orthonormal_basis();
    let (turn_sin, turn_cos) = aim_turn();
    let tangent = b1 * T::from_f64(turn_cos) + b2 * T::from_f64(turn_sin);
    let out = normal * T::from_f64(along) + tangent * T::from_f64(across);
    let from = aim + out * (beyond(body, faces, aim)? * T::from_f64(2.0));
    let d = (aim - from).normalize();
    let reach = selection_reach(body, faces, from)?;
    let crossings = match ray_crossings(body, geos, from, d, reach, band, AIMED_ROWS) {
        Ok(crossings) => crossings,
        Err(RayFault::Blocked(e) | RayFault::Fatal(e)) => {
            return Ok(Aimed::SetAside(SetAsideWhy::Limit(e)));
        }
        Err(RayFault::InBand(e)) => return Ok(Aimed::SetAside(SetAsideWhy::Crossing(e))),
        Err(RayFault::Graze | RayFault::Unread) => return Ok(Aimed::SetAside(SetAsideWhy::Graze)),
    };
    Ok(
        match crossings.first_count(
            AIMED_ROWS.order,
            band,
            RayHit::count,
            |a, b| {
                a.pair(body, b, AIMED_ROWS.order, band).map_err(|f| {
                    f.map(|(near, gap, far)| {
                        Unread::Pair(AimedPair {
                            shell,
                            aimed: face,
                            aim: at.clone(),
                            near,
                            far,
                            gap,
                        })
                    })
                })
            },
            |hits| {
                RayHit::unread(hits, |wedge, faces, gap| {
                    Unread::Mark(if wedge {
                        SetAsideWhy::Wedge { faces, gap }
                    } else {
                        SetAsideWhy::Corner { faces, gap }
                    })
                })
            },
        ) {
            Ok(Some(outward)) => Aimed::Side(side_before(outward)),
            Err(RayFault::InBand(Unread::Pair(pair))) => Aimed::Pair(pair),
            Err(RayFault::InBand(Unread::Mark(why))) => Aimed::SetAside(why),
            // The count reads no limit: a tie between two faces that do
            // not meet, a graze, or nothing that counts.
            Ok(None) | Err(_) => Aimed::SetAside(SetAsideWhy::Graze),
        },
    )
}
/// **What each solid's aimed rays read**, before the volume, as text:
/// the probe surface `test_support::aimed_rays` renders.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn aimed_rays_report<T: Decide>(body: &Body<T>, band: Band) -> Vec<String> {
    body.solids()
        .map(|(solid, _)| match SolidFaces::of(body, solid) {
            Ok(sel) => format!("{:?}", cast_aims(body, &sel, band)),
            Err(e) => format!("no selection: {e:?}"),
        })
        .collect()
}

/// **What each shell's aimed rays read**, before the volume, as text:
/// a shell's role is read off its own faces (`SolidFaces::of_shell`).
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn shell_aims_report<T: Decide>(body: &Body<T>, band: Band) -> Vec<String> {
    body.shells()
        .map(|(shell, _)| {
            format!(
                "{:?}",
                cast_aims(body, &SolidFaces::of_shell(body, shell), band)
            )
        })
        .collect()
}

/// **Each aim's own reading**, every face of every solid aimed at (not
/// only until one decides), as `(face, reading)` text: the curved
/// edge-meeting ray's probes read it.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn each_aim_report<T: Decide>(body: &Body<T>, band: Band) -> Vec<String> {
    let mut out = Vec::new();
    for (solid, _) in body.solids() {
        let Ok(sel) = SolidFaces::of(body, solid) else {
            out.push("no selection".to_owned());
            continue;
        };
        let geos: Vec<_> = match sel
            .faces()
            .iter()
            .map(|&f| face_geo(body, f, &sel.charts).map(|g| (f, g)))
            .collect()
        {
            Ok(geos) => geos,
            Err(e) => {
                out.push(format!("no geometry: {e:?}"));
                continue;
            }
        };
        for (face, geo) in &geos {
            let read = match aim_at(body, *face, geo, band) {
                Ok(Some((aim, normal))) => {
                    format!(
                        "{:?}",
                        aimed_ray(body, sel.faces(), &geos, *face, aim, normal, band)
                    )
                }
                Ok(None) => "no aim".to_owned(),
                Err(e) => format!("aim refuses: {e:?}"),
            };
            out.push(format!("{face:?}: {read}"));
        }
    }
    out
}

/// **A frame at a point of an edge, on one of its faces**: what the
/// probes aim at and about an edge ([`edge_frames`]).
#[cfg(any(test, feature = "test-support"))]
pub struct EdgeFrame {
    /// The edge's carrier kind, its key and its two faces' surface kinds.
    pub label: String,
    /// The face.
    pub face: FaceKey,
    /// The point of the edge.
    pub point: Point3<f64>,
    /// The unit direction into the face there, `N × τ`.
    pub into: Vec3<f64>,
    /// The face's outward unit normal there.
    pub normal: Vec3<f64>,
}

/// **Each edge's frames, on each of its faces**, at each fraction of
/// `fractions` along the edge's span (`N` the face's outward normal at
/// the point, `τ` its loop's tangent): the probes aim rays at and about
/// an edge from them. An edge whose face has no normal at the point is
/// left out.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn edge_frames(body: &Body<f64>, fractions: &[f64], band: Band) -> Vec<EdgeFrame> {
    let kind = |f: FaceKey| {
        body.get_face(f)
            .and_then(|fc| body.get_surface(fc.surface))
            .map_or("?".to_owned(), |s| format!("{:?}", s.kind()))
    };
    let mut out = Vec::new();
    for (face, _) in body.faces() {
        // A face whose loops do not walk has no edge to frame.
        for fe in crate::boolean::solid_contain::face_edges(body, face).unwrap_or_default() {
            let Some(edge) = body.edges.get(fe.edge) else {
                continue;
            };
            let Some(curve) = body
                .curves
                .get(edge.curve)
                .and_then(crate::null::CurveGeom::certified)
            else {
                continue;
            };
            let (t0, t1) = curve.params();
            for &x in fractions {
                let t = t0 + (t1 - t0) * x;
                let point = curve.carrier().eval(t);
                let mut tau = curve.carrier().deriv(t).normalize();
                if edge.he_minus == fe.half {
                    tau = tau * -1.0;
                }
                let Ok(Some(normal)) =
                    crate::face_normal::face_outward_normal_at(body, face, point, band)
                else {
                    continue;
                };
                let normal = normal.vec();
                out.push(EdgeFrame {
                    label: format!(
                        "{:?} {:?} [{} | {}]",
                        curve.carrier().kind(),
                        fe.edge,
                        kind(face),
                        kind(fe.across),
                    ),
                    face,
                    point,
                    into: normal.cross(tau).normalize(),
                    normal,
                });
            }
        }
    }
    out
}

/// **Rays aimed at and beside every edge**: for each face and each edge
/// of its loops, a ray aimed at the edge's middle offset `o` into the
/// face ([`edge_frames`]), for each `o` of `offsets`, read as an aimed
/// ray is. Each line is `edge kind, the two faces' surface kinds,
/// offset: reading`.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn edge_rays_report(body: &Body<f64>, offsets: &[f64], band: Band) -> Vec<String> {
    let mut out = Vec::new();
    for f in edge_frames(body, &[0.5], band) {
        for &o in offsets {
            out.push(format!(
                "{} o={o:e}: {}",
                f.label,
                ray_at_report(body, f.face, f.point + f.into * o, f.normal, band)
            ));
        }
    }
    out
}

/// **One ray aimed at `aim` on `face`** of the solid holding it, at the
/// aimed incidence to `normal`, read as an aimed ray is: the probes aim
/// it at and about an edge.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn ray_at_report<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    aim: Point3<T>,
    normal: Vec3<T>,
    band: Band,
) -> String {
    let Some(solid) = body
        .get_face(face)
        .and_then(|f| body.get_shell(f.shell))
        .map(|sh| sh.solid)
    else {
        return "no solid".to_owned();
    };
    let Ok(sel) = SolidFaces::of(body, solid) else {
        return "no selection".to_owned();
    };
    let geos: Result<Vec<_>, _> = sel
        .faces()
        .iter()
        .map(|&f| face_geo(body, f, &sel.charts).map(|g| (f, g)))
        .collect();
    match geos {
        Ok(geos) => format!(
            "{:?}",
            aimed_ray(body, sel.faces(), &geos, face, aim, normal, band)
        ),
        Err(e) => format!("no geometry: {e:?}"),
    }
}

/// **Every crossing of the ray `q + s·d`** with the solid holding
/// `face`, its faces' order turned left by `turn` and reversed where
/// `reversed` (an order permutation), each as `(from, to, count,
/// sideless)` — a mark is sideless — or the fault the walk raised, as
/// text: the probes read a whole ray's count off it.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn ray_counts(
    body: &Body<f64>,
    face: FaceKey,
    (q, d): (Point3<f64>, Vec3<f64>),
    band: Band,
    (turn, reversed): (usize, bool),
) -> Result<Vec<(f64, f64, i32, bool)>, String> {
    let solid = body
        .get_face(face)
        .and_then(|f| body.get_shell(f.shell))
        .map(|sh| sh.solid)
        .ok_or("no solid")?;
    let sel = SolidFaces::of(body, solid).map_err(|e| format!("{e:?}"))?;
    let mut geos: Vec<_> = sel
        .faces()
        .iter()
        .map(|&f| face_geo(body, f, &sel.charts).map(|g| (f, g)))
        .collect::<Result<_, _>>()
        .map_err(|e| format!("{e:?}"))?;
    if reversed {
        geos.reverse();
    }
    let n = geos.len().max(1);
    geos.rotate_left(turn % n);
    let reach = selection_reach(body, sel.faces(), q).map_err(|e| format!("{e:?}"))?;
    let crossings =
        ray_crossings(body, &geos, q, d, reach, band, AIMED_ROWS).map_err(|e| format!("{e:?}"))?;
    Ok(crossings
        .all()
        .into_iter()
        .map(|((from, to), hit, sideless)| (from, to, hit.count(), sideless || hit.mark.is_some()))
        .collect())
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
        // The limit is the one a ray meets at that face, from anywhere:
        // the face's geometry refuses.
        assert!(
            matches!(face_geo(&body, spline, &sel.charts), Err(e) if e == want),
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
