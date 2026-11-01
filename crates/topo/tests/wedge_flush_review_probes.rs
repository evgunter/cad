//! **The wedge on a block's top, over a grid of poses** (ignored
//! battery beside `wedge_flush_on_a_block_top.rs`): exact corner
//! standoffs, the glue door's reading and every op, as a table. It
//! varies the opening, the wedge's lengths and the block, which
//! `test_support::wedge_on_a_block_top` fixes, so it builds its own.
//!
//! Run: `cargo nextest run --release -p topo --test all --run-ignored
//! only --no-capture -E 'test(/wedge_flush_review_probes/)'`, with
//! `CAD_TOLERANCE_EPS` for the ε row; `PROBE_THETA` fixes the tilt
//! (m/m; default `k·ε` over `k ∈ {±1.2, ±2}`).

#![allow(clippy::expect_used, clippy::panic, clippy::print_stderr)]

use geom_core::test_support::upper;
use geom_core::{Band, Point3, Tol, Vec3};
use topo::test_support::{brick, face_facing, finished, mapped_cube};
use topo::{Body, CarrierDesc, face_carrier};

struct Pose {
    block_x: (f64, f64),
    block_y: (f64, f64),
    opening_deg: f64,
    eb_len: f64,
    ea_len: f64,
    height: f64,
    depth: f64,
    facing: f64,
}

fn build(pose: &Pose, theta: f64, tol: Tol) -> (topo::AtRestBody<f64>, topo::AtRestBody<f64>) {
    let phi = pose.opening_deg.to_radians();
    let p = Point3::new(0.5, 0.2, 1.0);
    let ea = Vec3::new(pose.ea_len, 0.0, 0.0);
    let eb = Vec3::new(phi.cos(), phi.sin(), theta * phi.sin()) * pose.eb_len;
    let (h, d) = (pose.height, pose.depth);
    let block = finished(
        "block",
        brick::<f64>(pose.block_x, pose.block_y, (0.0, 1.0), tol),
        tol,
    );
    let tool = finished(
        "tool",
        mapped_cube::<f64>(
            move |u, v, w| p + ea * u + eb * v + Vec3::new(0.0, 0.0, h * w - d),
            tol,
        ),
        tol,
    );
    (block, tool)
}

fn outcome<B>(r: Result<B, topo::BooleanError>) -> String {
    match r {
        Ok(_) => "BUILD".into(),
        Err(e) => {
            let (key, defect) = topo::test_support::offer_key(&e);
            let m = match &e {
                topo::BooleanError::Escalated { diag, .. } => {
                    format!(" m={:.3e}", upper(diag.margin))
                }
                _ => String::new(),
            };
            format!("{key}{}{m}", if defect { " DEFECT" } else { "" })
        }
    }
}

/// The exact (f64) standoff of each face's corners off the other's
/// carrier plane: the block top's four corners, the wedge face's four.
fn standoffs(
    pose: &Pose,
    theta: f64,
    block: &Body<f64>,
    fb: topo::FaceKey,
    tool: &Body<f64>,
    ft: topo::FaceKey,
) -> (f64, f64) {
    let plane = |b: &Body<f64>, f| match face_carrier(b, f) {
        Some(CarrierDesc::Plane { origin, normal }) => (origin, normal.normalize()),
        _ => panic!("plane"),
    };
    let off = |pts: &[Point3<f64>], (o, n): (Point3<f64>, Vec3<f64>)| {
        pts.iter()
            .map(|q| (*q - o).dot(n).abs())
            .fold(0.0, f64::max)
    };
    let (x, y) = (pose.block_x, pose.block_y);
    let block_pts = [
        Point3::new(x.0, y.0, 1.0),
        Point3::new(x.1, y.0, 1.0),
        Point3::new(x.0, y.1, 1.0),
        Point3::new(x.1, y.1, 1.0),
    ];
    let phi = pose.opening_deg.to_radians();
    let p = Point3::new(0.5, 0.2, 1.0);
    let ea = Vec3::new(pose.ea_len, 0.0, 0.0);
    let eb = Vec3::new(phi.cos(), phi.sin(), theta * phi.sin()) * pose.eb_len;
    let tool_pts = [p, p + ea, p + eb, p + ea + eb];
    (
        off(&block_pts, plane(tool, ft)),
        off(&tool_pts, plane(block, fb)),
    )
}

fn row(label: &str, pose: &Pose, theta: f64) {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let eps = band.zero();
    let (block, tool) = build(pose, theta, tol);
    let (fb, ft) = (face_facing(&block, 1.0), face_facing(&tool, pose.facing));
    let (block_off, tool_off) = standoffs(pose, theta, &block, fb, &tool, ft);
    let glue = match topo::flush_pair_relation(&block, fb, &tool, ft, band) {
        Ok(Ok(rel)) => format!("{rel:?}"),
        Ok(Err(topo::PlaneEqError::Escalated { rung, diag })) => {
            format!("Esc({rung:?} {:.2}ε)", upper(diag.margin) / eps)
        }
        other => format!("{other:?}"),
    };
    let ops = [
        ("A∪B", outcome(topo::union(&block, &tool, tol))),
        ("B∪A", outcome(topo::union(&tool, &block, tol))),
        ("A∩B", outcome(topo::intersect(&block, &tool, tol))),
        ("B∩A", outcome(topo::intersect(&tool, &block, tol))),
        ("A∖B", outcome(topo::subtract(&block, &tool, tol))),
        ("B∖A", outcome(topo::subtract(&tool, &block, tol))),
    ];
    eprintln!(
        "PROBE ε={eps:.2e} {label} θ={theta:.3e} block_off={:.2}ε tool_off={:.3}ε glue={glue}",
        block_off / eps,
        tool_off / eps
    );
    for (op, o) in ops {
        eprintln!("PROBE   {op} {o}");
    }
}

fn pr_poses() -> [(&'static str, Pose); 2] {
    let base = |height, depth, facing| Pose {
        block_x: (0.0, 3.0),
        block_y: (-2.0, 2.5),
        opening_deg: 5.0,
        eb_len: 1.0,
        ea_len: 1.0,
        height,
        depth,
        facing,
    };
    [
        ("standing", base(1.0, 0.0, -1.0)),
        ("sunk", base(0.5, 0.5, 1.0)),
    ]
}

/// The PR's own poses: exact standoffs, glue door, every op.
#[test]
#[ignore = "probe"]
fn probe_pr_poses() {
    let eps = Tol::witness().eps();
    let thetas: Vec<f64> = match std::env::var("PROBE_THETA") {
        Ok(t) => vec![t.parse().expect("θ")],
        Err(_) => [1.2, -1.2, 2.0, -2.0].iter().map(|k| k * eps).collect(),
    };
    for (label, pose) in pr_poses() {
        for &theta in &thetas {
            row(label, &pose, theta);
        }
    }
}

/// Other poses: opening angle, a longer wedge face, a block that barely
/// outreaches the wedge, larger tilt.
#[test]
#[ignore = "probe"]
fn probe_other_poses() {
    let eps = Tol::witness().eps();
    let mut poses = Vec::new();
    for (h, d, f, stand) in [(1.0, 0.0, -1.0, "standing"), (0.5, 0.5, 1.0, "sunk")] {
        for opening in [2.0, 5.0, 20.0, 60.0, 90.0] {
            for (eb_len, ea_len) in [(1.0, 1.0), (2.0, 2.0)] {
                for (bx, by, bname) in [
                    ((0.0, 3.0), (-2.0, 2.5), "big"),
                    ((0.3, 2.6), (0.1, 2.3), "tight"),
                ] {
                    poses.push((
                        format!("{stand} φ={opening} eb={eb_len} {bname}"),
                        Pose {
                            block_x: bx,
                            block_y: by,
                            opening_deg: opening,
                            eb_len,
                            ea_len,
                            height: h,
                            depth: d,
                            facing: f,
                        },
                    ));
                }
            }
        }
    }
    for (label, pose) in &poses {
        for k in [0.5, 1.2, 2.0, 4.0] {
            row(label, pose, k * eps);
        }
    }
}
