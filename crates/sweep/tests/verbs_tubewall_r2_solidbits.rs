//! **VERBS-TUBEWALL review probe (r2): the SOLID door's bits.**
//!
//! A cross-revision instrument, deliberately written against the
//! solid door ONLY so that the same file compiles at the merge base:
//! the harness runs it at the PR head, reverts
//! `crates/sweep/src/revolve/tube.rs` to the base, runs it again, and
//! diffs the two dumps. Rust's `{:?}` for `f64` is round-trip exact,
//! so the printed text IS the body's bits.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol, Vec3};
use sweep::{TubeWindow, tube_along_arc};

/// Prints one bit-faithful line per solid-door configuration, tagged
/// `R2SOLIDBITS`, for the harness to extract from `--nocapture`
/// output.
#[test]
fn r2_solid_door_bits_dump() {
    /// Major radius, minor radius, and the window as `(t0, t1)` or
    /// `None` for a full period.
    type Case = (f64, f64, Option<(f64, f64)>);
    let cases: [Case; 6] = [
        (2.0, 0.5, None),
        (2.0, 0.5, Some((0.25, 1.75))),
        (2.0, 0.3, None),
        (5.0, 1.25, Some((3.0, 6.0))),
        (10.0, 0.2, Some((0.1, 6.2))),
        (1.0, 0.9, Some((0.0, 3.0))),
    ];
    for (major, minor, arc) in cases {
        let window = match arc {
            None => TubeWindow::Full,
            Some((t0, t1)) => TubeWindow::Arc { t0, t1 },
        };
        let t = tube_along_arc::<f64>(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::unit_y(),
            Vec3::unit_x(),
            major,
            window,
            minor,
            Tol::witness(),
        )
        .expect("the solid tube builds");
        let props = topo::props::mass_properties(&t.body, Tol::witness()).expect("mass properties");
        println!(
            "R2SOLIDBITS\t{major:?}\t{minor:?}\t{arc:?}\tV={:?}\tA={:?}\tcav={}\t{:?}",
            props.volume,
            props.surface_area,
            t.cavities.len(),
            t.body
        );
    }
    // The five refusal doors, with their rendered messages: the solid
    // door's verdict sequence is part of what must not have moved.
    let bad = [
        (Vec3::unit_y() * 1.5, Vec3::unit_x(), 2.0, 0.5, None),
        (Vec3::unit_y(), Vec3::unit_y(), 2.0, 0.5, None),
        (
            Vec3::unit_y(),
            Vec3::unit_x(),
            2.0,
            0.5,
            Some((1.0_f64, 1.0_f64)),
        ),
        (
            Vec3::unit_y(),
            Vec3::unit_x(),
            2.0,
            0.5,
            Some((0.0, core::f64::consts::TAU)),
        ),
        (Vec3::unit_y(), Vec3::unit_x(), 0.4, 0.5, None),
    ];
    for (axis, u_ref, major, minor, arc) in bad {
        let window = match arc {
            None => TubeWindow::Full,
            Some((t0, t1)) => TubeWindow::Arc { t0, t1 },
        };
        let e = tube_along_arc::<f64>(
            Point3::new(0.0, 0.0, 0.0),
            axis,
            u_ref,
            major,
            window,
            minor,
            Tol::witness(),
        )
        .expect_err("refuses");
        println!("R2SOLIDBITS\tREFUSAL\t{e:?}\t{e}");
    }
}
