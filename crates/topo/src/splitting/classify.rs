//! Reduction step (ch. 14 `splitgenerate`, re-derived): the operand
//! gate (F5 → the C5 table, M5 PR 5), the cached vertex-vs-plane
//! trilean sweep (F6), and crossing insertion through the certified
//! `split_edge` lane — with the conic crossing-root lane for
//! circle/ellipse carriers.

use geom_brep::recourse::{Reading, RefusedArm, SizedDecision, SizedPass, StoredDefinite};
use geom_core::{Band, Decide, Margin, Point3, Sign, Tol, UnitVec3};
use slotmap::SecondaryMap;

use super::{PlaneSide, SplitPlane, SplitReduceError};
use crate::body::Body;
use crate::boolean::boxes::{BoxFrame, Span};
use crate::entity::{EdgeKey, FaceKey, VertexKey};
use crate::null::CurveGeom;
use crate::validate::decide;

/// **The operand gate, scoped to what the plane can reach** (C12.1:
/// the gate retires per arm, never wholesale).
///
/// A face passes if the split pipeline executes its `(kind × plane)`
/// arm — `Plane`, `Cylinder` and `Cone`. A face of any other kind
/// (`Sphere`, `Torus`, `Nurbs`, `Approx`) refuses typed only when the
/// plane MAY meet it: when its padded reach along the plane's normal
/// ([`gate_face_reach`]) is not definitely on one side of the plane
/// ([`reach_clears`]). Behind a reach that clears, the face has no
/// vertex on or across the plane, no edge crossing it and no section
/// through it, so every later stage carries it through whole. The
/// reach is the face's STORED locus, so an `Approx` face clears by its
/// fit, which is the geometry there is to cut; one the plane may meet
/// refuses by kind, since an arm executed against the fit would cut
/// the approximation, not the surface the modeller described.
///
/// The reach is read in the plane's own frame
/// ([`crate::boolean::boxes::BoxFrame::aimed`]), never in the world's
/// axes, so whether a cut clears a face is a fact about the two of
/// them and not about how the body is turned.
///
/// Edge carriers: `Line`, `Circle` and `Ellipse` pass. A `Spiric` or
/// `Nurbs` carrier refuses typed only when the plane may meet it
/// ([`edge_clears`]). A face whose surface key does not resolve is a
/// corrupt body and refuses as one; null scaffolding refuses as ever.
pub(super) fn gate_operand<T: Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    band: Band,
) -> Result<(), SplitReduceError> {
    let frame = BoxFrame::aimed(plane.normal);
    for (face_key, face) in body.faces() {
        let Some(surface) = body.get_surface(face.surface) else {
            return Err(SplitReduceError::Euler(
                crate::euler::EulerOpError::StaleGeometry {
                    key: crate::entity::GeomRef::Surface(face.surface),
                },
            ));
        };
        let kind = surface.kind();
        match kind {
            geom::SurfaceKind::Plane | geom::SurfaceKind::Cylinder | geom::SurfaceKind::Cone => {}
            geom::SurfaceKind::Sphere
            | geom::SurfaceKind::Torus
            | geom::SurfaceKind::Nurbs
            | geom::SurfaceKind::Approx => {
                if !reach_clears(gate_face_reach(body, face_key, band, &frame), plane, band) {
                    return Err(SplitReduceError::CurvedBooleanUnsupported {
                        face: face_key,
                        kind,
                    });
                }
            }
        }
    }
    for (edge_key, edge) in body.edges() {
        match body.get_curve_geom(edge.curve) {
            Some(CurveGeom::Certified(curve)) => match curve.carrier() {
                geom::Curve3::Line { .. }
                | geom::Curve3::Circle { .. }
                | geom::Curve3::Ellipse { .. } => {}
                // No crossing-root arm reads a spiric or a spline.
                geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => {
                    if !edge_clears(body, edge_key, plane, band) {
                        return Err(SplitReduceError::CurvedEdgeUnsupported { edge: edge_key });
                    }
                }
            },
            _ => return Err(SplitReduceError::ScaffoldingOperand { edge: edge_key }),
        }
    }
    Ok(())
}

/// Whether the plane certainly misses an edge: behind its own reach
/// (`census::edge_reach_in`) or behind the reach of either face it
/// bounds, since an edge lies on both its faces. A spline edge has no
/// sound box of its own (`EdgeBoxRule::NoSoundBox`), so its faces'
/// reaches are what clear it.
fn edge_clears<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    plane: &SplitPlane<T>,
    band: Band,
) -> bool {
    let frame = &BoxFrame::aimed(plane.normal);
    let bounding = body
        .get_edge(edge)
        .map(|e| [e.he_plus, e.he_minus].map(|he| body.face_of_half_edge(he)));
    reach_clears(crate::census::edge_reach_in(body, edge, frame), plane, band)
        || bounding
            .into_iter()
            .flatten()
            .flatten()
            .any(|face| reach_clears(gate_face_reach(body, face, band, frame), plane, band))
}

/// K name: a sphere face's polar axis, read as a unit direction for
/// its latitude zone.
const SPLIT_GATE_SPHERE_AXIS: &str = "split_gate_sphere_axis";

/// K name: a torus face's ring convention `R > r`, the major radius
/// less the minor (metres), decided before its chart rectangle is read
/// in closed form.
const SPLIT_GATE_TORUS_RING: &str = "split_gate_torus_ring";

/// A face's reach for the gate, in `frame`: `census::face_reach_in`
/// (the boolean's `FaceBoxRule` at this body's scalar), except where
/// the face's own patch of its carrier is certified and has a closed
/// form along a direction — a sphere face's latitude zone
/// ([`sphere_zone_reach`]) and a torus face's chart rectangle
/// ([`torus_window_reach`]). The rule's box for a sphere is the whole
/// ball, which a plane through any part of it meets, so a cap on a
/// cylinder would refuse every cut of the cylinder; its box for a torus
/// samples the rectangle and pays a subdivision charge that reaches
/// thousandths of the radii.
fn gate_face_reach<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
    frame: &BoxFrame<T>,
) -> Option<(Point3<T>, Point3<T>)> {
    let rule = crate::census::face_reach_in(body, face, band, frame)?;
    let f = body.get_face(face)?;
    let patch = match body.get_surface(f.surface)? {
        surface @ geom::Surface::Sphere { .. } => {
            sphere_zone_reach(body, face, f, surface, band, frame)
        }
        geom::Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => torus_window_reach(
            body,
            face,
            (*center, *axis, *u_ref),
            (*major_radius, *minor_radius),
            band,
            frame,
        ),
        _ => None,
    };
    Some(patch.unwrap_or(rule))
}

/// A sphere face's latitude zone, in `frame`: [`zone_extent`] per
/// coordinate over the window `solid_contain::sphere_chart_trim` pins.
///
/// The trim reads the window off the boundary, and the boundary of a
/// rectangle is also the boundary of its complement. The zone is
/// therefore taken only when the side the boundary traversal encodes
/// (`boundary_material_sign`, the iso-rectangle reading tier 3's curved
/// sense check runs) agrees with the face's `sense`, which places the
/// face on the rectangle's side of its rims. A face whose side is
/// unencoded, refused, or in disagreement, one outside the trim's
/// class, and one whose trim escalates all answer `None`, and keep the
/// ball.
fn sphere_zone_reach<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    f: &crate::entity::Face,
    surface: &geom::Surface<T>,
    band: Band,
    frame: &BoxFrame<T>,
) -> Option<(Point3<T>, Point3<T>)> {
    let geom::Surface::Sphere {
        center,
        radius,
        axis,
        ..
    } = surface
    else {
        return None;
    };
    let side_certified = crate::props::loop_edges(body, f.outer)
        .ok()
        .and_then(|(outer, _)| geom_brep::props::boundary_material_sign(surface, &outer, band).ok())
        .is_some_and(|side| {
            side == geom_brep::props::MaterialSign::Encoded(if f.sense {
                Sign::Positive
            } else {
                Sign::Negative
            })
        });
    if !side_certified {
        return None;
    }
    let trim =
        crate::boolean::solid_contain::sphere_chart_trim(body, face, *center, *radius, *axis, band)
            .ok()??;
    let unit = UnitVec3::new(*axis, SPLIT_GATE_SPHERE_AXIS, band).ok()?;
    let h = Span {
        lo: trim.south.map_or(T::zero() - *radius, |(h, _)| h),
        hi: trim.north.map_or(*radius, |(h, _)| h),
    };
    let (c, a) = (frame.point(*center), frame.vector(unit.get()));
    let ((xl, xh), (yl, yh), (zl, zh)) = (
        zone_extent(c.x, a.x, h, *radius),
        zone_extent(c.y, a.y, h, *radius),
        zone_extent(c.z, a.z, h, *radius),
    );
    Some((Point3::new(xl, yl, zl), Point3::new(xh, yh, zh)))
}

/// **One coordinate of a sphere's latitude zone, exactly**: the least
/// and greatest `e·p` over the zone of the sphere about `c` of radius
/// `r` whose axial coordinate lies in `h`, where `e` is a unit
/// direction, `c` is `e`'s component of the centre and `a = e·â` its
/// component of the unit polar axis.
///
/// A zone point is `c + h·â + ρ·û` with `û ⊥ â` unit and
/// `ρ = √(r² − h²)`, so `e·p − e·c = h·a + ρ·(e·û)` and `e·û` reaches
/// `±√(1 − a²)` on every parallel. The greatest value is therefore the
/// largest `G(h) = h·a + √(1 − a²)·√(r² − h²)` over `h`, and `G` is
/// concave on `[−r, r]` with its one stationary point at `h = r·a`
/// (where it is `r`, the ball's own support). The largest value over a
/// window is `G` at that point clamped into the window; the least is
/// the same reading of `−e`. The clamp is branch-free, so no sign is
/// decided here, and an `h` read a rounding off its true value moves
/// `G` by as little, since `G` is continuous.
fn zone_extent<T: Decide>(c: T, a: T, h: Span<T>, r: T) -> (T, T) {
    let s = (T::one() - a.powi(2)).max(T::zero()).sqrt();
    let most = |a: T| {
        let at = (r * a).max(h.lo).min(h.hi);
        at * a + s * (r.powi(2) - at.powi(2)).max(T::zero()).sqrt()
    };
    (c - most(T::zero() - a), c + most(a))
}

/// A torus face's chart rectangle, in `frame`: [`torus_rect_extent`]
/// per coordinate over the window the boundary's stored certified
/// pcurves pin (`boxes::torus_chart_window`, the walk the rule's own
/// box reads), when the ring convention `R > r` is decided. `None`
/// when no window reads or the convention is not decided, and the rule's
/// box stands.
fn torus_window_reach<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    (center, axis, u_ref): (Point3<T>, geom_core::Vec3<T>, geom_core::Vec3<T>),
    (major, minor): (T, T),
    band: Band,
    frame: &BoxFrame<T>,
) -> Option<(Point3<T>, Point3<T>)> {
    if !matches!(
        decide(SPLIT_GATE_TORUS_RING, Margin::of(major - minor), band),
        Ok(Sign::Positive)
    ) {
        return None;
    }
    let window = crate::boolean::boxes::torus_chart_window(
        &crate::boolean::boxes::face_window_steps(body, face)?,
        major,
        minor,
    )?;
    let (c, ax, ur, vr) = (
        frame.point(center),
        frame.vector(axis),
        frame.vector(u_ref),
        frame.vector(axis.cross(u_ref)),
    );
    let at = |c: T, p: T, q: T, a: T| torus_rect_extent(c, (p, q, a), (major, minor), window);
    let ((xl, xh), (yl, yh), (zl, zh)) = (
        at(c.x, ur.x, vr.x, ax.x),
        at(c.y, ur.y, vr.y, ax.y),
        at(c.z, ur.z, vr.z, ax.z),
    );
    Some((Point3::new(xl, yl, zl), Point3::new(xh, yh, zh)))
}

/// **One coordinate of a ring torus's chart rectangle, exactly**: the
/// least and greatest `e·S(u, v)` over `u ∈ U`, `v ∈ V`, where `e` is
/// a unit direction, `c` its component of the centre and `(p, q, a)`
/// its components of `u_ref`, `axis × u_ref` and the axis.
///
/// `e·S − c = (R + r·cos v)·(p·cos u + q·sin u) + r·a·sin v`. The
/// factor `R + r·cos v` is positive on a ring torus (`R > r`), so for
/// every `v` the best `u` is the one that maximizes
/// `p·cos u + q·sin u = A·cos(u − u*)` over `U`, whatever `v` is: call
/// that greatest value `M` ([`most_cos`]). What is left is
/// `R·M + r·(M·cos v + a·sin v) = R·M + r·B·cos(v − v*)` with
/// `B = √(M² + a²)`, maximized over `V` the same way. The least value is
/// the same reading of `−e`. Every step is an equality, so the extent
/// is the rectangle's own and turns with it.
fn torus_rect_extent<T: Decide>(
    c: T,
    (p, q, a): (T, T, T),
    (major, minor): (T, T),
    (u, v): crate::boolean::boxes::TorusWindowPair<T>,
) -> (T, T) {
    let most = |p: T, q: T, a: T| {
        let m = (p.powi(2) + q.powi(2)).sqrt() * most_cos(q.atan2(p), u);
        major * m + minor * (m.powi(2) + a.powi(2)).sqrt() * most_cos(a.atan2(m), v)
    };
    let neg = |x: T| T::zero() - x;
    (c - most(neg(p), neg(q), neg(a)), c + most(p, q, a))
}

/// The greatest `cos(t − t*)` over `t ∈ w`: one when `t*` lies in the
/// window modulo a turn, otherwise the better of the two ends, since a
/// window that misses the crest is monotone between them. The
/// membership is read branch-free (`select_le_zero` on the crest's
/// offset into the window less its width), and a crest a rounding off
/// an end gives either reading to within that rounding.
fn most_cos<T: Decide>(crest: T, w: Span<T>) -> T {
    let into = (crest - w.lo).reduce_periodic(T::tau());
    let ends = (w.lo - crest).cos().max((w.hi - crest).cos());
    (into - (w.hi - w.lo)).select_le_zero(T::one(), ends)
}

/// K name: an unarmed entity's padded reach, its distance from the
/// split plane along the plane's normal (metres).
const SPLIT_GATE_BOX_SIDE: &str = "split_gate_box_side";

/// Whether the plane certainly misses a reach read in the plane's
/// frame ([`BoxFrame::aimed`]), whose first coordinate is the
/// component along the plane's normal: the reach's gap from the
/// plane's own offset along that normal, less the boolean sweep's pad,
/// decided definitely positive. No reach (an entity the reach lane
/// cannot bound) answers `false`.
fn reach_clears<T: Decide>(
    reach: Option<(Point3<T>, Point3<T>)>,
    plane: &SplitPlane<T>,
    band: Band,
) -> bool {
    let Some((lo, hi)) = reach else {
        return false;
    };
    let pad = T::from_f64(crate::boolean::boxes::sweep_pad(band));
    let n = plane.normal.get();
    let at = n.x * plane.origin.x + n.y * plane.origin.y + n.z * plane.origin.z;
    let gap = (lo.x - at).max(at - hi.x) - pad;
    matches!(
        decide(SPLIT_GATE_BOX_SIDE, Margin::of(gap), band),
        Ok(Sign::Positive)
    )
}

/// F6: classify every vertex against the plane through the
/// `split_vertex_side` trilean (margin = signed distance in meters,
/// linear band), caching the verdict per vertex — one predicate site,
/// one evaluation per vertex (the book recomputes per incident edge).
/// In-band ⇒ typed [`SplitReduceError::SliverVertex`], never a snap.
pub(super) fn classify_vertices<T: Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    band: Band,
) -> Result<(SecondaryMap<VertexKey, PlaneSide>, Vec<VertexKey>), SplitReduceError> {
    let mut sides = SecondaryMap::new();
    let mut on_vertices = Vec::new();
    for (vertex_key, vertex) in body.vertices() {
        let p = *body
            .get_point(vertex.point)
            .ok_or(SplitReduceError::CorruptOperand { vertex: vertex_key })?;
        let margin = Margin::of(crate::sector_shape::plane_offset(
            plane.origin,
            plane.normal.get(),
            p,
        ));
        let side = match decide("split_vertex_side", margin, band) {
            Ok(Sign::Negative) => PlaneSide::Below,
            Ok(Sign::Positive) => PlaneSide::Above,
            Ok(Sign::Zero) => {
                on_vertices.push(vertex_key);
                PlaneSide::On
            }
            Err(diag) => {
                return Err(SplitReduceError::SliverVertex {
                    vertex: vertex_key,
                    diag,
                });
            }
        };
        sides.insert(vertex_key, side);
    }
    Ok((sides, on_vertices))
}

/// The plane-crossing roots of a **conic** carrier over its span
/// (M1 fix, M5 PR 5 review): the signed distance along the carrier is
/// the sinusoid `d(θ) = D + R·cos(θ − φ)` with `D = (center − q)·n̂`,
/// `R·cos φ = s_u·(û·n̂)`, `R·sin φ = s_v·(v̂·n̂)` (`s_u/s_v` the
/// semi-axes — `r/r` for a circle). Unlike a line, a conic edge can
/// cross the plane an EVEN number of times between same-side
/// endpoints (the belly case) or once beyond an ON endpoint — so
/// crossing detection is **root-based, endpoint-verdict-free**:
///
/// 1. `split_conic_belly_graze` — margin `R − |D|` (meters): Negative
///    ⇒ the carrier never meets the split plane — no crossing;
///    Positive ⇒ two distinct roots `φ ± acos(−D/R)`; Zero ⇒ the
///    plane grazes the carrier's extremum — ONE (double) root, whose
///    insertion (if in-span) leaves a same-side ON contact for the
///    sector classification (the established graze net); in-band ⇒
///    typed escalation (F6).
/// 2. Each root, translated into `[t₀, t₀ + τ)`, is classified
///    against the span by `split_conic_crossing_root` — the two
///    margins `(t − t₀)·meter` and `(t₁ − t)·meter` (meters at the
///    conservative minor-semi-axis meter): both Positive ⇒ a genuine
///    interior crossing (returned for insertion); any Zero ⇒ the root
///    sits at an existing endpoint vertex (the ON-endpoint belly case
///    — the vertex sweep already recorded it, nothing to insert); any
///    Negative ⇒ outside the span; in-band ⇒ typed escalation (the
///    crossing grazes an edge end — an ill-conditioned operand/plane
///    pair).
/// 3. Two interior roots are ordered ascending through
///    `split_conic_root_order` (never a raw comparison); ε-close
///    roots collapse to one insertion (Zero) or escalate (in-band).
///
/// Downstream, `split_edge`'s own interiority trilean and child
/// certification re-verify every insertion — this lane proposes,
/// never silently commits.
fn conic_crossing_roots<T: Decide>(
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    plane: &SplitPlane<T>,
    band: Band,
) -> Result<ConicPlaneMeet<T>, ()> {
    conic_plane_crossing_roots(carrier, t0, t1, plane.origin, plane.normal.get(), band)
}

/// What a conic carrier's span meets of a plane
/// ([`conic_plane_crossing_roots`]).
#[derive(Debug)]
pub(crate) enum ConicPlaneMeet<T> {
    /// The conic's plane is parallel to the query plane: the carrier
    /// lies wholly at `offset`, its centre's signed distance, so it is
    /// either IN the plane or never meets it. Which is the caller's
    /// decision.
    Parallel { offset: T },
    /// The carrier definitely never meets the plane.
    Miss,
    /// The roots interior to the span, ascending (possibly none), or
    /// the rung no verdict was reached on.
    Roots(Result<Vec<T>, ConicRootFault>),
}

/// A decision on where a crossing lands along its edge, where every
/// definite answer passes and only an undecided one refuses: the conic
/// root lane's and the Boolean's wall-root lane's. Its ending is the one
/// every door that splits an edge at a crossing states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum CrossingDecision {
    /// Whether a crossing lands strictly inside its edge (inside, at an
    /// end, and outside all pass).
    OnEdge,
    /// Which of two crossings on an edge comes first (either order, or
    /// one crossing, all pass).
    Order,
}

impl CrossingDecision {
    /// What the decision decides, as a clause with no colon or dash of
    /// its own.
    #[must_use]
    pub const fn subject(self) -> &'static str {
        match self {
            Self::OnEdge => crate::split::CROSSING_INTERIOR,
            Self::Order => "which of two crossings on an edge comes first",
        }
    }

    /// The decision's table row: a size the user may intend, passing on
    /// every definite sign.
    pub(crate) const fn sized(self) -> SizedDecision {
        let (lever, size) = match self {
            Self::OnEdge => (crate::split::CROSSING_LEVER, crate::split::CROSSING_SIZE),
            Self::Order => (
                "move the geometry so the two crossings on that edge lie clearly apart",
                "distance between the crossings",
            ),
        };
        SizedDecision {
            lever,
            size,
            passes: SizedPass::AnySign,
            stored: StoredDefinite::Lever,
            at_zero: None,
        }
    }

    /// The one ending `diag`'s escalation carries, at the operation that
    /// placed the crossing.
    #[must_use]
    pub(crate) fn ending_of(self, diag: &geom_core::Indeterminate) -> String {
        self.sized()
            .recourse(RefusedArm::Undecided(diag), Reading::Build)
    }
}

/// Which rung of [`conic_plane_crossing_roots`] escalated, with its
/// diagnostics. The first two ask whether the plane coincides with the
/// conic; the other two are [`CrossingDecision`]s.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConicRootFault {
    /// Whether the conic's plane is parallel to the query plane.
    PlaneParallel(geom_core::Indeterminate),
    /// Whether the conic reaches the plane, grazes it, or misses it.
    BellyGraze(geom_core::Indeterminate),
    /// Whether a root lands inside the span, at an end, or outside it.
    CrossingInterior(geom_core::Indeterminate),
    /// Which of two interior roots comes first.
    RootOrder(geom_core::Indeterminate),
}

impl ConicRootFault {
    /// The escalation's diagnostics, whichever rung raised it.
    #[must_use]
    pub fn diag(self) -> geom_core::Indeterminate {
        match self {
            Self::PlaneParallel(diag)
            | Self::BellyGraze(diag)
            | Self::CrossingInterior(diag)
            | Self::RootOrder(diag) => diag,
        }
    }

    /// The crossing decision the rung decides, or `None` for the two
    /// rungs that ask whether the plane coincides with the conic: every
    /// door that meets this fault routes it through here.
    #[must_use]
    pub fn decision(self) -> Option<CrossingDecision> {
        match self {
            Self::PlaneParallel(_) | Self::BellyGraze(_) => None,
            Self::CrossingInterior(_) => Some(CrossingDecision::OnEdge),
            Self::RootOrder(_) => Some(CrossingDecision::Order),
        }
    }

    /// What the rung decides, as a clause with no colon or dash of its
    /// own.
    #[must_use]
    pub fn subject(self) -> &'static str {
        match self {
            Self::PlaneParallel(_) => {
                "whether a curved edge's plane is parallel to the plane that cuts it"
            }
            Self::BellyGraze(_) => "whether a plane cuts a curved edge, grazes it or misses it",
            Self::CrossingInterior(_) => CrossingDecision::OnEdge.subject(),
            Self::RootOrder(_) => CrossingDecision::Order.subject(),
        }
    }
}

/// The plane-form core of [`conic_crossing_roots`], shared with the
/// boolean reduction sweep (M5 PR 9 — the same C12.1 machinery, the
/// same named trileans, against ANY plane rather than the split
/// lane's one). Semantics and return shape documented above.
pub(crate) fn conic_plane_crossing_roots<T: Decide>(
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    plane_origin: geom_core::Point3<T>,
    plane_normal: geom_core::Vec3<T>,
    band: Band,
) -> Result<ConicPlaneMeet<T>, ()> {
    let (center, axis, u_ref, s_u, s_v) = match *carrier {
        geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => (center, axis, u_ref, radius, radius),
        geom::Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        } => (center, axis, u_ref, major, minor),
        geom::Curve3::Line { .. } | geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => {
            return Err(());
        }
    };
    let v_ref = axis.cross(u_ref);
    let d0 = (center - plane_origin).dot(plane_normal);
    let a = u_ref.dot(plane_normal) * s_u;
    let b = v_ref.dot(plane_normal) * s_v;
    // powi, NEVER a*a: both amplitudes straddle zero on near-parallel
    // frames (a rim circle against a perpendicular side plane), and a
    // plain interval product's spurious negative low end makes the
    // sqrt refuse — the M2 interval-square bug class, found live here when
    // the boolean's IDEALIZED sweep (M5 PR 9) first drove this lane
    // over distant conic×plane pairs under the Interval scalar (the
    // realized lane's boxes never examine them, so only the brute
    // path escalated: a strategy divergence, the exact thing the
    // differential suite exists to catch).
    let r = (a.powi(2) + b.powi(2)).sqrt();
    // 0. The PARALLEL-frame gate: a conic whose plane is parallel to
    // the query plane (both amplitudes zero) either never meets it or
    // lies wholly IN it, and has no roots to find; `offset` tells the
    // two apart. Without the gate the in-plane case reaches the graze
    // arm with a 0/0 phase. The in-band twin escalates (F6).
    match decide("split_conic_plane_parallel", Margin::of(r), band) {
        Ok(Sign::Zero) => return Ok(ConicPlaneMeet::Parallel { offset: d0 }),
        Ok(Sign::Positive | Sign::Negative) => {}
        Err(diag) => {
            return Ok(ConicPlaneMeet::Roots(Err(ConicRootFault::PlaneParallel(
                diag,
            ))));
        }
    }
    // 1. Does the sinusoid reach zero at all — and how many roots?
    let both_roots = match decide("split_conic_belly_graze", Margin::of(r - d0.abs()), band) {
        Ok(Sign::Negative) => return Ok(ConicPlaneMeet::Miss),
        Ok(Sign::Positive) => true,
        // Graze: the double root, processed once (processing both
        // would split twice at coincident parameters and escalate on
        // the second interiority check — same refusal, worse site).
        Ok(Sign::Zero) => false,
        Err(diag) => return Ok(ConicPlaneMeet::Roots(Err(ConicRootFault::BellyGraze(diag)))),
    };
    // The sinusoid's phase, branch-stabilized (M5 S13): `atan2`'s cut
    // sits on the negative-`a` axis, and an interval `b` that touches
    // zero there (a revolve-built frame's honest trig slop) explodes
    // the enclosure to a full period even though the ROOT SET —
    // everything downstream consumes phi mod τ — is unchanged. On the
    // definitely-negative-`a` frame the same phase is computed as
    // `atan2(−b, −a) + π` (identical roots mod τ, cut now on the
    // benign axis). The frame trilean is a computation choice between
    // two mathematically identical formulas, so its degenerate and
    // in-band arms keep the direct formula (a deterministic tie-break,
    // D9 — near a ≈ 0 the cut is far from both).
    let phi = match decide("split_conic_phase_frame", Margin::of(a), band) {
        Ok(Sign::Negative) => (T::zero() - b).atan2(T::zero() - a) + T::pi(),
        Ok(Sign::Positive | Sign::Zero) | Err(_) => b.atan2(a),
    };
    // Clamped acos (rounding can push the ratio a hair outside ±1 at
    // the graze boundary; min/max are Real lattice ops).
    let arg = ((T::zero() - d0) / r)
        .min(T::one())
        .max(T::zero() - T::one());
    let delta = arg.acos();
    let tau = T::tau();
    // The conservative meter (radians → meters): the smaller semi-axis
    // MAGNITUDE (the stored semi-axes carry no order, `geom_brep::Conic`;
    // `s_v` in the ordinary order). An INF bound on the speed by being
    // the smaller principal rate, which is what the interiority claim
    // below needs — a root this meter proves clear of an endpoint is
    // clear of it in metres.
    let meter = geom_core::InfSpeed::new(s_u.abs().min(s_v.abs()));
    let mut roots: Vec<T> = Vec::with_capacity(2);
    let candidates: [Option<T>; 2] = if both_roots {
        [Some(phi + delta), Some(phi - delta)]
    } else {
        [Some(phi + delta), None]
    };
    let half = T::from_f64(0.5);
    let mid = (t0 + t1) * half;
    // Per-candidate interiority under TWO reduction anchors (M5 S13).
    // The verdict is about `c mod τ` against `[t₀, t₁]`, but any single
    // periodic-reduction anchor has one degenerate point where the
    // Interval floor straddles an integer and the enclosure explodes to
    // a full period. The two anchors are the two windows: the midpoint
    // one is `reduce_periodic_centred` about `mid`, the fallback is
    // `reduce_periodic` about `t₀`. Their bad points are DISTINCT for
    // every sub-period span, which is what makes the retry a different
    // representation rather than a second try at the same one — they
    // sit **(τ − span)/2 apart** on the circle, half the span's
    // COMPLEMENT (the same quantity `chord_join`'s window recentre
    // states for itself). Not half a period: that is the span → 0
    // limit, and the separation SHRINKS as the span grows, reaching
    // zero only at a full period. So the retry has room exactly when
    // the span is well under τ, and is degenerate exactly where the
    // sentence below says it is.
    // Anchored at the span MIDPOINT the bad point is the
    // midpoint's antipode (a definitely-EXTERIOR root — the re-run of a
    // split fragment meets the full circle's opposite intersection
    // exactly there), anchored at t₀ it is t₀ itself (a
    // definitely-ON-endpoint root — the fragment's own start vertex,
    // already swept). The two bad points coincide only on a full-period
    // span (a closed edge's lone vertex on the plane), so an
    // indeterminate verdict under one anchor retries the other — the
    // SAME margin in a non-degenerate representation, never a second
    // tolerance — and only a double failure escalates (F6).
    let verdict_at = |t: T| -> Result<(bool, T), geom_core::Indeterminate> {
        for margin in [
            Margin::metered(t - t0, meter),
            Margin::metered(t1 - t, meter),
        ] {
            match decide("split_conic_crossing_root", margin, band) {
                Ok(Sign::Positive) => {}
                // At an endpoint vertex (already swept) or outside
                // the span: nothing to insert for this root.
                Ok(Sign::Zero | Sign::Negative) => return Ok((false, t)),
                Err(diag) => return Err(diag),
            }
        }
        Ok((true, t))
    };
    for c in candidates.into_iter().flatten() {
        let centred = mid + (c - mid).reduce_periodic_centred(tau);
        let (interior, t) = match verdict_at(centred) {
            Ok(v) => v,
            Err(first) => {
                let anchored = t0 + (c - t0).reduce_periodic(tau);
                match verdict_at(anchored) {
                    Ok(v) => v,
                    Err(_) => {
                        return Ok(ConicPlaneMeet::Roots(Err(
                            ConicRootFault::CrossingInterior(first),
                        )));
                    }
                }
            }
        };
        if interior {
            roots.push(t);
        }
    }
    // Ascending insertion order, decided through the trilean door
    // (never a raw scalar comparison).
    if roots.len() == 2 {
        match decide(
            "split_conic_root_order",
            Margin::metered(roots[1] - roots[0], meter),
            band,
        ) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Negative) => roots.swap(0, 1),
            // Coincident-but-both-interior roots: a graze the graze
            // margin called Positive — insert once.
            Ok(Sign::Zero) => {
                roots.truncate(1);
            }
            Err(diag) => return Ok(ConicPlaneMeet::Roots(Err(ConicRootFault::RootOrder(diag)))),
        }
    }
    Ok(ConicPlaneMeet::Roots(Ok(roots)))
}

/// Crossing insertion (M1 fix — even-crossing completeness):
///
/// - **Line carriers** keep the M3 rule BIT-IDENTICALLY: a crossing
///   iff the cached endpoint verdicts are strictly opposite (complete
///   for lines — the affine distance has one root, and an ON endpoint
///   IS that root), split at the exact interpolation
///   `t = t₀ + (t₁ − t₀)·d₁/(d₁ − d₂)` (comparison-free; safe: the
///   strict-opposite-signs decision bounds the denominator away from
///   zero).
/// - **Conic carriers** use the root-based lane
///   ([`conic_crossing_roots`]), INDEPENDENT of endpoint verdicts:
///   same-side endpoints with a belly crossing the plane twice get
///   BOTH crossing vertices; an ON endpoint with one interior
///   crossing gets it; grazes land as single ON contacts for the
///   sector classification. Two roots split the parent then its
///   trailing child (ascending — the second root lives on the child's
///   span).
/// - **Spiric and spline carriers** have no crossing lane. One passes
///   uncut only where [`edge_clears`] certifies its whole locus on one
///   side of the plane, the operand gate's own test; any other refuses
///   [`SplitReduceError::CurvedEdgeUnsupported`]. Same-side endpoints
///   are not enough: a spline's belly can cross between them.
///
/// Both crossing lanes are certified by `split_edge` itself (the
/// `split_edge_param_interior` trilean + full child re-certification —
/// the honest lane the raw book formula lacks). New vertices are ON
/// **by construction** (declared coincidence): their verdicts are
/// cached without re-measuring.
pub(super) fn insert_crossings<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    plane: &SplitPlane<T>,
    sides: &mut SecondaryMap<VertexKey, PlaneSide>,
    on_vertices: &mut Vec<VertexKey>,
    tol: Tol,
) -> Result<(), SplitReduceError> {
    let band = geom_core::Band::linear(tol)?;
    // Snapshot: splitting adds edges; only operand edges can cross (a
    // split child's remaining crossing is handled through the parent's
    // precomputed root list below).
    let snapshot: Vec<_> = body.edges().map(|(k, e)| (k, e.clone())).collect();
    for (edge_key, edge) in snapshot {
        let start = |body: &Body<T>, he| {
            body.get_half_edge(he)
                .map(|h: &crate::entity::HalfEdge| h.start)
        };
        let u = start(body, edge.he_plus).ok_or(SplitReduceError::Euler(
            crate::euler::EulerOpError::StaleKey {
                key: crate::entity::EntityId::Edge(edge_key),
            },
        ))?;
        let v = start(body, edge.he_minus).ok_or(SplitReduceError::Euler(
            crate::euler::EulerOpError::StaleKey {
                key: crate::entity::EntityId::Edge(edge_key),
            },
        ))?;
        let curve = match body.get_curve_geom(edge.curve) {
            Some(CurveGeom::Certified(c)) => c.clone(),
            _ => return Err(SplitReduceError::ScaffoldingOperand { edge: edge_key }),
        };
        let (t0, t1) = curve.params();
        let roots: Vec<T> = match conic_crossing_roots(curve.carrier(), t0, t1, plane, band) {
            // Parallel either way: no root to insert, and a conic in
            // the plane has its endpoints ON through the vertex sides.
            Ok(ConicPlaneMeet::Parallel { .. } | ConicPlaneMeet::Miss) => continue,
            Ok(ConicPlaneMeet::Roots(Ok(roots))) => roots,
            Ok(ConicPlaneMeet::Roots(Err(fault))) => {
                return Err(SplitReduceError::CrossingEscalated {
                    edge: edge_key,
                    fault,
                });
            }
            // A spiric or spline carrier has no crossing lane: it passes
            // only behind a box the plane cannot meet, read here as at
            // the gate, so its whole locus is on one side.
            Err(()) if !matches!(curve.carrier(), geom::Curve3::Line { .. }) => {
                if edge_clears(body, edge_key, plane, band) {
                    continue;
                }
                return Err(SplitReduceError::CurvedEdgeUnsupported { edge: edge_key });
            }
            Err(()) => {
                // Line lane: endpoint verdicts strictly opposite ⇒ one
                // interpolated root.
                let crossing = matches!(
                    (sides[u], sides[v]),
                    (PlaneSide::Above, PlaneSide::Below) | (PlaneSide::Below, PlaneSide::Above)
                );
                if !crossing {
                    continue;
                }
                let dist = |body: &Body<T>, vk: VertexKey| -> Option<T> {
                    let p = *body.get_point(body.get_vertex(vk)?.point)?;
                    Some((p - plane.origin).dot(plane.normal.get()))
                };
                let (Some(d1), Some(d2)) = (dist(body, u), dist(body, v)) else {
                    return Err(SplitReduceError::CorruptOperand { vertex: u });
                };
                vec![t0 + (t1 - t0) * (d1 / (d1 - d2))]
            }
        };
        // Insert ascending: the first split leaves the parent with
        // [t₀, tₐ] and mints the trailing child [tₐ, t₁]; the second
        // root (if any) lives on that CHILD.
        let mut target = edge_key;
        for t in roots {
            // Any refusal here (in practice the certification lane's
            // strict-row ResidualExceeded) gets the crossing site
            // attached; the typed Euler error stays nested whole.
            let created = body.split_edge(target, t, tol).map_err(|source| {
                SplitReduceError::CrossingInsertion {
                    edge: target,
                    endpoints: (u, v),
                    source,
                }
            })?;
            sides.insert(created.vertex, PlaneSide::On);
            on_vertices.push(created.vertex);
            target = created.new_edge;
        }
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! DIRECT trilean rows for the conic crossing lane (M5 PR 5 fix
    //! pass, review M3): `split_conic_belly_graze` and
    //! `split_conic_crossing_root` each get their definite /
    //! exactly-degenerate / in-band arms against a pure band (the
    //! geom-core test discipline: never `Band::linear` in a lib test).

    use core::f64::consts::PI;

    use geom::Curve3;
    use geom_core::{Band, Point3, Vec3};

    use super::{ConicPlaneMeet, conic_crossing_roots, conic_plane_crossing_roots};
    use crate::splitting::SplitPlane;

    /// **A crossing's gap from an end is metered at the smaller semi-axis
    /// MAGNITUDE, in either stored order.** An ellipse stored swapped —
    /// `major = 0.01` along `u_ref`, `minor = 10` — runs at 0.01 m/rad at
    /// its `θ = π/2` vertex. A plane crosses it `δ = 5e-8` rad past that
    /// vertex, an arc of 5e-10 m: inside the zero band, so the crossing
    /// is the span's own end, not an interior root. Metered at the
    /// stored `minor` (10 m/rad) the gap read 5e-7 m and the root was
    /// certified interior, a split 5e-10 m from a vertex.
    #[test]
    fn a_crossing_at_an_end_is_metered_at_the_smaller_semi_axis() {
        let e = Curve3::Ellipse {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            major: 0.01,
            minor: 10.0,
            u_ref: Vec3::unit_x(),
        };
        let t0 = core::f64::consts::FRAC_PI_2;
        let delta = 5e-8;
        let x = 0.01 * (t0 + delta).cos();
        let got = roots_of(conic_plane_crossing_roots(
            &e,
            t0,
            t0 + 0.3,
            Point3::new(x, 0.0, 0.0),
            Vec3::unit_x(),
            band(),
        ))
        .unwrap();
        assert!(
            got.is_empty(),
            "a root at the span's own end certified interior: {got:?}"
        );
    }

    /// The roots arm's payload; any other arm fails the row.
    fn roots_of<T: core::fmt::Debug>(
        m: Result<ConicPlaneMeet<T>, ()>,
    ) -> Result<Vec<T>, geom_core::Indeterminate> {
        match m {
            Ok(ConicPlaneMeet::Roots(r)) => r.map_err(super::ConicRootFault::diag),
            other => panic!("expected the roots arm, got {other:?}"),
        }
    }

    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// Unit circle in the xy-plane about the origin (meter = 1, so
    /// parameter margins ARE meters).
    fn circle() -> Curve3<f64> {
        Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        }
    }

    fn plane_y(c: f64) -> SplitPlane<f64> {
        crate::test_support::split_plane(
            Point3::new(0.0, c, 0.0),
            Vec3::unit_y(),
            geom_core::Tol::witness(),
        )
    }

    /// `split_conic_belly_graze`, all three arms: definitely-secant
    /// (two roots), definitely-missing (no roots), exactly-tangent
    /// (one graze root), and in-band (typed escalation).
    #[test]
    fn belly_graze_trio() {
        let c = circle();
        // Secant (margin R − |D| = 1, definite): the two roots of
        // sin θ = 0.5 land in the span, ascending.
        let roots = roots_of(conic_crossing_roots(&c, 0.1, 6.0, &plane_y(0.5), band())).unwrap();
        assert_eq!(roots.len(), 2);
        assert!((roots[0] - core::f64::consts::FRAC_PI_6).abs() < 1e-12);
        assert!((roots[1] - (core::f64::consts::PI - core::f64::consts::FRAC_PI_6)).abs() < 1e-12);
        // Missing (margin −1, definite): no crossing at all.
        assert!(matches!(
            conic_crossing_roots(&c, 0.1, 6.0, &plane_y(2.0), band()),
            Ok(ConicPlaneMeet::Miss)
        ));
        // Exactly tangent (margin 0): ONE graze root at π/2.
        let roots = roots_of(conic_crossing_roots(&c, 0.1, 6.0, &plane_y(1.0), band())).unwrap();
        assert_eq!(roots.len(), 1);
        assert!((roots[0] - core::f64::consts::FRAC_PI_2).abs() < 1e-4);
        // In-band (margin −3ε): typed escalation, named.
        let diag = roots_of(conic_crossing_roots(
            &c,
            0.1,
            6.0,
            &plane_y(1.0 + 3e-9),
            band(),
        ))
        .unwrap_err();
        assert_eq!(diag.predicate, Some("split_conic_belly_graze"));
    }

    /// `split_conic_crossing_root`, all three arms: definitely
    /// interior (returned), exactly at an endpoint (skipped — the
    /// vertex sweep owns it), and in-band of an endpoint (typed
    /// escalation).
    #[test]
    fn crossing_root_trio() {
        let c = circle();
        // Roots of sin θ = 0 are θ ∈ {0, π}: with span [0, 2] the θ = 0
        // root sits EXACTLY at the endpoint (skipped, Zero arm) and the
        // θ = π root is definitely interior (returned).
        let roots = roots_of(conic_crossing_roots(
            &c,
            0.0,
            2.0 + 2.0,
            &plane_y(0.0),
            band(),
        ))
        .unwrap();
        assert_eq!(roots.len(), 1);
        assert!((roots[0] - core::f64::consts::PI).abs() < 1e-12);
        // Both roots definitely interior: span (−1, 4).
        let roots = roots_of(conic_crossing_roots(&c, -1.0, 4.0, &plane_y(0.0), band())).unwrap();
        assert_eq!(roots.len(), 2);
        assert!(roots[0].abs() < 1e-12 || (roots[0] - core::f64::consts::PI).abs() < 1e-12);
        // In-band: a root 5e-9 (meters, meter = r = 1) inside the
        // span end — typed escalation, named.
        let diag =
            roots_of(conic_crossing_roots(&c, -5e-9, 2.0, &plane_y(0.0), band())).unwrap_err();
        assert_eq!(diag.predicate, Some("split_conic_crossing_root"));
    }

    /// `split_conic_plane_parallel`'s `Zero` arm hands back the conic's
    /// offset, so a caller can tell a circle IN the plane from one in a
    /// parallel plane; a non-parallel miss is `Miss`, never this arm.
    #[test]
    fn a_parallel_frame_reports_its_offset() {
        let c = circle();
        let plane_z = |z: f64| {
            crate::test_support::split_plane(
                Point3::new(0.3, -0.2, z),
                Vec3::unit_z(),
                geom_core::Tol::witness(),
            )
        };
        for (z, want) in [(0.0, 0.0), (2.0, -2.0)] {
            match conic_crossing_roots(&c, 0.1, 6.0, &plane_z(z), band()) {
                Ok(ConicPlaneMeet::Parallel { offset }) => {
                    assert!((offset - want).abs() < 1e-12, "offset at z = {z}: {offset}");
                }
                other => panic!("z = {z}: expected the parallel arm, got {other:?}"),
            }
        }
    }

    /// Line carriers refuse the conic lane (the `Err(())` sentinel the
    /// caller maps to the bit-identical M3 interpolation path).
    #[test]
    fn line_carriers_take_the_m3_lane() {
        let line = Curve3::Line {
            origin: Point3::origin(),
            dir: Vec3::unit_x(),
        };
        assert!(conic_crossing_roots(&line, 0.0, 1.0, &plane_y(0.5), band()).is_err());
    }

    /// **The midpoint anchor's straddle row, at `Interval`** (issue
    /// 1191). Nothing in the shipped suites drives either anchor into a
    /// straddle, so this row does it on the site's own numbers.
    ///
    /// A candidate root sitting ON the span's start is the ordinary
    /// case — a re-run split fragment meets the full circle at its own
    /// start vertex — and at `Interval` its enclosure straddles `t₀`.
    /// The MIDPOINT anchor's jump is half a period away from there, so
    /// the recentred parameter comes back at the width of its input and
    /// the interiority verdict is reached; the `t₀` anchor's jump is
    /// exactly on it, which is why it is the FALLBACK and not the first
    /// try. The row measures both reductions on the same box so the
    /// design's premise — two jumps, half a period apart — is pinned
    /// rather than described.
    ///
    /// Consults no tolerance: the two widths are widths, and the band
    /// is only what the site's own margins need in order to run.
    #[test]
    fn a_root_on_the_span_start_reduces_at_input_width_under_the_midpoint_anchor() {
        use geom_core::{Bounds, Interval, Real};

        let ex = Interval::from_f64;
        let c = Curve3::Circle {
            center: Point3::new(ex(0.0), ex(0.0), ex(0.0)),
            axis: Vec3::new(ex(0.0), ex(0.0), ex(1.0)),
            radius: ex(1.0),
            u_ref: Vec3::new(ex(1.0), ex(0.0), ex(0.0)),
        };
        let plane = crate::test_support::split_plane(
            Point3::new(ex(0.0), ex(0.0), ex(0.0)),
            Vec3::new(ex(0.0), ex(1.0), ex(0.0)),
            geom_core::Tol::witness(),
        );
        // The span is the upper semicircle; the plane's two crossings
        // are its own endpoints.
        let (t0, t1) = (ex(0.0), ex(core::f64::consts::PI));
        let roots = roots_of(conic_crossing_roots(&c, t0, t1, &plane, band()))
            .expect("the endpoint roots classify — no anchor straddles them");
        assert!(
            roots.is_empty(),
            "both roots are the span's own endpoints and belong to the vertex sweep"
        );

        // The premise, measured: a hairline box about `t₀` is a
        // hairline under the midpoint anchor and a whole period under
        // the `t₀` anchor.
        //
        // DISPOSITION: like its chord_join twin, this half re-derives
        // both anchorings inline and so pins the two WINDOWS rather
        // than the site. The site's own pin is the
        // `conic_crossing_roots` call above, which reds if the anchor
        // order changes; this half says why that order is the right
        // one.
        let tau = Interval::tau();
        let mid = (t0 + t1) * ex(0.5);
        let cand = Interval::from_bounds(-1e-15, 1e-15);
        let centred = mid + (cand - mid).reduce_periodic_centred(tau);
        let anchored = t0 + (cand - t0).reduce_periodic(tau);
        assert!(
            centred.hi() - centred.lo() <= 1e-9,
            "the midpoint anchor widened: [{}, {}]",
            centred.lo(),
            centred.hi()
        );
        assert!(
            anchored.hi() - anchored.lo() >= core::f64::consts::TAU,
            "the t0 anchor is supposed to be the wide one here; it gave [{}, {}]",
            anchored.lo(),
            anchored.hi()
        );
    }

    /// The greatest and least of `f` over an `n × n` grid of
    /// `[a0, a1] × [b0, b1]`.
    fn sampled(
        n: usize,
        (a0, a1): (f64, f64),
        (b0, b1): (f64, f64),
        f: impl Fn(f64, f64) -> f64,
    ) -> (f64, f64) {
        let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
        for i in 0..=n {
            for j in 0..=n {
                let x = f(
                    a0 + (a1 - a0) * i as f64 / n as f64,
                    b0 + (b1 - b0) * j as f64 / n as f64,
                );
                lo = lo.min(x);
                hi = hi.max(x);
            }
        }
        (lo, hi)
    }

    /// `(lo, hi)` encloses the sampled `(slo, shi)` and exceeds it by no
    /// more than the grid's own chord error `slack`.
    fn assert_support(what: &str, (lo, hi): (f64, f64), (slo, shi): (f64, f64), slack: f64) {
        assert!(
            lo <= slo + 1e-12 && hi >= shi - 1e-12,
            "{what}: [{lo}, {hi}] must enclose the samples [{slo}, {shi}]"
        );
        assert!(
            slo - lo <= slack && hi - shi <= slack,
            "{what}: [{lo}, {hi}] is looser than the samples [{slo}, {shi}] by more than {slack}"
        );
    }

    /// **A sphere zone's extent along a direction is the zone's support
    /// there**: against a dense grid of the zone's own points, for
    /// directions from along the polar axis to across it and windows
    /// that hold the direction's crest and that miss it on either side.
    #[test]
    fn the_zone_extent_is_the_zones_support_along_every_direction() {
        let (c, r) = (0.25, 1.25);
        for (lo, hi) in [(0.75f64, 1.25f64), (-1.25, 0.75), (-0.3, 0.4), (1.0, 1.1)] {
            for a in [1.0, 0.955, 0.6, 0.0, -0.4, -0.97, -1.0] {
                let e_perp = (1.0f64 - a * a).sqrt();
                // The zone: polar angle θ with r·cos θ in the window,
                // azimuth φ; `e = (√(1 − a²), a, 0)`, the axis `ŷ`.
                let (t0, t1) = (
                    (hi / r).clamp(-1.0, 1.0).acos(),
                    (lo / r).clamp(-1.0, 1.0).acos(),
                );
                let samples = sampled(600, (t0, t1), (0.0, 2.0 * PI), |t, p| {
                    c * a + r * (e_perp * t.sin() * p.cos() + a * t.cos())
                });
                let got = super::zone_extent(c * a, a, super::Span { lo, hi }, r);
                assert_support(&format!("window [{lo}, {hi}], a = {a}"), got, samples, 1e-4);
            }
        }
    }

    /// **A ring torus's chart rectangle has its own extent along a
    /// direction**: against a dense grid of the rectangle's own points,
    /// for a full turn, a window that holds no crest, one across the
    /// seam's period and one wider than a turn, along tilted directions
    /// on both sides of the axis.
    #[test]
    fn the_torus_rect_extent_is_the_rectangles_support_along_every_direction() {
        let (major, minor) = (0.75, 0.25);
        // Axis `ŷ`, `u_ref = x̂`, so `axis × u_ref = −ẑ`.
        let point = |u: f64, v: f64| {
            let rho = major + minor * v.cos();
            Vec3::new(rho * u.cos(), minor * v.sin(), -rho * u.sin())
        };
        for (u, v) in [
            ((0.0, 2.0 * PI), (0.0, PI / 2.0)),
            ((1.0, 2.5), (-0.5, 2.0)),
            ((5.5, 7.0), (3.0, 4.0)),
            ((-1.0, 6.0), (2.5, 9.0)),
        ] {
            for e in [
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.3, 0.9, -0.2),
                Vec3::new(-0.8, 0.1, 0.5),
                Vec3::new(0.2, -0.7, 0.6),
                Vec3::new(0.0, 0.0, -1.0),
            ] {
                let e = e * (1.0 / e.norm());
                let samples = sampled(600, u, v, |a, b| e.dot(point(a, b)));
                let span = |(lo, hi)| super::Span { lo, hi };
                let got = super::torus_rect_extent(
                    0.0,
                    (e.x, -e.z, e.y),
                    (major, minor),
                    (span(u), span(v)),
                );
                assert_support(
                    &format!("window {u:?} × {v:?}, e = {e:?}"),
                    got,
                    samples,
                    1e-4,
                );
            }
        }
    }
}
