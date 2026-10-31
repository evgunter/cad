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
//! binding red on a planted stale build (DUAL-DESIGN DL3), no third state (§5),
//! E9 forfeiture on a domain-edge kink (§5), the curvature case where
//! the hull and the linearized sum part company (claim 6), and RSS
//! totality over bands (claim 7).
//!
//! # The widths were in ε, and the symbolic tier moved them
//!
//! Through M10-9 every box below that had to CERTIFY was sized as a
//! small multiple of ε, for the reason `m10_3_driver_interval.rs`
//! measures: the certification predicates are identities whose
//! interval enclosure widens with the box, and a leaf went definite
//! only once its own width was a fraction of ε; a macroscopic box
//! refused all of its mass as `Budget` and the stackup over a real
//! study refused `NothingCertified`. Under M10-10's tier (rule D with
//! amendment A1) the plate's whole-certifying half-width is a REAL
//! margin at about 0.018 (`m10_10_evidence_interval::m10_10_the_stackup_hulls_under_both_rule_sets`
//! with `CAD_M10_10_CEILINGS`), so the `ε/8` rows certify in ONE leaf
//! and the rows whose subject is a split (the band-totality row) are
//! scaled to a real ±0.05 study. The no-third-state row drives its
//! slab with the tier OFF (its subject is the refusal's shape, and
//! since E12 that box certifies with the tier on), which the row says
//! at the site.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::time::Instant;

use editor_core::UnitSym;
use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, SymbolicDials, drive};
use editor_core::stackup::{
    BindingViolation, Chamber, DivergedAt, LiftRefusal, RecordReading, Rss, Sensitivity,
    SensitivityOutcome, SensitivityRefusal, StackupRefusal, Unavailable, sensitivities, stackup,
};
use editor_core::{
    AssertionRelation, AssertionVerdict, CancelToken, CapEnd, Dimension, Distribution, DocEdit,
    EvalOptions, Evaluation, Formula, FreeValue, FreeVar, LoopProgram, MeasurePrimitive, Node,
    NodeResult, ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg, SitedRef, ValuePayload, VarId,
    VarName, evaluate,
};
use geom_core::Tol;

use fixture::{Recorder, fname, len, wall};

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
/// therefore proportional to the width of the LEAF each enclosure is
/// taken over, not to the machine epsilon — and the hull is the union
/// of the certified leaves' enclosures, so a tier that certifies the
/// study in fewer, wider leaves reports a wider hull at the same leaf
/// budget. Under A0 alone this `ε/8` study certified in 16 leaves and
/// the padding was `2·half`; under the form-level algebra (rule D with
/// A/B per node) in 4 leaves of twice the width with padding `4·half`;
/// with amendment A1 (the chart phase folds too) the whole box is ONE
/// leaf and the padding is `8·half` — four times the leaf's width
/// each time, exactly, plus the rounding below
/// (`m10_10_evidence_interval::m10_10_the_stackup_hulls_under_both_rule_sets`
/// prints all three; `work/props/certified-hull-padding-is-the-leaf-width-not-the-lane`
/// is the row). Since the circles store their authored carrier, each
/// radius enters the web's enclosure once, so the one leaf's hull IS
/// the true range and the padding is the rounding alone. A bound, not a
/// target — if it grows, the question is which leaves widened.
const PLATE_PADDING_PER_HALF_WIDTH: f64 = 0.0;
/// The rounding on top of the dependency padding: a 0.2-scale quantity
/// through a few dozen outward-rounded operations (measured ~1e-15).
const PLATE_ROUNDING: f64 = 1.0e-14;
/// The degraded-tangent row's hull rounding, as a fraction of the
/// half-width (measured 1.9e-3 at the 1e-12 row, where it is largest;
/// zero to four decimals at the wider rows).
const DEGRADED_HULL_ROUNDING_PER_HALF_WIDTH: f64 = 1.0e-2;

/// The variable `doc` declares as `name`, or an id it never minted.
fn v(doc: &editor_core::ProfileDoc, name: &str) -> editor_core::VarId {
    doc.var_named(name).unwrap_or(editor_core::VarId::new(0, 0))
}

fn eps() -> f64 {
    Tol::witness().eps()
}

fn name(n: &'static str) -> VarName {
    VarName::from_static(n)
}

fn param(n: &'static str, dim: Dimension) -> Formula {
    Formula::named(name(n), dim)
}

pub(crate) fn uniform(half: f64) -> Distribution {
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

fn continuous(dim: Dimension, value: f64, distribution: Option<Distribution>) -> FreeVar {
    FreeVar::Continuous {
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

/// The build of record a sensitivity report binds to: a build-path
/// evaluation asked for its content pin.
fn record(doc: &ProfileDoc) -> Evaluation<f64> {
    record_with(doc, EvalOptions::default())
}

/// [`record`] over `opts`.
fn record_with(doc: &ProfileDoc, opts: EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions { pin: true, ..opts },
        Tol::witness(),
    )
}

fn push(doc: &editor_core::ProfileDoc, edit: &DocEdit<ProfileProgram>) -> ProfileDoc {
    editor_core::apply(doc, edit, Tol::witness())
        .unwrap_or_else(|e| panic!("edit refused: {e}"))
        .doc
}

/// The variable `doc` declares as `n`.
fn var(doc: &ProfileDoc, n: &str) -> editor_core::VarId {
    doc.var_named(n).expect("the fixture declares it")
}

/// `doc`'s variable `n` as a refusal speaks it.
fn spoken(doc: &ProfileDoc, n: &str) -> editor_core::SpokenVar {
    doc.spoken_var(var(doc, n))
}

fn entry<'a>(
    doc: &ProfileDoc,
    entries: &'a [Sensitivity],
    n: &'static str,
) -> &'a SensitivityOutcome {
    &entries
        .iter()
        .find(|s| s.param == var(doc, n))
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
fn cyl_wall(ev: &Evaluation<f64>, doc: &ProfileDoc, node: RecipeNodeId) -> SitedRef {
    let mut faces = editor_core::select_where(
        ev,
        node,
        &editor_core::Selector::of(editor_core::NamePat::of_kind(editor_core::EntityKind::Face)),
        &[editor_core::GeomPred::SurfaceKind(
            editor_core::SurfaceKindSet::just(geom::SurfaceKind::Cylinder),
        )],
        &doc.var_env::<f64>(),
        Tol::witness(),
    )
    .expect("the surface-kind atom is exact");
    faces.sort();
    SitedRef::new(node, faces.remove(0))
}

/// The vertex of `node`'s body at `at`, by name.
fn vertex_at(ev: &Evaluation<f64>, node: RecipeNodeId, at: [f64; 3]) -> SitedRef {
    let v = editor_core::all_vertices(ev, node)
        .into_iter()
        .find(|v| {
            let p = editor_core::vertex_position(ev, node, v).expect("a vertex");
            p.x == at[0] && p.y == at[1] && p.z == at[2]
        })
        .unwrap_or_else(|| panic!("node {node:?} has a vertex at {at:?}"));
    SitedRef::new(node, v)
}

/// **The two-hole plate** (M10-2's e2e document, distributions from
/// M10-1): a plate whose extrude distance is `depth` and two hole tools
/// whose radius is `hole_r` — a PROFILE dimension, reaching the lane
/// only through the guided lift — with the web measure
/// `distance(wall, wall) − 2·hole_r` and an assertion on it. Returns
/// the document, the measure node and the assertion node.
pub(crate) fn plate(
    radius: Option<Distribution>,
    depth: Option<Distribution>,
) -> (ProfileDoc, VarId, RecipeNodeId) {
    plate_spaced(HOLE_X, radius, depth)
}

/// [`plate`] with the hole centres at `±hole_x` — the same parameter
/// set, nominals and distributions, different geometry.
fn plate_spaced(
    hole_x: f64,
    radius: Option<Distribution>,
    depth: Option<Distribution>,
) -> (ProfileDoc, VarId, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: name("hole_r"),
        def: editor_core::VarDecl::Free(continuous(Dimension::Length, R0, radius)),
    });
    r.push(DocEdit::DeclareVar {
        name: name("depth"),
        def: editor_core::VarDecl::Free(continuous(Dimension::Length, 0.1, depth)),
    });
    let plate_profile = r.insert(Node::Profile(ProfileProgram {
        loops: vec![
            LoopProgram::polygon([(-1.0, -0.5), (1.0, -0.5), (1.0, 0.5), (-1.0, 0.5)])
                .expect("finite plate corners"),
        ],
        ids: Vec::new(),
        on: None,
    }));
    let _plate = r.insert(Node::Extrude {
        profile: plate_profile.into(),
        distance: param("depth", Dimension::Length),
    });
    let mut holes = Vec::new();
    for cx in [-hole_x, hole_x] {
        let p = r.insert(Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [len(cx), len(0.0)],
                radius: param("hole_r", Dimension::Length),
            }],
            ids: Vec::new(),
            on: None,
        }));
        holes.push(r.insert(Node::Extrude {
            profile: p.into(),
            distance: len(0.1),
        }));
    }
    // The second hole joins the first's space where it stands.
    holes[1] = r.against(holes[0], holes[1], &editor_core::Frame::IDENTITY);
    // The wall names come from the selection door, the way a user gets
    // them: evaluate what is built so far, ask each hole for its
    // cylindrical faces, read at the extrude that owns them.
    let ev = eval(&r.doc);
    let wall_of = |node| cyl_wall(&ev, &r.doc, node);
    let measured = r.measure(
        &[MeasurePrimitive::Distance { a: 0, b: 1 }],
        &[wall_of(holes[0]), wall_of(holes[1])],
    );
    let radius = || param("hole_r", Dimension::Length);
    let web = Formula::sub(
        r.len_of(measured.outputs[0]),
        Formula::add(radius(), radius()).expect("Length + Length"),
    )
    .expect("Length - Length");

    let assertion = r.insert(Node::Assertion {
        value: web,
        bound: len(MIN_WEB),
        relation: AssertionRelation::AtLeast,
    });
    let web = fixture::assertion_value(&r.doc, assertion);
    (r.doc, web, assertion)
}

/// **The curvature case**: a value that is the SQUARE of a scalar
/// parameter, `m = a²`, and nothing else — no geometry, so the drive
/// certifies the whole analyzed box in one leaf and the row is about
/// the report's arithmetic alone.
fn square(nominal: f64, dist: Distribution) -> (ProfileDoc, VarId) {
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: name("a"),
        def: editor_core::VarDecl::Free(continuous(Dimension::Scalar, nominal, Some(dist))),
    });
    let a = || param("a", Dimension::Scalar);
    r.push(DocEdit::DeclareVar {
        name: name("m"),
        def: editor_core::VarDecl::Defined(Formula::mul(a(), a()).expect("Scalar · Scalar")),
    });
    let m = r.doc.resolve_var(&name("m").into()).expect("m is declared");
    (r.doc, m)
}

/// **The domain-edge kink**: a unit cube and its copy translated by
/// `t` along x, measured by the distance between the cube's vertex at
/// (1, 0, 0) and the copy's vertex that lands there at the nominal
/// `t = 1`. The value is `‖(t − 1, 0, 0)‖ = |t − 1|`, zero at the
/// nominal; its `Dual64` tangent is `0/0` through the norm's square
/// root — a degraded tangent under a perfectly finite value.
fn kink(dist: Distribution) -> (ProfileDoc, VarId) {
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: name("t"),
        def: editor_core::VarDecl::Free(continuous(Dimension::Length, 1.0, Some(dist))),
    });
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
                .expect("finite corners"),
        ],
        ids: Vec::new(),
        on: None,
    }));
    let cube = r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(1.0),
    });
    let copy = crate::m10_5_clearance_interval::slid_by(
        &mut r,
        cube,
        cube,
        [
            param("t", Dimension::Length),
            Formula::zero(Dimension::Length),
            Formula::zero(Dimension::Length),
        ],
    );
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
        SitedRef::new(cube, at(cube, 1.0)),
        SitedRef::new(copy, at(copy, 1.0)),
    ];
    let measured = r.measure(&[MeasurePrimitive::Distance { a: 0, b: 1 }], &refs);
    (r.doc, measured.outputs[0])
}

/// **The slab with a measured thickness**: a unit square extruded by
/// `depth`, measured cap to cap — a magnitude-slot parameter whose
/// sensitivity is exactly 1.
fn slab(half: f64) -> (ProfileDoc, VarId) {
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: name("depth"),
        def: editor_core::VarDecl::Free(continuous(Dimension::Length, 1.0, Some(uniform(half)))),
    });
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
                .expect("finite corners"),
        ],
        ids: Vec::new(),
        on: None,
    }));
    let block = r.insert(Node::Extrude {
        profile: p.into(),
        distance: param("depth", Dimension::Length),
    });
    let refs = vec![
        SitedRef::new(block, fname(block, RoleSeg::Cap(CapEnd::Start))),
        SitedRef::new(block, fname(block, RoleSeg::Cap(CapEnd::End))),
    ];
    let measured = r.measure(&[MeasurePrimitive::Distance { a: 0, b: 1 }], &refs);
    (r.doc, measured.outputs[0])
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
    let handed = record(&doc);
    let t1 = Instant::now();
    let report = stackup(
        &doc,
        measure,
        &analyzed,
        &verdict,
        Some(&handed),
        false,
        None,
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

    // The human form opens on the measured value, spoken from the
    // document the stackup was taken of (a rename keeps its identity).
    let named = push(
        &doc,
        &DocEdit::RenameVar {
            var: measure.into(),
            name: Some(name("web")),
        },
    );
    let rendered = report.render(&named, &analyzed);
    assert_eq!(
        rendered.lines().next(),
        Some("stackup of web"),
        "{rendered}"
    );
    let distance = doc
        .operation_of(doc.observed_outputs(measure)[0])
        .expect("the web reads a measure");

    // The goldening form prints a sensitivity's nodes and its variable
    // by full id, never the tag and never a spoken node or name. The
    // row is found by its variable: rows sort by id, not by name.
    let hole_r = var(&doc, "hole_r");
    let mut unliftable = report.clone();
    unliftable
        .per_param
        .iter_mut()
        .find(|p| p.param == hole_r)
        .expect("hole_r has a row")
        .sensitivity = SensitivityOutcome::Unliftable {
        node: distance,
        refusal: LiftRefusal::GuidedReplay { loop_: 0, step: 1 },
    };
    let golden = unliftable.serialize();
    assert!(
        golden.contains(&format!(
            "sensitivity=unliftable at node {}: the guided elaboration could not re-confirm \
             loop 0 step 1 at this pass's scalar",
            distance.full(),
        )),
        "{golden}"
    );
    assert!(!golden.contains("Measure "), "{golden}");
    assert!(!golden.contains("hole_r"), "{golden}");
    // The human form SPEAKS the node: by its spoken name, where the
    // content-keyed goldening form above holds its identity.
    let human = unliftable.render(&doc, &analyzed);
    assert!(
        human.contains(&format!(
            "unliftable at {}: the guided elaboration could not re-confirm loop 0 step 1",
            doc.spoken(distance)
        )),
        "{human}"
    );

    // The nominal: the plate's own formula, 2·0.30 − 2·0.2.
    assert!(
        (report
            .nominal
            .expect("this fixture measures a closed form, which has an f64 nominal")
            - 0.2)
            .abs()
            < 1e-12,
        "nominal {}",
        report
            .nominal
            .expect("this fixture measures a closed form, which has an f64 nominal")
    );
    assert_eq!(report.measurement, measure);

    // The sensitivities, analytically: −2 on the radius (each hole
    // wall moves inward by dr, the axis separation does not move), 0
    // on the depth (the web is a planar quantity). Exact, and marked.
    let rows: Vec<Sensitivity> = report
        .per_param
        .iter()
        .map(|p| Sensitivity {
            document: doc.id(),
            param: p.param,
            outcome: p.sensitivity.clone(),
        })
        .collect();
    match entry(&doc, &rows, "hole_r") {
        SensitivityOutcome::Derivative { value, chamber } => {
            assert_eq!(value.to_bits(), (-2.0f64).to_bits(), "∂web/∂hole_r");
            assert!(contains_nominal(chamber), "{chamber:?}");
        }
        other => panic!("hole_r: {other:?}"),
    }
    match entry(&doc, &rows, "depth") {
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
        let expect = if p.param == var(&doc, "hole_r") {
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
        wc.lo
            <= report
                .nominal
                .expect("this fixture measures a closed form, which has an f64 nominal")
                - 2.0 * half
            && report
                .nominal
                .expect("this fixture measures a closed form, which has an f64 nominal")
                + 2.0 * half
                <= wc.hi,
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
    // Pinned at BOTH ends and with the leaf count, so the pin is
    // monotone the right way: a regression to more, narrower leaves
    // (16 leaves, `2·half`) would pass a one-sided ceiling and hide
    // the tier's reach falling (R1 MIN-6, R2 m2). One leaf, the whole
    // box, and no dependency padding on it.
    assert_eq!(
        wc.leaves,
        1,
        "under M10-10's tier the ε/8 study is ONE leaf: {:?}",
        verdict.receipt()
    );
    assert!(
        (padding - PLATE_PADDING_PER_HALF_WIDTH * half).abs() <= PLATE_ROUNDING,
        "padding {padding:e} is not the measured {PLATE_PADDING_PER_HALF_WIDTH}·half \
         (leaves {}): the leaves moved",
        wc.leaves
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
///
/// At ±0.05 on both axes — a real study — rather than `ε/8`: under
/// M10-10's tier the `ε/8` box certifies in ONE leaf, and a leaf that
/// covers a band's whole support prices as 1 by the band's own rule
/// (`box_mass`), so the coverage refusal this row also asserts needs a
/// leaf that covers PART of a band, i.e. a drive that split. The
/// fixture's whole-certifying half-width is about 0.018 at the default
/// ε (`m10_10_evidence_interval::m10_10_the_stackup_hulls_under_both_rule_sets`
/// with `CAD_M10_10_CEILINGS`), so ±0.05 splits at every row.
#[test]
fn a_band_contributor_refuses_the_rss_whole_naming_every_band() {
    let half = 0.05;
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
        None,
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("the stackup refused: {e}"));
    match &report.rss {
        Rss::UnavailableBecause { blockers } => {
            let named: Vec<editor_core::VarId> = blockers.iter().map(|b| b.var().id()).collect();
            // Declaration order: the plate declares `hole_r` first.
            let want = vec![var(&doc, "hole_r"), var(&doc, "depth")];
            assert_eq!(named, want, "{blockers:?}");
            assert!(
                blockers
                    .iter()
                    .all(|b| matches!(b, Unavailable::BandHasNoMeasure { .. }))
            );
        }
        other => panic!("rss over two bands: {other:?}"),
    }
    assert!(report.per_param.iter().all(|p| p.contribution.is_ok()));
    assert!(
        report.worst_case.lo
            <= report
                .nominal
                .expect("this fixture measures a closed form, which has an f64 nominal")
            && report
                .nominal
                .expect("this fixture measures a closed form, which has an f64 nominal")
                <= report.worst_case.hi
    );
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
        None,
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
    let linearized_hi = report
        .nominal
        .expect("this fixture measures a closed form, which has an f64 nominal")
        + contribution;
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
    let entries = sensitivities(
        &doc,
        measure,
        None,
        Some(&verdict),
        false,
        None,
        Tol::witness(),
    )
    .expect("no refusal");
    match entry(&doc, &entries, "t") {
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
        None,
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("E9: tangent state reached a refusal: {e}"));
    assert_eq!(
        report
            .nominal
            .expect("this fixture measures a closed form, which has an f64 nominal")
            .to_bits(),
        0.0f64.to_bits()
    );
    let row = &report.per_param[0];
    assert!(matches!(
        row.sensitivity,
        SensitivityOutcome::TangentDegraded { .. }
    ));
    assert_eq!(
        row.contribution,
        Err(Unavailable::TangentDegraded {
            var: spoken(&doc, "t")
        })
    );
    assert_eq!(
        report.rss,
        Rss::UnavailableBecause {
            blockers: vec![Unavailable::TangentDegraded {
                var: spoken(&doc, "t")
            }]
        }
    );
    let wc = report.worst_case;
    // The measure is the norm itself, so the hull's top is the
    // half-width plus the interval lane's rounding only. Stated as a
    // fraction of the half-width so the bound means the same thing at
    // every CI ε row (measured hi/half: 1.0000 at the default row and
    // at 1e-6, 1.0019 at 1e-12); an absolute slack would say nothing at
    // the tight rows (issue 1646).
    println!(
        "EVIDENCE-ONLY degraded-tangent worst case: {wc:?} against half {half:e} \
         (hi/half {:.4})",
        wc.hi / half
    );
    assert!(
        wc.lo >= 0.0 && wc.hi <= half * (1.0 + DEGRADED_HULL_ROUNDING_PER_HALF_WIDTH),
        "{wc:?}"
    );
    assert_eq!(wc.leaves, verdict.certified().len());
}

/// **Claim 3 — the binding is red-capable.** A handed build of record
/// that read other inputs than the passes — another revision (a value
/// edit, a structural edit), another document, another tolerance, other
/// parameter bits, no pin, a canceled prefix — gets its typed violation
/// and never a sensitivity; a fresh pinned one binds. Every arm but
/// `Parts` is here; `msolve14_run_scalar::a5_sensitivities_cross_a_seated_mate`
/// has that one, over an assembly.
#[test]
fn the_binding_is_red_capable_on_a_stale_build() {
    let (doc, measure, assertion) = plate(None, None);
    // Named, so the web outlives the assertion the structural edit below
    // removes (VR7: an unnamed variable lives as long as its reader).
    let doc = push(
        &doc,
        &DocEdit::RenameVar {
            var: measure.into(),
            name: Some(name("web")),
        },
    );
    let refusal = |doc: &ProfileDoc, handed: &Evaluation<f64>| {
        sensitivities(
            doc,
            measure,
            Some(handed),
            None,
            false,
            None,
            Tol::witness(),
        )
        .err()
    };
    let handed = record(&doc);
    assert_eq!(refusal(&doc, &handed), None, "a fresh pinned build binds");

    // No pin: which revision it read cannot be said.
    let unpinned = eval(&doc);
    let got = refusal(&doc, &unpinned);
    assert_eq!(
        got,
        Some(SensitivityRefusal::Binding(BindingViolation::Unpinned {
            refusal: None
        }))
    );
    assert!(
        got.is_some_and(|r| r.to_string().contains("EvalOptions::pin")),
        "the refusal names the option that pins"
    );

    // Stale by a parameter edit: another revision.
    let edited = push(
        &doc,
        &DocEdit::SetVarValue {
            var: name("hole_r").into(),
            value: FreeValue::Continuous(0.21),
        },
    );
    let got = refusal(&edited, &handed);
    assert_eq!(
        got,
        Some(SensitivityRefusal::Binding(BindingViolation::Revision))
    );
    assert_eq!(
        got.map(|r| r.to_string()),
        Some(
            "binding violation: the f64 build of record read another revision of the \
             document — it changed since that build, so differentiating now would report a \
             sensitivity of a build nobody validated"
                .to_owned()
        )
    );

    // Stale by structure: a node removed.
    let shrunk = push(&doc, &DocEdit::DeleteNode { id: assertion });
    assert_eq!(
        refusal(&shrunk, &handed),
        Some(SensitivityRefusal::Binding(BindingViolation::Revision))
    );

    // Another document's build: refused by identity before its revision
    // is read (DI3).
    let mut foreign = record(&doc);
    let other = editor_core::DocumentId::derive("m10-4-the-binding-foreign");
    foreign.document = other;
    assert_eq!(
        refusal(&doc, &foreign),
        Some(SensitivityRefusal::Binding(
            BindingViolation::EvaluationOfAnotherDocument {
                expected: doc.id(),
                found: other,
            }
        ))
    );

    // Another tolerance. One process holds one ε, so no evaluation of
    // this process decided at another; the stamp is restamped to stand
    // for one that did.
    let stamped = record(&doc).tolerance();
    let coarser = record(&doc).restamped_tolerance(geom_core::Tolerance {
        eps: 2.0 * stamped.eps,
        ..stamped
    });
    assert_eq!(
        refusal(&doc, &coarser),
        Some(SensitivityRefusal::Binding(BindingViolation::Tolerance))
    );

    // Other parameter bits under the same revision: the f64 point
    // sample of a box that moves `hole_r` off its nominal.
    let hole_r = var(&doc, "hole_r");
    let moved = record_with(
        &doc,
        EvalOptions {
            param_box: Some(std::sync::Arc::new(editor_core::ParamBox::from_axes(
                [(hole_r, editor_core::BoxAxis::Varying { lo: 1e-3, hi: 1e-3 })]
                    .into_iter()
                    .collect(),
            ))),
            ..EvalOptions::default()
        },
    );
    assert_eq!(
        refusal(&doc, &moved),
        Some(SensitivityRefusal::Binding(BindingViolation::Binding {
            var: doc.spoken_var(hole_r)
        }))
    );

    // A record that refused before building is no build: it reports
    // itself complete, but its environment is the nominal one whatever
    // it was asked to bind. A seed (an f64 run carries no tangent) and
    // a box of width (an f64 run carries no span) each refuse it whole.
    let seeded = record_with(
        &doc,
        EvalOptions {
            seed: Some(hole_r),
            ..EvalOptions::default()
        },
    );
    assert!(seeded.refused_whole(), "an f64 run refuses a seed whole");
    assert_eq!(
        refusal(&doc, &seeded),
        Some(SensitivityRefusal::Binding(BindingViolation::NotABuild))
    );
    let spanned = record_with(
        &doc,
        EvalOptions {
            param_box: Some(std::sync::Arc::new(editor_core::ParamBox::from_axes(
                [(
                    hole_r,
                    editor_core::BoxAxis::Varying {
                        lo: -1e-3,
                        hi: 1e-3,
                    },
                )]
                .into_iter()
                .collect(),
            ))),
            ..EvalOptions::default()
        },
    );
    assert!(
        spanned.refused_whole(),
        "an f64 run refuses a box of width whole"
    );
    assert_eq!(
        refusal(&doc, &spanned),
        Some(SensitivityRefusal::Binding(BindingViolation::NotABuild))
    );
    assert!(
        !record(&doc).refused_whole() && !moved.refused_whole(),
        "a build, and a point sample, are builds"
    );

    // A canceled prefix validates nothing.
    let cancel = CancelToken::new();
    cancel.cancel();
    let partial = evaluate::<f64>(
        &doc,
        None,
        &cancel,
        &EvalOptions {
            pin: true,
            ..EvalOptions::default()
        },
        Tol::witness(),
    );
    assert_eq!(
        refusal(&doc, &partial),
        Some(SensitivityRefusal::Binding(BindingViolation::Incomplete))
    );
}

/// **The measured value is checked bit for bit.** Every input of the
/// handed build is the passes', and its measured value is perturbed by
/// one ulp: each pass's value half now differs from the build of
/// record's, and the driver refuses `MeasuredValueDrift` naming the
/// first parameter's pass, never a sensitivity.
#[test]
fn a_drifted_measured_value_refuses() {
    let (doc, measure) = slab(eps() / 8.0);
    let mut handed = record(&doc);
    let (node, _) = handed
        .read_port(measure)
        .expect("the thickness is a measure's output");
    let measured = match handed.nodes.get_mut(&node) {
        Some(NodeResult::Ok(v)) => match &mut v.payload {
            ValuePayload::Measure { value, .. } => value,
            other => panic!("a measure's payload: {other:?}"),
        },
        other => panic!("the measure built: {other:?}"),
    };
    // The pass measures the true value; the record is moved one ulp.
    let pass = *measured;
    *measured = f64::from_bits(pass.to_bits() + 1);
    let base = *measured;
    match sensitivities(
        &doc,
        measure,
        Some(&handed),
        None,
        false,
        None,
        Tol::witness(),
    ) {
        Err(refusal @ SensitivityRefusal::MeasuredValueDrift { .. }) => {
            assert_eq!(
                refusal,
                SensitivityRefusal::MeasuredValueDrift {
                    param: spoken(&doc, "depth"),
                    pass,
                    base: RecordReading::Measured(base),
                }
            );
            assert!(refusal.to_string().contains("D9 break"), "{refusal}");
        }
        other => panic!("a drifted measured value must refuse: {other:?}"),
    }
}

/// **A drift on every parameter refuses the same in either schedule.**
/// The plate's record is moved one ulp, so both passes — `hole_r`'s
/// and `depth`'s — drift. The refusal named is the first in parameter
/// order, never whichever pass the parallel schedule finished first
/// (D9): every result is collected before an error is chosen.
#[test]
fn a_drift_on_every_parameter_refuses_the_same_in_either_schedule() {
    let (doc, measure, _) = plate(Some(uniform(1e-3)), Some(uniform(1e-3)));
    let mut handed = record(&doc);
    // Every measure under the web moved one ulp, so the web drifts in
    // every pass.
    let mut moved = 0;
    for result in handed.nodes.values_mut() {
        if let NodeResult::Ok(v) = result
            && let ValuePayload::Measure { value, .. } = &mut v.payload
        {
            *value = f64::from_bits(value.to_bits() + 1);
            moved += 1;
        }
    }
    assert!(moved > 0, "the plate's web reads measures");
    let run = |parallel: bool| {
        sensitivities(
            &doc,
            measure,
            Some(&handed),
            None,
            parallel,
            None,
            Tol::witness(),
        )
        .err()
    };
    let sequential = run(false);
    let first = doc
        .free_vars()
        .find(|(_, free)| editor_core::analysis::is_axis(free))
        .map(|(id, _)| doc.spoken_var(id))
        .expect("the plate has toleranced parameters");
    match &sequential {
        Some(SensitivityRefusal::MeasuredValueDrift { param, .. }) => {
            assert_eq!(*param, first, "the first parameter's pass is named");
        }
        other => panic!("every pass drifts: {other:?}"),
    }
    for _ in 0..16 {
        assert_eq!(
            run(true),
            sequential,
            "the parallel schedule names the same"
        );
    }
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
    let local = sensitivities(&doc, measure, None, None, false, None, Tol::witness()).expect("ok");
    match entry(&doc, &local, "depth") {
        SensitivityOutcome::Derivative { value, chamber } => {
            assert_eq!(value.to_bits(), 1.0f64.to_bits());
            assert_eq!(*chamber, Chamber::LocalOnly);
        }
        other => panic!("{other:?}"),
    }
    // The tier OFF, deliberately: this row is about what a report says
    // when the drive certified NOTHING, and since E12 this fixture's
    // box certifies. `SymbolicDials::off()` is the honest way to ask
    // for a drive that does not — the alternative would be tuning the
    // fixture until the driver failed again, which is a worse pin on
    // the same claim.
    let verdict = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            symbolic: SymbolicDials::off(),
            ..config(32)
        },
        Tol::witness(),
    )
    .expect("builds");
    assert!(verdict.certified().is_empty(), "{:?}", verdict.receipt());
    let refused = sensitivities(
        &doc,
        measure,
        None,
        Some(&verdict),
        false,
        None,
        Tol::witness(),
    )
    .expect("ok");
    match entry(&doc, &refused, "depth") {
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
        None,
        Tol::witness(),
    ) {
        Err(StackupRefusal::NothingCertified {
            nominal,
            sensitivities,
            coverage,
            receipt,
        }) => {
            assert_eq!(
                nominal
                    .expect("a closed-form measure has an f64 nominal")
                    .to_bits(),
                1.0f64.to_bits()
            );
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
    let certified = sensitivities(
        &doc,
        measure,
        None,
        Some(&verdict),
        false,
        None,
        Tol::witness(),
    )
    .expect("ok");
    match entry(&doc, &certified, "depth") {
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
    let seq = sensitivities(
        &doc,
        measure,
        None,
        Some(&verdict),
        false,
        None,
        Tol::witness(),
    );
    let par = sensitivities(
        &doc,
        measure,
        None,
        Some(&verdict),
        true,
        None,
        Tol::witness(),
    );
    assert_eq!(seq, par);
    let seq = stackup(
        &doc,
        measure,
        &analyzed,
        &verdict,
        None,
        false,
        None,
        Tol::witness(),
    );
    let par = stackup(
        &doc,
        measure,
        &analyzed,
        &verdict,
        None,
        true,
        None,
        Tol::witness(),
    );
    assert_eq!(seq, par);
    assert!(seq.is_ok());
}

/// A measure that refuses through its own typed doors is a PER-ENTRY
/// refusal, never a driver failure; a variable that is not a scalar is
/// the driver's own typed refusal.
#[test]
fn a_refusing_measure_is_a_per_entry_refusal_not_a_driver_failure() {
    // Both parameters toleranced, so both are entries (VR8).
    let law = Some(Distribution::Normal { sigma: 1e-5 });
    let (doc, _, _) = plate(law, law);
    // A pair the v1 table has no closed form for: a cylinder wall
    // against a plane cap.
    let ev = eval(&doc);
    // The extrudes in document order: the plate, then one per hole.
    // Taken by kind rather than by literal id — the sketch frame is a
    // node too, so counting positions no longer finds them.
    let extrudes: Vec<_> = doc
        .ids()
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
            editor_core::SurfaceKindSet::just(geom::SurfaceKind::Cylinder),
        )],
        &doc.var_env::<f64>(),
        Tol::witness(),
    )
    .expect("exact atom");
    walls.sort();
    // The hole's wall is read at a copy standing in the plate's space,
    // as a measure's two sites share one.
    let (doc, hole_in) = crate::fixture::stood_in(doc, hole, plate_node);
    let refs = vec![
        SitedRef::new(hole_in, walls.remove(0)),
        SitedRef::new(plate_node, fname(plate_node, wall(&doc, plate_node, 0))),
    ];
    let (doc, measured) =
        fixture::measure(doc, &[MeasurePrimitive::Distance { a: 0, b: 1 }], &refs);
    let unsupported = measured.outputs[0];
    let entries = sensitivities(&doc, unsupported, None, None, false, None, Tol::witness())
        .expect("a refusing measure is not a driver failure");
    assert_eq!(entries.len(), 2);
    for e in &entries {
        assert!(
            matches!(e.outcome, SensitivityOutcome::MeasureRefused { .. }),
            "{e:?}"
        );
    }
    let body = doc.output(plate_node, 0).expect("the plate's body");
    assert_eq!(
        sensitivities(&doc, body, None, None, false, None, Tol::witness()).err(),
        Some(SensitivityRefusal::NotAScalar {
            var: doc.spoken_var(body)
        })
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
        sensitivities(
            &doc,
            measure,
            None,
            Some(&foreign),
            false,
            None,
            Tol::witness()
        )
        .err(),
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
            None,
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
    // do not. Relabelled too, so a node spoken from `doc` instead would
    // say no label.
    let edited = fixture::label_every_node(
        push(
            &doc,
            &DocEdit::SetVarValue {
                var: name("hole_r").into(),
                value: FreeValue::Continuous(0.21),
            },
        ),
        "edited",
    );
    let edited_box = analyzed_box(&edited, &AnalysisPolicy::default());
    assert_eq!(editor_core::ParamBox::of(&edited_box), *verdict.root());
    match &sensitivities(
        &edited,
        measure,
        None,
        Some(&verdict),
        false,
        None,
        Tol::witness(),
    ) {
        Err(
            refusal @ SensitivityRefusal::VerdictNotOfThisBuild {
                node,
                recorded,
                replayed,
                ..
            },
        ) => {
            let DivergedAt::Replayed(spoken) = &**node else {
                panic!("the edited document's replay holds the node: {node:?}");
            };
            assert_eq!(
                *spoken,
                edited.spoken(node.id()),
                "a node the replay holds is spoken from the document asked about"
            );
            assert_eq!(
                spoken.label().map(editor_core::Label::as_str),
                Some("edited")
            );
            assert!(
                refusal
                    .to_string()
                    .contains(&format!("a different content key at {spoken} — ")),
                "{refusal}"
            );
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
            None,
            Tol::witness()
        ),
        Err(StackupRefusal::Sensitivity(
            SensitivityRefusal::VerdictNotOfThisBuild { .. }
        ))
    ));

    // A foreign document with the SAME names, nominals and
    // distributions — only the hole spacing differs.
    let (other, other_measure, _) = plate_spaced(0.35, Some(uniform(half)), Some(uniform(half)));
    let other = fixture::label_every_node(other, "foreign");
    let other_box = analyzed_box(&other, &AnalysisPolicy::default());
    assert_eq!(editor_core::ParamBox::of(&other_box), *verdict.root());
    match &sensitivities(
        &other,
        other_measure,
        None,
        Some(&verdict),
        false,
        None,
        Tol::witness(),
    ) {
        Err(refusal @ SensitivityRefusal::VerdictNotOfThisBuild { node, .. }) => {
            // The record names a node of the document the drive ran
            // on; it is said by tag as the record's, never looked up
            // in the foreign document.
            let DivergedAt::Recorded(id) = &**node else {
                panic!("the foreign replay parts from the record at the record's node: {node:?}");
            };
            assert!(
                refusal.to_string().contains(&format!(
                    "a different content key at the drive record's node {id} — "
                )),
                "{refusal}"
            );
            assert!(!refusal.to_string().contains("foreign"), "{refusal}");
        }
        other => panic!("a foreign verdict must be refused by content: {other:?}"),
    }
    assert!(matches!(
        stackup(
            &other,
            other_measure,
            &other_box,
            &verdict,
            None,
            false,
            None,
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
        None,
        Tol::witness(),
    )
    .expect("a leaf of this build holding the nominal is a certificate over itself");
    assert!(matches!(
        entry(&doc, &marked, "hole_r"),
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
            None,
            Tol::witness()
        )
        .err(),
        Some(StackupRefusal::ForeignBox)
    );
}

/// **The lift-pinning e2e: a bore/pin fit.** The pin's radius `r` is a
/// profile dimension (a circle program's radius) and the C5 gap
/// `R − r − d` reads it only through the lifted cylinder carrier: the
/// stackup reports ∂gap/∂r = −1 exactly, chamber-certified, where a
/// lift through `from_f64` gave the silent zero. (The two-hole plate's
/// −2 is the measure EXPRESSION's own `− 2·hole_r` and would come out
/// without the lift; this row is the one where the lift is
/// load-bearing.)
#[test]
fn the_bore_pin_gap_stackup_pins_the_lift() {
    let half = eps() / 16.0;
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: name("r"),
        def: editor_core::VarDecl::Free(continuous(Dimension::Length, 0.2, Some(uniform(half)))),
    });
    // A plate with a hole: the bore's wall faces into it, so the fit
    // reads `R − r − d`.
    let bore_p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![
            LoopProgram::polygon([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)])
                .expect("finite corners"),
            LoopProgram::Circle {
                centre: [len(0.0), len(0.0)],
                radius: len(0.5),
            },
        ],
        ids: Vec::new(),
        on: None,
    }));
    let bore = r.insert(Node::Extrude {
        profile: bore_p.into(),
        distance: len(1.0),
    });
    let pin_p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::Circle {
            centre: [len(0.1), len(0.0)],
            radius: param("r", Dimension::Length),
        }],
        ids: Vec::new(),
        on: None,
    }));
    let pin = r.insert(Node::Extrude {
        profile: pin_p.into(),
        distance: len(1.0),
    });
    // The pin joins the bore's space where it stands.
    let pin = r.against(bore, pin, &editor_core::Frame::IDENTITY);
    let ev = eval(&r.doc);
    let refs = vec![cyl_wall(&ev, &r.doc, bore), cyl_wall(&ev, &r.doc, pin)];
    let measured = r.measure(&[MeasurePrimitive::Gap { outer: 0, inner: 1 }], &refs);
    let (measure, measure_value) = (measured.measures[0], measured.outputs[0]);
    let doc = r.doc;

    // A default evaluation seeded on `r` carries the tangent: the
    // profile is elaborated at the lane, where a lift through
    // `from_f64` gave a silent zero.
    let seeded = evaluate::<geom_core::Dual64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions {
            seed: Some(var(&doc, "r")),
            ..EvalOptions::default()
        },
        Tol::witness(),
    );
    let Some(NodeResult::Ok(v)) = seeded.result(measure) else {
        panic!("the gap measures")
    };
    let ValuePayload::Measure { value, .. } = &v.payload else {
        panic!("a measure")
    };
    assert_eq!(value.deriv, -1.0, "∂gap/∂r through the profile's carrier");

    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(256), Tol::witness()).expect("builds");
    assert!(!verdict.certified().is_empty(), "{:?}", verdict.receipt());
    let report = stackup(
        &doc,
        measure_value,
        &analyzed,
        &verdict,
        None,
        false,
        None,
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("{e}"));
    assert!(
        (report
            .nominal
            .expect("this fixture measures a closed form, which has an f64 nominal")
            - 0.2)
            .abs()
            < 1e-12,
        "gap {}",
        report
            .nominal
            .expect("this fixture measures a closed form, which has an f64 nominal")
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
        wc.lo
            <= report
                .nominal
                .expect("this fixture measures a closed form, which has an f64 nominal")
                - half
            && report
                .nominal
                .expect("this fixture measures a closed form, which has an f64 nominal")
                + half
                <= wc.hi,
        "{wc:?}"
    );
}

/// **A loft section carries its seed.** A loft reads its sections'
/// profile values in the lane scalar, so a seed on a section's width
/// rides through the skin to the walls: the distance from the loft's
/// origin corner to its `x = w` corner moves one for one with `w`. And
/// the drive over `w`'s box certifies in one leaf, the walls skinned
/// over the box.
#[test]
fn a_loft_section_seed_rides_the_section_to_the_measure() {
    let half = eps() / 16.0;
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: name("w"),
        def: editor_core::VarDecl::Free(continuous(Dimension::Length, 2.0, Some(uniform(half)))),
    });
    // Each section drawn on its own plane; the second's station slides
    // it up the first's z.
    let section = || {
        Node::Profile(ProfileProgram {
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
            ids: Vec::new(),
            on: None,
        })
    };
    let p0 = r.insert(section());
    let p1 = r.insert(section());
    let loft = r.insert(Node::Loft {
        first: p0.into(),
        then: vec![editor_core::LoftSection {
            profile: p1.into(),
            station: vec![editor_core::StationMove::Slide {
                along: editor_core::Axis3::Z,
                by: len(1.0),
            }],
        }],
        v_degree: Formula::count(1),
    });
    let ev = eval(&r.doc);
    if let Some(e) = ev.node_error(loft) {
        panic!("the loft did not build: {}", e.kind);
    }
    let refs = vec![
        vertex_at(&ev, loft, [0.0, 0.0, 0.0]),
        vertex_at(&ev, loft, [2.0, 0.0, 0.0]),
    ];
    let measured = r.measure(&[MeasurePrimitive::Distance { a: 0, b: 1 }], &refs);
    let (_measure, measure_value) = (measured.measures[0], measured.outputs[0]);
    let doc = r.doc;

    let entries =
        sensitivities(&doc, measure_value, None, None, false, None, Tol::witness()).expect("ok");
    match entry(&doc, &entries, "w") {
        SensitivityOutcome::Derivative { value, .. } => {
            assert!(
                (value - 1.0).abs() <= 1e-9,
                "the corner distance moves one for one with the width: {value}"
            );
        }
        other => panic!("a section seed must reach the measure: {other:?}"),
    }

    // The drive is where the box meets the section: at `Interval` the
    // section is wider than a point, and the loft skins it at the lane
    // scalar, so the box certifies in one leaf.
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let verdict = drive(&doc, &analyzed, &config(256), Tol::witness()).expect("builds");
    assert_eq!(
        verdict.receipt(),
        editor_core::drive::Receipt {
            certified: 1,
            refused: 0,
            splits: 0
        },
        "a boxed section certifies at Interval: {:?}",
        verdict.refused().first().map(|l| &l.reason)
    );
}

/// **DATUM — an untoleranced parameter is no axis, and blocks nothing.**
/// `depth` with no tolerance is a constant in every analysis lane
/// (VARIABLES-DESIGN VR8): it is no axis of the analysed box, the
/// stackup lists no row for it, and the RSS over the toleranced axes is
/// available — fixed is a modelling statement, not a missing spread.
/// Breaks if an untoleranced variable is listed, or blocks the column.
#[test]
fn an_untoleranced_parameter_is_no_axis_and_blocks_no_rss() {
    let half = eps() / 8.0;
    let (doc, measure, _) = plate(Some(uniform(half)), None);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    assert_eq!(analyzed.axis_std_deviation(v(&doc, "depth")), None);
    let verdict = drive(&doc, &analyzed, &config(1024), Tol::witness()).expect("builds");
    let report = stackup(
        &doc,
        measure,
        &analyzed,
        &verdict,
        None,
        false,
        None,
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("{e}"));
    let sigma_r = (2.0 * half) / f64::sqrt(12.0);
    match report.rss {
        Rss::Advisory { sigma } => assert!((sigma - 2.0 * sigma_r).abs() <= 1e-9 * sigma_r),
        other => panic!("an untoleranced parameter must not block the RSS: {other:?}"),
    }
    assert!(
        report
            .per_param
            .iter()
            .all(|p| p.param != var(&doc, "depth")),
        "an untoleranced parameter is no stackup row"
    );
}
