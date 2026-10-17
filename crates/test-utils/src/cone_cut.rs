//! **A double cone cut by a plane, in double-double arithmetic**: the
//! oracle the plane × cone rows judge a served line pair or ellipse
//! against, and read their fixtures' poses with, never reading the arm's
//! own bound.
//!
//! Every input is a stored `f64` (the cone's apex, axis and half-angle,
//! the plane's origin and normal, a served line's origin and direction
//! or ellipse's fields), read exactly and carried at about 106 bits, so
//! the oracle's own rounding is some `1e-32` of the lengths it reads.
//! Points are taken relative to the apex, which a double-double
//! difference of two `f64`s holds exactly.

/// A double-double: the unevaluated sum of two `f64`s.
#[derive(Clone, Copy, Debug)]
struct Dd(f64, f64);

fn two_sum(a: f64, b: f64) -> Dd {
    let s = a + b;
    let v = s - a;
    Dd(s, (a - (s - v)) + (b - v))
}

fn quick(a: f64, b: f64) -> Dd {
    let s = a + b;
    Dd(s, b - (s - a))
}

impl Dd {
    fn of(x: f64) -> Self {
        Self(x, 0.0)
    }
    fn add(self, o: Self) -> Self {
        let s = two_sum(self.0, o.0);
        quick(s.0, s.1 + self.1 + o.1)
    }
    fn neg(self) -> Self {
        Self(-self.0, -self.1)
    }
    fn sub(self, o: Self) -> Self {
        self.add(o.neg())
    }
    fn mul(self, o: Self) -> Self {
        let p = self.0 * o.0;
        quick(p, self.0.mul_add(o.0, -p) + (self.0 * o.1 + self.1 * o.0))
    }
    fn div(self, o: Self) -> Self {
        let q1 = self.0 / o.0;
        let r = self.sub(o.mul(Self::of(q1)));
        let q2 = r.0 / o.0;
        let r = r.sub(o.mul(Self::of(q2)));
        quick(q1, q2).add(Self::of(r.0 / o.0))
    }
    fn sqrt(self) -> Self {
        if self.0 <= 0.0 {
            return Self::of(0.0);
        }
        let x = self.0.sqrt();
        let r = self.sub(Self::of(x).mul(Self::of(x)));
        quick(x, r.0 / (2.0 * x))
    }
    fn abs(self) -> Self {
        if self.0 < 0.0 { self.neg() } else { self }
    }
    fn f(self) -> f64 {
        self.0 + self.1
    }
    /// `sin x` and `cos x` for `|x| ≤ π/2`, by their Taylor series.
    fn sin_cos(x: f64) -> (Self, Self) {
        let x = Self::of(x);
        let x2 = x.mul(x).neg();
        let (mut s, mut c) = (x, Self::of(1.0));
        let (mut ts, mut tc) = (x, Self::of(1.0));
        for k in 1..30 {
            let k = f64::from(k);
            ts = ts.mul(x2).div(Self::of((2.0 * k) * (2.0 * k + 1.0)));
            tc = tc.mul(x2).div(Self::of((2.0 * k - 1.0) * (2.0 * k)));
            s = s.add(ts);
            c = c.add(tc);
        }
        (s, c)
    }
}

/// A point or vector as three stored `f64`s.
pub type P3 = [f64; 3];

type V = [Dd; 3];

fn v(p: P3) -> V {
    p.map(Dd::of)
}
fn dot(a: V, b: V) -> Dd {
    a[0].mul(b[0]).add(a[1].mul(b[1])).add(a[2].mul(b[2]))
}
fn sub(a: V, b: V) -> V {
    [a[0].sub(b[0]), a[1].sub(b[1]), a[2].sub(b[2])]
}
fn add(a: V, b: V) -> V {
    [a[0].add(b[0]), a[1].add(b[1]), a[2].add(b[2])]
}
fn scale(a: V, k: Dd) -> V {
    a.map(|x| x.mul(k))
}
fn cross(a: V, b: V) -> V {
    [
        a[1].mul(b[2]).sub(a[2].mul(b[1])),
        a[2].mul(b[0]).sub(a[0].mul(b[2])),
        a[0].mul(b[1]).sub(a[1].mul(b[0])),
    ]
}
fn norm(a: V) -> Dd {
    dot(a, a).sqrt()
}
fn unit(a: V) -> V {
    scale(a, Dd::of(1.0).div(norm(a)))
}

/// A line through a point along a unit direction, apex-relative.
type Line = (V, V);

fn from_line(x: V, (o, d): Line) -> f64 {
    norm(cross(sub(x, o), d)).f()
}

/// Slants (signed distances from the apex) in `[lo, hi]`: both ends, an
/// even spread, and a geometric one out from the end nearer the apex.
fn slants(lo: f64, hi: f64, n: u32) -> Vec<f64> {
    let near = if lo > 0.0 {
        lo
    } else if hi < 0.0 {
        hi
    } else {
        0.0
    };
    let span = (hi - lo).max(f64::MIN_POSITIVE);
    let mut ts = vec![lo, hi];
    for i in 0..=n {
        let s = f64::from(i) / f64::from(n);
        ts.push(lo + span * s);
        let step = span * 1e-16_f64.powf(1.0 - s);
        for t in [near + step, near - step] {
            if (lo..=hi).contains(&t) {
                ts.push(t);
            }
        }
    }
    ts
}

/// The double cone of half-angle `alpha` about `axis` from `apex`, cut by
/// the plane through `origin` with normal `normal`. A slant `t` is a
/// signed distance from the apex along a generator, positive on the
/// nappe `axis` points into.
pub struct ConeCut {
    apex: V,
    a: V,
    n: V,
    e1: V,
    e2: V,
    c: Dd,
    s: Dd,
    qn: Dd,
    sin: Dd,
    cos: Dd,
}

impl ConeCut {
    /// The cut, read from the stored `f64`s.
    #[must_use]
    pub fn new(apex: P3, axis: P3, alpha: f64, (origin, normal): (P3, P3)) -> Self {
        let apex = v(apex);
        let a = unit(v(axis));
        let n = unit(v(normal));
        let c = dot(a, n);
        let perp = sub(n, scale(a, c));
        let s = norm(perp);
        let e1 = scale(perp, Dd::of(1.0).div(s));
        let (sin, cos) = Dd::sin_cos(alpha);
        Self {
            apex,
            a,
            n,
            e1,
            e2: cross(a, e1),
            c,
            s,
            qn: dot(sub(v(origin), apex), n),
            sin,
            cos,
        }
    }

    /// The apex's signed distance from the plane, `(apex − origin)·n̂`.
    #[must_use]
    pub fn gap(&self) -> f64 {
        self.qn.neg().f()
    }

    /// The section's two points `|t|` from the apex on the nappe `t`'s
    /// sign names, apex-relative: `z·c + r·s·cos φ = q·n` at height
    /// `z = t·cos α`, radius `r = |t|·sin α`.
    fn at(&self, t: f64) -> Option<[V; 2]> {
        let t = Dd::of(t);
        let (z, r) = (t.mul(self.cos), t.abs().mul(self.sin));
        if r.f() == 0.0 {
            return None;
        }
        let cos = self.qn.sub(z.mul(self.c)).div(r.mul(self.s));
        if cos.abs().f() > 1.0 {
            return None;
        }
        let sin = Dd::of(1.0).sub(cos.mul(cos)).sqrt();
        let base = add(scale(self.a, z), scale(self.e1, r.mul(cos)));
        let side = scale(self.e2, r.mul(sin));
        Some([add(base, side), sub(base, side)])
    }

    /// Where the section starts on the nappe of `sign`: the least `|t|`
    /// it holds, by bisection (admission is monotone in `|t|` on a nappe,
    /// the plane dipping into the cone). Infinite where the plane holds
    /// no section on that nappe out to `1e30`.
    #[must_use]
    pub fn start(&self, sign: f64) -> f64 {
        let mut out = 0.0;
        let mut inn = 1e-30;
        while self.at(sign * inn).is_none() {
            out = inn;
            inn *= 2.0;
            if inn > 1e30 {
                return f64::INFINITY;
            }
        }
        for _ in 0..200 {
            let m = 0.5 * (inn + out);
            if self.at(sign * m).is_some() {
                inn = m;
            } else {
                out = m;
            }
        }
        inn
    }

    /// The distance from `p` to the section `|t|` from the apex on
    /// either nappe, the nearer of its two points; infinite where there
    /// is none.
    fn from(&self, p: V, t: f64) -> f64 {
        self.at(t).map_or(f64::INFINITY, |xs| {
            xs.iter()
                .map(|&x| norm(sub(x, p)).f())
                .fold(f64::INFINITY, f64::min)
        })
    }

    /// **How far a stored ellipse stands off the cut**: over points of
    /// `center + u_ref·semi_u·cos θ + (axis × u_ref)·semi_v·sin θ`, each
    /// evaluated here from the stored fields, the farthest one stands
    /// from the double cone or from the plane, whichever is farther.
    #[must_use]
    pub fn ellipse_miss(&self, center: P3, axis: P3, semi_u: f64, semi_v: f64, u_ref: P3) -> f64 {
        let (center, u) = (sub(v(center), self.apex), v(u_ref));
        let w = cross(v(axis), u);
        let mut worst = 0.0_f64;
        for i in 0..=16 {
            let (s, c) = Dd::sin_cos(core::f64::consts::FRAC_PI_2 * f64::from(i) / 16.0);
            for (c, s) in [(c, s), (c.neg(), s), (c.neg(), s.neg()), (c, s.neg())] {
                let x = add(
                    center,
                    add(
                        scale(u, Dd::of(semi_u).mul(c)),
                        scale(w, Dd::of(semi_v).mul(s)),
                    ),
                );
                let h = dot(x, self.a);
                let r = norm(sub(x, scale(self.a, h)));
                let off_cone = r.mul(self.cos).sub(h.abs().mul(self.sin)).abs().f();
                let off_plane = dot(x, self.n).sub(self.qn).abs().f();
                worst = worst.max(off_cone.min(norm(x).f())).max(off_plane);
            }
        }
        worst
    }

    /// The generator pair the plane through the apex parallel to this one
    /// holds, computed here: `g·n = 0` at `cos u = −c·cos α/(s·sin α)` in
    /// the frame of the normal's part across the axis. `None` where that
    /// plane clears the cone or touches it.
    fn pair(&self) -> Option<[Line; 2]> {
        let cos = self.c.mul(self.cos).div(self.s.mul(self.sin)).neg();
        if cos.abs().f() >= 1.0 {
            return None;
        }
        let sin = Dd::of(1.0).sub(cos.mul(cos)).sqrt();
        let base = add(scale(self.a, self.cos), scale(self.e1, cos.mul(self.sin)));
        let side = scale(self.e2, sin.mul(self.sin));
        let origin = [Dd::of(0.0); 3];
        Some([(origin, add(base, side)), (origin, sub(base, side))])
    }

    /// **The miss**: over the section points and the lines' points whose
    /// slant lies in `[lo, hi]`, the farthest a section point stands from
    /// the nearer line, and the farthest a line point stands from the
    /// section. Each line is its stored `(origin, direction)`.
    ///
    /// A section point nearest a line point `p` stands as far from the
    /// apex as `p` to within their distance, so the search for it runs
    /// over that bracket of slants on each nappe, by golden section from
    /// the section's start out, about a first reading at `p`'s own slant
    /// or at the nearer start.
    #[must_use]
    pub fn miss(&self, lines: &[(P3, P3)], window: (f64, f64)) -> f64 {
        let lines: Vec<Line> = lines
            .iter()
            .map(|&(o, d)| (sub(v(o), self.apex), unit(v(d))))
            .collect();
        self.miss_of(&lines, window)
    }

    /// [`Self::miss`] of the generator pair this plane's parallel through
    /// the apex holds: how far the true line pair stands from the
    /// section over `[lo, hi]`. Infinite where there is no pair.
    #[must_use]
    pub fn pair_miss(&self, window: (f64, f64)) -> f64 {
        self.pair()
            .map_or(f64::INFINITY, |pair| self.miss_of(&pair, window))
    }

    fn miss_of(&self, lines: &[Line], (lo, hi): (f64, f64)) -> f64 {
        let ts = slants(lo, hi, 64);
        let starts = [self.start(1.0), self.start(-1.0)];
        let mut worst = 0.0_f64;
        let starts_in = starts
            .iter()
            .zip([1.0, -1.0])
            .map(|(&s, sign)| sign * s)
            .filter(|t| (lo..=hi).contains(t));
        for t in ts.iter().copied().chain(starts_in) {
            if let Some(xs) = self.at(t) {
                for x in xs {
                    let d = lines
                        .iter()
                        .map(|&l| from_line(x, l))
                        .fold(f64::INFINITY, f64::min);
                    worst = worst.max(d);
                }
            }
        }
        for &(o, d) in lines {
            let ad = dot(self.a, d);
            for &t in &ts {
                let z = Dd::of(t).mul(self.cos);
                let p = add(o, scale(d, z.sub(dot(self.a, o)).div(ad)));
                let reading = self.from(p, t).min(
                    starts
                        .iter()
                        .zip([1.0, -1.0])
                        .map(|(&s, sign)| self.from(p, sign * s))
                        .fold(f64::INFINITY, f64::min),
                );
                let mut best = reading;
                for (&start, sign) in starts.iter().zip([1.0, -1.0]) {
                    let (mut a, mut b) = ((t.abs() - reading).max(start), t.abs() + reading);
                    if a > b {
                        continue;
                    }
                    for _ in 0..80 {
                        let (m1, m2) = (a + (b - a) / 3.0, b - (b - a) / 3.0);
                        if self.from(p, sign * m1) < self.from(p, sign * m2) {
                            b = m2;
                        } else {
                            a = m1;
                        }
                    }
                    best = best.min(self.from(p, sign * 0.5 * (a + b)));
                }
                worst = worst.max(best);
            }
        }
        worst
    }
}
