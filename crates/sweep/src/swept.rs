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
//! given centre and radii bit-exactly, with no loop to validate — so it
//! cannot take a `ValidatedLoop` and cannot come through here. It
//! applies the same reversal convention by hand and
//! **says so at its own site**; that marker is the only thing tying
//! the two together, and it is deliberately not deleted.

use geom::Curve3;
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, MappedCurve, SketchSegment};
use geom_core::{
    Affine3, Arc2, Band, Decide, Indeterminate, Margin, Point2, Point3, Real, Sign, Tol, Vec3,
};
use profile::SegmentKind;
use topo::{
    Body, EdgeKey, EulerOpError, FaceKey, FaceSurface, HalfEdgeKey, MefSite, MevSite, SurfaceKey,
};

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
/// having no false negatives.
///
/// There is a **third path**, and the grep does not see it: a
/// `geom-brep` predicate this crate calls records under its own name,
/// from its own crate. `geom_brep::classify_dihedral`,
/// `classify_material_pairing` and — since FILLET-H6 hoisted the
/// must-carry rule out of two hand-rolled spellings here —
/// `geom_brep::tangent_second_order` each reach the recorder that way,
/// so `dihedral_arm`, `dihedral_wedge`, `material_wedge_side` and
/// `tangent_second_order` appear in this crate's K stream with no
/// `k_stats::decide` anywhere in `crates/sweep/src` to grep for. That
/// is the intended shape (one home per predicate, wherever the
/// predicate lives), not a leak — but a reader counting this crate's
/// recorder sites from the command above will undercount by it.
/// Stated as the command plus its one blind spot, rather than as a
/// count or a list of names.
pub(crate) fn decide<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Result<Sign, Indeterminate> {
    geom_core::k_stats::decide(name, margin, band)
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
/// Total by design: the turn is read through [`turn_negates`], so a
/// `Zero` turn (unreachable for a classified arc) takes the convex arm
/// exactly when [`turn_axis`] takes the positive one. Decided here once
/// rather than at each consumer, which is the reason this is a
/// function and not a rule each verb spells for itself.
pub(crate) fn centre_on_material_side(canonical_turn: Sign) -> bool {
    !turn_negates(canonical_turn)
}

/// A carrier class in SWEPT traversal order: the validated
/// [`SegmentKind`] as a traversal carries it, its sweep and turn the
/// traversal's — negated and flipped where the traversal runs the
/// canonical segment backwards.
///
/// A type of its own, not the canonical kind: a validated segment's
/// kind is oriented by the profile's canonical winding, and a body that
/// reads a traversal's orientation must not be handed that one. The
/// mints are the two traversals below and [`Traversed::half_turn`],
/// which takes no kind, so a canonical kind reaches a traversal's
/// reader through [`swept_segments`] or not at all.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Traversed<T: Real>(SegmentKind<T>);

impl<T: Real> Traversed<T> {
    /// A canonical kind traversed forward: itself.
    fn forward(kind: SegmentKind<T>) -> Self {
        Self(kind)
    }

    /// A canonical kind traversed backward: an arc's carrier kept, its
    /// sweep reversed ([`Arc2::reversed`]) and its turn flipped.
    ///
    /// Every variant answered by name, so a new sweep-bearing kind
    /// stops this compiling until its reversal is written here.
    fn backward(kind: SegmentKind<T>) -> Self {
        Self(match kind {
            SegmentKind::Arc { arc, turn } => SegmentKind::Arc {
                arc: arc.reversed(),
                turn: turn.flip(),
            },
            SegmentKind::Line => SegmentKind::Line,
        })
    }

    /// The half turn about `centre` of radius `radius`, traversed in
    /// the sense `turn` — for a builder with no validated loop to
    /// traverse (`revolve::tube`, which stores its caller's centre and
    /// radius). It takes no kind, so a canonical one cannot pass through
    /// it: the traversal is built here, in `turn`'s own orientation.
    ///
    /// The half-turn is spelled as the arc lowering spells a unit-bulge
    /// arc's sweep (`4·atan 1`), not as `T::pi()`: the certifier samples
    /// it at fractions `i/8`, and the symbolic tier folds the trig of
    /// `q·atan 1` in closed form (rule D) where a fraction of `π` other
    /// than a half-multiple stays an atom.
    pub(crate) fn half_turn(centre: Point2<T>, radius: T, turn: Sign) -> Self {
        let half = Arc2 {
            centre,
            radius,
            sweep: T::from_f64(4.0) * T::one().atan(),
        };
        let arc = if turn_negates(turn) {
            half.reversed()
        } else {
            half
        };
        Self(SegmentKind::Arc { arc, turn })
    }

    /// The run this traversal continues into `next` on one carrier
    /// (the full revolve's collapsed run, `revolve::full::Collapsed`):
    /// a line stays a line; an arc keeps its carrier and turn, its
    /// sweep the two summed — same turn, so same sign. A pair of
    /// different kinds is no run (the cosurface verdict never joins
    /// one).
    pub(crate) fn continued(self, next: Self) -> Self {
        Self(match (self.0, next.0) {
            (SegmentKind::Line, SegmentKind::Line) => SegmentKind::Line,
            (SegmentKind::Arc { arc, turn }, SegmentKind::Arc { arc: more, .. }) => {
                SegmentKind::Arc {
                    arc: Arc2 {
                        sweep: arc.sweep + more.sweep,
                        ..arc
                    },
                    turn,
                }
            }
            _ => unreachable!("a run never joins a line and an arc"),
        })
    }

    /// The carrier class, in this traversal's orientation.
    pub(crate) fn get(self) -> SegmentKind<T> {
        self.0
    }
}

/// The sketch-level chord data this module's lowering reads from a
/// swept segment: endpoints in swept traversal order, and the carrier
/// class (with an arc's carrier and sweep) in that order.
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
    /// The carrier class in swept traversal order.
    fn kind(&self) -> Traversed<T>;
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
    /// The carrier class in swept traversal order.
    pub(crate) kind: Traversed<T>,
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
    fn kind(&self) -> Traversed<T> {
        self.kind
    }
}

impl<T: Real> SweptSeg<T> {
    /// Segment `j` of the FORWARD traversal of `lp` — canonical segment
    /// `j` as itself, which is what the loft's walls read
    /// (`skin::vertex_segment`) outside a swept loop, and what
    /// [`swept_segments`] builds for every `j` when it does not reverse.
    pub(crate) fn forward(lp: &profile::ValidatedLoop<T>, j: usize) -> Self {
        let s = &lp.segments()[j];
        Self {
            a: s.start,
            b: s.end,
            kind: Traversed::forward(s.kind),
            canonical_vertex: j,
            canonical_segment: j,
        }
    }
}

/// Builds the swept traversal of one canonical loop: forward, or
/// reversed via the profile crate's reversal involution (endpoints
/// swapped, sweep negated, turn flipped; the carrier kept).
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
    (0..n)
        .map(|j| {
            if !reverse {
                return SweptSeg::forward(lp, j);
            }
            let s = &segs[n - 1 - j];
            SweptSeg {
                a: s.end,
                b: s.start,
                kind: Traversed::backward(s.kind),
                canonical_vertex: (n - j) % n,
                canonical_segment: n - 1 - j,
            }
        })
        .collect()
}

/// The segment as a `geom-brep` sketch segment (the description's
/// authoritative source data): the endpoints verbatim, and an arc's
/// carrier and sweep as the traversal carries them.
///
/// A free function over the accessors rather than a provided method:
/// the point of the trait is that the three accessors are all a verb
/// gets to supply, and a provided method is one an impl may quietly
/// override — which would put the body back to two.
pub(crate) fn sketch_segment<T: Real, S: SweptChord<T>>(seg: &S) -> SketchSegment<T> {
    let (a, b) = (seg.a(), seg.b());
    match seg.kind().get() {
        SegmentKind::Line => SketchSegment::Line { a, b },
        SegmentKind::Arc { arc, .. } => SketchSegment::Arc { a, b, arc },
    }
}

/// The arc parameter span |Δθ|: the sweep signed by the segment's
/// decided turn, read by [`turn_negates`] — a clockwise arc's span is
/// its reversal's sweep ([`Arc2::reversed`]), as in [`turn_axis`].
///
/// The turn is the profile's certified sign of the sweep, so this is
/// `|sweep|` to the bit at `f64` and `|sweep|`'s enclosure at
/// `Interval`. It is spelled through the turn, not `abs`, so that at
/// `Sym` it is `±sweep`, the node [`SketchSegment::eval`] turns
/// through, which rule D folds (`geom_core::sym::trig`); `abs(sweep)`
/// would be an opaque atom.
pub(crate) fn arc_span<T: Real>(turn: Sign, arc: Arc2<T>) -> T {
    if turn_negates(turn) {
        arc.reversed().sweep
    } else {
        arc.sweep
    }
}

/// **The crate's one reading of a turn**: `true` for a clockwise
/// (`Negative`) turn. [`turn_axis`], [`arc_span`],
/// [`centre_on_material_side`] and [`Traversed::half_turn`]
/// (`revolve::tube`'s circle traversal) all read it here, so `Zero` — unreachable for a classified arc, whose
/// turn is a certified non-zero sign — takes the positive arm in every
/// one of them at once. Total rather than loud for that reason: no
/// consumer can part from another on it.
pub(crate) fn turn_negates(turn: Sign) -> bool {
    matches!(turn, Sign::Negative)
}

/// The turn-signed carrier axis (crate docs): `+normal` for a
/// counterclockwise segment, `−normal` for a clockwise one, by
/// [`turn_negates`].
pub(crate) fn turn_axis<T: Real>(turn: Sign, normal: Vec3<T>) -> Vec3<T> {
    if turn_negates(turn) {
        Vec3::zero() - normal
    } else {
        normal
    }
}

/// **A latitude circle's rim identity, registered** (M10-9;
/// ERROR-DESIGN E12's "kept in reserve — discharge by provenance",
/// taken): the distance from the point a circle carrier was built
/// through to its centre IS its radius, stated by the builder that
/// guarantees it ([`geom_core::Real::register_equal`]). Its callers are
/// the revolve's latitude carriers (`revolve::surfaces`,
/// `revolve::full`), and the comment at each carries the theorem.
///
/// **Why the tier cannot prove it for itself, measured.** The squared
/// identity is a plain-form theorem wherever the coefficient ring can
/// afford the expansion, and the unsquared one needs the outer `sqrt`
/// discharged against an `abs` — rule C's shape, which folds on no
/// document at the shipped 256-bit ring and needs ~640 bits and up at
/// a leaf cost of minutes (`geom_core::sym`'s module docs,
/// M10's closed `plate-rim-residual-needs-the-wide-coefficient-ring`).
/// The ring width is a COST wall, and
/// this door is the recourse E12 named for exactly that case.
///
/// **What it touches: nothing.** `rim.norm()` is the node
/// `rim.normalize()` already divides by (`Vec3::normalize` is
/// `self / self.norm()` and node ids are content hashes), so the
/// registrant builds no expression the carrier did not already build,
/// and no value anywhere changes.
///
/// `tol` is the run's ε, which the door's inexact witnesses compare at
/// ([`geom_core::Real::register_equal`]). It ARRIVES from the caller —
/// every registrant on this path is reached from a builder that already
/// holds one, and kernel library code may not mint a tolerance witness.
pub(crate) fn register_rim_identity<T: Real>(rim: Vec3<T>, radius: T, tol: Tol) {
    rim.norm()
        .register_equal(radius, tol)
        .handle("a latitude carrier's ‖q − c‖ from its radius");
}

/// **A placed profile arc's rim IS its sketch rim** — the one fact the
/// sweep registers about a profile arc, rigidity, stated where it is
/// guaranteed ([`geom_core::Real::register_equal`]).
///
/// **The proof.** `rim` is `place(start) − place(centre)` for the arc's
/// swept start and its stored centre, and a rigid placement's linear
/// part is orthonormal, so `‖rim‖ = ‖start − centre‖` — the sketch rim
/// [`Arc2::rim`] spells. Where a caller's placement is NOT rigid the
/// door's own witness refuses the registration typed rather than
/// believing this paragraph (`Disputed` at an inexact witness, and at
/// [`geom_core::Interval`] the exact witness's `Contradicted`, which
/// `SymRegistration::handle` turns into an assertion).
///
/// **What it does not state, and why it need not.** The sketch rim's
/// own identity, `‖start − centre‖ = radius`, is the arc's
/// construction's to register — the profile's lowering does, on the
/// values it built (`Arc2::register_endpoints`) — and the tier's alias
/// is transitive, so this record chains the placed rim through the
/// sketch rim to the radius. A carrier the construction did not
/// register (copied across scalars, or written by a fixture) claims
/// nothing beyond rigidity, which is the ruling's point: nothing about
/// an arc's consistency is stored on it or claimed by a copy of it.
///
/// **What it touches: nothing.** `rim.norm()` is the node
/// `rim.normalize()` divides by and `arc.rim(start)` the node the
/// construction registered, so no value changes anywhere.
pub(crate) fn register_rigidity<T: Real>(rim: Vec3<T>, arc: Arc2<T>, start: Point2<T>, tol: Tol) {
    rim.norm()
        .register_equal(arc.rim(start), tol)
        .handle("a placed arc's ‖q − c‖ from its sketch rim");
}

/// **A placed arc's carrier end IS its sketch carrier end, placed** —
/// the second fact of rigidity the sweep registers about a profile arc
/// ([`geom_core::Real::register_equal`]), per component.
///
/// **The proof.** The carrier is the circle about `place(centre)` with
/// radius `radius`, reference direction `u_ref = rim/‖rim‖` for
/// `rim = place(start) − place(centre)`, and axis the turn-signed plane
/// normal, evaluated at the span `|Δθ|`. A rigid placement maps the
/// sketch plane's rotation about `centre` by the signed sweep onto the
/// 3-space rotation about that axis by `|Δθ|`, and the unit direction
/// `(start − centre)/‖start − centre‖` onto `u_ref`, so the carrier at
/// its span is `place` of the sketch carrier's own end,
/// [`Arc2::carrier_end`] from `start`. Nothing in it reads whether
/// `start` lies on the carrier, so it holds for every carrier, a copied
/// or table one included. Where the placement is not rigid, the door's
/// own witness refuses typed, as at [`register_rigidity`].
///
/// **What it chains through.** The construction registers the sketch
/// carrier end against the arc's far vertex (`Arc2::register_endpoints`),
/// which is the step that needs the start on the carrier; the tier's
/// alias applies inside the early walk, so the placed carrier end's form
/// is the placed far vertex's, and the carrier's far end reaches `q_to`
/// without a registration of the whole point against it. A carrier the
/// construction did not register claims nothing past rigidity, and its
/// far end discharges numerically or escalates.
///
/// **What it touches: nothing.** `Curve3::circle_at` over the spec's
/// own carrier and span is the node the certifier evaluates; the
/// placed carrier end is built here and thrown away.
fn register_placed_carrier_end<T: Real>(
    carrier: &Curve3<T>,
    param_end: T,
    place: Affine3<T>,
    arc: Arc2<T>,
    start: Point2<T>,
    tol: Tol,
) {
    let Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    } = *carrier
    else {
        return;
    };
    let end = Curve3::circle_at(center, axis, radius, u_ref, param_end);
    let sketch = arc.carrier_end(start);
    let placed = place.transform_point(Point3::new(sketch.x, sketch.y, T::zero()));
    for (built, held) in [(end.x, placed.x), (end.y, placed.y), (end.z, placed.z)] {
        built
            .register_equal(held, tol)
            .handle("a placed arc's carrier end from its placed sketch carrier end");
    }
}

/// The edge spec of a profile segment carried into 3-space by one
/// placement: `PlacedSegment` description, line or circle carrier per
/// the crate docs' carrier conventions (arc axis = turn-signed plane
/// normal, span |Δθ| from the segment's sweep and turn — see
/// [`arc_span`]).
///
/// `place` and `normal` are the placement the segment is lowered
/// through and its plane normal — the sketch placement for a base
/// lamina, the translated or rotated one for the swept copy. `tol` is
/// the run's ε, carried through to the rigidity the arc arm states
/// ([`register_rigidity`], [`register_placed_carrier_end`]) and used for
/// nothing else here.
pub(crate) fn placed_segment_spec<T: Real, S: SweptChord<T>>(
    seg: &S,
    place: Affine3<T>,
    normal: Vec3<T>,
    q_from: Point3<T>,
    q_to: Point3<T>,
    tol: Tol,
) -> EdgeCurveSpec<T> {
    let description = EdgeDescriptionSpec::Scaffold(MappedCurve::PlacedSegment {
        segment: sketch_segment(seg),
        place,
    });
    match seg.kind().get() {
        SegmentKind::Line => EdgeCurveSpec {
            description,
            carrier: Curve3::Line {
                origin: q_from,
                dir: (q_to - q_from).normalize(),
            },
            param_start: T::zero(),
            param_end: q_from.distance(q_to),
        },
        SegmentKind::Arc { arc, turn } => {
            let c_world = place.transform_point(Point3::new(arc.centre.x, arc.centre.y, T::zero()));
            let rim = q_from - c_world;
            // Rigidity, stated where it is guaranteed
            // (`register_rigidity` carries the proof). Bound out of the
            // expression below rather than spelled twice: one
            // subtraction, one node, one set of bits.
            register_rigidity(rim, arc, seg.a(), tol);
            let carrier = Curve3::Circle {
                center: c_world,
                axis: turn_axis(turn, normal),
                radius: arc.radius,
                u_ref: rim.normalize(),
            };
            let param_end = arc_span(turn, arc);
            // The carrier end, stated where it is guaranteed
            // (`register_placed_carrier_end` carries the proof), about the
            // very carrier and span the spec carries.
            register_placed_carrier_end(&carrier, param_end, place, arc, seg.a(), tol);
            EdgeCurveSpec {
                description,
                carrier,
                param_start: T::zero(),
                param_end,
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
        if let SegmentKind::Arc { arc, .. } = s.kind().get() {
            let apex = arc.apex(s.a(), s.b());
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
    match (prev.kind().get(), next.kind().get()) {
        (SegmentKind::Line, SegmentKind::Line) => {
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
        (SegmentKind::Arc { arc: a1, turn: t1 }, SegmentKind::Arc { arc: a2, turn: t2 }) => {
            if t1 != t2 {
                return Ok(false);
            }
            // Margin: center distance plus radius difference (meters,
            // direct — the profile crate's carrier-identity pattern).
            let margin = a1.centre.distance(a2.centre) + (a1.radius - a2.radius).abs();
            Ok(matches!(
                decide(names.arcs, Margin::of(margin), band)?,
                Sign::Zero
            ))
        }
        _ => Ok(false),
    }
}

/// One wall's run of a swept loop (crate README, "Walls: one per
/// run"): segments `first, first + 1, …, first + len − 1` (mod n) in
/// swept order. Vertex `first` carries the wall's leading strut; the
/// `len − 1` vertices after it are the run's stations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Run {
    /// The run's first segment (and leading vertex), swept order.
    pub(crate) first: usize,
    /// How many segments the run holds (≥ 1).
    pub(crate) len: usize,
}

impl Run {
    /// The run's segments, swept order, wrapping at `n`.
    pub(crate) fn segments(self, n: usize) -> impl Iterator<Item = usize> {
        (0..self.len).map(move |k| (self.first + k) % n)
    }

    /// The vertex the run ends at: the next run's leading vertex.
    pub(crate) fn end(self, n: usize) -> usize {
        (self.first + self.len) % n
    }
}

/// The wall runs of one swept loop, in ascending order of their first
/// segment, read off the loop's cosurface verdicts: `pair[j]` says
/// segment `j` continues segment `j − 1`'s carrier (`pair[0]` is the
/// wrap join), and `walled(j)` whether segment `j` sweeps a wall.
///
/// A run joins collinear lines, and cocircular same-turn arcs where
/// `arcs` says the verb builds a curved run whole (crate README, "Walls:
/// one per run"); an arc it does not join keeps its own wall on the
/// run's one surface key ([`shared_wall`]). A loop every join of which
/// continues one carrier is a circle cut into arcs — collinear lines
/// cannot close a simple loop — and it keeps its canonical cut (C12.5):
/// each arc its own run, the walls sharing one surface key across the
/// meridian struts between them.
pub(crate) fn wall_runs<T: Real, S: SweptChord<T>>(
    segs: &[S],
    pair: &[bool],
    walled: impl Fn(usize) -> bool,
    arcs: CurvedRuns,
) -> Vec<Run> {
    let n = segs.len();
    let is_line = |j: usize| matches!(segs[j].kind().get(), SegmentKind::Line);
    let joined = |j: usize| {
        let p = (j + n - 1) % n;
        pair[j] && walled(p) && walled(j) && (arcs == CurvedRuns::Whole || is_line(j))
    };
    let starts: Vec<usize> = (0..n).filter(|&j| !joined(j)).collect();
    if starts.is_empty() {
        // Every join continues one carrier, and a cosurface pair never
        // mixes kinds, so the loop is all lines or all arcs.
        if is_line(0) {
            unreachable!("a run of collinear lines closes the whole loop, which validation refuses");
        }
        return (0..n).map(|first| Run { first, len: 1 }).collect();
    }
    starts
        .iter()
        .enumerate()
        .map(|(i, &first)| {
            let next = starts.get(i + 1).copied().unwrap_or(starts[0] + n);
            Run {
                first,
                len: next - first,
            }
        })
        .collect()
}

/// A loop's cosurface verdicts, decided UP FRONT from the segments
/// alone (deterministic — no body state) and before any wall is minted:
/// `pair[j]` says whether segment `j` continues segment `j − 1 mod n`'s
/// carrier, so `pair[0]` is the wrap join. A pair across an unwalled
/// segment (`walled`) is structurally false. Deciding all n first is
/// what lets a run crossing the canonical start vertex resolve to ONE
/// wall. `escalated(j, source)` is the verb's typed refusal for an
/// in-band verdict at vertex `j`.
pub(crate) fn cosurface_pairs<T: Decide, S: SweptChord<T>, E>(
    segs: &[S],
    walled: impl Fn(usize) -> bool,
    names: CosurfaceNames,
    band: Band,
    escalated: impl Fn(usize, Indeterminate) -> E,
) -> Result<Vec<bool>, E> {
    let n = segs.len();
    (0..n)
        .map(|j| {
            let p = (j + n - 1) % n;
            if !(walled(p) && walled(j)) {
                return Ok(false);
            }
            cosurface(&segs[p], &segs[j], names, band).map_err(|source| escalated(j, source))
        })
        .collect()
}

/// Which vertices lead a run (carry its strut / start meridian), by
/// swept position.
pub(crate) fn run_leads(runs: &[Run], n: usize) -> Vec<bool> {
    let mut lead = vec![false; n];
    for run in runs {
        lead[run.first] = true;
    }
    lead
}

/// Whether a verb's run joins cocircular arcs ([`wall_runs`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CurvedRuns {
    /// A run of cocircular arcs is one wall.
    Whole,
    /// Each arc keeps its own wall, on the run's one surface key: the
    /// partial revolve, whose sphere and torus walls would carry a
    /// meridian in pieces mass properties do not fold
    /// (`work/band/partial-revolve-arc-runs-wait-on-the-meridian-fold.md`).
    Split,
}

/// The wall whose SURFACE KEY segment `j`'s wall shares, when `j`
/// leads a run that continues an earlier wall's carrier — an arc
/// [`wall_runs`] did not join, or a circle's canonical cut: `pair[j]`
/// shares the previous wall's key, and a run reaching `origin` (the
/// first run's lead) through the wrap shares the first wall's key. The
/// first run (rank 0) shares nothing. `faces` holds the walls minted so
/// far, by swept position.
pub(crate) fn shared_wall(
    pair: &[bool],
    faces: &[Option<FaceKey>],
    j: usize,
    origin: usize,
) -> Option<FaceKey> {
    let n = pair.len();
    let rank = |k: usize| (k + n - origin) % n;
    if rank(j) == 0 {
        None
    } else if pair[j] {
        faces[(j + n - 1) % n]
    } else if pair[origin] && ((rank(j) + 1)..n).all(|r| pair[(origin + r) % n]) {
        faces[origin]
    } else {
        None
    }
}

/// What [`build_run_walls`] minted, by swept position: each segment's
/// wall, and its far-side chain edge (extrude's top rim, a partial
/// revolve's end meridian). `None` where a run was not walled.
pub(crate) struct RunWalls {
    pub(crate) faces: Vec<Option<FaceKey>>,
    pub(crate) tops: Vec<Option<EdgeKey>>,
}

/// **The one run-wall builder** extrude and the partial revolve share
/// (crate README, "Walls: one per run"). For each run in order, from
/// `at(first)` — the strut's minus half at the run's lead — a
/// `mev` chain lays the far-side edge of every segment but the last,
/// minting each station's far vertex (`chain(s)`: the far end of
/// segment `s` and its edge spec, or the caller's typed refusal); then the closing `mef` lays the last
/// segment's edge and splits the wall off. It closes against the first
/// far-side half the first run laid when the run ends at the first
/// run's lead (the strut minus there was consumed), and against
/// `at(end)` otherwise. `wall(body, run, faces)` gives the
/// closing edge's spec and the wall's surface, or `None` for a run
/// that sweeps no wall (a revolve's on-axis segment).
pub(crate) fn build_run_walls<T: Decide, E: From<EulerOpError>>(
    body: &mut Body<T>,
    runs: &[Run],
    n: usize,
    at: impl Fn(usize) -> HalfEdgeKey,
    mut chain: impl FnMut(usize) -> Result<(Point3<T>, EdgeCurveSpec<T>), E>,
    mut wall: impl FnMut(
        &mut Body<T>,
        Run,
        &[Option<FaceKey>],
    ) -> Result<Option<(EdgeCurveSpec<T>, FaceSurface<T>)>, E>,
    tol: Tol,
) -> Result<RunWalls, E> {
    let mut faces: Vec<Option<FaceKey>> = vec![None; n];
    let mut tops: Vec<Option<EdgeKey>> = vec![None; n];
    let mut first_top: Option<HalfEdgeKey> = None;
    let origin = runs.first().map_or(0, |r| r.first);
    for (ri, run) in runs.iter().enumerate() {
        let Some((closing, surface)) = wall(body, *run, &faces)? else {
            continue;
        };
        let mut he1 = at(run.first);
        let segments: Vec<usize> = run.segments(n).collect();
        let Some((&last, stations)) = segments.split_last() else {
            unreachable!("a wall run holds at least one segment")
        };
        for (k, &s) in stations.iter().enumerate() {
            let (far, spec) = chain(s)?;
            let m = body.mev(MevSite::Fan { he1, he2: he1 }, far, spec, tol)?;
            if ri == 0 && k == 0 {
                first_top = Some(m.he_plus);
            }
            tops[s] = Some(m.edge);
            he1 = m.he_minus;
        }
        let end = run.end(n);
        let he2 = match first_top {
            Some(top) if end == origin => top,
            _ => at(end),
        };
        let mef = body.mef(MefSite::Chords { he1, he2 }, closing, surface, tol)?;
        if ri == 0 && first_top.is_none() {
            first_top = Some(mef.he_plus);
        }
        tops[last] = Some(mef.edge);
        for &s in &segments {
            faces[s] = Some(mef.face);
        }
    }
    Ok(RunWalls { faces, tops })
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
/// way (a wall's boundary iso, a rim a dihedral pass upgraded to an
/// intersection) are left alone: this states what THIS face knows
/// about its own boundary, it does not re-derive anyone else's
/// description.
///
/// No dihedral is read here. Its one caller is loft's two caps, whose
/// every edge is a cap–wall rim between a plane and a `Surface::Nurbs`
/// wall; D2 exempts NURBS-adjacent edges from the must-carry demand,
/// and loft's module doc says these rims are never classified.
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
        let scaffolded = body
            .get_curve_geom(curve_key)
            .and_then(topo::CurveGeom::certified)
            // Null scaffolding carries no description at all.
            .is_some_and(|c| matches!(c.description(), geom_brep::EdgeDescription::Scaffold(_)));
        if !scaffolded {
            continue;
        }
        body.describe_at_rest(edge, chart, tol)?;
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::sym::{session_counts, with_session};
    use geom_core::{Bounds, Interval, ParamSymbol, Sym, SymBudget};

    /// **The cap apex stays at the chord's scale at `Interval`.** Over
    /// the shallow-arc grid (chord `L` ∈ {1e-3, 1, 50}, bulge down to
    /// 1e-5, exact endpoints), the apex enclosure is no wider than a
    /// few ulps of the chord. The carrier spelling
    /// `centre − n̂·σ·radius` fails this by five decades on the flat
    /// arcs (3.6e-12 at unit chord and b = 1e-4): the centre carries
    /// the chord's width amplified by the radius.
    #[test]
    fn the_apex_enclosure_is_chord_scale_on_flat_arcs() {
        let iv = Interval::from_f64;
        for l in [1e-3, 1.0, 50.0] {
            for b in [0.5, 1e-2, 1e-3, 1e-4, 1e-5] {
                let (a, e) = (
                    Point2::new(iv(-l / 2.0), iv(0.0)),
                    Point2::new(iv(l / 2.0), iv(0.0)),
                );
                let lp = profile::test_support::bulge_loop(vec![
                    (a, iv(b)),
                    (e, iv(0.0)),
                    (Point2::new(iv(0.0), iv(-l)), iv(0.0)),
                ]);
                let profile::Segment::Arc(arc) = lp.segments()[0] else {
                    panic!("a nonzero bulge lowers to an arc");
                };
                let apex = arc.apex(a, e);
                let width = (apex.x.hi() - apex.x.lo()).max(apex.y.hi() - apex.y.lo());
                assert!(
                    width <= 16.0 * f64::EPSILON * l,
                    "L = {l:e}, b = {b:e}: the apex enclosure is {width:e} wide, past the \
                     chord's own scale"
                );
            }
        }
    }

    /// Where the arc [`placed_arc_readings`] places comes from.
    #[derive(Clone, Copy)]
    enum Carrier {
        /// Constructed at `Sym<Interval>` through the lattice, over a
        /// box of bulges: its lowering registers the endpoint facts.
        Constructed,
        /// Constructed at f64 and copied into `Sym<Interval>` field by
        /// field, as the pinned lift copies it: nothing registers its
        /// endpoint facts, and its rim is the radius only up to the f64
        /// rounding it was built with.
        Copied,
    }

    /// What the first arc of a lattice-lowered loop decides, placed, at
    /// `Sym<Interval>` under the shipped rules: the placed rim against
    /// the radius, the same residual inside a larger expression, the
    /// carrier's far end against the far vertex (one row per
    /// component), and the session's receipt.
    fn placed_arc_readings(
        place: Affine3<geom_core::Sym<Interval>>,
        carrier: Carrier,
    ) -> ([Option<Sign>; 5], geom_core::SymCounts) {
        use geom_core::sym::with_session_rules;
        use geom_core::{ParamSymbol, Sym, SymBudget, SymRules};
        use profile::{Bulge, Open, Start};
        type S = Sym<Interval>;
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the witness band");
        let budget = SymBudget {
            max_terms: 4096,
            max_degree: 128,
        };
        with_session_rules(budget, SymRules::shipped(), || {
            let lit = |x: f64| <S as Real>::from_f64(x);
            let (a, e, b, turn) = match carrier {
                Carrier::Constructed => {
                    // A clockwise arc bowing up off the top of a unit
                    // square, over a bulge box wide enough that the
                    // numeric channel cannot decide the rim.
                    let b = S::param(ParamSymbol::of("b"), Interval::from_bounds(-0.55, -0.45));
                    let closed = Open
                        .at(Point2::new(lit(0.0), lit(0.0)))
                        .arc_to(
                            Bulge {
                                p: Point2::new(lit(1.0), lit(0.0)),
                                b,
                            },
                            tol,
                        )
                        .expect("the arc authors")
                        .line_to(Point2::new(lit(1.0), lit(-1.0)), tol)
                        .expect("a leg down")
                        .line_to(Point2::new(lit(0.0), lit(-1.0)), tol)
                        .expect("a leg back")
                        .line_to(Start, tol)
                        .expect("the seam closes");
                    // The lowered arc straight off the stored loop: the
                    // chain under test is the lowering's registrations
                    // and the sweep's, and validation is not in it.
                    let lp = closed.loop_;
                    let profile::Segment::Arc(arc) = lp.segments()[0] else {
                        panic!("the first segment is the authored arc");
                    };
                    (lp.vertices()[0], lp.vertices()[1], (arc, b), Sign::Negative)
                }
                Carrier::Copied => {
                    // An ordinary arc whose f64 rim and radius enclose
                    // disjointly once copied into `Interval`.
                    let (a, e) = (
                        Point2::new(-79.674_068_761_865_71, -8.743_422_184_344_226),
                        Point2::new(-79.456_229_843_363_16, -7.494_651_585_468_935),
                    );
                    let closed = Open
                        .at(a)
                        .arc_to(
                            Bulge {
                                p: e,
                                b: 1.649_230_685_601_469_6,
                            },
                            tol,
                        )
                        .expect("the arc authors at f64")
                        .line_to(Start, tol)
                        .expect("the seam closes at f64");
                    let profile::Segment::Arc(arc) = closed.loop_.segments()[0] else {
                        panic!("the first segment is the authored arc");
                    };
                    (
                        a.map(lit),
                        e.map(lit),
                        (arc.map(lit), lit(1.0)),
                        Sign::Positive,
                    )
                }
            };
            let (arc, b) = b;
            let seg = SweptSeg {
                a,
                b: e,
                kind: Traversed::forward(SegmentKind::Arc { arc, turn }),
                canonical_vertex: 0,
                canonical_segment: 0,
            };
            let to3 = |p: Point2<S>| place.transform_point(Point3::new(p.x, p.y, lit(0.0)));
            let (q_from, q_to) = (to3(seg.a), to3(seg.b));
            let spec = placed_segment_spec(&seg, place, place.linear.c2, q_from, q_to, tol);
            let Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } = spec.carrier
            else {
                panic!("an arc places onto a circle carrier");
            };
            let sign = |m: S| m.sign_within(band).map(|d| d.sign).ok();
            let rim = (q_from - center).norm();
            let end = Curve3::circle_at(center, axis, radius, u_ref, spec.param_end);
            [
                sign(rim - radius),
                sign((rim + b) * b - (radius + b) * b),
                sign(end.x - q_to.x),
                sign(end.y - q_to.y),
                sign(end.z - q_to.z),
            ]
        })
    }

    /// A quarter turn about z followed by `shift`, at `Sym<Interval>`.
    fn quarter_turn_placement(shift: [f64; 3]) -> Affine3<geom_core::Sym<Interval>> {
        let lit = |x: f64| <geom_core::Sym<Interval> as Real>::from_f64(x);
        let v = |x, y, z| Vec3::new(lit(x), lit(y), lit(z));
        Affine3::from_parts(
            geom_core::Mat3::from_cols(v(0.0, 1.0, 0.0), v(-1.0, 0.0, 0.0), v(0.0, 0.0, 1.0)),
            v(shift[0], shift[1], shift[2]),
        )
    }

    /// **A copied carrier claims nothing past rigidity, and never
    /// contradicts.** The arc was constructed at f64 and copied into
    /// `Sym<Interval>`, so no construction registered its endpoint facts
    /// at this scalar, and its f64 rim and radius enclose disjointly
    /// there. The sweep's registrations read only the carrier and the
    /// placement, so the exact witness separates none of them; the rim
    /// and far end decide from their values, not from a registered
    /// identity.
    #[test]
    fn a_copied_carrier_places_without_contradiction_and_claims_nothing() {
        let ([rim, _, end @ ..], counts) =
            placed_arc_readings(quarter_turn_placement([0.0; 3]), Carrier::Copied);
        assert_eq!(counts.registrations_refused, 0, "no refusal: {counts:?}");
        assert_eq!(
            counts.registered, 0,
            "no decision rests on a registered identity: {counts:?}"
        );
        assert_eq!(rim, Some(Sign::Zero), "the rim, numerically: {counts:?}");
        assert_eq!(
            end,
            [Some(Sign::Zero); 3],
            "the far end, numerically: {counts:?}"
        );
    }

    /// **The placed rim chains through the sketch rim to the radius, and
    /// the placed far end through the sketch carrier end to the far
    /// vertex.**
    /// The lowering registers the arc's 2-D rims against its radius
    /// (`Arc2::register_endpoints`) and the sweep registers only
    /// rigidity (`register_rigidity`): the placed rim against the
    /// sketch rim. The tier's alias is transitive, so the placed rim's
    /// residual against the radius decides `Zero` over the whole box —
    /// through a quarter turn and a translation, and inside a larger
    /// expression — as a registered identity, where the numeric channel
    /// alone cannot decide it over that box.
    #[test]
    fn the_placed_rim_chains_through_the_sketch_rim_to_the_radius() {
        let ([rim, inside, end @ ..], counts) = placed_arc_readings(
            quarter_turn_placement([2.0, 3.0, 5.0]),
            Carrier::Constructed,
        );
        assert_eq!(rim, Some(Sign::Zero), "the placed rim: {counts:?}");
        assert_eq!(
            inside,
            Some(Sign::Zero),
            "inside a larger expression: {counts:?}"
        );
        // The far end chains the same way: the carrier at its span to
        // the placed sketch carrier end (the sweep), the sketch carrier
        // end to the far vertex (the lowering), and `q_to` is `place` of
        // that vertex — one node, minted by the same op on the same
        // operand.
        assert_eq!(end, [Some(Sign::Zero); 3], "the far end: {counts:?}");
        assert!(counts.registered > 0, "decided as registered: {counts:?}");
        assert_eq!(counts.registrations_refused, 0, "no refusal: {counts:?}");
    }

    fn budget() -> SymBudget {
        SymBudget {
            max_terms: 4096,
            max_degree: 128,
        }
    }

    /// How the tier answers a residual at the scalar door: the scalar's
    /// own `Decide::sign_within` on the residual (no recorder funnel, so
    /// no predicate name joins the crate's roster), read off the
    /// session's receipt.
    fn how<T: Decide>(m: T) -> &'static str {
        let band = Band::linear(Tol::witness()).expect("the witness tolerance has a linear band");
        let before = session_counts().expect("inside a session");
        let _ = m.sign_within(band);
        let after = session_counts().expect("inside a session");
        if after.registered > before.registered {
            "registered"
        } else if after.sign_gated > before.sign_gated {
            "sign_gated"
        } else if after.symbolic_zero > before.symbolic_zero {
            "theorem"
        } else {
            "numeric"
        }
    }

    /// One arc at the bulge `bulge` (a form) with the turn `turn`, on
    /// the chord `(0, 0) → (2, 0)`, lowered through
    /// [`placed_segment_spec`] at the identity placement. The sweep is
    /// the lowering's `4·atan b`, and the centre and radius are the
    /// sagitta closed forms, so the two registrants the arm runs state
    /// true identities.
    fn lowered<T: Real>(bulge: T, turn: Sign) -> EdgeCurveSpec<T> {
        let lit = T::from_f64;
        let (a, b) = (
            Point2::new(lit(0.0), lit(0.0)),
            Point2::new(lit(2.0), lit(0.0)),
        );
        let len = lit(2.0);
        let apothem = len * (lit(1.0) - bulge * bulge) / (lit(4.0) * bulge);
        let radius = (len * (lit(1.0) + bulge * bulge) / (lit(4.0) * bulge)).abs();
        let seg = SweptSeg {
            a,
            b,
            kind: Traversed::forward(SegmentKind::Arc {
                arc: Arc2 {
                    centre: Point2::new(lit(1.0), apothem),
                    radius,
                    sweep: lit(4.0) * bulge.atan(),
                },
                turn,
            }),
            canonical_vertex: 0,
            canonical_segment: 0,
        };
        let q = |p: Point2<T>| Point3::new(p.x, p.y, lit(0.0));
        placed_segment_spec(
            &seg,
            Affine3::identity(),
            Vec3::new(lit(0.0), lit(0.0), lit(1.0)),
            q(a),
            q(b),
            Tol::witness(),
        )
    }

    /// The carrier's samples against the pushforward's, at `s = i/8`:
    /// the carrier at `t = s·param_end` about the turn-signed axis, the
    /// pushforward (`SketchSegment::eval`) at `s·θ`, `θ` the sweep the
    /// description carries. About `−n` the carrier turns by `−t` in the
    /// sketch plane, so its sine enters with the turn's sign. Per
    /// sample, how the tier answers the cosine and the sine.
    fn samples<T: Decide>(
        spec: &EdgeCurveSpec<T>,
        turn: Sign,
    ) -> Vec<(&'static str, &'static str)> {
        let lit = T::from_f64;
        let EdgeDescriptionSpec::Scaffold(MappedCurve::PlacedSegment {
            segment: SketchSegment::Arc { arc, .. },
            ..
        }) = spec.description
        else {
            panic!("an arc lowers to a placed arc segment");
        };
        let theta = arc.sweep;
        let sigma = lit(if turn_negates(turn) { -1.0 } else { 1.0 });
        (0..=8)
            .map(|i| {
                let s = lit(f64::from(i) / 8.0);
                let t = spec.param_start + (spec.param_end - spec.param_start) * s;
                let (st, ct) = t.sin_cos();
                let sin = (s * theta).sin();
                let cos = lit(1.0) - lit(2.0) * (s * theta * lit(0.5)).sin().powi(2);
                (how(ct - cos), how(sigma * st - sin))
            })
            .collect()
    }

    /// **The carrier's span meets the pushforward's as a THEOREM at a
    /// parameter bulge of either sign**, at `Sym<f64>` (a point) and at
    /// `Sym<Interval>` (a box that keeps the sign). The span is the
    /// stored sweep signed by the decided turn ([`arc_span`]), so the
    /// carrier and the pushforward read the one `atan b` atom and rule D
    /// folds both. Spelled through `abs` (`|sweep|`), the carrier mints
    /// an atom related to `atan b` only through the sign of `b`, and the
    /// sine stays numeric — so this row reds if the carrier's span goes
    /// back to `abs`. Per scalar, three arcs: a positive parameter
    /// bulge, a negative one, and the reversal of the positive one (the
    /// bulge `0 − b`, the turn flipped: `swept_segments`' involution).
    #[test]
    fn the_carriers_span_meets_the_pushforwards_at_a_parameter_bulge_of_either_sign() {
        let cases: [(&str, bool, bool, Sign); 3] = [
            ("b > 0", false, false, Sign::Positive),
            ("b < 0", true, false, Sign::Negative),
            ("0 − b, b > 0 (reversed)", false, true, Sign::Negative),
        ];
        for (name, negative, reversed, turn) in cases {
            let at = |b: f64| if negative { -b } else { b };
            let (rows, counts) = with_session(budget(), || {
                let b = Sym::<f64>::param(ParamSymbol::of("bulge"), at(0.7));
                let bulge = if reversed { Sym::zero() - b } else { b };
                samples(&lowered(bulge, turn), turn)
            });
            assert!(
                rows.iter().all(|r| *r == ("theorem", "theorem")),
                "Sym<f64>, {name}: every sample's cosine and sine must be a theorem: \
                 {rows:?} ({counts:?})"
            );
            let (lo, hi) = if negative {
                (-0.77, -0.63)
            } else {
                (0.63, 0.77)
            };
            let (rows, counts) = with_session(budget(), || {
                let b =
                    Sym::<Interval>::param(ParamSymbol::of("bulge"), Interval::from_bounds(lo, hi));
                let bulge = if reversed { Sym::zero() - b } else { b };
                samples(&lowered(bulge, turn), turn)
            });
            assert!(
                rows.iter().all(|r| *r == ("theorem", "theorem")),
                "Sym<Interval> over [{lo}, {hi}], {name}: every sample's cosine and sine \
                 must be a theorem: {rows:?} ({counts:?})"
            );
        }
    }
}
