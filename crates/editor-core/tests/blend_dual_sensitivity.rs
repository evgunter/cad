//! **E4's sensitivity through a 3D blend**: a measure read off a
//! filleted cube, with the fillet radius and the extrude depth as the
//! document's continuous parameters, differentiates through the blend
//! — `stackup::sensitivities` agrees with central differences of the
//! `f64` evaluation and with the closed form.
//!
//! The corpus row at `Dual64` compares the value channel only, so a
//! blend that dropped its tangent would leave it green; this row reads
//! the tangent the stack-up reports.
//!
//! The measure is the distance between two vertices the fillet mints:
//! the top face's foot `(r, r, d)` and the bottom face's foot at the
//! opposite corner `(1 − r, 1 − r, 0)`, so
//! `m = √(2(1 − 2r)² + d²)`, `∂m/∂r = −4(1 − 2r)/m`, `∂m/∂d = d/m`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use editor_core::stackup::{Chamber, SensitivityOutcome, sensitivities};
use editor_core::{
    CancelToken, Dimension, DocEdit, DocParam, EvalOptions, Evaluation, Expr, LoopProgram,
    MeasureExpr, MeasurePrimitive, Node, ParamName, ProfileDoc, ProfileProgram, RecipeNodeId,
    SitedRef, UnitSym, ValuePayload, evaluate,
};
use geom_core::Tol;

use fixture::{Recorder, prism_edges};

/// The authored fillet radius and extrude depth.
const R0: f64 = 0.125;
const D0: f64 = 1.0;

/// The central-difference step.
const H: f64 = 1e-6;

/// The agreement the reported sensitivity owes the central difference.
/// `m` is smooth with `|m'''| = O(1)` here, so the truncation error is
/// `~h²/6 ≈ 2e-13`; the rounding error is `ε·m / h ≈ 3e-10`. `1e-7`
/// sits nearly three decades above that and several below the `O(1)`
/// error of a dropped tangent.
const FD_TOL: f64 = 1e-7;

/// The agreement owed to the closed form: the vertex positions are the
/// closed form's arithmetic up to rounding.
const CF_TOL: f64 = 1e-12;

fn name(n: &'static str) -> ParamName {
    ParamName::from_static(n)
}

fn length(value: f64) -> DocParam {
    DocParam::Continuous {
        dim: Dimension::Length,
        value,
        display_unit: UnitSym::canonical_for(Dimension::Length),
        distribution: None,
    }
}

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// The vertex of `node`'s body nearest `at`, which must lie within
/// `1e-12` of it.
fn vertex_at(ev: &Evaluation<f64>, node: RecipeNodeId, at: [f64; 3]) -> SitedRef {
    let (v, d) = editor_core::all_vertices(ev, node)
        .into_iter()
        .map(|v| {
            let p = editor_core::vertex_position(ev, node, &v).expect("a vertex");
            let d = ((p.x - at[0]).powi(2) + (p.y - at[1]).powi(2) + (p.z - at[2]).powi(2)).sqrt();
            (v, d)
        })
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .expect("the filleted cube has vertices");
    assert!(
        d <= 1e-12,
        "no vertex of node {node:?} at {at:?} (nearest {d})"
    );
    SitedRef::new(node, v)
}

/// The unit square extruded by `depth`, every edge filleted at
/// `radius`, measured foot to opposite foot.
fn filleted_cube() -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("radius"),
        value: length(R0),
    });
    r.push(DocEdit::SetDocParam {
        name: name("depth"),
        value: length(D0),
    });
    let frame = r.insert(fixture::xy_frame());
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane: frame,
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
                .expect("finite corners"),
        ],
        ids: Vec::new(),
    }));
    let cube = r.insert(Node::Extrude {
        profile,
        distance: Expr::param(name("depth"), Dimension::Length),
        side: ExtrudeSide::Along,
    });
    let blank = r.insert(Node::fillet(
        cube,
        Expr::param(name("radius"), Dimension::Length),
        prism_edges(&r.doc, cube, 4),
    ));
    let ev = eval(&r.doc);
    let refs = vec![
        vertex_at(&ev, blank, [R0, R0, D0]),
        vertex_at(&ev, blank, [1.0 - R0, 1.0 - R0, 0.0]),
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

/// The `f64` measure of `doc` with `param` set to `value`.
fn measured(doc: &ProfileDoc, measure: RecipeNodeId, param: &'static str, value: f64) -> f64 {
    let edited = editor_core::apply(
        doc,
        &DocEdit::SetDocParam {
            name: name(param),
            value: length(value),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .unwrap_or_else(|e| panic!("edit refused: {e}"))
    .doc;
    match eval(&edited).value(measure).map(|v| &v.payload) {
        Some(ValuePayload::Measure { value, .. }) => *value,
        other => panic!("{param} = {value}: the measure did not evaluate: {other:?}"),
    }
}

#[test]
fn a_fillet_radius_sensitivity_matches_finite_differences_of_the_f64_build() {
    let (doc, measure) = filleted_cube();
    let entries =
        sensitivities(&doc, measure, None, None, false, Tol::witness()).expect("the driver runs");
    let m0 = (2.0 * (1.0 - 2.0 * R0).powi(2) + D0 * D0).sqrt();
    assert_eq!(
        measured(&doc, measure, "radius", R0),
        m0,
        "the nominal measure"
    );
    let closed = [
        ("depth", D0, D0 / m0),
        ("radius", R0, -4.0 * (1.0 - 2.0 * R0) / m0),
    ];
    assert_eq!(entries.len(), closed.len(), "one entry per parameter");
    let mut misses = Vec::new();
    for (entry, &(param, nominal, want)) in entries.iter().zip(&closed) {
        assert_eq!(entry.param, name(param), "entries come in name order");
        let SensitivityOutcome::Derivative {
            value,
            chamber: Chamber::LocalOnly,
        } = entry.outcome
        else {
            panic!("{param}: not a local derivative: {:?}", entry.outcome)
        };
        let fd = (measured(&doc, measure, param, nominal + H)
            - measured(&doc, measure, param, nominal - H))
            / (2.0 * H);
        if (value - fd).abs() > FD_TOL {
            misses.push(format!("∂m/∂{param} = {value} vs central difference {fd}"));
        }
        if (value - want).abs() > CF_TOL {
            misses.push(format!("∂m/∂{param} = {value} vs closed form {want}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}
