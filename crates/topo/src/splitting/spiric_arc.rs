//! **A spiric edge of a planar loop, read on its own carrier** — the
//! boundary reading and the ray crossing count the in-plane walk
//! ([`super::containment`]) runs beside its conic row.
//!
//! The oval has no closed-form ray intersection the walk could certify
//! cheaply (the ray meets the torus's section in the roots of a
//! quartic), so both readings subdivide the arc's parameter window into
//! pieces and certify each piece from two bounds over the piece's own
//! window, read off the carrier's data ([`geom::spiric_rate_bounds`]):
//! its speed `|P′| ≤ S` and its acceleration `|P″| ≤ A`. A piece of
//! half-width `h` about `v_m` lies in the ball of radius `S·h` about
//! `P(v_m)`.
//!
//! **Monotone across a line.** Along a line of unit normal `n`, the
//! offset `s(v) = (P(v) − q)·n` has `|s″| ≤ A`. On a piece `[v_a, v_b]`
//! of width `2h`, the mean of `s′` is the chord's slope
//! `(s(v_b) − s(v_a))/2h`, and `s′(v)` differs from it by
//! `(1/2h)·|∫(s′(v) − s′(u))du| ≤ (A/2h)·∫|v − u|du ≤ (A/2h)·(2h)²/2 = A·h`
//! (the integral is largest at an end). So `s′` keeps the chord's sign
//! across the piece wherever `|s(v_b) − s(v_a)| > 2·A·h²`.
//!
//! A piece that settles neither way is halved, until its ball is within
//! the band's coincidence threshold (the boundary reading), a fixed
//! depth, or a piece budget. A reading that runs out of depth or budget
//! is not a reading in the band: the walk names it as the edge it could
//! not cross ([`super::containment::Uncrossable`]).

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use crate::validate::{decide, definitely_positive as positive};

/// The rows a caller reads a spiric edge's boundary under — its own, so
/// each caller's population stays separable in the telemetry.
#[derive(Clone, Copy)]
pub(crate) struct SpiricRows {
    /// The distance from the point to either end of the arc.
    pub(crate) end: &'static str,
    /// A piece's ball's clearance from the point, `|q − P(v_m)| − S·h`.
    pub(crate) clear: &'static str,
    /// The distance from the point to a point of the arc, `|q − P(v_m)|`.
    pub(crate) on: &'static str,
    /// A piece's ball's radius `S·h`: within the band's coincidence
    /// threshold the piece is a point, and is not halved again.
    pub(crate) leaf: &'static str,
}

/// The rows of a ray's crossing of a spiric arc — the walk's alone.
const CROSS_REACH: &str = "point_in_arc_loop_reach";
const CROSS_SIDE: &str = "point_in_arc_loop_spiric_side";
const CROSS_TURN: &str = "point_in_arc_loop_spiric_turn";
const CROSS_ADVANCE: &str = "point_in_arc_loop_spiric_advance";

/// How many times a piece is halved before a reading gives up on it.
/// The ray count reaches it only where a crossing sits within the band
/// of a halving point or the ray grazes the oval; the boundary reading,
/// whose pieces stop at the band, only on a carrier whose bounds outrun
/// `2⁴⁰` band widths.
const MAX_DEPTH: u32 = 40;

/// How many pieces one reading visits before it gives up — a bound on
/// the work. A reading that converges holds a handful of unsettled
/// pieces per level, so it is reached only where many levels each hold
/// many: a point that lies, to the band, along a long stretch of the
/// oval's offset curve.
const MAX_PIECES: usize = 4096;

/// A spiric oval: [`geom::Curve3::Spiric`]'s data, named so that a
/// [`SpiricArc`] can only be built from one.
#[derive(Clone, Copy)]
pub(crate) struct Oval<T: geom_core::Real> {
    pub(crate) center: Point3<T>,
    pub(crate) axis: Vec3<T>,
    pub(crate) u_ref: Vec3<T>,
    pub(crate) major: T,
    pub(crate) minor: T,
    pub(crate) offset: T,
}

/// An arc of a spiric oval.
#[derive(Clone, Copy)]
pub(crate) struct SpiricArc<T: geom_core::Real> {
    oval: Oval<T>,
    /// The carrier parameters of the arc's two ends.
    span: (T, T),
}

/// A piece `[va, vb]` of the arc's window, with its end points.
struct Piece<T: geom_core::Real> {
    va: T,
    vb: T,
    pa: Point3<T>,
    pb: Point3<T>,
    depth: u32,
}

/// A piece's half-width `h`, its ball `(P(v_m), S·h)`, and `A`.
struct Bounds<T: geom_core::Real> {
    h: T,
    center: Point3<T>,
    reach: T,
    accel: T,
}

impl<T: Decide> SpiricArc<T> {
    /// The arc `span` of `oval`.
    pub(crate) fn new(oval: Oval<T>, span: (T, T)) -> Self {
        Self { oval, span }
    }

    /// The point at carrier parameter `v`.
    fn point(&self, v: T) -> Point3<T> {
        let o = self.oval;
        geom::Curve3::Spiric {
            center: o.center,
            axis: o.axis,
            u_ref: o.u_ref,
            major_radius: o.major,
            minor_radius: o.minor,
            offset: o.offset,
        }
        .eval(v)
    }

    /// The arc's two end points.
    pub(crate) fn ends(&self) -> [Point3<T>; 2] {
        [self.point(self.span.0), self.point(self.span.1)]
    }

    /// The whole arc as one piece.
    fn root(&self) -> Piece<T> {
        let (va, vb) = self.span;
        Piece {
            va,
            vb,
            pa: self.point(va),
            pb: self.point(vb),
            depth: 0,
        }
    }

    /// A piece's bounds over its own window: `cos v` and `sin v` each
    /// move at most `h` from their values at `v_m`, so `ρ = R + r·cos v`
    /// stays in `[R + r·max(cos v_m − h, −1), R + r·min(cos v_m + h, 1)]`
    /// and `|sin v| ≤ min(|sin v_m| + h, 1)`.
    fn bounds(&self, p: &Piece<T>) -> Bounds<T> {
        let o = self.oval;
        let h = (p.vb - p.va).abs() * T::from_f64(0.5);
        let vm = geom::mid_param(p.va, p.vb);
        let (sm, cm) = vm.sin_cos();
        let one = T::one();
        let rho = (
            o.major + o.minor * (cm - h).max(T::zero() - one),
            o.major + o.minor * (cm + h).min(one),
        );
        let sin_max = (sm.abs() + h).min(one);
        let (speed, accel) = geom::spiric_rate_bounds(o.minor, o.offset, rho, sin_max);
        Bounds {
            h,
            center: self.point(vm),
            reach: speed * h,
            accel,
        }
    }

    /// A piece's two halves, the half nearer `va` on top of the stack.
    fn split(&self, p: Piece<T>, stack: &mut Vec<Piece<T>>) {
        let vm = geom::mid_param(p.va, p.vb);
        let pm = self.point(vm);
        let depth = p.depth + 1;
        stack.push(Piece {
            va: vm,
            vb: p.vb,
            pa: pm,
            pb: p.pb,
            depth,
        });
        stack.push(Piece {
            va: p.va,
            vb: vm,
            pa: p.pa,
            pb: pm,
            depth,
        });
    }

    /// **Where `q` sits against the arc.** `End` within the band of an
    /// end; `On` within the band of a point of the arc; `Off` once every
    /// piece's ball is definitely clear of `q`; `Unsettled` where a
    /// piece outlasts the depth or the piece budget before its ball is
    /// within the band — no reading in the band, and no answer.
    ///
    /// # Errors
    ///
    /// An in-band distance to an end; or, where a piece whose ball is
    /// within the band's coincidence threshold is settled neither way
    /// and no other piece puts `q` on the arc, the in-band margin it was
    /// read on.
    pub(crate) fn contact(
        &self,
        q: Point3<T>,
        rows: SpiricRows,
        band: Band,
    ) -> Result<SpiricHit, Indeterminate> {
        for end in self.ends() {
            if decide(rows.end, Margin::norm3(q - end), band)? == Sign::Zero {
                return Ok(SpiricHit::End);
            }
        }
        let mut stack = vec![self.root()];
        let mut visits = 0;
        // The first leaf no reading settled, by the margin it was read
        // in the band on.
        let mut in_band: Option<Indeterminate> = None;
        let mut exhausted = false;
        while let Some(p) = stack.pop() {
            visits += 1;
            let b = self.bounds(&p);
            let gap = (q - b.center).norm();
            let mut read = None;
            match decide(rows.clear, Margin::of(gap - b.reach), band) {
                Ok(Sign::Positive) => continue,
                Ok(_) => {}
                Err(diag) => read = Some(diag),
            }
            match decide(rows.on, Margin::of(gap), band) {
                Ok(Sign::Zero) => return Ok(SpiricHit::On),
                Ok(_) => {}
                Err(diag) => read = Some(diag),
            }
            if matches!(decide(rows.leaf, Margin::of(b.reach), band), Ok(Sign::Zero)) {
                match read {
                    Some(diag) => {
                        in_band.get_or_insert(diag);
                    }
                    None => exhausted = true,
                }
                continue;
            }
            if p.depth >= MAX_DEPTH || visits >= MAX_PIECES {
                exhausted = true;
                if visits >= MAX_PIECES {
                    break;
                }
                continue;
            }
            self.split(p, &mut stack);
        }
        if let Some(diag) = in_band {
            return Err(diag);
        }
        Ok(if exhausted {
            SpiricHit::Unsettled
        } else {
            SpiricHit::Off
        })
    }

    /// **How many times the ray `q + d·t`, `t > 0`, crosses the arc**,
    /// counted in the loop's plane (`side` the in-plane unit normal to
    /// `d`). `None` where a piece meeting the ray settles neither its
    /// crossing nor its miss by the depth — a graze, or a crossing at a
    /// piece's end — which abandons the ray (why that is sound: the ray
    /// loop in [`super::containment`]'s walk). The walk calls it only
    /// for a point [`Self::contact`] read `Off`.
    ///
    /// A piece is settled when its ball definitely misses the ray, or
    /// when `s(v) = (P(v) − q)·side` is definitely monotone on it (the
    /// module docs) with both ends' `s` definitely signed: same signs, no
    /// crossing; opposite, exactly one, counted when the ball is
    /// definitely ahead of `q` along `d` and not when it is definitely
    /// behind.
    pub(crate) fn crossings(
        &self,
        q: Point3<T>,
        d: Vec3<T>,
        side: Vec3<T>,
        band: Band,
    ) -> Option<usize> {
        let mut stack = vec![self.root()];
        let mut visits = 0;
        let mut count = 0;
        while let Some(p) = stack.pop() {
            visits += 1;
            let b = self.bounds(&p);
            let w = b.center - q;
            let (along, off) = (w.dot(d), w.dot(side));
            // The in-plane distance from the ball's centre to the ray.
            let nearest = (off.powi(2) + along.min(T::zero()).powi(2)).sqrt();
            if positive(CROSS_REACH, nearest - b.reach, band) {
                continue;
            }
            let (sa, sb) = ((p.pa - q).dot(side), (p.pb - q).dot(side));
            let ends = (
                decide(CROSS_SIDE, Margin::of(sa), band),
                decide(CROSS_SIDE, Margin::of(sb), band),
            );
            let monotone = positive(
                CROSS_TURN,
                (sb - sa).abs() - T::from_f64(2.0) * b.accel * b.h.powi(2),
                band,
            );
            if let (
                true,
                Ok(a @ (Sign::Positive | Sign::Negative)),
                Ok(z @ (Sign::Positive | Sign::Negative)),
            ) = (monotone, ends.0, ends.1)
            {
                if a == z {
                    continue;
                }
                if positive(CROSS_ADVANCE, along - b.reach, band) {
                    count += 1;
                    continue;
                }
                if matches!(
                    decide(CROSS_ADVANCE, Margin::of(along + b.reach), band),
                    Ok(Sign::Negative)
                ) {
                    continue;
                }
            }
            if p.depth >= MAX_DEPTH || visits >= MAX_PIECES {
                return None;
            }
            self.split(p, &mut stack);
        }
        Some(count)
    }
}

/// Where a point sits against a spiric arc — [`SpiricArc::contact`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SpiricHit {
    /// Definitely off the arc.
    Off,
    /// Within the band of a point of the arc, clear of its ends.
    On,
    /// Within the band of one of the arc's two ends.
    End,
    /// A piece outlasted the reading's depth or budget: not placed.
    Unsettled,
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::{Interval, Tol};

    const ROWS: SpiricRows = SpiricRows {
        end: "test_spiric_end",
        clear: "test_spiric_clear",
        on: "test_spiric_on",
        leaf: "test_spiric_leaf",
    };

    /// The oval `R = 2, r = 0.5` cut at `x = 0.8`, in the plane's
    /// `(y, z)` coordinates, and its arc `v ∈ [−2, 2]`.
    fn arc<T: Decide>() -> SpiricArc<T> {
        let f = T::from_f64;
        SpiricArc::new(
            Oval {
                center: Point3::new(f(0.0), f(0.0), f(0.0)),
                axis: Vec3::new(f(0.0), f(0.0), f(1.0)),
                u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
                major: f(2.0),
                minor: f(0.5),
                offset: f(0.8),
            },
            (f(-2.0), f(2.0)),
        )
    }

    fn at(v: f64) -> (f64, f64) {
        let rho = 2.0 + 0.5 * v.cos();
        ((rho * rho - 0.64).sqrt(), 0.5 * v.sin())
    }

    /// **The arc closed by its chord, read by the row, against the
    /// region sampled densely.** Every grid point of the oval's
    /// neighbourhood further than `1e-3` from the sampled boundary is
    /// definitely off the arc, and its ray along a fixed generic
    /// direction crosses the arc a certified number of times whose
    /// parity with the chord's is the sampled region. Every point here
    /// is inside the ball the arc lies in (about `P(0)`, radius `S·2`).
    /// On the arc it is on; at an end, the end; between the band's two
    /// thresholds off the arc, it refuses.
    fn the_row_reads_the_arcs_region<T: Decide>(lane: &str) {
        let band = Band::linear(Tol::witness()).unwrap();
        let k = arc::<T>();
        let f = T::from_f64;
        let n = 20_000;
        let mut boundary: Vec<(f64, f64)> = (0..=n)
            .map(|i| at(-2.0 + 4.0 * f64::from(i) / f64::from(n)))
            .collect();
        boundary.push(at(-2.0));
        let truth = |(y, z): (f64, f64)| -> (bool, f64) {
            let mut inside = false;
            let mut gap = f64::INFINITY;
            for w in boundary.windows(2) {
                let ((ay, az), (by, bz)) = (w[0], w[1]);
                if (az > z) != (bz > z) && y < ay + (z - az) / (bz - az) * (by - ay) {
                    inside = !inside;
                }
                let (dy, dz) = (by - ay, bz - az);
                let s = (((y - ay) * dy + (z - az) * dz) / (dy * dy + dz * dz)).clamp(0.0, 1.0);
                gap = gap.min((y - ay - s * dy).hypot(z - az - s * dz));
            }
            (inside, gap)
        };
        let (a, b) = (at(-2.0), at(2.0));
        let angle = 0.7_f64;
        let (dy, dz) = (angle.cos(), angle.sin());
        let point = |(y, z): (f64, f64)| Point3::new(f(0.8), f(y), f(z));
        let d = Vec3::new(f(0.0), f(dy), f(dz));
        let side = Vec3::new(f(1.0), f(0.0), f(0.0)).cross(d);
        let (mut asked, mut inside) = (0, 0);
        for i in 0..=24 {
            for j in 0..=24 {
                let q = (
                    1.2 + 1.4 * f64::from(i) / 24.0,
                    -0.7 + 1.4 * f64::from(j) / 24.0,
                );
                let (want, gap) = truth(q);
                if gap < 1e-3 {
                    continue;
                }
                asked += 1;
                inside += usize::from(want);
                assert_eq!(
                    k.contact(point(q), ROWS, band),
                    Ok(SpiricHit::Off),
                    "{lane} {q:?}: off the arc"
                );
                // The chord's crossing, in closed form.
                let cross = |(py, pz): (f64, f64)| (py - q.0) * dz - (pz - q.1) * dy;
                let chord = if (cross(a) > 0.0) == (cross(b) > 0.0) {
                    0
                } else {
                    let s = cross(a) / (cross(a) - cross(b));
                    let (hy, hz) = (a.0 + s * (b.0 - a.0), a.1 + s * (b.1 - a.1));
                    usize::from((hy - q.0) * dy + (hz - q.1) * dz > 0.0)
                };
                let got = k.crossings(point(q), d, side, band);
                let Some(count) = got else {
                    panic!("{lane} {q:?}: the ray was abandoned");
                };
                assert_eq!(
                    (count + chord) % 2 == 1,
                    want,
                    "{lane} {q:?}: {count} arc crossings"
                );
            }
        }
        assert!(
            asked >= 400 && inside >= 100 && asked - inside >= 100,
            "{lane}: not vacuous ({asked} asked, {inside} inside)"
        );
        let mid = point(at(0.3));
        assert_eq!(
            k.contact(mid, ROWS, band),
            Ok(SpiricHit::On),
            "{lane}: on the arc"
        );
        assert_eq!(
            k.contact(point(b), ROWS, band),
            Ok(SpiricHit::End),
            "{lane}: at its end"
        );
        // Off the arc's outermost point, along the plane, in the band.
        let (top, _) = at(0.0);
        let in_band = point((top + 0.5 * (band.zero() + band.escalate()), 0.0));
        assert!(
            k.contact(in_band, ROWS, band).is_err(),
            "{lane}: in the band of the arc, the reading refuses"
        );
    }

    /// The oval `(R, r, o)` in the `x = o` plane, its axis `z`.
    fn oval(big: f64, r: f64, o: f64) -> Oval<f64> {
        Oval {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
            major: big,
            minor: r,
            offset: o,
        }
    }

    /// `max |P′|/S` and `max |P″|/A` over the piece `[va, vb]`, sampled.
    fn ratios(k: &SpiricArc<f64>, (va, vb): (f64, f64), samples: u32) -> (f64, f64) {
        let p = Piece {
            va,
            vb,
            pa: k.point(va),
            pb: k.point(vb),
            depth: 0,
        };
        let b = k.bounds(&p);
        let speed = b.reach / b.h;
        let c = geom::Curve3::Spiric {
            center: k.oval.center,
            axis: k.oval.axis,
            u_ref: k.oval.u_ref,
            major_radius: k.oval.major,
            minor_radius: k.oval.minor,
            offset: k.oval.offset,
        };
        let (mut s, mut a) = (0.0_f64, 0.0_f64);
        for i in 0..=samples {
            let v = va + (vb - va) * f64::from(i) / f64::from(samples);
            s = s.max(c.deriv(v).norm() / speed);
            a = a.max(c.deriv2(v).norm() / b.accel);
        }
        (s, a)
    }

    /// **Each bound is nearly tight where it binds**, so a bound any
    /// much smaller would let a piece's ball miss its own arc, or a
    /// piece that is not monotone pass as one. The speed bound is exact
    /// on a cut through the axis (`o = 0`, where `f = ρ` and `|P′| = r`),
    /// over the whole period and over a piece. The acceleration bound
    /// binds hardest on a thin ring cut near tangency (`R = 1, r = 0.01,
    /// o = 0.9·(R − r)`), where the sampled `|P″|` reaches over 60% of
    /// `A` over the whole period and over a piece by the pinch — so `A`
    /// halved fails there.
    #[test]
    fn each_rate_bound_is_nearly_tight_where_it_binds() {
        let pi = core::f64::consts::PI;
        let whole = (0.0, core::f64::consts::TAU);
        let pinch = (pi - 0.05, pi);
        let through = SpiricArc::new(oval(2.0, 0.4, 0.0), (0.0, 1.0));
        let near = SpiricArc::new(oval(1.0, 0.01, 0.9 * 0.99), (0.0, 1.0));
        for (name, k, window) in [
            ("through the axis, whole", &through, whole),
            ("through the axis, a piece", &through, pinch),
            ("near tangency, whole", &near, whole),
            ("near tangency, by the pinch", &near, pinch),
        ] {
            let (s, a) = ratios(k, window, 100_000);
            assert!(
                s <= 1.0 + 1e-12 && a <= 1.0,
                "{name}: a bound fails, {s} {a}"
            );
            if core::ptr::eq(k, &through) {
                assert!(s >= 0.999, "{name}: |P′|/S = {s}");
            } else {
                assert!(a >= 0.6, "{name}: |P″|/A = {a}");
            }
        }
    }

    /// **A near-tangent cut reads far points and crosses them.** The
    /// review's case: `R = 1, r = 0.25` cut at `o = frac·(R − r)` as the
    /// cut nears tangency, read at the oval's own centre — a minor radius
    /// off the arc `v ∈ [−3, 3]`. The whole oval's speed bound is up to
    /// `10⁴` times its typical speed there; each piece's own window keeps
    /// the reading to the pieces by the pinch, and it answers `Off`, and
    /// a ray from there is counted.
    #[test]
    fn a_near_tangent_cut_reads_and_crosses_a_far_point() {
        let band = Band::linear(Tol::witness()).unwrap();
        for frac in [0.99, 0.9999, 0.999_999, 0.999_999_99] {
            let o = 0.75 * frac;
            let k = SpiricArc::new(oval(1.0, 0.25, o), (-3.0, 3.0));
            let mid_y = 0.5 * (k.point(0.0).y + k.point(core::f64::consts::PI).y);
            let q = Point3::new(o, mid_y, 0.0);
            assert_eq!(k.contact(q, ROWS, band), Ok(SpiricHit::Off), "frac {frac}");
            let d = Vec3::new(0.0, 0.3_f64.cos(), 0.3_f64.sin());
            let side = Vec3::new(1.0, 0.0, 0.0).cross(d);
            // One crossing: the ray leaves the oval's interior through
            // the arc (the gap `v ∈ [3, 2π − 3]` lies behind it).
            assert_eq!(k.crossings(q, d, side, band), Some(1), "frac {frac}");
        }
    }

    #[test]
    fn the_row_reads_the_arcs_region_at_f64() {
        the_row_reads_the_arcs_region::<f64>("f64");
    }

    #[test]
    fn the_row_reads_the_arcs_region_at_interval() {
        the_row_reads_the_arcs_region::<Interval>("Interval");
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod fuzz;
