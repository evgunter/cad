//! Reviewer R2's consumer suite for GUI-0 (PR #1094) — an independent
//! derivation of what the PR claims, driven through `viewer`'s public
//! surface exactly as an outside consumer would call it.
//!
//! Every randomized sweep here follows `memories/test-suite-cost.md`:
//! counterexample searches draw a fresh seed per run through
//! `test_utils::fuzz` (logged unconditionally, `CAD_FUZZ_SEED` replays,
//! counts ride `CAD_FUZZ_EFFORT`). Nothing in this file is a
//! print-only probe; every row asserts.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use bvh::{Aabb, Axis};
use pncad::geom_core::{Point3, Tol, Vec3};
use test_utils::fuzz;
use viewer::camera::{self, Camera, CameraOp, CameraOpError};
use viewer::input::{InputMap, PointerButton, ViewportEvent, ViewportSize};
use viewer::scene::{self, DisplayTolerance};

/// The pole clamp the camera documents (`|pitch| < π/2` strictly, held
/// away from the pole by a fixed margin). Re-derived from the public
/// accessors below rather than imported — the suite is a consumer.
const PITCH_LIMIT: f64 = std::f64::consts::FRAC_PI_2;

fn len3(v: Vec3<f64>) -> f64 {
    (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
}

fn dot3(a: Vec3<f64>, b: Vec3<f64>) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

/// A random non-degenerate box: centre within ±10³, extents spanning
/// six decades, always finite and non-inverted.
fn random_box(rng: &mut fuzz::Rng) -> Aabb {
    let cx = rng.range(-1.0e3, 1.0e3);
    let cy = rng.range(-1.0e3, 1.0e3);
    let cz = rng.range(-1.0e3, 1.0e3);
    let hx = 10f64.powf(rng.range(-3.0, 3.0));
    let hy = 10f64.powf(rng.range(-3.0, 3.0));
    let hz = 10f64.powf(rng.range(-3.0, 3.0));
    Aabb {
        min_x: cx - hx,
        min_y: cy - hy,
        min_z: cz - hz,
        max_x: cx + hx,
        max_y: cy + hy,
        max_z: cz + hz,
    }
}

/// A random valid operation — every draw is a genuine move, so a
/// refusal anywhere in the sweep is a finding, not noise.
fn random_op(rng: &mut fuzz::Rng) -> CameraOp {
    match rng.below(4) {
        0 => CameraOp::Orbit {
            yaw: rng.range(-8.0, 8.0),
            pitch: rng.range(-4.0, 4.0),
        },
        1 => CameraOp::Pan {
            right: rng.range(-100.0, 100.0),
            up: rng.range(-100.0, 100.0),
        },
        2 => CameraOp::Dolly {
            factor: 10f64.powf(rng.range(-1.5, 1.5)),
        },
        _ => CameraOp::Frame {
            bounds: random_box(rng),
            // Wide but sane band; the pathological thin-sliver aspect
            // is probed (and reported) separately in the review notes.
            aspect: rng.range(0.2, 5.0),
        },
    }
}

fn assert_camera_invariants(c: &Camera, context: &str) {
    for (name, v) in [
        ("target.x", c.target().x),
        ("target.y", c.target().y),
        ("target.z", c.target().z),
        ("distance", c.distance()),
        ("yaw", c.yaw()),
        ("pitch", c.pitch()),
        ("fov_y", c.fov_y()),
        ("scene_radius", c.scene_radius()),
    ] {
        assert!(
            v.is_finite(),
            "{name} not finite after {context}: {v} ({})",
            fuzz::replay()
        );
    }
    assert!(
        c.distance() >= c.min_distance() && c.distance() <= c.max_distance(),
        "distance {} escaped the band [{}, {}] after {context} ({})",
        c.distance(),
        c.min_distance(),
        c.max_distance(),
        fuzz::replay()
    );
    assert!(
        c.pitch().abs() < PITCH_LIMIT,
        "pitch {} reached the pole after {context} ({})",
        c.pitch(),
        fuzz::replay()
    );
    assert!(
        c.yaw() >= -std::f64::consts::PI && c.yaw() < std::f64::consts::PI,
        "yaw {} escaped [-pi, pi) after {context} ({})",
        c.yaw(),
        fuzz::replay()
    );
    let (r, u, f) = (c.right(), c.up(), c.forward());
    for (name, v) in [("right", r), ("up", u), ("forward", f)] {
        assert!(
            (len3(v) - 1.0).abs() < 1e-9,
            "{name} not unit after {context}: {} ({})",
            len3(v),
            fuzz::replay()
        );
    }
    assert!(
        dot3(r, u).abs() < 1e-9,
        "right.up after {context} ({})",
        fuzz::replay()
    );
    assert!(
        dot3(r, f).abs() < 1e-9,
        "right.forward after {context} ({})",
        fuzz::replay()
    );
    assert!(
        dot3(u, f).abs() < 1e-9,
        "up.forward after {context} ({})",
        fuzz::replay()
    );
    assert!(
        c.near() > 0.0 && c.near() < c.far(),
        "depth range disordered after {context}: near {} far {} ({})",
        c.near(),
        c.far(),
        fuzz::replay()
    );
}

/// The camera's documented invariants hold at every state reachable by
/// valid operations — not only along the shipped tests' hand-picked
/// paths — and `fold` is exactly sequential `apply`.
#[test]
fn random_valid_op_sequences_hold_every_camera_invariant() {
    let mut rng = fuzz::start("gui0-r2 camera invariant sweep");
    for seq in 0..fuzz::scaled(24) {
        let start = Camera::framing(&random_box(&mut rng), rng.range(0.2, 5.0))
            .expect("a non-degenerate random box frames");
        assert_camera_invariants(&start, "framing");
        let ops: Vec<CameraOp> = (0..fuzz::scaled(16)).map(|_| random_op(&mut rng)).collect();
        let mut stepped = start;
        for (i, op) in ops.iter().enumerate() {
            stepped = camera::apply(&stepped, op).unwrap_or_else(|e| {
                panic!(
                    "valid op {i} of seq {seq} refused: {e:?} ({})",
                    fuzz::replay()
                )
            });
            assert_camera_invariants(&stepped, &format!("op {i} of seq {seq}"));
        }
        let folded = camera::fold(&start, &ops).expect("the same ops fold");
        assert_eq!(
            folded,
            stepped,
            "fold disagrees with sequential apply on seq {seq} ({})",
            fuzz::replay()
        );
    }
}

/// The pan property, independently derived and taken at RANDOM camera
/// states, viewport sizes and drag vectors, on BOTH screen axes: a drag
/// of (dx, dy) pixels moves the world point that was at the cursor by
/// exactly (dx, dy) pixels. This is the row that catches a lost
/// fov / viewport-height / distance factor wherever the camera happens
/// to be, not just at the framing state.
#[test]
fn pan_keeps_the_point_under_the_cursor_at_random_states() {
    let mut rng = fuzz::start("gui0-r2 pan property sweep");
    let map = InputMap::default();
    for case in 0..fuzz::scaled(32) {
        let mut cam = Camera::framing(&random_box(&mut rng), 1.0).expect("frames");
        // Wander to an arbitrary state first (orbit + dolly only, so
        // the target stays where the box put it).
        for _ in 0..4 {
            let op = CameraOp::Orbit {
                yaw: rng.range(-3.0, 3.0),
                pitch: rng.range(-1.2, 1.2),
            };
            cam = camera::apply(&cam, &op).expect("orbit");
            let op = CameraOp::Dolly {
                factor: 10f64.powf(rng.range(-0.8, 0.8)),
            };
            cam = camera::apply(&cam, &op).expect("dolly");
        }
        let size = ViewportSize {
            width_px: rng.range(64.0, 4000.0),
            height_px: rng.range(64.0, 4000.0),
        };
        let aspect = size.aspect().expect("area");
        let (dx, dy) = (rng.range(-500.0, 500.0), rng.range(-500.0, 500.0));
        if dx == 0.0 && dy == 0.0 {
            continue;
        }
        let op = map
            .map(
                &ViewportEvent::Drag {
                    button: PointerButton::Secondary,
                    shift: false,
                    alt: false,
                    delta_px: [dx, dy],
                },
                size,
                &cam,
            )
            .expect("the pan button is bound");
        let panned = camera::apply(&cam, &op).expect("a finite pan");
        let before = cam
            .project(cam.target(), aspect)
            .expect("finite aspect")
            .expect("target in front of the eye");
        let after = panned
            .project(cam.target(), aspect)
            .expect("finite aspect")
            .expect("target still in front of the eye");
        let moved_x = (after[0] - before[0]) * 0.5 * size.width_px;
        // NDC +y is up; screen +y is down.
        let moved_y = -(after[1] - before[1]) * 0.5 * size.height_px;
        let tol = 1e-6 * (dx.abs() + dy.abs() + 1.0);
        assert!(
            (moved_x - dx).abs() < tol && (moved_y - dy).abs() < tol,
            "case {case}: drag ({dx}, {dy}) px moved the world point ({moved_x}, {moved_y}) px ({})",
            fuzz::replay()
        );
    }
}

/// Framing is a real fit for arbitrary (sane-aspect) boxes: every
/// corner of a RANDOM box lands inside the frustum through the same
/// `project` the renderer uses, and a re-fit after an arbitrary orbit
/// still fits.
#[test]
fn framing_fits_random_boxes_through_the_projection() {
    let mut rng = fuzz::start("gui0-r2 framing sweep");
    for case in 0..fuzz::scaled(24) {
        let b = random_box(&mut rng);
        let aspect = rng.range(0.2, 5.0);
        let cam = Camera::framing(&b, aspect).expect("a random box frames");
        let orbited = camera::apply(
            &cam,
            &CameraOp::Orbit {
                yaw: rng.range(-3.0, 3.0),
                pitch: rng.range(-1.2, 1.2),
            },
        )
        .expect("orbit");
        let refit = camera::apply(&orbited, &CameraOp::Frame { bounds: b, aspect })
            .expect("the same box re-frames");
        for (which, c) in [("framing", cam), ("refit", refit)] {
            for x in [b.min_x, b.max_x] {
                for y in [b.min_y, b.max_y] {
                    for z in [b.min_z, b.max_z] {
                        let ndc = c
                            .project(Point3::new(x, y, z), aspect)
                            .expect("finite aspect")
                            .unwrap_or_else(|| {
                                panic!(
                                    "case {case} {which}: corner behind the eye ({})",
                                    fuzz::replay()
                                )
                            });
                        assert!(
                            ndc[0].abs() <= 1.0 + 1e-9
                                && ndc[1].abs() <= 1.0 + 1e-9
                                && (-1e-9..=1.0 + 1e-9).contains(&ndc[2]),
                            "case {case} {which}: corner ({x}, {y}, {z}) at {ndc:?} ({})",
                            fuzz::replay()
                        );
                    }
                }
            }
        }
    }
}

/// `map_stream` is exactly "map each event, fold the camera through
/// the ops it keeps": the returned camera equals `fold` of the
/// returned ops from the start camera, and no stream ever yields more
/// ops than events.
#[test]
fn map_stream_agrees_with_folding_its_own_output() {
    let mut rng = fuzz::start("gui0-r2 stream consistency sweep");
    let map = InputMap::default();
    let buttons = [
        PointerButton::Primary,
        PointerButton::Secondary,
        PointerButton::Middle,
    ];
    for case in 0..fuzz::scaled(24) {
        let start = Camera::framing(&random_box(&mut rng), 1.5).expect("frames");
        let size = ViewportSize {
            width_px: rng.range(64.0, 3000.0),
            height_px: rng.range(64.0, 3000.0),
        };
        let events: Vec<ViewportEvent> = (0..fuzz::scaled(12))
            .map(|_| {
                if rng.below(4) == 0 {
                    ViewportEvent::Scroll {
                        units: rng.range(-4.0, 4.0),
                    }
                } else {
                    ViewportEvent::Drag {
                        button: buttons[rng.below(3)],
                        shift: rng.below(2) == 1,
                        // Random alt too: the property is about the
                        // fold agreeing with its own output, whatever
                        // the events bound to.
                        alt: rng.below(2) == 1,
                        // Includes exact zeros sometimes, which must
                        // bind to nothing.
                        delta_px: [
                            if rng.below(5) == 0 {
                                0.0
                            } else {
                                rng.range(-60.0, 60.0)
                            },
                            if rng.below(5) == 0 {
                                0.0
                            } else {
                                rng.range(-60.0, 60.0)
                            },
                        ],
                    }
                }
            })
            .collect();
        let (end, ops) =
            viewer::input::map_stream(&map, &start, size, &events).expect("finite events");
        assert!(
            ops.len() <= events.len(),
            "case {case}: more ops than events ({})",
            fuzz::replay()
        );
        let refolded = camera::fold(&start, &ops).expect("the emitted ops re-fold");
        assert_eq!(
            refolded,
            end,
            "case {case}: map_stream's camera disagrees with folding its ops ({})",
            fuzz::replay()
        );
    }
}

/// Refusals are typed, first-failure, and total: a non-finite delta
/// planted at a random position in an otherwise valid sequence stops
/// the fold with the right arm, and `apply` on the same camera still
/// succeeds for the valid prefix (nothing was mutated in place).
#[test]
fn a_planted_refusal_stops_the_fold_with_the_right_arm() {
    let mut rng = fuzz::start("gui0-r2 refusal sweep");
    for _ in 0..fuzz::scaled(16) {
        let start = Camera::framing(&random_box(&mut rng), 1.0).expect("frames");
        let mut ops: Vec<CameraOp> = (0..fuzz::scaled(8)).map(|_| random_op(&mut rng)).collect();
        let bad_at = rng.below(ops.len());
        let bad = [
            CameraOp::Orbit {
                yaw: f64::NAN,
                pitch: 0.0,
            },
            CameraOp::Pan {
                right: f64::INFINITY,
                up: 0.0,
            },
            CameraOp::Dolly { factor: -2.0 },
            CameraOp::Dolly { factor: f64::NAN },
        ][rng.below(4)];
        ops[bad_at] = bad;
        let err = camera::fold(&start, &ops).expect_err("the planted op must refuse");
        match (bad, err) {
            (CameraOp::Orbit { .. }, CameraOpError::NotFinite { what, .. }) => {
                assert_eq!(what, "yaw", "{}", fuzz::replay());
            }
            (CameraOp::Pan { .. }, CameraOpError::NotFinite { what, .. }) => {
                assert_eq!(what, "right", "{}", fuzz::replay());
            }
            (CameraOp::Dolly { factor }, CameraOpError::NonPositiveDolly { factor: got }) => {
                assert_eq!(factor, got, "{}", fuzz::replay());
            }
            (CameraOp::Dolly { factor }, CameraOpError::NotFinite { what, value }) => {
                assert!(
                    factor.is_nan() && what == "factor" && value.is_nan(),
                    "{}",
                    fuzz::replay()
                );
            }
            (planted, got) => {
                panic!(
                    "planted {planted:?}, got mismatched arm {got:?} ({})",
                    fuzz::replay()
                )
            }
        }
        // The valid prefix still folds — the refusal consumed nothing.
        let prefix = &ops[..bad_at];
        let _ = camera::fold(&start, prefix).expect("the valid prefix folds");
    }
}

/// The public-API scene path, with the winding/volume claim re-derived
/// from first principles at a RANDOM display tolerance: the enclosed
/// volume (divergence theorem over the triangle soup) must sit in
/// [nominal, nominal + bound(δ)], where bound(δ) is the inscribed-
/// polygon area deficit of the hole — (2/3)·perimeter·δ — times the
/// plate thickness, with a 3× safety factor and an f32 rounding
/// allowance. Monotone in δ, so it reds if δ stops being the lever or
/// the winding flips anywhere.
#[test]
fn the_enclosed_volume_error_is_bounded_by_delta_at_random_deltas() {
    let mut rng = fuzz::start("gui0-r2 volume bound sweep");
    let tol = Tol::witness();
    let (doc, _root) = scene::plate_with_hole(tol).expect("the plate authors");
    let plate = [0.060, 0.040, 0.008];
    let hole_r = 0.012;
    let nominal =
        plate[0] * plate[1] * plate[2] - std::f64::consts::PI * hole_r * hole_r * plate[2];
    for _ in 0..fuzz::scaled(3) {
        let delta_value = 10f64.powf(rng.range(-5.0, -3.0));
        let delta = DisplayTolerance::new(delta_value).expect("a positive tolerance");
        let mesh = scene::scene_of(&doc, delta, tol).expect("the plate tessellates");
        let mut total = 0.0f64;
        for t in mesh.positions().chunks_exact(3) {
            let p: Vec<[f64; 3]> = t
                .iter()
                .map(|v| [f64::from(v[0]), f64::from(v[1]), f64::from(v[2])])
                .collect();
            let (a, b, c) = (p[0], p[1], p[2]);
            total += a[0] * (b[1] * c[2] - b[2] * c[1])
                + a[1] * (b[2] * c[0] - b[0] * c[2])
                + a[2] * (b[0] * c[1] - b[1] * c[0]);
        }
        let enclosed = total / 6.0;
        let bound =
            3.0 * (2.0 / 3.0) * (2.0 * std::f64::consts::PI * hole_r) * delta_value * plate[2];
        // f32 vertex rounding: relative 2^-24 on ~0.06 m coordinates,
        // integrated over ~10^3..10^4 triangles — 1e-9 m^3 dwarfs it.
        let slack = 1.0e-9;
        assert!(
            enclosed >= nominal - slack,
            "delta {delta_value}: enclosed {enclosed} below nominal {nominal} — winding or \
             tessellation lost material ({})",
            fuzz::replay()
        );
        assert!(
            enclosed <= nominal + bound + slack,
            "delta {delta_value}: enclosed {enclosed} exceeds nominal {nominal} by more than \
             the chordal bound {bound} ({})",
            fuzz::replay()
        );
        // The join, at a random aspect: a camera framed on these bounds
        // holds every vertex the scene will draw.
        let aspect = rng.range(0.3, 4.0);
        let cam = Camera::framing(&mesh.bounds(), aspect).expect("the scene frames");
        for v in mesh.positions() {
            let ndc = cam
                .project(
                    Point3::new(f64::from(v[0]), f64::from(v[1]), f64::from(v[2])),
                    aspect,
                )
                .expect("finite aspect")
                .expect("vertex in front of the eye");
            assert!(
                ndc[0].abs() <= 1.0 && ndc[1].abs() <= 1.0 && (0.0..=1.0).contains(&ndc[2]),
                "vertex outside the frustum at aspect {aspect} ({})",
                fuzz::replay()
            );
        }
    }
}

/// The scene bounds measure the plate the document authored — checked
/// against the AUTHORED dimensions axis by axis, so a wrong body (or a
/// hand-built mesh swapped in) cannot satisfy it by accident.
#[test]
fn the_scene_bounds_are_the_documents_own_dimensions() {
    let tol = Tol::witness();
    let (doc, _root) = scene::plate_with_hole(tol).expect("the plate authors");
    let mesh = scene::scene_of(
        &doc,
        DisplayTolerance::new(1.0e-4).expect("a positive tolerance"),
        tol,
    )
    .expect("the plate tessellates");
    let b = mesh.bounds();
    for (axis, lo, hi) in [
        (Axis::X, 0.0, 0.060),
        (Axis::Y, 0.0, 0.040),
        (Axis::Z, 0.0, 0.008),
    ] {
        assert!(
            (b.min(axis) - lo).abs() < 1e-12 && (b.max(axis) - hi).abs() < 1e-12,
            "bounds on {axis:?}: [{}, {}] vs authored [{lo}, {hi}]",
            b.min(axis),
            b.max(axis)
        );
    }
}
