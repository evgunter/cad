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
use editor_core::mate::coset::{Coset, Subgroup, intersect};
use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    BodyRead, ConstraintId, ContactClass, Coord, DocEdit, EditError, EvalOptions, Formula, Mate,
    MateFault, Node, NodeErrorKind, NodeResult, Operand, ProfileDoc, RecipeNodeId, StableName,
    Target, TargetAxis, Value, WorldRead,
};
use geom_core::Tol;
use geom_core::linalg::{Affine3, Mat3, Point3, UnitVec3, Vec3};
use geom_core::predicate::Band;

fn band() -> Band {
    Band::linear(Tol::witness()).expect("a band")
}

fn unit(v: Vec3<f64>) -> UnitVec3<f64> {
    UnitVec3::new(v, topo::query::DATUM_UNIT_NORM, band()).expect("a direction")
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
        Node::place(vec![BodyRead::plain(Operand::output(body, 0))]),
    )
}

/// The world's xy plane, its normal turned to −z by a half-turn about
/// x — a plane facing down with no flip in its spelling.
fn world_floor_facing_down() -> WorldRead<Formula> {
    WorldRead::through(PoseRole::Xy)
        .spin(WorldRead::through(PoseRole::X), ang(std::f64::consts::PI))
}

/// `value` added to `placement`, its id.
fn add_value(
    doc: ProfileDoc,
    placement: RecipeNodeId,
    mate: ConstraintId,
    coord: Coord,
    by: Formula,
) -> Result<(ProfileDoc, ConstraintId), EditError> {
    let applied = doc.apply(
        &DocEdit::AddValue {
            placement,
            value: Box::new(Value {
                mate: Some(mate),
                coord,
                by,
            }),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )?;
    let id = applied.record.minted_constraints[0];
    Ok((applied.doc, id))
}

/// The value's number for `coord`: a length or an angle.
fn number(coord: Coord, v: f64) -> Formula {
    match coord {
        Coord::Slide(_) => len(v),
        Coord::Spin | Coord::SpinAbout(_) => ang(v),
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
    box_resting_on(WorldRead::through(PoseRole::Xy).flip(), values, order)
}

/// A box resting by its bottom face on the world plane `target`, with
/// values `(slide x, slide y, spin)` written in `order`.
fn box_resting_on(
    target: WorldRead<Formula>,
    values: [f64; 3],
    order: [usize; 3],
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, body) = a_box("s3c_floor", 1.0);
    let bottom = face_toward(&doc, body, [0.0, 0.0, -1.0]);
    let (doc, placement) = placed(doc, body);
    let (mut doc, mate) = fixture::constrain_at(
        doc,
        placement,
        Mate {
            on: plane_of(on_body(body, bottom)),
            to: Target::World(target),
            class: ContactClass::Rest,
        },
    );
    let coords = [
        Coord::Slide(TargetAxis::X),
        Coord::Slide(TargetAxis::Y),
        Coord::Spin,
    ];
    for k in order {
        doc = add_value(
            doc,
            placement,
            mate.id,
            coords[k],
            number(coords[k], values[k]),
        )
        .expect("a plane mate leaves the freedom")
        .0;
    }
    (doc, placement)
}

/// **Row 9 — a plane mate leaves two slides and a spin, charted on the
/// two bodies' own frames.** A box rests on the world floor by its
/// bottom face (a rest carries a flip on the target; the chart
/// normalises it to the copy's side, so both swung frames are their
/// own). At zero values the box's own coordinates agree with the
/// world's; at `(3, 4, 30°)` its origin stands at `(3, 4, 0)` turned
/// 30° about `z`. Every order of the three values places it alike.
///
/// *Breaks if* a slide is charted along the other slide's moved axis
/// (order-dependent), the spin is measured from the copy's own axis
/// rather than the target's swung `x`, or the flip is read on the
/// target's side (the frames would swing through the antipode).
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
/// target too.** The floor read spun 0.3 about `x` and 0.7 about `z`,
/// then flipped: values `(3.3, 4.7, 0.9)` in all six orders give one
/// map, bit for bit — the values are taken in one order whatever order
/// they are listed in.
///
/// *Breaks if* the solve takes the values in their listed order (a
/// tilted chart's slides round apart in the last bits).
#[test]
fn every_order_places_the_copy_to_the_bit_on_a_tilted_target() {
    let tilted = WorldRead::through(PoseRole::Xy)
        .spin(WorldRead::through(PoseRole::X), ang(0.3))
        .spin(WorldRead::through(PoseRole::Z), ang(0.7))
        .flip();
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
/// leaves.** A box resting on the floor with a slide along `x`: a
/// second slide along `x` on the same mate sets the freedom the first
/// already sets, so the door refuses it, naming both, though the spin
/// is still free (FORK-S3O).
///
/// *Breaks if* a value is admitted beside one of its own freedom, or
/// the door names only the later one.
#[test]
fn a_slide_set_twice_refuses_though_a_turn_is_free() {
    let (doc, body) = a_box("s3c_twice", 1.0);
    let bottom = face_toward(&doc, body, [0.0, 0.0, -1.0]);
    let (doc, placement) = placed(doc, body);
    let (doc, mate) = fixture::constrain_at(
        doc,
        placement,
        Mate {
            on: plane_of(on_body(body, bottom)),
            to: Target::World(WorldRead::through(PoseRole::Xy).flip()),
            class: ContactClass::Rest,
        },
    );
    let (doc, first) = add_value(
        doc,
        placement,
        mate.id,
        Coord::Slide(TargetAxis::X),
        len(1.0),
    )
    .expect("a plane mate leaves the slide");
    match add_value(
        doc,
        placement,
        mate.id,
        Coord::Slide(TargetAxis::X),
        len(2.0),
    ) {
        Err(EditError::MateRefused { fault, .. }) => assert!(
            matches!(
                &*fault,
                MateFault::Overconstrained { placement: p, constraints, free: 2 }
                    if *p == placement && constraints.len() == 2 && constraints.contains(&first)
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
    let (doc, mate) = fixture::constrain_at(
        doc,
        placement,
        Mate {
            on: face,
            to: Target::World(WorldRead::through(PoseRole::Yz)),
            class: ContactClass::Rest,
        },
    );
    let mut doc = doc;
    for (coord, by) in [
        (Coord::Slide(TargetAxis::X), len(0.0)),
        (Coord::Slide(TargetAxis::Y), len(0.0)),
        (Coord::Spin, ang(30.0_f64.to_radians())),
    ] {
        doc = add_value(doc, placement, mate.id, coord, by)
            .expect("a plane mate leaves the freedom")
            .0;
    }
    solved(&doc, placement)
}

/// **Ruling C-1 — the chart is continuous where a face is drafted.** A
/// box's wall facing `+x`, drafted by 0°, 1° and −1°, mated to the
/// world's `+x` plane under a held spin of 30°: each body's frame is
/// swung onto the wall's normal by the shortest turn, so the copy turns
/// by about the draft and no more — no flip, no jump through the
/// antipode.
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
/// *Breaks if* the swung reference has a branch at `φ = 0` (the
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

/// **Ruling C-1 — the antipode is decided, not banded.** The swing of
/// the body's own `z` onto `−z` exactly takes the half-turn about `x`
/// (`x` stays); a normal at a rounding-level tilt from `−z` is decided
/// the antipode too, and stays `x` bit for bit; a tilt the band cannot
/// tell from it refuses `PoseDegenerate { Chart }`; one well clear of
/// it takes the shortest turn.
///
/// *Breaks if* the antipode reads through `1/(1+c)` (a rounding-level
/// tilt turns `x` by an arbitrary angle), or a tilt in the band is
/// picked rather than refused.
#[test]
fn the_antipode_is_decided_exactly_and_the_band_refuses() {
    let band = band();
    let x = Vec3::new(1.0, 0.0, 0.0);
    let unit = |v: Vec3<f64>| UnitVec3::new(v, topo::query::DATUM_UNIT_NORM, band).unwrap();
    let exact = editor_core::reference_about(unit(Vec3::new(0.0, 0.0, -1.0)), 1.0, band)
        .expect("the antipode decides");
    assert_eq!(
        exact.to_array(),
        x.to_array(),
        "the half-turn about x keeps x"
    );
    let rounding = (std::f64::consts::PI).sin(); // 1.2e-16
    let tilted = unit(Vec3::new(0.0, rounding, -1.0));
    let at = editor_core::reference_about(tilted, 1.0, band).expect("decided the antipode");
    assert_eq!(
        at.to_array(),
        x.to_array(),
        "a rounding-level tilt is the antipode"
    );
    // A chord inside the band: the band's zero is ε; a chord of 1.5ε at
    // a unit arm sits between ε and K·ε.
    let chord = 1.5 * band.zero();
    let in_band = unit(Vec3::new(chord, 0.0, -1.0));
    assert!(
        matches!(
            editor_core::reference_about(in_band, 1.0, band),
            Err(NodeErrorKind::PoseDegenerate {
                construction: editor_core::pose::PoseConstruction::Chart
            })
        ),
        "a chord the band cannot decide refuses"
    );
    let clear = unit(Vec3::new(0.6, 0.0, -0.8));
    let at = editor_core::reference_about(clear, 1.0, band).expect("well clear of the antipode");
    // The shortest turn carrying z to (0.6, 0, −0.8) turns x to
    // (1 − a²/(1+c), −ab/(1+c), −a) = (1 − 0.36/0.2, 0, −0.6).
    assert!((at - Vec3::new(-0.8, 0.0, -0.6)).norm() < 1e-12, "{at:?}");
}

/// **Ruling C-1 end to end — a cap facing `−z` placed on a floor facing
/// `−z`.** Both swung frames sit at the antipode, decided exactly: the
/// box's own `x` meets the world's, and zero values put it where it is.
/// The same holds for the cap read at a rounding-level tilt (spun by
/// `sin π`, a half-revolve's end-face rounding).
///
/// *Breaks if* the antipode is banded rather than decided (the
/// evaluation refuses), or read through `1/(1+c)` (the copy turns).
#[test]
fn a_cap_at_the_antipode_places_without_a_turn() {
    let (doc, body) = a_box("s3c_cap", 1.0);
    let bottom = face_toward(&doc, body, [0.0, 0.0, -1.0]);
    let (doc, placement) = placed(doc, body);
    let (mut doc, mate) = fixture::constrain_at(
        doc,
        placement,
        Mate {
            on: plane_of(on_body(body, bottom)),
            to: Target::World(world_floor_facing_down()),
            class: ContactClass::Rest,
        },
    );
    for coord in [
        Coord::Slide(TargetAxis::X),
        Coord::Slide(TargetAxis::Y),
        Coord::Spin,
    ] {
        doc = add_value(doc, placement, mate.id, coord, number(coord, 0.0))
            .expect("a plane mate leaves the freedom")
            .0;
    }
    let at = solved(&doc, placement);
    assert!(
        turn_between(at.linear, Mat3::identity()) < 1e-12 && at.translation.norm() < 1e-12,
        "the antipode keeps x, and zero values put the box where it is: {at:?}"
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
    let (mut doc, mate) = fixture::constrain_at(
        doc,
        placement,
        Mate {
            on: tilted,
            to: Target::World(world_floor_facing_down()),
            class: ContactClass::Rest,
        },
    );
    doc = add_value(doc, placement, mate.id, Coord::Spin, ang(0.0))
        .expect("a plane mate leaves its spin")
        .0;
    let ev = fixture::run(&doc, &EvalOptions::default());
    assert!(
        !matches!(ev.result(placement), Some(NodeResult::Failed(_))),
        "the antipode at a rounding-level tilt is decided, never refused: {:?}",
        ev.result(placement)
    );
}

/// **Ruling C-2 — a mate a value names goes only once the value has
/// gone.** Removing the floor mate while its spin value names it
/// refuses `MateNamedByValue`; removing the value, then the mate, is
/// admitted.
///
/// *Breaks if* the mate goes and leaves a value setting a freedom of
/// nothing.
#[test]
fn a_mate_a_value_names_is_removed_only_after_it() {
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
    };
    match remove(&doc, mate) {
        Err(EditError::MateNamedByValue { mate: named, .. }) => assert_eq!(named, mate),
        other => panic!("the mate is named by its values: {other:?}"),
    }
    let mut doc = doc;
    for value in bundle.iter().filter(|c| c.value().is_some()) {
        doc = remove(&doc, value.id).expect("a value goes").doc;
    }
    remove(&doc, mate).expect("with its values gone, the mate goes");
}

/// **The "place part" gesture reproduces its pose.** A box placed at a
/// pose `P` (turned about a slanted axis, slid off the origin) by
/// `place_at`: a plane mate on its first planar face to the world plane
/// that face lies in at `P`, and three values computed from `P` — none
/// of them zero here. Re-evaluating puts the copy at `P` to the
/// rounding of one re-composition.
///
/// *Breaks if* the gesture writes zero values and leaves the copy at
/// the plane's foot, or its chart and the solve's disagree.
#[test]
fn the_place_gesture_reproduces_its_pose() {
    let (doc, body) = a_box("s3c_place_at", 1.0);
    let (doc, placement) = placed(doc, body);
    let turn = Mat3::rotation_about(Vec3::new(1.0, 2.0, 3.0), 0.7);
    let shift = Vec3::new(4.0, -1.5, 2.25);
    let pose = editor_core::Frame::from_affine(Affine3::from_parts(turn, shift));
    let shape = match doc.node(placement) {
        Some(Node::Place { shapes, .. }) => Operand::Var(shapes[0].read),
        _ => unreachable!("a placement was inserted"),
    };
    let placed_by =
        editor_core::place_at(&doc, &shape, &pose, &EvalOptions::default(), Tol::witness())
            .expect("a box has a planar face");
    assert_eq!(placed_by.by, editor_core::PlacedBy::PlanarFace);
    assert_eq!(placed_by.values.len(), 3, "two slides and a spin");
    assert!(
        placed_by
            .values
            .iter()
            .all(|(_, by)| by.literal_value().is_some_and(|v| v != 0.0)),
        "the values are computed from P, not assumed zero: {:?}",
        placed_by.values
    );
    let mate = placed_by.mate.expect("the gesture writes a plane mate");
    let (mut doc, mate) = fixture::constrain_at(doc, placement, mate);
    for (coord, by) in placed_by.values {
        doc = add_value(doc, placement, mate.id, coord, by)
            .expect("the gesture's values are its mate's freedoms")
            .0;
    }
    let at = solved(&doc, placement);
    assert!(
        turn_between(at.linear, turn) < 1e-12 && (at.translation - shift).norm() < 1e-12,
        "the copy stands at P: {at:?}"
    );
}

/// **The gesture flips the copy's side where its face would look down
/// the world's `z`, so its chart never reads at the swing's pole.** A
/// box placed at a turn of `ε` about `x`: its start cap (the first
/// planar face no variable reaches) faces `−z`, a chord the band cannot
/// tell from the chart's pole. `place_at` mates the cap's `Flip`, whose
/// normal faces `+z`, so the world plane is charted at its reference:
/// the plane mate and its values are written (`PlacedBy::PlanarFace`),
/// and the copy stands at `P`, as it does at a turn well clear of the
/// pole. The turn is the run's ε, not a literal.
///
/// *Breaks if* the gesture charts the unflipped face (and refuses or
/// pins at the pole), or the copy does not stand at `P`.
#[test]
fn the_place_gesture_flips_a_face_looking_down_so_its_chart_has_a_swing() {
    let (doc, body) = a_box("s3c_place_pole", 1.0);
    let (doc, placement) = placed(doc, body);
    let shape = match doc.node(placement) {
        Some(Node::Place { shapes, .. }) => Operand::Var(shapes[0].read),
        _ => unreachable!("a placement was inserted"),
    };
    let gesture = |angle: f64| {
        let turn = Mat3::rotation_about(Vec3::new(1.0, 0.0, 0.0), angle);
        let pose = editor_core::Frame::from_affine(Affine3::from_parts(turn, Vec3::zero()));
        let placed =
            editor_core::place_at(&doc, &shape, &pose, &EvalOptions::default(), Tol::witness())
                .unwrap_or_else(|refusal| panic!("the gesture places at {angle}: {refusal}"));
        (turn, placed)
    };
    for angle in [0.5, Tol::witness().eps()] {
        let (turn, near) = gesture(angle);
        assert_eq!(
            near.by,
            editor_core::PlacedBy::PlanarFace,
            "{angle}: {near:?}"
        );
        let mate = near.mate.expect("the gesture writes a plane mate");
        assert!(
            matches!(
                &mate.on,
                Operand::Pose(p) if matches!(**p, editor_core::pose::PoseDef::Flip { .. })
            ),
            "{angle}: the cap looks down, so its copy side is flipped: {:?}",
            mate.on
        );
        let (mut doc, mate) = fixture::constrain_at(doc.clone(), placement, mate);
        for (coord, by) in near.values {
            doc = add_value(doc, placement, mate.id, coord, by)
                .expect("the gesture's values are its mate's freedoms")
                .0;
        }
        let at = solved(&doc, placement);
        assert!(
            turn_between(at.linear, turn) < 1e-12 && at.translation.norm() < 1e-12,
            "{angle}: the copy stands at P: {at:?}"
        );
    }
}

/// **The migration's pin reproduces its pose.** `pin_at` reads a frame
/// off the copy's first axis-bearing edge and a vertex off it, and
/// equates it with the world's frame moved to where that frame sits at
/// `P`: the copy stands at `P`.
///
/// *Breaks if* the world read's spins compose in another order than the
/// angles were taken in.
#[test]
fn the_migration_pin_reproduces_its_pose() {
    let (doc, body) = a_box("s3c_pin_at", 1.0);
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
/// world** (C-4). A part whose one copy rests on its world floor by a
/// mate and three values, all numbers, instanced in a host and pinned
/// off the identity: inline re-pins the part placement where the
/// instance put its copy, so the inlined host's world is the one it
/// had. Pinned at the identity, the same instance inlines too.
///
/// *Breaks if* the inline moves the mate's world read and leaves the
/// value charted on the unmoved frame (the copy lands at its numbers in
/// the host's world, not moved with the rest).
#[test]
fn inline_moves_a_landing_under_a_world_value_with_its_instance() {
    let (part, _) = box_on_the_floor([0.5, 0.0, 0.0], [0, 1, 2]);
    let mut store = fixture::resolver::PartStore::new();
    let part_ref = store.insert(part, Tol::witness());
    let o = fixture::resolver::with_resolver(store.clone());
    let resolver: std::sync::Arc<dyn editor_core::PartResolver> = std::sync::Arc::new(store);
    let host = ProfileDoc::empty_derived("s3c_inline_host", Tol::witness());
    let (host, instance) = fixture::insert(host, Node::instantiate_part(part_ref));
    // The world's vertices, sorted: a re-pin re-composes the pose, so
    // the copy stands where it stood to the rounding of that one step.
    let world = |doc: &ProfileDoc, o: &EvalOptions| {
        let body = editor_core::product(doc, &fixture::run(doc, o), Tol::witness())
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
    let (mut doc, mate) = fixture::constrain_at(
        doc,
        placement,
        Mate {
            on: plane_of(on_body(body, bottom)),
            to: Target::World(WorldRead::through(PoseRole::Xy).flip()),
            class: ContactClass::Rest,
        },
    );
    let slide = Formula::named(
        editor_core::VarName::new("s").expect("a name"),
        editor_core::Dimension::Length,
    );
    for (coord, by) in [
        (Coord::Slide(TargetAxis::X), slide),
        (Coord::Slide(TargetAxis::Y), len(0.0)),
        (Coord::Spin, ang(0.0)),
    ] {
        doc = add_value(doc, placement, mate.id, coord, by)
            .expect("a plane mate leaves the freedom")
            .0;
    }
    let seeded = EvalOptions {
        seed: doc.var_named("s"),
        ..EvalOptions::default()
    };
    (doc, placement, seeded)
}

/// **Fork C-5 — a copy at rest is placed by no map, in every lane.** A
/// box pinned at the identity by `pin_at` stands there to rounding; the
/// fold decides it at rest and places it by no map in the `f64`, the
/// interval and the dual lane alike.
///
/// *Breaks if* the snap is one lane's (the lanes would disagree on
/// whether a map is applied), or reads another predicate than
/// `SolvedPoses::at_identity`'s.
#[test]
fn a_copy_pinned_at_the_identity_is_placed_by_no_map_in_every_lane() {
    let (doc, body) = a_box("s3c_at_rest", 1.0);
    let (doc, placement) = placed(doc, body);
    // `pin_at`'s mate itself, not the stand `fixture::pinned`
    // writes at the identity: the snap is this row's subject.
    let mate = fixture::pin_mate(
        &doc,
        placement,
        &editor_core::Frame::IDENTITY,
        &EvalOptions::default(),
    );
    let (doc, _) = fixture::constrain_at(doc, placement, mate);
    let (doc, _) = fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: editor_core::VarName::new("unread").expect("a name"),
            def: editor_core::VarDecl::Free(editor_core::FreeVar::continuous(
                editor_core::Dimension::Length,
                1.0,
            )),
        },
    );
    let seeded = EvalOptions {
        seed: doc.var_named("unread"),
        ..EvalOptions::default()
    };
    assert!(motion_of(&evaluated::<f64>(&doc, &EvalOptions::default()), placement).is_none());
    let boxed = motion_of(
        &evaluated::<geom_core::Interval>(&doc, &EvalOptions::default()),
        placement,
    );
    assert!(boxed.is_none(), "{boxed:?}");
    assert!(motion_of(&evaluated::<geom_core::Dual64>(&doc, &seeded), placement).is_none());
}

/// **Fork C-5 — a copy slid by more than the band is not at rest.** The
/// same box slid a micrometre along x, a thousand times the band's
/// zero: its map is kept.
///
/// *Breaks if* the snap reads a band wider than the tolerance's, or
/// snaps a decided displacement.
#[test]
fn a_copy_slid_past_the_band_keeps_its_map() {
    let (doc, placement, _) = box_slid_by_a_parameter(1e-6);
    let motion = motion_of(&evaluated::<f64>(&doc, &EvalOptions::default()), placement)
        .expect("a slid copy keeps its map");
    assert!((motion.to_affine().translation.x - 1e-6).abs() < 1e-15);
}

/// **Fork C-5 — a pose that moves with a parameter keeps its
/// derivative.** The box slid by `s`, declared at 0: its bundle reads a
/// declared variable, so it runs its map in every lane (C-5 narrowed),
/// the identity at the nominal; a dual run seeded at `s` carries the
/// slide's unit tangent, and its translation's derivative along x is
/// one.
///
/// *Breaks if* the bundle is snapped at rest (the value lane would
/// place it by no map), or the dual lane drops the tangent.
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

/// **Fork C-5 — a seeded copy at rest has the value lane's bits and
/// its own tangent.** A box on the world floor spun by `t`, declared at
/// a whole turn: its bundle reads a declared variable, so every lane
/// runs its map (C-5 narrowed), the `f64` lane's the identity to
/// rounding. A dual run seeded at `t` places it by a map whose value
/// channel is the `f64` lane's bits and whose tangent is the spin's:
/// the turn about `z` moves the copy's `x` along `y` at unit rate.
///
/// *Breaks if* a lane snaps the map (the value lane's would be no map,
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
    let (mut doc, mate) = fixture::constrain_at(
        doc,
        placement,
        Mate {
            on: plane_of(on_body(body, bottom)),
            to: Target::World(WorldRead::through(PoseRole::Xy).flip()),
            class: ContactClass::Rest,
        },
    );
    let turn = Formula::named(
        editor_core::VarName::new("t").expect("a name"),
        editor_core::Dimension::Angle,
    );
    for (coord, by) in [
        (Coord::Slide(TargetAxis::X), len(0.0)),
        (Coord::Slide(TargetAxis::Y), len(0.0)),
        (Coord::Spin, turn),
    ] {
        doc = add_value(doc, placement, mate.id, coord, by)
            .expect("a plane mate leaves the freedom")
            .0;
    }
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

/// **Fork C-5 — a box spanning the band encloses the value lane at its
/// corners.** A box on the floor spun by `t`, declared at a tenth of ε
/// rad, boxed over `[ε/10, 6ε/10]` (ε the run's, 1e-9 by default): the
/// bundle reads a declared variable, so the value lane runs its map at
/// each corner (C-5 narrowed) and keeps its turn at both, the low one
/// less than the band's zero and the high one more. The box's map holds
/// both. A map that wide moves the copy's edges by about the band's
/// zero, so the box run may refuse to certify them — an escalation;
/// what it never does is answer a map that leaves a corner out.
///
/// *Breaks if* a corner's value-lane map lies outside the box's, or the
/// value lane snaps a corner of a bundle that reads a variable.
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
    let (mut doc, mate) = fixture::constrain_at(
        doc,
        placement,
        Mate {
            on: plane_of(on_body(body, bottom)),
            to: Target::World(WorldRead::through(PoseRole::Xy).flip()),
            class: ContactClass::Rest,
        },
    );
    let turn = Formula::named(
        editor_core::VarName::new("t").expect("a name"),
        editor_core::Dimension::Angle,
    );
    for (coord, by) in [
        (Coord::Slide(TargetAxis::X), len(0.0)),
        (Coord::Slide(TargetAxis::Y), len(0.0)),
        (Coord::Spin, turn),
    ] {
        doc = add_value(doc, placement, mate.id, coord, by)
            .expect("a plane mate leaves the freedom")
            .0;
    }
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
/// height is a variable, its copy standing on its start cap (a face no
/// variable reaches): the copy is in the world and placed by no map at
/// the nominal, at another value, and under an interval box over the
/// height (a tenth of ε wide, so the body itself certifies), where a pin
/// read off a face the height reaches would carry the height's width
/// into the copy.
///
/// *Breaks if* the stand reads a face the height reaches or a number
/// (a map would appear under the box), or places the copy anywhere but
/// the world.
#[test]
fn a_copy_stands_where_its_body_stands_at_every_value() {
    for nominal in [1.0, 2.5] {
        let (doc, placement) = a_box_of_height_standing(nominal);
        let ev = evaluated::<f64>(&doc, &EvalOptions::default());
        assert!(motion_of(&ev, placement).is_none(), "at h = {nominal}");
        match ev.result(placement) {
            Some(NodeResult::Ok(value)) => {
                assert_eq!(value.space, Some(editor_core::Space::World));
            }
            other => panic!("the placement evaluates: {other:?}"),
        }
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
/// *Breaks if* the inline drops the copy's pin, or carries it unmoved
/// under a moved landing (the copy would stay at the part's identity).
#[test]
fn inline_carries_a_standing_copy_at_the_identity_and_at_a_moved_landing() {
    let (part, _) = a_box_of_height_standing(1.0);
    let mut store = fixture::resolver::PartStore::new();
    let part_ref = store.insert(part, Tol::witness());
    let o = fixture::resolver::with_resolver(store.clone());
    let resolver: std::sync::Arc<dyn editor_core::PartResolver> = std::sync::Arc::new(store);
    let host = ProfileDoc::empty_derived("s3c_world_pin_host", Tol::witness());
    let (host, instance) = fixture::insert(host, Node::instantiate_part(part_ref));
    let still = fixture::pinned(host.clone(), instance, &o);
    let inlined = editor_core::inline(&still, instance, &resolver, Tol::witness())
        .expect("at the identity the part's world is the host's");
    let [copy] = inlined.doc.placements()[..] else {
        panic!("one placement in the host");
    };
    let at = solved(&inlined.doc, copy);
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
        .expect("a moved landing pins the copy at the move");
    let [copy] = inlined.doc.placements()[..] else {
        panic!("one placement in the host");
    };
    let at = solved(&inlined.doc, copy);
    assert!(
        turn_between(at.linear, Mat3::identity()) < 1e-12
            && (at.translation - Vec3::new(3.0, 0.0, 0.0)).norm() < 1e-12,
        "the copy stands at the move: {at:?}"
    );
}

/// **Inline folds its landing into the world read it moves**: a part
/// whose copy is pinned by `pin_at` at a generic pose, instanced at a
/// generic pose and inlined, then instanced again at a third and
/// inlined in turn, lands its copy at the three poses composed, and
/// its pin's world read is no deeper than `pin_at` wrote it: each
/// inline takes the run of world moves off and writes the composed
/// motion as one run, so flattening a nested assembly level by level
/// never reaches `MAX_WORLD_READ_DEPTH`.
///
/// *Breaks if* inline appends its moves on top of the read's own (the
/// second inline refuses `TooDeep`, or the depth grows).
#[test]
fn a_part_inlined_twice_lands_at_the_composed_pose_at_its_pins_depth() {
    let pose = |axis: [f64; 3], angle: f64, t: [f64; 3]| Affine3 {
        linear: Mat3::rotation_about(Vec3::new(axis[0], axis[1], axis[2]), angle),
        translation: Vec3::new(t[0], t[1], t[2]),
    };
    let poses = [
        pose([1.0, 2.0, 3.0], 0.7, [0.4, -1.1, 2.3]),
        pose([-2.0, 0.5, 1.0], 1.9, [3.0, 0.25, -0.5]),
        pose([0.3, -1.0, 0.2], -2.4, [-1.5, 2.0, 0.75]),
    ];
    let depth = |doc: &ProfileDoc| {
        doc.placements()
            .into_iter()
            .flat_map(|at| fixture::bundle(doc, at))
            .filter_map(|c| match &c.mate()?.to {
                Target::World(world) => Some(world.depth()),
                Target::Pose(_) => None,
            })
            .max()
            .expect("the copy is pinned by a world read")
    };
    let (part, body) = a_box("s3c_inline_twice_part", 1.0);
    let (part, placement) = placed(part, body);
    let part = fixture::pin(
        part,
        placement,
        &editor_core::Frame::from_affine(poses[0]),
        &EvalOptions::default(),
    );
    let pinned_depth = depth(&part);
    let mut store = fixture::resolver::PartStore::new();
    let mut doc = part;
    for (level, &at) in poses[1..].iter().enumerate() {
        let doc_ref = store.insert(doc, Tol::witness());
        let o = fixture::resolver::with_resolver(store.clone());
        let resolver: std::sync::Arc<dyn editor_core::PartResolver> =
            std::sync::Arc::new(store.clone());
        let host = ProfileDoc::empty_derived(&format!("s3c_inline_twice_{level}"), Tol::witness());
        let (host, instance) = fixture::insert(host, Node::instantiate_part(doc_ref));
        let host = fixture::pin(host, instance, &editor_core::Frame::from_affine(at), &o);
        doc = editor_core::inline(&host, instance, &resolver, Tol::witness())
            .unwrap_or_else(|e| panic!("inline {level} lands: {e}"))
            .doc;
        assert!(
            depth(&doc) <= pinned_depth,
            "inline {level} holds the pin's depth {pinned_depth}, not {}",
            depth(&doc)
        );
    }
    let [copy] = doc.placements()[..] else {
        panic!("one placement in the flattened host");
    };
    let at = solved(&doc, copy);
    let want = poses[2] * poses[1] * poses[0];
    let off = [at.linear.c0, at.linear.c1, at.linear.c2, at.translation]
        .iter()
        .zip([
            want.linear.c0,
            want.linear.c1,
            want.linear.c2,
            want.translation,
        ])
        .map(|(a, b)| (*a - b).norm())
        .fold(0.0_f64, f64::max);
    assert!(
        off < 1e-14,
        "the copy stands at the three poses composed, {off:e} off: {at:?}"
    );
}
