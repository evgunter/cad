//! Review probes for PR #5773 (review/atrest-link-ZmfEDg): `offset` against
//! analytic oracles, `lowest_on` against dense sampling.
#![allow(clippy::expect_used, clippy::panic, clippy::print_stderr, clippy::unwrap_used)]

use super::*;

fn band() -> Band {
    Band::linear(geom_core::Tol::witness()).expect("band")
}
fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}
const O: Point3<f64> = Point3::new(0.0, 0.0, 0.0);

fn of_fans(p0: Point3<f64>, fans: &[(Vec3<f64>, Vec<Point3<f64>>)]) -> Link<f64> {
    let sectors = fans
        .iter()
        .map(|(m, pts)| Sector {
            surface: geom::Surface::Plane { origin: p0, normal: *m, u_ref: (pts[0] - p0).normalize() },
            sense: true,
            outward: *m,
            apex: p0,
            wedges: pts.windows(2).map(|w| (w[0], w[1])).collect(),
            closed: false,
        })
        .collect();
    Link::of(p0, sectors).expect("link")
}

fn corner(hole: bool) -> Link<f64> {
    let s = if hole { 1.0 } else { -1.0 };
    let (x, y, z) = (p(s, 0.0, 0.0), p(0.0, s, 0.0), p(0.0, 0.0, s));
    of_fans(O, &[(v(1.0, 0.0, 0.0), vec![y, z]), (v(0.0, 1.0, 0.0), vec![z, x]), (v(0.0, 0.0, 1.0), vec![x, y])])
}
fn saddle() -> Link<f64> {
    let (x, y, z) = (p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), p(0.0, 0.0, -1.0));
    let (nx, ny) = (p(-1.0, 0.0, 0.0), p(0.0, -1.0, 0.0));
    of_fans(O, &[(v(0.0, 0.0, 1.0), vec![y, nx, ny, x]), (v(0.0, 1.0, 0.0), vec![x, z]), (v(1.0, 0.0, 0.0), vec![z, y])])
}
fn ridge() -> Link<f64> {
    let (y, ny) = (p(0.0, 1.0, 0.0), p(0.0, -1.0, 0.0));
    of_fans(O, &[(v(0.0, 0.0, 1.0), vec![y, p(-1.0, 0.0, 0.0), ny]), (v(1.0, 0.0, 0.0), vec![ny, p(0.0, 0.0, -1.0), y])])
}
/// A reflex (concave) edge along y: material where x ≤ 0 or z ≤ 0.
fn trough() -> Link<f64> {
    let (y, ny) = (p(0.0, 1.0, 0.0), p(0.0, -1.0, 0.0));
    of_fans(O, &[(v(0.0, 0.0, 1.0), vec![ny, p(1.0, 0.0, 0.0), y]), (v(1.0, 0.0, 0.0), vec![y, p(0.0, 0.0, 1.0), ny])])
}

fn pos(a: f64) -> f64 {
    a.max(0.0)
}
fn hyp(a: &[f64]) -> f64 {
    a.iter().map(|x| x * x).sum::<f64>().sqrt()
}
fn sd_block(q: Point3<f64>) -> f64 {
    if q.x <= 0.0 && q.y <= 0.0 && q.z <= 0.0 { -(-q.x).min(-q.y).min(-q.z) } else { hyp(&[pos(q.x), pos(q.y), pos(q.z)]) }
}
fn sd_ridge(q: Point3<f64>) -> f64 {
    if q.x <= 0.0 && q.z <= 0.0 { -(-q.x).min(-q.z) } else { hyp(&[pos(q.x), pos(q.z)]) }
}
fn sd_saddle(q: Point3<f64>) -> f64 {
    let (x, y, z) = (q.x, q.y, q.z);
    let t_in = if x <= 0.0 || y <= 0.0 { 0.0 } else { x.min(y) };
    let dt = hyp(&[z, t_in]);
    let da = hyp(&[y, (-x).max(0.0), pos(z)]);
    let db = hyp(&[x, (-y).max(0.0), pos(z)]);
    let d = dt.min(da).min(db);
    let inside = z <= 0.0 && !(x > 0.0 && y > 0.0);
    if inside { -d } else { d }
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64) * 2.0 - 1.0
    }
    fn pt(&mut self) -> Point3<f64> {
        p(self.next(), self.next(), self.next())
    }
}

type Case = (&'static str, Link<f64>, fn(Point3<f64>) -> f64);
fn cases() -> Vec<Case> {
    vec![
        ("block", corner(false), sd_block as fn(Point3<f64>) -> f64),
        ("hole", corner(true), |q| -sd_block(p(-q.x, -q.y, -q.z))),
        ("saddle", saddle(), sd_saddle),
        ("ridge", ridge(), sd_ridge),
        ("trough", trough(), |q| -sd_ridge(p(-q.x, -q.y, -q.z))),
    ]
}

/// Claim 3: offset is the signed distance to the cone's boundary.
#[test]
fn probe_offset_matches_the_analytic_signed_distance() {
    let b = band();
    let mut rng = Lcg(7);
    let mut report = Vec::new();
    for (name, link, sd) in cases() {
        let (mut bad, mut refused) = (Vec::new(), 0);
        for _ in 0..20000 {
            let q = rng.pt();
            match link.offset(q, b) {
                Ok(got) => {
                    if (got - sd(q)).abs() > 1e-12 {
                        bad.push((q, got, sd(q)));
                    }
                }
                Err(_) => refused += 1,
            }
        }
        // Probes on the boundary, on rays, on extended planes, beside them.
        let specials = [p(0.0, 0.0, 0.0), p(-0.3, -0.4, 0.0), p(0.0, 0.0, -0.5), p(0.0, 0.0, 0.5),
            p(0.5, 0.5, 0.0), p(0.5, -0.5, 0.0), p(-0.5, 0.5, 0.0), p(0.5, 0.0, 0.5), p(0.0, 0.5, 0.5),
            p(0.5, 0.5, 1e-7), p(0.5, 0.5, -1e-7), p(0.5, 0.5, 1e-3), p(1e-7, 0.5, 0.5)];
        let mut sp = Vec::new();
        for q in specials {
            sp.push(format!("{q:?} -> {:?} (oracle {})", link.offset(q, b), sd(q)));
        }
        report.push(format!("{name}: {} mismatches (first {:?}), {refused} refused of 20000\n  {}", bad.len(), bad.first(), sp.join("\n  ")));
    }
    // Continuity through zero along lines crossing the boundary.
    let mut cont = Vec::new();
    for (name, link, sd) in cases() {
        let (a, c) = (p(-0.3, 0.2, 0.25), p(0.35, -0.15, -0.2));
        let mut prev: Option<f64> = None;
        let mut worst = 0.0f64;
        for i in 0..=4000 {
            let t = f64::from(i) / 4000.0;
            let q = a + (c - a) * t;
            if let Ok(o) = link.offset(q, b) {
                if let Some(pv) = prev { worst = worst.max((o - pv).abs()); }
                prev = Some(o);
                let _ = sd(q);
            }
        }
        cont.push(format!("{name}: largest step {worst:e} (segment length/4000 = {:e})", (c - a).norm() / 4000.0));
    }
    eprintln!("PROBE offset\n{}\n{}", report.join("\n"), cont.join("\n"));
}

/// Claim 4: lowest_on's candidates hold the true minimum.
#[test]
fn probe_lowest_on_matches_dense_sampling() {
    let b = band();
    let mut rng = Lcg(11);
    let mut out = Vec::new();
    for (name, link, sd) in cases() {
        let (mut worst, mut refused, mut worse_than_oracle) = (0.0f64, 0, 0.0f64);
        let mut worst_seg = None;
        for _ in 0..1500 {
            let (a, c) = (rng.pt(), rng.pt());
            let got = match link.lowest_on((a, c), b) {
                Ok(g) => g,
                Err(_) => { refused += 1; continue; }
            };
            let mut best = f64::INFINITY;
            let mut best_t = 0.0;
            let n = 4000;
            for i in 0..=n {
                let t = f64::from(i) / f64::from(n);
                let o = sd(a + (c - a) * t);
                if o < best { best = o; best_t = t; }
            }
            // refine by ternary-ish local search on the oracle
            let (mut lo, mut hi) = ((best_t - 1.0 / f64::from(n)).max(0.0), (best_t + 1.0 / f64::from(n)).min(1.0));
            for _ in 0..200 {
                let m1 = lo + (hi - lo) / 3.0;
                let m2 = hi - (hi - lo) / 3.0;
                if sd(a + (c - a) * m1) < sd(a + (c - a) * m2) { hi = m2 } else { lo = m1 }
            }
            best = best.min(sd(a + (c - a) * (0.5 * (lo + hi))));
            let excess = got - best;
            if excess > worst { worst = excess; worst_seg = Some((a, c, got, best)); }
            worse_than_oracle = worse_than_oracle.min(excess);
        }
        out.push(format!("{name}: max(lowest_on - true min) = {worst:e} at {worst_seg:?}; min excess {worse_than_oracle:e}; {refused}/1500 refused"));
    }
    eprintln!("PROBE lowest_on\n{}", out.join("\n"));
}

/// Where does lowest_on refuse? Offset at points exactly on one extended
/// plane, at random, and the refusal rate of offset there.
#[test]
fn probe_offset_on_extended_planes() {
    let b = band();
    let mut rng = Lcg(5);
    let mut out = Vec::new();
    for (name, link, sd) in cases() {
        let mut per = Vec::new();
        for (k, s) in link.sectors.iter().enumerate() {
            let (mut refused, mut n_out, mut n_in, mut wrong) = (0, 0, 0, 0);
            let mut first = None;
            for _ in 0..4000 {
                let q0 = rng.pt();
                let q = q0 - s.outward * s.outward.dot(q0 - link.p);
                let inside_sector = sd(q).abs() < 1e-12;
                if inside_sector { n_in += 1 } else { n_out += 1 }
                match link.offset(q, b) {
                    Err(_) => { refused += 1; if first.is_none() { first = Some((q, sd(q))); } }
                    Ok(o) => if (o - sd(q)).abs() > 1e-9 { wrong += 1 },
                }
            }
            per.push(format!("sector {k}: {refused} refused ({n_in} on the sector, {n_out} off it), {wrong} wrong; first refusal {first:?}"));
        }
        out.push(format!("{name}:\n  {}", per.join("\n  ")));
    }
    eprintln!("PROBE planes\n{}", out.join("\n"));
}

/// lowest_on's candidates, replicated, each tagged with the forms it came from.
fn candidates(link: &Link<f64>, (x0, x1): (Point3<f64>, Point3<f64>), b: Band) -> Vec<(f64, String)> {
    let e = x1 - x0;
    let mut forms: Vec<([f64; 3], String)> = Vec::new();
    let rd = Reading { band: b, slack: None };
    let mut kept: Vec<(Point3<f64>, Vec3<f64>, Point3<f64>)> = Vec::new();
    let w = x0 - link.p;
    forms.push(([e.norm_squared(), 2.0 * w.dot(e), w.norm_squared()], "apex".into()));
    for (si, s) in link.sectors.iter().enumerate() {
        let w = x0 - s.apex;
        let (o0, o1) = (s.outward.dot(w), s.outward.dot(e));
        forms.push(([o1.powi(2), 2.0 * o0 * o1, o0.powi(2)], format!("plane{si}")));
        let last = s.wedges.len().saturating_sub(1);
        for (k, &(a, bb)) in s.wedges.iter().enumerate() {
            let rays = if k == last { vec![a, bb] } else { vec![a] };
            for r in rays {
                if kept.iter().any(|&(c, m, q)| on_ray(c, m, q, r, rd)) { continue; }
                kept.push((s.apex, s.outward, r));
                let u = (r - s.apex).normalize();
                let (wp, ep) = (w - u * u.dot(w), e - u * u.dot(e));
                forms.push(([ep.norm_squared(), 2.0 * wp.dot(ep), wp.norm_squared()], format!("ray{si}.{k}{r:?}")));
            }
        }
    }
    let mut ts = vec![(0.0, "end0".to_string()), (1.0, "end1".to_string())];
    for (f, n) in &forms { ts.push((-f[1] / (2.0 * f[0]), format!("near {n}"))); }
    for (i, (f, nf)) in forms.iter().enumerate() {
        for (g, ng) in &forms[i + 1..] {
            let (a, bq, c) = (f[0] - g[0], f[1] - g[1], f[2] - g[2]);
            let r = (bq.powi(2) - 4.0 * a * c).max(0.0).sqrt();
            for (j, t) in [(-bq + r) / (2.0 * a), (-bq - r) / (2.0 * a), 2.0 * c / (-bq + r), 2.0 * c / (-bq - r)].into_iter().enumerate() {
                ts.push((t, format!("tie {nf} = {ng} form{j}")));
            }
        }
    }
    ts.into_iter().filter(|(t, _)| t.is_finite()).map(|(t, n)| (t.clamp(0.0, 1.0), n)).collect()
}

#[test]
fn probe_which_candidate_refuses() {
    let b = band();
    let mut rng = Lcg(11);
    let mut out = Vec::new();
    for (name, link, sd) in cases() {
        let mut kinds: std::collections::BTreeMap<String, usize> = Default::default();
        let mut example = None;
        for _ in 0..1500 {
            let (a, c) = (rng.pt(), rng.pt());
            if link.lowest_on((a, c), b).is_ok() { continue; }
            for (t, n) in candidates(&link, (a, c), b) {
                let q = a + (c - a) * t;
                if link.offset(q, b).is_err() {
                    let key = n.split(" form").next().unwrap_or("").chars().filter(|ch| !ch.is_ascii_digit() || true).collect::<String>();
                    let key: String = key.split('P').next().unwrap_or("").to_string();
                    *kinds.entry(key).or_default() += 1;
                    if example.is_none() { example = Some((a, c, t, q, n.clone(), sd(q), link.reads(q, b))); }
                    break;
                }
            }
        }
        out.push(format!("{name}: {kinds:?}\n   e.g. {example:?}"));
    }
    eprintln!("PROBE which\n{}", out.join("\n"));
}

/// Contact: generic segments that touch the boundary and stay outside the
/// material; their lowest offset is 0. Rate of refusal and of a wrong value.
#[test]
fn probe_lowest_on_at_contact() {
    let b = band();
    let mut rng = Lcg(3);
    let (ridge, block) = (ridge(), corner(false));
    let (mut refused, mut wrong, mut ok) = (0, 0, 0);
    let (mut brefused, mut bok, mut bwrong) = (0, 0, 0);
    let mut example = None;
    for _ in 0..2000 {
        // Through a ridge point, crossing it outside the material.
        let y0 = rng.next() * 0.5;
        let (dx, dy, dz) = (rng.next().abs() + 0.05, rng.next(), -(rng.next().abs() + 0.05));
        let d = v(dx, dy, dz);
        let (s0, s1) = (0.2 + 0.8 * rng.next().abs(), 0.2 + 0.8 * rng.next().abs());
        let (a, c) = (p(0.0, y0, 0.0) - d * s0, p(0.0, y0, 0.0) + d * s1);
        match ridge.lowest_on((a, c), b) {
            Ok(g) if g.abs() < 1e-9 => ok += 1,
            Ok(g) => { wrong += 1; let _ = g; }
            Err(e) => { refused += 1; if example.is_none() { example = Some((a, c, e)); } }
        }
        // Resting on the block's top face (z = 0, x, y < 0), generic direction.
        let (a, c) = (p(-0.1 - 0.8 * rng.next().abs(), -0.1 - 0.8 * rng.next().abs(), 0.0), p(-0.1 - 0.8 * rng.next().abs(), -0.1 - 0.8 * rng.next().abs(), 0.0));
        match block.lowest_on((a, c), b) {
            Ok(g) if g.abs() < 1e-9 => bok += 1,
            Ok(_) => bwrong += 1,
            Err(_) => brefused += 1,
        }
    }
    eprintln!("PROBE contact eps={:e}: ridge crossing: {ok} read 0, {wrong} wrong, {refused} refused (e.g. {example:?}); block top face: {bok} read 0, {bwrong} wrong, {brefused} refused", geom_core::Tol::witness().eps());
}

/// Claim 6: does an Interval offset over a box enclose the offset at every
/// point of the box? A block corner's link read at q = (s, -0.5, 0),
/// s in [-0.1, 0.1]: on the top face for s <= 0 (offset 0), beside the
/// block for s > 0 (offset s, up to 0.1).
#[test]
fn probe_interval_offset_over_a_box_encloses_every_point() {
    use geom_core::Interval;
    let b = band();
    let i = Interval::from_f64;
    let ip = |x: f64, y: f64, z: f64| Point3::new(i(x), i(y), i(z));
    let iv = |x: f64, y: f64, z: f64| Vec3::new(i(x), i(y), i(z));
    let fan = |m: Vec3<Interval>, pts: Vec<Point3<Interval>>| Sector {
        surface: geom::Surface::Plane { origin: ip(0.0, 0.0, 0.0), normal: m, u_ref: (pts[0] - ip(0.0, 0.0, 0.0)).normalize() },
        sense: true,
        outward: m,
        apex: ip(0.0, 0.0, 0.0),
        wedges: pts.windows(2).map(|w| (w[0], w[1])).collect(),
        closed: false,
    };
    let (x, y, z) = (ip(-1.0, 0.0, 0.0), ip(0.0, -1.0, 0.0), ip(0.0, 0.0, -1.0));
    let block = Link::of(
        ip(0.0, 0.0, 0.0),
        vec![fan(iv(1.0, 0.0, 0.0), vec![y, z]), fan(iv(0.0, 1.0, 0.0), vec![z, x]), fan(iv(0.0, 0.0, 1.0), vec![x, y])],
    )
    .expect("link");
    let mut lines = Vec::new();
    for (lo, hi) in [(-0.1, 0.1), (0.0, 0.1), (0.05, 0.1), (-1e-3, 1e-3)] {
        let q = Point3::new(Interval::from_bounds(lo, hi), i(-0.5), i(0.0));
        let got = block.offset(q, b);
        let pt = |s: f64| corner(false).offset(p(s, -0.5, 0.0), b);
        lines.push(format!("s in [{lo}, {hi}]: interval {:?}; f64 at lo {:?}, at hi {:?}", got.map(|v| v.enclosure_probe()), pt(lo), pt(hi)));
    }
    // Segment version: the Gap edge arm.
    let q0 = Point3::new(Interval::from_bounds(-0.1, 0.1), i(-0.5), i(0.0));
    let q1 = Point3::new(Interval::from_bounds(-0.1, 0.1), i(-0.4), i(0.0));
    lines.push(format!("segment, x in [-0.1, 0.1]: interval lowest {:?}; f64 at x=0.1: {:?}", block.lowest_on((q0, q1), b).map(|v| v.enclosure_probe()), corner(false).lowest_on((p(0.1, -0.5, 0.0), p(0.1, -0.4, 0.0)), b)));
    eprintln!("PROBE interval box\n{}", lines.join("\n"));
}
