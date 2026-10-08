//! Reduction step (ch. 14 `splitgenerate`, re-derived): the operand
//! gate (F5 → the C5 table, M5 PR 5), the cached vertex-vs-plane
//! trilean sweep (F6), and crossing insertion through the certified
//! `split_edge` lane — with the conic crossing-root lane for
//! circle/ellipse carriers.

use geom_brep::recourse::{Reading, RefusedArm, SizedDecision, SizedPass, StoredDefinite};
use geom_core::{Band, Decide, Margin, Point3, Sign, Tol};
use slotmap::SecondaryMap;

use super::{PlaneSide, SplitPlane, SplitReduceError};
use crate::body::Body;
use crate::boolean::boxes::BoxFrame;
use crate::entity::{EdgeKey, FaceKey, VertexKey};
use crate::null::CurveGeom;
use crate::validate::decide;

/// **The carrier gate, scoped to what the plane can reach** (C12.1:
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
/// ([`edge_clears`]). A face whose surface key, or an edge whose curve
/// key, does not resolve is a torn body and panics (the operand is a
/// public body, at rest), and so does a null edge: every door that
/// reaches here holds its body tier-2 clean first (a finished operand,
/// or a test-support door's own read), and tier 2 refuses one.
pub(super) fn carrier_gate<T: Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    band: Band,
) -> Result<(), SplitReduceError> {
    let frame = BoxFrame::aimed(plane.normal);
    for (face_key, face) in body.faces() {
        let surface = body.face_surface_linked(face_key, face);
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
        match body.edge_curve_linked(edge_key, edge) {
            CurveGeom::Certified(curve) => match curve.carrier() {
                geom::Curve3::Line { .. }
                | geom::Curve3::Circle { .. }
                | geom::Curve3::Ellipse { .. } => {}
                // No crossing-root arm reads a spiric or a spline.
                geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => {
                    if !edge_clears(body, edge_key, plane, band, &frame) {
                        return Err(SplitReduceError::CurvedEdgeUnsupported { edge: edge_key });
                    }
                }
            },
            CurveGeom::NullScaffold(_) => unreachable!(
                "{edge_key:?} is a null edge, and every door reaching here holds its body tier-2 \
                 clean, which refuses a null edge at rest"
            ),
        }
    }
    Ok(())
}

/// Whether the plane certainly misses an edge: behind its own reach
/// (`census::edge_reach_in`) or behind the reach of either face it
/// bounds, since an edge lies on both its faces. A spline edge has no
/// sound box of its own (`EdgeBoxRule::NoSoundBox`), so its faces'
/// reaches are what clear it. `frame` is the plane's
/// ([`BoxFrame::aimed`] at its normal).
///
/// `edge` is one this call's caller read out of the body: the gate's
/// arena walk over the at-rest operand, or `insert_crossings`' snapshot
/// of it, where only `split_edge`s of other snapshot edges stand
/// between, and they remove no edge and rewrite no other edge's halves.
/// So `edge` is proven, and its halves and their faces are links, which
/// panic on a miss ([`crate::live::OPERATORS_KEEP_LINKS`]).
fn edge_clears<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    plane: &SplitPlane<T>,
    band: Band,
    frame: &BoxFrame<T>,
) -> bool {
    let e = crate::live::proven(&body.edges, edge, crate::entity::EntityId::Edge);
    reach_clears(crate::census::edge_reach_in(body, edge, frame), plane, band)
        || [e.he_plus, e.he_minus].into_iter().any(|he| {
            let face = body.face_of_linked(he);
            reach_clears(gate_face_reach(body, face, band, frame), plane, band)
        })
}

/// K name: a torus face's ring convention `R > r`, the major radius
/// less the minor (metres), decided before its chart rectangle is read
/// in closed form.
const SPLIT_GATE_TORUS_RING: &str = "split_gate_torus_ring";

/// A face's reach for the gate, in `frame`: `census::face_reach_in`
/// (the boolean's `FaceBoxRule` at this body's scalar), except where a
/// torus face's chart rectangle is read in closed form
/// ([`torus_window_reach`]): the rule's box for a torus samples the
/// rectangle and pays a subdivision charge that reaches thousandths of
/// the radii.
///
/// `face` is read out of the body by the caller (the gate's arena walk,
/// or a half's face in [`edge_clears`]), so it is proven, and its
/// surface is a link ([`crate::live::OPERATORS_KEEP_LINKS`]): either
/// miss panics. `None` is the reach's own: no claim to make.
fn gate_face_reach<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
    frame: &BoxFrame<T>,
) -> Option<(Point3<T>, Point3<T>)> {
    let f = crate::live::proven(&body.faces, face, crate::entity::EntityId::Face);
    let patch = match body.face_surface_linked(face, f) {
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
    patch.or_else(|| crate::census::face_reach_in(body, face, band, frame))
}

/// A torus face's chart rectangle, in `frame`: `boxes::torus_rect_extent`
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
    let window = crate::census::torus_chart_window(body, face, major, minor)?;
    let (c, ax, ur, vr) = (
        frame.point(center),
        frame.vector(axis),
        frame.vector(u_ref),
        frame.vector(axis.cross(u_ref)),
    );
    let at = |c: T, p: T, q: T, a: T| {
        crate::boolean::boxes::torus_rect_extent(c, (p, q, a), (major, minor), window)
    };
    let ((xl, xh), (yl, yh), (zl, zh)) = (
        at(c.x, ur.x, vr.x, ax.x),
        at(c.y, ur.y, vr.y, ax.y),
        at(c.z, ur.z, vr.z, ax.z),
    );
    Some((Point3::new(xl, yl, zl), Point3::new(xh, yh, zh)))
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
    // The frame turns about the world origin, so a reach's first
    // coordinate is `n·p`, and the world origin's own offset from the
    // plane turns it into `n·(p − q)`.
    let origin = crate::sector_shape::plane_offset(
        plane.origin,
        plane.normal.get(),
        Point3::new(T::zero(), T::zero(), T::zero()),
    );
    let gap = (lo.x + origin).max(T::zero() - (hi.x + origin)) - pad;
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
    for (vertex_key, p) in body.vertex_points() {
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

/// Which lane finds where a carrier's span crosses a plane
/// ([`plane_crossing_lane`]), keyed on the carrier's kind.
#[derive(Debug)]
pub(crate) enum PlaneCrossingLane<T> {
    /// A line: its signed distance is affine, with one root that the
    /// endpoints' sides place, so the caller's endpoint lane owns it.
    Line,
    /// A circle or an ellipse: what the root lane found.
    Conic(ConicPlaneMeet<T>),
    /// A spiric or a spline: no lane finds its crossings. Its endpoints'
    /// sides say nothing about its interior, which can cross the plane
    /// and come back between same-side ends, and a parameter
    /// interpolated between them is not a point on the plane. The caller
    /// refuses it, or passes it only behind a certificate that its whole
    /// locus clears the plane.
    Unlaned,
}

/// What a conic carrier's span meets of a plane
/// ([`PlaneCrossingLane::Conic`]).
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

/// Which rung of the conic root lane ([`plane_crossing_lane`]) escalated, with its
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

/// The lane that finds where a carrier's span crosses a plane
/// ([`PlaneCrossingLane`]), the split lane's and the boolean sweep's
/// alike. On a **conic** it finds the crossing roots over the span: the
/// signed distance along the carrier is
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
///    plane grazes the carrier's extremum — ONE (double) root, `φ` or
///    `φ + π` as `split_conic_graze_side` decides D's sign, whose
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
pub(crate) fn plane_crossing_lane<T: Decide>(
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    plane_origin: geom_core::Point3<T>,
    plane_normal: geom_core::Vec3<T>,
    band: Band,
) -> PlaneCrossingLane<T> {
    match (geom_brep::Conic::of(carrier), carrier) {
        (Some(conic), _) => PlaneCrossingLane::Conic(conic_plane_meet(
            conic,
            t0,
            t1,
            plane_origin,
            plane_normal,
            band,
        )),
        (None, geom::Curve3::Line { .. }) => PlaneCrossingLane::Line,
        (None, _) => PlaneCrossingLane::Unlaned,
    }
}

/// The conic arm of [`plane_crossing_lane`].
fn conic_plane_meet<T: Decide>(
    conic: geom_brep::Conic<T>,
    t0: T,
    t1: T,
    plane_origin: geom_core::Point3<T>,
    plane_normal: geom_core::Vec3<T>,
    band: Band,
) -> ConicPlaneMeet<T> {
    let candidates = match conic_plane_candidates(conic, plane_origin, plane_normal, band) {
        Ok(candidates) => candidates,
        Err(meet) => return meet,
    };
    let (s_u, s_v) = (conic.major, conic.minor);
    span_roots(candidates, t0, t1, s_u, s_v, band)
}

/// **The carrier half of [`conic_plane_meet`]**: where the whole conic
/// meets the plane, as carrier parameters (one for a graze, two for a
/// crossing), before any span is read. `Err` carries the answer that
/// needs no span — parallel, a miss, or the fault a rung escalated.
pub(crate) fn conic_plane_candidates<T: Decide>(
    conic: geom_brep::Conic<T>,
    plane_origin: geom_core::Point3<T>,
    plane_normal: geom_core::Vec3<T>,
    band: Band,
) -> Result<[Option<T>; 2], ConicPlaneMeet<T>> {
    let geom_brep::Conic {
        center,
        axis,
        u_ref,
        major: s_u,
        minor: s_v,
    } = conic;
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
        Ok(Sign::Zero) => return Err(ConicPlaneMeet::Parallel { offset: d0 }),
        Ok(Sign::Positive | Sign::Negative) => {}
        Err(diag) => {
            return Err(ConicPlaneMeet::Roots(Err(ConicRootFault::PlaneParallel(
                diag,
            ))));
        }
    }
    // 1. Does the sinusoid reach zero at all — and how many roots?
    let both_roots = match decide("split_conic_belly_graze", Margin::of(r - d0.abs()), band) {
        Ok(Sign::Negative) => return Err(ConicPlaneMeet::Miss),
        Ok(Sign::Positive) => true,
        // Graze: the double root, processed once (processing both
        // would split twice at coincident parameters and escalate on
        // the second interiority check — same refusal, worse site).
        Ok(Sign::Zero) => false,
        Err(diag) => return Err(ConicPlaneMeet::Roots(Err(ConicRootFault::BellyGraze(diag)))),
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
    // A graze's one root is the sinusoid's extremum nearest the plane:
    // φ with the centre below the plane (D < 0), φ + π with it above. The
    // residue's own roots lie up to √(2ε/R) radians either side, far
    // outside ε.
    let candidates: [Option<T>; 2] = if both_roots {
        // The roots solve cos(θ − φ) = −D/R. Clamped acos (rounding can
        // push the ratio a hair outside ±1; min/max are Real lattice ops).
        let delta = ((T::zero() - d0) / r)
            .min(T::one())
            .max(T::zero() - T::one())
            .acos();
        [Some(phi + delta), Some(phi - delta)]
    } else {
        let graze_side = |diag| Err(ConicPlaneMeet::Roots(Err(ConicRootFault::BellyGraze(diag))));
        match decide("split_conic_graze_side", Margin::of(d0), band) {
            Ok(Sign::Negative) => [Some(phi), None],
            Ok(Sign::Positive) => [Some(phi + T::pi()), None],
            // |D| is within ε of R ≥ Kε, so it reads Zero only when
            // K ≤ 2 and the conic's reach is itself inside the band.
            Ok(Sign::Zero) => {
                return graze_side(crate::invalid_margin::invalid(
                    band,
                    "split_conic_graze_side",
                ));
            }
            Err(diag) => return graze_side(diag),
        }
    };
    Ok(candidates)
}

/// The span half of [`conic_plane_meet`]: the candidates interior to
/// `[t0, t1]`, ascending.
fn span_roots<T: Decide>(
    candidates: [Option<T>; 2],
    t0: T,
    t1: T,
    s_u: T,
    s_v: T,
    band: Band,
) -> ConicPlaneMeet<T> {
    let tau = T::tau();
    // The conservative meter (radians → meters): the smaller semi-axis
    // MAGNITUDE (the stored semi-axes carry no order, `geom_brep::Conic`;
    // `s_v` in the ordinary order). An INF bound on the speed by being
    // the smaller principal rate, which is what the interiority claim
    // below needs — a root this meter proves clear of an endpoint is
    // clear of it in metres.
    let meter = geom_core::InfSpeed::new(s_u.abs().min(s_v.abs()));
    let mut roots: Vec<T> = Vec::with_capacity(2);
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
    // COMPLEMENT. Not half a period: that is the span → 0
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
                        return ConicPlaneMeet::Roots(Err(ConicRootFault::CrossingInterior(first)));
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
            Err(diag) => return ConicPlaneMeet::Roots(Err(ConicRootFault::RootOrder(diag))),
        }
    }
    ConicPlaneMeet::Roots(Ok(roots))
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
///   ([`plane_crossing_lane`]), INDEPENDENT of endpoint verdicts:
///   same-side endpoints with a belly crossing the plane twice get
///   BOTH crossing vertices; an ON endpoint with one interior
///   crossing gets it; grazes land as single ON contacts for the
///   sector classification. Two roots split the parent then its
///   trailing child (ascending — the second root lives on the child's
///   span).
/// - **Spiric and spline carriers** have no crossing lane. One passes
///   uncut only where [`edge_clears`] certifies its whole locus on one
///   side of the plane, the carrier gate's own test; any other refuses
///   [`SplitReduceError::CurvedEdgeUnsupported`]. Same-side endpoints
///   are not enough: a spline's belly can cross between them.
///
/// Both crossing lanes are certified by `split_edge` itself (the
/// `split_edge_param_interior` trilean + full child re-certification —
/// the honest lane the raw book formula lacks). New vertices are ON by
/// the decision that placed them: a crossing root lies on the plane, and
/// a graze root within ε of it by `split_conic_belly_graze`'s Zero.
/// Their verdicts are cached without re-measuring.
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
        let start = |body: &Body<T>, he, slot| {
            crate::live::linked(
                &body.half_edges,
                he,
                crate::entity::EntityId::HalfEdge,
                crate::entity::EntityId::Edge(edge_key),
                slot,
            )
            .start
        };
        let u = start(body, edge.he_plus, "he_plus");
        let v = start(body, edge.he_minus, "he_minus");
        // The snapshot's record is the live one: no `split_edge` before
        // this one touched this edge. So the curve is a link, and its
        // miss panics (`live::OPERATORS_KEEP_LINKS`).
        let curve = match body.edge_curve_linked(edge_key, &edge) {
            CurveGeom::Certified(c) => c.clone(),
            CurveGeom::NullScaffold(_) => unreachable!(
                "{edge_key:?} is a null edge, and every door reaching here holds its body tier-2 \
                 clean, which refuses a null edge at rest"
            ),
        };
        let (t0, t1) = curve.params();
        let roots: Vec<T> = match plane_crossing_lane(
            curve.carrier(),
            t0,
            t1,
            plane.origin,
            plane.normal.get(),
            band,
        ) {
            // Parallel either way: no root to insert, and a conic in
            // the plane has its endpoints ON through the vertex sides.
            PlaneCrossingLane::Conic(ConicPlaneMeet::Parallel { .. } | ConicPlaneMeet::Miss) => {
                continue;
            }
            PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(Ok(roots))) => roots,
            PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(Err(fault))) => {
                return Err(SplitReduceError::CrossingEscalated {
                    edge: edge_key,
                    fault,
                });
            }
            // It passes only behind a box the plane cannot meet, read
            // here as at the gate, so its whole locus is on one side.
            PlaneCrossingLane::Unlaned => {
                if edge_clears(body, edge_key, plane, band, &BoxFrame::aimed(plane.normal)) {
                    continue;
                }
                return Err(SplitReduceError::CurvedEdgeUnsupported { edge: edge_key });
            }
            PlaneCrossingLane::Line => {
                // Endpoint verdicts strictly opposite ⇒ one
                // interpolated root.
                let crossing = matches!(
                    (sides[u], sides[v]),
                    (PlaneSide::Above, PlaneSide::Below) | (PlaneSide::Below, PlaneSide::Above)
                );
                if !crossing {
                    continue;
                }
                let dist = |vk: VertexKey| {
                    let p = body.resolve_vertex_point(vk, crate::live::Proven);
                    (p - plane.origin).dot(plane.normal.get())
                };
                let (d1, d2) = (dist(u), dist(v));
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
                    source: source.from_driver(),
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

    use geom::Curve3;
    use geom_core::{Band, Point3, Vec3};

    use super::{ConicPlaneMeet, PlaneCrossingLane, plane_crossing_lane};
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
        let got = roots_of(plane_crossing_lane(
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
        m: PlaneCrossingLane<T>,
    ) -> Result<Vec<T>, geom_core::Indeterminate> {
        match m {
            PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(r)) => {
                r.map_err(super::ConicRootFault::diag)
            }
            other => panic!("expected the roots arm, got {other:?}"),
        }
    }

    /// The lane against a split plane.
    fn on_split_plane<T: geom_core::Decide>(
        carrier: &Curve3<T>,
        t0: T,
        t1: T,
        plane: &SplitPlane<T>,
        band: Band,
    ) -> PlaneCrossingLane<T> {
        plane_crossing_lane(carrier, t0, t1, plane.origin, plane.normal.get(), band)
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
    /// (two roots), definitely-missing (no roots), tangent within ε
    /// (one graze root), and in-band (typed escalation).
    #[test]
    fn belly_graze_trio() {
        use core::f64::consts::FRAC_PI_2;
        let c = circle();
        // Secant (margin R − |D| = 1, definite): the two roots of
        // sin θ = 0.5 land in the span, ascending.
        let roots = roots_of(on_split_plane(&c, 0.1, 6.0, &plane_y(0.5), band())).unwrap();
        assert_eq!(roots.len(), 2);
        assert!((roots[0] - core::f64::consts::FRAC_PI_6).abs() < 1e-12);
        assert!((roots[1] - (core::f64::consts::PI - core::f64::consts::FRAC_PI_6)).abs() < 1e-12);
        // Missing (margin −1, definite): no crossing at all.
        assert!(matches!(
            on_split_plane(&c, 0.1, 6.0, &plane_y(2.0), band()),
            PlaneCrossingLane::Conic(ConicPlaneMeet::Miss)
        ));
        // Tangent within ε, either side (margin 0, ±ε/2), above the
        // centre and below it: ONE graze root, at the extremum. Inside,
        // the residue's own roots lie √(2·5e-10) ≈ 3.2e-5 either side.
        for (c0, want) in [(1.0, FRAC_PI_2), (-1.0, 3.0 * FRAC_PI_2)] {
            for y in [c0, c0 - 5e-10, c0 + 5e-10] {
                let roots = roots_of(on_split_plane(&c, 0.1, 6.0, &plane_y(y), band())).unwrap();
                assert_eq!(roots.len(), 1, "y = {y}: one graze root");
                assert!(
                    (roots[0] - want).abs() < 1e-12,
                    "y = {y}: the graze root {} sits at the extremum {want}",
                    roots[0]
                );
            }
        }
        // In-band (margin −3ε): typed escalation, named.
        let diag =
            roots_of(on_split_plane(&c, 0.1, 6.0, &plane_y(1.0 + 3e-9), band())).unwrap_err();
        assert_eq!(diag.predicate, Some("split_conic_belly_graze"));
    }

    /// Under `Interval` the graze root is the extremum to the phase's
    /// own width, with no acos of a ratio at ±1 widening it.
    #[test]
    fn an_interval_graze_root_is_as_tight_as_its_phase() {
        use geom_core::{Bounds, Interval, Real};
        let ex = Interval::from_f64;
        let c = Curve3::Circle {
            center: Point3::new(ex(0.0), ex(0.0), ex(0.0)),
            axis: Vec3::new(ex(0.0), ex(0.0), ex(1.0)),
            radius: ex(1.0),
            u_ref: Vec3::new(ex(1.0), ex(0.0), ex(0.0)),
        };
        for y in [1.0, 1.0 - 5e-10] {
            let plane = crate::test_support::split_plane(
                Point3::new(ex(0.0), ex(y), ex(0.0)),
                Vec3::new(ex(0.0), ex(1.0), ex(0.0)),
                geom_core::Tol::witness(),
            );
            let roots = roots_of(on_split_plane(&c, ex(0.1), ex(6.0), &plane, band())).unwrap();
            assert_eq!(roots.len(), 1, "y = {y}: one graze root");
            let r = roots[0];
            assert!(
                r.lo() <= core::f64::consts::FRAC_PI_2
                    && core::f64::consts::FRAC_PI_2 <= r.hi()
                    && r.hi() - r.lo() <= 1e-14,
                "y = {y}: the graze root [{}, {}] encloses π/2 tightly",
                r.lo(),
                r.hi()
            );
        }
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
        let roots = roots_of(on_split_plane(&c, 0.0, 2.0 + 2.0, &plane_y(0.0), band())).unwrap();
        assert_eq!(roots.len(), 1);
        assert!((roots[0] - core::f64::consts::PI).abs() < 1e-12);
        // Both roots definitely interior: span (−1, 4).
        let roots = roots_of(on_split_plane(&c, -1.0, 4.0, &plane_y(0.0), band())).unwrap();
        assert_eq!(roots.len(), 2);
        assert!(roots[0].abs() < 1e-12 || (roots[0] - core::f64::consts::PI).abs() < 1e-12);
        // In-band: a root 5e-9 (meters, meter = r = 1) inside the
        // span end — typed escalation, named.
        let diag = roots_of(on_split_plane(&c, -5e-9, 2.0, &plane_y(0.0), band())).unwrap_err();
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
            match on_split_plane(&c, 0.1, 6.0, &plane_z(z), band()) {
                PlaneCrossingLane::Conic(ConicPlaneMeet::Parallel { offset }) => {
                    assert!((offset - want).abs() < 1e-12, "offset at z = {z}: {offset}");
                }
                other => panic!("z = {z}: expected the parallel arm, got {other:?}"),
            }
        }
    }

    /// **Each carrier kind lands on its own lane**: a line on the
    /// endpoint lane, a circle and an ellipse on the root lane, and a
    /// spiric and a spline on none. The answer is keyed on the kind
    /// alone, whatever the plane: the spiric here never meets `y = ½`
    /// (its `y ≥ √(ρ² − ¼) > 2`), and it is still `Unlaned`, because a
    /// curved carrier answered `Line` anywhere would have its crossings
    /// read off its endpoints.
    #[test]
    fn each_carrier_kind_lands_on_its_own_lane() {
        let plane = plane_y(0.5);
        for (carrier, want) in [
            (
                Curve3::Line {
                    origin: Point3::origin(),
                    dir: Vec3::unit_y(),
                },
                "line",
            ),
            (circle(), "conic"),
            (
                Curve3::Ellipse {
                    center: Point3::origin(),
                    axis: Vec3::unit_z(),
                    major: 2.0,
                    minor: 1.0,
                    u_ref: Vec3::unit_x(),
                },
                "conic",
            ),
            (
                Curve3::Spiric {
                    center: Point3::origin(),
                    axis: Vec3::unit_z(),
                    u_ref: Vec3::unit_x(),
                    major_radius: 2.0,
                    minor_radius: 1.0,
                    offset: 0.5,
                },
                "unlaned",
            ),
            (
                Curve3::Nurbs(std::sync::Arc::new(
                    geom::NurbsCurve3::new(
                        geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1)
                            .unwrap(),
                        vec![Point3::origin(), Point3::new(0.0, 1.0, 0.0)],
                        vec![1.0; 2],
                    )
                    .unwrap(),
                )),
                "unlaned",
            ),
        ] {
            let got = match on_split_plane(&carrier, 0.0, 1.0, &plane, band()) {
                PlaneCrossingLane::Line => "line",
                PlaneCrossingLane::Conic(_) => "conic",
                PlaneCrossingLane::Unlaned => "unlaned",
            };
            assert_eq!(got, want, "{carrier:?}");
        }
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
        let roots = roots_of(on_split_plane(&c, t0, t1, &plane, band()))
            .expect("the endpoint roots classify — no anchor straddles them");
        assert!(
            roots.is_empty(),
            "both roots are the span's own endpoints and belong to the vertex sweep"
        );

        // The premise, measured: a hairline box about `t₀` is a
        // hairline under the midpoint anchor and a whole period under
        // the `t₀` anchor.
        //
        // DISPOSITION: this half re-derives both anchorings inline and
        // so pins the two WINDOWS rather than the site. The site's own
        // pin is the `on_split_plane` call above, which reds if the
        // anchor order changes; this half says why that order is the
        // right one.
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
}

/// **A torn curve panics at the split gate and at the crossing
/// insertion**, where a read that took the miss for scaffolding refused
/// it as `ScaffoldingOperand`.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod torn_rows {
    use geom_core::{Band, Point3, Tol, Vec3};

    use crate::entity::EntityId;
    use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};

    #[test]
    fn a_torn_curve_panics_at_the_gate_and_the_crossing_insertion() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let mut body = crate::test_support_fixtures::geometric_cube::<f64>(tol).body;
        let plane = crate::test_support_fixtures::split_plane(
            Point3::new(0.0, 0.0, 0.5),
            Vec3::unit_z(),
            tol,
        );
        let (edge, curve) = body.edges().next().map(|(k, e)| (k, e.curve)).unwrap();
        let (mut sides, mut on) = super::classify_vertices(&body, &plane, band).unwrap();
        body.curves.remove(curve);
        let named = format!("{}'s curve names", EntityId::Edge(edge));
        assert_torn_op_panics("carrier_gate", &mut body, &[&named, ROW_FOUR], |b| {
            super::carrier_gate(b, &plane, band)
        });
        assert_torn_op_panics("insert_crossings", &mut body, &[&named, ROW_FOUR], |b| {
            super::insert_crossings(b, &plane, &mut sides, &mut on, tol)
        });
    }

    /// **The gate's reads past a face it walked panic on a torn link.**
    /// `edge_clears` reads both faces of an edge the plane crosses, and a
    /// half whose loop was dropped panics naming it, where a read of the
    /// miss as no face would clear the edge on its other face alone.
    /// `gate_face_reach` reads the face's surface, and a dropped one
    /// panics naming it, where a read of the miss as no claim to make
    /// would answer the census's box.
    #[test]
    fn the_gate_panics_on_a_torn_face_of_an_edge_and_a_torn_surface() {
        use crate::boolean::boxes::BoxFrame;
        use crate::live::OPERATORS_KEEP_LINKS;
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let fresh = || crate::test_support_fixtures::geometric_cube::<f64>(tol).body;
        let plane = crate::test_support_fixtures::split_plane(
            Point3::new(0.0, 0.0, 0.5),
            Vec3::unit_z(),
            tol,
        );
        let frame = BoxFrame::aimed(plane.normal);

        let mut body = fresh();
        let crosses = |b: &crate::body::Body<f64>, e| {
            let (u, v) = b.edge_vertices(e).unwrap();
            let z = |w| b.get_point(b.get_vertex(w).unwrap().point).unwrap().z;
            (z(u) - 0.5) * (z(v) - 0.5) < 0.0
        };
        let (edge, data) = body
            .edges()
            .find(|&(k, _)| crosses(&body, k))
            .map(|(k, e)| (k, e.clone()))
            .unwrap();
        assert!(
            !super::edge_clears(&body, edge, &plane, band, &frame),
            "the plane meets an edge it crosses"
        );
        let lost = body.get_half_edge(data.he_plus).unwrap().parent_loop;
        body.loops.remove(lost);
        let named = format!(
            "{}'s parent_loop names {}",
            EntityId::HalfEdge(data.he_plus),
            EntityId::Loop(lost)
        );
        assert_torn_op_panics(
            "edge_clears",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| super::edge_clears(b, edge, &plane, band, &frame),
        );

        let mut body = fresh();
        let (face, surface) = body.faces().next().map(|(k, f)| (k, f.surface)).unwrap();
        assert!(
            super::gate_face_reach(&body, face, band, &frame).is_some(),
            "a sound face has a reach"
        );
        body.surfaces.remove(surface);
        let named = format!("{}'s surface names", EntityId::Face(face));
        assert_torn_op_panics(
            "gate_face_reach",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| super::gate_face_reach(b, face, band, &frame),
        );
    }
}
