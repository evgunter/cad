//! **The section certificate's spline × plane rows**: the classifier
//! through the march door (C3's whole reading, read as components) and
//! the per-pair rule over what it lists. The op-level rows are
//! `crates/sweep/tests/germ_interior_oval.rs` and
//! `crates/sweep/tests/nurbs_plane_guard.rs`.
//!
//! Every row names the mutant that turns it red.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::search_kit::band;
use super::section_cert_rows::{IN, OUT, at, p, plane, v};
use super::*;
use geom::Surface;
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3};

/// The section through the march door, over a reach the spline arm
/// does not read.
fn read(f: &Surface<f64>, g: &Surface<f64>) -> Section<f64> {
    classify(
        f,
        g,
        Reach {
            centre: p(0.0, 0.0, 0.0),
            radius: 1.0,
        },
        Some(geom_brep::SsiLane::marched()),
        band(),
    )
}

/// A biquadratic over `[0, 1]²` whose height is `z(i, j)` at control
/// point `(i, j)`.
fn net(z: &dyn Fn(usize, usize) -> f64) -> Surface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = (0..9)
        .map(|n| {
            let (i, j) = (n / 3, n % 3);
            #[allow(clippy::cast_precision_loss)]
            p(0.5 * i as f64, 0.5 * j as f64, z(i, j))
        })
        .collect();
    Surface::Nurbs(std::sync::Arc::new(
        geom::NurbsSurface::new(k.clone(), k, control, vec![1.0; 9]).unwrap(),
    ))
}

/// The dome `z = −(x − ½)² − (y − ½)²`, its top `z = 0` at the centre.
fn dome() -> Surface<f64> {
    let g = [-0.25, 0.25, -0.25];
    net(&|i, j| g[i] + g[j])
}

/// The horizontal plane at height `h`.
fn level(h: f64) -> Surface<f64> {
    plane(p(0.5, 0.5, h), v(0.0, 0.0, 1.0))
}

/// The one component of a section of one.
fn lone(s: &Section<f64>) -> Component<f64> {
    match s {
        Section::Components { parts, .. } if parts.len() == 1 => parts[0],
        other => panic!("not one component: {other:?}"),
    }
}

/// The witness's distance from the dome's section at height `h`, the
/// circle of radius `√−h` about `(½, ½, h)`.
fn off_the_circle(w: Point3<f64>, h: f64) -> f64 {
    let r = ((w.x - 0.5).powi(2) + (w.y - 0.5).powi(2)).sqrt();
    ((r - (-h).sqrt()).powi(2) + (w.z - h).powi(2)).sqrt()
}

/// **A loop inside a plane and a wall is never cleared, and its witness
/// is placed on the band widened by its pad.** The dome cut at
/// `z = −0.1` meets the plane in the circle of radius `√0.1` about the
/// centre, inside the wall's rectangle: one closed component, reaching
/// no chart boundary, its witness within its pad of the circle and
/// that pad a few bands, not the wall's size. With no event its
/// witness decides it: `In` both faces is R-loop, no verdict is R-undec,
/// and it clears only `Out` of a face (W3); with an event it is the lone
/// evented component (W4). Against a face whose edge stands `d` from the
/// witness, past the band but inside the band widened by the pad,
/// the placement reads the witness on the edge, so it does not clear.
/// The same plane stored with its origin a kilometre off reads the same
/// loop. Red against the boundary clause read true for a closed loop
/// (every verdict clears as `Reaches`), against the pad dropped
/// between the section and the placement (the component's or the rule's
/// hand-off; the boolean's own widening is
/// `nurbs_plane_guard::a_spline_witness_by_an_edge_is_placed_on_the_widened_band`),
/// against a pad that does not scale with the band (a tube's rung),
/// and against the plane read at its stored origin rather than its
/// pivot (at ε 1e-12, where the kilometre's reach no longer settles). One reading of each plane serves every assertion: a loop's
/// accounting is the expensive one.
#[test]
fn a_loop_inside_a_plane_and_a_wall_is_never_cleared() {
    let s = read(&level(-0.1), &dome());
    let c = lone(&s);
    assert!(c.reaches.is_none(), "a loop reaches no side: {c:?}");
    let w = c.witness.expect("a loop carries a witness");
    let off = off_the_circle(w, -0.1);
    assert!(
        off <= c.pad && c.pad <= 1e3 * band().zero(),
        "the witness stands {off:e} off the circle, within its pad {:e}, a few bands of {:e}",
        c.pad,
        band().zero()
    );
    let rule = |evented, place| certify(&s, evented, band(), |_| Sheet::Not, place);
    assert_eq!(rule(false, at(IN, IN)), Err(Refusal::Loop), "in both");
    assert_eq!(
        rule(false, at(None, None)),
        Err(Refusal::Undecided),
        "placed nowhere"
    );
    assert_eq!(
        rule(false, at(IN, OUT)),
        Ok(vec![Cleared::Out(Side::G)]),
        "out of G"
    );
    assert_eq!(
        rule(true, at(IN, IN)),
        Ok(vec![Cleared::LoneEvented]),
        "evented"
    );
    // The rule pads the band by twice the pad ([`Component::pad`]): an
    // edge one pad past the band's zero reads the witness on it.
    let d = band().zero() + c.pad;
    let edge = |_, wide: Band| [if d > wide.zero() { OUT } else { None }, IN];
    assert_eq!(
        certify(&s, false, band(), |_| Sheet::Not, edge),
        Err(Refusal::Undecided),
        "a witness inside its pad of an edge does not clear"
    );
    let far = read(&plane(p(1000.0, 1000.0, -0.1), v(0.0, 0.0, 1.0)), &dome());
    assert_eq!(
        certify(&far, false, band(), |_| Sheet::Not, at(IN, IN)),
        Err(Refusal::Loop),
        "the plane stored a kilometre off reads the same loop: {far:?}"
    );
}

/// **A spline witness stands within its pad of the section**, at
/// other heights of the dome's cut: a wider and a narrower circle. Red
/// against a pad that under-states the bracket (the certificate's
/// sup bound alone).
#[test]
fn a_spline_witness_stands_within_its_residual_of_the_section() {
    for h in [-0.2, -0.03] {
        let c = lone(&read(&level(h), &dome()));
        let off = off_the_circle(c.witness.expect("a loop carries a witness"), h);
        assert!(
            off <= c.pad,
            "h {h}: the witness stands {off:e} off, past its pad {:e}",
            c.pad
        );
    }
}

/// **A loop hugging the wall's sides is a loop.** The dome cut just
/// above its rim's lowest points, `d` inside all four sides: one closed
/// component, reaching no side. Red against a loop read as reaching the
/// boundary because it comes near it.
#[test]
fn a_loop_hugging_the_walls_sides_is_a_loop() {
    for d in [1e-3, 1e-5, 1e-7] {
        let s = read(&level(-(0.5 - d) * (0.5 - d)), &dome());
        let c = lone(&s);
        assert!(
            c.reaches.is_none(),
            "d {d:e}: a loop reads as reaching: {s:?}"
        );
        assert_eq!(
            certify(&s, false, band(), |_| Sheet::Not, at(None, None)),
            Err(Refusal::Undecided),
            "d {d:e}: a loop placed nowhere is not cleared"
        );
    }
}

/// A hill near `u = 0.2` (control columns 1, 2, rows 2–4 at height 1)
/// and the wall's `u = 1` column raised to 1, everything else 0: at
/// `z = ½` the plane meets it in one closed loop round the hill and one
/// open branch near the raised side, on disjoint supports.
fn hill_and_ramp() -> Surface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.2, 0.4, 0.6, 0.8, 1.0, 1.0, 1.0], 2).unwrap();
    let control = (0..49)
        .map(|n| {
            let (i, j) = (n / 7, n % 7);
            let hill = (1..=2).contains(&i) && (2..=4).contains(&j);
            let z = if hill || i == 6 { 1.0 } else { 0.0 };
            #[allow(clippy::cast_precision_loss)]
            p(i as f64 / 6.0, j as f64 / 6.0, z)
        })
        .collect();
    Surface::Nurbs(std::sync::Arc::new(
        geom::NurbsSurface::new(k.clone(), k, control, vec![1.0; 49]).unwrap(),
    ))
}

/// **A loop beside an open branch is not cleared by an event on the
/// pair.** The event may lie on the open branch, so W4 must not read the
/// loop as the lone component: the count holds the open branch. Red
/// against `single` counting closed components alone (the loop clears
/// `LoneEvented`).
#[test]
fn a_loop_beside_an_open_branch_is_not_cleared_by_an_event() {
    let s = read(&level(0.5), &hill_and_ramp());
    let Section::Components { parts, single } = &s else {
        panic!("components: {s:?}")
    };
    let loops = parts.iter().filter(|c| c.reaches.is_none()).count();
    let open = parts.iter().filter(|c| c.reaches == Some(Side::G)).count();
    assert_eq!((loops, open, *single), (1, 1, false), "{s:?}");
    assert_eq!(
        certify(&s, true, band(), |_| Sheet::Not, at(IN, IN)),
        Err(Refusal::Undecided),
        "an evented pair with a loop beside an open branch"
    );
}

/// A tube about the `z` axis, `z ∈ [0, 1]`: its `u` row a quadratic
/// closed on itself at the corner `(1, −1)` where `closed`, or stopping
/// short of it, linear in `v` along the axis.
fn tube(closed: bool) -> Surface<f64> {
    let ku =
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let last = if closed { (1.0, -1.0) } else { (1.0, -0.5) };
    let ring = [(1.0, -1.0), (1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0), last];
    let control = ring
        .iter()
        .flat_map(|&(x, y)| [p(x, y, 0.0), p(x, y, 1.0)])
        .collect();
    Surface::Nurbs(std::sync::Arc::new(
        geom::NurbsSurface::new(ku, kv, control, vec![1.0; 10]).unwrap(),
    ))
}

/// **A branch across a closed channel's seam is not cleared by reaching
/// it.** A plane square to the tube's axis meets it in one branch from
/// one `u` side of its chart to the other. Where the `u` row closes on
/// itself those sides are a seam a face may lie across with no edge on
/// it, so W1′ does not clear the branch and, placed nowhere, it refuses
/// undecided; where the row stops short they are the wall's boundary,
/// and it clears. Red against W1′ read of the kind (the seam's branch
/// clears).
#[test]
fn a_branch_across_a_closed_channels_seam_is_not_cleared_by_reaching_it() {
    for (closed, want) in [
        (true, Err(Refusal::Undecided)),
        (false, Ok(vec![Cleared::Reaches(Side::G)])),
    ] {
        let s = read(&level(0.5), &tube(closed));
        let built = geom_brep::pair_section_with(
            &level(0.5),
            &tube(closed),
            Some(geom_brep::SsiLane::marched()),
            band(),
        );
        let Ok(geom_brep::SectionRead::Built(built)) = built else {
            panic!("closed {closed}: the section is read: {built:?}")
        };
        assert!(
            matches!(
                built.components(),
                geom_brep::Components::Counted(c) if c.len() == 1 && matches!(
                    c[0].shape,
                    geom_brep::ComponentShape::Open { reaches: [_, r] } if r.u_side && !r.v_side
                )
            ),
            "closed {closed}: one branch from one u side to the other: {built:?}"
        );
        assert_eq!(
            certify(&s, false, band(), |_| Sheet::Not, at(None, None)),
            want,
            "closed {closed}: {s:?}"
        );
    }
}

/// **A plane cutting a wall in one open branch is cleared.** The ramp
/// `z = x` cut at `z = 0.4` meets the plane in the line `x = 0.4` from
/// one side of the rectangle to the other: one component reaching the
/// wall's chart boundary, which clears it whatever its witness places
/// (W1′), on the wall's side in either order. Red against the open
/// branch read as a loop (its witness places nothing: R-undec).
#[test]
fn a_plane_cutting_a_wall_in_one_open_branch_is_cleared() {
    let ramp = net(&|i, _| 0.5 * f64::from(u8::try_from(i).unwrap()));
    for (f, g, side) in [
        (level(0.4), ramp.clone(), Side::G),
        (ramp.clone(), level(0.4), Side::F),
    ] {
        let s = read(&f, &g);
        assert_eq!(lone(&s).reaches, Some(side), "{s:?}");
        assert_eq!(
            certify(&s, false, band(), |_| Sheet::Not, at(IN, IN)),
            Ok(vec![Cleared::Reaches(side)])
        );
    }
}

/// **A plane on a wall's side is cleared.** The plane `z = 0` holds the
/// bottom row of the wall `z = y` rising from it: the boundary pass
/// reports a `Side` region there, the one component, which reaches the
/// wall's chart boundary and clears (W1′). Red against the region
/// listed as a loop (it carries no witness: R-undec).
#[test]
fn a_plane_on_a_walls_side_is_cleared() {
    let rise = net(&|_, j| 0.5 * f64::from(u8::try_from(j).unwrap()));
    let s = read(&level(0.0), &rise);
    let c = lone(&s);
    assert!(
        c.reaches == Some(Side::G) && c.witness.is_none(),
        "a region: {s:?}"
    );
    assert_eq!(
        certify(&s, false, band(), |_| Sheet::Not, at(None, None)),
        Ok(vec![Cleared::Reaches(Side::G)])
    );
}

/// **A graze is refused by name.** The trough `z = (x − ½)² − y/2` cut
/// at `z = 0` meets the plane in `y = 2(x − ½)²`, which touches the
/// rectangle's side `y = 0` at `x = ½` while the surfaces cross: the
/// boundary pass refuses the graze, and the guard reads it as a
/// tangency, named by the row that decided it. Red against the whole
/// reading's refusals all read as R-reach.
#[test]
fn a_graze_is_refused_by_name() {
    let g = [0.25, -0.25, 0.25];
    let trough = net(&|i, j| g[i] - 0.25 * f64::from(u8::try_from(j).unwrap()));
    let s = read(&level(0.0), &trough);
    assert!(
        matches!(s, Section::Tangent("ssi_boundary_crossing")),
        "the graze is a tangency: {s:?}"
    );
}

/// **W0 stays the first door.** A plane clear of the dome's net is apart
/// before any reading, and with no march door a plane that cuts it is
/// R-lane: the scalar does not march.
#[test]
fn w0_is_the_first_door_and_no_lane_has_no_arm() {
    let apart = read(&level(0.5), &dome());
    assert!(
        matches!(&apart, Section::Components { parts, .. } if parts.is_empty()),
        "{apart:?}"
    );
    let unread = classify(
        &level(-0.1),
        &dome(),
        Reach {
            centre: p(0.0, 0.0, 0.0),
            radius: 1.0,
        },
        None,
        band(),
    );
    assert!(matches!(unread, Section::NoLane("f64")), "{unread:?}");
}
