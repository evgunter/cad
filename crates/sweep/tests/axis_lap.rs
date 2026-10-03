//! **Box cuts of a cylinder whose cutter face meets a cap along a chord
//! with ONE rim arc between its ends.** The chord's two ends are then
//! adjacent on the cap's loop, and the join's adjacency skip asks
//! whether the rim arc between them IS the section segment. On the
//! boolean lanes that is structural: the segment is that edge only when
//! the matched germs' locus names it, and a chord across the cap lies
//! inside the cap, so the arc is never it and the chord is minted.
//!
//! The rod is `r = 0.5` about `z` over `z ∈ [0, 4]`, an extruded
//! circle: two semicircles meeting at `(±0.5, 0)`, so its wall carries
//! two ruling edges at `x = ±0.5, y = 0`. Every cutter spans `x ∈ [−1,
//! 1]` unless it says otherwise. What each pose does:
//!
//! - a FULL-LENGTH flat (the cutter past both caps) builds, certifies,
//!   and has the analytic volume, at every depth — through the axis,
//!   off it, and from either side;
//! - a LAP (the cutter from `z = 3` past the far cap, so one end wall
//!   sits inside the rod) off the axis, or through the axis across the
//!   rulings (`x = 0`), builds at the analytic volume: the cutter's
//!   edges pierce the wall, and the pierce rings join;
//! - a lap in the plane `y = 0`, which holds both ruling edges, builds
//!   under every op at the analytic volume — and so does the all-planar
//!   diamond prism whose side edges sit in that same plane: each section
//!   segment along a ruling names that edge at both of its ends;
//! - OBLIQUE caps (ellipse rims, from the plane split) flatted the same
//!   way take the same arm with an ellipse arc, mint their chords, and
//!   refuse one door later, where the containment door cannot measure
//!   an obliquely trimmed wall in closed form.
//! - a flat cutter with a thin half-rod on the axis leaves role
//!   resolution only the rim's CHORD midpoint to probe, which is on
//!   neither flanking region, and the join refuses `SectionLoopMixed`
//!   (`work/join/role-resolution-interior-tiers-certify-only-planar-region-faces`);
//! - a blind D pocket in a block builds from either face: from the
//!   bottom its floor's chord has the D's arc between its ends; from the
//!   top the D's arc side closes the ring-lane run the flat side's
//!   copies open, so role resolution winds that run by the arc the join
//!   mints, not by a straight chord.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use sweep::ExtrudeSide;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::{brick, finished};
use sweep::{Extrusion, extrude};
use topo::{AtRestBody, Body, BooleanError, SplitJoinError};

const R: f64 = 0.5;
const LEN: f64 = 4.0;

fn tol() -> Tol {
    Tol::witness()
}

fn extruded(plane: SketchPlane<f64>, lp: profile::ProfileLoop<f64>, h: f64) -> Body<f64> {
    let profile = Profile::new(plane, vec![lp]).validate(tol()).unwrap();
    extrude(
        &profile,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body
}

fn polygon(pts: &[(f64, f64)]) -> profile::ProfileLoop<f64> {
    bulge_loop(pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect())
}

/// The rod: an extruded circle.
fn rod() -> AtRestBody<f64> {
    let disc = profile::circle(Point2::new(0.0, 0.0), R, tol()).unwrap();
    finished(
        "the rod",
        extruded(SketchPlane::xy(), disc.into(), LEN),
        tol(),
    )
}

/// The rod's all-planar twin: a square turned 45°, its corners where
/// the rod's semicircles meet, so its side edges `x = ±0.5, y = 0` are
/// the rod's rulings.
fn diamond() -> AtRestBody<f64> {
    let square = polygon(&[(R, 0.0), (0.0, R), (-R, 0.0), (0.0, -R)]);
    finished(
        "the diamond",
        extruded(SketchPlane::xy(), square, LEN),
        tol(),
    )
}

fn cut(
    a: &AtRestBody<f64>,
    x: (f64, f64),
    y: (f64, f64),
    z: (f64, f64),
) -> Result<AtRestBody<f64>, BooleanError> {
    let cutter = finished("the cutter", brick(x, y, z, tol()), tol());
    topo::subtract(a, &cutter, tol()).map(|r| r.body().expect("a body remains").body.clone())
}

/// The lap: the cutter starts inside the rod at `z = 3` and runs past
/// the far cap.
const LAP: (f64, f64) = (3.0, 4.5);
/// The full-length flat: the cutter runs past both caps.
const FLAT: (f64, f64) = (-1.0, 5.0);
const ACROSS: (f64, f64) = (-1.0, 1.0);

/// The area of the disc segment `y ≥ d` of radius `R`.
fn segment(d: f64) -> f64 {
    R * R * (d / R).acos() - d * (R * R - d * d).sqrt()
}

/// The body certifies at rest and has `expect` for its volume.
fn assert_sound(body: &Body<f64>, expect: f64, what: &str) {
    topo::validate_geometric_certificate(body, tol())
        .unwrap_or_else(|e| panic!("{what}: the result does not certify at rest: {e:?}"));
    let v = topo::mass_properties(body, tol()).unwrap().volume;
    assert!(
        (v - expect).abs() < 1e-9,
        "{what}: volume {v} against the analytic {expect}"
    );
}

/// **The row's own pose, and its mirror, under every op.** The cutter's
/// face `y = 0` holds the operand's two side edges over `z ∈ [3, 4]`,
/// and its end wall `z = 3` sits inside the operand, so each side edge
/// carries a section segment from a vertex-vertex site to a
/// vertex-on-face site. The cap chord lies on the diameter between the
/// semicircles' shared vertices, so ONE semicircle lies between its
/// ends, and the cutter's face must take the straight diameter there.
/// The rod and its planar twin build alike, at the closed form, through
/// tiers 2, 3 and 3′ — rod ∖ and rod ∩ included, which a flank rule
/// that only made the two ends agree built a semicircle wrong.
#[test]
fn an_axis_lap_builds_every_op_as_its_planar_twin_does() {
    let (rod_v, diamond_v, cutter_v) = (PI * R * R * LEN, 2.0 * R * R * LEN, 2.0 * 1.5);
    for y in [(0.0, 1.0), (-1.0, 0.0)] {
        let cutter = finished("the lap cutter", brick(ACROSS, y, LAP, tol()), tol());
        for (name, body, v, held) in [
            ("rod", rod(), rod_v, PI * R * R / 2.0),
            ("diamond", diamond(), diamond_v, R * R),
        ] {
            for (op, r, want) in [
                ("∪", topo::union(&body, &cutter, tol()), v + cutter_v - held),
                ("∖", topo::subtract(&body, &cutter, tol()), v - held),
                ("∩", topo::intersect(&body, &cutter, tol()), held),
            ] {
                let what = format!("{name} {op} the lap at y ∈ {y:?}");
                let r = r.unwrap_or_else(|e| panic!("{what}: {e:?}"));
                let bb = r.body().expect("a body remains");
                topo::validate_closed(&bb.body).unwrap_or_else(|e| panic!("{what}: tier 2: {e:?}"));
                topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
                    .unwrap_or_else(|e| panic!("{what}: tier 3′: {e:?}"));
                assert_sound(&bb.body, want, &what);
            }
        }
    }
}

/// Laps off the rulings: the cutter's end-wall edges pierce the rod's
/// wall inside a face, the plane through the axis at `x = 0` included,
/// so the axis alone is not what the lap above refuses on. Each pierce
/// mints a ring in the wall, the ring's chords take their arcs from the
/// wall face's window, and the lap is the rod less one metre of the
/// disc segment.
#[test]
fn laps_off_the_rulings_build_at_the_analytic_volume() {
    let rod_v = PI * R * R * LEN;
    for (x, y, d) in [
        (ACROSS, (0.2, 1.0), 0.2),
        (ACROSS, (0.35, 1.0), 0.35),
        ((0.0, 1.0), ACROSS, 0.0),
        ((-1.0, 0.0), ACROSS, 0.0),
    ] {
        let body =
            cut(&rod(), x, y, LAP).unwrap_or_else(|e| panic!("lap at x ∈ {x:?}, y ∈ {y:?}: {e:?}"));
        assert_sound(
            &body,
            rod_v - segment(d) * (LEN - LAP.0),
            &format!("lap at x ∈ {x:?}, y ∈ {y:?}"),
        );
    }
}

/// Full-length flats at every depth: the cap chord has one rim arc
/// between its ends at each cap, and each is minted. The result is the
/// rod less the segment prism.
#[test]
fn full_length_flats_build_at_the_analytic_volume() {
    let rod_v = PI * R * R * LEN;
    for (x, y, d) in [
        (ACROSS, (0.0, 1.0), 0.0),
        (ACROSS, (-1.0, 0.0), 0.0),
        (ACROSS, (0.2, 1.0), 0.2),
        (ACROSS, (0.35, 1.0), 0.35),
        ((0.0, 1.0), ACROSS, 0.0),
    ] {
        let body =
            cut(&rod(), x, y, FLAT).unwrap_or_else(|e| panic!("x ∈ {x:?}, y ∈ {y:?}: {e:?}"));
        assert_sound(
            &body,
            rod_v - segment(d) * LEN,
            &format!("flat at x ∈ {x:?}, y ∈ {y:?}"),
        );
    }
}

/// **The ellipse carrier on the same arm.** The rod is split by the
/// planes `z = 0.5 + tan θ · y` and `z = 3.5 + tan θ · y` (tilted `θ`
/// about `x`) and the part between them kept: both caps are planar
/// faces bounded by ellipse arcs meeting at the rulings. The flat
/// `y ≥ 0.2` cuts each cap along a chord with one ellipse arc between
/// its ends, so every conic that arm meets is an ellipse.
///
/// The arm mints both chords; the result then refuses
/// `Containment(VolumeUncertified)`, because the containment door's
/// orientation probe measures the body in closed form and an
/// obliquely trimmed wall has none; the boolean's volume backstop
/// measures the same shapes through the certified quadrature
/// (`work/contact/at-infinity-probe-measures-in-closed-form-only`).
#[test]
fn an_oblique_cap_flats_through_its_ellipse_arc() {
    let theta = 20f64.to_radians();
    let normal = Vec3::new(0.0, -theta.sin(), theta.cos());
    let part = |body: &Body<f64>, z0: f64, above: bool| -> Body<f64> {
        let split = topo::split(
            body,
            &topo::test_support::split_plane(
                Point3::new(0.0, 0.0, z0),
                normal,
                geom_core::Tol::witness(),
            ),
            tol(),
        )
        .expect("the oblique split runs");
        let topo::SplitPart::Body(kept) = (if above { split.above } else { split.below }) else {
            panic!("the rod has material on the kept side of z0 = {z0}");
        };
        kept
    };
    let capped = finished(
        "the oblique-capped rod",
        part(&part(&rod(), 3.5, false), 0.5, true),
        tol(),
    );
    // Between two parallel planes 3 apart along z, over the disc.
    assert_sound(&capped, 3.0 * PI * R * R, "the oblique-capped rod");
    let err = cut(&capped, ACROSS, (0.2, 1.0), FLAT).expect_err("the flat refuses");
    assert!(
        matches!(
            err,
            BooleanError::Containment(topo::PointInSolidError::VolumeUncertified)
        ),
        "{err:?}"
    );
}

/// **Role resolution reads a curved edge at a point ON it.** The cutter
/// is the half-space `y ≥ 0` over the rod's whole length, with a thin
/// half-rod (`r = 0.1`) on the axis, bulging either way. Every rod
/// vertex sits on the cutter's boundary, so each loop's regions decide
/// at an edge: a rim semicircle's midpoint along its carrier, never its
/// chord midpoint (the circle's centre, on neither flanking region,
/// which once read both loops alike). The half rod remains, less the
/// bump where it bulges into it and plus the bump where it bulges away.
#[test]
fn a_rim_semicircle_decides_role_resolution_at_its_own_midpoint() {
    let half = PI * R * R / 2.0;
    let bump = PI * 0.1 * 0.1 / 2.0;
    for (bulge, area) in [(1.0, half - bump), (-1.0, half + bump)] {
        let cutter = extruded(
            SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, -1.0))),
            bulge_loop(vec![
                (Point2::new(-1.0, 0.0), 0.0),
                (Point2::new(-0.1, 0.0), bulge),
                (Point2::new(0.1, 0.0), 0.0),
                (Point2::new(1.0, 0.0), 0.0),
                (Point2::new(1.0, 1.0), 0.0),
                (Point2::new(-1.0, 1.0), 0.0),
            ]),
            6.0,
        );
        let cutter = finished("the half-space cutter", cutter, tol());
        let r = topo::subtract(&rod(), &cutter, tol())
            .unwrap_or_else(|e| panic!("bulge {bulge}: {e:?}"));
        let body = &r.body().expect("a half rod remains").body;
        assert_sound(body, area * LEN, &format!("bulge {bulge}"));
    }
}

/// The block `[−1, 1]² × [0, 1]` minus a D-profile rod (chord `x = 0.3`,
/// major arc `r = 0.5` about the origin) extruded `1.0` from `z = z0`.
fn d_pocket(z0: f64) -> Result<topo::BooleanResult<f64>, BooleanError> {
    let block = extruded(
        SketchPlane::xy(),
        polygon(&[(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
        1.0,
    );
    let block = finished("the block", block, tol());
    let c = sweep::test_support::rod_chord_at(0.3);
    let d = extruded(
        SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0))),
        bulge_loop(vec![
            (Point2::new(0.3, c.half), c.wall_bulge),
            (Point2::new(0.3, -c.half), 0.0),
        ]),
        1.0,
    );
    let d = finished("the D rod", d, tol());
    topo::subtract(&block, &d, tol())
}

/// **A blind D pocket, from either face.** Entering through the BOTTOM
/// face, the block's floor meets the D's flat wall along a chord whose
/// ends are adjacent on the floor's new ring with the major arc between
/// them — the plane×plane arm's conic question. Entering through the
/// TOP face, the flat side joins first and the arc side's match is
/// handed a ring run of the flat side's two copies: closed by the
/// straight chord it encloses nothing in either role order, and closed
/// by the arc the join mints it is the D, counterclockwise in exactly
/// one. Both build at the block less the D's area over the pocket's
/// depth `0.5`, hold tiers 2 and 3′ and the at-rest certificate, and
/// are legal operands.
#[test]
fn a_blind_d_pocket_builds_from_either_face() {
    for (face, z0) in [("bottom", -0.5), ("top", 0.5)] {
        let what = format!("the {face}-entry D pocket");
        let r = d_pocket(z0).unwrap_or_else(|e| panic!("{what} builds: {e:?}"));
        let bb = r.body().expect("a body remains");
        topo::validate_closed(&bb.body).unwrap_or_else(|e| panic!("{what}: tier 2: {e:?}"));
        topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
            .unwrap_or_else(|e| panic!("{what}: tier 3′: {e:?}"));
        assert_sound(&bb.body, 4.0 - (PI * R * R - segment(0.3)) * 0.5, &what);
        sweep::test_support::assert_legal_operand(&what, &bb.body, tol());
    }
}

/// **A split whose section is too nearly a circle to name reads as the
/// split's own refusal.** The plane is tilted from square to the rod by
/// the angle that puts the section ellipse's semi-axis difference,
/// `R(1/cos θ − 1)`, in the middle of the band — so the tilt itself is
/// decided and the carrier's kind is not. The escalation says what it
/// hinged on and offers the split's levers: no declaration (a split
/// takes none) and no carrier to construct (no door the user reaches
/// builds one).
#[test]
fn a_split_whose_section_is_nearly_a_circle_offers_the_splits_levers() {
    let band = geom_core::Band::linear(tol()).unwrap();
    let difference = (band.zero() + band.escalate()) / 2.0;
    let theta = (R / (R + difference)).acos();
    let err = topo::split(
        &rod(),
        &topo::test_support::split_plane(
            Point3::new(0.0, 0.0, LEN / 2.0),
            Vec3::new(0.0, -theta.sin(), theta.cos()),
            geom_core::Tol::witness(),
        ),
        tol(),
    )
    .expect_err("the section's kind is undecided");
    let text = err.to_string();
    println!("[axis_lap] the nearly-circular split: {text}");
    assert!(
        matches!(
            err,
            topo::SplitError::Join(SplitJoinError::Section {
                source: geom_brep::SectionError::Carrier(geom::EllipseInvalid::Escalated(_)),
                ..
            })
        ),
        "the carrier's constructor escalates: {err:?}"
    );
    assert!(
        text.starts_with(
            "whether the curve is a circle or an ellipse is undecided for the section through \
             a curved face: "
        ) && text.ends_with(&format!("Recourse: {}", geom_core::NO_DECLARATION_RECOURSE)),
        "{text}"
    );
    assert!(
        test_utils::refusal::subjectless_escalations(&text).is_empty(),
        "{text}"
    );
}

/// A C — the annular sector about `(−0.5, 0)` between radii `ro` and
/// `ri`, sweeping `sweep` and open about `+x`, its sides one arc each —
/// on the plane `z = z0`, extruded `h`, with its area.
fn annular_sector(ro: f64, ri: f64, sweep: f64, z0: f64, h: f64) -> (Body<f64>, f64) {
    let (cx, g) = (-0.5, PI - sweep / 2.0);
    let at = |r: f64, a: f64| Point2::new(cx + r * a.cos(), r * a.sin());
    let b = (sweep / 4.0).tan();
    let lp = bulge_loop(vec![
        (at(ro, g), b),
        (at(ro, -g), 0.0),
        (at(ri, -g), -b),
        (at(ri, g), 0.0),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    (extruded(plane, lp, h), sweep / 2.0 * (ro * ro - ri * ri))
}

/// **An engraved C builds at every sweep.** A blind annular-sector
/// pocket whose sides are one arc each: each arc meets two lines at
/// sharp corners, and on the cap's ring its match closes a run whose
/// straight chord, past a sweep near 130°, winds the other way than the
/// arc the join mints. Sunk `0.05` into the top and bottom caps of the
/// cylinder `r = 1`, `z ∈ [0, 2.5]` and into a box's top face, cut
/// through the cylinder, and stood `0.05` proud as a boss, each at
/// sweeps 135°, 180° and 270° and radii `0.25 / 0.15` and `0.4 / 0.1`,
/// builds at its closed form through tiers 2 and 3′ and the at-rest
/// certificate.
#[test]
fn an_engraved_one_arc_c_builds_at_every_sweep() {
    let cyl = extruded(
        SketchPlane::xy(),
        profile::circle(Point2::new(0.0, 0.0), 1.0, tol())
            .unwrap()
            .into(),
        2.5,
    );
    let (vc, vbox) = (PI * 2.5, 2.0 * 2.0 * 1.0);
    let block = brick((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.0), tol());
    for (ro, ri) in [(0.25, 0.15), (0.4, 0.1)] {
        for deg in [135.0_f64, 180.0, 270.0] {
            let sweep = deg.to_radians();
            let c = |z0, h| annular_sector(ro, ri, sweep, z0, h);
            let (top, a) = c(2.45, 0.1);
            let poses = [
                (
                    "the top cap",
                    topo::subtract(&cyl, &top, tol()),
                    vc - a * 0.05,
                ),
                (
                    "the bottom cap",
                    topo::subtract(&cyl, &c(-0.05, 0.1).0, tol()),
                    vc - a * 0.05,
                ),
                (
                    "a box's top face",
                    topo::subtract(&block, &c(0.95, 0.1).0, tol()),
                    vbox - a * 0.05,
                ),
                (
                    "a through cut",
                    topo::subtract(&cyl, &c(-1.0, 4.5).0, tol()),
                    vc - a * 2.5,
                ),
                ("a boss", topo::union(&cyl, &top, tol()), vc + a * 0.05),
            ];
            for (pose, r, want) in poses {
                let what = format!("{pose}, {deg}°, radii {ro} / {ri}");
                let r = r.unwrap_or_else(|e| panic!("{what}: builds: {e:?}"));
                let bb = r.body().unwrap_or_else(|| panic!("{what}: a body"));
                topo::validate_closed(&bb.body).unwrap_or_else(|e| panic!("{what}: tier 2: {e:?}"));
                topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
                    .unwrap_or_else(|e| panic!("{what}: tier 3′: {e:?}"));
                assert_sound(&bb.body, want, &what);
            }
        }
    }
}
