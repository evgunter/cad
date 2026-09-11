//! **The torus wall's offset SENSE, tabulated** — the vessel scene's
//! first finding, generalized past the vessel.
//!
//! `demos/tour/src/torusvessel.rs` pins it LIVE, on the scene's own two
//! bodies: the bellied vessel's cavity stores `r − t` and the waisted
//! twin's stores `r + t`. A pin on two bodies says the door answered
//! THOSE two. This file is the other half — the claim the scene's prose
//! makes about the CLASS, executed:
//!
//! 1. **The sense is a property of the FACE, not of the surface kind.**
//!    A torus wall's inward offset moves the minor radius toward the
//!    tube's centre when the material is inside the tube (BELLIED) and
//!    away from it when the material faces the axis (WAISTED), and the
//!    only thing that decides which is the face's own orientation.
//!    Swept below over both senses, four wall thicknesses and two
//!    scales, with the stored minor radius asserted BIT for bit — every
//!    fixture station is dyadic, so a tolerance there would be hiding
//!    the claim rather than making it.
//! 2. **The junction corner is the moved shoulder plane against the
//!    moved profile circle, and it is the root on the material side.**
//!
//!    ```text
//!    ρ = R + s·√((r − s·t)² − (a − t)²)      s = +1 bellied, −1 waisted
//!    ```
//!
//!    The row asserts the corner IS there and that the OTHER root of
//!    the same quadratic is NOT — which is the claim a sense flip
//!    breaks without any validation tier noticing, since a corner
//!    solved to the wrong root still closes a valid two-shell body.
//! 3. **One body can carry BOTH senses, and each chart moves its own
//!    way in ONE `shell` call.** The hourglass fixture's meridian is a
//!    bellied band stacked on a waisted one; its hollow stores `r − t`
//!    on the first and `r + t` on the second. That is the row the two
//!    single-sense bodies cannot make: a door reading the KIND would
//!    move both the same way and still produce a tier-3 solid.
//!
//! `torax_axial` is the kernel's own table for this arm and it owns the
//! corner arithmetic, the rigid-re-pose commutation and the WAISTED
//! floor (`torax_the_torus_arms_floor_is_the_ring_closing`). What is
//! here is what a CONSUMER can see from outside the kernel, on bodies
//! authored the way the tour authors them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::authoring::{p2, validated};
use pncad::geom::Surface;
use pncad::geom_core::{Point3, Tol, Vec2};
use pncad::prelude::{Open, Start};
use pncad::profile::{ArcSweep, Center, ProfileLoop, SketchPlane};
use pncad::sweep::{Revolution, RevolveAxis, revolve};
use pncad::topo::Body;

/// The scene's meridian, in units of `1/64` m before scaling: foot
/// radius, band radius, neck radius, foot station, shoulder station,
/// mouth station, tube radius, band half-height, tube centre station.
const FOOT: f64 = 5.0 / 64.0;
const BAND: f64 = 9.0 / 64.0;
const NECK: f64 = 7.0 / 64.0;
const Y_FOOT: f64 = 4.0 / 64.0;
const Y_SHOULDER: f64 = 12.0 / 64.0;
const Y_MOUTH: f64 = 24.0 / 64.0;
const TUBE: f64 = 5.0 / 64.0;
const HALF: f64 = 4.0 / 64.0;
const H_TUBE: f64 = 8.0 / 64.0;

/// The two centres on the junctions' perpendicular bisector: bellied
/// inside them, waisted outside.
const BELLIED: f64 = 6.0 / 64.0;
const WAISTED: f64 = 12.0 / 64.0;

fn revolved(lp: ProfileLoop<f64>, tol: Tol) -> Body<f64> {
    revolve(
        &validated(SketchPlane::xy(), vec![lp], tol).expect("the meridian validates"),
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .expect("the meridian revolves")
    .body
}

/// The scene's vessel at `scale`, about `centre_rho·scale`. `scale` is
/// dyadic at every call site, so every station stays exact.
fn vessel(centre_rho: f64, winding: ArcSweep, scale: f64, tol: Tol) -> Body<f64> {
    let k = |v: f64| v * scale;
    revolved(
        Open.at(p2(0.0, 0.0))
            .line_to(p2(k(FOOT), 0.0), tol)
            .expect("the base disc")
            .line_to(p2(k(FOOT), k(Y_FOOT)), tol)
            .expect("the foot")
            .line_to(p2(k(BAND), k(Y_FOOT)), tol)
            .expect("the lower shoulder")
            .arc_to(
                Center {
                    c: p2(k(centre_rho), k(H_TUBE)),
                    winding,
                    p: p2(k(BAND), k(Y_SHOULDER)),
                },
                tol,
            )
            .expect("the band")
            .line_to(p2(k(NECK), k(Y_SHOULDER)), tol)
            .expect("the upper shoulder")
            .line_to(p2(k(NECK), k(Y_MOUTH)), tol)
            .expect("the neck")
            .line_to(p2(0.0, k(Y_MOUTH)), tol)
            .expect("the mouth disc")
            .line_to(Start, tol)
            .expect("the axis closes the meridian")
            .into(),
        tol,
    )
}

/// **Both senses in one meridian**: a BELLIED band from `Y_FOOT` to
/// `Y_SHOULDER` about `(6/64, 8/64)`, then a WAISTED one from
/// `Y_SHOULDER` to `20/64` about `(10/64, 16/64)` — the second arc's
/// own 3-4-5 twice over, so its junction residuals are exactly zero
/// too. Both bands are proper rings (`R > r`) and neither meets the
/// other: they share the shoulder annulus at `Y_SHOULDER`.
fn hourglass(tol: Tol) -> Body<f64> {
    revolved(
        Open.at(p2(0.0, 0.0))
            .line_to(p2(FOOT, 0.0), tol)
            .expect("the base disc")
            .line_to(p2(FOOT, Y_FOOT), tol)
            .expect("the foot")
            .line_to(p2(BAND, Y_FOOT), tol)
            .expect("the lower shoulder")
            .arc_to(
                Center {
                    c: p2(BELLIED, H_TUBE),
                    winding: ArcSweep::Ccw,
                    p: p2(BAND, Y_SHOULDER),
                },
                tol,
            )
            .expect("the bellied band")
            .line_to(p2(NECK, Y_SHOULDER), tol)
            .expect("the mid shoulder")
            .arc_to(
                Center {
                    c: p2(10.0 / 64.0, 16.0 / 64.0),
                    winding: ArcSweep::Cw,
                    p: p2(NECK, 20.0 / 64.0),
                },
                tol,
            )
            .expect("the waisted band")
            .line_to(p2(NECK, Y_MOUTH), tol)
            .expect("the neck")
            .line_to(p2(0.0, Y_MOUTH), tol)
            .expect("the mouth disc")
            .line_to(Start, tol)
            .expect("the axis closes the meridian")
            .into(),
        tol,
    )
}

/// `+1` on the tube's outer half (bellied), `−1` on its inner half.
fn sense_of(centre_rho: f64) -> f64 {
    if centre_rho < BAND { 1.0 } else { -1.0 }
}

/// Every torus face's stored minor radius, as bits, sorted and
/// deduplicated.
fn stored_minors(body: &Body<f64>) -> Vec<u64> {
    let mut out: Vec<f64> = body
        .faces()
        .filter_map(|(_, f)| match body.get_surface(f.surface) {
            Some(Surface::Torus { minor_radius, .. }) => Some(*minor_radius),
            _ => None,
        })
        .collect();
    out.sort_by(f64::total_cmp);
    out.dedup_by(|a, b| a.to_bits() == b.to_bits());
    out.into_iter().map(f64::to_bits).collect()
}

fn axial(p: Point3<f64>) -> (f64, f64) {
    ((p.x * p.x + p.z * p.z).sqrt(), p.y)
}

fn corners(body: &Body<f64>) -> Vec<(f64, f64)> {
    body.vertices()
        .map(|(_, v)| axial(*body.get_point(v.point).expect("a vertex carries a point")))
        .collect()
}

/// `1e-14` m absolute, `torax_axial::has_corner`'s bound — on bodies
/// whose radii run from `5e-2` to `1e0` m, so between `1e-14` and
/// `2e-13` relative, stated both ways.
const CORNER_TOL: f64 = 1e-14;

fn has_corner(body: &Body<f64>, rho: f64, h: f64) -> bool {
    corners(body)
        .iter()
        .any(|q| (q.0 - rho).abs() <= CORNER_TOL && (q.1 - h).abs() <= CORNER_TOL)
}

/// **The sweep**: one sense, one scale, every wall thickness.
fn sense_row(centre_rho: f64, winding: ArcSweep, scale: f64, what: &str) {
    let tol = Tol::witness();
    let s = sense_of(centre_rho);
    let body = vessel(centre_rho, winding, scale, tol);
    assert_eq!(
        pncad::topo::validate_geometric(&body, tol),
        Ok(()),
        "{what}: the operand is tier-3 valid before anything is asked of it"
    );
    assert_eq!(
        stored_minors(&body),
        vec![(TUBE * scale).to_bits()],
        "{what}: the operand carries ONE torus chart, at the authored tube radius"
    );

    for k in [1.0_f64, 2.0, 3.0, 4.0] {
        let t = scale * k / 128.0;
        let hollow = pncad::topo::shell(&body, t, tol)
            .unwrap_or_else(|e| panic!("{what} at t = {k}/128 x {scale}: {e}"))
            .body;
        assert_eq!(
            pncad::topo::validate_geometric(&hollow, tol),
            Ok(()),
            "{what} at t = {k}/128 x {scale}: tier 3"
        );
        assert_eq!(
            hollow.shells().count(),
            2,
            "{what} at t = {k}/128 x {scale}: outer + cavity"
        );

        // Finding 1: the stored minor radius, bit for bit.
        let moved = TUBE * scale - s * t;
        let mut want = vec![(TUBE * scale).to_bits(), moved.to_bits()];
        want.sort_unstable();
        assert_eq!(
            stored_minors(&hollow),
            want,
            "{what} at t = {k}/128 x {scale}: the cavity's chart must store {moved} = \
             r {} t. A door reading the surface KIND rather than the face would move \
             both charts the same way and still leave a tier-3 solid",
            if s > 0.0 { "-" } else { "+" }
        );

        // Finding 2: the junction corner, and the root it is NOT.
        let r = TUBE * scale - s * t;
        let a = HALF * scale - t;
        let reach = (r * r - a * a).sqrt();
        let rho = centre_rho * scale + s * reach;
        let other = centre_rho * scale - s * reach;
        for h in [Y_FOOT * scale + t, Y_SHOULDER * scale - t] {
            assert!(
                has_corner(&hollow, rho, h),
                "{what} at t = {k}/128 x {scale}: no corner at the closed form \
                 (ρ, h) = ({rho}, {h}); the body has {:?}",
                corners(&hollow)
            );
            assert!(
                !has_corner(&hollow, other, h),
                "{what} at t = {k}/128 x {scale}: a corner sits at the quadratic's \
                 OTHER root ({other}, {h}) — the root on the far side of the tube, \
                 which is not this face's material side"
            );
        }
    }
}

/// **The bellied sense** — the one no `torax_axial` fixture had, and
/// the one this tour's vessel ships.
#[test]
fn the_bellied_band_moves_its_minor_radius_toward_the_tube_centre() {
    sense_row(BELLIED, ArcSweep::Ccw, 1.0, "the bellied vessel");
    sense_row(BELLIED, ArcSweep::Ccw, 4.0, "the bellied vessel x4");
}

/// **The waisted sense** — `torax_axial`'s own, reached from the tour's
/// authoring surface and on the SAME two junction stations, so nothing
/// but the arc's centre differs between this row and the one above.
#[test]
fn the_waisted_band_moves_its_minor_radius_away_from_the_tube_centre() {
    sense_row(WAISTED, ArcSweep::Cw, 1.0, "the waisted vessel");
    sense_row(WAISTED, ArcSweep::Cw, 4.0, "the waisted vessel x4");
}

/// **Finding 3: one body, both senses, one call.** The hourglass
/// carries a bellied band under a waisted one. Its hollow stores
/// `r − t` on the first and `r + t` on the second — two torus charts
/// of the same authored radius moving OPPOSITE ways, which is the
/// statement neither single-sense body can make.
#[test]
fn one_body_carrying_both_senses_moves_each_chart_its_own_way() {
    let tol = Tol::witness();
    let body = hourglass(tol);
    assert_eq!(
        pncad::topo::validate_geometric(&body, tol),
        Ok(()),
        "the hourglass operand is tier-3 valid"
    );
    assert_eq!(
        stored_minors(&body),
        vec![TUBE.to_bits()],
        "both authored bands have the SAME tube radius, so the operand carries one \
         distinct stored minor radius — which is what makes the hollow's two a result \
         and not a restatement of the input"
    );

    let t = 1.0 / 128.0;
    let hollow = pncad::topo::shell(&body, t, tol)
        .expect("a body carrying both senses hollows in one call")
        .body;
    assert_eq!(
        pncad::topo::validate_geometric(&hollow, tol),
        Ok(()),
        "the hourglass hollow: tier 3"
    );
    assert_eq!(hollow.shells().count(), 2, "outer + cavity");

    let mut want = vec![TUBE.to_bits(), (TUBE - t).to_bits(), (TUBE + t).to_bits()];
    want.sort_unstable();
    assert_eq!(
        stored_minors(&hollow),
        want,
        "the two cavity charts must store {} and {} — the bellied band's minor radius \
         moving IN and the waisted band's moving OUT, in the same call, off one \
         authored number",
        TUBE - t,
        TUBE + t
    );

    // And the two bands' junction corners, each at its own root.
    //
    // **The MID SHOULDER is shared and moves ONCE**, which is what
    // makes this fixture worth building rather than stacking two
    // separate vessels. Material stands below it — the bellied band is
    // wider there than the waisted one is above — so it moves DOWN,
    // and the same moved plane stands `HALF − t` from the bellied
    // band's tube centre and `HALF + t` from the waisted band's. Two
    // different half-chords off one displacement.
    let mid = Y_SHOULDER - t;
    let bell_r = TUBE - t;

    // The bellied band, both ends: its lower junction is the lower
    // shoulder moved UP, its upper is the mid shoulder moved down, and
    // both stand `HALF − t` from `H_TUBE`.
    let bell_reach = (bell_r * bell_r - (HALF - t) * (HALF - t)).sqrt();
    let bell_rho = BELLIED + bell_reach;
    for h in [Y_FOOT + t, mid] {
        assert!(
            has_corner(&hollow, bell_rho, h),
            "the bellied band has no corner at its closed form (ρ, h) = \
             ({bell_rho}, {h}); the body has {:?}",
            corners(&hollow)
        );
    }

    // The waisted band above it, about `(10/64, 16/64)`. Its lower
    // junction is that SAME moved plane, now `HALF + t` from its own
    // tube centre; its upper is the neck CYLINDER, so the roles swap
    // and the closed form solves for `h` instead.
    let (waist_c, waist_h) = (10.0 / 64.0, 16.0 / 64.0);
    let waist_r = TUBE + t;
    let waist_rho = waist_c - (waist_r * waist_r - (HALF + t) * (HALF + t)).sqrt();
    assert!(
        has_corner(&hollow, waist_rho, mid),
        "the waisted band has no corner at its closed form (ρ, h) = \
         ({waist_rho}, {mid}) — the SAME moved plane the bellied band's upper \
         junction rides, at the other band's own half-chord; the body has {:?}",
        corners(&hollow)
    );
    let neck_rho = NECK - t;
    let waist_top =
        waist_h + (waist_r * waist_r - (waist_c - neck_rho) * (waist_c - neck_rho)).sqrt();
    assert!(
        has_corner(&hollow, neck_rho, waist_top),
        "the waisted band's cylinder junction is not at its closed form \
         (ρ, h) = ({neck_rho}, {waist_top}); the body has {:?}",
        corners(&hollow)
    );
}
