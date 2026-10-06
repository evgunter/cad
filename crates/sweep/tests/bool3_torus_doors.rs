//! Issue 1011, the TORUS half: the torus doors of
//! `topo::boolean::point_in_solid`.
//!
//! Before this unit `face_geo` resolved `{Plane, Cylinder, Cone,
//! Sphere}` and refused every torus face as `KindUnsupported` — a
//! HEALTHY body and a missing capability, and the refusal every
//! containment door downstream of the pair-scoped operand gate
//! inherited. The arm this unit lands is the ray×torus QUARTIC, answered
//! only on a CERTIFIED real-root count, plus the torus chart's own two
//! angular windows.
//!
//! # The two classes, and why there are only two
//!
//! The sphere and the cone each need a closed-GROUP class beside their
//! per-face one because their charts have a singular junction — a pole,
//! an apex — that the boundary walk cannot cross, so a window it reports
//! as a full period there is an ARTEFACT. A ring torus has no such
//! junction. So the classes here are just:
//!
//! - the **closed group** — every boundary edge shared with another face
//!   of the same torus surface, so the union covers the whole torus and
//!   there is no boundary left to read a window from. This is what a
//!   full revolve of a CLOSED profile mints: two half-tori sharing their
//!   two full-period parallels, each wrapping the major azimuth through
//!   a self-mated seam;
//! - the **windowed face** — the major and minor windows the walk pins,
//!   either of which may be a whole period, which on this chart is a
//!   fact about the face rather than a failure to see its edge.
//!
//! The four fixtures below are exactly the four corners of that: no
//! window (the donut), the minor window alone (the spool's band), the
//! major window alone (the quarter donut), and both (the quarter spool).
//!
//! # The rows, and what each one is the only witness for
//!
//! - **interior / exterior / boundary** on a donut, whose every face is
//!   a torus face, so nothing else can be answering;
//! - **the four-root ray**: from the centre of the hole a ray in the
//!   midplane meets the tube FOUR times, at `±(R ± r)`. Every arm before
//!   this one had at most two roots to fold, and a quartic answered as
//!   if it were a quadratic reads the hole as material;
//! - **each window**, on the face class that carries it: a point on the
//!   torus CARRIER but outside the face's own window is outside the
//!   solid, not on its boundary;
//! - **an uncertain root count sets its ray aside rather than
//!   answering**, at the tube's top circle — the tangency locus, where
//!   the plane `h = r` touches the torus along a whole circle and the
//!   quartic's roots merge — and another ray answers;
//! - **the shell about that circle is the residual one**, measured and
//!   guarded: the rays the quartic cannot count there are set aside
//!   rather than refusing the query, so the circle leaves no shell of
//!   its own;
//! - **the consumer unlock**: a disjoint union whose no-crossings
//!   fallback walks a containment door. Red on main as
//!   `KindUnsupported { kind: Torus }`, green here;
//! - **the refusal that stays**, and **the radii regime that is not
//!   mintable**.
//!
//! ε honesty: every probe offset comes from [`away`], which is BOOL-2's
//! expression unchanged rather than a fourth per-suite spelling, and the
//! shell it has to clear is measured here on every run.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use crate::common::approx::band;
use geom_core::{Band, Point2, Point3, Tol};
use profile::{Profile, ProfileLoop, SketchPlane, test_support::bulge_loop};
use revolve_common::*;
use sweep::test_support::{brick, finished};
use sweep::{Revolution, revolve};
use topo::{Body, BooleanError, PointInSolidError, SolidContainment, point_in_solid};

/// The donut's torus: centre at the origin, axis `+y`.
const DONUT_R: f64 = 1.0;
const DONUT_MINOR: f64 = 0.3;

/// **The fixture scale these bodies are built at.** Every body here is a
/// unit revolve about the y axis: nothing is further than `R + r = 1.3`
/// from the axis and nothing reaches beyond `y = ±1`. [`away`]'s ceiling
/// is stated against that, and
/// [`the_clamp_floor_clears_the_shell_about_the_top_circle`] asserts it from
/// the body's own vertices rather than leaving it as prose a later
/// fixture edit could falsify.
const FIXTURE_EXTENT: f64 = DONUT_R + DONUT_MINOR;

/// A **donut**: a full circle profile of radius `r` about `(R, 0)`,
/// revolved fully about the y axis. Every face is a torus face — no
/// plane, no sphere, no cylinder — so a row on this body is a row on
/// this arm alone. Its two faces are the CLOSED GROUP class: they share
/// both full-period parallels and each wraps the major azimuth through
/// its own self-mated seam meridian.
fn donut() -> Body<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(DONUT_R, -DONUT_MINOR), 1.0),
        (Point2::new(DONUT_R, DONUT_MINOR), 1.0),
    ]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// The **spool** profile: a unit square section whose outer edge bulges
/// OUT into a torus band, closed top and bottom by planar discs.
///
/// The bulge chord runs `(1, 0) → (1, 1)`, so with bulge `b` the sagitta
/// is `s = b/2`, the arc radius `r = (1/4 + s²)/(2s)` and the arc centre
/// sits at `x = 1 + s − r`. At `b = 0.6` that is `R = 0.7333`,
/// `r = 0.5667` — a RING torus (`R > r`), which is the only regime the
/// kernel represents (see
/// [`a_spindle_torus_is_not_mintable_through_the_public_door`]).
const SPOOL_BULGE: f64 = 0.6;

fn spool_arc() -> (f64, f64) {
    let s = SPOOL_BULGE / 2.0;
    let r = (0.25 + s * s) / (2.0 * s);
    (1.0 + s - r, r)
}

fn spool_loop() -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), SPOOL_BULGE),
        (Point2::new(1.0, 1.0), 0.0),
        (Point2::new(0.0, 1.0), 0.0),
    ])
}

/// A full revolve of [`spool_loop`]: the torus band wraps the major
/// azimuth and is trimmed in the MINOR angle by the arc's own ends.
fn spool() -> Body<f64> {
    revolve(
        &validated(vec![spool_loop()]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// A quarter revolve of the donut's circle: the torus face is trimmed in
/// the MAJOR azimuth and wraps the minor angle.
fn quarter_donut() -> Body<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(DONUT_R, -DONUT_MINOR), 1.0),
        (Point2::new(DONUT_R, DONUT_MINOR), 1.0),
    ]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// A quarter revolve of [`spool_loop`]: BOTH windows trim the torus
/// face, which is the only body here that exercises them together.
fn quarter_spool() -> Body<f64> {
    revolve(
        &validated(vec![spool_loop()]),
        axis_y(),
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap()
    .body
}

fn pis(body: &Body<f64>, q: Point3<f64>) -> SolidContainment {
    point_in_solid(body, q, band(), Tol::witness())
        .unwrap_or_else(|e| panic!("point_in_solid refused at {q:?}: {e}"))
}

/// The probe offset — **BOOL-2's expression, unchanged**, because this
/// arm's probes have the same two jobs against the same two kinds of
/// shell and a second spelling would only be a second thing to keep in
/// step. What differs is the shell it has to clear, and that is derived
/// and measured here rather than inherited.
///
/// # The torus's one shell
///
/// The **residual** shell, where the boundary pre-pass's exact signed
/// distance to the tube compares against `Zero`: linear in ε, about
/// `K·ε`. **Measured 9.7e-9 at the default row**, at both equators, at a
/// generic 45° surface point and above the tube's top circle alike.
/// Every probe in this suite clears it by five orders.
///
/// The quartic degenerates on a locus of (point, direction) pairs, and
/// the tube's top and bottom circles, where the tangent plane is
/// perpendicular to the axis, put part of that locus about the BODY: a
/// near-horizontal ray from a point above the top circle has two root
/// pairs merging. Such a ray is set aside and another ray of the
/// schedule answers, so the locus leaves no shell wider than the
/// residual one, which
/// [`the_clamp_floor_clears_the_shell_about_the_top_circle`] measures.
///
/// The clamp saturates at every shipped ε row (floor at 1e-9 and 1e-12,
/// ceiling at 1e-6), exactly as BOOL-2 records: `1e6·ε` lands inside
/// `[1e-3, 0.1]` only for `ε ∈ (1e-9, 1e-7)` and no row draws one.
fn away() -> f64 {
    // The expression itself lives in `revolve_common::probe_offset`,
    // which carries the part of the argument every containment suite
    // shares (the ε-scaling, the clamp and its saturation). What stays
    // here is the SHELL this suite has to clear, which is this arm's own.
    probe_offset()
}

/// The signed distance from `q` to the donut's tube: negative inside.
/// Closed form, independent of the kernel — the oracle the sweep row
/// measures against.
fn donut_clearance(q: Point3<f64>) -> f64 {
    let rho = q.x.hypot(q.z);
    ((rho - DONUT_R).powi(2) + q.y * q.y).sqrt() - DONUT_MINOR
}

/// **The arm, on a body whose every face is a torus face.** Interior,
/// exterior and boundary, at the four cardinal points of the tube and
/// off the chart seam, so no row can be passing because some other
/// kind's arm answered.
#[test]
fn torus_door_classifies_the_donut_interior_exterior_and_boundary() {
    let body = donut();
    assert!(
        body.faces().all(|(_, f)| matches!(
            body.get_surface(f.surface),
            Some(geom::Surface::Torus { .. })
        )),
        "the donut must carry torus faces and nothing else"
    );
    // Interior: the tube's spine circle, on the seam and off it.
    for q in [
        Point3::new(DONUT_R, 0.0, 0.0),
        Point3::new(0.0, 0.0, DONUT_R),
        Point3::new(-DONUT_R, 0.0, 0.0),
        Point3::new(0.7, 0.1, 0.7),
    ] {
        assert_eq!(pis(&body, q), SolidContainment::In, "interior at {q:?}");
    }
    // Exterior: the hole, the far field, and above the tube.
    for q in [
        Point3::origin(),
        Point3::new(3.0, 0.0, 0.0),
        Point3::new(0.0, 3.0, 0.0),
        Point3::new(DONUT_R, DONUT_MINOR + 0.5, 0.0),
    ] {
        assert_eq!(pis(&body, q), SolidContainment::Out, "exterior at {q:?}");
    }
    // Boundary: the outer and inner equators, and the top circle.
    for q in [
        Point3::new(DONUT_R + DONUT_MINOR, 0.0, 0.0),
        Point3::new(DONUT_R - DONUT_MINOR, 0.0, 0.0),
        Point3::new(0.0, 0.0, -(DONUT_R + DONUT_MINOR)),
        Point3::new(DONUT_R, DONUT_MINOR, 0.0),
    ] {
        assert_eq!(
            pis(&body, q),
            SolidContainment::OnBoundary,
            "boundary at {q:?}"
        );
    }
}

/// **The four-root ray.** From the centre of the hole, a ray in the
/// midplane meets the tube at `t = ±(R − r)` and `t = ±(R + r)` — four
/// real roots, two of them ahead of the query. Every earlier arm in this
/// door had at most two roots to fold, and a quartic folded as if it
/// were a quadratic answers `In` here: it would see the far wall and
/// miss the near one.
///
/// The count is not asserted through a private door — it is the geometry
/// itself: the midplane section of a ring torus is the pair of concentric
/// circles `ρ = R ± r`, which a ray through the centre crosses four
/// times. What the row pins is that the door reads the NEAREST of those
/// crossings and reports the hole as free space, and that a probe a
/// hair's breadth inside the near wall reads `In`.
#[test]
fn the_four_root_ray_through_the_hole_reads_the_nearest_wall() {
    let body = donut();
    assert_eq!(
        pis(&body, Point3::origin()),
        SolidContainment::Out,
        "the hole is free space, and every ray from it crosses four walls"
    );
    // Just inside the inner equator, on the same midplane: the same
    // four-root rays, one wall nearer.
    let d = away();
    assert_eq!(
        pis(&body, Point3::new(DONUT_R - DONUT_MINOR + d, 0.0, 0.0)),
        SolidContainment::In
    );
    assert_eq!(
        pis(&body, Point3::new(DONUT_R - DONUT_MINOR - d, 0.0, 0.0)),
        SolidContainment::Out
    );
    // Off the chart seam, so no answer here is the seam meridian's
    // doing: the same inner-equator pair at 45°, where both the hole
    // probe and the material probe sit at a generic azimuth.
    let s = core::f64::consts::FRAC_1_SQRT_2;
    let at = |rho: f64| Point3::new(rho * s, 0.0, rho * s);
    assert_eq!(
        pis(&body, at(DONUT_R - DONUT_MINOR - d)),
        SolidContainment::Out,
        "the hole, at a generic azimuth"
    );
    assert_eq!(
        pis(&body, at(DONUT_R - DONUT_MINOR + d)),
        SolidContainment::In
    );
    assert_eq!(pis(&body, at(DONUT_R)), SolidContainment::In);
}

/// **The MINOR window alone.** The spool's torus band wraps the major
/// azimuth — a full revolve, with a self-mated seam — and is trimmed by
/// the arc's own two ends in the minor angle. A point on the torus's
/// CARRIER beyond either end is outside the solid, not on its boundary:
/// the carrier there is the continuation of the tube past the band, and
/// the band does not bound it.
#[test]
fn the_minor_window_trims_the_spool_band() {
    let body = spool();
    let (big_r, small_r) = spool_arc();
    assert!(big_r > small_r, "the spool's band must be a ring torus");
    let d = away();
    // The band's own surface, at the widest point of the bulge (minor
    // angle zero, the outer equator of its tube) — on the boundary, and
    // material just inside it.
    assert_eq!(
        pis(&body, Point3::new(big_r + small_r, 0.5, 0.0)),
        SolidContainment::OnBoundary
    );
    assert_eq!(
        pis(&body, Point3::new(big_r + small_r - d, 0.5, 0.0)),
        SolidContainment::In
    );
    assert_eq!(
        pis(&body, Point3::new(big_r + small_r + d, 0.5, 0.0)),
        SolidContainment::Out
    );
    // The CARRIER past the band's minor window: the tube continues below
    // `y = 0` and above `y = 1`, where the spool's own boundary is a
    // planar disc. A point on that continuation is outside the solid.
    for y in [-0.2_f64, 1.2] {
        let rho = big_r + (small_r.powi(2) - (y - 0.5).powi(2)).max(0.0).sqrt();
        assert_eq!(
            pis(&body, Point3::new(rho, y, 0.0)),
            SolidContainment::Out,
            "the tube's carrier past the band's minor window is not the solid"
        );
    }
}

/// **The MAJOR window alone.** A quarter revolve of the donut's circle:
/// the torus faces wrap the minor angle (each is bounded by two full
/// meridian half-circles at the cap azimuths) and are trimmed in the
/// major azimuth. Probes at the SAME distance from the axis and the same
/// height, differing only in azimuth: the window admits the swept
/// quadrant and refuses the rest.
///
/// The fourth quadrant — the one MIRRORED across a cap — is asked in
/// [`a_revolved_disc_caps_interior_is_on_the_face`] below, beside
/// the cap reading that decides it: the quarter revolve's caps are
/// two-arc DISCS, and the planar arm must read a hit inside one as
/// inside the face, or the cap is transparent to the ray sweep and that
/// quadrant reads as material.
#[test]
fn the_major_window_trims_the_quarter_donut() {
    let body = quarter_donut();
    // The body really is a quarter: its volume is the closed form.
    let v = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
    let quarter = 2.0 * core::f64::consts::PI.powi(2) * DONUT_R * DONUT_MINOR.powi(2) / 4.0;
    assert!(
        (v - quarter).abs() < 1e-9,
        "the fixture must be a quarter of the ring: {v} vs {quarter}"
    );
    // The swept quadrant's own mid-azimuth, and the two quadrants that
    // are not mirrored across either cap.
    let deg = |d: f64| {
        let a = d.to_radians();
        Point3::new(DONUT_R * a.cos(), 0.0, DONUT_R * a.sin())
    };
    assert_eq!(pis(&body, deg(315.0)), SolidContainment::In);
    assert_eq!(pis(&body, deg(45.0)), SolidContainment::Out);
    assert_eq!(pis(&body, deg(135.0)), SolidContainment::Out);
    // The cap planes bound it, and the far field does not.
    assert_eq!(pis(&body, deg(0.0)), SolidContainment::OnBoundary);
    assert_eq!(pis(&body, deg(270.0)), SolidContainment::OnBoundary);
    assert_eq!(
        pis(&body, Point3::new(0.0, 3.0, 0.0)),
        SolidContainment::Out
    );
}

/// **A revolved disc face's interior is the face (issue 1076's
/// `point_in_face` site).** The quarter donut's cap is a two-arc disc,
/// and a point on its plane, inside the disc and off its seam diameter,
/// reads `OnBoundary` — the planar arm crosses the cap's arcs on their
/// circle rather than reading the zero-area polygon through its two
/// vertices, which answered `In` here. With the cap read, the quadrant
/// mirrored across it is not material.
#[test]
fn a_revolved_disc_caps_interior_is_on_the_face() {
    let body = quarter_donut();
    // On the cap plane x = 0, inside the disc, off its seam diameter.
    assert_eq!(
        pis(&body, Point3::new(0.0, 0.0, -(DONUT_R + 0.1))),
        SolidContainment::OnBoundary
    );
    // And the quadrant mirrored across that cap is NOT material.
    let a = 225.0_f64.to_radians();
    assert_eq!(
        pis(
            &body,
            Point3::new(DONUT_R * a.cos(), 0.0, DONUT_R * a.sin())
        ),
        SolidContainment::Out
    );
}

/// **Both windows at once**, which only a partial revolve of an arc
/// mints: the quarter spool's torus face is trimmed in the major azimuth
/// by the revolve and in the minor angle by the arc. The row separates
/// the two by moving one coordinate at a time from a point the solid
/// contains.
#[test]
fn both_windows_trim_the_quarter_spool() {
    let body = quarter_spool();
    let (big_r, small_r) = spool_arc();
    let d = away();
    let s = core::f64::consts::FRAC_1_SQRT_2;
    // A point just inside the band, at 45° of the swept quadrant.
    let rho = big_r + small_r - d;
    let quadrant = |x: f64, z: f64| Point3::new(x, 0.5, z);
    let mut inside = None;
    for (x, z) in [
        (rho * s, rho * s),
        (-rho * s, rho * s),
        (rho * s, -rho * s),
        (-rho * s, -rho * s),
    ] {
        if pis(&body, quadrant(x, z)) == SolidContainment::In {
            assert!(inside.is_none(), "only one quadrant is swept");
            inside = Some((x, z));
        }
    }
    let (x, z) = inside.expect("the swept quadrant contains material");
    // Move in the MINOR angle only, keeping the azimuth: past the arc's
    // end the carrier is no longer the face.
    let y_out: f64 = 1.2;
    let rho_out = big_r + (small_r.powi(2) - (y_out - 0.5).powi(2)).max(0.0).sqrt();
    let scale = rho_out / rho;
    assert_eq!(
        pis(&body, Point3::new(x * scale, y_out, z * scale)),
        SolidContainment::Out,
        "outside the minor window at an azimuth the major window admits"
    );
    // Move in the MAJOR azimuth only, keeping the minor angle: the
    // opposite quadrant is at the same minor angle and is not the face.
    assert_eq!(
        pis(&body, quadrant(-x, -z)),
        SolidContainment::Out,
        "outside the major window at a minor angle the minor window admits"
    );
}

/// **An uncertain root count sets its ray aside rather than answering.**
///
/// The plane `h = r` is TANGENT to the torus along its whole top circle,
/// so a query point on the axis-parallel line through that circle sees
/// near-horizontal rays whose root pairs have merged: the quartic's
/// discriminant lands in the band (`topo`'s `torus_predicate_rows` reaches
/// it at the tangent pose). Such a ray answers nothing — not a guessed
/// parity, and not a miss — and another ray of the schedule answers.
/// Walked in from a distance the door answers at, every probe clear of
/// the residual band reads `Out`.
#[test]
fn an_uncertain_root_count_sets_its_ray_aside() {
    let body = donut();
    let mut d = 1e-2_f64;
    while d > band().escalate() {
        let q = Point3::new(0.0, DONUT_MINOR + d, DONUT_R);
        assert_eq!(
            pis(&body, q),
            SolidContainment::Out,
            "{d:e} above the tube's top circle"
        );
        d /= 1.02;
    }
}

/// The shell about the tube's top circle at a given band: the OUTERMOST
/// offset above it at which the door declines to answer, walked
/// multiplicatively inward from an offset it answers at.
///
/// **A wrong answer is not a shell**, and the walk says which is which
/// rather than folding every non-`Out` into the measurement. Every probe
/// here is outside the tube by construction, so:
///
/// * a refusal counts toward the shell;
/// * `OnBoundary` is HONEST while the probe is inside the residual
///   band — the boundary pre-pass compares an exact signed distance
///   against `Zero`, and this walk runs down to offsets far below it —
///   so it counts toward the shell, but only there. Claimed further out
///   it is a defect;
/// * `In` is a wrong answer at any offset and fails the row, because
///   folding one into `shell` would let a defect WIDEN the measured
///   shell instead of going red.
fn top_circle_shell(body: &Body<f64>, b: Band) -> f64 {
    let mut shell = 0.0_f64;
    let mut d = 0.1_f64;
    while d > 1e-13 {
        let q = Point3::new(0.0, DONUT_MINOR + d, DONUT_R);
        match point_in_solid(body, q, b, Tol::witness()) {
            Ok(SolidContainment::Out) => {}
            Ok(SolidContainment::In) => panic!(
                "the door answered In at {q:?}, which is {d:e} OUTSIDE the tube — a \
                 wrong answer, not a shell"
            ),
            Ok(SolidContainment::OnBoundary) => {
                assert!(
                    d <= b.escalate(),
                    "the door called {q:?} ON the boundary at {d:e} clear of the tube, \
                     outside its own residual band {:e} — a wrong answer, not a shell",
                    b.escalate()
                );
                shell = shell.max(d);
            }
            Err(_) => shell = shell.max(d),
        }
        d /= 1.05;
    }
    shell
}

/// **The guard for [`away`]'s derivation** (Q6: a claim about ε
/// behaviour is either mechanically checked or carries the written
/// reason it cannot be). Three things, each a way the derivation could
/// go silently wrong:
///
/// 1. the fixture is still the size the ceiling is stated against, read
///    off the body's own vertices;
/// 2. the shell about the top circle is still the residual one — no
///    wider than the band — at the drawn ε, and LINEAR in ε at two
///    fixed bands: a ray the quartic cannot count that refused the query
///    again would widen it past the band, by the cube root of `K·ε` it
///    measured when it did;
/// 3. the floor still CLEARS that shell.
#[test]
fn the_clamp_floor_clears_the_shell_about_the_top_circle() {
    let body = donut();
    // (1) the fixture-size invariant, from the geometry itself.
    for (_, p) in body.points() {
        assert!(
            p.x.hypot(p.z) <= FIXTURE_EXTENT + 1e-9 && p.y.abs() <= 1.0,
            "the unit-sized-body invariant [`away`]'s ceiling rests on is broken by {p:?}"
        );
    }
    // (2) the shell, measured at the band the run drew.
    let shell = top_circle_shell(&body, band());
    assert!(
        shell <= band().escalate(),
        "the shell {shell:e} about the tube's top circle is wider than the residual \
         band {:e}: a ray the quartic cannot count refuses the query again",
        band().escalate()
    );
    // (2b) the EXPONENT, at two fixed bands three decades apart: linear
    // gives 1000, the cube law of a refusing count 10.
    let coarse = top_circle_shell(&body, Band::new(1e-9, 1e-8).unwrap());
    let fine = top_circle_shell(&body, Band::new(1e-12, 1e-11).unwrap());
    let ratio = coarse / fine;
    assert!(
        (500.0..2000.0).contains(&ratio),
        "the shell's ε-exponent moved: three decades of ε change it by {ratio:.2}×, \
         where the residual band's linear law predicts 1000"
    );
    // (3) the clearance.
    assert!(
        away() > shell * 2.0,
        "the probe offset {} no longer clears the shell {shell:e} by 2× — raise the \
         FLOOR in `away`, never widen the band or move the probe off the geometry it \
         means",
        away()
    );
}

/// **The consumer unlock.** A disjoint union has no crossings at all, so
/// the pipeline falls through to the containment fallback, which walks
/// every face of the torus-bearing operand regardless of box overlap. On
/// main that door refuses `KindUnsupported { kind: Torus }` — the
/// pair-scoped operand gate admits the operation (the boxes are units
/// apart) and the containment question then cannot be asked. With the
/// arm the union assembles.
///
/// The operand is the DONUT rather than the spool: a full revolve's
/// planar discs are two half-discs sharing one plane key, which the
/// maximal-faces precondition (F7) refuses before any containment door is
/// reached — a planar precondition, nothing to do with this arm. The
/// donut has no planar face at all.
#[test]
fn a_disjoint_union_with_a_donut_now_assembles() {
    let a = finished("the donut", donut(), Tol::witness());
    let b = brick((5.0, 6.0), (0.0, 1.0), (-1.0, 0.0), Tol::witness());
    let b = finished("the brick", b, Tol::witness());
    let out = match topo::union(&a, &b, Tol::witness()) {
        Ok(out) => out,
        Err(BooleanError::Containment(e)) => panic!(
            "the containment door still refuses a torus operand — this is the \
             refusal issue 1011's torus half retires: {e}"
        ),
        Err(other) => panic!("unexpected refusal: {other:?}"),
    };
    let result = out.body().expect("a disjoint union is not empty");
    assert_eq!(result.kind, topo::BooleanResultKind::Assembly);
    assert_eq!(topo::validate_closed(&result.body), Ok(()));
    // Both operands' material survives, and the containment door had to
    // answer for each.
    assert_eq!(
        pis(&result.body, Point3::new(DONUT_R, 0.0, 0.0)),
        SolidContainment::In
    );
    assert_eq!(
        pis(&result.body, Point3::new(5.5, 0.5, -0.5)),
        SolidContainment::In
    );
    assert_eq!(
        pis(&result.body, Point3::new(3.0, 0.5, -0.5)),
        SolidContainment::Out
    );
    assert_eq!(pis(&result.body, Point3::origin()), SolidContainment::Out);
}

/// **The refusal that stays.** The torus arm retires `KindUnsupported`
/// for `Torus` and for nothing else; `Nurbs` and `Approx` keep the
/// variant, and keep it as a capability claim about a HEALTHY body. The
/// message must stop offering "express it without a torus face" as the
/// recourse for a kind that now has an arm.
#[test]
fn the_kind_refusal_no_longer_names_the_torus() {
    let body = donut();
    for q in [
        Point3::new(DONUT_R, 0.0, 0.0),
        Point3::new(4.0, 4.0, 4.0),
        Point3::origin(),
    ] {
        let r = point_in_solid(&body, q, band(), Tol::witness());
        assert!(
            !matches!(r, Err(PointInSolidError::KindUnsupported { .. })),
            "a torus face must not refuse by kind any more: {r:?}"
        );
    }
    let msg = PointInSolidError::KindUnsupported {
        face: body.faces().next().unwrap().0,
        kind: geom::SurfaceKind::Nurbs,
    }
    .to_string();
    assert!(msg.contains("The solid itself is fine"), "{msg}");
    assert!(!msg.contains("corrupt"), "{msg}");
    assert!(
        !msg.contains("torus"),
        "the recourse must not tell a caller to avoid a kind that has an arm: {msg}"
    );
    assert!(msg.contains("spline"), "{msg}");

    // The arm's OWN refusal, for a torus face in neither class. No
    // public door mints one today — it wants a ringed torus face, or an
    // oblique (Villarceau) boundary circle — so what is pinned here is
    // the claim the message makes, not a body that reaches it.
    let msg = PointInSolidError::PartialTorusFace {
        face: body.faces().next().unwrap().0,
    }
    .to_string();
    assert!(msg.contains("The solid itself is fine"), "{msg}");
    assert!(msg.contains("Recourse"), "{msg}");
    assert!(
        msg.contains("(parallels and meridians)"),
        "the recourse names the boundary the torus arm can read: {msg}"
    );
}

/// **The radii regime that is not mintable.** Issue 1011's torus half
/// asks for both radii regimes if both are mintable through public
/// doors. They are not: `Surface::Torus` carries the ring convention
/// `R > r`, the revolve door refuses a spindle at construction, and
/// `topo::validate`'s tier-3 check reports `DegenerateTorus` on any face
/// carrying one at rest — so the containment arm is written for ring
/// tori and premises `R > r` where it needs it (the residual's
/// `ρ ≥ R − r > 0`).
///
/// This row is the receipt for the REVOLVE door and only that one: a
/// profile whose arc centre falls on the far side of the axis-parallel
/// line through the arc — a spindle — does not produce a body.
///
/// # What this receipt does NOT cover
///
/// There are three doors, not two, and the count in `validate.rs` was
/// wrong until this unit corrected it. Besides revolve:
///
/// * **`step-import`** reads `TOROIDAL_SURFACE`'s two radii verbatim, so
///   it can carry a spindle in from a file;
/// * **the BLEND lane**, reachable through the public `fillet_edges`,
///   mints `Surface::Torus` from a spine radius `s` and a blend radius
///   `r`. Its own arms document predicate 3 (`SpineIrregular`) as the
///   refusal for `0 < s ≤ r`, which is the spindle and horn case — but
///   that is the blend lane's claim about itself and this suite has not
///   measured it, and the surgery arms mint tori too.
///
/// Neither is measured here, and neither should be read as covered by
/// the row below. What IS closed regardless of the door is the concrete
/// hazard: a spindle reaching the minor-window trim would divide by a
/// vanishing radial, so `point_on_torus_in_face` decides the ring
/// convention at entry (`geom::ring_torus`) and takes a typed
/// refusal instead of a poison frame. On a ring torus it never fires.
#[test]
fn a_spindle_torus_is_not_mintable_through_the_public_door() {
    // A shallow bulge on the same chord: sagitta 0.15, arc radius
    // 0.908, centre at x = 0.242 — the centre is nearer the axis than
    // the tube radius, so `R < r`.
    let s = 0.3 / 2.0;
    let r = (0.25 + s * s) / (2.0 * s);
    let big_r = 1.0 + s - r;
    assert!(big_r < r, "this profile really does describe a spindle");
    let lp = bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 0.3),
        (Point2::new(1.0, 1.0), 0.0),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp]).validate(Tol::witness());
    let refused = match profile {
        Err(_) => true,
        Ok(vp) => revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).is_err(),
    };
    assert!(
        refused,
        "a spindle torus must not reach a body — the containment arm, the surface \
         convention and tier-3 check 1 all premise R > r"
    );
}

/// **The analytic oracle.** Two and a half thousand points on a lattice
/// through the donut, each classified by the closed-form signed distance
/// to the tube and by the door, with the band's own width excluded so
/// the oracle is never asked about a point the door is entitled to call
/// `OnBoundary`.
///
/// Two things are asserted, and the difference between them is the whole
/// posture: **zero wrong answers**, always; and an escalation RATE bound,
/// because an escalation is not a wrong answer — it is this door
/// declining a query it cannot certify — but an arm that declined
/// everywhere would still be broken and must go red.
#[test]
fn an_analytic_oracle_sweep_over_the_donut() {
    let body = donut();
    let mut wrong = Vec::new();
    let mut escalated = 0usize;
    let mut total = 0usize;
    for i in 0..17 {
        for j in 0..13 {
            for k in 0..11 {
                let q = Point3::new(
                    -1.6 + 3.2 * f64::from(i) / 16.0,
                    -0.6 + 1.2 * f64::from(j) / 12.0,
                    -1.6 + 3.2 * f64::from(k) / 10.0,
                );
                let c = donut_clearance(q);
                // Skip the band the door may honestly call OnBoundary,
                // widened to the suite's probe floor ([`away`]).
                if c.abs() < 1e-3 {
                    continue;
                }
                total += 1;
                let want = if c < 0.0 {
                    SolidContainment::In
                } else {
                    SolidContainment::Out
                };
                match point_in_solid(&body, q, band(), Tol::witness()) {
                    Ok(got) if got == want => {}
                    Ok(got) => wrong.push(format!("{q:?} clearance {c:.9} answered {got:?}")),
                    Err(_) => escalated += 1,
                }
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "{} wrong answers from the torus arm, e.g. {}",
        wrong.len(),
        wrong[0]
    );
    assert!(total > 2000, "the lattice must actually cover the body");
    assert!(
        escalated * 20 < total,
        "the arm declined {escalated} of {total} queries — a margined arm escalates \
         near its own degeneracies, but not everywhere"
    );
}
