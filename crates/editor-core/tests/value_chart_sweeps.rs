//! **The value chart over whole sweeps** (D10 `[placement.values]`):
//! a copy hung under a plate the chart reads through every tilt short of
//! the pole, every orientation written backwards against a level plate,
//! a source face on no axis of its own, and the door's count of the
//! slides a rest leaves. Each row prints what it saw; the asserts are
//! the claims.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::Dimension;
use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    CapEnd, Coord, DocumentId, EvalOptions, Formula, Frame, Node, Operand, Orientation, Placed,
    ProfileDoc, RecipeNodeId, RoleSeg, SignedAxis, StableName, TargetAxis,
};
use fixture::{head, head_at, insert, len, on_frame, solve};
use geom_core::Tol;

fn prism(doc: ProfileDoc, pts: Vec<(f64, f64)>, z: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(doc, [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![pts]);
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(z),
        },
    )
}

fn block(doc: ProfileDoc, x: f64, y: f64, z: f64) -> (ProfileDoc, RecipeNodeId) {
    prism(doc, vec![(0.0, 0.0), (x, 0.0), (x, y), (0.0, y)], z)
}

fn face(doc: &ProfileDoc, body: RecipeNodeId, at: [f64; 3], normal: [f64; 3]) -> StableName {
    let read = editor_core::test_support::plane_on(doc, body, at, normal, Tol::witness()).read;
    let Operand::Pose(def) = read else {
        panic!("a plane read: {read:?}")
    };
    let PoseDef::Plane {
        face: Operand::Select { names, .. },
    } = *def
    else {
        panic!("a plane of one face")
    };
    names[0].clone()
}

type Out = Result<Frame, String>;

/// Write the bundle through the door and solve; any refusal as text.
fn place(doc: ProfileDoc, placement: RecipeNodeId, placed: Placed) -> (Out, ProfileDoc) {
    let mut doc = doc;
    for edit in fixture::bundle_edits(&doc, placement, placed.bundle()) {
        match editor_core::apply(&doc, &edit, Tol::witness()) {
            Ok(a) => doc = a.doc,
            Err(e) => return (Err(format!("door: {e}")), doc),
        }
    }
    let poses = solve(&doc, &EvalOptions::default(), Tol::witness());
    let out = match poses.fault(placement) {
        Some(f) => Err(format!("fault: {f} [{f:?}]")),
        None => poses
            .placement(&doc, placement)
            .map_err(|e| format!("unplaced: {e:?}")),
    };
    (out, doc)
}

fn ori(x: SignedAxis, z: SignedAxis) -> Orientation {
    Orientation::new(x, z).unwrap()
}

fn lin(f: &Frame, v: [f64; 3]) -> [f64; 3] {
    [0, 1, 2].map(|r| (0..3).map(|c| f.columns[c][r] * v[c]).sum())
}

fn apply(f: &Frame, p: [f64; 3]) -> [f64; 3] {
    let moved = lin(f, p);
    [0, 1, 2].map(|r| f.translation[r] + moved[r])
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn dist(a: &Frame, b: &Frame) -> f64 {
    (0..3)
        .map(|i| (a.translation[i] - b.translation[i]).abs())
        .chain((0..3).flat_map(|c| (0..3).map(move |r| (a.columns[c][r] - b.columns[c][r]).abs())))
        .fold(0.0, f64::max)
}

/// A floor turned by `moves`, a unit box resting on it by its bottom.
/// Returns the floor's top normal and a point on it in the world (the
/// floor's own frame is read off a bare copy at the same moves).
fn on_floor(
    moves: Vec<(Coord, Formula)>,
    values: Vec<(Coord, Formula)>,
    orient: Orientation,
    direction: bool,
) -> Out {
    let doc = ProfileDoc::empty(DocumentId::derive("r1-floor"), Tol::witness());
    let (doc, body) = block(doc, 1.0, 1.0, 1.0);
    let (doc, floor, _) = fixture::floor(doc, moves);
    let (doc, placement) = insert(
        doc,
        fixture::stood(editor_core::BodyRead::plain(Operand::from(body))),
    );
    let bottom = fixture::fname(body, RoleSeg::Cap(CapEnd::Start));
    let mut mate = if direction {
        let n = |p: Operand| {
            Operand::Pose(Box::new(PoseDef::Flip {
                pose: RoleRead::through(p, PoseRole::Normal),
            }))
        };
        // A direction mate: the bottom's normal flipped (`+z`) on the
        // flipped floor's normal flipped back (the floor's `+z`), so the
        // copy stands upright as the plane rest stands it.
        let to = Operand::Pose(Box::new(PoseDef::Flip {
            pose: RoleRead::through(fixture::flip(floor), PoseRole::Normal),
        }));
        fixture::mate(n(fixture::plane(head(bottom))), to)
    } else {
        fixture::mate(fixture::plane(head(bottom)), fixture::flip(floor))
    };
    mate.orient = orient;
    place(
        doc,
        placement,
        Placed {
            mates: vec![mate],
            values,
        },
    )
    .0
}

fn rx(t: f64) -> Vec<(Coord, Formula)> {
    vec![(Coord::Spin(Some(TargetAxis::X)), fixture::ang(t))]
}

/// The floor's up normal at `rx(t)`: R_x(t)·z.
fn up_x(t: f64) -> [f64; 3] {
    [0.0, -t.sin(), t.cos()]
}

/// P1: every bundle pins its copy, so the settle's count dispatch only
/// ever sees as many slides as the mate leaves: fewer refuse at the door.
/// **A rest written with one slide refuses at the door**, level or
/// tilted, mated by a plane or by a direction: a plane leaves two slides
/// and a direction three, and the one unwritten is named.
///
/// *Breaks if* the door admits a bundle short of pinning its copy.
#[test]
fn a_rest_written_with_one_slide_refuses_at_the_door() {
    let x = |v| (Coord::Slide(TargetAxis::X), len(v));
    let sp = (Coord::SPIN, Formula::zero(Dimension::Angle));
    for t in [0.0, 0.5] {
        for direction in [false, true] {
            let got = on_floor(
                rx(t),
                vec![x(3.0), sp.clone()],
                Orientation::IDENTITY,
                direction,
            );
            assert!(
                matches!(&got, Err(e) if e.starts_with("door:") && e.contains("unwritten")),
                "t={t} direction={direction}: {got:?}"
            );
        }
    }
}

/// The boss hung under a plate tilted by a chain (as in the PR's row),
/// generalised: any chain, any orientation.
fn hung(chain: Vec<(Coord, Formula)>, orient: Orientation, values: Vec<(Coord, Formula)>) -> Out {
    let doc = ProfileDoc::empty(DocumentId::derive("value-chart-sweep-hung"), Tol::witness());
    let (doc, plate) = block(doc, 20.0, 10.0, 2.0);
    let (doc, boss) = block(doc, 1.0, 1.0, 1.0);
    let bottom = face(&doc, plate, [10.0, 5.0, 0.0], [0.0, 0.0, -1.0]);
    let (doc, b) = fixture::in_world_by(doc, plate, Placed::unmated(chain).values);
    let (doc, placement) = insert(
        doc,
        fixture::stood(editor_core::BodyRead::plain(Operand::from(boss))),
    );
    let top = fixture::fname(boss, RoleSeg::Cap(CapEnd::End));
    let mut mate = fixture::rest(head(top), head_at(b, bottom));
    mate.orient = orient;
    place(
        doc,
        placement,
        Placed {
            mates: vec![mate],
            values,
        },
    )
    .0
}

fn std_values_for(o: Orientation) -> Vec<(Coord, Formula)> {
    let j = o.z().index();
    let mut v: Vec<(Coord, Formula)> = TargetAxis::ALL
        .into_iter()
        .filter(|a| a.index() != j)
        .zip([10.0, 0.0])
        .map(|(a, by)| (Coord::Slide(a), len(by)))
        .collect();
    v.push((Coord::SPIN, Formula::zero(Dimension::Angle)));
    v
}

/// **A plate swept through all but the pole moves its hung copy
/// continuously**: the plate's tilt from −(π−δ) to π−δ about `x`, `y`
/// and an oblique chain, under five orientations; between steps the
/// copy's frame moves by at most the lever times the step, and nothing
/// refuses.
///
/// *Breaks if* the chart has a branch short of the pole (a jump), or
/// refuses short of its band.
#[test]
fn a_plate_swept_through_all_but_the_pole_moves_its_hung_copy_continuously() {
    plate_sweep(
        &[
            Orientation::IDENTITY,
            ori(SignedAxis::PlusY, SignedAxis::PlusZ),
            ori(SignedAxis::MinusX, SignedAxis::PlusZ),
            ori(SignedAxis::PlusX, SignedAxis::PlusY),
            ori(SignedAxis::PlusZ, SignedAxis::MinusX),
        ],
        &["x", "y", "oblique"],
        400,
    );
}

/// **The plate sweep, at a few stations**: the fast set's twin of
/// [`a_plate_swept_through_all_but_the_pole_moves_its_hung_copy_continuously`],
/// two orientations about the oblique chain in sixteen steps.
///
/// *Breaks if* as the full sweep, wherever a jump is as wide as a step.
#[test]
fn a_plate_swept_at_a_few_stations_moves_its_hung_copy_continuously() {
    plate_sweep(
        &[
            Orientation::IDENTITY,
            ori(SignedAxis::PlusZ, SignedAxis::MinusX),
        ],
        &["oblique"],
        16,
    );
}

/// The plate sweep under `orients`, about each of `axes`, in `steps`
/// steps from −(π−δ) to π−δ: no refusal, and no step moving the copy by
/// more than the lever times the step.
fn plate_sweep(orients: &[Orientation], axes: &[&str], steps: u32) {
    let delta = 1e-3;
    let mut report = Vec::new();
    for &o in orients {
        for &axis in axes {
            let chain = |t: f64| -> Vec<(Coord, Formula)> {
                match axis {
                    "x" => vec![(Coord::Spin(Some(TargetAxis::X)), fixture::ang(t))],
                    "y" => vec![(Coord::Spin(Some(TargetAxis::Y)), fixture::ang(t))],
                    _ => vec![
                        (Coord::Spin(Some(TargetAxis::Z)), fixture::ang(0.7)),
                        (Coord::Spin(Some(TargetAxis::X)), fixture::ang(t)),
                    ],
                }
            };
            let lo = -(std::f64::consts::PI - delta);
            let h = 2.0 * (std::f64::consts::PI - delta) / f64::from(steps);
            let mut last: Option<Frame> = None;
            let mut worst = 0.0_f64;
            let mut refusals = Vec::new();
            for k in 0..=steps {
                let t = lo + h * f64::from(k);
                match hung(chain(t), o, std_values_for(o)) {
                    Ok(f) => {
                        if let Some(l) = &last {
                            worst = worst.max(dist(&f, l) / h);
                        }
                        last = Some(f);
                    }
                    Err(e) => {
                        refusals.push(format!("{t:.4}: {}", &e[..e.len().min(90)]));
                        last = None;
                    }
                }
            }
            let line = format!(
                "{o} about {axis}: worst rate {worst:.2}/rad, {} refusals {:?}",
                refusals.len(),
                refusals.iter().take(3).collect::<Vec<_>>()
            );
            println!("{line}");
            report.push((worst, refusals.len(), line));
        }
    }
    // The lever is ~ 23 (the boss at x = 10 under a plate whose reach is
    // 20 × 10 × 2); a jump shows as a rate ≫ that.
    let bad: Vec<_> = report
        .iter()
        .filter(|(w, r, _)| *w > 40.0 || *r > 0)
        .map(|(_, _, l)| l.clone())
        .collect();
    assert!(bad.is_empty(), "jumps or refusals:\n{}", bad.join("\n"));
}

/// **Every orientation written backwards refuses, and its `write`
/// places**: under a level plate, of the 24 orientations each one
/// putting the boss's `z` on `−z` refuses `Backwards`, naming the
/// orientation `toward` picks, which then places the boss; every other
/// orientation places.
///
/// *Breaks if* the pole is answered, a refusal names an orientation that
/// still refuses, or an orientation short of the pole refuses.
#[test]
fn every_orientation_written_backwards_refuses_and_its_write_places() {
    let mut bad = Vec::new();
    for o in Orientation::all() {
        // The pole for this orientation is where O·z = −d, d the plate's
        // flipped bottom's normal = R·z. Level plate: pole iff O·z = −z.
        let level = hung(vec![], o, std_values_for(o));
        let backwards = o.z() == SignedAxis::MinusZ;
        match (&level, backwards) {
            (Err(e), true) if e.contains("Backwards") => {
                let w = o.toward(
                    geom_core::linalg::Vec3::new(0.0, 0.0, 1.0),
                    geom_core::linalg::Vec3::new(0.0, 0.0, 1.0),
                );
                println!("{o}: refuses, write {w}: {e}");
                if !e.contains(&format!("write: {w:?}")) {
                    bad.push(format!("{o}: the fault does not name {w:?}: {e}"));
                }
                if let Err(e2) = hung(vec![], w, std_values_for(w)) {
                    bad.push(format!("{o}: writing {w} still refuses: {e2}"));
                }
            }
            (Ok(_), false) => {}
            (Ok(f), true) => bad.push(format!("{o}: backwards but placed at {:?}", f.translation)),
            (Err(e), _) => bad.push(format!("{o}: refused {e}")),
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **A wedge resting on its sloped face stays on a tilting floor**: the
/// source face on no axis of its own, under three orientations and each
/// pair of written slides, the floor tilted through 3 rad; where the
/// slides span, the wedge's slope lies on the floor, antiparallel, and
/// moves continuously with the tilt.
///
/// *Breaks if* the span check admits a pair that cannot place the
/// wedge, or the chart jumps on an oblique source.
#[test]
fn a_wedge_resting_on_its_sloped_face_stays_on_a_tilting_floor() {
    wedge_sweep(
        &[
            Orientation::IDENTITY,
            ori(SignedAxis::PlusY, SignedAxis::PlusZ),
            ori(SignedAxis::PlusX, SignedAxis::MinusY),
        ],
        &["x,y", "x,z", "y,z"],
        300,
    );
}

/// **The wedge sweep, at a few stations**: the fast set's twin of
/// [`a_wedge_resting_on_its_sloped_face_stays_on_a_tilting_floor`], one
/// orientation and each pair of slides, the floor tilted through 3 rad
/// in ten steps.
///
/// *Breaks if* as the full sweep, wherever a jump is as wide as a step.
#[test]
fn a_wedge_resting_at_a_few_stations_stays_on_a_tilting_floor() {
    wedge_sweep(
        &[ori(SignedAxis::PlusY, SignedAxis::PlusZ)],
        &["x,y", "x,z", "y,z"],
        10,
    );
}

/// The wedge sweep under `orients`, for each pair of written slides
/// named in `pairs`, the floor tilted from −1.5 to 1.5 rad in `steps`
/// steps: where the slides span, the slope lies on the floor,
/// antiparallel, and no step moves the wedge by more than the lever
/// times the step.
fn wedge_sweep(orients: &[Orientation], pairs: &[&str], steps: u32) {
    let mut bad = Vec::new();
    let run = |t: f64, values: Vec<(Coord, Formula)>, o: Orientation| -> Out {
        let doc = ProfileDoc::empty(
            DocumentId::derive("value-chart-sweep-wedge"),
            Tol::witness(),
        );
        let (doc, wedge) = prism(doc, vec![(0.0, 0.0), (4.0, 0.0), (0.0, 3.0)], 1.0);
        let slope = face(&doc, wedge, [2.0, 1.5, 0.5], [0.6, 0.8, 0.0]);
        let (doc, floor, _) = fixture::floor(doc, rx(t));
        let (doc, placement) = insert(
            doc,
            fixture::stood(editor_core::BodyRead::plain(Operand::from(wedge))),
        );
        let mut mate = fixture::mate(fixture::plane(head(slope)), fixture::flip(floor));
        mate.orient = o;
        place(
            doc,
            placement,
            Placed {
                mates: vec![mate],
                values,
            },
        )
        .0
    };
    let x = |v| (Coord::Slide(TargetAxis::X), len(v));
    let y = |v| (Coord::Slide(TargetAxis::Y), len(v));
    let z = |v| (Coord::Slide(TargetAxis::Z), len(v));
    let sp = (Coord::SPIN, fixture::ang(0.3));
    let h = 3.0 / f64::from(steps);
    for &o in orients {
        for (label, vals) in [
            ("x,y", vec![x(3.0), y(4.0), sp.clone()]),
            ("x,z", vec![x(3.0), z(4.0), sp.clone()]),
            ("y,z", vec![y(3.0), z(4.0), sp.clone()]),
        ]
        .into_iter()
        .filter(|(label, _)| pairs.contains(label))
        {
            // The pair spans what the plane leaves iff the copy's mated
            // direction, where the orientation puts it, has a component
            // along the axis the pair leaves out.
            let n = o.apply(geom_core::linalg::Vec3::new(0.6, 0.8, 0.0));
            let out = ["y,z", "x,z", "x,y"]
                .iter()
                .position(|p| p == &label)
                .expect("a pair of two slides");
            let admitted = [n.x, n.y, n.z][out] != 0.0;
            let mut last: Option<Frame> = None;
            let mut worst = 0.0_f64;
            let mut first = None;
            let mut placed = 0;
            for k in 0..=steps {
                let t = -1.5 + h * f64::from(k);
                match run(t, vals.clone(), o) {
                    Ok(f) => {
                        placed += 1;
                        // A point of the slope, `(4, 0, 0)`, on the floor's plane.
                        let off = dot(apply(&f, [4.0, 0.0, 0.0]), up_x(t));
                        let nrm = lin(&f, [0.6, 0.8, 0.0]);
                        let anti = dot(nrm, up_x(t)) + 1.0;
                        if off.abs() > 1e-9 || anti.abs() > 1e-9 {
                            bad.push(format!("{o} {label} t={t}: off {off:e} anti {anti:e}"));
                        }
                        if let Some(l) = &last {
                            worst = worst.max(dist(&f, l) / h);
                        }
                        last = Some(f);
                    }
                    Err(e) => {
                        if admitted || !e.contains("SlidesDoNotSpan") {
                            bad.push(format!("{o} {label} t={t}: refused {e}"));
                        }
                        if first.is_none() {
                            first = Some(format!("t={t}: {}", &e[..e.len().min(140)]));
                        }
                        last = None;
                    }
                }
            }
            if admitted && placed == 0 {
                bad.push(format!("{o} {label}: admitted, and placed nowhere"));
            }
            println!("wedge {o} {label}: worst rate {worst:.2}, first refusal {first:?}");
            if worst > 40.0 {
                bad.push(format!("wedge {o} {label}: jump rate {worst}"));
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
