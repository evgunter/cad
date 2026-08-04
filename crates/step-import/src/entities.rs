//! AP214 entity resolution (Leg B): the typed walk from parsed
//! records to kernel-typed geometry and a shell/face/loop/edge
//! description ready for assembly.
//!
//! The mapping is the **inverse of the writer's identity mapping**
//! (`step-export`'s crate docs): every `axis2_placement_3d` becomes a
//! kernel frame field for field, every elementary surface and conic
//! its kernel twin, control points / knots / weights exact. Nothing
//! is renormalized or re-derived — the one recognized convention is
//! the cone's apex placement (`radius = 0.0`), where the placement
//! location IS the kernel apex. Units and uncertainty are *read*
//! (parsed, not assumed), refusing typed on anything the subset does
//! not cover.

use std::collections::BTreeMap;

use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};
use geom_curves::{Curve3, NurbsCurve3};
use geom_surfaces::Surface;

use crate::error::StepImportError;
use crate::geometry;
use crate::parse::{Instance, Record, StepFile, Value};

/// A resolved model: the shape content plus the file's declared
/// uncertainty (D7's ε_in default).
#[derive(Debug)]
pub(crate) struct Model {
    /// The declared `UNCERTAINTY_MEASURE_WITH_UNIT` value (meters).
    pub(crate) uncertainty_m: f64,
    /// What the data section carries.
    pub(crate) shape: Shape,
}

/// The shape content of the data section.
#[derive(Debug)]
pub(crate) enum Shape {
    /// `MANIFOLD_SOLID_BREP`s, in entity-id order (the writer's
    /// emission order).
    Solids(Vec<SolidSpec>),
    /// A `GEOMETRIC_CURVE_SET` wireframe (curve-only file): the
    /// reconstructed carriers, in set order.
    Wireframe(Vec<Curve3<f64>>),
}

/// One `MANIFOLD_SOLID_BREP`: a closed shell to assemble.
#[derive(Debug)]
pub(crate) struct SolidSpec {
    /// The MSB entity id.
    pub(crate) id: u64,
    /// The shell's faces, in `CLOSED_SHELL` order.
    pub(crate) faces: Vec<FaceSpec>,
    /// Every edge referenced by this shell's loops, keyed by
    /// `EDGE_CURVE` id (deterministic order).
    pub(crate) edges: BTreeMap<u64, EdgeSpec>,
    /// Every vertex referenced, keyed by `VERTEX_POINT` id.
    pub(crate) vertices: BTreeMap<u64, Point3<f64>>,
}

/// One `ADVANCED_FACE`.
#[derive(Debug)]
pub(crate) struct FaceSpec {
    /// The mapped kernel surface, field for field.
    pub(crate) surface: Surface<f64>,
    /// `same_sense`, honored verbatim as [`topo::Face::sense`].
    pub(crate) sense: bool,
    /// The bounds: outer first (`FACE_OUTER_BOUND`), rings after in
    /// stored order.
    pub(crate) loops: Vec<LoopSpec>,
}

/// One face bound's edge loop.
#[derive(Debug)]
pub(crate) struct LoopSpec {
    /// True for the `FACE_OUTER_BOUND`.
    pub(crate) outer: bool,
    /// The oriented edge uses, in cycle order.
    pub(crate) uses: Vec<EdgeUse>,
}

/// One `ORIENTED_EDGE`: which edge, traversed which way.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct EdgeUse {
    /// The `EDGE_CURVE` entity id.
    pub(crate) edge: u64,
    /// The orientation flag: `true` traverses start → end (the
    /// carrier's increasing parameter — the edge's plus half).
    pub(crate) forward: bool,
}

/// One `EDGE_CURVE`: vertices, exact carrier, and the derived
/// carrier-parameter interval.
#[derive(Debug)]
pub(crate) struct EdgeSpec {
    /// Start `VERTEX_POINT` id (the plus half's start).
    pub(crate) start: u64,
    /// End `VERTEX_POINT` id.
    pub(crate) end: u64,
    /// The mapped kernel carrier, exact.
    pub(crate) carrier: Curve3<f64>,
    /// Carrier parameter at `start` (derived — the file carries no
    /// trim parameters in this subset; [`geometry::endpoint_params`]).
    pub(crate) t0: f64,
    /// Carrier parameter at `end`; `t0 < t1`, periodic carriers
    /// normalized to the increasing-parameter arc (full period for a
    /// self-loop).
    pub(crate) t1: f64,
}

/// The resolver: the parsed file plus typed accessors that turn
/// dangling references and shape mismatches into named errors.
pub(crate) struct Resolver<'a> {
    file: &'a StepFile,
}

impl<'a> Resolver<'a> {
    /// The instance behind `id`, or a dangling-reference error naming
    /// the referencing instance `from`.
    fn instance(&self, from: u64, id: u64) -> Result<&'a Instance, StepImportError> {
        self.file
            .data
            .get(&id)
            .ok_or(StepImportError::DanglingReference { from, to: id })
    }

    /// The single record of a **simple** instance, checked against
    /// `expected` keyword.
    fn simple(
        &self,
        from: u64,
        id: u64,
        expected: &'static str,
    ) -> Result<&'a [Value], StepImportError> {
        let instance = self.instance(from, id)?;
        match instance.records.as_slice() {
            [(kw, args)] if kw == expected => Ok(args),
            [(kw, _)] => Err(StepImportError::WrongEntityType {
                id,
                expected,
                found: kw.clone(),
            }),
            records => Err(StepImportError::WrongEntityType {
                id,
                expected,
                found: complex_name(records),
            }),
        }
    }
}

/// A complex instance's display name: the component keywords joined.
fn complex_name(records: &[Record]) -> String {
    let mut out = String::from("complex(");
    for (i, (kw, _)) in records.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        out.push_str(kw);
    }
    out.push(')');
    out
}

// ---- Value accessors: shape mismatches become named errors. --------

/// `value` as an entity reference.
fn as_ref(id: u64, value: &Value, expected: &'static str) -> Result<u64, StepImportError> {
    match value {
        Value::Ref(r) => Ok(*r),
        _ => Err(StepImportError::MalformedRecord { id, expected }),
    }
}

/// `value` as a real (`str::parse::<f64>` — bit-exact against the
/// writer's printer).
fn as_real(id: u64, value: &Value, expected: &'static str) -> Result<f64, StepImportError> {
    match value {
        Value::Number(raw) => raw.parse().map_err(|_| StepImportError::MalformedReal {
            id,
            token: raw.clone(),
        }),
        _ => Err(StepImportError::MalformedRecord { id, expected }),
    }
}

/// `value` as a nonnegative integer (degrees, multiplicities).
fn as_usize(id: u64, value: &Value, expected: &'static str) -> Result<usize, StepImportError> {
    match value {
        Value::Number(raw) => raw.parse().map_err(|_| StepImportError::MalformedRecord {
            id,
            expected,
        }),
        _ => Err(StepImportError::MalformedRecord { id, expected }),
    }
}

/// `value` as a list slice.
fn as_list<'v>(
    id: u64,
    value: &'v Value,
    expected: &'static str,
) -> Result<&'v [Value], StepImportError> {
    match value {
        Value::List(items) => Ok(items),
        _ => Err(StepImportError::MalformedRecord { id, expected }),
    }
}

/// `value` as a BOOLEAN enum (`.T.` / `.F.`).
fn as_bool(id: u64, value: &Value, expected: &'static str) -> Result<bool, StepImportError> {
    match value {
        Value::Enum(name) if name == "T" => Ok(true),
        Value::Enum(name) if name == "F" => Ok(false),
        _ => Err(StepImportError::MalformedRecord { id, expected }),
    }
}

impl<'a> Resolver<'a> {
    /// `CARTESIAN_POINT('', (x, y, z))` → kernel point, exact.
    fn point(&self, from: u64, id: u64) -> Result<Point3<f64>, StepImportError> {
        let args = self.simple(from, id, "CARTESIAN_POINT")?;
        let [_, coords] = args else {
            return Err(StepImportError::MalformedRecord {
                id,
                expected: "CARTESIAN_POINT(name, (x, y, z))",
            });
        };
        let expected = "three coordinates";
        let [x, y, z] = as_list(id, coords, expected)? else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        Ok(Point3::new(
            as_real(id, x, expected)?,
            as_real(id, y, expected)?,
            as_real(id, z, expected)?,
        ))
    }

    /// `DIRECTION('', (x, y, z))` → kernel vector, exact (emitted as
    /// stored — never renormalized here either).
    fn direction(&self, from: u64, id: u64) -> Result<Vec3<f64>, StepImportError> {
        let args = self.simple(from, id, "DIRECTION")?;
        let [_, ratios] = args else {
            return Err(StepImportError::MalformedRecord {
                id,
                expected: "DIRECTION(name, (x, y, z))",
            });
        };
        let expected = "three direction ratios";
        let [x, y, z] = as_list(id, ratios, expected)? else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        Ok(Vec3::new(
            as_real(id, x, expected)?,
            as_real(id, y, expected)?,
            as_real(id, z, expected)?,
        ))
    }

    /// `AXIS2_PLACEMENT_3D('', #location, #axis, #ref_direction)` →
    /// the kernel frame `(origin, axis, u_ref)` field for field. The
    /// schema allows `$` axis/ref_direction; the subset does not (the
    /// writer always emits the full frame) — defaulted axes would be a
    /// re-derivation, refused rather than guessed.
    fn placement(
        &self,
        from: u64,
        id: u64,
    ) -> Result<(Point3<f64>, Vec3<f64>, Vec3<f64>), StepImportError> {
        let args = self.simple(from, id, "AXIS2_PLACEMENT_3D")?;
        let expected = "AXIS2_PLACEMENT_3D(name, #location, #axis, #ref_direction) \
                        with all three references present (the exported subset)";
        let [_, location, axis, ref_dir] = args else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        let origin = self.point(id, as_ref(id, location, expected)?)?;
        let axis = self.direction(id, as_ref(id, axis, expected)?)?;
        let u_ref = self.direction(id, as_ref(id, ref_dir, expected)?)?;
        Ok((origin, axis, u_ref))
    }

    /// An `ADVANCED_FACE`'s surface reference → the kernel surface,
    /// field for field (the writer's printer table, inverted). Any
    /// surface type outside the five elementary kinds refuses typed —
    /// `B_SPLINE_SURFACE_WITH_KNOTS` is the named M7 frontier.
    fn surface(&self, from: u64, id: u64) -> Result<Surface<f64>, StepImportError> {
        let instance = self.instance(from, id)?;
        let [(kw, args)] = instance.records.as_slice() else {
            return Err(StepImportError::UnsupportedEntity {
                id,
                keyword: complex_name(&instance.records),
            });
        };
        match kw.as_str() {
            "PLANE" => {
                let expected = "PLANE(name, #placement)";
                let [_, placement] = args.as_slice() else {
                    return Err(StepImportError::MalformedRecord { id, expected });
                };
                let (origin, normal, u_ref) =
                    self.placement(id, as_ref(id, placement, expected)?)?;
                Ok(Surface::Plane {
                    origin,
                    normal,
                    u_ref,
                })
            }
            "CYLINDRICAL_SURFACE" => {
                let expected = "CYLINDRICAL_SURFACE(name, #placement, radius)";
                let [_, placement, radius] = args.as_slice() else {
                    return Err(StepImportError::MalformedRecord { id, expected });
                };
                let (origin, axis, u_ref) =
                    self.placement(id, as_ref(id, placement, expected)?)?;
                Ok(Surface::Cylinder {
                    origin,
                    axis,
                    radius: as_real(id, radius, expected)?,
                    u_ref,
                })
            }
            "CONICAL_SURFACE" => {
                // The apex-placement convention (writer subset): the
                // placement location IS the kernel apex iff the record
                // radius is exactly 0.0. A nonzero radius would need
                // the apex derived (location − (radius / tan α)·axis)
                // — a computation, not an identity — and is deferred
                // to the M7-2 adoption ladder, refused here. STEP's
                // axial v vs the kernel's slant-arc-length v (the
                // fixed cos α factor) has nothing to act on in this
                // subset: no trim parameters or pcurves cross the
                // wire, and the surface LOCUS fields are identical.
                let expected =
                    "CONICAL_SURFACE(name, #placement, 0.0, semi_angle) — the apex \
                     placement (radius = 0.0) is the exported subset";
                let [_, placement, radius, semi_angle] = args.as_slice() else {
                    return Err(StepImportError::MalformedRecord { id, expected });
                };
                if as_real(id, radius, expected)? != 0.0 {
                    return Err(StepImportError::MalformedRecord { id, expected });
                }
                let (apex, axis, u_ref) =
                    self.placement(id, as_ref(id, placement, expected)?)?;
                Ok(Surface::Cone {
                    apex,
                    axis,
                    half_angle: as_real(id, semi_angle, expected)?,
                    u_ref,
                })
            }
            "SPHERICAL_SURFACE" => {
                let expected = "SPHERICAL_SURFACE(name, #placement, radius)";
                let [_, placement, radius] = args.as_slice() else {
                    return Err(StepImportError::MalformedRecord { id, expected });
                };
                let (center, axis, u_ref) =
                    self.placement(id, as_ref(id, placement, expected)?)?;
                Ok(Surface::Sphere {
                    center,
                    radius: as_real(id, radius, expected)?,
                    axis,
                    u_ref,
                })
            }
            "TOROIDAL_SURFACE" => {
                let expected = "TOROIDAL_SURFACE(name, #placement, major, minor)";
                let [_, placement, major, minor] = args.as_slice() else {
                    return Err(StepImportError::MalformedRecord { id, expected });
                };
                let (center, axis, u_ref) =
                    self.placement(id, as_ref(id, placement, expected)?)?;
                Ok(Surface::Torus {
                    center,
                    axis,
                    major_radius: as_real(id, major, expected)?,
                    minor_radius: as_real(id, minor, expected)?,
                    u_ref,
                })
            }
            other => Err(StepImportError::UnsupportedEntity {
                id,
                keyword: other.to_owned(),
            }),
        }
    }

    /// An `EDGE_CURVE`'s (or curve set's) curve reference → the kernel
    /// carrier, exact. Covers the writer's four printers: `LINE`,
    /// `CIRCLE`, `ELLIPSE`, `B_SPLINE_CURVE_WITH_KNOTS` (simple
    /// non-rational form and the `RATIONAL_B_SPLINE_CURVE` complex
    /// instance).
    fn curve(&self, from: u64, id: u64) -> Result<Curve3<f64>, StepImportError> {
        let instance = self.instance(from, id)?;
        if instance.records.len() > 1 {
            return self.rational_bspline(id, &instance.records);
        }
        let [(kw, args)] = instance.records.as_slice() else {
            return Err(StepImportError::MalformedRecord {
                id,
                expected: "a curve record",
            });
        };
        match kw.as_str() {
            "LINE" => {
                let expected = "LINE(name, #point, #vector)";
                let [_, point, vector] = args.as_slice() else {
                    return Err(StepImportError::MalformedRecord { id, expected });
                };
                let origin = self.point(id, as_ref(id, point, expected)?)?;
                let vec_id = as_ref(id, vector, expected)?;
                let vargs = self.simple(id, vec_id, "VECTOR")?;
                let vexpected = "VECTOR(name, #direction, 1.0) — the subset's line \
                                 parameter is arc length, so the magnitude is 1.0";
                let [_, dir_ref, magnitude] = vargs else {
                    return Err(StepImportError::MalformedRecord {
                        id: vec_id,
                        expected: vexpected,
                    });
                };
                // Magnitude 1.0 keeps the STEP parameter equal to the
                // kernel's arc-length parameter (the writer's
                // convention); any other magnitude would rescale the
                // parameterization — a re-derivation, refused.
                if as_real(vec_id, magnitude, vexpected)? != 1.0 {
                    return Err(StepImportError::MalformedRecord {
                        id: vec_id,
                        expected: vexpected,
                    });
                }
                let dir = self.direction(vec_id, as_ref(vec_id, dir_ref, vexpected)?)?;
                Ok(Curve3::Line { origin, dir })
            }
            "CIRCLE" => {
                let expected = "CIRCLE(name, #placement, radius)";
                let [_, placement, radius] = args.as_slice() else {
                    return Err(StepImportError::MalformedRecord { id, expected });
                };
                let (center, axis, u_ref) =
                    self.placement(id, as_ref(id, placement, expected)?)?;
                Ok(Curve3::Circle {
                    center,
                    axis,
                    radius: as_real(id, radius, expected)?,
                    u_ref,
                })
            }
            "ELLIPSE" => {
                let expected = "ELLIPSE(name, #placement, semi_axis_1, semi_axis_2)";
                let [_, placement, major, minor] = args.as_slice() else {
                    return Err(StepImportError::MalformedRecord { id, expected });
                };
                let (center, axis, u_ref) =
                    self.placement(id, as_ref(id, placement, expected)?)?;
                Ok(Curve3::Ellipse {
                    center,
                    axis,
                    major: as_real(id, major, expected)?,
                    minor: as_real(id, minor, expected)?,
                    u_ref,
                })
            }
            "B_SPLINE_CURVE_WITH_KNOTS" => {
                let expected = "B_SPLINE_CURVE_WITH_KNOTS(name, degree, (points), \
                                form, closed, self_intersect, (mults), (knots), spec)";
                let [_, degree, points, _, _, _, mults, knots, _] = args.as_slice() else {
                    return Err(StepImportError::MalformedRecord { id, expected });
                };
                let control = self.control_points(id, points, expected)?;
                let weights = vec![1.0; control.len()];
                let knots = self.knot_vector(id, degree, mults, knots, expected)?;
                self.nurbs(id, knots, control, weights)
            }
            other => Err(StepImportError::UnsupportedEntity {
                id,
                keyword: other.to_owned(),
            }),
        }
    }

    /// The `RATIONAL_B_SPLINE_CURVE` complex instance (weights ≠ 1):
    /// components in alphabetical order, the writer's emission —
    /// degree and control points on `B_SPLINE_CURVE`, knots on
    /// `B_SPLINE_CURVE_WITH_KNOTS`, weights on
    /// `RATIONAL_B_SPLINE_CURVE`.
    fn rational_bspline(
        &self,
        id: u64,
        records: &[Record],
    ) -> Result<Curve3<f64>, StepImportError> {
        let expected = "the RATIONAL_B_SPLINE_CURVE complex instance (BOUNDED_CURVE \
                        B_SPLINE_CURVE B_SPLINE_CURVE_WITH_KNOTS CURVE \
                        GEOMETRIC_REPRESENTATION_ITEM RATIONAL_B_SPLINE_CURVE \
                        REPRESENTATION_ITEM)";
        let mut base: Option<&[Value]> = None;
        let mut with_knots: Option<&[Value]> = None;
        let mut rational: Option<&[Value]> = None;
        for (kw, args) in records {
            match kw.as_str() {
                "B_SPLINE_CURVE" => base = Some(args),
                "B_SPLINE_CURVE_WITH_KNOTS" => with_knots = Some(args),
                "RATIONAL_B_SPLINE_CURVE" => rational = Some(args),
                "BOUNDED_CURVE" | "CURVE" | "GEOMETRIC_REPRESENTATION_ITEM"
                | "REPRESENTATION_ITEM" => {}
                other => {
                    return Err(StepImportError::UnsupportedEntity {
                        id,
                        keyword: format!("complex-instance component {other}"),
                    });
                }
            }
        }
        let (Some(base), Some(wk), Some(rational)) = (base, with_knots, rational) else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        let [degree, points, _, _, _] = base else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        let [mults, knots, _] = wk else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        let [weight_list] = rational else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        let control = self.control_points(id, points, expected)?;
        let mut weights = Vec::new();
        for w in as_list(id, weight_list, expected)? {
            weights.push(as_real(id, w, expected)?);
        }
        let knots = self.knot_vector(id, degree, mults, knots, expected)?;
        self.nurbs(id, knots, control, weights)
    }

    /// A control-point reference list → kernel points.
    fn control_points(
        &self,
        id: u64,
        points: &Value,
        expected: &'static str,
    ) -> Result<Vec<Point3<f64>>, StepImportError> {
        let mut out = Vec::new();
        for p in as_list(id, points, expected)? {
            out.push(self.point(id, as_ref(id, p, expected)?)?);
        }
        Ok(out)
    }

    /// The `(multiplicities) / (knots)` pair → the kernel's flat
    /// clamped knot vector, exact (run-length decode is the inverse of
    /// the writer's exact-equality encode; no ε enters).
    fn knot_vector(
        &self,
        id: u64,
        degree: &Value,
        mults: &Value,
        knots: &Value,
        expected: &'static str,
    ) -> Result<KnotVector, StepImportError> {
        let degree = as_usize(id, degree, expected)?;
        let mults = as_list(id, mults, expected)?;
        let values = as_list(id, knots, expected)?;
        if mults.len() != values.len() {
            return Err(StepImportError::MalformedRecord { id, expected });
        }
        let mut flat = Vec::new();
        for (m, v) in mults.iter().zip(values) {
            let m = as_usize(id, m, expected)?;
            let v = as_real(id, v, expected)?;
            flat.extend(std::iter::repeat_n(v, m));
        }
        KnotVector::clamped(flat, degree).map_err(|_| StepImportError::MalformedRecord {
            id,
            expected: "a clamped knot vector (degree+1 end multiplicities, \
                       nondecreasing interior knots)",
        })
    }

    /// A validated kernel NURBS carrier from exact components.
    fn nurbs(
        &self,
        id: u64,
        knots: KnotVector,
        control: Vec<Point3<f64>>,
        weights: Vec<f64>,
    ) -> Result<Curve3<f64>, StepImportError> {
        NurbsCurve3::new(knots, control, weights)
            .map(|payload| Curve3::Nurbs(std::sync::Arc::new(payload)))
            .map_err(|_| StepImportError::MalformedRecord {
                id,
                expected: "a structurally valid B-spline (control/weight/knot counts \
                           consistent, weights positive)",
            })
    }

    /// `VERTEX_POINT('', #point)` → the vertex position.
    fn vertex(&self, from: u64, id: u64) -> Result<Point3<f64>, StepImportError> {
        let args = self.simple(from, id, "VERTEX_POINT")?;
        let expected = "VERTEX_POINT(name, #point)";
        let [_, point] = args else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        self.point(id, as_ref(id, point, expected)?)
    }

    /// One `ADVANCED_FACE`, its bounds and their loops, accumulating
    /// shared vertices/edges into the shell tables.
    fn face(
        &self,
        from: u64,
        id: u64,
        edges: &mut BTreeMap<u64, EdgeSpec>,
        vertices: &mut BTreeMap<u64, Point3<f64>>,
    ) -> Result<FaceSpec, StepImportError> {
        let args = self.simple(from, id, "ADVANCED_FACE")?;
        let expected = "ADVANCED_FACE(name, (bounds), #surface, same_sense)";
        let [_, bounds, surface_ref, same_sense] = args else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        let surface_id = as_ref(id, surface_ref, expected)?;
        let surface = self.surface(id, surface_id)?;
        let sense = as_bool(id, same_sense, expected)?;
        let mut loops = Vec::new();
        for bound in as_list(id, bounds, expected)? {
            let bound_id = as_ref(id, bound, expected)?;
            loops.push(self.bound(id, bound_id, edges, vertices)?);
        }
        // Exactly one FACE_OUTER_BOUND, first (the writer's emission
        // order; a face with no outer bound has no material side).
        let outer_count = loops.iter().filter(|l| l.outer).count();
        if outer_count != 1 || !loops[0].outer {
            return Err(StepImportError::Topology {
                id,
                what: "an ADVANCED_FACE needs exactly one FACE_OUTER_BOUND, listed \
                       before its rings (the exported subset's order)",
            });
        }
        Ok(FaceSpec {
            surface,
            sense,
            loops,
        })
    }

    /// One `FACE_OUTER_BOUND` / `FACE_BOUND` and its `EDGE_LOOP`.
    fn bound(
        &self,
        from: u64,
        id: u64,
        edges: &mut BTreeMap<u64, EdgeSpec>,
        vertices: &mut BTreeMap<u64, Point3<f64>>,
    ) -> Result<LoopSpec, StepImportError> {
        let instance = self.instance(from, id)?;
        let [(kw, args)] = instance.records.as_slice() else {
            return Err(StepImportError::WrongEntityType {
                id,
                expected: "FACE_OUTER_BOUND or FACE_BOUND",
                found: complex_name(&instance.records),
            });
        };
        let outer = match kw.as_str() {
            "FACE_OUTER_BOUND" => true,
            "FACE_BOUND" => false,
            other => {
                return Err(StepImportError::WrongEntityType {
                    id,
                    expected: "FACE_OUTER_BOUND or FACE_BOUND",
                    found: other.to_owned(),
                });
            }
        };
        let expected = "FACE_BOUND(name, #edge_loop, .T.) — the exported subset \
                        states loops in STEP's own orientation, so a reversed bound \
                        is outside it";
        let [_, loop_ref, orientation] = args.as_slice() else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        // Orientation .T. only: the writer emits the stored (already
        // STEP-convention) traversal. Honoring a .F. bound would mean
        // reversing a loop — M7-2 vocabulary, refused here rather
        // than guessed (D7: honored, not healed).
        if !as_bool(id, orientation, expected)? {
            return Err(StepImportError::Topology { id, what: expected });
        }
        let loop_id = as_ref(id, loop_ref, expected)?;
        let largs = self.simple(id, loop_id, "EDGE_LOOP")?;
        let lexpected = "EDGE_LOOP(name, (oriented edges))";
        let [_, oriented] = largs else {
            return Err(StepImportError::MalformedRecord {
                id: loop_id,
                expected: lexpected,
            });
        };
        let mut uses = Vec::new();
        for oe in as_list(loop_id, oriented, lexpected)? {
            let oe_id = as_ref(loop_id, oe, lexpected)?;
            let oargs = self.simple(loop_id, oe_id, "ORIENTED_EDGE")?;
            let oexpected = "ORIENTED_EDGE(name, *, *, #edge_curve, orientation)";
            let [_, _, _, edge_ref, orientation] = oargs else {
                return Err(StepImportError::MalformedRecord {
                    id: oe_id,
                    expected: oexpected,
                });
            };
            let edge_id = as_ref(oe_id, edge_ref, oexpected)?;
            let forward = as_bool(oe_id, orientation, oexpected)?;
            if !edges.contains_key(&edge_id) {
                let spec = self.edge(oe_id, edge_id, vertices)?;
                edges.insert(edge_id, spec);
            }
            uses.push(EdgeUse {
                edge: edge_id,
                forward,
            });
        }
        if uses.is_empty() {
            return Err(StepImportError::Topology {
                id: loop_id,
                what: "an EDGE_LOOP with no oriented edges — an empty loop is a \
                       mid-construction state, not part of a finished solid",
            });
        }
        Ok(LoopSpec { outer, uses })
    }

    /// One `EDGE_CURVE`: vertices, carrier, and the derived parameter
    /// interval.
    fn edge(
        &self,
        from: u64,
        id: u64,
        vertices: &mut BTreeMap<u64, Point3<f64>>,
    ) -> Result<EdgeSpec, StepImportError> {
        let args = self.simple(from, id, "EDGE_CURVE")?;
        let expected = "EDGE_CURVE(name, #start, #end, #curve, .T.) — the exported \
                        subset's carriers run start → end (same_sense .T.)";
        let [_, start_ref, end_ref, curve_ref, same_sense] = args else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        // same_sense .T. only: the writer's carriers always run
        // start → end (the he_plus forward contract). A .F. edge would
        // need the carrier's parameterization reversed — bit-moving,
        // outside the identity subset.
        if !as_bool(id, same_sense, expected)? {
            return Err(StepImportError::Topology { id, what: expected });
        }
        let start = as_ref(id, start_ref, expected)?;
        let end = as_ref(id, end_ref, expected)?;
        for v in [start, end] {
            if !vertices.contains_key(&v) {
                let p = self.vertex(id, v)?;
                vertices.insert(v, p);
            }
        }
        let carrier = self.curve(id, as_ref(id, curve_ref, expected)?)?;
        let p_start = vertices[&start];
        let p_end = vertices[&end];
        let (t0, t1) =
            geometry::endpoint_params(id, &carrier, p_start, p_end, start == end)?;
        Ok(EdgeSpec {
            start,
            end,
            carrier,
            t0,
            t1,
        })
    }

    /// One `MANIFOLD_SOLID_BREP` and its `CLOSED_SHELL`.
    fn solid(&self, id: u64, args: &[Value]) -> Result<SolidSpec, StepImportError> {
        let expected = "MANIFOLD_SOLID_BREP(name, #closed_shell)";
        let [_, shell_ref] = args else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        let shell_id = as_ref(id, shell_ref, expected)?;
        let sargs = self.simple(id, shell_id, "CLOSED_SHELL")?;
        let sexpected = "CLOSED_SHELL(name, (faces))";
        let [_, face_list] = sargs else {
            return Err(StepImportError::MalformedRecord {
                id: shell_id,
                expected: sexpected,
            });
        };
        let mut edges = BTreeMap::new();
        let mut vertices = BTreeMap::new();
        let mut faces = Vec::new();
        for face_ref in as_list(shell_id, face_list, sexpected)? {
            let face_id = as_ref(shell_id, face_ref, sexpected)?;
            faces.push(self.face(shell_id, face_id, &mut edges, &mut vertices)?);
        }
        if faces.is_empty() {
            return Err(StepImportError::Topology {
                id: shell_id,
                what: "a CLOSED_SHELL with no faces",
            });
        }
        // Manifold precondition: every edge is used exactly twice,
        // once forward and once reversed (across all loops of the
        // shell). Anything else cannot assemble as a closed oriented
        // 2-manifold and refuses before any surgery starts.
        let mut counts: BTreeMap<u64, (usize, usize)> = BTreeMap::new();
        for face in &faces {
            for lp in &face.loops {
                for use_ in &lp.uses {
                    let entry = counts.entry(use_.edge).or_insert((0, 0));
                    if use_.forward {
                        entry.0 += 1;
                    } else {
                        entry.1 += 1;
                    }
                }
            }
        }
        for (edge_id, (fwd, rev)) in counts {
            if (fwd, rev) != (1, 1) {
                return Err(StepImportError::Topology {
                    id: edge_id,
                    what: "an EDGE_CURVE must be traversed exactly twice per shell, \
                           once each way (closed oriented 2-manifold); this one is not",
                });
            }
        }
        Ok(SolidSpec {
            id,
            faces,
            edges,
            vertices,
        })
    }

    /// A `GEOMETRIC_CURVE_SET`'s curves, in set order.
    fn curve_set(&self, id: u64, args: &[Value]) -> Result<Vec<Curve3<f64>>, StepImportError> {
        let expected = "GEOMETRIC_CURVE_SET(name, (elements))";
        let [_, elements] = args else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        let mut curves = Vec::new();
        for element in as_list(id, elements, expected)? {
            curves.push(self.curve(id, as_ref(id, element, expected)?)?);
        }
        Ok(curves)
    }
}

/// Checks one unit component of the `GLOBAL_UNIT_ASSIGNED_CONTEXT`
/// against the subset: unprefixed `SI_UNIT($, .METRE. | .RADIAN. |
/// .STERADIAN.)`. Parsed, not assumed (M7-1 spec Leg B) — anything
/// else (a prefix, a conversion-based unit) refuses typed.
fn check_unit(id: u64, records: &[Record]) -> Result<(), StepImportError> {
    for (kw, args) in records {
        if kw == "SI_UNIT" {
            let [prefix, name] = args.as_slice() else {
                return Err(StepImportError::UnsupportedUnit {
                    id,
                    found: complex_name(records),
                });
            };
            let ok = *prefix == Value::Null
                && matches!(name, Value::Enum(n) if n == "METRE" || n == "RADIAN" || n == "STERADIAN");
            if !ok {
                return Err(StepImportError::UnsupportedUnit {
                    id,
                    found: complex_name(records),
                });
            }
            return Ok(());
        }
    }
    Err(StepImportError::UnsupportedUnit {
        id,
        found: complex_name(records),
    })
}

/// Resolves the parsed file into a [`Model`] (module docs).
pub(crate) fn resolve(file: &StepFile) -> Result<Model, StepImportError> {
    let r = Resolver { file };

    // The header must declare a schema (Part 21's FILE_SCHEMA). The
    // subset does not pin the schema NAME — the entities themselves
    // are checked record by record — but a headerless file is not an
    // exchange file.
    if !file.header.iter().any(|(kw, _)| kw == "FILE_SCHEMA") {
        return Err(StepImportError::Syntax {
            line: 1,
            expected: "a FILE_SCHEMA record in the HEADER section",
        });
    }

    // Units and uncertainty: every SI_UNIT record in the data section
    // must be inside the subset, and the representation context must
    // declare a length uncertainty (ε_in's file default). Scanned in
    // id order (deterministic).
    let mut uncertainty = None;
    for (&id, instance) in &file.data {
        if instance.records.iter().any(|(kw, _)| kw == "SI_UNIT") {
            check_unit(id, &instance.records)?;
        }
        if let [(kw, args)] = instance.records.as_slice() {
            if kw == "UNCERTAINTY_MEASURE_WITH_UNIT" {
                let expected = "UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(value), \
                                #unit, name, description)";
                let [Value::Typed(measure, inner), ..] = args.as_slice() else {
                    return Err(StepImportError::MalformedRecord { id, expected });
                };
                if measure != "LENGTH_MEASURE" {
                    return Err(StepImportError::MalformedRecord { id, expected });
                }
                let [value] = inner.as_slice() else {
                    return Err(StepImportError::MalformedRecord { id, expected });
                };
                uncertainty = Some(as_real(id, value, expected)?);
            }
        }
    }
    let Some(uncertainty_m) = uncertainty else {
        return Err(StepImportError::MissingUncertainty);
    };

    // Shape content: MANIFOLD_SOLID_BREPs (id order — the writer's
    // emission order), else a GEOMETRIC_CURVE_SET wireframe.
    let mut solids = Vec::new();
    let mut curve_set = None;
    for (&id, instance) in &file.data {
        if let [(kw, args)] = instance.records.as_slice() {
            match kw.as_str() {
                "MANIFOLD_SOLID_BREP" => solids.push(r.solid(id, args)?),
                "GEOMETRIC_CURVE_SET" if curve_set.is_none() => {
                    curve_set = Some(r.curve_set(id, args)?);
                }
                _ => {}
            }
        }
    }
    let shape = if solids.is_empty() {
        match curve_set {
            Some(curves) => Shape::Wireframe(curves),
            None => return Err(StepImportError::NothingToImport),
        }
    } else {
        Shape::Solids(solids)
    };
    Ok(Model {
        uncertainty_m,
        shape,
    })
}
