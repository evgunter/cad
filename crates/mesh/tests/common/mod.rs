//! Shared helpers for the tessellation acceptance suites (M2 PR 6):
//! body builders (through public profile/sweep APIs only), the
//! full-mesh acceptance check, exact surface distances, and the D9
//! byte-identity dump.
#![allow(dead_code)]
// one instance per binary; no single consumer uses all of it
// Why a helper tree allows these: `crates/editor-core/tests/fixture/mod.rs`.
#![allow(clippy::unwrap_used, clippy::expect_used)]
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza

/// The Euler-door witnesses the iso-rectangle shape door is measured
/// on, shared with `curved`'s in-crate rows (its own header says how).
pub mod witness_bodies;

use geom::Surface;
use geom_core::Tol;
use geom_core::{Point2, Point3, Vec2};
use mesh::validate::{check_mesh, signed_volume};
use mesh::{Mesh, tessellate};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::Body;

/// The run's ε as a bare `f64` — **the suites' door, and the reason
/// `crates/mesh`'s test half still spells ε that way at all.**
///
/// Production code cannot: `sizing::Eps` carries the four named
/// operations every shipped ε read goes through, and it is
/// `pub(crate)` — a mesh-local fact about mesh's own terminal reads,
/// not public API. `tests/` is a separate crate, so it cannot name
/// the type, and publishing it to serve test suites would grow the
/// crate's API surface for readers who are not consumers of it. What
/// the suites do with the band is also not the kind of read the type
/// is about: an acceptance slack (`delta + eps()`), fixture placement
/// at a chosen multiple of it (`0.9 * eps`, `5.0 * eps`), a bar on a
/// reported certificate, and one message that prints it — sizing or
/// narrating a probe, not deciding a mesh.
///
/// **"The door" is a claim about this tree and it is checkable: every
/// suite that reads ε calls this function, with one stated
/// exception.** `r1_probe_bool_route` mints its own inline because it
/// carries no `mod common;` and reaches ε only to print the ambient
/// value — pulling the whole helper module into that suite to save
/// one line would cost more than it buys. Every other reader — the
/// three acceptance-slack rows, `budget_meter`, `m7_nurbs_trimmed`,
/// `issue896_pole_guard`, `r2_bool_door`, `r2_split_door` and
/// `fitted_refusals`' message — comes through here.
pub fn eps() -> f64 {
    Tol::witness().get().eps
}

/// A one-loop, four-line quad SECTION for the loft/sweep doors
/// (LIB-U3 profile vocabulary) — the crate's single copy of the
/// formerly per-file `quad()` clones.
pub fn quad(pts: [(f64, f64); 4]) -> sweep::Section {
    vec![ProfileLoop::polygon(
        pts.iter().map(|&(x, y)| Point2::new(x, y)),
    )]
}

pub fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

pub fn validated(loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .unwrap()
}

pub fn axis_y() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    }
}

// ---- bodies -----------------------------------------------------------

/// L-prism: L-shaped hexagon (area 3) extruded to height 1.
pub fn l_prism() -> Body<f64> {
    let lp = ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(2.0, 0.0),
        p2(2.0, 1.0),
        p2(1.0, 1.0),
        p2(1.0, 2.0),
        p2(0.0, 2.0),
    ]);
    extrude(
        &validated(vec![lp]),
        Extrusion::Distance(1.0),
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// Holed prism: 3×3 square with a centered 1×1 square hole, height 1.
pub fn holed_prism() -> Body<f64> {
    let outer = ProfileLoop::polygon([p2(0.0, 0.0), p2(3.0, 0.0), p2(3.0, 3.0), p2(0.0, 3.0)]);
    let hole = ProfileLoop::polygon([p2(1.0, 1.0), p2(2.0, 1.0), p2(2.0, 2.0), p2(1.0, 2.0)]);
    extrude(
        &validated(vec![outer, hole]),
        Extrusion::Distance(1.0),
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// Rounded-square prism: 2×2 square, corners rounded at radius
/// 0.5 (quarter-circle bulges tan(π/8)), height 1.
pub fn rounded_prism() -> Body<f64> {
    let b = (core::f64::consts::FRAC_PI_8).tan();
    let r = 0.5;
    let v = |pos: Point2<f64>, bulge: f64| ProfileVertex::new(pos, bulge);
    let mut lp = ProfileLoop::new(vec![
        v(p2(r, 0.0), 0.0),
        v(p2(2.0 - r, 0.0), b),
        v(p2(2.0, r), 0.0),
        v(p2(2.0, 2.0 - r), b),
        v(p2(2.0 - r, 2.0), 0.0),
        v(p2(r, 2.0), b),
        v(p2(0.0, 2.0 - r), 0.0),
        v(p2(0.0, r), b),
    ]);
    // Every joint is an exact quarter-arc/side tangency -- declared
    // (the #101 discipline).
    let n = lp.vertices().len();
    lp = lp.with_tangent_joints((0..n).collect());
    extrude(
        &validated(vec![lp]),
        Extrusion::Distance(1.0),
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// The ball: unit half-disc revolved fully (two-band sphere, poles).
pub fn ball() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -1.0), 1.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// The cone: right triangle (base radius 1, height 1) revolved fully
/// (apex fan + base disc).
pub fn cone() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.0, 1.0)]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// A PARTIAL revolve of the cone profile — base radius `s`, height 1,
/// swept `theta` — so the cone face carries the apex as a chart
/// singularity AND has a bounded azimuth span. Issue #678's shape:
/// both variables that reach `nu == 2` (a tiny `s`, which saturates
/// the sagitta cap at every δ, and a narrow `theta`) live here.
pub fn cone_wedge(s: f64, theta: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(s, 0.0), p2(0.0, 1.0)]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(theta),
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// A PARTIAL revolve of the unit half-disc — the ball's profile swept
/// `theta` — so the sphere face carries BOTH poles as chart
/// singularities and has a bounded azimuth span. The sphere twin of
/// [`cone_wedge`], and the shape that reaches `curved::pole_columns`'s
/// sphere arm.
pub fn sphere_wedge(theta: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -1.0), 1.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(theta),
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// The washer: rectangle [1,2]×[0,1] revolved fully (genus 1, slit
/// annuli + full-2π cylinder walls).
pub fn washer() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// The donut: circle of radius 0.5 centered at (2, 0) (two arc
/// segments) revolved fully — a single torus surface, both meridians
/// `Seam`.
pub fn donut() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(2.0, -0.5), 1.0),
        ProfileVertex::new(p2(2.0, 0.5), 1.0),
    ]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// Partial wedge: rectangle [1,2]×[0,1] revolved by +π/2.
pub fn wedge() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// Axis-touching partial wedge: unit square [0,1]×[0,1] revolved by
/// +π/2 (the axis edge is an ordinary boundary edge shared by the two
/// wedge caps).
pub fn axis_wedge() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap()
    .body
}

// ---- oracles ----------------------------------------------------------

/// Exact distance from a point to the complete analytic locus.
pub fn dist_to_surface(surface: &Surface<f64>, p: Point3<f64>) -> f64 {
    match *surface {
        Surface::Plane { origin, normal, .. } => (p - origin).dot(normal).abs(),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let w = p - origin;
            let rho = (w - axis * w.dot(axis)).norm();
            (rho - radius).abs()
        }
        Surface::Sphere { center, radius, .. } => ((p - center).norm() - radius).abs(),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let w = p - apex;
            let h = w.dot(axis);
            let rho = (w - axis * h).norm();
            let (s, c) = half_angle.sin_cos();
            // Both nappes; fall back to the apex where no foot exists.
            let mut best = f64::INFINITY;
            for hh in [h, -h] {
                if rho * s + hh * c >= 0.0 {
                    best = best.min((rho * c - hh * s).abs());
                }
            }
            best.min(w.norm())
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let w = p - center;
            let h = w.dot(axis);
            let rho = (w - axis * h).norm();
            let d_circle = ((rho - major_radius).powi(2) + h * h).sqrt();
            (d_circle - minor_radius).abs()
        }
        // No closed-form distance oracle for a spline locus or a fit.
        Surface::Nurbs(_) | Surface::Approx(_) => f64::NAN,
    }
}

/// A byte-comparable dump (positions as exact bits + full index/key
/// structure) — the D9 rebuild oracle.
pub fn dump(mesh: &Mesh) -> String {
    let mut s = String::new();
    for p in &mesh.positions {
        s.push_str(&format!(
            "{:016x} {:016x} {:016x}\n",
            p.x.to_bits(),
            p.y.to_bits(),
            p.z.to_bits()
        ));
    }
    for patch in &mesh.patches {
        s.push_str(&format!("{:?} {:?}\n", patch.face, patch.triangles));
    }
    for b in &mesh.boundaries {
        s.push_str(&format!(
            "{:?} {:?} {:?} {:?}\n",
            b.edge, b.points, b.start_vertex, b.end_vertex
        ));
    }
    s
}

/// Runs the full acceptance battery on one body at one δ and returns
/// the mesh: watertight + consistently wound, positive volume (within
/// the certified band of `exact` = (volume, area) when given), the
/// chordal bound by dense sampling, back-reference integrity, boundary
/// conformity, and D9 byte-identical rebuild.
pub fn check_mesh_acceptance(body: &Body<f64>, delta: f64, exact: Option<(f64, f64)>) -> Mesh {
    let mesh = tessellate(body, delta, Tol::witness()).unwrap();

    // (i) watertight, manifold, consistently wound.
    assert_eq!(check_mesh(&mesh), Ok(()));

    // (ii) orientation: positive signed volume, within the implied band.
    let v = signed_volume(&mesh);
    assert!(v > 0.0, "signed volume {v} not positive");
    if let Some((v_exact, a_exact)) = exact {
        let band = 2.0 * a_exact * (delta + eps());
        assert!(
            (v - v_exact).abs() <= band,
            "volume {v} vs exact {v_exact} outside band {band} at delta {delta}"
        );
    }

    // (iii) chordal bound: dense barycentric sampling of every
    // triangle against its face's exact surface.
    let slack = delta + eps() + 1e-9;
    for patch in &mesh.patches {
        let face = body.get_face(patch.face).expect("patch face is live");
        let surface = body.get_surface(face.surface).unwrap();
        for tri in &patch.triangles {
            let [a, b, c] = [
                mesh.positions[tri[0] as usize],
                mesh.positions[tri[1] as usize],
                mesh.positions[tri[2] as usize],
            ];
            for i in 0..=4u32 {
                for j in 0..=(4 - i) {
                    let k = 4 - i - j;
                    let (li, lj, lk) = (f64::from(i) / 4.0, f64::from(j) / 4.0, f64::from(k) / 4.0);
                    let p = Point3::origin()
                        + (a - Point3::origin()) * li
                        + (b - Point3::origin()) * lj
                        + (c - Point3::origin()) * lk;
                    let d = dist_to_surface(surface, p);
                    assert!(
                        d <= slack,
                        "chordal violation {d} > {slack} on face {:?}",
                        patch.face
                    );
                }
            }
        }
    }

    // (iv) back-reference integrity: boundary polylines match topology.
    let mut seen_edges = std::collections::HashSet::new();
    for bp in &mesh.boundaries {
        let edge = body.get_edge(bp.edge).expect("polyline edge is live");
        assert!(seen_edges.insert(bp.edge), "edge listed twice");
        assert!(bp.points.len() >= 2);
        let he = body.get_half_edge(edge.he_plus).unwrap();
        assert_eq!(he.start, bp.start_vertex);
        assert_eq!(body.half_edge_end(edge.he_plus).unwrap(), bp.end_vertex);
        for (vk, idx) in [
            (bp.start_vertex, bp.points[0]),
            (bp.end_vertex, *bp.points.last().unwrap()),
        ] {
            let vp = *body.get_point(body.get_vertex(vk).unwrap().point).unwrap();
            let mp = mesh.positions[idx as usize];
            assert_eq!(
                (vp.x.to_bits(), vp.y.to_bits(), vp.z.to_bits()),
                (mp.x.to_bits(), mp.y.to_bits(), mp.z.to_bits()),
                "polyline endpoint is not the vertex point bitwise"
            );
        }
    }
    assert_eq!(mesh.boundaries.len(), body.edges().count());
    assert_eq!(mesh.patches.len(), body.faces().count());

    // Boundary conformity: every polyline segment is an edge of
    // exactly two triangles (the watertight seam between two patches).
    let mut edge_uses: std::collections::HashMap<(u32, u32), u32> =
        std::collections::HashMap::new();
    for patch in &mesh.patches {
        for tri in &patch.triangles {
            for k in 0..3 {
                let (x, y) = (tri[k], tri[(k + 1) % 3]);
                *edge_uses.entry((x.min(y), x.max(y))).or_insert(0) += 1;
            }
        }
    }
    for bp in &mesh.boundaries {
        for w in bp.points.windows(2) {
            let key = (w[0].min(w[1]), w[0].max(w[1]));
            assert_eq!(
                edge_uses.get(&key),
                Some(&2),
                "boundary segment {key:?} of {:?} not shared by exactly 2 triangles",
                bp.edge
            );
        }
    }

    // (v) determinism: byte-identical rebuild.
    let again = tessellate(body, delta, Tol::witness()).unwrap();
    assert_eq!(dump(&mesh), dump(&again), "rebuild not byte-identical");

    mesh
}
