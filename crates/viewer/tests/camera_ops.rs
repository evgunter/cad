//! The camera's claimed invariants, replayed as operation sequences
//! with no renderer anywhere (G1: layer 3 is headless-testable).
//!
//! Each test names an invariant the camera's own docs assert, and the
//! set is chosen so that a change to `apply` that breaks any one of
//! them reds here rather than in somebody's eyes.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use bvh::Aabb;
use viewer::camera::{self, Camera, CameraError, CameraOp, CameraOpError};

use crate::common;
use common::{corners, plate_bounds};

/// The plate framed on a 16:9 pane.
fn framed() -> Camera {
    common::framed(16.0 / 9.0)
}

/// A framing that does not actually contain the scene is the one
/// camera bug a user reads as "the app is broken", so it is asserted
/// through the same projection the renderer uses.
#[test]
fn framing_puts_the_whole_scene_inside_the_frustum() {
    let aspect = 16.0 / 9.0;
    let camera = Camera::framing(&plate_bounds(), aspect).expect("the plate frames");
    for corner in corners(&plate_bounds()) {
        let ndc = camera
            .project(corner, aspect)
            .expect("a finite aspect projects")
            .expect("every corner is in front of the eye");
        assert!(
            ndc[0].abs() <= 1.0 && ndc[1].abs() <= 1.0,
            "corner {corner:?} projects outside the viewport at {ndc:?}"
        );
        assert!(
            (0.0..=1.0).contains(&ndc[2]),
            "corner {corner:?} projects outside the depth range at {ndc:?}"
        );
    }
}

/// **The aspect is load-bearing, two-sidedly.** Containment alone is a
/// one-sided assertion: the fit backs off by the *smaller* half-angle,
/// so a projection that forgot to divide x by the aspect still lands
/// inside `[-1, 1]` at every aspect — both reviewers measured the
/// dropped-`/aspect` mutant surviving the containment rows above, and
/// the credit for catching it belonged to a pan row two files away.
///
/// This row takes the claim where it is made. It pins the projection's
/// own algebra: for a fixed camera, scaling the viewport's aspect by
/// `k` scales the horizontal NDC coordinate by exactly `1/k` and
/// leaves the vertical one alone. A dropped division reds it, and so
/// does a division applied to the wrong axis.
#[test]
fn framing_uses_the_aspect_it_is_given() {
    let camera = framed();
    let probe = pncad::geom_core::Point3::new(0.05, 0.033, 0.006);
    let base = camera
        .project(probe, 1.0)
        .expect("a finite aspect projects")
        .expect("the probe is in front of the eye");
    for k in [0.5f64, 1.25, 3.0] {
        let scaled = camera
            .project(probe, k)
            .expect("a finite aspect projects")
            .expect("the probe is in front of the eye");
        assert!(
            (scaled[0] * k - base[0]).abs() < 1e-12,
            "aspect {k}: x is {} where the projection's own algebra says {}",
            scaled[0],
            base[0] / k
        );
        assert_eq!(
            scaled[1], base[1],
            "aspect {k}: the vertical axis must not depend on the aspect"
        );
    }
    // And the fit is snug, not merely contained: a camera parked at
    // the far end of the band would satisfy containment and show a
    // speck. The binding axis must actually reach out toward the edge.
    for aspect in [0.4f64, 1.0, 2.5] {
        let camera = Camera::framing(&plate_bounds(), aspect).expect("the plate frames");
        let worst = corners(&plate_bounds())
            .into_iter()
            .map(|corner| {
                let ndc = camera
                    .project(corner, aspect)
                    .expect("a finite aspect projects")
                    .expect("every corner is in front of the eye");
                ndc[0].abs().max(ndc[1].abs())
            })
            .fold(0.0f64, f64::max);
        assert!(
            (0.4..=1.0).contains(&worst),
            "aspect {aspect}: the framed plate fills {worst} of the frustum — \
             a fit that loose is not a fit"
        );
    }
}

/// A viewport too narrow to fit the scene refuses, and says what it
/// needed — it does not clamp into the zoom band and return a camera
/// that silently fails its own containment postcondition.
#[test]
fn a_viewport_too_narrow_to_fit_refuses_rather_than_clamping() {
    let mut refusals = 0;
    for aspect in [0.02f64, 0.005, 0.001] {
        match Camera::framing(&plate_bounds(), aspect) {
            Err(CameraError::Unfittable {
                required,
                max_distance,
                aspect: reported,
            }) => {
                refusals += 1;
                assert!(
                    required > max_distance,
                    "the refusal's own numbers disagree"
                );
                assert_eq!(reported, aspect);
            }
            Err(other) => panic!("aspect {aspect}: unexpected refusal {other:?}"),
            Ok(camera) => {
                // Fitting is the other acceptable answer, but only if
                // it really fits.
                for corner in corners(&plate_bounds()) {
                    let ndc = camera
                        .project(corner, aspect)
                        .expect("projects")
                        .expect("in front of the eye");
                    assert!(
                        ndc[0].abs() <= 1.0 && ndc[1].abs() <= 1.0,
                        "aspect {aspect}: Ok with {corner:?} at {ndc:?} — neither fit nor refused"
                    );
                }
            }
        }
    }
    assert!(
        refusals > 0,
        "no aspect in the sweep reached the band's ceiling — the row is measuring nothing"
    );
}

/// Framing keeps the orientation it was given: a fit is not a reset.
#[test]
fn framing_an_already_framed_camera_keeps_its_orientation() {
    let camera = framed();
    let turned = camera::apply(
        &camera,
        &CameraOp::Orbit {
            yaw: 0.7,
            pitch: -0.2,
        },
    )
    .expect("a finite orbit");
    let refitted = camera::apply(
        &turned,
        &CameraOp::Frame {
            bounds: plate_bounds(),
            aspect: 16.0 / 9.0,
        },
    )
    .expect("the plate re-frames");
    assert_eq!(refitted.yaw(), turned.yaw());
    assert_eq!(refitted.pitch(), turned.pitch());
}

/// Orbit is a group action on the azimuth: two orbits compose into
/// one, up to the half-open wrap.
#[test]
fn orbit_composes_and_wraps() {
    let camera = framed();
    let once = camera::fold(
        &camera,
        &[
            CameraOp::Orbit {
                yaw: 1.1,
                pitch: 0.0,
            },
            CameraOp::Orbit {
                yaw: 2.3,
                pitch: 0.0,
            },
        ],
    )
    .expect("finite orbits");
    let together = camera::apply(
        &camera,
        &CameraOp::Orbit {
            yaw: 3.4,
            pitch: 0.0,
        },
    )
    .expect("a finite orbit");
    assert!(
        (once.yaw() - together.yaw()).abs() < 1e-12,
        "orbit composition: {} vs {}",
        once.yaw(),
        together.yaw()
    );
    // Twenty turns leave the angle in the canonical interval rather
    // than out where a float's resolution has collapsed.
    let spun = camera::fold(
        &camera,
        &std::iter::repeat_n(
            CameraOp::Orbit {
                yaw: std::f64::consts::TAU,
                pitch: 0.0,
            },
            20,
        )
        .collect::<Vec<_>>(),
    )
    .expect("finite orbits");
    assert!(
        spun.yaw() >= -std::f64::consts::PI && spun.yaw() < std::f64::consts::PI,
        "yaw escaped the half-open interval: {}",
        spun.yaw()
    );
}

/// Orbiting past the pole saturates instead of flipping the world
/// upside down — the reason the pitch is clamped at all.
#[test]
fn orbit_saturates_at_the_poles() {
    let camera = framed();
    for direction in [1.0f64, -1.0] {
        let far = camera::apply(
            &camera,
            &CameraOp::Orbit {
                yaw: 0.0,
                pitch: direction * 100.0,
            },
        )
        .expect("a finite orbit");
        // Read from the camera, not restated: a literal here is a
        // hand-synced copy of a private constant.
        let limit = Camera::pitch_limit();
        assert!(
            (far.pitch().abs() - limit).abs() < 1e-12,
            "pitch did not saturate: {}",
            far.pitch()
        );
        assert!(
            far.up().z > 0.0,
            "the camera's up vector fell past the pole: {:?}",
            far.up()
        );
        // A second push does not move it further.
        let further = camera::apply(
            &far,
            &CameraOp::Orbit {
                yaw: 0.0,
                pitch: direction * 100.0,
            },
        )
        .expect("a finite orbit");
        assert_eq!(further.pitch(), far.pitch());
    }
}

/// Orbit turns the eye and nothing else.
#[test]
fn orbit_moves_neither_the_target_nor_the_distance() {
    let camera = framed();
    let turned = camera::apply(
        &camera,
        &CameraOp::Orbit {
            yaw: 0.9,
            pitch: 0.3,
        },
    )
    .expect("a finite orbit");
    assert_eq!(turned.target().x, camera.target().x);
    assert_eq!(turned.target().y, camera.target().y);
    assert_eq!(turned.target().z, camera.target().z);
    assert_eq!(turned.distance(), camera.distance());
}

/// Pan slides the target inside the view plane: the component along
/// the view direction is zero, and the orientation is untouched.
#[test]
fn pan_moves_the_target_only_in_the_view_plane() {
    let camera = framed();
    let panned = camera::apply(
        &camera,
        &CameraOp::Pan {
            right: 0.013,
            up: -0.007,
        },
    )
    .expect("a finite pan");
    let delta = [
        panned.target().x - camera.target().x,
        panned.target().y - camera.target().y,
        panned.target().z - camera.target().z,
    ];
    let f = camera.forward();
    let along = delta[0] * f.x + delta[1] * f.y + delta[2] * f.z;
    assert!(
        along.abs() < 1e-15,
        "pan moved the target along the view direction by {along}"
    );
    let r = camera.right();
    let u = camera.up();
    assert!((delta[0] * r.x + delta[1] * r.y + delta[2] * r.z - 0.013).abs() < 1e-15);
    assert!((delta[0] * u.x + delta[1] * u.y + delta[2] * u.z + 0.007).abs() < 1e-15);
    assert_eq!(panned.yaw(), camera.yaw());
    assert_eq!(panned.pitch(), camera.pitch());
    assert_eq!(panned.distance(), camera.distance());
}

/// Inside the zoom band a dolly and its inverse are the identity, so
/// a wheel roll up and back leaves the view where it was.
#[test]
fn dolly_is_reversible_inside_the_band() {
    let camera = framed();
    let there = camera::apply(&camera, &CameraOp::Dolly { factor: 0.5 }).expect("a positive dolly");
    assert!(there.distance() > there.min_distance());
    let back = camera::apply(&there, &CameraOp::Dolly { factor: 2.0 }).expect("a positive dolly");
    assert!(
        (back.distance() - camera.distance()).abs() < 1e-15 * camera.distance(),
        "dolly was not reversible: {} vs {}",
        back.distance(),
        camera.distance()
    );
}

/// The band is a real clamp at both ends, and the target never moves.
#[test]
fn dolly_clamps_to_the_scene_derived_band() {
    let camera = framed();
    let close = camera::fold(
        &camera,
        &std::iter::repeat_n(CameraOp::Dolly { factor: 0.5 }, 40).collect::<Vec<_>>(),
    )
    .expect("positive dollies");
    assert_eq!(close.distance(), close.min_distance());
    let far = camera::fold(
        &camera,
        &std::iter::repeat_n(CameraOp::Dolly { factor: 2.0 }, 40).collect::<Vec<_>>(),
    )
    .expect("positive dollies");
    assert_eq!(far.distance(), far.max_distance());
    assert_eq!(close.target().x, camera.target().x);
    assert_eq!(far.target().x, camera.target().x);
}

/// The depth range stays ordered and positive everywhere in the band,
/// which is what keeps the projection matrix finite.
#[test]
fn the_depth_range_is_ordered_across_the_whole_zoom_band() {
    let camera = framed();
    let mut current = camera;
    for _ in 0..40 {
        assert!(
            current.near() > 0.0 && current.near() < current.far(),
            "depth range at distance {}: near {} far {}",
            current.distance(),
            current.near(),
            current.far()
        );
        current =
            camera::apply(&current, &CameraOp::Dolly { factor: 0.5 }).expect("a positive dolly");
    }
    let mut current = camera;
    for _ in 0..40 {
        assert!(current.near() > 0.0 && current.near() < current.far());
        current =
            camera::apply(&current, &CameraOp::Dolly { factor: 2.0 }).expect("a positive dolly");
    }
}

/// The camera frame is orthonormal and right-handed at every
/// reachable orientation — the property the view matrix rests on.
#[test]
fn the_camera_frame_stays_orthonormal() {
    let camera = framed();
    let mut current = camera;
    for step in 0..64 {
        let (r, u, f) = (current.right(), current.up(), current.forward());
        for (name, v) in [("right", r), ("up", u), ("forward", f)] {
            let len = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
            assert!(
                (len - 1.0).abs() < 1e-12,
                "{name} is not unit at step {step}: {len}"
            );
        }
        assert!((r.x * u.x + r.y * u.y + r.z * u.z).abs() < 1e-12);
        assert!((r.x * f.x + r.y * f.y + r.z * f.z).abs() < 1e-12);
        assert!((u.x * f.x + u.y * f.y + u.z * f.z).abs() < 1e-12);
        // right × up = −forward for a right-handed view frame looking
        // down −z.
        let cross = [
            r.y * u.z - r.z * u.y,
            r.z * u.x - r.x * u.z,
            r.x * u.y - r.y * u.x,
        ];
        assert!((cross[0] + f.x).abs() < 1e-12);
        assert!((cross[1] + f.y).abs() < 1e-12);
        assert!((cross[2] + f.z).abs() < 1e-12);
        current = camera::apply(
            &current,
            &CameraOp::Orbit {
                yaw: 0.31,
                pitch: 0.11,
            },
        )
        .expect("a finite orbit");
    }
}

/// An operation that is not a move is refused typed, and the caller
/// keeps the camera it had.
#[test]
fn operations_that_are_not_moves_are_refused_typed() {
    let camera = framed();
    // The payload's value is NaN, which is not equal to itself — so
    // the arm and the field name are what is asserted here.
    assert!(matches!(
        camera::apply(
            &camera,
            &CameraOp::Orbit {
                yaw: f64::NAN,
                pitch: 0.0
            }
        ),
        Err(CameraOpError::NotFinite { what: "yaw", value }) if value.is_nan()
    ));
    assert!(matches!(
        camera::apply(
            &camera,
            &CameraOp::Pan {
                right: f64::INFINITY,
                up: 0.0
            }
        ),
        Err(CameraOpError::NotFinite { what: "right", .. })
    ));
    assert!(matches!(
        camera::apply(&camera, &CameraOp::Dolly { factor: 0.0 }),
        Err(CameraOpError::NonPositiveDolly { factor: 0.0 })
    ));
    assert!(matches!(
        camera::apply(&camera, &CameraOp::Dolly { factor: -1.0 }),
        Err(CameraOpError::NonPositiveDolly { .. })
    ));
    // A degenerate box has no framing: refused, never defaulted to a
    // made-up scale.
    let point = Aabb {
        min_x: 1.0,
        min_y: 1.0,
        min_z: 1.0,
        max_x: 1.0,
        max_y: 1.0,
        max_z: 1.0,
    };
    assert!(matches!(
        camera::apply(
            &camera,
            &CameraOp::Frame {
                bounds: point,
                aspect: 1.0
            }
        ),
        Err(CameraOpError::Unframeable(_))
    ));
}

/// A refused operation in the middle of a fold stops the fold: the
/// caller learns which operation failed rather than getting a camera
/// that silently skipped one.
#[test]
fn a_fold_stops_at_the_first_refusal() {
    let camera = framed();
    let result = camera::fold(
        &camera,
        &[
            CameraOp::Orbit {
                yaw: 0.2,
                pitch: 0.0,
            },
            CameraOp::Dolly { factor: -1.0 },
            CameraOp::Orbit {
                yaw: 0.2,
                pitch: 0.0,
            },
        ],
    );
    assert!(matches!(
        result,
        Err(CameraOpError::NonPositiveDolly { .. })
    ));
}

/// A viewport with no area has no projection, and says so.
#[test]
fn a_viewport_with_no_area_has_no_projection() {
    let camera = framed();
    assert!(camera.projection_matrix(0.0).is_err());
    assert!(camera.projection_matrix(f64::NAN).is_err());
    assert!(camera.projection_matrix(-1.0).is_err());
}
