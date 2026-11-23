//! **A spiric edge of a planar loop, read on its own carrier** — the
//! boundary reading the in-plane walk ([`super::containment`]) runs
//! beside its conic row, and its ray crossing count.
//!
//! The oval has no closed-form foot the walk could certify cheaply, so
//! the boundary reading subdivides the arc's parameter window into
//! pieces and certifies each piece from a bound over the piece's own
//! window, read off the carrier's data ([`geom::spiric_window_rate_bounds`]):
//! its speed `|P′| ≤ S`. A piece of half-width `h` about `v_m` lies in
//! the ball of radius `S·h` about `P(v_m)`. The halving and the reading
//! over the pieces are [`super::pieced_arc`]'s.
//!
//! A ray in the loop's plane meets the oval where the oval crosses the
//! ray's square plane, which the spiric × plane door reads exactly (the
//! line where that plane meets the oval's is the ray's own), so the
//! count is a conic arc's ([`super::containment::arc_crossings`]).

use geom_core::{Band, Decide, Indeterminate, Point3, SupSpeed, Vec3};

use crate::geometry::SurfaceKey;
use crate::splitting::pieced_arc::{PieceBall, PiecedArc};

/// A spiric oval: [`geom::Curve3::Spiric`]'s data, named so that a
/// [`SpiricArc`] can only be built from one.
#[derive(Clone, Copy)]
pub(crate) struct Oval<T: geom_core::Real> {
    /// The stored spiric's frame, carried.
    pub(crate) frame: geom::Frame<T>,
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
pub(crate) struct Piece<T: geom_core::Real> {
    va: T,
    vb: T,
    pa: Point3<T>,
    pb: Point3<T>,
    depth: u32,
}

/// A piece's ball `(P(v_m), S·h)`, `h` its half-width.
struct Bounds<T: geom_core::Real> {
    center: Point3<T>,
    reach: T,
}

impl<T: Decide> SpiricArc<T> {
    /// The arc `span` of `oval`.
    pub(crate) fn new(oval: Oval<T>, span: (T, T)) -> Self {
        Self { oval, span }
    }

    /// The oval's carrier.
    fn carrier(&self) -> geom::Curve3<T> {
        let o = self.oval;
        geom::Curve3::Spiric {
            frame: o.frame,
            major_radius: o.major,
            minor_radius: o.minor,
            offset: o.offset,
        }
    }

    /// The point at carrier parameter `v`.
    fn point(&self, v: T) -> Point3<T> {
        self.carrier().eval(v)
    }

    /// A piece's bounds over its own window: `cos v` and `sin v` each
    /// move at most `h` from their values at `v_m`, so `ρ = R + r·cos v`
    /// stays in `[R + r·max(cos v_m − h, −1), R + r·min(cos v_m + h, 1)]`
    /// and `|sin v| ≤ min(|sin v_m| + h, 1)`.
    fn bounds(&self, p: &Piece<T>) -> Bounds<T> {
        let o = self.oval;
        let h = (p.vb - p.va).abs() * T::from_f64(0.5);
        let vm = geom::mid_param(p.va, p.vb);
        let (speed, _) = geom::spiric_window_rate_bounds((o.major, o.minor, o.offset), vm, h);
        Bounds {
            center: self.point(vm),
            reach: geom_core::bisect::reach(SupSpeed::new(speed), (p.va, p.vb)),
        }
    }

    /// **How many times the ray `q + d·t`, `t > 0`, crosses the arc**, an
    /// edge of a loop of unit normal `normal` whose face's record is
    /// `face`: the arc across the ray's square plane
    /// ([`super::containment::arc_crossings`]). `None` grazes the ray;
    /// `on_carrier` says the boundary pass put `q` on the oval and off
    /// the arc.
    ///
    /// # Errors
    ///
    /// An in-band margin, which sets the ray aside.
    pub(in crate::splitting) fn crossings(
        &self,
        face: SurfaceKey,
        on_carrier: bool,
        normal: Vec3<T>,
        (q, d): (Point3<T>, Vec3<T>),
        band: Band,
    ) -> Result<Option<usize>, Indeterminate> {
        super::containment::arc_crossings(
            face,
            (&self.carrier(), self.span),
            on_carrier,
            normal,
            q,
            d,
            band,
        )
    }
}

impl<T: Decide> PiecedArc<T> for SpiricArc<T> {
    type Piece = Piece<T>;

    fn ends(&self) -> [Point3<T>; 2] {
        [self.point(self.span.0), self.point(self.span.1)]
    }

    /// The whole arc as one piece.
    fn roots(&self) -> Vec<Piece<T>> {
        let (va, vb) = self.span;
        vec![Piece {
            va,
            vb,
            pa: self.point(va),
            pb: self.point(vb),
            depth: 0,
        }]
    }

    fn depth(piece: &Piece<T>) -> u32 {
        piece.depth
    }

    /// A piece's two halves, the half nearer `va` first.
    fn halves(&self, p: Piece<T>) -> [Piece<T>; 2] {
        let vm = geom::mid_param(p.va, p.vb);
        let pm = self.point(vm);
        let depth = p.depth + 1;
        [
            Piece {
                va: p.va,
                vb: vm,
                pa: p.pa,
                pb: pm,
                depth,
            },
            Piece {
                va: vm,
                vb: p.vb,
                pa: pm,
                pb: p.pb,
                depth,
            },
        ]
    }

    /// The ball `(P(v_m), S·h)`, its centre a point of the arc.
    fn ball(&self, p: &Piece<T>) -> PieceBall<T> {
        let b = self.bounds(p);
        PieceBall {
            center: b.center,
            reach: b.reach,
            on: b.center,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::splitting::pieced_arc::{ArcHit, ArcRows};
    use geom_core::{Interval, Tol, Vec3};

    const ROWS: ArcRows = crate::rows::TEST_SPIRIC_ARC;

    /// The oval `R = 2, r = 0.5` cut at `x = 0.8`, in the plane's
    /// `(y, z)` coordinates, and its arc `v ∈ [−2, 2]`.
    fn arc<T: Decide>() -> SpiricArc<T> {
        let f = T::from_f64;
        SpiricArc::new(
            Oval {
                frame: geom::Frame::new(
                    Point3::new(f(0.0), f(0.0), f(0.0)),
                    Vec3::new(f(0.0), f(0.0), f(1.0)),
                    Vec3::new(f(1.0), f(0.0), f(0.0)),
                ),
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
        let normal = Vec3::new(f(1.0), f(0.0), f(0.0));
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
                    Ok(ArcHit::Off),
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
                let got = k.crossings(SurfaceKey::default(), false, normal, (point(q), d), band);
                let Ok(Some(count)) = got else {
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
        assert!(
            matches!(k.contact(mid, ROWS, band), Ok(ArcHit::On(_))),
            "{lane}: on the arc"
        );
        assert_eq!(
            k.contact(point(b), ROWS, band),
            Ok(ArcHit::End),
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
            frame: geom::Frame::new(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(1.0, 0.0, 0.0),
            ),
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
        let h = 0.5 * (vb - va).abs();
        let speed = k.bounds(&p).reach / h;
        let (_, accel) = geom::spiric_window_rate_bounds(
            (k.oval.major, k.oval.minor, k.oval.offset),
            geom::mid_param(va, vb),
            h,
        );
        let c = geom::Curve3::Spiric {
            frame: k.oval.frame,
            major_radius: k.oval.major,
            minor_radius: k.oval.minor,
            offset: k.oval.offset,
        };
        let (mut s, mut a) = (0.0_f64, 0.0_f64);
        for i in 0..=samples {
            let v = va + (vb - va) * f64::from(i) / f64::from(samples);
            s = s.max(c.deriv(v).norm() / speed);
            a = a.max(c.deriv2(v).norm() / accel);
        }
        (s, a)
    }

    /// **Each bound is nearly tight where it binds**, so a bound any
    /// much smaller would let a piece's ball miss its own arc. The speed bound is exact
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
    /// a ray from there is counted. A cut within the band of the pinch,
    /// which the mint refuses, refuses the ray typed on the regime's
    /// margin.
    #[test]
    fn a_near_tangent_cut_reads_and_crosses_a_far_point() {
        let band = Band::linear(Tol::witness()).unwrap();
        let gaps = [0.99, 0.9999, 0.999_999, 0.999_999_99]
            .map(|frac| 0.75 * (1.0 - frac))
            .into_iter()
            .chain([5e-10, 1e-11, 1e-13]);
        for gap in gaps {
            let (o, frac) = (0.75 - gap, gap);
            let k = SpiricArc::new(oval(1.0, 0.25, o), (-3.0, 3.0));
            let mid_y = 0.5 * (k.point(0.0).y + k.point(core::f64::consts::PI).y);
            let q = Point3::new(o, mid_y, 0.0);
            assert_eq!(k.contact(q, ROWS, band), Ok(ArcHit::Off), "gap {frac}");
            let d = Vec3::new(0.0, 0.3_f64.cos(), 0.3_f64.sin());
            let normal = Vec3::new(1.0, 0.0, 0.0);
            // One crossing: the ray leaves the oval's interior through
            // the arc (the gap `v ∈ [3, 2π − 3]` lies behind it). A cut
            // the mint refuses, within the band of the pinch, reads its
            // regime in band or past the inner equator: the ray is set
            // aside on the regime's margin, typed, never a fault.
            let got = k.crossings(SurfaceKey::default(), false, normal, (q, d), band);
            let minted = geom::Curve3::spiric(
                Point3::origin(),
                Vec3::unit_z(),
                Vec3::unit_x(),
                1.0,
                0.25,
                o,
                band,
            )
            .is_ok();
            if minted {
                assert!(matches!(got, Ok(Some(1))), "gap {frac}: {got:?}");
            } else {
                assert!(
                    matches!(got, Err(d) if d.predicate == Some("curve_face_pose_spiric_two_ovals")),
                    "gap {frac}: {got:?}"
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod fuzz;
