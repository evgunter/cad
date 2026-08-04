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
use crate::chart;
use crate::normalize;
use crate::geometry;
use crate::parse::{Instance, Record, StepFile, Value};
use crate::units::{self, UnitKind};
use crate::{FaceCensus, NormalizationKind, StructureNormalization};

/// A resolved `AXIS2_PLACEMENT_3D`: `(origin, axis, u_ref)` — the
/// kernel frame, field for field.
type Frame = (Point3<f64>, Vec3<f64>, Vec3<f64>);

/// A resolved model: the shape content plus the file's declared
/// uncertainty (D7's ε_in default).
#[derive(Debug)]
pub(crate) struct Model {
    /// The declared `UNCERTAINTY_MEASURE_WITH_UNIT` value, scaled into
    /// kernel meters with every other length.
    pub(crate) uncertainty_m: f64,
    /// What the data section carries.
    pub(crate) shape: Shape,
    /// The structure normalizations minted during resolution, as data.
    pub(crate) normalizations: Vec<StructureNormalization>,
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
    /// The `ADVANCED_FACE` entity instance this face came from (a
    /// normalization's minted faces keep the id of the face they
    /// replace, so a reported census mapping names a real record).
    pub(crate) id: u64,
    /// The mapped kernel surface, field for field.
    pub(crate) surface: Surface<f64>,
    /// `same_sense`, honored verbatim as [`topo::Face::sense`].
    pub(crate) sense: bool,
    /// The bounds: outer first (`FACE_OUTER_BOUND`), rings after in
    /// stored order.
    pub(crate) loops: Vec<LoopSpec>,
}

/// One `FACE_BOUND` / `FACE_OUTER_BOUND` as read: what the file said
/// about outerness (before the inference decides), the loop's uses
/// with the bound's own orientation already composed in, and — for a
/// `VERTEX_LOOP` — the single vertex that bounds a closed face.
#[derive(Debug)]
pub(crate) struct BoundSpec {
    /// True iff the record was a `FACE_OUTER_BOUND`.
    pub(crate) stated_outer: bool,
    /// The oriented edge uses, in traversal order (empty for a vertex
    /// loop).
    pub(crate) uses: Vec<EdgeUse>,
    /// The `VERTEX_POINT` id, for a `VERTEX_LOOP` bound.
    pub(crate) vertex_loop: Option<u64>,
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
    /// The file's SI length factor into kernel meters (M7-2 Leg A):
    /// 1.0 for the kernel's own dialect, 1e-3 for FreeCAD's
    /// millimeters. EVERY length the resolver reads passes through
    /// [`Resolver::as_length`].
    length_scale: f64,
    /// ε_in in kernel meters — the file's scaled declared uncertainty,
    /// the floor of every interpretation budget the resolver spends
    /// ([`crate::tolerance`]).
    eps_in: f64,
    /// The next unused entity id, for the structure normalizations that
    /// mint topology the file does not carry (Leg C's edge-free
    /// sphere). Seeded past every id in the file, so a minted vertex or
    /// edge can never collide with a stated one.
    next_id: std::cell::Cell<u64>,
    /// The normalizations minted so far, carried out as data
    /// ([`crate::StructureNormalization`]) — never silent.
    normalizations: std::cell::RefCell<Vec<StructureNormalization>>,
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
/// writer's printer), with **negative zero normalized to `+0.0`**.
///
/// `-0.` is common in the foreign dialect (`DIRECTION('',(1.,0.,-0.))`
/// on nearly every FreeCAD placement). It denotes the same real number
/// as `0.`, but it is a different f64 bit pattern, and the importer
/// compares surface records **bitwise** to restore writer-side key
/// sharing — so two records identical as geometry would fail to share
/// a key purely on a printed sign. Normalizing here moves no value
/// (`-0.0 == 0.0`), states one representative, and is the only
/// normalization the numeric path performs.
fn as_real(id: u64, value: &Value, expected: &'static str) -> Result<f64, StepImportError> {
    match value {
        Value::Number(raw) => raw
            .parse::<f64>()
            .map(|v| if v == 0.0 { 0.0 } else { v })
            .map_err(|_| StepImportError::MalformedReal {
                id,
                token: raw.clone(),
            }),
        _ => Err(StepImportError::MalformedRecord { id, expected }),
    }
}

/// `value` as a nonnegative integer (degrees, multiplicities).
fn as_usize(id: u64, value: &Value, expected: &'static str) -> Result<usize, StepImportError> {
    match value {
        Value::Number(raw) => raw
            .parse()
            .map_err(|_| StepImportError::MalformedRecord { id, expected }),
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
    /// `value` as a **length**, scaled into kernel meters (module
    /// docs' Leg A rule: every length the file states passes here).
    fn as_length(
        &self,
        id: u64,
        value: &Value,
        expected: &'static str,
    ) -> Result<f64, StepImportError> {
        let raw = as_real(id, value, expected)?;
        // Multiplication by an exact power of ten is one correctly
        // rounded operation; the unprefixed case multiplies by 1.0 and
        // moves no bits at all (the own-corpus identity is untouched).
        Ok(raw * self.length_scale)
    }

    /// A fresh entity id for minted topology (module docs).
    fn mint_id(&self) -> u64 {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        id
    }

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
            self.as_length(id, x, expected)?,
            self.as_length(id, y, expected)?,
            self.as_length(id, z, expected)?,
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
    fn placement(&self, from: u64, id: u64) -> Result<Frame, StepImportError> {
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
                let (origin, axis, u_ref) = self.placement(id, as_ref(id, placement, expected)?)?;
                Ok(Surface::Cylinder {
                    origin,
                    axis,
                    radius: self.as_length(id, radius, expected)?,
                    u_ref,
                })
            }
            "CONICAL_SURFACE" => {
                // Two placement conventions, one kernel surface.
                //
                // The kernel's own writer places the cone AT its apex
                // (`radius = 0.0`), where the placement location IS the
                // kernel apex and the mapping is an identity. Open
                // CASCADE - every FreeCAD file - places it at the BASE
                // circle instead, `radius` being that circle's radius;
                // an apex-form cone never appears there. STEP's
                // parameterization is `S(u, v) = location + axis*v +
                // radial(u)*(radius + v*tan a)`, so the apex is its
                // `v = -radius/tan a` point:
                //
                //     apex = location - axis*(radius / tan a)
                //
                // - a DERIVATION, not an identity, and stated as one:
                // a computation on the file's own numbers (one divide,
                // one multiply-add), which the vertices the file states
                // on the same surface then pin through the kernel's own
                // certification gates. A cone the file already places
                // at its apex still takes the identity path bit for
                // bit: `radius == 0.0` short-circuits the arithmetic.
                //
                // STEP's axial v vs the kernel's slant-arc-length v
                // (the fixed cos a factor) still has nothing to act on:
                // no trim parameters or pcurves cross the wire, and the
                // surface LOCUS fields are what adoption consumes.
                let expected = "CONICAL_SURFACE(name, #placement, radius >= 0, \
                     semi_angle in (0, pi/2))";
                let [_, placement, radius, semi_angle] = args.as_slice() else {
                    return Err(StepImportError::MalformedRecord { id, expected });
                };
                let radius = self.as_length(id, radius, expected)?;
                let half_angle = as_real(id, semi_angle, expected)?;
                // The kernel's stated convention: the half-angle lies
                // strictly inside (0, pi/2) - 0 degenerates to a line,
                // pi/2 to a plane, and neither is a cone. Checked here
                // rather than trusted, since the derivation below
                // divides by its tangent.
                if !(radius.is_finite()
                    && radius >= 0.0
                    && half_angle > 0.0
                    && half_angle < std::f64::consts::FRAC_PI_2)
                {
                    return Err(StepImportError::MalformedRecord { id, expected });
                }
                let (location, axis, u_ref) = self.placement(id, as_ref(id, placement, expected)?)?;
                let apex = if radius == 0.0 {
                    location
                } else {
                    location - axis * (radius / half_angle.tan())
                };
                Ok(Surface::Cone {
                    apex,
                    axis,
                    half_angle,
                    u_ref,
                })
            }
            "SPHERICAL_SURFACE" => {
                let expected = "SPHERICAL_SURFACE(name, #placement, radius)";
                let [_, placement, radius] = args.as_slice() else {
                    return Err(StepImportError::MalformedRecord { id, expected });
                };
                let (center, axis, u_ref) = self.placement(id, as_ref(id, placement, expected)?)?;
                Ok(Surface::Sphere {
                    center,
                    radius: self.as_length(id, radius, expected)?,
                    axis,
                    u_ref,
                })
            }
            "TOROIDAL_SURFACE" => {
                let expected = "TOROIDAL_SURFACE(name, #placement, major, minor)";
                let [_, placement, major, minor] = args.as_slice() else {
                    return Err(StepImportError::MalformedRecord { id, expected });
                };
                let (center, axis, u_ref) = self.placement(id, as_ref(id, placement, expected)?)?;
                Ok(Surface::Torus {
                    center,
                    axis,
                    major_radius: self.as_length(id, major, expected)?,
                    minor_radius: self.as_length(id, minor, expected)?,
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
                let (center, axis, u_ref) = self.placement(id, as_ref(id, placement, expected)?)?;
                Ok(Curve3::Circle {
                    center,
                    axis,
                    radius: self.as_length(id, radius, expected)?,
                    u_ref,
                })
            }
            "ELLIPSE" => {
                let expected = "ELLIPSE(name, #placement, semi_axis_1, semi_axis_2)";
                let [_, placement, major, minor] = args.as_slice() else {
                    return Err(StepImportError::MalformedRecord { id, expected });
                };
                let (center, axis, u_ref) = self.placement(id, as_ref(id, placement, expected)?)?;
                Ok(Curve3::Ellipse {
                    center,
                    axis,
                    major: self.as_length(id, major, expected)?,
                    minor: self.as_length(id, minor, expected)?,
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
                "BOUNDED_CURVE"
                | "CURVE"
                | "GEOMETRIC_REPRESENTATION_ITEM"
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
    ///
    /// Answers a **list** of faces: normally one, but a closed face
    /// with no edges (a whole sphere under a `VERTEX_LOOP`) is not
    /// representable as one kernel face and adopts as the kernel's own
    /// canonical splitting, reported as data
    /// ([`crate::StructureNormalization`]).
    ///
    /// Outerness (M7-2 Leg B): `FACE_OUTER_BOUND` is honored where the
    /// file states it and cross-checked against the geometric
    /// inference; where the file states only `FACE_BOUND`s — every
    /// FreeCAD face — outerness is inferred ([`crate::chart`]) with a
    /// single bound outer by definition.
    fn face(
        &self,
        from: u64,
        id: u64,
        edges: &mut BTreeMap<u64, EdgeSpec>,
        vertices: &mut BTreeMap<u64, Point3<f64>>,
    ) -> Result<Vec<FaceSpec>, StepImportError> {
        let args = self.simple(from, id, "ADVANCED_FACE")?;
        let expected = "ADVANCED_FACE(name, (bounds), #surface, same_sense)";
        let [_, bounds, surface_ref, same_sense] = args else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        let surface_id = as_ref(id, surface_ref, expected)?;
        let surface = self.surface(id, surface_id)?;
        let sense = as_bool(id, same_sense, expected)?;
        let mut bound_specs = Vec::new();
        for bound in as_list(id, bounds, expected)? {
            let bound_id = as_ref(id, bound, expected)?;
            bound_specs.push(self.bound(id, bound_id, edges, vertices)?);
        }
        if bound_specs.is_empty() {
            return Err(StepImportError::Topology {
                id,
                what: "an ADVANCED_FACE with no bounds",
            });
        }
        // The edge-free closed face: one VERTEX_LOOP and nothing else.
        if bound_specs.iter().any(|b| b.vertex_loop.is_some()) {
            return self.edge_free_face(id, surface, surface_id, sense, &bound_specs, edges, vertices);
        }

        let mut loops: Vec<LoopSpec> = bound_specs
            .iter()
            .map(|b| LoopSpec {
                outer: false,
                uses: b.uses.clone(),
            })
            .collect();
        let outer = self.outer_bound_index(id, &surface, &loops, &bound_specs, edges)?;
        loops[outer].outer = true;
        // The outer bound leads (assembly reads `loops[0]` as the
        // face's outer cycle and the rest as its rings, in order).
        loops.swap(0, outer);
        Ok(vec![FaceSpec {
            id,
            surface,
            sense,
            loops,
        }])
    }

    /// Which bound is outer (module docs' Leg B rule; the inference
    /// itself is [`crate::chart::infer_outer`]).
    fn outer_bound_index(
        &self,
        id: u64,
        surface: &Surface<f64>,
        loops: &[LoopSpec],
        bounds: &[BoundSpec],
        edges: &BTreeMap<u64, EdgeSpec>,
    ) -> Result<usize, StepImportError> {
        let stated: Vec<usize> = bounds
            .iter()
            .enumerate()
            .filter_map(|(i, b)| b.stated_outer.then_some(i))
            .collect();
        if stated.len() > 1 {
            return Err(StepImportError::Topology {
                id,
                what: "an ADVANCED_FACE with more than one FACE_OUTER_BOUND — a face \
                       has one outer bound",
            });
        }
        // A single bound is outer BY DEFINITION: there is nothing for
        // it to be inside of. Stated, because it is a definition and
        // not a measurement.
        if loops.len() == 1 {
            return Ok(0);
        }
        let rings: Vec<Vec<Point3<f64>>> = loops
            .iter()
            .map(|lp| self.ring_samples(lp, edges))
            .collect::<Result<_, _>>()?;
        let inferred = chart::infer_outer(surface, &rings, self.eps_in);
        match (stated.first().copied(), inferred) {
            // The kernel's own dialect: honored AND cross-checked. A
            // disagreement between what the file says and what its own
            // geometry says is a typed error, not a preference.
            (Some(s), Ok(i)) if s == i => Ok(s),
            (Some(_), Ok(_)) => Err(StepImportError::Topology {
                id,
                what: "an ADVANCED_FACE whose stated FACE_OUTER_BOUND is not the bound \
                       its own geometry makes outer — the file contradicts itself",
            }),
            // A stated outer bound stands even where the inference
            // cannot answer: the file said it, and refusing there would
            // narrow the subset the kernel's own writer emits.
            (Some(s), Err(_)) => Ok(s),
            (None, Ok(i)) => Ok(i),
            (None, Err(refusal)) => Err(StepImportError::Topology {
                id,
                what: refusal.what(),
            }),
        }
    }

    /// A bound's 3-D sample polygon, in traversal order: each edge's
    /// carrier sampled over its own interval, the far endpoint left to
    /// the next edge (the ring closes on itself).
    fn ring_samples(
        &self,
        lp: &LoopSpec,
        edges: &BTreeMap<u64, EdgeSpec>,
    ) -> Result<Vec<Point3<f64>>, StepImportError> {
        /// Samples per edge. Enough that a full-circle edge's azimuth
        /// steps stay well inside the chart unwrapper's half-period
        /// window (τ/16 there), so a wrap is detected as a wrap.
        const PER_EDGE: usize = 16;
        let mut out = Vec::with_capacity(lp.uses.len() * PER_EDGE);
        for use_ in &lp.uses {
            let spec = edges.get(&use_.edge).ok_or(StepImportError::Topology {
                id: use_.edge,
                what: "internal: an edge use without a resolved edge",
            })?;
            for k in 0..PER_EDGE {
                #[allow(clippy::cast_precision_loss)]
                let f = k as f64 / PER_EDGE as f64;
                // Traversal order: the reversed use walks the interval
                // backwards, so the polygon is the loop's own cycle.
                let f = if use_.forward { f } else { 1.0 - f };
                out.push(spec.carrier.eval(spec.t0 + (spec.t1 - spec.t0) * f));
            }
        }
        Ok(out)
    }


    /// The sum of every numeric literal's print-precision term in one
    /// instance's own records, as a LENGTH budget in kernel meters
    /// (the parser keeps each token's printed text — [`crate::tolerance`]).
    /// Non-length literals in the same record can only widen the budget
    /// conservatively, which is the safe direction for a gate.
    fn instance_length_eps(&self, id: u64) -> f64 {
        let Some(instance) = self.file.data.get(&id) else {
            return self.eps_in;
        };
        fn walk(value: &Value, acc: &mut f64, scale: f64) {
            match value {
                Value::Number(raw) => {
                    *acc = acc.max(crate::tolerance::print_half_ulp(raw) * scale);
                }
                Value::List(items) | Value::Typed(_, items) => {
                    for v in items {
                        walk(v, acc, scale);
                    }
                }
                _ => {}
            }
        }
        let mut acc = self.eps_in;
        for (_, args) in &instance.records {
            for v in args {
                walk(v, &mut acc, self.length_scale);
            }
        }
        acc
    }

    /// **The edge-free closed face** (M7-2 Leg C): a whole sphere
    /// arrives as one `ADVANCED_FACE` whose only bound is a
    /// `VERTEX_LOOP` — Open CASCADE drops the seam and both degenerate
    /// pole edges on export, so the file states 1 face / 0 edges /
    /// 1 vertex.
    ///
    /// The kernel's half-edge structure has no such state: a face's
    /// boundary is a cycle of half-edges, and a closed face with none
    /// is not a body the Euler operators can reach. What IS fully
    /// determined is the **locus** — the sphere record explains every
    /// point of it — so the adoption is D7 stage-3 repair in its
    /// letter: the locus is adopted whole and only the boundary-graph
    /// tessellation is re-minted, as the kernel's own canonical
    /// splitting of a ball (the census a natively revolved sphere
    /// carries: 2 faces / 2 edges / 2 vertices, two half-lune faces
    /// meeting along two pole-to-pole meridians). The mapping is
    /// carried out on the import record as data
    /// ([`crate::StructureNormalization`]) — a reported normalization,
    /// never a silent one. Volume and validity are exact as always.
    ///
    /// The vertex loop's own point must lie ON the sphere within the
    /// interpretation budget; it is the file's one statement about this
    /// face's boundary, and a point elsewhere would mean the file is
    /// describing something the sphere record does not.
    fn edge_free_face(
        &self,
        id: u64,
        surface: Surface<f64>,
        surface_id: u64,
        sense: bool,
        bounds: &[BoundSpec],
        edges: &mut BTreeMap<u64, EdgeSpec>,
        vertices: &mut BTreeMap<u64, Point3<f64>>,
    ) -> Result<Vec<FaceSpec>, StepImportError> {
        let [bound] = bounds else {
            return Err(StepImportError::Topology {
                id,
                what: "an ADVANCED_FACE mixing a VERTEX_LOOP with other bounds — a \
                       vertex loop bounds a CLOSED face, which has no room for rings",
            });
        };
        let Some(vertex_id) = bound.vertex_loop else {
            return Err(StepImportError::Topology {
                id,
                what: "internal: an edge-free face without its vertex loop",
            });
        };
        let Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } = surface
        else {
            return Err(StepImportError::Topology {
                id,
                what: "an ADVANCED_FACE bounded by a VERTEX_LOOP on a surface that is \
                       not a sphere — the sphere is the one closed locus this kernel \
                       can re-tessellate from a single point, so anything else \
                       refuses rather than guessing a splitting",
            });
        };
        // The budget: ε_in, widened only by what the sphere record's
        // and the vertex point's own printed text can support.
        let eps = self
            .instance_length_eps(surface_id)
            .max(self.instance_length_eps(vertex_id))
            .max(self.eps_in);
        let p = self.vertex(id, vertex_id)?;
        if ((p - center).norm() - radius).abs() > eps {
            return Err(StepImportError::Topology {
                id: vertex_id,
                what: "a VERTEX_LOOP point that does not lie on its face's sphere \
                       within the file's own interpretation budget — the file's one \
                       statement about this closed face's boundary contradicts its \
                       surface",
            });
        }

        // The canonical splitting, in the kernel's own sphere chart:
        // the two poles (v = ±π/2) joined by the two half-meridians
        // through u = 0 and u = π. `v_ref` is the chart's second
        // in-plane basis vector (`azimuth_frame`'s), so a circle with
        // axis ∓v_ref and reference direction the sphere's own axis
        // starts AT the north pole (angle 0) and reaches the south at
        // angle π, sweeping through u = π and u = 0 respectively.
        let v_ref = axis.cross(u_ref);
        let north = center + axis * radius;
        let south = center - axis * radius;
        let (nv, sv) = (self.mint_id(), self.mint_id());
        vertices.insert(nv, north);
        vertices.insert(sv, south);
        let mut meridian = |circle_axis: Vec3<f64>| -> Result<u64, StepImportError> {
            let eid = self.mint_id();
            let carrier = Curve3::Circle {
                center,
                axis: circle_axis,
                radius,
                u_ref: axis,
            };
            let (t0, t1) = geometry::endpoint_params(eid, &carrier, north, south, false)?;
            edges.insert(
                eid,
                EdgeSpec {
                    start: nv,
                    end: sv,
                    carrier,
                    t0,
                    t1,
                },
            );
            Ok(eid)
        };
        let through_pi = meridian(-v_ref)?;
        let through_seam = meridian(v_ref)?;

        let lune = |a: u64, a_fwd: bool, b: u64, b_fwd: bool| FaceSpec {
            id,
            surface: surface.clone(),
            sense,
            loops: vec![LoopSpec {
                outer: true,
                uses: vec![
                    EdgeUse {
                        edge: a,
                        forward: a_fwd,
                    },
                    EdgeUse {
                        edge: b,
                        forward: b_fwd,
                    },
                ],
            }],
        };
        // Each lune walks one meridian down and the other back up. The
        // face's `same_sense` is honored, not healed: a reversed face
        // has its material on the other side, so its cycles run the
        // other way round.
        let faces = if sense {
            vec![
                lune(through_pi, true, through_seam, false),
                lune(through_pi, false, through_seam, true),
            ]
        } else {
            vec![
                lune(through_seam, true, through_pi, false),
                lune(through_seam, false, through_pi, true),
            ]
        };
        self.normalizations
            .borrow_mut()
            .push(StructureNormalization {
                face: id,
                kind: NormalizationKind::EdgeFreeSphere,
                file_census: FaceCensus {
                    faces: 1,
                    edges: 0,
                    vertices: 1,
                },
                kernel_census: FaceCensus {
                    faces: 2,
                    edges: 2,
                    vertices: 2,
                },
            });
        Ok(faces)
    }

    /// One `FACE_OUTER_BOUND` / `FACE_BOUND` and its loop.
    fn bound(
        &self,
        from: u64,
        id: u64,
        edges: &mut BTreeMap<u64, EdgeSpec>,
        vertices: &mut BTreeMap<u64, Point3<f64>>,
    ) -> Result<BoundSpec, StepImportError> {
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
        let expected = "FACE_BOUND(name, #loop, orientation)";
        let [_, loop_ref, orientation] = args.as_slice() else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        // Orientation, HONORED (M7-2 Leg B): `.F.` means the bound
        // traverses its loop against the loop's own stated cycle, so
        // the loop is reversed — the sequence turned around AND every
        // oriented edge's direction flipped, which is exactly the
        // reversal of the cycle as a closed walk. It is composed into
        // the realized half-edge directions here, once, rather than
        // carried as a second flag the whole pipeline must remember.
        //
        // It is NOT redundant with the owning face's `same_sense`: the
        // measured corpus has 4 planar caps with face `.F.` and bound
        // `.T.` (cylinder/cone caps and a fillet-corner face), so the
        // two flags carry independent meaning and healing them into a
        // pair would silently reverse those four faces' material side.
        let forward_bound = as_bool(id, orientation, expected)?;
        let loop_id = as_ref(id, loop_ref, expected)?;
        // A VERTEX_LOOP bounds a closed face by a single point (the
        // whole-sphere case); it carries no edges at all.
        if let [(kw, vargs)] = self.instance(id, loop_id)?.records.as_slice()
            && kw == "VERTEX_LOOP"
        {
            let vexpected = "VERTEX_LOOP(name, #vertex)";
            let [_, vertex_ref] = vargs.as_slice() else {
                return Err(StepImportError::MalformedRecord {
                    id: loop_id,
                    expected: vexpected,
                });
            };
            let vid = as_ref(loop_id, vertex_ref, vexpected)?;
            // Read (so a malformed vertex refuses right here) but NOT
            // recorded: no edge uses it, and the shell tables hold only
            // vertices the assembled complex reaches.
            self.vertex(loop_id, vid)?;
            return Ok(BoundSpec {
                stated_outer: outer,
                uses: Vec::new(),
                vertex_loop: Some(vid),
            });
        }
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
            if let std::collections::btree_map::Entry::Vacant(slot) = edges.entry(edge_id) {
                let spec = self.edge(oe_id, edge_id, vertices)?;
                slot.insert(spec);
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
        if !forward_bound {
            uses.reverse();
            for u in &mut uses {
                u.forward = !u.forward;
            }
        }
        Ok(BoundSpec {
            stated_outer: outer,
            uses,
            vertex_loop: None,
        })
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
            if let std::collections::btree_map::Entry::Vacant(slot) = vertices.entry(v) {
                let p = self.vertex(id, v)?;
                slot.insert(p);
            }
        }
        let carrier = self.curve(id, as_ref(id, curve_ref, expected)?)?;
        let p_start = vertices[&start];
        let p_end = vertices[&end];
        let (t0, t1) = geometry::endpoint_params(id, &carrier, p_start, p_end, start == end)?;
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
            faces.extend(self.face(shell_id, face_id, &mut edges, &mut vertices)?);
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
        let mut solid = SolidSpec {
            id,
            faces,
            edges,
            vertices,
        };
        // The reported structure normalizations for periodic faces the
        // kernel cannot represent as stated (Leg C).
        normalize::normalize_shell(
            &mut solid,
            &mut || self.mint_id(),
            &mut self.normalizations.borrow_mut(),
        )?;
        Ok(solid)
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

/// Reads one unit component of the `GLOBAL_UNIT_ASSIGNED_CONTEXT` into
/// its [`UnitKind`]: an `SI_UNIT(prefix, name)` whose name is
/// `.METRE.` / `.RADIAN.` / `.STERADIAN.`, with a resolved
/// [`units`] prefix factor on lengths (M7-2 Leg A — `.MILLI.` is what
/// FreeCAD writes, and the prefix is table data, not a special case).
/// Parsed, not assumed (M7-1 spec Leg B) — anything else (a
/// conversion-based unit, a prefixed angle) refuses typed.
fn check_unit(id: u64, records: &[Record]) -> Result<UnitKind, StepImportError> {
    for (kw, args) in records {
        if kw == "SI_UNIT" {
            let found = || complex_name(records);
            let [prefix, name] = args.as_slice() else {
                return Err(StepImportError::UnsupportedUnit { id, found: found() });
            };
            let prefix = match prefix {
                Value::Null => None,
                Value::Enum(p) => Some(p.as_str()),
                _ => return Err(StepImportError::UnsupportedUnit { id, found: found() }),
            };
            let Value::Enum(name) = name else {
                return Err(StepImportError::UnsupportedUnit { id, found: found() });
            };
            return units::si_unit_kind(id, prefix, name, found);
        }
    }
    Err(StepImportError::UnsupportedUnit {
        id,
        found: complex_name(records),
    })
}

/// The SI **length** factor of `records` (a `LENGTH_UNIT` composed with
/// an `SI_UNIT` metre), or `None` when the record is not a length.
fn si_length_factor(id: u64, records: &[Record]) -> Result<Option<f64>, StepImportError> {
    if !records.iter().any(|(kw, _)| kw == "LENGTH_UNIT") {
        return Ok(None);
    }
    match check_unit(id, records)? {
        UnitKind::Length(factor) => Ok(Some(factor)),
        // A LENGTH_UNIT composed with a radian is a malformed context,
        // not a unit the subset merely lacks.
        _ => Err(StepImportError::UnsupportedUnit {
            id,
            found: complex_name(records),
        }),
    }
}

/// Units and uncertainty **by resolution** (M7-1 review MAJOR-1): the
/// old presence-scan only fired on instances *containing* an
/// `SI_UNIT` record, so a `CONVERSION_BASED_UNIT` length context (an
/// inch/mm file's normal form) imported silently as metres. Now every
/// `GEOMETRIC_REPRESENTATION_CONTEXT`'s `GLOBAL_UNIT_ASSIGNED_CONTEXT`
/// references are resolved and each must be a subset-form SI unit
/// ([`check_unit`]), a subset SI **length** unit must exist among
/// them, and every declared uncertainty must be a `LENGTH_MEASURE`
/// over a resolved subset length unit. The presence-scan stays as a
/// belt (an unreferenced prefixed unit still refuses).
fn resolve_units_and_uncertainty(
    r: &Resolver<'_>,
    file: &StepFile,
) -> Result<(f64, f64), StepImportError> {
    for (&id, instance) in &file.data {
        if instance.records.iter().any(|(kw, _)| kw == "SI_UNIT") {
            check_unit(id, &instance.records)?;
        }
    }
    // The file's ONE length scale (M7-2 Leg A). Two contexts declaring
    // different length units would leave every coordinate's meaning
    // ambiguous, so a second, distinct factor refuses typed rather
    // than letting the first one win silently.
    let mut length_scale: Option<(u64, f64)> = None;
    let mut note_scale = |id: u64, factor: f64| -> Result<(), StepImportError> {
        match length_scale {
            None => length_scale = Some((id, factor)),
            Some((_, prev)) if prev.to_bits() == factor.to_bits() => {}
            Some(_) => {
                return Err(StepImportError::UnsupportedUnit {
                    id,
                    found: "a second, different SI length unit (one file, one length \
                            scale — two make every coordinate ambiguous)"
                        .to_owned(),
                });
            }
        }
        Ok(())
    };
    let mut uncertainty: Option<f64> = None;
    for (&id, instance) in &file.data {
        if !instance
            .records
            .iter()
            .any(|(kw, _)| kw == "GEOMETRIC_REPRESENTATION_CONTEXT")
        {
            continue;
        }
        let mut has_length_unit = false;
        for (kw, args) in &instance.records {
            match kw.as_str() {
                "GLOBAL_UNIT_ASSIGNED_CONTEXT" => {
                    let expected = "GLOBAL_UNIT_ASSIGNED_CONTEXT((unit references))";
                    let [units] = args.as_slice() else {
                        return Err(StepImportError::MalformedRecord { id, expected });
                    };
                    for unit in as_list(id, units, expected)? {
                        let uid = as_ref(id, unit, expected)?;
                        let unit_instance = r.instance(id, uid)?;
                        check_unit(uid, &unit_instance.records)?;
                        if let Some(factor) = si_length_factor(uid, &unit_instance.records)? {
                            has_length_unit = true;
                            note_scale(uid, factor)?;
                        }
                    }
                }
                "GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT" => {
                    let expected = "GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((uncertainty references))";
                    let [list] = args.as_slice() else {
                        return Err(StepImportError::MalformedRecord { id, expected });
                    };
                    for entry in as_list(id, list, expected)? {
                        let uid = as_ref(id, entry, expected)?;
                        let (raw, factor, unit_id) = r.uncertainty_value(id, uid)?;
                        note_scale(unit_id, factor)?;
                        // Into kernel meters with every other length
                        // (D7's ε_in rule unchanged; 1.E-07 mm → 1e-10 m).
                        let value = raw * factor;
                        match uncertainty {
                            None => uncertainty = Some(value),
                            Some(prev) if prev.to_bits() == value.to_bits() => {}
                            Some(_) => {
                                // Two distinct declared uncertainties
                                // give ε_in no honest single default —
                                // ambiguity is a typed refusal (D7).
                                return Err(StepImportError::MalformedRecord {
                                    id: uid,
                                    expected: "a single distance_accuracy_value (multiple \
                                               distinct declared uncertainties are ambiguous)",
                                });
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        if !has_length_unit {
            return Err(StepImportError::UnsupportedUnit {
                id,
                found: "a representation context without a subset SI length unit \
                        (a metre, prefixed or not)"
                    .to_owned(),
            });
        }
    }
    let scale = length_scale.map_or(1.0, |(_, f)| f);
    Ok((scale, uncertainty.ok_or(StepImportError::MissingUncertainty)?))
}

impl<'a> Resolver<'a> {
    /// One `UNCERTAINTY_MEASURE_WITH_UNIT`, by resolution: the value
    /// must be a `LENGTH_MEASURE` and its `#unit` must resolve to an SI
    /// length unit. Answers `(raw value, that unit's factor into
    /// meters, the unit's entity id)` — the caller scales, so ε_in
    /// lands in kernel meters like every other length, and a foreign
    /// unit can never silently rescale it.
    fn uncertainty_value(&self, from: u64, id: u64) -> Result<(f64, f64, u64), StepImportError> {
        let args = self.simple(from, id, "UNCERTAINTY_MEASURE_WITH_UNIT")?;
        let expected = "UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(value), #unit, \
                        name, description)";
        let [Value::Typed(measure, inner), unit_ref, ..] = args else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        if measure != "LENGTH_MEASURE" {
            return Err(StepImportError::MalformedRecord { id, expected });
        }
        let [value] = inner.as_slice() else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        let uid = as_ref(id, unit_ref, expected)?;
        let unit_instance = self.instance(id, uid)?;
        let Some(factor) = si_length_factor(uid, &unit_instance.records)? else {
            return Err(StepImportError::UnsupportedUnit {
                id: uid,
                found: complex_name(&unit_instance.records),
            });
        };
        Ok((as_real(id, value, expected)?, factor, uid))
    }
}

/// The **assembly layer** (M7-2 Leg D), accepted at the identity and
/// refused anywhere else.
///
/// FreeCAD's `Import.export` path (what GUI users hit) wraps a
/// multi-body document in AP214's assembly vocabulary:
/// `NEXT_ASSEMBLY_USAGE_OCCURRENCE` links product to component,
/// `CONTEXT_DEPENDENT_SHAPE_REPRESENTATION` links the component's own
/// representation to the assembly root through a complex
/// `( REPRESENTATION_RELATIONSHIP … REPRESENTATION_RELATIONSHIP_WITH_
/// TRANSFORMATION(#t) SHAPE_REPRESENTATION_RELATIONSHIP() )`, and
/// `#t` is an `ITEM_DEFINED_TRANSFORMATION` naming two placements: the
/// component's frame and where it sits in the assembly.
///
/// The product-structure entities are ignorable (the importer already
/// ignores the whole PRODUCT family), but the transform is NOT: a
/// non-identity one means the geometry as stated is NOT where the
/// assembly puts it, and importing the raw geometry would silently
/// place every body wrong. Every measured file's transform is the
/// identity (both placements the same origin/axis/reference frame),
/// because FreeCAD bakes placement into the exported shape. So this
/// pass **traverses and accepts the identity**, and refuses typed —
/// naming the transform entity — on anything else. Full assembly
/// instancing is a later unit; it is not built here on speculation.
fn check_assembly_transforms(r: &Resolver<'_>, file: &StepFile) -> Result<(), StepImportError> {
    for (&id, instance) in &file.data {
        for (kw, args) in &instance.records {
            if kw != "REPRESENTATION_RELATIONSHIP_WITH_TRANSFORMATION" {
                continue;
            }
            let expected = "REPRESENTATION_RELATIONSHIP_WITH_TRANSFORMATION(\
                            #item_defined_transformation)";
            let [transform] = args.as_slice() else {
                return Err(StepImportError::MalformedRecord { id, expected });
            };
            let tid = as_ref(id, transform, expected)?;
            let targs = r.simple(id, tid, "ITEM_DEFINED_TRANSFORMATION")?;
            let texpected = "ITEM_DEFINED_TRANSFORMATION(name, description, \
                             #placement_1, #placement_2)";
            let [_, _, from_ref, to_ref] = targs else {
                return Err(StepImportError::MalformedRecord {
                    id: tid,
                    expected: texpected,
                });
            };
            let a = r.placement(tid, as_ref(tid, from_ref, texpected)?)?;
            let b = r.placement(tid, as_ref(tid, to_ref, texpected)?)?;
            // Identity at the file's own interpretation budget: the
            // origins coincide within ε_in (a length), and the two
            // direction fields coincide within what their printed
            // text supports (dimensionless — a direction ratio is not
            // a length, so ε_in does not apply to it and only the
            // print-precision term does).
            let dir_eps = r
                .instance_length_eps(as_ref(tid, from_ref, texpected)?)
                .max(r.instance_length_eps(as_ref(tid, to_ref, texpected)?));
            let identity = (a.0 - b.0).norm() <= r.eps_in
                && (a.1 - b.1).norm() <= dir_eps
                && (a.2 - b.2).norm() <= dir_eps;
            if !identity {
                return Err(StepImportError::Structure {
                    id: tid,
                    what: "a non-identity assembly transform — the component's geometry \
                           as stated is not where the assembly places it, and importing \
                           it unplaced would put the body in the wrong location; \
                           assembly instancing is a later unit, refused here rather \
                           than guessed",
                });
            }
        }
    }
    Ok(())
}

/// Shape content **by resolution** (M7-1 review MINOR-4): solids come
/// from `ADVANCED_BREP_SHAPE_REPRESENTATION` items and the wireframe
/// from `GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION` items —
/// never from a bare keyword scan — and every `MANIFOLD_SOLID_BREP` /
/// `GEOMETRIC_CURVE_SET` in the data section must be referenced by
/// one of them (an orphan is refused rather than guessed to be model
/// content or silently dropped). Mixed solid+wireframe content and a
/// second curve set are outside the subset, refused typed.
fn resolve_shape(r: &Resolver<'_>, file: &StepFile) -> Result<Shape, StepImportError> {
    let mut solids = Vec::new();
    let mut wireframe: Option<(u64, Vec<Curve3<f64>>)> = None;
    let mut referenced: std::collections::BTreeSet<u64> = std::collections::BTreeSet::new();
    for (&id, instance) in &file.data {
        let [(kw, args)] = instance.records.as_slice() else {
            continue;
        };
        match kw.as_str() {
            // Solid roots. `ADVANCED_BREP_SHAPE_REPRESENTATION` is the
            // AP214 form for a B-rep, and the only one the kernel's own
            // writer emits. Plain `SHAPE_REPRESENTATION` is what Open
            // CASCADE writes for a COMPOUND of solids (and for an
            // assembly's root), holding the MANIFOLD_SOLID_BREPs
            // directly. M7-1 refused those solids as orphans; M7-2
            // narrows that refusal deliberately — it still fires for a
            // genuinely unreferenced solid, which is what it was for.
            "ADVANCED_BREP_SHAPE_REPRESENTATION" | "SHAPE_REPRESENTATION" => {
                let expected = "a shape representation (name, (items), #context)";
                let [_, items, _] = args.as_slice() else {
                    return Err(StepImportError::MalformedRecord { id, expected });
                };
                for item in as_list(id, items, expected)? {
                    let item_id = as_ref(id, item, expected)?;
                    let item_instance = r.instance(id, item_id)?;
                    match item_instance.records.as_slice() {
                        [(k, sargs)] if k == "MANIFOLD_SOLID_BREP" => {
                            referenced.insert(item_id);
                            solids.push(r.solid(item_id, sargs)?);
                        }
                        // The representation's own placement item.
                        [(k, _)] if k == "AXIS2_PLACEMENT_3D" => {}
                        records => {
                            return Err(StepImportError::UnsupportedEntity {
                                id: item_id,
                                keyword: complex_name(records),
                            });
                        }
                    }
                }
            }
            "GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION" => {
                let expected = "GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION(\
                                name, (items), #context)";
                let [_, items, _] = args.as_slice() else {
                    return Err(StepImportError::MalformedRecord { id, expected });
                };
                for item in as_list(id, items, expected)? {
                    let item_id = as_ref(id, item, expected)?;
                    let item_instance = r.instance(id, item_id)?;
                    match item_instance.records.as_slice() {
                        [(k, sargs)] if k == "GEOMETRIC_CURVE_SET" => {
                            if wireframe.is_some() {
                                return Err(StepImportError::Structure {
                                    id: item_id,
                                    what: "a second GEOMETRIC_CURVE_SET — the subset \
                                           carries at most one wireframe",
                                });
                            }
                            referenced.insert(item_id);
                            wireframe = Some((item_id, r.curve_set(item_id, sargs)?));
                        }
                        [(k, _)] if k == "AXIS2_PLACEMENT_3D" => {}
                        records => {
                            return Err(StepImportError::UnsupportedEntity {
                                id: item_id,
                                keyword: complex_name(records),
                            });
                        }
                    }
                }
            }
            _ => {}
        }
    }
    check_assembly_transforms(r, file)?;
    // Orphan check (fail-loud: a shape item no representation
    // references would otherwise vanish or be guessed into the model).
    for (&id, instance) in &file.data {
        if let [(kw, _)] = instance.records.as_slice()
            && (kw == "MANIFOLD_SOLID_BREP" || kw == "GEOMETRIC_CURVE_SET")
            && !referenced.contains(&id)
        {
            return Err(StepImportError::Structure {
                id,
                what: "a solid/curve-set no shape representation references — \
                       refusing rather than guessing whether it is model content",
            });
        }
    }
    match (solids.is_empty(), wireframe) {
        (false, None) => Ok(Shape::Solids(solids)),
        (true, Some((_, curves))) => Ok(Shape::Wireframe(curves)),
        (true, None) => Err(StepImportError::NothingToImport),
        (false, Some((wid, _))) => Err(StepImportError::Structure {
            id: wid,
            what: "solid and wireframe content in one file is outside the subset",
        }),
    }
}

/// A resolver over `file` with no unit knowledge yet — the unit pass's
/// own instrument (it reads enumerations and one uncertainty literal,
/// never a coordinate, so a unit scale of 1 is honest there).
fn unit_pass_resolver(file: &StepFile) -> Resolver<'_> {
    Resolver {
        file,
        length_scale: 1.0,
        eps_in: 0.0,
        next_id: std::cell::Cell::new(0),
        normalizations: std::cell::RefCell::new(Vec::new()),
    }
}

/// Resolves the parsed file into a [`Model`] (module docs).
pub(crate) fn resolve(file: &StepFile) -> Result<Model, StepImportError> {
    let r = unit_pass_resolver(file);

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

    // Two passes, in this order by necessity: the unit context says
    // what a coordinate MEANS, so it is resolved before a single
    // coordinate is read. The geometry pass then runs on a resolver
    // that carries the file's length scale and its scaled ε_in.
    let (length_scale, uncertainty_m) = resolve_units_and_uncertainty(&r, file)?;
    let r = Resolver {
        file,
        length_scale,
        eps_in: uncertainty_m,
        // Past every stated id, so minted topology cannot collide.
        next_id: std::cell::Cell::new(
            file.data.keys().next_back().copied().unwrap_or(0).saturating_add(1),
        ),
        normalizations: std::cell::RefCell::new(Vec::new()),
    };
    let shape = resolve_shape(&r, file)?;
    Ok(Model {
        uncertainty_m,
        shape,
        normalizations: r.normalizations.into_inner(),
    })
}
