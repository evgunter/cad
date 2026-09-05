//! **The torus-walled vessel** — a wall whose meridian arc is centred
//! OFF the axis, hollowed.
//!
//! The teapot next door is `shell`'s designated demo and its belly is a
//! SPHERE zone: one arc about a centre ON the axis. Push that centre
//! off the axis and the revolve mints a TORUS, and until #1494 that one
//! step made the body structurally unhollowable — the meridian
//! reduction had no torus arm, the body fell to the per-chart loop, and
//! the C5 table refused its plane × torus pair. `teapot`'s wall 1 was
//! exactly that shape, pinned by exactly that refusal.
//!
//! Both doors are open now. TORAX (#1494) taught the offset-axial
//! reduction that a coaxial torus's meridian is a circle centred
//! `(R, h_c)` — the sphere's circle with one more number — and C5ARMS
//! PR-1 (#1577) routed the plane × torus SECTION the same bodies' rims
//! ask for. So this scene is the shape the teapot's wall 1 was standing
//! in front of, built as a part: a vessel with a genuine donut band in
//! its wall, hollowed, opened, and metered against closed forms it
//! derives here.
//!
//! Two panels, one meridian:
//!
//! - **the sealed vessel** — `shell`, a wall and a cavity in one solid.
//!   Drawn see-through, because a cavity cannot be read from an opaque
//!   render at any camera (the hollow ring's founding reason). Its
//!   remaining wall is STEP, declared at the body.
//! - **the opened cup** — `shell_open` at the mouth chart, then
//!   `merge_coplanar_faces`. One shell, one annular rim, genus 0, and
//!   it leaves as STEP, which the sealed body cannot.
//!
//! # Findings this scene records (the demo-purpose rule)
//!
//! 1. **The torus arm's offset sense is read off the FACE, and the
//!    BELLIED sense — the one no fixture had — moves the minor radius
//!    the other way.** `torax_axial`'s two consumers are both WAISTED:
//!    their arcs bulge toward the axis about a centre outside them, so
//!    the material faces the axis and an inward wall moves the minor
//!    radius OUT, to `r + t`. This vessel's band is BELLIED — the arc
//!    bulges away from the axis, the material is the tube's own inside,
//!    and an inward wall moves the minor radius IN, to `r − t`. Both
//!    are asserted here on ONE meridian and TWO centres: the same two
//!    junction stations, the same `5/64` tube radius, the two centres
//!    on their perpendicular bisector. The bellied twin is what the
//!    scene ships; the waisted twin is built and measured beside it,
//!    because a sense claim needs both signs.
//!
//!    `tests/verbs_torus_vessel.rs` is the CLASS behind those two
//!    bodies: both senses over four wall thicknesses and two scales,
//!    the corner asserted at the closed form AND absent at the
//!    quadratic's other root, plus one body carrying BOTH bands whose
//!    hollow moves each chart its own way in a single `shell` call —
//!    which is the row neither single-sense body can make, since a
//!    door reading the surface KIND would move both the same way and
//!    still leave a tier-3 solid.
//! 2. **The bellied sense cannot reach the floor TORAX brackets, and
//!    the reason is arithmetic rather than luck.** The waisted arm's
//!    own floor is the ring convention closing — `r + t` climbing to
//!    `R`, which `torax_the_torus_arms_floor_is_the_ring_closing`
//!    pins at the `OffsetError::TorusRing` mint. An inward wall on a
//!    BELLIED band SHRINKS the minor radius, so `R > r` only gets
//!    safer and that mint is unreachable from this side. What stops
//!    this vessel instead is the OPERAND's own `wall_clearance` gate,
//!    on a pair that has nothing to do with the torus: the two
//!    shoulder annuli, facing each other across the belly. Bracketed
//!    below, with the payload.
//! 3. **The 1031B coda arrives unchanged on a torus-walled body.** The
//!    opened cup carries the same seam-split coplanar annuli the
//!    teapot cup does — the revolve's seam cuts every latitude annulus
//!    in half — and `merge_coplanar_faces` repairs them to the same
//!    census, 25 → 19 faces. That number is a function of the
//!    meridian's STEP COUNT, not of what the wall between the steps
//!    is: this vessel's belly is a torus where the teapot's was a
//!    stepped cylinder, and the drop is identical.
//! 4. **The rim of a SECTIONED vessel still refuses, and it is the
//!    klein elbow's wall on this scene's own body.** Revolve this same
//!    meridian a quarter turn — a cutaway, the display model a potter's
//!    catalogue wants — and the moved meridian cap cuts the torus in a
//!    SPIRIC quartic. `Curve3` has no quartic carrier, so the latitude
//!    mint names the off-axis centre it will not carry and refuses.
//!    Probed live below (wall 1). The sphere half of the same rim
//!    family is BUILT (`torax_the_sphere_lune_rim_solves_in_closed_
//!    form`); what stands between a sphere lune and `shell` is a
//!    different door again, the props inventory's
//!    (`torax_the_sphere_lune_next_door_is_the_props_inventory`). The
//!    torus half is design-gated on VERBS-RIMCAP's PR-2 conversation,
//!    so this wall is measured-red and stays that way until that is
//!    funded.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use pncad::authoring::{p2, validated};
use pncad::geom::Surface;
use pncad::geom_core::{Point3, Tol, Vec2};
use pncad::prelude::{Open, Start};
use pncad::profile::{ArcSweep, Center, ProfileLoop, SketchPlane};
use pncad::sweep::{Revolution, RevolveAxis, revolve};
use pncad::topo::{Body, FaceKey, ShellError};

use crate::{SceneBody, Stop, View};

// ---------------------------------------------------------------------
// The meridian. Every station is a dyadic rational in metres, so the
// profile's own arithmetic is exact and the closed forms below are
// compared against numbers no rounding entered.
// ---------------------------------------------------------------------

/// The foot's radius — the cylinder the vessel stands on.
const R_FOOT: f64 = 5.0 / 64.0;
/// The radius the band starts and ends at: both shoulders' outer rim.
const R_BAND: f64 = 9.0 / 64.0;
/// The neck's radius, which the mouth's rim reads.
const R_NECK: f64 = 7.0 / 64.0;
/// Where the foot ends and the lower shoulder steps out.
const Y_FOOT: f64 = 4.0 / 64.0;
/// Where the band ends and the upper shoulder steps back in.
const Y_SHOULDER: f64 = 12.0 / 64.0;
/// The mouth's plane.
const Y_MOUTH: f64 = 24.0 / 64.0;

/// The band's tube radius — the meridian arc's own radius.
const R_TUBE: f64 = 5.0 / 64.0;
/// The band's half-height: both junction stations stand this far from
/// the tube's centre station, so the arc is the 3-4-5 twice over and
/// both junction residuals are exactly zero.
const A_HALF: f64 = 4.0 / 64.0;
/// The tube's centre station, the midpoint of the two junctions.
const H_TUBE: f64 = 8.0 / 64.0;

/// **The BELLIED centre**: `3/64` inside the junctions' radius, so the
/// arc bulges AWAY from the axis to `R + r = 11/64` and the material is
/// the tube's own inside. `R > r` by `1/64`, which is the ring
/// convention with room to spare — and an inward wall gives it more.
const R_BELLIED: f64 = 6.0 / 64.0;
/// **The WAISTED centre**: the OTHER point of the junctions'
/// perpendicular bisector at distance [`R_TUBE`], `3/64` outside them,
/// so the same arc bulges TOWARD the axis to `R − r = 7/64` and the
/// material faces the axis. `torax_axial`'s two fixtures' sense.
const R_WAISTED: f64 = 12.0 / 64.0;

/// The wall thickness. A TENTH of the band's tube radius and a tenth
/// of the foot's — the narrowest run the cavity has to fit inside — so
/// every junction below is transversal by a wide margin rather than by
/// a hair, and the clearance gate is nowhere near it.
const WALL: f64 = 1.0 / 128.0;

/// The chord budget: an absolute sagitta, half the hollow ring's on a
/// body of about the ring's size. The vessel is `3/8` m tall and its
/// belly reaches `11/64` m, and the wall this scene is about is
/// `1/128` m thick — so the budget has to be small against the WALL
/// and not just against the body, or the two boundaries' meshes would
/// read as one surface at the rim.
const DELTA: f64 = 1e-3;

/// **Finding 2's premise, checked where it is authored.** The floor
/// bracket below asserts that this vessel's wall is stopped by the
/// OPERAND's clearance gate on the two shoulder annuli, and that the
/// bellied arm therefore has no mint floor of its own to reach. That
/// reading holds because the shoulders' half-clearance is below the
/// tube radius: a wall can never grow far enough to pinch the tube to
/// nothing. Re-proportion the meridian past this line and the finding
/// needs re-deriving, so the compiler refuses the build rather than
/// letting the prose go quietly stale.
const _: () = assert!(R_TUBE > (Y_SHOULDER - Y_FOOT) / 2.0);

/// **The vessel's meridian, once, for either sense.** Base disc, foot,
/// lower shoulder, ONE ARC, upper shoulder, neck, mouth disc — and the
/// only thing `centre_rho` and `winding` change is which side of the
/// junction chord the arc bulges to. That is the whole of the sense
/// claim, expressed where it cannot drift: two bodies, one profile
/// builder, one pair of stations.
fn meridian(centre_rho: f64, winding: ArcSweep, tol: Tol) -> ProfileLoop<f64> {
    Open.at(p2(0.0, 0.0))
        .line_to(p2(R_FOOT, 0.0), tol)
        .expect("the base disc")
        .line_to(p2(R_FOOT, Y_FOOT), tol)
        .expect("the foot")
        .line_to(p2(R_BAND, Y_FOOT), tol)
        .expect("the lower shoulder")
        .arc_to(
            Center {
                c: p2(centre_rho, H_TUBE),
                winding,
                p: p2(R_BAND, Y_SHOULDER),
            },
            tol,
        )
        .expect("an arc about a centre off the axis is a torus")
        .line_to(p2(R_NECK, Y_SHOULDER), tol)
        .expect("the upper shoulder")
        .line_to(p2(R_NECK, Y_MOUTH), tol)
        .expect("the neck")
        .line_to(p2(0.0, Y_MOUTH), tol)
        .expect("the mouth disc")
        .line_to(Start, tol)
        .expect("the axis closes the meridian")
        .into()
}

/// The meridian revolved about `+y` through the origin, so a vertex's
/// axial coordinates are `(hypot(x, z), y)`.
fn revolved(lp: ProfileLoop<f64>, turn: Revolution<f64>, tol: Tol) -> Body<f64> {
    revolve(
        &validated(SketchPlane::xy(), vec![lp], tol).expect("the meridian validates"),
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        turn,
        tol,
    )
    .expect("the meridian revolves")
    .body
}

/// The scene's own body: the bellied vessel, fully revolved.
fn bellied(tol: Tol) -> Body<f64> {
    revolved(
        meridian(R_BELLIED, ArcSweep::Ccw, tol),
        Revolution::Full,
        tol,
    )
}

/// The sense twin: the same stations, the other centre.
fn waisted(tol: Tol) -> Body<f64> {
    revolved(
        meridian(R_WAISTED, ArcSweep::Cw, tol),
        Revolution::Full,
        tol,
    )
}

// ---------------------------------------------------------------------
// The closed forms. One function for both senses and every wall
// thickness, so the operand's numbers and the cavity's are two
// evaluations of one form and the wall is their difference.
// ---------------------------------------------------------------------

/// `sense = +1` on the tube's OUTER half (bellied), `−1` on its inner
/// half (waisted).
fn sense_of(centre_rho: f64) -> f64 {
    if centre_rho < R_BAND { 1.0 } else { -1.0 }
}

/// **The vessel's boundary moved inward by `t`, in closed form**:
/// `(volume, area)` of the solid of revolution it bounds.
///
/// `t = 0` is the operand's own boundary; `t = WALL` is the cavity's.
/// Every station moves by `t` along its own inward normal, so the three
/// runs keep their heights exactly (both ends of each shift together)
/// and only the radii move:
///
/// ```text
/// foot    ρ = R_FOOT − t          over Y_FOOT
/// band    ρ = R ± √(r'² − u²)     over u ∈ [−a', a'],  r' = r ∓ t, a' = a − t
/// neck    ρ = R_NECK − t          over Y_MOUTH − Y_SHOULDER
/// ```
///
/// with the sign the tube's material side. The band's volume is
/// `π∫ρ² du` integrated term by term (the cross term is the circular
/// segment's own `∫√(r²−u²)`), and its area is Pappus on the arc,
/// `2π∫ρ ds = 4πr'(R·θ₀ + sense·a')` with `sin θ₀ = a'/r'` — the `r'
/// sin θ₀ = a'` cancellation is why that one is so short.
fn boundary(centre_rho: f64, t: f64) -> (f64, f64) {
    let s = sense_of(centre_rho);
    let big_r = centre_rho;
    let r = R_TUBE - s * t;
    let a = A_HALF - t;
    let theta0 = (a / r).asin();

    let (foot, neck) = (R_FOOT - t, R_NECK - t);
    // The band's own junction radius — the SAME closed form the corner
    // rows below assert against, called rather than re-typed, so an
    // area that agreed with a corner nobody had is not a thing this
    // scene can produce.
    let junction = junction_rho(centre_rho, t);

    let v_band = PI
        * (2.0 * a * big_r * big_r
            + 2.0 * s * big_r * (a * (r * r - a * a).sqrt() + r * r * theta0)
            + 2.0 * r * r * a
            - 2.0 * a * a * a / 3.0);
    let volume = PI * foot * foot * Y_FOOT + v_band + PI * neck * neck * (Y_MOUTH - Y_SHOULDER);

    let a_band = 4.0 * PI * r * (big_r * theta0 + s * a);
    let area = PI * foot * foot
        + 2.0 * PI * foot * Y_FOOT
        + PI * (junction * junction - foot * foot)
        + a_band
        + PI * (junction * junction - neck * neck)
        + 2.0 * PI * neck * (Y_MOUTH - Y_SHOULDER)
        + PI * neck * neck;
    (volume, area)
}

/// The junction corner the cavity's shoulder × band meeting solves to,
/// in closed form: the moved shoulder plane is a station `a − t` from
/// the tube's centre, the moved profile circle has radius `r ∓ t`
/// about `(R, h_c)`, and the corner is the root on the tube's own
/// material side.
fn junction_rho(centre_rho: f64, t: f64) -> f64 {
    let s = sense_of(centre_rho);
    let r = R_TUBE - s * t;
    let a = A_HALF - t;
    centre_rho + s * (r * r - a * a).sqrt()
}

// ---------------------------------------------------------------------
// Reading the built bodies from outside
// ---------------------------------------------------------------------

/// `p` in the `(ρ, h)` half-plane of the `y` axis.
fn axial(p: Point3<f64>) -> (f64, f64) {
    ((p.x * p.x + p.z * p.z).sqrt(), p.y)
}

/// `(ρ, h)` is among `body`'s corners to `1e-14` m — an ABSOLUTE bound
/// on a body whose radii are ~1e-1 m, so a relative agreement of about
/// `1e-13`, stated both ways. `torax_axial::has_corner`'s bound, on
/// this scene's own bodies.
fn assert_corner(body: &Body<f64>, rho: f64, h: f64, what: &str) {
    let all: Vec<(f64, f64)> = body
        .vertices()
        .map(|(_, v)| axial(*body.get_point(v.point).expect("a vertex carries a point")))
        .collect();
    assert!(
        all.iter()
            .any(|q| (q.0 - rho).abs() <= 1e-14 && (q.1 - h).abs() <= 1e-14),
        "{what}: no corner at the closed form (ρ, h) = ({rho}, {h}); the body has {all:?}"
    );
}

/// Every torus face's stored minor radius, sorted and deduplicated.
fn stored_minors(body: &Body<f64>) -> Vec<f64> {
    let mut out: Vec<f64> = body
        .faces()
        .filter_map(|(_, f)| match body.get_surface(f.surface) {
            Some(Surface::Torus { minor_radius, .. }) => Some(*minor_radius),
            _ => None,
        })
        .collect();
    out.sort_by(f64::total_cmp);
    out.dedup_by(|a, b| a.to_bits() == b.to_bits());
    out
}

/// The faces of the plane chart at station `y`.
fn plane_chart_at(body: &Body<f64>, y: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect()
}

fn census(body: &Body<f64>) -> (usize, usize, usize) {
    (
        body.vertices().count(),
        body.edges().count(),
        body.faces().count(),
    )
}

fn genus(body: &Body<f64>) -> i64 {
    let (v, e, f) = census(body);
    let r: usize = body.faces().map(|(_, x)| x.rings.len()).sum();
    let s = body.shells().count();
    s as i64 - (v as i64 - e as i64 + f as i64 - r as i64) / 2
}

/// **The sense assertion, run on a built body**: the cavity's torus
/// chart stores `r ∓ t` for the material side its face declares, BIT
/// for bit — both numbers are dyadic, so a tolerance here would be
/// hiding the claim rather than making it.
fn assert_offset_sense(hollow: &Body<f64>, centre_rho: f64, what: &str) {
    let s = sense_of(centre_rho);
    let moved = R_TUBE - s * WALL;
    let got: Vec<u64> = stored_minors(hollow).iter().map(|r| r.to_bits()).collect();
    let mut want = vec![R_TUBE.to_bits(), moved.to_bits()];
    want.sort_unstable();
    assert_eq!(
        got,
        want,
        "{what}: the outer band keeps the authored {R_TUBE} and the cavity's stores \
         {moved} = r {} t — if this flips, the offset is reading the KIND and not the \
         face",
        if s > 0.0 { "−" } else { "+" }
    );
}

pub fn stops(tol: Tol) -> Vec<Stop> {
    let body = bellied(tol);
    assert_eq!(census(&body), (14, 26, 14), "the vessel's operand census");
    assert_eq!(genus(&body), 0, "a vessel is a ball");
    assert_eq!(
        pncad::topo::validate_geometric(&body, tol),
        Ok(()),
        "the operand: tier 3"
    );
    let props = pncad::topo::mass_properties(&body, tol).expect("the operand's props");
    let (v_out, a_out) = boundary(R_BELLIED, 0.0);
    assert!(
        ((props.volume - v_out) / v_out).abs() < 1e-12,
        "operand V = {} vs the closed form {v_out}",
        props.volume
    );
    assert!(
        ((props.surface_area - a_out) / a_out).abs() < 1e-12,
        "operand A = {} vs the closed form {a_out}",
        props.surface_area
    );
    assert_eq!(props.volume_pad, 0.0, "closed forms need no pad");

    // ---- THE HERO: the sealed hollow ----
    //
    // Structurally impossible before #1494: the meridian reduction had
    // no torus arm, so this body fell to the per-chart loop and the C5
    // table refused its plane x torus pair. It is ATTEMPTED live on
    // every pass, exactly as the wall it replaced was.
    let sealed = pncad::topo::shell(&body, WALL, tol)
        .expect("a torus-walled vessel hollows through the axial door's torus arm")
        .body;
    assert_eq!(
        pncad::topo::validate_geometric(&sealed, tol),
        Ok(()),
        "the sealed vessel: tier 3"
    );
    assert_eq!(sealed.shells().count(), 2, "outer boundary + cavity");
    assert_eq!(
        census(&sealed),
        (28, 52, 28),
        "the operand's 14/26/14 twice — the cavity is that same boundary offset inward \
         and inserted whole through the shared void door"
    );
    assert_eq!(genus(&sealed), 0, "a sealed hollow is genus 0");
    assert_offset_sense(&sealed, R_BELLIED, "the bellied vessel");

    // The corner solves, against the closed forms derived above. Both
    // shoulders stand `A_HALF - WALL` from the tube's centre station,
    // so both junctions take the SAME radius — the band is centred on
    // its own two stations.
    let rho = junction_rho(R_BELLIED, WALL);
    assert_corner(
        &sealed,
        rho,
        Y_FOOT + WALL,
        "the cavity's lower shoulder x band junction",
    );
    assert_corner(
        &sealed,
        rho,
        Y_SHOULDER - WALL,
        "the cavity's upper shoulder x band junction",
    );
    // The two line x line corners of the same cavity, for completeness:
    // a corner solved onto the wrong root would still leave a valid
    // two-shell body, which is the claim these rows exist to break.
    assert_corner(
        &sealed,
        R_FOOT - WALL,
        Y_FOOT + WALL,
        "the cavity's foot x lower shoulder junction",
    );
    assert_corner(
        &sealed,
        R_NECK - WALL,
        Y_SHOULDER - WALL,
        "the cavity's neck x upper shoulder junction",
    );

    // The wall and the capacity, both against the same closed form at
    // two thicknesses.
    let (v_cav, a_cav) = boundary(R_BELLIED, WALL);
    let props_s = pncad::topo::mass_properties(&sealed, tol).expect("the sealed vessel's props");
    let v_wall = v_out - v_cav;
    assert!(
        ((props_s.volume - v_wall) / v_wall).abs() < 1e-12,
        "sealed V = {} vs the two boundaries' difference {v_wall}",
        props_s.volume
    );
    assert!(
        ((props_s.surface_area - (a_out + a_cav)) / (a_out + a_cav)).abs() < 1e-12,
        "sealed A = {} vs the two boundaries' sum {}",
        props_s.surface_area,
        a_out + a_cav
    );
    assert_eq!(props_s.volume_pad, 0.0, "closed forms need no pad");

    // The capacity, asked for DIRECTLY of the body rather than
    // inferred from a difference: the cavity is the `Void`-role shell
    // and its signed volume is that closed form negated.
    let classes = pncad::topo::classify_shells(&sealed, tol).expect("per-shell classification");
    let voids: Vec<_> = classes
        .iter()
        .filter(|c| c.role == pncad::topo::ShellRole::Void)
        .collect();
    assert_eq!(voids.len(), 1, "one cavity, one Void shell");
    assert!(
        ((voids[0].volume + v_cav) / v_cav).abs() < 1e-12,
        "the capacity is {} vs the closed form -{v_cav}",
        voids[0].volume
    );
    let litres = v_cav * 1000.0;

    // ---- THE SENSE TWIN: the same stations, the other centre ----
    let twin = waisted(tol);
    assert_eq!(census(&twin), (14, 26, 14), "the twin's operand census");
    let twin_hollow = pncad::topo::shell(&twin, WALL, tol)
        .expect("the waisted twin hollows through the same arm")
        .body;
    assert_eq!(
        pncad::topo::validate_geometric(&twin_hollow, tol),
        Ok(()),
        "the waisted twin: tier 3"
    );
    assert_eq!(twin_hollow.shells().count(), 2, "outer boundary + cavity");
    assert_offset_sense(&twin_hollow, R_WAISTED, "the waisted twin");
    let twin_rho = junction_rho(R_WAISTED, WALL);
    assert_corner(
        &twin_hollow,
        twin_rho,
        Y_FOOT + WALL,
        "the twin's lower shoulder x band junction",
    );
    assert_corner(
        &twin_hollow,
        twin_rho,
        Y_SHOULDER - WALL,
        "the twin's upper shoulder x band junction",
    );
    let (twin_v_out, _) = boundary(R_WAISTED, 0.0);
    let (twin_v_cav, _) = boundary(R_WAISTED, WALL);
    let twin_wall = twin_v_out - twin_v_cav;
    let twin_props =
        pncad::topo::mass_properties(&twin_hollow, tol).expect("the waisted twin's props");
    assert!(
        ((twin_props.volume - twin_wall) / twin_wall).abs() < 1e-12,
        "the twin's wall V = {} vs the same closed form at the other sense {twin_wall}",
        twin_props.volume
    );

    // ---- THE FLOOR, bracketed, and it is NOT the torus arm's ----
    //
    // Finding 2, executed. The waisted arm's floor is the ring
    // convention closing; the bellied arm has none, because an inward
    // wall shrinks the minor radius. What stops this vessel is the
    // operand's own clearance gate on the two SHOULDER annuli, which
    // face each other across the belly at Y_SHOULDER - Y_FOOT.
    let below = Y_SHOULDER - Y_FOOT - WALL;
    pncad::topo::shell(&body, below / 2.0, tol)
        .expect("a wall just under half the shoulders' clearance hollows");
    let at = pncad::topo::shell(&body, (Y_SHOULDER - Y_FOOT) / 2.0, tol)
        .expect_err("two walls that consume the shoulders' whole clearance leave no cavity");
    let ShellError::WallClearance {
        gap,
        needed,
        face,
        other,
    } = at
    else {
        panic!("the floor here is the operand clearance gate's, got {at}");
    };
    assert_ne!(face, other, "the gate names two distinct faces");
    assert!(
        (gap - (Y_SHOULDER - Y_FOOT)).abs() <= 1e-15
            && (needed - (Y_SHOULDER - Y_FOOT)).abs() <= 1e-15,
        "the shoulders stand {} m apart and two walls of half that need all of it, got \
         gap {gap} needed {needed}",
        Y_SHOULDER - Y_FOOT
    );

    // ---- THE OPENED CUP, then the 1031B coda ----
    let mouth = plane_chart_at(&body, Y_MOUTH);
    assert_eq!(
        mouth.len(),
        2,
        "the mouth is ONE plane worn by two half-discs — a full revolve's seam cut — \
         and the rim lift moves a chart as one"
    );
    let mut cup = pncad::topo::shell_open(&body, WALL, &mouth, tol)
        .expect("the vessel opens at its mouth")
        .body;
    assert_eq!(
        pncad::topo::validate_geometric(&cup, tol),
        Ok(()),
        "the cup: tier 3"
    );
    let rings: usize = cup.faces().map(|(_, f)| f.rings.len()).sum();
    assert_eq!(
        (rings, genus(&cup), cup.shells().count()),
        (1, 0, 1),
        "ONE rim annulus carrying ONE ring, genus 0 as `topo::shell`'s docs promise a \
         cup is, and the cavity fused into the boundary"
    );
    let cup_before = census(&cup);
    assert_eq!(
        cup_before,
        (26, 48, 25),
        "the cup's census before the merge"
    );
    let props_c = pncad::topo::mass_properties(&cup, tol).expect("the cup's props");
    // Opening lifts the cavity's mouth cap from its own station up to
    // the mouth plane, so the cup is the WALL less that plug — a
    // cylinder of the cavity's neck radius, one wall tall.
    let plug = PI * (R_NECK - WALL) * (R_NECK - WALL) * WALL;
    let v_cup = v_wall - plug;
    assert!(
        ((props_c.volume - v_cup) / v_cup).abs() < 1e-12,
        "cup V = {} vs the wall's closed form less the lifted mouth plug {v_cup}",
        props_c.volume
    );

    // The coda. The revolve's seam cuts every latitude annulus in half,
    // and `merge_coplanar_faces` puts them back — the teapot cup's own
    // numbers, on a body whose belly is a torus.
    let merged = cup
        .merge_coplanar_faces(tol)
        .expect("the cup's coplanar pairs must merge");
    let cup_after = census(&cup);
    assert_eq!(
        (cup_before, cup_after),
        ((26, 48, 25), (24, 36, 19)),
        "six faces absorbed, two poles killed, twelve edges eaten — the teapot cup's \
         census drop exactly, which is what makes it a function of the meridian's STEP \
         COUNT rather than of the wall between the steps"
    );
    let annuli = merged
        .groups
        .iter()
        .filter(|g| !g.rings_made.is_empty())
        .count();
    let poles = merged
        .groups
        .iter()
        .filter(|g| !g.killed_vertices.is_empty())
        .count();
    let closures = merged
        .skipped
        .iter()
        .filter(|s| {
            matches!(
                s.reason,
                pncad::topo::MergeCoplanarError::PeriodClosure { .. }
            )
        })
        .count();
    assert_eq!(
        (annuli, poles, closures),
        (4, 2, 6),
        "four full-valence latitude annuli, two pole-split base caps, six \
         period-closure skips — a curved run that would close its chart's full period \
         is a seam the merge declines by design"
    );
    assert_eq!(
        pncad::topo::validate_geometric(&cup, tol),
        Ok(()),
        "the merged cup: tier 3"
    );
    assert_eq!(
        (
            cup.faces().map(|(_, f)| f.rings.len()).sum::<usize>(),
            genus(&cup),
            cup.shells().count()
        ),
        (5, 0, 1),
        "the merge mints four annulus rings beside the rim's and moves no locus, so the \
         genus is where it was"
    );
    let props_m = pncad::topo::mass_properties(&cup, tol).expect("the merged cup's props");
    assert!(
        ((props_m.volume - v_cup) / v_cup).abs() < 1e-12,
        "the merge moves no locus, so the volume is the cup's: {} vs {v_cup}",
        props_m.volume
    );

    // ---- WALL 1: the SECTIONED vessel's rim ----
    //
    // A quarter turn of the same meridian — the cutaway a catalogue
    // wants. The moved meridian cap stands a wall off the axis and
    // parallel to it, and a plane in that posture cuts a torus in a
    // SPIRIC quartic, which `Curve3` cannot carry.
    let quarter = revolved(
        meridian(R_BELLIED, ArcSweep::Ccw, tol),
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        tol,
    );
    assert_eq!(
        pncad::topo::validate_geometric(&quarter, tol),
        Ok(()),
        "the sectioned vessel is a valid body — it is the HOLLOW that has no carrier"
    );
    let sectioned = pncad::topo::shell(&quarter, WALL, tol);
    crate::walls::wall(
        "torus-walled vessel",
        1,
        "hollow the vessel SECTIONED — a quarter turn of the same meridian, whose \
         moved meridian cap cuts the torus band in a spiric quartic",
        sectioned,
        |e| {
            matches!(
                e,
                ShellError::Face { error, .. }
                    if matches!(
                        &**error,
                        pncad::topo::ReplaceFaceError::TogetherAxialEdge { what, .. }
                            if *what == "a circular edge between two charts whose centre is off the axis"
                    )
            )
        },
        "VERBS-RIMCAP's PR-2 has funded the torus half of the rim carrier. Retire this \
         probe, retire `torax_the_klein_elbow_rim_refuses_at_the_carrier_mint` with it \
         — they are one gate — and ship the sectioned vessel as this scene's third \
         panel, which is the picture it was always for",
    );

    let (sv, se, sf) = census(&sealed);
    let (cv, ce, cf) = cup_after;

    vec![
        Stop {
            name: "torusvessel",
            caption: "THE TORUS-WALLED VESSEL, HOLLOWED (a donut band in the wall)"
                .to_string(),
            // Montage membership is CURATED (Ev, montage-v3,
            // 2026-08-30), and this scene has not been through that
            // pass. Everything it pins is a number, an export or a
            // typed refusal, none of which is a pixel; the standalone
            // render is where the shape is looked at.
            montage: false,
            story: "the teapot's belly is a sphere zone — one arc about a centre ON the \
                    axis. Push that centre OFF the axis and the wall is a TORUS, which \
                    is the shape `teapot`'s wall 1 used to pin as unhollowable. It \
                    hollows: a wall 7.8 mm thick around a cavity the size of the \
                    contents, in ONE solid. Drawn see-through, because a cavity cannot \
                    be read from an opaque render at any camera",
            ops: "revolve(meridian, +y, Full) -> shell(vessel, t = 7.8125 mm): the \
                  offset-axial reduction takes the band's meridian as a circle centred \
                  (R, h_c) — the sphere's circle with one more number — and every \
                  corner is solved against all the surfaces meeting it at once",
            delta: DELTA,
            note: Some(format!(
                "{sv} vertices, {se} edges, {sf} faces over TWO shells in one solid — \
                 the operand's 14/26/14 twice, since the cavity is that same boundary \
                 offset inward and inserted whole through the shared void door. Genus \
                 0, tier 3. THE BAND IS A REAL TORUS: R = {R_BELLIED} m, r = {R_TUBE} \
                 m, its arc the 3-4-5 twice over about ({R_BELLIED}, {H_TUBE}), so both \
                 junction residuals are exactly zero. V = {:.9} m³ of WALL against the \
                 difference of two evaluations of ONE closed form — the boundary moved \
                 inward by t, whose band term is π∫(R + √(r'²−u²))² du integrated term \
                 by term — and A = {:.9} m² against their sum, at zero enclosure pad. \
                 The capacity is asked for DIRECTLY: `classify_shells` gives the cavity \
                 the Void role and its signed volume is {:.9} m³, its {litres:.4} litres \
                 negated by the orientation convention. THE OFFSET SENSE IS THE FINDING. \
                 `torax_axial`'s two consumers are both WAISTED — their arcs bulge \
                 toward the axis about a centre outside them, so the material faces the \
                 axis and an inward wall moves the minor radius OUT, to r + t. This \
                 band is BELLIED: it bulges away from the axis, the material is the \
                 tube's own inside, and the cavity's chart stores r − t = {} m, \
                 asserted BIT for bit because both numbers are dyadic. Run on ONE \
                 meridian and TWO centres — the same two junction stations, the same \
                 {R_TUBE} m tube radius, the two points of their perpendicular \
                 bisector at that distance — so the sense claim cannot drift into a \
                 claim that two fixtures agree: the WAISTED twin is built beside this \
                 body on every pass and its cavity stores {} m. Both junction corners \
                 solve to ρ = R + √((r−t)² − (a−t)²) = {rho:.15} m at h = {} and {} — \
                 the moved shoulder plane against the moved profile circle, a \
                 circle-line root in the meridian half-plane, on the tube's own \
                 material side. THE FLOOR IS NOT THE TORUS ARM'S, AND CANNOT BE. The \
                 waisted arm's own floor is the ring convention closing, r + t \
                 climbing to R, which `torax_the_torus_arms_floor_is_the_ring_closing` \
                 pins at the OffsetError::TorusRing mint; an inward wall on a bellied \
                 band SHRINKS r, so R > r only gets safer and that mint is unreachable \
                 from this side. What stops this vessel is the OPERAND's own \
                 wall_clearance gate on a pair with no torus in it — the two shoulder \
                 annuli, facing each other across the belly: {gap} m of clearance, \
                 which a wall of half that consumes entirely. Bracketed on every pass. \
                 WHAT STILL REFUSES is the SECTIONED vessel (wall 1 above). A quarter \
                 turn of this meridian is a valid body, but its hollow's moved meridian \
                 cap stands a wall off the axis and parallel to it, and a plane in that \
                 posture cuts a torus in a SPIRIC quartic — so the latitude mint names \
                 the off-axis centre it will not carry and refuses \
                 TogetherAxialEdge. That is the klein elbow's wall \
                 (`torax_the_klein_elbow_rim_refuses_at_the_carrier_mint`) on this \
                 scene's own body, and the two retire together. The SPHERE half of the \
                 same rim family is BUILT — a plane cuts a sphere in a circle, always, \
                 and `torax_the_sphere_lune_rim_solves_in_closed_form` carries it — \
                 but a sphere lune still does not reach `shell`, at a different door \
                 again: the flux arm's props_band_coplanar premise cannot give tier 3 \
                 the volume its +V invariant needs \
                 (`torax_the_sphere_lune_next_door_is_the_props_inventory`). THE \
                 SEALED BODY'S OWN WALL IS STEP: the writer's outward/void classifier \
                 has closed forms for planar faces only, so this multi-shell CURVED \
                 solid refuses CurvedShellClassification — declared at the body and \
                 probed on every pass, the same standing gate klein's wall 6, the ring \
                 scene and `hollowtorus` name. The cup panel next door is ONE shell and \
                 exports",
                props_s.volume,
                props_s.surface_area,
                voids[0].volume,
                R_TUBE - WALL,
                R_TUBE + WALL,
                Y_FOOT + WALL,
                Y_SHOULDER - WALL,
            )),
            // The vessel's axis is +y, and 24 degrees up is where the
            // TRANSPARENT read is best: the cavity's own neck and foot
            // are visible through the wall, which is the only thing a
            // sealed hollow has to show.
            //
            // Stated because it is easy to want otherwise: the two
            // shoulder annuli — the pair the clearance gate names — are
            // NOT visible at this camera or any other. The band bulges
            // to 11/64 past both of their 9/64 rims, so they sit under
            // its overhang from every direction. That is the shape
            // being what it is rather than the camera being wrong, and
            // it is why the gate's payload is asserted as two numbers
            // above instead of being pointed at in the picture.
            view: View {
                elev: 24.0,
                azim: -58.0,
                up: 'y',
            },
            bodies: vec![
                SceneBody::plain("torusvessel", [0.76, 0.48, 0.36], sealed)
                    .transparent(45)
                    .step_at_frontier(
                        |e| {
                            matches!(
                                e,
                                pncad::step_export::StepExportError::CurvedShellClassification { .. }
                            )
                        },
                        "the writer's outward/void classifier has grown a curved arm. Say \
                         so in klein's findings entry 7 and in docs/KERNEL-VERBS.md's \
                         hollow-ring STEP row, and retire ALL FOUR probes of this one \
                         gate together: klein's WALL 6, the `ring` scene's \
                         `step_at_frontier`, `tubewall`'s `hollowtorus`, and this one",
                    ),
            ],
        },
        Stop {
            name: "torusvesselcup",
            caption: "THE SAME VESSEL, OPENED AT ITS MOUTH (one annular rim, coplanar \
                      pairs merged)"
                .to_string(),
            montage: false,
            story: "`shell_open` on the same meridian, designating the mouth's chart. \
                    The cavity fuses into the boundary and the wall's thickness comes \
                    back as an annular RIM you can look down — one shell, genus 0, no \
                    transparency needed. Then the 1031B coda: the revolve's seam cuts \
                    every latitude annulus in half, and `merge_coplanar_faces` puts \
                    them back",
            ops: "revolve(meridian, +y, Full) -> shell_open(vessel, t = 7.8125 mm, the \
                  mouth's chart) -> merge_coplanar_faces(): the seam is retired before \
                  the glue through the Euler doors alone, and the merge's arc-bounded \
                  winding arm is what lets a survivor whose outline and ring are both \
                  CIRCLES take its role",
            delta: DELTA,
            note: Some(format!(
                "{cv} vertices, {ce} edges, {cf} faces in ONE shell, genus 0, tier 3 — \
                 after the merge; {} / {} / {} before it. The rings go the OTHER way, \
                 1 -> 5: the mouth's annular rim, plus the four the merge mints as it \
                 closes each latitude annulus back up. THE CODA, MEASURED: \
                 25 → 19 faces, 26 → 24 vertices, 48 → 36 edges. Four groups mint a \
                 RING — the merge's own full-valence class, which on this meridian is \
                 the two shoulders and their cavity twins — two pole-split base caps \
                 close beside them, and six pairs \
                 are declined as PeriodClosure — a curved run that would close its \
                 chart's full period is a seam the merge refuses by design, and that is \
                 not a failure. THOSE ARE THE TEAPOT CUP'S OWN NUMBERS, on a body whose \
                 belly is a TORUS where the teapot's was a stepped cylinder: the drop is \
                 a function of the meridian's STEP COUNT, not of the wall between the \
                 steps. The merge moves no locus, and the volume says so — V = {:.9} m³ \
                 both sides of it, against the sealed wall's closed form less the plug \
                 the lift opens (a cylinder of the cavity's neck radius, one wall tall). \
                 A = {:.9} m². THE CUP LEAVES AS STEP, which the sealed panel next door \
                 cannot: the writer's outward/void classifier refuses a multi-shell \
                 CURVED solid, and a cup is ONE shell. What the merge buys past the \
                 picture is an OPERAND: the unmerged cup's coplanar pairs are what the \
                 boolean gate's F7 maximal-faces precondition refuses, and \
                 `verbs_1031b_arcwind` is where that differential is pinned on the \
                 teapot's cup",
                cup_before.0, cup_before.1, cup_before.2, props_m.volume, props_m.surface_area,
            )),
            // Ten degrees higher than the sealed panel and swung round
            // to the other quadrant: the subject here is the annular
            // RIM at the top of the neck, which needs elevation to
            // read as an annulus rather than as a line, and the belly
            // still fills the frame beneath it.
            view: View {
                elev: 34.0,
                azim: -122.0,
                up: 'y',
            },
            bodies: vec![SceneBody::plain("torusvesselcup", [0.76, 0.48, 0.36], cup)],
        },
    ]
}
