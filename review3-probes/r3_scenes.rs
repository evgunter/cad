// review-3 probe scenes for PR 4256 (shared by the topo and editor-core probes
// via include!). Needs in scope: Pose, MEET, PLATE, posed_box, posed_pyramid,
// AtRestBody, Tol, union, subtract, BooleanResult.

/// A germ at MEET (local frame).
#[derive(Clone, Debug)]
pub enum G {
    Half,
    All,
    Cone(Vec<[f64; 3]>),
    Fan(Vec<[f64; 3]>),
    U(Box<G>, Box<G>),
    D(Box<G>, Box<G>),
}

fn r3_solve3(m: [[f64; 3]; 3], d: [f64; 3]) -> [f64; 3] {
    let det = |a: [f64; 3], b: [f64; 3], c: [f64; 3]| {
        a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])
            + a[2] * (b[0] * c[1] - b[1] * c[0])
    };
    let d0 = det(m[0], m[1], m[2]);
    [det(d, m[1], m[2]) / d0, det(m[0], d, m[2]) / d0, det(m[0], m[1], d) / d0]
}

impl G {
    pub fn has(&self, d: [f64; 3]) -> bool {
        match self {
            G::Half => d[2] < 0.0,
            G::All => true,
            G::Cone(b) => (1..b.len() - 1).any(|i| {
                let l = r3_solve3([b[0], b[i], b[i + 1]], d);
                l.iter().all(|&x| x > 0.0)
            }),
            G::Fan(b) => {
                let n = b.len();
                let (p0, p1, p2) = (b[0], b[1], b[2]);
                let u = [0, 1, 2].map(|k| p1[k] - p0[k]);
                let v = [0, 1, 2].map(|k| p2[k] - p0[k]);
                let nn = [
                    u[1] * v[2] - u[2] * v[1],
                    u[2] * v[0] - u[0] * v[2],
                    u[0] * v[1] - u[1] * v[0],
                ];
                let num: f64 = (0..3).map(|k| nn[k] * p0[k]).sum();
                let den: f64 = (0..3).map(|k| nn[k] * d[k]).sum();
                if den.abs() < 1e-300 || num / den <= 0.0 {
                    return false;
                }
                let s = num / den;
                let q = d.map(|x| x * s);
                let ax = (0..3).max_by(|&i, &j| nn[i].abs().total_cmp(&nn[j].abs())).unwrap();
                let (i, j) = match ax {
                    0 => (1, 2),
                    1 => (2, 0),
                    _ => (0, 1),
                };
                let mut inside = false;
                for k in 0..n {
                    let (a, c) = (b[k], b[(k + 1) % n]);
                    if (a[j] > q[j]) != (c[j] > q[j])
                        && q[i] < (c[i] - a[i]) * (q[j] - a[j]) / (c[j] - a[j]) + a[i]
                    {
                        inside = !inside;
                    }
                }
                inside
            }
            G::U(a, b) => a.has(d) || b.has(d),
            G::D(a, b) => a.has(d) && !b.has(d),
        }
    }
    /// 0 In, 1 On, 2 Out
    pub fn side(&self, d: [f64; 3]) -> u8 {
        let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        let d = d.map(|x| x / l);
        let (mut i, mut o) = (0, 0);
        for k in 0..400 {
            let z = 1.0 - (2.0 * f64::from(k) + 1.0) / 400.0;
            let (s, c) = (2.399_963 * f64::from(k)).sin_cos();
            let r = z.mul_add(-z, 1.0).sqrt();
            let u = [r * c, r * s, z];
            let q = [0, 1, 2].map(|j| d[j] + 1e-7 * u[j]);
            if self.has(q) { i += 1 } else { o += 1 }
        }
        match (i, o) {
            (_, 0) => 0,
            (0, _) => 2,
            _ => 1,
        }
    }
}

fn gu(a: G, b: G) -> G { G::U(Box::new(a), Box::new(b)) }
fn gd(a: G, b: G) -> G { G::D(Box::new(a), Box::new(b)) }

fn r3_corners(bearing: f64, rise: f64, r: f64) -> [[f64; 3]; 3] {
    let corner = |d: f64, r: f64| {
        let (s, c) = (bearing + d).to_radians().sin_cos();
        [r * c, r * s, rise]
    };
    [corner(15.0, r), corner(-15.0, r), corner(0.0, 0.6 * r)]
}
fn r3_mixn(base: &[[f64; 3]], w: &[Vec<f64>], s: f64) -> Vec<[f64; 3]> {
    w.iter()
        .map(|w| [0, 1, 2].map(|k| s * (0..base.len()).map(|i| w[i] * base[i][k]).sum::<f64>()))
        .collect()
}
fn r3_nest(base: &[[f64; 3]], s: f64) -> Vec<[f64; 3]> {
    let n = base.len();
    let w: Vec<Vec<f64>> = (0..n)
        .map(|i| (0..n).map(|j| if i == j { 0.6 } else { 0.4 / (n as f64 - 1.0) }).collect())
        .collect();
    r3_mixn(base, &w, s)
}
fn r3_pol(deg: f64, r: f64, z: f64) -> [f64; 3] {
    let (s, k) = deg.to_radians().sin_cos();
    [r * k, r * s, z]
}
fn r3_apex(base: &[[f64; 3]], pose: &Pose, tol: Tol) -> AtRestBody<f64> {
    let n = base.len();
    let (mut normal, mut centre) = ([0.0; 3], [0.0; 3]);
    for i in 0..n {
        let (a, b) = (base[i], base[(i + 1) % n]);
        normal[0] += (a[1] - b[1]) * (a[2] + b[2]);
        normal[1] += (a[2] - b[2]) * (a[0] + b[0]);
        normal[2] += (a[0] - b[0]) * (a[1] + b[1]);
        for k in 0..3 {
            centre[k] += a[k] / n as f64;
        }
    }
    let mut at: Vec<[f64; 3]> = base.iter().map(|q| [0, 1, 2].map(|k| MEET[k] + q[k])).collect();
    if (0..3).map(|k| normal[k] * centre[k]).sum::<f64>() > 0.0 {
        at.reverse();
    }
    posed_pyramid(&at, MEET, pose, tol)
}

/// A deterministic random pyramid base (3 corners), apex at MEET.
fn r3_random_bases(seed: u64, n: usize) -> Vec<Vec<[f64; 3]>> {
    let mut s = seed;
    let mut rnd = move || {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((s >> 11) as f64) / ((1u64 << 53) as f64)
    };
    let mut out = Vec::new();
    while out.len() < n {
        // an axis, then three corners around it at half-angle h
        let z = 2.0 * rnd() - 1.0;
        let ph = 6.283185307 * rnd();
        let r = (1.0 - z * z).sqrt();
        let a = [r * ph.cos(), r * ph.sin(), z];
        let h = 0.15 + 0.6 * rnd();
        let t0 = 6.283185307 * rnd();
        // a frame
        let up = if a[2].abs() < 0.9 { [0.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0] };
        let cr = |x: [f64; 3], y: [f64; 3]| [x[1] * y[2] - x[2] * y[1], x[2] * y[0] - x[0] * y[2], x[0] * y[1] - x[1] * y[0]];
        let nz = |x: [f64; 3]| { let l = (x[0]*x[0]+x[1]*x[1]+x[2]*x[2]).sqrt(); x.map(|v| v / l) };
        let e1 = nz(cr(a, up));
        let e2 = cr(a, e1);
        let len = 0.2 + 0.2 * rnd();
        let b: Vec<[f64; 3]> = (0..3)
            .map(|k| {
                let t = t0 + 2.0944 * k as f64 + 0.4 * (rnd() - 0.5);
                let (s1, c1) = t.sin_cos();
                [0, 1, 2].map(|j| len * (a[j] + h * (c1 * e1[j] + s1 * e2[j])))
            })
            .collect();
        out.push(b);
    }
    out
}

fn r3_apex_try(base: &[[f64; 3]], pose: &Pose, tol: Tol, skipped: &mut Vec<String>, what: &str) -> Option<AtRestBody<f64>> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| r3_apex(base, pose, tol))) {
        Ok(b) => Some(b),
        Err(_) => { skipped.push(format!("{what}: pyramid not built")); None }
    }
}

pub struct Scene {
    pub label: String,
    pub y: AtRestBody<f64>,
    pub yg: G,
    pub probes: Vec<(String, AtRestBody<f64>, G)>,
}

fn r3_try(what: &str, r: Result<BooleanResult<f64>, topo::BooleanError>, skipped: &mut Vec<String>) -> Option<AtRestBody<f64>> {
    match r {
        Ok(BooleanResult::Body(r)) => Some(r.body),
        other => {
            skipped.push(format!("{what}: {:?}", other.map(|_| ())));
            None
        }
    }
}

/// Every scene at `pose`. `random` random probes per scene.
pub fn r3_scenes(pose: &Pose, tol: Tol, random: usize, skipped: &mut Vec<String>) -> Vec<Scene> {
    let p = |b: &[[f64; 3]]| r3_apex(b, pose, tol);
    let cone = |b: &[[f64; 3]]| G::Cone(b.to_vec());
    let fan = |b: &[[f64; 3]]| G::Fan(b.to_vec());
    let plate_b = posed_box("the plate", PLATE, pose, tol);
    let block_b = posed_box("block", [(1.0, 2.0), (0.5, 1.5), (0.3, 1.5)], pose, tol);
    let arch = r3_corners(60.0, 0.5, 0.4).to_vec();
    let void = r3_corners(120.0, -0.5, 0.4).to_vec();
    let mut out: Vec<Scene> = Vec::new();
    macro_rules! tr { ($w:expr, $e:expr) => { match r3_try($w, $e, skipped) { Some(b) => b, None => return out } } }
    macro_rules! trs { ($w:expr, $e:expr) => { r3_try($w, $e, skipped) } }
    let one_b = tr!("one", union(&plate_b, &p(&arch), tol));
    let one = gu(G::Half, cone(&arch));
    let hvoid = r3_nest(&arch, 0.7);
    let hollow_b = trs!("hollow", subtract(&one_b, &p(&hvoid), tol));
    let hollow = gd(one.clone(), cone(&hvoid));
    let hisle = r3_nest(&hvoid, 0.7);
    let deep_b = hollow_b.as_ref().and_then(|h| trs!("deep", union(h, &p(&hisle), tol)));
    let deep = gu(hollow.clone(), cone(&hisle));
    // depth 4 over the plate: a void in the island in the void in the arch
    let hv2 = r3_nest(&hisle, 0.7);
    let mut ys: Vec<(String, AtRestBody<f64>, G, Vec<Vec<[f64; 3]>>)> = Vec::new();
    // nests probes: a pyramid strictly inside each listed cone
    if let Some(deep4_b) = deep_b.as_ref().and_then(|d| trs!("deep4", subtract(d, &p(&hv2), tol))) {
        ys.push(("depth 4 over the plate".into(), deep4_b, gd(deep.clone(), cone(&hv2)), vec![r3_nest(&hv2, 0.7), r3_nest(&hisle, 1.3), hisle.clone()]));
    }
    if let Some(d) = &deep_b { ys.push(("deep (depth 3)".into(), d.clone(), deep.clone(), vec![r3_nest(&hisle, 0.7), r3_nest(&hvoid, 1.2)])); }
    if let Some(h) = &hollow_b { ys.push(("hollow".into(), h.clone(), hollow.clone(), vec![r3_nest(&hvoid, 0.7)])); }
    // 4-face void apex in the plate, and in the arch
    let void4 = vec![r3_pol(100.0, 0.45, -0.5), r3_pol(140.0, 0.45, -0.5), r3_pol(140.0, 0.25, -0.5), r3_pol(100.0, 0.25, -0.5)];
    if let Some(b) = trs!("cavity4", subtract(&plate_b, &p(&void4), tol)) {
        ys.push(("quad void in the plate".into(), b, gd(G::Half, cone(&void4)), vec![r3_nest(&void4, 1.4), r3_nest(&void4, 0.7)]));
    }
    let arch4 = vec![r3_pol(40.0, 0.45, 0.5), r3_pol(80.0, 0.45, 0.5), r3_pol(80.0, 0.25, 0.5), r3_pol(40.0, 0.25, 0.5)];
    let hvoid4 = r3_nest(&arch4, 0.7);
    if let Some(one4) = trs!("one4", union(&plate_b, &p(&arch4), tol)) {
        if let Some(b) = trs!("hollow4", subtract(&one4, &p(&hvoid4), tol)) {
            ys.push(("quad void in a quad arch".into(), b.clone(), gd(gu(G::Half, cone(&arch4)), cone(&hvoid4)), vec![r3_nest(&hvoid4, 0.7)]));
            let isle4 = r3_nest(&hvoid4, 0.7);
            if let Some(b2) = trs!("deep4q", union(&b, &p(&isle4), tol)) {
                ys.push(("quad island, quad void, quad arch".into(), b2, gu(gd(gu(G::Half, cone(&arch4)), cone(&hvoid4)), cone(&isle4)), vec![r3_nest(&isle4, 0.7), r3_nest(&hvoid4, 0.7)]));
            }
        }
    }
    // bare quad arch with a quad void (pure pairs)
    if let Some(b) = trs!("bare4", subtract(&p(&arch4), &p(&hvoid4), tol)) {
        ys.push(("bare quad arch, quad void".into(), b, gd(cone(&arch4), cone(&hvoid4)), vec![r3_nest(&hvoid4, 0.7)]));
    }
    // 5-face void buried in a block, an island in it (pure pairs)
    let void5: Vec<[f64; 3]> = (0..5).map(|k| r3_pol(120.0 + 72.0 * k as f64, 0.3, -0.45)).collect();
    if let Some(b) = trs!("buried5", subtract(&block_b, &p(&void5), tol)) {
        ys.push(("pentagonal void buried".into(), b.clone(), gd(G::All, cone(&void5)), vec![r3_nest(&void5, 0.7)]));
        let isle5 = r3_nest(&void5, 0.7);
        if let Some(b2) = trs!("buried5i", union(&b, &p(&isle5), tol)) {
            ys.push(("pentagonal void buried, island".into(), b2, gu(gd(G::All, cone(&void5)), cone(&isle5)), vec![r3_nest(&isle5, 0.7), r3_nest(&void5, 0.7)]));
        }
    }
    // near-flat edge on a quad void: c1, c2, c3 nearly collinear in the base.
    for (tag, dl) in [("1e-3", 1e-3), ("1e-6", 1e-6), ("1e-8", 1e-8), ("1e-10", 1e-10), ("-1e-8", -1e-8), ("-1e-3", -1e-3)] {
        let c1 = r3_pol(100.0, 0.45, -0.5);
        let c3 = r3_pol(140.0, 0.45, -0.5);
        let mid = [0, 1, 2].map(|k| 0.5 * (c1[k] + c3[k]));
        // outward (away from the base's centre, bearing 120) is +radial
        let rad = r3_pol(120.0, 1.0, 0.0);
        let c2 = [mid[0] + dl * rad[0], mid[1] + dl * rad[1], -0.5];
        let c0 = r3_pol(120.0, 0.15, -0.5);
        // c0 near the centre: base is c0, c1, c2, c3 — convex where dl > 0
        let q = vec![c0, c1, c2, c3];
        let g = if dl > 0.0 { cone(&q) } else { fan(&q) };
        let Some(qp) = r3_apex_try(&q, pose, tol, skipped, &format!("flat {tag}")) else { continue };
        if let Some(b) = trs!(&format!("flat {tag} plate"), subtract(&plate_b, &qp, tol)) {
            ys.push((format!("near-flat quad void {tag} in the plate"), b, gd(G::Half, g.clone()), vec![r3_nest(&q, 1.4), r3_nest(&q, 0.7)]));
        }
        if let Some(b) = trs!(&format!("flat {tag} buried"), subtract(&block_b, &qp, tol)) {
            let isle = r3_nest(&q, 0.6);
            ys.push((format!("near-flat quad void {tag} buried"), b.clone(), gd(G::All, g.clone()), vec![r3_nest(&q, 0.7)]));
            if let Some(b2) = trs!(&format!("flat {tag} buried isle"), union(&b, &p(&isle), tol)) {
                ys.push((format!("near-flat quad void {tag} buried, island"), b2, gu(gd(G::All, g.clone()), cone(&isle)), vec![r3_nest(&isle, 0.7), r3_nest(&q, 0.7)]));
            }
        }
    }
    // a dart void (saddle complement): in the plate, and buried with an island
    let dart = vec![r3_pol(100.0, 0.45, -0.5), r3_pol(120.0, 0.6, -0.5), r3_pol(140.0, 0.45, -0.5), r3_pol(120.0, 0.5, -0.5)];
    if let Some(b) = trs!("dart void plate", subtract(&plate_b, &p(&dart), tol)) {
        ys.push(("dart void in the plate".into(), b, gd(G::Half, fan(&dart)), vec![]));
    }
    if let Some(b) = trs!("dart void buried", subtract(&block_b, &p(&dart), tol)) {
        ys.push(("dart void buried".into(), b, gd(G::All, fan(&dart)), vec![]));
    }
    // two voids in one arch
    let v1 = r3_mixn(&arch, &[vec![0.8, 0.1, 0.1], vec![0.6, 0.3, 0.1], vec![0.6, 0.1, 0.3]], 0.7);
    let v2 = r3_mixn(&arch, &[vec![0.1, 0.8, 0.1], vec![0.3, 0.6, 0.1], vec![0.1, 0.6, 0.3]], 0.7);
    if let Some(b) = trs!("two voids", subtract(&one_b, &p(&v1), tol)) {
        if let Some(b2) = trs!("two voids b", subtract(&b, &p(&v2), tol)) {
            let g = gd(gd(one.clone(), cone(&v1)), cone(&v2));
            ys.push(("two voids in one arch".into(), b2.clone(), g.clone(), vec![r3_nest(&v1, 0.7), r3_nest(&v2, 0.7), r3_mixn(&arch, &[vec![0.6, 0.3, 0.1], vec![0.3, 0.6, 0.1], vec![0.45, 0.45, 0.1]], 0.5)]));
            let i1 = r3_nest(&v1, 0.7);
            if let Some(b3) = trs!("two voids isle", union(&b2, &p(&i1), tol)) {
                ys.push(("two voids in one arch, island in one".into(), b3, gu(g, cone(&i1)), vec![r3_nest(&i1, 0.7), r3_nest(&v2, 0.7)]));
            }
        }
    }
    // bare arch with two voids (pure pairs)
    if let Some(b) = trs!("bare two voids", subtract(&p(&arch), &p(&v1), tol)) {
        if let Some(b2) = trs!("bare two voids b", subtract(&b, &p(&v2), tol)) {
            ys.push(("bare arch, two voids".into(), b2, gd(gd(cone(&arch), cone(&v1)), cone(&v2)), vec![r3_nest(&v1, 0.7), r3_nest(&v2, 0.7)]));
        }
    }
    // voids beside each other in a block, an island in one (pure pairs)
    let w1 = r3_corners(30.0, 0.0, 0.4).iter().map(|c| [c[0], c[1], -0.3]).collect::<Vec<_>>();
    let w2 = r3_corners(210.0, -0.4, 0.4).to_vec();
    if let Some(b) = trs!("block w1", subtract(&block_b, &p(&w1), tol)) {
        if let Some(b2) = trs!("block w2", subtract(&b, &p(&w2), tol)) {
            let g = gd(gd(G::All, cone(&w1)), cone(&w2));
            ys.push(("two voids apart, buried".into(), b2.clone(), g.clone(), vec![r3_nest(&w1, 0.7), r3_nest(&w2, 0.7)]));
            let i1 = r3_nest(&w1, 0.7);
            if let Some(b3) = trs!("block w isle", union(&b2, &p(&i1), tol)) {
                ys.push(("two voids apart buried, island in one".into(), b3.clone(), gu(g.clone(), cone(&i1)), vec![r3_nest(&i1, 0.7), r3_nest(&w2, 0.7)]));
                let vi = r3_nest(&i1, 0.7);
                if let Some(b4) = trs!("block w isle void", subtract(&b3, &p(&vi), tol)) {
                    ys.push(("buried depth 3 beside a void".into(), b4, gd(gu(g, cone(&i1)), cone(&vi)), vec![r3_nest(&vi, 0.7)]));
                }
            }
        }
    }
    // void whose cone has a ray on the arch's face
    let vf = r3_mixn(&arch, &[vec![0.5, 0.5, 0.0], vec![0.6, 0.2, 0.2], vec![0.2, 0.6, 0.2]], 0.7);
    if let Some(b) = trs!("void on arch face", subtract(&one_b, &p(&vf), tol)) {
        ys.push(("void with a ray on the arch's face".into(), b, gd(one.clone(), cone(&vf)), vec![r3_nest(&vf, 0.7)]));
    }
    // void with a ray in the plate's top
    let vt = vec![r3_pol(100.0, 0.45, 0.0), r3_pol(140.0, 0.45, -0.5), r3_pol(120.0, 0.25, -0.5)];
    if let Some(b) = trs!("void on top", subtract(&plate_b, &p(&vt), tol)) {
        ys.push(("void with a ray in the top".into(), b, gd(G::Half, cone(&vt)), vec![r3_nest(&vt, 0.7)]));
    }
    ys.push(("one".into(), one_b.clone(), one.clone(), vec![]));
    if let Some(b) = trs!("both", subtract(&one_b, &p(&void), tol)) {
        ys.push(("both".into(), b, gd(one.clone(), cone(&void)), vec![r3_nest(&void, 1.4)]));
    }
    if let Some(b) = trs!("bare hollow", subtract(&p(&arch), &p(&hvoid), tol)) {
        ys.push(("bare hollow".into(), b, gd(cone(&arch), cone(&hvoid)), vec![r3_nest(&hvoid, 0.7)]));
    }
    let bvoid = void.clone();
    if let Some(b) = trs!("buried", subtract(&block_b, &p(&bvoid), tol)) {
        let bi = r3_nest(&bvoid, 0.7);
        if let Some(b2) = trs!("buried island", union(&b, &p(&bi), tol)) {
            ys.push(("buried island".into(), b2, gu(gd(G::All, cone(&bvoid)), cone(&bi)), vec![r3_nest(&bi, 0.7)]));
        }
    }
    // fallback witnesses: a convex partner beside an unread dart
    if std::env::var("R3_FALLBACK").is_ok() {
        ys.clear();
        skipped.clear();
        let darta = vec![r3_pol(200.0, 0.45, 0.5), r3_pol(230.0, 0.6, 0.5), r3_pol(260.0, 0.45, 0.5), r3_pol(230.0, 0.5, 0.5)];
        let in_darta: Vec<[f64; 3]> = [r3_pol(222.0, 0.5, 0.5), r3_pol(238.0, 0.5, 0.5), r3_pol(230.0, 0.56, 0.5)].iter().map(|c| c.map(|x| 0.8 * x)).collect();
        if let Some(b) = trs!("arch and dart", union(&p(&arch), &p(&darta), tol)) {
            ys.push(("bare arch and a bare dart, apart".into(), b, gu(cone(&arch), fan(&darta)), vec![in_darta.clone(), r3_nest(&arch, 0.7)]));
        }
        let dartv = vec![r3_pol(100.0, 0.45, -0.45), r3_pol(120.0, 0.6, -0.45), r3_pol(140.0, 0.45, -0.45), r3_pol(120.0, 0.5, -0.45)];
        let isl: Vec<[f64; 3]> = [r3_pol(112.0, 0.5, -0.45), r3_pol(128.0, 0.5, -0.45), r3_pol(120.0, 0.56, -0.45)].iter().map(|c| c.map(|x| 0.7 * x)).collect();
        let in_isl = r3_nest(&isl, 0.7);
        if let Some(b) = trs!("buried dart", subtract(&block_b, &p(&dartv), tol)) {
            if let Some(b2) = trs!("buried dart isle", union(&b, &p(&isl), tol)) {
                ys.push(("buried dart void, convex island".into(), b2, gu(gd(G::All, fan(&dartv)), cone(&isl)), vec![in_isl.clone(), [r3_pol(105.0, 0.47, -0.45), r3_pol(110.0, 0.5, -0.45), r3_pol(108.0, 0.46, -0.45)].iter().map(|c| c.map(|x| 0.9 * x)).collect()]));
            }
        }
        // a dart arch with a convex void inside its convex part
        let dvoid: Vec<[f64; 3]> = [r3_pol(222.0, 0.5, 0.5), r3_pol(238.0, 0.5, 0.5), r3_pol(230.0, 0.56, 0.5)].iter().map(|c| c.map(|x| 0.7 * x)).collect();
        if let Some(b) = trs!("dart hollow", subtract(&p(&darta), &p(&dvoid), tol)) {
            ys.push(("bare dart arch, convex void".into(), b, gd(fan(&darta), cone(&dvoid)), vec![r3_nest(&dvoid, 0.7), [r3_pol(205.0, 0.47, 0.5), r3_pol(210.0, 0.5, 0.5), r3_pol(208.0, 0.46, 0.5)].iter().map(|c| c.map(|x| 0.9 * x)).collect()]));
        }
    }
    // the island in the void: oracle scene, for the diff
    let isle = r3_nest(&void, 0.7);
    if std::env::var("R3_FALLBACK").is_err() { if let Some(cav) = trs!("cavity", subtract(&plate_b, &p(&void), tol)) {
        ys.push(("cavity".into(), cav.clone(), gd(G::Half, cone(&void)), vec![r3_nest(&void, 1.4)]));
        if let Some(b) = trs!("island", union(&cav, &p(&isle), tol)) {
            ys.push(("island in the cavity".into(), b, gu(gd(G::Half, cone(&void)), cone(&isle)), vec![r3_nest(&isle, 0.7)]));
        }
    } }
    let std: Vec<(String, Vec<[f64; 3]>)> = vec![
        ("cone".into(), r3_corners(240.0, 0.7, 0.5).to_vec()),
        ("over".into(), r3_corners(50.0, 0.7, 0.5).to_vec()),
        ("hang".into(), r3_corners(240.0, -0.6, 0.5).to_vec()),
        ("hang_over".into(), r3_corners(130.0, -0.6, 0.5).to_vec()),
    ];
    let rnd = r3_random_bases(0x5eed_4256, random);
    for (label, y, yg, nests) in ys {
        let mut probes = Vec::new();
        for (n, b) in &std {
            probes.push((n.clone(), p(b), cone(b)));
        }
        for (k, b) in nests.iter().enumerate() {
            if let Some(pb) = r3_apex_try(b, pose, tol, skipped, &format!("{label} nest{k}")) { probes.push((format!("nest{k}"), pb, cone(b))); }
        }
        for (k, b) in rnd.iter().enumerate() {
            if let Some(pb) = r3_apex_try(b, pose, tol, skipped, &format!("rnd{k}")) { probes.push((format!("rnd{k}"), pb, cone(b))); }
        }
        out.push(Scene { label, y, yg, probes });
    }
    out
}
