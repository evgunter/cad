//! **The pose decides circle against ellipse once** (Ev, PR 5156,
//! `circular-axes-refuses-between-the-rim-alignment-band-and-the-axes-band`):
//! a section whose tilt the table reads definitely off square is the
//! exact `Ellipse`, however near-circular its semi-axes.
//!
//! Three tilts per scale `r ∈ {1e-2, 1, 1e2}` (and `1e3` for the
//! section rows), at the run's ε, all past
//! the pose's own band and all where the semi-axes differ by at most
//! about Kε/4, far inside the band: just clear of the
//! pose band, where `a − b ≈ ε/2`, and where `a − b ≈ Kε/4`. At each the
//! table serves the ellipse and the served carrier misses both surfaces
//! by rounding along its whole length (measured in double-double), a
//! split of a real body succeeds with both halves passing tiers 1–3 at
//! their closed-form volumes, and a Boolean of the tilted operand
//! against a box builds at its closed-form volume. A tilt in the pose's
//! band still refuses, naming the pose's own predicate.
//!
//! Each section row stands its configuration in three frames (as built,
//! turned about a generic axis, and turned then carried about 1 km off),
//! and the body rows are repeated densely in the turned frame: an
//! axis-aligned frame rounds most of the section's products exactly,
//! which a carrier's frame cannot count on. A sliver crossing of equal
//! walls serves its long bisector semi-axis to a few `u`, down to where
//! `1 − cos φ` rounds to nothing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom::{Curve3, Surface};
use geom_brep::Reach;
use geom_brep::intersect::{
    EqualCylinderSection, PlaneConeSection, PlaneCylinderSection, SectionError,
    cylinder_cylinder_section, plane_cone_section, plane_cylinder_section,
};
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::test_support::{brick, finished};
use topo::{AtRestBody, Body};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

const SCALES: [f64; 3] = [1e-2, 1.0, 1e2];

/// The section rows' scales: [`SCALES`] and a wall a kilometre across.
const SECTION_SCALES: [f64; 4] = [1e-2, 1.0, 1e2, 1e3];

/// The body rows' scales in the turned frame.
const TURNED_SCALES: [f64; 7] = [1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0];

/// The sines of the three tilts across the old gap for a section
/// whose pose margin is `sin θ` levered at `lever` and whose semi-axes
/// differ by `spread·sin²θ`: just clear of the pose band, where the
/// semi-axes differ by ε/2, and where they differ by Kε/4.
fn gap_sines(lever: f64, spread: f64) -> [f64; 3] {
    let (eps, hi) = (tol().eps(), band().escalate());
    let s = [
        4.0 * hi / lever,
        (eps / 2.0 / spread).sqrt(),
        (hi / 4.0 / spread).sqrt(),
    ];
    assert!(
        s[0] < s[1] && s[1] < s[2],
        "lever {lever:e}: the gap is open at this ε ({s:?})"
    );
    s
}

/// [`gap_sines`] and, where the frame's rounding is largest against
/// the tilt, five more just past the pose band.
fn dense_sines(lever: f64, spread: f64) -> Vec<f64> {
    let hi = band().escalate();
    [6.0, 10.0, 16.0, 25.0, 40.0]
        .map(|k| k * hi / lever)
        .into_iter()
        .chain(gap_sines(lever, spread))
        .collect()
}

/// The sine of a tilt in the pose's band at `lever`.
fn banded_sine(lever: f64) -> f64 {
    (band().zero() + band().escalate()) / 2.0 / lever
}

// ---- double-double, for measuring a served carrier's miss ----

#[derive(Clone, Copy)]
struct Dd(f64, f64);

fn two_sum(a: f64, b: f64) -> Dd {
    let s = a + b;
    let bb = s - a;
    Dd(s, (a - (s - bb)) + (b - bb))
}

impl Dd {
    fn of(x: f64) -> Self {
        Dd(x, 0.0)
    }
    fn add(self, o: Self) -> Self {
        let s = two_sum(self.0, o.0);
        let lo = s.1 + self.1 + o.1;
        two_sum(s.0, lo)
    }
    fn neg(self) -> Self {
        Dd(-self.0, -self.1)
    }
    fn sub(self, o: Self) -> Self {
        self.add(o.neg())
    }
    fn mul(self, o: Self) -> Self {
        let p = self.0 * o.0;
        let e = self.0.mul_add(o.0, -p);
        two_sum(p, e + self.0 * o.1 + self.1 * o.0)
    }
    fn div(self, o: Self) -> Self {
        let q = self.0 / o.0;
        let r = self.sub(o.mul(Dd::of(q)));
        two_sum(q, r.0 / o.0)
    }
    fn sqrt(self) -> Self {
        if self.0 <= 0.0 {
            return Dd::of(0.0);
        }
        let s = self.0.sqrt();
        let r = self.sub(Dd::of(s).mul(Dd::of(s)));
        two_sum(s, r.0 / (2.0 * s))
    }
    fn value(self) -> f64 {
        self.0 + self.1
    }
}

type V = [Dd; 3];

fn v(p: Vec3<f64>) -> V {
    [Dd::of(p.x), Dd::of(p.y), Dd::of(p.z)]
}

fn between(p: Point3<f64>, q: Point3<f64>) -> V {
    let d = |a: f64, b: f64| Dd::of(a).sub(Dd::of(b));
    [d(p.x, q.x), d(p.y, q.y), d(p.z, q.z)]
}

fn dot(a: V, b: V) -> Dd {
    a[0].mul(b[0]).add(a[1].mul(b[1])).add(a[2].mul(b[2]))
}

fn axb(a: V, s: Dd) -> V {
    [a[0].mul(s), a[1].mul(s), a[2].mul(s)]
}

fn minus(a: V, b: V) -> V {
    [a[0].sub(b[0]), a[1].sub(b[1]), a[2].sub(b[2])]
}

/// `p`'s distance off the plane through `q` with normal `n` (any length).
fn off_plane(p: Point3<f64>, q: Point3<f64>, n: Vec3<f64>) -> f64 {
    dot(between(p, q), v(n))
        .div(dot(v(n), v(n)).sqrt())
        .value()
        .abs()
}

/// `p`'s height along and distance from the line `o + t·a`.
fn about(p: Point3<f64>, o: Point3<f64>, a: Vec3<f64>) -> (Dd, Dd) {
    let (w, a) = (between(p, o), v(a));
    let aa = dot(a, a);
    let h = dot(w, a).div(aa);
    let radial = minus(w, axb(a, h));
    (h.mul(aa.sqrt()), dot(radial, radial).sqrt())
}

/// How far `p` misses the stored surface `s`, a cylinder or a cone.
fn off_surface(p: Point3<f64>, s: &Surface<f64>) -> f64 {
    match *s {
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => about(p, origin, axis).1.sub(Dd::of(radius)).value().abs(),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let (h, rho) = about(p, apex, axis);
            let (sin, cos) = half_angle.sin_cos();
            rho.mul(Dd::of(cos)).sub(h.mul(Dd::of(sin))).value().abs()
        }
        _ => unreachable!("a cylinder or a cone"),
    }
}

/// The largest miss of `curve` off each of `surfaces`, over its whole
/// period, against the rounding of its own evaluation: a multiple of
/// the unit roundoff at the curve's own coordinate scale. A non-finite
/// semi-axis, sample or miss fails the row.
fn assert_on(
    what: &str,
    curve: &Curve3<f64>,
    plane: Option<(Point3<f64>, Vec3<f64>)>,
    surfaces: &[&Surface<f64>],
) {
    assert_on_within(what, curve, plane, surfaces, f64::INFINITY);
}

/// [`assert_on`] over the part of the curve within `reach` of its
/// centre, at the rounding of that part's coordinates.
fn assert_on_within(
    what: &str,
    curve: &Curve3<f64>,
    plane: Option<(Point3<f64>, Vec3<f64>)>,
    surfaces: &[&Surface<f64>],
    reach: f64,
) {
    let Curve3::Ellipse {
        center,
        semi_u,
        semi_v,
        ..
    } = *curve
    else {
        panic!("{what}: an ellipse, got {curve:?}");
    };
    assert!(
        semi_u.is_finite() && semi_v.is_finite(),
        "{what}: the served semi-axes are {semi_u:e} and {semi_v:e}"
    );
    let scale = (center - Point3::origin()).norm() + semi_u.abs().max(semi_v.abs()).min(reach);
    let rounding = 8.0 * f64::EPSILON * scale;
    let (mut worst_plane, mut worst, mut read) = (0.0_f64, 0.0_f64, 0);
    // The whole period, or where the reach is finite, stations spread
    // evenly along the larger semi-axis's span within it, both sides.
    let ts: Vec<f64> = if reach.is_finite() {
        let (major, along_u) = (semi_u.abs().max(semi_v.abs()), semi_u.abs() >= semi_v.abs());
        (0..=4096)
            .flat_map(|i| {
                let x = (f64::from(i) / 2048.0 - 1.0) * reach.min(major) / major;
                if along_u {
                    [x.acos(), -x.acos()]
                } else {
                    [x.asin(), PI - x.asin()]
                }
            })
            .collect()
    } else {
        (0..1 << 14)
            .map(|i| f64::from(i) / f64::from(1 << 14) * 2.0 * PI)
            .collect()
    };
    for t in ts {
        let p = curve.eval(t);
        assert!(
            p.x.is_finite() && p.y.is_finite() && p.z.is_finite(),
            "{what}: the carrier evaluates to {p:?} at {t}"
        );
        if (p - center).norm() > reach {
            continue;
        }
        read += 1;
        let misses = plane
            .map(|(q, n)| off_plane(p, q, n))
            .into_iter()
            .chain(surfaces.iter().map(|s| off_surface(p, s)));
        for (i, miss) in misses.enumerate() {
            assert!(miss.is_finite(), "{what}: a miss of {miss:e} at {t}");
            if i == 0 && plane.is_some() {
                worst_plane = worst_plane.max(miss);
            } else {
                worst = worst.max(miss);
            }
        }
    }
    assert!(read > 64, "{what}: {read} samples within the reach");
    assert!(
        worst_plane <= rounding && worst <= rounding,
        "{what}: the served ellipse misses the plane by {worst_plane:e} and the walls by \
         {worst:e}, past rounding {rounding:e} (ε {:e})",
        tol().eps()
    );
}

fn validated_halves(
    what: &str,
    body: &AtRestBody<f64>,
    q: Point3<f64>,
    n: Vec3<f64>,
    volumes: [f64; 2],
) {
    let plane = topo::test_support::split_plane(q, n, tol());
    let got = topo::splitting::split(body, &plane, tol())
        .unwrap_or_else(|e| panic!("{what}: the split refused: {e}"));
    for (side, part, want) in [
        ("above", &got.above, volumes[0]),
        ("below", &got.below, volumes[1]),
    ] {
        let half = part
            .body()
            .unwrap_or_else(|| panic!("{what}: the {side} half is empty"));
        assert_volume(&format!("{what}, {side}"), half, want);
    }
}

/// `body` passes tiers 1–3 and its volume is `want` within its pad.
fn assert_volume(what: &str, body: &Body<f64>, want: f64) {
    topo::validate_closed(body).unwrap_or_else(|e| panic!("{what}: tiers 1–2: {e:?}"));
    topo::validate_geometric(body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3: {e:?}"));
    let props = topo::mass_properties(
        &topo::test_support::finished("the measured body", topo::Body::clone(body), tol()),
        tol(),
        topo::test_support::pinned_resolution(tol()),
    )
    .unwrap_or_else(|e| panic!("{what}: the volume refused {e:?}"));
    let slack = props.enclosure.volume_pad + 64.0 * f64::EPSILON * want.abs();
    assert!(
        (props.enclosure.volume - want).abs() <= slack,
        "{what}: volume {:e} against {want:e} (± {slack:e})",
        props.enclosure.volume
    );
}

fn rotated(body: &Body<f64>, about: Vec3<f64>, angle: f64) -> AtRestBody<f64> {
    moved_body(
        body,
        &Affine3::rotation_about_axis(Point3::origin(), about, angle),
    )
}

fn moved_body(body: &Body<f64>, map: &Affine3<f64>) -> AtRestBody<f64> {
    finished(
        "the moved body",
        topo::transform_rigid(body, &geom_core::test_support::rigid(map, tol()), tol()).unwrap(),
        tol(),
    )
}

/// A turn about a generic axis, which no coordinate of a row's
/// configuration survives exactly.
fn turn() -> Affine3<f64> {
    Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.3, -0.5, 0.8).normalize(), 0.7)
}

/// The frames a section row stands its configuration in.
fn frames() -> [(&'static str, Affine3<f64>); 3] {
    [
        ("as built", Affine3::identity()),
        ("turned", turn()),
        (
            "far",
            Affine3::translation(Vec3::new(700.0, -500.0, 400.0)) * turn(),
        ),
    ]
}

fn moved(map: &Affine3<f64>, s: &Surface<f64>) -> Surface<f64> {
    let (p, v) = (|x| map.transform_point(x), |x| map.transform_vec(x));
    match *s {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => Surface::Plane {
            origin: p(origin),
            normal: v(normal),
            u_ref: v(u_ref),
        },
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => Surface::Cylinder {
            origin: p(origin),
            axis: v(axis),
            radius,
            u_ref: v(u_ref),
        },
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => Surface::Cone {
            apex: p(apex),
            axis: v(axis),
            half_angle,
            u_ref: v(u_ref),
        },
        _ => unreachable!("a plane, a cylinder or a cone"),
    }
}

// ---- plane × cylinder ----

/// The disk of radius `r` about `z` extruded over `z ∈ [0, r]`.
fn rod(r: f64) -> Body<f64> {
    sweep::test_support::prism(
        vec![(Point2::new(r, 0.0), 1.0), (Point2::new(-r, 0.0), 1.0)],
        r,
        tol(),
    )
}

fn wall(r: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: r,
        u_ref: Vec3::unit_x(),
    }
}

fn tilted_z(s: f64) -> Vec3<f64> {
    Vec3::new(s, 0.0, (1.0 - s * s).sqrt())
}

/// The table serves the tilted ellipse, on both surfaces to rounding,
/// in every frame.
#[test]
fn a_near_square_plane_cuts_a_cylinder_in_its_exact_ellipse() {
    for (frame, map) in frames() {
        for r in SECTION_SCALES {
            let cyl = moved(&map, &wall(r));
            let q = map.transform_point(Point3::new(0.0, 0.0, 0.5 * r));
            let reach = Reach::Measured {
                at: q,
                lever: 5.0_f64.sqrt() * r,
            };
            let section = |s: f64| {
                let plane = moved(
                    &map,
                    &Surface::Plane {
                        origin: Point3::new(0.0, 0.0, 0.5 * r),
                        normal: tilted_z(s),
                        u_ref: Vec3::unit_y(),
                    },
                );
                plane_cylinder_section(&plane, &cyl, &reach, band())
            };
            for s in gap_sines(r, r / 2.0) {
                let what = format!("{frame}, r {r:e}, sin θ {s:e}");
                let got = section(s);
                let Ok(PlaneCylinderSection::TiltedEllipse(e)) = &got else {
                    panic!("{what}: the tilted ellipse, got {got:?}");
                };
                let n = map.transform_vec(tilted_z(s));
                assert_on(&what, e, Some((q, n)), &[&cyl]);
            }
            let got = section(banded_sine(r));
            assert!(
                matches!(&got, Err(SectionError::Escalated(d)) if d.predicate == Some("pc_rim_alignment")),
                "{frame}, r {r:e} in the pose's band: got {got:?}"
            );
        }
    }
}

/// A split of the rod at each gap tilt: two halves at `π r²·r/2` each.
#[test]
fn a_near_square_split_of_a_rod_validates_its_halves() {
    for r in SCALES {
        let body = finished("the rod", rod(r), tol());
        let q = Point3::new(0.0, 0.0, 0.5 * r);
        let half = PI * r * r * r / 2.0;
        for s in gap_sines(r, r / 2.0) {
            validated_halves(
                &format!("r {r:e}, sin θ {s:e}"),
                &body,
                q,
                tilted_z(s),
                [half, half],
            );
        }
    }
}

/// The rod turned `θ` about `y`, minus the box over `z ≥ r/2`: the
/// rod's length below the box's face is `r/(2 cos θ)` along its axis,
/// and the cut's wedges cancel about it.
#[test]
fn a_near_square_rod_minus_a_box_builds() {
    for r in SCALES {
        let cutter = finished(
            "the box",
            brick(
                (-2.0 * r, 2.0 * r),
                (-2.0 * r, 2.0 * r),
                (0.5 * r, 2.0 * r),
                tol(),
            ),
            tol(),
        );
        for s in gap_sines(r, r / 2.0) {
            let what = format!("r {r:e}, sin θ {s:e}");
            let rod = rotated(&rod(r), Vec3::unit_y(), s.asin());
            let got = topo::subtract(&rod, &cutter, tol())
                .unwrap_or_else(|e| panic!("{what}: the subtract refused: {e}"));
            let cos = (1.0 - s * s).sqrt();
            assert_volume(
                &what,
                &got.body().expect("a body").body,
                PI * r * r * (0.5 * r / cos),
            );
        }
    }
}

// ---- plane × cone ----

/// The frustum of scale `S`: radius `r0 = S/2 + S/2` at `y = 0` and
/// `r1 = S/2` at `y = S`, half-angle `atan ½`, apex above on `y`.
struct Frustum {
    /// `None` where the body is pinned to refuse as limb 2's kernel
    /// limit: at 1 km and ε = 1e-12, where its cone's rim rounds at its
    /// own extent past the band
    /// (`work/offset/eps-min-leaves-no-certification-headroom-at-the-box-edge.md`).
    body: Option<Body<f64>>,
    cone: Surface<f64>,
    r0: f64,
    r1: f64,
    /// The apex's height over the base.
    apex: f64,
}

fn frustum(scale: f64) -> Frustum {
    let alpha = 0.5_f64.atan();
    let (h, r1) = (scale, 0.5 * scale);
    let r0 = r1 + h * alpha.tan();
    let body = sweep::test_support::try_revolved_about_y_at(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(r0, 0.0), 0.0),
            (Point2::new(r1, h), 0.0),
            (Point2::new(0.0, h), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    );
    let pinned: &[f64] = if scale >= 1e3 { &[1e-12] } else { &[] };
    let body = test_utils::kernel_limit::pinned(body, tol().eps(), pinned, "the frustum");
    let apex = h + r1 / alpha.tan();
    Frustum {
        body,
        cone: Surface::Cone {
            apex: Point3::new(0.0, apex, 0.0),
            axis: Vec3::new(0.0, -1.0, 0.0),
            half_angle: alpha,
            u_ref: Vec3::unit_x(),
        },
        r0,
        r1,
        apex,
    }
}

/// **A zero pad is the closed form's, rounding and all.** A frustum
/// measures with `volume_pad == 0.0` (every face takes a closed form),
/// and its volume is off its exact value by the rounding of its faces'
/// flux terms, which no pad states (`topo::MassProperties`): within
/// `4·u·|V|`, as built and turned, at every body scale ([`SCALES`]).
/// Measured at 2.2 or less of `u·|V|`. The bound is in units of the
/// volume, not of the terms: a walk that read the faces about a point
/// far from the body would grow its terms and its error together, and
/// reds here (moving every shell's reference 1 km off does).
///
/// A pose carried far is not a row here. There the rigid motion rounds
/// each vertex by about `u` of its distance, which moves the stored
/// geometry off the frustum this row's closed form names; that is
/// the motion's rounding, not the measurement's.
#[test]
fn a_frustums_zero_pad_carries_its_closed_forms_rounding() {
    let u = f64::EPSILON / 2.0;
    for scale in SCALES {
        let f = frustum(scale);
        let want = PI * scale / 3.0 * (f.r0 * f.r0 + f.r0 * f.r1 + f.r1 * f.r1);
        for (pose, map) in [("as built", Affine3::identity()), ("turned", turn())] {
            let body = moved_body(f.body.as_ref().expect("the frustum builds"), &map);
            let props =
                topo::mass_properties(&body, tol(), topo::test_support::pinned_resolution(tol()))
                    .expect("it measures");
            assert_eq!(
                props.enclosure.volume_pad, 0.0,
                "{scale:e} {pose}: every face is closed-form"
            );
            let err = (props.enclosure.volume - want).abs();
            assert!(
                err <= 4.0 * u * want,
                "{scale:e} {pose}: {} against {want}, off by {err:e}, {:.2} of u·|V|",
                props.enclosure.volume,
                err / (u * want)
            );
        }
    }
}

fn tilted_y(s: f64) -> Vec3<f64> {
    Vec3::new(s, (1.0 - s * s).sqrt(), 0.0)
}

/// The volume of the cone from its apex to the plane at distance
/// `delta` from it, tilted `θ` off square: a third of the section
/// ellipse's area times `delta`, the semi-axes the table's closed form.
fn apex_cone_volume(delta: f64, s: f64) -> f64 {
    let alpha = 0.5_f64.atan();
    let (sa, ca) = alpha.sin_cos();
    let c = (1.0 - s * s).sqrt();
    let k = (ca * c - sa * s) * (ca * c + sa * s);
    let (major, minor) = (delta * sa * ca / k, delta * sa / k.sqrt());
    PI * major * minor * delta / 3.0
}

/// The frustum's semi-axis spread per `sin²θ` at the rim radius `ρ`:
/// `a − b ≈ ρ·sin²θ·(1/2 + tan²α)`, `tan α = 1/2`.
fn cone_spread(rho: f64) -> f64 {
    rho * 0.75
}

#[test]
fn a_near_square_plane_cuts_a_frustum_in_its_exact_ellipse() {
    for (frame, map) in frames() {
        for scale in SECTION_SCALES {
            let f = frustum(scale);
            let cone = moved(&map, &f.cone);
            let q0 = Point3::new(0.0, 0.5 * scale, 0.0);
            let q = map.transform_point(q0);
            let rim = 0.5 * (f.r0 + f.r1);
            let lever = (2.0 * f.r0).max((scale * scale + (f.r0 + f.r1).powi(2)).sqrt());
            let reach = Reach::Measured {
                at: map.transform_point(Point3::new(0.0, f.apex, 0.0)),
                lever,
            };
            let section = |s: f64| {
                let plane = moved(
                    &map,
                    &Surface::Plane {
                        origin: q0,
                        normal: tilted_y(s),
                        u_ref: Vec3::unit_z(),
                    },
                );
                plane_cone_section(&plane, &cone, &reach, band())
            };
            for s in gap_sines(f.r1, cone_spread(rim)) {
                let what = format!("{frame}, scale {scale:e}, sin θ {s:e}");
                let got = section(s);
                let Ok(PlaneConeSection::TiltedEllipse(e)) = &got else {
                    panic!("{what}: the tilted ellipse, got {got:?}");
                };
                let n = map.transform_vec(tilted_y(s));
                assert_on(&what, e, Some((q, n)), &[&cone]);
            }
            let got = section(banded_sine(lever));
            assert!(
                matches!(&got, Err(SectionError::Escalated(d)) if d.predicate == Some("pn_axis_normal")),
                "{frame}, scale {scale:e} in the pose's band: got {got:?}"
            );
        }
    }
}

/// A split of the frustum at each gap tilt (levered at its top rim, the
/// rim edge's own plane-parallel question): the halves are the frustum
/// below and above the plane through its mid-height axis point.
#[test]
fn a_near_square_split_of_a_frustum_validates_its_halves() {
    for scale in SCALES {
        let f = frustum(scale);
        let body = finished(
            "the frustum",
            f.body.clone().expect("the frustum builds"),
            tol(),
        );
        let q = Point3::new(0.0, 0.5 * scale, 0.0);
        let rim = 0.5 * (f.r0 + f.r1);
        let whole = PI * (f.r0 * f.r0 * f.apex - f.r1 * f.r1 * (f.apex - scale)) / 3.0;
        for s in gap_sines(f.r1, cone_spread(rim)) {
            let cut = apex_cone_volume((f.apex - 0.5 * scale) * (1.0 - s * s).sqrt(), s);
            let above = cut - PI * f.r1 * f.r1 * (f.apex - scale) / 3.0;
            validated_halves(
                &format!("scale {scale:e}, sin θ {s:e}"),
                &body,
                q,
                tilted_y(s),
                [above, whole - above],
            );
        }
    }
}

/// The frustum turned `θ` about `z`, minus the box over `y ≥ S/2`: what
/// is left is the cone from its apex to the base less the cone from its
/// apex to the box's face.
#[test]
fn a_near_square_frustum_minus_a_box_builds() {
    for scale in SCALES {
        let f = frustum(scale);
        let y0 = 0.5 * scale;
        let cutter = finished(
            "the box",
            brick(
                (-4.0 * scale, 4.0 * scale),
                (y0, 4.0 * scale),
                (-4.0 * scale, 4.0 * scale),
                tol(),
            ),
            tol(),
        );
        let rim = 0.5 * (f.r0 + f.r1);
        for s in gap_sines(f.r1, cone_spread(rim)) {
            let what = format!("scale {scale:e}, sin θ {s:e}");
            let turned = rotated(
                f.body.as_ref().expect("the frustum builds"),
                Vec3::unit_z(),
                s.asin(),
            );
            let got = topo::subtract(&turned, &cutter, tol())
                .unwrap_or_else(|e| panic!("{what}: the subtract refused: {e}"));
            let delta = f.apex * (1.0 - s * s).sqrt() - y0;
            let want = PI * f.r0 * f.r0 * f.apex / 3.0 - apex_cone_volume(delta, s);
            assert_volume(&what, &got.body().expect("a body").body, want);
        }
    }
}

// ---- the equal-cylinder bisector ----

/// Equal walls of radius `r` about `z` and about `z` turned `φ` about
/// `y`, through the origin: the bisector across both axes is the
/// near-circular ellipse (semi-axes `r/cos(φ/2)` and `r`), the one along
/// them the long one (`r/sin(φ/2)` and `r`). Both are served; each
/// misses both walls by rounding, the near-circular one along its whole
/// length and the long one over the walls' reach, in every frame.
#[test]
fn declared_equal_walls_meeting_at_a_sliver_are_the_bisector_ellipses() {
    for (frame, map) in frames() {
        for r in SECTION_SCALES {
            bisector_rows(frame, &map, r);
        }
    }
}

/// Equal walls of radius `r` about `z` and about `z` turned `φ` about
/// `y`, through the origin, carried by `map`.
fn sliver_walls(map: &Affine3<f64>, r: f64, phi: f64) -> (Surface<f64>, Surface<f64>) {
    let turned = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::new(phi.sin(), 0.0, phi.cos()),
        radius: r,
        u_ref: Vec3::unit_y(),
    };
    (moved(map, &wall(r)), moved(map, &turned))
}

fn bisector_rows(frame: &str, map: &Affine3<f64>, r: f64) {
    let reach = Reach::Measured {
        at: map.transform_point(Point3::origin()),
        lever: r,
    };
    let walls = |phi: f64| sliver_walls(map, r, phi);
    // a − b = r·(sec(φ/2) − 1) ≈ r·sin²φ/8.
    for s in gap_sines(r, r / 8.0) {
        let what = format!("{frame}, r {r:e}, sin φ {s:e}");
        let (a, b) = walls(s.asin());
        let got = cylinder_cylinder_section(&a, &b, &reach.ball(), band());
        let Ok(EqualCylinderSection::TwoEllipses { e1, e2, .. }) = &got else {
            panic!("{what}: the two bisector ellipses, got {got:?}");
        };
        let near = |e: &Curve3<f64>| match *e {
            Curve3::Ellipse { semi_u, semi_v, .. } => (semi_u - semi_v).abs() < r,
            _ => false,
        };
        let (round, long) = if near(e1) { (e1, e2) } else { (e2, e1) };
        assert!(
            near(round) && !near(long),
            "{what}: one near-circular, one long"
        );
        assert_on(&format!("{what}, across"), round, None, &[&a, &b]);
        assert_on_within(&format!("{what}, along"), long, None, &[&a, &b], 4.0 * r);
    }
    let (a, b) = walls(banded_sine(r).asin());
    let got = cylinder_cylinder_section(&a, &b, &reach.ball(), band());
    assert!(
        matches!(&got, Err(SectionError::Escalated(d)) if d.predicate == Some("cc_axes_parallel")),
        "{frame}, r {r:e} in the pose's band: got {got:?}"
    );
}

/// `r / sin(φ/2)` for the stored axes `a` and `b`, `φ` their angle,
/// in double-double: half the length of `â − b̂`.
fn long_semi_axis(r: f64, a: Vec3<f64>, b: Vec3<f64>) -> f64 {
    let unit = |x: Vec3<f64>| {
        let x = v(x);
        axb(x, Dd::of(1.0).div(dot(x, x).sqrt()))
    };
    let d = minus(unit(a), unit(b));
    Dd::of(2.0 * r).div(dot(d, d).sqrt()).value()
}

/// A sliver crossing of equal walls, `φ` down to where `1 − cos φ`
/// rounds to nothing: the long bisector ellipse's semi-axis is
/// `r / sin(φ/2)` to a few `u` of the stored axes' own angle, in every
/// frame, or the pose reads the axes parallel or refuses in its band.
#[test]
fn a_sliver_crossing_serves_its_long_semi_axis_to_a_few_u() {
    for (frame, map) in frames() {
        let mut served = 0;
        for (r, phi) in [1.0, 4.0]
            .into_iter()
            .flat_map(|r| [2e-9, 8e-9, 4e-8, 1e-6, 1e-4].map(|phi| (r, phi)))
        {
            let what = format!("{frame}, r {r:e}, φ {phi:e}");
            let (a, b) = sliver_walls(&map, r, phi);
            let reach = Reach::Measured {
                at: map.transform_point(Point3::origin()),
                lever: r,
            };
            let got = cylinder_cylinder_section(&a, &b, &reach.ball(), band());
            let (Surface::Cylinder { axis: aa, .. }, Surface::Cylinder { axis: ab, .. }) = (&a, &b)
            else {
                unreachable!("two cylinders");
            };
            match &got {
                Ok(EqualCylinderSection::TwoEllipses { e1, e2, .. }) => {
                    let semi = |e: &Curve3<f64>| match *e {
                        Curve3::Ellipse { semi_u, semi_v, .. } => semi_u.max(semi_v),
                        _ => panic!("{what}: an ellipse, got {e:?}"),
                    };
                    let long = semi(e1).max(semi(e2));
                    let want = long_semi_axis(r, *aa, *ab);
                    assert!(
                        long.is_finite() && (long - want).abs() <= 8.0 * f64::EPSILON * want,
                        "{what}: the long semi-axis is {long:e}, against {want:e}"
                    );
                    served += 1;
                }
                Err(SectionError::Escalated(d))
                    if d.predicate == Some("cc_axes_parallel")
                        && phi * r < 2.0 * band().escalate() => {}
                // Parallel by the pose, through one origin: one wall.
                Err(SectionError::CoincidentSurfaces) if phi * r < 2.0 * band().escalate() => {}
                _ => panic!("{what}: the bisector ellipses, got {got:?}"),
            }
        }
        // φ = 1e-4 is past the pose's band at every ε row, at both radii.
        assert!(served >= 2, "{frame}: {served} crossings served");
    }
}

/// [`a_near_square_split_of_a_rod_validates_its_halves`] densely over
/// the scale and the tilt, in the turned frame.
#[test]
fn a_turned_near_square_split_of_a_rod_validates_its_halves() {
    let map = turn();
    for r in TURNED_SCALES {
        let body = moved_body(&rod(r), &map);
        let q = map.transform_point(Point3::new(0.0, 0.0, 0.5 * r));
        let half = PI * r * r * r / 2.0;
        for s in dense_sines(r, r / 2.0) {
            validated_halves(
                &format!("turned, r {r:e}, sin θ {s:e}"),
                &body,
                q,
                map.transform_vec(tilted_z(s)),
                [half, half],
            );
        }
    }
}

/// [`a_near_square_rod_minus_a_box_builds`] densely over the scale and
/// the tilt, in the turned frame.
#[test]
fn a_turned_near_square_rod_minus_a_box_builds() {
    let map = turn();
    for r in TURNED_SCALES {
        let cutter = moved_body(
            &brick(
                (-2.0 * r, 2.0 * r),
                (-2.0 * r, 2.0 * r),
                (0.5 * r, 2.0 * r),
                tol(),
            ),
            &map,
        );
        for s in dense_sines(r, r / 2.0) {
            let what = format!("turned, r {r:e}, sin θ {s:e}");
            let tilt = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), s.asin());
            let rod = moved_body(&rod(r), &(map * tilt));
            let got = topo::subtract(&rod, &cutter, tol())
                .unwrap_or_else(|e| panic!("{what}: the subtract refused: {e}"));
            let cos = (1.0 - s * s).sqrt();
            assert_volume(
                &what,
                &got.body().expect("a body").body,
                PI * r * r * (0.5 * r / cos),
            );
        }
    }
}
