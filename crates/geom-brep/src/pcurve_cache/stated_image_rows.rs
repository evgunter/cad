//! **The edge certificate bounds a stated chart image over its span**
//! (`certify.rs` `stated_bound`): a carrier off its spline wall anywhere
//! on the span, between every sample of the edge's schedule and of the
//! incidence read alike, does not certify, and ends read off the chart.
//!
//! The wall is `spline_door_rows`' ring (the unit cylinder over
//! `[0, 1.2]` rad in two rational quadratic sub-arcs, `z ∈ [0, 1]`) and,
//! for a non-rational chart, the parabolic cylinder `y = x²` over
//! `x ∈ [−1, 1]` (one quadratic Bézier piece across, `z ∈ [0, 1]`). The
//! carrier is the column `u = ½` (the ruling at `0.6` rad, or at `x = 0`)
//! as a polyline whose one vertex is pushed off the wall along its normal.
//! It is stated by the column as an iso line ([`Pcurve::IsoLine`]) or as a
//! curve in UV ([`Pcurve::General`]).
//!
//! Built with `PCERT_RADIAL_PAIR_MUTANT=stated-image-sampled` (the edge
//! certificate's stated image metered at its schedule's samples again),
//! every row that reads a bump between the samples goes red.

test_utils::gated_to![
    "crates/geom-brep/src/certify.rs",
    "crates/geom-brep/src/pcurve_cache/stated.rs",
    "crates/geom-brep/src/pcurve_cache/spline_incidence.rs",
];

use std::sync::Arc;

use geom::{Curve3, NurbsCurve2, NurbsCurve3, NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Decide, Interval, Point2, Point3, Vec2, Vec3};

use super::Pcurve;
use super::spline_door_rows::{band, in_band, ring};
use crate::{
    CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, IncidenceRead, NurbsLane,
    SurfaceKey,
};

/// A chart the rows read, with its column `u = ½` in 3-D at height `z`
/// and that point's unit normal.
#[derive(Clone, Copy, Debug)]
enum Wall {
    /// The rational ring.
    Ring,
    /// The non-rational parabolic cylinder.
    Parabola,
}

impl Wall {
    fn surface<T: Decide>(self) -> Surface<T> {
        match self {
            Self::Ring => ring::<T>(),
            Self::Parabola => {
                let mut control = Vec::new();
                for x in [-1.0, 0.0, 1.0] {
                    // The quadratic Bézier `(x, x²)` has the middle
                    // control point at `(0, −1)`.
                    let y = if x == 0.0 { -1.0 } else { 1.0 };
                    for z in [0.0, 1.0] {
                        control.push(Point3::new(x, y, z));
                    }
                }
                let net = NurbsSurface::new(
                    KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
                    KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
                    control,
                    vec![1.0; 6],
                )
                .unwrap();
                Surface::Nurbs(Arc::new(net.map_scalar(T::from_f64)))
            }
        }
    }

    /// The column's foot at height `z` and the wall's outward unit
    /// normal there (the column is a straight ruling on both walls).
    fn column_at(self) -> ([f64; 3], [f64; 3]) {
        match self {
            Self::Ring => {
                let a: f64 = 0.6;
                ([a.cos(), a.sin(), 0.0], [a.cos(), a.sin(), 0.0])
            }
            // `u = ½` is `x = 0`, the parabola's vertex: the normal is `y`.
            Self::Parabola => ([0.0, 0.0, 0.0], [0.0, -1.0, 0.0]),
        }
    }

    /// The exact distance of `p` from the wall, near the column
    /// (an oracle in `f64`, independent of the kernel's reads).
    fn distance(self, p: [f64; 3]) -> f64 {
        match self {
            Self::Ring => (p[0].hypot(p[1]) - 1.0).abs(),
            // Nearest point `(x, x²)` to `(px, py)`: the root of the
            // cubic `2x³ + (1 − 2py)x − px = 0`, by Newton from `px`.
            Self::Parabola => {
                let (px, py) = (p[0], p[1]);
                let mut x = px;
                for _ in 0..60 {
                    let f = 2.0 * x * x * x + (1.0 - 2.0 * py) * x - px;
                    let df = 6.0 * x * x + 1.0 - 2.0 * py;
                    x -= f / df;
                }
                (px - x).hypot(py - x * x)
            }
        }
    }
}

/// The column polyline rising `z = t` over `t ∈ [0, 1]`, degree
/// `degree`, with the control point at `peak` pushed `height` off the
/// wall along its normal, the bump's neighbours `width` either side.
/// `height` zero is the column itself, on the wall, with its extra knots.
#[derive(Clone, Copy, Debug)]
struct Bump {
    degree: usize,
    peak: f64,
    width: f64,
    height: f64,
}

impl Bump {
    /// R2: a degree-1 bump a millimetre high, wholly inside `(0, 3/256)`,
    /// between every residual sample (`i/8`) and every read sample
    /// (`i/64`).
    const R2: Self = Self {
        degree: 1,
        peak: 1.0 / 128.0,
        width: 1.0 / 256.0,
        height: 1e-3,
    };

    fn net(self, wall: Wall) -> NurbsCurve3<f64> {
        let (foot, normal) = wall.column_at();
        let mut ts = vec![
            0.0,
            self.peak - self.width,
            self.peak,
            self.peak + self.width,
            1.0,
        ];
        ts.dedup();
        let p = self.degree;
        // Control parameters: the column's points at `ts`, the middle one
        // pushed; a clamped knot vector whose Greville abscissae are `ts`
        // at degree 1, and are spread over them at a higher degree.
        let control: Vec<Point3<f64>> = ts
            .iter()
            .map(|&t| {
                let h = if t == self.peak { self.height } else { 0.0 };
                Point3::new(foot[0] + h * normal[0], foot[1] + h * normal[1], t)
            })
            .collect();
        let n = control.len();
        assert!(n > p, "a bump needs more control points than its degree");
        let mut knots = vec![0.0; p + 1];
        for j in 1..n - p {
            // The averaging rule: interior knots at the means of the
            // control parameters, so the net is on its column at zero
            // height.
            knots.push(ts[j..j + p].iter().sum::<f64>() / p as f64);
        }
        knots.extend(vec![1.0; p + 1]);
        NurbsCurve3::new(
            KnotVector::clamped(knots, p).unwrap(),
            control,
            vec![1.0; n],
        )
        .unwrap()
    }

    /// The carrier's largest distance from the wall, by a dense walk in
    /// `f64` and at the bump's own peak (the oracle the rows' verdicts are checked against).
    fn miss(self, wall: Wall) -> f64 {
        let net = self.net(wall);
        let n = 1u32 << 13;
        (0..=n)
            .map(|i| f64::from(i) / f64::from(n))
            .chain([self.peak])
            .map(|t| {
                let q = net.eval(t);
                wall.distance([q.x, q.y, q.z])
            })
            .fold(0.0, f64::max)
    }

    fn carrier<T: Decide>(self, wall: Wall) -> Curve3<T> {
        Curve3::Nurbs(Arc::new(self.net(wall).map_scalar(T::from_f64)))
    }
}

/// The column `u = ½`, `v = t`, stated as an iso line or a curve in UV.
#[derive(Clone, Copy, Debug)]
enum Stated {
    Iso,
    General,
}

impl Stated {
    fn image<T: Decide>(self) -> Pcurve<T> {
        let c = T::from_f64;
        match self {
            Self::Iso => Pcurve::IsoLine {
                p0: Point2::new(c(0.5), c(0.0)),
                pl: Vec2::new(c(0.0), c(1.0)),
            },
            Self::General => {
                let image = NurbsCurve2::new(
                    KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
                    vec![Point2::new(0.5, 0.0), Point2::new(0.5, 1.0)],
                    vec![1.0, 1.0],
                )
                .unwrap();
                Pcurve::General(Arc::new(image.map_scalar(T::from_f64)))
            }
        }
    }
}

/// The edge certificate of a carrier on a wall, described by a stated
/// image, at one scalar that may certify.
type Edge<T> = fn(Wall, Curve3<T>, Stated) -> Result<(), CertifyError>;

macro_rules! edge_at {
    ($name:ident, $t:ty) => {
        fn $name(wall: Wall, carrier: Curve3<$t>, stated: Stated) -> Result<(), CertifyError> {
            let c = <$t as geom_core::Real>::from_f64;
            let mut table: slotmap::SlotMap<SurfaceKey, Surface<$t>> = slotmap::SlotMap::with_key();
            let key = table.insert(wall.surface::<$t>());
            let (start, end) = (carrier.eval(c(0.0)), carrier.eval(c(1.0)));
            let spec = EdgeCurveSpec {
                description: EdgeDescriptionSpec::chart_image(key, stated.image()),
                carrier,
                param_start: c(0.0),
                param_end: c(1.0),
            };
            EdgeCurve::certify(
                spec,
                start,
                end,
                |k| table.get(k).cloned(),
                geom_core::test_support::fixture_lever(<$t as geom_core::Real>::one()),
                band(),
                NurbsLane::certified(),
            )
            .map(|_| ())
        }
    };
}
edge_at!(edge_f64, f64);
edge_at!(edge_interval, Interval);

fn read(got: &Result<(), CertifyError>) -> Option<IncidenceRead> {
    got.as_ref().err().and_then(CertifyError::incidence_read)
}

fn is_off(got: &Result<(), CertifyError>) -> bool {
    matches!(read(got), Some(IncidenceRead::Off(_)))
}

/// **R2**: the bump between both sample sets ends read off the chart at
/// both scalars and through both stated images, never certified.
fn r2<T: Decide>(edge: Edge<T>) {
    for stated in [Stated::Iso, Stated::General] {
        let got = edge(Wall::Ring, Bump::R2.carrier(Wall::Ring), stated);
        assert!(
            is_off(&got),
            "R2 ({stated:?}, at {}): a carrier a millimetre off its wall between the samples \
             ends off the chart: {got:?}",
            T::NAME
        );
    }
}

#[test]
fn r2_a_bump_between_the_samples_ends_off_at_f64() {
    r2::<f64>(edge_f64);
}

#[test]
fn r2_a_bump_between_the_samples_ends_off_at_interval() {
    r2::<Interval>(edge_interval);
}

/// **A bump narrowed toward the band**: in band of the wall it ends in
/// the read's lever ([`IncidenceRead::Undecided`]); just past the band,
/// read off. Neither certifies.
fn toward_the_band<T: Decide>(edge: Edge<T>) {
    let b = band();
    for (height, want_off) in [(in_band(), false), (4.0 * b.escalate(), true)] {
        let bump = Bump { height, ..Bump::R2 };
        let got = edge(Wall::Ring, bump.carrier(Wall::Ring), Stated::General);
        let ok = if want_off {
            is_off(&got)
        } else {
            matches!(read(&got), Some(IncidenceRead::Undecided(_)))
        };
        assert!(
            ok,
            "a bump {height:e} off the wall (band {:e}..{:e}), at {}: {got:?}",
            b.zero(),
            b.escalate(),
            T::NAME
        );
    }
}

#[test]
fn a_bump_toward_the_band_is_read_at_f64() {
    toward_the_band::<f64>(edge_f64);
}

#[test]
fn a_bump_toward_the_band_is_read_at_interval() {
    toward_the_band::<Interval>(edge_interval);
}

/// **A carrier on its wall still certifies**, on the rational ring and
/// the non-rational parabola: the ruling as a line stated by its iso
/// column, and as a polyline with extra knots (whose structure is not
/// the column's, so the iso lane does not hold it) stated by its curve
/// in UV.
fn on_the_wall<T: Decide>(edge: Edge<T>) {
    let mut misread = Vec::new();
    for wall in [Wall::Ring, Wall::Parabola] {
        let (foot, _) = wall.column_at();
        let line = Curve3::line(
            Point3::new(T::from_f64(foot[0]), T::from_f64(foot[1]), T::zero()),
            Vec3::new(T::zero(), T::zero(), T::one()),
        );
        let flat = Bump {
            height: 0.0,
            ..Bump::R2
        };
        for (what, carrier, stated) in [
            ("line", line, Stated::Iso),
            ("polyline", flat.carrier::<T>(wall), Stated::General),
        ] {
            let got = edge(wall, carrier, stated);
            if got.is_err() {
                misread.push(format!("{wall:?} {what} ({stated:?}): {got:?}"));
            }
        }
    }
    assert!(
        misread.is_empty(),
        "a carrier on its wall certifies, at {}:\n{}",
        T::NAME,
        misread.join("\n")
    );
}

#[test]
fn a_carrier_on_its_wall_certifies_at_f64() {
    on_the_wall::<f64>(edge_f64);
}

#[test]
fn a_carrier_on_its_wall_certifies_at_interval() {
    on_the_wall::<Interval>(edge_interval);
}

/// **Counterexample search** (shape 1): bumps of every degree, place and
/// width on both walls, checked against the oracle's distance. A bump the
/// oracle puts past the band never certifies, and one past a micrometre
/// ends read off; a flat one (height zero) never reads off.
#[test]
fn no_bump_certifies_past_the_band() {
    let b = band();
    let mut rng = test_utils::fuzz::start("stated_image_rows");
    let mut bad = Vec::new();
    for _ in 0..test_utils::fuzz::scaled(24) {
        let wall = if rng.below(2) == 0 {
            Wall::Ring
        } else {
            Wall::Parabola
        };
        let width = rng.log_uniform(1e-4, 1.0 / 64.0);
        let bump = Bump {
            degree: 1 + rng.below(3),
            peak: rng.range(2.0 * width, 1.0 - 2.0 * width),
            width,
            height: [0.0, 1e-3, 1e-6, 4.0 * b.escalate()][rng.below(4)],
        };
        let miss = bump.miss(wall);
        let got = edge_f64(wall, bump.carrier(wall), Stated::General);
        let wrong = if bump.height == 0.0 {
            is_off(&got)
        } else if miss > 1e-6 {
            !is_off(&got)
        } else {
            miss > 2.0 * b.escalate() && got.is_ok()
        };
        if wrong {
            bad.push(format!("{wall:?} {bump:?} (oracle miss {miss:e}): {got:?}"));
        }
    }
    assert!(
        bad.is_empty(),
        "a bump past the band certified, or a flat carrier read off ({}):\n{}",
        test_utils::fuzz::replay(),
        bad.join("\n")
    );
}
