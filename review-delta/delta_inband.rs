//! Delta review of PR 4026: an independent in-band census of a built
//! body, by plain segment geometry (no kernel predicate): every vertex
//! pair, vertex-edge pair and edge-edge pair closer than the band
//! (`eps·k`), each tagged with whether a recorded vv contact names it.
#![allow(dead_code, clippy::unwrap_used)]

use geom_core::Tol;
use topo::{Body, ContactRecords, VertexKey};

type V3 = [f64; 3];
fn sub(a: V3, b: V3) -> V3 { [a[0] - b[0], a[1] - b[1], a[2] - b[2]] }
fn dot(a: V3, b: V3) -> f64 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }
fn norm(a: V3) -> f64 { dot(a, a).sqrt() }

pub fn point_segment(p: V3, (s0, s1): (V3, V3)) -> (f64, f64) {
    let d = sub(s1, s0);
    let r = sub(p, s0);
    let t = (dot(r, d) / dot(d, d)).clamp(0.0, 1.0);
    (norm([0, 1, 2].map(|k| r[k] - d[k] * t)), t)
}

/// Least distance between segments, and the parameters (sa, sb).
pub fn segment_segment(a: (V3, V3), b: (V3, V3)) -> (f64, f64, f64) {
    // dense robust: minimise over a's param by golden search is overkill;
    // use the closed form plus the four endpoint candidates.
    let (d1, d2, r) = (sub(a.1, a.0), sub(b.1, b.0), sub(a.0, b.0));
    let (aa, ee, f) = (dot(d1, d1), dot(d2, d2), dot(d2, r));
    let (c, bb) = (dot(d1, r), dot(d1, d2));
    let denom = aa * ee - bb * bb;
    let mut cands = Vec::new();
    if denom > 1e-300 {
        let sc = ((bb * f - c * ee) / denom).clamp(0.0, 1.0);
        let tc = ((bb * sc + f) / ee).clamp(0.0, 1.0);
        let sc2 = ((bb * tc - c) / aa).clamp(0.0, 1.0);
        cands.push((sc2, tc));
    }
    for (p, s_is_a, s) in [(a.0, true, 0.0), (a.1, true, 1.0), (b.0, false, 0.0), (b.1, false, 1.0)] {
        if s_is_a {
            let (_, t) = point_segment(p, b);
            cands.push((s, t));
        } else {
            let (_, t) = point_segment(p, a);
            cands.push((t, s));
        }
    }
    let mut best = (f64::INFINITY, 0.0, 0.0);
    for (s, t) in cands {
        let pa = [0, 1, 2].map(|k| a.0[k] + d1[k] * s);
        let pb = [0, 1, 2].map(|k| b.0[k] + d2[k] * t);
        let g = norm(sub(pa, pb));
        if g < best.0 {
            best = (g, s, t);
        }
    }
    best
}

pub fn band(tol: Tol) -> f64 {
    tol.eps() * tol.get().k
}

/// Every in-band pair of the body, as text lines. `recorded` holds the
/// result's vv contacts (vertex keys in result keys).
pub fn inband(body: &Body<f64>, contacts: &ContactRecords, tol: Tol) -> Vec<String> {
    // Report out to three bands, each line with its gap, so pairs just
    // outside the escalate edge show too.
    let reach = 3.0 * band(tol);
    let band = band(tol);
    let pt = |k: VertexKey| {
        let p = topo::readback::vertex_point_ref(body, k).unwrap();
        [p.x, p.y, p.z]
    };
    let verts: Vec<(VertexKey, V3)> = body.vertex_points().map(|(k, _)| (k, pt(k))).collect();
    let edges: Vec<(VertexKey, VertexKey)> = body
        .edges()
        .map(|(_, ed)| {
            (
                body.get_half_edge(ed.he_plus).unwrap().start,
                body.get_half_edge(ed.he_minus).unwrap().start,
            )
        })
        .collect();
    let rec = |a: VertexKey, b: VertexKey| {
        contacts.vv.iter().any(|c| (c.a, c.b) == (a, b) || (c.a, c.b) == (b, a))
    };
    let same_point = |a: VertexKey, b: VertexKey| {
        body.get_vertex(a).unwrap().point == body.get_vertex(b).unwrap().point
    };
    let mut out = Vec::new();
    for (i, &(ka, pa)) in verts.iter().enumerate() {
        for &(kb, pb) in &verts[i + 1..] {
            let g = norm(sub(pa, pb));
            if g < reach && !same_point(ka, kb) {
                out.push(format!(
                    "VV gap={g:.3e} recorded={} same_point={} at {pa:?}",
                    rec(ka, kb),
                    same_point(ka, kb)
                ));
            }
        }
    }
    for &(kv, p) in &verts {
        for &(e0, e1) in &edges {
            if kv == e0 || kv == e1 {
                continue;
            }
            let s = (pt(e0), pt(e1));
            let (g, _) = point_segment(p, s);
            let ends = norm(sub(p, s.0)).min(norm(sub(p, s.1)));
            if g < reach && ends >= reach {
                out.push(format!("VE gap={g:.3e} at {p:?}"));
            }
        }
    }
    for (i, &(a0, a1)) in edges.iter().enumerate() {
        for &(b0, b1) in &edges[i + 1..] {
            let (sa, sb) = ((pt(a0), pt(a1)), (pt(b0), pt(b1)));
            let shares = [a0, a1].iter().any(|k| *k == b0 || *k == b1);
            if shares {
                // far-end fold: the shorter edge's far end against the longer.
                let (la, lb) = (norm(sub(sa.1, sa.0)), norm(sub(sb.1, sb.0)));
                let (short, sk, long, ok) = if la <= lb { (sa, (a0, a1), sb, [b0, b1]) } else { (sb, (b0, b1), sa, [a0, a1]) };
                let sharedk = if ok.contains(&sk.0) { sk.0 } else { sk.1 };
                let far = if sharedk == sk.0 { short.1 } else { short.0 };
                let (g, _) = point_segment(far, long);
                if g < reach {
                    out.push(format!("EE-FOLD gap={g:.3e} short={short:?} long={long:?}"));
                }
                continue;
            }
            let (g, s, t) = segment_segment(sa, sb);
            if g < reach {
                let la = norm(sub(sa.1, sa.0));
                let lb = norm(sub(sb.1, sb.0));
                let interior = s * la > band && (1.0 - s) * la > band && t * lb > band && (1.0 - t) * lb > band;
                // overlap length: sample the shorter edge for in-band stretch
                let (short, long) = if la <= lb { (sa, sb) } else { (sb, sa) };
                let n = 2000;
                let mut inb = 0;
                for q in 0..=n {
                    let u = f64::from(q) / f64::from(n);
                    let p = [0, 1, 2].map(|k| short.0[k] + (short.1[k] - short.0[k]) * u);
                    if point_segment(p, long).0 < reach {
                        inb += 1;
                    }
                }
                let olen = la.min(lb) * f64::from(inb) / f64::from(n + 1);
                if !interior && inb < 3 {
                    continue;
                }
                out.push(format!(
                    "EE gap={g:.3e} interior={interior} samples={inb} inband_len~{olen:.3e} a={sa:?} b={sb:?}"
                ));
            }
        }
    }
    out
}

/// Faces running through two distinct vertices on one point.
pub fn face_two_copies(body: &Body<f64>) -> Vec<String> {
    let mut out = Vec::new();
    for (face, f) in body.faces() {
        let mut met: Vec<VertexKey> = Vec::new();
        for &l in std::iter::once(&f.outer).chain(&f.rings) {
            if let topo::LoopBoundary::Cycle { first } = body.get_loop(l).unwrap().boundary {
                for he in body.loop_cycle(first).unwrap() {
                    let v = body.get_half_edge(he).unwrap().start;
                    if !met.contains(&v) {
                        met.push(v);
                    }
                }
            }
        }
        for (i, &a) in met.iter().enumerate() {
            for &b in &met[i + 1..] {
                if body.get_vertex(a).unwrap().point == body.get_vertex(b).unwrap().point {
                    out.push(format!("FACE2V face {face:?} meets two vertices on one point"));
                }
            }
        }
    }
    out
}

/// Every independent finding on a built result (none for a refusal).
pub fn ind_lines(
    r: &Result<topo::BooleanResult<f64>, topo::BooleanError>,
    tol: Tol,
) -> Vec<String> {
    match r {
        Ok(res) => match res.body() {
            Some(bb) => {
                let mut v = inband(&bb.body, &bb.contacts, tol);
                v.extend(face_two_copies(&bb.body));
                v
            }
            None => Vec::new(),
        },
        Err(_) => Vec::new(),
    }
}
