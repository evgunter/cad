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

/// The section through the march door, over a reach the spline arm
/// does not read.
fn read(f: &Surface<f64>, g: &Surface<f64>) -> Section<f64> {
    classify_with(
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

/// The one component of a lone section.
fn lone(s: &Section<f64>) -> Component<f64> {
    match s {
        Section::Components { parts, single } if parts.len() == 1 => {
            assert!(*single, "a lone component is certified single: {s:?}");
            parts[0]
        }
        other => panic!("not one component: {other:?}"),
    }
}

/// **A loop inside a plane and a wall is never cleared, and its witness
/// is placed on the band widened by its residual.** The dome cut at
/// `z = −0.1` meets the plane in the circle of radius `√0.1` about the
/// centre, inside the wall's rectangle: one closed component, reaching
/// no chart boundary, with a witness `residual` off it. With no event
/// its witness decides it: `In` both faces is R-loop, no verdict is
/// R-undec, and it clears only `Out` of a face (W3); with an event it is
/// the lone evented component (W4). Against a face whose edge stands `d`
/// from the witness, past the band but inside the band widened by the
/// residual, the placement reads the witness on the edge, so it does
/// not clear. Red against the boundary clause read true for a closed
/// loop (every verdict clears as `Reaches`), against the whole reading
/// dropped (R-reach), and against the residual dropped anywhere between
/// the certificate and the placement (the component's, the rule's
/// hand-off, or the widening), where the face would place the witness
/// `Out` and clear a loop the section may still hold inside it. One
/// reading serves every assertion: a loop's accounting is the
/// expensive one.
#[test]
fn a_loop_inside_a_plane_and_a_wall_is_never_cleared() {
    let s = read(&level(-0.1), &dome());
    let c = lone(&s);
    assert!(c.reaches.is_none(), "a loop reaches no side: {c:?}");
    assert!(c.residual > 0.0, "a settled witness: {c:?}");
    let w = c.witness.expect("a loop carries a witness");
    let r = ((w.x - 0.5).powi(2) + (w.y - 0.5).powi(2)).sqrt();
    assert!(
        (r - 0.1_f64.sqrt()).abs() < 1e-6 && (w.z + 0.1).abs() < 1e-6,
        "the witness lies on the circle: {w:?}"
    );
    let rule = |evented, place| certify(&s, evented, |_| Sheet::Not, place);
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
    let d = band().zero() + c.residual / 2.0;
    let edge = |_, r| {
        let wide = witness_band(band(), r).expect("a band that wide exists");
        [if d > wide.zero() { OUT } else { None }, IN]
    };
    assert_eq!(
        certify(&s, false, |_| Sheet::Not, edge),
        Err(Refusal::Undecided),
        "a witness inside its residual of an edge does not clear"
    );
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
            certify(&s, false, |_| Sheet::Not, at(IN, IN)),
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
        certify(&s, false, |_| Sheet::Not, at(None, None)),
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
/// before any reading, and with no march door a plane that cuts it has
/// no arm.
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
        band(),
    );
    assert!(matches!(unread, Section::Intractable), "{unread:?}");
}

/// r1 probe: the dome loop's residual against the witness's true
/// distance from the circle, and the same loop read through a plane
/// whose origin stands a kilometre off.
#[test]
fn r1_probe_residual_and_far_origin() {
    let t = std::time::Instant::now();
    let s = read(&level(-0.1), &dome());
    let c = lone(&s);
    let w = c.witness.unwrap();
    let r = ((w.x - 0.5).powi(2) + (w.y - 0.5).powi(2)).sqrt();
    let dist = ((r - 0.1_f64.sqrt()).powi(2) + (w.z + 0.1).powi(2)).sqrt();
    eprintln!(
        "R1 near: residual {:e}, true distance {dist:e}, band zero {:e}, {:.2?}",
        c.residual,
        band().zero(),
        t.elapsed()
    );
    assert!(dist <= c.residual, "residual is an upper bound");
    let t = std::time::Instant::now();
    let far = plane(p(1000.0, 1000.0, -0.1), v(0.0, 0.0, 1.0));
    let s = read(&far, &dome());
    eprintln!("R1 far: {s:?} in {:.2?}", t.elapsed());
    let c = lone(&s);
    assert!(c.reaches.is_none(), "far origin: still a loop: {s:?}");
}

/// r1 probe: a loop standing `d` inside all four sides of the wall
/// (the dome cut just above its rim's lowest points) is a loop, never a
/// component that reaches the boundary.
#[test]
fn r1_probe_a_loop_hugging_the_sides_is_not_open() {
    for d in [1e-3, 1e-5, 1e-7] {
        let t = std::time::Instant::now();
        let h = -(0.5 - d) * (0.5 - d);
        let s = read(&level(h), &dome());
        eprintln!("R1 hug d={d:e}: {s:?} in {:.2?}", t.elapsed());
        if let Section::Components { parts, .. } = &s {
            assert!(
                parts.iter().all(|c| c.reaches.is_none()),
                "d={d:e}: a closed loop {d} inside the sides read as reaching: {s:?}"
            );
        }
    }
}

/// r1 probe: the far-origin plane's refusal, by name.
#[test]
fn r1_probe_far_origin_cause() {
    let far = plane(p(1000.0, 1000.0, -0.1), v(0.0, 0.0, 1.0));
    let near = plane(p(0.5, 0.5, -0.1), v(0.0, 0.0, 1.0));
    for (what, pl) in [("near", &near), ("far", &far)] {
        let got =
            geom_brep::pair_section_with(pl, &dome(), Some(geom_brep::SsiLane::marched()), band());
        let shown = match &got {
            Ok(geom_brep::SectionRead::Built(s)) => {
                format!("Built({} components)", s.components().len())
            }
            Ok(geom_brep::SectionRead::Elsewhere(_)) => "Elsewhere".to_string(),
            Err(e) => format!("Err({e})"),
        };
        eprintln!("R1 cause {what}: {shown}");
    }
}

/// r1 counterexample (class e): the witness's residual should scale
/// with ε. Measured: 0.2165 m at ε 1e-6, 1e-9 and 1e-12, while the
/// witness's true distance from the circle is 1.3e-9, 9.3e-13 and
/// 1.2e-15. Red at every ε: the residual is the tube's rung (the
/// ladder's widest), a label the chart tube does not bound in metres.
#[test]
fn r1_residual_scales_with_eps() {
    let s = read(&level(-0.1), &dome());
    let c = lone(&s);
    assert!(
        c.residual <= 1e3 * band().zero(),
        "residual {:e} against band zero {:e}",
        c.residual,
        band().zero()
    );
}
