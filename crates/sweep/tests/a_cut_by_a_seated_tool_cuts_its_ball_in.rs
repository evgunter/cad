//! **A cut whose tool holds a closed ball beside a seated pair builds
//! and reads back to the tool** (`crates/topo/README.md`,
//! `[topo.cut-material]`): the slab `[0, 4]² × [0, 1]` cut by one body of
//! three solids: two bricks above the slab, one resting on the other
//! across a planar record, and a radius-0.5 ball poking 0.3 into the
//! slab's top with its seam clear of the slab. The cut unites the bricks
//! into its material and keeps the ball standing beside them. No edge of
//! either body crosses the other, so the extent scan cuts the ball's face
//! along its meridian through the circle
//! (`topo::BooleanNaming::chart_cuts`), on the material. (A brick that
//! crossed the slab would put the op on the crossings path, where the
//! ball meets the interior-loop guard, `work/germ/interior-loop-cut-in.md`.)
//! Everything the cut states of its tool is read back to the tool
//! (`material::Through::restate`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Tol, Vec3};
use sweep::test_support::{ball_poled_y, finished};
use topo::{BooleanInputs, BooleanOp, BooleanResult, OperandRecords};

fn t() -> Tol {
    Tol::witness()
}

/// The cap `r`, `h` tall.
fn cap(r: f64, h: f64) -> f64 {
    core::f64::consts::PI * h * h * (3.0 * r - h) / 3.0
}

/// **The cut builds at its closed form and states only the tool's
/// cells**: the slab less the ball's cap,
/// tier-3 valid, the ball's face cut in on the material, and every
/// operand-B face its holdings and records name a face of the tool. Red
/// where the cut refuses, measures wrong, or names a cell the material
/// minted (`work/seal/a-cut-by-a-sphere-bearing-tool-on-a-planar-seat-aborts-on-its-re-chart.md`).
#[test]
fn a_cut_by_a_seated_tool_cuts_its_ball_in_and_reads_back_to_the_tool() {
    let slab = finished(
        "the slab",
        topo::test_support::brick::<f64>((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), t()),
        t(),
    );
    let mut tool = topo::test_support::brick::<f64>((0.5, 1.5), (0.5, 1.5), (2.0, 3.0), t());
    let upper = topo::test_support::brick::<f64>((0.5, 1.5), (0.5, 1.5), (3.0, 4.0), t());
    topo::graft_disjoint_all_keyed(&mut tool, &upper).unwrap();
    let ball = ball_poled_y(0.5, Vec3::new(3.0, 3.0, 1.2), t());
    topo::graft_disjoint_all_keyed(&mut tool, &ball).unwrap();
    let tool = finished("the tool", tool, t());
    let seat: Vec<topo::FaceKey> = tool
        .faces()
        .map(|(f, _)| f)
        .filter(|&f| {
            matches!(tool.face_surface(f), Some(geom::Surface::Plane { frame })
                if (frame.origin().z - 3.0).abs() < 1e-12 && frame.axis().z.abs() > 0.5)
        })
        .collect();
    assert_eq!(seat.len(), 2, "the premise: the bricks meet on z = 3");
    let mut records = OperandRecords::default();
    records.ff.push(topo::CarriedRecord {
        contact: topo::FfContact {
            a: seat[0].min(seat[1]),
            b: seat[0].max(seat[1]),
        },
        record: 0,
    });
    topo::test_support::take_cut_ins();
    let out = topo::boolean_op_with(
        BooleanOp::Subtract,
        &slab,
        &tool,
        &BooleanInputs {
            records_a: OperandRecords::default(),
            records_b: records,
        },
        topo::SweepStrategy::Realized,
        t(),
    )
    .unwrap_or_else(|e| panic!("the cut builds: {e:?}"));
    let BooleanResult::Body(out) = out else {
        panic!("a body");
    };
    topo::validate_geometric(&out.body, t()).expect("tier 3");
    let want = 16.0 - cap(0.5, 0.3);
    let v = topo::test_support::pinned_volume("the cut", &out.body, t()).volume;
    assert!((v - want).abs() < 1e-9, "volume {v}, closed form {want}");
    assert!(
        topo::test_support::take_cut_ins()
            .iter()
            .any(|&[_, b]| b > 0),
        "the ball's face is cut in on the material"
    );
    let tool_faces: std::collections::BTreeSet<topo::FaceKey> =
        tool.faces().map(|(f, _)| f).collect();
    for (face, held) in out.holdings.faces() {
        for &(place, f) in held
            .faces()
            .unwrap_or_else(|| panic!("{face:?} reads: {held:?}"))
        {
            if place == 1 {
                assert!(
                    tool_faces.contains(&f),
                    "{face:?} holds {f:?}, no face of the tool"
                );
            }
        }
    }
    for row in &out.coincidences {
        for cell in row.every_cell() {
            if let topo::RowCell::Input {
                input: topo::Operand::B,
                cell: topo::Cell::Face(f),
            } = cell
            {
                assert!(
                    tool_faces.contains(&f),
                    "row {row:?} names {f:?}, no face of the tool"
                );
            }
        }
    }
}
