//! MSOLVE-14 acceptance — **the mate solve runs at the evaluation's own
//! scalar** (`ASSEMBLY.md` A11 (5): the fold's inputs are read at the
//! evaluation's scalar and over its parameters).
//!
//! A placement folds its bundle where it evaluates, so every pose its
//! mates read and every number its values and pose constructions hold
//! is bound in the evaluation's own lane. The rows, by the spec's
//! acceptance:
//!
//! - **A3, the `f64` fence**: a mate corpus — every kind of mate the
//!   fold has a branch for, a LOOSE bundle and an OVERCONSTRAINED one,
//!   a parametric seat under a family, a parametric standoff, a
//!   parametric world mate, a chain of bundles — solved and evaluated
//!   at `f64`, digested, and pinned; and the same corpus at `Dual64`
//!   reading the `f64` bits on its value channel.
//! - **A2**: a parameter a bundle reads — a value's slide, a standoff,
//!   a value's spin — moves the mated part in the run that binds it,
//!   on a `Dual64` seed run by its tangent and on an `Interval` box run
//!   by an enclosure: the bolt and its row (∂B/∂s = −2, ∂copy2/∂s = 0),
//!   a standoff and a slid, spun seat.
//! - **A4**: a seeded pass threaded an unseeded `Dual64` prior reuses no
//!   zero-tangent pose (the memo key carries the pose's two channels).
//! - **A5**: `stackup::sensitivities` and certified `clearance` over the
//!   seated bolt.
//! - **C5**: one document's structure — spaces, faults' presence — is
//!   the same in every lane.
//!
//! Every analysis-lane answer is held to an INDEPENDENT computation:
//! `f64` evaluations at sampled parameter values, and for the bolt the
//! closed form.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    BoxAxis, CancelToken, CapEnd, Coord, Dimension, DocEdit, DocRef, DocumentId, EvalOptions,
    Evaluation, Formula, FreeVar, MateRef, Node, NodeResult, Operand, ParamBox, ProfileDoc,
    RecipeNodeId, RoleSeg, SitedRef, StableName, TargetAxis, VarName, all_vertices, evaluate,
    vertex_position,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{ang, head, insert, len, on_frame, scl, solve, step};
use geom_core::{Dual64, Interval, Tol};

// ---- Substrate ----

/// The slab's side and height, and the bolt's: a `9 x 9 x 1` slab wide
/// enough for three bolts on it, and a `1 x 1 x 3` bolt.
const SLAB_WIDTH: f64 = 9.0;
const SLAB_HEIGHT: f64 = 1.0;
const BOLT_WIDTH: f64 = 1.0;
const BOLT_HEIGHT: f64 = 3.0;

/// The bolt pattern's spacing parameter and its nominal.
const SPACING: f64 = 2.0;
/// The standoff parameter and its nominal.
const GAP: f64 = 0.5;

fn spacing() -> VarName {
    VarName::from_static("s")
}

/// The in-plane slide of the drift row's target plane.
fn drift() -> VarName {
    VarName::from_static("drift")
}

fn gap() -> VarName {
    VarName::from_static("gap")
}

fn lift() -> VarName {
    VarName::from_static("lift")
}

fn turn() -> VarName {
    VarName::from_static("twist")
}

/// A `w x w x h` block from the origin, as a whole part document, and
/// its body.
fn block(label: &str, w: f64, h: f64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (w, 0.0), (w, w), (0.0, w)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(h),
        },
    )
}

/// A part's own names for what a mate reads off it besides its caps:
/// the lateral edges at its profile's four canonical vertices, the
/// cap vertices over them, and the rim edges of its first segment.
#[derive(Clone)]
struct Names {
    lateral: Vec<StableName>,
    corner: Vec<StableName>,
}

impl Names {
    fn of(doc: &ProfileDoc, body: RecipeNodeId) -> Self {
        let v = |k| fixture::vpiece(doc, body, 0, k);
        Self {
            lateral: (0..4)
                .map(|k| fixture::ename(body, RoleSeg::LateralEdge(v(k))))
                .collect(),
            corner: (0..4)
                .map(|k| fixture::cap_vertex(body, CapEnd::End, v(k)))
                .collect(),
        }
    }
}

/// The two parts every document here instantiates, in one store.
struct Parts {
    store: PartStore,
    slab: DocRef,
    slab_body: RecipeNodeId,
    slab_names: Names,
    bolt: DocRef,
    bolt_body: RecipeNodeId,
    bolt_names: Names,
}

fn parts(label: &str) -> Parts {
    let mut store = PartStore::new();
    let (slab_doc, slab_part) = block(&format!("{label}-slab"), SLAB_WIDTH, SLAB_HEIGHT);
    let slab_names = Names::of(&slab_doc, slab_part);
    let (slab, slab_body) = store.insert_part((slab_doc, slab_part), Tol::witness());
    let (bolt_doc, bolt_part) = block(&format!("{label}-bolt"), BOLT_WIDTH, BOLT_HEIGHT);
    let bolt_names = Names::of(&bolt_doc, bolt_part);
    let (bolt, bolt_body) = store.insert_part((bolt_doc, bolt_part), Tol::witness());
    Parts {
        store,
        slab,
        slab_body,
        slab_names,
        bolt,
        bolt_body,
        bolt_names,
    }
}

/// A pose slid `by` along a direction or an axis's direction.
fn slid(pose: Operand, along: RoleRead<Operand>, by: Formula) -> Operand {
    Operand::Pose(Box::new(PoseDef::Slide {
        pose: RoleRead::plain(pose),
        along,
        by,
    }))
}

/// A plane slid `by` along its own normal.
fn standoff(plane: Operand, by: Formula) -> Operand {
    Operand::Pose(Box::new(PoseDef::Slide {
        pose: RoleRead::plain(plane.clone()),
        along: RoleRead::through(plane, PoseRole::Normal),
        by,
    }))
}

/// The frame through `axis` and `point`.
fn through(axis: Operand, point: Operand) -> Operand {
    Operand::Pose(Box::new(PoseDef::Through {
        axis: RoleRead::plain(axis),
        point: RoleRead::plain(point),
    }))
}

impl Parts {
    fn opts(&self) -> EvalOptions {
        with_resolver(self.store.clone())
    }
    /// The slab's top cap, as `slab` places it.
    fn slab_top(&self, slab: RecipeNodeId) -> crate::fixture::resolver::PartName {
        in_part(slab, self.slab_body, CapEnd::End)
    }
    /// The bolt's bottom cap, as `bolt` places it.
    fn bolt_foot(&self, bolt: RecipeNodeId) -> crate::fixture::resolver::PartName {
        in_part(bolt, self.bolt_body, CapEnd::Start)
    }
    /// The bolt's top cap, as `bolt` places it.
    fn bolt_head(&self, bolt: RecipeNodeId) -> crate::fixture::resolver::PartName {
        in_part(bolt, self.bolt_body, CapEnd::End)
    }
    /// The bolt's lateral edge `k`, as an axis `bolt` places.
    fn bolt_edge(&self, bolt: RecipeNodeId, k: usize) -> Operand {
        fixture::axis(SitedRef::new(bolt, self.bolt_names.lateral[k].clone()))
    }
    /// The slab's lateral edge `k`, as an axis `slab` places.
    fn slab_edge(&self, slab: RecipeNodeId, k: usize) -> Operand {
        fixture::axis(SitedRef::new(slab, self.slab_names.lateral[k].clone()))
    }
    /// **One of the slab's two side directions**, as an axis: the `x`
    /// of the frame standing on its edge at canonical vertex 0 and aimed
    /// at the top corner over vertex `k` (1 or 3, its neighbours) — a
    /// direction along a side whatever way the edge runs.
    fn slab_side(&self, slab: RecipeNodeId, k: usize) -> RoleRead<Operand> {
        RoleRead::through(
            through(
                self.slab_edge(slab, 0),
                fixture::point_of(SitedRef::new(slab, self.slab_names.corner[k].clone())),
            ),
            PoseRole::X,
        )
    }
    /// **A vertical line through the slab at `(x, y)` from its corner**
    /// at canonical vertex 0, along its two sides.
    fn slab_line(&self, slab: RecipeNodeId, x: f64, y: f64) -> Operand {
        slid(
            slid(self.slab_edge(slab, 0), self.slab_side(slab, 1), len(x)),
            self.slab_side(slab, 3),
            len(y),
        )
    }
}

/// **A seat**: `mover`'s bottom cap resting on `onto`'s top cap,
/// `by` above it, and the mover's origin at `(x, y)` on the target
/// space's axes as the least turn onto the mated direction carries
/// them, turned `spin` about the cap's own normal — the rest and its
/// three values, in `mover`'s bundle. Answers the rest's address.
#[allow(clippy::too_many_arguments)]
fn seat(
    doc: ProfileDoc,
    mover: RecipeNodeId,
    foot: impl crate::fixture::Sited,
    onto: impl crate::fixture::Sited,
    by: Formula,
    (x, y, spin): (Formula, Formula, Formula),
) -> (ProfileDoc, MateRef) {
    fixture::mated(
        doc,
        mover,
        fixture::mate(
            fixture::plane(head(foot)),
            fixture::flip(standoff(fixture::plane(head(onto)), by)),
        ),
        &[
            (Coord::Slide(TargetAxis::X), x),
            (Coord::Slide(TargetAxis::Y), y),
            (Coord::SPIN, spin),
        ],
    )
}

/// **The values seating a bolt `(1, 1)` from the pinned slab's upper
/// corner along its own sides**: that point, `U + R·(1, 1, 0)` with the
/// corner `U = Rz(turn)·(5, 0, lift)` and the slab's turn
/// `R = Rz(turn)·Rx(0.3)`, read on the seat's chart — the world's `x`
/// and `y` carried by the least turn from the bolt's mated `−z` onto
/// the mated direction `−n`, the slab's normal `n = R·z` reversed (the
/// same turn as `z` onto `n`), from the world's origin.
fn seat_on_the_pinned_slab(turn: f64, lift: f64) -> (f64, f64) {
    let (ct, st, c3, s3) = (turn.cos(), turn.sin(), 0.3_f64.cos(), 0.3_f64.sin());
    let rz = |v: [f64; 3]| [ct * v[0] - st * v[1], st * v[0] + ct * v[1], v[2]];
    let rx = |v: [f64; 3]| [v[0], c3 * v[1] - s3 * v[2], s3 * v[1] + c3 * v[2]];
    let r = |v: [f64; 3]| rz(rx(v));
    let corner = rz([5.0, 0.0, lift]);
    let along = r([1.0, 1.0, 0.0]);
    let p: [f64; 3] = [0, 1, 2].map(|i| corner[i] + along[i]);
    // The shortest turn taking `z` onto `n`: 0.3 about `a = (cos turn,
    // sin turn, 0)`, Rodrigues on `x` and `y`.
    let a = [ct, st, 0.0];
    let swing = |v: [f64; 3]| {
        let dot = a[0] * v[0] + a[1] * v[1];
        let cross = [
            a[1] * v[2] - a[2] * v[1],
            a[2] * v[0] - a[0] * v[2],
            a[0] * v[1] - a[1] * v[0],
        ];
        [0, 1, 2].map(|i| v[i] * c3 + cross[i] * s3 + a[i] * dot * (1.0 - c3))
    };
    let dot = |u: [f64; 3], v: [f64; 3]| u[0] * v[0] + u[1] * v[1] + u[2] * v[2];
    (
        dot(p, swing([1.0, 0.0, 0.0])),
        dot(p, swing([0.0, 1.0, 0.0])),
    )
}

/// Literal `(x, y, spin)` values.
fn at(x: f64, y: f64, spin: f64) -> (Formula, Formula, Formula) {
    (len(x), len(y), ang(spin))
}

fn declare(doc: ProfileDoc, name: VarName, v: f64, dim: Dimension) -> ProfileDoc {
    step(
        doc,
        DocEdit::DeclareVar {
            name,
            def: editor_core::VarDecl::Free(FreeVar::continuous(dim, v)),
        },
    )
    .0
}

fn set_value(doc: ProfileDoc, name: VarName, v: f64) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetVarValue {
            var: name.into(),
            value: editor_core::FreeValue::Continuous(v),
        },
    )
    .0
}

fn named(name: VarName, dim: Dimension) -> Formula {
    Formula::named(name, dim)
}

/// A slab (pinned to the world at the identity over its own geometry)
/// and `n` bolts whose bundles are empty, each placed by whatever mates
/// the caller adds.
fn slab_and_bolts(
    p: &Parts,
    label: &str,
    n: usize,
) -> (ProfileDoc, RecipeNodeId, Vec<RecipeNodeId>) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (mut doc, slab) =
        fixture::insert_pinned(doc, crate::fixture::mated_instance(p.slab), &p.opts());
    let mut bolts = Vec::new();
    for _ in 0..n {
        let (next, bolt) = insert(doc, fixture::mated_instance(p.bolt));
        doc = next;
        bolts.push(bolt);
    }
    (doc, slab, bolts)
}

/// **The slab placed in the world, turned off its axes**: by `turn`
/// about the world's z after 0.3 about its x, with its upper corner at
/// `Rz(turn)·(5, 0, lift)`, `turn` and `lift` declared at 0.25 and 2.
fn turned_slab(p: &Parts, label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let doc = declare(doc, lift(), 2.0, Dimension::Length);
    let doc = declare(doc, turn(), 0.25, Dimension::Angle);
    let (doc, slab) = insert(doc, fixture::mated_instance(p.slab));
    // The motion the world read `slide x 5, spin x 0.3, slide z lift,
    // spin z turn` (each move about the world's own axes) put the slab's
    // upper corner frame at, written as the moves it is: `turn` about z
    // after 0.3 about x, the slab's origin a unit below that corner,
    // `Rz(turn)·(5, 0, lift) − Rz(turn)·Rx(0.3)·(0, 0, 1)`.
    let cos = || Formula::cos(named(turn(), Dimension::Angle)).expect("an angle's cosine");
    let sin = || Formula::sin(named(turn(), Dimension::Angle)).expect("an angle's sine");
    let metres = |k: f64, f: Formula| Formula::mul(len(k), f).expect("a length");
    let plus = |a: Formula, b: Formula| Formula::add(a, b).expect("a sum");
    let (c3, s3) = (0.3_f64.cos(), 0.3_f64.sin());
    let values = vec![
        (Coord::Spin(Some(TargetAxis::X)), ang(0.3)),
        (
            Coord::Spin(Some(TargetAxis::Z)),
            named(turn(), Dimension::Angle),
        ),
        (
            Coord::Slide(TargetAxis::X),
            plus(metres(5.0, cos()), metres(-s3, sin())),
        ),
        (
            Coord::Slide(TargetAxis::Y),
            plus(metres(5.0, sin()), metres(s3, cos())),
        ),
        (
            Coord::Slide(TargetAxis::Z),
            plus(named(lift(), Dimension::Length), len(-c3)),
        ),
    ];
    (
        fixture::written(doc, slab, editor_core::Placed::unmated(values)),
        slab,
    )
}

// ---- The documents the rows share ----

/// **The patterned bolt** (the designers' worked example): the bolt
/// seated on the slab with its origin at `(6 − 2s, 4)`, and a row of it
/// along `+x` slid `scalar(k)·s`, three members. Member #2
/// stands at `6` for every `s`, so the bolt moves by `−2` per unit of
/// `s`, copy #1 by `−1` and copy #2 not at all.
struct Bolted {
    doc: ProfileDoc,
    opts: EvalOptions,
    slab: RecipeNodeId,
    bolt: RecipeNodeId,
    pattern: RecipeNodeId,
    /// The seat's rest, a constraint of the bolt's bundle.
    mate: MateRef,
}

fn bolted(label: &str) -> Bolted {
    let p = parts(label);
    let (doc, slab, bolts) = slab_and_bolts(&p, label, 1);
    let bolt = bolts[0];
    let doc = declare(doc, spacing(), SPACING, Dimension::Length);
    let x = Formula::sub(
        len(6.0),
        Formula::mul(scl(2.0), named(spacing(), Dimension::Length)).expect("2·s is a length"),
    )
    .expect("a length less a length");
    let (doc, mate) = seat(
        doc,
        bolt,
        p.bolt_foot(bolt),
        p.slab_top(slab),
        Formula::zero(Dimension::Length),
        (x, len(4.0), Formula::zero(Dimension::Angle)),
    );
    let (doc, k) = fixture::index(doc, "k", 3);
    let slide = Formula::mul(fixture::scalar_of(k), named(spacing(), Dimension::Length))
        .expect("a count times a length");
    let (doc, pattern) = fixture::family_of(doc, bolt, vec![(Coord::Slide(TargetAxis::X), slide)]);
    Bolted {
        doc,
        opts: p.opts(),
        slab,
        bolt,
        pattern,
        mate,
    }
}

/// **The bolt on a parametric standoff**: seated on the slab `gap`
/// above its top cap — a slide of the cap's plane along its own normal
/// — with its origin at `(2, 3)` and turned 0.4 rad, so the bolt moves
/// by `e_z` per unit of `gap`.
struct Lifted {
    doc: ProfileDoc,
    opts: EvalOptions,
    bolt: RecipeNodeId,
    /// The seat's rest, a constraint of the bolt's bundle.
    mate: MateRef,
}

fn lifted(label: &str) -> Lifted {
    let p = parts(label);
    let (doc, slab, bolts) = slab_and_bolts(&p, label, 1);
    let bolt = bolts[0];
    let doc = declare(doc, gap(), GAP, Dimension::Length);
    let (doc, mate) = seat(
        doc,
        bolt,
        p.bolt_foot(bolt),
        p.slab_top(slab),
        named(gap(), Dimension::Length),
        at(2.0, 3.0, 0.4),
    );
    Lifted {
        doc,
        opts: p.opts(),
        bolt,
        mate,
    }
}

/// **A shaft in two coaxial bores**: the bolt held by two coaxial mates
/// on ONE line — its lateral edge on the same vertical line of the slab
/// twice — and a rest whose standoff `rise` lifts it one for one; a
/// third pin, the bolt's next lateral edge on that line slid one bolt
/// width along the slab's side, clocks it. Either bore is then one the
/// rest of the bundle already fixes, so the bundle is overconstrained;
/// `idle` is declared and read by nothing.
struct Shaft {
    doc: ProfileDoc,
    opts: EvalOptions,
    bolt: RecipeNodeId,
    /// The bolt's bundle, in the order it was authored.
    mates: Vec<MateRef>,
}

const RISE: f64 = 1.0;
const IDLE: f64 = 0.5;

fn rise() -> VarName {
    VarName::from_static("rise")
}

fn idle() -> VarName {
    VarName::from_static("idle")
}

/// The second bore's line is the first's slid along the slab's side by
/// `off.0` and then by `off.1`: `(0, 0)` for one line.
fn shaft(label: &str, off: (f64, f64)) -> Shaft {
    let p = parts(label);
    let (doc, slab, bolts) = slab_and_bolts(&p, label, 1);
    let bolt = bolts[0];
    let doc = declare(doc, rise(), RISE, Dimension::Length);
    let doc = declare(doc, idle(), IDLE, Dimension::Length);
    let line = || p.slab_line(slab, 4.0, 4.0);
    let mates = [
        fixture::mate(p.bolt_edge(bolt, 0), line()),
        fixture::mate(
            p.bolt_edge(bolt, 0),
            slid(
                slid(line(), p.slab_side(slab, 1), len(off.0)),
                p.slab_side(slab, 1),
                len(off.1),
            ),
        ),
        fixture::mate(
            fixture::plane(head(p.bolt_foot(bolt))),
            fixture::flip(standoff(
                fixture::plane(head(p.slab_top(slab))),
                named(rise(), Dimension::Length),
            )),
        ),
        fixture::mate(
            p.bolt_edge(bolt, 1),
            slid(line(), p.slab_side(slab, 1), len(BOLT_WIDTH)),
        ),
    ];
    // The first bore through the door; the rest beside it past the
    // door, the state a doctored save leaves, which the fold refuses.
    let [first, rest @ ..] = mates;
    let (mut doc, first) = fixture::constrain_at(doc, bolt, first);
    let mut refs = vec![first];
    for mate in rest {
        let (next, id) = editor_core::test_support::beside_past_the_bundle_rule(
            &doc,
            bolt,
            mate,
            Tol::witness(),
        );
        doc = next;
        refs.push(MateRef::new(bolt, id));
    }
    Shaft {
        doc,
        opts: p.opts(),
        bolt,
        mates: refs,
    }
}

// ---- The mate corpus ----

/// **The mate corpus the fence is taken over** — one document per shape
/// of bundle the fold places or refuses, each with its resolver. Built
/// through the ordinary doors, so the documents are what an author
/// makes, but for the two overconstrained bundles, which no door admits
/// and which are written past it as a doctored save would hold them.
fn corpus() -> Vec<(&'static str, ProfileDoc, EvalOptions)> {
    let mut out = Vec::new();

    // Two seats: a rest and its three values, twice.
    let p = parts("msolve14-c-seat");
    let (doc, slab, bolts) = slab_and_bolts(&p, "msolve14-c-seat", 2);
    let (doc, _) = seat(
        doc,
        bolts[0],
        p.bolt_foot(bolts[0]),
        p.slab_top(slab),
        Formula::zero(Dimension::Length),
        at(1.0, 1.0, 0.0),
    );
    let (doc, _) = seat(
        doc,
        bolts[1],
        p.bolt_foot(bolts[1]),
        p.slab_top(slab),
        Formula::zero(Dimension::Length),
        at(5.0, 1.0, 0.3),
    );
    out.push(("seats", doc, p.opts()));

    // A coaxial mate cut to DETERMINED by its two values: a slide
    // along the pin's line seating the bolt's foot 0.25 above the
    // slab's top, and a turn about that line.
    let p = parts("msolve14-c-coax");
    let (doc, slab, bolts) = slab_and_bolts(&p, "msolve14-c-coax", 1);
    let (doc, _) = fixture::mated(
        doc,
        bolts[0],
        fixture::mate(p.bolt_edge(bolts[0], 0), p.slab_line(slab, 4.0, 4.0)),
        &[
            (Coord::Slide(TargetAxis::Z), len(SLAB_HEIGHT + 0.25)),
            (Coord::SPIN, ang(0.3)),
        ],
    );
    out.push(("coaxial-slide-spin", doc, p.opts()));

    // A rest, then a frame mate beside it past the door (the state a
    // doctored save leaves): the placement refuses its bundle
    // OVERCONSTRAINED, naming both.
    let p = parts("msolve14-c-over");
    let (doc, slab, bolts) = slab_and_bolts(&p, "msolve14-c-over", 1);
    let bolt = bolts[0];
    let (doc, _) = fixture::constrain_at(
        doc,
        bolt,
        fixture::rest(head(p.bolt_foot(bolt)), head(p.slab_top(slab))),
    );
    let frame = |at: RecipeNodeId, body: RecipeNodeId, names: &Names, edge: Operand, end| {
        let corner = match end {
            CapEnd::End => names.corner[1].clone(),
            CapEnd::Start => fixture::cap_vertex(
                body,
                CapEnd::Start,
                match names.corner[1].path.as_slice() {
                    [RoleSeg::CapVertex(_, v)] => *v,
                    other => panic!("a cap vertex, not {other:?}"),
                },
            ),
        };
        through(edge, fixture::point_of(SitedRef::new(at, corner.clone())))
    };
    let (doc, _) = editor_core::test_support::beside_past_the_bundle_rule(
        &doc,
        bolt,
        fixture::mate(
            frame(
                bolt,
                p.bolt_body,
                &p.bolt_names,
                p.bolt_edge(bolt, 0),
                CapEnd::Start,
            ),
            frame(
                slab,
                p.slab_body,
                &p.slab_names,
                p.slab_edge(slab, 0),
                CapEnd::End,
            ),
        ),
        Tol::witness(),
    );
    out.push(("overconstrained", doc, p.opts()));

    // The patterned bolt.
    let b = bolted("msolve14-c-bolt");
    out.push(("patterned-bolt", b.doc, b.opts));

    // The parametric standoff.
    let l = lifted("msolve14-c-lift");
    out.push(("standoff", l.doc, l.opts));

    // A parametric slab — values on the world's axes, two of them
    // reading parameters — and a seat on the pinned slab,
    // and a seat on that: a chain of bundles back to the world.
    let p = parts("msolve14-c-pin");
    let (doc, slab) = turned_slab(&p, "msolve14-c-pin");
    // A seat's values are charted on the world's axes carried by the
    // least turn onto the mated direction, the slab's normal
    // `n = Rz(turn)·Rx(0.3)·z` reversed, from the world's origin
    // (§5′.2). The bolt is meant to stand `(1, 1)` along the slab's own
    // sides from its upper corner, so its values are that point read on
    // the chart at the parameters' nominal values (`turn = 0.25`,
    // `lift = 2`); the bolt seated on its head stands over it, at the
    // same numbers.
    let (x, y) = seat_on_the_pinned_slab(0.25, 2.0);
    let (doc, top) = insert(doc, fixture::mated_instance(p.bolt));
    let (doc, _) = seat(
        doc,
        top,
        p.bolt_foot(top),
        p.slab_top(slab),
        Formula::zero(Dimension::Length),
        at(x, y, 0.0),
    );
    let (doc, third) = insert(doc, fixture::mated_instance(p.bolt));
    let (doc, _) = seat(
        doc,
        third,
        p.bolt_foot(third),
        p.bolt_head(top),
        Formula::zero(Dimension::Length),
        at(x, y, 0.0),
    );
    out.push(("pinned-chain", doc, p.opts()));

    // Two bolts on the slab and a third seated on the second: a chain
    // of bundles, each pinned to the copy before it.
    let p = parts("msolve14-c-chain");
    let (doc, slab, bolts) = slab_and_bolts(&p, "msolve14-c-chain", 3);
    let (doc, _) = seat(
        doc,
        bolts[0],
        p.bolt_foot(bolts[0]),
        p.slab_top(slab),
        Formula::zero(Dimension::Length),
        at(1.0, 1.0, 0.0),
    );
    let (doc, _) = seat(
        doc,
        bolts[1],
        p.bolt_foot(bolts[1]),
        p.slab_top(slab),
        Formula::zero(Dimension::Length),
        at(4.0, 1.0, 0.0),
    );
    // The third stands over the second: a seat's values are read from
    // the world's origin, so it writes where the second stands.
    let (doc, _) = seat(
        doc,
        bolts[2],
        p.bolt_foot(bolts[2]),
        p.bolt_head(bolts[1]),
        Formula::zero(Dimension::Length),
        at(4.0, 1.0, 0.0),
    );
    out.push(("chain", doc, p.opts()));

    // A shaft in two coaxial bores on one line, a rest and a third
    // pin, beside one another past the door: each fixes what the
    // others fix.
    let s = shaft("msolve14-c-shaft", (0.0, 0.0));
    out.push(("shaft", s.doc, s.opts));
    out
}

// ---- The digest ----

/// FNV-1a 64 over bytes.
struct Fnv(u64);

impl Fnv {
    fn new() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }
    fn feed(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.0 ^= u64::from(*b);
            self.0 = self.0.wrapping_mul(0x1000_0000_01b3);
        }
    }
    fn frame(&mut self, f: &editor_core::Frame) {
        for x in f.columns.iter().flatten().chain(f.translation.iter()) {
            self.feed(&x.to_bits().to_le_bytes());
        }
    }
}

/// **What the solve answers about a document, at `f64`**: every node in
/// document order with its fault and space, and every instance's
/// relative pose and world placement by bits (or the
/// refusal, by its `Debug`, whose `f64`s are shortest round-trips) —
/// then the document's evaluation, through the shared evaluation
/// digest, and every node's error.
fn solve_digest(h: &mut Fnv, doc: &ProfileDoc, opts: &EvalOptions) {
    let poses = solve(doc, opts, Tol::witness());
    for id in doc.ids() {
        h.feed(format!("#{id:?}").as_bytes());
        h.feed(format!("fault={:?}", poses.fault(id)).as_bytes());
        h.feed(format!("space={:?}", poses.space(id)).as_bytes());
        if let Some(rel) = poses.relative(id) {
            h.frame(&rel);
        }
        if matches!(doc.node(id), Some(Node::InstantiatePart { .. })) {
            match poses.placement(doc, id) {
                Ok(f) => h.frame(&f),
                Err(e) => h.feed(format!("{e:?}").as_bytes()),
            }
        }
    }
    let ev = run_at::<f64>(doc, opts, None);
    h.feed(&fixture::digest::digest(&ev).to_le_bytes());
    for id in doc.ids() {
        h.feed(format!("{:?}", ev.node_error(id).map(|e| &e.kind)).as_bytes());
    }
}

fn run_at<T: editor_core::EvalScalar>(
    doc: &ProfileDoc,
    opts: &EvalOptions,
    prior: Option<&Evaluation<T>>,
) -> Evaluation<T> {
    evaluate::<T>(doc, prior, &CancelToken::new(), opts, Tol::witness())
}

/// **The corpus digest, per ε row**: every fault, space, solved pose
/// and evaluation (each body where it stands, composed through
/// `in_space`, with its name tables) this file's mate corpus gives, fed
/// in corpus order. The generic solve at `f64` is the nominal solve
/// bit for bit, so this pins the generic solve against itself; the
/// id-free rows beside it pin what it solves.
///
/// One number per row because the documents are not ε-free: a part's
/// content pin hashes its recorded ε, and an instance's id hashes the
/// pin, so every id the digest feeds moves with the row. The rows are
/// the three the hosted matrix runs; any other ε has no measurement and
/// the row fails rather than pass on nothing.
const MAIN_CORPUS_DIGEST: [(f64, u64); 3] = [
    (1e-9, 0xb820_871c_94ff_0c0b),
    (1e-6, 0xef7d_754d_6fdc_f337),
    (1e-12, 0xf38a_1cb4_4a53_85fc),
];

/// **Each seat of the corpus stands on what it is seated on**: in the
/// frame of the copy a bolt rests on, the bolt's origin is where its
/// generator means it — `(1, 1, 1)` on the pinned slab's top, and
/// `(0, 0, 3)` on a bolt's head for a bolt seated over it — at the
/// parameters' nominal values.
///
/// *Breaks if* a seat's values are read from the target body's frame
/// on one side and written from the space's on the other (a bolt
/// floats off the slab or off the head it rests on).
#[test]
fn each_seat_stands_on_what_it_is_seated_on() {
    let docs = corpus();
    let local = |label: &str, onto: usize, mover: usize| {
        let (_, doc, opts) = docs
            .iter()
            .find(|(l, ..)| *l == label)
            .unwrap_or_else(|| panic!("the corpus holds {label}"));
        let poses = solve(doc, opts, Tol::witness());
        let instances: Vec<RecipeNodeId> = doc
            .ids()
            .iter()
            .copied()
            .filter(|&id| matches!(doc.node(id), Some(Node::InstantiatePart { .. })))
            .collect();
        let at = |k: usize| {
            poses
                .placement(doc, instances[k])
                .unwrap_or_else(|e| panic!("{label}'s copy {k} places: {e:?}"))
                .affine::<f64>()
        };
        let t = (at(onto).inverse() * at(mover)).translation;
        [t.x, t.y, t.z]
    };
    for (label, onto, mover, want) in [
        ("pinned-chain", 0, 1, [1.0, 1.0, SLAB_HEIGHT]),
        ("pinned-chain", 1, 2, [0.0, 0.0, BOLT_HEIGHT]),
        ("chain", 2, 3, [0.0, 0.0, BOLT_HEIGHT]),
    ] {
        let got = local(label, onto, mover);
        assert!(
            got.iter().zip(want).all(|(g, w)| (g - w).abs() < 1e-9),
            "{label}: copy {mover} stands at {got:?} in copy {onto}'s frame, meant {want:?}"
        );
    }
}

/// **A3, the `f64` fence**: the corpus's solved poses, spaces, faults and
/// evaluations are byte-identical to the pre-generic tree's.
///
/// RE-BLESSED twice for the edge curve's `Debug` the digest feeds: when
/// check 2's turn moved from the edge's `Certificate` onto the edge (PR
/// 5728), and when `Intersection`'s witness became its selector
/// (`selector: Witness(p)` where it spelled `witness: p`). Each time, with
/// that `Debug` written in main's shape, every digest here reproduced
/// main's value at all three ε rows, so nothing else moved.
#[test]
fn a3_the_f64_solve_is_mains_bit_for_bit_on_the_mate_corpus() {
    let eps = Tol::witness().eps();
    let Some(&(_, main)) = MAIN_CORPUS_DIGEST
        .iter()
        .find(|(e, _)| e.to_bits() == eps.to_bits())
    else {
        panic!("no main digest was measured at ε = {eps:e}; measure one on main's tree")
    };
    let mut h = Fnv::new();
    for (label, doc, opts) in corpus() {
        h.feed(label.as_bytes());
        solve_digest(&mut h, &doc, &opts);
    }
    assert_eq!(
        h.0, main,
        "the f64 solve moved on the mate corpus at ε = {eps:e}: {:#018x}",
        h.0
    );
}

// ---- Reading a lane's answer, and the independent one ----

/// Every vertex of `node`'s body, by name, at the lane's scalar.
fn vertices<T: geom_core::Decide>(
    ev: &Evaluation<T>,
    node: RecipeNodeId,
) -> Vec<(StableName, geom_core::Point3<T>)> {
    let names = all_vertices(ev, node);
    assert!(!names.is_empty(), "node {node:?} has vertices");
    names
        .into_iter()
        .map(|n| {
            let p = vertex_position(ev, node, &n)
                .unwrap_or_else(|e| panic!("vertex {n:?} of {node:?} reads: {e:?}"));
            (n, p)
        })
        .collect()
}

/// Every vertex of `node`, by name, in an `f64` evaluation of `doc` with
/// `param` bound at `value` — the independent computation a lane's
/// answer is held to.
fn f64_vertices_at(
    doc: &ProfileDoc,
    opts: &EvalOptions,
    node: RecipeNodeId,
    param: VarName,
    value: f64,
) -> BTreeMap<StableName, [f64; 3]> {
    let doc = set_value(doc.clone(), param, value);
    let ev = run_at::<f64>(&doc, opts, None);
    assert!(
        ev.node_error(node).is_none(),
        "the f64 build at {value} evaluates node {node:?}: {:?}",
        ev.node_error(node)
    );
    vertices(&ev, node)
        .into_iter()
        .map(|(n, p)| (n, [p.x, p.y, p.z]))
        .collect()
}

/// The member of a family a vertex of the family's value belongs to.
fn copy_of(name: &StableName) -> u32 {
    match name.path.first() {
        Some(editor_core::RoleSeg::Member { at, .. }) => at[0].1,
        other => panic!("a family's vertex is member-wrapped, got {other:?}"),
    }
}

/// The variable `doc` names `param`.
fn var_of(doc: &ProfileDoc, param: &VarName) -> editor_core::VarId {
    doc.var_named(param.as_str())
        .expect("the fixture declares it")
}

fn seeded(doc: &ProfileDoc, opts: &EvalOptions, param: VarName) -> EvalOptions {
    EvalOptions {
        seed: Some(var_of(doc, &param)),
        ..opts.clone()
    }
}

fn boxed(doc: &ProfileDoc, opts: &EvalOptions, param: VarName, lo: f64, hi: f64) -> EvalOptions {
    EvalOptions {
        param_box: Some(Arc::new(ParamBox::from_axes(BTreeMap::from([(
            var_of(doc, &param),
            BoxAxis::Varying { lo, hi },
        )])))),
        ..opts.clone()
    }
}

/// **A seed run's tangent at every vertex of `node` is the central
/// difference of two `f64` builds** at `nominal ± h` — exact up to
/// rounding where the motion is affine in the parameter (a translation,
/// or a rotation the parameter does not turn); [`assert_turn_tangents_match`]
/// takes a finer step for one the parameter turns. The value channel is
/// the `f64` nominal build's bits. Returns each vertex's tangent.
fn assert_tangents_match(
    doc: &ProfileDoc,
    opts: &EvalOptions,
    ev: &Evaluation<Dual64>,
    node: RecipeNodeId,
    param: VarName,
    nominal: f64,
    what: &str,
) -> Vec<(StableName, [f64; 3])> {
    tangents_match(doc, opts, ev, node, (param, nominal), (1e-3, 1e-9), what)
}

/// [`assert_tangents_match`] for a parameter that TURNS the vertices:
/// the central difference's truncation is `h²/6` of the third
/// derivative, so the step is `1e-5` and the slack `1e-8`.
fn assert_turn_tangents_match(
    doc: &ProfileDoc,
    opts: &EvalOptions,
    ev: &Evaluation<Dual64>,
    node: RecipeNodeId,
    param: VarName,
    nominal: f64,
    what: &str,
) -> Vec<(StableName, [f64; 3])> {
    tangents_match(doc, opts, ev, node, (param, nominal), (1e-5, 1e-8), what)
}

fn tangents_match(
    doc: &ProfileDoc,
    opts: &EvalOptions,
    ev: &Evaluation<Dual64>,
    node: RecipeNodeId,
    (param, nominal): (VarName, f64),
    (h, slack): (f64, f64),
    what: &str,
) -> Vec<(StableName, [f64; 3])> {
    let at = f64_vertices_at(doc, opts, node, param.clone(), nominal);
    let up = f64_vertices_at(doc, opts, node, param.clone(), nominal + h);
    let down = f64_vertices_at(doc, opts, node, param, nominal - h);
    let mut out = Vec::new();
    for (name, p) in vertices(ev, node) {
        let value = [p.x.value, p.y.value, p.z.value];
        let tangent = [p.x.deriv, p.y.deriv, p.z.deriv];
        assert!(
            tangent.iter().all(|t| t.is_finite()),
            "{what}: the tangent of {name:?} is finite: {tangent:?}"
        );
        assert_eq!(
            value.map(f64::to_bits),
            at[&name].map(f64::to_bits),
            "{what}: the value channel of {name:?} is the f64 build's"
        );
        for k in 0..3 {
            let fd = (up[&name][k] - down[&name][k]) / (2.0 * h);
            assert!(
                (tangent[k] - fd).abs() <= slack,
                "{what}: ∂/∂param of {name:?}[{k}] is {} where the f64 builds give {fd}",
                tangent[k]
            );
        }
        out.push((name, tangent));
    }
    out
}

/// **A box run's enclosure of every vertex of `node` contains that
/// vertex in the `f64` builds at the box's two corners and its
/// nominal** — so where the vertex moves across the box, the enclosure
/// spans that motion.
fn assert_encloses_corners(
    doc: &ProfileDoc,
    opts: &EvalOptions,
    ev: &Evaluation<Interval>,
    node: RecipeNodeId,
    (param, nominal): (VarName, f64),
    (lo, hi): (f64, f64),
    what: &str,
) {
    use geom_core::Bounds;
    let samples = [nominal + lo, nominal, nominal + hi]
        .map(|v| f64_vertices_at(doc, opts, node, param.clone(), v));
    for (name, p) in vertices(ev, node) {
        for (k, x) in [p.x, p.y, p.z].into_iter().enumerate() {
            for (s, at) in samples.iter().enumerate() {
                let truth = at[&name][k];
                assert!(
                    x.lo() <= truth && truth <= x.hi(),
                    "{what}: the enclosure [{}, {}] of {name:?}[{k}] omits {truth}, its f64 \
                     position at sample {s}",
                    x.lo(),
                    x.hi()
                );
            }
        }
    }
}

/// **A box narrow enough for the placement door**: `±w·ε` about the
/// nominal. An instance's body is placed through `topo`'s rigid
/// transform, which certifies each placed edge on its placed carrier,
/// and a box-wide translation widens those two independently past the
/// band — a refusal every boxed placer meets today
/// (`work/topotail/a-boxed-rotation-refuses-not-rigid-at-every-placer.md`,
/// its "Measured" section). A box a fraction of ε wide is placed, and
/// the rows that read the BODY read it over one; the rows that read
/// the SOLVE read it over a wide one ([`assert_pose_encloses_corners`]).
fn narrow(w: f64) -> (f64, f64) {
    let eps = Tol::witness().eps();
    (-w * eps, w * eps)
}

/// **The solve's own pose of `instance` over a box encloses its `f64`
/// pose at both corners and the nominal** — read at the solve, through
/// [`editor_core::mate::solve_document_at`] at `Interval` under the box,
/// and held to [`solve`] at `f64` on the document with the parameter
/// bound at each sample. The solve is the evaluation, so its box is one
/// the placement door admits ([`narrow`]). Returns the enclosure's
/// widest translation component.
fn assert_pose_encloses_corners(
    doc: &ProfileDoc,
    opts: &EvalOptions,
    instance: RecipeNodeId,
    param: VarName,
    nominal: f64,
    (lo, hi): (f64, f64),
    what: &str,
) -> f64 {
    use geom_core::Bounds;
    let poses = editor_core::mate::solve_document_at::<Interval>(
        doc,
        &boxed(doc, opts, param.clone(), lo, hi),
        Tol::witness(),
    );
    assert!(
        poses.fault(instance).is_none(),
        "{what}: the solve over the box poses {instance:?}: {:?}",
        poses.fault(instance)
    );
    let map = poses.relative_map(instance).expect("posed over the box");
    let entries = |m: &geom_core::Affine3<Interval>| {
        [m.linear.c0, m.linear.c1, m.linear.c2, m.translation]
            .into_iter()
            .flat_map(|c| [c.x, c.y, c.z])
            .collect::<Vec<_>>()
    };
    let got = entries(&map);
    for v in [nominal + lo, nominal, nominal + hi] {
        let at = set_value(doc.clone(), param.clone(), v);
        let truth = solve(&at, opts, Tol::witness())
            .relative(instance)
            .expect("posed at f64")
            .affine::<f64>();
        let truth = [
            truth.linear.c0,
            truth.linear.c1,
            truth.linear.c2,
            truth.translation,
        ]
        .into_iter()
        .flat_map(|c| [c.x, c.y, c.z]);
        for (k, (x, t)) in got.iter().zip(truth).enumerate() {
            assert!(
                x.lo() <= t && t <= x.hi(),
                "{what}: entry {k} of the pose over the box, [{}, {}], omits {t}, its f64 \
                 pose at {v}",
                x.lo(),
                x.hi()
            );
        }
    }
    let t = map.translation;
    [t.x, t.y, t.z]
        .into_iter()
        .map(|x| x.hi() - x.lo())
        .fold(0.0, f64::max)
}

// ---- A2: a parameter a bundle reads moves its mated part in the run that binds it ----

/// **The patterned bolt, seeded on its spacing** (the designers' worked
/// example): the bolt instance — seated at `6 − 2s` and the family's
/// source — moves by `∂B/∂s = −2` along `x`, member #1 by `−1`, and
/// member #2 by `∂copy2/∂s = 0`. Every vertex's tangent is the central
/// difference of two `f64` builds, and the value channel is the nominal
/// build's bits.
///
/// *Breaks if* a value's number is read anywhere but the lane's own
/// environment, or loses its tangent on the way to the pose.
#[test]
fn a2_a_seed_on_the_bolts_spacing_moves_the_bolt_by_minus_two_and_holds_copy_two() {
    let b = bolted("msolve14-a2-seed");
    let ev = run_at::<Dual64>(&b.doc, &seeded(&b.doc, &b.opts, spacing()), None);
    assert!(
        ev.node_error(b.mate.placement).is_none(),
        "{:?}",
        ev.node_error(b.mate.placement)
    );
    let bolt = assert_tangents_match(
        &b.doc,
        &b.opts,
        &ev,
        b.bolt,
        spacing(),
        SPACING,
        "the bolt instance",
    );
    for (name, t) in &bolt {
        assert_eq!(t, &[-2.0, 0.0, 0.0], "∂B/∂s at {name:?}");
    }
    let copies = assert_tangents_match(
        &b.doc,
        &b.opts,
        &ev,
        b.pattern,
        spacing(),
        SPACING,
        "the family",
    );
    for (name, t) in &copies {
        let want = [-2.0, -1.0, 0.0][copy_of(name) as usize];
        assert_eq!(t, &[want, 0.0, 0.0], "∂copy/∂s at {name:?}");
    }
    assert!(
        copies.iter().any(|(n, _)| copy_of(n) == 2),
        "member #2 is among the family's vertices"
    );
}

/// **The patterned bolt over a box on its spacing.** At the solve: the
/// bolt's pose encloses its `f64` pose at both corners and the nominal,
/// and is at least twice the box wide along `x` — not the nominal's
/// point. Through the evaluation: every vertex of the bolt, the family
/// and the slab — whose face the seat reads, and which holds still — is
/// enclosed at both corners and the nominal.
///
/// *Breaks if* a box run's fold reads a value at the nominal.
#[test]
fn a2_a_box_on_the_bolts_spacing_encloses_the_bolt_at_every_corner() {
    let b = bolted("msolve14-a2-box");
    let (lo, hi) = narrow(0.05);
    let width = assert_pose_encloses_corners(
        &b.doc,
        &b.opts,
        b.bolt,
        spacing(),
        SPACING,
        (lo, hi),
        "the bolted box, at the solve",
    );
    assert!(
        width >= 2.0 * (hi - lo),
        "the bolt's pose spans the box twice over: {width}"
    );
    let ev = run_at::<Interval>(&b.doc, &boxed(&b.doc, &b.opts, spacing(), lo, hi), None);
    assert!(
        ev.node_error(b.mate.placement).is_none(),
        "{:?}",
        ev.node_error(b.mate.placement)
    );
    for node in [b.bolt, b.pattern, b.slab] {
        assert_encloses_corners(
            &b.doc,
            &b.opts,
            &ev,
            node,
            (spacing(), SPACING),
            (lo, hi),
            "the bolted box, evaluated",
        );
    }
}

/// **Over a wide box the bolt builds**: a copy is its source's body
/// under its frame (D10 `[placement.frame]`), never rebuilt, so the
/// box-wide motion is the copy's frame and no mapped carrier is
/// certified again — the placement door's re-certification, which
/// refused over this box while a placement mapped its source, is not
/// run. The family reading the bolt builds its members the same way.
///
/// *Breaks if* a copy is rebuilt under a box-wide motion (the mapped
/// carriers refuse certification), or the family is poisoned through
/// the bolt.
#[test]
fn a2_over_a_wide_box_the_bolt_builds_its_copy_unbuilt() {
    let b = bolted("msolve14-a2-wide");
    let ev = run_at::<Interval>(
        &b.doc,
        &boxed(&b.doc, &b.opts, spacing(), -0.25, 0.25),
        None,
    );
    assert!(
        ev.node_error(b.bolt).is_none(),
        "the bolt builds over the box: {:?}",
        ev.node_error(b.bolt)
    );
    assert!(
        matches!(ev.result(b.pattern), Some(NodeResult::Ok(_))),
        "the family builds over the box: {:?}",
        ev.node_error(b.pattern)
    );
}

/// **A standoff whose length is seeded**: the bolt rests on a plane its
/// bundle constructs, the slab's top cap slid `gap` along its normal,
/// so it moves by `e_z` per unit of `gap`, every vertex's tangent the
/// `f64` builds' central difference.
///
/// *Breaks if* a pose construction's number is read anywhere but the
/// lane's own environment.
#[test]
fn a2_a_seed_on_a_standoff_moves_the_mated_part() {
    let l = lifted("msolve14-a2-lift-seed");
    let ev = run_at::<Dual64>(&l.doc, &seeded(&l.doc, &l.opts, gap()), None);
    assert!(
        ev.node_error(l.mate.placement).is_none(),
        "{:?}",
        ev.node_error(l.mate.placement)
    );
    let tangents =
        assert_tangents_match(&l.doc, &l.opts, &ev, l.bolt, gap(), GAP, "the lifted bolt");
    for (name, t) in tangents {
        let want = [0.0, 0.0, 1.0];
        assert!(
            (0..3).all(|k| (t[k] - want[k]).abs() <= 1e-12),
            "∂B/∂gap at {name:?} is {t:?}, want {want:?}"
        );
    }
}

/// **The same standoff over a box on its length**: the bolt's pose
/// encloses its `f64` pose at both corners at the solve, and every
/// vertex of the bolt is enclosed through the evaluation.
///
/// *Breaks if* a box run's fold reads a pose construction's number at
/// the nominal.
#[test]
fn a2_a_box_on_a_standoff_encloses_the_mated_part() {
    let l = lifted("msolve14-a2-lift-box");
    let (lo, hi) = narrow(0.05);
    let width = assert_pose_encloses_corners(
        &l.doc,
        &l.opts,
        l.bolt,
        gap(),
        GAP,
        (lo, hi),
        "the lifted box, at the solve",
    );
    assert!(width >= hi - lo, "the pose widens: {width}");
    let ev = run_at::<Interval>(&l.doc, &boxed(&l.doc, &l.opts, gap(), lo, hi), None);
    assert!(
        ev.node_error(l.mate.placement).is_none(),
        "{:?}",
        ev.node_error(l.mate.placement)
    );
    assert_encloses_corners(
        &l.doc,
        &l.opts,
        &ev,
        l.bolt,
        (gap(), GAP),
        (lo, hi),
        "the lifted box, evaluated",
    );
}

// ---- A2's third set of names: a value's slide and spin ----

/// **A bolt seated with a slide and a spin two parameters drive**: the
/// rest's `x` value is `slide` and its spin value `spin`, so the bolt
/// moves with both.
struct Slid {
    doc: ProfileDoc,
    opts: EvalOptions,
    bolt: RecipeNodeId,
    /// The seat's rest, a constraint of the bolt's bundle.
    mate: MateRef,
}

const SLIDE: f64 = 2.0;
const SPIN: f64 = 0.3;

fn slide() -> VarName {
    VarName::from_static("slide")
}

fn spin() -> VarName {
    VarName::from_static("spin")
}

fn slid_bolt(label: &str) -> Slid {
    let p = parts(label);
    let (doc, slab, bolts) = slab_and_bolts(&p, label, 1);
    let doc = declare(doc, slide(), SLIDE, Dimension::Length);
    let doc = declare(doc, spin(), SPIN, Dimension::Angle);
    let (doc, mate) = seat(
        doc,
        bolts[0],
        p.bolt_foot(bolts[0]),
        p.slab_top(slab),
        Formula::zero(Dimension::Length),
        (
            named(slide(), Dimension::Length),
            len(3.0),
            named(spin(), Dimension::Angle),
        ),
    );
    Slid {
        doc,
        opts: p.opts(),
        bolt: bolts[0],
        mate,
    }
}

/// **A seed on a value moves the mated part in that lane**: seeded on
/// the slide, the bolt moves by `(1, 0, 0)`; seeded on the spin, each
/// vertex by its `f64` builds' central difference — a value's own
/// parameters, a set of names a run binds beside a world mate's and a
/// pose construction's.
///
/// *Breaks if* a value's number is read anywhere but the lane's own
/// environment.
#[test]
fn a2_a_seed_on_a_value_moves_the_mated_part() {
    let s = slid_bolt("msolve14-a2-offset-seed");
    let ev = run_at::<Dual64>(&s.doc, &seeded(&s.doc, &s.opts, slide()), None);
    assert!(
        ev.node_error(s.mate.placement).is_none(),
        "{:?}",
        ev.node_error(s.mate.placement)
    );
    for (name, t) in assert_tangents_match(
        &s.doc,
        &s.opts,
        &ev,
        s.bolt,
        slide(),
        SLIDE,
        "the slid bolt",
    ) {
        assert!(
            (t[0] - 1.0).abs() <= 1e-12 && t[1].abs() <= 1e-12 && t[2].abs() <= 1e-12,
            "∂B/∂slide at {name:?} is {t:?}"
        );
    }
    let ev = run_at::<Dual64>(&s.doc, &seeded(&s.doc, &s.opts, spin()), None);
    assert!(
        ev.node_error(s.mate.placement).is_none(),
        "{:?}",
        ev.node_error(s.mate.placement)
    );
    let turned = assert_turn_tangents_match(
        &s.doc,
        &s.opts,
        &ev,
        s.bolt,
        spin(),
        SPIN,
        "the turned bolt",
    );
    assert!(
        turned.iter().any(|(_, t)| t[0].abs() + t[1].abs() > 0.1),
        "the turn moves the bolt in the plane"
    );
}

/// **A box on a value encloses the mated part**: over a box on either
/// parameter, the solve's pose of the bolt encloses its `f64` pose at
/// both corners and the nominal; over the box on the slide, so does
/// every vertex of the evaluated bolt.
///
/// *Breaks if* a box run's fold reads a value at the nominal.
#[test]
fn a2_a_box_on_a_value_encloses_the_mated_part() {
    let s = slid_bolt("msolve14-a2-offset-box");
    let (lo, hi) = narrow(0.05);
    let width = assert_pose_encloses_corners(
        &s.doc,
        &s.opts,
        s.bolt,
        slide(),
        SLIDE,
        (lo, hi),
        "the slid box, at the solve",
    );
    assert!(width >= hi - lo, "the slide widens the pose: {width}");
    assert_pose_encloses_corners(
        &s.doc,
        &s.opts,
        s.bolt,
        spin(),
        SPIN,
        (lo, hi),
        "the turned box, at the solve",
    );
    let ev = run_at::<Interval>(&s.doc, &boxed(&s.doc, &s.opts, slide(), lo, hi), None);
    assert!(
        ev.node_error(s.mate.placement).is_none(),
        "{:?}",
        ev.node_error(s.mate.placement)
    );
    assert_encloses_corners(
        &s.doc,
        &s.opts,
        &ev,
        s.bolt,
        (slide(), SLIDE),
        (lo, hi),
        "the slid box, evaluated",
    );
}

/// **A box on a spin value places the copy it turns**: evaluated at
/// `Interval` over a box a twentieth of a radian wide on the spin, the
/// placement's map is a fold of a mate and values, admitted as the
/// rigid motion it is at every point of the box, so neither the
/// placement nor the bolt it places refuses at the rigidity door
/// (`NodeErrorKind::Motion`), which a widened rotation's enclosure
/// cannot pass by its seven margins alone. Over a box this wide the
/// bolt still refuses, at the placed edge's endpoint certification
/// (`work/shelf/transform-rigid-recertifies-images-enclosed-apart.md`).
///
/// *Breaks if* a placement's boxed rotation is decided by its widened
/// matrix's orthogonality residuals rather than admitted as a rotation.
#[test]
fn a2_a_wide_box_on_a_spin_is_no_rigidity_refusal() {
    let s = slid_bolt("msolve14-a2-spin-wide");
    let ev = run_at::<Interval>(&s.doc, &boxed(&s.doc, &s.opts, spin(), -0.025, 0.025), None);
    for (what, node) in [("the placement", s.mate.placement), ("the bolt", s.bolt)] {
        let kind = ev.node_error(node).map(|e| &e.kind);
        assert!(
            !matches!(kind, Some(editor_core::NodeErrorKind::Motion(_))),
            "{what} refuses at the rigidity door over a boxed spin: {kind:?}"
        );
    }
}

/// **A tilted target plane moved within itself moves no placed map**:
/// a bolt rests on the turned slab's top, its target that top slid
/// along the slab's own side by `drift` — a move inside the plane,
/// which a plane's value forgets. Over a box on `drift` the bolt's
/// placed map is as thin as the evaluation's rounding, and its `f64`
/// pose at the box's corners is the nominal's.
///
/// *Breaks if* a plane's value holds a point in the plane that the
/// fold reads: the box on `drift` then widens the map by its own width.
#[test]
fn a_tilted_plane_slid_within_itself_leaves_the_map_thin() {
    use geom_core::Bounds;
    let p = parts("msolve14-drift");
    let (doc, slab) = turned_slab(&p, "msolve14-drift");
    let doc = declare(doc, drift(), 0.0, Dimension::Length);
    let (doc, bolt) = insert(doc, fixture::mated_instance(p.bolt));
    let (x, y) = seat_on_the_pinned_slab(0.25, 2.0);
    let (doc, mate) = fixture::mated(
        doc,
        bolt,
        fixture::mate(
            fixture::plane(head(p.bolt_foot(bolt))),
            fixture::flip(slid(
                fixture::plane(head(p.slab_top(slab))),
                p.slab_side(slab, 1),
                named(drift(), Dimension::Length),
            )),
        ),
        &[
            (Coord::Slide(TargetAxis::X), len(x)),
            (Coord::Slide(TargetAxis::Y), len(y)),
            (Coord::SPIN, Formula::zero(Dimension::Angle)),
        ],
    );
    let opts = p.opts();
    let (lo, hi) = narrow(0.05);
    let poses = editor_core::mate::solve_document_at::<Interval>(
        &doc,
        &boxed(&doc, &opts, drift(), lo, hi),
        Tol::witness(),
    );
    assert!(poses.fault(bolt).is_none(), "{:?}", poses.fault(bolt));
    let map = poses
        .relative_map(mate.placement)
        .expect("posed over the box");
    let widest = [map.linear.c0, map.linear.c1, map.linear.c2, map.translation]
        .into_iter()
        .flat_map(|c| [c.x, c.y, c.z])
        .map(|e| e.hi() - e.lo())
        .fold(0.0_f64, f64::max);
    assert!(
        widest <= 1e-13,
        "the drift's box, {} wide, widens the placed map to {widest}",
        hi - lo
    );
    let at = |v: f64| {
        solve(&set_value(doc.clone(), drift(), v), &opts, Tol::witness())
            .relative(bolt)
            .expect("posed at f64")
            .affine::<f64>()
    };
    let nominal = at(0.0);
    for v in [lo, hi, 1.0] {
        let moved = at(v);
        let gap = (moved.translation - nominal.translation).norm();
        assert!(gap <= 1e-12, "a drift of {v} moves the bolt by {gap}");
    }
}

/// **A box on a tilted standoff widens the placed map by the box
/// alone**: a bolt rests on the turned slab's top lifted `gap` along its
/// normal, its origin set by its two slides. Over a box on `gap` the
/// placed translation is `x·X + y·Y + d·depth`, so each component is
/// as wide as the box's share of it along the slab's normal `d`, and
/// the linear part is as thin as the evaluation's rounding.
///
/// *Breaks if* the slides are set as differences of the folded map's
/// translation with itself, which on a chart off the axes carries the
/// depth's width into the slid components twice.
#[test]
fn a_box_on_a_tilted_standoff_widens_the_map_by_the_box_alone() {
    use geom_core::Bounds;
    let p = parts("msolve14-tilted-gap");
    let (doc, slab) = turned_slab(&p, "msolve14-tilted-gap");
    let doc = declare(doc, gap(), GAP, Dimension::Length);
    let (doc, bolt) = insert(doc, fixture::mated_instance(p.bolt));
    let (x, y) = seat_on_the_pinned_slab(0.25, 2.0);
    let (doc, mate) = seat(
        doc,
        bolt,
        p.bolt_foot(bolt),
        p.slab_top(slab),
        named(gap(), Dimension::Length),
        at(x, y, 0.0),
    );
    let opts = p.opts();
    let (lo, hi) = narrow(0.05);
    let poses = editor_core::mate::solve_document_at::<Interval>(
        &doc,
        &boxed(&doc, &opts, gap(), lo, hi),
        Tol::witness(),
    );
    assert!(poses.fault(bolt).is_none(), "{:?}", poses.fault(bolt));
    let map = poses
        .relative_map(mate.placement)
        .expect("posed over the box");
    let width = |e: Interval| e.hi() - e.lo();
    let linear = [map.linear.c0, map.linear.c1, map.linear.c2]
        .into_iter()
        .flat_map(|c| [c.x, c.y, c.z])
        .map(width)
        .fold(0.0_f64, f64::max);
    assert!(
        linear <= 1e-13,
        "the box on the standoff turns the map: {linear}"
    );
    // The slab's normal `Rz(0.25)·Rx(0.3)·z`.
    let (s3, c3) = (0.3_f64.sin(), 0.3_f64.cos());
    let (st, ct) = (0.25_f64.sin(), 0.25_f64.cos());
    let d = [st * s3, -ct * s3, c3];
    let t = map.translation;
    for (k, (e, n)) in [t.x, t.y, t.z].into_iter().zip(d).enumerate() {
        let share = (hi - lo) * n.abs();
        assert!(
            width(e) <= share + 1e-13,
            "translation component {k} is {} wide, its share of the box {share}",
            width(e)
        );
    }
}

// ---- C2: a clocking with no radius to turn ----

/// **`placement` is refused as overconstrained, naming both of the
/// shaft's bores, or escalates**: the bundle pins the bolt with the two
/// coaxial bores on one line, either of which the rest already fix, so
/// the solve refuses it naming each ([`MateFault::Overconstrained`]);
/// where a decision on the way could not settle, the fault is
/// `Indeterminate` over a margin of `kind` (a point at `f64` and
/// `Dual64`, an enclosure at `Interval` — never a poisoned one), on the
/// placement's own log. Answers whether it escalated.
///
/// [`MateFault::Overconstrained`]: editor_core::MateFault::Overconstrained
fn overconstrained_or_escalates<T: geom_core::Decide>(
    ev: &Evaluation<T>,
    s: &Shaft,
    kind: geom_core::MarginKind,
    what: &str,
) -> bool {
    let error = ev
        .node_error(s.bolt)
        .unwrap_or_else(|| panic!("{what}: the shaft's bundle is refused"));
    if let Some(diag) = escalation(ev, s.bolt) {
        assert_eq!(diag.margin.kind(), kind, "{what}: {diag:?}");
        assert!(
            !error.escalations.is_empty(),
            "{what}: the escalation is on the placement's log"
        );
        return true;
    }
    assert!(
        matches!(
            &error.kind,
            editor_core::NodeErrorKind::Mate(fault)
                if matches!(
                    &**fault,
                    editor_core::MateFault::Overconstrained { placement, constraints, .. }
                        if *placement == s.bolt
                            && constraints.contains(&s.mates[0].id)
                            && constraints.contains(&s.mates[1].id)
                )
        ),
        "{what}: the bundle is refused overconstrained, naming both bores: {error:?}"
    );
    false
}

/// **A shaft in two coaxial bores is overconstrained in every lane**
/// ([`shaft`]): the bundle holds two bores, a rest and a third bore
/// beside one another (past the door, as a doctored save leaves them),
/// and the placement refuses it naming both bores — at `f64`, at
/// `Dual64` seeded on `rise` and on `idle`, and at `Interval` over a
/// box on `rise` a fraction of ε wide. The rule reads the families,
/// so no lane measures anything to escalate on ([`INTERVAL_ESCALATIONS`]
/// holds no shaft row).
///
/// *Breaks if* a lane decides the bundle's overconstraint otherwise
/// than `f64` does.
#[test]
fn c2_a_shaft_in_two_coaxial_bores_is_overconstrained_in_every_lane() {
    use geom_core::MarginKind;
    let eps = Tol::witness().eps();
    let label = "shaft";
    let s = shaft("msolve14-c2-shaft", (0.0, 0.0));
    let f = run_at::<f64>(&s.doc, &s.opts, None);
    assert!(
        !overconstrained_or_escalates(&f, &s, MarginKind::Value, "f64"),
        "f64 decides"
    );
    for param in [rise(), idle()] {
        let d = run_at::<Dual64>(&s.doc, &seeded(&s.doc, &s.opts, param.clone()), None);
        assert!(
            !overconstrained_or_escalates(
                &d,
                &s,
                MarginKind::Value,
                &format!("Dual64 seeded on {param:?}")
            ),
            "Dual64 decides"
        );
    }
    let escalates = INTERVAL_ESCALATIONS
        .iter()
        .any(|(e, l, _)| e.to_bits() == eps.to_bits() && *l == label);
    let (lo, hi) = narrow(0.05);
    let i = run_at::<Interval>(&s.doc, &boxed(&s.doc, &s.opts, rise(), lo, hi), None);
    assert_eq!(
        overconstrained_or_escalates(&i, &s, MarginKind::Enclosure, "Interval"),
        escalates,
        "Interval escalates exactly where the corpus's escalations say"
    );
}

/// **A near-degenerate shaft refuses as any shaft does, measuring
/// nothing**: its second bore's line a rounding off the first's (slid a
/// non-dyadic distance along the slab's side and back), or three ε off
/// it, inside the band. In every lane — `f64`, `Dual64` unseeded and
/// seeded on `rise`, `Interval` unboxed — the bolt's bundle is refused
/// overconstrained by its families, with no escalation: the bores'
/// nearness is never decided.
///
/// *Breaks if* a lane measures the bores (escalating in the band, or
/// reading a NaN out of a near-degenerate fold).
#[test]
fn c2_a_near_degenerate_shaft_decides_or_escalates_and_never_reads_nan() {
    use geom_core::MarginKind;
    let eps = Tol::witness().eps();
    for (variant, off) in [("rounding", (0.1, -0.1)), ("in-band", (3.0 * eps, 0.0))] {
        let s = shaft(&format!("msolve14-c2-near-{variant}"), off);
        let f = run_at::<f64>(&s.doc, &s.opts, None);
        let escalated =
            overconstrained_or_escalates(&f, &s, MarginKind::Value, &format!("{variant}, f64"));
        assert!(!escalated, "{variant}: the bundle rule measures nothing");
        for opts in [s.opts.clone(), seeded(&s.doc, &s.opts, rise())] {
            let d = run_at::<Dual64>(&s.doc, &opts, None);
            assert_eq!(
                overconstrained_or_escalates(
                    &d,
                    &s,
                    MarginKind::Value,
                    &format!("{variant}, Dual64 {:?}", opts.seed),
                ),
                escalated,
                "{variant}: Dual64 decides where f64 does"
            );
        }
        let i = run_at::<Interval>(&s.doc, &s.opts, None);
        overconstrained_or_escalates(
            &i,
            &s,
            MarginKind::Enclosure,
            &format!("{variant}, Interval"),
        );
    }
}

// ---- A4: the memo key carries the pose's two channels ----

/// **A seeded pass threaded an unseeded `Dual64` prior reuses no
/// zero-tangent pose** — `stackup`'s arrangement, through the front
/// door. The bolt's DAG inputs are the seed-free slab and part, and its
/// one read of the seed is its seat's slide value, whose value channel
/// is the prior's: a key that fed that channel alone would serve the
/// prior's bolt, tangent zero, and copy #2 would then move by `+2`.
///
/// *Breaks if* a placement's key misses a tangent its values carry.
#[test]
fn a4_a_seeded_pass_over_an_unseeded_prior_reuses_no_zero_tangent_pose() {
    let b = bolted("msolve14-a4");
    let base_opts = b.opts.clone();
    let base = run_at::<Dual64>(&b.doc, &base_opts, None);
    for (name, p) in vertices(&base, b.bolt) {
        assert_eq!(
            [p.x.deriv, p.y.deriv, p.z.deriv],
            [0.0; 3],
            "the unseeded base carries no tangent at {name:?}"
        );
    }
    let pass = run_at::<Dual64>(&b.doc, &seeded(&b.doc, &b.opts, spacing()), Some(&base));
    for (name, p) in vertices(&pass, b.bolt) {
        assert_eq!(
            [p.x.deriv, p.y.deriv, p.z.deriv],
            [-2.0, 0.0, 0.0],
            "the seeded pass's bolt at {name:?} carries ∂B/∂s, not the prior's zero"
        );
    }
    for (name, p) in vertices(&pass, b.pattern) {
        let want = [-2.0, -1.0, 0.0][copy_of(&name) as usize];
        assert_eq!(p.x.deriv, want, "∂copy/∂s at {name:?} over the prior");
    }
}

// ---- A3's other half, and C5: one structure, every lane ----

/// What a lane's evaluation says about the structure: each placement's
/// value kind and space, or the class of its fault ([`fault_class`]).
fn structure<T: editor_core::EvalScalar>(
    doc: &ProfileDoc,
    ev: &Evaluation<T>,
) -> Vec<(RecipeNodeId, String)> {
    doc.placements()
        .into_iter()
        .map(|id| {
            let said = match ev.result(id) {
                Some(NodeResult::Ok(v)) => {
                    format!("{} in {:?}", v.payload.kind_name(), doc.node_space(id))
                }
                Some(other) => format!("failed: {}", fault_class(other)),
                None => "absent".to_string(),
            };
            (id, said)
        })
        .collect()
}

/// A failed result's class: the error kind's variant, and for a mate
/// fault the fault's — never its numbers, which are the lane's.
fn fault_class<T: geom_core::Decide>(r: &NodeResult<T>) -> String {
    let variant = |debug: String| {
        debug
            .split(|c: char| !c.is_alphanumeric())
            .next()
            .unwrap_or_default()
            .to_string()
    };
    match r {
        NodeResult::Failed(e) => match &e.kind {
            editor_core::NodeErrorKind::Mate(fault) => {
                format!("Mate::{}", variant(format!("{fault:?}")))
            }
            other => variant(format!("{other:?}")),
        },
        NodeResult::Poisoned { .. } => "Poisoned".to_string(),
        NodeResult::Ok(_) => unreachable!("an Ok result has a value, not a fault class"),
    }
}

/// **Every escalation an unboxed `Interval` run of the corpus makes**:
/// the ε row, the document, and the predicate whose enclosure straddled
/// the band. An identically-zero membership margin (the constructed
/// candidate's own residual) encloses zero only as tightly as the
/// lane's rounding, so at a fine enough ε it escalates with no box at
/// all (the spec's §5 (b),
/// `work/msolve/an-identically-zero-margin-escalates-at-a-fine-eps.md`).
/// Over this corpus, measured at ε = 1e-9, 1e-6 and 1e-12: none.
const INTERVAL_ESCALATIONS: [(f64, &str, &str); 0] = [];

/// The escalation `id` fails with in `ev`, if its fault is one: the
/// first escalation its own log carries (certifying the placed copy's
/// edges, downstream of the solve). No mate fault carries one of its
/// own.
fn escalation<T: geom_core::Decide>(
    ev: &Evaluation<T>,
    id: RecipeNodeId,
) -> Option<geom_core::predicate::Indeterminate> {
    let error = ev.node_error(id)?;
    error.escalations.first().map(|e| e.source)
}

/// **An interval lane's structure is the `f64` one's, or one of the
/// corpus's own escalations** ([`INTERVAL_ESCALATIONS`]): where an
/// interval decision could not settle, the placement faults
/// `Indeterminate` over an enclosure (never a poisoned margin), under a
/// predicate the table names for this document at this ε; the
/// escalation is on a placement's own log; everything else reads as at
/// `f64`. A document the table names escalates.
fn assert_interval_structure(
    doc: &ProfileDoc,
    f: &Evaluation<f64>,
    i: &Evaluation<Interval>,
    label: &str,
) {
    use geom_core::MarginKind;
    let eps = Tol::witness().eps();
    let allowed: Vec<&str> = INTERVAL_ESCALATIONS
        .iter()
        .filter(|(e, l, _)| e.to_bits() == eps.to_bits() && *l == label)
        .map(|(_, _, predicate)| *predicate)
        .collect();
    let log_escalated = fixture::constrained(doc)
        .into_iter()
        .any(|id| match i.result(id) {
            Some(NodeResult::Ok(v)) => !v.escalations.is_empty(),
            _ => i.node_error(id).is_some_and(|e| !e.escalations.is_empty()),
        });
    let want = structure(doc, f);
    let got = structure(doc, i);
    let mut moved = Vec::new();
    for ((id, w), (_, g)) in want.iter().zip(&got) {
        if w == g {
            continue;
        }
        let diag = escalation(i, *id).unwrap_or_else(|| {
            panic!(
                "{label}: Interval reads {g} at {id:?} where f64 reads {w}, and its fault is no \
                 escalation"
            )
        });
        assert_eq!(
            diag.margin.kind(),
            MarginKind::Enclosure,
            "{label}: the escalation at {id:?} is over an enclosure, not a poisoned margin: {diag:?}"
        );
        assert!(
            diag.predicate.is_some_and(|p| allowed.contains(&p)),
            "{label}: Interval escalates at {id:?} on {:?} at ε = {eps:e}, which the corpus's \
             escalations do not list ({allowed:?})",
            diag.predicate
        );
        moved.push(*id);
    }
    assert_eq!(
        moved.is_empty(),
        allowed.is_empty(),
        "{label}: at ε = {eps:e} the corpus's escalations list {allowed:?}, and Interval \
         escalated at {moved:?}"
    );
    if !moved.is_empty() {
        assert!(
            log_escalated,
            "{label}: Interval escalated at {moved:?} with no escalation on a placement's log"
        );
    }
}

/// **C5 and A3's dual half**: over the whole corpus, the solve's
/// structure — every placement's space, and which placements fail —
/// is the `f64` build's at `Dual64` (unseeded and under every seed) and
/// at `Interval` (unboxed), save where an interval decision ESCALATES
/// ([`assert_interval_structure`]); and at `Dual64` every instance's
/// every vertex reads the `f64` build's bits on its value channel and a
/// finite tangent.
#[test]
fn c5_one_documents_structure_is_the_same_in_every_lane_and_the_dual_value_is_f64s() {
    for (label, doc, opts) in corpus() {
        let f = run_at::<f64>(&doc, &opts, None);
        let d = run_at::<Dual64>(&doc, &opts, None);
        let i = run_at::<Interval>(&doc, &opts, None);
        let want = structure(&doc, &f);
        assert_eq!(structure(&doc, &d), want, "{label}: Dual64's structure");
        assert_interval_structure(&doc, &f, &i, label);
        // Every free continuous variable is a seed; a defined one takes
        // none (its derivative is its inputs').
        let params: Vec<editor_core::VarId> = doc
            .free_vars()
            .filter(|(_, free)| matches!(free, editor_core::FreeVar::Continuous { .. }))
            .map(|(id, _)| id)
            .collect();
        let seeds = std::iter::once(None).chain(params.into_iter().map(Some));
        for seed in seeds {
            let o = EvalOptions {
                seed,
                ..opts.clone()
            };
            let d = run_at::<Dual64>(&doc, &o, None);
            assert_eq!(structure(&doc, &d), want, "{label}: seeded {seed:?}");
            for id in doc.ids() {
                if !matches!(doc.node(id), Some(Node::InstantiatePart { .. }))
                    || f.node_error(id).is_some()
                {
                    continue;
                }
                let at: BTreeMap<StableName, [u64; 3]> = vertices(&f, id)
                    .into_iter()
                    .map(|(n, p)| (n, [p.x, p.y, p.z].map(f64::to_bits)))
                    .collect();
                for (name, p) in vertices(&d, id) {
                    assert_eq!(
                        [p.x.value, p.y.value, p.z.value].map(f64::to_bits),
                        at[&name],
                        "{label}, seed {seed:?}: the value channel at {name:?} is f64's"
                    );
                    assert!(
                        [p.x.deriv, p.y.deriv, p.z.deriv]
                            .iter()
                            .all(|t| t.is_finite()),
                        "{label}, seed {seed:?}: the tangent at {name:?} is finite: {:?}",
                        [p.x.deriv, p.y.deriv, p.z.deriv]
                    );
                }
            }
        }
    }
}

// ---- A5: the two analysis doors over a seated assembly ----

/// **`stackup::sensitivities` over the seated bolt**: the distance
/// from a vertex of the bolt to a vertex of the slab, differentiated in
/// the spacing, is the closed form — ∂m/∂s crosses the seat's value.
///
/// *Breaks if* the driver's pass loses the value's tangent.
#[test]
fn a5_sensitivities_cross_a_seated_mate() {
    use editor_core::stackup::{SensitivityOutcome, SensitivityRefusal, sensitivities};
    use editor_core::{MeasurePrimitive, SitedRef};
    let b = bolted("msolve14-a5-stackup");
    let ev = run_at::<f64>(&b.doc, &b.opts, None);
    let foot = all_vertices(&ev, b.bolt)[0].clone();
    let corner = all_vertices(&ev, b.slab)[0].clone();
    let (doc, m) = crate::fixture::measure_node(
        &b.doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            SitedRef::new(b.bolt, foot.clone()),
            SitedRef::new(b.slab, corner.clone()),
        ],
    );
    // The closed form: the bolt moves by `−2` per unit of spacing along
    // `x` and the slab holds still, so `∂|d|/∂s = d · (−2, 0, 0) / |d|`
    // over the vertices' own `f64` positions.
    let p = vertex_position(&ev, b.bolt, &foot).expect("the bolt's vertex");
    let q = vertex_position(&ev, b.slab, &corner).expect("the slab's vertex");
    let d = [p.x - q.x, p.y - q.y, p.z - q.z];
    let fd = -2.0 * d[0] / (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    assert!(fd.abs() > 0.1, "the measure moves with the spacing: {fd}");
    let resolver = b.opts.resolver.clone().expect("the store resolves");
    // The spacing toleranced, so the driver varies it (VR8: an
    // untoleranced variable is a constant of the analysis).
    let spacing = doc.var_named("s").expect("the spacing parameter");
    let (doc, _) = crate::fixture::step(
        doc,
        editor_core::DocEdit::SetVarDistribution {
            var: spacing.into(),
            distribution: Some(editor_core::Distribution::Normal { sigma: 1e-4 }),
        },
    );
    let entries = sensitivities(
        &doc,
        crate::fixture::output(&doc, m),
        None,
        None,
        false,
        Some(&resolver),
        Tol::witness(),
    )
    .expect("the driver runs");
    assert_eq!(entries.len(), 1, "one entry, the toleranced spacing's");
    let entry = entries
        .iter()
        .find(|e| e.param == spacing)
        .expect("an entry for the spacing");
    match &entry.outcome {
        SensitivityOutcome::Derivative { value, .. } => assert!(
            (value - fd).abs() <= 1e-12,
            "∂m/∂s is {value} where the closed form gives {fd}"
        ),
        other => panic!("a derivative across the seat, not {other:?}"),
    }
    // A record build over no resolver is not this build: it resolved no
    // part where the passes resolved the bolt's, and the binding says so.
    // Over the store it binds.
    let record = |opts: EvalOptions| {
        sensitivities(
            &doc,
            crate::fixture::output(&doc, m),
            Some(&run_at::<f64>(
                &doc,
                &EvalOptions { pin: true, ..opts },
                None,
            )),
            None,
            false,
            Some(&resolver),
            Tol::witness(),
        )
    };
    assert!(record(b.opts.clone()).is_ok(), "the store's build binds");
    assert_eq!(
        record(EvalOptions::default()).err(),
        Some(SensitivityRefusal::Binding(
            editor_core::stackup::BindingViolation::Parts
        )),
        "a record build over another resolver fails the binding"
    );
    // The other direction, which is what an edited part file looks
    // like (A4: its bytes no longer hash to the reference's pin, so the
    // pin resolves to nothing): the record resolved the bolt over the
    // store, and the passes run over a store that no longer holds it.
    let gone = with_resolver(PartStore::new())
        .resolver
        .expect("a resolver over the emptied store");
    assert_eq!(
        sensitivities(
            &doc,
            crate::fixture::output(&doc, m),
            Some(&run_at::<f64>(
                &doc,
                &EvalOptions {
                    pin: true,
                    ..b.opts.clone()
                },
                None,
            )),
            None,
            false,
            Some(&gone),
            Tol::witness(),
        )
        .err(),
        Some(SensitivityRefusal::Binding(
            editor_core::stackup::BindingViolation::Parts
        )),
        "passes over a store whose part file changed fail the binding"
    );
}

/// **Certified `clearance` over the seated bolt**: the separation
/// between the bolt and copy #2 is `2s − 1`, three at the nominal, so a
/// leaf around it holds a clearance of 2.5 and violates one of 3.5 —
/// each answered over the leaf, the bundle folded at its scalar.
///
/// *Breaks if* the leaf's fold holds the bolt at its nominal pose.
#[test]
fn a5_certified_clearance_runs_over_a_seated_mate() {
    use editor_core::clearance::{
        ClearanceQuery, ClearanceVerdict, FaceScope, Selection, clearance_with,
    };
    let b = bolted("msolve14-a5-clearance");
    let resolver = b.opts.resolver.clone().expect("the store resolves");
    let (lo, hi) = narrow(0.05);
    let leaf = ParamBox::from_axes(BTreeMap::from([(
        var_of(&b.doc, &spacing()),
        BoxAxis::Varying { lo, hi },
    )]));
    let copy2 = Selection {
        at: b.pattern,
        body: 2,
        faces: FaceScope::All,
    };
    let ask = |c: f64| {
        clearance_with(
            &b.doc,
            &leaf,
            &Selection::body_of(b.bolt),
            &copy2,
            &ClearanceQuery {
                bound: editor_core::clearance::ClearanceBound::at_least(c),
                ..ClearanceQuery::strictly_positive(Tol::witness())
            }
            .resolved_by(&resolver),
        )
        .verdict()
        .clone()
    };
    assert!(
        matches!(ask(2.5), ClearanceVerdict::Holds),
        "2s − 1 = 3 clears 2.5 over the leaf: {:?}",
        ask(2.5)
    );
    assert!(
        matches!(ask(3.5), ClearanceVerdict::Violated(_)),
        "2s − 1 = 3 violates 3.5 over the leaf: {:?}",
        ask(3.5)
    );
    // The verdicts alone cannot tell a solve at the leaf's scalar from
    // one held at the nominal: the leaf is a fraction of ε (the
    // placement door's limit) and they are decided against ε. The
    // leaf's own evaluation can: the bolt moves by −2 per unit of `s`,
    // so its enclosure holds the f64 builds' bolt at both corners, which
    // a bolt held at its nominal pose would not.
    let ev = run_at::<Interval>(&b.doc, &boxed(&b.doc, &b.opts, spacing(), lo, hi), None);
    assert_encloses_corners(
        &b.doc,
        &b.opts,
        &ev,
        b.bolt,
        (spacing(), SPACING),
        (lo, hi),
        "the clearance leaf",
    );
}

/// R1 probe (INTENT E review): clearance asked in either selection's
/// frame answers alike.
#[test]
fn r1_clearance_answers_alike_in_either_order() {
    use editor_core::clearance::{ClearanceQuery, FaceScope, Selection, clearance_with};
    let b = bolted("msolve14-a5-clearance");
    let resolver = b.opts.resolver.clone().expect("the store resolves");
    let (lo, hi) = narrow(0.05);
    let leaf = ParamBox::from_axes(BTreeMap::from([(
        var_of(&b.doc, &spacing()),
        BoxAxis::Varying { lo, hi },
    )]));
    let copy2 = Selection {
        at: b.pattern,
        body: 2,
        faces: FaceScope::All,
    };
    let bolt = Selection::body_of(b.bolt);
    let ask = |x: &Selection, y: &Selection, c: f64| {
        let r = clearance_with(
            &b.doc,
            &leaf,
            x,
            y,
            &ClearanceQuery {
                bound: editor_core::clearance::ClearanceBound::at_least(c),
                ..ClearanceQuery::strictly_positive(Tol::witness())
            }
            .resolved_by(&resolver),
        );
        format!("{:?}", r.verdict())
            .chars()
            .take(60)
            .collect::<String>()
    };
    let mut apart = Vec::new();
    for c in [
        2.5, 2.9, 2.99, 2.999, 2.9999, 2.99999, 3.0, 3.00001, 3.0001, 3.001, 3.01, 3.5,
    ] {
        let (p, q) = (ask(&bolt, &copy2, c), ask(&copy2, &bolt, c));
        eprintln!("c={c}: bolt-first {p} | copy-first {q}");
        let kind = |s: &str| s.split([' ', '(', '{']).next().unwrap_or("").to_owned();
        if kind(&p) != kind(&q) {
            apart.push(format!("c={c}: {p} vs {q}"));
        }
    }
    assert!(apart.is_empty(), "the order decides: {apart:#?}");
}
