//! GUI-0 reviewer suite (R1) — an INDEPENDENT derivation of what the
//! unit claims, written against the public surface rather than against
//! the diff.
//!
//! The shipped suites in `camera_ops.rs`, `input_mapping.rs` and
//! `scene_build.rs` are good and this file deliberately does not
//! restate them. What it adds is the set of properties a review
//! derived from the *contracts* — `Camera`'s "invariants hold at every
//! reachable state", `mesh::FacePatch`'s outward-winding contract, and
//! `projection_matrix`'s claim to be a perspective projection with the
//! stated field of view AND aspect — and then checked could actually
//! go red.
//!
//! Three of these rows exist because a mutation survived the shipped
//! suite:
//!
//! - `the_projection_carries_the_aspect_ratio`: deleting `/ aspect`
//!   from `projection_matrix` leaves BOTH shipped framing rows green
//!   (measured at the frozen head: the plate is small enough in x that
//!   a 1.78× horizontal stretch still lands inside the frustum). This
//!   row derives the projection from the field of view instead of
//!   sampling one fixture, and reds on that mutation.
//! - `the_drawn_surface_is_closed_and_consistently_oriented`: the
//!   signed-volume row states a property of a SUM, and a sum can be
//!   right while the surface is not — a small flipped patch, a dropped
//!   triangle, a seam that does not close. (Measured honestly: flipping
//!   the spike's first patch reds the shipped volume row too, because
//!   that patch is a large one. The point is that edge pairing does not
//!   depend on which patch or how many triangles.) Edge pairing is the
//!   surface-level statement no single number can stand in for.
//! - `a_fold_returns_the_FIRST_refusal`: the shipped fold row asserts
//!   only that *a* `NonPositiveDolly` comes back, which a fold that
//!   kept going and returned the last error would also satisfy. Two
//!   different refusal arms in both orders pin the "first".
//!
//! # Randomised rows, per `memories/test-suite-cost.md`
//!
//! Two rows are counterexample searches (*for all sampled x, P(x)*), so
//! the seed VARIES per run and is logged unconditionally. Replay a red
//! run with `GUI0_R1_SEED=<the printed value>`; buy depth with
//! `GUI0_R1_EFFORT=<n>` (counts are multiples of it). The generator is
//! a five-line SplitMix64 inlined here rather than a new dev-dependency
//! on `viewer`, whose manifest this suite deliberately does not touch.
//!
//! # One reporting row
//!
//! `framing_at_an_extreme_aspect_should_contain_or_refuse` is
//! `#[ignore]`d: it encodes the contract `Camera::fitted` documents and
//! is RED at the frozen head (review finding: at aspect ≲ 0.03 the
//! zoom-band clamp silently wins and the framed camera does not contain
//! the scene). Un-ignore it when that is either fixed or the contract
//! is narrowed in prose.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use std::collections::HashMap;

use bvh::{Aabb, Axis};
use pncad::geom_core::{Point3, Tol};
use viewer::camera::{self, Camera, CameraError, CameraOp, CameraOpError};
use viewer::input::{InputMap, PointerButton, ViewportEvent, ViewportSize};
use viewer::scene::{self, DisplayTolerance};

// ---------------------------------------------------------------- rng

/// SplitMix64. A counterexample search wants a fresh draw per run, and
/// this suite refuses to grow `viewer` a dev-dependency to get one.
struct Rng(u64);

impl Rng {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform in `[lo, hi)`.
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        let unit = (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64;
        lo + unit * (hi - lo)
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }
}

/// A fresh seed unless `GUI0_R1_SEED` names one; printed either way, so
/// a red run in a CI log is reproducible.
fn seed(label: &str) -> u64 {
    let s = match std::env::var("GUI0_R1_SEED") {
        Ok(text) => text.parse().expect("GUI0_R1_SEED must be a u64"),
        Err(_) => std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("a clock after 1970")
            .as_nanos() as u64,
    };
    println!("{label}: GUI0_R1_SEED={s}");
    s
}

/// Counts are multiples of this. Ships at the level a gated run should
/// cost; depth is one env var away.
fn effort() -> usize {
    std::env::var("GUI0_R1_EFFORT")
        .ok()
        .and_then(|t| t.parse().ok())
        .unwrap_or(1usize)
        .max(1)
}

// ------------------------------------------------------------ fixtures

/// The spike's plate, stated here from its own declared literals
/// rather than read back out of `scene`.
const PLATE: [f64; 3] = [0.060, 0.040, 0.008];
const HOLE_CENTRE: [f64; 2] = [0.030, 0.020];
const HOLE_RADIUS: f64 = 0.012;

fn plate_bounds() -> Aabb {
    Aabb {
        min_x: 0.0,
        min_y: 0.0,
        min_z: 0.0,
        max_x: PLATE[0],
        max_y: PLATE[1],
        max_z: PLATE[2],
    }
}

fn corners(b: &Aabb) -> Vec<Point3<f64>> {
    let mut out = Vec::new();
    for x in [b.min(Axis::X), b.max(Axis::X)] {
        for y in [b.min(Axis::Y), b.max(Axis::Y)] {
            for z in [b.min(Axis::Z), b.max(Axis::Z)] {
                out.push(Point3::new(x, y, z));
            }
        }
    }
    out
}

fn built(delta: f64) -> viewer::SceneMesh {
    let tol = Tol::witness();
    let (doc, _root) = scene::plate_with_hole(tol).expect("the plate authors");
    let d = DisplayTolerance::new(delta).expect("a positive display tolerance");
    scene::scene_of(&doc, d, tol).expect("the plate tessellates")
}

/// Every claim `Camera`'s module docs make about a reachable state,
/// checked on one camera. Labelled per assertion so a merged row still
/// names the property that broke (`memories/test-suite-cost.md`).
fn assert_camera_contract(camera: &Camera, provenance: &str) {
    let limit = std::f64::consts::FRAC_PI_2;
    for (name, value) in [
        ("target.x", camera.target().x),
        ("target.y", camera.target().y),
        ("target.z", camera.target().z),
        ("distance", camera.distance()),
        ("yaw", camera.yaw()),
        ("pitch", camera.pitch()),
        ("fov_y", camera.fov_y()),
        ("scene_radius", camera.scene_radius()),
    ] {
        assert!(
            value.is_finite(),
            "[{provenance}] {name} is not finite: {value}"
        );
    }
    assert!(
        camera.distance() >= camera.min_distance() && camera.distance() <= camera.max_distance(),
        "[{provenance}] distance {} escaped the band {}..{}",
        camera.distance(),
        camera.min_distance(),
        camera.max_distance()
    );
    assert!(
        camera.pitch().abs() < limit,
        "[{provenance}] pitch reached the pole: {}",
        camera.pitch()
    );
    assert!(
        camera.yaw() >= -std::f64::consts::PI && camera.yaw() < std::f64::consts::PI,
        "[{provenance}] yaw escaped [-pi, pi): {}",
        camera.yaw()
    );
    assert!(
        camera.near() > 0.0 && camera.near() < camera.far(),
        "[{provenance}] depth range not ordered: near {} far {}",
        camera.near(),
        camera.far()
    );
    let (r, u, f) = (camera.right(), camera.up(), camera.forward());
    for (name, v) in [("right", r), ("up", u), ("forward", f)] {
        let len = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
        assert!(
            (len - 1.0).abs() < 1e-12,
            "[{provenance}] {name} is not unit: {len}"
        );
    }
    assert!(
        u.z > 0.0,
        "[{provenance}] the up vector fell past the pole: {u:?}"
    );
    assert!(
        (r.x * u.x + r.y * u.y + r.z * u.z).abs() < 1e-12,
        "[{provenance}] right and up are not orthogonal"
    );
}

// ------------------------------------------------------------ the rows

/// COUNTEREXAMPLE SEARCH. Whatever sequence of operations a user's
/// pointer can produce, the camera that comes out still satisfies every
/// claim its own module docs make — and an operation that is refused
/// leaves the caller holding a camera that still does.
///
/// The shipped suite asserts these invariants one at a time on chosen
/// fixtures; this row asserts all of them at once, everywhere a random
/// walk through the whole vocabulary can reach.
#[test]
fn the_camera_contract_survives_random_operation_walks() {
    let mut rng = Rng(seed("the_camera_contract_survives_random_operation_walks"));
    let walks = 48 * effort();
    let steps = 16;
    let mut refusals = 0usize;

    for walk in 0..walks {
        let aspect = rng.range(0.2, 5.0);
        let mut camera = Camera::framing(&plate_bounds(), aspect).expect("the plate frames");
        assert_camera_contract(&camera, &format!("walk {walk} step 0"));
        for step in 0..steps {
            // A quarter of the draws are deliberately not moves.
            let op = match rng.below(8) {
                0 => CameraOp::Orbit {
                    yaw: if rng.below(2) == 0 {
                        f64::NAN
                    } else {
                        rng.range(-9.0, 9.0)
                    },
                    pitch: rng.range(-9.0, 9.0),
                },
                1 => CameraOp::Dolly {
                    factor: rng.range(-2.0, 0.0),
                },
                2 | 3 => CameraOp::Orbit {
                    yaw: rng.range(-9.0, 9.0),
                    pitch: rng.range(-9.0, 9.0),
                },
                4 => CameraOp::Pan {
                    right: rng.range(-0.2, 0.2),
                    up: rng.range(-0.2, 0.2),
                },
                5 | 6 => CameraOp::Dolly {
                    factor: rng.range(0.05, 20.0),
                },
                _ => CameraOp::Frame {
                    bounds: plate_bounds(),
                    aspect: rng.range(0.2, 5.0),
                },
            };
            match camera::apply(&camera, &op) {
                Ok(next) => {
                    camera = next;
                    assert_camera_contract(
                        &camera,
                        &format!("walk {walk} step {step} after {op:?}"),
                    );
                }
                Err(error) => {
                    refusals += 1;
                    // A refusal is a value, and the camera is untouched.
                    assert_camera_contract(
                        &camera,
                        &format!("walk {walk} step {step} after refusal {error:?}"),
                    );
                }
            }
            // The projection exists and is finite at every reachable state.
            let m = camera
                .view_projection(aspect)
                .expect("a positive aspect projects");
            for col in &m {
                for value in col {
                    assert!(
                        value.is_finite(),
                        "walk {walk} step {step}: view_projection carried {value}"
                    );
                }
            }
        }
    }
    // Anti-vacuity: the refusal arm of the walk must actually have been
    // reached, or this row is only testing the happy path.
    assert!(
        refusals > walks / 4,
        "the walk barely produced refusals ({refusals} in {} steps) — the generator has drifted",
        walks * steps
    );
}

/// COUNTEREXAMPLE SEARCH. `projection_matrix` claims to be a
/// perspective projection with the camera's field of view AND the given
/// aspect. Derived from the field of view rather than sampled: a point
/// offset from the target by `dr` along `right` and `du` along `up`
/// must land at `ndc = (dr / (d·tan(fov/2)·aspect), du / (d·tan(fov/2)))`.
///
/// This is the row that reds when the `/ aspect` term goes missing —
/// a mutation both shipped framing rows survive.
#[test]
fn the_projection_carries_the_field_of_view_and_the_aspect() {
    let mut rng = Rng(seed(
        "the_projection_carries_the_field_of_view_and_the_aspect",
    ));
    let cases = 96 * effort();
    for case in 0..cases {
        let aspect = rng.range(0.25, 4.0);
        let camera = camera::apply(
            &Camera::framing(&plate_bounds(), aspect).expect("the plate frames"),
            &CameraOp::Orbit {
                yaw: rng.range(-3.0, 3.0),
                pitch: rng.range(-1.2, 1.2),
            },
        )
        .expect("a finite orbit");

        let d = camera.distance();
        let half_height = d * (camera.fov_y() * 0.5).tan();
        // Stay well inside the frustum so the assertion is about the
        // projection and not about clipping.
        let dr = rng.range(-0.6, 0.6) * half_height * aspect;
        let du = rng.range(-0.6, 0.6) * half_height;

        let (r, u, t) = (camera.right(), camera.up(), camera.target());
        let point = Point3::new(
            t.x + r.x * dr + u.x * du,
            t.y + r.y * dr + u.y * du,
            t.z + r.z * dr + u.z * du,
        );
        let ndc = camera
            .project(point, aspect)
            .expect("a positive aspect projects")
            .expect("a point in the target plane is in front of the eye");

        let want_x = dr / (half_height * aspect);
        let want_y = du / half_height;
        assert!(
            (ndc[0] - want_x).abs() < 1e-9,
            "case {case}: ndc.x {} against {want_x} (aspect {aspect}) — the aspect term is wrong",
            ndc[0]
        );
        assert!(
            (ndc[1] - want_y).abs() < 1e-9,
            "case {case}: ndc.y {} against {want_y} — the field-of-view term is wrong",
            ndc[1]
        );
    }
}

/// COUNTEREXAMPLE SEARCH. The pan binding's whole claim is that the
/// point under the cursor stays under the cursor. The shipped row pins
/// it once, on one horizontal 137 px drag; this one pins BOTH axes over
/// random drags, viewport shapes, camera orientations and zoom levels.
#[test]
fn a_pan_moves_the_cursor_point_by_the_dragged_distance_on_both_axes() {
    let mut rng = Rng(seed(
        "a_pan_moves_the_cursor_point_by_the_dragged_distance_on_both_axes",
    ));
    let cases = 64 * effort();
    let map = InputMap::default();
    for case in 0..cases {
        let size = ViewportSize {
            width_px: rng.range(64.0, 3840.0),
            height_px: rng.range(64.0, 2160.0),
        };
        let aspect = size.aspect().expect("a viewport with area");
        let camera = camera::fold(
            &Camera::framing(&plate_bounds(), aspect).expect("the plate frames"),
            &[
                CameraOp::Orbit {
                    yaw: rng.range(-3.0, 3.0),
                    pitch: rng.range(-1.2, 1.2),
                },
                CameraOp::Dolly {
                    factor: rng.range(0.2, 5.0),
                },
            ],
        )
        .expect("finite operations");

        let dx = rng.range(-400.0, 400.0);
        let dy = rng.range(-400.0, 400.0);
        let event = ViewportEvent::Drag {
            button: PointerButton::Secondary,
            shift: false,
            alt: false,
            delta_px: [dx, dy],
        };
        let Some(op) = map.map(&event, size, &camera) else {
            panic!("case {case}: the pan button produced no operation for {event:?}");
        };
        let panned = camera::apply(&camera, &op).expect("a finite pan");

        let before = camera
            .project(camera.target(), aspect)
            .expect("projects")
            .expect("in front of the eye");
        let after = panned
            .project(camera.target(), aspect)
            .expect("projects")
            .expect("in front of the eye");
        let moved_x = (after[0] - before[0]) * 0.5 * size.width_px;
        // NDC y is up, screen y is down.
        let moved_y = -(after[1] - before[1]) * 0.5 * size.height_px;
        assert!(
            (moved_x - dx).abs() < 1e-6 * dx.abs().max(1.0),
            "case {case}: a {dx} px horizontal drag moved the point {moved_x} px"
        );
        assert!(
            (moved_y - dy).abs() < 1e-6 * dy.abs().max(1.0),
            "case {case}: a {dy} px vertical drag moved the point {moved_y} px"
        );
    }
}

/// `mesh::FacePatch`'s contract is that the triangles bound the solid
/// with a consistent outward orientation. The signed-volume row proves
/// the SUM is right, which a partial flip can survive; this proves the
/// SURFACE is right — every undirected edge is used exactly twice, once
/// in each direction, which is what "closed and consistently oriented"
/// actually means and which no single number can fake.
///
/// Corner positions are `f32` copies of entries in one shared `f64`
/// table, so shared vertices are bit-identical and exact keying is
/// sound.
#[test]
fn the_drawn_surface_is_closed_and_consistently_oriented() {
    for delta in [1.0e-3, 1.0e-4, 1.0e-5] {
        let mesh = built(delta);
        let key = |p: [f32; 3]| [p[0].to_bits(), p[1].to_bits(), p[2].to_bits()];
        let mut balance: HashMap<([u32; 3], [u32; 3]), i64> = HashMap::new();
        for triangle in mesh.positions().chunks_exact(3) {
            let (a, b, c) = (key(triangle[0]), key(triangle[1]), key(triangle[2]));
            assert!(
                a != b && b != c && c != a,
                "delta {delta}: a triangle has a repeated corner"
            );
            for (from, to) in [(a, b), (b, c), (c, a)] {
                if from <= to {
                    *balance.entry((from, to)).or_default() += 1;
                } else {
                    *balance.entry((to, from)).or_default() -= 1;
                }
            }
        }
        let broken: Vec<_> = balance.iter().filter(|(_, v)| **v != 0).collect();
        assert!(
            broken.is_empty(),
            "delta {delta}: {} of {} edges are not paired head-to-tail — the surface is open \
             or an inconsistently wound triangle is in it",
            broken.len(),
            balance.len()
        );
        assert!(
            balance.len() > 3 * mesh.stats().triangles / 4,
            "delta {delta}: implausibly few distinct edges ({}) for {} triangles",
            balance.len(),
            mesh.stats().triangles
        );
    }
}

/// The scene really is the document's plate with the document's hole:
/// no drawn vertex sits inside the hole's cylinder by more than the
/// chordal sag δ allows, and the hole is genuinely present (vertices
/// land on its wall). Derived from the literals the document declares,
/// not from what the tessellator returned.
#[test]
fn the_drawn_scene_is_the_declared_plate_with_the_declared_hole() {
    for delta in [1.0e-3, 1.0e-4, 1.0e-5] {
        let mesh = built(delta);
        let mut on_the_wall = 0usize;
        for p in mesh.positions() {
            let (x, y, z) = (f64::from(p[0]), f64::from(p[1]), f64::from(p[2]));
            assert!(
                (-1e-9..=PLATE[0] + 1e-9).contains(&x) && (-1e-9..=PLATE[1] + 1e-9).contains(&y),
                "delta {delta}: a vertex escaped the plate outline at ({x}, {y})"
            );
            assert!(
                (-1e-9..=PLATE[2] + 1e-9).contains(&z),
                "delta {delta}: a vertex escaped the extrusion at z = {z}"
            );
            let radial = ((x - HOLE_CENTRE[0]).powi(2) + (y - HOLE_CENTRE[1]).powi(2)).sqrt();
            // An INSCRIBED polygon never reaches inside the true radius
            // by more than the sag the tolerance buys.
            assert!(
                radial >= HOLE_RADIUS - delta - 1e-9 || radial > HOLE_RADIUS,
                "delta {delta}: a vertex at radius {radial} is inside the {HOLE_RADIUS} hole \
                 by more than the chordal sag"
            );
            if (radial - HOLE_RADIUS).abs() <= delta + 1e-9 {
                on_the_wall += 1;
            }
        }
        assert!(
            on_the_wall >= 6,
            "delta {delta}: only {on_the_wall} vertices lie on the hole wall — the hole is missing"
        );
    }
}

/// A framed camera contains the scene across every viewport shape the
/// application can realistically hand it. The shipped rows check one
/// aspect each; this sweeps the range, and asserts the framing is TIGHT
/// as well as containing, so a camera that "fits" by retreating to the
/// far end of the zoom band cannot pass.
#[test]
fn framing_contains_and_is_tight_across_realistic_aspects() {
    let mesh = built(1.0e-4);
    let bounds = mesh.bounds();
    for aspect in [0.1, 0.4, 0.75, 1.0, 4.0 / 3.0, 16.0 / 9.0, 3.0, 10.0] {
        let camera = Camera::framing(&bounds, aspect).expect("the scene frames");
        let mut worst: f64 = 0.0;
        for corner in corners(&bounds) {
            let ndc = camera
                .project(corner, aspect)
                .expect("a positive aspect projects")
                .expect("every corner is in front of the eye");
            assert!(
                ndc[0].abs() <= 1.0 && ndc[1].abs() <= 1.0 && (0.0..=1.0).contains(&ndc[2]),
                "aspect {aspect}: corner {corner:?} projects to {ndc:?}, outside the frustum"
            );
            worst = worst.max(ndc[0].abs().max(ndc[1].abs()));
        }
        assert!(
            worst > 0.25,
            "aspect {aspect}: the framed scene fills only {worst} of the frustum — \
             a fit that far away is not a fit"
        );
    }
}

/// `Camera::fitted` documents that a frame "backs off far enough that
/// the bounding sphere fits", and `Camera` refuses rather than
/// inventing state everywhere else. At an extreme aspect the
/// scene-derived zoom band clamps the computed distance and the result
/// used to be neither: a camera that does not contain its scene,
/// returned `Ok`.
///
/// **UN-IGNORED in the fix pass** (was `#[ignore]`d and RED at the
/// frozen head, R1's own reporting row): `fitted` now refuses
/// `CameraError::Unfittable` instead of clamping, so both acceptable
/// answers — fit, or refuse — are the only ones reachable, and this
/// row gates them.
#[test]
fn framing_at_an_extreme_aspect_should_contain_or_refuse() {
    let bounds = plate_bounds();
    for aspect in [0.05, 0.02, 0.005] {
        let Ok(camera) = Camera::framing(&bounds, aspect) else {
            continue; // refusing is the other acceptable answer
        };
        for corner in corners(&bounds) {
            let ndc = camera
                .project(corner, aspect)
                .expect("projects")
                .expect("in front of the eye");
            assert!(
                ndc[0].abs() <= 1.0 && ndc[1].abs() <= 1.0,
                "aspect {aspect}: framing returned Ok with corner {corner:?} at {ndc:?}, \
                 outside the frustum — it neither contained the scene nor refused"
            );
        }
    }
}

/// A fold stops at the FIRST refusal, distinguishably: two different
/// refusal arms, in both orders, must come back in the order they were
/// met. The shipped row's single arm cannot tell "stopped at the first"
/// from "kept going and returned the last".
#[test]
fn a_fold_returns_the_first_refusal_and_only_that_one() {
    let camera = Camera::framing(&plate_bounds(), 1.5).expect("the plate frames");
    let dolly = CameraOp::Dolly { factor: -1.0 };
    let nan_orbit = CameraOp::Orbit {
        yaw: f64::NAN,
        pitch: 0.0,
    };
    let good = CameraOp::Orbit {
        yaw: 0.25,
        pitch: 0.0,
    };

    assert!(
        matches!(
            camera::fold(&camera, &[good, dolly, nan_orbit]),
            Err(CameraOpError::NonPositiveDolly { .. })
        ),
        "the dolly came first and must be the refusal that comes back"
    );
    assert!(
        matches!(
            camera::fold(&camera, &[good, nan_orbit, dolly]),
            Err(CameraOpError::NotFinite { what: "yaw", .. })
        ),
        "the NaN orbit came first and must be the refusal that comes back"
    );
    // And a fold with no refusal is exactly the composition.
    let folded = camera::fold(&camera, &[good, good]).expect("two finite orbits");
    let twice = camera::apply(
        &camera,
        &CameraOp::Orbit {
            yaw: 0.5,
            pitch: 0.0,
        },
    )
    .expect("a finite orbit");
    assert!(
        (folded.yaw() - twice.yaw()).abs() < 1e-12,
        "fold is not composition: {} vs {}",
        folded.yaw(),
        twice.yaw()
    );
}

/// The doors refuse what is not a value, with the arm that names the
/// reason — and the signatures that carry them mention no toolkit type,
/// no document arena key and no window (G1's boundary rule, asserted as
/// a type ascription that stops compiling if it stops being true).
#[test]
fn the_layer_three_doors_refuse_typed_and_name_no_toolkit_type() {
    // If any of these gains an `egui`, `wgpu` or arena-key parameter,
    // this file stops compiling.
    let _apply: fn(&Camera, &CameraOp) -> Result<Camera, CameraOpError> = camera::apply;
    let _framing: fn(&Aabb, f64) -> Result<Camera, CameraError> = Camera::framing;
    let _map: fn(&InputMap, &ViewportEvent, ViewportSize, &Camera) -> Option<CameraOp> =
        InputMap::map;

    let camera = Camera::framing(&plate_bounds(), 1.5).expect("the plate frames");
    assert!(matches!(
        Camera::framing(&plate_bounds(), 0.0),
        Err(CameraError::UnusableBounds)
    ));
    assert!(matches!(
        Camera::framing(
            &Aabb {
                min_x: 1.0,
                min_y: 1.0,
                min_z: 1.0,
                max_x: 0.0,
                max_y: 0.0,
                max_z: 0.0,
            },
            1.0
        ),
        Err(CameraError::UnusableBounds)
    ));
    assert!(matches!(
        Camera::new(
            Point3::new(0.0, 0.0, 0.0),
            1.0,
            0.0,
            0.0,
            std::f64::consts::PI,
            1.0
        ),
        Err(CameraError::FieldOfViewOutOfRange { .. })
    ));
    assert!(matches!(
        camera::apply(
            &camera,
            &CameraOp::Frame {
                bounds: plate_bounds(),
                aspect: f64::NAN
            }
        ),
        Err(CameraOpError::Unframeable(CameraError::NotFinite {
            what: "aspect",
            ..
        }))
    ));
    // The display tolerance door takes finite-and-positive, and says so
    // when it does not get one.
    assert!(DisplayTolerance::new(f64::MIN_POSITIVE).is_ok());
    assert!(DisplayTolerance::new(-0.0).is_err());
    assert!(DisplayTolerance::new(f64::INFINITY).is_err());
}
