//! **A far thin disc's sign is read off its exact volume.** A 4-arc disc
//! (one cylindrical wall, so the walk is not all-planar) of radius 1 mm
//! and height `h`, kilometres from the world origin. Its `f64` divergence
//! sum is right to four digits here, but an enclosure of it taken about
//! the world origin is wider than the volume, so an inside-out disc used
//! to pass check 7 and a valid one to escalate in the shell
//! classification, in point containment and in a boolean. Oracle: the
//! disc's volume `π r² h` and its membership, read off the construction.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::test_support::{brick, cylinder_of_arcs_at};
use topo::{
    Body, ShellRole, SolidContainment, ValidationError, classify_shells, intersect, point_in_solid,
    subtract, transform_rigid, validate_geometric,
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

/// The disc cut from a 1 mm cylinder by a slab `h` thick tilted `tilt`
/// about x, at the origin: its wall is trimmed by two ellipses, so the
/// walk measures it through the certified quadrature, not a closed form.
/// `None`, standing down loudly, where the fixture does not build.
fn tilted_cut(h: f64, tilt: f64, tol: Tol) -> Option<Body<f64>> {
    let built = std::panic::catch_unwind(|| {
        let cylinder = cylinder_of_arcs_at(4, R, Point2::new(0.0, 0.0), -3e-3, 6e-3, tol);
        let slab = brick::<f64>((-4e-3, 4e-3), (-4e-3, 4e-3), (0.0, h), tol);
        let tilt = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), tilt);
        let slab = transform_rigid(&slab, &tilt, tol).expect("the slab tilts");
        intersect(&cylinder, &slab, tol)
            .expect("the cut answers")
            .body()
            .expect("a body")
            .body
            .clone()
    });
    built.map_or_else(
        |_| {
            test_utils::vacuity::stood_down(
                "a tilted cut the fixture cannot build",
                "the cylinder ∩ slab does not build at this ε",
            );
            None
        },
        Some,
    )
}

/// **A far tilted cut disc is read by its exact volume**, its wall
/// through the quadrature lane: the upright disc passes check 7 and
/// reads `Outer`, the inside-out one is refused `NegativeVolume` and
/// reads `Void`. Its volume `π r² h / cos tilt` is some 500× the band's
/// escalation over its area at ε 1e-9, so the sign is far from the band;
/// a quadrature face taken about the world origin used to leave the
/// valid disc refused `VolumeSignUnresolved` 5 km out.
#[test]
fn far_tilted_cut_discs_are_read_by_their_exact_volume() {
    let tol = Tol::witness();
    let mut read = 0;
    for h in [1e-6, 1e-5] {
        if !buildable(h, tol) {
            continue;
        }
        for tilt in [0.3, 0.8] {
            let Some(cut) = tilted_cut(h, tilt, tol) else {
                continue;
            };
            for d in [5e3, 2e4] {
                for angle in [0.0, 0.7] {
                    let what = format!("h {h:e}, tilt {tilt}, {d:e} m, turned {angle}");
                    let turn = Affine3::rotation_about_axis(
                        Point3::origin(),
                        Vec3::new(0.3, 0.5, 0.81),
                        angle,
                    );
                    let out = Affine3::translation(Vec3::new(0.6 * d, 0.48 * d, 0.64 * d));
                    let Ok(upright) = transform_rigid(&cut, &turn, tol)
                        .and_then(|b| transform_rigid(&b, &out, tol))
                    else {
                        test_utils::vacuity::stood_down(
                            "a placement the rigid map refuses",
                            "the cut disc does not place at this ε",
                        );
                        continue;
                    };
                    read += 1;
                    let inverted = upright.revert().expect("the disc reverts");
                    let solid = inverted.solids().next().expect("one solid").0;
                    for (body, name, check7, role) in [
                        (&upright, "upright", Ok(()), ShellRole::Outer),
                        (
                            &inverted,
                            "inside out",
                            Err(vec![ValidationError::NegativeVolume { solid }]),
                            ShellRole::Void,
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
                    }
                }
            }
        }
    }
    // Two of the four cuts build at ε 1e-9 and 1e-12 (the other two run
    // their own quadrature out of budget at the origin); at 1e-9 each
    // places at all four positions, at 1e-12 at half of them; at 1e-6
    // none is a body.
    let floor = match tol.eps() {
        e if e <= 1e-12 => 4,
        e if e <= 1e-9 => 8,
        _ => 0,
    };
    assert!(
        read >= floor,
        "only {read} cut discs were read at ε {:e}",
        tol.eps()
    );
}
