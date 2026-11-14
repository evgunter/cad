//! Per-edge chord points: computed **once** from each edge's certified
//! carrier and consumed by both adjacent faces (the watertightness
//! half of the pure-function invariant, crate docs).
//!
//! Chord counts are deterministic ceil arithmetic from δ, in two steps.
//!
//! **The carrier's own count**: a chord's worst point lands at the
//! edge's target from its carrier (the edge-chord theorem at
//! `sizing::sagitta_step`). The target is the least of the two faces'
//! ([`edge_target`], `trimmed::face_target`): δ, or the headroom's share
//! of it beside an arm that sizes its lattice there, so that arm's trim
//! polygon is the one its lattice was sized beside.
//!
//! - Line carriers: 1 chord (a segment of the exact locus).
//! - Circle carriers (radius ρ, forward span Δt): per-chord step
//!   φ = 2·acos(1 − δ/ρ) (the closed-form sagitta bound
//!   ρ(1 − cos(φ/2)) ≤ δ), capped at π/4 (`sizing::MAX_ANGULAR_STEP`);
//!   n = ceil(Δt/φ).
//! - Ellipse, spiric and B-spline carriers: the curvature-bound step
//!   from each kind's closed-form or hull `sup|C″|`.
//!
//! **The one rule** ([`tighten`]): a face whose lane lays a lattice
//! certifies triangles about that lattice's cells, so a boundary chord
//! may span at most one lattice step on each chart axis. On each side
//! whose face has a lattice, `n ≥ ⌈s_u·Δt/h_u⌉` and `⌈s_v·Δt/h_v⌉`, with
//! `(h_u, h_v)` the face's steps at its target ([`FaceLattice::of`]) and
//! `(s_u, s_v)` the per-axis speed of the image the face's lane reads
//! ([`FaceLattice::speeds`]): the iso image on the curved lane, the
//! stored row's [`Pcurve::speed_bound`] on the trimmed lane's sphere and
//! spline arms. This is `crates/mesh/README.md` `[mesh.sizing]`'s *rim
//! chords use the face's grid step*, stated once; a face reaches a count
//! through it and through its target, and nowhere else.
//!
//! **Crease joints**: an arc rim of `m ≥ 2` sub-arcs meets its face's C⁰
//! creases at the sub-arc joints, and the trimmed lane cuts its triangles
//! along those creases, so the count is rounded up to a multiple of `m`:
//! point `q·k` of `q·m` then sits at the joint's own parameter,
//! `t₀ + Δt·geom_brep::sub_arc_joint(k, m)`, the expression
//! `sweep::skin::segment_curve` places the joint knot with.
//!
//! So chord points are a pure function of (carrier + interval, endpoint
//! points, each adjacent face's lattice and the half-edge's image on it,
//! δ).
//!
//! Polyline endpoints are the topology vertices' points **bitwise**
//! (never `carrier(t₀)`, which is only within ε of them) so every
//! polyline meeting at a vertex shares its mesh vertex id; interior
//! points are `carrier(t₀ + (t₁−t₀)·i/n)` in `he_plus`-forward order.

use std::collections::HashMap;

use geom::Curve3;
use geom_brep::{NoSpeedBound, Pcurve};
use geom_core::Bounds;
use geom_core::interval::Interval;
use geom_core::interval::certification::Certification;
use topo::{Body, EdgeKey};

use crate::chart_table::ChartRow;
use crate::nurbs_cert::{FaceBounds, face_bound};
use crate::sizing::{ceil_count, curvature_step, ellipse_step, sagitta_step, spiric_step};
use crate::types::TessellateError;

/// The chord pass's output: every edge's chord-point ids and the
/// parameter schedule they were sampled at.
///
/// One value rather than two, because they are ONE derivation and
/// every consumer that reads the ids at a parameter reads both — the
/// trimmed lane evaluates pcurves at exactly the schedule the 3-D
/// chords were minted on, which is the property that keeps a face's
/// boundary and its neighbour's the same points.
pub(crate) struct ChordPass {
    /// Per-edge chord-point mesh ids, `he_plus`-forward.
    pub ids: HashMap<EdgeKey, Vec<u32>>,
    /// The matching per-edge chord parameters (endpoints included).
    pub params: HashMap<EdgeKey, Vec<f64>>,
}

/// Computes every edge's chord-point ids (minting interior points into
/// `positions`), in edge-arena order. `vids` maps topology vertices to
/// their already-minted mesh ids.
pub(crate) fn compute_chords(
    body: &Body<f64>,
    delta: f64,
    vids: &HashMap<topo::VertexKey, u32>,
    positions: &mut Vec<geom_core::Point3<f64>>,
    bounds: &mut FaceBounds,
) -> Result<ChordPass, TessellateError> {
    let mut chords = HashMap::new();
    // Chord PARAMETERS per edge (`he_plus`-forward, endpoints
    // included) — the trimmed-face lane evaluates pcurves at exactly
    // the chord schedule, so both stay one derivation (M5 PR 11).
    let mut params = HashMap::new();
    for (ek, edge) in body.edges() {
        let curve = body
            .get_curve_geom(edge.curve)
            .ok_or(TessellateError::MissingEntity { what: "edge curve" })?
            .certified()
            .ok_or(TessellateError::NullScaffoldEdge { edge: ek })?;
        let (t0, t1) = curve.params();
        let span = t1 - t0;
        let target = edge_target(body, ek, delta)?;
        let n = match *curve.carrier() {
            Curve3::Line(_) => 1,
            Curve3::Circle(..) => {
                ceil_count(span, sagitta_step(target, circle_radius(curve.carrier())))?
            }
            // Ellipse arcs (curved-cut boundaries, M5 PR 5): the
            // certified-conservative curvature-bound step.
            Curve3::Ellipse(ref e) => ceil_count(span, ellipse_step(target, e))?,
            // Spiric arcs (the hollowed partial revolve's torus rim):
            // the curvature-bound step from the closed-form `sup|C″|`
            // (`spiric_step`).
            Curve3::Spiric {
                major_radius,
                minor_radius,
                offset,
                ..
            } => ceil_count(
                span,
                spiric_step(target, major_radius, minor_radius, offset),
            )?,
            // B-spline carriers (rung-3 edges at rest, M5 PR 9):
            // the hull-bounded sagitta generalization (C9/PR 11) —
            // secant deviation on a parameter step h is ≤ h²·sup|C″|/8
            // (Taylor with integral remainder; needs C¹, i.e. interior
            // multiplicities ≤ p − 1), and sup|C″| is a control-
            // coefficient convexity fact via iterated derivative
            // hulls — on the CONTROL NET directly for an integral
            // carrier, through the second-order quotient statement
            // for a RATIONAL one (M8-5; a
            // rational-walled body's seam edges read back rational,
            // so this gate is what makes the face bound reachable).
            Curve3::Nurbs(ref n) => nurbs_chord_count(n, span, target, ek)?,
        };
        // The one rule (module docs), on every carrier kind: a straight
        // wall edge is one 3-D chord but many UV steps of its wall's
        // lattice.
        let n = tighten(body, curve, ek, delta, bounds, n)?;
        let (vs, ve) = edge_ends(body, ek)?;
        let start_id = *vids.get(&vs).ok_or(TessellateError::MissingEntity {
            what: "start vertex",
        })?;
        let end_id = *vids
            .get(&ve)
            .ok_or(TessellateError::MissingEntity { what: "end vertex" })?;
        let mut ids = Vec::with_capacity(n + 1);
        let mut ts = Vec::with_capacity(n + 1);
        ids.push(start_id);
        ts.push(t0);
        for i in 1..n {
            let t = t0 + span * geom_brep::sub_arc_joint(i, n);
            #[allow(clippy::cast_possible_truncation)]
            let id = positions.len() as u32;
            positions.push(curve.carrier().eval(t));
            ids.push(id);
            ts.push(t);
        }
        ids.push(end_id);
        ts.push(t1);
        chords.insert(ek, ids);
        params.insert(ek, ts);
    }
    Ok(ChordPass {
        ids: chords,
        params,
    })
}

/// Chord count for a B-spline carrier from the hull-bounded sagitta
/// (module note at the call site): componentwise `sup|C″|` bounds give
/// `|C″| ≤ √(Σ sup²)`, and the per-step bound `h²·M/8 ≤ δ` sizes
/// `h`. Integral carriers take the direct control-hull arm; RATIONAL
/// carriers (M8-5) take [`rational_carrier_m_bound`] — the surface
/// bound's second-order quotient statement one dimension down. (The former
/// blanket rational refusal claimed "arc-bearing profiles refuse at
/// the rational-wall gate" — false since M8-2's rational span meter:
/// arc-walled bodies BUILD now, and their seam edges read back
/// rational, which is exactly why this arm exists.)
fn nurbs_chord_count(
    n: &geom::NurbsCurve3<f64>,
    span: f64,
    target: f64,
    ek: EdgeKey,
) -> Result<usize, TessellateError> {
    // Every weight is strictly positive and finite: the door's own
    // condition, and the convex-combination licence each hull fact below
    // reads. Which arm bounds `C″` is the second-derivative door's.
    let kv = n.knots_f64_interim();
    let p = kv.degree();
    if p < 2 {
        // A single-segment degree-1 carrier is an exact chord — for a
        // RATIONAL one too: the image is still the segment between its
        // two control points (a Möbius reparameterization moves the
        // parameter, not the locus, and the single chord's deviation
        // is a locus fact). One with interior knots is a C⁰ polyline
        // whose kinks a uniform parameter schedule would miss —
        // refused, not guessed at.
        return if kv.interior_knots().next().is_none() {
            Ok(1)
        } else {
            Err(TessellateError::UnsupportedCurve {
                edge: ek,
                note: "degree-1 B-spline carrier with interior knots (a C⁰ polyline) — \
                       the uniform chord schedule cannot pin its kinks; split the edge",
            })
        };
    }
    // C¹ needed for the secant bound: interior multiplicities ≤ p − 1
    // (p ≥ 2 here — degree 1 returned above).
    if kv.interior_knots().any(|(_, m)| m > p - 1) {
        return Err(TessellateError::UnsupportedCurve {
            edge: ek,
            note: "B-spline carrier with a C⁰ kink (interior multiplicity = degree) — \
                   the hull sagitta bound needs C¹; split the edge at the kink",
        });
    }
    // The non-rational hull is `geom`'s, so the export lane's node-count
    // schedule and this chord schedule read one spelling of the iterated
    // difference-coefficient bound; the door names which structure
    // denied a bound, and a rational carrier goes to the second-order
    // quotient statement below.
    let refuse = |note| TessellateError::UnsupportedCurve { edge: ek, note };
    let m_bound = match geom::nonrational_second_derivative_sup(n) {
        Ok(bound) => bound,
        Err(geom::SecondDerivativeUnbounded::Rational) => rational_carrier_m_bound(n, ek)?,
        Err(geom::SecondDerivativeUnbounded::DegreeBelowTwo) => {
            return Err(refuse(
                "B-spline carrier of degree below 2 — the hull sagitta bound \
                 needs a second derivative to hull",
            ));
        }
        Err(geom::SecondDerivativeUnbounded::DerivativeKnotVector) => {
            return Err(refuse(
                "B-spline carrier whose derivative knot vector fails to \
                 materialise — outside the certified chord inventory",
            ));
        }
        Err(geom::SecondDerivativeUnbounded::PoisonedHull) => {
            return Err(refuse(
                "B-spline carrier second-derivative hull is \
                 unbounded/refused — outside the certified chord inventory",
            ));
        }
    };
    if m_bound == 0.0 {
        return Ok(1);
    }
    ceil_count(span, curvature_step(target, m_bound))
}

/// Certified `sup‖C″‖` for a RATIONAL B-spline carrier (M8-5): the
/// second-order quotient statement
/// ([`geom::NurbsCurve3::accel_upper_bound`]; the derivation is
/// `geom_core::spline::derivative`'s "The second order"), read in
/// certification arithmetic off the described carrier, every value a
/// point, so the bound's upper end is certified. `C″ = M/W³` on each
/// dyadic piece of each span (halved from `M` and `W³` formed once over
/// the span), and the bound is the largest
/// `‖m_k‖/b_k` over them. A refusal (a degree above the statement's
/// ceiling, a poisoned piece) refuses typed.
///
/// # Errors
///
/// [`TessellateError::UnsupportedCurve`] — a degree above 18 (`C″`'s
/// quotient sits at `3p`, past [`geom_core::spline::QUOTIENT_MAX_DEGREE`]),
/// or a bound that does not certify: a weight spread wide enough at a
/// high enough degree lets the weight power's coefficient enclosures
/// reach zero (`geom_core::spline::derivative`'s module docs, "The
/// practical ceiling").
fn rational_carrier_m_bound(
    n: &geom::NurbsCurve3<f64>,
    ek: EdgeKey,
) -> Result<f64, TessellateError> {
    if 3 * n.knots_f64_interim().degree() > geom_core::spline::QUOTIENT_MAX_DEGREE {
        return Err(TessellateError::UnsupportedCurve {
            edge: ek,
            note: "rational B-spline carrier of degree above 18, past the degree its \
                   second-derivative bound reaches (three times the degree, at most 56); \
                   describe the edge at degree 18 or below",
        });
    }
    let bound = n.map_scalar(Interval::point).accel_upper_bound();
    if bound.is_certified() && bound.hi().is_finite() {
        Ok(bound.hi())
    } else {
        Err(TessellateError::UnsupportedCurve {
            edge: ek,
            note: "rational B-spline carrier whose second-derivative bound does not certify: \
                   at high degree a wide weight spread lets the weight power reach zero \
                   within its rounding — outside the certified chord inventory",
        })
    }
}

/// **The one chord rule.** For each side of the edge whose face lays a
/// lattice ([`FaceLattice::of`]), with steps `(h_u, h_v)` at the face's
/// target, the count rises to `⌈s_u·Δt/h_u⌉` and `⌈s_v·Δt/h_v⌉`, with
/// `(s_u, s_v)` the per-axis speed of the image that face's lane reads
/// ([`FaceLattice::speeds`]). A chord's chart step is at most its speed
/// bound times its parameter step, so no boundary chord spans more than
/// one lattice step on either axis. The count is then rounded up to a
/// multiple of every adjacent arc rim's sub-arc count (module docs, crease
/// joints).
fn tighten(
    body: &Body<f64>,
    curve: &geom_brep::EdgeCurve<f64>,
    ek: EdgeKey,
    delta: f64,
    bounds: &mut FaceBounds,
    mut n: usize,
) -> Result<usize, TessellateError> {
    let (t0, t1) = curve.params();
    let span = t1 - t0;
    let mut joints = 1;
    let sides = sides_of(body, ek)?;
    for s in [sides.plus, sides.minus] {
        let surface = crate::tessellate::face_surface(body, s.face)?;
        let Some(lattice) = FaceLattice::of(body, s.face, surface, curve, delta, bounds)? else {
            continue;
        };
        let (su, sv) = lattice.speeds(body, s.half_edge, s.face, curve, ek, &mut joints)?;
        let (hu, hv) = lattice.steps;
        if let Reads::SphereRows = lattice.reads {
            // An arc passing near a pole asks about `ρ_c·Δt/(r·d·h_u)`
            // chords at polar distance `d` (the schedule is uniform, so as
            // fine everywhere as at the closest approach). Past the arm's
            // per-edge work budget (`trimmed::MAX_SPHERE_EDGE_CHORDS`, whose
            // doc states it) it refuses typed, read on the raw ratios so
            // that an arc through or near the pole refuses by its own cause
            // before `ceil_count`'s `MAX_COUNT` would.
            #[allow(clippy::cast_precision_loss)]
            let budget = crate::trimmed::MAX_SPHERE_EDGE_CHORDS as f64;
            if !(su * span / hu <= budget && sv * span / hv <= budget) {
                return Err(TessellateError::SphereTrimShape {
                    face: s.face,
                    edge: ek,
                    cause: crate::types::SphereTrimCause::ChordBudget,
                });
            }
        }
        // An axis the image does not move along asks nothing of it.
        let count = |s: f64, h: f64| {
            if s == 0.0 {
                Ok(1)
            } else {
                ceil_count(s * span, h)
            }
        };
        n = n.max(count(su, hu)?).max(count(sv, hv)?);
    }
    Ok(n.div_ceil(joints) * joints)
}

/// A face's lattice as the chord rule reads it: its steps per chart axis
/// at the face's target, and which image of a boundary half-edge its
/// lane reads.
///
/// A step is the one the lane's cell bound needs on that axis: `+∞` along
/// a ruling (a cylinder's or a cone's `v`), where no cell bound reads the
/// extent, whatever rows the lane lays there for its triangles' shape.
struct FaceLattice {
    steps: (f64, f64),
    reads: Reads,
}

/// Which image of a half-edge a face's lane reads.
enum Reads {
    /// The curved lane walks its boundary by the iso classification
    /// (`topo::chart_iso::classify_kind`) on the face's chart, and reads
    /// no stored row (a spindle sheet stores none).
    Iso(topo::chart::Chart<f64>),
    /// The trimmed lane's sphere arm reads the stored row: a parallel's
    /// or a meridian's harmonic image, a tilted circle's projected one.
    SphereRows,
    /// The trimmed lane's spline arm reads the stored row.
    SplineRows,
}

impl FaceLattice {
    /// The lattice of face `fk` beside `curve`, or `None` for a face whose
    /// lane lays none the rule sizes against: the planar lane, the trimmed
    /// lane's cylinder arm (its boundary is chorded at its target alone,
    /// `work/tesslane/the-trimmed-cylinder-arm-chords-its-boundary-to-no-lattice.md`),
    /// and a face the trimmed lane's chart roster refuses. Its steps are
    /// the face's chart row's ([`ChartRow::steps`]):
    ///
    /// - **Curved lane**, beside a circle (its rims and meridians, the iso
    ///   sides its lattice is sized against: a ruling moves along a ruled
    ///   axis only, and any other carrier is no iso side the walk reads,
    ///   so the lane refuses that face at its own guard), at δ. Uncapped;
    ///   why the cap moves no count is at
    ///   [`crate::sizing::torus_grid_steps`].
    /// - **Trimmed sphere arm**: at the face's target
    ///   ([`crate::trimmed::arm_steps`]).
    /// - **Spline arm** (a described NURBS face or an approximating
    ///   surface's fit): the face's whole-patch certificate steps at the
    ///   lane's lattice budget, from the shared [`FaceBounds`] memo, so the
    ///   Hessian hull is assembled once per face for the whole run.
    fn of(
        body: &Body<f64>,
        fk: topo::FaceKey,
        surface: &geom::Surface<f64>,
        curve: &geom_brep::EdgeCurve<f64>,
        delta: f64,
        bounds: &mut FaceBounds,
    ) -> Result<Option<Self>, TessellateError> {
        use crate::tessellate::Lane;
        Ok(match crate::tessellate::lane_of(body, fk, surface)? {
            Lane::Planar => None,
            Lane::Curved if !matches!(curve.carrier(), Curve3::Circle(..)) => None,
            Lane::Curved => {
                let Some(chart) = topo::chart::Chart::of(surface) else {
                    unreachable!("Chart::of answers every curved-lane surface: {surface:?}")
                };
                Some(Self {
                    steps: ChartRow::of(body, fk, surface, bounds)?.steps(delta),
                    reads: Reads::Iso(chart),
                })
            }
            Lane::Trimmed => {
                // The spline row borrows the face's cell table, which this
                // pass is the one to assemble (`face_bound`).
                if let Some(payload) = surface.spline_chart() {
                    if payload.is_placeholder() {
                        return Err(TessellateError::UnsupportedSurface { face: fk });
                    }
                    face_bound(bounds, payload, fk)?;
                }
                match *surface {
                    geom::Surface::Cone { .. }
                    | geom::Surface::Torus { .. }
                    | geom::Surface::Spindle { .. } => return Ok(None),
                    geom::Surface::Sphere { .. }
                        if !crate::trimmed::takes_sphere_arm(body, fk, surface)? =>
                    {
                        return Ok(None);
                    }
                    _ => {}
                }
                let row = ChartRow::of(body, fk, surface, bounds)?;
                match row {
                    ChartRow::Spline { .. } => Some(Self {
                        steps: row.steps(delta),
                        reads: Reads::SplineRows,
                    }),
                    ChartRow::Sphere { .. } => Some(Self {
                        steps: crate::trimmed::arm_steps(body, fk, &row, delta)?,
                        reads: Reads::SphereRows,
                    }),
                    ChartRow::Cylinder { .. }
                    | ChartRow::Cone { .. }
                    | ChartRow::Torus { .. }
                    | ChartRow::Spindle { .. } => None,
                }
            }
        })
    }

    /// The per-axis speed `(s_u, s_v)` of the image of `hek` this face's
    /// lane reads. An arc rim of `m > 1` sub-arcs folds `m` into `joints`.
    fn speeds(
        &self,
        body: &Body<f64>,
        hek: topo::HalfEdgeKey,
        fk: topo::FaceKey,
        curve: &geom_brep::EdgeCurve<f64>,
        ek: EdgeKey,
        joints: &mut usize,
    ) -> Result<(f64, f64), TessellateError> {
        let (t0, t1) = curve.params();
        let refuse = |note| TessellateError::UnsupportedCurve { edge: ek, note };
        let cache = match self.reads {
            // The iso image of a circle, exactly: a rim's carrier
            // parameter is its azimuth, a meridian's its minor angle or
            // latitude. The classifier is total on circles; its `None`
            // is for conic and spline carriers, surfaced typed rather than
            // defaulted so a widened classifier cannot pick a direction.
            Reads::Iso(ref chart) => {
                return match topo::chart_iso::classify_kind(chart, curve) {
                    Some(topo::chart_iso::TravKind::Rim { .. }) => Ok((1.0, 0.0)),
                    Some(topo::chart_iso::TravKind::Meridian { .. }) => Ok((0.0, 1.0)),
                    None => Err(refuse(
                        "a circle edge on a curved-lane face that the iso-curve classifier \
                         left unclassified — neither rim nor meridian, so no grid direction \
                         sizes its chords",
                    )),
                };
            }
            Reads::SphereRows => {
                let cache = body.pcurve(hek).ok_or_else(|| {
                    refuse(
                        "trimmed sphere face half-edge carries no stored pcurve cache — \
                         without one the chord schedule has no UV step bound",
                    )
                })?;
                if !matches!(
                    cache.pcurve(),
                    Pcurve::Harmonic { .. } | Pcurve::Projected(_)
                ) {
                    return Err(refuse(
                        "trimmed sphere face half-edge carries an image that is neither \
                         harmonic nor projected — no sphere mint produces one",
                    ));
                }
                cache
            }
            Reads::SplineRows => {
                let cache = body.pcurve(hek).ok_or_else(|| {
                    refuse(
                        "NURBS-face half-edge carries no stored pcurve cache — caches mint at \
                         loft/sweep assembly and STEP adoption; without one the chord \
                         schedule has no certified UV step bound",
                    )
                })?;
                spline_image(cache.pcurve(), (t0, t1), ek, joints)?;
                cache
            }
        };
        cache
            .pcurve()
            .speed_bound(t0, t1)
            .map_err(|e| match (e, &self.reads) {
                (NoSpeedBound::ThroughSingularSet, Reads::SphereRows) => {
                    TessellateError::SphereTrimShape {
                        face: fk,
                        edge: ek,
                        cause: crate::types::SphereTrimCause::ThroughPole,
                    }
                }
                (e, _) => refuse(no_speed_note(e)),
            })
    }
}

/// What a spline chart's row must be beyond a bounded speed: a form a
/// spline chart mints, and an arc rim whose sub-arc joints the schedule
/// can land on.
///
/// THE CREASE CHORD POINT. Two sub-arcs of an arc rim meet at a knot of
/// multiplicity two, a C⁰ crease of the chart in `u`, and the trimmed lane
/// needs a boundary point exactly there to cut its triangles at it. A
/// schedule of `q·spans` points puts point `q·k` at
/// `t0 + span·sub_arc_joint(k, spans)`, the joint's own parameter, so the
/// count is rounded up to a multiple of `spans` ([`tighten`]). That needs
/// the rim's turn to be this edge's interval, bit for bit.
fn spline_image(
    image: &Pcurve<f64>,
    (t0, t1): (f64, f64),
    ek: EdgeKey,
    joints: &mut usize,
) -> Result<(), TessellateError> {
    let refuse = |note| TessellateError::UnsupportedCurve { edge: ek, note };
    match image {
        Pcurve::IsoArc {
            t0: arc_t0,
            angle,
            breaks,
            ..
        } => {
            let spans = geom_brep::iso_arc_spans(breaks);
            if spans > 1 {
                if *arc_t0 != t0 || *angle != t1 - t0 {
                    return Err(refuse(
                        "an arc-rim pcurve whose turn is not its edge's parameter interval, so \
                         no chord point lands on its sub-arc joints (the C⁰ creases of its \
                         NURBS face)",
                    ));
                }
                *joints = lcm(*joints, spans);
            }
            Ok(())
        }
        // A spiric's chart images live on its own cutting plane and the
        // two tori that hold it, a focal section's on a cone or a torus, a
        // projected image on an analytic chart: no spline chart mints one,
        // so such a row is a corrupt cache rather than a frontier.
        Pcurve::Spiric { .. } => Err(refuse(
            "NURBS-face half-edge carries a SPIRIC pcurve — a spiric's chart images live on \
             its own cutting plane and the two tori that hold it, so no spline chart mints one",
        )),
        Pcurve::FocalSection(_) => Err(refuse(
            "NURBS-face half-edge carries a FOCAL-SECTION pcurve — that image lives on a cone \
             or torus chart, so no spline chart mints one",
        )),
        Pcurve::Projected(_) => Err(refuse(
            "NURBS-face half-edge carries a PROJECTED pcurve — that image lives on an \
             analytic chart, so no spline chart mints one",
        )),
        Pcurve::Harmonic { .. }
        | Pcurve::IsoLine { .. }
        | Pcurve::General(_)
        | Pcurve::Fitted(_) => Ok(()),
    }
}

/// The chord pass's note for an image with no certified speed bound.
fn no_speed_note(e: NoSpeedBound) -> &'static str {
    match e {
        NoSpeedBound::Fitted => {
            "NURBS-face half-edge carries a FITTED pcurve — no certified UV speed bound is \
             wired for a fitted image's chord schedule; its first tessellation consumer is \
             the edge×NURBS-face boolean layer (the cut-loft unit)"
        }
        NoSpeedBound::RationalGeneral => {
            "NURBS-face half-edge carries a RATIONAL general curve-in-UV pcurve — the \
             certified UV speed here is the polynomial net's difference hull, and a varying \
             weight vector's derivative is not that net; no mint produces a rational chart \
             image of this class"
        }
        NoSpeedBound::OutsideGeneralDomain => {
            "a general curve-in-UV pcurve read outside its own knot domain — the control-net \
             hull bounds the image's speed on its domain and says nothing about the end \
             span's polynomial extended past it"
        }
        NoSpeedBound::NetUnbounded => {
            "a chart image whose derivative bound is unbounded/refused — outside the \
             certified chord inventory"
        }
        NoSpeedBound::NoSubArc => {
            "an arc-rim pcurve with no sub-arc structure — a malformed cache, not a \
             chord-schedule question"
        }
        NoSpeedBound::OffRegime => {
            "a closed-form chart image outside the regime its speed bound holds in — no \
             certified UV step bound sizes its chords"
        }
        NoSpeedBound::ThroughSingularSet => {
            "a projected chart image through its chart's singular set — the chart's inverse \
             has no speed bound there"
        }
    }
}

/// The least common multiple of two sub-arc counts.
fn lcm(a: usize, b: usize) -> usize {
    let (mut x, mut y) = (a, b);
    while y != 0 {
        (x, y) = (y, x % y);
    }
    a / x * b
}

/// The deviation target an edge's own carrier is chorded at: the least
/// of its faces' targets ([`crate::trimmed::face_target`]), so the trim
/// polygon of an arm that sizes at its headroom is the one its lattice
/// was sized beside.
fn edge_target(body: &Body<f64>, ek: EdgeKey, delta: f64) -> Result<f64, TessellateError> {
    adjacent_faces(body, ek)?
        .into_iter()
        .try_fold(delta, |least, fk| {
            Ok(least.min(crate::trimmed::face_target(body, fk, delta)?))
        })
}

/// The radius of a circle carrier (caller guarantees the variant).
fn circle_radius(carrier: &Curve3<f64>) -> f64 {
    match *carrier {
        Curve3::Circle(geom::Circle { radius, .. }) => radius,
        _ => f64::NAN,
    }
}

/// The two end vertices of `ek` ([`Body::edge_vertices`]), the edge's
/// miss named as the tessellator names it.
pub(crate) fn edge_ends(
    body: &Body<f64>,
    ek: EdgeKey,
) -> Result<(topo::VertexKey, topo::VertexKey), TessellateError> {
    body.edge_vertices(ek)
        .ok_or(TessellateError::MissingEntity { what: "edge" })
}

/// An edge's two sides, the edge's miss named as the tessellator names
/// it.
fn sides_of(body: &Body<f64>, ek: EdgeKey) -> Result<topo::EdgeSides, TessellateError> {
    topo::readback::edge_sides(body, ek)
        .map_err(|_| TessellateError::MissingEntity { what: "edge" })
}

/// The (≤ 2 distinct) faces adjacent to an edge.
fn adjacent_faces(body: &Body<f64>, ek: EdgeKey) -> Result<Vec<topo::FaceKey>, TessellateError> {
    let (plus, minus) = sides_of(body, ek)?.faces();
    Ok(if plus == minus {
        vec![plus]
    } else {
        vec![plus, minus]
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::nurbs_cert::tests::Domination;
    use geom::NurbsCurve3;
    use geom_core::Point3;
    use geom_core::spline::KnotVector;
    use topo::EdgeKey;

    /// **The count formula, from a hand-built sup.** The arm's answer
    /// feeds `n ≥ ⌈s·Δt/h⌉` and nothing else rows that step, so a
    /// `ceil` quietly become a `floor` (or an off-by-one in the step
    /// division) would be caught only by another suite's goldens.
    ///
    /// The numbers are chosen so `ceil` and `floor` DISAGREE: `s·Δt/h`
    /// is `2.5` and `3.0` respectively, never an integer by accident.
    #[test]
    fn the_chord_count_is_the_ceiling_of_the_speed_times_the_span_over_the_step() {
        // s = 5, Δt = 0.5, h = 1 → 2.5 → 3 chords.
        assert_eq!(ceil_count(5.0 * 0.5, 1.0), Ok(3));
        // exactly on the boundary: 3.0 → 3, not 4.
        assert_eq!(ceil_count(6.0 * 0.5, 1.0), Ok(3));
        // a sup below one step still buys one chord, never zero.
        assert_eq!(ceil_count(1e-9 * 0.5, 1.0), Ok(1));
    }

    fn wiggle() -> NurbsCurve3<f64> {
        let pts: Vec<Point3<f64>> = (0..7)
            .map(|i| {
                let t = f64::from(i) / 6.0;
                Point3::new(t, (3.0 * t * (1.0 - t)).powi(2), 0.3 * t.powi(2))
            })
            .collect();
        NurbsCurve3::interpolate(&pts, 3).unwrap()
    }

    /// **A constant weight is the integral arm's.** The wiggle's net
    /// with every weight one constant (`2`, `3`, `0.1`) is the same
    /// polynomial curve, so its chord
    /// count is the unit-weight curve's at every target (the control
    /// hull reads the net alone); the rational arm's quotient bound on
    /// the same curve is its own, and looser.
    #[test]
    fn a_constant_weight_carrier_counts_as_its_polynomial() {
        let unit = wiggle();
        let (d0, d1) = unit.domain();
        for c in [2.0, 3.0, 0.1] {
            let constant = NurbsCurve3::new(
                unit.knot_values().clone(),
                unit.control().to_vec(),
                vec![c; unit.control().len()],
            )
            .unwrap();
            for target in [1e-2, 1e-4, 1e-6] {
                let count = |n: &NurbsCurve3<f64>| {
                    nurbs_chord_count(n, d1 - d0, target, EdgeKey::default()).expect("in inventory")
                };
                assert_eq!(
                    count(&constant),
                    count(&unit),
                    "weights {c}, δ = {target:e}"
                );
            }
        }
    }

    /// The hull-bounded sagitta generalization: the chord count sizes
    /// the secant deviation under δ — verified against a dense
    /// per-segment sampling oracle.
    #[test]
    fn nurbs_chords_bound_the_secant_deviation() {
        let n = wiggle();
        let (d0, d1) = n.knots_f64_interim().domain();
        for target in [1e-2, 1e-3, 1e-4] {
            let count =
                nurbs_chord_count(&n, d1 - d0, target, EdgeKey::default()).expect("in inventory");
            assert!(count >= 1);
            #[allow(clippy::cast_precision_loss)]
            let h = (d1 - d0) / count as f64;
            let mut worst = 0.0f64;
            for seg in 0..count {
                #[allow(clippy::cast_precision_loss)]
                let a = d0 + h * seg as f64;
                let b = a + h;
                let (pa, pb) = (n.eval(a), n.eval(b));
                for k in 1..32 {
                    let t = a + h * f64::from(k) / 32.0;
                    let p = n.eval(t);
                    let lam = f64::from(k) / 32.0;
                    let chord = Point3::new(
                        pa.x + (pb.x - pa.x) * lam,
                        pa.y + (pb.y - pa.y) * lam,
                        pa.z + (pb.z - pa.z) * lam,
                    );
                    worst = worst.max((p - chord).norm());
                }
            }
            assert!(
                worst <= target * 1.0000001,
                "secant deviation {worst} exceeds the certified budget {target}"
            );
        }
    }

    /// Review probe (adopted): the SPIKE carrier — an interpolated
    /// cubic with one control point far off the line concentrates
    /// |C″| in one span, driving the measured secant deviation to
    /// 0.990–0.999 of the certified budget (the review's
    /// falsification sweep). The pin is two-sided: never OVER budget
    /// (soundness), and at least half of it (the fixture stays
    /// adversarial — a slack rewrite of the bound fails here too).
    #[test]
    fn adversarial_spike_stays_inside_but_near_the_budget() {
        let pts: Vec<Point3<f64>> = [
            (0.0, 0.0),
            (0.2, 0.01),
            (0.4, 0.02),
            (0.5, 0.9),
            (0.6, 0.02),
            (0.8, 0.01),
            (1.0, 0.0),
        ]
        .iter()
        .map(|&(x, y)| Point3::new(x, y, 0.0))
        .collect();
        let n = NurbsCurve3::interpolate(&pts, 3).unwrap();
        let (d0, d1) = n.knots_f64_interim().domain();
        for target in [1e-2, 1e-3, 1e-4] {
            let count =
                nurbs_chord_count(&n, d1 - d0, target, EdgeKey::default()).expect("in inventory");
            #[allow(clippy::cast_precision_loss)]
            let h = (d1 - d0) / count as f64;
            let mut worst = 0.0f64;
            for seg in 0..count {
                #[allow(clippy::cast_precision_loss)]
                let a = d0 + h * seg as f64;
                let b = a + h;
                let (pa, pb) = (n.eval(a), n.eval(b));
                for k in 1..64 {
                    let t = a + h * f64::from(k) / 64.0;
                    let p = n.eval(t);
                    let lam = f64::from(k) / 64.0;
                    let chord = Point3::new(
                        pa.x + (pb.x - pa.x) * lam,
                        pa.y + (pb.y - pa.y) * lam,
                        pa.z + (pb.z - pa.z) * lam,
                    );
                    worst = worst.max((p - chord).norm());
                }
            }
            assert!(
                worst <= target * 1.0000001,
                "spike deviation {worst} exceeds the certified budget {target}"
            );
            assert!(
                worst >= target * 0.5,
                "spike deviation {worst} fell below half the budget {target} — the \
                 adversarial fixture went slack (review measured 0.990-0.999)"
            );
        }
    }

    /// CONSCIOUS FLIP (M8-5): `rational_nurbs_carrier_refuses_typed`
    /// re-derived as the positive row. Rational carriers are metered
    /// now (the quotient `sup|C″|` of
    /// [`rational_carrier_m_bound`]), and the pin is the bound's
    /// honesty against the TRUTH: on adversarial rational carriers the
    /// dense-sampled secant deviation stays inside the certified
    /// budget on every segment, and the sampled sup of `‖C″‖` itself
    /// is dominated by a real (> 0) bound.
    #[test]
    fn rational_carrier_chords_bound_the_secant_deviation() {
        // The exact unit quarter circle (the arc-walled seam class)
        // and a steep-weight wiggle (weights alternating 0.4–3.0 on
        // the interpolated cubic's control).
        let w = core::f64::consts::FRAC_1_SQRT_2;
        let arc = NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
            vec![
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, 1.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
            ],
            vec![1.0, w, 1.0],
        )
        .unwrap();
        let base = wiggle();
        let steep: Vec<f64> = (0..base.control().len())
            .map(|i| match i % 4 {
                0 => 0.4,
                1 => 3.0,
                2 => 1.0,
                _ => 0.6,
            })
            .collect();
        let wiggle_r = NurbsCurve3::new(
            base.knots_f64_interim().clone(),
            base.control().to_vec(),
            steep,
        )
        .unwrap();
        for (name, n) in [("quarter_arc", arc), ("steep_wiggle", wiggle_r)] {
            let (d0, d1) = n.knots_f64_interim().domain();
            for target in [1e-2, 1e-3, 1e-4] {
                let count = nurbs_chord_count(&n, d1 - d0, target, EdgeKey::default())
                    .expect("rational carriers are in the chord inventory now");
                assert!(count >= 1, "{name}: a real count, not a fabricated zero");
                #[allow(clippy::cast_precision_loss)]
                let h = (d1 - d0) / count as f64;
                let mut worst = 0.0f64;
                for seg in 0..count {
                    #[allow(clippy::cast_precision_loss)]
                    let a = d0 + h * seg as f64;
                    let b = a + h;
                    let (pa, pb) = (n.eval(a), n.eval(b));
                    for k in 1..64 {
                        let t = a + h * f64::from(k) / 64.0;
                        let p = n.eval(t);
                        let lam = f64::from(k) / 64.0;
                        let chord = Point3::new(
                            pa.x + (pb.x - pa.x) * lam,
                            pa.y + (pb.y - pa.y) * lam,
                            pa.z + (pb.z - pa.z) * lam,
                        );
                        worst = worst.max((p - chord).norm());
                    }
                }
                assert!(
                    worst <= target * 1.0000001,
                    "{name}: secant deviation {worst} exceeds the certified budget \
                     {target} over {count} chords"
                );
            }
        }
    }

    /// The refusal row the flip keeps: an ILLEGAL rational carrier
    /// (non-positive weight) cannot even be described —
    /// `NurbsCurve3::new` refuses at the door, which is why
    /// [`nurbs_chord_count`] carries no licence check of its own.
    #[test]
    fn illegal_rational_carrier_weight_refuses_at_the_door() {
        let n = wiggle();
        for bad in [0.0, -0.5, f64::NAN] {
            let mut weights = vec![1.0; n.control().len()];
            weights[2] = bad;
            assert!(
                NurbsCurve3::new(n.knots_f64_interim().clone(), n.control().to_vec(), weights)
                    .is_err(),
                "weight {bad} must refuse at construction"
            );
        }
    }

    // ------------------------------------------------------------------
    // R1 REVIEW PROBES (M8-5, PR #322): adversarial rational carriers
    // beyond the PR's — extreme/near-zero weights and the C¹
    // multiplicity edge — checked BOTH ways: the m-bound dominates the
    // dense-sampled true sup‖C″‖ (via deriv2, plus the chord counts
    // keep the measured secant deviation inside δ).
    // ------------------------------------------------------------------

    fn r1_secant_worst(n: &NurbsCurve3<f64>, count: usize) -> f64 {
        let (d0, d1) = n.knots_f64_interim().domain();
        #[allow(clippy::cast_precision_loss)]
        let h = (d1 - d0) / count as f64;
        let mut worst = 0.0f64;
        for seg in 0..count {
            #[allow(clippy::cast_precision_loss)]
            let a = d0 + h * seg as f64;
            let b = a + h;
            let (pa, pb) = (n.eval(a), n.eval(b));
            for k in 1..128 {
                let t = a + h * f64::from(k) / 128.0;
                let p = n.eval(t);
                let lam = f64::from(k) / 128.0;
                let chord = Point3::new(
                    pa.x + (pb.x - pa.x) * lam,
                    pa.y + (pb.y - pa.y) * lam,
                    pa.z + (pb.z - pa.z) * lam,
                );
                worst = worst.max((p - chord).norm());
            }
        }
        worst
    }

    /// The δ schedule the R1 carriers sweep. Per-fixture, because the
    /// count scales as δ^(−½) and one fixture's bound is conservative
    /// enough to make the finest row dominate the crate — see
    /// [`r1_extreme_weight_carrier`].
    const R1_DELTAS: [f64; 3] = [1e-2, 1e-3, 1e-4];

    fn r1_check(name: &str, n: &NurbsCurve3<f64>, deltas: &[f64]) {
        // (a) m-bound vs dense-sampled true sup|C''|. δ-free: every
        // carrier gets this arm in full.
        let m = rational_carrier_m_bound(n, EdgeKey::default()).expect("in inventory");
        let (d0, d1) = n.knots_f64_interim().domain();
        let mut truth = 0.0f64;
        for k in 0..=4000 {
            let t = d0 + (d1 - d0) * f64::from(k) / 4000.0;
            truth = truth.max(n.deriv2(t).norm());
        }
        let d = Domination::sampled_under_certified(&[("sup|C''|", truth, m)]);
        assert!(d.holds(), "{name}: {d}");
        println!("{name}: truth/bound = {:.4}", truth / m);
        // (b) chord counts keep the secant inside target.
        for &target in deltas {
            let count =
                nurbs_chord_count(n, d1 - d0, target, EdgeKey::default()).expect("in inventory");
            let worst = r1_secant_worst(n, count);
            assert!(
                worst <= target * 1.0000001,
                "{name}: secant {worst} exceeds {target} over {count} chords"
            );
        }
    }

    /// Extreme weights (1e-2 .. 1e2) on a multi-span cubic.
    ///
    /// SHORT δ SCHEDULE (no 1e-4), and the reasoning is the one this
    /// tree has already ratified twice on the sibling SURFACE arm:
    /// `nurbs_cert.rs:1106-1111` ("the lattice's falsification power is
    /// PER TRIANGLE, not δ-dependent, so it takes coarser deltas") and
    /// `nurbs_cert.rs:1513-1516` ("the per-triangle claim d ≤ cert(uv)
    /// is grid-independent, so a coarser grid still falsifies"). The
    /// claim asserted in `r1_check`'s arm (b) is likewise PER SEGMENT —
    /// `worst <= target` on every one of `count` segments — so a finer
    /// δ only multiplies how many segments are checked; it adds no
    /// falsification power per segment. What it does add is cost: the
    /// count scales as δ^(−½), and THIS fixture's bound is the extreme
    /// case (measured `truth/bound = 0.0023`, i.e. ~435x conservative,
    /// exactly the conservatism `nurbs_cert` capped its grid for), so
    /// its 1e-4 row alone was ~70% of a test that was in turn 98% of
    /// the whole `chords` module (12.55 s of 12.77 s, measured).
    ///
    /// WHAT IS LOST: the δ = 1e-4 schedule on THIS fixture only. The
    /// finest row stays exercised, on every other carrier in the
    /// module: `r1_near_zero_weight_carrier` and
    /// `r1_rational_mult_p_minus_one_carrier` (both still pass the full
    /// `R1_DELTAS` through this same helper, and cost 0.05 s / 0.08 s
    /// doing it), plus `rational_carrier_chords_bound_the_secant_
    /// deviation`'s two rational fixtures and the polynomial
    /// `nurbs_chords_bound_the_secant_deviation` /
    /// `adversarial_spike_stays_inside_but_near_the_budget`. Arm (a),
    /// the bound-honesty claim that is this fixture's actual point, is
    /// δ-free and untouched.
    #[test]
    fn r1_extreme_weight_carrier() {
        let base = wiggle();
        let w: Vec<f64> = (0..base.control().len())
            .map(|i| [1e-2, 1.0, 1e2, 0.3, 7.0][i % 5])
            .collect();
        let n =
            NurbsCurve3::new(base.knots_f64_interim().clone(), base.control().to_vec(), w).unwrap();
        r1_check("extreme_weight_carrier", &n, &R1_DELTAS[..2]);
    }

    /// Near-zero-touching legal weight (1e-5) amid O(1).
    #[test]
    fn r1_near_zero_weight_carrier() {
        let base = wiggle();
        let mut w = vec![1.0; base.control().len()];
        w[3] = 1e-5;
        let n =
            NurbsCurve3::new(base.knots_f64_interim().clone(), base.control().to_vec(), w).unwrap();
        r1_check("near_zero_weight_carrier", &n, &R1_DELTAS);
    }

    /// Interior multiplicity EXACTLY p−1 (the C¹ edge) on a RATIONAL
    /// cubic — |C''| jumps at the double knot.
    #[test]
    fn r1_rational_mult_p_minus_one_carrier() {
        let kv =
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let pts: Vec<Point3<f64>> = (0..kv.control_count())
            .map(|i| {
                let t = i as f64 / 5.0;
                Point3::new(t, (2.5 * t).sin(), 0.4 * t.powi(2))
            })
            .collect();
        let w: Vec<f64> = (0..pts.len()).map(|i| [0.3, 2.0, 0.9][i % 3]).collect();
        let n = NurbsCurve3::new(kv, pts, w).unwrap();
        r1_check("rational_mult_p1_carrier", &n, &R1_DELTAS);
    }

    /// Builds a rational cubic whose single interior knot has the given
    /// multiplicity.
    fn mult_cubic(multiplicity: usize) -> NurbsCurve3<f64> {
        let mut knots = vec![0.0; 4];
        knots.extend(core::iter::repeat_n(0.5, multiplicity));
        knots.extend(core::iter::repeat_n(1.0, 4));
        let kv = KnotVector::clamped(knots, 3).expect("clamped cubic");
        #[allow(clippy::cast_precision_loss)]
        let pts: Vec<Point3<f64>> = (0..kv.control_count())
            .map(|i| {
                let t = i as f64 / 6.0;
                Point3::new(t, (3.0 * t).cos(), 0.7 * t)
            })
            .collect();
        let w: Vec<f64> = (0..pts.len()).map(|i| [1.4, 0.5, 2.2][i % 3]).collect();
        NurbsCurve3::new(kv, pts, w).expect("legal rational cubic")
    }

    /// **A rational carrier past the second-order ceiling refuses
    /// typed.** A rational Bézier of degree 19 reads `C″` at
    /// `3 · 19 = 57`, past the statement's `56`: the chord pass names the
    /// ceiling rather than sizing a count from a `NaN` bound.
    #[test]
    fn a_rational_carrier_past_the_ceiling_refuses_typed() {
        let kv = KnotVector::clamped([vec![0.0; 20], vec![1.0; 20]].concat(), 19).unwrap();
        let pts = (0..20)
            .map(|i| {
                let t = f64::from(i) / 19.0;
                Point3::new(t, t * t, 0.0)
            })
            .collect();
        let mut w = vec![1.0; 20];
        w[3] = 2.0;
        let n = NurbsCurve3::new(kv, pts, w).unwrap();
        match nurbs_chord_count(&n, 1.0, 1e-3, EdgeKey::default()) {
            Err(TessellateError::UnsupportedCurve { note, .. }) => assert!(
                note.contains("degree above 18") && note.contains("56"),
                "{note}"
            ),
            other => panic!("a degree-19 rational carrier was not refused typed: {other:?}"),
        }
    }

    /// **A rational carrier with an empty span is bounded.** The
    /// quotient's span loop ([`geom_core::spline::accel_pieces`]) draws
    /// each window from a `Span`, so a window past the net or below the
    /// degree is unrepresentable and the loop's only exit is the
    /// empty-span skip — which an interior knot of multiplicity ≥ 2
    /// produces (`knots[s] == knots[s+1]` for an `s` inside
    /// `[first_span, last_span]`). The fixture's bound and chords are
    /// checked as every carrier's are.
    ///
    /// The `any(span(s).is_none())` assertion is the anti-slack guard:
    /// it fails loudly if the fixture ever stops presenting an empty
    /// span, which would silently make this row cover nothing.
    ///
    /// Multiplicity `p` itself is checked too, and it is a REFUSAL, not
    /// a bound: a C⁰ kink leaves the certified inventory at the
    /// degree/kink gate, before any span is read.
    #[test]
    fn a_rational_carrier_with_an_empty_span_is_bounded() {
        // p − 1 = 2: inside the inventory, and it presents an empty span.
        let n = mult_cubic(2);
        let kv = n.knots_f64_interim();
        assert!(
            (kv.first_span()..=kv.last_span()).any(|s| kv.span(s).is_none()),
            "the multiplicity-2 fixture must present an empty span — \
             otherwise this row stops covering the skip"
        );
        // The full δ schedule, as the other cheap carriers take: this
        // fixture is `rational_mult_p1_carrier`'s sibling in cost, not
        // `extreme_weight`'s, so there is nothing here to trim.
        r1_check("rational_mult_2_carrier", &n, &R1_DELTAS);

        // p = 3: refused, typed, and NOT by either deleted message.
        let n = mult_cubic(3);
        let (d0, d1) = n.knots_f64_interim().domain();
        match nurbs_chord_count(&n, d1 - d0, 1e-3, EdgeKey::default()) {
            Err(TessellateError::UnsupportedCurve { note, .. }) => assert!(
                note.contains("C⁰ kink"),
                "multiplicity-p refusal should name the C⁰ kink, got {note:?}"
            ),
            other => panic!("multiplicity p must leave the inventory, got {other:?}"),
        }
    }
}
