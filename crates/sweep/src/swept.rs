//! The lowering every profile sweep shares: the swept-traversal record
//! and the builder that fills it, the carrier class of one swept
//! segment, the sketch-level quantities derived from it (apex, span,
//! turn-signed axis), the arc material-side rule, the edge spec a
//! placed segment mints, the cap-plane point list, the cosurface
//! decision, and the two crate-wide accessors (the classification
//! funnel, a face's surface key).
//!
//! This module is a sibling of the sweep verbs, not a member of one:
//! its consumers are `extrude`, `revolve` and `loft`, and a core
//! hosted inside one of its consumers is the shape that drifts.
//!
//! What is deliberately NOT here: anything a verb decides
//! differently. The **wall-orientation sense** is per-verb, and the
//! split is by segment kind: on ARC walls every verb that has them
//! reads the canonical turn, which is why that arm is here as
//! [`centre_on_material_side`] and is called from both verbs rather
//! than spelled twice. On LINE walls they diverge and no body is
//! shared — extrude's are Newell-outward and always `true`, revolve's
//! read a canonical Δz (cylinder, cone) or Δr (plane annulus), and
//! loft is uniform `true`. The **strut carrier** is per-verb: a
//! translation trajectory is a line, a rotation trajectory is a
//! circle, and the two specs agree on neither arity, carrier nor
//! `MappedCurve` variant.
//!
//! The **swept-traversal builder** is not in that list: it is here.
//! [`swept_segments`] is the one place a validated loop is relabelled
//! into traversal order, forward or reversed, and [`SweptSeg`] the
//! record it fills. Each verb still decides *whether* to reverse for
//! its own reason — extrude for `w·n < 0`, revolve for θ > 0, loft
//! never — but the relabelling is one rule with one implementation.
//! Extrude needs one field more than the record carries, and takes it
//! the way [`SweptChord`] prescribes: its own record
//! (`extrude::WallSeg`) wraps these fields and adds the orientation
//! bit, and every shared body below reads it through the trait, so no
//! shared body can see that bit.
//!
//! **One qualifier, and it is load-bearing: `from a validated loop`.**
//! `revolve::tube` mints its two-arc traversal directly from the
//! caller's intent values, because its whole purpose is to store the
//! given centre and radii bit-exactly rather than reconstruct them
//! from bulges — so it cannot take a `ValidatedLoop` and cannot come
//! through here. It applies the same reversal convention by hand and
//! **says so at its own site**; that marker is the only thing tying
//! the two together, and it is deliberately not deleted.

use geom::Curve3;
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, MappedCurve, SketchSegment};
use geom_core::{
    Affine3, Band, Decide, Indeterminate, Margin, Point2, Point3, Real, Sign, Tol, Vec2, Vec3,
};
use topo::{Body, EulerOpError, FaceKey, SurfaceKey};

/// The classification funnel of this shared lowering, and of `extrude`
/// and `revolve` above it (the `geom-brep` pattern).
///
/// Delegates to the unified recorder funnel
/// [`geom_core::k_stats::decide`], so every decision's predicate name
/// reaches the margin-telemetry recorder. The name is a parameter —
/// each verb keeps its own predicate names through one shared body.
///
/// **This is not the crate's only funnel, nor its only door to the
/// recorder**, and the population is a grep rather than a list here:
/// `rg 'geom_core::k_stats::decide' crates/sweep/src` catches every
/// module that reaches the recorder, because the only two ways to
/// reach it are that path written out at the call site and that path
/// imported at the top of a module — the second is why grepping for
/// `decide(` instead finds neither this funnel's callers nor
/// `revolve::tube`'s four. It over-catches by the doc comments that
/// name the funnel, which are prose, not calls; that is the price of
/// having no false negatives. Stated as the command rather than as a
/// count or a list of names.
pub(crate) fn decide<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Result<Sign, Indeterminate> {
    geom_core::k_stats::decide(name, margin, band)
}

/// A segment's carrier class in swept traversal order (the canonical
/// classification carried through any reversal — never re-decided from
/// scalar data here).
///
/// Field-for-field the shape of `profile::SegmentKind`, and
/// deliberately a separate type: `turn` here is the **swept** turn,
/// flipped by a reversal, where the profile crate's is the canonical
/// one — the same data under different orientation.
///
/// The correspondence is not kept by hand. [`swept_segments`] is the
/// only place one is built from the other, and its `match` is
/// exhaustive over `profile::SegmentKind`, so an arm added there stops
/// this crate compiling until it is answered here.
#[derive(Clone, Copy, Debug)]
pub(crate) enum SweptKind<T: Real> {
    Line,
    Arc {
        center: Point2<T>,
        radius: T,
        /// Turn sense in swept traversal: `Positive` = counterclockwise
        /// in sketch coordinates. Never `Zero` (upstream classification;
        /// kept total — a `Zero` would take the `Positive` arm).
        turn: Sign,
    },
}

/// Whether an arc's carrier centre lies on the material side of its
/// chord, from the segment's CANONICAL turn: `true` unless the turn is
/// `Negative`.
///
/// The profile's canonical winding is material-left (outers
/// counterclockwise, holes clockwise) and a counterclockwise arc curves
/// around its centre, so the centre is left of the chord — the material
/// side — exactly when the canonical turn is `Positive`. Concavity is a
/// property of the 2-D region against the carrier alone, so the sweep
/// direction never enters; callers pass a canonical turn, never a swept
/// one.
///
/// Total by design. `Zero` is unreachable for a classified arc (a zero
/// turn classifies as a line) and takes the convex arm, the
/// [`turn_axis`] posture — decided here once rather than at each
/// consumer, which is the reason this is a function and not a rule
/// each verb spells for itself.
pub(crate) fn centre_on_material_side(canonical_turn: Sign) -> bool {
    !matches!(canonical_turn, Sign::Negative)
}

/// The sketch-level chord data this module's lowering reads from a
/// swept segment: endpoints in swept traversal order, the bulge in
/// that order, and the carrier class.
///
/// It is a trait and not just [`SweptSeg`] because a verb may carry
/// more than a swept traversal does — extrude's record adds a
/// wall-orientation bit, derived from the canonical turn, that would
/// be wrong for the other verbs. Everything below this line reads a
/// chord and nothing else; anything a verb adds stays above it.
pub(crate) trait SweptChord<T: Real> {
    /// Start point, sketch coordinates.
    fn a(&self) -> Point2<T>;
    /// End point, sketch coordinates.
    fn b(&self) -> Point2<T>;
    /// The bulge in swept traversal order.
    fn bulge(&self) -> T;
    /// The carrier class in swept traversal order.
    fn kind(&self) -> SweptKind<T>;
}

/// One segment of a swept loop in swept traversal order, with the
/// canonical indices it came from.
///
/// This is what a swept traversal *is*, for every verb: the canonical
/// loop's chord data relabelled into traversal order. A verb that
/// needs more attaches its own field to its own record and reaches
/// this one through [`SweptChord`] — see `extrude::WallSeg`, whose
/// extra field is the wall face's orientation bit.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SweptSeg<T: Real> {
    /// Start point, sketch coordinates. Vertex `j` of the swept chain
    /// is segment `j`'s start.
    pub(crate) a: Point2<T>,
    /// End point.
    pub(crate) b: Point2<T>,
    /// The bulge in swept traversal (negated by reversal).
    pub(crate) bulge: T,
    /// The carrier class in swept traversal order.
    pub(crate) kind: SweptKind<T>,
    /// Canonical index of the start vertex. Error reporting only.
    pub(crate) canonical_vertex: usize,
    /// Canonical index of the segment: the index in the loop's
    /// CANONICAL segment slice that this traversal segment retraces.
    ///
    /// Not error reporting only — `extrude::wall_segments` indexes the
    /// canonical slice with it to read the turn that decides a wall
    /// face's orientation, so a wrong value here is wrong geometry,
    /// not a wrong message. It is set from the traversal's own
    /// relabelling below and never from anything else.
    pub(crate) canonical_segment: usize,
}

impl<T: Real> SweptChord<T> for SweptSeg<T> {
    fn a(&self) -> Point2<T> {
        self.a
    }
    fn b(&self) -> Point2<T> {
        self.b
    }
    fn bulge(&self) -> T {
        self.bulge
    }
    fn kind(&self) -> SweptKind<T> {
        self.kind
    }
}

/// Builds the swept traversal of one canonical loop: forward, or
/// reversed via the profile crate's reversal involution (endpoints
/// swapped, bulge negated, turn flipped).
///
/// **The one home of that involution for a validated loop** — every
/// caller that has one comes through here. Each verb reverses for its
/// own reason (extrude for `w·n < 0`, revolve for θ > 0, loft never),
/// but the relabelling itself is one rule, and reversal is a
/// relabelling only: the carrier class is carried through, never
/// re-decided from scalar data. Swept segment `j` retraces canonical
/// segment `n − 1 − j`, and swept vertex `j` is canonical vertex
/// `(n − j) mod n`.
///
/// **The qualifier is not a hedge.** `revolve::tube` has no validated
/// loop — it stores the caller's radii instead of reconstructing them
/// — so it writes the same relabelling out by hand for its two known
/// arcs, and its site says so. A change to the rule here is a change
/// to those constants (S131).
pub(crate) fn swept_segments<T: Real>(
    lp: &profile::ValidatedLoop<T>,
    reverse: bool,
) -> Vec<SweptSeg<T>> {
    let segs = lp.segments();
    let n = segs.len();
    let mut out = Vec::with_capacity(n);
    for j in 0..n {
        let (s, a, b, bulge, canonical_vertex, canonical_segment) = if reverse {
            let s = &segs[n - 1 - j];
            (
                s,
                s.end,
                s.start,
                T::zero() - s.bulge,
                (n - j) % n,
                n - 1 - j,
            )
        } else {
            let s = &segs[j];
            (s, s.start, s.end, s.bulge, j, j)
        };
        let kind = match s.kind {
            profile::SegmentKind::Line => SweptKind::Line,
            profile::SegmentKind::Arc {
                center,
                radius,
                turn,
            } => SweptKind::Arc {
                center,
                radius,
                turn: if reverse { turn.flip() } else { turn },
            },
        };
        out.push(SweptSeg {
            a,
            b,
            bulge,
            kind,
            canonical_vertex,
            canonical_segment,
        });
    }
    out
}

/// The segment as a `geom-brep` sketch segment (the description's
/// authoritative source data).
///
/// A free function over the accessors rather than a provided method:
/// the point of the trait is that the four accessors are all a verb
/// gets to supply, and a provided method is one an impl may quietly
/// override — which would put the body back to two.
pub(crate) fn sketch_segment<T: Real, S: SweptChord<T>>(seg: &S) -> SketchSegment<T> {
    match seg.kind() {
        SweptKind::Line => SketchSegment::Line {
            a: seg.a(),
            b: seg.b(),
        },
        SweptKind::Arc { .. } => SketchSegment::Arc {
            a: seg.a(),
            b: seg.b(),
            bulge: seg.bulge(),
        },
    }
}

/// The arc apex (the profile crate's exact sagitta closed form:
/// `midpoint − n̂·(L·b/2)`, n̂ the left normal of the chord direction) —
/// an on-carrier interior point of the segment, and the point that
/// keeps a 2-vertex loop plane-determining.
///
/// Takes the raw chord data rather than a [`SweptChord`]: the axis
/// classification needs the apex of a canonical profile segment, which
/// is not part of any swept traversal.
pub(crate) fn arc_apex<T: Real>(a: Point2<T>, b: Point2<T>, bulge: T) -> Point2<T> {
    let chord = b - a;
    let len = chord.norm();
    let u = chord.normalize();
    let nhat = Vec2::new(T::zero() - u.y, u.x);
    let mid = a.lerp(b, T::from_f64(0.5));
    mid - nhat * (len * bulge * T::from_f64(0.5))
}

/// The arc parameter span θ = 4·atan|bulge| (the sanctioned bulge
/// re-inspection — never endpoint `atan2`).
pub(crate) fn arc_span<T: Real>(bulge: T) -> T {
    T::from_f64(4.0) * bulge.abs().atan()
}

/// The turn-signed carrier axis (crate docs): `+normal` for a
/// counterclockwise segment, `−normal` for a clockwise one. `Zero` is
/// unreachable for classified arcs (a zero turn classified as a line);
/// kept total by taking the positive arm.
pub(crate) fn turn_axis<T: Real>(turn: Sign, normal: Vec3<T>) -> Vec3<T> {
    match turn {
        Sign::Positive | Sign::Zero => normal,
        Sign::Negative => Vec3::zero() - normal,
    }
}

/// The edge spec of a profile segment carried into 3-space by one
/// placement: `PlacedSegment` description, line or circle carrier per
/// the crate docs' carrier conventions (arc axis = turn-signed plane
/// normal, span θ = 4·atan|bulge| from the stored bulge).
///
/// `place` and `normal` are the placement the segment is lowered
/// through and its plane normal — the sketch placement for a base
/// lamina, the translated or rotated one for the swept copy.
pub(crate) fn placed_segment_spec<T: Real, S: SweptChord<T>>(
    seg: &S,
    place: Affine3<T>,
    normal: Vec3<T>,
    q_from: Point3<T>,
    q_to: Point3<T>,
) -> EdgeCurveSpec<T> {
    let description = EdgeDescriptionSpec::Scaffold(MappedCurve::PlacedSegment {
        segment: sketch_segment(seg),
        place,
    });
    match seg.kind() {
        SweptKind::Line => EdgeCurveSpec {
            description,
            carrier: Curve3::Line {
                origin: q_from,
                dir: (q_to - q_from).normalize(),
            },
            param_start: T::zero(),
            param_end: q_from.distance(q_to),
        },
        SweptKind::Arc {
            center,
            radius,
            turn,
        } => {
            let c_world = place.transform_point(Point3::new(center.x, center.y, T::zero()));
            EdgeCurveSpec {
                description,
                carrier: Curve3::Circle {
                    center: c_world,
                    axis: turn_axis(turn, normal),
                    radius,
                    u_ref: (q_from - c_world).normalize(),
                },
                param_start: T::zero(),
                param_end: arc_span(seg.bulge()),
            }
        }
    }
}

/// The world points determining a cap plane, in forward swept order:
/// every loop vertex, plus every arc segment's apex. The apexes keep
/// 2-vertex loops (the minimal circle) plane-determining — Newell needs
/// three points and a 2-vertex cap has only two vertices — and they
/// carry the traversal's winding faithfully (each sits between its
/// segment's endpoints in loop order).
///
/// `qs` are the world vertices and `place` the matching placement, so
/// a rotated or translated cap passes the rotated or translated pair.
pub(crate) fn cap_points<T: Real, S: SweptChord<T>>(
    segs: &[S],
    qs: &[Point3<T>],
    place: Affine3<T>,
) -> Vec<Point3<T>> {
    let mut pts = Vec::with_capacity(segs.len() * 2);
    for (j, s) in segs.iter().enumerate() {
        pts.push(qs[j]);
        if matches!(s.kind(), SweptKind::Arc { .. }) {
            let apex = arc_apex(s.a(), s.b(), s.bulge());
            pts.push(place.transform_point(Point3::new(apex.x, apex.y, T::zero())));
        }
    }
    pts
}

/// The predicate names one verb's cosurface decision reports under —
/// the K recorder meters each sweep's line and arc margins separately,
/// so the names are per-verb data passed into the shared body rather
/// than a property of the body.
///
/// **A table of row names, so a new value here is a roster change**
/// (`docs/K-REPORT.md`, "The inventory method, restated"): a name
/// reaching the funnel as a field is invisible to any grep for a
/// literal at the decide site.
#[derive(Clone, Copy, Debug)]
pub(crate) struct CosurfaceNames {
    /// The line/line predicate (chord collinearity).
    pub(crate) lines: &'static str,
    /// The arc/arc predicate (carrier identity).
    pub(crate) arcs: &'static str,
}

/// Decides whether two consecutive segments sweep the
/// identical-by-construction surface (crate docs, cosurface sharing):
/// collinear lines share one surface, same-carrier same-turn arcs
/// share one. The sweep's own surface family does not enter — swept
/// surfaces of identical generators under one motion coincide, so the
/// margins are the sketch-level ones for every verb. Mixed kinds never
/// share (structurally); arcs with opposite turns never share
/// (structurally — a same-carrier opposite-turn pair is an overlap the
/// profile validator already refused; checked defensively).
pub(crate) fn cosurface<T: Decide, S: SweptChord<T>>(
    prev: &S,
    next: &S,
    names: CosurfaceNames,
    band: Band,
) -> Result<bool, Indeterminate> {
    match (prev.kind(), next.kind()) {
        (SweptKind::Line, SweptKind::Line) => {
            // Margin: perpendicular distance of the next chord's far
            // endpoint from the previous chord's carrier line (meters,
            // direct displacement).
            let t = (prev.b() - prev.a()).normalize();
            let d = next.b() - prev.a();
            let margin = t.perp_dot(d);
            Ok(matches!(
                decide(names.lines, Margin::of(margin), band)?,
                Sign::Zero
            ))
        }
        (
            SweptKind::Arc {
                center: c1,
                radius: r1,
                turn: t1,
            },
            SweptKind::Arc {
                center: c2,
                radius: r2,
                turn: t2,
            },
        ) => {
            if t1 != t2 {
                return Ok(false);
            }
            // Margin: center distance plus radius difference (meters,
            // direct — the profile crate's carrier-identity pattern).
            let margin = c1.distance(c2) + (r1 - r2).abs();
            Ok(matches!(
                decide(names.arcs, Margin::of(margin), band)?,
                Sign::Zero
            ))
        }
        _ => Ok(false),
    }
}

/// Resolves a face's surface key (total: a stale key surfaces as the
/// operator-layer typed error, which every sweep verb's error enum
/// absorbs through its `From<EulerOpError>`).
pub(crate) fn face_surface_key<T: Real>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<SurfaceKey, EulerOpError> {
    Ok(body
        .get_face(face)
        .ok_or(EulerOpError::StaleKey {
            key: topo::EntityId::Face(face),
        })?
        .surface)
}

/// Every edge of `face` still described through the **scaffolding
/// door**, re-stated as an image in that face's OWN chart — carrier
/// and interval verbatim, the pushforward it was scaffolded from kept
/// as its authority record (`EdgeCurveSpec::at_rest_in_chart`).
///
/// D3's transience fence: the door is for edges whose surfaces do not
/// exist yet. A cap's rim is minted before the cap's plane is known
/// (the plane is fitted THROUGH the rim), so it must go through the
/// door — and the moment the plane exists the rim is at rest in it and
/// says so. Edges the construction has already described some other
/// way (a cap–wall intersection, a wall's boundary iso) are left
/// alone: this states what THIS face knows about its own boundary, it
/// does not re-derive anyone else's description.
pub(crate) fn describe_face_rim_at_rest<T: Decide>(
    body: &mut Body<T>,
    face: FaceKey,
    tol: Tol,
) -> Result<(), EulerOpError> {
    let chart = face_surface_key(body, face)?;
    let stale = || EulerOpError::StaleKey {
        key: topo::EntityId::Face(face),
    };
    let face_data = body.get_face(face).ok_or_else(stale)?.clone();
    let mut edges: Vec<topo::EdgeKey> = Vec::new();
    for lk in core::iter::once(&face_data.outer).chain(&face_data.rings) {
        let topo::LoopBoundary::Cycle { first } = body.get_loop(*lk).ok_or_else(stale)?.boundary
        else {
            continue;
        };
        for he in body.loop_cycle(first).ok_or_else(stale)? {
            edges.push(body.get_half_edge(he).ok_or_else(stale)?.edge);
        }
    }
    for edge in edges {
        let curve_key = body
            .get_edge(edge)
            .ok_or(EulerOpError::StaleKey {
                key: topo::EntityId::Edge(edge),
            })?
            .curve;
        let Some(curve) = body
            .get_curve_geom(curve_key)
            .and_then(topo::CurveGeom::certified)
        else {
            continue; // null scaffolding carries no description at all
        };
        if !matches!(curve.description(), geom_brep::EdgeDescription::Scaffold(_)) {
            continue;
        }
        let spec = curve.restated_spec().at_rest_in_chart(chart, false);
        body.set_edge_curve(edge, spec, tol)?;
    }
    Ok(())
}
