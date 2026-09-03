//! **The E4 sensitivity driver and the E5 stackup**
//! (`docs/ERROR-DESIGN.md` E4/E5/E9; `docs/M10-4-SPEC.md` §2–§6).
//!
//! Everything here goes through the public doors a consumer has:
//! `analyzed_box` for the box, `drive` for the verdict, `sensitivities`
//! and `stackup` for the answers, the report's own fields to read them.
//!
//! The rows are the spec's review claims, one each, plus the worked
//! example: the two-hole plate's stackup (§6), the REQUIRED
//! profile-dimension pin riding inside it (`hole_r` drives two circle
//! profiles and its sensitivity is read through the guided lift), the
//! pairing hook red on a planted stale build (§3), no third state (§5),
//! E9 forfeiture on a domain-edge kink (§5), the curvature case where
//! the hull and the linearized sum part company (claim 6), and RSS
//! totality over bands (claim 7).
//!
//! # The widths are in ε, and that is the honest limit
//!
//! Every box below that must CERTIFY is sized as a small multiple of ε,
//! for the reason `m10_3_driver_interval.rs` measures: the certification
//! predicates are identities whose interval enclosure widens with the
//! box, and a leaf goes definite only once its own width is a fraction
//! of ε. A macroscopic tolerance box refuses all of its mass as
//! `Budget`, so today's stackup over a real study has NO certified leaf
//! and refuses `NothingCertified` — pinned below rather than described.
//!
//! The file's basename carries `interval` because the driver, the
//! chamber mark's certified variant and the gating `worst_case` all
//! live behind that feature; the hosted lane is asked for by trailer,
//! never inferred from the name.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::time::Instant;

use editor_core::UnitSym;
use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, drive};
use editor_core::stackup::{
    Chamber, PairingViolation, Rss, Sensitivity, SensitivityOutcome, SensitivityRefusal,
    StackupRefusal, Unavailable, sensitivities, stackup,
};
use editor_core::{
    AssertionDir, AssertionVerdict, CancelToken, CapEnd, Dimension, Distribution, DocEdit,
    DocParam, DocParamValue, EvalOptions, Evaluation, Expr, LoopProgram, MeasureExpr,
    MeasurePrimitive, MeasureRef, Node, NodeResult, ParamName, ProfileDoc, ProfileProgram,
    RecipeNodeId, RoleSeg, ValuePayload, evaluate,
};
use geom_core::Tol;

use fixture::{Recorder, ang, fname, len, scl, wall};

/// Hole centres at x = ±`HOLE_X`; the web is `2·HOLE_X − 2·hole_r`.
const HOLE_X: f64 = 0.30;
/// The authored hole radius.
const R0: f64 = 0.2;
/// The assertion's bound: the web must be at least this.
const MIN_WEB: f64 = 0.0005;
/// The interval lane's padding on the plate's worst case beyond the
/// true range, MEASURED (the e2e row prints the number it bounds), as
/// a multiple of the radius axis's half-width: each hole's cylinder
/// axis is recovered from the LIFTED arc's endpoints and bulge, and
/// interval arithmetic cannot see that the radius cancels out of the
/// centre — the dependency problem, M10-3's headline — so the recovered
/// axis widens by the radius's own width, once per hole. The padding is
/// therefore proportional to the BOX (`2·half`, exactly, plus the
/// rounding below), not to the machine epsilon. A bound, not a target
/// — if it grows, the question is why the lane widened.
const PLATE_PADDING_PER_HALF_WIDTH: f64 = 2.0;
/// The rounding on top of the dependency padding: a 0.2-scale quantity
/// through a few dozen outward-rounded operations (measured ~1e-15).
const PLATE_ROUNDING: f64 = 1.0e-14;

fn eps() -> f64 {
    Tol::witness().eps()
}

fn name(n: &str) -> ParamName {
    ParamName::new(n)
}

fn param(n: &str, dim: Dimension) -> Expr {
    Expr::param(name(n), dim)
}

fn uniform(half: f64) -> Distribution {
    Distribution::Uniform {
        lo: -half,
        hi: half,
    }
}

fn band(half: f64) -> Distribution {
    Distribution::Band {
        lo: -half,
        hi: half,
    }
}

fn continuous(dim: Dimension, value: f64, distribution: Option<Distribution>) -> DocParam {
    DocParam::Continuous {
        dim,
        value,
        display_unit: UnitSym::canonical_for(dim),
        distribution,
    }
}

fn config(max_leaves: usize) -> DriveConfig {
    DriveConfig {
        max_leaves,
        ..DriveConfig::default()
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

fn push(doc: &ProfileDoc, edit: &DocEdit<ProfileProgram>) -> ProfileDoc {
    editor_core::apply(doc, edit, Tol::witness())
        .unwrap_or_else(|e| panic!("edit refused: {e}"))
        .doc
}

fn entry<'a>(entries: &'a [Sensitivity], n: &str) -> &'a SensitivityOutcome {
    &entries
        .iter()
        .find(|s| s.param == name(n))
        .unwrap_or_else(|| panic!("no entry for {n}"))
        .outcome
}

/// Certified, and by a leaf that holds the nominal — through the
/// library's own predicate (`ParamBox::contains_nominal`), so a wrong
/// rule there fails here rather than being re-implemented beside it.
fn contains_nominal(chamber: &Chamber) -> bool {
    match chamber {
        Chamber::ChamberCertified { leaf, .. } => leaf.contains_nominal(),
        Chamber::LocalOnly => false,
    }
}

/// One cylindrical wall of a circular extrude, read at that extrude,
/// found the way a user finds it (the selection door).
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

/// The vertex of `node`'s body at `at`, by name.
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

/// **The two-hole plate** (M10-2's e2e document, distributions from
/// M10-1): a plate whose extrude distance is `depth` and two hole tools
/// whose radius is `hole_r` — a PROFILE dimension, reaching the lane
/// only through the guided lift — with the web measure
/// `distance(wall, wall) − 2·hole_r` and an assertion on it. Returns
/// the document, the measure node and the assertion node.
fn plate(
    radius: Option<Distribution>,
    depth: Option<Distribution>,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    plate_spaced(HOLE_X, radius, depth)
}

/// [`plate`] with the hole centres at `±hole_x` — the same parameter
/// set, nominals and distributions, different geometry.
fn plate_spaced(
    hole_x: f64,
    radius: Option<Distribution>,
    depth: Option<Distribution>,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("hole_r"),
        value: continuous(Dimension::Length, R0, radius),
    });
    r.push(DocEdit::SetDocParam {
        name: name("depth"),
        value: continuous(Dimension::Length, 0.1, depth),
    });
    // One frame, named by every profile below: two sketches meant to
    // share a plane bind the same id.
    let frame = r.insert(fixture::xy_frame());
    let plate_profile = r.insert(Node::Profile(ProfileProgram {
        plane: frame,
        loops: vec![
            LoopProgram::polygon([(-1.0, -0.5), (1.0, -0.5), (1.0, 0.5), (-1.0, 0.5)])
                .expect("finite plate corners"),
        ],
    }));
    let _plate = r.insert(Node::Extrude {
        profile: plate_profile,
        distance: param("depth", Dimension::Length),
    });
    let mut holes = Vec::new();
    for cx in [-hole_x, hole_x] {
        let p = r.insert(Node::Profile(ProfileProgram {
            plane: frame,
            loops: vec![LoopProgram::Circle {
                centre: [len(cx), len(0.0)],
                radius: param("hole_r", Dimension::Length),
            }],
        }));
        holes.push(r.insert(Node::Extrude {
            profile: p,
            distance: len(0.1),
        }));
    }
    // The wall names come from the selection door, the way a user gets
    // them: evaluate what is built so far, ask each hole for its
    // cylindrical faces, read at the extrude that owns them.
    let ev = eval(&r.doc);
    let wall_of = |node| cyl_wall(&ev, &r.doc, node);
    let radius = || MeasureExpr::value(param("hole_r", Dimension::Length));
    let web = MeasureExpr::sub(
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        MeasureExpr::add(radius(), radius()).expect("Length + Length"),
    )
    .expect("Length - Length");
    let measure = r.insert(
        Node::measure(web, vec![wall_of(holes[0]), wall_of(holes[1])]).expect("indices in range"),
    );
    let assertion = r.insert(Node::Assertion {
        measure,
        bound: Expr::literal(MIN_WEB, Dimension::Length).expect("finite"),
        dir: AssertionDir::AtLeast,
    });
    (r.doc, measure, assertion)
}

/// **The curvature case**: a measure that is the SQUARE of a scalar
/// parameter, `m = a²`, and nothing else — no geometry, so the drive
/// certifies the whole analyzed box in one leaf and the row is about
/// the report's arithmetic alone.
fn square(nominal: f64, dist: Distribution) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("a"),
        value: continuous(Dimension::Scalar, nominal, Some(dist)),
    });
    let a = || MeasureExpr::value(param("a", Dimension::Scalar));
    let m = r.insert(
        Node::measure(
            MeasureExpr::mul(a(), a()).expect("Scalar · Scalar"),
            Vec::new(),
        )
        .expect("no references to address"),
    );
    (r.doc, m)
}

/// **The domain-edge kink**: a unit cube and its copy translated by
/// `t` along x, measured by the distance between the cube's vertex at
/// (1, 0, 0) and the copy's vertex that lands there at the nominal
/// `t = 1`. The value is `‖(t − 1, 0, 0)‖ = |t − 1|`, zero at the
/// nominal; its `Dual64` tangent is `0/0` through the norm's square
/// root — a degraded tangent under a perfectly finite value.
fn kink(dist: Distribution) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("t"),
        value: continuous(Dimension::Length, 1.0, Some(dist)),
    });
    // One frame, named by every profile below: two sketches meant to
    // share a plane bind the same id.
    let frame = r.insert(fixture::xy_frame());
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: frame,
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
                .expect("finite corners"),
        ],
    }));
    let cube = r.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
    });
    let copy = r.insert(Node::Transform {
        input: cube,
        translation: [param("t", Dimension::Length), len(0.0), len(0.0)],
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: ang(0.0),
    });
    let ev = eval(&r.doc);
    let at = |node, x: f64| {
        editor_core::all_vertices(&ev, node)
            .into_iter()
            .find(|v| {
                let p = editor_core::vertex_position(&ev, node, v).expect("a vertex");
                p.x == x && p.y == 0.0 && p.z == 0.0
            })
            .unwrap_or_else(|| panic!("node {node:?} has a vertex at ({x}, 0, 0)"))
    };
    let refs = vec![
        MeasureRef::new(cube, at(cube, 1.0)),
        MeasureRef::new(copy, at(copy, 1.0)),
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

/// **The slab with a measured thickness**: a unit square extruded by
/// `depth`, measured cap to cap — a magnitude-slot parameter whose
/// sensitivity is exactly 1.
fn slab(half: f64) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("depth"),
        value: continuous(Dimension::Length, 1.0, Some(uniform(half))),
    });
    // One frame, named by every profile below: two sketches meant to
    // share a plane bind the same id.
    let frame = r.insert(fixture::xy_frame());
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: frame,
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
                .expect("finite corners"),
        ],
    }));
    let block = r.insert(Node::Extrude {
        profile: p,
        distance: param("depth", Dimension::Length),
    });
    let refs = vec![
        MeasureRef::new(block, fname(block, RoleSeg::Cap(CapEnd::Bottom))),
        MeasureRef::new(block, fname(block, RoleSeg::Cap(CapEnd::Top))),
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

// -------------------------------------------------------------- e2e

/// **The worked example's stackup half** (§6). A full `Stackup` on the
/// web: nominal re-derived, `worst_case` from certified leaves and
/// consistent with the assertion's verdict, ∂web/∂hole_r = −2 by the
/// plate's own formula (through the guided lift — the profile pin),
/// ∂web/∂depth = 0, every mark chamber-certified by a leaf holding the
/// nominal, RSS the linearized spread, coverage summing to 1.
#[test]
fn the_two_hole_plate_stackup() {
    let half = eps() / 8.0;
    let (doc, measure, assertion) = plate(Some(uniform(half)), Some(uniform(half)));
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let t0 = Instant::now();
    let verdict = drive(&doc, &analyzed, &config(1024), Tol::witness()).expect("the plate builds");
    let t_drive = t0.elapsed();
    assert!(
        !verdict.certified().is_empty(),
        "the ε-scale box must certify: {:?}",
        verdict.receipt()
    );
    let handed = eval(&doc);
    let t1 = Instant::now();
    let report = stackup(
        &doc,
        measure,
        &analyzed,
        &verdict,
        Some(&handed),
        false,
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("the stackup refused: {e}"));
    let t_stackup = t1.elapsed();
    // EVIDENCE-ONLY: the v1 cost decision's measurement (one interval
    // evaluation per certified leaf plus n dual passes), beside the
    // drive it rides on.
    println!(
        "stackup cost: drive {:?} over {} leaves ({} certified); stackup {:?} \
         ({} params, {} worst-case leaves)",
        t_drive,
        verdict.receipt().certified + verdict.receipt().refused,
        verdict.receipt().certified,
        t_stackup,
        report.per_param.len(),
        report.worst_case.leaves
    );

    // The nominal: the plate's own formula, 2·0.30 − 2·0.2.
    assert!(
        (report.nominal - 0.2).abs() < 1e-12,
        "nominal {}",
        report.nominal
    );
    assert_eq!(report.measurement, measure);

    // The sensitivities, analytically: −2 on the radius (each hole
    // wall moves inward by dr, the axis separation does not move), 0
    // on the depth (the web is a planar quantity). Exact, and marked.
    let rows: Vec<Sensitivity> = report
        .per_param
        .iter()
        .map(|p| Sensitivity {
            param: p.param.clone(),
            outcome: p.sensitivity.clone(),
        })
        .collect();
    match entry(&rows, "hole_r") {
        SensitivityOutcome::Derivative { value, chamber } => {
            assert_eq!(value.to_bits(), (-2.0f64).to_bits(), "∂web/∂hole_r");
            assert!(contains_nominal(chamber), "{chamber:?}");
        }
        other => panic!("hole_r: {other:?}"),
    }
    match entry(&rows, "depth") {
        SensitivityOutcome::Derivative { value, chamber } => {
            // IEEE zero, not bit zero: a zero tangent arrives SIGNED on
            // other fixtures (a sign factor times zero), and this row
            // is about the value, not the sign of nothing.
            assert_eq!(*value, 0.0, "∂web/∂depth");
            assert!(contains_nominal(chamber), "{chamber:?}");
        }
        other => panic!("depth: {other:?}"),
    }
    // The mark, once, at the top — the same one every row carries.
    assert!(contains_nominal(&report.chamber), "{:?}", report.chamber);
    // Contributions: |∂m/∂p|·half-width over the ANALYZED box, labeled
    // advisory and chamber-exceeding; beside each, the same product
    // over the certified leaf's own half-width, which is what the mark
    // covers. On a drive that split, the box span is the larger.
    for p in &report.per_param {
        let expect = if p.param == name("hole_r") {
            2.0 * half
        } else {
            0.0
        };
        assert_eq!(p.contribution, Ok(expect), "{:?}", p.param);
        let span = p
            .chamber_span
            .expect("certified rows carry the chamber span");
        assert!(span.half_width <= half, "{:?}: {span:?}", p.param);
        assert!(span.contribution <= expect, "{:?}: {span:?}", p.param);
        println!(
            "EVIDENCE-ONLY {:?}: box half-width {half:e}, chamber half-width {:e} \
             ({:.1}x smaller)",
            p.param,
            span.half_width,
            half / span.half_width
        );
    }

    // The gating number: the hull over the certified leaves encloses
    // the nominal, is as narrow as the box (the web is linear in the
    // radius, so the true range is 0.2 ± 2·half), and sits wholly
    // above the assertion's bound — consistent with the verdict the
    // build reported.
    let wc = report.worst_case;
    assert_eq!(wc.leaves, verdict.certified().len());
    // The hull ENCLOSES the true range `nominal ± 2·half` (the web is
    // linear in the radius with slope −2), and exceeds it by the
    // interval lane's padding alone — measured here and bounded by
    // [`PLATE_PADDING_PER_HALF_WIDTH`] times the half-width plus
    // [`PLATE_ROUNDING`], stated in the honest-limits section of the
    // PR rather than hidden in a slack term.
    assert!(
        wc.lo <= report.nominal - 2.0 * half && report.nominal + 2.0 * half <= wc.hi,
        "the hull must enclose the true range: {wc:?}"
    );
    let padding = (wc.hi - wc.lo) - 4.0 * half;
    println!(
        "EVIDENCE-ONLY worst-case padding: hull width {:e}, true range {:e}, padding {padding:e} \
         ({:.2} ε)",
        wc.hi - wc.lo,
        4.0 * half,
        padding / eps()
    );
    assert!(
        padding <= PLATE_PADDING_PER_HALF_WIDTH * half + PLATE_ROUNDING,
        "padding {padding:e} exceeds the measured bound"
    );
    assert!(
        wc.lo >= MIN_WEB,
        "the worst case must clear the bound: {wc:?}"
    );
    match handed.value(assertion).map(|v| &v.payload) {
        Some(ValuePayload::Assertion(AssertionVerdict::Holds { .. })) => {}
        other => panic!("the nominal build's assertion holds: {other:?}"),
    }

    // RSS: √Σ(∂m/∂pᵢ·σᵢ)² with σ = width/√12 for a uniform, so
    // σ_web = 2·σ_r exactly (the depth term contributes 0).
    let sigma_r = (2.0 * half) / f64::sqrt(12.0);
    match report.rss {
        Rss::Advisory { sigma } => assert!(
            (sigma - 2.0 * sigma_r).abs() <= 1e-9 * sigma_r.max(1e-300),
            "rss {sigma} vs {}",
            2.0 * sigma_r
        ),
        other => panic!("rss: {other:?}"),
    }

    // Coverage: M10-3's accounting verbatim, summing to 1; a bounded
    // distribution's tail is exactly zero.
    let total = report.coverage.total().expect("no band here");
    assert!((total - 1.0).abs() <= 1e-9, "coverage sums to {total}");
    assert_eq!(report.coverage.unanalyzed, Ok(0.0));
    assert_eq!(&report.coverage, verdict.accounting());
}

/// **Claim 7 — RSS totality.** One band kills the RSS whole, and EVERY
/// band contributor is named — never the first of several. The
/// contributions stay (a band's limits are real limits) and so does
/// the gating worst case.
#[test]
fn a_band_contributor_refuses_the_rss_whole_naming_every_band() {
    let half = eps() / 8.0;
    let (doc, measure, _) = plate(Some(band(half)), Some(band(half)));
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(1024), Tol::witness()).expect("builds");
    assert!(!verdict.certified().is_empty());
    let report = stackup(
        &doc,
        measure,
        &analyzed,
        &verdict,
        None,
        false,
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("the stackup refused: {e}"));
    match &report.rss {
        Rss::UnavailableBecause { blockers } => {
            let named: Vec<&ParamName> = blockers.iter().map(Unavailable::param).collect();
            assert_eq!(named, vec![&name("depth"), &name("hole_r")], "{blockers:?}");
            assert!(
                blockers
                    .iter()
                    .all(|b| matches!(b, Unavailable::BandHasNoMeasure { .. }))
            );
        }
        other => panic!("rss over two bands: {other:?}"),
    }
    assert!(report.per_param.iter().all(|p| p.contribution.is_ok()));
    assert!(report.worst_case.lo <= report.nominal && report.nominal <= report.worst_case.hi);
    // Mass over a band does not price, and the accounting says so
    // rather than inventing a shape.
    assert!(report.coverage.total().is_err());
}

/// **Claim 6 — `worst_case` honesty.** `m = a²` over `a ∈ 1 ± 0.5`: the
/// linearized band is `1 ± 1.0`, the true range is `[0.25, 2.25]`, and
/// the certified hull IS the true range — it is not the linearized sum,
/// and where the two differ the hull is the one that is right.
#[test]
fn worst_case_is_the_hull_not_the_linearized_sum() {
    let (doc, measure) = square(1.0, uniform(0.5));
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(16), Tol::witness()).expect("builds");
    assert_eq!(verdict.certified().len(), 1, "{:?}", verdict.receipt());
    let report = stackup(
        &doc,
        measure,
        &analyzed,
        &verdict,
        None,
        false,
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("{e}"));
    let row = &report.per_param[0];
    match &row.sensitivity {
        SensitivityOutcome::Derivative { value, chamber } => {
            assert_eq!(value.to_bits(), 2.0f64.to_bits(), "d(a²)/da at 1");
            assert!(matches!(chamber, Chamber::ChamberCertified { .. }));
        }
        other => panic!("{other:?}"),
    }
    let contribution = row.contribution.clone().expect("available");
    assert_eq!(contribution.to_bits(), 1.0f64.to_bits());
    let linearized_hi = report.nominal + contribution;
    let wc = report.worst_case;
    assert!(
        wc.hi > linearized_hi,
        "the hull's top {} must exceed the linearized top {linearized_hi}",
        wc.hi
    );
    assert!(
        (wc.lo - 0.25).abs() <= 1e-12 && (wc.hi - 2.25).abs() <= 1e-12,
        "{wc:?}"
    );
}

/// **Claim 5 — E9.** A degraded tangent forfeits exactly its uses and
/// refuses nothing: the value channel certifies cleanly, the stackup
/// still gates on `worst_case`, the `per_param` row and the RSS say
/// `Unavailable` naming the parameter, and no path here is an error.
#[test]
fn tangent_poison_forfeits_its_uses_and_never_refuses() {
    let half = eps() / 16.0;
    let (doc, measure) = kink(uniform(half));
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(256), Tol::witness()).expect("builds");
    assert!(
        !verdict.certified().is_empty(),
        "the value channel certifies: {:?}",
        verdict.receipt()
    );
    // The driver alone: a forfeiture entry, not a refusal.
    let entries = sensitivities(&doc, measure, None, Some(&verdict), false, Tol::witness())
        .expect("no refusal");
    match entry(&entries, "t") {
        SensitivityOutcome::TangentDegraded { tangent } => assert!(!tangent.is_finite()),
        other => panic!("a 0/0 tangent is the forfeiture state: {other:?}"),
    }
    // The report: the row and the RSS forfeit, the gate stands.
    let report = stackup(
        &doc,
        measure,
        &analyzed,
        &verdict,
        None,
        false,
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("E9: tangent state reached a refusal: {e}"));
    assert_eq!(report.nominal.to_bits(), 0.0f64.to_bits());
    let row = &report.per_param[0];
    assert!(matches!(
        row.sensitivity,
        SensitivityOutcome::TangentDegraded { .. }
    ));
    assert_eq!(
        row.contribution,
        Err(Unavailable::TangentDegraded { param: name("t") })
    );
    assert_eq!(
        report.rss,
        Rss::UnavailableBecause {
            blockers: vec![Unavailable::TangentDegraded { param: name("t") }]
        }
    );
    let wc = report.worst_case;
    assert!(wc.lo >= 0.0 && wc.hi <= half + 1e-9, "{wc:?}");
    assert_eq!(wc.leaves, verdict.certified().len());
}

/// **Claim 3 — the pairing hook is red-capable.** A STALE handed build
/// — the document edited between builds — gets the typed violation and
/// never a sensitivity; a fresh one passes; a node-set change and a
/// canceled prefix are their own typed arms.
#[test]
fn the_pairing_hook_is_red_capable_on_a_stale_build() {
    let (doc, measure, assertion) = plate(None, None);
    let handed = eval(&doc);
    let fresh = sensitivities(&doc, measure, Some(&handed), None, false, Tol::witness());
    assert!(fresh.is_ok(), "{fresh:?}");

    // Stale by a parameter edit: the radius cone re-keys.
    let edited = push(
        &doc,
        &DocEdit::SetDocParamValue {
            name: name("hole_r"),
            value: DocParamValue::Continuous(0.21),
        },
    );
    match sensitivities(&edited, measure, Some(&handed), None, false, Tol::witness()) {
        Err(SensitivityRefusal::Pairing(PairingViolation::ContentKey {
            node,
            handed,
            rebuilt,
        })) => {
            assert_ne!(handed, rebuilt);
            assert!(edited.node(node).is_some());
        }
        other => panic!("a stale handed build must be a ContentKey violation: {other:?}"),
    }

    // Stale by structure: a node removed.
    let shrunk = push(&doc, &DocEdit::DeleteNode { id: assertion });
    assert_eq!(
        sensitivities(&shrunk, measure, Some(&handed), None, false, Tol::witness()).err(),
        Some(SensitivityRefusal::Pairing(PairingViolation::NodeSet))
    );

    // A canceled prefix validates nothing.
    let cancel = CancelToken::new();
    cancel.cancel();
    let partial = evaluate::<f64>(&doc, None, &cancel, &EvalOptions::default(), Tol::witness());
    assert_eq!(
        sensitivities(&doc, measure, Some(&partial), None, false, Tol::witness()).err(),
        Some(SensitivityRefusal::Pairing(PairingViolation::Incomplete))
    );
}

/// **Claim 4 — no third state.** Without a drive every mark is
/// `LocalOnly`; with a drive whose nominal sits in refused mass (a
/// macroscopic box, all `Budget`) every mark is `LocalOnly` and the
/// stackup refuses `NothingCertified` — there is no unmarked number
/// and no report without its gate. The sensitivity itself is still
/// read (∂thickness/∂depth = 1): local, and said so.
#[test]
fn no_drive_or_a_refused_nominal_marks_local_only_and_gates_nothing() {
    let (doc, measure) = slab(0.05);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let local = sensitivities(&doc, measure, None, None, false, Tol::witness()).expect("ok");
    match entry(&local, "depth") {
        SensitivityOutcome::Derivative { value, chamber } => {
            assert_eq!(value.to_bits(), 1.0f64.to_bits());
            assert_eq!(*chamber, Chamber::LocalOnly);
        }
        other => panic!("{other:?}"),
    }
    let verdict = drive(&doc, &analyzed, &config(32), Tol::witness()).expect("builds");
    assert!(verdict.certified().is_empty(), "{:?}", verdict.receipt());
    let refused =
        sensitivities(&doc, measure, None, Some(&verdict), false, Tol::witness()).expect("ok");
    match entry(&refused, "depth") {
        SensitivityOutcome::Derivative { chamber, .. } => assert_eq!(*chamber, Chamber::LocalOnly),
        other => panic!("{other:?}"),
    }
    // The refusal carries what the run produced: the nominal, the
    // `LocalOnly` sensitivities, and the drive's accounting and
    // receipt — a real study's answer is legible from it alone.
    match stackup(
        &doc,
        measure,
        &analyzed,
        &verdict,
        None,
        false,
        Tol::witness(),
    ) {
        Err(StackupRefusal::NothingCertified {
            nominal,
            sensitivities,
            coverage,
            receipt,
        }) => {
            assert_eq!(nominal.to_bits(), 1.0f64.to_bits());
            assert_eq!(sensitivities, refused);
            assert_eq!(&*coverage, verdict.accounting());
            assert_eq!(receipt, verdict.receipt());
            assert!(receipt.holds() && receipt.certified == 0);
        }
        other => panic!("a macroscopic box certifies nothing today: {other:?}"),
    }
    // The control: a box the drive certifies marks the same parameter
    // chamber-certified by a leaf holding the nominal.
    let (doc, measure) = slab(eps() / 16.0);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(64), Tol::witness()).expect("builds");
    let certified =
        sensitivities(&doc, measure, None, Some(&verdict), false, Tol::witness()).expect("ok");
    match entry(&certified, "depth") {
        SensitivityOutcome::Derivative { chamber, .. } => assert!(contains_nominal(chamber)),
        other => panic!("{other:?}"),
    }
}

/// **Claim 2 (D9) — schedule independence.** The driver and the report
/// are bit-identical under the parallel and the sequential schedule.
#[test]
fn the_driver_and_the_report_are_schedule_independent() {
    let half = eps() / 8.0;
    let (doc, measure, _) = plate(Some(uniform(half)), Some(uniform(half)));
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(1024), Tol::witness()).expect("builds");
    let seq = sensitivities(&doc, measure, None, Some(&verdict), false, Tol::witness());
    let par = sensitivities(&doc, measure, None, Some(&verdict), true, Tol::witness());
    assert_eq!(seq, par);
    let seq = stackup(
        &doc,
        measure,
        &analyzed,
        &verdict,
        None,
        false,
        Tol::witness(),
    );
    let par = stackup(
        &doc,
        measure,
        &analyzed,
        &verdict,
        None,
        true,
        Tol::witness(),
    );
    assert_eq!(seq, par);
    assert!(seq.is_ok());
}

/// A measure that refuses through its own typed doors is a PER-ENTRY
/// refusal, never a driver failure; a node that is not a measure is
/// the driver's own typed refusal.
#[test]
fn a_refusing_measure_is_a_per_entry_refusal_not_a_driver_failure() {
    let (doc, _, _) = plate(None, None);
    // A pair the v1 table has no closed form for: a cylinder wall
    // against a plane cap.
    let ev = eval(&doc);
    // The extrudes in document order: the plate, then one per hole.
    // Taken by kind rather than by literal id — the sketch frame is a
    // node too, so counting positions no longer finds them.
    let extrudes: Vec<_> = doc
        .order()
        .iter()
        .copied()
        .filter(|&id| matches!(doc.node(id), Some(Node::Extrude { .. })))
        .collect();
    let plate_node = extrudes[0];
    let hole = extrudes[1];
    let mut walls = editor_core::select_where(
        &ev,
        hole,
        &editor_core::Selector::of(editor_core::NamePat::of_kind(editor_core::EntityKind::Face)),
        &[editor_core::GeomPred::SurfaceKind(
            editor_core::SurfaceKindSet::just(geom_brep::SurfaceKind::Cylinder),
        )],
        &doc.param_env::<f64>(),
        Tol::witness(),
    )
    .expect("exact atom");
    walls.sort();
    let refs = vec![
        MeasureRef::new(hole, walls.remove(0)),
        MeasureRef::new(plate_node, fname(plate_node, wall(0))),
    ];
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::measure(
                MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
                refs,
            )
            .expect("indices in range"),
        },
    );
    let unsupported = *doc.order().last().expect("inserted");
    let entries = sensitivities(&doc, unsupported, None, None, false, Tol::witness())
        .expect("a refusing measure is not a driver failure");
    assert_eq!(entries.len(), 2);
    for e in &entries {
        assert!(
            matches!(e.outcome, SensitivityOutcome::MeasureRefused { .. }),
            "{e:?}"
        );
    }
    assert_eq!(
        sensitivities(&doc, plate_node, None, None, false, Tol::witness()).err(),
        Some(SensitivityRefusal::NotAMeasure { node: plate_node })
    );
}

/// A verdict driven over another document's parameters, or an analyzed
/// box that is not the verdict's root, refuse typed rather than
/// marking or pricing anything.
#[test]
fn a_foreign_verdict_or_box_refuses_typed() {
    let (doc, measure, _) = plate(Some(uniform(eps() / 8.0)), None);
    let (other, _) = square(1.0, uniform(0.5));
    let foreign = drive(
        &other,
        &analyzed_box(&other, &AnalysisPolicy::default()),
        &config(16),
        Tol::witness(),
    )
    .expect("builds");
    assert_eq!(
        sensitivities(&doc, measure, None, Some(&foreign), false, Tol::witness()).err(),
        Some(SensitivityRefusal::ForeignVerdict)
    );
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(256), Tol::witness()).expect("builds");
    let (wider, _, _) = plate(Some(uniform(eps() / 4.0)), None);
    let other_box = analyzed_box(&wider, &AnalysisPolicy::default());
    assert_eq!(
        stackup(
            &doc,
            measure,
            &other_box,
            &verdict,
            None,
            false,
            Tol::witness()
        )
        .err(),
        Some(StackupRefusal::ForeignBox)
    );
}

/// **The verdict is tied to the build by content.** A value edit
/// between the drive and the driver, and a wholly different document
/// with the SAME parameter names, nominals and distributions, are
/// both refused `VerdictNotOfThisBuild` — in the driver and in the
/// report — because the verdict's certified leaf replays over the new
/// document with a different content key at the first node whose bits
/// moved. A verdict driven over a DIFFERENT box of the SAME document
/// is accepted by the driver: its leaf is of this build, holds the
/// nominal, and is a true certificate over itself — pairing a box's
/// spreads with a verdict's leaves is the report's check.
#[test]
fn a_stale_or_foreign_verdict_is_refused_by_content() {
    let half = eps() / 8.0;
    let (doc, measure, _) = plate(Some(uniform(half)), Some(uniform(half)));
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(1024), Tol::witness()).expect("builds");
    assert!(!verdict.certified().is_empty());

    // A value edit the box cannot see: the nominal moves, the offsets
    // do not.
    let edited = push(
        &doc,
        &DocEdit::SetDocParamValue {
            name: name("hole_r"),
            value: DocParamValue::Continuous(0.21),
        },
    );
    let edited_box = analyzed_box(&edited, &AnalysisPolicy::default());
    assert_eq!(editor_core::ParamBox::of(&edited_box), *verdict.root());
    match sensitivities(
        &edited,
        measure,
        None,
        Some(&verdict),
        false,
        Tol::witness(),
    ) {
        Err(SensitivityRefusal::VerdictNotOfThisBuild {
            node,
            recorded,
            replayed,
            ..
        }) => {
            assert!(edited.node(node).is_some());
            assert_ne!(recorded, replayed);
        }
        other => panic!("a stale verdict must be refused by content: {other:?}"),
    }
    assert!(matches!(
        stackup(
            &edited,
            measure,
            &edited_box,
            &verdict,
            None,
            false,
            Tol::witness()
        ),
        Err(StackupRefusal::Sensitivity(
            SensitivityRefusal::VerdictNotOfThisBuild { .. }
        ))
    ));

    // A foreign document with the SAME names, nominals and
    // distributions — only the hole spacing differs.
    let (other, other_measure, _) = plate_spaced(0.35, Some(uniform(half)), Some(uniform(half)));
    let other_box = analyzed_box(&other, &AnalysisPolicy::default());
    assert_eq!(editor_core::ParamBox::of(&other_box), *verdict.root());
    assert!(matches!(
        sensitivities(
            &other,
            other_measure,
            None,
            Some(&verdict),
            false,
            Tol::witness()
        ),
        Err(SensitivityRefusal::VerdictNotOfThisBuild { .. })
    ));
    assert!(matches!(
        stackup(
            &other,
            other_measure,
            &other_box,
            &verdict,
            None,
            false,
            Tol::witness()
        ),
        Err(StackupRefusal::Sensitivity(
            SensitivityRefusal::VerdictNotOfThisBuild { .. }
        ))
    ));

    // The same document, driven over a different (narrower) box: its
    // leaves are of this build and hold the nominal — accepted by the
    // driver, marked by that drive's leaf.
    let (narrow, narrow_measure, _) = plate(Some(uniform(half / 2.0)), Some(uniform(half / 2.0)));
    let narrow_box = analyzed_box(&narrow, &AnalysisPolicy::default());
    let narrow_verdict =
        drive(&narrow, &narrow_box, &config(1024), Tol::witness()).expect("builds");
    assert_ne!(*narrow_verdict.root(), *verdict.root());
    let marked = sensitivities(
        &doc,
        measure,
        None,
        Some(&narrow_verdict),
        false,
        Tol::witness(),
    )
    .expect("a leaf of this build holding the nominal is a certificate over itself");
    assert!(matches!(
        entry(&marked, "hole_r"),
        SensitivityOutcome::Derivative { chamber, .. } if contains_nominal(chamber)
    ));
    let _ = narrow_measure;
    // And the report refuses the mismatched pairing of spreads and
    // leaves, typed.
    assert_eq!(
        stackup(
            &doc,
            measure,
            &analyzed,
            &narrow_verdict,
            None,
            false,
            Tol::witness()
        )
        .err(),
        Some(StackupRefusal::ForeignBox)
    );
}

/// **The lift-pinning e2e: a bore/pin fit.** The pin's radius `r` is a
/// profile dimension (a circle program's radius) and the C5 gap
/// `R − r − d` reads it only through the lifted cylinder carrier: the
/// stackup reports ∂gap/∂r = −1 exactly, chamber-certified, where the
/// pinned lift gives the silent zero. (The two-hole plate's −2 is the
/// measure EXPRESSION's own `− 2·hole_r` and would come out under
/// either lift; this row is the one where the lift is load-bearing.)
#[test]
fn the_bore_pin_gap_stackup_pins_the_lift() {
    let half = eps() / 16.0;
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("r"),
        value: continuous(Dimension::Length, 0.2, Some(uniform(half))),
    });
    // One frame, named by the bore and the pin alike: they are drawn
    // on the same plane, so they bind the same id.
    let frame = r.insert(fixture::xy_frame());
    let bore_p = r.insert(Node::Profile(ProfileProgram {
        plane: frame,
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
        plane: frame,
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
    let measure = r.insert(
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 0, inner: 1 }),
            refs,
        )
        .expect("indices in range"),
    );
    let doc = r.doc;

    // The silent zero the driver never uses, shown beside the fix.
    let pinned = evaluate::<geom_core::Dual64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions {
            seed: Some(name("r")),
            ..EvalOptions::default()
        },
        Tol::witness(),
    );
    let Some(NodeResult::Ok(v)) = pinned.result(measure) else {
        panic!("the gap measures")
    };
    let ValuePayload::Measure { value, .. } = &v.payload else {
        panic!("a measure")
    };
    assert_eq!(value.deriv, 0.0, "the pinned lift's silent zero");

    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(256), Tol::witness()).expect("builds");
    assert!(!verdict.certified().is_empty(), "{:?}", verdict.receipt());
    let report = stackup(
        &doc,
        measure,
        &analyzed,
        &verdict,
        None,
        false,
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("{e}"));
    assert!(
        (report.nominal - 0.2).abs() < 1e-12,
        "gap {}",
        report.nominal
    );
    match &report.per_param[0].sensitivity {
        SensitivityOutcome::Derivative { value, chamber } => {
            assert_eq!(
                value.to_bits(),
                (-1.0f64).to_bits(),
                "∂gap/∂r through the lift"
            );
            assert!(contains_nominal(chamber));
        }
        other => panic!("{other:?}"),
    }
    let wc = report.worst_case;
    assert!(
        wc.lo <= report.nominal - half && report.nominal + half <= wc.hi,
        "{wc:?}"
    );
}

/// **MAJ-2's valve.** A loft's section stays `f64` in every lane
/// (C6/D9), so a seed on the section's width has no channel to ride:
/// the driver reports `Unliftable { PinnedSection }` naming the section
/// and the parameter — never a finite zero — and the report forfeits
/// that row's advisory columns to it while `worst_case` still gates.
#[test]
fn a_loft_section_seed_is_the_typed_valve_never_a_zero() {
    let half = eps() / 16.0;
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("w"),
        value: continuous(Dimension::Length, 2.0, Some(uniform(half))),
    });
    // A frame per section height: the sections are drawn on DIFFERENT
    // planes, so they are different nodes.
    let frame_at = |r: &mut Recorder, z: f64| {
        r.insert(Node::Datum(editor_core::Datum::Frame {
            origin: [len(0.0), len(0.0), len(z)],
            u: [scl(1.0), scl(0.0), scl(0.0)],
            v: [scl(0.0), scl(1.0), scl(0.0)],
        }))
    };
    let section = |plane| {
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::Chain(vec![
                editor_core::ProgramStep::At([len(0.0), len(0.0)]),
                editor_core::ProgramStep::LineTo(editor_core::ProgramTarget::Point([
                    param("w", Dimension::Length),
                    len(0.0),
                ])),
                editor_core::ProgramStep::LineTo(editor_core::ProgramTarget::Point([
                    param("w", Dimension::Length),
                    len(1.0),
                ])),
                editor_core::ProgramStep::LineTo(editor_core::ProgramTarget::Point([
                    len(0.0),
                    len(1.0),
                ])),
                editor_core::ProgramStep::LineTo(editor_core::ProgramTarget::Start),
            ])],
        })
    };
    let f0 = frame_at(&mut r, 0.0);
    let p0 = r.insert(section(f0));
    let f1 = frame_at(&mut r, 1.0);
    let p1 = r.insert(section(f1));
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
    let measure = r.insert(
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            refs,
        )
        .expect("indices in range"),
    );
    let doc = r.doc;

    let entries = sensitivities(&doc, measure, None, None, false, Tol::witness()).expect("ok");
    match entry(&entries, "w") {
        SensitivityOutcome::Unliftable { node, refusal } => {
            assert_eq!(*node, loft, "the loft is where the seed stops");
            assert_eq!(
                *refusal,
                editor_core::LiftRefusal::PinnedSection {
                    section: p0,
                    param: name("w"),
                }
            );
        }
        other => panic!("a section seed must be the typed valve: {other:?}"),
    }

    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(256), Tol::witness()).expect("builds");
    if verdict.certified().is_empty() {
        // EVIDENCE-ONLY: the loft's interval certification is not
        // this row's subject; the valve above is.
        println!("loft drive certified nothing: {:?}", verdict.receipt());
        return;
    }
    let report = stackup(
        &doc,
        measure,
        &analyzed,
        &verdict,
        None,
        false,
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("{e}"));
    let row = &report.per_param[0];
    assert!(matches!(
        row.sensitivity,
        SensitivityOutcome::Unliftable { .. }
    ));
    assert_eq!(
        row.contribution,
        Err(Unavailable::Unliftable { param: name("w") })
    );
    assert!(row.chamber_span.is_none());
    assert_eq!(
        report.rss,
        Rss::UnavailableBecause {
            blockers: vec![Unavailable::Unliftable { param: name("w") }]
        }
    );
    assert!(report.worst_case.lo <= 2.0 && 2.0 <= report.worst_case.hi);
}

/// **DATUM — an undistributed parameter is a point mass in the RSS.**
/// `depth` with no distribution has σ = 0 exactly: its term is zero and
/// it does NOT block the column (E2's opt-in rule read literally —
/// fixed is a modelling statement, not a missing spread). Disclosed as
/// a deviation from E5's "every contributor carries a measure" read
/// strictly; pinned here so the reading is a datum, not an accident.
#[test]
fn an_undistributed_parameter_is_a_point_mass_in_the_rss() {
    let half = eps() / 8.0;
    let (doc, measure, _) = plate(Some(uniform(half)), None);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    assert_eq!(analyzed.axis_std_deviation(&name("depth")), Some(Ok(0.0)));
    let verdict = drive(&doc, &analyzed, &config(1024), Tol::witness()).expect("builds");
    let report = stackup(
        &doc,
        measure,
        &analyzed,
        &verdict,
        None,
        false,
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("{e}"));
    let sigma_r = (2.0 * half) / f64::sqrt(12.0);
    match report.rss {
        Rss::Advisory { sigma } => assert!((sigma - 2.0 * sigma_r).abs() <= 1e-9 * sigma_r),
        other => panic!("a fixed parameter must not block the RSS: {other:?}"),
    }
    let depth = report
        .per_param
        .iter()
        .find(|p| p.param == name("depth"))
        .expect("depth row");
    assert_eq!(depth.contribution, Ok(0.0));
}
