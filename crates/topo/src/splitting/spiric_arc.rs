//! **A spiric edge of a planar loop, read on its own carrier** — the
//! boundary reading and the ray crossing count the in-plane walk
//! ([`super::containment`]) runs beside its conic row.
//!
//! The oval has no closed-form ray intersection the walk could certify
//! cheaply (the ray meets the torus's section in the roots of a
//! quartic), so both readings subdivide the arc's parameter window into
//! pieces and certify each piece from two bounds read off the carrier's
//! own data: its speed `|P′| ≤ S` and its acceleration `|P″| ≤ A`
//! ([`SpiricArc::of`]). A piece of half-width `h` about `v_m` lies in the
//! ball of radius `S·h` about `P(v_m)`, and its distance from any line
//! of unit normal `n` — `s(v) = (P(v) − q)·n` — is monotone wherever the
//! chord's own offset outruns the curvature,
//! `|s(v_b) − s(v_a)| > 4·A·h²` (`s′` deviates from its mean, the chord's
//! slope, by at most `A·2h`). A piece that settles neither is halved, to
//! a fixed depth.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use crate::validate::decide;

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
    /// The name the reading escalates under where a piece it could not
    /// settle was read in the band on no margin of its own — which only
    /// its depth or piece budget can leave. It never reaches the funnel
    /// ([`crate::invalid_margin`]).
    pub(crate) depth: &'static str,
}

/// The rows of a ray's crossing of a spiric arc — the walk's alone.
const CROSS_REACH: &str = "point_in_arc_loop_reach";
const CROSS_SIDE: &str = "point_in_arc_loop_spiric_side";
const CROSS_TURN: &str = "point_in_arc_loop_spiric_turn";
const CROSS_ADVANCE: &str = "point_in_arc_loop_spiric_advance";

/// How many times a piece is halved before the reading gives up: the
/// piece's ball is then `S·w/2⁴¹` across, under the band of any
/// committed ε for a carrier of metre scale.
const MAX_DEPTH: u32 = 40;

/// How many pieces one reading visits before it gives up — a bound on
/// the work, never reached by a reading that converges (each level holds
/// a handful of unsettled pieces).
const MAX_PIECES: usize = 4096;

/// An arc of a [`geom::Curve3::Spiric`] oval, with the carrier's speed
/// and acceleration bounds.
#[derive(Clone)]
pub(crate) struct SpiricArc<T: geom_core::Real> {
    carrier: geom::Curve3<T>,
    /// The carrier parameters of the arc's two ends.
    span: (T, T),
    /// `S ≥ |dP/dv|` over the whole oval.
    speed: T,
    /// `A ≥ |d²P/dv²|` over the whole oval.
    accel: T,
}

/// A piece `[va, vb]` of the arc's window, with its end points.
struct Piece<T: geom_core::Real> {
    va: T,
    vb: T,
    pa: Point3<T>,
    pb: Point3<T>,
    depth: u32,
}

impl<T: Decide> SpiricArc<T> {
    /// The arc `span` of `carrier`, or `None` for any other carrier.
    ///
    /// With `ρ = R + r·cos v` and `f = √(ρ² − o²)`, `P = c + u·o + m·f +
    /// axis·(r·sin v)`, so `P′ = m·f′ + axis·(r·cos v)` and `P″ = m·f″ −
    /// axis·(r·sin v)`. The speed bound is the carrier's own,
    /// `r(R − r)/f_min` with `f_min = √((R − r)² − o²)`. From `f′ = ρρ′/f`,
    /// `f″ = (ρ′² + ρρ″)/f − (ρρ′)²/f³`, and `|ρ′|, |ρ″| ≤ r`, `ρ ≤ R + r`,
    /// `f ≥ f_min` give `|f″| ≤ r(r + R + r)/f_min + ((R + r)·r)²/f_min³`;
    /// `A` is that plus `r`.
    pub(crate) fn of(carrier: &geom::Curve3<T>, span: (T, T)) -> Option<Self> {
        let geom::Curve3::Spiric {
            major_radius: big,
            minor_radius: r,
            offset,
            ..
        } = *carrier
        else {
            return None;
        };
        let inner = big - r;
        let outer = big + r;
        let f_min = (inner.powi(2) - offset.powi(2)).sqrt();
        let speed = r * inner / f_min;
        let accel = r * (r + outer) / f_min + (outer * r).powi(2) / f_min.powi(3) + r;
        Some(Self {
            carrier: carrier.clone(),
            span,
            speed,
            accel,
        })
    }

    /// The point at carrier parameter `v`.
    fn point(&self, v: T) -> Point3<T> {
        self.carrier.eval(v)
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

    /// A piece's half-width, and the ball `(P(v_m), S·h)` holding it.
    fn ball(&self, p: &Piece<T>) -> (T, Point3<T>, T) {
        let h = (p.vb - p.va).abs() * T::from_f64(0.5);
        let c = self.point(geom::mid_param(p.va, p.vb));
        (h, c, self.speed * h)
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
    /// piece's ball is definitely clear of `q`.
    ///
    /// # Errors
    ///
    /// An in-band distance to an end; or, where some piece is settled
    /// neither way by the time its ball is a point (or by the depth, or
    /// the piece budget) and no other piece puts `q` on the arc, the
    /// in-band margin that piece was read on — `rows.depth` when none
    /// was.
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
        // The first piece no halving could settle, by the margin it
        // was last read in the band on.
        let mut unsettled: Option<Indeterminate> = None;
        while let Some(p) = stack.pop() {
            visits += 1;
            let (_, c, reach) = self.ball(&p);
            let gap = (q - c).norm();
            let mut read = None;
            match decide(rows.clear, Margin::of(gap - reach), band) {
                Ok(Sign::Positive) => continue,
                Ok(_) => {}
                Err(diag) => read = Some(diag),
            }
            match decide(rows.on, Margin::of(gap), band) {
                Ok(Sign::Zero) => return Ok(SpiricHit::On),
                Ok(_) => {}
                Err(diag) => read = Some(diag),
            }
            let leaf = matches!(decide(rows.leaf, Margin::of(reach), band), Ok(Sign::Zero));
            if leaf || p.depth >= MAX_DEPTH || visits >= MAX_PIECES {
                unsettled.get_or_insert_with(|| {
                    read.unwrap_or_else(|| crate::invalid_margin::invalid(band, rows.depth))
                });
                if visits >= MAX_PIECES {
                    break;
                }
                continue;
            }
            self.split(p, &mut stack);
        }
        if let Some(diag) = unsettled {
            return Err(diag);
        }
        Ok(SpiricHit::Off)
    }

    /// **How many times the ray `q + d·t`, `t > 0`, crosses the arc**,
    /// counted in the loop's plane (`side` the in-plane unit normal to
    /// `d`). `None` where a piece meeting the ray settles neither its
    /// crossing nor its miss by the depth — a graze, or a crossing at a
    /// piece's end — which abandons the ray (why that is sound: the ray
    /// loop in [`super::containment`]'s walk). The point must be
    /// definitely off the arc ([`Self::contact`]).
    ///
    /// A piece is settled when its ball definitely misses the ray, or
    /// when `s(v) = (P(v) − q)·side` is definitely monotone on it with
    /// both ends' `s` definitely signed: same signs, no crossing;
    /// opposite, exactly one, counted when the ball is definitely ahead
    /// of `q` along `d` and not when it is definitely behind.
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
            let (h, c, reach) = self.ball(&p);
            let w = c - q;
            let (along, off) = (w.dot(d), w.dot(side));
            // The in-plane distance from the ball's centre to the ray.
            let nearest = (off.powi(2) + along.min(T::zero()).powi(2)).sqrt();
            if positive(CROSS_REACH, nearest - reach, band) {
                continue;
            }
            let (sa, sb) = ((p.pa - q).dot(side), (p.pb - q).dot(side));
            let ends = (
                decide(CROSS_SIDE, Margin::of(sa), band),
                decide(CROSS_SIDE, Margin::of(sb), band),
            );
            let four = T::from_f64(4.0);
            let monotone = positive(
                CROSS_TURN,
                (sb - sa).abs() - four * self.accel * h.powi(2),
                band,
            );
            if let (
                true,
                Ok(a @ (Sign::Positive | Sign::Negative)),
                Ok(b @ (Sign::Positive | Sign::Negative)),
            ) = (monotone, ends.0, ends.1)
            {
                if a == b {
                    continue;
                }
                if positive(CROSS_ADVANCE, along - reach, band) {
                    count += 1;
                    continue;
                }
                if matches!(
                    decide(CROSS_ADVANCE, Margin::of(along + reach), band),
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

/// Whether `margin` is definitely positive on `row`.
fn positive<T: Decide>(row: &'static str, margin: T, band: Band) -> bool {
    matches!(decide(row, Margin::of(margin), band), Ok(Sign::Positive))
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
        depth: "test_spiric_depth",
    };

    /// The oval `R = 2, r = 0.5` cut at `x = 0.8`, in the plane's
    /// `(y, z)` coordinates, and its arc `v ∈ [−2, 2]`.
    fn arc<T: Decide>() -> SpiricArc<T> {
        let f = T::from_f64;
        let carrier = geom::Curve3::Spiric {
            center: Point3::new(f(0.0), f(0.0), f(0.0)),
            axis: Vec3::new(f(0.0), f(0.0), f(1.0)),
            u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
            major_radius: f(2.0),
            minor_radius: f(0.5),
            offset: f(0.8),
        };
        SpiricArc::of(&carrier, (f(-2.0), f(2.0))).expect("a spiric")
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

    /// **The speed and acceleration bounds hold, and the speed bound is
    /// nearly tight.** Sampled densely over whole ovals of three tori —
    /// a fat ring, a thin one, and `R = 10, r = 1` cut at `offset = 5`,
    /// where the oval's speed near `v = π/2` comes within 4% of its bound
    /// — `|P′|` never exceeds `S` and `|P″|` never exceeds `A`; on the
    /// third, the sampled speed reaches 95% of `S`, so a bound a tenth
    /// smaller would let a piece's ball miss its own arc.
    #[test]
    fn the_carriers_bounds_hold_and_the_speed_bound_is_nearly_tight() {
        for (big, r, offset) in [(2.0, 1.0, 0.5), (3.0, 0.2, 2.5), (10.0, 1.0, 5.0)] {
            let carrier = geom::Curve3::Spiric {
                center: Point3::new(0.0, 0.0, 0.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
                major_radius: big,
                minor_radius: r,
                offset,
            };
            let k = SpiricArc::of(&carrier, (0.0, 1.0)).expect("a spiric");
            let (mut speed, mut accel) = (0.0_f64, 0.0_f64);
            for i in 0..20_000 {
                let v = core::f64::consts::TAU * f64::from(i) / 20_000.0;
                speed = speed.max(carrier.deriv(v).norm());
                accel = accel.max(carrier.deriv2(v).norm());
            }
            let torus = format!("R = {big}, r = {r}, offset = {offset}");
            assert!(
                speed <= k.speed,
                "{torus}: speed {speed} over its bound {}",
                k.speed
            );
            assert!(
                accel <= k.accel,
                "{torus}: acceleration {accel} over its bound {}",
                k.accel
            );
            if big == 10.0 {
                assert!(
                    speed >= 0.95 * k.speed,
                    "{torus}: {speed} against {}",
                    k.speed
                );
            }
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
