//! **The two-hole plate, as the tour authors it** — ERROR-DESIGN's
//! worked example, in this crate so the ceiling measurement and the
//! census can drive the SAME document the demo prints.
//!
//! It is a port, not a second design: `demos/tour/src/tolerance.rs`
//! remains the consumer-seat authoring of it (selection door and all),
//! and the numbers here are that file's constants. What this module
//! adds is a door the crate's own suites can call, so a measurement of
//! the plate's ceiling does not have to live in a demo that is not run
//! under the ε matrix.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    Datum, Dimension, Distribution, DocEdit, DocParam, EntityKind, Expr, GeomPred, LoopProgram,
    MeasureExpr, MeasurePrimitive, MeasureRef, NamePat, Node, ParamName, ProfileDoc,
    ProfileProgram, RecipeNodeId, Selector, SurfaceKindSet, UnitSym, select_where,
};
use geom_core::Tol;

use fixture::Recorder;

/// The nominal hole spacing, in metres (3.1 mm) — the tour's own.
pub(crate) const SPACING: f64 = 3.1e-3;
/// The nominal hole radius, in metres (1.25 mm).
pub(crate) const RADIUS: f64 = 1.25e-3;
/// The nominal web: `SPACING − 2·RADIUS` = 0.6 mm.
pub(crate) const WEB: f64 = SPACING - 2.0 * RADIUS;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("finite length")
}

fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("finite scalar")
}

fn param(n: &str) -> Expr {
    Expr::param(ParamName::new(n), Dimension::Length)
}

/// The plate, its two holes, the web measure and its assertion.
///
/// The two tolerances are separate arguments rather than one scale
/// because their RATIO decides whether the RSS and the certified worst
/// case disagree — a modelling choice, not a size.
pub(crate) fn plate(
    spacing_half_width: f64,
    radius_sigma: f64,
    tol: Tol,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    let declare = |r: &mut Recorder, n: &str, value: f64, distribution: Distribution| {
        r.push(DocEdit::SetDocParam {
            name: ParamName::new(n),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: Some(distribution),
            },
        });
    };
    declare(
        &mut r,
        "half_spacing",
        SPACING / 2.0,
        Distribution::Uniform {
            lo: -spacing_half_width,
            hi: spacing_half_width,
        },
    );
    for n in ["hole_a_r", "hole_b_r"] {
        declare(
            &mut r,
            n,
            RADIUS,
            Distribution::Normal {
                sigma: radius_sigma,
            },
        );
    }

    let plane = r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(1.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(1.0), scl(0.0)],
    }));
    let plate_profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![
            LoopProgram::polygon([
                (-4.0e-3, -2.0e-3),
                (4.0e-3, -2.0e-3),
                (4.0e-3, 2.0e-3),
                (-4.0e-3, 2.0e-3),
            ])
            .expect("finite plate corners"),
        ],
    }));
    let _plate = r.insert(Node::Extrude {
        profile: plate_profile,
        distance: len(1.0e-3),
    });

    let hole = |r: &mut Recorder, centre: Expr, radius: &str| {
        let profile = r.insert(Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::Circle {
                centre: [centre, len(0.0)],
                radius: param(radius),
            }],
        }));
        r.insert(Node::Extrude {
            profile,
            distance: len(1.0e-3),
        })
    };
    let hole_a = hole(
        &mut r,
        Expr::sub(len(0.0), param("half_spacing")).expect("a length"),
        "hole_a_r",
    );
    let hole_b = hole(&mut r, param("half_spacing"), "hole_b_r");

    // The wall of each hole, through the selection door the tour uses.
    let refs = {
        let ev: editor_core::Evaluation<f64> = editor_core::evaluate(
            &r.doc,
            None,
            &editor_core::CancelToken::new(),
            &editor_core::EvalOptions::default(),
            tol,
        );
        let env = r.doc.param_env::<f64>();
        let wall = |node: RecipeNodeId| {
            let mut faces = select_where(
                &ev,
                node,
                &Selector::of(NamePat::of_kind(
                    EntityKind::Face,
                )),
                &[GeomPred::SurfaceKind(
                    SurfaceKindSet::just(geom_brep::SurfaceKind::Cylinder),
                )],
                &env,
                tol,
            )
            .expect("the surface-kind atom is exact");
            faces.sort();
            assert!(!faces.is_empty(), "a hole extrude has a cylindrical wall");
            MeasureRef::new(node, faces.remove(0))
        };
        vec![wall(hole_a), wall(hole_b)]
    };

    // web = distance(wall_a, wall_b) - r_a - r_b.
    let radius_of = |n: &str| MeasureExpr::value(param(n));
    let web = MeasureExpr::sub(
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        MeasureExpr::add(radius_of("hole_a_r"), radius_of("hole_b_r")).expect("Length + Length"),
    )
    .expect("Length - Length");

    let measure = r.insert(Node::measure(web, refs).expect("both indices in range"));
    let assertion = r.insert(Node::Assertion {
        measure,
        bound: len(WEB - 1.0e-4),
        dir: editor_core::AssertionDir::AtLeast,
    });
    (r.doc, measure, assertion)
}
