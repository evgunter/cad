//! **The torus's carrier-swap rows**: a torus face bounded by rims,
//! meridians and Villarceau circles (the spirics a bitangent plane cuts,
//! each a whole circle), with its circles rewritten as exact rational
//! quadratics so that each mints a projected image, and spiric walls read
//! through the lane's spiric arm.
//!
//! Every truth is independent of the lane. A chart rectangle's area and
//! flux about the centre are the area density `r·ρ` and the flux density
//! `r·ρ·(R·cos v + r)` integrated by composite Simpson in `v`; a band
//! between a Villarceau circle and its turn by `β` about the axis is
//! `β/2π` of the whole torus (the circles' turns tile the torus, and the
//! densities do not depend on `u`), whose area `4π²Rr` and flux `6π²Rr²`
//! (three times its volume) are closed forms; a complement is the whole
//! less the region. `A⃗ = ½∮(P − c)×P′` is read by composite Simpson on
//! each circle's own parametrization, so the flux about a point is the
//! flux about the centre plus `(c − about)·A⃗`, and the swapped carriers'
//! `A⃗` in the chart term is the lane's reading, so no bias cancels.
//!
//! The rows cover a loop that winds nothing in both senses and as the
//! complement, bands about the outer and the inner equator (loops that
//! wind the axis, the inner one across the tube angle's `±π`), a sector
//! between two meridians (loops that wind the tube), a band of
//! Villarceau circles (loops that wind both ways at once), a torus `1e3`
//! out, and the spiric arm: a closed loop of a spiric, two rims and a
//! meridian against a 2-D integral, and its junctions' closure.

use super::*;
use crate::props::spiric::{SpiricEdgeQ, WallImageQ};
use core::f64::consts::{PI, TAU};

/// A torus `c + (R + r·cos v)·(cos u·û + sin u·ŵ) + r·sin v·â`,
/// `ŵ = â × û`.
#[derive(Clone, Copy)]
struct Donut {
    c: Point3<f64>,
    axis: Vec3<f64>,
    u_ref: Vec3<f64>,
    big: f64,
    r: f64,
}

impl Donut {
    fn tilted(c: Point3<f64>, big: f64, r: f64) -> Self {
        let axis = Vec3::new(0.2, -0.3, 1.0);
        let axis = axis / axis.norm();
        let u_ref = axis.cross(Vec3::new(1.0, 0.0, 0.0));
        Self {
            c,
            axis,
            u_ref: u_ref / u_ref.norm(),
            big,
            r,
        }
    }

    fn w(self) -> Vec3<f64> {
        self.axis.cross(self.u_ref)
    }

    fn radial(self, u: f64) -> Vec3<f64> {
        self.u_ref * u.cos() + self.w() * u.sin()
    }

    fn surface(self) -> Surface<f64> {
        Surface::Torus {
            center: self.c,
            axis: self.axis,
            major_radius: self.big,
            minor_radius: self.r,
            u_ref: self.u_ref,
        }
    }

    /// `[area, flux about the centre]` of the whole torus under its
    /// outward normal: `4π²Rr` and three times its volume `2π²Rr²`.
    fn whole(self) -> [f64; 2] {
        [
            4.0 * PI * PI * self.big * self.r,
            6.0 * PI * PI * self.big * self.r * self.r,
        ]
    }

    /// `[area, flux about the centre]` of the chart rectangle
    /// `[u₀, u₁] × [v₀, v₁]` under the outward normal, the densities
    /// integrated by composite Simpson in `v`.
    fn rectangle(self, (u0, u1): (f64, f64), (v0, v1): (f64, f64)) -> [f64; 2] {
        let n = 4096_u32;
        let h = (v1 - v0) / f64::from(n);
        let mut sum = [0.0, 0.0];
        for i in 0..=n {
            let v = v0 + h * f64::from(i);
            let w = if i == 0 || i == n {
                1.0
            } else if i % 2 == 1 {
                4.0
            } else {
                2.0
            };
            let rho = self.big + self.r * v.cos();
            sum[0] += w * self.r * rho;
            sum[1] += w * self.r * rho * (self.big * v.cos() + self.r);
        }
        sum.map(|s| s * h / 3.0 * (u1 - u0))
    }
}

/// A circle `q + ρ·(cos φ·e₁ + sin φ·e₂)` over `[φ₀, φ₁]`, traversed
/// forward or back.
#[derive(Clone, Copy)]
struct Arc {
    q: Point3<f64>,
    e1: Vec3<f64>,
    e2: Vec3<f64>,
    rho: f64,
    phi: (f64, f64),
    forward: bool,
}

impl Arc {
    fn at(self, phi: f64) -> Point3<f64> {
        self.q + (self.e1 * phi.cos() + self.e2 * phi.sin()) * self.rho
    }

    /// The rim `v = level` from `u = a` to `u = b`.
    fn rim(d: Donut, level: f64, (a, b): (f64, f64), forward: bool) -> Self {
        Self {
            q: d.c + d.axis * (d.r * level.sin()),
            e1: d.u_ref,
            e2: d.w(),
            rho: d.big + d.r * level.cos(),
            phi: (a, b),
            forward,
        }
    }

    /// The meridian `u = at` from `v = a` to `v = b`.
    fn meridian(d: Donut, at: f64, (a, b): (f64, f64), forward: bool) -> Self {
        Self {
            q: d.c + d.radial(at) * d.big,
            e1: d.radial(at),
            e2: d.axis,
            rho: d.r,
            phi: (a, b),
            forward,
        }
    }

    /// The Villarceau circle the bitangent plane through the axis at
    /// azimuth `at` cuts, whole: centre `c + r·ŵ_at`, radius `R`, in the
    /// plane of `ŵ_at` and `cos α·û_at + sin α·â`, `sin α = r/R`. Forward,
    /// it winds the axis once backwards and the tube once forwards.
    fn villarceau(d: Donut, at: f64, forward: bool) -> Self {
        let (s, c) = ((d.r / d.big), (1.0 - (d.r / d.big).powi(2)).sqrt());
        let (u_hat, w_hat) = (d.radial(at), d.radial(at + PI / 2.0));
        Self {
            q: d.c + w_hat * d.r,
            e1: w_hat,
            e2: u_hat * c + d.axis * s,
            rho: d.big,
            phi: (0.0, TAU),
            forward,
        }
    }

    /// The circle as an exact rational quadratic on `[0, 1]`, in its own
    /// parameter's direction, one segment per third of a turn or less.
    fn net(self) -> NurbsCurve3<f64> {
        let (a, b) = self.phi;
        let segments = ((b - a).abs() / (TAU / 3.0)).ceil().max(1.0);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let n = segments as usize;
        let half = (b - a) / (2.0 * segments);
        let mut control = vec![self.at(a)];
        let mut weights = vec![1.0];
        let mut knots = vec![0.0; 3];
        for k in 0..n {
            #[allow(clippy::cast_precision_loss)]
            let s = a + (b - a) * (k as f64) / segments;
            let m = s + half;
            control
                .push(self.q + (self.e1 * m.cos() + self.e2 * m.sin()) * (self.rho / half.cos()));
            weights.push(half.cos());
            control.push(self.at(s + 2.0 * half));
            weights.push(1.0);
            if k + 1 < n {
                #[allow(clippy::cast_precision_loss)]
                knots.extend([(k + 1) as f64 / segments; 2]);
            }
        }
        knots.extend([1.0; 3]);
        net(knots, 2, control, weights)
    }

    /// `½∫(P − c)×P′` over the arc, traversal folded in, by composite
    /// Simpson on its own parametrization.
    fn vector_area(self, c: Point3<f64>) -> Vec3<f64> {
        let integrand = |phi: f64| {
            let p = self.at(phi) - c;
            let dp = (self.e2 * phi.cos() - self.e1 * phi.sin()) * self.rho;
            p.cross(dp) * 0.5
        };
        let (a, b) = self.phi;
        let n = 4096_u32;
        let h = (b - a) / f64::from(n);
        let mut sum = integrand(a) + integrand(b);
        for i in 1..n {
            let w = if i % 2 == 1 { 4.0 } else { 2.0 };
            sum = sum + integrand(a + h * f64::from(i)) * w;
        }
        let va = sum * (h / 3.0);
        if self.forward { va } else { -va }
    }
}

/// One edge of a torus fixture: its arc, its harmonic image where it has
/// one (a rim's or a meridian's), and whether it is swapped for its
/// exact net.
struct TorusEdge {
    arc: Arc,
    harmonic: Option<TrimEdgeQ>,
    swap: bool,
}

fn rim(d: Donut, level: f64, us: (f64, f64), forward: bool, swap: bool) -> TorusEdge {
    TorusEdge {
        arc: Arc::rim(d, level, us, forward),
        harmonic: Some(harmonic(chan(0.0, 1.0), chan(level, 0.0), us, forward)),
        swap,
    }
}

fn meridian(d: Donut, at: f64, vs: (f64, f64), forward: bool, swap: bool) -> TorusEdge {
    TorusEdge {
        arc: Arc::meridian(d, at, vs, forward),
        harmonic: Some(harmonic(chan(at, 0.0), chan(0.0, 1.0), vs, forward)),
        swap,
    }
}

fn villarceau(d: Donut, at: f64, forward: bool) -> TorusEdge {
    TorusEdge {
        arc: Arc::villarceau(d, at, forward),
        harmonic: None,
        swap: true,
    }
}

/// The face of `loops` on `d` under the sense `outward`, its chart term
/// about `about`: each swapped edge projected, its carrier's vector area
/// read by the lane's rule, the rest of `A⃗` the fixture's own.
fn torus_face(
    d: Donut,
    outward: bool,
    loops: &[Vec<TorusEdge>],
    about: Point3<f64>,
) -> Result<RotationalFaceQ, PropsError> {
    let band = Band::linear(Tol::witness()).unwrap();
    let surface = d.surface();
    let chart = RotationalChartQ::Torus {
        major: pt(d.big),
        minor: pt(d.r),
        outward,
    };
    let lever = d.c - about;
    let lever = [pt(lever.x), pt(lever.y), pt(lever.z)];
    let mut closed = Vec3::zero();
    let mut rationals = Vec::new();
    let mut ribbon = 0.0_f64;
    let mut terms = Vec::new();
    for edges in loops {
        let mut out = Vec::new();
        for e in edges {
            if !e.swap {
                closed = closed + e.arc.vector_area(d.c);
                out.push(EdgeTermQ::Harmonic(e.harmonic.unwrap()));
                continue;
            }
            let n = e.arc.net();
            let carrier = Curve3::Nurbs(std::sync::Arc::new(n.clone()));
            let image = chart_pcurve_over(&carrier, 0.0, 1.0, &surface, band).map_err(|_| {
                PropsError::QuadratureUnsupported {
                    what: "the fixture's carrier mints no image on the torus",
                }
            })?;
            let env = PcurveCache::certify_projected(
                image.projected().unwrap().clone(),
                0.0,
                1.0,
                &carrier,
                &surface,
                band,
                Some(crate::FittedLane::<f64>::certified()),
            )
            .unwrap()
            .certificate()
            .envelope;
            let at = |t: f64| {
                let p = image.eval(t);
                (pt(p.x), pt(p.y))
            };
            out.push(EdgeTermQ::Projected(ProjectedEdgeQ::of_image(
                image.projected().unwrap(),
                chart,
                (pt(0.0), pt(1.0)),
                e.arc.forward,
                [at(0.0), at(1.0)],
                pt(env),
                pt,
            )?));
            let iv = Interval::from_certified;
            rationals.push(
                rational_edge_q(
                    &n.map_scalar(iv),
                    pt(0.0),
                    pt(1.0),
                    e.arc.forward,
                    Point3::new(iv(d.c.x), iv(d.c.y), iv(d.c.z)),
                )
                .unwrap(),
            );
            ribbon += ribbon_of(&n, env);
        }
        terms.push(out);
    }
    let dot = |v: Vec3<f64>| lever[0] * pt(v.x) + lever[1] * pt(v.y) + lever[2] * pt(v.z);
    Ok(RotationalFaceQ {
        chart,
        chart_term: ChartTermQ {
            closed: dot(closed),
            lever,
            rationals,
            spirics: Vec::new(),
            ribbon,
        },
        loops: terms,
    })
}

fn torus_run(
    d: Donut,
    outward: bool,
    loops: &[Vec<TorusEdge>],
    about: Point3<f64>,
) -> Result<FaceCutBounds, PropsError> {
    let band = Band::linear(Tol::witness()).unwrap();
    met(rotational_cut_face_rounds::<f64>(
        &torus_face(d, outward, loops, about)?,
        band,
        RoundWindow::SCHEDULE,
        &mut to_target(1e-9),
    ))
}

/// The lane's enclosures of `loops` hold the region's `[area, flux about
/// the centre]` under the outward normal, `region`, read under the
/// face's sense and carried to `about` by the arcs' `A⃗`.
fn torus_holds(
    label: &str,
    d: Donut,
    outward: bool,
    loops: &[Vec<TorusEdge>],
    region: [f64; 2],
    about: Point3<f64>,
) {
    let va = loops
        .iter()
        .flatten()
        .fold(Vec3::zero(), |va, e| va + e.arc.vector_area(d.c));
    let s = if outward { 1.0 } else { -1.0 };
    let flux = s * region[1] + (d.c - about).dot(va);
    let got = torus_run(d, outward, loops, about)
        .unwrap_or_else(|e| panic!("{label}: the lane refused: {e:?}"));
    encloses(got.flux, flux, &format!("{label} flux"));
    encloses(got.area, region[0], &format!("{label} area"));
}

/// The whole torus less `region`.
fn complement(d: Donut, region: [f64; 2]) -> [f64; 2] {
    let whole = d.whole();
    [whole[0] - region[0], whole[1] - region[1]]
}

/// The chart rectangle `[u₀, u₁] × [v₀, v₁]` wound about its inside
/// (`forward`) or against it, each of its four sides harmonic or swapped
/// as `swap` says: bottom rim, right meridian, top rim, left meridian.
fn patch(
    d: Donut,
    (u0, u1): (f64, f64),
    (v0, v1): (f64, f64),
    forward: bool,
    swap: [bool; 4],
) -> Vec<Vec<TorusEdge>> {
    let mut sides = vec![
        rim(d, v0, (u0, u1), forward, swap[0]),
        meridian(d, u1, (v0, v1), forward, swap[1]),
        rim(d, v1, (u0, u1), !forward, swap[2]),
        meridian(d, u0, (v0, v1), !forward, swap[3]),
    ];
    if !forward {
        sides.reverse();
    }
    vec![sides]
}

/// **A loop that winds nothing, its sides swapped in turn, in both
/// senses and as the complement**: wound about its inside under the
/// outward normal it bounds the patch, and under the inward one the rest
/// of the torus; wound against it, the reverse. Each holds its closed
/// form, its side read off the sense.
#[test]
fn a_patch_holds_its_closed_forms_in_either_sense() {
    let d = Donut::tilted(Point3::new(0.4, -0.2, 0.3), 1.3, 0.45);
    let about = Point3::new(0.1, 0.5, -0.2);
    let (us, vs) = ((0.3, 1.9), (-0.7, 1.1));
    let inside = d.rectangle(us, vs);
    let swaps = [
        [true, false, false, false],
        [false, true, false, false],
        [false, false, true, true],
        [true, true, true, true],
    ];
    for swap in swaps {
        for (label, forward, outward, region) in [
            ("the patch", true, true, inside),
            ("the rest, inward", true, false, complement(d, inside)),
            ("the patch, inward", false, false, inside),
            ("the rest", false, true, complement(d, inside)),
        ] {
            torus_holds(
                &format!("{label} (swapped {swap:?})"),
                d,
                outward,
                &patch(d, us, vs, forward, swap),
                region,
                about,
            );
        }
    }
}

/// **A patch across the tube angle's `±π`** (the inner equator inside
/// it), every side swapped: the images' levels name the patch on a
/// branch a turn apart from the harmonic rims', and the lane carries
/// each onto the chain's lift.
#[test]
fn a_patch_across_the_inner_equator_holds_its_closed_form() {
    let d = Donut::tilted(Point3::new(0.4, -0.2, 0.3), 1.3, 0.45);
    let about = Point3::new(0.1, 0.5, -0.2);
    let (us, vs) = ((-0.4, 0.8), (2.6, 3.9));
    let inside = d.rectangle(us, vs);
    for swap in [[true; 4], [false, true, false, true], [true, false, true, false]] {
        torus_holds(
            &format!("across ±π (swapped {swap:?})"),
            d,
            true,
            &patch(d, us, vs, true, swap),
            inside,
            about,
        );
    }
}

/// **Bands about the outer and the inner equator**: two rims winding the
/// axis once each way, either swapped. The outer band holds the outer
/// equator `v = 0`, the inner one `v = π`, and each one's rim levels on
/// the images sit as their branches name them. Each holds its closed
/// form, and the rest of the torus its complement.
#[test]
fn bands_about_both_equators_hold_their_closed_forms() {
    let d = Donut::tilted(Point3::new(0.4, -0.2, 0.3), 1.3, 0.45);
    let about = Point3::new(0.1, 0.5, -0.2);
    for (equator, (low, high)) in [("outer", (-0.6, 0.5)), ("inner", (2.7, 3.8))] {
        let band = d.rectangle((0.0, TAU), (low, high));
        for swap in [[true, false], [false, true], [true, true]] {
            let loops = |forward: bool| {
                vec![
                    vec![rim(d, low, (0.0, TAU), forward, swap[0])],
                    vec![rim(d, high, (0.0, TAU), !forward, swap[1])],
                ]
            };
            torus_holds(
                &format!("the {equator} band (swapped {swap:?})"),
                d,
                true,
                &loops(true),
                band,
                about,
            );
            torus_holds(
                &format!("the rest of the {equator} band (swapped {swap:?})"),
                d,
                true,
                &loops(false),
                complement(d, band),
                about,
            );
        }
    }
}

/// **A sector between two meridians**: loops that wind the tube once
/// each way, the seam their lifts open read at each chain's start. Both
/// meridians swapped, and one, and the sector's complement.
#[test]
fn a_sector_between_meridians_holds_its_closed_form() {
    let d = Donut::tilted(Point3::new(0.4, -0.2, 0.3), 1.3, 0.45);
    let about = Point3::new(0.1, 0.5, -0.2);
    let (u0, u1) = (0.4, 2.1);
    let sector = d.rectangle((u0, u1), (0.0, TAU));
    for swap in [[true, true], [true, false], [false, true]] {
        let loops = |forward: bool| {
            vec![
                vec![meridian(d, u1, (-PI, PI), forward, swap[0])],
                vec![meridian(d, u0, (-PI, PI), !forward, swap[1])],
            ]
        };
        torus_holds(
            &format!("the sector (swapped {swap:?})"),
            d,
            true,
            &loops(true),
            sector,
            about,
        );
        torus_holds(
            &format!("the rest of the sector (swapped {swap:?})"),
            d,
            true,
            &loops(false),
            complement(d, sector),
            about,
        );
    }
}

/// **A band of Villarceau circles**: the circle the bitangent plane cuts
/// at azimuth `0` and its turn by `β`, each winding the axis and the tube
/// once at once, swapped (no circle of the chart's own). The band
/// between them is `β/2π` of the torus, and its complement the rest;
/// both senses.
#[test]
fn a_band_of_villarceau_circles_holds_its_share_of_the_torus() {
    let d = Donut::tilted(Point3::new(0.4, -0.2, 0.3), 1.3, 0.45);
    let about = Point3::new(0.1, 0.5, -0.2);
    let beta = 1.1;
    let whole = d.whole();
    let share = whole.map(|x| x * beta / TAU);
    for (label, forward, outward, region) in [
        ("the band", false, true, share),
        ("the rest", true, true, complement(d, share)),
        ("the band, inward", true, false, share),
    ] {
        torus_holds(
            label,
            d,
            outward,
            &[
                vec![villarceau(d, 0.0, forward)],
                vec![villarceau(d, beta, !forward)],
            ],
            region,
            about,
        );
    }
}

/// **A torus far out**: the patch and the outer band on a torus `1e3`
/// from the origin, about the origin, so the chart term carries the
/// position.
#[test]
fn a_far_torus_holds_its_closed_forms() {
    let d = Donut::tilted(Point3::new(1e3, -0.7e3, 0.4e3), 1.3, 0.45);
    let about = Point3::origin();
    let (us, vs) = ((0.3, 1.9), (-0.7, 1.1));
    torus_holds(
        "the far patch",
        d,
        true,
        &patch(d, us, vs, true, [true, false, true, false]),
        d.rectangle(us, vs),
        about,
    );
    torus_holds(
        "the far band",
        d,
        true,
        &[
            vec![rim(d, -0.6, (0.0, TAU), true, true)],
            vec![rim(d, 0.5, (0.0, TAU), false, false)],
        ],
        d.rectangle((0.0, TAU), (-0.6, 0.5)),
        about,
    );
}

/// **Refusals**: a torus that is not a ring torus, and a projected image
/// whose deck level is no name of the tube angle.
#[test]
fn the_toruss_arm_refuses_what_it_cannot_read() {
    let d = Donut::tilted(Point3::new(0.4, -0.2, 0.3), 1.3, 0.45);
    let about = Point3::new(0.1, 0.5, -0.2);
    let band = Band::linear(Tol::witness()).unwrap();
    let mut face = torus_face(
        d,
        true,
        &patch(d, (0.3, 1.9), (-0.7, 1.1), true, [true, false, false, false]),
        about,
    )
    .unwrap();
    face.chart = RotationalChartQ::Torus {
        major: pt(0.4),
        minor: pt(0.45),
        outward: true,
    };
    let spindle =
        rotational_cut_face_rounds::<f64>(&face, band, RoundWindow::SCHEDULE, &mut to_target(1e-9));
    assert!(
        matches!(spindle, Err(PropsError::QuadratureUnsupported { .. })),
        "a spindle torus: {spindle:?}"
    );
    let surface = d.surface();
    let arc = Arc::rim(d, 0.3, (0.0, 1.0), true);
    let carrier = Curve3::Nurbs(std::sync::Arc::new(arc.net()));
    let image = chart_pcurve_over(&carrier, 0.0, 1.0, &surface, band)
        .unwrap()
        .map_affine(crate::pcurve_cache::ChartMap {
            u: crate::pcurve_cache::AxisMap::identity(),
            v: crate::pcurve_cache::AxisMap::shift(1e-3),
        })
        .unwrap();
    let chart = RotationalChartQ::Torus {
        major: pt(d.big),
        minor: pt(d.r),
        outward: true,
    };
    let shifted = ProjectedEdgeQ::of_image(
        image.projected().unwrap(),
        chart,
        (pt(0.0), pt(1.0)),
        true,
        [(pt(0.0), pt(0.0)); 2],
        Interval::zero(),
        pt,
    );
    assert!(
        matches!(shifted, Err(PropsError::QuadratureUnsupported { .. })),
        "a deck level naming no tube angle: {shifted:?}"
    );
}

// ---------------------------------------------------------------------
// The spiric arm
// ---------------------------------------------------------------------

/// A spiric `(R, r, d)` about the origin, axis `ẑ`, cutting plane normal
/// `x̂`, over `(t₀, t₁)`: `P(t) = n·d + m·f(t) + a·r·sin t`, `m = a × n`.
fn spiric(big: f64, r: f64, d: f64, (t0, t1): (f64, f64), forward: bool) -> SpiricEdgeQ {
    SpiricEdgeQ {
        centre: [Interval::zero(); 3],
        axis: [pt(0.0), pt(0.0), pt(1.0)],
        normal: [pt(1.0), pt(0.0), pt(0.0)],
        major: pt(big),
        minor: pt(r),
        offset: pt(d),
        t0: pt(t0),
        t1: pt(t1),
        forward,
    }
}

/// The spiric arm's edge: its image `u = u₀ + σ·atan2(f, d)`,
/// `v = σ·t`, and its ends read off it.
fn spiric_arm(e: SpiricEdgeQ, sigma: f64, u0: f64, env: f64) -> EdgeTermQ {
    let at = |t: Interval| {
        let (c, _) = trig_of(t);
        let f = ((e.major + e.minor * c).sqr() - e.offset.sqr()).sqrt();
        (
            pt(u0) + pt(sigma) * geom_core::Real::atan2(f, e.offset),
            pt(sigma) * t,
        )
    };
    EdgeTermQ::Spiric(SpiricWallQ {
        edge: e,
        image: WallImageQ {
            sense: sigma,
            v0: Interval::zero(),
            env: pt(env),
        },
        ends: [at(e.t0), at(e.t1)],
    })
}

/// A spiric-armed face on the torus `(R, r)` about the origin.
fn spiric_face(big: f64, r: f64, outward: bool, loops: Vec<Vec<EdgeTermQ>>) -> RotationalFaceQ {
    RotationalFaceQ {
        chart: RotationalChartQ::Torus {
            major: pt(big),
            minor: pt(r),
            outward,
        },
        chart_term: ChartTermQ {
            closed: Interval::zero(),
            lever: [Interval::zero(); 3],
            rationals: Vec::new(),
            spirics: Vec::new(),
            ribbon: 0.0,
        },
        loops,
    }
}

/// **A closed loop of a spiric, two rims and a meridian against a direct
/// 2-D integral** of the chart densities over the region it bounds, at
/// both `σ` (the loop winds the other way at `σ = −1`, so its face is
/// read under the inward normal there).
#[test]
#[allow(clippy::cast_precision_loss)]
fn a_spiric_loop_measures_the_region_it_bounds() {
    let (big, r, d, t0, t1, um) = (1.2_f64, 0.225_f64, 0.05_f64, 0.3_f64, 2.1_f64, -0.5_f64);
    for sigma in [1.0_f64, -1.0] {
        let f = |t: f64| ((big + r * t.cos()).powi(2) - d * d).sqrt();
        let u = |t: f64| sigma * f(t).atan2(d);
        let n = 100_000;
        let h = (t1 - t0) / n as f64;
        let (mut area, mut flux) = (0.0, 0.0);
        for i in 0..n {
            let t = t0 + h * (i as f64 + 0.5);
            let v = sigma * t;
            let rho = big + r * v.cos();
            let width = u(t) - um;
            area += r * rho * width * h;
            flux += r * rho * (big * v.cos() + r) * width * h;
        }
        let loops = vec![vec![
            spiric_arm(spiric(big, r, d, (t0, t1), true), sigma, 0.0, 0.0),
            EdgeTermQ::Harmonic(harmonic(
                chan(u(t1), um - u(t1)),
                chan(sigma * t1, 0.0),
                (0.0, 1.0),
                true,
            )),
            EdgeTermQ::Harmonic(harmonic(
                chan(um, 0.0),
                chan(sigma * t1, sigma * (t0 - t1)),
                (0.0, 1.0),
                true,
            )),
            EdgeTermQ::Harmonic(harmonic(
                chan(um, u(t0) - um),
                chan(sigma * t0, 0.0),
                (0.0, 1.0),
                true,
            )),
        ]];
        // The region winds positively at `σ = 1` when `u(t) > um`.
        let outward = sigma * area > 0.0;
        let got = met(rotational_cut_face_rounds::<f64>(
            &spiric_face(big, r, outward, loops),
            Band::linear(Tol::witness()).unwrap(),
            RoundWindow::SCHEDULE,
            &mut to_target(1e-9),
        ))
        .unwrap_or_else(|e| panic!("σ {sigma}: {e:?}"));
        let s = if outward { 1.0 } else { -1.0 };
        let near = |x: Interval, w: f64| x.lo() - 1e-8 <= w && w <= x.hi() + 1e-8;
        assert!(near(got.area, area.abs()), "σ {sigma}: area {:?} vs {area}", got.area);
        assert!(
            near(got.flux, s * sigma * flux),
            "σ {sigma}: flux {:?} vs {flux}",
            got.flux
        );
    }
}

/// A whole oval `(R, r, d)` from `t₀` closed by a meridian back at its
/// end's azimuth less `du`, the meridian's start held at `wide` about
/// that: the loop the spiric arm's closure rows read.
fn oval_loop(big: f64, r: f64, d: f64, t0: f64, du: f64, wide: f64) -> Vec<EdgeTermQ> {
    let arm = spiric_arm(spiric(big, r, d, (t0, t0 + TAU), true), 1.0, 0.0, 0.0);
    let EdgeTermQ::Spiric(w) = arm else {
        unreachable!("the arm is a spiric")
    };
    let [(_, va), (ub, vb)] = w.ends;
    let mut back = harmonic(
        HarmChan {
            c0: ub + pt(du),
            ca: Interval::zero(),
            cb: Interval::zero(),
            cl: Interval::zero(),
        },
        HarmChan {
            c0: vb,
            ca: Interval::zero(),
            cb: Interval::zero(),
            cl: va - vb,
        },
        (0.0, 1.0),
        true,
    );
    back.u.c0 = back.u.c0 + Interval::from_bounds(-wide, wide);
    vec![arm, EdgeTermQ::Harmonic(back)]
}

/// **The spiric arm's junctions close as the lane's every junction
/// does**, on fat tori whose oval meets its meridian at the inner equator
/// (`ρ = R − r`): set off by `ρ·δ = 2ε` the loop measures, at `f64` and
/// at the interval scalar; set off by `20ε` it is open
/// ([`PropsError::ChartLoopOpen`]); a start held `20ε` wide about the
/// junction escalates; and the oval alone winds the tube, which no face
/// bounds alone ([`PropsError::LoopsWindChart`]).
#[test]
fn the_spiric_arms_junctions_close_as_the_lanes_do() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let eps = tol.eps();
    for r in [0.5, 0.6] {
        let rho = 1.0 - r;
        let face = |gap: f64, wide: f64| {
            spiric_face(1.0, r, true, vec![oval_loop(1.0, r, 0.05, PI, gap / rho, wide)])
        };
        let near = face(2.0 * eps, 0.0);
        let at_f64 =
            rotational_cut_face_rounds::<f64>(&near, band, RoundWindow::SCHEDULE, &mut to_target(eps));
        assert!(at_f64.is_ok(), "r = {r}, f64: {at_f64:?}");
        let at_iv = rotational_cut_face_rounds::<Interval>(
            &near,
            band,
            RoundWindow::SCHEDULE,
            &mut to_target(eps),
        );
        assert!(at_iv.is_ok(), "r = {r}, interval: {at_iv:?}");
        let far = rotational_cut_face_rounds::<f64>(
            &face(20.0 * eps, 0.0),
            band,
            RoundWindow::SCHEDULE,
            &mut to_target(eps),
        );
        assert!(
            matches!(far, Err(PropsError::ChartLoopOpen)),
            "r = {r}, 20ε: {far:?}"
        );
        let wide = rotational_cut_face_rounds::<f64>(
            &face(0.0, 10.0 * eps / rho),
            band,
            RoundWindow::SCHEDULE,
            &mut to_target(eps),
        );
        assert!(
            matches!(wide, Err(PropsError::Escalated { .. })),
            "r = {r}, an undecidable junction: {wide:?}"
        );
    }
    let mut alone = oval_loop(1.2, 0.275, 0.05, 0.0, 0.0, 0.0);
    alone.truncate(1);
    let winds = rotational_cut_face_rounds::<f64>(
        &spiric_face(1.2, 0.275, true, vec![alone]),
        band,
        RoundWindow::SCHEDULE,
        &mut to_target(eps),
    );
    assert!(
        matches!(winds, Err(PropsError::LoopsWindChart)),
        "the oval alone winds the tube: {winds:?}"
    );
}

/// **Two spiric loops leave the torus less both, and one wound with its
/// face refuses**: two whole ovals, of cuts at offsets `0.3` and `0.05`,
/// each closed by its meridian, wind against the chart, so each alone
/// bounds the torus less its oval's region; together they bound the
/// torus less both, `A₁ + A₂ − 4π²Rr` of the single faces' areas. The
/// second run backwards bounds its region, wound with a face that is the
/// first's complement ([`PropsError::RingWoundWithFace`]).
#[test]
fn spiric_rings_are_read_against_their_face() {
    let band = Band::linear(Tol::witness()).unwrap();
    let reversed = |l: Vec<EdgeTermQ>| -> Vec<EdgeTermQ> {
        l.into_iter()
            .rev()
            .map(|e| match e {
                EdgeTermQ::Spiric(mut w) => {
                    w.edge.forward = !w.edge.forward;
                    EdgeTermQ::Spiric(w)
                }
                EdgeTermQ::Harmonic(mut h) => {
                    h.forward = !h.forward;
                    EdgeTermQ::Harmonic(h)
                }
                EdgeTermQ::Projected(p) => EdgeTermQ::Projected(p),
            })
            .collect()
    };
    let run = |loops: Vec<Vec<EdgeTermQ>>| {
        met(rotational_cut_face_rounds::<f64>(
            &spiric_face(1.2, 0.275, true, loops),
            band,
            RoundWindow::SCHEDULE,
            &mut to_target(1e-9),
        ))
    };
    let first = oval_loop(1.2, 0.275, 0.3, 0.0, 0.0, 0.0);
    let second = oval_loop(1.2, 0.275, 0.05, 0.0, 0.0, 0.0);
    let a_first = run(vec![first.clone()]).unwrap().area;
    let a_second = run(vec![second.clone()]).unwrap().area;
    let both = run(vec![first.clone(), second.clone()]).unwrap().area;
    let whole = 4.0 * PI * PI * 1.2 * 0.275;
    let want = mid(a_first) + mid(a_second) - whole;
    assert!(
        both.lo() - 1e-8 <= want && want <= both.hi() + 1e-8,
        "the torus less both: {both:?} vs {want}"
    );
    let with = run(vec![first, reversed(second)]);
    assert!(
        matches!(with, Err(PropsError::RingWoundWithFace)),
        "a ring wound with its face: {with:?}"
    );
}

/// **The map-residual pad pays the area the envelope can move**: with a
/// synthetic envelope `e` on a spiric (its own wall image has none), the
/// area width grows by at least `2·L·e`, `L` the arc's length from the
/// sampled chart speed, and the flux width by at least `2·L·e·(R + r)`,
/// the flux density's bound, and by no more than sixteen times each.
#[test]
#[allow(clippy::cast_precision_loss)]
fn the_spiric_arms_residual_pad_pays_the_area_the_envelope_moves() {
    let (big, r, t0, t1, env) = (3.0_f64, 1.25_f64, 2.6_f64, 3.6_f64, 1e-6_f64);
    let d = ((big - r).powi(2) - 0.05_f64.powi(2)).sqrt();
    let speed = |t: f64| {
        let rho = big + r * t.cos();
        let f = (rho * rho - d * d).sqrt();
        let du = d * r * t.sin() / (rho * f);
        ((rho * du).powi(2) + r * r).sqrt()
    };
    let length = (0..4000)
        .map(|i| speed(t0 + (t1 - t0) * ((i as f64 + 0.5) / 4000.0)) * (t1 - t0) / 4000.0)
        .sum::<f64>();
    let widths = |env: f64| {
        let face = spiric_face(
            big,
            r,
            true,
            vec![vec![spiric_arm(spiric(big, r, d, (t0, t1), true), 1.0, 0.0, env)]],
        );
        let chart = face.chart;
        let entry = edge_entry(&face.loops[0][0], chart).unwrap();
        let (t, _) = torus_edge_terms(&face.loops[0][0], &entry, (pt(big), pt(r)), 64);
        (t[0].width(), t[1].width())
    };
    let ((f1, a1), (f0, a0)) = (widths(env), widths(0.0));
    for (name, grew, density) in [("flux", f1 - f0, big + r), ("area", a1 - a0, 1.0)] {
        let want = 2.0 * length * env * density;
        assert!(
            want <= grew && grew <= 16.0 * want,
            "{name}: the pad grew the width by {grew}, the envelope moves {want}"
        );
    }
}
