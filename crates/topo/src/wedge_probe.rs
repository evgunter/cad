//! TEMPORARY measurement probe (not committed).
use crate::body::Body;
use geom_core::{Tol, Vec3};

fn arc(b: &Body<f64>, e: crate::EdgeKey, u: Vec3<f64>, e1: Vec3<f64>) -> Option<(f64, f64, f64)> {
    let e2 = u.cross(e1);
    let d = b.get_edge(e)?;
    let mut dirs = Vec::new();
    for he in [d.he_plus, d.he_minus] {
        let f = b.face_of_half_edge(he)?;
        let face = b.get_face(f)?;
        let geom::Surface::Plane { normal, .. } = *b.get_surface(face.surface)? else { return None };
        let n = if face.sense { normal } else { -normal };
        let p = b.half_edge_start_point(he)?;
        let q = b.get_point(b.get_vertex(b.half_edge_end(he)?)?.point).copied()?;
        let t = (q - p).normalize();
        let dir = n.cross(t);
        dirs.push((dir, n));
    }
    let ang = |v: Vec3<f64>| { let a = v.dot(e2).atan2(v.dot(e1)); if a < 0.0 { a + std::f64::consts::TAU } else { a } };
    let (d1, n1) = dirs[0];
    let (d2, _) = dirs[1];
    let (a, bb) = (ang(d1), ang(d2));
    let s = (-n1).dot(u.cross(d1));
    let tau = std::f64::consts::TAU;
    Some(if s > 0.0 { (a, (bb - a).rem_euclid(tau), s) } else { (bb, (a - bb).rem_euclid(tau), s) })
}

pub fn report(b: &Body<f64>) -> String {
    let Ok(lines) = b.coincident_lines(Tol::witness()) else { return "undecided".into() };
    let mut out = String::new();
    for l in lines {
        let [ea, eb] = l.edges;
        let d = b.get_edge(ea).unwrap();
        let p = b.half_edge_start_point(d.he_plus).unwrap();
        let q = b.get_point(b.get_vertex(b.half_edge_end(d.he_plus).unwrap()).unwrap().point).copied().unwrap();
        let u = (q - p).normalize();
        let f = b.face_of_half_edge(d.he_plus).unwrap();
        let face = b.get_face(f).unwrap();
        let Some(&geom::Surface::Plane { normal, .. }) = b.get_surface(face.surface) else { out += "curved; "; continue };
        let n = if face.sense { normal } else { -normal };
        let e1 = n.cross(u).normalize();
        match (arc(b, ea, u, e1), arc(b, eb, u, e1)) {
            (Some((a0, aw, _)), Some((b0, bw, _))) => {
                let tau = std::f64::consts::TAU;
                let inside = |x: f64, s: f64, w: f64| (x - s).rem_euclid(tau) < w;
                let n = 7200;
                let ov = (0..n).filter(|&i| { let x = (i as f64 + 0.5) * tau / n as f64; inside(x, a0, aw) && inside(x, b0, bw) }).count() as f64 * 360.0 / n as f64;
                out += &format!("[{ea:?} {:.1}° | {eb:?} {:.1}° overlap {ov:.1}° from {:?} to {:?}] ", aw.to_degrees(), bw.to_degrees(), l.ends[0], l.ends[1]);
            }
            _ => out += "curved face; ",
        }
    }
    out
}

thread_local! {
    pub static PROBE_LINE: std::cell::RefCell<Option<[[f64; 3]; 2]>> = const { std::cell::RefCell::new(None) };
}

pub fn line_report(b: &Body<f64>) -> String {
    let Some([p, q]) = PROBE_LINE.with(|l| l.borrow_mut().take()) else { return String::new() };
    let p = geom_core::Point3::new(p[0], p[1], p[2]);
    let q = geom_core::Point3::new(q[0], q[1], q[2]);
    let u = (q - p).normalize();
    let on = |x: geom_core::Point3<f64>| (x - p).cross(u).norm() < 1e-9;
    let mut out = format!("LINE {p:?}->{q:?}: ");
    let mut e1 = None;
    for (k, d) in b.edges() {
        let a = b.half_edge_start_point(d.he_plus).unwrap();
        let c = b.get_point(b.get_vertex(b.half_edge_end(d.he_plus).unwrap()).unwrap().point).copied().unwrap();
        if !(on(a) && on(c)) { continue; }
        let f = b.face_of_half_edge(d.he_plus).unwrap();
        let face = b.get_face(f).unwrap();
        let Some(&geom::Surface::Plane { normal, .. }) = b.get_surface(face.surface) else { out += "curved "; continue };
        let n = if face.sense { normal } else { -normal };
        let basis = *e1.get_or_insert_with(|| { let w = n.cross(u); if w.norm() > 1e-9 { w.normalize() } else { w } });
        let (s, w, _) = arc(b, k, u, basis).unwrap_or((f64::NAN, f64::NAN, 0.0));
        out += &format!("[{k:?} z {:.3}..{:.3} from {:.1}° width {:.1}°] ", a.z, c.z, s.to_degrees(), w.to_degrees());
    }
    out
}
