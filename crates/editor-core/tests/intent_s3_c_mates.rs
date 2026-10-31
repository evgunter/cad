//! **INTENT stage 3 PR C — a mate relates two poses**
//! (`docs/INTENT-STAGE3-SPEC.md` §4 and §10 rows 8–9).
//!
//! The coset rows build the representatives stage 3 A owed: a point's
//! (`Spherical`), a direction's (`Parallel`) and the pure translation
//! groups meet every family in the table, and the first mate that folds
//! a point or a direction folds them.
//!
//! The value rows (§10 row 9, rulings C-1 to C-3) place a box by a mate
//! and its values: the chart each value is read on, which faults the
//! door refuses and which the evaluation does, the "place part" gesture
//! and the migration's pin.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::result_large_err
)]

use crate::fixture::{self, ang, len};
use editor_core::Dimension;
use editor_core::mate::coset::{Coset, Subgroup, intersect};
use editor_core::pose::{PoseDef, RoleRead};
use editor_core::{
    BodyRead, Coord, DocEdit, EditError, EvalOptions, Formula, Mate, MateFault, Node, NodeResult,
    Operand, ProfileDoc, RecipeNodeId, StableName, TargetAxis,
};
use geom_core::Tol;
use geom_core::linalg::{Affine3, Mat3, Point3, UnitVec3, Vec3};
use geom_core::predicate::Band;

fn band() -> Band {
    Band::linear(Tol::witness()).expect("a band")
}

fn unit(v: Vec3<f64>) -> UnitVec3<f64> {
    UnitVec3::new(v, topo::query::POSE_UNIT_NORM, band()).expect("a direction")
}

/// A deterministic stream of numbers in `[-1, 1)`, so a row's failure
/// names the draw that broke it.
struct Draw(u64);

impl Draw {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        #[allow(clippy::cast_precision_loss)]
        let x = (self.0 >> 11) as f64 / (1_u64 << 53) as f64;
        2.0 * x - 1.0
    }

    fn vec(&mut self) -> Vec3<f64> {
        Vec3::new(self.next(), self.next(), self.next())
    }

    fn point(&mut self) -> Point3<f64> {
        Point3::origin() + self.vec() * 2.0
    }

    fn direction(&mut self) -> UnitVec3<f64> {
        unit(self.vec() + Vec3::new(0.0, 0.0, 0.1))
    }

    fn angle(&mut self) -> f64 {
        self.next() * 3.0
    }

    fn rigid(&mut self) -> Affine3<f64> {
        let axis = self.direction().get();
        Affine3::from_parts(Mat3::rotation_about(axis, self.angle()), self.vec() * 2.0)
    }
}

/// An element of `g`, drawn: what a coset of `g` may differ from its
/// representative by.
fn element(g: Subgroup<f64>, d: &mut Draw) -> Affine3<f64> {
    let turn = |point: Point3<f64>, axis: Vec3<f64>, angle: f64| {
        Affine3::rotation_about_axis(point, axis, angle)
    };
    let shift = |v: Vec3<f64>| Affine3::from_parts(Mat3::identity(), v);
    match g {
        Subgroup::Se3 => d.rigid(),
        Subgroup::Planar { normal } => {
            shift(d.vec().reject_from(normal.get())) * turn(d.point(), normal.get(), d.angle())
        }
        Subgroup::Cylindrical { point, direction } => {
            shift(direction.get() * d.next()) * turn(point, direction.get(), d.angle())
        }
        Subgroup::Prismatic { direction } => shift(direction.get() * d.next()),
        Subgroup::Revolute { point, direction } => turn(point, direction.get(), d.angle()),
        Subgroup::Spherical { point } => turn(point, d.direction().get(), d.angle()),
        Subgroup::Parallel { direction } => {
            shift(d.vec()) * turn(d.point(), direction.get(), d.angle())
        }
        Subgroup::Translation => shift(d.vec()),
        Subgroup::PlaneTranslation { normal } => shift(d.vec().reject_from(normal.get())),
        Subgroup::Trivial => Affine3::identity(),
    }
}

/// The ten proper families, each at generic parameters drawn from `d`.
fn generic(d: &mut Draw) -> Vec<Subgroup<f64>> {
    vec![
        Subgroup::Se3,
        Subgroup::Planar {
            normal: d.direction(),
        },
        Subgroup::Cylindrical {
            point: d.point(),
            direction: d.direction(),
        },
        Subgroup::Prismatic {
            direction: d.direction(),
        },
        Subgroup::Revolute {
            point: d.point(),
            direction: d.direction(),
        },
        Subgroup::Spherical { point: d.point() },
        Subgroup::Parallel {
            direction: d.direction(),
        },
        Subgroup::Translation,
        Subgroup::PlaneTranslation {
            normal: d.direction(),
        },
        Subgroup::Trivial,
    ]
}

/// **Two cosets sharing a pose meet where their families meet
/// transversally, and nowhere else**, in either order: each coset is
/// `g·h⁻¹·x*` for a drawn element `h` of `g`, so `x*` lies in both.
/// Where the families meet transversally (SE(3) with anything, a ball
/// with a direction or the translations) the coset is constructed, its
/// family the meet's, and it holds `x*`; every other pair the bundle
/// rule refuses, and
/// [`intersect`] answers none.
///
/// *Breaks if* a transverse pair has no representative, or one that
/// leaves `x*` out (a ball's point carried elsewhere), or a pair that
/// fixes an equation twice is met.
#[test]
fn every_pair_of_families_sharing_a_pose_meets_only_where_transverse() {
    let mut d = Draw(0x9e37_79b9_7f4a_7c15);
    let mut pairs = Vec::new();
    for _ in 0..4 {
        let row = generic(&mut d);
        let col = generic(&mut d);
        for g1 in &row {
            for g2 in &col {
                pairs.push((*g1, *g2));
            }
        }
    }
    let mut met_any = 0;
    for (i, (g1, g2)) in pairs.into_iter().enumerate() {
        let x = d.rigid();
        let h1 = element(g1, &mut d);
        let h2 = element(g2, &mut d);
        let held = Coset {
            subgroup: g1,
            representative: h1.inverse() * x,
        };
        let added = Coset {
            subgroup: g2,
            representative: h2.inverse() * x,
        };
        let at = format!("pair {i}: {} against {}", g1.name(), g2.name());
        let transverse = g1.family().transverse(g2.family());
        let Some(met) = intersect(held, added) else {
            assert!(transverse.is_none(), "{at}: a transverse pair was not met");
            continue;
        };
        assert_eq!(Some(met.subgroup.family()), transverse, "{at}");
        // `x*` lies in the met coset: `x*·rep⁻¹` is an element of its
        // subgroup — for a ball and a direction's revolute one fixing
        // its point and its direction, for a ball and the translations'
        // trivial group the identity.
        let h = x * met.representative.inverse();
        match met.subgroup {
            Subgroup::Revolute { point, direction } => {
                assert!(
                    (h.transform_point(point) - point).norm() < 1e-9
                        && (h.linear * direction.get() - direction.get()).norm() < 1e-9,
                    "{at}: the coset leaves the shared pose out"
                );
                met_any += 1;
            }
            Subgroup::Trivial if g1.family() != g2.family() => {
                let p = Point3::new(1.0, 2.0, 3.0);
                assert!(
                    (h.transform_point(p) - p).norm() < 1e-9,
                    "{at}: the coset leaves the shared pose out"
                );
            }
            _ => {}
        }
    }
    assert!(
        met_any >= 2,
        "a ball and a direction met, both orders: {met_any}"
    );
}

// ---- Values ----

/// A box `size` on a side, extruded from the world xy plane, its own
/// coordinates the world's: the shape a copy is placed from.
fn a_box(id: &str, size: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = fixture::on_frame(
        ProfileDoc::empty_derived(id, Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (size, 0.0), (size, size), (0.0, size)]],
    );
    fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(size),
        },
    )
}

/// The face of `body` whose outward normal is `n`.
fn face_toward(doc: &ProfileDoc, body: RecipeNodeId, n: [f64; 3]) -> StableName {
    let ev = fixture::run(doc, &EvalOptions::default());
    editor_core::all_faces(&ev, body)
        .into_iter()
        .find(|name| {
            let at = editor_core::face_frame(&ev, body, name).expect("a face's frame");
            let s = if at.sense { 1.0 } else { -1.0 };
            let normal = at.axis * s;
            (normal - Vec3::new(n[0], n[1], n[2])).norm() < 1e-9
        })
        .unwrap_or_else(|| panic!("no face of the box faces {n:?}"))
}

/// The edge of `body` running along `dir` through `through`.
fn edge_along(
    doc: &ProfileDoc,
    body: RecipeNodeId,
    through: [f64; 3],
    dir: [f64; 3],
) -> StableName {
    let ev = fixture::run(doc, &EvalOptions::default());
    editor_core::all_edges(&ev, body)
        .into_iter()
        .find(|name| {
            let Ok(at) = editor_core::edge_frame(&ev, body, name) else {
                return false;
            };
            let d = Vec3::new(dir[0], dir[1], dir[2]);
            let off = at.origin - Point3::new(through[0], through[1], through[2]);
            at.axis.cross(d).norm() < 1e-9 && off.cross(d).norm() < 1e-9
        })
        .unwrap_or_else(|| panic!("no edge of the box along {dir:?} through {through:?}"))
}

fn on_body(body: RecipeNodeId, name: StableName) -> Operand {
    Operand::select(Operand::output(body, 0), vec![name])
}

fn pose(def: PoseDef<Operand, Formula>) -> Operand {
    Operand::Pose(Box::new(def))
}

fn plane_of(face: Operand) -> Operand {
    pose(PoseDef::Plane { face })
}

/// `pose` spun about `about` by `turn` radians.
fn spun(pose_: Operand, about: Operand, turn: f64) -> Operand {
    pose(PoseDef::Spin {
        pose: RoleRead::plain(pose_),
        about: RoleRead::plain(about),
        by: ang(turn),
    })
}

/// A loose placement of `body`.
fn placed(doc: ProfileDoc, body: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    fixture::insert(
        doc,
        crate::fixture::stood(BodyRead::plain(Operand::output(body, 0))),
    )
}

/// The top of a floor turned to face `−z` by a half-turn about the
/// world's `x` — a plane facing down with no flip in its spelling.
fn floor_facing_down(doc: ProfileDoc) -> (ProfileDoc, Operand) {
    let (doc, top, _) = fixture::floor(
        doc,
        vec![(Coord::Spin(Some(TargetAxis::X)), ang(std::f64::consts::PI))],
    );
    (doc, top)
}

/// The world's `xy`, facing down: a floor's top, flipped, which a box's
/// bottom face rests on.
fn floor_flipped(doc: ProfileDoc) -> (ProfileDoc, Operand) {
    let (doc, top, _) = fixture::floor(doc, Vec::new());
    (doc, fixture::flip(top))
}

/// **`mate` written onto `placement` whole, its `values` in the order
/// given**, then the constant `0` on each freedom they leave
/// ([`fixture::squared`]): every bundle pins its copy.
fn seat(
    doc: ProfileDoc,
    placement: RecipeNodeId,
    mate: Mate<Formula>,
    values: impl IntoIterator<Item = (Coord, Formula)>,
) -> ProfileDoc {
    let mut values: Vec<(Coord, Formula)> = values.into_iter().collect();
    let full = fixture::squared(&doc, mate.clone(), &values);
    for (coord, by) in full.values {
        if !values.iter().any(|(at, _)| *at == coord) {
            values.push((coord, by));
        }
    }
    let placed = editor_core::Placed {
        mates: vec![mate],
        values,
    };
    fixture::write_bundle(doc, placement, placed.bundle()).0
}

/// The value's number for `coord`: a length or an angle.
fn number(coord: Coord, v: f64) -> Formula {
    match coord {
        Coord::Slide(_) => len(v),
        Coord::SPIN | Coord::Spin(_) => ang(v),
    }
}

/// `placement`'s copy where its bundle solved it, in its space.
fn solved(doc: &ProfileDoc, placement: RecipeNodeId) -> Affine3<f64> {
    let poses = editor_core::solve_document(doc, &EvalOptions::default(), Tol::witness());
    if let Some(fault) = poses.fault(placement) {
        panic!("placement {placement:?} refused: {fault}");
    }
    poses
        .relative_map(placement)
        .expect("the placement's copy has a pose")
}

/// The angle of the turn `a·bᵀ`, as `atan2(sin, cos)` from its skew
/// part and its trace: an `acos` of the trace alone resolves no turn
/// below about 1.5e-8 rad, where the cosine rounds to one.
fn turn_between(a: Mat3<f64>, b: Mat3<f64>) -> f64 {
    let r = a * b.transpose();
    let cos = (r.c0.x + r.c1.y + r.c2.z - 1.0) / 2.0;
    let sin = Vec3::new(r.c1.z - r.c2.y, r.c2.x - r.c0.z, r.c0.y - r.c1.x).norm() / 2.0;
    sin.atan2(cos)
}

/// A box resting on the world floor by its bottom face, with values
/// `(slide x, slide y, spin)` written in `order`.
fn box_on_the_floor(values: [f64; 3], order: [usize; 3]) -> (ProfileDoc, RecipeNodeId) {
    box_resting_on(Vec::new(), values, order)
}

/// A box resting by its bottom face on the top of a floor placed by
/// `moves` (the world's `xy` moved by them, [`fixture::floor`]),
/// flipped, with values `(slide x, slide y, spin)` written in `order`.
fn box_resting_on(
    moves: Vec<(Coord, Formula)>,
    values: [f64; 3],
    order: [usize; 3],
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, body) = a_box("s3c_floor", 1.0);
    let bottom = face_toward(&doc, body, [0.0, 0.0, -1.0]);
    let (doc, placement) = placed(doc, body);
    let (doc, top, _) = fixture::floor(doc, moves);
    let written = Mate {
        on: plane_of(on_body(body, bottom)),
        to: fixture::flip(top),
        orient: editor_core::Orientation::IDENTITY,
    };
    let coords = [
        Coord::Slide(TargetAxis::X),
        Coord::Slide(TargetAxis::Y),
        Coord::SPIN,
    ];
    let doc = seat(
        doc,
        placement,
        written,
        order.map(|k| (coords[k], number(coords[k], values[k]))),
    );
    (doc, placement)
}

/// **Row 9 — a plane mate leaves two slides and a spin.** A box rests
/// on the world floor by its bottom face (a rest carries a flip on the
/// target, so the mated direction is the floor's flipped normal `−z`,
/// where the identity orientation puts the box's bottom: the least turn
/// is the identity). At zero values the box's own coordinates agree with
/// the world's; at `(3, 4, 30°)` its origin stands at `(3, 4, 0)` turned
/// 30° about `z`, the floor's own normal, whichever side the flip is
/// on. Every order of the three values places it alike.
///
/// *Breaks if* a slide is charted along the other slide's moved axis
/// (order-dependent), the spin is measured about another axis than the
/// floor's own normal, or the least turn is not the identity where the
/// orientation puts the face on the floor.
#[test]
fn a_plane_mate_leaves_two_slides_and_a_spin_in_any_order() {
    let orders = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    for order in orders {
        let (doc, p) = box_on_the_floor([0.0, 0.0, 0.0], order);
        let at = solved(&doc, p);
        assert!(
            turn_between(at.linear, Mat3::identity()) < 1e-12 && at.translation.norm() < 1e-12,
            "{order:?}: zero values agree with the world: {at:?}"
        );
    }
    let turn = 30.0_f64.to_radians();
    let expected = Mat3::rotation_about(Vec3::new(0.0, 0.0, 1.0), turn);
    let mut first: Option<Affine3<f64>> = None;
    for order in orders {
        let (doc, p) = box_on_the_floor([3.0, 4.0, turn], order);
        let at = solved(&doc, p);
        assert!(
            turn_between(at.linear, expected) < 1e-12,
            "{order:?}: turned by the spin about z: {at:?}"
        );
        assert!(
            (at.translation - Vec3::new(3.0, 4.0, 0.0)).norm() < 1e-12,
            "{order:?}: slid to (3, 4, 0): {at:?}"
        );
        if let Some(held) = first {
            assert_eq!(
                format!("{held:?}"),
                format!("{at:?}"),
                "{order:?}: the order of the values moves no bit"
            );
        }
        first.get_or_insert(at);
    }
}

/// **Every listing order places the copy to the bit, on a tilted
/// target too.** A floor spun 0.3 about `x` and 0.7 about `z`, its
/// top flipped: values `(3.3, 4.7, 0.9)` in all six orders give one
/// map, bit for bit — the values are taken in one order whatever order
/// they are listed in.
///
/// *Breaks if* the solve takes the values in their listed order (a
/// tilted chart's slides round apart in the last bits).
#[test]
fn every_order_places_the_copy_to_the_bit_on_a_tilted_target() {
    let tilted = vec![
        (Coord::Spin(Some(TargetAxis::X)), ang(0.3)),
        (Coord::Spin(Some(TargetAxis::Z)), ang(0.7)),
    ];
    let orders = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    let maps: Vec<String> = orders
        .iter()
        .map(|&order| {
            let (doc, p) = box_resting_on(tilted.clone(), [3.3, 4.7, 0.9], order);
            format!("{:?}", solved(&doc, p))
        })
        .collect();
    assert!(
        maps.iter().all(|m| *m == maps[0]),
        "one map whatever the order: {maps:#?}"
    );
}

/// **A freedom set twice refuses at the door, whatever else the bundle
/// leaves.** A box's bundle resting it on the floor with two slides
/// along `x` on the one mate: the second sets the freedom the first
/// already sets, so the door refuses the bundle, naming both, though
/// the slide along `y` and the spin are still free (FORK-S3O).
///
/// *Breaks if* a value is admitted beside one of its own freedom, or
/// the door names only the later one.
#[test]
fn a_slide_set_twice_refuses_though_a_turn_is_free() {
    let (doc, body) = a_box("s3c_twice", 1.0);
    let bottom = face_toward(&doc, body, [0.0, 0.0, -1.0]);
    let (doc, placement) = placed(doc, body);
    let (doc, floor) = floor_flipped(doc);
    let written = Mate {
        on: plane_of(on_body(body, bottom)),
        to: floor,
        orient: editor_core::Orientation::IDENTITY,
    };
    let twice = editor_core::Placed {
        mates: vec![written],
        values: vec![
            (Coord::Slide(TargetAxis::X), len(1.0)),
            (Coord::Slide(TargetAxis::X), len(2.0)),
        ],
    };
    match doc.apply(&twice.write(placement), Tol::witness()) {
        Err(EditError::MateRefused { fault, .. }) => assert!(
            matches!(
                &*fault,
                MateFault::Overconstrained { placement: p, constraints, free: 2 }
                    if *p == placement && constraints.len() == 2
            ),
            "{fault}"
        ),
        other => panic!("the door refuses the second slide: {other:?}"),
    }
}

/// The box placed against a wall facing `+x` by its `x`-face spun by
/// `draft` about the edge along `y` at its foot, and by `roll` about
/// the edge along `z` — a drafted wall, and a prism's flat at angle
/// `roll` — holding a spin of 30° and zero slides.
fn box_on_the_wall(draft: f64, roll: f64) -> Affine3<f64> {
    let (doc, body) = a_box("s3c_wall", 1.0);
    let wall = face_toward(&doc, body, [1.0, 0.0, 0.0]);
    let foot = edge_along(&doc, body, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let upright = edge_along(&doc, body, [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let face = spun(
        spun(
            plane_of(on_body(body, wall)),
            pose(PoseDef::Axis {
                of: on_body(body, foot),
            }),
            draft,
        ),
        pose(PoseDef::Axis {
            of: on_body(body, upright),
        }),
        roll,
    );
    let (doc, placement) = placed(doc, body);
    // The world's `yz`, facing `+x`: a floor turned a quarter turn about
    // the world's `y`.
    let (doc, wall, _) = fixture::floor(
        doc,
        vec![(
            Coord::Spin(Some(TargetAxis::Y)),
            Formula::turn_over(4).expect("a quarter turn"),
        )],
    );
    let written = Mate {
        on: face,
        to: wall,
        orient: editor_core::Orientation::IDENTITY,
    };
    let doc = doc;
    let doc = seat(
        doc,
        placement,
        written,
        // The wall faces `+x`, which the mate takes: the slides across it.
        [
            (
                Coord::Slide(TargetAxis::Y),
                Formula::zero(Dimension::Length),
            ),
            (
                Coord::Slide(TargetAxis::Z),
                Formula::zero(Dimension::Length),
            ),
            (Coord::SPIN, ang(30.0_f64.to_radians())),
        ],
    );
    solved(&doc, placement)
}

/// **Ruling C-1 — the chart is continuous where a face is drafted.** A
/// box's wall facing `+x`, drafted by 0°, 1° and −1°, mated to the
/// world's `+x` plane under a held spin of 30°: the least turn carries
/// the copy's oriented mated direction onto the wall's normal, so the
/// copy turns by about the draft and no more — no flip, no jump.
///
/// *Breaks if* the reference is a projection that changes branch at
/// zero draft (a target `x` projected, a `y` taken when parallel): the
/// copy would turn by a right angle or a half-turn between the drafts.
#[test]
fn a_drafted_wall_under_a_held_spin_turns_by_its_draft() {
    let level = box_on_the_wall(0.0, 0.0);
    for draft in [1.0_f64, -1.0] {
        let tilted = box_on_the_wall(draft.to_radians(), 0.0);
        let turned = turn_between(tilted.linear, level.linear);
        assert!(
            turned <= draft.abs().to_radians() * 1.000_001,
            "a {draft}° draft turns the copy by {}°",
            turned.to_degrees()
        );
    }
}

/// **Ruling C-1 — a prism's flat through angle zero.** The flat's
/// normal turned by `φ` about `z` through −1°, 0° and 1°, under a held
/// spin of 30°: the copy turns by a turn proportional to `φ` (the
/// flat's own and the spin re-measured about the turned normal),
/// continuously through 0.
///
/// *Breaks if* the chart has a branch at `φ = 0` (the
/// projection fork's), where the copy would jump.
#[test]
fn a_prisms_flat_through_zero_turns_by_its_angle() {
    let level = box_on_the_wall(0.0, 0.0);
    for roll in [1.0_f64, -1.0] {
        let rolled = box_on_the_wall(0.0, roll.to_radians());
        let turned = turn_between(rolled.linear, level.linear);
        // The flat's turn and the spin's re-measure about the new normal
        // compose to √2·φ at a small φ: proportional to φ, so no jump.
        assert!(
            turned <= 2.0 * roll.abs().to_radians(),
            "a flat at {roll}° turns the copy by {}°",
            turned.to_degrees()
        );
    }
}

/// **A cap facing `−z` placed on a floor facing `−z`.** The identity
/// orientation puts the cap where the mate does, so the least turn is
/// the identity: the box's own `x` meets the world's, and zero values put
/// it where it is. The cap read at a rounding-level tilt (spun by
/// `sin π`, a half-revolve's end-face rounding) is a rounding-level least
/// turn, never refused: `−z` is no pole of the chart.
///
/// *Breaks if* the chart has a pole short of the orientation written
/// backwards (the evaluation refuses), or the least turn is not the
/// identity where `n = d` (the copy turns).
#[test]
fn a_cap_at_the_antipode_places_without_a_turn() {
    let (doc, body) = a_box("s3c_cap", 1.0);
    let bottom = face_toward(&doc, body, [0.0, 0.0, -1.0]);
    let (doc, placement) = placed(doc, body);
    let (doc, down) = floor_facing_down(doc);
    let written = Mate {
        on: plane_of(on_body(body, bottom)),
        to: down,
        orient: editor_core::Orientation::IDENTITY,
    };
    let doc = seat(
        doc,
        placement,
        written,
        [
            Coord::Slide(TargetAxis::X),
            Coord::Slide(TargetAxis::Y),
            Coord::SPIN,
        ]
        .map(|coord| (coord, number(coord, 0.0))),
    );
    let at = solved(&doc, placement);
    assert!(
        turn_between(at.linear, Mat3::identity()) < 1e-12 && at.translation.norm() < 1e-12,
        "the identity least turn keeps x, and zero values put the box where it is: {at:?}"
    );

    // The same cap read at a rounding-level tilt: spun about the foot
    // edge along x by an angle whose sine is sin π, the rounding a
    // half-revolve's end face carries.
    let (doc, body) = a_box("s3c_cap_tilted", 1.0);
    let bottom = face_toward(&doc, body, [0.0, 0.0, -1.0]);
    let along_x = edge_along(&doc, body, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let tilted = spun(
        plane_of(on_body(body, bottom)),
        pose(PoseDef::Axis {
            of: on_body(body, along_x),
        }),
        std::f64::consts::PI.sin(),
    );
    let (doc, placement) = placed(doc, body);
    let (doc, down) = floor_facing_down(doc);
    let written = Mate {
        on: tilted,
        to: down,
        orient: editor_core::Orientation::IDENTITY,
    };
    let doc = seat(
        doc,
        placement,
        written,
        [(Coord::SPIN, Formula::zero(Dimension::Angle))],
    );
    let ev = fixture::run(&doc, &EvalOptions::default());
    assert!(
        !matches!(ev.result(placement), Some(NodeResult::Failed(_))),
        "a rounding-level tilt is a rounding-level least turn, never refused: {:?}",
        ev.result(placement)
    );
}

/// **A mate taken out frees what it fixed, and a value taken out is
/// its freedom written back to `0`** (`DocEdit::RemoveConstraint`, the
/// bundle written whole again): the box on the floor loses its plane
/// mate and keeps its two slides, now on the world's axes, the lift and
/// the chain's three turns written the constant `0` — its spin was the
/// turn about the floor's own normal, which no chain link is; a value taken
/// out leaves the mate and the other two, and the constant `0` in its
/// place. Either way the copy stays pinned.
///
/// *Breaks if* a removal leaves a bundle short of pinning its copy, or
/// drops a value whose freedom the remaining mates still leave.
#[test]
fn a_constraint_taken_out_leaves_its_bundle_pinned() {
    use editor_core::TargetAxis::{X, Y, Z};
    let (doc, p) = box_on_the_floor([0.0, 0.0, 0.5], [2, 0, 1]);
    let bundle = fixture::bundle(&doc, p);
    let mate = bundle
        .iter()
        .find(|c| c.mate().is_some())
        .expect("a mate")
        .id;
    let remove = |doc: &ProfileDoc, constraint| {
        doc.apply(
            &DocEdit::RemoveConstraint {
                placement: p,
                constraint,
            },
            Tol::witness(),
        )
        .unwrap_or_else(|refusal| panic!("the bundle is written whole again: {refusal}"))
        .doc
    };
    let coords = |doc: &ProfileDoc| -> Vec<Coord> {
        fixture::bundle(doc, p)
            .iter()
            .filter_map(|c| c.value().map(|v| v.coord))
            .collect()
    };
    let freed = remove(&doc, mate);
    assert!(
        fixture::bundle(&freed, p)
            .iter()
            .all(|c| c.mate().is_none()),
        "the mate went"
    );
    assert_eq!(
        coords(&freed),
        [
            Coord::Slide(X),
            Coord::Slide(Y),
            Coord::Slide(Z),
            Coord::Spin(Some(X)),
            Coord::Spin(Some(Y)),
            Coord::Spin(Some(Z)),
        ],
        "the values it left are kept, and 0 on the rest"
    );
    let value = bundle
        .iter()
        .find(|c| c.value().is_some())
        .expect("a value")
        .id;
    let zeroed = remove(&doc, value);
    assert_eq!(
        fixture::bundle(&zeroed, p).len(),
        4,
        "the mate and three values"
    );
    assert!(
        fixture::bundle(&zeroed, p).iter().any(|c| c.id == mate),
        "the mate stays"
    );
}

/// **Values on the world's axes reproduce a pose.** A bundle with no
/// mate whose spins about `x`, `y` and `z` are the pose's angles in that
/// order, and whose slides are its origin, stands the copy at `P`.
///
/// *Breaks if* the chain's spins compose in another order than they are
/// written in.
#[test]
fn the_migration_pin_reproduces_its_pose() {
    let (doc, body) = a_box("s3c_pinned", 1.0);
    let (doc, placement) = placed(doc, body);
    let turn = Mat3::rotation_about(Vec3::new(-2.0, 0.5, 1.0), 2.1);
    let shift = Vec3::new(-3.0, 0.5, 7.0);
    let pose = editor_core::Frame::from_affine(Affine3::from_parts(turn, shift));
    let doc = fixture::pin(doc, placement, &pose, &EvalOptions::default());
    let at = solved(&doc, placement);
    assert!(
        turn_between(at.linear, turn) < 1e-12 && (at.translation - shift).norm() < 1e-12,
        "the copy stands at P: {at:?}"
    );
}

/// **The unobservable-value report** (the folded issue
/// `an-ineffective-pose-move-is-not-reported-unobservable`). A plane
/// slid along a direction lying in it, and a spin value about the axis
/// a cylinder is round about, are named; a slide along the plane's
/// normal and a box's spin are not.
///
/// *Breaks if* the report names a number that moves something, or
/// misses a move inside its pose's own symmetry.
#[test]
fn a_number_that_moves_nothing_is_reported() {
    let (doc, body) = a_box("s3c_unobservable", 1.0);
    let top = face_toward(&doc, body, [0.0, 0.0, 1.0]);
    let foot = edge_along(&doc, body, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let slide = |along: Operand| {
        pose(PoseDef::Slide {
            pose: RoleRead::plain(plane_of(on_body(body, top.clone()))),
            along: RoleRead::plain(along),
            by: len(0.25),
        })
    };
    let in_plane = slide(pose(PoseDef::Axis {
        of: on_body(body, foot),
    }));
    let across = slide(pose(PoseDef::Axis {
        of: on_body(
            body,
            edge_along(&doc, body, [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        ),
    }));
    // Each move is written where a split reads its tool, which holds it
    // as a pose variable.
    let tool = |doc: ProfileDoc, def: Operand| {
        let (doc, split) = fixture::insert(
            doc,
            Node::Split {
                target: Operand::output(body, 0).into(),
                tool: RoleRead::plain(def),
            },
        );
        let Some(Node::Split { tool, .. }) = doc.node(split) else {
            unreachable!("a split was inserted")
        };
        let var = tool.read;
        (doc, var)
    };
    let (doc, quiet) = tool(doc, in_plane);
    let (doc, loud) = tool(doc, across);
    let ev = fixture::run(&doc, &EvalOptions::default());
    let report = editor_core::unobservable(&doc, &ev, Tol::witness());
    let named = |var| {
        report
            .iter()
            .any(|u| matches!(u, editor_core::Unobservable::PoseMove { pose, .. } if *pose == var))
    };
    assert!(
        named(quiet),
        "a slide inside the plane moves nothing: {report:?}"
    );
    assert!(
        !named(loud),
        "a slide along the normal moves the plane: {report:?}"
    );
}

/// **Inline carries a moving landing for a value charted on the part's
/// world** (ASSEMBLY A4). A part whose box rests on a floor by a mate
/// and three values, all numbers, instanced in a host and placed off the
/// identity: inline copies the part verbatim into a space of its own and
/// places it under the instance's bundle, so the inlined host's world
/// is the one it had. Placed at the identity, the same instance inlines
/// too.
///
/// *Breaks if* the inline composes the landing into the part's values
/// (the copy would land at its numbers in the host's world, not moved
/// with the rest).
#[test]
fn inline_moves_a_landing_under_a_world_value_with_its_instance() {
    let (part, _) = box_on_the_floor([0.5, 0.0, 0.0], [0, 1, 2]);
    let mut store = fixture::resolver::PartStore::new();
    let part = crate::fixture::assemble_world_copies(part);
    let part_ref = store.insert(part, Tol::witness());
    let o = fixture::resolver::with_resolver(store.clone());
    let resolver: std::sync::Arc<dyn editor_core::PartResolver> = std::sync::Arc::new(store);
    let host = ProfileDoc::empty_derived("s3c_inline_host", Tol::witness());
    let (host, instance) = fixture::insert(host, crate::fixture::mated_instance(part_ref));
    // The world's vertices, sorted: a re-pin re-composes the pose, so
    // the copy stands where it stood to the rounding of that one step.
    let world = |doc: &ProfileDoc, o: &EvalOptions| {
        let body = crate::fixture::product_in_world(doc, &fixture::run(doc, o), Tol::witness())
            .expect("the world gathers");
        let mut points: Vec<[f64; 3]> =
            body.vertex_points().map(|(_, p)| [p.x, p.y, p.z]).collect();
        points.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
        points
    };
    let same = |a: Vec<[f64; 3]>, b: Vec<[f64; 3]>| {
        a.len() == b.len()
            && a.iter()
                .zip(&b)
                .all(|(p, q)| p.iter().zip(q).all(|(x, y)| (x - y).abs() <= 1e-12))
    };
    let moved = fixture::pin(
        host.clone(),
        instance,
        &editor_core::Frame::translation([3.0, 0.0, 0.0]),
        &o,
    );
    let inlined = editor_core::inline(&moved, instance, &resolver, Tol::witness())
        .unwrap_or_else(|e| panic!("a moving landing under a world value inlines: {e}"));
    assert!(
        same(
            world(&inlined.doc, &EvalOptions::default()),
            world(&moved, &o)
        ),
        "the copy moved with the instance"
    );
    let still = fixture::pinned(host, instance, &o);
    let inlined = editor_core::inline(&still, instance, &resolver, Tol::witness())
        .expect("at the identity the part's world is the host's");
    assert!(
        same(
            world(&inlined.doc, &EvalOptions::default()),
            world(&still, &o)
        ),
        "and at the identity it stays"
    );
    assert!(
        !same(world(&moved, &o), world(&still, &o)),
        "the pin moved the instance"
    );
}

/// `doc` evaluated at scalar `T` under `o`.
fn evaluated<T: editor_core::EvalScalar>(
    doc: &ProfileDoc,
    o: &EvalOptions,
) -> editor_core::Evaluation<T> {
    editor_core::evaluate::<T>(
        doc,
        None,
        &editor_core::CancelToken::new(),
        o,
        Tol::witness(),
    )
}

/// `placement`'s motion in `ev`, its placement evaluated.
fn motion_of<T: editor_core::EvalScalar>(
    ev: &editor_core::Evaluation<T>,
    placement: RecipeNodeId,
) -> Option<geom_core::RigidMotion<T>> {
    match ev.result(placement) {
        Some(NodeResult::Ok(value)) => value.motion,
        Some(NodeResult::Poisoned { through }) => {
            panic!(
                "the placement is poisoned through {through:?}: {:?}",
                ev.node_error(*through)
            )
        }
        other => panic!("the placement evaluates: {other:?}"),
    }
}

/// A box on the world floor whose slide along x is the length `s`,
/// declared at `nominal`; `s` the seed of a dual run.
fn box_slid_by_a_parameter(nominal: f64) -> (ProfileDoc, RecipeNodeId, EvalOptions) {
    let (doc, body) = a_box("s3c_parametric", 1.0);
    let (doc, _) = fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: editor_core::VarName::new("s").expect("a name"),
            def: editor_core::VarDecl::Free(editor_core::FreeVar::continuous(
                editor_core::Dimension::Length,
                nominal,
            )),
        },
    );
    let bottom = face_toward(&doc, body, [0.0, 0.0, -1.0]);
    let (doc, placement) = placed(doc, body);
    let (doc, floor) = floor_flipped(doc);
    let written = Mate {
        on: plane_of(on_body(body, bottom)),
        to: floor,
        orient: editor_core::Orientation::IDENTITY,
    };
    let slide = Formula::named(
        editor_core::VarName::new("s").expect("a name"),
        editor_core::Dimension::Length,
    );
    let doc = seat(
        doc,
        placement,
        written,
        [
            (Coord::Slide(TargetAxis::X), slide),
            (
                Coord::Slide(TargetAxis::Y),
                Formula::zero(Dimension::Length),
            ),
            (Coord::SPIN, Formula::zero(Dimension::Angle)),
        ],
    );
    let seeded = EvalOptions {
        seed: doc.var_named("s"),
        ..EvalOptions::default()
    };
    (doc, placement, seeded)
}

/// **FORK-REST — only the identity bundle is placed by no map, and in
/// every lane.** Whether a copy is placed by no map is read off its
/// recipe: the identity bundle ([`editor_core::stand`]: no mate, the
/// constant `0` on every freedom) is placed by none in the `f64`, the
/// interval, the dual and the symbolic lane alike (and the probe lane,
/// under `--features probe`), while a box placed by typed zeros runs
/// its map in every lane, though the map comes out as the identity.
///
/// *Breaks if* any lane decides from a value that a copy is at rest
/// (the lanes would disagree on whether a map is applied), or the
/// identity bundle is placed by a map.
#[test]
fn only_the_identity_bundle_is_placed_by_no_map_in_every_lane() {
    let lanes = |doc: &ProfileDoc, placement| {
        let seeded = EvalOptions {
            seed: doc.var_named("unread"),
            ..EvalOptions::default()
        };
        let plain = EvalOptions::default();
        let lanes = vec![
            motion_of(&evaluated::<f64>(doc, &plain), placement).is_none(),
            motion_of(&evaluated::<geom_core::Interval>(doc, &plain), placement).is_none(),
            motion_of(&evaluated::<geom_core::Dual64>(doc, &seeded), placement).is_none(),
            motion_of(&evaluated::<geom_core::Sym<f64>>(doc, &plain), placement).is_none(),
        ];
        #[cfg(feature = "probe")]
        let lanes = {
            let mut lanes = lanes;
            lanes.push(motion_of(&evaluated::<geom_core::Probe>(doc, &plain), placement).is_none());
            lanes
        };
        lanes
    };
    let every = |placed_by_no_map: bool| {
        vec![placed_by_no_map; if cfg!(feature = "probe") { 5 } else { 4 }]
    };
    let unread = |doc| {
        fixture::step(
            doc,
            DocEdit::DeclareVar {
                name: editor_core::VarName::new("unread").expect("a name"),
                def: editor_core::VarDecl::Free(editor_core::FreeVar::continuous(
                    editor_core::Dimension::Length,
                    1.0,
                )),
            },
        )
        .0
    };
    let (doc, body) = a_box("s3c_at_rest", 1.0);
    let (doc, placement) = placed(doc, body);
    let stood = fixture::pinned(doc.clone(), placement, &EvalOptions::default());
    assert_eq!(
        lanes(&unread(stood), placement),
        every(true),
        "the identity bundle is placed by no map"
    );
    let typed = TargetAxis::ALL
        .into_iter()
        .map(|axis| (Coord::Slide(axis), len(0.0)))
        .chain(
            TargetAxis::ALL
                .into_iter()
                .map(|axis| (Coord::Spin(Some(axis)), ang(0.0))),
        )
        .collect();
    let pinned = fixture::written(
        doc.clone(),
        placement,
        editor_core::Placed::of(&doc, Vec::new(), typed),
    );
    assert_eq!(
        lanes(&unread(pinned), placement),
        every(false),
        "typed zeros run their map"
    );
}

/// **A copy slid by a micrometre stands there.** The same box slid a
/// micrometre along x, a thousand times the band's zero: its map is
/// the slide.
///
/// *Breaks if* the slide is lost from the copy's map.
#[test]
fn a_copy_slid_past_the_band_keeps_its_map() {
    let (doc, placement, _) = box_slid_by_a_parameter(1e-6);
    let motion = motion_of(&evaluated::<f64>(&doc, &EvalOptions::default()), placement)
        .expect("a slid copy keeps its map");
    assert!((motion.to_affine().translation.x - 1e-6).abs() < 1e-15);
}

/// **A pose that moves with a parameter keeps its derivative.** The
/// box slid by `s`, declared at 0: it is no identity bundle, so it runs
/// its map in every lane (FORK-REST), the identity at the nominal; a
/// dual run seeded at `s` carries the slide's unit tangent, and its
/// translation's derivative along x is one.
///
/// *Breaks if* the map is dropped at rest (the value lane would place
/// it by no map), or the dual lane drops the tangent.
#[test]
fn a_parametric_pose_at_a_nominal_identity_keeps_its_derivative() {
    let (doc, placement, seeded) = box_slid_by_a_parameter(0.0);
    let nominal = motion_of(&evaluated::<f64>(&doc, &EvalOptions::default()), placement)
        .expect("a bundle reading a variable runs its map at the nominal")
        .to_affine();
    assert_eq!(
        format!("{nominal:?}"),
        format!("{:?}", Affine3::<f64>::identity()),
        "at the nominal the map is the identity"
    );
    let motion = motion_of(&evaluated::<geom_core::Dual64>(&doc, &seeded), placement)
        .expect("a seeded pose keeps its map");
    let x = motion.to_affine().translation.x;
    assert_eq!(x.value.to_bits(), 0.0_f64.to_bits(), "{x:?}");
    assert_eq!(x.deriv.to_bits(), 1.0_f64.to_bits(), "{x:?}");
}

/// **A seeded copy at rest has the value lane's bits and its own
/// tangent.** A box on the world floor spun by `t`, declared at a whole
/// turn: it is no identity bundle, so every lane runs its map
/// (FORK-REST), the `f64` lane's the identity to rounding. A dual run seeded at `t` places it by a map whose value
/// channel is the `f64` lane's bits and whose tangent is the spin's:
/// the turn about `z` moves the copy's `x` along `y` at unit rate.
///
/// *Breaks if* a lane drops the map (the value lane's would be no map,
/// the dual's tangent lost), or the dual's value channel rounds off
/// the `f64` lane's.
#[test]
fn a_seeded_copy_at_rest_has_the_value_lanes_bits_and_its_tangent() {
    let (doc, body) = a_box("s3c_spun_at_rest", 1.0);
    let (doc, _) = fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: editor_core::VarName::new("t").expect("a name"),
            def: editor_core::VarDecl::Free(editor_core::FreeVar::continuous(
                editor_core::Dimension::Angle,
                std::f64::consts::TAU,
            )),
        },
    );
    let bottom = face_toward(&doc, body, [0.0, 0.0, -1.0]);
    let (doc, placement) = placed(doc, body);
    let (doc, floor) = floor_flipped(doc);
    let written = Mate {
        on: plane_of(on_body(body, bottom)),
        to: floor,
        orient: editor_core::Orientation::IDENTITY,
    };
    let turn = Formula::named(
        editor_core::VarName::new("t").expect("a name"),
        editor_core::Dimension::Angle,
    );
    let doc = seat(
        doc,
        placement,
        written,
        [
            (
                Coord::Slide(TargetAxis::X),
                Formula::zero(Dimension::Length),
            ),
            (
                Coord::Slide(TargetAxis::Y),
                Formula::zero(Dimension::Length),
            ),
            (Coord::SPIN, turn),
        ],
    );
    let value_lane = motion_of(&evaluated::<f64>(&doc, &EvalOptions::default()), placement)
        .expect("a bundle reading a variable runs its map at the nominal")
        .to_affine();
    let seeded = EvalOptions {
        seed: doc.var_named("t"),
        ..EvalOptions::default()
    };
    let map = motion_of(&evaluated::<geom_core::Dual64>(&doc, &seeded), placement)
        .expect("a seeded pose keeps its map")
        .to_affine();
    let entries =
        [map.linear.c0, map.linear.c1, map.linear.c2, map.translation].map(|c| [c.x, c.y, c.z]);
    let want = [
        value_lane.linear.c0,
        value_lane.linear.c1,
        value_lane.linear.c2,
        value_lane.translation,
    ]
    .map(|c| [c.x, c.y, c.z]);
    for (column, want) in entries.iter().zip(want) {
        for (e, w) in column.iter().zip(want) {
            assert_eq!(e.value.to_bits(), f64::to_bits(w), "{map:?}");
        }
    }
    assert!(
        (entries[0][1].deriv - 1.0).abs() < 1e-12 && (entries[1][0].deriv + 1.0).abs() < 1e-12,
        "the turn's tangent moves x along y: {map:?}"
    );
}

/// **A box spanning the band encloses the value lane at its
/// corners.** A box on the floor spun by `t`, declared at a tenth of ε
/// rad, boxed over `[ε/10, 6ε/10]` (ε the run's, 1e-9 by default): the
/// bundle is no identity bundle, so the value lane runs its map at
/// each corner (FORK-REST) and keeps its turn at both, the low one
/// less than the band's zero and the high one more. The box's map holds
/// both. A map that wide moves the copy's edges by about the band's
/// zero, so the box run may refuse to certify them — an escalation;
/// what it never does is answer a map that leaves a corner out.
///
/// *Breaks if* a corner's value-lane map lies outside the box's, or the
/// value lane drops a corner's map.
#[test]
fn a_box_spanning_the_band_encloses_the_value_lane_at_its_corners() {
    // The corners are tenths of the run's ε, so the low one moves less
    // than the band's zero and the high one more at every ε.
    let eps = Tol::witness().eps();
    let (doc, body) = a_box("s3c_spanning", 1.0);
    let (doc, _) = fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: editor_core::VarName::new("t").expect("a name"),
            def: editor_core::VarDecl::Free(editor_core::FreeVar::continuous(
                editor_core::Dimension::Angle,
                0.1 * eps,
            )),
        },
    );
    let bottom = face_toward(&doc, body, [0.0, 0.0, -1.0]);
    let (doc, placement) = placed(doc, body);
    let (doc, floor) = floor_flipped(doc);
    let written = Mate {
        on: plane_of(on_body(body, bottom)),
        to: floor,
        orient: editor_core::Orientation::IDENTITY,
    };
    let turn = Formula::named(
        editor_core::VarName::new("t").expect("a name"),
        editor_core::Dimension::Angle,
    );
    let doc = seat(
        doc,
        placement,
        written,
        [
            (
                Coord::Slide(TargetAxis::X),
                Formula::zero(Dimension::Length),
            ),
            (
                Coord::Slide(TargetAxis::Y),
                Formula::zero(Dimension::Length),
            ),
            (Coord::SPIN, turn),
        ],
    );
    let t = doc.var_named("t").expect("declared");
    let at = |value: f64| {
        let d = fixture::step(
            doc.clone(),
            DocEdit::DefineVar {
                var: editor_core::VarName::new("t").expect("a name").into(),
                def: editor_core::VarDecl::Free(editor_core::FreeVar::continuous(
                    editor_core::Dimension::Angle,
                    value,
                )),
                fresh: Vec::new(),
            },
        )
        .0;
        motion_of(&evaluated::<f64>(&d, &EvalOptions::default()), placement)
            .map_or_else(Affine3::identity, |m| m.to_affine())
    };
    let (low, high) = (at(0.1 * eps), at(0.6 * eps));
    assert!(
        low.linear.c0.y != 0.0 && high.linear.c0.y != 0.0,
        "a bundle reading a variable keeps its turn at each corner: {low:?} {high:?}"
    );
    let boxed = EvalOptions {
        param_box: Some(std::sync::Arc::new(
            editor_core::analysis::ParamBox::from_axes(std::collections::BTreeMap::from([(
                t,
                editor_core::analysis::BoxAxis::Varying {
                    lo: 0.0,
                    hi: 0.5 * eps,
                },
            )])),
        )),
        ..EvalOptions::default()
    };
    let run = evaluated::<geom_core::Interval>(&doc, &boxed);
    let map = match run.result(placement) {
        Some(NodeResult::Ok(value)) => value.motion.expect("the box keeps a map").to_affine(),
        Some(NodeResult::Failed(error)) => {
            assert!(
                !error.escalations.is_empty(),
                "a box that does not answer escalates: {error:?}"
            );
            return;
        }
        other => panic!("the placement evaluates or refuses: {other:?}"),
    };
    let entries = |m: &Affine3<f64>| {
        [m.linear.c0, m.linear.c1, m.linear.c2, m.translation].map(|c| [c.x, c.y, c.z])
    };
    let boxed_entries =
        [map.linear.c0, map.linear.c1, map.linear.c2, map.translation].map(|c| [c.x, c.y, c.z]);
    for corner in [low, high] {
        for (column, want) in boxed_entries.iter().zip(entries(&corner)) {
            for (e, w) in column.iter().zip(want) {
                use geom_core::Bounds;
                assert!(
                    e.lo() <= w && w <= e.hi(),
                    "the box encloses each corner: {e:?} against {w}"
                );
            }
        }
    }
}

// ---- A copy standing where its body stands ----

/// A box whose height is the length `h`, declared at `nominal`, and one
/// copy of it standing where its body stands ([`editor_core::stand`]).
fn a_box_of_height_standing(nominal: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, _) = fixture::step(
        ProfileDoc::empty_derived("s3c_world_pin", Tol::witness()),
        DocEdit::DeclareVar {
            name: editor_core::VarName::new("h").expect("a name"),
            def: editor_core::VarDecl::Free(editor_core::FreeVar::continuous(
                editor_core::Dimension::Length,
                nominal,
            )),
        },
    );
    let (doc, profile) = fixture::on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, body) = fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: Formula::named(
                editor_core::VarName::new("h").expect("a name"),
                editor_core::Dimension::Length,
            ),
        },
    );
    let (doc, placement) = placed(doc, body);
    let doc = fixture::pinned(doc, placement, &EvalOptions::default());
    (doc, placement)
}

/// **A copy stands where its body stands, at every value.** A box whose
/// height is a variable, its copy standing by the identity bundle, which
/// reads nothing of the copy: the copy is in the world and placed by no
/// map at the nominal, at another value, and under an interval box over
/// the height (a tenth of ε wide, so the body itself certifies), where a
/// pin read off a face the height reaches would carry the height's width
/// into the copy.
///
/// *Breaks if* the stand reads the copy's geometry or a number (a map
/// would appear under the box), or places the copy anywhere but the
/// world.
#[test]
fn a_copy_stands_where_its_body_stands_at_every_value() {
    for nominal in [1.0, 2.5] {
        let (doc, placement) = a_box_of_height_standing(nominal);
        let ev = evaluated::<f64>(&doc, &EvalOptions::default());
        assert!(motion_of(&ev, placement).is_none(), "at h = {nominal}");
        assert!(
            matches!(ev.result(placement), Some(NodeResult::Ok(_))),
            "the placement evaluates"
        );
        assert_eq!(doc.node_space(placement), Some(editor_core::VarId::WORLD));
    }
    let (doc, placement) = a_box_of_height_standing(1.0);
    let h = doc.var_named("h").expect("declared");
    let boxed = EvalOptions {
        param_box: Some(std::sync::Arc::new(
            editor_core::analysis::ParamBox::from_axes(std::collections::BTreeMap::from([(
                h,
                editor_core::analysis::BoxAxis::Varying {
                    lo: -0.1 * Tol::witness().eps(),
                    hi: 0.1 * Tol::witness().eps(),
                },
            )])),
        )),
        ..EvalOptions::default()
    };
    let ev = evaluated::<geom_core::Interval>(&doc, &boxed);
    assert!(
        motion_of(&ev, placement).is_none(),
        "no width under the box"
    );
}

/// **Inline carries a standing copy at the identity, and at a moved
/// landing.** A part whose copy stands where its body stands, instanced
/// in a host: pinned at the identity, the inlined copy stands where its
/// body stands; pinned off the identity, the copy stands at the
/// instance's motion.
///
/// *Breaks if* the inline drops the instance's bundle, or composes it
/// into the part (the copy would stay at the part's identity).
#[test]
fn inline_carries_a_standing_copy_at_the_identity_and_at_a_moved_landing() {
    let (part, _) = a_box_of_height_standing(1.0);
    let mut store = fixture::resolver::PartStore::new();
    let part = crate::fixture::assemble_world_copies(part);
    let part_ref = store.insert(part, Tol::witness());
    let o = fixture::resolver::with_resolver(store.clone());
    let resolver: std::sync::Arc<dyn editor_core::PartResolver> = std::sync::Arc::new(store);
    let host = ProfileDoc::empty_derived("s3c_world_pin_host", Tol::witness());
    let (host, instance) = fixture::insert(host, crate::fixture::mated_instance(part_ref));
    let still = fixture::pinned(host.clone(), instance, &o);
    let inlined = editor_core::inline(&still, instance, &resolver, Tol::witness())
        .expect("at the identity the part's world is the host's");
    let at = solved(&inlined.doc, inlined.placement);
    assert!(
        turn_between(at.linear, Mat3::identity()) < 1e-12 && at.translation.norm() < 1e-12,
        "the copy stands where its body stands: {at:?}"
    );
    let moved = fixture::pin(
        host,
        instance,
        &editor_core::Frame::translation([3.0, 0.0, 0.0]),
        &o,
    );
    let inlined = editor_core::inline(&moved, instance, &resolver, Tol::witness())
        .expect("a moved landing places the copy at the move");
    let at = solved(&inlined.doc, inlined.placement);
    assert!(
        turn_between(at.linear, Mat3::identity()) < 1e-12
            && (at.translation - Vec3::new(3.0, 0.0, 0.0)).norm() < 1e-12,
        "the copy stands at the move: {at:?}"
    );
}

/// **A part inlined twice lands at the composed pose**: a part whose
/// copy stands at a generic pose by values on its world's axes,
/// instanced at a generic pose and inlined, then instanced again at a
/// third and inlined in turn, lands its copy at the three poses
/// composed, each level a placement of its own under the bundle its
/// instance held, so nothing is composed into a number.
///
/// *Breaks if* inline composes a landing into the part's values, or
/// drops a level's bundle.
#[test]
fn a_part_inlined_twice_lands_at_the_composed_pose() {
    let pose = |axis: [f64; 3], angle: f64, t: [f64; 3]| Affine3 {
        linear: Mat3::rotation_about(Vec3::new(axis[0], axis[1], axis[2]), angle),
        translation: Vec3::new(t[0], t[1], t[2]),
    };
    let poses = [
        pose([1.0, 2.0, 3.0], 0.7, [0.4, -1.1, 2.3]),
        pose([-2.0, 0.5, 1.0], 1.9, [3.0, 0.25, -0.5]),
        pose([0.3, -1.0, 0.2], -2.4, [-1.5, 2.0, 0.75]),
    ];
    let sorted = |mut points: Vec<Vec3<f64>>| {
        points.sort_by(|a, b| a.to_array().partial_cmp(&b.to_array()).expect("finite"));
        points
    };
    let (part, body) = a_box("s3c_inline_twice_part", 1.0);
    let own = match fixture::run(&part, &EvalOptions::default()).result(body) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            editor_core::ValuePayload::Body(b) => sorted(
                b.body()
                    .vertex_points()
                    .map(|(_, p)| Vec3::new(p.x, p.y, p.z))
                    .collect(),
            ),
            other => panic!("a box is a body: {other:?}"),
        },
        other => panic!("the box evaluates: {other:?}"),
    };
    let (part, placement) = placed(part, body);
    let part = fixture::pin(
        part,
        placement,
        &editor_core::Frame::from_affine(poses[0]),
        &EvalOptions::default(),
    );
    let mut store = fixture::resolver::PartStore::new();
    let mut doc = part;
    for (level, &at) in poses[1..].iter().enumerate() {
        let doc_ref = store.insert(crate::fixture::assemble_world_copies(doc), Tol::witness());
        let o = fixture::resolver::with_resolver(store.clone());
        let resolver: std::sync::Arc<dyn editor_core::PartResolver> =
            std::sync::Arc::new(store.clone());
        let host = ProfileDoc::empty_derived(&format!("s3c_inline_twice_{level}"), Tol::witness());
        let (host, instance) = fixture::insert(host, crate::fixture::mated_instance(doc_ref));
        let host = fixture::pin(host, instance, &editor_core::Frame::from_affine(at), &o);
        doc = editor_core::inline(&host, instance, &resolver, Tol::witness())
            .unwrap_or_else(|e| panic!("inline {level} lands: {e}"))
            .doc;
    }
    let product = crate::fixture::product_in_world(
        &doc,
        &fixture::run(&doc, &EvalOptions::default()),
        Tol::witness(),
    )
    .expect("the flattened host has a product");
    let got = sorted(
        product
            .vertex_points()
            .map(|(_, p)| Vec3::new(p.x, p.y, p.z))
            .collect(),
    );
    let composed = poses[2] * poses[1] * poses[0];
    let want = sorted(
        own.iter()
            .map(|&v| composed.linear * v + composed.translation)
            .collect(),
    );
    assert_eq!(got.len(), want.len());
    let off = got
        .iter()
        .zip(&want)
        .map(|(a, b)| (*a - *b).norm())
        .fold(0.0_f64, f64::max);
    assert!(
        off < 1e-12,
        "the copy stands at the three poses composed, {off:e} off"
    );
}
