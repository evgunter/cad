//! The value chart reads a written orientation (D10 `[placement.values]`,
//! `work/msolve/the-value-chart-reads-a-written-orientation.md`).
//!
//! Beside a mate fixing a direction `d`, the mate's orientation `O` puts
//! the copy's mated direction at `O·d_c`, and the least turn carries that
//! onto `d`: the slides run along the target's axes as the least turn
//! carries them, and the spin is the turn left about the target's own
//! direction, whatever side spells the contact's flip. The one pole is
//! the orientation written backwards, which refuses in its band, naming
//! the orientation to write; nothing refuses short of it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::pose::PoseDef;
use editor_core::{
    CapEnd, Coord, DocumentId, EvalOptions, Frame, MateFault, Node, Operand, Orientation, Placed,
    ProfileDoc, RecipeNodeId, RoleSeg, SignedAxis, StableName, TargetAxis,
};
use fixture::{head, head_at, insert, len, on_frame, solve};
use geom_core::Tol;

/// A box `x` by `y` by `z` on its own xy, at its own origin.
fn block(doc: ProfileDoc, x: f64, y: f64, z: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (x, 0.0), (x, y), (0.0, y)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(z),
        },
    )
}

/// The name of `body`'s planar face through `at` with outward normal
/// `normal`, both in its own coordinates.
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

/// What placing the hung boss came to: its frame, or the fault.
type Hung = Result<Frame, MateFault>;

/// **A unit boss hung under plate copy `B`**: a 20 × 10 × 2 plate in the
/// world turned `tilt` about the world's `about`, and a unit boss whose
/// top rests against `B`'s bottom face (outward `−z`, so the mated
/// direction, its flip, is `B`'s `+z`), under `orient`, with
/// `Slide(X) = 10` and the other slide and the spin `0`.
fn hung(tilt: f64, about: TargetAxis, orient: Orientation) -> Hung {
    let (doc, placement) = hung_doc(tilt, about, orient);
    let poses = solve(&doc, &EvalOptions::default(), Tol::witness());
    match poses.fault(placement) {
        Some(fault) => Err(fault.clone()),
        None => Ok(poses
            .placement(&doc, placement)
            .expect("the boss is placed")),
    }
}

/// The document [`hung`] solves, and the boss's placement.
fn hung_doc(tilt: f64, about: TargetAxis, orient: Orientation) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive("value-chart-hung"), Tol::witness());
    let (doc, plate) = block(doc, 20.0, 10.0, 2.0);
    let (doc, boss) = block(doc, 1.0, 1.0, 1.0);
    let bottom = face(&doc, plate, [10.0, 5.0, 0.0], [0.0, 0.0, -1.0]);
    let (doc, b) = fixture::in_world_by(
        doc,
        plate,
        Placed::unmated(vec![(Coord::Spin(Some(about)), fixture::ang(tilt))]).values,
    );
    let (doc, placement) = insert(
        doc,
        fixture::stood(editor_core::BodyRead::plain(Operand::from(boss))),
    );
    let top = fixture::fname(boss, RoleSeg::Cap(CapEnd::End));
    let mut mate = fixture::rest(head(top), head_at(b, bottom));
    mate.orient = orient;
    let placed = Placed {
        mates: vec![mate],
        values: vec![
            (Coord::Slide(TargetAxis::X), len(10.0)),
            (Coord::Slide(TargetAxis::Y), len(0.0)),
            (Coord::SPIN, fixture::ang(0.0)),
        ],
    };
    let (doc, _) = fixture::write_bundle(doc, placement, placed.bundle());
    (doc, placement)
}

/// The orientation putting the copy's `x` along `x` and its `z` along
/// `z`.
fn oriented(x: SignedAxis, z: SignedAxis) -> Orientation {
    Orientation::new(x, z).expect("x and z lie along two axes")
}

/// **The antipode rows: a boss hung under a tilted plate stays put.** The
/// plate turned by a tilt from 1e-12 to 0.1 about the world's `x` or `y`,
/// either sign: the boss's origin stays within `O(tilt)` of `x = +10`,
/// and the map is continuous through zero — each tilt's origin within
/// `O(tilt)` of the untilted one's, on both sides.
///
/// *Breaks if* the chart has a pole short of the orientation written
/// backwards: a chart turning the target's `+z` onto a mated direction
/// near its `−z` puts the boss at `x ≈ −10` for a tilt of 1e-9.
#[test]
fn a_boss_hung_under_a_tilted_plate_stays_within_its_tilt_of_its_place() {
    let level = hung(0.0, TargetAxis::Y, Orientation::IDENTITY).expect("the boss hangs level");
    assert!(
        (level.translation[0] - 10.0).abs() < 1e-12,
        "level, the boss stands at x = +10: {:?}",
        level.translation
    );
    for about in [TargetAxis::X, TargetAxis::Y] {
        for magnitude in [1e-12, 1e-9, 1e-6, 1e-3, 0.1] {
            for tilt in [magnitude, -magnitude] {
                let at = hung(tilt, about, Orientation::IDENTITY)
                    .unwrap_or_else(|f| panic!("{about:?} {tilt:e}: the boss hangs: {f}"));
                // The slide runs along x carried by the least turn, and
                // the boss's origin sits one unit below the plate's
                // bottom along the mated direction: each moves by at most
                // its lever times the tilt.
                assert!(
                    (at.translation[0] - 10.0).abs() <= 11.0 * tilt.abs() + 1e-12,
                    "{about:?} {tilt:e}: the boss stays near x = +10: {:?}",
                    at.translation
                );
                let moved = (0..3)
                    .map(|i| (at.translation[i] - level.translation[i]).abs())
                    .fold(0.0_f64, f64::max);
                assert!(
                    moved <= 11.0 * tilt.abs() + 1e-12,
                    "{about:?} {tilt:e}: continuous through zero, moved {moved:e}"
                );
            }
        }
    }
}

/// **The pole band refuses, naming the orientation to write.** The boss
/// hung under the plate with its orientation written backwards (its `z`
/// along `−z`, so the least turn would have to carry `−z` onto the mated
/// `+z`): at the plate's tilt of 0 and of 1e-12, inside the band about the
/// pole, the placement refuses `Backwards`, naming the orientation it
/// was written with and the identity to write; at a tilt of 0.1, clear of
/// the band, it is placed — by the least turn, nearly a half turn — and
/// does not refuse.
///
/// *Breaks if* the pole is answered (a half turn about an arbitrary
/// axis), a refusal names another orientation, or a tilt short of the
/// band refuses.
#[test]
fn the_pole_band_refuses_naming_the_orientation_to_write() {
    let backwards = oriented(SignedAxis::PlusX, SignedAxis::MinusZ);
    for tilt in [0.0, 1e-12] {
        match hung(tilt, TargetAxis::Y, backwards) {
            Err(MateFault::Backwards { orient, write, .. }) => {
                assert_eq!(orient, backwards, "{tilt:e}: names the orientation written");
                assert_eq!(
                    write,
                    Orientation::IDENTITY,
                    "{tilt:e}: names the one to write"
                );
                let said = MateFault::Backwards {
                    mate: editor_core::MateRef::new(
                        RecipeNodeId::WORLD,
                        editor_core::ConstraintId::placeholder(1),
                    ),
                    orient,
                    write,
                }
                .to_string();
                assert!(
                    said.contains("x: +x, z: +z"),
                    "{tilt:e}: the sentence names it: {said}"
                );
            }
            other => panic!("{tilt:e}: the pole band refuses Backwards, got {other:?}"),
        }
    }
    let clear = hung(0.1, TargetAxis::Y, backwards);
    assert!(
        clear.is_ok(),
        "a tilt clear of the band is placed: {clear:?}"
    );
}

/// **A floor swept through a quarter turn moves its copy continuously.**
/// A box resting on a floor turned about the world's `x` by every
/// degree from 0° to 90°, at values `(3, 4, 30°)`: from one degree to
/// the next the box's origin and axes move by at most the floor's own
/// lever on them — no jump anywhere on the sweep, the quarter turn,
/// where the floor is a wall, included.
///
/// *Breaks if* the chart has a branch short of the pole: a reference
/// axis chosen per direction jumps where its choice changes.
#[test]
fn a_floor_swept_through_a_quarter_turn_moves_its_copy_continuously() {
    let at = |degrees: f64| -> Frame {
        let doc = ProfileDoc::empty(DocumentId::derive("value-chart-sweep"), Tol::witness());
        let (doc, body) = block(doc, 1.0, 1.0, 1.0);
        let (doc, floor, _) = fixture::floor(
            doc,
            vec![(
                Coord::Spin(Some(TargetAxis::X)),
                fixture::ang(degrees.to_radians()),
            )],
        );
        let (doc, placement) = insert(
            doc,
            fixture::stood(editor_core::BodyRead::plain(Operand::from(body))),
        );
        let bottom = fixture::fname(body, RoleSeg::Cap(CapEnd::Start));
        let mate = fixture::mate(fixture::plane(head(bottom)), fixture::flip(floor));
        let placed = Placed {
            mates: vec![mate],
            values: vec![
                (Coord::Slide(TargetAxis::X), len(3.0)),
                (Coord::Slide(TargetAxis::Y), len(4.0)),
                (Coord::SPIN, fixture::ang(30.0_f64.to_radians())),
            ],
        };
        let (doc, _) = fixture::write_bundle(doc, placement, placed.bundle());
        solve(&doc, &EvalOptions::default(), Tol::witness())
            .placement(&doc, placement)
            .unwrap_or_else(|r| panic!("{degrees}°: the box rests on the floor: {r:?}"))
    };
    let step = 1.0_f64.to_radians();
    let mut last = at(0.0);
    for degree in 1..=90 {
        let now = at(f64::from(degree));
        let moved =
            (0..3)
                .map(|i| (now.translation[i] - last.translation[i]).abs())
                .chain((0..3).flat_map(|c| {
                    (0..3).map(move |r| (now.columns[c][r] - last.columns[c][r]).abs())
                }))
                .fold(0.0_f64, f64::max);
        // The origin sits at most five units from the floor's axis, so a
        // degree of the floor moves it by at most 5·step, and an axis by
        // at most 2·step.
        assert!(
            moved <= 6.0 * step,
            "{degree}°: the copy moved {moved:e} for one degree of the floor"
        );
        last = now;
    }
}

/// **A rest turns the same way whichever side spells its flip.** A unit
/// box rests on a level floor at values `(3, 4, 0.5)`, its contact
/// spelled twice: the flip on the floor's plane (as the viewer's mate
/// tool and Python's `Mate` docs write it), and the flip on the box's
/// bottom (as sketch-on-face writes it). Both stand the box at one pose,
/// and the positive spin turns it counter-clockwise seen from `+z`: its
/// `x` lies at `(cos 0.5, sin 0.5, 0)`.
///
/// *Breaks if* the spin turns about the mated direction with the flip
/// included, which points down for the floor-side flip and turns that
/// box clockwise.
#[test]
fn a_rest_spins_the_same_way_whichever_side_carries_its_flip() {
    let spin = 0.5_f64;
    let at = |copy_side: bool| -> Frame {
        let doc = ProfileDoc::empty(DocumentId::derive("value-chart-flip-side"), Tol::witness());
        let (doc, body) = block(doc, 1.0, 1.0, 1.0);
        let (doc, floor, _) = fixture::floor(doc, Vec::new());
        let (doc, placement) = insert(
            doc,
            fixture::stood(editor_core::BodyRead::plain(Operand::from(body))),
        );
        let bottom = fixture::plane(head(fixture::fname(body, RoleSeg::Cap(CapEnd::Start))));
        let mate = if copy_side {
            fixture::mate(fixture::flip(bottom), floor)
        } else {
            fixture::mate(bottom, fixture::flip(floor))
        };
        let placed = Placed {
            mates: vec![mate],
            values: vec![
                (Coord::Slide(TargetAxis::X), len(3.0)),
                (Coord::Slide(TargetAxis::Y), len(4.0)),
                (Coord::SPIN, fixture::ang(spin)),
            ],
        };
        let (doc, _) = fixture::write_bundle(doc, placement, placed.bundle());
        solve(&doc, &EvalOptions::default(), Tol::witness())
            .placement(&doc, placement)
            .unwrap_or_else(|r| panic!("copy side {copy_side}: the box rests: {r:?}"))
    };
    let (floor_side, copy_side) = (at(false), at(true));
    for c in 0..3 {
        for r in 0..3 {
            assert!(
                (floor_side.columns[c][r] - copy_side.columns[c][r]).abs() < 1e-12,
                "one turn whichever side flips: {floor_side:?} vs {copy_side:?}"
            );
        }
        assert!(
            (floor_side.translation[c] - copy_side.translation[c]).abs() < 1e-12,
            "one place whichever side flips: {floor_side:?} vs {copy_side:?}"
        );
    }
    let x = floor_side.columns[0];
    assert!(
        (x[0] - spin.cos()).abs() < 1e-12
            && (x[1] - spin.sin()).abs() < 1e-12
            && x[2].abs() < 1e-12,
        "a positive spin turns the box counter-clockwise seen from +z: x = {x:?}"
    );
}

/// **A document written before a mate carried its orientation refuses
/// at load, naming it.** The hung boss's document saved, and every
/// `orient` taken off its mates — the shape a build before the
/// orientation wrote: the load refuses `Unreadable`, its detail naming
/// the missing field, its recourse once.
///
/// *Breaks if* a mate without its orientation loads as the identity,
/// silently placing an older document's copies by a chart it was not
/// written for.
#[test]
fn a_document_whose_mates_carry_no_orientation_refuses_naming_it() {
    use editor_core::{PersistError, REGENERATE_RECOURSE, load, save};
    fn strip(v: &mut serde_json::Value) -> usize {
        match v {
            serde_json::Value::Object(map) => {
                let here = usize::from(map.remove("orient").is_some());
                here + map.values_mut().map(strip).sum::<usize>()
            }
            serde_json::Value::Array(items) => items.iter_mut().map(strip).sum(),
            _ => 0,
        }
    }
    let (doc, _) = hung_doc(0.0, TargetAxis::Y, Orientation::IDENTITY);
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let (header, body) = text.split_once('\n').expect("a header line");
    let mut v: serde_json::Value = serde_json::from_str(body).expect("the body is JSON");
    assert!(strip(&mut v) > 0, "the save carries the mate's orientation");
    let older = format!(
        "{header}\n{}",
        serde_json::to_string_pretty(&v).expect("writes")
    );
    match load(&older, Tol::witness()) {
        Err(err @ PersistError::Unreadable { .. }) => {
            let msg = err.to_string();
            assert_eq!(msg.matches(REGENERATE_RECOURSE).count(), 1, "{msg}");
            let PersistError::Unreadable { detail, .. } = err else {
                unreachable!()
            };
            assert!(
                detail.contains("missing field `orient`"),
                "the refusal names the field: {detail}"
            );
        }
        other => panic!("a mate with no orientation refuses unreadable, got {other:?}"),
    }
}
