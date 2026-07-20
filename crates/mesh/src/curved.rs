//! Curved-face tessellation: UV-rectangle domain from the boundary
//! walk, interior grid sampling, CDT in parameter space, pole-fan
//! collapse, per-triangle certificates.
//!
//! Every curved M2 face is a swept UV rectangle (crate docs); the
//! boundary polygon from [`crate::walk`] has bitwise-straight sides,
//! so the domain is convex and no inside/outside classification is
//! needed — every CDT triangle is kept except pole-degenerate ones.
//! Boundary polyline segments are inserted as CDT **constraints**, so
//! the triangulation conforms to the shared chord segments in both
//! adjacent faces (the watertightness guarantee).
//!
//! Pole fans: pole corners enter the CDT at their (distinct) UV
//! locations but map to the single pole mesh vertex; any triangle with
//! two corners of the same mesh vertex is degenerate in 3-D and is
//! dropped, which collapses the strip along the pole side into a fan
//! around the pole (one dropped triangle per collapsed side; its two
//! non-collapsed edges become the identified fan edges — manifoldness
//! is re-checked by the mesh validator).
//!
//! Grid sizing (heuristic; the certificates are the guarantee), from
//! δ_s = δ/2 and φ = [`crate::chords::sagitta_angle`]:
//! cylinder — hu = φ(δ_s, r), no interior rows (ruled in v);
//! cone — hu = φ(δ_s, ρ_max), rows every ρ_max·hu slant meters (ruled
//! in v, but rows keep triangles azimuth-local so the radius-scaled
//! certificate stays tight); sphere — hu = hv = φ(δ_s, r); torus —
//! hu = hv = √(δ_s/(3(R+2r))) (matching the boundary chord
//! tightening in [`crate::chords`]).

use std::collections::HashMap;

use geom_core::Point3;
use geom_surfaces::Surface;
use spade::{ConstrainedDelaunayTriangulation, Point2 as SpadePoint, Triangulation};
use topo::{Body, EdgeKey, FaceKey};

use crate::cert;
use crate::chords::{ceil_count, sagitta_angle, torus_grid_step};
use crate::types::TessellateError;
use crate::walk::{Chart, ChartKind, loop_polygon};

/// The call's tolerance bundle: δ (the promise), δ_s = δ/2 (sizing),
/// and the run's kernel ε (pole identification only — never sizing).
pub(crate) struct Tol {
    /// The chordal tolerance δ.
    pub delta: f64,
    /// The sizing target δ_s = δ/2.
    pub delta_s: f64,
    /// The kernel ε (pole/apex vertex identification).
    pub eps: f64,
}

/// Tessellates one curved face into outward-wound triangles,
/// appending interior grid points to `positions`.
pub(crate) fn tessellate_curved(
    body: &Body<f64>,
    fk: FaceKey,
    surface: &Surface<f64>,
    chords: &HashMap<EdgeKey, Vec<u32>>,
    positions: &mut Vec<Point3<f64>>,
    tol: &Tol,
) -> Result<Vec<[u32; 3]>, TessellateError> {
    let face = body
        .get_face(fk)
        .ok_or(TessellateError::MissingEntity { what: "face" })?;
    if !face.rings.is_empty() {
        return Err(TessellateError::RingOnCurvedFace { face: fk });
    }
    let chart = Chart::of(surface).ok_or(TessellateError::MissingEntity {
        what: "curved chart",
    })?;
    let polygon = loop_polygon(body, &chart, chords, positions, fk, face.outer, tol.eps)?;
    if polygon.len() < 3 {
        return Err(TessellateError::MissingEntity {
            what: "degenerate curved boundary",
        });
    }

    // Domain bbox and orientation.
    let (mut u0, mut u1, mut v0, mut v1) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
    let mut area2 = 0.0;
    for (i, e) in polygon.iter().enumerate() {
        u0 = u0.min(e.u);
        u1 = u1.max(e.u);
        v0 = v0.min(e.v);
        v1 = v1.max(e.v);
        let n = &polygon[(i + 1) % polygon.len()];
        area2 += e.u * n.v - n.u * e.v;
    }
    let flip = area2 < 0.0;

    // Grid steps per kind (module docs).
    let (nu, nv) = grid_steps(
        &chart,
        tol.delta_s,
        u1 - u0,
        v1 - v0,
        v0.abs().max(v1.abs()),
    )?;

    // CDT: boundary entries (fixed walk order) + constraints + grid.
    let mut cdt: ConstrainedDelaunayTriangulation<SpadePoint<f64>> =
        ConstrainedDelaunayTriangulation::new();
    // Per-CDT-vertex metadata, indexed by handle index (insertion order).
    let mut meta: Vec<(f64, f64, u32, bool)> = Vec::new();
    let insert = |cdt: &mut ConstrainedDelaunayTriangulation<SpadePoint<f64>>,
                  meta: &mut Vec<(f64, f64, u32, bool)>,
                  u: f64,
                  v: f64,
                  id: u32,
                  pole: bool|
     -> Result<spade::handles::FixedVertexHandle, TessellateError> {
        let h = cdt
            .insert(SpadePoint::new(u, v))
            .map_err(|_| TessellateError::Triangulation { face: fk })?;
        if h.index() == meta.len() {
            meta.push((u, v, id, pole));
        }
        Ok(h)
    };
    let mut handles = Vec::with_capacity(polygon.len());
    for e in &polygon {
        handles.push(insert(&mut cdt, &mut meta, e.u, e.v, e.id, e.pole)?);
    }
    for i in 0..handles.len() {
        let (a, b) = (handles[i], handles[(i + 1) % handles.len()]);
        if a != b && !cdt.exists_constraint(a, b) {
            // `add_constraint` panics on crossing constraints (corrupt
            // input geometry) — pre-check to keep failure typed.
            if !cdt.can_add_constraint(a, b) {
                return Err(TessellateError::Triangulation { face: fk });
            }
            cdt.add_constraint(a, b);
        }
    }
    let (uspan, vspan) = (u1 - u0, v1 - v0);
    for j in 1..nv {
        #[allow(clippy::cast_precision_loss)]
        let v = v0 + vspan * (j as f64 / nv as f64);
        for i in 1..nu {
            #[allow(clippy::cast_precision_loss)]
            let u = u0 + uspan * (i as f64 / nu as f64);
            let p = surface.eval(u, v);
            #[allow(clippy::cast_possible_truncation)]
            let id = positions.len() as u32;
            let h = insert(&mut cdt, &mut meta, u, v, id, false)?;
            if h.index() == meta.len() - 1 && meta[meta.len() - 1].2 == id {
                positions.push(p);
            }
        }
    }

    // Emit: drop pole-degenerate triangles, certify the rest.
    let mut triangles = Vec::new();
    let mut worst: f64 = 0.0;
    for f in cdt.inner_faces() {
        let vs = f.vertices();
        let m: Vec<(f64, f64, u32, bool)> = vs.iter().map(|v| meta[v.fix().index()]).collect();
        let ids = [m[0].2, m[1].2, m[2].2];
        if ids[0] == ids[1] || ids[1] == ids[2] || ids[0] == ids[2] {
            continue; // pole-collapsed sliver
        }
        let tri = [
            positions[ids[0] as usize],
            positions[ids[1] as usize],
            positions[ids[2] as usize],
        ];
        let uv = [[m[0].0, m[0].1], [m[1].0, m[1].1], [m[2].0, m[2].1]];
        let pole = [m[0].3, m[1].3, m[2].3];
        let bound = match chart.kind {
            ChartKind::Cylinder { r } => cert::cert_cylinder(chart.anchor, chart.axis, r, tri),
            ChartKind::Sphere { r } => cert::cert_sphere(chart.anchor, r, tri),
            ChartKind::Cone { half_angle } => cert::cert_cone(half_angle, uv, pole),
            ChartKind::Torus { major, minor } => cert::cert_torus(major, minor, uv),
        };
        // Sticky-NaN accumulation: `f64::max` would silently drop a
        // poisoned bound.
        if bound.is_nan() || worst.is_nan() || bound > worst {
            worst = bound;
        }
        triangles.push(if flip { [ids[0], ids[2], ids[1]] } else { ids });
    }
    if worst.is_nan() || worst > tol.delta {
        return Err(TessellateError::CertificateExceeded {
            face: fk,
            bound: worst,
            requested: tol.delta,
        });
    }
    Ok(triangles)
}

/// Interior grid step counts (nu, nv) for the face's UV spans.
fn grid_steps(
    chart: &Chart,
    delta_s: f64,
    uspan: f64,
    vspan: f64,
    v_absmax: f64,
) -> Result<(usize, usize), TessellateError> {
    let cap = core::f64::consts::FRAC_PI_4;
    match chart.kind {
        ChartKind::Cylinder { r } => {
            let hu = sagitta_angle(delta_s, r);
            Ok((ceil_count(uspan, hu)?, 1))
        }
        ChartKind::Cone { half_angle } => {
            let rho_max = v_absmax * half_angle.sin();
            let hu = sagitta_angle(delta_s, rho_max);
            let hv = rho_max * hu;
            Ok((ceil_count(uspan, hu)?, ceil_count(vspan, hv)?))
        }
        ChartKind::Sphere { r } => {
            let h = sagitta_angle(delta_s, r);
            Ok((ceil_count(uspan, h)?, ceil_count(vspan, h)?))
        }
        ChartKind::Torus { major, minor } => {
            let h = torus_grid_step(delta_s, major, minor).min(cap);
            Ok((ceil_count(uspan, h)?, ceil_count(vspan, h)?))
        }
    }
}
