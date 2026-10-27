//! **The torus-walled revolves the offset-axial door is measured on**:
//! the barrel and the teapot's belly (full revolves whose one curved
//! wall is a meridian arc about a centre OFF the axis), the klein elbow
//! (a disc swept a quarter turn about an axis off its centre), and the
//! tour's sectioned vessel with the cavity the axial door carves in it
//! (a partial revolve whose rims mint as spirics); and the volume
//! oracle their hollows are measured against ([`assert_hollow_volume`]).
//!
//! A row that hollows one of these and a row that asks `point_in_solid`
//! of it are about each other only while they build THE SAME BODY
//! ([`super::cavity`]'s rule). Body authoring, so it routes here
//! ([`super`]'s routing rule).
//!
//! **Deliberately not absorbed**, and the whole of it:
//!
//! - `review_arms3_r1_probes`' `torus_barrel`, a different barrel under
//!   the same name (an annular meridian about another centre);
//! - `shell7_dump`'s `torus_vessel`, the tour's torus vessel revolved a
//!   FULL turn with its band spelled as a bulge — a different body from
//!   [`vessel_quarter`];
//! - `torax_interval`'s barrel, [`torus_barrel`]'s `Interval` twin, whose
//!   bulge is enclosed in interval arithmetic from the same dyadic
//!   stations — a generic home would have to carry that enclosure;
//! - `spiric_rim`'s scalar-generic `vessel_loop` / `vessel_at`, the
//!   sectioned vessel spelled once for every deciding scalar with its
//!   band as the exact 3-4-5 bulge `1/2` rather than an arc about its
//!   centre, which [`vessel_quarter`] keeps because the tour does.

use geom_core::{Band, Point2, Tol, Vec2};
use profile::path::{Open, Start};
use profile::{ArcSweep, Center, Profile, ProfileLoop, SketchPlane};
use sweep::test_support::revolved_about_y;
use sweep::{Revolution, RevolveAxis, revolve};
use topo::Body;

use super::bulge;
use super::charts::hollow_moves;
use super::oracles::simpson;

/// **The barrel bulged about a centre OFF the axis.** The same two
/// junction stations and the same `5/64` meridian radius as the tour's
/// sphere-zone barrel, about the OTHER centre on their perpendicular
/// bisector — so the wall is a TORUS: `R = 6/64`, `r = 5/64`,
/// `h_c = 4/64`, a 3-4-5 at each junction with both residuals exactly
/// zero.
pub fn torus_barrel() -> Body<f64> {
    let c = Point2::new(6.0 / 64.0, 1.0 / 16.0);
    let (lo, hi) = (
        Point2::new(3.0 / 64.0, 0.0),
        Point2::new(3.0 / 64.0, 8.0 / 64.0),
    );
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (lo, bulge(lo, hi, c)),
            (hi, 0.0),
            (Point2::new(0.0, 8.0 / 64.0), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    )
}

/// **The teapot's wall-1 belly.** The pot's own foot and mouth, its
/// belly bulged about `(7/64, 5/64)` — off the axis, so a TORUS with
/// `R = 7/64`, `r = 5/64`, `h_c = 5/64`.
pub fn torus_belly() -> Body<f64> {
    let c = Point2::new(7.0 / 64.0, 5.0 / 64.0);
    let (lo, hi) = (
        Point2::new(4.0 / 64.0, 1.0 / 64.0),
        Point2::new(3.0 / 64.0, 8.0 / 64.0),
    );
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(4.0 / 64.0, 0.0), 0.0),
            (lo, bulge(lo, hi, c)),
            (hi, 0.0),
            (Point2::new(0.0, 8.0 / 64.0), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    )
}

/// **The klein elbow**: the cross-section `loops` (a disc, or a disc's
/// two offset circles) revolved a quarter turn backwards about the axis
/// `x = 1.2` running `−y` — the loop arc of the tour's Klein bottle,
/// whose spine radius is `1.2`. A disc centred on the sketch origin
/// makes its wall a torus; the partial turn leaves two planar caps and
/// the rims between them and the wall.
pub fn klein_elbow(loops: Vec<ProfileLoop<f64>>) -> Body<f64> {
    let tol = Tol::witness();
    let profile = Profile::new(SketchPlane::xy(), loops)
        .validate(tol)
        .expect("the elbow's cross-section validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(1.2, 0.0),
            dir: Vec2::new(0.0, -1.0),
        },
        Revolution::Partial(-core::f64::consts::FRAC_PI_2),
        tol,
    )
    .expect("the elbow revolves")
    .body
}

/// **The sectioned vessel**: the tour's torus-walled vessel meridian
/// (`demos/tour/src/torusvessel.rs`, the BELLIED centre), spelled from
/// the same stations and revolved a quarter turn, so the sectioned
/// vessel's door is measured on the scene's own body.
pub fn vessel_quarter() -> Body<f64> {
    let tol = Tol::witness();
    let (r_foot, r_band, r_neck) = (5.0 / 64.0, 9.0 / 64.0, 7.0 / 64.0);
    let (y_foot, y_shoulder, y_mouth) = (4.0 / 64.0, 12.0 / 64.0, 24.0 / 64.0);
    let (h_tube, r_bellied) = (8.0 / 64.0, 6.0 / 64.0);
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(r_foot, 0.0), tol)
        .expect("the base disc")
        .line_to(Point2::new(r_foot, y_foot), tol)
        .expect("the foot")
        .line_to(Point2::new(r_band, y_foot), tol)
        .expect("the lower shoulder")
        .arc_to(
            Center {
                c: Point2::new(r_bellied, h_tube),
                winding: ArcSweep::Ccw,
                p: Point2::new(r_band, y_shoulder),
            },
            tol,
        )
        .expect("the band")
        .line_to(Point2::new(r_neck, y_shoulder), tol)
        .expect("the upper shoulder")
        .line_to(Point2::new(r_neck, y_mouth), tol)
        .expect("the neck")
        .line_to(Point2::new(0.0, y_mouth), tol)
        .expect("the mouth disc")
        .line_to(Start, tol)
        .expect("the axis closes the meridian")
        .into();
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .expect("the meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        tol,
    )
    .expect("the meridian revolves")
    .body
}

/// **The sectioned vessel and its cavity through the axial door** —
/// `(quarter, cavity)`: the body `shell` builds and stops on at tier 3,
/// taken BEFORE tier 3 so its carriers can be read.
pub fn vessel_cavity(t: f64) -> (Body<f64>, Body<f64>) {
    let quarter = vessel_quarter();
    let mut cavity = quarter.clone();
    let band = Band::linear(Tol::witness()).expect("band");
    topo::offset_charts_together(
        &mut cavity,
        &hollow_moves(&quarter, t),
        band,
        Tol::witness(),
    )
    .expect("the vessel's corners solve and its rims mint");
    (quarter, cavity)
}

/// **The turn a cut revolve keeps at radius `ρ`.** A partial revolve
/// of turn `theta` whose `caps` meridian caps (0, 1 or 2) have moved `t`
/// inward along their normals keeps, at radius `ρ` from the axis, the
/// azimuth window `θ − caps·asin(t/ρ)` (each moved cap is a plane
/// parallel to the axis at stand-off `t`, which a circle of radius `ρ`
/// meets at `asin(t/ρ)` from its old meridian), and nothing where that
/// closes.
fn kept_turn(theta: f64, t: f64, caps: f64, rho: f64) -> f64 {
    if rho <= t {
        return if caps == 0.0 { theta } else { 0.0 };
    }
    (theta - caps * (t / rho).asin()).max(0.0)
}

/// `∫ ρ·(θ − caps·asin(t/ρ)) dρ` in closed form,
/// `θρ²/2 − caps·(ρ²·asin(t/ρ) + t·√(ρ² − t²))/2`, and the radius
/// `t/sin(θ/caps)` below which the window is closed.
fn cut_moment(theta: f64, t: f64, caps: f64, rho: f64) -> f64 {
    theta * rho * rho / 2.0
        - caps * (rho * rho * (t / rho).asin() + t * (rho * rho - t * t).sqrt()) / 2.0
}

/// **The volume of a disc's cut revolve**: the disc of radius `a`
/// centred `rho_c` from the axis, revolved `theta` with `caps` caps
/// moved `t` inward — `∬ ρ·kept_turn dA`, by `ρ = ρ_c + a·cos φ` (the
/// chord at `ρ` is `2a·sin φ`, so the integrand is smooth; adaptive
/// Simpson, cross-checked against a 50-digit quadrature by
/// `scripts/spiric-quadrature-references.py`).
pub fn disc_cut_volume(theta: f64, t: f64, caps: f64, rho_c: f64, a: f64) -> f64 {
    simpson(
        &|phi| {
            let rho = rho_c + a * phi.cos();
            2.0 * a * a * phi.sin().powi(2) * rho * kept_turn(theta, t, caps, rho)
        },
        0.0,
        core::f64::consts::PI,
    )
}

/// **The volume of a strip's cut revolve**: the region
/// `lo ≤ ρ ≤ hi`, `floor(ρ) ≤ y ≤ top`, revolved `theta` with `caps`
/// caps moved `t` inward (adaptive Simpson; the integrand is smooth on
/// the strip).
pub fn strip_cut_volume(
    theta: f64,
    t: f64,
    caps: f64,
    (lo, hi): (f64, f64),
    top: f64,
    floor: impl Fn(f64) -> f64,
) -> f64 {
    simpson(
        &|rho| rho * kept_turn(theta, t, caps, rho) * (top - floor(rho)),
        lo,
        hi,
    )
}

/// **The sectioned vessel's hollow volume at wall `t`** — the quarter
/// turn of [`vessel_quarter`]'s meridian less its cavity: the meridian
/// inset by `t` along every edge but the axis (the moved faces meet in
/// mitres at the two reflex shoulders), revolved with both caps moved
/// `t` inward. Each height's inner integral is [`cut_moment`]'s closed
/// form; Simpson in the height over each smooth piece.
pub fn vessel_hollow_volume(t: f64) -> f64 {
    let theta = core::f64::consts::FRAC_PI_2;
    let u = 1.0 / 64.0;
    let (r_foot, r_neck, y_foot, y_shoulder, y_mouth) =
        (5.0 * u, 7.0 * u, 4.0 * u, 12.0 * u, 24.0 * u);
    // The band: the circle of radius `b` about `(a, c)`, outer side.
    let (a, b, c) = (6.0 * u, 5.0 * u, 8.0 * u);
    let band = |rad: f64| move |y: f64| a + (rad * rad - (y - c).powi(2)).sqrt();
    let solid = |rho: f64| theta * rho * rho / 2.0;
    let outer = solid(r_foot) * y_foot
        + simpson(&|y| solid(band(b)(y)), y_foot, y_shoulder)
        + solid(r_neck) * (y_mouth - y_shoulder);
    let rho_0 = t / (theta / 2.0).sin();
    let cut = |rho: f64| cut_moment(theta, t, 2.0, rho) - cut_moment(theta, t, 2.0, rho_0);
    let cavity = cut(r_foot - t) * y_foot
        + simpson(&|y| cut(band(b - t)(y)), y_foot + t, y_shoulder - t)
        + cut(r_neck - t) * (y_mouth - y_shoulder);
    outer - cavity
}

/// **The hollow's certified volume holds `want`.** Its bracket
/// `volume ± volume_pad` must contain the oracle to `1e-10` of it (the
/// oracles' stated tolerance), and the pad must sit at the quadrature's
/// reporting level: each face converges once its flux width over three
/// times its area is under `1024·ε`, so the summed volume half-width is
/// under `512·ε` times the surface area — a pad that converged to
/// something worth reporting, not merely to something sound.
///
/// Every row's measured midpoint sits 1–2% of its pad below the oracle.
/// The oracles agree with 50-digit references to `2e-15`
/// (`scripts/spiric-quadrature-references.py`), so the offset is the
/// enclosure's own: each piece's remainder reads the hull of `g₄` over
/// the piece, which is not centred on `g₄`'s mean value there, and the
/// bracket's midpoint inherits that lean, well inside the bracket.
pub fn assert_hollow_volume(label: &str, hollow: &Body<f64>, want: f64) {
    let tol = Tol::witness();
    let props = topo::mass_properties(
        &topo::test_support::finished("the measured body", topo::Body::clone(hollow), tol),
        tol,
        topo::test_support::pinned_resolution(tol),
    )
    .unwrap_or_else(|e| panic!("{label}: the hollow measures, got {e:?}"));
    println!(
        "[measured] {label}: volume {} ± {}, oracle {want}",
        props.enclosure.volume, props.enclosure.volume_pad
    );
    assert!(
        (props.enclosure.volume - want).abs() <= props.enclosure.volume_pad + 1e-10 * want.abs(),
        "{label}: volume {} ± {} misses the oracle {want}",
        props.enclosure.volume,
        props.enclosure.volume_pad
    );
    let reporting = 512.0 * tol.eps() * props.enclosure.surface_area;
    assert!(
        props.enclosure.volume_pad > 0.0 && props.enclosure.volume_pad <= reporting,
        "{label}: a spiric face is a quadrature, and its pad {} is under the reporting \
         level {reporting}",
        props.enclosure.volume_pad
    );
}

/// **The wall area of a disc's cut revolve**: the torus tube of radius
/// `a` about the spine radius `rho_c`, revolved `theta` with `caps` caps
/// moved `t` inward — `∫ a·ρ(v)·kept_turn(ρ(v)) dv`, `ρ = ρ_c + a·cos v`.
pub fn disc_cut_wall_area(theta: f64, t: f64, caps: f64, rho_c: f64, a: f64) -> f64 {
    simpson(
        &|v| {
            let rho = rho_c + a * v.cos();
            a * rho * kept_turn(theta, t, caps, rho)
        },
        0.0,
        core::f64::consts::TAU,
    )
}

/// **One moved cap's area**: the plane at stand-off `t` from the axis
/// cuts the disc's tube (radius `a` about `rho_c`) in an oval of chord
/// `√((ρ_c + w)² − t²) − √((ρ_c − w)² − t²)` at height `z`,
/// `w = √(a² − z²)` — by `z = a·sin φ`. At `t = 0` it is the disc's `πa²`.
pub fn cut_cap_area(t: f64, rho_c: f64, a: f64) -> f64 {
    simpson(
        &|phi| {
            let w = a * phi.cos();
            let chord = ((rho_c + w).powi(2) - t * t).sqrt() - ((rho_c - w).powi(2) - t * t).sqrt();
            chord * a * phi.cos()
        },
        -core::f64::consts::FRAC_PI_2,
        core::f64::consts::FRAC_PI_2,
    )
}

/// **The hollow's certified surface area holds `want`**: its bracket
/// `surface_area ± area_pad` contains the oracle to `1e-10` of it, so a
/// wall area wrong by more than the pad — the spiric lane's `G` channel
/// — reaches `mass_properties` and goes red.
pub fn assert_hollow_area(label: &str, hollow: &Body<f64>, want: f64) {
    let props = topo::mass_properties(
        &topo::test_support::finished(
            "the measured body",
            topo::Body::clone(hollow),
            Tol::witness(),
        ),
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap_or_else(|e| panic!("{label}: the hollow measures, got {e:?}"));
    println!(
        "[measured] {label}: surface area {} ± {}, oracle {want}",
        props.enclosure.surface_area, props.enclosure.area_pad
    );
    assert!(
        (props.enclosure.surface_area - want).abs() <= props.enclosure.area_pad + 1e-10 * want,
        "{label}: surface area {} ± {} misses the oracle {want}",
        props.enclosure.surface_area,
        props.enclosure.area_pad
    );
}
