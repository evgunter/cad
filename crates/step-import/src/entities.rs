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

use geom::{Curve3, NurbsCurve3};
use geom::{NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Affine3, Mat3, Point3, Vec3};

use crate::chart;
use crate::error::StepImportError;
use crate::geometry;
use crate::normalize;
use crate::parse::{Instance, Record, StepFile, Value};
use crate::recognize;
use crate::recognize_curve;
use crate::signed_zero;
use crate::units::{self, UnitKind};
use crate::{CurvePromotion, FaceCensus, NormalizationKind, StructureNormalization};

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
    /// The D7 stage-1 curve promotions (#327) applied during
    /// resolution, by ascending curve entity id and deduplicated — a
    /// carrier shared by several edges is read once per edge and is
    /// ONE promotion.
    pub(crate) curve_promotions: Vec<CurvePromotion>,
    /// **What the file asks to be MATERIALIZED**, in resolution order:
    /// one entry per placed instance of a solid (M8 instancing), or —
    /// for a file whose assembly places nothing — one unplaced entry
    /// per solid. Never empty for a `Shape::Solids` model, and every
    /// entry's `solid` indexes that vector.
    pub(crate) instances: Vec<SolidInstance>,
}

/// One materialization of one `MANIFOLD_SOLID_BREP`: which solid, and
/// the rigid frame the assembly places that copy in.
///
/// A component representation named by three
/// `NEXT_ASSEMBLY_USAGE_OCCURRENCE`s yields three of these over the
/// same `solid` index with three different maps — the copies are made
/// at materialization (each its own body, each mapped through
/// [`topo::transform_rigid`] and grafted in), never by sharing
/// topology.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SolidInstance {
    /// Index into the model's `Shape::Solids` vector.
    pub(crate) solid: usize,
    /// The shape representation that NAMES this instance's solid —
    /// the component representation for an assembly, and the
    /// representation the solid resolved under otherwise. The key of
    /// the A7 association (component → instances → solids).
    pub(crate) component: u64,
    /// What the ASSEMBLY says about this copy, or `None` for a file
    /// whose assembly places nothing. Every entity id inside names a
    /// real record — an unplaced instance carries no ids at all rather
    /// than a sentinel, so a refusal can never point at `#0`.
    pub(crate) placed: Option<Placed>,
}

/// The assembly's statement about one instance (see
/// [`SolidInstance::placed`]).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Placed {
    /// The rigid map, composed along the placement chain — `None` is
    /// the identity (the component is where its own representation
    /// says it is).
    pub(crate) map: Option<Affine3<f64>>,
    /// The `ITEM_DEFINED_TRANSFORMATION` of this instance's OWN
    /// (innermost) placement — the record a refusal names.
    pub(crate) transform: u64,
    /// The `REPRESENTATION_RELATIONSHIP` complex that states this
    /// occurrence.
    pub(crate) relationship: u64,
    /// The `NEXT_ASSEMBLY_USAGE_OCCURRENCE` the relationship is linked
    /// to through `CONTEXT_DEPENDENT_SHAPE_REPRESENTATION` /
    /// `PRODUCT_DEFINITION_SHAPE`, where the file states one.
    pub(crate) occurrence: Option<u64>,
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
    /// Edge ids of seam generators MINTED by the band re-mint
    /// (M7-5): D1 states each one spatially as its surface's u_ref
    /// half-plane, so adoption must certify it as
    /// [`geom_brep::EdgeGeometry::Seam`] or refuse — the conventional
    /// mapped-curve fallback is withheld for these ids
    /// ([`crate::adopt`]).
    pub(crate) band_seams: std::collections::BTreeSet<u64>,
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
    /// True for a detected SEAMLESS periodic band (M7-5): a
    /// cylinder/torus face whose two bounds each wrap the chart's full
    /// u period, with no seam generator between them. Tagged at
    /// [`Resolver::face`] and consumed at shell level by
    /// `normalize::band_seam`, which re-mints the face as one
    /// single-loop face joined by a minted seam generator (and clears
    /// the tag); a tagged face never reaches assembly.
    pub(crate) band: bool,
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
    /// True when the record's `same_sense` was `.F.` and `start`/`end`
    /// above are therefore the file's END and START (M7-4 Leg E). The
    /// carrier is untouched; this bit composes into the orientation of
    /// every `ORIENTED_EDGE` that uses the edge.
    pub(crate) reversed: bool,
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
    /// The file's angle factor into kernel radians (M7-4 Leg B): 1.0
    /// for a radian context, the file's own declared π/180 for the
    /// wild's dominant `CONVERSION_BASED_UNIT('DEGREE', …)`. Only one
    /// slot in the subset states an angle — a `CONICAL_SURFACE`'s
    /// `semi_angle` — and it passes through [`Resolver::as_angle`].
    angle_scale: f64,
    /// ε_in in kernel meters — the file's declared uncertainty, scaled
    /// with every other length. This is THE interpretation budget: D7's
    /// input tolerance, one number per file, spent by every gate that
    /// has to decide what the file means. Print truncation (Open
    /// CASCADE's 12–13 significant digits) is ~1e-12 RELATIVE, so it
    /// stays far under this absolute budget for any part smaller than
    /// about 100 m; a model large enough to invert that comparison
    /// fails adoption TYPED, with the residual in the refusal, and the
    /// remedy is the per-call ε_in override — D7's own remedy path,
    /// not a hole this reader papers over with a per-literal budget.
    eps_in: f64,
    /// The next unused entity id, for the structure normalizations that
    /// mint topology the file does not carry (Leg C's edge-free
    /// sphere). Seeded past every id in the file, so a minted vertex or
    /// edge can never collide with a stated one.
    next_id: std::cell::Cell<u64>,
    /// The normalizations minted so far, carried out as data
    /// ([`crate::StructureNormalization`]) — never silent.
    normalizations: std::cell::RefCell<Vec<StructureNormalization>>,
    /// The curve promotions fired so far, keyed by carrier entity id
    /// so a carrier read once per incident edge records once
    /// ([`crate::CurvePromotion`]) — never silent.
    curve_promotions: std::cell::RefCell<BTreeMap<u64, CurvePromotion>>,
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
/// writer's printer), with **negative zero normalized to `+0.0`**
/// through [`crate::signed_zero`], which carries the argument. This is
/// the only normalization the numeric path performs.
fn as_real(id: u64, value: &Value, expected: &'static str) -> Result<f64, StepImportError> {
    match value {
        Value::Number(raw) => raw
            .parse::<f64>()
            .map(signed_zero::plus_zero_scalar)
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

    /// One stated angle into kernel radians. A radian context
    /// multiplies by 1.0 and moves no bits; a degree context spends
    /// the file's own declared conversion, so the half-angle a
    /// `CONICAL_SURFACE` states in degrees is not read as radians.
    fn as_angle(
        &self,
        id: u64,
        value: &Value,
        expected: &'static str,
    ) -> Result<f64, StepImportError> {
        Ok(as_real(id, value, expected)? * self.angle_scale)
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
        let ratios = Vec3::new(
            as_real(id, x, expected)?,
            as_real(id, y, expected)?,
            as_real(id, z, expected)?,
        );
        // **Ratios, not a unit vector (M7-4 Leg C).** ISO 10303-42's
        // `direction` is a ratio triple with no normalization
        // requirement, and the wild uses that latitude: an inch-context
        // translator writes `(0.0393700787402, 0., 0.)` for +x. Every
        // kernel consumer — the line carrier whose parameter is arc
        // length, the frames whose axes are orthonormal — wants the
        // unit vector, so a triple that is NOT unit is divided by its
        // own norm.
        //
        // A triple that IS unit is taken **verbatim, bit for bit**.
        // That is not an optimization: this reader's whole discipline
        // is that a stated field is adopted, not re-derived, and
        // `d / d.norm()` moves the last bits of a direction the writer
        // printed exactly (`√½` squares to 0.9999999999999998, whose
        // root is not 1). The window is ε_in — the same budget, and
        // the same argument, the assembly pass already spends on a
        // direction field: dimensionless, but a unit vector's slack is
        // bounded by it just as tightly.
        let norm = ratios.norm();
        if !(norm.is_finite() && norm > 0.0) {
            return Err(StepImportError::MalformedRecord {
                id,
                expected: "three direction ratios that are not all zero and are finite \
                           (a zero triple names no direction)",
            });
        }
        Ok(if (norm - 1.0).abs() <= self.eps_in {
            ratios
        } else {
            ratios / norm
        })
    }

    /// `AXIS2_PLACEMENT_3D('', #location, #axis, #ref_direction)` →
    /// the kernel frame `(origin, axis, u_ref)`, field for field where
    /// the file states the fields.
    ///
    /// **Unset axis / ref_direction (M7-4).** Both are optional in the
    /// schema, and an older PDE/Lib-lineage writer leaves
    /// `ref_direction` `$` on every circle it emits. Filling them in is
    /// not a guess: ISO 10303-42's own `build_axes` / `first_proj_axis`
    /// say exactly what an unset field means — the axis defaults to
    /// `(0,0,1)`, and the reference direction to the first coordinate
    /// axis not parallel to it, projected perpendicular. Reading that
    /// derived attribute is reading the file, in the same sense that
    /// reading a stated one is; what would be a guess is inventing a
    /// DIFFERENT default. The stated fields still take precedence and
    /// are still adopted bit for bit.
    ///
    /// The projection is always applied to the reference direction,
    /// stated or not, because the kernel's frame is orthonormal and
    /// the schema's `ref_direction` need not be perpendicular to the
    /// axis — but the ε_in window in [`Resolver::direction`] means an
    /// already-perpendicular unit pair comes through untouched.
    fn placement(&self, from: u64, id: u64) -> Result<Frame, StepImportError> {
        let args = self.simple(from, id, "AXIS2_PLACEMENT_3D")?;
        let expected = "AXIS2_PLACEMENT_3D(name, #location, #axis, #ref_direction)";
        let [_, location, axis, ref_dir] = args else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        let origin = self.point(id, as_ref(id, location, expected)?)?;
        let axis = match axis {
            Value::Null => Vec3::new(0.0, 0.0, 1.0),
            _ => self.direction(id, as_ref(id, axis, expected)?)?,
        };
        let stated = match ref_dir {
            Value::Null => None,
            _ => Some(self.direction(id, as_ref(id, ref_dir, expected)?)?),
        };
        // A file that already states an orthonormal frame is adopted
        // bit for bit: the projection below is arithmetic, and running
        // it on a `ref_direction` that is already perpendicular would
        // move the last bits of a field the writer printed exactly.
        // The window is ε_in, as everywhere else a direction is
        // compared here.
        if let Some(u_ref) = stated
            && axis.dot(u_ref).abs() <= self.eps_in
        {
            return Ok((origin, axis, u_ref));
        }
        // `first_proj_axis`: the file's reference direction when it has
        // one, else the first coordinate axis the placement's own axis
        // is not parallel to.
        let candidate = stated.unwrap_or_else(|| {
            if axis.x.abs() < 1.0 {
                Vec3::new(1.0, 0.0, 0.0)
            } else {
                Vec3::new(0.0, 1.0, 0.0)
            }
        });
        // How much of the candidate lies ALONG the axis — a projection
        // coefficient, named rather than inlined so the subtraction
        // below reads as "remove the axial part" and so the expression
        // is not the `v * v`-shaped text the interval-square tripwire
        // watches for (there is no square here: `axis` scales a dot of
        // two DIFFERENT vectors, and this crate is f64-only besides).
        let along = axis.dot(candidate);
        let perpendicular = candidate - axis * along;
        let norm = perpendicular.norm();
        if !(norm.is_finite() && norm > 0.0) {
            return Err(StepImportError::MalformedRecord {
                id,
                expected: "AXIS2_PLACEMENT_3D(name, #location, #axis, #ref_direction) \
                           whose reference direction is not parallel to its axis (a \
                           parallel pair states no frame)",
            });
        }
        let u_ref = if (norm - 1.0).abs() <= self.eps_in {
            perpendicular
        } else {
            perpendicular / norm
        };
        Ok((origin, axis, u_ref))
    }

    /// An `ADVANCED_FACE`'s surface reference → the kernel surface,
    /// field for field (the writer's printer table, inverted). Covers
    /// the five elementary kinds plus the writer's two NURBS arms
    /// (M7-3): the non-rational simple `B_SPLINE_SURFACE_WITH_KNOTS`
    /// and the `RATIONAL_B_SPLINE_SURFACE` complex instance — the
    /// curve twin ([`Self::curve`]/[`Self::rational_bspline`]) one
    /// dimension up. Anything else refuses typed.
    fn surface(&self, from: u64, id: u64) -> Result<Surface<f64>, StepImportError> {
        let instance = self.instance(from, id)?;
        if instance.records.len() > 1 {
            return self.rational_bspline_surface(id, &instance.records);
        }
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
                let half_angle = self.as_angle(id, semi_angle, expected)?;
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
                let (location, axis, u_ref) =
                    self.placement(id, as_ref(id, placement, expected)?)?;
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
            // Both radii are read VERBATIM: D3's ring convention
            // `R > r > 0` is not enforced here. It is enforced at rest,
            // by `topo::validate`'s tier-3 check 1 (`DegenerateTorus`) —
            // the one net that covers this door and `sweep::revolve`
            // alike, so a horn or spindle cannot reach a body's rest
            // state through either.
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
            "QUASI_UNIFORM_SURFACE" => {
                let expected = "QUASI_UNIFORM_SURFACE(name, u_degree, v_degree, \
                                ((points)), form, u_closed, v_closed, self_intersect)";
                let [_, du, dv, points, _, _, _, _] = args.as_slice() else {
                    return Err(StepImportError::MalformedRecord { id, expected });
                };
                let (nu, nv, control) = self.control_net(id, points, expected)?;
                let weights = vec![1.0; control.len()];
                let ku = quasi_uniform_knots(id, as_usize(id, du, expected)?, nu)?;
                let kv = quasi_uniform_knots(id, as_usize(id, dv, expected)?, nv)?;
                self.nurbs_surface(id, ku, kv, control, weights)
            }
            "B_SPLINE_SURFACE_WITH_KNOTS" => {
                let expected = "B_SPLINE_SURFACE_WITH_KNOTS(name, u_degree, v_degree, \
                                ((points)), form, u_closed, v_closed, self_intersect, \
                                (u_mults), (v_mults), (u_knots), (v_knots), spec)";
                let [
                    _,
                    du,
                    dv,
                    points,
                    _,
                    _,
                    _,
                    _,
                    u_mults,
                    v_mults,
                    u_knots,
                    v_knots,
                    _,
                ] = args.as_slice()
                else {
                    return Err(StepImportError::MalformedRecord { id, expected });
                };
                let (nu, nv, control) = self.control_net(id, points, expected)?;
                let weights = vec![1.0; control.len()];
                let ku = self.knot_vector(id, du, u_mults, u_knots, nu, expected)?;
                let kv = self.knot_vector(id, dv, v_mults, v_knots, nv, expected)?;
                self.nurbs_surface(id, ku, kv, control, weights)
            }
            other => Err(StepImportError::UnsupportedEntity {
                id,
                keyword: other.to_owned(),
            }),
        }
    }

    /// The `RATIONAL_B_SPLINE_SURFACE` complex instance (weights ≠ 1):
    /// components in alphabetical order, the writer's emission —
    /// degrees and the control net on `B_SPLINE_SURFACE`, knots on
    /// `B_SPLINE_SURFACE_WITH_KNOTS`, the weight net on
    /// `RATIONAL_B_SPLINE_SURFACE` ([`Self::rational_bspline`]'s
    /// layout one dimension up).
    fn rational_bspline_surface(
        &self,
        id: u64,
        records: &[Record],
    ) -> Result<Surface<f64>, StepImportError> {
        let expected = "the RATIONAL_B_SPLINE_SURFACE complex instance (B_SPLINE_SURFACE \
                        B_SPLINE_SURFACE_WITH_KNOTS BOUNDED_SURFACE \
                        GEOMETRIC_REPRESENTATION_ITEM RATIONAL_B_SPLINE_SURFACE \
                        REPRESENTATION_ITEM SURFACE)";
        let mut base: Option<&[Value]> = None;
        let mut with_knots: Option<&[Value]> = None;
        let mut rational: Option<&[Value]> = None;
        for (kw, args) in records {
            match kw.as_str() {
                "B_SPLINE_SURFACE" => base = Some(args),
                "B_SPLINE_SURFACE_WITH_KNOTS" => with_knots = Some(args),
                "RATIONAL_B_SPLINE_SURFACE" => rational = Some(args),
                "BOUNDED_SURFACE"
                | "SURFACE"
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
        let [du, dv, points, _, _, _, _] = base else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        let [u_mults, v_mults, u_knots, v_knots, _] = wk else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        let [weight_net] = rational else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        let (nu, nv, control) = self.control_net(id, points, expected)?;
        let mut weights = Vec::with_capacity(control.len());
        for row in as_list(id, weight_net, expected)? {
            let row = as_list(id, row, expected)?;
            if row.len() != nv {
                return Err(StepImportError::MalformedRecord { id, expected });
            }
            for w in row {
                weights.push(as_real(id, w, expected)?);
            }
        }
        if weights.len() != control.len() {
            return Err(StepImportError::MalformedRecord { id, expected });
        }
        let ku = self.knot_vector(id, du, u_mults, u_knots, nu, expected)?;
        let kv = self.knot_vector(id, dv, v_mults, v_knots, nv, expected)?;
        self.nurbs_surface(id, ku, kv, control, weights)
    }

    /// A LIST OF LIST control-net reference → `(nu, nv, points)` in
    /// the kernel's row-major `iu·nv + iv` layout (outer index u — the
    /// writer's emission, row for row). Ragged or empty nets refuse.
    fn control_net(
        &self,
        id: u64,
        points: &Value,
        expected: &'static str,
    ) -> Result<(usize, usize, Vec<Point3<f64>>), StepImportError> {
        let rows = as_list(id, points, expected)?;
        let mut out = Vec::new();
        let mut nv = None;
        for row in rows {
            let row = as_list(id, row, expected)?;
            if *nv.get_or_insert(row.len()) != row.len() {
                return Err(StepImportError::MalformedRecord { id, expected });
            }
            for p in row {
                out.push(self.point(id, as_ref(id, p, expected)?)?);
            }
        }
        match nv {
            Some(nv) if nv > 0 => Ok((rows.len(), nv, out)),
            _ => Err(StepImportError::MalformedRecord { id, expected }),
        }
    }

    /// A validated kernel NURBS surface from exact components
    /// ([`Self::nurbs`] one dimension up).
    fn nurbs_surface(
        &self,
        id: u64,
        knots_u: KnotVector,
        knots_v: KnotVector,
        control: Vec<Point3<f64>>,
        weights: Vec<f64>,
    ) -> Result<Surface<f64>, StepImportError> {
        NurbsSurface::new(knots_u, knots_v, control, weights)
            .map(|payload| Surface::Nurbs(std::sync::Arc::new(payload)))
            .map_err(|_| StepImportError::MalformedRecord {
                id,
                expected: "a structurally valid B-spline surface (control net matching \
                           the knot vectors' counts, weights positive)",
            })
    }

    /// An `EDGE_CURVE`'s (or curve set's) curve reference → the kernel
    /// carrier, exact. Covers the writer's four printers — `LINE`,
    /// `CIRCLE`, `ELLIPSE`, `B_SPLINE_CURVE_WITH_KNOTS` (simple
    /// non-rational form and the `RATIONAL_B_SPLINE_CURVE` complex
    /// instance) — plus the knots-implied `QUASI_UNIFORM_CURVE`
    /// sub-type ([`quasi_uniform_knots`]), which the writer never
    /// emits but I-DEAS-lineage translators do.
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
                let vexpected = "VECTOR(name, #direction, magnitude > 0) — the line's \
                                 direction must be a direction";
                let [_, dir_ref, magnitude] = vargs else {
                    return Err(StepImportError::MalformedRecord {
                        id: vec_id,
                        expected: vexpected,
                    });
                };
                // **Any positive magnitude (M7-4 Leg C).** The file's
                // line parameter is not arc length — ST-Developer
                // writes `10.`, an inch translator `25.4` — and the
                // kernel's is. Both facts survive together because the
                // magnitude has nowhere to go: no trim parameters
                // cross the wire, so the parameter interval is
                // re-derived from the two vertices against the carrier
                // ([`geometry::endpoint_params`]), and re-deriving it
                // against the UNIT direction is exactly the rescaling.
                // What the magnitude must not do is silently survive
                // into a non-unit `dir`, which would make the kernel's
                // t neither arc length nor the file's parameter — so
                // the direction is normalized here, and the wild's
                // non-unit `DIRECTION` ratios (an inch file's
                // `(0.0393700787402, 0., 0.)`) come out right by the
                // same division. A file already stating unit ratios at
                // magnitude 1 divides by an exact 1.0 and moves no
                // bits.
                let magnitude = as_real(vec_id, magnitude, vexpected)?;
                if !(magnitude.is_finite() && magnitude > 0.0) {
                    return Err(StepImportError::MalformedRecord {
                        id: vec_id,
                        expected: "VECTOR(name, #direction, magnitude) with a finite, \
                                   strictly positive magnitude (a zero or non-finite \
                                   one describes no line)",
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
            "QUASI_UNIFORM_CURVE" => {
                let expected = "QUASI_UNIFORM_CURVE(name, degree, (points), form, \
                                closed, self_intersect)";
                let [_, degree, points, _, _, _] = args.as_slice() else {
                    return Err(StepImportError::MalformedRecord { id, expected });
                };
                let control = self.control_points(id, points, expected)?;
                let knots =
                    quasi_uniform_knots(id, as_usize(id, degree, expected)?, control.len())?;
                let weights = vec![1.0; control.len()];
                self.nurbs(id, knots, control, weights)
            }
            "B_SPLINE_CURVE_WITH_KNOTS" => {
                let expected = "B_SPLINE_CURVE_WITH_KNOTS(name, degree, (points), \
                                form, closed, self_intersect, (mults), (knots), spec)";
                let [_, degree, points, _, _, _, mults, knots, _] = args.as_slice() else {
                    return Err(StepImportError::MalformedRecord { id, expected });
                };
                let control = self.control_points(id, points, expected)?;
                let weights = vec![1.0; control.len()];
                let knots = self.knot_vector(id, degree, mults, knots, control.len(), expected)?;
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
        let knots = self.knot_vector(id, degree, mults, knots, control.len(), expected)?;
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

    /// The refusal text for a knot-multiplicity list that does not
    /// sum to the count ISO 10303-42 fixes ([`Resolver::knot_vector`]).
    const MULT_BUDGET: &'static str = "knot multiplicities summing to \
         control_points + degree + 1 — ISO 10303-42's count for a clamped \
         B-spline. A larger sum describes a different curve, and expanding \
         it before checking would let the file choose this reader's \
         allocation size";

    /// The `(multiplicities) / (knots)` pair → the kernel's flat
    /// clamped knot vector, exact (run-length decode is the inverse of
    /// the writer's exact-equality encode; no ε enters).
    ///
    /// # The multiplicity budget, checked BEFORE the expansion
    ///
    /// Run-length decoding means the FILE states how much memory this
    /// reader is about to allocate, and it is reachable from an
    /// ordinary `EDGE_CURVE`. `(2000000000, 2000000000)` asks for
    /// 16 GB, and the allocator's answer to that is `abort` — strictly
    /// worse than a panic, because no `catch_unwind` can see it, so
    /// the crate's own "every file comes back with a RESULT" row
    /// cannot catch the class at all. (Found by the M7-4 review's
    /// hostile-knot probe; the defect is older than this unit and is
    /// fixed here because this unit is what made the promise.)
    ///
    /// The bound is not an arbitrary cap. ISO 10303-42's
    /// `b_spline_curve_with_knots` fixes the total exactly: a clamped
    /// curve of degree `d` over `n` control points has `n + d + 1`
    /// knots, and that is the whole allocation. A file whose
    /// multiplicities sum to anything else is not describing this
    /// curve, whether it overshot by one or by two billion — so one
    /// typed refusal covers the honest malformation and the hostile
    /// one alike, and it is reached without allocating anything. The
    /// running sum is checked arithmetic, because the overflow is
    /// reachable too.
    fn knot_vector(
        &self,
        id: u64,
        degree: &Value,
        mults: &Value,
        knots: &Value,
        control_points: usize,
        expected: &'static str,
    ) -> Result<KnotVector, StepImportError> {
        let degree = as_usize(id, degree, expected)?;
        let mults = as_list(id, mults, expected)?;
        let values = as_list(id, knots, expected)?;
        if mults.len() != values.len() {
            return Err(StepImportError::MalformedRecord { id, expected });
        }
        let budget = control_points.saturating_add(degree).saturating_add(1);
        let mut total: usize = 0;
        for m in mults {
            total = match total.checked_add(as_usize(id, m, expected)?) {
                Some(t) if t <= budget => t,
                _ => {
                    return Err(StepImportError::MalformedRecord {
                        id,
                        expected: Self::MULT_BUDGET,
                    });
                }
            };
        }
        if total != budget {
            return Err(StepImportError::MalformedRecord {
                id,
                expected: Self::MULT_BUDGET,
            });
        }
        let mut flat = Vec::with_capacity(total);
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

    /// A validated kernel NURBS carrier from exact components — and
    /// the **D7 stage-1 CURVE recognition site** (#327).
    ///
    /// Every NURBS carrier the file states passes through here (the
    /// `B_SPLINE_CURVE_WITH_KNOTS` arm, the `QUASI_UNIFORM_CURVE` arm,
    /// and the `RATIONAL_B_SPLINE_CURVE` complex), and each is tested
    /// for promotion to an analytic kind exactly as every NURBS
    /// SURFACE is tested at [`Self::face`]. This is the right site for
    /// the same reason that one is: it is upstream of everything that
    /// asks a carrier what it IS — the adoption ladder's `MappedCurve`
    /// rungs, `endpoint_params`' conic arm, the pcurve mint, the
    /// re-export printer — so a promoted carrier is analytic
    /// everywhere, with no second reading anywhere to disagree with
    /// the first.
    ///
    /// Promotion is verified-not-trusted ([`crate::recognize_curve`]):
    /// it fires iff the certificate holds at ε_in, the file's own form
    /// flag is never consulted, and a carrier that certifies nowhere
    /// stays NURBS silently. It is reported as data through
    /// [`crate::CurvePromotion`].
    fn nurbs(
        &self,
        id: u64,
        knots: KnotVector,
        control: Vec<Point3<f64>>,
        weights: Vec<f64>,
    ) -> Result<Curve3<f64>, StepImportError> {
        NurbsCurve3::new(knots, control, weights)
            .map(
                |payload| match recognize_curve::recognize(&payload, self.eps_in) {
                    recognize_curve::CurveRecognition::Promoted {
                        curve,
                        residual,
                        kind,
                    } => {
                        self.curve_promotions.borrow_mut().insert(
                            id,
                            CurvePromotion {
                                curve: id,
                                kind,
                                residual,
                            },
                        );
                        curve
                    }
                    // Both non-promoting outcomes stay NURBS. The
                    // ill-conditioned one has no escalation site for
                    // curves (recognizer docs): no gate needs a curve
                    // promotion in order to import at all, so the honest
                    // answer is the same one a refuted carrier gets.
                    recognize_curve::CurveRecognition::StaysNurbs
                    | recognize_curve::CurveRecognition::IllConditioned { .. } => {
                        Curve3::Nurbs(std::sync::Arc::new(payload))
                    }
                },
            )
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
        let mut surface = self.surface(id, surface_id)?;
        let mut sense = as_bool(id, same_sense, expected)?;
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
        // **D7 stage-1 surface recognition (ruling #256).** Every NURBS
        // surface is tested for promotion HERE — at the face, before
        // the multi-bound gate below, which is exactly the gate a
        // promoted plane's trim rings must reach as a Plane (a
        // normalize-level pass would be too late; the refusal fires in
        // this function). Promotion is verified-not-trusted: it fires
        // iff the recognizer's residual certifies at ε_in, and a patch
        // that certifies nowhere stays NURBS silently — the state
        // whose import behavior this crate already documents. The
        // promotion is reported through the normalizations channel
        // (census identity; the recorded residual bounds the motion),
        // and the face's `same_sense` composes with the chart
        // orientation so the promoted chart means what the NURBS chart
        // meant.
        let mut ill_conditioned = None;
        if let Surface::Nurbs(ref patch) = surface
            && !patch.is_placeholder()
        {
            match recognize::recognize(patch.as_ref(), self.eps_in) {
                recognize::Recognition::Promoted {
                    surface: promoted,
                    residual,
                    kind,
                } => {
                    if recognize::chart_flipped(patch.as_ref(), &promoted) {
                        sense = !sense;
                    }
                    let census = bounds_census(&bound_specs, edges);
                    self.normalizations
                        .borrow_mut()
                        .push(StructureNormalization {
                            face: id,
                            kind: NormalizationKind::SurfacePromotion { to: kind, residual },
                            file_census: census,
                            kernel_census: census,
                        });
                    surface = promoted;
                }
                recognize::Recognition::StaysNurbs => {}
                recognize::Recognition::IllConditioned { kind, margin } => {
                    ill_conditioned = Some((kind, margin));
                }
            }
        }
        // The edge-free closed face: one VERTEX_LOOP and nothing else.
        if bound_specs.iter().any(|b| b.vertex_loop.is_some()) {
            return self.edge_free_face(id, surface, sense, &bound_specs, edges, vertices);
        }

        let mut loops: Vec<LoopSpec> = bound_specs
            .iter()
            .map(|b| LoopSpec {
                outer: false,
                uses: b.uses.clone(),
            })
            .collect();
        // **A ring on a curved face (M7-4).** The kernel's mass
        // properties have no construction for one — its curved patches
        // are swept UV rectangles, and `topo::mass_properties` says so
        // by name (`RingOnCurvedFace`), which makes tier-3 validity
        // refuse with it. This crate promises a body that is
        // tier-valid at rest, so the honest place to stop is here,
        // naming the face, rather than at the far end holding a body
        // whose volume nothing can compute.
        //
        // What can also arrive this way is not a hole: it is Open
        // CASCADE's SEAMLESS periodic face — a cylinder's lateral
        // band, or a fillet torus's, stated as its two rim circles
        // with no seam generator between them. The kernel's own
        // writer never emits one (it splits a periodic face at its
        // seam). Since M7-5 the cylinder and torus cases NORMALIZE:
        // the shape is recognized here (two bounds, each wrapping the
        // chart's full u period) and tagged through to shell level,
        // where `normalize::band_seam` re-mints the face as one
        // single-loop face joined by a minted seam generator at the
        // surface's own u_ref azimuth — the edge-free sphere's
        // license, recorded as a `StructureNormalization`. The band
        // face has no outer bound to infer (its two rims are not
        // inside one another — `SeamDependent` both ways), so it
        // returns before the outerness walk; the mint's single loop
        // is outer by construction.
        if loops.len() > 1 && !matches!(surface, Surface::Plane { .. }) {
            if is_periodic_band(&surface, &loops, edges) {
                return Ok(vec![FaceSpec {
                    id,
                    surface,
                    sense,
                    loops,
                    band: true,
                }]);
            }
            // A multi-bound curved face refuses HERE — and since D7
            // stage-1 recognition (above), a NURBS surface reaches
            // this gate only AFTER promotion was tried: rings on
            // promoted PLANES have already left through the
            // plane-guard on this branch, so what refuses is rings on
            // genuinely curved patches — promoted cylinders included
            // (the kernel has no volume construction for a curved
            // face with rings) — and rings on NURBS that certified as
            // no implemented analytic kind. Where the face could ONLY
            // import by promotion and the recognizer's estimator was
            // ill-conditioned at ε_in, the refusal is D7's typed
            // ambiguity instead of this bare topology one.
            if let Some((kind, margin)) = ill_conditioned {
                return Err(StepImportError::RecognitionAmbiguous {
                    id,
                    surface: surface_id,
                    kind,
                    margin,
                });
            }
            return Err(StepImportError::Topology {
                id,
                what: "a curved ADVANCED_FACE with more than one bound that is not a \
                       recognized periodic band — an interior ring on a curved patch \
                       (a promoted cylinder's, or a NURBS patch's that certified as no \
                       implemented analytic kind at ε_in — stage-1 recognition \
                       promotes certified planes and cylinders, and rings on promoted \
                       planes import; the kernel has no volume construction for a \
                       curved face with rings) or a seamless periodic band on a chart \
                       the band re-mint does not cover (cylinder and torus bands \
                       normalize; a cone or sphere-zone band would take the same \
                       seam-generator re-mint, extended to its chart)",
            });
        }
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
            band: false,
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
        // A single bound is outer BY DEFINITION: there is nothing for
        // it to be inside of. Stated, because it is a definition and
        // not a measurement. (Several bounds all stated outer on a
        // ONE-bound face is not reachable — the list has one entry.)
        if loops.len() == 1 {
            return Ok(0);
        }
        let rings: Vec<Vec<Point3<f64>>> = loops
            .iter()
            .map(|lp| self.ring_samples(lp, edges))
            .collect::<Result<_, _>>()?;
        let inferred = chart::infer_outer(surface, &rings, self.eps_in);
        // **Several stated outer bounds (M7-4).** A closed periodic
        // face — a cylinder's lateral band — genuinely has no outer
        // bound: its two boundary circles are not inside one another,
        // and the Onshape-lineage writer measured in the wild marks
        // BOTH `FACE_OUTER_BOUND` for exactly that reason. The file is
        // not contradicting itself there; it is declining to choose,
        // in a vocabulary with no way to say so — and on that face it
        // is right, which is why the inference below answers
        // `SeamDependent` for both rings.
        //
        // So: the geometry breaks the tie when it can, and when it
        // cannot the file's own bound ORDER does. That second branch
        // is not a guess about geometry — the kernel's `loops[0]` is
        // where the face's traversal starts, not a claim that one
        // circle encloses the other — and it is the same latitude the
        // single-stated case below already takes when the inference
        // declines (a NIST washer's cylindrical bands, stated
        // `FACE_OUTER_BOUND` + `FACE_BOUND`, import through it today).
        // What still refuses is a file whose geometry names an outer
        // bound it did NOT state: that is a contradiction, not a
        // declination.
        if stated.len() > 1 {
            return match inferred {
                Ok(i) if stated.contains(&i) => Ok(i),
                Ok(_) => Err(StepImportError::Topology {
                    id,
                    what: "an ADVANCED_FACE with several FACE_OUTER_BOUNDs, none of \
                           which is the bound its own geometry makes outer — the file \
                           contradicts itself",
                }),
                Err(_) => Ok(stated[0]),
            };
        }
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
        // The budget is ε_in — the file's own declared uncertainty,
        // scaled into kernel metres (D7's rule, unchanged).
        let eps = self.eps_in;
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
        let v_ref = signed_zero::plus_zero(axis.cross(u_ref));
        let north = signed_zero::plus_zero_point(center + axis * radius);
        let south = signed_zero::plus_zero_point(center - axis * radius);
        let (nv, sv) = (self.mint_id(), self.mint_id());
        vertices.insert(nv, north);
        vertices.insert(sv, south);
        let mut meridian = |circle_axis: Vec3<f64>| -> Result<u64, StepImportError> {
            let eid = self.mint_id();
            let carrier = Curve3::Circle {
                center,
                axis: signed_zero::plus_zero(circle_axis),
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
                    // A minted edge: the importer states its own
                    // start → end, so there is no sense to compose.
                    reversed: false,
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
            band: false,
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
            // Leg E's composition: an `EDGE_CURVE` read from its other
            // end (`same_sense` `.F.`) flips what "forward" means for
            // every use of it. The two orientation statements — the
            // edge's and the use's — multiply, exactly once, here.
            let reversed = edges.get(&edge_id).is_some_and(|spec| spec.reversed);
            uses.push(EdgeUse {
                edge: edge_id,
                forward: forward != reversed,
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
        let expected = "EDGE_CURVE(name, #start, #end, #curve, same_sense)";
        let [_, start_ref, end_ref, curve_ref, same_sense] = args else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        // **`same_sense` .F. (M7-4 Leg E).** The flag says whether the
        // carrier's increasing parameter runs start → end. The kernel's
        // `he_plus` always does, so a `.F.` edge is the SAME edge read
        // from the other end, and that is how it is taken: the two
        // vertices swap, the derived interval comes out `t0 < t1` on
        // the carrier exactly as stated, and the reversal rides on the
        // spec for every `ORIENTED_EDGE` that uses this edge to compose
        // into its own orientation flag.
        //
        // What this deliberately does NOT do is reverse the carrier.
        // Negating a line's direction or flipping a circle's frame
        // would move bits the file printed, break the export fixed
        // point, and put a curve in the body that no record states.
        // Composing into the half-edge direction moves nothing — it is
        // bookkeeping about traversal, which is where orientation lives
        // in a half-edge structure anyway.
        let same_sense = as_bool(id, same_sense, expected)?;
        let (start_ref, end_ref) = if same_sense {
            (start_ref, end_ref)
        } else {
            (end_ref, start_ref)
        };
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
            reversed: !same_sense,
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
            band_seams: std::collections::BTreeSet::new(),
        };
        // The reported structure normalizations for periodic faces the
        // kernel cannot represent as stated (Leg C).
        normalize::normalize_shell(
            &mut solid,
            self.eps_in,
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

/// Whether a multi-bound curved face is Open CASCADE's SEAMLESS
/// periodic band (M7-5): a cylinder or torus face with exactly two
/// bounds, each a closed chain whose chart image wraps the full u
/// period exactly once, the two in opposite directions (any coherent
/// band's rims oppose, whichever `same_sense` the face states).
///
/// The recognition is chain-agnostic — a rim may be one self-loop
/// circle (every fixture's shape) or several arcs — because it reads
/// the CHART, not the chain: each use is sampled along its own
/// parameter interval through the kernel's chart inverse
/// ([`chart::uv_of`]) and the loop's total u displacement is
/// accumulated with unwrapping. Anything that does not read cleanly
/// (a point off the chart, a step too long to unwrap) is simply not
/// detected as a band and takes the gate's typed refusal — detection
/// never guesses.
/// The boundary-graph census a face's stated bounds contribute — the
/// identity census a surface promotion records on both sides of its
/// mapping (the promotion changes the surface DESCRIPTION, never the
/// tessellation). Distinct edges and distinct vertices over every
/// bound; a `VERTEX_LOOP` bound contributes its one vertex.
fn bounds_census(bounds: &[BoundSpec], edges: &BTreeMap<u64, EdgeSpec>) -> FaceCensus {
    let mut edge_ids = std::collections::BTreeSet::new();
    let mut vertex_ids = std::collections::BTreeSet::new();
    for bound in bounds {
        if let Some(v) = bound.vertex_loop {
            vertex_ids.insert(v);
        }
        for u in &bound.uses {
            edge_ids.insert(u.edge);
            if let Some(spec) = edges.get(&u.edge) {
                vertex_ids.insert(spec.start);
                vertex_ids.insert(spec.end);
            }
        }
    }
    FaceCensus {
        faces: 1,
        edges: edge_ids.len(),
        vertices: vertex_ids.len(),
    }
}

/// The implied clamped knot vector of a `QUASI_UNIFORM_CURVE` /
/// `QUASI_UNIFORM_SURFACE` record, synthesized closed-form.
///
/// ISO 10303-42's `quasi_uniform_knots` states the SHAPE, not the
/// values: end knots at multiplicity `degree + 1`, interior knots at
/// multiplicity 1, evenly spaced. Any even spacing describes the same
/// locus up to an affine reparameterization (which nothing downstream
/// observes — parameter intervals are re-derived against the carrier,
/// pcurves are re-minted), so the synthesis fixes integer spacing
/// `0, 1, …, control_points − degree` as the canonical instance. No ε
/// enters: the knots are exact small integers.
fn quasi_uniform_knots(
    id: u64,
    degree: usize,
    control_points: usize,
) -> Result<KnotVector, StepImportError> {
    let expected = "a quasi-uniform B-spline with more control points than its \
                    degree (fewer describe no spline of that degree)";
    if control_points <= degree {
        return Err(StepImportError::MalformedRecord { id, expected });
    }
    let spans = control_points - degree;
    let mut flat = Vec::with_capacity(control_points + degree + 1);
    flat.extend(std::iter::repeat_n(0.0, degree + 1));
    for k in 1..spans {
        flat.push(k as f64);
    }
    flat.extend(std::iter::repeat_n(spans as f64, degree + 1));
    KnotVector::clamped(flat, degree).map_err(|_| StepImportError::MalformedRecord {
        id,
        expected: "a clamped knot vector synthesized from the quasi-uniform shape",
    })
}

fn is_periodic_band(
    surface: &Surface<f64>,
    loops: &[LoopSpec],
    edges: &BTreeMap<u64, EdgeSpec>,
) -> bool {
    if !matches!(surface, Surface::Cylinder { .. } | Surface::Torus { .. }) {
        return false;
    }
    let [a, b] = loops else {
        return false;
    };
    let (Some(wa), Some(wb)) = (
        loop_u_wrap(surface, a, edges),
        loop_u_wrap(surface, b, edges),
    ) else {
        return false;
    };
    // Full-period wrap, once, each way. The 1e-6 rad slack is a
    // recognition tolerance on a TOPOLOGICAL count (the wrap number is
    // an integer multiple of 2π up to sampling arithmetic), not a
    // geometric budget — nothing minted depends on it.
    let full = |w: f64| (w.abs() - core::f64::consts::TAU).abs() < 1e-6;
    full(wa) && full(wb) && wa * wb < 0.0
}

/// The loop's total signed chart-u displacement: each use sampled
/// along its traversal, mapped through [`chart::uv_of`], adjacent
/// samples unwrapped across the seam. `None` when any sample misses
/// the chart or two adjacent samples are more than a quarter period
/// apart (too sparse to unwrap without guessing).
fn loop_u_wrap(
    surface: &Surface<f64>,
    lp: &LoopSpec,
    edges: &BTreeMap<u64, EdgeSpec>,
) -> Option<f64> {
    use core::f64::consts::{PI, TAU};
    // 16 steps per edge puts adjacent samples of a full-period rim
    // ~0.39 rad apart — far inside the unwrap threshold below.
    const STEPS: usize = 16;
    let mut total = 0.0;
    let mut prev: Option<f64> = None;
    for use_ in &lp.uses {
        let spec = edges.get(&use_.edge)?;
        for k in 0..=STEPS {
            let f = k as f64 / STEPS as f64;
            let f = if use_.forward { f } else { 1.0 - f };
            let t = spec.t0 + (spec.t1 - spec.t0) * f;
            let u = chart::uv_of(surface, spec.carrier.eval(t))?.x;
            if let Some(p) = prev {
                let mut d = u - p;
                if d > PI {
                    d -= TAU;
                } else if d < -PI {
                    d += TAU;
                }
                if d.abs() > PI / 2.0 {
                    return None;
                }
                total += d;
            }
            prev = Some(u);
        }
    }
    Some(total)
}

/// How deep a chain of `CONVERSION_BASED_UNIT`s may nest before the
/// resolver calls it pathological. One level is the whole wild corpus
/// (inch over millimetre, degree over radian); a few more cost
/// nothing; an unbounded walk over a file that defines a unit in terms
/// of itself would not terminate, and a cyclic file is malformed
/// rather than merely deep.
const CONVERSION_DEPTH_LIMIT: u32 = 8;

impl Resolver<'_> {
    /// One `CONVERSION_BASED_UNIT(name, #factor)` complex → its
    /// [`UnitKind`] (M7-4 Leg B).
    ///
    /// The factor is a `*_MEASURE_WITH_UNIT` naming a value and the
    /// base unit that value is stated in, and BOTH come from the file:
    /// `INCH` resolves as `25.4 × (whatever #17 is)`, so a file whose
    /// inch is declared over metres and one whose inch is declared
    /// over millimetres both import at their own arithmetic. Nothing
    /// here consults a table of unit names — the `'INCH'` label is
    /// documentation, and this resolver never reads it.
    fn conversion_unit(
        &self,
        id: u64,
        records: &[Record],
        args: &[Value],
        depth: u32,
    ) -> Result<UnitKind, StepImportError> {
        let found = || complex_name(records);
        if depth >= CONVERSION_DEPTH_LIMIT {
            return Err(StepImportError::UnsupportedUnit {
                id,
                found: format!(
                    "{} at the end of a {CONVERSION_DEPTH_LIMIT}-deep conversion chain \
                     (a unit defined in terms of itself has no factor)",
                    found()
                ),
            });
        }
        // The quantity the complex claims to be: its NAMED_UNIT
        // subtype component. Exactly one must be present — a complex
        // claiming to be both a length and an angle states no unit.
        let quantities: Vec<&str> = records
            .iter()
            .map(|(kw, _)| kw.as_str())
            .filter(|kw| {
                kw.ends_with("_UNIT") && *kw != "CONVERSION_BASED_UNIT" && *kw != "NAMED_UNIT"
            })
            .collect();
        let [declared] = quantities.as_slice() else {
            return Err(StepImportError::UnsupportedUnit { id, found: found() });
        };
        let [_, factor_ref] = args else {
            return Err(StepImportError::MalformedRecord {
                id,
                expected: "CONVERSION_BASED_UNIT(name, #conversion_factor)",
            });
        };
        let (value, base) = self.measure_with_unit(
            id,
            as_ref(
                id,
                factor_ref,
                "the conversion factor of a CONVERSION_BASED_UNIT",
            )?,
            depth,
        )?;
        units::conversion_kind(id, declared, value, base, found)
    }

    /// One `*_MEASURE_WITH_UNIT(MEASURE(value), #unit)` → the raw
    /// value and the unit it is stated in. This is the conversion
    /// expression a `CONVERSION_BASED_UNIT` points at; the measure's
    /// own keyword is not pinned here (the schema pairs it with the
    /// unit subtype, and [`units::conversion_kind`] checks that the
    /// resolved base agrees with the quantity claimed) — what IS
    /// pinned is that it is a measure with a unit at all.
    fn measure_with_unit(
        &self,
        from: u64,
        id: u64,
        depth: u32,
    ) -> Result<(f64, UnitKind), StepImportError> {
        let instance = self.instance(from, id)?;
        let expected = "a *_MEASURE_WITH_UNIT(MEASURE(value), #unit) conversion factor";
        let [(kw, args)] = instance.records.as_slice() else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        if !kw.ends_with("_MEASURE_WITH_UNIT") {
            return Err(StepImportError::WrongEntityType {
                id,
                expected,
                found: kw.clone(),
            });
        }
        let [Value::Typed(measure, inner), unit_ref] = args.as_slice() else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        if !measure.ends_with("_MEASURE") {
            return Err(StepImportError::MalformedRecord { id, expected });
        }
        let [value] = inner.as_slice() else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        let base_id = as_ref(id, unit_ref, expected)?;
        let base = self.instance(id, base_id)?;
        let kind = check_unit(self, base_id, &base.records, depth + 1)?;
        Ok((as_real(id, value, expected)?, kind))
    }
}

/// Reads one unit component of the `GLOBAL_UNIT_ASSIGNED_CONTEXT` into
/// its [`UnitKind`]: an `SI_UNIT(prefix, name)` whose name is
/// `.METRE.` / `.RADIAN.` / `.STERADIAN.`, with a resolved
/// [`units`] prefix factor on lengths (M7-2 Leg A — `.MILLI.` is what
/// FreeCAD writes, and the prefix is table data, not a special case).
/// Parsed, not assumed (M7-1 spec Leg B) — anything else (a prefixed
/// angle, a mass) refuses typed.
///
/// **M7-4 Leg B** adds the wild's dominant form beside it: a
/// `CONVERSION_BASED_UNIT` resolves through the conversion expression
/// the FILE states ([`Resolver::conversion_unit`]), so an inch is
/// 25.4 of whatever base the file names and never 25.4 by assumption.
fn check_unit(
    r: &Resolver<'_>,
    id: u64,
    records: &[Record],
    depth: u32,
) -> Result<UnitKind, StepImportError> {
    for (kw, args) in records {
        if kw == "CONVERSION_BASED_UNIT" {
            return r.conversion_unit(id, records, args, depth);
        }
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

/// The **length** factor of `records` into meters (a `LENGTH_UNIT`
/// composed with an `SI_UNIT` metre or with a `CONVERSION_BASED_UNIT`
/// over one), or `None` when the record is not a length.
fn length_factor(
    r: &Resolver<'_>,
    id: u64,
    records: &[Record],
) -> Result<Option<f64>, StepImportError> {
    if !records.iter().any(|(kw, _)| kw == "LENGTH_UNIT") {
        return Ok(None);
    }
    match check_unit(r, id, records, 0)? {
        UnitKind::Length(factor) => Ok(Some(factor)),
        // A LENGTH_UNIT composed with a radian is a malformed context,
        // not a unit the subset merely lacks.
        _ => Err(StepImportError::UnsupportedUnit {
            id,
            found: complex_name(records),
        }),
    }
}

/// The representation contexts the file's own **geometry roots** name
/// (M7-4 Leg B): for every representation that holds a
/// `MANIFOLD_SOLID_BREP` or a `GEOMETRIC_CURVE_SET`, its context
/// reference.
///
/// This runs before a single coordinate is read — it looks only at
/// keywords and reference slots — because the answer decides what a
/// coordinate MEANS. It is deliberately narrower than "every
/// `GEOMETRIC_REPRESENTATION_CONTEXT` in the file": Open CASCADE emits
/// one dimensionless `GEOMETRIC_REPRESENTATION_CONTEXT(2)
/// PARAMETRIC_REPRESENTATION_CONTEXT()` per pcurve
/// `DEFINITIONAL_REPRESENTATION` (145 in one measured file), which
/// declares no length unit and needs none — it is a chart domain, not
/// model space — and translators leave whole unreferenced 3D contexts
/// behind. Neither states the solid's units, so neither is consulted.
///
/// An empty answer is not an error here: the shape pass owns the
/// "nothing to import" refusal and names the defect better.
fn content_roots(r: &Resolver<'_>, file: &StepFile) -> Result<ContentRoots, StepImportError> {
    let mut out = ContentRoots::default();
    for (&id, instance) in &file.data {
        let [(kw, args)] = instance.records.as_slice() else {
            continue;
        };
        if !kw.ends_with("SHAPE_REPRESENTATION") {
            continue;
        }
        let expected = "a shape representation (name, (items), #context)";
        let [_, items, context] = args.as_slice() else {
            continue;
        };
        let mut carries_content = false;
        for item in as_list(id, items, expected)? {
            let item_id = as_ref(id, item, expected)?;
            if let [(k, _)] = r.instance(id, item_id)?.records.as_slice()
                && (k == "MANIFOLD_SOLID_BREP" || k == "GEOMETRIC_CURVE_SET")
            {
                carries_content = true;
            }
        }
        if carries_content {
            out.reps.insert(id);
            let cid = as_ref(id, context, expected)?;
            let context = r.instance(id, cid)?;
            if !context
                .records
                .iter()
                .any(|(kw, _)| kw == "GEOMETRIC_REPRESENTATION_CONTEXT")
            {
                return Err(StepImportError::WrongEntityType {
                    id: cid,
                    expected: "a GEOMETRIC_REPRESENTATION_CONTEXT (the context a solid's \
                               coordinates are stated in)",
                    found: complex_name(&context.records),
                });
            }
            out.contexts.insert(cid);
        }
    }
    Ok(out)
}

impl Resolver<'_> {
    /// One `ITEM_DEFINED_TRANSFORMATION(name, description,
    /// #placement_1, #placement_2)` as the rigid map carrying the
    /// first frame onto the second — `None` when the two frames are
    /// the identity of one another at ε_in (M7-4 Leg D).
    ///
    /// The frames are built orthonormal from the file's fields: the
    /// axis is the third column, the reference direction is projected
    /// perpendicular to it for the first, and the second is their
    /// cross product. The map is then `B ∘ A⁻¹`, and `A⁻¹` is `Aᵀ`
    /// because `A` is orthonormal by construction. The rigidity that
    /// survives this construction is therefore about the FILE's
    /// fields, and that is what is checked: a `ref_direction` parallel
    /// to the axis leaves no frame at all, and a placement pair whose
    /// composed determinant is not +1 at ε_in is a mirror.
    fn item_defined_transformation(
        &self,
        from: u64,
        id: u64,
    ) -> Result<Option<Affine3<f64>>, StepImportError> {
        // The schema's `transformation` SELECT also admits a
        // `functionally_defined_transformation` —
        // `CARTESIAN_TRANSFORMATION_OPERATOR_3D` and friends — and THAT
        // is where a file can state a mirror or a scale: an operator
        // carries a scale factor and three independent axes, so its
        // determinant is whatever the file wrote. A placement PAIR
        // cannot say either thing (see the determinant note below), so
        // this is the refusal Leg D's mirror clause actually lives in,
        // and it names the operator.
        let instance = self.instance(from, id)?;
        if !instance
            .records
            .iter()
            .any(|(kw, _)| kw == "ITEM_DEFINED_TRANSFORMATION")
        {
            return Err(StepImportError::Structure {
                id,
                what: "an assembly transformation stated as an operator rather than a \
                       pair of placements — an operator can carry a scale factor and a \
                       mirrored (determinant −1) axis triple, which change what the \
                       component IS rather than where it sits, and which way a mirrored \
                       solid's faces then point is the file's intent, not this reader's \
                       to guess",
            });
        }
        let args = self.simple(from, id, "ITEM_DEFINED_TRANSFORMATION")?;
        let expected = "ITEM_DEFINED_TRANSFORMATION(name, description, #placement_1, \
                        #placement_2)";
        let [_, _, from_ref, to_ref] = args else {
            return Err(StepImportError::MalformedRecord { id, expected });
        };
        let a = self.placement(id, as_ref(id, from_ref, expected)?)?;
        let b = self.placement(id, as_ref(id, to_ref, expected)?)?;
        // Identity at ε_in: the origins coincide within the file's own
        // declared uncertainty, and so do the two direction fields
        // (dimensionless, but ε_in is the one budget this importer
        // spends and a unit vector's slack is bounded by it just as
        // tightly). Checked on the STATED fields, before any
        // orthonormalization, so an identity pair takes the same path
        // it took in M7-2 and moves nothing.
        if (a.0 - b.0).norm() <= self.eps_in
            && (a.1 - b.1).norm() <= self.eps_in
            && (a.2 - b.2).norm() <= self.eps_in
        {
            return Ok(None);
        }
        let basis = |f: Frame| -> Option<Mat3<f64>> {
            let z = f.1;
            // The reference direction's axial part, removed to leave
            // the frame's first column (named for the same two reasons
            // as in `Resolver::placement`: it reads as a projection,
            // and it is not the `v * v` text the interval-square
            // tripwire watches for — no square is taken here).
            let along = z.dot(f.2);
            let x = f.2 - z * along;
            let n = x.norm();
            (n.is_finite() && n > 0.0).then(|| {
                let x = x / n;
                Mat3::from_cols(x, z.cross(x), z)
            })
        };
        let refuse = |what: &'static str| StepImportError::Structure { id, what };
        let (ba, bb) = (
            basis(a).ok_or_else(|| {
                refuse(
                    "an assembly placement whose reference direction is parallel to its \
                     axis — that pair states no frame to place anything in",
                )
            })?,
            basis(b).ok_or_else(|| {
                refuse(
                    "an assembly placement whose reference direction is parallel to its \
                     axis — that pair states no frame to place anything in",
                )
            })?,
        );
        // B ∘ A⁻¹, with A⁻¹ = Aᵀ (A is orthonormal by construction).
        let ai = ba.transpose();
        let linear = bb * ai;
        let translation = (b.0 - Point3::origin()) - linear * (a.0 - Point3::origin());
        let map = Affine3::from_parts(linear, translation);
        // **On the determinant.** ISO 10303-42's `build_axes` makes an
        // `axis2_placement_3d` a RIGHT-HANDED frame whatever its fields
        // say — the third column is the axis, the first is the
        // reference direction projected perpendicular, and the second
        // is their cross product — so `B ∘ A⁻¹` of two of them is a
        // rotation by construction and this check cannot fire on a
        // well-formed pair. It is not therefore decorative: `axis` and
        // `ref_direction` arrive through the ε_in window in
        // [`Resolver::direction`], which adopts a stated near-unit
        // triple verbatim, so a file's slack rides into the columns and
        // the composed determinant is only 1 to within it. This is
        // where that slack is measured against the same budget, before
        // the map reaches a kernel door whose predicates are decided
        // and whose refusal would name nothing in the file.
        let det = linear.determinant();
        if (det - 1.0).abs() > self.eps_in {
            return Err(refuse(
                "an assembly transform whose composed frames are not a rigid motion at \
                 the file's own declared uncertainty — the placement pair states \
                 direction fields too slack to compose into a rotation",
            ));
        }
        Ok(Some(map))
    }
}

/// The file's geometry roots: which representations hold the solids /
/// curve sets, and which contexts those representations name.
#[derive(Debug, Default)]
struct ContentRoots {
    /// Representation instance ids that hold model content.
    reps: std::collections::BTreeSet<u64>,
    /// The contexts those representations name — the units that govern.
    contexts: std::collections::BTreeSet<u64>,
}

/// The scales and uncertainty one file states.
#[derive(Clone, Copy, Debug)]
pub(crate) struct UnitContext {
    /// Meters per file length unit.
    pub(crate) length_scale: f64,
    /// Radians per file angle unit.
    pub(crate) angle_scale: f64,
    /// The declared uncertainty in kernel meters, when the governing
    /// contexts declare one. `None` is not yet an error: a file with
    /// no geometry to import should say *that*, not complain about a
    /// tolerance nothing would have spent.
    pub(crate) uncertainty_m: Option<f64>,
}

/// Units and uncertainty **by resolution** (M7-1 review MAJOR-1): the
/// old presence-scan only fired on instances *containing* an
/// `SI_UNIT` record, so a `CONVERSION_BASED_UNIT` length context (an
/// inch/mm file's normal form) imported silently as metres. Every
/// governing context's `GLOBAL_UNIT_ASSIGNED_CONTEXT` references are
/// resolved and each must be a unit this reader can carry
/// ([`check_unit`]), a length unit must exist among them, and every
/// declared uncertainty must be a `LENGTH_MEASURE` over a resolved
/// length unit.
///
/// **Which contexts govern (M7-4 Leg B).** `governing` is the set of
/// contexts the file's own geometry roots name — the third argument of
/// the representations that actually hold a `MANIFOLD_SOLID_BREP` or
/// `GEOMETRIC_CURVE_SET` ([`content_roots`]). Resolving by that set
/// rather than by a sweep over every unit instance in the file is what
/// the wild forced and what the M7-1 review's own principle asks for:
/// a file may carry a kg/m³ `DERIVED_UNIT` for a mass property, a
/// dimensionless `GEOMETRIC_REPRESENTATION_CONTEXT(2)` per pcurve
/// (145 of them in one measured file), and a second 3D context no
/// representation references — none of which say anything about what
/// a coordinate in the solid MEANS. A sweep refused all three; the
/// geometry's own context answers the only question being asked.
fn resolve_units_and_uncertainty(
    r: &Resolver<'_>,
    governing: &std::collections::BTreeSet<u64>,
) -> Result<UnitContext, StepImportError> {
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
                    found: "a second, different length unit (one file, one length \
                            scale — two make every coordinate ambiguous)"
                        .to_owned(),
                });
            }
        }
        Ok(())
    };
    // The file's ONE angle scale, by the same argument: a semi_angle
    // read in the wrong angular unit is a different cone.
    let mut angle_scale: Option<(u64, f64)> = None;
    let mut note_angle = |id: u64, factor: f64| -> Result<(), StepImportError> {
        match angle_scale {
            None => angle_scale = Some((id, factor)),
            Some((_, prev)) if prev.to_bits() == factor.to_bits() => {}
            Some(_) => {
                return Err(StepImportError::UnsupportedUnit {
                    id,
                    found: "a second, different plane-angle unit (one file, one angle \
                            scale — two make every stated angle ambiguous)"
                        .to_owned(),
                });
            }
        }
        Ok(())
    };
    let mut uncertainty: Option<f64> = None;
    for &id in governing {
        let instance = r.instance(id, id)?;
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
                        match check_unit(r, uid, &unit_instance.records, 0)? {
                            UnitKind::Length(_) => {}
                            UnitKind::Angle(factor) => note_angle(uid, factor)?,
                            UnitKind::SolidAngle => {}
                        }
                        if let Some(factor) = length_factor(r, uid, &unit_instance.records)? {
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
                found: "a representation context without a length unit (a metre, \
                        prefixed or not, or a conversion based on one)"
                    .to_owned(),
            });
        }
    }
    Ok(UnitContext {
        length_scale: length_scale.map_or(1.0, |(_, f)| f),
        angle_scale: angle_scale.map_or(1.0, |(_, f)| f),
        uncertainty_m: uncertainty,
    })
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
        let Some(factor) = length_factor(self, uid, &unit_instance.records)? else {
            return Err(StepImportError::UnsupportedUnit {
                id: uid,
                found: complex_name(&unit_instance.records),
            });
        };
        Ok((as_real(id, value, expected)?, factor, uid))
    }
}

/// The **assembly layer** — resolved into the instance list the
/// materializer works from (M7-2 Leg D → M7-4 Leg D → M8 instancing).
///
/// FreeCAD's `Import.export` path (what GUI users hit) and every AP214
/// assembly translator wrap multi-body content in the same vocabulary:
/// `NEXT_ASSEMBLY_USAGE_OCCURRENCE` links product to component,
/// `CONTEXT_DEPENDENT_SHAPE_REPRESENTATION` links the component's own
/// representation to the assembly root through a complex
/// `( REPRESENTATION_RELATIONSHIP('', '', #rep_1, #rep_2) …
/// REPRESENTATION_RELATIONSHIP_WITH_TRANSFORMATION(#t)
/// SHAPE_REPRESENTATION_RELATIONSHIP() )`, and `#t` is an
/// `ITEM_DEFINED_TRANSFORMATION` naming two placements: the
/// component's own frame and where THIS OCCURRENCE sits in the
/// assembly.
///
/// **The association was always right there.** `rep_1` is the
/// component representation and `rep_2` the assembly root, so the
/// complex record says *which content* the transform places — one
/// occurrence, one map, one component. M7-2 traversed these and
/// accepted only the identity; M7-4 Leg D added the one RIGID map, but
/// insisted it cover ALL of the file's content, because a placed body
/// was placed by transforming the finished body once and there was
/// nowhere to put a second frame. This pass reads the same records the
/// same way and keeps the per-relationship association instead of
/// collapsing it:
///
/// - each relationship contributes **one instance per solid of the
///   representation it names**, carrying that relationship's map;
/// - a component representation named by three occurrences therefore
///   materializes three times, each copy mapped by its own frame
///   ([`SolidInstance`]);
/// - the identity (at ε_in) is carried as `None` — nothing moves, the
///   M7-2 behavior bit for bit — and an identity-placed component is
///   still RECORDED as covered (the coverage bug M8 fixed: the old
///   pass `continue`d past its own bookkeeping, so an identity
///   component read as never placed at all);
/// - a mirror (det = −1) or any scaling refuses typed at
///   [`Resolver::item_defined_transformation`], naming the transform.
///   A mirror is not a placement — it reverses handedness, and which
///   way a mirrored solid's faces then point is a question about the
///   file's intent that an importer must not answer by guessing.
///
/// **Order** is the relationship entity's id, ascending (`file.data`
/// is a `BTreeMap`), and within one relationship the solids in the
/// order [`resolve_shape`] found them — entity-id order end to end,
/// D9's determinism, and what `Shape::Solids` has always documented.
///
/// **What still refuses**: content the assembly does not place at all,
/// beside components it does. A solid the file never places would
/// otherwise ride along at its stated coordinates while its neighbours
/// move — placing only some of a file's content, guessed. And a
/// representation whose content ANOTHER representation already claims
/// (the same `MANIFOLD_SOLID_BREP` named twice, deduplicated at
/// resolution) cannot be placed independently: its map would have
/// nowhere to land, so it refuses rather than being dropped.
fn resolve_instances(
    r: &Resolver<'_>,
    file: &StepFile,
    roots: &ContentRoots,
    owners: &std::collections::BTreeMap<u64, Vec<usize>>,
    solids: usize,
) -> Result<Vec<SolidInstance>, StepImportError> {
    // One edge of the assembly graph, in relationship-entity-id order:
    // (relationship id, rep_1 = the placed representation, rep_2 = the
    // representation it is placed INTO, transform id, the map).
    let mut relationships: Vec<Edge> = Vec::new();
    let mut placed_reps = std::collections::BTreeSet::new();
    for (&id, instance) in &file.data {
        let mut related: Option<(u64, u64)> = None;
        let mut transform: Option<u64> = None;
        for (kw, args) in &instance.records {
            match kw.as_str() {
                "REPRESENTATION_RELATIONSHIP" => {
                    let expected = "REPRESENTATION_RELATIONSHIP(name, description, \
                                    #rep_1, #rep_2)";
                    let [_, _, rep_1, rep_2] = args.as_slice() else {
                        return Err(StepImportError::MalformedRecord { id, expected });
                    };
                    // BOTH ends. `rep_2` is what makes the chain
                    // visible: a nested assembly's inner relationship
                    // places a component into a SUB-ASSEMBLY, and only
                    // `rep_2` says so.
                    related = Some((as_ref(id, rep_1, expected)?, as_ref(id, rep_2, expected)?));
                }
                "REPRESENTATION_RELATIONSHIP_WITH_TRANSFORMATION" => {
                    let expected = "REPRESENTATION_RELATIONSHIP_WITH_TRANSFORMATION(\
                                    #item_defined_transformation)";
                    let [t] = args.as_slice() else {
                        return Err(StepImportError::MalformedRecord { id, expected });
                    };
                    transform = Some(as_ref(id, t, expected)?);
                }
                _ => {}
            }
        }
        let Some(tid) = transform else { continue };
        // A transform that relates nothing places nothing, and reading
        // it as "the whole file" is the guess this pass exists to stop.
        let Some((rep, into)) = related else {
            return Err(StepImportError::MalformedRecord {
                id,
                expected: "a REPRESENTATION_RELATIONSHIP beside the \
                           _WITH_TRANSFORMATION half (the transform names no \
                           representation to place)",
            });
        };
        let map = r.item_defined_transformation(id, tid)?;
        // BEFORE the identity test, not after: an identity-placed
        // component IS covered by the assembly, and recording that is
        // what the stray check below reads.
        placed_reps.insert(rep);
        relationships.push(Edge {
            id,
            rep,
            into,
            tid,
            map,
        });
    }

    // A file whose assembly places nothing — no relationship carries a
    // transform at all — materializes exactly its solids, unplaced, in
    // the order they resolved. This is every own-corpus and most wild
    // files, and it is the same body it has always been.
    if relationships.is_empty() {
        let mut out = Vec::with_capacity(solids);
        for (&rep, mine) in owners {
            for &solid in mine {
                out.push(SolidInstance {
                    solid,
                    component: rep,
                    placed: None,
                });
            }
        }
        // `owners` is keyed by representation id, so the walk above is
        // representation order, not solid order; the model's contract
        // is the SOLIDS' own resolution order (D9, entity-id).
        out.sort_by_key(|i| i.solid);
        debug_assert_eq!(out.len(), solids, "every solid materializes once");
        return Ok(out);
    }
    // `NEXT_ASSEMBLY_USAGE_OCCURRENCE` by relationship, for the A7
    // record: `CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#rr, #pds)` and
    // `PRODUCT_DEFINITION_SHAPE(name, description, #pd)` link the
    // transform-carrying relationship to the product OCCURRENCE it
    // places. This is descriptive metadata — which occurrence, for a
    // later re-adoption as an assembly document — so a file that omits
    // or reshapes the link records `None` rather than refusing: it
    // must never turn an importing file into a refusing one.
    let mut occurrences: std::collections::BTreeMap<u64, u64> = std::collections::BTreeMap::new();
    for (&id, instance) in &file.data {
        let [(kw, args)] = instance.records.as_slice() else {
            continue;
        };
        if kw != "CONTEXT_DEPENDENT_SHAPE_REPRESENTATION" {
            continue;
        }
        let [rr, pds] = args.as_slice() else { continue };
        let expected = "CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#rr, #pds)";
        let (Ok(rr), Ok(pds)) = (as_ref(id, rr, expected), as_ref(id, pds, expected)) else {
            continue;
        };
        let Ok(pds) = r.instance(id, pds) else {
            continue;
        };
        let [(kw, args)] = pds.records.as_slice() else {
            continue;
        };
        if kw != "PRODUCT_DEFINITION_SHAPE" {
            continue;
        }
        let [_, _, pd] = args.as_slice() else {
            continue;
        };
        let Ok(pd) = as_ref(id, pd, expected) else {
            continue;
        };
        if r.instance(id, pd).is_ok_and(|i| {
            i.records
                .iter()
                .any(|(kw, _)| kw == "NEXT_ASSEMBLY_USAGE_OCCURRENCE")
        }) {
            occurrences.insert(rr, pd);
        }
    }

    // Every representation that carries model content must be placed by
    // the assembly; one it never names would ride along at its stated
    // coordinates while its neighbours move.
    if let Some(&stray) = roots.reps.iter().find(|id| !placed_reps.contains(id)) {
        return Err(StepImportError::Structure {
            id: stray,
            what: "a shape representation the assembly's placements do not cover, \
                   beside components they do — placing only some of a file's content \
                   is a guess about the rest, refused",
        });
    }

    // **The chain walk.** An assembly is a GRAPH of representations:
    // each relationship is an edge `rep_1 → rep_2` carrying a map, a
    // component's own relationship places it into whatever holds it,
    // and a NESTED assembly places that holder into something else
    // again. One instance is therefore one PATH from a component
    // representation up to a representation nothing places — and its
    // frame is the composition of that path's maps, outermost last.
    //
    // Reading only the first edge is exactly the silent-wrong-geometry
    // hole this walk exists to close: a sub-assembly's own frame would
    // be dropped and its parts would import at their sub-assembly
    // coordinates, which is a plausible body at the wrong place.
    let mut from: std::collections::BTreeMap<u64, Vec<usize>> = std::collections::BTreeMap::new();
    for (i, e) in relationships.iter().enumerate() {
        from.entry(e.rep).or_default().push(i);
    }
    let mut out = Vec::new();
    let mut used = vec![false; relationships.len()];
    for (start, e) in relationships.iter().enumerate() {
        let mine = owners.get(&e.rep).map_or(&[][..], Vec::as_slice);
        if mine.is_empty() {
            // Not a component's own edge. Either an intermediate one
            // (walked below, as the continuation of some component's
            // path) or the deduplication case, refused next.
            //
            // A content representation whose solids another
            // representation already claims (the same MSB named twice
            // — deduplicated at resolution, so it materializes under
            // the FIRST namer) cannot also be placed here: this map
            // has nowhere to land.
            if roots.reps.contains(&e.rep) {
                return Err(StepImportError::Structure {
                    id: e.id,
                    what: "a placed representation whose solids another representation \
                           already names — one body cannot take two independent \
                           assembly frames, refused rather than silently dropping one",
                });
            }
            continue;
        }
        for path in chains(&relationships, &from, start)? {
            // Compose outermost-last: a point of the component goes
            // through its own frame first, then through each holder's.
            let mut map: Option<Affine3<f64>> = None;
            for &edge in &path {
                used[edge] = true;
                map = match (map, relationships[edge].map) {
                    (m, None) => m,
                    (None, Some(outer)) => Some(outer),
                    (Some(inner), Some(outer)) => Some(compose(&outer, &inner)),
                };
            }
            for &solid in mine {
                out.push(SolidInstance {
                    solid,
                    component: e.rep,
                    placed: Some(Placed {
                        map,
                        transform: e.tid,
                        relationship: e.id,
                        occurrence: occurrences.get(&e.id).copied(),
                    }),
                });
            }
        }
    }
    // Every stated placement must have PLACED something. A relationship
    // no component's chain reaches states a frame that governs no
    // geometry — which is either a file this reader misunderstands or a
    // component it failed to find, and both are refusals rather than a
    // body assembled around a transform nobody applied.
    if let Some(i) = used.iter().position(|u| !u) {
        return Err(StepImportError::Structure {
            id: relationships[i].id,
            what: "an assembly placement no component's placement chain reaches — its \
                   transform would govern no geometry, refused rather than importing \
                   a body around a frame nobody applied",
        });
    }
    Ok(out)
}

/// One edge of the assembly graph (see [`resolve_instances`]).
#[derive(Clone, Copy, Debug)]
struct Edge {
    /// The `REPRESENTATION_RELATIONSHIP` complex's entity id.
    id: u64,
    /// `rep_1` — the representation this edge PLACES.
    rep: u64,
    /// `rep_2` — the representation it is placed into.
    into: u64,
    /// The `ITEM_DEFINED_TRANSFORMATION`'s entity id.
    tid: u64,
    /// The map, `None` at the identity.
    map: Option<Affine3<f64>>,
}

/// Every path from edge `start` up to a representation nothing places,
/// as edge-index lists innermost-first.
///
/// A representation placed into several holders (or a holder itself
/// occurring several times) branches, and each branch is a distinct
/// instance — that IS assembly instancing, one level up. Paths are
/// enumerated in ascending relationship-entity-id order at every
/// branch, so the instance list is entity-id deterministic end to end
/// (D9), exactly as the flat case always was.
///
/// # Errors
///
/// A cycle refuses typed: a representation that (transitively) places
/// itself states no finite set of instances, and unrolling it to some
/// depth would be a guess.
fn chains(
    edges: &[Edge],
    from: &std::collections::BTreeMap<u64, Vec<usize>>,
    start: usize,
) -> Result<Vec<Vec<usize>>, StepImportError> {
    let mut out = Vec::new();
    let mut stack = vec![vec![start]];
    while let Some(path) = stack.pop() {
        // Total by construction (a path is never empty and every index
        // came from `edges`), and written totally anyway — this pass
        // has no panicking door.
        let Some(last) = path.last().and_then(|&i| edges.get(i)).copied() else {
            continue;
        };
        let next = from.get(&last.into).map_or(&[][..], Vec::as_slice);
        if next.is_empty() {
            out.push(path);
            continue;
        }
        // Descending here so the ascending order comes back off the
        // LIFO stack — the branch order is the file's own entity ids.
        for &edge in next.iter().rev() {
            if path.iter().any(|&p| edges[p].rep == edges[edge].rep) {
                return Err(StepImportError::Structure {
                    id: edges[edge].id,
                    what: "an assembly placement chain that returns to a representation \
                           it already places — a representation that places itself states \
                           no finite set of instances, refused rather than unrolled to a \
                           guessed depth",
                });
            }
            let mut branch = path.clone();
            branch.push(edge);
            stack.push(branch);
        }
    }
    // The LIFO above yields paths in reverse branch order; sort back to
    // the file's own order (a path is a list of ascending-id edges, so
    // lexicographic on the edge ids IS entity-id order).
    out.sort_by_key(|p| p.iter().map(|&i| edges[i].id).collect::<Vec<_>>());
    Ok(out)
}

/// `outer ∘ inner` — apply `inner` first. Plain composition of two
/// affine maps; both are rigid by the time they get here (each came
/// through [`Resolver::item_defined_transformation`]'s determinant
/// gate), and a composition of rotations is a rotation, so the kernel's
/// own `transform_rigid` door still has the last word on the result.
fn compose(outer: &Affine3<f64>, inner: &Affine3<f64>) -> Affine3<f64> {
    Affine3::from_parts(
        outer.linear * inner.linear,
        outer.linear * inner.translation + outer.translation,
    )
}

/// Shape content **by resolution** (M7-1 review MINOR-4): solids come
/// from `ADVANCED_BREP_SHAPE_REPRESENTATION` items and the wireframe
/// from `GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION` items —
/// never from a bare keyword scan — and every `MANIFOLD_SOLID_BREP` /
/// `GEOMETRIC_CURVE_SET` in the data section must be referenced by
/// one of them (an orphan is refused rather than guessed to be model
/// content or silently dropped). Mixed solid+wireframe content and a
/// second curve set are outside the subset, refused typed.
fn resolve_shape(
    r: &Resolver<'_>,
    file: &StepFile,
) -> Result<(Shape, std::collections::BTreeMap<u64, Vec<usize>>), StepImportError> {
    let mut solids = Vec::new();
    // Which representation NAMED each solid, as `rep -> solid indices`.
    // The assembly layer places representations, so materialization
    // needs the association resolution already establishes; a solid
    // named twice (a NIST translator writes the same MSB into both an
    // `ADVANCED_BREP_SHAPE_REPRESENTATION` and a plain
    // `SHAPE_REPRESENTATION`) belongs to the FIRST namer in entity-id
    // order — the same one `referenced` already dedupes it to.
    let mut owners: std::collections::BTreeMap<u64, Vec<usize>> = std::collections::BTreeMap::new();
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
                        // One solid, however many representations name
                        // it. A NIST translator writes the same
                        // `MANIFOLD_SOLID_BREP` into both an
                        // `ADVANCED_BREP_SHAPE_REPRESENTATION` and a
                        // plain `SHAPE_REPRESENTATION`; resolving it
                        // per reference imported the part twice, as two
                        // coincident solids — a body no file describes.
                        [(k, sargs)] if k == "MANIFOLD_SOLID_BREP" => {
                            if referenced.insert(item_id) {
                                owners.entry(id).or_default().push(solids.len());
                                solids.push(r.solid(item_id, sargs)?);
                            }
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
        (false, None) => Ok((Shape::Solids(solids), owners)),
        (true, Some((_, curves))) => Ok((Shape::Wireframe(curves), owners)),
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
        angle_scale: 1.0,
        eps_in: 0.0,
        next_id: std::cell::Cell::new(0),
        normalizations: std::cell::RefCell::new(Vec::new()),
        curve_promotions: std::cell::RefCell::new(BTreeMap::new()),
    }
}

/// Resolves the parsed file into a [`Model`] (module docs).
///
/// `eps_override` is [`crate::ImportOptions::eps_in`], plumbed to the
/// resolver so the per-call override governs INTERPRETATION — D7's
/// contract for ε_in (stage-1 surface recognition decides promotion at
/// it; the direction/frame verbatim-adoption windows and the placement
/// rigidity check spend the same budget). The model still carries the
/// file's own declared uncertainty, which is what
/// [`crate::StepImport::eps_in`] reports when no override was given.
pub(crate) fn resolve(
    file: &StepFile,
    eps_override: Option<f64>,
) -> Result<Model, StepImportError> {
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

    // Three passes, in this order by necessity: which contexts govern
    // is a structural question (keywords and reference slots only),
    // its answer says what a coordinate MEANS, and only then does the
    // geometry pass run — on a resolver carrying the file's length and
    // angle scales and its scaled ε_in.
    let roots = content_roots(&r, file)?;
    let units = resolve_units_and_uncertainty(&r, &roots.contexts)?;
    let r = Resolver {
        file,
        length_scale: units.length_scale,
        angle_scale: units.angle_scale,
        // The override wins where given (doc above). A file with no
        // declared uncertainty and no override gets no interpretation
        // budget here; the shape pass below refuses first when there
        // is nothing to import, and `MissingUncertainty` is raised
        // after it, when a body really was on the table.
        eps_in: eps_override.unwrap_or(units.uncertainty_m.unwrap_or(0.0)),
        // Past every stated id, so minted topology cannot collide.
        next_id: std::cell::Cell::new(
            file.data
                .keys()
                .next_back()
                .copied()
                .unwrap_or(0)
                .saturating_add(1),
        ),
        normalizations: std::cell::RefCell::new(Vec::new()),
        curve_promotions: std::cell::RefCell::new(BTreeMap::new()),
    };
    let (shape, owners) = resolve_shape(&r, file)?;
    // The instance list is resolved for a SOLID model only: a
    // wireframe has no body to place, and an assembly of wireframes is
    // not in the subset.
    let instances = match shape {
        Shape::Solids(ref solids) => resolve_instances(&r, file, &roots, &owners, solids.len())?,
        Shape::Wireframe(_) => Vec::new(),
    };
    Ok(Model {
        instances,
        uncertainty_m: units
            .uncertainty_m
            .ok_or(StepImportError::MissingUncertainty)?,
        shape,
        normalizations: r.normalizations.into_inner(),
        curve_promotions: r.curve_promotions.into_inner().into_values().collect(),
    })
}

/// **R1 review probe (PR #264, C2c)** — review branch only: the
/// QUASI_UNIFORM implied-knot synthesis, multi-span shape, must be
/// bit-identical to the stated-knots form (integer spacing, clamped).
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod r1_review_probes {
    use super::quasi_uniform_knots;
    use geom_core::spline::KnotVector;

    #[test]
    fn quasi_uniform_synthesis_matches_stated_integer_knots_bitwise() {
        // degree 2, 5 control points → spans = 3 → [0,0,0,1,2,3,3,3].
        let synth = quasi_uniform_knots(1, 2, 5).unwrap();
        let stated = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0], 2).unwrap();
        assert_eq!(
            format!("{synth:?}"),
            format!("{stated:?}"),
            "synthesized vs stated multi-span knots"
        );
        // dm1's actual shape: degree 1, 2 points → [0,0,1,1].
        let dm1 = quasi_uniform_knots(1, 1, 2).unwrap();
        let dm1_stated = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        assert_eq!(format!("{dm1:?}"), format!("{dm1_stated:?}"));
        // Degenerate: control_points == degree refuses.
        assert!(quasi_uniform_knots(1, 2, 2).is_err());
    }
}
