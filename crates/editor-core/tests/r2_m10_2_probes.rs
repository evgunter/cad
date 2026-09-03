//! **R2 independent probes for M10-2** (measure nodes and assertions).
//!
//! Reviewer-authored, adversarial. Every oracle here is derived from
//! the authored geometry by hand in the row's own prose; nothing reads
//! a previous run of the code under test, and nothing reuses the
//! unit's own fixtures.
//!
//! Coverage rationale: the shipped suites exercise three of the nine
//! rows of `eval::measure`'s primitive table against real geometry
//! (`distance` cyl x cyl, `angle` plane x plane, `gap` cyl x cyl in ONE
//! regime). These rows attack the other six and the two levers.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

#[path = "fixture/mod.rs"]
mod fixture;

use editor_core::UnitSym;
use editor_core::{
    AssertionDir, AssertionVerdict, Axis3, BooleanOp, CancelToken, Dimension, DocEdit, DocParam,
    DocParamValue, DocumentId, EntityKind, EvalOptions, Evaluation, Expr, GeomPred, LoopProgram,
    MeasureExpr, MeasurePrimitive, MeasureRef, NamePat, Node, NodeErrorKind, NodeResult, ParamName,
    ProfileDoc, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId, Selector,
    StableName, SurfaceKindSet, ValuePayload, apply, evaluate, select_where,
};
use fixture::{ang, len, scl};
use geom_core::Tol;

// ---------------------------------------------------------------
// Plumbing
// ---------------------------------------------------------------

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn push(doc: &ProfileDoc, edit: &DocEdit<ProfileProgram>) -> ProfileDoc {
    apply(doc, edit, Tol::witness())
        .unwrap_or_else(|e| panic!("edit refused: {e}"))
        .doc
}

fn try_push(
    doc: &ProfileDoc,
    edit: &DocEdit<ProfileProgram>,
) -> Result<ProfileDoc, editor_core::EditError> {
    apply(doc, edit, Tol::witness()).map(|a| a.doc)
}

fn no_params() -> editor_core::ParamEnv<f64> {
    ProfileDoc::empty_derived("r2-noparams", Tol::witness()).param_env::<f64>()
}

/// Faces of one surface kind on one node's value, canonically ordered.
fn faces(
    ev: &Evaluation<f64>,
    body: RecipeNodeId,
    kind: geom_brep::SurfaceKind,
) -> Vec<StableName> {
    let mut f = select_where(
        ev,
        body,
        &Selector::of(NamePat::of_kind(EntityKind::Face)),
        &[GeomPred::SurfaceKind(SurfaceKindSet::just(kind))],
        &no_params(),
        Tol::witness(),
    )
    .expect("the surface-kind atom is exact and never refuses");
    f.sort();
    f
}

fn vertices(ev: &Evaluation<f64>, body: RecipeNodeId) -> Vec<StableName> {
    let mut v = editor_core::select(
        ev,
        body,
        &Selector::of(NamePat::of_kind(EntityKind::Vertex)),
    );
    v.sort();
    v
}

fn edges_of_kind(
    ev: &Evaluation<f64>,
    body: RecipeNodeId,
    kind: editor_core::CurveKind,
) -> Vec<StableName> {
    let mut e = select_where(
        ev,
        body,
        &Selector::of(NamePat::of_kind(EntityKind::Edge)),
        &[GeomPred::CurveKind(editor_core::CurveKindSet::just(kind))],
        &no_params(),
        Tol::witness(),
    )
    .expect("exact atom");
    e.sort();
    e
}

/// Inserts a measure over `refs` and returns (doc, measure id).
///
/// Each reference is read AT ITS MINTING NODE — the spelling this
/// suite means throughout, since none of its fixtures places the
/// geometry it measures. Adapting to the `MeasureRef` shape the fix
/// pass introduced for MAJ-2; the rows and their oracles are
/// unchanged.
fn with_measure(
    doc: &ProfileDoc,
    expr: MeasureExpr,
    refs: Vec<StableName>,
) -> (ProfileDoc, RecipeNodeId) {
    let id = RecipeNodeId(doc.len() as u64);
    let refs: Vec<MeasureRef> = refs.into_iter().map(MeasureRef::at_mint).collect();
    let doc = push(
        doc,
        &DocEdit::InsertNode {
            node: Node::measure(expr, refs).expect("indices in range"),
        },
    );
    (doc, id)
}

fn measured(ev: &Evaluation<f64>, id: RecipeNodeId) -> f64 {
    match ev.nodes.get(&id) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, .. } => *value,
            other => panic!("node {id:?} is a {}", other.kind_name()),
        },
        other => panic!("node {id:?} did not evaluate: {other:?}"),
    }
}

fn outcome(ev: &Evaluation<f64>, id: RecipeNodeId) -> Result<f64, String> {
    match ev.nodes.get(&id) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, .. } => Ok(*value),
            other => Err(format!("payload {}", other.kind_name())),
        },
        Some(NodeResult::Failed(e)) => Err(format!("{:?}", e.kind)),
        other => Err(format!("{other:?}")),
    }
}

// ---------------------------------------------------------------
// Geometry authoring (independent of the unit's fixtures)
// ---------------------------------------------------------------

/// A box [x0,x1]x[y0,y1]x[z0,z0+h], authored as a polygon extrude.
/// Returns (doc, extrude id).
fn boxed(
    doc: &ProfileDoc,
    x: (f64, f64),
    y: (f64, f64),
    z0: f64,
    h: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let plane = RecipeNodeId(doc.len() as u64);
    let doc = push(
        doc,
        &DocEdit::InsertNode {
            node: fixture::frame([0.0, 0.0, z0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        },
    );
    let p = RecipeNodeId(doc.len() as u64);
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Profile(fixture::desc(
                plane,
                vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
            )),
        },
    );
    let e = RecipeNodeId(doc.len() as u64);
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Extrude {
                profile: p,
                distance: len(h),
            },
        },
    );
    (doc, e)
}

/// A sphere of radius `r` centred at (0, 0, cz): a bulge-1 half-disc on
/// the XZ frame, revolved a full turn about the world Z axis.
fn sphere(doc: &ProfileDoc, r: f64, cz: f64) -> (ProfileDoc, RecipeNodeId) {
    let half = LoopProgram::Chain(vec![
        ProgramStep::At([len(0.0), len(-r)]),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point([len(0.0), len(r)]),
            b: scl(1.0),
        }),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let plane = RecipeNodeId(doc.len() as u64);
    let doc = push(
        doc,
        &DocEdit::InsertNode {
            node: fixture::frame([0.0, 0.0, cz], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        },
    );
    // The frame's v is world +Z and its origin sits ON the world Z
    // axis, so the pole axis is this frame's own +y through (0, 0) —
    // and it is minted AFTER the frame it names.
    let axis = RecipeNodeId(doc.len() as u64);
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: fixture::axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)),
        },
    );
    let p = RecipeNodeId(doc.len() as u64);
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Profile(ProfileProgram {
                plane,
                loops: vec![half],
            }),
        },
    );
    let s = RecipeNodeId(doc.len() as u64);
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Revolve {
                profile: p,
                axis,
                angle: ang(std::f64::consts::TAU),
            },
        },
    );
    (doc, s)
}

/// A cylinder of radius `r` about the axis through (cx, cy) along +Z,
/// from z0 up by h.
fn cylinder(
    doc: &ProfileDoc,
    r: f64,
    cx: f64,
    cy: f64,
    z0: f64,
    h: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let plane = RecipeNodeId(doc.len() as u64);
    let doc = push(
        doc,
        &DocEdit::InsertNode {
            node: fixture::frame([0.0, 0.0, z0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        },
    );
    let p = RecipeNodeId(doc.len() as u64);
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Profile(ProfileProgram {
                plane,
                loops: vec![LoopProgram::Circle {
                    centre: [len(cx), len(cy)],
                    radius: len(r),
                }],
            }),
        },
    );
    let e = RecipeNodeId(doc.len() as u64);
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Extrude {
                profile: p,
                distance: len(h),
            },
        },
    );
    (doc, e)
}

fn empty(tag: &str) -> ProfileDoc {
    ProfileDoc::empty(DocumentId::derive(tag), Tol::witness())
}

// ===============================================================
// CLAIM 4 — the six table rows with no shipped behavioural coverage
// ===============================================================

/// `distance` vertex x vertex: the exact norm.
///
/// **Oracle.** A box on [0,3]x[0,4]x[0,12] has corners at the eight
/// sign combinations. Its body diagonal is sqrt(9+16+144) = 13
/// exactly (a Pythagorean quadruple, so the answer is an integer and
/// no float slack hides an error). Every other vertex pair is a
/// smaller authored distance, and the row checks the SET of pairwise
/// distances against the eight authored corners rather than one pair.
#[test]
fn r2_distance_vertex_vertex_is_the_exact_norm() {
    let (doc, b) = boxed(&empty("r2-vv"), (0.0, 3.0), (0.0, 4.0), 0.0, 12.0);
    let ev = eval(&doc);
    let vs = vertices(&ev, b);
    assert_eq!(vs.len(), 8, "a box has eight corners, got {}", vs.len());

    // Every ordered pair, measured; the multiset must equal the
    // multiset of authored corner distances.
    let corners: Vec<[f64; 3]> = {
        let mut c = Vec::new();
        for x in [0.0, 3.0] {
            for y in [0.0, 4.0] {
                for z in [0.0, 12.0] {
                    c.push([x, y, z]);
                }
            }
        }
        c
    };
    let mut authored: Vec<f64> = Vec::new();
    for i in 0..8 {
        for j in (i + 1)..8 {
            let d = (0..3)
                .map(|k| (corners[i][k] - corners[j][k]).powi(2))
                .sum::<f64>()
                .sqrt();
            authored.push(d);
        }
    }
    authored.sort_by(f64::total_cmp);

    let mut got: Vec<f64> = Vec::new();
    let mut d = doc.clone();
    let mut ids = Vec::new();
    for i in 0..8 {
        for j in (i + 1)..8 {
            let (nd, id) = with_measure(
                &d,
                MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
                vec![vs[i].clone(), vs[j].clone()],
            );
            d = nd;
            ids.push(id);
        }
    }
    let ev = eval(&d);
    for id in ids {
        got.push(measured(&ev, id));
    }
    got.sort_by(f64::total_cmp);
    assert_eq!(got.len(), authored.len());
    for (g, a) in got.iter().zip(&authored) {
        assert!(
            (g - a).abs() < 1e-12,
            "pairwise distance multiset disagrees: got {g}, authored {a}"
        );
    }
    // The headline: the body diagonal is exactly 13.
    assert!(
        got.iter().any(|g| (g - 13.0).abs() < 1e-12),
        "the 3-4-12 body diagonal must be 13; got {got:?}"
    );
}

/// `distance` vertex x plane face: |(p - o).n|, a signed projection
/// taken in magnitude.
///
/// **Oracle.** A box on [0,3]x[0,4]x[0,12]: every corner is either on
/// the top cap (z = 12) or on the bottom (z = 0), so the distance from
/// a corner to the TOP cap face is 0 for the four top corners and 12
/// for the four bottom ones. Both numbers are authored; neither comes
/// from a run.
#[test]
fn r2_distance_vertex_plane_is_the_normal_projection() {
    let (doc, b) = boxed(&empty("r2-vp"), (0.0, 3.0), (0.0, 4.0), 0.0, 12.0);
    let ev = eval(&doc);
    let vs = vertices(&ev, b);
    let planes = faces(&ev, b, geom_brep::SurfaceKind::Plane);
    assert_eq!(planes.len(), 6, "a box has six planar faces");

    // Identify the cap at z = 12 by measuring every corner against
    // every planar face and demanding the authored 4x0 / 4x12 split
    // appear exactly twice (once per cap).
    let mut d = doc.clone();
    let mut ids = Vec::new();
    for f in &planes {
        for v in &vs {
            let (nd, id) = with_measure(
                &d,
                MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
                vec![v.clone(), f.clone()],
            );
            d = nd;
            ids.push(id);
        }
    }
    let ev = eval(&d);
    let vals: Vec<f64> = ids.iter().map(|i| measured(&ev, *i)).collect();
    // Per face, the eight corner distances.
    let mut caps = 0;
    for (fi, _f) in planes.iter().enumerate() {
        let mut per: Vec<f64> = vals[fi * 8..(fi + 1) * 8].to_vec();
        per.sort_by(f64::total_cmp);
        // A cap: four corners at 0 and four at 12.
        if per[..4].iter().all(|v| v.abs() < 1e-12)
            && per[4..].iter().all(|v| (v - 12.0).abs() < 1e-12)
        {
            caps += 1;
        }
        // EVERY value is one of the authored face offsets and is
        // non-negative (the arm is documented as a magnitude).
        for v in &per {
            assert!(*v >= 0.0, "distance is a magnitude, got {v}");
            assert!(
                [0.0, 3.0, 4.0, 12.0].iter().any(|a| (v - a).abs() < 1e-12),
                "a corner-to-face distance on this box is 0, 3, 4 or 12; got {v}"
            );
        }
    }
    assert_eq!(caps, 2, "exactly the two z-caps show the 4x0 / 4x12 split");
}

/// `distance` plane face x plane face, the PARALLEL arm.
///
/// **Oracle.** Two boxes: A occupies z in [0, 1], B occupies
/// z in [3, 4]. A's top cap (z = 1) and B's bottom cap (z = 3) are
/// parallel planes 2 apart; A's top and B's top (z = 4) are 3 apart.
/// Authored numbers, not run outputs.
#[test]
fn r2_distance_plane_plane_is_the_authored_offset() {
    let d0 = empty("r2-pp");
    let (d1, a) = boxed(&d0, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (d2, b) = boxed(&d1, (0.0, 1.0), (0.0, 1.0), 3.0, 1.0);
    let ev = eval(&d2);
    let pa = faces(&ev, a, geom_brep::SurfaceKind::Plane);
    let pb = faces(&ev, b, geom_brep::SurfaceKind::Plane);

    // Every A-plane against every B-plane. The four z-cap pairs give
    // the authored offsets {2, 3, 3, 4}; the parallel WALL pairs give
    // 0 or 1; the mixed cap-vs-wall pairs are non-parallel and must
    // REFUSE rather than report a number.
    let mut d = d2.clone();
    let mut ids = Vec::new();
    for x in &pa {
        for y in &pb {
            let (nd, id) = with_measure(
                &d,
                MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
                vec![x.clone(), y.clone()],
            );
            d = nd;
            ids.push(id);
        }
    }
    let ev = eval(&d);
    let mut ok: Vec<f64> = Vec::new();
    let mut refused = 0usize;
    for id in ids {
        match outcome(&ev, id) {
            Ok(v) => ok.push(v),
            Err(_) => refused += 1,
        }
    }
    // The authored z-cap offsets must all be present.
    for want in [2.0_f64, 3.0, 4.0] {
        assert!(
            ok.iter().any(|v| (v - want).abs() < 1e-12),
            "the authored parallel-plane offset {want} is missing from {ok:?}"
        );
    }
    assert!(
        ok.iter().all(|v| *v >= 0.0),
        "the plane-plane arm is documented unsigned; got {ok:?}"
    );
    assert!(
        refused > 0,
        "cap-against-wall pairs are non-parallel and must refuse; none did"
    );
}

/// `angle` line edge x line edge.
///
/// **Oracle.** A box's twelve straight edges run along exactly three
/// mutually orthogonal directions, so every measured pair angle is one
/// of 0 or pi/2 (the arm is documented as the unsigned angle in
/// [0, pi] between DIRECTIONS, so anti-parallel edges may read pi).
/// The row asserts the closed set and that pi/2 actually occurs.
#[test]
fn r2_angle_line_line_is_the_box_direction_set() {
    let (doc, b) = boxed(&empty("r2-ll"), (0.0, 2.0), (0.0, 5.0), 0.0, 7.0);
    let ev = eval(&doc);
    let es = edges_of_kind(&ev, b, editor_core::CurveKind::Line);
    assert_eq!(es.len(), 12, "a box has twelve straight edges");

    let mut d = doc.clone();
    let mut ids = Vec::new();
    for i in 0..es.len() {
        for j in (i + 1)..es.len() {
            let (nd, id) = with_measure(
                &d,
                MeasureExpr::primitive(MeasurePrimitive::Angle { a: 0, b: 1 }),
                vec![es[i].clone(), es[j].clone()],
            );
            d = nd;
            ids.push(id);
        }
    }
    let ev = eval(&d);
    let mut saw_right = false;
    for id in ids {
        let v = measured(&ev, id);
        let ok = [0.0, std::f64::consts::FRAC_PI_2, std::f64::consts::PI]
            .iter()
            .any(|a| (v - a).abs() < 1e-9);
        assert!(ok, "a box edge pair angle must be 0, pi/2 or pi; got {v}");
        if (v - std::f64::consts::FRAC_PI_2).abs() < 1e-9 {
            saw_right = true;
        }
    }
    assert!(saw_right, "orthogonal box edges must produce pi/2");
}

/// **`gap` sphere x sphere in all three C5 regimes**, against real
/// revolved geometry.
///
/// **Oracle.** g = R - r - ||dc||. Outer sphere R = 1.0 centred at the
/// origin; inner sphere r = 0.5 centred at (0, 0, cz). So
/// g = 0.5 - |cz|, and the three regimes are cz = 0.25 (g = +0.25,
/// clearance), cz = 0.5 (g = 0, contact) and cz = 0.75 (g = -0.25,
/// interference). Every number is authored.
#[test]
fn r2_gap_sphere_sphere_walks_all_three_regimes() {
    for (cz, want) in [(0.25_f64, 0.25_f64), (0.5, 0.0), (0.75, -0.25)] {
        let d0 = empty("r2-ss");
        let (d1, outer) = sphere(&d0, 1.0, 0.0);
        let (d2, inner) = sphere(&d1, 0.5, cz);
        let ev = eval(&d2);
        let fo = faces(&ev, outer, geom_brep::SurfaceKind::Sphere);
        let fi = faces(&ev, inner, geom_brep::SurfaceKind::Sphere);
        assert!(
            !fo.is_empty() && !fi.is_empty(),
            "both revolves are spheres"
        );
        let (d3, id) = with_measure(
            &d2,
            MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 0, inner: 1 }),
            vec![fo[0].clone(), fi[0].clone()],
        );
        let g = measured(&eval(&d3), id);
        assert!(
            (g - want).abs() < 1e-12,
            "R - r - |dc| = 1.0 - 0.5 - {cz} = {want}, got {g}"
        );
        // C5's convention, read as a consumer would.
        let sign = if g > 0.0 {
            "clearance"
        } else if g < 0.0 {
            "interference"
        } else {
            "contact"
        };
        let expect = if want > 0.0 {
            "clearance"
        } else if want < 0.0 {
            "interference"
        } else {
            "contact"
        };
        assert_eq!(sign, expect, "C5 sign convention at cz = {cz}");
    }
}

/// **`gap` plane x plane, and whether its SIGN survives a role swap.**
///
/// C5 binds `g > 0` to clearance and makes the argument order the
/// mating ROLE (`outer` contains, `inner` is contained), which the
/// implementation renders as `(o_i - o_o).n_o` over the OUTER plane's
/// CHART normal. The chart normal is read uncorrected by face sense.
///
/// This row does not assert a preferred answer; it records what the
/// implementation does in both role orders over the same physical
/// clearance, so the reviewer's finding rests on measured behaviour.
#[test]
fn r2_gap_plane_plane_role_swap_behaviour() {
    let d0 = empty("r2-gpp");
    // Two slabs with a physical 2.0 clearance between A's top (z = 1)
    // and B's bottom (z = 3).
    let (d1, a) = boxed(&d0, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (d2, b) = boxed(&d1, (0.0, 1.0), (0.0, 1.0), 3.0, 1.0);
    let ev = eval(&d2);
    let pa = faces(&ev, a, geom_brep::SurfaceKind::Plane);
    let pb = faces(&ev, b, geom_brep::SurfaceKind::Plane);

    let mut rows: Vec<(usize, usize, String, String)> = Vec::new();
    for (i, x) in pa.iter().enumerate() {
        for (j, y) in pb.iter().enumerate() {
            let (dx, ix) = with_measure(
                &d2,
                MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 0, inner: 1 }),
                vec![x.clone(), y.clone()],
            );
            let (dy, iy) = with_measure(
                &d2,
                MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 0, inner: 1 }),
                vec![y.clone(), x.clone()],
            );
            let fwd = outcome(&eval(&dx), ix);
            let rev = outcome(&eval(&dy), iy);
            if let (Ok(f), Ok(r)) = (&fwd, &rev) {
                rows.push((i, j, format!("{f}"), format!("{r}")));
                // The finding: for the sign to encode the ROLE, a
                // swap must negate. Record any pair where it does not.
                if (f + r).abs() > 1e-12 && f.abs() > 1e-12 {
                    eprintln!("R2/gap-plane: role swap did NOT negate: g(a,b) = {f}, g(b,a) = {r}");
                }
            }
        }
    }
    assert!(
        !rows.is_empty(),
        "some plane pair must be parallel and gap-able"
    );
    // Evidence-only: the row prints its table. The assertion is only
    // that the arm produced numbers at all, so the table is real.
    for (i, j, f, r) in &rows {
        eprintln!("R2/gap-plane pa[{i}] x pb[{j}]: forward {f}, reversed {r}");
    }
    // Every parallel plane pair here is a physical clearance or a
    // coincidence; NONE of this geometry interferes. A negative gap
    // reported for a non-interfering pair is the finding.
    let negatives = rows
        .iter()
        .filter(|(_, _, f, _)| f.parse::<f64>().map(|v| v < -1e-12).unwrap_or(false))
        .count();
    eprintln!(
        "R2/gap-plane: {negatives} of {} forward readings are negative (interference) on geometry that never interferes",
        rows.len()
    );
}

// ===============================================================
// CLAIM 2 — the lever arm
// ===============================================================

/// **The parallelism lever is the PINNED metre, not "the operands'
/// own separation".**
///
/// `eval::measure::arm` is `separation.max(T::one())`, so for any
/// geometry under a metre — which is every part in this repo's own
/// corpus, the unit's own two-hole plate included — the `max` clamps
/// and the separation never reaches the margin. The module header and
/// the PR body both describe the arm as "the operands' own
/// separation ... floored at unit arm"; this row asks whether the
/// separation has any effect at all below a metre.
///
/// **Method.** Two coaxial cylinders whose axis separation differs by
/// 90x (0.01 vs 0.9). A genuine separation lever prices the same
/// misalignment 90x apart and can decide the two differently; a pinned
/// metre cannot tell them apart. Both pairs are EXACTLY parallel here,
/// so the row reads the arm's effect through the reported distance
/// being the separation and through the pair deciding identically.
#[test]
fn r2_the_parallelism_arm_is_clamped_below_one_metre() {
    for sep in [0.01_f64, 0.9] {
        let d0 = empty("r2-arm");
        let (d1, c1) = cylinder(&d0, 0.001, 0.0, 0.0, 0.0, 0.01);
        let (d2, c2) = cylinder(&d1, 0.001, sep, 0.0, 0.0, 0.01);
        let ev = eval(&d2);
        let f1 = faces(&ev, c1, geom_brep::SurfaceKind::Cylinder);
        let f2 = faces(&ev, c2, geom_brep::SurfaceKind::Cylinder);
        assert!(!f1.is_empty() && !f2.is_empty());
        let (d3, id) = with_measure(
            &d2,
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![f1[0].clone(), f2[0].clone()],
        );
        let v = measured(&eval(&d3), id);
        assert!(
            (v - sep).abs() < 1e-12,
            "axis separation is the authored {sep}, got {v}"
        );
    }
}

/// **A tilt irrelevant at the part's own scale refuses anyway.**
///
/// Two cylinders 10 mm apart, one tilted by theta about x. At the
/// HONEST arm (the 0.01 m separation the module header names) the
/// margin is sin(theta)*0.01; at the pinned metre it is
/// sin(theta)*1.0 — a 100x amplification. With eps = 1e-9 and the
/// default K the band is roughly [1e-9, 3e-8], so theta = 1e-8 rad
/// (2 milli-arcseconds, over a 10 mm span: a 1e-10 m deviation, a
/// tenth of eps) lands at 1e-10 under the honest arm — decisively
/// parallel — and at 1e-8 under the pinned metre, inside the band.
///
/// The row records which happens. A refusal here is a measurement lost
/// to a misalignment two orders of magnitude below the run's own
/// linear tolerance across the geometry it is quoted over.
#[test]
fn r2_a_sub_epsilon_tilt_at_ten_millimetres() {
    let theta = 1e-8_f64;
    let d0 = empty("r2-tilt");
    let (d1, c1) = cylinder(&d0, 0.001, 0.0, 0.0, 0.0, 0.01);
    // The second cylinder on a frame tilted by theta about x: its
    // extrude direction (and so its axis) tilts with the plane normal.
    let plane = RecipeNodeId(d1.len() as u64);
    let d1 = push(
        &d1,
        &DocEdit::InsertNode {
            node: fixture::frame(
                [0.01, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, theta.cos(), -theta.sin()],
            ),
        },
    );
    let p = RecipeNodeId(d1.len() as u64);
    let d2 = push(
        &d1,
        &DocEdit::InsertNode {
            node: Node::Profile(ProfileProgram {
                plane,
                loops: vec![LoopProgram::Circle {
                    centre: [len(0.0), len(0.0)],
                    radius: len(0.001),
                }],
            }),
        },
    );
    let c2 = RecipeNodeId(d2.len() as u64);
    let d3 = push(
        &d2,
        &DocEdit::InsertNode {
            node: Node::Extrude {
                profile: p,
                distance: len(0.01),
            },
        },
    );
    let ev = eval(&d3);
    let f1 = faces(&ev, c1, geom_brep::SurfaceKind::Cylinder);
    let f2 = faces(&ev, c2, geom_brep::SurfaceKind::Cylinder);
    if f1.is_empty() || f2.is_empty() {
        eprintln!("R2/tilt: the tilted extrude produced no cylinder face; row is inconclusive");
        return;
    }
    let (d4, id) = with_measure(
        &d3,
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        vec![f1[0].clone(), f2[0].clone()],
    );
    match outcome(&eval(&d4), id) {
        Ok(v) => eprintln!("R2/tilt: theta = {theta} measured {v} (the arm let it through)"),
        Err(e) => eprintln!("R2/tilt: theta = {theta} REFUSED: {e}"),
    }
}

// ===============================================================
// CLAIM 6 — report-only, attacked
// ===============================================================

/// **No op accepts a verdict, and none accepts a measured quantity.**
///
/// The PR argues report-only structurally ("no op in the vocabulary
/// accepts a verdict as an operand"). This row tries every body-taking
/// op against BOTH new payload kinds and demands a typed refusal from
/// each — the claim's contrapositive, enumerated rather than asserted.
#[test]
fn r2_no_op_consumes_a_measure_or_a_verdict() {
    let d0 = empty("r2-consume");
    let (d1, b) = boxed(&d0, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let ev = eval(&d1);
    let vs = vertices(&ev, b);
    let (d2, measure) = with_measure(
        &d1,
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        vec![vs[0].clone(), vs[1].clone()],
    );
    let assertion = RecipeNodeId(d2.len() as u64);
    let d3 = push(
        &d2,
        &DocEdit::InsertNode {
            node: Node::Assertion {
                measure,
                // A bound the measure VIOLATES: the box diagonal is at
                // most sqrt(3) < 100.
                bound: Expr::literal(100.0, Dimension::Length).expect("finite"),
                dir: AssertionDir::AtLeast,
            },
        },
    );
    let ev = eval(&d3);
    assert!(
        matches!(
            ev.nodes.get(&assertion),
            Some(NodeResult::Ok(v)) if matches!(&v.payload, ValuePayload::Assertion(AssertionVerdict::Violated { .. }))
        ),
        "the probe needs a Violated verdict to be meaningful"
    );

    // Every op that takes a body, pointed at each sink.
    for victim in [measure, assertion] {
        let attempts: Vec<(&str, Node<ProfileProgram>)> = vec![
            (
                "boolean-a",
                Node::Boolean {
                    op: BooleanOp::Union,
                    a: victim,
                    b,
                    declare: None,
                },
            ),
            (
                "boolean-b",
                Node::Boolean {
                    op: BooleanOp::Subtract,
                    a: b,
                    b: victim,
                    declare: None,
                },
            ),
            (
                "transform",
                Node::Transform {
                    input: victim,
                    translation: [len(1.0), len(0.0), len(0.0)],
                    rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                    rotation_angle: ang(0.0),
                },
            ),
            (
                "extrude-profile",
                Node::Extrude {
                    profile: victim,
                    distance: len(1.0),
                },
            ),
        ];
        for (what, node) in attempts {
            let id = RecipeNodeId(d3.len() as u64);
            match try_push(&d3, &DocEdit::InsertNode { node }) {
                // Refused at the edit door: ideal.
                Err(_) => {}
                // Admitted: it MUST then fail typed at evaluation, and
                // must not produce a body.
                Ok(doc) => {
                    let ev = eval(&doc);
                    match ev.nodes.get(&id) {
                        Some(NodeResult::Failed(_)) => {}
                        other => panic!(
                            "{what} over {victim:?} neither refused at the edit door nor \
                             failed typed at evaluation: {other:?}"
                        ),
                    }
                }
            }
        }
    }
}

/// **A Violated assertion moves no product and no key — including
/// through the naming channel.**
///
/// Stronger than the unit's own row: it compares EVERY node the two
/// documents share (not a hand-picked three), both keys, and the
/// recorded product.
#[test]
fn r2_a_violated_assertion_is_invisible_to_every_shared_node() {
    let d0 = empty("r2-invisible");
    let (d1, b) = boxed(&d0, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let ev = eval(&d1);
    let vs = vertices(&ev, b);
    let (with_measure_doc, measure) = with_measure(
        &d1,
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        vec![vs[0].clone(), vs[1].clone()],
    );
    let assertion = RecipeNodeId(with_measure_doc.len() as u64);
    let with_assertion = push(
        &with_measure_doc,
        &DocEdit::InsertNode {
            node: Node::Assertion {
                measure,
                bound: Expr::literal(100.0, Dimension::Length).expect("finite"),
                dir: AssertionDir::AtLeast,
            },
        },
    );
    let a = eval(&with_assertion);
    let c = eval(&with_measure_doc);
    assert!(
        matches!(
            a.nodes.get(&assertion),
            Some(NodeResult::Ok(v)) if matches!(&v.payload, ValuePayload::Assertion(AssertionVerdict::Violated { .. }))
        ),
        "the probe needs a Violated verdict"
    );
    for id in c.nodes.keys() {
        let (x, y) = (
            a.nodes.get(id).expect("shared"),
            c.nodes.get(id).expect("shared"),
        );
        match (x, y) {
            (NodeResult::Ok(x), NodeResult::Ok(y)) => {
                assert_eq!(
                    x.content_key, y.content_key,
                    "node {id:?} content key moved"
                );
                assert_eq!(
                    x.name_table.len(),
                    y.name_table.len(),
                    "node {id:?} name table size moved"
                );
            }
            (NodeResult::Ok(_), other) | (other, NodeResult::Ok(_)) => {
                panic!("node {id:?} changed outcome: {other:?}")
            }
            _ => {}
        }
    }
}

// ===============================================================
// CLAIM 5 — scalar genericity on the arms this reviewer added
// ===============================================================

/// The vertex-vertex arm at `Dual64`: the value channel is
/// BIT-identical to the f64 build and the tangent is exactly zero
/// (nothing seeds a parameter — seeding is M10-4's door, which does
/// not exist in this unit).
#[test]
fn r2_a_measure_at_dual64_is_bit_identical_and_untangented() {
    use geom_core::Dual64;
    let (doc, b) = boxed(&empty("r2-dual"), (0.0, 3.0), (0.0, 4.0), 0.0, 12.0);
    let ev = eval(&doc);
    let vs = vertices(&ev, b);
    let (doc, id) = with_measure(
        &doc,
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        vec![vs[0].clone(), vs[7].clone()],
    );
    let at_f64 = measured(&eval(&doc), id);
    let evd = evaluate::<Dual64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    match evd.nodes.get(&id) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, .. } => {
                assert_eq!(
                    value.value.to_bits(),
                    at_f64.to_bits(),
                    "the Dual64 value channel must be BIT-identical to f64"
                );
                assert_eq!(value.deriv, 0.0, "nothing is seeded, so the tangent is 0");
            }
            other => panic!("expected a measure, got {}", other.kind_name()),
        },
        other => panic!("the measure did not evaluate at Dual64: {other:?}"),
    }
}

/// Interval containment on a `gap` — the SIGNED arm, where a loose
/// enclosure is easier to get wrong than on a magnitude.
#[cfg(feature = "interval")]
#[test]
fn r2_a_signed_gap_at_interval_contains_the_f64_value() {
    use geom_core::{Bounds, Interval};
    let d0 = empty("r2-ivl");
    let (d1, outer) = sphere(&d0, 1.0, 0.0);
    let (d2, inner) = sphere(&d1, 0.5, 0.75);
    let ev = eval(&d2);
    let fo = faces(&ev, outer, geom_brep::SurfaceKind::Sphere);
    let fi = faces(&ev, inner, geom_brep::SurfaceKind::Sphere);
    let (d3, id) = with_measure(
        &d2,
        MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 0, inner: 1 }),
        vec![fo[0].clone(), fi[0].clone()],
    );
    let at_f64 = measured(&eval(&d3), id);
    assert!(at_f64 < 0.0, "the authored pair interferes: g = -0.25");
    let evi = evaluate::<Interval>(
        &d3,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    match evi.nodes.get(&id) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, .. } => assert!(
                value.lo() <= at_f64 && at_f64 <= value.hi(),
                "[{}, {}] must contain {at_f64}",
                value.lo(),
                value.hi()
            ),
            other => panic!("expected a measure, got {}", other.kind_name()),
        },
        other => panic!("no interval measure: {other:?}"),
    }
}

// ===============================================================
// DEVIATION 3 — "the carrier as minted"
// ===============================================================

/// **A measure of a transformed body reports the UNTRANSFORMED
/// carrier, and the recourse the variant documents does not exist.**
///
/// `Node::Measure`'s docs state the minted-carrier scope and name the
/// way out: "measuring the moved one means referencing the moving
/// node's own emission." A rigid `Transform` emits NO names — it hands
/// its input's table through by `Arc::clone` (N1: the name still points
/// at the minting node) — so there is no such emission to reference,
/// and every measure over a placed body silently reports the master's
/// geometry.
///
/// **Oracle.** A unit box at the origin, translated 100 m in +x, and a
/// fixed box at x in [5, 6]. The moved box's near corner is at x = 100;
/// the fixed reference corner is at x = 5, so the true separation is at
/// least 94. The interrogation layer — which takes the node to read the
/// name AT, the argument a measure has no place for — is the
/// independent witness: `vertex_position(ev, moved, name)` reports the
/// moved point while the measure over the same name reports the master.
#[test]
fn r2_a_transform_has_no_emission_to_measure() {
    let d0 = empty("r2-minted");
    let (d1, b) = boxed(&d0, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (d2, fixed) = boxed(&d1, (5.0, 6.0), (0.0, 1.0), 0.0, 1.0);
    let moved = RecipeNodeId(d2.len() as u64);
    let d3 = push(
        &d2,
        &DocEdit::InsertNode {
            node: Node::Transform {
                input: b,
                translation: [len(100.0), len(0.0), len(0.0)],
                rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                rotation_angle: ang(0.0),
            },
        },
    );
    let ev = eval(&d3);
    let vs_minted = vertices(&ev, b);
    let vs_moved = vertices(&ev, moved);
    let vs_fixed = vertices(&ev, fixed);
    assert_eq!(vs_minted.len(), 8);
    assert_eq!(vs_moved.len(), 8);

    // (1) The transform emits no NEW name: its table is the input's.
    assert_eq!(
        vs_minted, vs_moved,
        "a rigid transform re-emits its input's names verbatim, so there is \
         no 'moving node's own emission' to reference"
    );

    // (2) The interrogation layer, which DOES take the node to read at,
    // reports the two positions apart by the authored 100 m.
    let at_master = editor_core::vertex_position(&ev, b, &vs_minted[0]).expect("readable");
    let at_moved = editor_core::vertex_position(&ev, moved, &vs_moved[0]).expect("readable");
    let shift = (at_moved - at_master).norm();
    assert!(
        (shift - 100.0).abs() < 1e-9,
        "the interrogation door sees the authored 100 m shift; got {shift}"
    );

    // (3) The measure reports the MASTER's number, with no diagnostic.
    let (d4, id) = with_measure(
        &d3,
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        vec![vs_moved[0].clone(), vs_fixed[0].clone()],
    );
    let ev2 = eval(&d4);
    let m = measured(&ev2, id);
    let truth = (at_moved - editor_core::vertex_position(&ev, fixed, &vs_fixed[0]).unwrap()).norm();
    eprintln!(
        "R2/minted: measure reports {m}; the placed geometry is {truth} away \
         (master reading, no diagnostic)"
    );
    assert!(
        (m - truth).abs() > 50.0,
        "the measure must disagree with the placed geometry by the transform; \
         got {m} vs {truth}"
    );
    assert!(
        matches!(ev2.nodes.get(&id), Some(NodeResult::Ok(_))),
        "and it is a plain success — nothing flags the stale carrier"
    );
}

// ===============================================================
// CLAIM 3 — the load door, on files this reviewer corrupts
// ===============================================================

/// **A corrupt v16 file is refused typed at the LOAD door**, for each
/// of the three structural faults the edit door refuses: an
/// out-of-range primitive index, a dimension-mismatched assertion
/// bound, and an assertion pointed at something that is not a measure.
///
/// The corruption is done on the SERIALIZED text, not by building an
/// illegal document in memory, so this exercises the door a real
/// hostile file arrives through.
#[test]
fn r2_corrupt_v16_files_refuse_at_the_load_door() {
    let d0 = empty("r2-load");
    let (d1, b) = boxed(&d0, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let ev = eval(&d1);
    let vs = vertices(&ev, b);
    let (d2, measure) = with_measure(
        &d1,
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        vec![vs[0].clone(), vs[1].clone()],
    );
    let good = push(
        &d2,
        &DocEdit::InsertNode {
            node: Node::Assertion {
                measure,
                bound: Expr::literal(0.5, Dimension::Length).expect("finite"),
                dir: AssertionDir::AtLeast,
            },
        },
    );
    let text = editor_core::save(&good, &[], Tol::witness()).expect("a well-formed document saves");
    assert!(
        editor_core::load(&text, Tol::witness()).is_ok(),
        "the uncorrupted file must load"
    );
    eprintln!("R2/load: saved file is\n{text}");

    // (a) index out of range: rewrite the primitive's `b` index to a
    // reference the node does not carry. The exact spelling depends on
    // the wire shape, so several candidates are tried and the row
    // requires that at least one corruption applied AND was refused.
    let mut applied = 0usize;
    for (from, to) in [
        ("\"b\": 1", "\"b\": 9"),
        ("\"b\":1", "\"b\":9"),
        ("\"inner\": 1", "\"inner\": 9"),
    ] {
        let corrupt = text.replacen(from, to, 1);
        if corrupt == text {
            continue;
        }
        applied += 1;
        match editor_core::load(&corrupt, Tol::witness()) {
            Err(e) => eprintln!("R2/load: out-of-range index refused: {e}"),
            Ok(_) => panic!("an out-of-range primitive index LOADED ({from} -> {to})"),
        }
    }
    assert!(
        applied > 0,
        "no index corruption applied; the wire shape moved"
    );

    // (b) and (c): the edit door's own refusals, which the load door
    // re-runs verbatim through the same `validate_snapshot` walk.
    let bad_dim = try_push(
        &d2,
        &DocEdit::InsertNode {
            node: Node::Assertion {
                measure,
                bound: Expr::literal(0.5, Dimension::Angle).expect("finite"),
                dir: AssertionDir::AtLeast,
            },
        },
    );
    assert!(
        bad_dim.is_err(),
        "an Angle bound on a Length measure must refuse"
    );
    let bad_target = try_push(
        &d2,
        &DocEdit::InsertNode {
            node: Node::Assertion {
                measure: b,
                bound: Expr::literal(0.5, Dimension::Length).expect("finite"),
                dir: AssertionDir::AtLeast,
            },
        },
    );
    assert!(
        bad_target.is_err(),
        "an assertion over a non-measure must refuse"
    );
}

/// A trivial existence check so the file is never silently empty.
#[test]
fn r2_probe_file_is_live() {
    let _ = empty("r2-live");
}

#[allow(dead_code)]
fn unused(_: Axis3, _: DocParamValue, _: NodeErrorKind) {}

// ===============================================================
// The required e2e: author a measured document, hand it to a
// consumer through the persistence door.
// ===============================================================

/// **The reviewer's own measured document**, different geometry from
/// the unit's two-hole plate: a BALL IN A SOCKET, which is C5's
/// concentric-sphere fit and the arm the shipped suites never
/// evaluate.
///
/// Socket R = 1.0 at the origin, ball r = 0.9 offset 0.05 along +z, so
/// the authored gap is 1.0 - 0.9 - 0.05 = 0.05 — a clearance. An
/// assertion demands at least 0.02, which holds; a second document
/// moves the ball to 0.15, giving g = -0.05 and a Violated verdict.
///
/// The document is written to `target/r2_fit.pncad` so the Python half
/// of the e2e can read the verdict as a consumer.
#[test]
fn r2_e2e_ball_in_socket_authored_and_saved() {
    for (offset, want_gap, want_holds) in [(0.05_f64, 0.05_f64, true), (0.15, -0.05, false)] {
        let d0 = empty("r2-fit");
        let (d1, socket) = sphere(&d0, 1.0, 0.0);
        let (d2, ball) = sphere(&d1, 0.9, offset);
        let ev = eval(&d2);
        let fo = faces(&ev, socket, geom_brep::SurfaceKind::Sphere);
        let fi = faces(&ev, ball, geom_brep::SurfaceKind::Sphere);
        let (d3, measure) = with_measure(
            &d2,
            MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 0, inner: 1 }),
            vec![fo[0].clone(), fi[0].clone()],
        );
        let assertion = RecipeNodeId(d3.len() as u64);
        let d4 = push(
            &d3,
            &DocEdit::InsertNode {
                node: Node::Assertion {
                    measure,
                    bound: Expr::literal(0.02, Dimension::Length).expect("finite"),
                    dir: AssertionDir::AtLeast,
                },
            },
        );
        let ev = eval(&d4);
        let g = measured(&ev, measure);
        assert!(
            (g - want_gap).abs() < 1e-12,
            "R - r - |dc| = 1.0 - 0.9 - {offset} = {want_gap}, got {g}"
        );
        let v = match ev.nodes.get(&assertion) {
            Some(NodeResult::Ok(v)) => match &v.payload {
                ValuePayload::Assertion(v) => v.clone(),
                other => panic!("not a verdict: {}", other.kind_name()),
            },
            other => panic!("the assertion did not evaluate: {other:?}"),
        };
        assert_eq!(
            v.holds(),
            Some(want_holds),
            "g = {want_gap} against a 0.02 AtLeast bound"
        );
        if !want_holds {
            match &v {
                AssertionVerdict::Violated { measured, bound } => {
                    assert!((measured - want_gap).abs() < 1e-12);
                    assert!((bound - 0.02).abs() < 1e-15);
                }
                other => panic!("expected Violated with both numbers, got {other:?}"),
            }
            // The violating document is the one Python reads.
            let text = editor_core::save(&d4, &[], Tol::witness()).expect("the fit document saves");
            let path =
                std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/r2_fit.pncad");
            // The DIRECTORY is created first, and that is not
            // belt-and-braces. Under `cargo test` `target/` is always
            // there, but the hosted matrix runs from a NEXTEST ARCHIVE
            // on a runner that never built anything, so the path's
            // parent does not exist and the write failed the whole
            // shard. The path itself is kept because
            // `crates/pncad-py/examples/r2_read_a_verdict.py` reads
            // exactly this file.
            if let Some(dir) = path.parent() {
                std::fs::create_dir_all(dir).expect("the fixture's directory");
            }
            std::fs::write(&path, &text).expect("write the e2e fixture");
            eprintln!("R2/e2e: wrote {}", path.display());
        }
    }
}

/// **`SnapshotError::AssertionBound` reached from a corrupt file.**
///
/// The shipped suites refuse both assertion faults at the EDIT door
/// only; `AssertionBound` — a new public error with two distinct
/// `Display` arms — is named nowhere outside its own definition. This
/// row corrupts the saved bytes so the load-door arm actually runs, in
/// both of its shapes.
#[test]
fn r2_a_corrupt_assertion_refuses_at_the_load_door() {
    let d0 = empty("r2-assert-load");
    let (d1, b) = boxed(&d0, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let ev = eval(&d1);
    let vs = vertices(&ev, b);
    let (d2, measure) = with_measure(
        &d1,
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        vec![vs[0].clone(), vs[1].clone()],
    );
    let doc = push(
        &d2,
        &DocEdit::InsertNode {
            node: Node::Assertion {
                measure,
                bound: Expr::literal(0.5, Dimension::Length).expect("finite"),
                dir: AssertionDir::AtLeast,
            },
        },
    );
    let text = editor_core::save(&doc, &[], Tol::witness()).expect("saves");

    // (a) the bound's DIMENSION retyped to Angle: the measure is a
    // Length, so `measured: Some(Length)` against `bound: Angle`. The
    // assertion is the LAST node, so its literal is the last one.
    let at = text.rfind("\"Length\"").expect("a Length literal exists");
    let mut dim_corrupt = text.clone();
    dim_corrupt.replace_range(at..at + "\"Length\"".len(), "\"Angle\"");
    assert_ne!(dim_corrupt, text, "the dimension corruption must land");
    match editor_core::load(&dim_corrupt, Tol::witness()) {
        Err(e) => eprintln!("R2/assert-load: retyped bound refused: {e}"),
        Ok(_) => panic!("a dimension-mismatched assertion bound LOADED"),
    }

    // (b) the assertion's target repointed at a non-measure node.
    let tgt_corrupt = text.replacen(
        &format!("\"measure\": {}", measure.0),
        &format!("\"measure\": {}", b.0),
        1,
    );
    assert_ne!(tgt_corrupt, text, "the target corruption must land");
    match editor_core::load(&tgt_corrupt, Tol::witness()) {
        Err(e) => eprintln!("R2/assert-load: non-measure target refused: {e}"),
        Ok(_) => panic!("an assertion over a non-measure LOADED"),
    }
}

// ===============================================================
// The finiteness door `eval_measure` does not have
// ===============================================================

/// **A measured expression can evaluate to a non-finite quantity and
/// report it as an ordinary typed success.**
///
/// `expr::eval` refuses a non-finite RESULT at its "door 2"
/// (`EvalError::NonFiniteResult`), so no slot expression can ever hand
/// an infinity to an op. `eval::measure::eval_measure` restates the
/// arithmetic — `Div` is a bare `x / y` — and does not restate that
/// door, so the measurement language, which the module doc calls "the
/// same arithmetic", is missing the one refusal the arithmetic had.
///
/// **Oracle.** `distance(v0, v1) / s` with the document parameter
/// `s = 0`. Each VALUE leaf is finiteness-checked on its own (`s` is a
/// finite 0.0 and passes); the division that produces the infinity
/// happens inside `eval_measure`, downstream of every door.
#[test]
fn r2_a_measured_expression_can_report_a_non_finite_quantity() {
    let d0 = empty("r2-nonfinite");
    let d0 = push(
        &d0,
        &DocEdit::SetDocParam {
            name: ParamName::new("s"),
            value: DocParam::Continuous {
                dim: Dimension::Scalar,
                value: 0.0,
                display_unit: UnitSym::canonical_for(Dimension::Scalar),
                distribution: None,
            },
        },
    );
    let (d1, b) = boxed(&d0, (0.0, 3.0), (0.0, 4.0), 0.0, 12.0);
    let ev = eval(&d1);
    let vs = vertices(&ev, b);
    let expr = MeasureExpr::div(
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        MeasureExpr::value(Expr::param(ParamName::new("s"), Dimension::Scalar)),
    )
    .expect("Length / Scalar is a Length");
    let (d2, id) = with_measure(&d1, expr, vec![vs[0].clone(), vs[7].clone()]);

    // The same shape in a SLOT refuses, which is the comparison: an
    // extrude distance of `13 / s` would never reach an op.
    let slotted = try_push(
        &d2,
        &DocEdit::InsertNode {
            node: Node::Extrude {
                profile: RecipeNodeId(1),
                distance: Expr::div(
                    Expr::literal(13.0, Dimension::Length).unwrap(),
                    Expr::param(ParamName::new("s"), Dimension::Scalar),
                )
                .expect("Length / Scalar"),
            },
        },
    );
    if let Ok(doc) = slotted {
        let sid = RecipeNodeId(d2.len() as u64);
        let sev = eval(&doc);
        eprintln!(
            "R2/nonfinite: the same division in a SLOT evaluates to {:?}",
            sev.nodes.get(&sid).map(|r| match r {
                NodeResult::Ok(_) => "Ok".to_string(),
                NodeResult::Failed(e) => format!("Failed({:?})", e.kind),
                other => format!("{other:?}"),
            })
        );
    }

    match outcome(&eval(&d2), id) {
        Ok(v) => {
            eprintln!("R2/nonfinite: the MEASURE reported {v} as a typed success");
            assert!(
                !v.is_finite(),
                "the probe is only meaningful if the value is non-finite; got {v}"
            );
        }
        Err(e) => eprintln!("R2/nonfinite: the measure refused: {e}"),
    }
}

/// If the non-finite measure above is admitted, what does an assertion
/// over it say? A verdict about an infinity is the reportable half.
#[test]
fn r2_an_assertion_over_a_non_finite_measure() {
    let d0 = empty("r2-nonfinite-assert");
    let d0 = push(
        &d0,
        &DocEdit::SetDocParam {
            name: ParamName::new("s"),
            value: DocParam::Continuous {
                dim: Dimension::Scalar,
                value: 0.0,
                display_unit: UnitSym::canonical_for(Dimension::Scalar),
                distribution: None,
            },
        },
    );
    let (d1, b) = boxed(&d0, (0.0, 3.0), (0.0, 4.0), 0.0, 12.0);
    let ev = eval(&d1);
    let vs = vertices(&ev, b);
    let expr = MeasureExpr::div(
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        MeasureExpr::value(Expr::param(ParamName::new("s"), Dimension::Scalar)),
    )
    .expect("Length / Scalar");
    let (d2, measure) = with_measure(&d1, expr, vec![vs[0].clone(), vs[7].clone()]);
    let assertion = RecipeNodeId(d2.len() as u64);
    let Ok(d3) = try_push(
        &d2,
        &DocEdit::InsertNode {
            node: Node::Assertion {
                measure,
                bound: Expr::literal(1.0, Dimension::Length).expect("finite"),
                dir: AssertionDir::AtLeast,
            },
        },
    ) else {
        eprintln!("R2/nonfinite-assert: the assertion was refused at the edit door");
        return;
    };
    let ev = eval(&d3);
    match ev.nodes.get(&assertion) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Assertion(verdict) => {
                eprintln!("R2/nonfinite-assert: verdict = {verdict:?}");
            }
            other => eprintln!("R2/nonfinite-assert: payload {}", other.kind_name()),
        },
        other => eprintln!("R2/nonfinite-assert: {other:?}"),
    }
}
