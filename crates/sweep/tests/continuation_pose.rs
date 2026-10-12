//! **Whether two faces on one carrier continue each other is a fact
//! about the pair, not its pose.**
//!
//! A rod cut by the slab `z ∈ [0, 1]` is a plug whose top and bottom lie
//! on the slab's own planes, sense for sense. Each second operand below
//! puts a face on each of those planes, so the operation decides every
//! top-and-bottom pair one carrier, undeclared, and must read from the
//! geometry whether the two faces meet. The whole scene is turned
//! together through [`poses`]; every pose gets the same verdict at its
//! closed form: every op that builds, in both orders, builds a valid
//! body of the volume the closed forms give, with the same face count
//! and the same number of face-pair rows as the unturned scene, and an
//! op that refuses refuses the same way at every pose.
//!
//! - **apart**: the rod tilted `θ` toward +x, so the plug's top and
//!   bottom are ellipse-bounded, beside a brick a gap past the plug's
//!   reach. The world boxes of the ellipse edges and the brick's edges
//!   overlap once the scene is turned and not unturned, so a reading
//!   that takes their overlap for a meeting flips with the pose. Nothing
//!   meets, so no face pair is recorded at any pose. Whether these
//!   results are legal operands of a next boolean is not asserted: the
//!   at-infinity probe refuses an ellipse-trimmed wall
//!   (`restread/at-infinity-probe-measures-in-closed-form-only`).
//! - **plugged**: the upright plug back in the slab it was cut from.
//!   The union is the slab again, its top and bottom continuing across
//!   the two rim circles and the hole's wall resting on the plug's; the
//!   differences give each operand back; the intersection is empty.
//! - **tilted plug**: the plug tilted `θ` back in its hole, its rims
//!   ellipses, the hole's wall resting on the plug's across them. The
//!   intersections are empty and the differences give each operand
//!   back; the union refuses
//!   `GermEdgeCarrierUnsupported` at every pose
//!   (`cleave/a-tilted-plug-union-refuses-at-the-germ-segment-and-section-copy-rosters`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::test_support::{brick, prism_on, sketch_from_axes};
use topo::test_support::finished;
use topo::{AtRestBody, Body, BooleanError, BooleanResult, Cell, RowCell};

/// The rod's radius.
const R: f64 = 0.25;
/// The apart scene's tilt off the z axis, toward +x: `cos θ = ½`.
const TILT: f64 = PI / 3.0;
/// The slab: `x ∈ [−3, 3]`, `y ∈ [−1, 1]`, `z ∈ [0, 1]`.
const SLAB: [(f64, f64); 3] = [(-3.0, 3.0), (-1.0, 1.0), (0.0, 1.0)];
const SLAB_VOLUME: f64 = 12.0;

/// The rigid poses the scene is turned through, together: the identity
/// and three turns about each world axis and about a skew one, all
/// through the origin.
fn poses() -> Vec<(String, Affine3<f64>)> {
    let axes = [
        ("x", Vec3::new(1.0, 0.0, 0.0)),
        ("y", Vec3::new(0.0, 1.0, 0.0)),
        ("z", Vec3::new(0.0, 0.0, 1.0)),
        ("(1,2,3)", Vec3::new(1.0, 2.0, 3.0).normalize()),
    ];
    let mut out = vec![("identity".to_owned(), Affine3::identity())];
    for (name, axis) in axes {
        for angle in [0.3, PI / 4.0, 1.2] {
            out.push((
                format!("{angle:.4} about {name}"),
                Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), axis, angle),
            ));
        }
    }
    out
}

/// The rod tilted `tilt` toward +x: a disc of radius [`R`] extruded 4
/// along `(sin tilt, 0, cos tilt)`, centred on `(0, 0, ½)`, so both its
/// caps lie outside the slab.
fn rod(tilt: f64, tol: Tol) -> Body<f64> {
    let (s, c) = tilt.sin_cos();
    let axis = Vec3::new(s, 0.0, c);
    let plane = sketch_from_axes(
        Point3::new(0.0, 0.0, 0.5) - axis * 2.0,
        Vec3::new(c, 0.0, -s),
        Vec3::new(0.0, 1.0, 0.0),
        tol,
    );
    let disc = vec![(Point2::new(R, 0.0), 1.0), (Point2::new(-R, 0.0), 1.0)];
    prism_on(plane, disc, 4.0, tol)
}

/// The plug's closed form: the slab cuts the rod in ellipses of area
/// `πr² / cos tilt`, a unit apart.
fn plug_volume(tilt: f64) -> f64 {
    PI * R * R / tilt.cos()
}

/// The plug (`slab ∩ rod`) and the holed slab (`slab ∖ rod`).
fn plug_and_hole(tilt: f64, tol: Tol) -> (AtRestBody<f64>, AtRestBody<f64>) {
    let slab = finished("the slab", brick(SLAB[0], SLAB[1], SLAB[2], tol), tol);
    let rod = finished("the rod", rod(tilt, tol), tol);
    let body = |what: &str, out| match out {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!("{what}: {other:?}"),
    };
    (
        body("slab ∩ rod", topo::intersect(&slab, &rod, tol)),
        body("slab ∖ rod", topo::subtract(&slab, &rod, tol)),
    )
}

fn posed(what: &str, body: &Body<f64>, map: &Affine3<f64>, tol: Tol) -> AtRestBody<f64> {
    let b = topo::transform_rigid(
        body,
        &geom_core::RigidMotion::check_rigid(map, geom_core::Band::linear(tol).unwrap()).unwrap(),
        tol,
    )
    .unwrap_or_else(|e| panic!("{what}: {e:?}"));
    finished(what, b, tol)
}

/// What one op reads, compared across poses.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Reading {
    Body {
        faces: usize,
        face_pair_rows: usize,
    },
    Empty,
    /// The germ join's typed refusal of an edge carrier it cannot pair.
    GermEdgeRefused,
}

/// What one op is held to.
#[derive(Debug, Clone, Copy)]
enum Want {
    /// A body of this volume.
    Builds(f64),
    /// No body: the operands share no volume.
    Empty,
    /// The germ join's typed refusal.
    GermEdgeRefused,
}

/// The rows naming a face of each operand.
fn face_pair_rows(rows: &[topo::Coincidence]) -> usize {
    rows.iter()
        .filter(|row| {
            matches!(
                row.cells,
                [
                    RowCell::Input {
                        cell: Cell::Face(_),
                        ..
                    },
                    RowCell::Input {
                        cell: Cell::Face(_),
                        ..
                    }
                ]
            )
        })
        .count()
}

/// The five ops on `a` and `b`, each held to every validation tier and
/// to its closed form (the intersections empty), and, where `legal`, to
/// being a legal operand of the next boolean.
fn every_op(
    label: &str,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    (union, [a_minus_b, b_minus_a]): (Want, [f64; 2]),
    legal: bool,
) -> Vec<Reading> {
    let tol = Tol::witness();
    [
        ("∪", topo::union(a, b, tol), union),
        ("a ∩ b", topo::intersect(a, b, tol), Want::Empty),
        ("b ∩ a", topo::intersect(b, a, tol), Want::Empty),
        ("a ∖ b", topo::subtract(a, b, tol), Want::Builds(a_minus_b)),
        ("b ∖ a", topo::subtract(b, a, tol), Want::Builds(b_minus_a)),
    ]
    .into_iter()
    .map(|(op, out, want)| {
        let label = format!("{label}, {op}");
        let truth = match want {
            Want::Builds(volume) => volume,
            Want::GermEdgeRefused => {
                assert!(
                    matches!(out, Err(BooleanError::GermEdgeCarrierUnsupported { .. })),
                    "{label}: the germ join refuses the edge carrier: {out:?}"
                );
                return Reading::GermEdgeRefused;
            }
            Want::Empty => {
                assert!(
                    matches!(out, Ok(BooleanResult::Empty)),
                    "{label}: the operands share no volume, so the op is empty: {out:?}"
                );
                return Reading::Empty;
            }
        };
        let out = out.unwrap_or_else(|e| panic!("{label}: refused {e:?}"));
        let BooleanResult::Body(out) = out else {
            panic!("{label}: came back empty");
        };
        assert_eq!(topo::validate(&out.body), Ok(()), "{label}: validate");
        assert_eq!(topo::validate_closed(&out.body), Ok(()), "{label}: closed");
        assert_eq!(
            topo::validate_geometric(&out.body, tol),
            Ok(()),
            "{label}: tier 3"
        );
        if legal {
            sweep::test_support::assert_legal_operand(&label, &out.body, tol);
        }
        let m = topo::mass_properties(&out.body, tol)
            .unwrap_or_else(|e| panic!("{label}: mass properties {e:?}"));
        assert!(
            (m.volume - truth).abs() <= m.volume_pad + 1e-9 * truth,
            "{label}: volume {} ± {} against the closed form {truth}",
            m.volume,
            m.volume_pad
        );
        Reading::Body {
            faces: out.body.faces().count(),
            face_pair_rows: face_pair_rows(&out.coincidences),
        }
    })
    .collect()
}

/// `scene` at every pose, each pose's readings asserted equal to the
/// identity's; returns the identity's.
fn alike_at_every_pose(
    scene: &str,
    run: impl Fn(&str, &Affine3<f64>) -> Vec<Reading>,
) -> Vec<Reading> {
    let eps = Tol::witness().eps();
    let mut poses = poses().into_iter();
    let (_, identity) = poses.next().expect("the identity leads");
    let reference = run(&format!("{scene}, ε {eps:e}, identity"), &identity);
    for (pose, map) in poses {
        assert_eq!(
            run(&format!("{scene}, ε {eps:e}, {pose}"), &map),
            reference,
            "{scene}, ε {eps:e}: {pose} reads differently from the identity"
        );
    }
    reference
}

/// **apart**: the tilted plug and a unit brick a tenth past its top's
/// reach along x (the top ellipse ends at `x = tan θ / 2 + r / cos θ`),
/// their tops on `z = 1` and bottoms on `z = 0`.
#[test]
fn a_plug_clear_of_a_brick_on_its_planes_reads_alike_at_every_pose() {
    let tol = Tol::witness();
    let (plug, _) = plug_and_hole(TILT, tol);
    let x0 = 0.5 * TILT.tan() + R / TILT.cos() + 0.1;
    let beside = brick::<f64>((x0, x0 + 1.0), (-0.5, 0.5), (0.0, 1.0), tol);
    let (v_a, v_b) = (plug_volume(TILT), 1.0);
    let reference = alike_at_every_pose("apart", |label, map| {
        every_op(
            label,
            &posed("the plug", &plug, map, tol),
            &posed("the brick", &beside, map, tol),
            (Want::Builds(v_a + v_b), [v_a, v_b]),
            false,
        )
    });
    for reading in reference {
        if let Reading::Body { face_pair_rows, .. } = reading {
            assert_eq!(
                face_pair_rows, 0,
                "apart: no face of the plug meets one of the brick's"
            );
        }
    }
}

/// **plugged**: the upright plug in its hole. The union is the slab's
/// six faces, with the rows of the face pairs that meet.
#[test]
fn a_plug_in_its_hole_reads_alike_at_every_pose() {
    let tol = Tol::witness();
    let (plug, hole) = plug_and_hole(0.0, tol);
    let (v_a, v_b) = (plug_volume(0.0), SLAB_VOLUME - plug_volume(0.0));
    let reference = alike_at_every_pose("plugged", |label, map| {
        every_op(
            label,
            &posed("the plug", &plug, map, tol),
            &posed("the holed slab", &hole, map, tol),
            (Want::Builds(SLAB_VOLUME), [v_a, v_b]),
            true,
        )
    });
    assert_eq!(
        reference[0],
        Reading::Body {
            faces: 6,
            face_pair_rows: 6
        },
        "plugged: the union is the slab, and its top, bottom and wall pairs are recorded"
    );
}

/// **tilted plug**: the plug tilted `0.3` rad and [`TILT`] back in its
/// hole. The intersections are empty, each difference gives its operand
/// back with its own faces and the six face-pair rows the plugged union
/// records, and the union refuses `GermEdgeCarrierUnsupported`.
#[test]
fn a_tilted_plug_in_its_hole_reads_alike_at_every_pose() {
    let tol = Tol::witness();
    for tilt in [0.3, TILT] {
        let (plug, hole) = plug_and_hole(tilt, tol);
        let (v_a, v_b) = (plug_volume(tilt), SLAB_VOLUME - plug_volume(tilt));
        let reference = alike_at_every_pose(&format!("tilted {tilt:.4}"), |label, map| {
            every_op(
                label,
                &posed("the plug", &plug, map, tol),
                &posed("the holed slab", &hole, map, tol),
                (Want::GermEdgeRefused, [v_a, v_b]),
                false,
            )
        });
        assert_eq!(
            reference[3..],
            [
                Reading::Body {
                    faces: 4,
                    face_pair_rows: 6
                },
                Reading::Body {
                    faces: 8,
                    face_pair_rows: 6
                },
            ],
            "tilted {tilt}: each difference is its operand's faces, with the face pairs \
             the plugged union records"
        );
    }
}
