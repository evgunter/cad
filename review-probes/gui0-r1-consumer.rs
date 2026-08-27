//! An OUTSIDE consumer of the `viewer` crate's public surface: a
//! program that is not in the workspace, drives the camera vocabulary
//! and the scene path headlessly, and reports what it finds.
//!
//! This is the review's end-to-end exercise, not a gate.

use std::collections::HashMap;

use bvh::{Aabb, Axis};
use pncad::geom_core::{Point3, Tol};
use viewer::camera::{self, Camera, CameraOp, CameraOpError};
use viewer::input::{self, InputMap, PointerButton, ViewportEvent, ViewportSize};
use viewer::scene::{self, DisplayTolerance, SceneError};

fn main() {
    let tol = pncad::tolerance::witness();
    println!("== 1. the scene, through the public doors ==");
    let (doc, root) = scene::plate_with_hole(tol).expect("author");
    println!(
        "   nodes = {}, roots = {}, root = {root:?}",
        doc.order().len(),
        doc.roots().len()
    );

    for exp in [3i32, 4, 5] {
        let d = DisplayTolerance::new(10f64.powi(-exp)).expect("delta");
        let mesh = scene::scene_of(&doc, d, tol).expect("scene");
        let s = mesh.stats();
        let b = mesh.bounds();
        println!(
            "   d=1e-{exp}: faces={} tris={} bounds=({:.4},{:.4},{:.4})..({:.4},{:.4},{:.4})",
            s.faces,
            s.triangles,
            b.min(Axis::X),
            b.min(Axis::Y),
            b.min(Axis::Z),
            b.max(Axis::X),
            b.max(Axis::Y),
            b.max(Axis::Z),
        );
    }

    let mesh = scene::scene_of(&doc, DisplayTolerance::new(1e-4).expect("d"), tol).expect("scene");

    println!("\n== 2. is the drawn triangle soup closed? (edge pairing over exact f32) ==");
    let mut edges: HashMap<([u32; 3], [u32; 3]), i32> = HashMap::new();
    let key = |p: [f32; 3]| [p[0].to_bits(), p[1].to_bits(), p[2].to_bits()];
    for t in mesh.positions().chunks_exact(3) {
        let (a, b, c) = (key(t[0]), key(t[1]), key(t[2]));
        for (u, v) in [(a, b), (b, c), (c, a)] {
            if u <= v {
                *edges.entry((u, v)).or_default() += 1;
            } else {
                *edges.entry((v, u)).or_default() -= 1;
            }
        }
    }
    let unbalanced = edges.values().filter(|v| **v != 0).count();
    println!(
        "   distinct undirected edges = {}, unbalanced = {unbalanced}",
        edges.len()
    );

    println!("\n== 3. bounds come from the position TABLE, not the drawn corners ==");
    let mut lo = [f32::INFINITY; 3];
    let mut hi = [f32::NEG_INFINITY; 3];
    for p in mesh.positions() {
        for i in 0..3 {
            lo[i] = lo[i].min(p[i]);
            hi[i] = hi[i].max(p[i]);
        }
    }
    let b = mesh.bounds();
    println!("   drawn   x {:.6}..{:.6}", lo[0], hi[0]);
    println!("   claimed x {:.6}..{:.6}", b.min(Axis::X), b.max(Axis::X));

    println!("\n== 4. camera framing: how much slack is there? ==");
    for aspect in [16.0 / 9.0, 1.0, 0.5, 0.1, 0.02] {
        match Camera::framing(&mesh.bounds(), aspect) {
            Ok(cam) => {
                let mut worst: f64 = 0.0;
                let mut behind = 0;
                for p in mesh.positions() {
                    let pt = Point3::new(f64::from(p[0]), f64::from(p[1]), f64::from(p[2]));
                    match cam.project(pt, aspect) {
                        Ok(Some(ndc)) => worst = worst.max(ndc[0].abs().max(ndc[1].abs())),
                        Ok(None) => behind += 1,
                        Err(e) => println!("   projection refused: {e:?}"),
                    }
                }
                println!(
                    "   aspect {aspect:>5.2}: distance {:.5} (band {:.5}..{:.5}) worst |ndc| = {worst:.4}, behind = {behind}",
                    cam.distance(),
                    cam.min_distance(),
                    cam.max_distance()
                );
            }
            Err(e) => println!("   aspect {aspect:>5.2}: framing refused {e:?}"),
        }
    }

    println!("\n== 5. the operation vocabulary, and its refusals ==");
    let cam = Camera::framing(&mesh.bounds(), 1.6).expect("frame");
    let ops = [
        CameraOp::Orbit {
            yaw: 0.3,
            pitch: 0.2,
        },
        CameraOp::Dolly { factor: 0.5 },
        CameraOp::Pan {
            right: 0.01,
            up: -0.005,
        },
    ];
    let walked = camera::fold(&cam, &ops).expect("fold");
    println!(
        "   after 3 ops: yaw {:.4} pitch {:.4} dist {:.5} target ({:.4},{:.4},{:.4})",
        walked.yaw(),
        walked.pitch(),
        walked.distance(),
        walked.target().x,
        walked.target().y,
        walked.target().z
    );
    // Two DIFFERENT refusals in one fold: which one comes back?
    let mixed = [
        CameraOp::Dolly { factor: -1.0 },
        CameraOp::Orbit {
            yaw: f64::NAN,
            pitch: 0.0,
        },
    ];
    println!(
        "   fold(-dolly, NaN-orbit) = {:?}",
        camera::fold(&cam, &mixed)
    );
    let mixed_rev = [
        CameraOp::Orbit {
            yaw: f64::NAN,
            pitch: 0.0,
        },
        CameraOp::Dolly { factor: -1.0 },
    ];
    println!(
        "   fold(NaN-orbit, -dolly) = {:?}",
        camera::fold(&cam, &mixed_rev)
    );

    println!("\n== 6. the input mapping, from outside ==");
    let map = InputMap::default();
    let vp = ViewportSize {
        width_px: 1280.0,
        height_px: 720.0,
    };
    let stream = [
        ViewportEvent::Drag {
            button: PointerButton::Primary,
            shift: false,
            delta_px: [40.0, 0.0],
        },
        ViewportEvent::Drag {
            button: PointerButton::Primary,
            shift: true,
            delta_px: [40.0, 0.0],
        },
        ViewportEvent::Drag {
            button: PointerButton::Middle,
            shift: false,
            delta_px: [40.0, 0.0],
        },
        ViewportEvent::Drag {
            button: PointerButton::Middle,
            shift: true,
            delta_px: [40.0, 0.0],
        },
        ViewportEvent::Drag {
            button: PointerButton::Secondary,
            shift: false,
            delta_px: [40.0, 0.0],
        },
        ViewportEvent::Scroll { units: 2.0 },
    ];
    for ev in &stream {
        println!("   {ev:?}\n     -> {:?}", map.map(ev, vp, &cam));
    }
    let (end, produced) = input::map_stream(&map, &cam, vp, &stream).expect("stream");
    println!(
        "   stream produced {} ops, final distance {:.5}",
        produced.len(),
        end.distance()
    );

    println!("\n== 7. refusal paths at the doors ==");
    for bad in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::MIN_POSITIVE] {
        match DisplayTolerance::new(bad) {
            Ok(d) => {
                let r = scene::scene_of(&doc, d, tol);
                println!(
                    "   delta {bad:e} ACCEPTED by the door; scene_of -> {}",
                    match r {
                        Ok(m) => format!("ok, {} tris", m.stats().triangles),
                        Err(e) => format!("{e:?}"),
                    }
                );
            }
            Err(SceneError::InvalidDisplayTolerance { delta }) => {
                println!("   delta {bad:e} refused: InvalidDisplayTolerance {{ delta: {delta:e} }}")
            }
            Err(other) => println!("   delta {bad:e} refused with {other:?}"),
        }
    }
    let empty = Aabb {
        min_x: 1.0,
        min_y: 1.0,
        min_z: 1.0,
        max_x: 0.0,
        max_y: 0.0,
        max_z: 0.0,
    };
    println!(
        "   framing an inverted box: {:?}",
        Camera::framing(&empty, 1.0)
    );
    println!(
        "   framing with aspect 0:   {:?}",
        Camera::framing(&mesh.bounds(), 0.0)
    );
    println!(
        "   apply(Frame aspect NaN): {:?}",
        camera::apply(
            &cam,
            &CameraOp::Frame {
                bounds: mesh.bounds(),
                aspect: f64::NAN
            }
        )
    );

    println!("\n== 8. does the public surface leak a layer-2 arena key? ==");
    // Purely a compile-time observation: the whole camera/input surface
    // is f64, Aabb and the crate's own enums. Nothing here names a
    // RecipeNodeId, an arena key, or a stable ref.
    let _: fn(&Camera, &CameraOp) -> Result<Camera, CameraOpError> = camera::apply;
    println!(
        "   camera::apply: fn(&Camera, &CameraOp) -> Result<Camera, CameraOpError>  [no toolkit, no arena key]"
    );

    println!("\n== 9. does Tol reach the scene as a parameter? ==");
    println!(
        "   scene_of(&doc, delta, tol) — tol is a parameter; the only mint is the bin's pncad::tolerance::witness()"
    );
    let _ = Tol::witness;
}
