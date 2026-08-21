//! The Part 21 emitter and the body→AP214 topology walk (crate docs
//! for the entity mapping and conformance decisions).
//!
//! # Emission order (D9: one fixed traversal, ids by first encounter)
//!
//! Solids in arena order → shells in `Solid::shells` order → faces in
//! `Shell::faces` order → the outer loop then rings in stored order →
//! half-edges in loop-cycle (`next`) order. Vertex and edge entities
//! are minted at first encounter and cached (lookup-only maps — no
//! hash iteration on any path that shapes output). The preamble
//! (units, uncertainty, context, product chain) follows the geometry;
//! the header is a fixed template over [`StepOptions`].

use std::fmt::Write as _;

use geom::Curve3;
use geom::Surface;
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};
use topo::{Body, CurveGeom, Edge, EdgeKey, FaceKey, LoopBoundary, LoopKey, ShellKey, VertexKey};

use crate::real::fmt_real;
use crate::volume::shell_signed_volume;
use crate::{SharedIds, StepExportError, StepOptions, quoted};
use geom_core::Tol;

/// The surface variant's name, for typed refusals and for the
/// curved-shell classification message.
pub(crate) fn surface_kind(surface: &Surface<f64>) -> &'static str {
    match surface {
        Surface::Plane { .. } => "plane",
        Surface::Cylinder { .. } => "cylinder",
        Surface::Cone { .. } => "cone",
        Surface::Sphere { .. } => "sphere",
        Surface::Torus { .. } => "torus",
        // The two Nurbs states are told apart (the shared
        // `is_placeholder` discriminator, M6-3): the mvfs "no
        // description yet" placeholder (all-poison control points) is
        // a mid-surgery fact and REFUSES; a described NURBS surface
        // exports natively as B_SPLINE_SURFACE_WITH_KNOTS since M6-3
        // — this name now appears only in messages that classify
        // kinds (e.g. the multi-shell curved classification), never
        // as a face-printer refusal.
        Surface::Nurbs(payload) => {
            if payload.is_placeholder() {
                "nurbs placeholder"
            } else {
                "nurbs surface"
            }
        }
    }
}

/// The curve variant's name, for typed refusals.
pub(crate) fn carrier_kind(carrier: &Curve3<f64>) -> &'static str {
    match carrier {
        Curve3::Line { .. } => "line",
        Curve3::Circle { .. } => "circle",
        Curve3::Ellipse { .. } => "ellipse",
        Curve3::Nurbs(_) => "nurbs curve",
    }
}

/// The certified carrier of `edge`, refusing null scaffolding with the
/// typed mid-surgery error.
pub(crate) fn certified_carrier<'a>(
    body: &'a Body<f64>,
    edge_key: EdgeKey,
    edge: &Edge,
) -> Result<&'a Curve3<f64>, StepExportError> {
    let geom = body
        .get_curve_geom(edge.curve)
        .ok_or(StepExportError::Corrupt {
            what: "edge curve key does not resolve",
        })?;
    match geom {
        CurveGeom::Certified(curve) => Ok(curve.carrier()),
        CurveGeom::NullScaffold(_) => Err(StepExportError::NullScaffoldEdge { edge: edge_key }),
    }
}

/// The Part 21 emitter: id allocation and record accumulation.
struct Writer<'a> {
    body: &'a Body<f64>,
    next_id: u64,
    data: String,
    shared: SharedIds,
}

/// `#id` reference list: `#1, #2, #3`.
fn refs(ids: &[u64]) -> String {
    let mut out = String::new();
    for (i, id) in ids.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        let _ = write!(out, "#{id}");
    }
    out
}

/// Formats a knot vector's runs as STEP's `(multiplicities)` /
/// `(distinct knots)` pair, both as ready-to-splice argument text.
///
/// The run-length encoding itself is `KnotVector::knot_runs`, which
/// cuts on **exact f64 equality** — knots are structure, and structure
/// identity is bitwise-value identity, never a tolerance question. So
/// the pair round-trips to the identical flat vector, and no ε enters
/// this path. What is left here is formatting: the separators and the
/// `fmt_real` refusal, which is why this walks the runs rather than
/// collecting them.
fn run_length_knots(
    knots: &KnotVector,
    context: &'static str,
) -> Result<(String, String), StepExportError> {
    let mut mults = String::new();
    let mut values = String::new();
    for (index, (value, run)) in knots.knot_runs().enumerate() {
        if index > 0 {
            mults.push_str(", ");
            values.push_str(", ");
        }
        let _ = write!(mults, "{run}");
        values.push_str(&fmt_real(value, context)?);
    }
    Ok((mults, values))
}

impl<'a> Writer<'a> {
    fn new(body: &'a Body<f64>) -> Self {
        Self {
            body,
            next_id: 0,
            data: String::new(),
            shared: SharedIds::default(),
        }
    }

    /// Appends one entity instance record and returns its id.
    fn emit(&mut self, record: &str) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        let _ = writeln!(self.data, "#{id} = {record};");
        id
    }

    /// `CARTESIAN_POINT` from a kernel point (coordinates in meters —
    /// the file's unit context is unprefixed metres, D6).
    fn cartesian_point(
        &mut self,
        p: Point3<f64>,
        context: &'static str,
    ) -> Result<u64, StepExportError> {
        let (x, y, z) = (
            fmt_real(p.x, context)?,
            fmt_real(p.y, context)?,
            fmt_real(p.z, context)?,
        );
        Ok(self.emit(&format!("CARTESIAN_POINT('', ({x}, {y}, {z}))")))
    }

    /// `DIRECTION` from a kernel vector (unit by the conventions of
    /// every stored normal/axis/dir — emitted as stored, never
    /// renormalized: renormalization would move bits, D9).
    fn direction(&mut self, v: Vec3<f64>, context: &'static str) -> Result<u64, StepExportError> {
        let (x, y, z) = (
            fmt_real(v.x, context)?,
            fmt_real(v.y, context)?,
            fmt_real(v.z, context)?,
        );
        Ok(self.emit(&format!("DIRECTION('', ({x}, {y}, {z}))")))
    }

    /// `AXIS2_PLACEMENT_3D` from a kernel frame `(origin, axis,
    /// ref_dir)` — the shared right-handed placement every elementary
    /// surface and conic carries. Emission order is location, axis,
    /// ref_direction, placement (the order M4's inline plane arm used,
    /// preserved so the planar goldens stay byte-identical).
    ///
    /// The three kernel fields go in **verbatim**: every kernel frame
    /// stores `axis` unit, `ref_dir` unit and ⊥ `axis`, and the derived
    /// second in-plane direction is `axis × ref_dir` — which is exactly
    /// ISO 10303-42's `axis2_placement_3d` convention (`z = axis`,
    /// `x = ref_direction` after the standard's orthogonalization,
    /// `y = z × x`). No renormalization, no orthogonalization, no
    /// negation: the placement IS the kernel frame (D9 — any repair
    /// here would move bits and export a different surface).
    fn axis2_placement(
        &mut self,
        origin: Point3<f64>,
        axis: Vec3<f64>,
        ref_dir: Vec3<f64>,
        origin_context: &'static str,
        axis_context: &'static str,
        ref_context: &'static str,
    ) -> Result<u64, StepExportError> {
        let cp = self.cartesian_point(origin, origin_context)?;
        let a = self.direction(axis, axis_context)?;
        let r = self.direction(ref_dir, ref_context)?;
        Ok(self.emit(&format!("AXIS2_PLACEMENT_3D('', #{cp}, #{a}, #{r})")))
    }

    /// `VERTEX_POINT` for a kernel vertex (first encounter mints, later
    /// encounters reuse — vertices are shared by every adjacent edge).
    fn vertex_point(&mut self, vertex: VertexKey) -> Result<u64, StepExportError> {
        if let Some(&id) = self.shared.vertex_points.get(&vertex) {
            return Ok(id);
        }
        let v = self
            .body
            .get_vertex(vertex)
            .ok_or(StepExportError::Corrupt {
                what: "vertex key does not resolve",
            })?;
        let p = self
            .body
            .get_point(v.point)
            .copied()
            .ok_or(StepExportError::Corrupt {
                what: "vertex point key does not resolve",
            })?;
        let cp = self.cartesian_point(p, "vertex position")?;
        let id = self.emit(&format!("VERTEX_POINT('', #{cp})"));
        self.shared.vertex_points.insert(vertex, id);
        Ok(id)
    }

    /// `EDGE_CURVE` for a kernel edge: vertices from `he_plus`
    /// (start→end in carrier parameter direction, the M2 forward
    /// contract), geometry from the certified carrier, `same_sense =
    /// .T.` by that same contract.
    fn edge_curve(&mut self, edge_key: EdgeKey) -> Result<u64, StepExportError> {
        if let Some(&id) = self.shared.edge_curves.get(&edge_key) {
            return Ok(id);
        }
        let edge = self
            .body
            .get_edge(edge_key)
            .ok_or(StepExportError::Corrupt {
                what: "edge key does not resolve",
            })?;
        let he_plus = self
            .body
            .get_half_edge(edge.he_plus)
            .ok_or(StepExportError::Corrupt {
                what: "edge he_plus does not resolve",
            })?;
        let start = self.vertex_point(he_plus.start)?;
        let end_vertex = self
            .body
            .half_edge_end(edge.he_plus)
            .ok_or(StepExportError::Corrupt {
                what: "edge he_plus end does not resolve",
            })?;
        let end = self.vertex_point(end_vertex)?;
        let carrier = certified_carrier(self.body, edge_key, edge)?;
        // The curve printers — one closed match over `Curve3`, every
        // arm an EXACT native AP214 entity (crate docs, "the analytic
        // subset"). Everything around this match is carrier-agnostic.
        let curve = match *carrier {
            Curve3::Line { origin, dir } => {
                let cp = self.cartesian_point(origin, "line origin")?;
                let d = self.direction(dir, "line direction")?;
                // Magnitude 1.0: `dir` is unit by convention, so the
                // STEP line parameter stays arc length in meters,
                // matching the kernel parameterization.
                let vector = self.emit(&format!("VECTOR('', #{d}, 1.0)"));
                self.emit(&format!("LINE('', #{cp}, #{vector})"))
            }
            // `circle`: P(θ) = location + radius·(cos θ·x + sin θ·y)
            // with `y = z × x` — the kernel's `Circle` formula field
            // for field, including the seam at `u_ref` (θ = 0) and the
            // right-hand winding about `axis`. Exact, native, no
            // parameter remapping.
            Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => {
                let placement = self.axis2_placement(
                    center,
                    axis,
                    u_ref,
                    "circle centre",
                    "circle axis",
                    "circle u_ref",
                )?;
                let r = fmt_real(radius, "circle radius")?;
                self.emit(&format!("CIRCLE('', #{placement}, {r})"))
            }
            // `ellipse`: P(θ) = location + semi_axis_1·cos θ·x +
            // semi_axis_2·sin θ·y — again the kernel's formula verbatim
            // (`semi_axis_1 = major` along `u_ref`, `semi_axis_2 =
            // minor` along `axis × u_ref`), so the eccentric-anomaly
            // parameterization survives unchanged. AP214 HAS this
            // entity: the rational-quadratic Bézier form (NURBS Book
            // §7.3, shape factor k = w₀w₂/w₁²) is NOT used — it would
            // be an equally exact but strictly worse encoding, hiding
            // the axes every reader wants and reparameterizing the
            // curve for nothing.
            Curve3::Ellipse {
                center,
                axis,
                major,
                minor,
                u_ref,
            } => {
                let placement = self.axis2_placement(
                    center,
                    axis,
                    u_ref,
                    "ellipse centre",
                    "ellipse axis",
                    "ellipse u_ref (semi-major direction)",
                )?;
                let a = fmt_real(major, "ellipse semi-major axis")?;
                let b = fmt_real(minor, "ellipse semi-minor axis")?;
                self.emit(&format!("ELLIPSE('', #{placement}, {a}, {b})"))
            }
            Curve3::Nurbs(ref payload) => {
                if payload.is_placeholder() {
                    // The "no description yet" carrier placeholder —
                    // the curve-side twin of the mvfs surface
                    // placeholder. Mid-surgery, never exportable.
                    return Err(StepExportError::UnsupportedCurve {
                        edge: edge_key,
                        kind: "nurbs placeholder",
                    });
                }
                self.b_spline_curve(payload.knots(), payload.control(), payload.weights())?
            }
        };
        let id = self.emit(&format!("EDGE_CURVE('', #{start}, #{end}, #{curve}, .T.)"));
        self.shared.edge_curves.insert(edge_key, id);
        Ok(id)
    }

    /// `B_SPLINE_CURVE_WITH_KNOTS` for a validated kernel NURBS
    /// carrier — the **non-rational** simple entity when every weight
    /// is exactly 1.0, otherwise the `RATIONAL_B_SPLINE_CURVE` complex
    /// instance (ISO 10303-42's rational subtype is an AND-composition,
    /// so Part 21 spells it as a complex-entity instance with the
    /// component names in alphabetical order — the same rule the unit
    /// context records already follow).
    ///
    /// Field mapping, all exact:
    ///
    /// - `degree` ← the knot vector's degree; `control_points_list` ←
    ///   the control points in stored order (one `CARTESIAN_POINT`
    ///   each, no sharing — control points are curve-private data, not
    ///   topology).
    /// - `knot_multiplicities` / `knots` ← the flat clamped knot vector
    ///   run-length-encoded by **exact f64 equality**, which is the
    ///   kernel's own multiplicity predicate (`KnotVector`'s type docs:
    ///   knots are structure, structure identity is bitwise). The
    ///   clamped-v1 invariant means the ends come out at multiplicity
    ///   `degree + 1`, exactly what `.UNSPECIFIED.` knot_spec allows.
    /// - `curve_form` is `.UNSPECIFIED.`: the kernel does not certify
    ///   polyline/circular/elliptic FORM of a NURBS carrier, and
    ///   claiming a form it has not proven would be a lie in a field
    ///   readers act on.
    /// - `closed_curve` and `self_intersect` are `.U.` (LOGICAL
    ///   unknown), for the same reason: the kernel carries no
    ///   closure/self-intersection certificate for a fitted carrier.
    ///   `.F.`/`.F.` — what most writers emit — would be an unbacked
    ///   claim; `.U.` is the honest value the type provides.
    /// - `weights_data` (rational arm) ← the stored weights, strictly
    ///   positive by construction, printed round-trip exact.
    ///
    /// Note that the weights are **f64 structure** (C6) even when the
    /// control points are not, so the rational/non-rational decision is
    /// an exact comparison, not a tolerance question.
    fn b_spline_curve(
        &mut self,
        knots: &KnotVector,
        control: &[Point3<f64>],
        weights: &[f64],
    ) -> Result<u64, StepExportError> {
        let mut points = Vec::with_capacity(control.len());
        for p in control {
            points.push(self.cartesian_point(*p, "b-spline curve control point")?);
        }
        let points = refs(&points);
        let degree = knots.degree();
        let (mults, values) = run_length_knots(knots, "b-spline curve knot")?;
        if weights.iter().all(|w| *w == 1.0) {
            Ok(self.emit(&format!(
                "B_SPLINE_CURVE_WITH_KNOTS('', {degree}, ({points}), .UNSPECIFIED., .U., .U., \
                 ({mults}), ({values}), .UNSPECIFIED.)"
            )))
        } else {
            let mut w = String::new();
            for (i, weight) in weights.iter().enumerate() {
                if i > 0 {
                    w.push_str(", ");
                }
                w.push_str(&fmt_real(*weight, "b-spline curve weight")?);
            }
            Ok(self.emit(&format!(
                "( BOUNDED_CURVE() B_SPLINE_CURVE({degree}, ({points}), .UNSPECIFIED., .U., .U.) \
                 B_SPLINE_CURVE_WITH_KNOTS(({mults}), ({values}), .UNSPECIFIED.) CURVE() \
                 GEOMETRIC_REPRESENTATION_ITEM() RATIONAL_B_SPLINE_CURVE(({w})) \
                 REPRESENTATION_ITEM('') )"
            )))
        }
    }

    /// `B_SPLINE_SURFACE_WITH_KNOTS` for a validated kernel NURBS
    /// surface (M6-3) — the **non-rational** simple entity when every
    /// weight is exactly 1.0, otherwise the `RATIONAL_B_SPLINE_SURFACE`
    /// complex instance (the AND-composition spelled as a
    /// complex-entity instance with component names in alphabetical
    /// order — [`Self::b_spline_curve`]'s rule, one dimension up; both
    /// arms written even though the first corpus body is non-rational,
    /// the curve writer's precedent, pinned at record level).
    ///
    /// Field mapping, all exact, mirroring the curve writer:
    ///
    /// - `u_degree`/`v_degree` ← the knot vectors' degrees;
    ///   `control_points_list` ← LIST OF LIST, outer index u (the
    ///   kernel's row-major `iu·nv + iv` layout maps row for row).
    /// - knot pairs ← run-length-encoded by **exact f64 equality**
    ///   per direction (the kernel's own multiplicity predicate).
    /// - `surface_form` is `.UNSPECIFIED.`; `u_closed`/`v_closed`/
    ///   `self_intersect` are `.U.` — the kernel certifies none of
    ///   those claims for a skinned patch, and `.U.` is the honest
    ///   LOGICAL the type provides (the curve writer's reasoning,
    ///   verbatim).
    /// - `weights_data` (rational arm) ← the stored weights as a LIST
    ///   OF LIST in the same layout, printed round-trip exact.
    fn b_spline_surface(
        &mut self,
        surface: &geom::NurbsSurface<f64>,
    ) -> Result<u64, StepExportError> {
        let (nu, nv) = surface.control_counts();
        let mut rows = Vec::with_capacity(nu);
        for iu in 0..nu {
            let mut row = Vec::with_capacity(nv);
            for iv in 0..nv {
                row.push(self.cartesian_point(
                    surface.control()[iu * nv + iv],
                    "b-spline surface control point",
                )?);
            }
            rows.push(format!("({})", refs(&row)));
        }
        let points = rows.join(", ");
        let (du, dv) = (surface.knots_u().degree(), surface.knots_v().degree());
        let (u_mults, u_values) = run_length_knots(surface.knots_u(), "b-spline surface u-knot")?;
        let (v_mults, v_values) = run_length_knots(surface.knots_v(), "b-spline surface v-knot")?;
        if surface.weights().iter().all(|w| *w == 1.0) {
            Ok(self.emit(&format!(
                "B_SPLINE_SURFACE_WITH_KNOTS('', {du}, {dv}, ({points}), .UNSPECIFIED., .U., \
                 .U., .U., ({u_mults}), ({v_mults}), ({u_values}), ({v_values}), .UNSPECIFIED.)"
            )))
        } else {
            let mut wrows = Vec::with_capacity(nu);
            for iu in 0..nu {
                let mut wrow = String::new();
                for iv in 0..nv {
                    if iv > 0 {
                        wrow.push_str(", ");
                    }
                    wrow.push_str(&fmt_real(
                        surface.weights()[iu * nv + iv],
                        "b-spline surface weight",
                    )?);
                }
                wrows.push(format!("({wrow})"));
            }
            let w = wrows.join(", ");
            Ok(self.emit(&format!(
                "( B_SPLINE_SURFACE({du}, {dv}, ({points}), .UNSPECIFIED., .U., .U., .U.) \
                 B_SPLINE_SURFACE_WITH_KNOTS(({u_mults}), ({v_mults}), ({u_values}), \
                 ({v_values}), .UNSPECIFIED.) BOUNDED_SURFACE() \
                 GEOMETRIC_REPRESENTATION_ITEM() RATIONAL_B_SPLINE_SURFACE(({w})) \
                 REPRESENTATION_ITEM('') SURFACE() )"
            )))
        }
    }

    /// `EDGE_LOOP` of `ORIENTED_EDGE`s for one loop, wrapped as
    /// `FACE_OUTER_BOUND` (outer) or `FACE_BOUND` (ring), both with
    /// orientation `.T.`: the stored traversal direction is already
    /// STEP's (interior-left ⇒ outer CCW / rings CW about the outward
    /// normal — crate docs).
    ///
    /// **The bound orientation stays `.T.` under M5 S10, for either
    /// face sense.** ISO 10303-42 *composes* the bound's orientation
    /// flag with the owning `face_surface.same_sense`: the bound is
    /// stated CCW about the face's outward normal, and `same_sense`
    /// says how that outward normal relates to the surface's own. Our
    /// loops carry exactly that statement already (interior-left), and
    /// [`Self::advanced_face`] emits the sense bit as `same_sense`, so
    /// the composition comes out right with the flag left alone.
    /// Flipping the flag here as well would compose the reversal twice
    /// and hand the reader an inside-out face — the same double-count
    /// hazard the tessellator's winding sites carry. `ORIENTED_EDGE`'s
    /// flag is likewise untouched: it says which half of the edge the
    /// loop traverses, a fact about the half-edge, not about the
    /// face's material side.
    fn face_bound(&mut self, loop_key: LoopKey, outer: bool) -> Result<u64, StepExportError> {
        let loop_ = self
            .body
            .get_loop(loop_key)
            .ok_or(StepExportError::Corrupt {
                what: "loop key does not resolve",
            })?;
        let first = match loop_.boundary {
            LoopBoundary::Empty { .. } => {
                return Err(StepExportError::EmptyLoop { loop_: loop_key });
            }
            LoopBoundary::Cycle { first } => first,
        };
        let cycle = self
            .body
            .loop_cycle(first)
            .ok_or(StepExportError::Corrupt {
                what: "loop cycle does not close",
            })?;
        let mut oriented = Vec::with_capacity(cycle.len());
        for he_key in cycle {
            let he = self
                .body
                .get_half_edge(he_key)
                .ok_or(StepExportError::Corrupt {
                    what: "half-edge key does not resolve",
                })?;
            let edge_key = he.edge;
            let edge = self
                .body
                .get_edge(edge_key)
                .ok_or(StepExportError::Corrupt {
                    what: "half-edge edge key does not resolve",
                })?;
            let flag = if he_key == edge.he_plus {
                ".T."
            } else if he_key == edge.he_minus {
                ".F."
            } else {
                return Err(StepExportError::Corrupt {
                    what: "half-edge is neither half of its edge",
                });
            };
            let ec = self.edge_curve(edge_key)?;
            oriented.push(self.emit(&format!("ORIENTED_EDGE('', *, *, #{ec}, {flag})")));
        }
        let el = self.emit(&format!("EDGE_LOOP('', ({}))", refs(&oriented)));
        let kind = if outer {
            "FACE_OUTER_BOUND"
        } else {
            "FACE_BOUND"
        };
        Ok(self.emit(&format!("{kind}('', #{el}, .T.)")))
    }

    /// `ADVANCED_FACE` for a kernel face: the surface printer first
    /// (so an out-of-subset face refuses by its surface, the primary
    /// fact, not incidentally by a boundary carrier), then bounds
    /// (outer first, rings in stored order), then `same_sense`.
    ///
    /// **`same_sense` IS [`topo::Face::sense`]** (M5 S10). This is not
    /// a mapping the exporter computes — ISO 10303-42's
    /// `face_surface.same_sense` and the kernel's orientation bit are
    /// the same predicate ("does the face's material side agree with
    /// its surface's own normal?"), which is why S10 ratified the bit
    /// in that shape. It emits `.T.`/`.F.` directly; before S10 the
    /// answer was hardcoded `.T.` because the stored normal simply WAS
    /// the outward normal (M1 ratification). Since M5 S11 constructors
    /// mint `sense: false` on material-against-chart walls — all
    /// curved in this build, hence outside the planar subset — so
    /// planar output stays byte-identical and the `.F.` arm is pinned
    /// through the hand-flip door (`m5_s11_same_sense`).
    ///
    /// Consequently the surface's own `AXIS2_PLACEMENT_3D` keeps
    /// emitting the **chart** normal unchanged (see the `direction`
    /// call below) — that is correct *precisely because* `same_sense`
    /// now carries the flip. A reversed face must export its true
    /// surface, with the reversal stated once, in the one field STEP
    /// provides for it; negating the axis instead would export a
    /// different plane and (with `.F.`) double-count. Likewise the
    /// bounds stay `.T.` — see [`Self::face_bound`].
    fn advanced_face(&mut self, face_key: FaceKey) -> Result<u64, StepExportError> {
        let face = self
            .body
            .get_face(face_key)
            .ok_or(StepExportError::Corrupt {
                what: "face key does not resolve",
            })?;
        let outer = face.outer;
        let rings = face.rings.clone();
        // The S10 orientation bit, emitted verbatim as `same_sense`
        // (fn docs: the two are the same predicate, not a conversion).
        let same_sense = if face.sense { ".T." } else { ".F." };
        let surface = self
            .body
            .get_surface(face.surface)
            .ok_or(StepExportError::Corrupt {
                what: "face surface key does not resolve",
            })?;
        // The surface printers: every ELEMENTARY_SURFACE of the
        // kernel's closed enum maps to its exact AP214 twin, the
        // kernel frame going in verbatim as the placement (see
        // [`Self::axis2_placement`]) — never a B-spline approximation
        // of an analytic kind (crate docs). A DESCRIBED NURBS surface
        // prints natively since M6-3 (`Self::b_spline_surface`, the
        // loft walls); the one refusal left is the mvfs placeholder,
        // a mid-surgery fact rather than a frontier.
        let surface_id = match *surface {
            Surface::Plane {
                origin,
                normal,
                u_ref,
            } => {
                let placement = self.axis2_placement(
                    origin,
                    normal,
                    u_ref,
                    "plane origin",
                    "plane normal",
                    "plane u_ref",
                )?;
                self.emit(&format!("PLANE('', #{placement})"))
            }
            // `cylindrical_surface`: S(u, v) = location + radius·(cos
            // u·x + sin u·y) + v·z — the kernel's `Cylinder` formula
            // field for field (`origin` the v = 0 axis point, `u_ref`
            // the seam at u = 0). Both parameters keep their kernel
            // meaning (u radians, v metres).
            Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => {
                let placement = self.axis2_placement(
                    origin,
                    axis,
                    u_ref,
                    "cylinder origin",
                    "cylinder axis",
                    "cylinder u_ref",
                )?;
                let r = fmt_real(radius, "cylinder radius")?;
                self.emit(&format!("CYLINDRICAL_SURFACE('', #{placement}, {r})"))
            }
            // `conical_surface`: S(u, v) = location + (radius + v·tan
            // α)·(cos u·x + sin u·y) + v·z, with the placement's
            // location free to sit anywhere on the axis. The kernel
            // stores the APEX, so the exact encoding is location =
            // apex with `radius = 0.0` (ISO 10303-42's WHERE rule on
            // conical_surface is `radius >= 0.0` — the apex placement
            // is inside the schema, and it is the encoding that needs
            // no invented offset constant, D9).
            //
            // The one field that is NOT an identity: STEP's v runs
            // along the AXIS while the kernel's runs along the SLANT
            // (|∂S/∂v| = 1, `Surface::Cone`'s docs). The two
            // parameterizations of the same locus differ by the fixed
            // factor cos α. That is invisible here — this writer emits
            // no pcurves and no trim parameters, so only the surface
            // LOCUS crosses the wire, and the locus is identical.
            // (When pcurves land, the cos α factor is the thing to
            // carry.) `semi_angle` is the kernel `half_angle`
            // verbatim, both in (0, π/2) radians.
            Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => {
                let placement = self.axis2_placement(
                    apex,
                    axis,
                    u_ref,
                    "cone apex",
                    "cone axis",
                    "cone u_ref",
                )?;
                let angle = fmt_real(half_angle, "cone half-angle")?;
                self.emit(&format!("CONICAL_SURFACE('', #{placement}, 0.0, {angle})"))
            }
            // `spherical_surface`: S(u, v) = centre + radius·(cos v·
            // (cos u·x + sin u·y) + sin v·z) — the kernel's `Sphere`
            // formula field for field (longitude u about the pole
            // `axis`, latitude v, seam meridian at `u_ref`).
            Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            } => {
                let placement = self.axis2_placement(
                    center,
                    axis,
                    u_ref,
                    "sphere centre",
                    "sphere axis",
                    "sphere u_ref",
                )?;
                let r = fmt_real(radius, "sphere radius")?;
                self.emit(&format!("SPHERICAL_SURFACE('', #{placement}, {r})"))
            }
            // `toroidal_surface`: S(u, v) = centre + (R + r·cos v)·(cos
            // u·x + sin u·y) + r·sin v·z — the kernel's `Torus`
            // formula field for field (major azimuth u about `axis`,
            // minor angle v with v = 0 on the outer equator, seam at
            // `u_ref`).
            Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            } => {
                let placement = self.axis2_placement(
                    center,
                    axis,
                    u_ref,
                    "torus centre",
                    "torus axis",
                    "torus u_ref",
                )?;
                let major = fmt_real(major_radius, "torus major radius")?;
                let minor = fmt_real(minor_radius, "torus minor radius")?;
                self.emit(&format!(
                    "TOROIDAL_SURFACE('', #{placement}, {major}, {minor})"
                ))
            }
            // `b_spline_surface_with_knots` (M6-3, walk row 12): the
            // described NURBS surface exports natively, mirroring the
            // curve writer arm for arm — the non-rational simple
            // entity when every weight is exactly 1.0, otherwise the
            // `RATIONAL_B_SPLINE_SURFACE` complex instance. The mvfs
            // PLACEHOLDER keeps refusing (`surface_kind` names it).
            Surface::Nurbs(ref payload) => {
                if payload.is_placeholder() {
                    return Err(StepExportError::UnsupportedSurface {
                        face: face_key,
                        kind: surface_kind(surface),
                    });
                }
                self.b_spline_surface(payload)?
            }
        };
        let mut bounds = Vec::with_capacity(1 + rings.len());
        bounds.push(self.face_bound(outer, true)?);
        for ring in rings {
            bounds.push(self.face_bound(ring, false)?);
        }
        Ok(self.emit(&format!(
            "ADVANCED_FACE('', ({}), #{surface_id}, {same_sense})",
            refs(&bounds)
        )))
    }

    /// `CLOSED_SHELL` over the shell's faces (stored order).
    fn closed_shell(&mut self, shell_key: ShellKey) -> Result<u64, StepExportError> {
        let shell = self
            .body
            .get_shell(shell_key)
            .ok_or(StepExportError::Corrupt {
                what: "shell key does not resolve",
            })?;
        let face_keys = shell.faces.clone();
        let mut faces = Vec::with_capacity(face_keys.len());
        for face_key in face_keys {
            faces.push(self.advanced_face(face_key)?);
        }
        // A faceless CLOSED_SHELL would violate the schema (its face
        // set has a 1..* cardinality). Currently unreachable — an
        // empty body refuses NothingToExport and mvfs-grade skeletons
        // refuse in the face walk — but a latent door if a future op
        // ever mints a shell with an empty face list.
        Ok(self.emit(&format!("CLOSED_SHELL('', ({}))", refs(&faces))))
    }

    /// One `MANIFOLD_SOLID_BREP` per shell of every solid, with the
    /// multi-shell outward/void classification (crate docs: positive
    /// shells are independent solids, negative shells refuse).
    /// Single-shell solids skip classification by design — tier-2
    /// validity and the +V invariant own lone-shell orientation.
    fn manifold_solids(&mut self, name: &str) -> Result<Vec<u64>, StepExportError> {
        // Snapshot the walk order first (arena order for solids, stored
        // order for their shells) so classification runs before any
        // emission for the offending solid.
        let solid_shells: Vec<Vec<ShellKey>> = self
            .body
            .solids()
            .map(|(_, solid)| solid.shells.clone())
            .collect();
        let mut msbs = Vec::new();
        for shells in solid_shells {
            if shells.len() > 1 {
                for &shell_key in &shells {
                    let shell = self
                        .body
                        .get_shell(shell_key)
                        .ok_or(StepExportError::Corrupt {
                            what: "solid shell key does not resolve",
                        })?;
                    let volume = shell_signed_volume(self.body, shell_key, shell)?;
                    if !volume.is_finite() || volume == 0.0 {
                        return Err(StepExportError::ShellVolumeIndeterminate {
                            shell: shell_key,
                            volume,
                        });
                    }
                    if volume < 0.0 {
                        return Err(StepExportError::VoidShellUnsupported {
                            shell: shell_key,
                            volume,
                        });
                    }
                }
            }
            for shell_key in shells {
                let cs = self.closed_shell(shell_key)?;
                msbs.push(self.emit(&format!("MANIFOLD_SOLID_BREP({name}, #{cs})")));
            }
        }
        Ok(msbs)
    }
}

/// Serializes the whole document (the public entry point's engine).
pub(crate) fn write_document(
    body: &Body<f64>,
    options: &StepOptions,
    tol: Tol,
) -> Result<String, StepExportError> {
    let name = quoted(&options.product_name, "product name")?;
    let mut w = Writer::new(body);

    // Geometry + topology.
    let msbs = w.manifold_solids(&name)?;
    if msbs.is_empty() {
        return Err(StepExportError::NothingToExport);
    }

    // World placement (the representation's own axis system — identity).
    let world_origin = w.cartesian_point(Point3::origin(), "world origin")?;
    let world_z = w.direction(Vec3::unit_z(), "world z")?;
    let world_x = w.direction(Vec3::unit_x(), "world x")?;
    let world_axis = w.emit(&format!(
        "AXIS2_PLACEMENT_3D('', #{world_origin}, #{world_z}, #{world_x})"
    ));

    // Units: unprefixed SI metres/radians/steradians (kernel-internal
    // geometry is metres, D6/D4 ¶4). Complex-instance components in
    // alphabetical order as Part 21 requires.
    let lu = w.emit("( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT($, .METRE.) )");
    let au = w.emit("( NAMED_UNIT(*) PLANE_ANGLE_UNIT() SI_UNIT($, .RADIAN.) )");
    let su = w.emit("( NAMED_UNIT(*) SI_UNIT($, .STERADIAN.) SOLID_ANGLE_UNIT() )");

    // Uncertainty: the run's ambient ε (or the explicit override),
    // wrapped in the LENGTH_MEASURE select as the schema requires.
    let eps = match options.uncertainty_m {
        Some(value) => {
            if !(value.is_finite() && value > 0.0) {
                return Err(StepExportError::InvalidUncertainty { value });
            }
            value
        }
        None => tol.eps(),
    };
    let eps_str = fmt_real(eps, "uncertainty")?;
    let unc = w.emit(&format!(
        "UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE({eps_str}), #{lu}, \
         'distance_accuracy_value', '')"
    ));
    let ctx = w.emit(&format!(
        "( GEOMETRIC_REPRESENTATION_CONTEXT(3) \
         GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#{unc})) \
         GLOBAL_UNIT_ASSIGNED_CONTEXT((#{lu}, #{au}, #{su})) \
         REPRESENTATION_CONTEXT('Context #1', '3D') )"
    ));

    // The advanced-B-rep representation and the AP214 product chain.
    let mut items = vec![world_axis];
    items.extend_from_slice(&msbs);
    let absr = w.emit(&format!(
        "ADVANCED_BREP_SHAPE_REPRESENTATION('', ({}), #{ctx})",
        refs(&items)
    ));
    let app = w.emit("APPLICATION_CONTEXT('core data for automotive mechanical design processes')");
    w.emit(&format!(
        "APPLICATION_PROTOCOL_DEFINITION('international standard', 'automotive_design', \
         2000, #{app})"
    ));
    let pctx = w.emit(&format!("PRODUCT_CONTEXT('', #{app}, 'mechanical')"));
    let prod = w.emit(&format!("PRODUCT({name}, {name}, '', (#{pctx}))"));
    let pdf = w.emit(&format!("PRODUCT_DEFINITION_FORMATION('', '', #{prod})"));
    let pdc = w.emit(&format!(
        "PRODUCT_DEFINITION_CONTEXT('part definition', #{app}, 'design')"
    ));
    let pd = w.emit(&format!("PRODUCT_DEFINITION('design', '', #{pdf}, #{pdc})"));
    let pds = w.emit(&format!("PRODUCT_DEFINITION_SHAPE('', '', #{pd})"));
    w.emit(&format!("SHAPE_DEFINITION_REPRESENTATION(#{pds}, #{absr})"));
    w.emit(&format!(
        "PRODUCT_RELATED_PRODUCT_CATEGORY('part', $, (#{prod}))"
    ));

    // Header (fixed template — FILE_SCHEMA names the AP the data
    // section actually instantiates: AP214 ed. 2).
    let file_name = quoted(
        &format!("{}.step", options.product_name),
        "product name (file name)",
    )?;
    let timestamp = quoted(&options.timestamp, "timestamp")?;
    let author = quoted(&options.author, "author")?;
    let organization = quoted(&options.organization, "organization")?;
    let originating = quoted(&options.originating_system, "originating system")?;
    Ok(format!(
        "ISO-10303-21;\n\
         HEADER;\n\
         FILE_DESCRIPTION((''), '2;1');\n\
         FILE_NAME({file_name}, {timestamp}, ({author}), ({organization}), \
         'step-export', {originating}, '');\n\
         FILE_SCHEMA(('AUTOMOTIVE_DESIGN {{ 1 0 10303 214 1 1 1 1 }}'));\n\
         ENDSEC;\n\
         DATA;\n\
         {}\
         ENDSEC;\n\
         END-ISO-10303-21;\n",
        w.data
    ))
}

#[cfg(test)]
mod tests {
    //! Record-level pins for the `B_SPLINE_CURVE_WITH_KNOTS` arm.
    //!
    //! These are unit tests rather than acceptance rows because **no
    //! body at rest carries a `Curve3::Nurbs` carrier**: the kernel's
    //! only mint site for one is an SSI rung-3 branch, and no public
    //! constructor reaches it in this build (the repo's single
    //! certified rung-3 edge is a hand-built scaffold in `topo`'s own
    //! suite, guarded by a fit-sample budget). The arm exists because
    //! the entity is part of the curved subset the plan names and
    //! because the alternative — refusing a carrier the schema can
    //! carry exactly — would be a worse frontier than an untravelled
    //! code path. Pinning the exact record text is what keeps that path
    //! honest until a body arrives to walk it.

    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use geom::NurbsCurve3;
    use geom_core::Point3;
    use geom_core::spline::KnotVector;

    use super::*;

    /// Emits one NURBS carrier into a fresh writer and returns the
    /// final record (the entity under test; the control points are the
    /// records before it).
    fn emitted(curve: &NurbsCurve3<f64>) -> String {
        let body = Body::<f64>::new();
        let mut w = Writer::new(&body);
        w.b_spline_curve(curve.knots(), curve.control(), curve.weights())
            .expect("a validated curve prints");
        w.data
            .lines()
            .next_back()
            .expect("at least one record")
            .to_owned()
    }

    /// A **non-rational** curve takes the simple entity: unit weights
    /// are recognized exactly (weights are f64 structure, C6 — the
    /// decision is `== 1.0`, not a tolerance), so no
    /// `RATIONAL_B_SPLINE_CURVE` composition appears.
    #[test]
    fn unit_weights_emit_the_simple_entity() {
        let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 2.0, 0.0),
            Point3::new(3.0, 2.0, 0.0),
            Point3::new(4.0, 0.0, 0.0),
        ];
        let curve = NurbsCurve3::new(knots, control, vec![1.0; 4]).unwrap();
        assert_eq!(
            emitted(&curve),
            "#5 = B_SPLINE_CURVE_WITH_KNOTS('', 3, (#1, #2, #3, #4), .UNSPECIFIED., \
             .U., .U., (4, 4), (0.0, 1.0), .UNSPECIFIED.);"
        );
    }

    /// A **rational** curve takes the complex instance, components in
    /// alphabetical order (`BOUNDED_CURVE` before `B_SPLINE_CURVE`:
    /// Part 21 orders by the entity NAME, and `O` < `_` in ASCII).
    /// The payload here is the exact quarter circle of NURBS Book
    /// Eq. 7.33 — `w₁ = cos θ` — which is also the concrete answer to
    /// "why do conics not go this way": the same arc is one `CIRCLE`
    /// record with five reals.
    #[test]
    fn mixed_weights_emit_the_rational_complex_instance() {
        let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let control = vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
        ];
        let w1 = core::f64::consts::FRAC_1_SQRT_2;
        let curve = NurbsCurve3::new(knots, control, vec![1.0, w1, 1.0]).unwrap();
        assert_eq!(
            emitted(&curve),
            "#4 = ( BOUNDED_CURVE() B_SPLINE_CURVE(2, (#1, #2, #3), .UNSPECIFIED., .U., .U.) \
             B_SPLINE_CURVE_WITH_KNOTS((3, 3), (0.0, 1.0), .UNSPECIFIED.) CURVE() \
             GEOMETRIC_REPRESENTATION_ITEM() RATIONAL_B_SPLINE_CURVE((1.0, \
             0.7071067811865476, 1.0)) REPRESENTATION_ITEM('') );"
        );
    }

    /// Interior knots run-length-encode by exact equality, ends come
    /// out clamped at multiplicity `degree + 1`, and the pair
    /// reconstructs the flat vector the kernel stores.
    #[test]
    fn interior_knot_multiplicities_encode_exactly() {
        let flat = vec![0.0, 0.0, 0.0, 0.25, 0.5, 0.5, 1.0, 1.0, 1.0];
        let knots = KnotVector::clamped(flat.clone(), 2).unwrap();
        let (mults, values) = run_length_knots(&knots, "test").unwrap();
        assert_eq!(mults, "3, 1, 2, 3");
        assert_eq!(values, "0.0, 0.25, 0.5, 1.0");
        // Round trip: multiplicities × values rebuild the flat vector.
        let rebuilt: Vec<f64> = mults
            .split(", ")
            .zip(values.split(", "))
            .flat_map(|(m, v)| {
                let v: f64 = v.parse().unwrap();
                std::iter::repeat_n(v, m.parse().unwrap())
            })
            .collect();
        assert_eq!(rebuilt, flat);
    }

    /// **The two `Surface::Nurbs` states are told apart.** This row
    /// pins the surface records at byte level exactly as the curve
    /// rows above pin theirs — both arms, the non-rational simple
    /// entity and the RATIONAL complex instance, even though the first
    /// corpus body is non-rational: cheap, and it keeps the arm from
    /// being dead code.
    #[test]
    fn the_nurbs_surface_arms_emit_and_the_placeholder_refuses() {
        let placeholder = Surface::<f64>::nurbs_placeholder();
        assert_eq!(surface_kind(&placeholder), "nurbs placeholder");

        let patch = geom::NurbsSurface::new(
            KnotVector::unit_segment(1),
            KnotVector::unit_segment(1),
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, 1.0, 0.25),
            ],
            vec![1.0; 4],
        )
        .expect("a validated bilinear patch");
        assert_eq!(
            surface_kind(&Surface::Nurbs(patch.clone().into())),
            "nurbs surface"
        );

        // Record pins, both arms (the curve pins' idiom).
        let body = Body::<f64>::new();
        let mut w = Writer::new(&body);
        w.b_spline_surface(&patch).expect("non-rational arm emits");
        assert!(
            w.data.contains(
                "B_SPLINE_SURFACE_WITH_KNOTS('', 1, 1, ((#1, #2), (#3, #4)), .UNSPECIFIED., \
                 .U., .U., .U., (2, 2), (2, 2), (0.0, 1.0), (0.0, 1.0), .UNSPECIFIED.)"
            ),
            "non-rational record: {}",
            w.data
        );

        let rational = geom::NurbsSurface::new(
            KnotVector::unit_segment(1),
            KnotVector::unit_segment(1),
            patch.control().to_vec(),
            vec![1.0, 2.0, 1.0, 1.0],
        )
        .expect("a rational patch");
        let mut w = Writer::new(&body);
        w.b_spline_surface(&rational).expect("rational arm emits");
        assert!(
            w.data.contains(
                "( B_SPLINE_SURFACE(1, 1, ((#1, #2), (#3, #4)), .UNSPECIFIED., .U., .U., .U.) \
                 B_SPLINE_SURFACE_WITH_KNOTS((2, 2), (2, 2), (0.0, 1.0), (0.0, 1.0), \
                 .UNSPECIFIED.) BOUNDED_SURFACE() GEOMETRIC_REPRESENTATION_ITEM() \
                 RATIONAL_B_SPLINE_SURFACE(((1.0, 2.0), (1.0, 1.0))) \
                 REPRESENTATION_ITEM('') SURFACE() )"
            ),
            "rational record: {}",
            w.data
        );
    }

    /// The "no description yet" NURBS carrier refuses typed rather
    /// than printing poison — the curve-side twin of the mvfs surface
    /// placeholder, and the one live `UnsupportedCurve` case.
    #[test]
    fn the_carrier_placeholder_refuses_typed() {
        let placeholder = Curve3::<f64>::nurbs_placeholder();
        let Curve3::Nurbs(ref payload) = placeholder else {
            panic!("the placeholder is a NURBS curve");
        };
        assert!(payload.is_placeholder());
        assert_eq!(carrier_kind(&placeholder), "nurbs curve");
    }
}
