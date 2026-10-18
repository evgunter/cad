//! MSOLVE-8 acceptance — **a levered clash names its arm, the coset's
//! directions carry the witness** (the MSOLVE-8 spec's rows A1–A3 and
//! its review claims C1–C4).
//!
//! Three of the coset fold's membership margins are a pure number
//! levered by the bundle's arm — a sine, a Frobenius departure from the
//! identity, a reachability defect — and a refusal that quoted their
//! product as a bare metre figure hid the arm. The `c1` rows reach
//! each of the three through the ordinary doors with a document that
//! trips it and pin that the refusal carries
//! `Clash::Levered(Lever::Residual { value, arm })` with `value` the
//! number the predicate decided on, re-derived from the two mates'
//! cosets over the poses bound where the fold binds them, and `arm`
//! the lever the FOLD decided it over. The `c2` rows pin the witness
//! road: the parallel verdict and the line it mints are one decision
//! at every ulp around the band's edge, straight at the coset and
//! through the doors. The `c4` row measures the sentence on
//! `MateFault` about its mate-less `PosesOfAnotherDocument` arm. Every
//! row runs at `Tol::witness()`.
//!
//! The documents: a loose copy of a unit box `B`, its mates' targets
//! read off a second box `T` standing beside it — a construction, of
//! the world's kind — so every pose on both sides is a read the fold
//! binds and the row can bind again.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::Formula;

use editor_core::mate::coset::{Arm, Coset, FoldStop, Subgroup, intersect, intersect_subgroups};
use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    CapEnd, Clash, ContactClass, Coord, DocEdit, DocumentId, EvalOptions, Evaluation, Lever,
    MateFault, MateRef, MateSide, Node, NodeErrorClass, NodeErrorKind, NodeResult, Operand,
    OperandSlot, PoseValue, ProfileDoc, RecipeNodeId, RoleSeg, SitedRef, SlideAxis, Space,
    StableName, Value, VarId,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{FIXTURE_MATE_AXIS, ang, insert, len, run, solve};
use geom_core::Tol;
use geom_core::linalg::{Affine3, Mat3, UnitVec3, Vec3};
use geom_core::predicate::Band;

// ---- Substrate ----

/// `arm` through the one door a lever is formed by, as a datum term
/// over parts of no reach.
fn lever(arm: f64) -> Arm {
    Arm::of(0.0, arm).expect("a finite arm the format can decide over")
}

/// A unit box, the square `[x0, x0 + 1] × [0, 1]` on its own xy
/// extruded 1 tall, and its body.
fn unit_box(doc: ProfileDoc, x0: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = insert(
        doc,
        Node::Profile(fixture::desc(vec![vec![
            (x0, 0.0),
            (x0 + 1.0, 0.0),
            (x0 + 1.0, 1.0),
            (x0, 1.0),
        ]])),
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    )
}

/// The box `b` copied by the loose placement `copy`, the box `t` its
/// mates' targets are read off, standing beside it.
struct Rig {
    doc: ProfileDoc,
    b: RecipeNodeId,
    t: RecipeNodeId,
    copy: RecipeNodeId,
}

fn rig(label: &str) -> Rig {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, b) = unit_box(doc, 0.0);
    let (doc, t) = unit_box(doc, 3.0);
    let (doc, copy) = insert(
        doc,
        Node::place(vec![editor_core::BodyRead::plain(Operand::Node(b))]),
    );
    Rig { doc, b, t, copy }
}

impl Rig {
    /// A read of `name`, an entity of `body`, at `body`.
    fn at(body: RecipeNodeId, name: StableName) -> Operand {
        SitedRef::new(body, name).into()
    }

    /// `body`'s lateral edge at canonical profile vertex `v`, as an axis.
    fn lateral(&self, body: RecipeNodeId, v: usize) -> Operand {
        fixture::axis(Self::at(
            body,
            fixture::ename(
                body,
                RoleSeg::LateralEdge(fixture::vpiece(&self.doc, body, 0, v)),
            ),
        ))
    }

    /// `body`'s rim edge at `end` over canonical segment `k`, as an axis.
    fn rim(&self, body: RecipeNodeId, end: CapEnd, k: usize) -> Operand {
        fixture::axis(Self::at(
            body,
            fixture::rim_edge(body, end, fixture::piece(&self.doc, body, 0, k)),
        ))
    }

    /// `body`'s cap vertex at `end` over canonical profile vertex `v`,
    /// as a point.
    fn corner(&self, body: RecipeNodeId, end: CapEnd, v: usize) -> Operand {
        fixture::point_of(Self::at(
            body,
            fixture::cap_vertex(body, end, fixture::vpiece(&self.doc, body, 0, v)),
        ))
    }

    /// `body`'s cap at `end`, as a plane.
    fn cap(&self, body: RecipeNodeId, end: CapEnd) -> Operand {
        fixture::plane(fixture::cap_ref(body, end))
    }

    /// `body`'s side wall over canonical segment `k`, as a plane.
    fn side(&self, body: RecipeNodeId, k: usize) -> Operand {
        fixture::plane(Self::at(
            body,
            fixture::fname(
                body,
                RoleSeg::Lateral(fixture::piece(&self.doc, body, 0, k).into()),
            ),
        ))
    }

    /// Adds `on ≡ to` to the copy's bundle.
    fn add(&mut self, on: Operand, to: Operand) -> MateRef {
        let (doc, m) = fixture::constrain_at(
            self.doc.clone(),
            self.copy,
            fixture::mate(on, to, ContactClass::Rest),
        );
        self.doc = doc;
        m
    }

    /// The pose `mate`'s `side` reads, bound where the fold binds it.
    fn bound(&self, ev: &Evaluation<f64>, mate: MateRef, side: MateSide) -> PoseValue<f64> {
        let m = fixture::constraint(&self.doc, mate);
        let var: VarId = match side {
            MateSide::On => m.on,
            MateSide::To => *m.to.pose().expect("a target read off geometry"),
        };
        editor_core::eval::bound_pose(
            &self.doc,
            ev,
            OperandSlot::Side {
                constraint: mate.id,
                side,
            },
            RoleRead::plain(var),
            Tol::witness(),
        )
        .unwrap_or_else(|e| panic!("the {side:?} pose of {mate:?} binds: {e:?}"))
    }

    /// `mate`'s coset over its bound poses, as the fold forms it.
    fn coset(&self, ev: &Evaluation<f64>, mate: MateRef) -> Coset {
        editor_core::mate::mate_coset(
            &self.bound(ev, mate, MateSide::On),
            &self.bound(ev, mate, MateSide::To),
        )
    }

    /// **The bundle's lever, as the formula spells it**: the copied
    /// box's reach, plus the farthest any mate's two poses sit from
    /// their origins, grouped as the fold groups it.
    fn lever(&self, ev: &Evaluation<f64>, mates: &[MateRef]) -> f64 {
        let Some(editor_core::ValuePayload::Body(body)) = ev.value(self.b).map(|v| &v.payload)
        else {
            panic!("the box evaluates to a body");
        };
        let reach = editor_core::mate::part_reach(&**body).expect("the box is bounded");
        let datum = mates
            .iter()
            .map(|&m| {
                from_origin(&self.bound(ev, m, MateSide::On))
                    + from_origin(&self.bound(ev, m, MateSide::To))
            })
            .fold(0.0_f64, f64::max);
        reach + datum
    }

    /// The copy's bundle refusal, if it refused.
    fn fault(&self, ev: &Evaluation<f64>) -> Option<MateFault> {
        match ev.result(self.copy) {
            Some(NodeResult::Failed(e)) => match &e.kind {
                NodeErrorKind::Mate(fault) => Some((**fault).clone()),
                other => panic!("expected a mate refusal, got {other:?}"),
            },
            Some(NodeResult::Ok(_)) => None,
            other => panic!("the copy neither evaluated nor refused: {other:?}"),
        }
    }
}

/// A pose's distance from its coordinates' origin, as the fold's arm
/// spells it: its frame's translation's length.
fn from_origin(pose: &PoseValue<f64>) -> f64 {
    let o = match *pose {
        PoseValue::Point { position } => position - geom_core::linalg::Point3::origin(),
        PoseValue::Direction { .. } => Vec3::new(0.0, 0.0, 0.0),
        PoseValue::Axis { origin, .. } | PoseValue::Plane { origin, .. } => {
            origin - geom_core::linalg::Point3::origin()
        }
        PoseValue::Frame(f) => f.to_affine().translation,
    };
    (o.x.powi(2) + o.y.powi(2) + o.z.powi(2)).sqrt()
}

/// The frame through `axis` and `point` (origin the point's foot on
/// the axis, `z` the axis, `x` towards the point).
fn through(axis: Operand, point: Operand) -> Operand {
    Operand::Pose(Box::new(PoseDef::Through {
        axis: RoleRead::plain(axis),
        point: RoleRead::plain(point),
    }))
}

/// A pose slid `by` metres along an axis's direction.
fn slid(pose: Operand, along: Operand, by: f64) -> Operand {
    Operand::Pose(Box::new(PoseDef::Slide {
        pose: RoleRead::plain(pose),
        along: RoleRead::plain(along),
        by: len(by),
    }))
}

/// A plane slid `by` metres along its own normal.
fn lifted(plane: Operand, by: f64) -> Operand {
    Operand::Pose(Box::new(PoseDef::Slide {
        pose: RoleRead::plain(plane.clone()),
        along: RoleRead::through(plane, PoseRole::Normal),
        by: len(by),
    }))
}

/// A pose turned `by` radians about an axis.
fn spun(pose: Operand, about: Operand, by: f64) -> Operand {
    Operand::Pose(Box::new(PoseDef::Spin {
        pose: RoleRead::plain(pose),
        about: RoleRead::plain(about),
        by: fixture::ang(by),
    }))
}

/// The Frobenius norm of `q − I`, spelled as the predicate spells it.
fn frobenius_departure(q: Mat3<f64>) -> f64 {
    let i = Mat3::identity();
    let d = Mat3::from_cols(q.c0 - i.c0, q.c1 - i.c1, q.c2 - i.c2);
    (d.c0.norm_squared() + d.c1.norm_squared() + d.c2.norm_squared()).sqrt()
}

fn bits3(v: Vec3<f64>) -> [u64; 3] {
    [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()]
}

/// The residual lever a contradictory refusal carries, with the
/// sentence checked to name the predicate and both halves and to call
/// the number what it is.
fn residual_of(fault: &MateFault, predicate: &str) -> (f64, f64) {
    let MateFault::Contradictory {
        predicate: named,
        clash,
        ..
    } = fault
    else {
        panic!("expected CONTRADICTORY, got {fault:?}");
    };
    assert_eq!(*named, predicate);
    let Clash::Levered(Lever::Residual { value, arm }) = *clash else {
        panic!("`{predicate}` levers a pure number, so its refusal carries a residual: {clash:?}");
    };
    assert!(arm > 0.0, "the arm is the bundle's lever: {arm}");
    let message = fault.to_string();
    assert!(
        !message.contains(predicate),
        "the predicate's name rides the payload, not the sentence: {message:?}"
    );
    for want in [
        &format!("a dimensionless residual of {value}"),
        &format!("on a {arm} m arm"),
        &format!("a deviation of {} m", clash.deviation().unwrap()),
    ] {
        assert!(message.contains(want), "{message:?} is missing {want:?}");
    }
    assert!(
        !message.contains(" rad"),
        "a residual is never printed as radians: {message:?}"
    );
    (value, arm)
}

// ---- A1 / C1: the three residual predicates, value and arm ----

/// **`mate_member_rotation_identity` levers the Frobenius departure,
/// over the FOLD's arm.** Two frame mates on one copy frame, whose
/// targets — frames read off `T`, one standing up its lateral edge and
/// one lying along a rim edge — disagree in rotation, leave a trivial
/// residual whose candidate rotation is the held mate's; the added
/// mate's membership measures `‖Q − I‖_F` for the relative rotation
/// `Q`, re-derived here from the two cosets over the bound poses. The
/// arm is the bundle's: the copy's reach and the farthest pose pair.
///
/// *Breaks if* the residual is anything but the predicate's own
/// number, or the arm anything but the fold's.
#[test]
fn c1_rotation_identity_value_and_arm() {
    let mut r = rig("msolve8-c1-rot-id");
    let on = || through(r.lateral(r.b, 0), r.corner(r.b, CapEnd::Start, 1));
    let up = through(r.lateral(r.t, 0), r.corner(r.t, CapEnd::End, 1));
    let lying = through(r.rim(r.t, CapEnd::End, 0), r.corner(r.t, CapEnd::End, 2));
    let (on1, on2) = (on(), on());
    let held = r.add(on1, up);
    let added = r.add(on2, lying);
    let ev = run(&r.doc, &EvalOptions::default());
    let fault = r.fault(&ev).expect("the pair refuses");
    let (value, arm) = residual_of(&fault, "mate_member_rotation_identity");
    let q = r.coset(&ev, held).representative.linear
        * r.coset(&ev, added).representative.linear.inverse();
    assert_eq!(
        value.to_bits(),
        frobenius_departure(q).to_bits(),
        "the residual is the Frobenius departure the predicate decided on: {value}"
    );
    assert_eq!(
        arm.to_bits(),
        r.lever(&ev, &[held, added]).to_bits(),
        "the fold's arm is the bundle's"
    );
    let MateFault::Contradictory {
        held: h, added: a, ..
    } = &fault
    else {
        unreachable!()
    };
    assert_eq!((*h, *a), (held, added));
}

/// **`mate_member_axis_fixed` levers `‖Q·n − n‖`.** A frame mate holds
/// the copy rigid; a plane mate added between the copy's top cap and a
/// side wall of `T` asks the wall's normal to be fixed by a rotation
/// that carries the cap's onto it. The value is that departure,
/// re-derived from the target's own normal and the two cosets; the
/// arm is the bundle's.
///
/// *Breaks if* the residual is anything but the predicate's own
/// number, or the arm anything but the fold's.
#[test]
fn c1_axis_fixed_value_and_arm() {
    let mut r = rig("msolve8-c1-axis-fixed");
    let held = r.add(
        through(r.lateral(r.b, 0), r.corner(r.b, CapEnd::Start, 1)),
        through(r.lateral(r.t, 0), r.corner(r.t, CapEnd::End, 1)),
    );
    let added = r.add(r.cap(r.b, CapEnd::End), r.side(r.t, 0));
    let ev = run(&r.doc, &EvalOptions::default());
    let fault = r.fault(&ev).expect("the pair refuses");
    let (value, arm) = residual_of(&fault, "mate_member_axis_fixed");
    let q = r.coset(&ev, held).representative.linear
        * r.coset(&ev, added).representative.linear.inverse();
    let PoseValue::Plane { normal, .. } = r.bound(&ev, added, MateSide::To) else {
        panic!("the wall reads as a plane");
    };
    let n = normal.get();
    assert_eq!(
        value.to_bits(),
        (q * n - n).norm().to_bits(),
        "the residual is the normal's departure under the relative rotation: {value}"
    );
    assert_eq!(arm.to_bits(), r.lever(&ev, &[held, added]).to_bits());
}

/// **`mate_rotation_two_axis_reachable` levers the reach.** A rest of
/// the copy's bottom cap on `T`'s top cap, and a coaxial mate between
/// the copy's upright lateral edge and `T`'s lying rim edge: the
/// copy's two directions are parallel and the targets' perpendicular,
/// so no rotation reaches both. The reachability defect is the
/// difference of the two directions' inner products across the sides,
/// re-derived from the targets' own directions and the cosets'
/// rotations.
///
/// *Breaks if* the residual is anything but the predicate's own
/// number, or the arm anything but the fold's.
#[test]
fn c1_two_axis_reach_value_and_arm() {
    let mut r = rig("msolve8-c1-reach");
    let first = r.add(
        r.cap(r.b, CapEnd::Start),
        fixture::flip(r.cap(r.t, CapEnd::End)),
    );
    let second = r.add(r.lateral(r.b, 0), r.rim(r.t, CapEnd::End, 0));
    let ev = run(&r.doc, &EvalOptions::default());
    let fault = r.fault(&ev).expect("the pair refuses");
    let (value, arm) = residual_of(&fault, "mate_rotation_two_axis_reachable");
    let PoseValue::Plane { normal: a1, .. } = r.bound(&ev, first, MateSide::To) else {
        panic!("the cap reads as a plane");
    };
    let PoseValue::Axis { dir: a2, .. } = r.bound(&ev, second, MateSide::To) else {
        panic!("the rim reads as an axis");
    };
    let (a1, a2) = (a1.get(), a2.get());
    let (q1, q2) = (
        r.coset(&ev, first).representative.linear,
        r.coset(&ev, second).representative.linear,
    );
    let v = (q1 * q2.transpose()) * a2;
    let reach = v.dot(a1) - a2.dot(a1);
    assert_eq!(
        value.to_bits(),
        reach.to_bits(),
        "the residual is the reach the predicate decided on: {value} vs {reach}"
    );
    assert_eq!(arm.to_bits(), r.lever(&ev, &[first, second]).to_bits());
}

/// **A length predicate measures a length.** Two rests of the copy's
/// bottom cap on `T`'s top cap, the second lifted half a unit, refuse
/// on `translation_in_plane` with a `Clash::Length` of that half unit;
/// two coaxial mates of one lateral edge of the copy onto one of `T`'s
/// and the same slid half a unit along a rim refuse on a `mate_member_`
/// length.
///
/// *Breaks if* a translation's clash is quoted as anything but a
/// length.
#[test]
fn c1_a_length_predicate_measures_a_length() {
    let mut r = rig("msolve8-c1-len-plane");
    r.add(
        r.cap(r.b, CapEnd::Start),
        fixture::flip(r.cap(r.t, CapEnd::End)),
    );
    r.add(
        r.cap(r.b, CapEnd::Start),
        fixture::flip(lifted(r.cap(r.t, CapEnd::End), 0.5)),
    );
    let ev = run(&r.doc, &EvalOptions::default());
    let fault = r.fault(&ev).expect("the pair refuses");
    let MateFault::Contradictory {
        predicate, clash, ..
    } = &fault
    else {
        panic!("{fault:?}")
    };
    assert_eq!(*predicate, "mate_member_translation_in_plane");
    let Clash::Length { metres } = *clash else {
        panic!("a translation predicate measures a LENGTH outright: {clash:?}");
    };
    assert!((metres.abs() - 0.5).abs() < 1e-12, "{metres}");
    assert!(fault.to_string().contains("a clash of"), "{fault}");

    let mut r = rig("msolve8-c1-len-axis");
    r.add(r.lateral(r.b, 0), r.lateral(r.t, 0));
    r.add(
        r.lateral(r.b, 0),
        slid(r.lateral(r.t, 0), r.rim(r.t, CapEnd::End, 0), 0.5),
    );
    let ev = run(&r.doc, &EvalOptions::default());
    let fault = r.fault(&ev).expect("the pair refuses");
    let MateFault::Contradictory {
        predicate, clash, ..
    } = &fault
    else {
        panic!("{fault:?}")
    };
    assert!(predicate.starts_with("mate_member_"), "{predicate}");
    assert!(matches!(clash, Clash::Length { .. }), "{clash:?}");
}

// ---- A2 / C2: the witness is the column, and one decision at the band's edge ----

/// The in-plane perturbation shapes a boundary search tilts a normal
/// by.
const SHAPES: [(f64, f64); 6] = [
    (1.0, 0.0),
    (0.0, 1.0),
    (0.6, 0.8),
    (
        -core::f64::consts::FRAC_1_SQRT_2,
        core::f64::consts::FRAC_1_SQRT_2,
    ),
    (0.3, -0.9),
    (0.28, 0.96),
];

/// `n1` tilted by `s` along `shape` in the plane perpendicular to it,
/// as a witness.
fn tilted(n1: Vec3<f64>, shape: (f64, f64), s: f64, band: Band) -> Option<UnitVec3<f64>> {
    let (b1, b2) = n1.orthonormal_basis();
    UnitVec3::new(
        n1 + b1 * (shape.0 * s) + b2 * (shape.1 * s),
        FIXTURE_MATE_AXIS,
        band,
    )
    .ok()
}

/// The tilt at which the levered sine crosses the band's escalate
/// threshold, by bisection, so a sweep of the ulps around it straddles
/// the edge.
fn boundary_tilt(n1: Vec3<f64>, shape: (f64, f64), arm: f64, band: Band) -> f64 {
    let escalate = band.escalate();
    let (mut lo, mut hi) = (escalate / arm * 0.25, escalate / arm * 4.0);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if mid <= lo || mid >= hi {
            break;
        }
        match tilted(n1, shape, mid, band).map(|n2| (n1.cross(n2.get()) * arm).norm()) {
            Some(l) if l >= escalate => hi = mid,
            _ => lo = mid,
        }
    }
    hi
}

/// The verdict the ONE spelling of the parallel margin gives — the
/// levered vector's norm against the band — for two witnesses at an
/// arm.
fn one_spelling(n1: Vec3<f64>, n2: Vec3<f64>, arm: f64, band: Band) -> &'static str {
    let m = (n1.cross(n2) * arm).norm();
    if m <= band.zero() {
        "parallel"
    } else if m >= band.escalate() {
        "line"
    } else {
        "escalates"
    }
}

/// **The parallel verdict and the line it mints are ONE decision.**
/// Two planar subgroups whose normals sit within ±400 ulps of the
/// tilt at which the levered sine crosses K·ε, over four normals,
/// seven arms and six tilt shapes (134 400 pairs): the table's answer
/// — the planar residual, the prismatic line, or the escalation under
/// `mate_axes_parallel` — is exactly the classification of
/// `‖(n1 × n2) · arm‖` against the band at every ulp, and the line
/// is that vector's own normalize. Spelled as two decisions (the sine
/// times the arm decided, then the levered vector minted) the two
/// lengths differ by up to two ulps and straddle the edge; spelled
/// once they cannot, and no escalation names any other predicate.
///
/// The full coset door answers the same: the parallel pair keeps its
/// plane, the line solves (the translation stage divides by the sine
/// the table decided, so a line the table mints is a pose), and the
/// escalation is the table's own.
#[test]
fn c2_parallel_boundary_direct() {
    let band = Band::linear(Tol::witness()).unwrap();
    let unit = |v: Vec3<f64>| UnitVec3::new(v, FIXTURE_MATE_AXIS, band).unwrap();
    let (mut total, mut escalations, mut lines, mut planes) = (0_usize, 0_usize, 0_usize, 0_usize);
    for n1 in [
        unit(Vec3::new(0.0, 0.0, 1.0)),
        unit(Vec3::new(0.3, 0.2, 1.0)),
        unit(Vec3::new(-0.7, 0.1, 0.4)),
        unit(Vec3::new(1.0, 1.0, 1.0)),
    ] {
        for arm in [1.0_f64, 2.0, 2.449_489_742_783_178, 3.7, 7.25, 123.456, 1e3] {
            for shape in SHAPES {
                let base = boundary_tilt(n1.get(), shape, arm, band).to_bits() as i64;
                for k in -400_i64..400 {
                    let s = f64::from_bits((base + k) as u64);
                    let Some(n2) = tilted(n1.get(), shape, s, band) else {
                        continue;
                    };
                    total += 1;
                    let held = Coset {
                        subgroup: Subgroup::Planar { normal: n1 },
                        representative: Affine3::identity(),
                    };
                    let added = Coset {
                        subgroup: Subgroup::Planar { normal: n2 },
                        representative: Affine3::identity(),
                    };
                    let want = one_spelling(n1.get(), n2.get(), arm, band);
                    let got = match intersect_subgroups(
                        held.subgroup,
                        added.subgroup,
                        band,
                        lever(arm),
                    ) {
                        Ok(Subgroup::Planar { .. }) => "parallel",
                        Ok(Subgroup::Prismatic { direction }) => {
                            let line = n1.get().cross(n2.get()) * arm;
                            assert_eq!(
                                bits3(direction.get()),
                                bits3(line.normalize()),
                                "the line is the levered cross product's own normalize"
                            );
                            "line"
                        }
                        Err(d) => {
                            assert_eq!(d.predicate, Some("mate_axes_parallel"), "{d:?}");
                            "escalates"
                        }
                        other => panic!("at s={s:e} arm={arm}: {other:?}"),
                    };
                    let door = match intersect(held, added, band, lever(arm)) {
                        Ok(Coset {
                            subgroup: Subgroup::Planar { .. },
                            ..
                        }) => "parallel",
                        Ok(Coset {
                            subgroup: Subgroup::Prismatic { .. },
                            ..
                        }) => "line",
                        Err(FoldStop::Indeterminate(d))
                            if d.predicate == Some("mate_axes_parallel") =>
                        {
                            "escalates"
                        }
                        other => panic!("at s={s:e} arm={arm}: {other:?}"),
                    };
                    assert_eq!(door, got, "the full door at s={s:e} arm={arm}");
                    assert_eq!(
                        got,
                        want,
                        "n1={:?} arm={arm} shape={shape:?} s={s:e}",
                        n1.get()
                    );
                    match got {
                        "parallel" => planes += 1,
                        "line" => lines += 1,
                        _ => escalations += 1,
                    }
                }
            }
        }
    }
    assert!(
        escalations > 0 && lines > 0,
        "the sweep straddles the edge: {total} pairs, {planes} planar, {lines} lines, \
         {escalations} escalations"
    );
}

/// The copy's bottom cap resting on `T`'s top cap, and on `T`'s top
/// cap turned `s` about one of its rims: two plane mates whose target
/// normals stand `s` apart. Answers the rig, its evaluation and the two
/// mates.
fn two_rests(s: f64) -> (Rig, Evaluation<f64>, [MateRef; 2]) {
    let mut r = rig("msolve8-c2-doors");
    let first = r.add(
        r.cap(r.b, CapEnd::Start),
        fixture::flip(r.cap(r.t, CapEnd::End)),
    );
    let second = r.add(
        r.cap(r.b, CapEnd::Start),
        fixture::flip(spun(r.cap(r.t, CapEnd::End), r.rim(r.t, CapEnd::End, 0), s)),
    );
    let ev = run(&r.doc, &EvalOptions::default());
    (r, ev, [first, second])
}

/// The verdict the ONE spelling of the parallel margin gives the two
/// rests' bound target normals over the bundle's bound lever.
fn want(r: &Rig, ev: &Evaluation<f64>, mates: [MateRef; 2], band: Band) -> &'static str {
    let normal = |m| match r.bound(ev, m, MateSide::To) {
        PoseValue::Plane { normal, .. } => normal.get(),
        other => panic!("a cap reads as a plane, got {other:?}"),
    };
    one_spelling(
        normal(mates[0]),
        normal(mates[1]),
        r.lever(ev, &mates),
        band,
    )
}

/// **Through the doors, at the edge.** Two rests of one copy cap on
/// target planes `s` apart, `s` stepped an ulp at a time across the
/// tilt at which the parallel verdict's levered sine crosses K·ε, found
/// by bisection over the doors' own verdicts: at every step the fold's
/// answer is the one spelling's over the poses and lever it bound —
/// planes it calls parallel leave the copy free on them, a line it
/// mints refuses the copy (one cap cannot rest on two planes at an
/// angle) on any predicate but the parallel one, and an escalation is
/// the parallel predicate's own. The sweep straddles the edge.
///
/// *Breaks if* the door decides the parallel verdict by any spelling
/// but the levered vector's norm.
#[test]
fn c2_parallel_boundary_through_doors() {
    let band = Band::linear(Tol::witness()).unwrap();
    let verdict = |s: f64| {
        let (r, ev, mates) = two_rests(s);
        let got = match r.fault(&ev) {
            None => "parallel",
            Some(MateFault::Indeterminate { diag, .. })
                if diag.predicate == Some("mate_axes_parallel") =>
            {
                "escalates"
            }
            Some(fault) => {
                assert!(
                    !matches!(fault, MateFault::Indeterminate { ref diag, .. }
                        if diag.predicate == Some("mate_axes_parallel")),
                    "{fault:?}"
                );
                "line"
            }
        };
        (got, want(&r, &ev, mates, band))
    };
    // The edge: the copy's lever is a few units, so the escalate tilt
    // lies within a factor of ten either way of K·ε over one unit.
    let (mut lo, mut hi) = (band.escalate() * 0.1, band.escalate() * 10.0);
    assert_eq!(
        verdict(lo).1,
        "escalates",
        "below the edge the band escalates"
    );
    assert_eq!(verdict(hi).1, "line", "above it the table mints a line");
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if mid <= lo || mid >= hi {
            break;
        }
        if verdict(mid).1 == "line" {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    let (mut lines, mut escalations) = (0_usize, 0_usize);
    let base = hi.to_bits() as i64;
    for k in -8_i64..8 {
        let s = f64::from_bits((base + k) as u64);
        let (got, want) = verdict(s);
        assert_eq!(got, want, "at s = {s:e}");
        match got {
            "line" => lines += 1,
            "escalates" => escalations += 1,
            _ => {}
        }
    }
    assert!(
        lines > 0 && escalations > 0,
        "the sweep straddles the edge: {lines} lines, {escalations} escalations"
    );
}

// ---- C4: the mate-less arm, measured ----

/// `doc` with the freedom `coord` of `mate` set to `by`.
fn with_value(doc: ProfileDoc, mate: MateRef, coord: Coord, by: Formula) -> ProfileDoc {
    editor_core::apply(
        &doc,
        &DocEdit::AddValue {
            placement: mate.placement,
            value: Box::new(Value {
                mate: mate.id,
                coord,
                by,
            }),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .unwrap_or_else(|e| panic!("the value is admitted: {e}"))
    .doc
}

/// **`PosesOfAnotherDocument` reaches no row.** A document of five
/// instances — three pinned to the world, two seated on two of them —
/// records it against nothing; `SolvedPoses::placement` raises it for
/// every instance when read against another document and places every
/// one against its own; and no node result of the evaluation carries
/// it.
///
/// *Breaks if* a solve answers a document it is not of, or a row
/// carries the mispairing.
#[test]
fn c4_poses_of_another_document_reaches_no_row() {
    let mut store = PartStore::new();
    let (part, body) = unit_box(
        ProfileDoc::empty(
            DocumentId::derive("msolve8-c4-mispair-part"),
            Tol::witness(),
        ),
        0.0,
    );
    let (doc_ref, body) = store.insert_part((part, body), Tol::witness());
    let o = with_resolver(store);
    let mut doc = ProfileDoc::empty(DocumentId::derive("msolve8-c4-mispair"), Tol::witness());
    let mut ids = Vec::new();
    for i in 0..5 {
        let (next, id) = if i % 2 == 0 {
            fixture::insert_pinned(doc, Node::instantiate_part(doc_ref), &o)
        } else {
            insert(doc, fixture::mated_instance(doc_ref))
        };
        doc = next;
        ids.push(id);
    }
    for (lower, upper) in [(ids[0], ids[1]), (ids[2], ids[3])] {
        let (next, rest) = fixture::constrain_at(
            doc,
            upper,
            fixture::rest(
                fixture::head(in_part(upper, body, CapEnd::Start)),
                fixture::head(in_part(lower, body, CapEnd::End)),
                ContactClass::Rest,
            ),
        );
        doc = [
            (Coord::Slide(SlideAxis::X), len(0.0)),
            (Coord::Slide(SlideAxis::Y), len(0.0)),
            (Coord::Spin, ang(0.0)),
        ]
        .into_iter()
        .fold(next, |doc, (coord, by)| with_value(doc, rest, coord, by));
    }
    let tol = Tol::witness();
    let poses = solve(&doc, &o, tol);
    for id in doc.ids() {
        assert!(
            !matches!(
                poses.fault(id),
                Some(MateFault::PosesOfAnotherDocument { .. })
            ),
            "{id:?}: {:?}",
            poses.fault(id)
        );
    }
    let other = ProfileDoc::empty(DocumentId::derive("msolve8-c4-mispair-other"), tol);
    for &id in &ids {
        assert!(
            matches!(
                poses.placement(&other, id),
                Err(editor_core::PoseRefusal::Mate(ref f))
                    if matches!(**f, MateFault::PosesOfAnotherDocument { .. })
            ),
            "{id:?}"
        );
        assert!(
            poses.placement(&doc, id).is_ok(),
            "{id:?}: the own document places"
        );
    }
    let ev = run(&doc, &o);
    for id in doc.ids() {
        let Some(e) = ev.node_error(id) else {
            continue;
        };
        let classes: Vec<NodeErrorClass> = core::iter::once(e.kind.class())
            .chain(e.kind.carried_chain().map(|l| l.refusal.kind().class()))
            .collect();
        assert!(
            !classes.contains(&NodeErrorClass::MatePosesOfAnotherDocument),
            "{id:?} refuses on another document's poses: {e}"
        );
    }
}

// ---- A3: verdicts that must not move under the witness ----

/// **A determined bundle determines, and a V-block leaves its line.**
/// The A11 rule-1 exemplar — a coaxial mate and its slide and spin
/// values — pins the copy standing on `T`, its lateral edge on `T`'s
/// and slid the box's unit height along it; two plane mates on two
/// adjacent side walls, flush with `T`'s (the planes' line minted by
/// the parallel verdict), leave the prismatic residual along that
/// line, so that copy is loose.
///
/// *Breaks if* the exemplar's verdicts move under the witness.
#[test]
fn a_determined_bundle_and_a_v_block_keep_their_verdicts_under_the_witness() {
    let mut r = rig("msolve8-determined");
    let coaxial = r.add(r.lateral(r.b, 0), r.lateral(r.t, 0));
    r.doc = with_value(r.doc, coaxial, Coord::Slide(SlideAxis::Z), len(1.0));
    r.doc = with_value(r.doc, coaxial, Coord::Spin, ang(0.0));
    let poses = solve(&r.doc, &EvalOptions::default(), Tol::witness());
    assert!(
        poses.fault(r.copy).is_none() && poses.space(r.copy) == Some(Space::Loose { root: r.t }),
        "the exemplar is DETERMINED: {:?}",
        poses.fault(r.copy)
    );
    let seated = poses.relative(r.copy).expect("the copy is placed");
    let close = |a: &[f64], b: &[f64]| a.iter().zip(b).all(|(x, y)| (x - y).abs() <= 1e-12);
    let identity = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    assert!(
        seated
            .columns
            .iter()
            .zip(&identity)
            .all(|(a, b)| close(a, b))
            && close(&seated.translation, &[3.0, 0.0, 1.0]),
        "the copy stands on `T`, flush: {seated:?}"
    );

    let mut r = rig("msolve8-v-block");
    for k in [0, 1] {
        r.add(r.side(r.b, k), r.side(r.t, k));
    }
    let poses = solve(&r.doc, &EvalOptions::default(), Tol::witness());
    assert_eq!(poses.fault(r.copy), None, "the V-block refuses nothing");
    assert_eq!(
        poses.space(r.copy),
        Some(Space::Loose { root: r.copy }),
        "the V-block leaves its copy loose"
    );
}
