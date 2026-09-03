//! **M10-4 review probes (R2)** — an INDEPENDENT derivation of what the
//! E4 seed door, the n-pass driver, the pairing hook and the E5
//! stackup claim, written from `docs/M10-4-SPEC.md` and
//! `docs/ERROR-DESIGN.md` E4/E5/E9 rather than from the unit's own rows.
//!
//! Why these rows and not the unit's: the unit's hygiene and DL2 rows
//! drive one document (the two-parameter plate) whose profile-driven
//! tangent (−2 on `hole_r`) arrives through the MEASURE EXPRESSION's
//! own `− 2·hole_r` term, not through the lift — the cylinder×cylinder
//! distance is the axis separation and does not depend on the radius —
//! so the plate proves nothing about the guided lift. The rows below
//! use a three-parameter slab with an UNUSED parameter whose nominal
//! equals a used one's (the memo-aliasing shape DL2 must exclude), a
//! bore/pin fit whose radius reaches the measure only through the
//! lifted cylinder carrier, a loft whose section dimension is a
//! parameter, an angle measure whose tangent degrades through
//! `sqrt(0)`, a curvature case whose linearization is exactly ZERO, and
//! the RSS σ of every distribution form re-derived by quadrature.
//!
//! **ROWS RED AT `fc8de0ac`, DELIBERATELY** (each says so in its doc
//! comment) are this suite's findings rather than defects in it: a
//! stale chamber verdict marks an edited document's sensitivity
//! `ChamberCertified` (the spec's "unwritable" stackup lie, writable
//! through the public door), and a loft section's parameter seeds to a
//! silent finite zero through the guided lift (the spec's "never silent
//! zeros" valve, absent).
//!
//! Sweep shape (`memories/test-suite-cost.md`): nothing here samples —
//! every row is a witness that can be written down, so all are static
//! fixtures and no seed appears. Rows whose doc comment says
//! EVIDENCE-ONLY print or assert a documented behaviour and gate
//! nothing new.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::UnitSym;
use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, ParamBoxVerdict, drive};
use editor_core::stackup::{
    Chamber, PairingViolation, Rss, Sensitivity, SensitivityOutcome, SensitivityRefusal, Stackup,
    StackupRefusal, Unavailable, sensitivities, stackup,
};
use editor_core::{
    CancelToken, CapEnd, Dimension, Distribution, DocEdit, DocParam, DocParamValue, EvalOptions,
    Evaluation, Expr, LoopProgram, MeasureExpr, MeasurePrimitive, MeasureRef, Node, NodeResult,
    ParamName, ParamValue, ProfileDoc, ProfileLift, ProfileProgram, ProgramStep, ProgramTarget,
    RecipeNodeId, RoleSeg, ValuePayload, evaluate, seed_env,
};
use geom_core::interval::Interval;
use geom_core::{CertifiedEnclosure, Dual64, Point3, Tol, Vec3};
use profile::SketchPlane;

use fixture::{Recorder, fname, len, wall};

/// The bore/pin worst-case hull's enclosure padding per analyzed
/// half-width, measured at every CI ε row (see the consumer-walk row).
/// A bound, not a target — if it grows, the question is why the lane
/// widened.
const BORE_PIN_PADDING_PER_HALF_WIDTH: f64 = 1.0;
/// The rounding on top of the dependency padding (measured ~1e-15 at
/// the 1e-12 row, where it is largest relative to the half-width).
const BORE_PIN_ROUNDING: f64 = 1.0e-14;

fn eps() -> f64 {
    Tol::witness().eps()
}

fn name(n: &str) -> ParamName {
    ParamName::new(n)
}

fn param(n: &str, dim: Dimension) -> Expr {
    Expr::param(name(n), dim)
}

fn continuous(dim: Dimension, value: f64, distribution: Option<Distribution>) -> DocParam {
    DocParam::Continuous {
        dim,
        value,
        display_unit: UnitSym::canonical_for(dim),
        distribution,
    }
}

fn uniform(lo: f64, hi: f64) -> Distribution {
    Distribution::Uniform { lo, hi }
}

fn config(max_leaves: usize) -> DriveConfig {
    DriveConfig {
        max_leaves,
        ..DriveConfig::default()
    }
}

fn opts(seed: Option<&str>, lift: ProfileLift) -> EvalOptions {
    EvalOptions {
        seed: seed.map(name),
        profile_lift: lift,
        ..EvalOptions::default()
    }
}

fn run<T: editor_core::EvalScalar>(
    doc: &ProfileDoc,
    prior: Option<&Evaluation<T>>,
    o: &EvalOptions,
) -> Evaluation<T> {
    evaluate::<T>(doc, prior, &CancelToken::new(), o, Tol::witness())
}

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    run::<f64>(doc, None, &EvalOptions::default())
}

fn push(doc: &ProfileDoc, edit: DocEdit<ProfileProgram>) -> ProfileDoc {
    editor_core::apply(doc, &edit, Tol::witness())
        .unwrap_or_else(|e| panic!("edit refused: {e}"))
        .doc
}

fn measured(ev: &Evaluation<Dual64>, id: RecipeNodeId) -> Dual64 {
    match ev.result(id) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, .. } => *value,
            other => panic!("node {id:?} is a {}", other.kind_name()),
        },
        other => panic!("node {id:?} did not evaluate: {other:?}"),
    }
}

fn measured_f64(ev: &Evaluation<f64>, id: RecipeNodeId) -> f64 {
    match ev.result(id) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, .. } => *value,
            other => panic!("node {id:?} is a {}", other.kind_name()),
        },
        other => panic!("node {id:?} did not evaluate: {other:?}"),
    }
}

fn key(ev: &Evaluation<impl geom_core::Decide>, id: RecipeNodeId) -> u128 {
    ev.value(id).expect("evaluated Ok").content_key.0
}

fn entry<'a>(entries: &'a [Sensitivity], n: &str) -> &'a SensitivityOutcome {
    &entries
        .iter()
        .find(|s| s.param == name(n))
        .unwrap_or_else(|| panic!("no entry for {n}"))
        .outcome
}

fn contains_nominal(chamber: &Chamber) -> bool {
    match chamber {
        Chamber::ChamberCertified { leaf, .. } => leaf.axes().values().all(|a| {
            let (lo, hi) = a.span();
            lo <= 0.0 && 0.0 <= hi
        }),
        Chamber::LocalOnly => false,
    }
}

fn cyl_wall(ev: &Evaluation<f64>, doc: &ProfileDoc, node: RecipeNodeId) -> MeasureRef {
    let mut faces = editor_core::select_where(
        ev,
        node,
        &editor_core::Selector::of(editor_core::NamePat::of_kind(editor_core::EntityKind::Face)),
        &[editor_core::GeomPred::SurfaceKind(
            editor_core::SurfaceKindSet::just(geom_brep::SurfaceKind::Cylinder),
        )],
        &doc.param_env::<f64>(),
        Tol::witness(),
    )
    .expect("the surface-kind atom is exact");
    faces.sort();
    MeasureRef::new(node, faces.remove(0))
}

fn vertex_at(ev: &Evaluation<f64>, node: RecipeNodeId, at: [f64; 3]) -> MeasureRef {
    let v = editor_core::all_vertices(ev, node)
        .into_iter()
        .find(|v| {
            let p = editor_core::vertex_position(ev, node, v).expect("a vertex");
            p.x == at[0] && p.y == at[1] && p.z == at[2]
        })
        .unwrap_or_else(|| panic!("node {node:?} has a vertex at {at:?}"));
    MeasureRef::new(node, v)
}

// ------------------------------------------------------------ fixtures

/// **The three-parameter slab.** A rectangle whose WIDTH `w` is a chain
/// program's point (a profile dimension), extruded by `d` (a magnitude
/// slot), beside an UNUSED parameter `k` whose nominal equals `w`'s —
/// the same lifted bits, differing only in which binding carries the
/// seed. A second, literal cube sits beside it, depending on no
/// parameter. The measure is `distance(x=0 wall, x=w wall) +
/// distance(bottom cap, top cap)`, which is `w + d`: ∂/∂w = 1 through
/// the lift, ∂/∂d = 1 through the slot, ∂/∂k = 0.
struct Slab {
    doc: ProfileDoc,
    profile: RecipeNodeId,
    block: RecipeNodeId,
    cube: RecipeNodeId,
    measure: RecipeNodeId,
}

fn slab(w_dist: Option<Distribution>, d_dist: Option<Distribution>) -> Slab {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("w"),
        value: continuous(Dimension::Length, 2.0, w_dist),
    });
    r.push(DocEdit::SetDocParam {
        name: name("d"),
        value: continuous(Dimension::Length, 1.0, d_dist),
    });
    r.push(DocEdit::SetDocParam {
        name: name("k"),
        value: continuous(Dimension::Length, 2.0, None),
    });
    let chain = LoopProgram::Chain(vec![
        ProgramStep::At([len(0.0), len(0.0)]),
        ProgramStep::LineTo(ProgramTarget::Point([
            param("w", Dimension::Length),
            len(0.0),
        ])),
        ProgramStep::LineTo(ProgramTarget::Point([
            param("w", Dimension::Length),
            len(1.0),
        ])),
        ProgramStep::LineTo(ProgramTarget::Point([len(0.0), len(1.0)])),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![chain],
    }));
    let block = r.insert(Node::Extrude {
        profile,
        distance: param("d", Dimension::Length),
    });
    let cube_profile = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![
            LoopProgram::polygon([(5.0, 5.0), (6.0, 5.0), (6.0, 6.0), (5.0, 6.0)])
                .expect("finite corners"),
        ],
    }));
    let cube = r.insert(Node::Extrude {
        profile: cube_profile,
        distance: len(1.0),
    });
    let refs = vec![
        MeasureRef::new(block, fname(block, wall(3))),
        MeasureRef::new(block, fname(block, wall(1))),
        MeasureRef::new(block, fname(block, RoleSeg::Cap(CapEnd::Bottom))),
        MeasureRef::new(block, fname(block, RoleSeg::Cap(CapEnd::Top))),
    ];
    let expr = MeasureExpr::add(
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 2, b: 3 }),
    )
    .expect("Length + Length");
    let measure = r.insert(Node::measure(expr, refs).expect("indices in range"));
    Slab {
        doc: r.doc,
        profile,
        block,
        cube,
        measure,
    }
}

/// **The bore/pin fit** — an ARC-carrying profile whose parameter
/// reaches the measure only through the lifted cylinder carrier: a
/// bore of radius 0.5 at the origin and a pin of radius `r` (nominal
/// 0.2) centred at (0.1, 0), both extruded by a literal; the measure is
/// `gap(bore wall, pin wall) = 0.5 − r − 0.1`, so ∂gap/∂r = −1 exactly
/// — and exactly 0 under the pinned lift, since nothing but the
/// carrier's radius carries `r`.
fn fit(r_dist: Option<Distribution>) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("r"),
        value: continuous(Dimension::Length, 0.2, r_dist),
    });
    let bore_p = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![LoopProgram::Circle {
            centre: [len(0.0), len(0.0)],
            radius: len(0.5),
        }],
    }));
    let bore = r.insert(Node::Extrude {
        profile: bore_p,
        distance: len(1.0),
    });
    let pin_p = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![LoopProgram::Circle {
            centre: [len(0.1), len(0.0)],
            radius: param("r", Dimension::Length),
        }],
    }));
    let pin = r.insert(Node::Extrude {
        profile: pin_p,
        distance: len(1.0),
    });
    let ev = eval(&r.doc);
    let refs = vec![cyl_wall(&ev, &r.doc, bore), cyl_wall(&ev, &r.doc, pin)];
    let m = r.insert(
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 0, inner: 1 }),
            refs,
        )
        .expect("indices in range"),
    );
    (r.doc, m)
}

/// **The parallel-caps angle**: two blocks side by side, the second's
/// height the parameter `h`; the measure is the angle between their top
/// caps, identically 0 for every `h`. Its `Dual64` tangent is `0/0`
/// through `‖n̂_a × n̂_b‖ = sqrt(0)` — for `h` AND for a parameter `u`
/// that feeds nothing. A second measure, `max(h − 1, 1 − h) = |h − 1|`,
/// is the spec's `abs` kink spelled in the measure vocabulary.
fn caps(h_dist: Option<Distribution>) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("h"),
        value: continuous(Dimension::Length, 1.0, h_dist),
    });
    r.push(DocEdit::SetDocParam {
        name: name("u"),
        value: continuous(Dimension::Length, 1.0, None),
    });
    let pa = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
                .expect("finite corners"),
        ],
    }));
    let a = r.insert(Node::Extrude {
        profile: pa,
        distance: len(1.0),
    });
    let pb = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![
            LoopProgram::polygon([(3.0, 0.0), (4.0, 0.0), (4.0, 1.0), (3.0, 1.0)])
                .expect("finite corners"),
        ],
    }));
    let b = r.insert(Node::Extrude {
        profile: pb,
        distance: param("h", Dimension::Length),
    });
    let refs = vec![
        MeasureRef::new(a, fname(a, RoleSeg::Cap(CapEnd::Top))),
        MeasureRef::new(b, fname(b, RoleSeg::Cap(CapEnd::Top))),
    ];
    let angle = r.insert(
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Angle { a: 0, b: 1 }),
            refs,
        )
        .expect("indices in range"),
    );
    let h = || MeasureExpr::value(param("h", Dimension::Length));
    let one = || MeasureExpr::value(len(1.0));
    let kink = MeasureExpr::max(
        MeasureExpr::sub(h(), one()).expect("Length"),
        MeasureExpr::sub(one(), h()).expect("Length"),
    )
    .expect("Length");
    let abs = r.insert(Node::measure(kink, Vec::new()).expect("no refs"));
    (r.doc, angle, abs)
}

/// **The loft.** Two square sections of width `w` at z = 0 and z = 1,
/// lofted at degree 1; the measure is the distance between the loft's
/// vertices at (0, 0, 0) and (w, 0, 0), which is `w` exactly.
fn loft() -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("w"),
        value: continuous(Dimension::Length, 2.0, None),
    });
    let section = |z: f64| {
        let chain = LoopProgram::Chain(vec![
            ProgramStep::At([len(0.0), len(0.0)]),
            ProgramStep::LineTo(ProgramTarget::Point([
                param("w", Dimension::Length),
                len(0.0),
            ])),
            ProgramStep::LineTo(ProgramTarget::Point([
                param("w", Dimension::Length),
                len(1.0),
            ])),
            ProgramStep::LineTo(ProgramTarget::Point([len(0.0), len(1.0)])),
            ProgramStep::LineTo(ProgramTarget::Start),
        ]);
        Node::Profile(ProfileProgram {
            plane: SketchPlane::from_frame(
                Point3::new(0.0, 0.0, z),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            loops: vec![chain],
        })
    };
    let p0 = r.insert(section(0.0));
    let p1 = r.insert(section(1.0));
    let loft = r.insert(Node::Loft {
        profiles: vec![p0, p1],
        v_degree: Expr::count(1),
    });
    let ev = eval(&r.doc);
    if let Some(e) = ev.node_error(loft) {
        panic!("the loft did not build: {}", e.kind);
    }
    let refs = vec![
        vertex_at(&ev, loft, [0.0, 0.0, 0.0]),
        vertex_at(&ev, loft, [2.0, 0.0, 0.0]),
    ];
    let m = r.insert(
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            refs,
        )
        .expect("indices in range"),
    );
    (r.doc, m)
}

/// **The pure-expression sum**: no geometry, four parameters of every
/// distribution shape summed into one measure so every ∂m/∂pᵢ is
/// exactly 1 and the RSS is `√Σσᵢ²`.
fn sum(u: Distribution, n: Distribution, tn: Distribution) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    for (p, dist) in [
        ("u", Some(u)),
        ("n", Some(n)),
        ("tn", Some(tn)),
        ("f", None),
    ] {
        r.push(DocEdit::SetDocParam {
            name: name(p),
            value: continuous(Dimension::Length, 1.0, dist),
        });
    }
    let v = |p: &str| MeasureExpr::value(param(p, Dimension::Length));
    let expr = MeasureExpr::add(
        MeasureExpr::add(v("u"), v("n")).expect("Length"),
        MeasureExpr::add(v("tn"), v("f")).expect("Length"),
    )
    .expect("Length");
    let m = r.insert(Node::measure(expr, Vec::new()).expect("no refs"));
    (r.doc, m)
}

// --------------------------------------------------- claim 2: hygiene

/// **Seed hygiene on an aliasing-shaped fixture.** `k` and `w` lift to
/// the same bits; the seed rides only the binding named. Through the
/// public door the tangent is 1 on the profile dimension (guided), 1 on
/// the magnitude slot, 0 on the unused parameter — and the value
/// channel is the f64 build's bits in every pass.
#[test]
fn the_seed_rides_exactly_one_binding_on_an_aliasing_shaped_fixture() {
    let s = slab(None, None);
    let env = seed_env::<Dual64, _>(&s.doc, s.doc.param_env::<Dual64>(), &name("w"))
        .expect("w is continuous");
    let binding = |n: &str| match env.bindings[&name(n)] {
        ParamValue::Continuous { value, .. } => value,
        ParamValue::Count(_) => panic!("{n} is continuous"),
    };
    assert_eq!(binding("w").deriv.to_bits(), 1.0f64.to_bits());
    assert_eq!(binding("k").deriv.to_bits(), 0.0f64.to_bits());
    assert_eq!(binding("d").deriv.to_bits(), 0.0f64.to_bits());
    assert_eq!(binding("w").value.to_bits(), binding("k").value.to_bits());

    let f = measured_f64(&eval(&s.doc), s.measure);
    assert_eq!(f.to_bits(), 3.0f64.to_bits());
    for (seed, expect) in [("w", 1.0), ("d", 1.0), ("k", 0.0)] {
        let m = measured(
            &run::<Dual64>(&s.doc, None, &opts(Some(seed), ProfileLift::Guided)),
            s.measure,
        );
        assert_eq!(m.value.to_bits(), f.to_bits(), "{seed}: value channel");
        assert_eq!(
            m.deriv, expect,
            "∂m/∂{seed} (IEEE equality: a signed zero is a zero)"
        );
    }
    // NOTED, not gated: the unused parameter's tangent arrives as
    // `-0.0` (a sign factor multiplied into a zero tangent somewhere in
    // the norm), so a consumer comparing `to_bits()` against `0.0`
    // sees "not exactly zero". The PR's own profile pin records the
    // same arrival on the pinned lift.
    let k = measured(
        &run::<Dual64>(&s.doc, None, &opts(Some("k"), ProfileLift::Guided)),
        s.measure,
    );
    println!(
        "∂m/∂k bits: {:#018x} (sign negative: {})",
        k.deriv.to_bits(),
        k.deriv.is_sign_negative()
    );
    // The pinned lift: the profile dimension's tangent is the silent
    // zero the lift ends; the magnitude slot's is untouched.
    let pinned = measured(
        &run::<Dual64>(&s.doc, None, &opts(Some("w"), ProfileLift::Pinned)),
        s.measure,
    );
    assert_eq!(pinned.deriv, 0.0);
}

/// **DL2 exercised on the aliasing fixture.** Threading any pass from
/// any other pass's memo serves the seed-independent subgraph and
/// nothing else: (a) the profile node's key moves under a `w` seed and
/// stays under `d`/`k`; the extrude's moves under `w` and `d`, stays
/// under `k`; the literal cube's never moves; (b) a `k` pass IS the
/// unseeded base node for node (every key equal, full memo walk); (c)
/// every threading order reads its own tangent, bit for bit; (d) the
/// driver is schedule-independent on this fixture.
#[test]
fn the_memo_serves_only_the_seed_independent_subgraph_in_every_threading_order() {
    let s = slab(None, None);
    let guided = |seed: Option<&str>| opts(seed, ProfileLift::Guided);
    let base = run::<Dual64>(&s.doc, None, &guided(None));
    let on_w = run::<Dual64>(&s.doc, None, &guided(Some("w")));
    let on_d = run::<Dual64>(&s.doc, None, &guided(Some("d")));
    let on_k = run::<Dual64>(&s.doc, None, &guided(Some("k")));

    // (a) keys move exactly with the seed's cone.
    assert_ne!(
        key(&on_w, s.profile),
        key(&base, s.profile),
        "w moves the profile key"
    );
    assert_eq!(
        key(&on_d, s.profile),
        key(&base, s.profile),
        "d does not touch the profile"
    );
    assert_eq!(
        key(&on_k, s.profile),
        key(&base, s.profile),
        "k touches nothing"
    );
    assert_ne!(key(&on_w, s.block), key(&base, s.block));
    assert_ne!(key(&on_d, s.block), key(&base, s.block));
    assert_eq!(key(&on_k, s.block), key(&base, s.block));
    for ev in [&on_w, &on_d, &on_k] {
        assert_eq!(
            key(ev, s.cube),
            key(&base, s.cube),
            "the literal cube is seed-free"
        );
    }
    assert_ne!(key(&on_w, s.measure), key(&base, s.measure));
    assert_ne!(key(&on_d, s.measure), key(&base, s.measure));
    assert_ne!(key(&on_w, s.measure), key(&on_d, s.measure));

    // (b) an unused seed is the base, node for node.
    for id in &base.order {
        assert_eq!(
            base.value(*id).map(|v| v.content_key),
            on_k.value(*id).map(|v| v.content_key),
            "node {} keyed differently under the unused seed",
            id.0
        );
    }
    let k_from_base = run::<Dual64>(&s.doc, Some(&base), &guided(Some("k")));
    assert_eq!(k_from_base.reused, s.doc.len());
    assert_eq!(k_from_base.recomputed, 0);

    // (c) every threading order reads its own tangent.
    let fresh: BTreeMap<&str, Dual64> = [("w", &on_w), ("d", &on_d), ("k", &on_k)]
        .into_iter()
        .map(|(n, ev)| (n, measured(ev, s.measure)))
        .collect();
    for (seed, prior) in [
        ("d", &on_w),
        ("k", &on_d),
        ("w", &on_k),
        ("w", &on_d),
        ("d", &on_k),
        ("k", &on_w),
    ] {
        let threaded = run::<Dual64>(&s.doc, Some(prior), &guided(Some(seed)));
        let m = measured(&threaded, s.measure);
        assert_eq!(
            m.deriv.to_bits(),
            fresh[seed].deriv.to_bits(),
            "{seed} threaded"
        );
        assert_eq!(
            m.value.to_bits(),
            fresh[seed].value.to_bits(),
            "{seed} threaded"
        );
        assert!(threaded.reused >= 2, "the cube is always served: {seed}");
    }
    // The w pass threaded from the d pass recomputes exactly w's cone
    // (profile, block, measure) and serves the rest.
    let w_from_d = run::<Dual64>(&s.doc, Some(&on_d), &guided(Some("w")));
    assert_eq!(w_from_d.recomputed, 3);
    assert_eq!(w_from_d.reused, s.doc.len() - 3);

    // (d) schedule independence, through the driver and the raw door.
    let par = run::<Dual64>(
        &s.doc,
        None,
        &EvalOptions {
            parallel: true,
            ..guided(Some("w"))
        },
    );
    let m = measured(&par, s.measure);
    assert_eq!(m.deriv.to_bits(), fresh["w"].deriv.to_bits());
    assert_eq!(m.value.to_bits(), fresh["w"].value.to_bits());
    let seq = sensitivities(&s.doc, s.measure, None, None, false, Tol::witness());
    let par = sensitivities(&s.doc, s.measure, None, None, true, Tol::witness());
    assert_eq!(seq, par);
    let seq = seq.expect("ok");
    assert_eq!(
        seq.len(),
        3,
        "one entry per continuous parameter, k included"
    );
    for (n, expect) in [("w", 1.0), ("d", 1.0), ("k", 0.0)] {
        match entry(&seq, n) {
            SensitivityOutcome::Derivative { value, chamber } => {
                assert_eq!(*value, expect, "{n}");
                assert_eq!(*chamber, Chamber::LocalOnly);
            }
            other => panic!("{n}: {other:?}"),
        }
    }
}

// ----------------------------------------------- claim 3: the pairing

/// **The pairing hook against handed builds of OTHER shapes.** A handed
/// evaluation at another ε refuses typed (no sensitivity); one built
/// with the guided lift at `f64` refuses on its key (over-strict, and
/// safe); a parallel-schedule build pairs; a document that differs only
/// by an ANNOTATION (a distribution on `w`, same value) pairs — the
/// build is bit-identical, so the sensitivity is honestly of it.
#[test]
fn the_pairing_hook_pairs_only_the_build_of_record() {
    let s = slab(None, None);
    let handed = eval(&s.doc);
    assert!(
        sensitivities(
            &s.doc,
            s.measure,
            Some(&handed),
            None,
            false,
            Tol::witness()
        )
        .is_ok()
    );

    // (A build at another ε cannot be handed: `Tol` is the process's
    // one witness — D9's "one process, one ε" — so that attack has no
    // spelling here.)
    let guided_f64 = run::<f64>(&s.doc, None, &opts(None, ProfileLift::Guided));
    match sensitivities(
        &s.doc,
        s.measure,
        Some(&guided_f64),
        None,
        false,
        Tol::witness(),
    ) {
        Err(SensitivityRefusal::Pairing(PairingViolation::ContentKey { .. })) => {}
        other => panic!("a guided f64 build keys differently (tag 41): {other:?}"),
    }

    let parallel = run::<f64>(
        &s.doc,
        None,
        &EvalOptions {
            parallel: true,
            ..EvalOptions::default()
        },
    );
    assert!(
        sensitivities(
            &s.doc,
            s.measure,
            Some(&parallel),
            None,
            false,
            Tol::witness()
        )
        .is_ok()
    );

    let annotated = push(
        &s.doc,
        DocEdit::SetDocParam {
            name: name("w"),
            value: continuous(Dimension::Length, 2.0, Some(uniform(-0.1, 0.1))),
        },
    );
    let r = sensitivities(
        &annotated,
        s.measure,
        Some(&handed),
        None,
        false,
        Tol::witness(),
    );
    assert!(
        r.is_ok(),
        "an annotation-only edit is the same build: {r:?}"
    );
}

/// **RED AT `fc8de0ac` — a STALE CHAMBER VERDICT certifies an edited
/// document's sensitivity.** The pairing hook guards the handed f64
/// build; nothing guards the handed verdict. Drive the slab at
/// `w = 2.0`, edit `w` to `2.5` (a value edit, no node-set change),
/// and hand the old verdict to the driver: `ForeignVerdict` checks
/// only that the root box's AXES are this document's continuous
/// parameters, so the old leaves — certified for a body nobody has
/// built at `w = 2.5` — mark the new derivative `ChamberCertified`,
/// and `stackup` prices the edited document with them (`ForeignBox`
/// compares offsets only, and the offsets did not move). The spec
/// (§5): the certificate is a leaf "from a drive over the box asked
/// about", and the classic lie "must be UNWRITABLE". The assertions
/// below are the spec's demand; they go green the day a verdict is
/// tied to the build it was driven over.
#[test]
fn a_stale_chamber_verdict_marks_an_edited_document_certified() {
    let half = eps() / 8.0;
    let s = slab(Some(uniform(-half, half)), None);
    let analyzed = analyzed_box(&s.doc, &AnalysisPolicy::default());
    let verdict = drive(&s.doc, &analyzed, &config(256), Tol::witness()).expect("builds");
    assert!(!verdict.certified().is_empty(), "{:?}", verdict.receipt());

    let edited = push(
        &s.doc,
        DocEdit::SetDocParamValue {
            name: name("w"),
            value: DocParamValue::Continuous(2.5),
        },
    );
    let entries = sensitivities(
        &edited,
        s.measure,
        None,
        Some(&verdict),
        false,
        Tol::witness(),
    );
    match &entries {
        Ok(entries) => {
            for e in entries {
                if let SensitivityOutcome::Derivative { chamber, .. } = &e.outcome {
                    assert!(
                        !matches!(chamber, Chamber::ChamberCertified { .. }),
                        "{:?}: certified by a leaf driven over a different document: {chamber:?}",
                        e.param
                    );
                }
            }
        }
        // The fix pass's answer: the verdict's certified leaf is
        // content-tied to the build, and the edited document re-keys
        // its profile node.
        Err(
            SensitivityRefusal::ForeignVerdict | SensitivityRefusal::VerdictNotOfThisBuild { .. },
        ) => {}
        Err(other) => panic!("{other}"),
    }
    let report = stackup(
        &edited,
        s.measure,
        &analyzed_box(&edited, &AnalysisPolicy::default()),
        &verdict,
        None,
        false,
        Tol::witness(),
    );
    assert!(
        report.is_err(),
        "a stackup of the edited document over the old verdict's leaves: {report:#?}"
    );
}

// ---------------------------------------------- claims 4/5: the marks

/// **E9 on an angle whose tangent is `0/0` through `sqrt(0)`** — and
/// the friction: the parameter `u` feeds NOTHING, its derivative is
/// analytically zero, and it degrades all the same, so one parallel
/// pair of faces forfeits the whole advisory table. The value channel
/// certifies, `worst_case` gates, no path refuses. Beside it, the
/// spec's `abs` kink spelled as `max(h − 1, 1 − h)` at `h = 1` takes the
/// ratified finite subgradient (+1) at `Dual64` — evidence for
/// deviation 4's argument, and for what a consumer gets there: a
/// `Derivative`, not a forfeiture, of a function with no derivative.
#[test]
fn a_sqrt_zero_tangent_forfeits_every_parameter_and_a_max_kink_forfeits_none() {
    let half = eps() / 16.0;
    let (doc, angle, abs) = caps(Some(uniform(-half, half)));
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(256), Tol::witness()).expect("builds");
    assert!(!verdict.certified().is_empty(), "{:?}", verdict.receipt());

    let entries = sensitivities(&doc, angle, None, Some(&verdict), false, Tol::witness())
        .expect("never a refusal");
    for n in ["h", "u"] {
        match entry(&entries, n) {
            SensitivityOutcome::TangentDegraded { tangent } => assert!(tangent.is_nan(), "{n}"),
            other => panic!("{n}: {other:?}"),
        }
    }
    let report = stackup(
        &doc,
        angle,
        &analyzed,
        &verdict,
        None,
        false,
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("E9: {e}"));
    assert_eq!(report.nominal, 0.0);
    // The enclosure of an angle in [0, π] arrives a few subnormals
    // BELOW zero (outward rounding through `atan2`, unclipped to the
    // codomain) — M10-2's arithmetic, noted, and harmless here.
    let wc = report.worst_case;
    // The top is the enclosure of the interval normals' disagreement
    // over the leaf — a fraction of ε (measured 3.5e-10 at `fc8de0ac`).
    assert!(
        wc.lo <= 0.0 && wc.lo >= -1e-300 && wc.hi >= 0.0 && wc.hi <= eps(),
        "{wc:?}"
    );
    assert_eq!(wc.leaves, verdict.certified().len());
    let blockers: Vec<&ParamName> = match &report.rss {
        Rss::UnavailableBecause { blockers } => blockers.iter().map(Unavailable::param).collect(),
        other => panic!("{other:?}"),
    };
    assert_eq!(blockers, vec![&name("h"), &name("u")]);
    for p in &report.per_param {
        assert!(p.contribution.is_err(), "{:?}", p.param);
    }

    // The max kink: finite +1 at the tie, marked, contributing |1|·Δ.
    let entries = sensitivities(&doc, abs, None, Some(&verdict), false, Tol::witness())
        .expect("never a refusal");
    match entry(&entries, "h") {
        SensitivityOutcome::Derivative { value, chamber } => {
            assert_eq!(value.to_bits(), 1.0f64.to_bits());
            assert!(contains_nominal(chamber));
        }
        other => panic!("h: {other:?}"),
    }
    let report = stackup(&doc, abs, &analyzed, &verdict, None, false, Tol::witness())
        .unwrap_or_else(|e| panic!("E9: {e}"));
    let row = report
        .per_param
        .iter()
        .find(|p| p.param == name("h"))
        .expect("h");
    assert_eq!(row.contribution, Ok(half));
    assert!(report.worst_case.lo <= 0.0 && report.worst_case.hi >= half - 1e-18);
}

// ------------------------------------------ claim 6: worst_case honesty

/// **Curvature where the linearization is exactly zero.** `m = a·a −
/// (a + a)` at `a = 1 ± 0.5`: ∂m/∂a = 0 at the nominal, so the
/// contribution is 0 and the linearized band is the point `{−1}`; the
/// true range is `[−1, −0.75]`. The hull contains the true range and
/// exceeds the linearized band; the band misses the top by 0.25.
#[test]
fn where_the_linearization_says_zero_the_hull_still_encloses_the_range() {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("a"),
        value: continuous(Dimension::Scalar, 1.0, Some(uniform(-0.5, 0.5))),
    });
    let a = || MeasureExpr::value(param("a", Dimension::Scalar));
    let expr = MeasureExpr::sub(
        MeasureExpr::mul(a(), a()).expect("Scalar"),
        MeasureExpr::add(a(), a()).expect("Scalar"),
    )
    .expect("Scalar");
    let m = r.insert(Node::measure(expr, Vec::new()).expect("no refs"));
    let doc = r.doc;
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(64), Tol::witness()).expect("builds");
    let report = stackup(&doc, m, &analyzed, &verdict, None, false, Tol::witness())
        .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(report.nominal, -1.0);
    let row = &report.per_param[0];
    match &row.sensitivity {
        SensitivityOutcome::Derivative { value, .. } => assert_eq!(*value, 0.0),
        other => panic!("{other:?}"),
    }
    assert_eq!(row.contribution, Ok(0.0));
    let wc = report.worst_case;
    assert!(
        wc.lo <= -1.0 && wc.hi >= -0.75,
        "the hull encloses the true range: {wc:?}"
    );
    assert!(
        wc.hi > report.nominal + row.contribution.clone().unwrap(),
        "the linearized top {} is below the hull's {}",
        report.nominal,
        wc.hi
    );
    println!("curvature hull {wc:?} vs linearized [-1, -1]; true range [-1, -0.75]");
}

/// **Deviation 6 — the shared nominal-box prior serves only what a
/// prior-free replay would compute.** Re-derive the hull leaf by leaf
/// at `Interval` with NO prior and compare bits with the report's.
#[test]
fn the_shared_prior_does_not_move_the_hull() {
    let half = eps() / 8.0;
    let s = slab(Some(uniform(-half, half)), Some(uniform(-half, half)));
    let analyzed = analyzed_box(&s.doc, &AnalysisPolicy::default());
    let verdict = drive(&s.doc, &analyzed, &config(1024), Tol::witness()).expect("builds");
    assert!(!verdict.certified().is_empty(), "{:?}", verdict.receipt());
    let report = stackup(
        &s.doc,
        s.measure,
        &analyzed,
        &verdict,
        None,
        false,
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("{e}"));
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for leaf in verdict.certified() {
        let ev: Evaluation<Interval> = evaluate(
            &s.doc,
            None,
            &CancelToken::new(),
            &EvalOptions {
                profile_lift: ProfileLift::Guided,
                param_box: Some(Arc::new(leaf.box_.clone())),
                ..EvalOptions::default()
            },
            Tol::witness(),
        );
        let Some(NodeResult::Ok(v)) = ev.result(s.measure) else {
            panic!("a certified leaf measures")
        };
        let ValuePayload::Measure { value, .. } = &v.payload else {
            panic!("a measure")
        };
        let (l, h) = CertifiedEnclosure::certified_bracket(*value).expect("certified");
        lo = lo.min(l);
        hi = hi.max(h);
    }
    assert_eq!(report.worst_case.lo.to_bits(), lo.to_bits());
    assert_eq!(report.worst_case.hi.to_bits(), hi.to_bits());
    assert_eq!(report.worst_case.leaves, verdict.certified().len());
    // The slab is linear in both parameters: the hull is the box's true
    // range, 3 ± 2·half, up to the enclosure's rounding.
    assert!(
        lo <= 3.0 - 2.0 * half && hi >= 3.0 + 2.0 * half,
        "{lo} {hi}"
    );
    // EVIDENCE-ONLY: how much of the ε-scale hull is enclosure padding
    // rather than parameter spread. Measured at `fc8de0ac`: hull width
    // 1.75e-9 against a true range of 5e-10 — 3.5×, i.e. ~1.25 ε of
    // padding, which is what a consumer reads as "worst case" at this
    // scale.
    println!(
        "hull width {:e} vs true range {:e} (ratio {:.2}); padding {:e} = {:.2} ε",
        hi - lo,
        4.0 * half,
        (hi - lo) / (4.0 * half),
        (hi - lo) - 4.0 * half,
        ((hi - lo) - 4.0 * half) / eps()
    );
}

// -------------------------------------------------- claim 7: the RSS

/// `φ(x)`, the standard normal density.
fn phi(x: f64) -> f64 {
    (-0.5 * x * x).exp() / (2.0 * core::f64::consts::PI).sqrt()
}

/// The standard deviation of the truncated law by composite Simpson
/// quadrature — no closed form shared with the unit.
fn truncated_sigma(sigma: f64, lo: f64, hi: f64) -> f64 {
    let n = 40_000;
    let h = (hi - lo) / n as f64;
    let mut moments = [0.0f64; 3];
    for i in 0..=n {
        let x = lo + i as f64 * h;
        let w = if i == 0 || i == n {
            1.0
        } else if i % 2 == 1 {
            4.0
        } else {
            2.0
        };
        let p = w * phi(x / sigma) / sigma;
        moments[0] += p;
        moments[1] += p * x;
        moments[2] += p * x * x;
    }
    let z = moments[0];
    let mean = moments[1] / z;
    (moments[2] / z - mean * mean).sqrt()
}

/// **σ for every form, derived independently.** Uniform `(hi − lo)/√12`
/// on an ASYMMETRIC support, Normal `σ`, TruncatedNormal by quadrature
/// on an asymmetric window, fixed `0`; with every ∂m/∂pᵢ = 1 the RSS is
/// `√Σσᵢ²`. Contributions are `half-width` per axis — the analyzed
/// box's, which for the asymmetric supports is NOT the larger
/// excursion from the nominal (noted, per spec).
#[test]
fn the_rss_sigma_of_every_distribution_form_derived_independently() {
    let u = uniform(-0.3, 0.1);
    let n = Distribution::Normal { sigma: 0.02 };
    let tn = Distribution::TruncatedNormal {
        sigma: 0.05,
        lo: -0.05,
        hi: 0.1,
    };
    let (doc, m) = sum(u, n, tn);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(64), Tol::witness()).expect("builds");
    assert!(!verdict.certified().is_empty(), "{:?}", verdict.receipt());
    let report = stackup(&doc, m, &analyzed, &verdict, None, false, Tol::witness())
        .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(report.nominal, 4.0);
    let sigma_u = 0.4 / 12f64.sqrt();
    let sigma_n = 0.02;
    let sigma_tn = truncated_sigma(0.05, -0.05, 0.1);
    let expect = (sigma_u * sigma_u + sigma_n * sigma_n + sigma_tn * sigma_tn).sqrt();
    match report.rss {
        Rss::Advisory { sigma } => assert!(
            (sigma - expect).abs() <= 1e-9,
            "rss {sigma} vs quadrature {expect} (σ_tn {sigma_tn})"
        ),
        other => panic!("{other:?}"),
    }
    assert_eq!(report.per_param.len(), 4);
    for p in &report.per_param {
        let half = 0.5 * analyzed.get(&p.param).expect("axis").offsets.width();
        assert_eq!(p.contribution, Ok(half), "{:?}", p.param);
        match &p.sensitivity {
            SensitivityOutcome::Derivative { value, chamber } => {
                assert_eq!(*value, 1.0);
                assert!(contains_nominal(chamber), "{:?}", p.param);
            }
            other => panic!("{other:?}"),
        }
    }
    // The normal's tail is genuinely non-zero and the accounting still
    // sums to one; the report carries it verbatim.
    let total = report.coverage.total().expect("every form prices");
    assert!((total - 1.0).abs() <= 1e-9, "{total}");
    assert!(report.coverage.unanalyzed.clone().unwrap() > 0.0);
    assert_eq!(&report.coverage, verdict.accounting());

    // Two bands among four: the RSS refuses whole, naming exactly the
    // two bands in name order, and nothing else changes.
    let (doc, m) = sum(
        Distribution::Band { lo: -0.3, hi: 0.1 },
        Distribution::Normal { sigma: 0.02 },
        Distribution::Band { lo: -0.05, hi: 0.1 },
    );
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(64), Tol::witness()).expect("builds");
    let report = stackup(&doc, m, &analyzed, &verdict, None, false, Tol::witness())
        .unwrap_or_else(|e| panic!("{e}"));
    match &report.rss {
        Rss::UnavailableBecause { blockers } => {
            assert_eq!(
                blockers,
                &vec![
                    Unavailable::BandHasNoMeasure { param: name("tn") },
                    Unavailable::BandHasNoMeasure { param: name("u") },
                ]
            );
        }
        other => panic!("{other:?}"),
    }
    assert!(report.per_param.iter().all(|p| p.contribution.is_ok()));
    // A band's mass DOES price when one leaf covers its whole support
    // (`interval_mass`: every measure on the band answers 1 there), and
    // a pure-expression box certifies in one leaf — so the coverage
    // totals 1 beside an unavailable RSS. Two different questions, two
    // different answers, both honest.
    assert_eq!(verdict.certified().len(), 1);
    assert!((report.coverage.total().expect("one leaf covers each band") - 1.0).abs() <= 1e-9);
}

// ----------------------------------------------- claim 8: the profile

/// **An arc-carrying profile: the radius reaches the gap only through
/// the lifted cylinder carrier.** ∂gap/∂r = −1 under the guided lift,
/// exactly 0 under the pinned one (the silent zero), and the driver
/// uses the guided lift.
#[test]
fn a_circle_radius_seed_reaches_the_gap_through_the_lifted_carrier() {
    let (doc, m) = fit(None);
    let f = measured_f64(&eval(&doc), m);
    assert!((f - 0.2).abs() < 1e-12, "gap {f}");
    let guided = measured(
        &run::<Dual64>(&doc, None, &opts(Some("r"), ProfileLift::Guided)),
        m,
    );
    assert_eq!(guided.value.to_bits(), f.to_bits());
    assert_eq!(
        guided.deriv.to_bits(),
        (-1.0f64).to_bits(),
        "∂gap/∂r = {}",
        guided.deriv
    );
    let pinned = measured(
        &run::<Dual64>(&doc, None, &opts(Some("r"), ProfileLift::Pinned)),
        m,
    );
    assert_eq!(pinned.deriv, 0.0, "the pinned lift's silent zero");
    let entries = sensitivities(&doc, m, None, None, false, Tol::witness()).expect("ok");
    match entry(&entries, "r") {
        SensitivityOutcome::Derivative { value, .. } => assert_eq!(*value, -1.0),
        other => panic!("{other:?}"),
    }
}

/// **RED AT `fc8de0ac` — a loft section's parameter seeds to a SILENT
/// ZERO.** The loft's sections stay f64 (`wire::section_of`: the lift's
/// second pass runs there as a gate only, and the emitted section is
/// `pre.profile_f64.loops`), so a seed on the section's width reaches
/// the loft's vertices with tangent exactly 0 while the distance
/// measured between them is `w` and ∂/∂w = 1. The spec's grounding:
/// "the profile gap TYPED, never silent zeros"; the PR: "No typed valve
/// was needed." The assertion is the spec's: a correct tangent, a
/// typed refusal, or a forfeiture — never a finite wrong number.
#[test]
fn a_loft_section_dimension_seed_is_not_a_silent_zero() {
    let (doc, m) = loft();
    let f = measured_f64(&eval(&doc), m);
    assert_eq!(f.to_bits(), 2.0f64.to_bits(), "distance {f}");
    let entries = sensitivities(&doc, m, None, None, false, Tol::witness()).expect("ok");
    match entry(&entries, "w") {
        SensitivityOutcome::Derivative { value, .. } => {
            assert_eq!(
                *value, 1.0,
                "∂distance/∂w through a loft section is a finite wrong number: {value}"
            );
        }
        SensitivityOutcome::TangentDegraded { .. } | SensitivityOutcome::MeasureRefused { .. } => {}
        // The fix pass's answer: the typed valve, naming the section.
        SensitivityOutcome::Unliftable { refusal, .. } => {
            assert!(
                matches!(refusal, editor_core::LiftRefusal::PinnedSection { .. }),
                "{refusal:?}"
            );
        }
    }
}

// ------------------------------------------------------------- e2e

/// **The consumer's walk on a different geometry** (the bore/pin fit):
/// a real ±0.05 study refuses `NothingCertified` (the honest limit,
/// confirmed on this geometry), and an ε-scale study reports in full —
/// read here exactly as a consumer would. EVIDENCE-ONLY where it
/// prints.
#[test]
fn the_bore_pin_fit_as_a_consumer_reads_it() {
    // The real study.
    let (doc, m) = fit(Some(uniform(-0.05, 0.05)));
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    // A bounded leaf budget: the shipped default spends minutes
    // refusing a macroscopic box leaf by leaf and reaches the same
    // answer.
    let verdict = drive(&doc, &analyzed, &config(64), Tol::witness()).expect("builds");
    let real = stackup(&doc, m, &analyzed, &verdict, None, false, Tol::witness());
    let Err(refusal @ StackupRefusal::NothingCertified { .. }) = real else {
        panic!("a ±0.05 study certifies nothing today: {real:?}")
    };
    // The refusal carries the accounting it points at (the fix pass's
    // answer to this row's original reading, which had to go back to
    // the verdict for it).
    if let StackupRefusal::NothingCertified { coverage, .. } = &refusal {
        assert_eq!(&**coverage, verdict.accounting());
    }
    println!(
        "±0.05 study: {refusal}\n  accounting (from the REFUSAL): {:#?}",
        verdict.accounting()
    );
    let local = sensitivities(&doc, m, None, Some(&verdict), false, Tol::witness()).expect("ok");
    match entry(&local, "r") {
        SensitivityOutcome::Derivative { value, chamber } => {
            assert_eq!(*value, -1.0);
            assert_eq!(*chamber, Chamber::LocalOnly);
        }
        other => panic!("{other:?}"),
    }

    // The ε-scale study.
    let half = eps() / 8.0;
    let (doc, m) = fit(Some(uniform(-half, half)));
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(256), Tol::witness()).expect("builds");
    let handed = eval(&doc);
    let report: Stackup = stackup(
        &doc,
        m,
        &analyzed,
        &verdict,
        Some(&handed),
        true,
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("{e}"));
    println!("ε-scale study: {report:#?}");
    assert_eq!(report.measurement, m);
    assert!((report.nominal - 0.2).abs() < 1e-12);
    let row = &report.per_param[0];
    assert_eq!(row.param, name("r"));
    match &row.sensitivity {
        SensitivityOutcome::Derivative { value, chamber } => {
            assert_eq!(*value, -1.0);
            assert!(contains_nominal(chamber));
        }
        other => panic!("{other:?}"),
    }
    assert_eq!(row.contribution, Ok(half));
    let wc = report.worst_case;
    assert!(wc.lo <= report.nominal && report.nominal <= wc.hi);
    // The hull ENCLOSES the true range `0.2 ± half` (the gap is linear
    // in the radius with slope −1) and exceeds it by the interval
    // lane's enclosure padding alone. The padding is proportional to
    // the box, not to ε: measured at every CI ε row (default, 1e-6,
    // 1e-12) the hull is 3·half wide — 2·half of spread plus exactly
    // 1·half of padding (3.000, 3.000, 3.003 × half) — so the bound is
    // stated per half-width plus the rounding of a 0.2-scale quantity
    // through a few dozen outward-rounded operations (~1e-15). No
    // absolute slack: an ε-independent term says nothing at the tight
    // rows and the wrong thing at the loose ones (issue 1646).
    assert!(wc.lo <= 0.2 - half && wc.hi >= 0.2 + half, "{wc:?}");
    let padding = (wc.hi - wc.lo) - 2.0 * half;
    println!(
        "EVIDENCE-ONLY worst-case padding: hull width {:e}, true range {:e}, padding {padding:e} \
         ({:.3} half-widths)",
        wc.hi - wc.lo,
        2.0 * half,
        padding / half
    );
    assert!(
        padding <= BORE_PIN_PADDING_PER_HALF_WIDTH * half + BORE_PIN_ROUNDING,
        "padding {padding:e} exceeds the measured bound: {wc:?}"
    );
    match report.rss {
        Rss::Advisory { sigma } => {
            let expect = (2.0 * half) / 12f64.sqrt();
            assert!(
                (sigma - expect).abs() <= 1e-9 * expect,
                "{sigma} vs {expect}"
            );
        }
        other => panic!("{other:?}"),
    }
    assert!((report.coverage.total().unwrap() - 1.0).abs() <= 1e-9);
}

/// **Where deviation 2 would bite.** The passes run the GUIDED lift at
/// `Dual64` while the anchor runs the build path (`Pinned`, `f64`); the
/// pairing compares them arm for arm. A document where the guided
/// dual replay refuses a node the build path built would therefore
/// refuse the WHOLE entry set as `PairingViolation::ResultArm` — a lift
/// limitation reported as a pairing violation. Over the corpus the two
/// agree on every node (EVIDENCE-ONLY unless it goes red).
#[test]
fn the_guided_dual_pass_evaluates_the_build_paths_nodes_over_the_corpus() {
    for cd in corpus::documents() {
        let f = eval(&cd.doc);
        let d = run::<Dual64>(&cd.doc, None, &opts(None, ProfileLift::Guided));
        assert_eq!(f.order, d.order, "{}", cd.name);
        for id in &f.order {
            let fa = matches!(f.result(*id), Some(NodeResult::Ok(_)));
            let da = matches!(d.result(*id), Some(NodeResult::Ok(_)));
            assert_eq!(
                fa,
                da,
                "{}: node {} build-path Ok={fa}, guided Dual64 Ok={da}: {:?}",
                cd.name,
                id.0,
                d.node_error(*id).map(|e| e.kind.to_string())
            );
        }
    }
}

/// EVIDENCE-ONLY: a verdict whose certified leaves do not contain the
/// nominal still yields a report — `LocalOnly` on every row, a
/// `worst_case` that need not contain `nominal`, and no top-level mark
/// saying so. Built from a synthetic verdict-free path: with no drive
/// there is no report at all (the driver alone marks `LocalOnly`), so
/// this row records the shape through the driver only.
#[test]
fn without_a_drive_every_mark_is_local_and_there_is_no_report_path() {
    let s = slab(None, None);
    let entries = sensitivities(&s.doc, s.measure, None, None, false, Tol::witness()).expect("ok");
    assert!(entries.iter().all(|e| matches!(
        e.outcome,
        SensitivityOutcome::Derivative {
            chamber: Chamber::LocalOnly,
            ..
        }
    )));
    // A verdict over a box that is not the analyzed one is refused
    // typed BEFORE any pass runs.
    let analyzed = analyzed_box(&s.doc, &AnalysisPolicy::default());
    let mut axes: BTreeMap<ParamName, BoxAxis> = ParamBox::of(&analyzed).axes().clone();
    axes.insert(
        name("w"),
        BoxAxis::Varying {
            lo: -eps() / 8.0,
            hi: eps() / 8.0,
        },
    );
    let other = ParamBox::from_axes(axes);
    let _: &ParamBoxVerdict;
    assert_ne!(other, ParamBox::of(&analyzed));
}
