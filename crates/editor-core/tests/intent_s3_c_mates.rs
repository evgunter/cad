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
use editor_core::mate::coset::{Arm, Coset, FoldStop, Subgroup, intersect, intersect_subgroups};
use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    BodyRead, ConstraintId, ContactClass, Coord, DocEdit, EditError, EvalOptions, ExtrudeSide,
    Formula, Mate, MateFault, MateRef, Node, NodeErrorKind, NodeResult, Operand, ProfileDoc,
    RecipeNodeId, RefusingReach, SlideAxis, StableName, Target, Value, WorldRead,
};
use geom_core::Tol;
use geom_core::linalg::{Affine3, Mat3, Point3, UnitVec3, Vec3};
use geom_core::predicate::Band;

fn band() -> Band {
    Band::linear(Tol::witness()).expect("a band")
}

fn arm() -> Arm {
    Arm::of(0.0, 10.0).expect("ten metres is in range")
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
        Subgroup::Trivial | Subgroup::Empty => Affine3::identity(),
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

/// The pairs whose generic draw would miss the table's special
/// verdicts: a ball on a line or at the other ball, a direction along
/// a plane's normal or a line, a line lying in a plane.
fn aligned(d: &mut Draw) -> Vec<(Subgroup<f64>, Subgroup<f64>)> {
    let u = d.direction();
    let p = d.point();
    let across = unit(u.get().cross(d.vec()));
    vec![
        (
            Subgroup::Spherical {
                point: p + u.get() * 1.5,
            },
            Subgroup::Cylindrical {
                point: p,
                direction: u,
            },
        ),
        (
            Subgroup::Spherical { point: p },
            Subgroup::Revolute {
                point: p + u.get() * 0.5,
                direction: u,
            },
        ),
        (
            Subgroup::Spherical { point: p },
            Subgroup::Spherical { point: p },
        ),
        (
            Subgroup::Parallel { direction: u },
            Subgroup::Planar { normal: u },
        ),
        (
            Subgroup::Parallel { direction: u },
            Subgroup::Cylindrical {
                point: p,
                direction: u,
            },
        ),
        (
            Subgroup::Parallel { direction: u },
            Subgroup::Revolute {
                point: p,
                direction: u,
            },
        ),
        (
            Subgroup::Parallel { direction: u },
            Subgroup::Parallel { direction: u },
        ),
        (
            Subgroup::PlaneTranslation { normal: u },
            Subgroup::Planar { normal: u },
        ),
        (
            Subgroup::PlaneTranslation { normal: u },
            Subgroup::Cylindrical {
                point: p,
                direction: across,
            },
        ),
        (
            Subgroup::PlaneTranslation { normal: u },
            Subgroup::Prismatic { direction: across },
        ),
    ]
}

/// **Two cosets sharing a pose meet in a coset of the table's residual
/// that holds it**, in either order: each coset is `g·h⁻¹·x*` for a
/// drawn element `h` of `g`, so `x*` lies in both and the intersection
/// is not empty. The representative the fold builds must lie in both
/// (the fold's own membership check) and its coset must hold `x*`
/// (checked here against `x*`'s trivial coset).
///
/// *Breaks if* a pair with a ball, a direction or the translations has
/// no representative ([`FoldStop`] refuses), or one that leaves `x*`
/// out: a ball's turn about a line taken from the line's
/// representative rather than solved puts the ball's point elsewhere,
/// and the fold refuses a pair that meets.
#[test]
fn every_pair_of_families_sharing_a_pose_folds_to_a_coset_holding_it() {
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
    for _ in 0..4 {
        for (g1, g2) in aligned(&mut d) {
            pairs.push((g1, g2));
            pairs.push((g2, g1));
        }
    }
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
        let met = intersect(held, added, band(), arm())
            .unwrap_or_else(|stop| panic!("{at}: two cosets sharing a pose refused {stop:?}"));
        let residual = intersect_subgroups(g1, g2, band(), arm()).expect("the table decides");
        assert_eq!(met.subgroup.family(), residual.family(), "{at}");
        let pinned = Coset {
            subgroup: Subgroup::Trivial,
            representative: x,
        };
        intersect(met, pinned, band(), arm())
            .unwrap_or_else(|stop| panic!("{at}: the coset leaves the shared pose out: {stop:?}"));
    }
}

/// **A ball held at the wrong radius from a line, or two balls whose
/// chords differ, refuse with the clash measured**: the representative
/// stage turns as far as an angle can carry, and the membership check
/// names what no angle reaches.
///
/// *Breaks if* the mismatch is solved away (a turn that scales, or a
/// representative taken from one side alone that the other's check
/// never measures) or refuses without a measured clash.
#[test]
fn a_ball_off_its_radius_or_chord_refuses_with_the_clash_measured() {
    let z = unit(Vec3::new(0.0, 0.0, 1.0));
    let line = Coset {
        subgroup: Subgroup::Cylindrical {
            point: Point3::origin(),
            direction: z,
        },
        representative: Affine3::identity(),
    };
    // The ball's own point sits 1 m off the axis; the line's coset
    // carries it to a point 1 m off; the ball asks for 2 m.
    let ball = Coset {
        subgroup: Subgroup::Spherical {
            point: Point3::new(2.0, 0.0, 0.5),
        },
        representative: Affine3::from_parts(Mat3::identity(), Vec3::new(1.0, 0.0, 0.0)),
    };
    for (held, added) in [(ball, line), (line, ball)] {
        match intersect(held, added, band(), arm()) {
            Err(FoldStop::Clash { clash, .. }) => {
                assert!(clash.deviation().is_some_and(|m| m > 0.5), "{clash:?}");
            }
            other => panic!("a ball off its radius: {other:?}"),
        }
    }
    let ball_at = |point: Point3<f64>, shift: Vec3<f64>| Coset {
        subgroup: Subgroup::Spherical { point },
        representative: Affine3::from_parts(Mat3::identity(), shift),
    };
    // Copy points 1 m apart; the target points 3 m apart.
    let first = ball_at(Point3::origin(), Vec3::zero());
    let second = ball_at(Point3::new(0.0, 3.0, 0.0), Vec3::new(0.0, 2.0, 0.0));
    match intersect(first, second, band(), arm()) {
        Err(FoldStop::Clash { clash, .. }) => {
            assert!(clash.deviation().is_some_and(|m| m > 1.0), "{clash:?}");
        }
        other => panic!("two balls whose chords differ: {other:?}"),
    }
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
            side: ExtrudeSide::Along,
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
            value: Box::new(Value { mate, coord, by }),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &RefusingReach,
    )?;
    let id = applied.record.minted_constraints[0];
    Ok((applied.doc, id))
}

/// The value's number for `coord`: a length or an angle.
fn number(coord: Coord, v: f64) -> Formula {
    match coord {
        Coord::Slide(_) => len(v),
        Coord::Spin => ang(v),
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

/// The fault `placement` evaluated to.
fn evaluated_fault(doc: &ProfileDoc, placement: RecipeNodeId) -> NodeErrorKind {
    let ev = fixture::run(doc, &EvalOptions::default());
    match ev.result(placement) {
        Some(NodeResult::Failed(e)) => NodeErrorKind::Mate(match &e.kind {
            NodeErrorKind::Mate(fault) => fault.clone(),
            other => panic!("the placement refused otherwise: {other:?}"),
        }),
        other => panic!("the placement evaluated: {other:?}"),
    }
}

/// The angle of the turn `a·bᵀ`.
fn turn_between(a: Mat3<f64>, b: Mat3<f64>) -> f64 {
    let r = a * b.transpose();
    let trace = r.c0.x + r.c1.y + r.c2.z;
    ((trace - 1.0) / 2.0).clamp(-1.0, 1.0).acos()
}

/// A box resting on the world floor by its bottom face, with values
/// `(slide x, slide y, spin)` written in `order`.
fn box_on_the_floor(values: [f64; 3], order: [usize; 3]) -> (ProfileDoc, RecipeNodeId) {
    let (doc, body) = a_box("s3c_floor", 1.0);
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
    let coords = [
        Coord::Slide(SlideAxis::X),
        Coord::Slide(SlideAxis::Y),
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
        (Coord::Slide(SlideAxis::X), len(0.0)),
        (Coord::Slide(SlideAxis::Y), len(0.0)),
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
        Coord::Slide(SlideAxis::X),
        Coord::Slide(SlideAxis::Y),
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

/// **Ruling C-3 — a freedom the bundle's other constraints fix is the
/// placement's refusal where it evaluates.** A box resting on the floor
/// by its bottom and against the world's `+x` wall by its `x`-face: the
/// two plane mates leave only a slide along `y`. A spin value on the
/// floor mate is admitted at the door (the plane's kind leaves a spin)
/// and refused at evaluation, `NotAFreedom` with no kind; a slide along
/// the floor mate's `y` is a freedom the bundle leaves, and places.
///
/// *Breaks if* the door folds the bundle to refuse it (C-3's interim is
/// that the residual's refusals are the evaluation's), or the
/// evaluation turns the copy anyway and breaks the wall mate.
#[test]
fn a_value_the_bundle_already_fixes_refuses_where_it_evaluates() {
    let (doc, body) = a_box("s3c_corner", 1.0);
    let bottom = face_toward(&doc, body, [0.0, 0.0, -1.0]);
    let side = face_toward(&doc, body, [1.0, 0.0, 0.0]);
    let (doc, placement) = placed(doc, body);
    let (doc, floor) = fixture::constrain_at(
        doc,
        placement,
        Mate {
            on: plane_of(on_body(body, bottom)),
            to: Target::World(WorldRead::through(PoseRole::Xy).flip()),
            class: ContactClass::Rest,
        },
    );
    let (doc, _wall) = fixture::constrain_at(
        doc,
        placement,
        Mate {
            on: plane_of(on_body(body, side)),
            to: Target::World(WorldRead::through(PoseRole::Yz)),
            class: ContactClass::Rest,
        },
    );
    let (spun, value) = add_value(doc.clone(), placement, floor.id, Coord::Spin, ang(0.3))
        .expect("the door admits a spin a plane mate leaves");
    match evaluated_fault(&spun, placement) {
        NodeErrorKind::Mate(fault) => assert!(
            matches!(
                *fault,
                MateFault::NotAFreedom {
                    value: at,
                    coord: Coord::Spin,
                    kind: None,
                } if at == MateRef::new(placement, value)
            ),
            "{fault}"
        ),
        other => panic!("the placement refuses the value: {other:?}"),
    }
    // The freedom the bundle leaves is the slide along the world's y
    // (the floor chart's swung y).
    let (slid, _) = add_value(
        doc,
        placement,
        floor.id,
        Coord::Slide(SlideAxis::Y),
        len(2.0),
    )
    .expect("a plane mate leaves the slide");
    let at = solved(&slid, placement);
    assert!(
        (at.translation.y - 2.0).abs() < 1e-12,
        "slid along y by its value: {at:?}"
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
            &RefusingReach,
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
    let (mut doc, mate) = fixture::constrain_at(doc, placement, placed_by.mate);
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

/// **Inline refuses a moving landing for a value charted on the part's
/// world** (C-4). A part whose one copy rests on its world floor by a
/// mate and three values, instanced in a host and pinned off the
/// identity: the part's world mates would move with the instance, but a
/// value is charted on the world's own frame, which stays, so inline
/// refuses `ValueOnAMovedWorld` naming the value. Pinned at the
/// identity, the same instance inlines.
///
/// *Breaks if* the inline moves the mate's world read and leaves the
/// value charted on the unmoved frame (the copy lands at its numbers in
/// the host's world, not moved with the rest).
#[test]
fn inline_refuses_a_moved_landing_under_a_world_value() {
    let (part, p) = box_on_the_floor([0.5, 0.0, 0.0], [0, 1, 2]);
    let value = fixture::bundle(&part, p)
        .iter()
        .find(|c| c.value().is_some())
        .expect("a value")
        .id;
    let mut store = fixture::resolver::PartStore::new();
    let part_ref = store.insert(part, Tol::witness());
    let o = fixture::resolver::with_resolver(store.clone());
    let resolver: std::sync::Arc<dyn editor_core::PartResolver> = std::sync::Arc::new(store);
    let host = ProfileDoc::empty_derived("s3c_inline_host", Tol::witness());
    let (host, instance) = fixture::insert(host, Node::instantiate_part(part_ref));
    let moved = fixture::pin(
        host.clone(),
        instance,
        &editor_core::Frame::translation([3.0, 0.0, 0.0]),
        &o,
    );
    match editor_core::inline(&moved, instance, &resolver, Tol::witness()) {
        Err(editor_core::InlineError::ValueOnAMovedWorld {
            placement,
            value: named,
        }) => {
            assert_eq!(placement.id(), p, "the part placement is named");
            assert_eq!(named, value, "the value is named");
        }
        other => panic!("a moving landing under a world value refuses: {other:?}"),
    }
    let still = fixture::pinned(host, instance, &o);
    editor_core::inline(&still, instance, &resolver, Tol::witness())
        .expect("at the identity the part's world is the host's");
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
        (Coord::Slide(SlideAxis::X), slide),
        (Coord::Slide(SlideAxis::Y), len(0.0)),
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
    let doc = fixture::pinned(doc, placement, &EvalOptions::default());
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
/// derivative.** The box slid by `s`, declared at 0: at the nominal it
/// stands at the identity, but a dual run seeded at `s` carries the
/// slide's unit tangent, so its map is kept and its translation's
/// derivative along x is one.
///
/// *Breaks if* the dual lane snaps on its value channel alone (the
/// derivative would read zero).
#[test]
fn a_parametric_pose_at_a_nominal_identity_keeps_its_derivative() {
    let (doc, placement, seeded) = box_slid_by_a_parameter(0.0);
    assert!(
        motion_of(&evaluated::<f64>(&doc, &EvalOptions::default()), placement).is_none(),
        "at the nominal the copy is at rest"
    );
    let motion = motion_of(&evaluated::<geom_core::Dual64>(&doc, &seeded), placement)
        .expect("a seeded pose keeps its map");
    let x = motion.to_affine().translation.x;
    assert_eq!(x.value.to_bits(), 0.0_f64.to_bits(), "{x:?}");
    assert_eq!(x.deriv.to_bits(), 1.0_f64.to_bits(), "{x:?}");
}

/// **Fork C-5 — a seeded copy at rest has the value lane's bits and
/// its own tangent.** A box on the world floor spun by `t`, declared at
/// a whole turn: the `f64` lane's map is the identity to rounding, and
/// the fold places the copy by no map. A dual run seeded at `t` places
/// it by a map whose value channel is the identity's exact bits — the
/// `f64` lane's answer — and whose tangent is the spin's: the turn
/// about `z` moves the copy's `x` along `y` at unit rate.
///
/// *Breaks if* the dual lane keeps the unsnapped map (its value channel
/// rounds off the `f64` lane's), or snaps the map whole (the tangent is
/// lost).
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
        (Coord::Slide(SlideAxis::X), len(0.0)),
        (Coord::Slide(SlideAxis::Y), len(0.0)),
        (Coord::Spin, turn),
    ] {
        doc = add_value(doc, placement, mate.id, coord, by)
            .expect("a plane mate leaves the freedom")
            .0;
    }
    assert!(
        motion_of(&evaluated::<f64>(&doc, &EvalOptions::default()), placement).is_none(),
        "a whole turn is at rest in the value lane"
    );
    let seeded = EvalOptions {
        seed: doc.var_named("t"),
        ..EvalOptions::default()
    };
    let map = motion_of(&evaluated::<geom_core::Dual64>(&doc, &seeded), placement)
        .expect("a seeded pose keeps its map")
        .to_affine();
    let entries =
        [map.linear.c0, map.linear.c1, map.linear.c2, map.translation].map(|c| [c.x, c.y, c.z]);
    let identity = [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 0.0],
    ];
    for (column, want) in entries.iter().zip(identity) {
        for (e, w) in column.iter().zip(want) {
            assert_eq!(e.value.to_bits(), f64::to_bits(w), "{map:?}");
        }
    }
    assert!(
        (entries[0][1].deriv - 1.0).abs() < 1e-12 && (entries[1][0].deriv + 1.0).abs() < 1e-12,
        "the turn's tangent moves x along y: {map:?}"
    );
}
