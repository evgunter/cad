//! **Whether a result's sphere faces are maximal over the cuts its
//! operands wrote**: every edge with one sphere on both sides lies on a
//! circle an operand edge on that sphere lies on (its seam meridians, a
//! rim the author cut), so no cut the kernel made survives between two
//! pieces of one face. A reader over stored carriers that evaluates no
//! surface, so it routes here.

use topo::{Body, EdgeKey, Surface};

/// The plane `(point, normal)` of every circle an edge of `bodies`
/// carries.
fn edge_planes(bodies: &[&Body<f64>]) -> Vec<(geom_core::Point3<f64>, geom_core::Vec3<f64>)> {
    bodies
        .iter()
        .flat_map(|b| b.edges().map(move |(e, _)| (*b, e)))
        .filter_map(|(b, e)| match carrier(b, e)? {
            geom::Curve3::Circle(c) => Some((geom::Curve3::Circle(c).eval(0.0), c.axis())),
            _ => None,
        })
        .collect()
}

fn carrier(body: &Body<f64>, e: EdgeKey) -> Option<geom::Curve3<f64>> {
    let curve = body.get_edge(e)?.curve;
    Some(body.get_curve_geom(curve)?.certified()?.carrier().clone())
}

/// The points of `e` at its ends and its middle.
fn edge_points(body: &Body<f64>, e: EdgeKey) -> Vec<geom_core::Point3<f64>> {
    let Some(c) = body
        .get_edge(e)
        .and_then(|d| body.get_curve_geom(d.curve))
        .and_then(|g| g.certified())
    else {
        return Vec::new();
    };
    let (t0, t1) = c.params();
    [t0, 0.5 * (t0 + t1), t1]
        .into_iter()
        .map(|t| c.carrier().eval(t))
        .collect()
}

/// **Every edge of `result` with one sphere on both sides lies on a
/// circle an edge of `operands` carries**, `label` naming the failure.
pub fn assert_spheres_maximal(label: &str, result: &Body<f64>, operands: &[&Body<f64>]) {
    let planes = edge_planes(operands);
    let sphere = |e: EdgeKey, plus: bool| {
        let d = result.get_edge(e)?;
        let h = result.get_half_edge(if plus { d.he_plus } else { d.he_minus })?;
        let f = result.get_loop(h.parent_loop)?.face;
        match result.face_surface(f) {
            Ok(Surface::Sphere { frame, radius, .. }) => {
                let c = frame.origin();
                Some(([c.x, c.y, c.z], *radius))
            }
            _ => None,
        }
    };
    for (e, _) in result.edges() {
        let (Some(p), Some(m)) = (sphere(e, true), sphere(e, false)) else {
            continue;
        };
        let same = (0..3).all(|i| (p.0[i] - m.0[i]).abs() < 1e-9) && (p.1 - m.1).abs() < 1e-9;
        if !same {
            continue;
        }
        let points = edge_points(result, e);
        let written = planes
            .iter()
            .any(|&(q, n)| points.iter().all(|&x| (x - q).dot(n).abs() < 1e-7));
        assert!(
            written,
            "{label}: edge {e:?} between two pieces of one sphere lies on no circle its \
             operands wrote: a cut the kernel made survives"
        );
    }
}
