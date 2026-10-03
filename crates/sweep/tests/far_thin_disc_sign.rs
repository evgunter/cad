//! **A far thin disc's sign is read off its exact volume.** A 4-arc disc
//! (one cylindrical wall, so the walk is not all-planar) of radius 1 mm
//! and height `h`, kilometres from the world origin. Its `f64` divergence
//! sum is right to four digits here, but an enclosure of it taken about
//! the world origin is wider than the volume, so an inside-out disc used
//! to pass check 7 and a valid one to escalate in the shell
//! classification, in point containment and in a boolean. Oracle: the
//! disc's volume `π r² h` and its membership, read off the construction.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::{Band, Point2, Point3, Tol};
use sweep::test_support::{brick, cylinder_of_arcs_at};
use topo::{
    Body, ShellRole, SolidContainment, ValidationError, classify_shells, point_in_solid, subtract,
    validate_geometric,
};

/// The discs these rows read: `(h, d)`, every height thin enough that
/// the world-origin enclosure straddled at the distances it is placed at.
const DISCS: [(f64, f64); 6] = [
    (1e-7, 1e3),
    (1e-7, 5e3),
    (1e-7, 2e4),
    (1e-6, 5e3),
    (1e-6, 2e4),
    (1e-5, 2e4),
];

const R: f64 = 1e-3;

/// The disc of height `h` whose axis foot is `d` metres out along a skew
/// direction, and that foot; `None`, standing down loudly, where the
/// fixture's own extrusion refuses — a position whose rounding is not
/// well below ε (`ulp(2e4) ≈ 3.6e-12`, at ε 1e-12) is not a body here.
fn disc(h: f64, d: f64, tol: Tol) -> Option<(Body<f64>, Point3<f64>)> {
    let foot = Point3::new(0.6 * d, 0.48 * d, 0.64 * d);
    let built = std::panic::catch_unwind(|| {
        cylinder_of_arcs_at(4, R, Point2::new(foot.x, foot.y), foot.z, h, tol)
    });
    match built {
        Ok(body) => Some((body, foot)),
        Err(_) => {
            test_utils::vacuity::stood_down(
                "a disc the fixture cannot build",
                "a disc this far from the origin is below this ε's resolution",
            );
            None
        }
    }
}

/// Whether a disc of height `h` is a body at this ε (its walls clear the
/// band), standing down loudly where it is not.
fn buildable(h: f64, tol: Tol) -> bool {
    if h >= 15.0 * tol.eps() {
        return true;
    }
    test_utils::vacuity::stood_down(
        "a disc wall below the band",
        "a disc this thin is not a body at this ε, so its sign is not read here",
    );
    false
}

#[test]
fn far_thin_discs_are_read_by_their_exact_volume() {
    use SolidContainment::{In, Out};
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("a band");
    let mut read = 0;
    for (h, d) in DISCS {
        if !buildable(h, tol) {
            continue;
        }
        let what = format!("r {R:e} × h {h:e} at {d:e} m");
        let Some((upright, foot)) = disc(h, d, tol) else {
            continue;
        };
        read += 1;
        let inside = Point3::new(foot.x, foot.y, foot.z + 0.5 * h);
        let beside = Point3::new(foot.x + 10.0 * R, foot.y, foot.z + 0.5 * h);
        let inverted = upright.revert().expect("the disc reverts");
        let solid = inverted.solids().next().expect("one solid").0;
        for (body, name, check7, role, at) in [
            (&upright, "upright", Ok(()), ShellRole::Outer, [In, Out]),
            (
                &inverted,
                "inside out",
                Err(vec![ValidationError::NegativeVolume { solid }]),
                ShellRole::Void,
                [Out, In],
            ),
        ] {
            assert_eq!(
                validate_geometric(body, tol),
                check7,
                "{what}, {name}: check 7"
            );
            let roles: Vec<ShellRole> = classify_shells(body, tol)
                .unwrap_or_else(|e| panic!("{what}, {name}: the shell classifies: {e}"))
                .iter()
                .map(|c| c.role)
                .collect();
            assert_eq!(roles, vec![role], "{what}, {name}: the shell's role");
            let got = [inside, beside].map(|p| {
                point_in_solid(body, p, band, tol)
                    .unwrap_or_else(|e| panic!("{what}, {name}: containment answers: {e}"))
            });
            assert_eq!(got, at, "{what}, {name}: inside and beside the disc");
        }
    }
    // Every disc builds at ε 1e-9, and the three within 5 km at 1e-12;
    // at 1e-6 none is a body.
    if tol.eps() <= 1e-9 {
        assert!(
            read >= 3,
            "only {read} discs were read at ε {:e}",
            tol.eps()
        );
    }
}

/// A box holding the far thin disc as a cavity: a valid two-shell solid,
/// built by subtracting the disc and passing tier 3.
#[test]
fn a_box_less_a_far_thin_disc_is_a_valid_hollow() {
    let tol = Tol::witness();
    let (h, d) = (1e-6, 5e3);
    if !buildable(h, tol) {
        return;
    }
    let Some((disc, foot)) = disc(h, d, tol) else {
        panic!("the 5 km disc builds wherever its walls clear the band");
    };
    let a = 4.0 * R;
    let boxy = brick::<f64>(
        (foot.x - a, foot.x + a),
        (foot.y - a, foot.y + a),
        (foot.z - a, foot.z + a),
        tol,
    );
    let hollow = subtract(&boxy, &disc, tol).expect("the subtract answers");
    let body = &hollow.body().expect("a body").body;
    let shells: usize = body
        .solids()
        .map(|(s, _)| body.shells_of_solid(s).map_or(0, |v| v.len()))
        .sum();
    assert_eq!(shells, 2, "one solid, its outer shell and the cavity");
    assert_eq!(
        validate_geometric(body, tol),
        Ok(()),
        "the hollow passes tier 3"
    );
}
