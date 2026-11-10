//! **ψ through the one table is the old spelling, bit for bit, in
//! sector** (PR 6500). Before the image read its channels through
//! [`geom::ChartChannels`] it spelled them itself; that spelling is
//! kept here as the oracle (D3: a second method exists only as a test
//! oracle) and the routed ψ is compared with it bitwise.
//!
//! The comparison is in sector, where a certified piece is read: the
//! point's radial part, rotated back by the piece's azimuth centre (and
//! flipped through a mirror nappe), has `x′ > 1e-12`, and on a torus
//! the tube direction rotated back by its centre does too. One
//! difference is allowed, the image's moved baseline: the sphere
//! latitude may differ in the sign of a zero (`one_reading_rows`'s
//! `the_sphere_latitude_at_negative_zero_height_reads_positive_zero`).
//! At `Interval` the comparison holds under the backend's 2Prod
//! validity floor (`2^-960`) too: the old spelling multiplies the radial
//! part by `σ = ±1`, and a product by a point power of two is exact at
//! every normal magnitude.
//!
//! The draws are a fixed seed: a fixture identifier, so the row reads the
//! same points on every run and at every scalar. The review of PR 6500
//! ran the same comparison over about 27 M `f64` reads, Interval boxes
//! and `Dual`s and found the same.

use geom::Quarter;
use geom_core::{Bounds, Interval, Real, Vec3};

use super::{FramedCarrier, ProjectedChart, ProjectedImage, breaks_vector, rotate_box};
use crate::offset::Nappe;

/// The rotation the old spelling wrote out.
fn old_quarter<T: Real>(m: i32, x: T, y: T) -> (T, T) {
    match m.rem_euclid(4) {
        0 => (x, y),
        1 => (y, T::zero() - x),
        2 => (T::zero() - x, T::zero() - y),
        _ => (T::zero() - y, x),
    }
}

/// The old `branch_azimuth`.
fn old_branch<T: Real>(m: i32, x: T, y: T) -> T {
    let (xr, yr) = old_quarter(m, x, y);
    T::pi() * T::from_f64(0.5 * f64::from(m)) + yr.atan2(xr)
}

/// The old `ProjectedImage::raw`.
fn old_raw<T: Real>(chart: &ProjectedChart<T>, m: i32, n: i32, p: Vec3<T>) -> (T, T) {
    if matches!(chart, ProjectedChart::Plane) {
        return (p.x, p.y);
    }
    let sigma = chart.sigma();
    let u = old_branch(m, p.x * sigma, p.y * sigma);
    let rho = (p.x.powi(2) + p.y.powi(2)).sqrt();
    let v = match *chart {
        ProjectedChart::Plane => unreachable!("answered above"),
        ProjectedChart::Cylinder => p.z,
        ProjectedChart::Cone { sin, cos, .. } => p.z * cos + sigma * rho * sin,
        ProjectedChart::Sphere => p.z.atan2(rho),
        ProjectedChart::Torus { major } => old_branch(n, rho - major, p.z),
    };
    (u, v)
}

/// A fixed-seed draw: coordinates at every scale the review read,
/// signed zeros among them.
struct Draw(u64);

impl Draw {
    fn unit(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        #[allow(clippy::cast_precision_loss)]
        let r = (self.0 >> 11) as f64 / (1u64 << 53) as f64;
        r
    }

    fn coord(&mut self) -> f64 {
        let (r, s) = (self.unit(), if self.unit() < 0.5 { -1.0 } else { 1.0 });
        let scale = match (r * 10.0).floor() {
            0.0 => 0.0,
            1.0 => 1e-300,
            2.0 => 1e-9,
            3.0 => 1e-15,
            _ => 3.0,
        };
        s * scale * self.unit()
    }
}

/// The charts the review read: every kind, needle to flat cones on both
/// nappes, tori from ring to a zero major radius.
fn charts<T: Real>(lift: impl Fn(f64) -> T) -> Vec<ProjectedChart<T>> {
    let mut out = vec![
        ProjectedChart::Plane,
        ProjectedChart::Cylinder,
        ProjectedChart::Sphere,
    ];
    for a in [0.05f64, 0.4, 1.3, 1.55] {
        for nappe in [Nappe::Opening, Nappe::Mirror] {
            out.push(ProjectedChart::Cone {
                sin: lift(a.sin()),
                cos: lift(a.cos()),
                nappe,
            });
        }
    }
    for major in [2.0, 1.0, 0.5, 1e-9, 0.0] {
        out.push(ProjectedChart::Torus { major: lift(major) });
    }
    out
}

/// The scalar's view for the rows: its bits, its lower end, and whether
/// it has an end under the 2Prod floor.
struct View<T> {
    bits: fn(T) -> Vec<u64>,
    rep: fn(T) -> f64,
    under_floor: fn(T) -> bool,
}

/// The rows' one body: `points` draws per chart, every centre pair,
/// answering how many in-sector reads it compared and how many of those
/// had a coordinate under the floor.
fn compare<T: Real>(
    lift: impl Fn(f64) -> T + Copy,
    point: impl Fn(&mut Draw) -> Vec3<T>,
    view: &View<T>,
    points: usize,
) -> (usize, usize) {
    let View {
        bits,
        rep,
        under_floor,
    } = *view;
    let mut draw = Draw(0x6500);
    let (mut compared, mut floored) = (0, 0);
    for chart in charts(lift) {
        let sigma = match chart {
            ProjectedChart::Cone {
                nappe: Nappe::Mirror,
                ..
            } => -1.0,
            _ => 1.0,
        };
        for _ in 0..points {
            let p = point(&mut draw);
            let (px, py, pz) = (rep(p.x), rep(p.y), rep(p.z));
            for m in -4..8 {
                if !matches!(chart, ProjectedChart::Plane)
                    && old_quarter(m, sigma * px, sigma * py).0 <= 1e-12
                {
                    continue;
                }
                for n in [-1, 0, 1, 2, 3] {
                    if let ProjectedChart::Torus { major } = chart {
                        let rho = (px * px + py * py).sqrt();
                        if old_quarter(n, rho - rep(major), pz).0 <= 1e-12 {
                            continue;
                        }
                    }
                    let image = ProjectedImage {
                        chart,
                        carrier: FramedCarrier::Line {
                            point: Vec3::zero(),
                            dir: Vec3::zero(),
                            origin: T::zero(),
                            span: T::one(),
                        },
                        breaks: breaks_vector(&[0.0, 1.0]),
                        azimuth: vec![Quarter(m)],
                        tube: vec![Quarter(n)],
                        u_off: T::zero(),
                        v_off: T::zero(),
                        v_sign: T::one(),
                    };
                    let new = image.raw(p, 0);
                    let old = old_raw(&chart, m, n, p);
                    let same_v = bits(new.y) == bits(old.1)
                        || (matches!(chart, ProjectedChart::Sphere)
                            && bits(new.y)
                                .iter()
                                .zip(bits(old.1))
                                .all(|(&a, b)| f64::from_bits(a) == f64::from_bits(b)));
                    let same = bits(new.x) == bits(old.0) && same_v;
                    floored +=
                        usize::from(under_floor(p.x) || under_floor(p.y) || under_floor(p.z));
                    assert!(
                        same,
                        "{:?} m {m} n {n} at {:?}: new ({:?}, {:?}), old ({:?}, {:?})",
                        chart.kind(),
                        (bits(p.x), bits(p.y), bits(p.z)),
                        bits(new.x),
                        bits(new.y),
                        bits(old.0),
                        bits(old.1),
                    );
                    compared += 1;
                }
            }
        }
    }
    (compared, floored)
}

/// **ψ is the old spelling bit for bit, in sector**, at `f64`.
#[test]
fn psi_is_the_old_spelling_bit_for_bit_at_f64() {
    let view = View {
        bits: |x: f64| vec![x.to_bits()],
        rep: |x| x,
        under_floor: |_| false,
    };
    let (compared, _) = compare(
        |x| x,
        |d| Vec3::new(d.coord(), d.coord(), d.coord()),
        &view,
        300,
    );
    assert!(compared > 10_000, "compared {compared} reads");
}

/// The same at `Interval`, on boxes up to `0.3` wide; the bits compared
/// are both ends of each enclosure, under the 2Prod floor as above it
/// (module docs).
#[test]
fn psi_is_the_old_spelling_bit_for_bit_at_interval() {
    /// The backend's 2Prod validity floor, `2^-960`
    /// (`interval-transcendentals`' `TWO_PROD_VALID_MIN`).
    const FLOOR: f64 = f64::from_bits(0x03F0_0000_0000_0000);
    let wide = |d: &mut Draw| {
        let a = d.coord();
        let w = [0.0, 1e-12, 1e-6, 0.3][(d.unit() * 4.0).floor() as usize % 4];
        Interval::from_bounds(a, a + w)
    };
    let view = View {
        bits: |x: Interval| vec![x.lo().to_bits(), x.hi().to_bits()],
        rep: Interval::lo,
        under_floor: |x: Interval| {
            [x.lo(), x.hi()]
                .iter()
                .any(|e| e.abs() > 0.0 && e.abs() < FLOOR)
        },
    };
    let (compared, floored) = compare(
        Interval::from_f64,
        |d| Vec3::new(wide(d), wide(d), wide(d)),
        &view,
        60,
    );
    assert!(compared > 2_000, "compared {compared} reads");
    // The draws reach the floor (the `1e-300` class), so the rows read
    // under it are compared, not skipped.
    assert!(floored > 0, "no read under the floor was compared");
}

/// **The box rotation is the quarter's on its corners**: at `f64`, where
/// a range is two ordered numbers, `rotate_box` reads the min and max of
/// [`Quarter::rotate`] at the box's two corners, at every quarter.
#[test]
fn the_box_rotation_is_the_quarters_on_its_corners() {
    let mut draw = Draw(0xb0c5);
    for _ in 0..200 {
        let (a, b, c, d) = (draw.coord(), draw.coord(), draw.coord(), draw.coord());
        let (x, y) = ((a.min(b), a.max(b)), (c.min(d), c.max(d)));
        for m in -4..8 {
            let q = Quarter(m);
            let ((x0, y0), (x1, y1)) = (q.rotate(x.0, y.0), q.rotate(x.1, y.1));
            let want = ((x0.min(x1), x0.max(x1)), (y0.min(y1), y0.max(y1)));
            let got = rotate_box(q, x, y);
            assert!(
                got.0.0 == want.0.0
                    && got.0.1 == want.0.1
                    && got.1.0 == want.1.0
                    && got.1.1 == want.1.1,
                "m {m}, box {x:?} × {y:?}: {got:?}, the corners' {want:?}"
            );
        }
    }
}
