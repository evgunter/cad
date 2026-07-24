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

use geom_core::{Point3, Tolerance, Vec3};
use geom_curves::Curve3;
use geom_surfaces::Surface;
use topo::{Body, CurveGeom, Edge, EdgeKey, FaceKey, LoopBoundary, LoopKey, ShellKey, VertexKey};

use crate::real::fmt_real;
use crate::volume::shell_signed_volume;
use crate::{SharedIds, StepExportError, StepOptions, quoted};

/// The surface variant's name, for typed refusals.
pub(crate) fn surface_kind(surface: &Surface<f64>) -> &'static str {
    match surface {
        Surface::Plane { .. } => "plane",
        Surface::Cylinder { .. } => "cylinder",
        Surface::Cone { .. } => "cone",
        Surface::Sphere { .. } => "sphere",
        Surface::Torus { .. } => "torus",
        Surface::Nurbs => "nurbs placeholder",
    }
}

/// The curve variant's name, for typed refusals.
pub(crate) fn carrier_kind(carrier: &Curve3<f64>) -> &'static str {
    match carrier {
        Curve3::Line { .. } => "line",
        Curve3::Circle { .. } => "circle",
        Curve3::Nurbs => "nurbs placeholder",
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

    /// `VERTEX_POINT` for a kernel vertex (first encounter mints, later
    /// encounters reuse — vertices are shared by every adjacent edge).
    fn vertex_point(&mut self, vertex: VertexKey) -> Result<u64, StepExportError> {
        if let Some(&id) = self.shared.vertex_points.get(&vertex) {
            return Ok(id);
        }
        let v = self.body.get_vertex(vertex).ok_or(StepExportError::Corrupt {
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
        let edge = self.body.get_edge(edge_key).ok_or(StepExportError::Corrupt {
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
        // The analytic-subset curve printers. M5 adds arms here
        // (circle → CIRCLE, NURBS → B_SPLINE_CURVE_WITH_KNOTS);
        // everything around this match is carrier-agnostic.
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
            ref other => {
                return Err(StepExportError::UnsupportedCurve {
                    edge: edge_key,
                    kind: carrier_kind(other),
                });
            }
        };
        let id = self.emit(&format!("EDGE_CURVE('', #{start}, #{end}, #{curve}, .T.)"));
        self.shared.edge_curves.insert(edge_key, id);
        Ok(id)
    }

    /// `EDGE_LOOP` of `ORIENTED_EDGE`s for one loop, wrapped as
    /// `FACE_OUTER_BOUND` (outer) or `FACE_BOUND` (ring), both with
    /// orientation `.T.`: the stored traversal direction is already
    /// STEP's (interior-left ⇒ outer CCW / rings CW about the outward
    /// normal — crate docs).
    fn face_bound(&mut self, loop_key: LoopKey, outer: bool) -> Result<u64, StepExportError> {
        let loop_ = self.body.get_loop(loop_key).ok_or(StepExportError::Corrupt {
            what: "loop key does not resolve",
        })?;
        let first = match loop_.boundary {
            LoopBoundary::Empty { .. } => {
                return Err(StepExportError::EmptyLoop { loop_: loop_key });
            }
            LoopBoundary::Cycle { first } => first,
        };
        let cycle = self.body.loop_cycle(first).ok_or(StepExportError::Corrupt {
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
            let edge = self.body.get_edge(edge_key).ok_or(StepExportError::Corrupt {
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
        let kind = if outer { "FACE_OUTER_BOUND" } else { "FACE_BOUND" };
        Ok(self.emit(&format!("{kind}('', #{el}, .T.)")))
    }

    /// `ADVANCED_FACE` for a kernel face: bounds (outer first, rings in
    /// stored order), the surface printer, `same_sense = .T.` (the
    /// stored normal IS the outward normal — M1 ratification).
    fn advanced_face(&mut self, face_key: FaceKey) -> Result<u64, StepExportError> {
        let face = self.body.get_face(face_key).ok_or(StepExportError::Corrupt {
            what: "face key does not resolve",
        })?;
        let outer = face.outer;
        let rings = face.rings.clone();
        let mut bounds = Vec::with_capacity(1 + rings.len());
        bounds.push(self.face_bound(outer, true)?);
        for ring in rings {
            bounds.push(self.face_bound(ring, false)?);
        }
        let surface = self
            .body
            .get_surface(face.surface)
            .ok_or(StepExportError::Corrupt {
                what: "face surface key does not resolve",
            })?;
        // The analytic-subset surface printers. M5 adds arms here
        // (CYLINDRICAL_SURFACE, CONICAL_SURFACE, SPHERICAL_SURFACE,
        // TOROIDAL_SURFACE, B_SPLINE_SURFACE_WITH_KNOTS) — exporting
        // analytic carriers AS analytic entities is this writer's
        // reason to exist (crate docs).
        let surface_id = match *surface {
            Surface::Plane {
                origin,
                normal,
                u_ref,
            } => {
                let cp = self.cartesian_point(origin, "plane origin")?;
                let axis = self.direction(normal, "plane normal")?;
                let ref_dir = self.direction(u_ref, "plane u_ref")?;
                let placement =
                    self.emit(&format!("AXIS2_PLACEMENT_3D('', #{cp}, #{axis}, #{ref_dir})"));
                self.emit(&format!("PLANE('', #{placement})"))
            }
            ref other => {
                return Err(StepExportError::UnsupportedSurface {
                    face: face_key,
                    kind: surface_kind(other),
                });
            }
        };
        Ok(self.emit(&format!(
            "ADVANCED_FACE('', ({}), #{surface_id}, .T.)",
            refs(&bounds)
        )))
    }

    /// `CLOSED_SHELL` over the shell's faces (stored order).
    fn closed_shell(&mut self, shell_key: ShellKey) -> Result<u64, StepExportError> {
        let shell = self.body.get_shell(shell_key).ok_or(StepExportError::Corrupt {
            what: "shell key does not resolve",
        })?;
        let face_keys = shell.faces.clone();
        let mut faces = Vec::with_capacity(face_keys.len());
        for face_key in face_keys {
            faces.push(self.advanced_face(face_key)?);
        }
        Ok(self.emit(&format!("CLOSED_SHELL('', ({}))", refs(&faces))))
    }

    /// One `MANIFOLD_SOLID_BREP` per shell of every solid, with the
    /// multi-shell outward/void classification (crate docs: positive
    /// shells are independent solids, negative shells refuse).
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
                    let shell =
                        self.body
                            .get_shell(shell_key)
                            .ok_or(StepExportError::Corrupt {
                                what: "solid shell key does not resolve",
                            })?;
                    let volume = shell_signed_volume(self.body, shell)?;
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
        None => Tolerance::get().eps,
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
    let app =
        w.emit("APPLICATION_CONTEXT('core data for automotive mechanical design processes')");
    w.emit(&format!(
        "APPLICATION_PROTOCOL_DEFINITION('international standard', 'automotive_design', \
         2000, #{app})"
    ));
    let pctx = w.emit(&format!("PRODUCT_CONTEXT('', #{app}, 'mechanical')"));
    let prod = w.emit(&format!("PRODUCT({name}, {name}, '', (#{pctx}))"));
    let pdf = w.emit(&format!("PRODUCT_DEFINITION_FORMATION('', '', #{prod})"));
    let pdc = w.emit(&format!("PRODUCT_DEFINITION_CONTEXT('part definition', #{app}, 'design')"));
    let pd = w.emit(&format!("PRODUCT_DEFINITION('design', '', #{pdf}, #{pdc})"));
    let pds = w.emit(&format!("PRODUCT_DEFINITION_SHAPE('', '', #{pd})"));
    w.emit(&format!("SHAPE_DEFINITION_REPRESENTATION(#{pds}, #{absr})"));
    w.emit(&format!("PRODUCT_RELATED_PRODUCT_CATEGORY('part', $, (#{prod}))"));

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
