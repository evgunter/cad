//! **INTENT stage 5 B — interference between copies is a finding**
//! (`docs/INTENT-STAGE5-SPEC.md` §3, §6 rows 5–10; D10, Assertions).
//!
//! At rest, an overlap of two copies' material is an
//! [`editor_core::InterferenceFinding`] of the at-rest check, one per
//! connected overlap, never a refusal. A holding assertion quiets one
//! only when it reads a `Gap` directly over an opposed pair of the two
//! copies' faces, admits only negative values, its two faces bound the
//! overlap, and every face bounding it lies between their carriers.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture::{fname, insert, len, on_frame, place, square, wall};

use editor_core::{
    AssertionRelation, Formula, InterferenceFinding, MeasurePrimitive, Node, Overlap, ProfileDoc,
    RecipeNodeId, RoleSeg, SitedRef, StableName,
};
use geom_core::Tol;
use topo::{CensusContact, ValidationError};

/// **One copy of `body` in the world, `z` up its own z**: where a
/// construction drawn `z` up the world's z axis stands.
fn place_up(doc: ProfileDoc, body: RecipeNodeId, z: f64) -> (ProfileDoc, RecipeNodeId) {
    crate::fixture::in_world(doc, body, &editor_core::Frame::translation([0.0, 0.0, z]))
}

/// A block extruded `height` from a square of half-side `h` at
/// `(cx, cy)` on its own xy: the extrude, which a copy places `z` up
/// the world's z axis ([`place_up`]).
fn block(doc: ProfileDoc, (cx, cy): (f64, f64), h: f64, height: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, cy, h)],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
        },
    )
}

/// The `square` wall facing +x (segment 1) and the one facing −x
/// (segment 3), as `block`'s extrude names them.
pub(crate) fn plus_x(doc: &ProfileDoc, b: RecipeNodeId) -> StableName {
    fname(b, wall(doc, b, 1))
}

pub(crate) fn minus_x(doc: &ProfileDoc, b: RecipeNodeId) -> StableName {
    fname(b, wall(doc, b, 3))
}

/// **Two copies overlapping in one slab**: a unit block `a`, and a
/// smaller block `b` whose −x wall sits 0.05 inside `a`'s +x wall,
/// strictly inside `a` across y and z, so every vertex of `b` on that
/// wall is inside `a`'s material and nothing is coplanar. `a` placed
/// once at the identity and `b` once 0.25 up: `(doc, a, b, copy of a,
/// copy of b)`.
pub(crate) fn slab_overlap() -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
) {
    let doc = ProfileDoc::empty_derived("intent-s5-b-slab", Tol::witness());
    let (doc, a) = block(doc, (0.0, 0.0), 0.5, 1.0);
    let (doc, b) = block(doc, (0.75, 0.0), 0.3, 0.5);
    let (doc, p) = place(doc, a);
    let (doc, q) = place_up(doc, b, 0.25);
    (doc, a, b, p, q)
}

/// **What the at-rest check reports**: its interference findings, and
/// every finding beside them, over the product's gathered body, whose
/// keys the census's verdicts name.
struct AtRest {
    interference: Vec<InterferenceFinding>,
    findings: Vec<editor_core::CheckFinding>,
    body: topo::AtRestBody<f64>,
}

/// The at-rest check over `doc` at `run`.
fn at_rest(
    doc: &ProfileDoc,
    run: &editor_core::Evaluation<f64>,
) -> Result<AtRest, editor_core::ChecksError> {
    let findings = crate::fixture::at_rest(doc, run)?;
    let body = editor_core::product_recorded(doc, run, Tol::witness())
        .unwrap_or_else(|e| panic!("the product gathers: {e}"))
        .body;
    Ok(AtRest {
        interference: crate::fixture::interference(&findings),
        findings,
        body,
    })
}

fn assembled(doc: &ProfileDoc) -> AtRest {
    let run = corpus::eval::<f64>(doc);
    at_rest(doc, &run).unwrap_or_else(|e| panic!("the check could not run: {e}"))
}

/// The faces a bounded finding names, as `(placement, the body's own
/// name)`.
fn sites(finding: &InterferenceFinding, doc: &ProfileDoc) -> Vec<(RecipeNodeId, StableName)> {
    let Overlap::Bounded { faces } = &finding.overlap else {
        panic!("the overlap is bounded: {:?}", finding.overlap)
    };
    faces
        .iter()
        .map(|site| {
            let placement = doc
                .operation_of(site.copy.var)
                .expect("a copy is a placement's output");
            // A copy names its faces as its body does.
            (placement, site.face.clone())
        })
        .collect()
}

/// **(B, test 5) An overlap is a finding, not a refusal.** The at-rest
/// check reports one loud finding, whose evidence is the census's
/// decided `InstanceInterference` and
/// whose site names `a`'s +x wall and `b`'s −x wall among the faces
/// bounding it.
#[test]
fn two_overlapping_copies_report_one_loud_finding() {
    let (doc, a, b, p, q) = slab_overlap();
    let assembly = assembled(&doc);
    let [finding] = assembly.interference.as_slice() else {
        panic!("one overlap, one finding: {:?}", assembly.interference)
    };
    assert!(finding.is_loud(), "no assertion is written, so it is loud");
    assert_eq!(
        (finding.a.var, finding.b.var),
        (doc.output(p, 0).unwrap(), doc.output(q, 0).unwrap()),
        "the finding names the two copies in gather order"
    );
    assert!(
        finding
            .evidence
            .iter()
            .any(|e| matches!(e, ValidationError::InstanceInterference { .. })),
        "the census decided it as a vertex inside the other's material: {:?}",
        finding.evidence
    );
    let sites = sites(finding, &doc);
    for (copy, face) in [(p, plus_x(&doc, a)), (q, minus_x(&doc, b))] {
        assert!(
            sites.contains(&(copy, face.clone())),
            "{face:?} of copy {copy:?} bounds the overlap: {sites:?}"
        );
    }
    assert!(
        !sites.contains(&(p, minus_x(&doc, a))),
        "a's far wall does not bound it: {sites:?}"
    );
}

/// **(B, test 8) A pierce is interference.** A bar through a plate's
/// two side walls, every vertex of each outside the other's material:
/// the census sees only edges piercing faces, and the check reports one
/// finding for the pair rather than refusing an unrecorded contact.
#[test]
fn a_bar_through_a_plate_with_no_vertex_inside_is_one_finding() {
    let doc = ProfileDoc::empty_derived("intent-s5-b-pierce", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 0.0), 0.5, 1.0);
    // The bar: x ∈ [−1, 1], y ∈ [−0.1, 0.1], z ∈ [0.4, 0.6].
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(-1.0, -0.1), (1.0, -0.1), (1.0, 0.1), (-1.0, 0.1)]],
    );
    let (doc, bar) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(0.2),
        },
    );
    let (doc, p) = place(doc, plate);
    let (doc, q) = place_up(doc, bar, 0.4);
    let assembly = assembled(&doc);
    let [finding] = assembly.interference.as_slice() else {
        panic!("one overlap, one finding: {:?}", assembly.interference)
    };
    // No vertex is inside: the evidence is the pierces alone, which
    // decide the pair, so the containment arm leaves nothing undecided
    // beside them.
    assert!(
        !finding.evidence.is_empty()
            && finding.evidence.iter().all(|e| matches!(
                e,
                ValidationError::UnrecordedContact {
                    contact: CensusContact::EdgeFacePierce { .. },
                    ..
                }
            )),
        "the census's evidence is pierces: {:?}",
        finding.evidence
    );
    let sites = sites(finding, &doc);
    for (copy, face) in [(p, plus_x(&doc, plate)), (p, minus_x(&doc, plate))] {
        assert!(
            sites.contains(&(copy, face.clone())),
            "the plate's wall {face:?} bounds the overlap: {sites:?}"
        );
    }
    assert!(
        sites.iter().any(|(copy, _)| *copy == q),
        "the bar's faces bound it too: {sites:?}"
    );
}

/// A `Gap` over two faces read at `sites`, and an assertion bounding it
/// by `relation` and `bound` metres: the document after them, and the
/// assertion.
fn asserted_gap(
    doc: ProfileDoc,
    sites: [SitedRef; 2],
    relation: AssertionRelation,
    bound: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, measure) = crate::fixture::measure_node(
        &doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        sites.to_vec(),
    );
    let value: Formula = crate::fixture::value_of(&doc, measure);
    insert(
        doc,
        Node::Assertion { claim: editor_core::Claim::Value {
            value,
            bound: len(bound),
            relation,
        } },
    )
}

/// **(B, test 6, the unplaced row) A gap over unplaced bodies' faces
/// quiets nothing**, though it holds: it is a statement about bodies in
/// a space no placement brings to the world, not about the copies. The
/// slab pair is built a second time, `b` against `a`, and left
/// unplaced.
#[test]
fn a_holding_gap_over_the_unplaced_bodies_quiets_nothing() {
    let (doc, _, _, _, _) = slab_overlap();
    let (doc, a) = block(doc, (0.0, 0.0), 0.5, 1.0);
    let (doc, b) = block(doc, (0.75, 0.0), 0.3, 0.5);
    let (doc, b_at) = crate::fixture::slid(doc, a, b, [0.0; 3]);
    let (doc, assertion) = asserted_gap(
        doc.clone(),
        [
            SitedRef::new(a, plus_x(&doc, a)),
            SitedRef::new(b_at, minus_x(&doc, b)),
        ],
        AssertionRelation::AtMost,
        -0.01,
    );
    let run = corpus::eval::<f64>(&doc);
    assert!(
        matches!(
            run.value(assertion).map(|v| &v.payload),
            Some(editor_core::ValuePayload::Assertion(
                editor_core::AssertionVerdict::Holds { .. }
            ))
        ),
        "the assertion holds over the bodies (gap −0.05 ≤ −0.01)"
    );
    let assembly = at_rest(&doc, &run).expect("the check reports");
    let [finding] = assembly.interference.as_slice() else {
        panic!("one finding: {:?}", assembly.interference)
    };
    assert!(finding.is_loud(), "the bodies are not the copies");
}

/// A `Gap` over faces of two world copies, read where each copy holds
/// it (`face`), bounded by `relation` and `bound`.
pub(crate) fn gap_at_copies(
    doc: ProfileDoc,
    (p, outer): (RecipeNodeId, StableName),
    (q, inner): (RecipeNodeId, StableName),
    relation: AssertionRelation,
    bound: f64,
) -> (ProfileDoc, RecipeNodeId) {
    asserted_gap(
        doc,
        [SitedRef::new(p, outer), SitedRef::new(q, inner)],
        relation,
        bound,
    )
}

/// The one finding of `doc`'s assembly.
fn only_finding(doc: &ProfileDoc) -> InterferenceFinding {
    let assembly = assembled(doc);
    let [finding] = assembly.interference.as_slice() else {
        panic!("one finding: {:?}", assembly.interference)
    };
    finding.clone()
}

/// **(B, test 6) Quiet by a one-sided negative bound, and only by
/// one.** The slab overlap's `Gap(a's +x wall, b's −x wall)` over the
/// two copies is −0.05. Under `≤ −0.01` and `= −0.05` the finding is
/// listed and quiet, naming the assertion; under `≤ 0` (the bound is
/// not negative), `≥ −0.1` (the relation admits clearance) and
/// `≤ −0.1` (`Violated`) it stays loud.
///
/// Red if the rule reads the measure's value instead of the verdict
/// (the violated row quiets), admits a straddling bound (`≤ 0` quiets),
/// or ignores the relation (`≥` quiets).
#[test]
fn a_holding_negative_gap_over_the_copies_quiets_its_overlap_and_nothing_else_does() {
    let (doc, a, b, p, q) = slab_overlap();
    let (outer, inner) = (plus_x(&doc, a), minus_x(&doc, b));
    for (relation, bound, quiet) in [
        (AssertionRelation::AtMost, -0.01, true),
        (AssertionRelation::Equal, -0.05, true),
        (AssertionRelation::AtMost, 0.0, false),
        (AssertionRelation::AtLeast, -0.1, false),
        (AssertionRelation::AtMost, -0.1, false),
    ] {
        let (doc, assertion) = gap_at_copies(
            doc.clone(),
            (p, outer.clone()),
            (q, inner.clone()),
            relation,
            bound,
        );
        let finding = only_finding(&doc);
        assert_eq!(
            finding.quiet,
            quiet.then_some(assertion),
            "gap −0.05 under {} {bound}",
            relation.symbol()
        );
        assert!(
            matches!(finding.overlap, Overlap::Bounded { .. }),
            "a quiet finding is still listed with its site"
        );
    }
}

/// **(B, test 7) A second overlap of the same pair stays loud.** A
/// U-shaped copy whose two lugs both overlap one block, their +x walls
/// two faces on one carrier: two findings, and the assertion over the
/// first lug's wall quiets the first alone (the clevis's second lug).
///
/// Red if the site is the copy pair or the carrier pair (both quiet).
#[test]
fn an_assertion_on_one_lug_leaves_the_second_lug_loud() {
    let doc = ProfileDoc::empty_derived("intent-s5-b-clevis", Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (-0.5, -0.5),
            (0.5, -0.5),
            (0.5, -0.2),
            (0.0, -0.2),
            (0.0, 0.2),
            (0.5, 0.2),
            (0.5, 0.5),
            (-0.5, 0.5),
        ]],
    );
    let (doc, clevis) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let (doc, pin) = block(doc, (0.75, 0.0), 0.3, 0.5);
    let (doc, p) = place(doc, clevis);
    let (doc, q) = place_up(doc, pin, 0.25);
    let (first, second) = (
        fname(clevis, wall(&doc, clevis, 1)),
        fname(clevis, wall(&doc, clevis, 5)),
    );
    let (doc, assertion) = gap_at_copies(
        doc.clone(),
        (p, first.clone()),
        (q, minus_x(&doc, pin)),
        AssertionRelation::AtMost,
        -0.01,
    );
    let assembly = assembled(&doc);
    assert_eq!(assembly.interference.len(), 2, "one finding per lug");
    for finding in &assembly.interference {
        let sites = sites(finding, &doc);
        let expected = if sites.contains(&(p, first.clone())) {
            Some(assertion)
        } else {
            assert!(
                sites.contains(&(p, second.clone())),
                "the other is the second lug's: {sites:?}"
            );
            None
        };
        assert_eq!(
            finding.quiet, expected,
            "the assertion speaks for its own lug only"
        );
    }
}

/// **(B) An overlap reaching past the asserted carriers stays loud.**
/// `b` is an L whose foot sinks 0.2 into `a` beside the 0.05 slab, one
/// connected overlap bounded by the asserted pair: the assertion's
/// faces bound it, but the foot lies beyond `b`'s −x wall's carrier.
///
/// Red if the rule only asks that the assertion's faces bound the
/// overlap (the sunk flange quiets).
#[test]
fn an_overlap_with_a_sunk_flange_stays_loud() {
    let doc = ProfileDoc::empty_derived("intent-s5-b-flange", Tol::witness());
    let (doc, a) = block(doc, (0.0, 0.0), 0.5, 1.0);
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.45, -0.3),
            (1.05, -0.3),
            (1.05, 0.3),
            (0.3, 0.3),
            (0.3, 0.1),
            (0.45, 0.1),
        ]],
    );
    let (doc, b) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(0.5),
        },
    );
    let (doc, p) = place(doc, a);
    let (doc, q) = place_up(doc, b, 0.25);
    let (doc, _) = gap_at_copies(
        doc.clone(),
        (p, plus_x(&doc, a)),
        (q, fname(b, wall(&doc, b, 5))),
        AssertionRelation::AtMost,
        -0.01,
    );
    let finding = only_finding(&doc);
    assert!(
        finding.is_loud(),
        "the foot is beyond the asserted carriers: {finding:?}"
    );
}

/// **(B, test 10) A failed requirement gates nothing.** The quieting
/// assertion's measure names a face its copy does not hold, so the
/// measure refuses and the assertion has no verdict: the finding is
/// loud, and the assertion reports its
/// own failure.
#[test]
fn a_gap_whose_face_vanished_quiets_nothing_and_gates_nothing() {
    let (doc, a, b, p, q) = slab_overlap();
    let vanished = fname(a, RoleSeg::Cap(editor_core::CapEnd::End));
    let (doc, assertion) = asserted_gap(
        doc.clone(),
        [
            SitedRef::new(p, plus_x(&doc, a)),
            SitedRef::new(q, vanished),
        ],
        AssertionRelation::AtMost,
        -0.01,
    );
    let _ = b;
    let run = corpus::eval::<f64>(&doc);
    assert!(
        !matches!(
            run.value(assertion).map(|v| &v.payload),
            Some(editor_core::ValuePayload::Assertion(_))
        ),
        "the assertion has no verdict"
    );
    let assembly = at_rest(&doc, &run).expect("a failed assertion gates nothing");
    let [finding] = assembly.interference.as_slice() else {
        panic!("one finding: {:?}", assembly.interference)
    };
    assert!(finding.is_loud(), "and quiets nothing");
}

/// A plate with a 0.5 bore and a pin of radius `pin_r` through it,
/// each placed once: `(doc, plate, pin, copy of plate, copy of pin,
/// the bore's wall, the pin's wall)`.
pub(crate) fn bore_and_pin(
    pin_r: f64,
) -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    StableName,
    StableName,
) {
    bore_pin_and_holes(pin_r, &[])
}

/// [`bore_and_pin`] with further holes `(cx, cy, r)` through the plate.
fn bore_pin_and_holes(
    pin_r: f64,
    holes: &[(f64, f64, f64)],
) -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    StableName,
    StableName,
) {
    use crate::fixture::{in_world, piece};
    use editor_core::{LoopProgram, ProfileProgram};
    let circle = |r| LoopProgram::<Formula>::circle(0.0, 0.0, r).expect("a literal circle");
    let prism = |doc: ProfileDoc, loops, h| {
        let (doc, profile) = insert(
            doc,
            Node::Profile(ProfileProgram {
                loops,
                ids: Vec::new(),
                on: None,
            }),
        );
        insert(
            doc,
            Node::Extrude {
                profile: profile.into(),
                distance: len(h),
            },
        )
    };
    let doc = ProfileDoc::empty_derived("intent-s5-b-press", Tol::witness());
    let outline = crate::fixture::desc(vec![square(0.0, 0.0, 1.0)])
        .loops
        .remove(0);
    let mut loops = vec![outline, circle(0.5)];
    loops.extend(
        holes.iter().map(|&(cx, cy, r)| {
            LoopProgram::<Formula>::circle(cx, cy, r).expect("a literal circle")
        }),
    );
    let (doc, plate) = prism(doc, loops, 1.0);
    let (doc, pin) = prism(doc, vec![circle(pin_r)], 2.0);
    let (doc, p) = place(doc, plate);
    let (doc, q) = in_world(doc, pin, &editor_core::Frame::translation([0.0, 0.0, -0.5]));
    let lateral =
        |doc: &ProfileDoc, node, l| fname(node, RoleSeg::Lateral(piece(doc, node, l, 0).into()));
    let (bore, wall) = (lateral(&doc, plate, 1), lateral(&doc, pin, 0));
    (doc, p, q, plate, bore, wall)
}

/// **(B, test 6, curved) A press fit quiets under its bore's gap.** A
/// pin of radius 0.505, and one 2000 ε over, through a plate's 0.5 bore. The census decides
/// the pair interferes (the bore's rim vertices inside the pin, the
/// pin's seams piercing the plate) and leaves its curved face pairs
/// undecided, which are the same overlap: one finding, the annulus
/// between the two cylinders, loud unasserted and quiet under
/// `Gap(bore, pin) ≤ −0.001` over the two copies. The overlap's faces
/// lie on the asserted carriers themselves, which is where the
/// containment check's coincident faces glue.
#[test]
fn a_pin_pressed_into_a_bore_is_quiet_under_its_gap() {
    // 5 mm of interference, and 2000 ε (2 µm at the default row): the
    // second is a real press fit, its containment check's coincident
    // faces nearest the boolean's band and still well above it.
    let fine = 2000.0 * Tol::witness().eps();
    for (pin_r, bound) in [(0.505, -0.001), (0.5 + fine, -fine / 2.0)] {
        let (doc, p, q, _, bore, wall) = bore_and_pin(pin_r);
        let loud = only_finding(&doc);
        assert!(
            loud.is_loud(),
            "unasserted, the press fit is loud (pin {pin_r})"
        );
        assert!(
            loud.evidence
                .iter()
                .any(|e| matches!(e, ValidationError::InstanceInterference { .. })),
            "the census decided the overlap (pin {pin_r}): {:?}",
            loud.evidence
        );
        let (doc, assertion) = gap_at_copies(
            doc.clone(),
            (p, bore),
            (q, wall),
            AssertionRelation::AtMost,
            bound,
        );
        let finding = only_finding(&doc);
        assert_eq!(
            finding.quiet,
            Some(assertion),
            "quiet under Gap <= {bound} (pin {pin_r}): {finding:?}"
        );
    }
}

/// **(B, curved containment) A pin that bottoms through a blind bore is
/// not quieted by the bore's gap.** The plate is a union of the holed
/// plate and a floor slab under it, so the bore is blind at `z = 0`; the
/// pin is pressed into it (radius 0.505) and runs on into the floor. The
/// overlap is one solid, the annulus and the slug in the floor beneath
/// the bore, and both asserted faces bound it, but the slug lies inside
/// the bore's carrier, away from the plate's material: the region past
/// the carrier keeps it, so the bore's own gap does not quiet the
/// finding. (A `Gap` between the plate's top and the pin's bottom does:
/// `intent_s5_c_quietable::a_blind_bore_pin_is_quietable_and_refuses_until_quieted`.)
#[test]
fn a_blind_bore_pin_is_not_quieted_by_the_bores_gap() {
    use crate::fixture::{in_world, piece, slid};
    use editor_core::{Bodies, LoopProgram, ProfileProgram};
    let circle = |r| LoopProgram::<Formula>::circle(0.0, 0.0, r).expect("a literal circle");
    let prism = |doc: ProfileDoc, loops, h| {
        let (doc, profile) = insert(
            doc,
            Node::Profile(ProfileProgram {
                loops,
                ids: Vec::new(),
                on: None,
            }),
        );
        insert(
            doc,
            Node::Extrude {
                profile: profile.into(),
                distance: len(h),
            },
        )
    };
    let outline = || {
        crate::fixture::desc(vec![square(0.0, 0.0, 1.0)])
            .loops
            .remove(0)
    };
    let doc = ProfileDoc::empty_derived("intent-s5-b-blind", Tol::witness());
    let (doc, holed) = prism(doc, vec![outline(), circle(0.5)], 1.0);
    let (doc, floor) = prism(doc, vec![outline()], 0.5);
    let (doc, floor) = slid(doc, holed, floor, [0.0, 0.0, -0.5]);
    let (doc, plate) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![holed.into(), floor.into()]),
        },
    );
    let (doc, pin) = prism(doc, vec![circle(0.505)], 1.7);
    let (doc, p) = place(doc, plate);
    let (doc, q) = in_world(doc, pin, &editor_core::Frame::translation([0.0, 0.0, -0.2]));
    let lateral =
        |doc: &ProfileDoc, node, l| fname(node, RoleSeg::Lateral(piece(doc, node, l, 0).into()));
    let (bore, wall) = (lateral(&doc, holed, 1), lateral(&doc, pin, 0));
    assert!(
        only_finding(&doc).is_loud(),
        "unasserted, the overlap is loud"
    );
    // The union spells the bore through its own descent, so the site
    // is read off the finding: the plate copy's face whose name
    // descends from the holed plate's bore.
    let Overlap::Bounded { faces } = only_finding(&doc).overlap else {
        panic!("the overlap is sited")
    };
    let bore_path = format!("{:?}", bore.path);
    let site = faces
        .iter()
        .find(|site| format!("{:?}", site.face).contains(&bore_path))
        .expect("the bore bounds the overlap")
        .face
        .clone();
    let (doc, assertion) = asserted_gap(
        doc,
        [SitedRef::new(p, site), SitedRef::new(q, wall)],
        AssertionRelation::AtMost,
        -0.001,
    );
    let run = corpus::eval::<f64>(&doc);
    assert!(
        matches!(
            run.value(assertion).map(|v| &v.payload),
            Some(editor_core::ValuePayload::Assertion(
                editor_core::AssertionVerdict::Holds { .. }
            ))
        ),
        "the bore's gap holds: {:?}",
        run.node_error(assertion)
    );
    let finding = only_finding(&doc);
    assert!(
        finding.quiet.is_none() && matches!(finding.overlap, Overlap::Bounded { .. }),
        "the slug past the bore's carrier keeps the finding loud and sited: {finding:?}"
    );
}

/// **(B) A clearance fit the census cannot decide is could-not-look.** A
/// pin one box pad (`topo::test_support::sweep_pad`) inside the same bore: wide
/// enough that every vertex and edge reads definitely clear of the
/// other part, so the census decides no contact and no overlap, and
/// narrower than the two pads the depth gap takes, so it cannot part
/// the curved pairs either. Each is a could-not-look finding, which
/// refuses nothing.
///
/// Red if undecided pairs between copies are reported as interference
/// without a decided overlap.
#[test]
fn a_clearance_fit_the_census_cannot_decide_is_could_not_look() {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let (doc, ..) = bore_and_pin(0.5 - topo::test_support::sweep_pad(band));
    let run = corpus::eval::<f64>(&doc);
    let checked = at_rest(&doc, &run).expect("the check runs");
    let blind = crate::fixture::could_not_look(&checked.findings);
    assert!(
        checked.interference.is_empty()
            && !blind.is_empty()
            && blind.iter().all(|f| matches!(
                f,
                editor_core::AtRestEvidence::NoLane(ValidationError::CensusUndecidable { .. })
            )),
        "the census's undecided pairs: {blind:?}"
    );
}

/// **(B) A clearance fit apart past the band assembles.** A pin of
/// radius 0.49 in the same bore: the census parts the curved pairs on
/// the gap in depth about the common axis, so the copies stand apart
/// with no finding.
///
/// Red if a clearance the census can measure is refused or reported
/// as interference.
#[test]
fn a_clearance_fit_apart_in_depth_assembles() {
    let (doc, ..) = bore_and_pin(0.49);
    let assembly = assembled(&doc);
    assert!(
        assembly.interference.is_empty(),
        "a 0.01 clearance is no interference: {:?}",
        assembly.interference
    );
}

/// **(B) A quiet finding carries no verdict nothing checked.** The
/// press fit again, its plate drilled with a small hole beside the pin:
/// radius `0.05`, on the diagonal, its wall 1.5 of the census's box pads
/// (`sweep_pad`) clear of the pin's wall. The census parts two faces
/// only on a gap past both faces' pads, so it leaves the pin's wall and
/// that hole's wall undecided (a `CensusUndecidable` face pair), a pair
/// outside the overlap the assertion's containment check covers, so the
/// finding stays loud and carries the verdict.
///
/// The pads hold the pair undecided, not a missing direction: the line
/// between the two axes reads the gap, and the gap is less than the two
/// pads, as for `sweep`'s `cross_solid_apart::a_rod_between_the_pads_refuses`.
///
/// Red if an undecided verdict anywhere on the pair rides a quiet
/// finding.
#[test]
fn an_undecided_pair_beside_the_press_fit_keeps_it_loud() {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let gap = 1.5 * topo::test_support::sweep_pad(band);
    let at = (0.555 + gap) * core::f64::consts::FRAC_1_SQRT_2;
    let (doc, p, q, _, bore, wall) = bore_pin_and_holes(0.505, &[(at, at, 0.05)]);
    let (doc, _) = gap_at_copies(
        doc.clone(),
        (p, bore),
        (q, wall),
        AssertionRelation::AtMost,
        -0.001,
    );
    let assembly = assembled(&doc);
    let [finding] = assembly.interference.as_slice() else {
        panic!("one overlap: {:?}", assembly.interference)
    };
    let Overlap::Bounded { faces } = &finding.overlap else {
        panic!("bounded: {:?}", finding.overlap)
    };
    // The side hole's wall: the product's one cylinder of its radius.
    let hole_wall = |f: topo::FaceKey| {
        let body = &assembly.body;
        matches!(
            body.get_face(f).and_then(|f| body.get_surface(f.surface)),
            Some(geom::Surface::Cylinder { radius, .. }) if (radius - 0.05).abs() < 1e-12
        )
    };
    let beside = finding.evidence.iter().any(|e| {
        matches!(
            e,
            ValidationError::CensusUndecidable {
                a: topo::EntityId::Face(a),
                b: topo::EntityId::Face(b),
                ..
            } if hole_wall(*a) || hole_wall(*b)
        )
    });
    assert!(
        beside,
        "the census left the side hole's wall undecided: {:?}",
        finding.evidence
    );
    assert!(
        finding.is_loud(),
        "an undecided pair outside the overlap's {} faces keeps it loud",
        faces.len()
    );
}

/// **(B) A touch between an interfering pair keeps its finding loud**
/// (D1 (iii), D10). `b` is a C wrapped around the unit block `a` clear
/// of it but at two places: a tab whose −x wall sits 0.05 inside `a`'s
/// +x wall (the slab overlap), and a foot resting flush on `a`'s −x
/// wall, its face strictly inside that wall. The census records nothing
/// for a pair it decided interfere (`topo::record_at_rest`), so the
/// foot's touches ride the finding as evidence, and the holding
/// `Gap ≤ −0.01` over the tab, which bounds the overlap, does not speak
/// for them.
///
/// Red if a touch between the pair counts as covered by the overlap's
/// assertion.
#[test]
fn a_touch_beside_a_quieted_overlap_keeps_it_loud() {
    let doc = ProfileDoc::empty_derived("intent-s5-b-slab-touch", Tol::witness());
    let (doc, a) = block(doc, (0.0, 0.0), 0.5, 1.0);
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        // The tab's −x wall is the last segment.
        vec![vec![
            (0.45, -0.3),
            (1.05, -0.3),
            (1.05, 1.0),
            (-1.0, 1.0),
            (-1.0, -0.25),
            (-0.5, -0.25),
            (-0.5, 0.25),
            (-0.8, 0.25),
            (-0.8, 0.8),
            (0.75, 0.8),
            (0.75, 0.3),
            (0.45, 0.3),
        ]],
    );
    let (doc, b) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(0.5),
        },
    );
    let (doc, p) = place(doc, a);
    let (doc, q) = place_up(doc, b, 0.25);
    let (doc, assertion) = gap_at_copies(
        doc.clone(),
        (p, plus_x(&doc, a)),
        (q, fname(b, wall(&doc, b, 11))),
        AssertionRelation::AtMost,
        -0.01,
    );
    let assembly = assembled(&doc);
    let [finding] = assembly.interference.as_slice() else {
        panic!("one overlap: {:?}", assembly.interference)
    };
    assert!(
        finding
            .evidence
            .iter()
            .any(|e| matches!(e, ValidationError::UnrecordedContact { .. })),
        "the lump's touches ride the pair's finding: {:?}",
        finding.evidence
    );
    assert_eq!(
        finding.quiet, None,
        "the assertion {assertion:?} bounds the overlap and not the touch"
    );
}

/// An extrude of the polygon `corners` on its own xy, `height` up.
fn prism(doc: ProfileDoc, corners: Vec<(f64, f64)>, height: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![corners],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
        },
    )
}

/// **An overlap that shows only where the copies touch is one loud
/// finding.** A triangular prism stood 0.2 up inside a unit block, its
/// corners on three of the block's walls and its upright edges lying in
/// them: no vertex of either is strictly inside the other, no edge
/// pierces a face and no faces share a plane, so the census decides the
/// overlap at a touch (`TouchInterference`). The gate reports one loud
/// finding for the two copies, with that verdict as its evidence and no
/// undecided pair beside it. Red, the pair refused as undecidable or
/// cleared, where a decided crossing touch is read as anything else.
#[test]
fn a_prism_inscribed_in_a_block_is_one_loud_finding_at_its_touches() {
    let doc = ProfileDoc::empty_derived("atrest-touch-interference", Tol::witness());
    let (doc, block) = block(doc, (0.0, 0.0), 0.5, 1.0);
    let (doc, wedge) = prism(doc, vec![(0.0, -0.5), (0.5, 0.0), (-0.5, 0.0)], 0.6);
    let (doc, p) = place(doc, block);
    let (doc, q) = place_up(doc, wedge, 0.2);
    let assembly = assembled(&doc);
    let [finding] = assembly.interference.as_slice() else {
        panic!("one overlap, one finding: {:?}", assembly.interference)
    };
    assert!(finding.is_loud(), "no assertion is written, so it is loud");
    assert_eq!(
        (finding.a.var, finding.b.var),
        (doc.output(p, 0).unwrap(), doc.output(q, 0).unwrap()),
        "the finding names the two copies"
    );
    assert!(
        finding
            .evidence
            .iter()
            .any(|e| matches!(e, ValidationError::TouchInterference { .. }))
            && !finding
                .evidence
                .iter()
                .any(|e| matches!(e, ValidationError::CensusUndecidable { .. })),
        "the census decides the overlap at a touch, with nothing undecided beside it: {:?}",
        finding.evidence
    );
}

/// **An overlap only pierces show is quiet under its gap.** A unit
/// block, and a block wider than it across y sunk 0.05 into its +x wall
/// half-way up: no vertex of either is inside the other, and the
/// census decides the overlap by the pierces of the wide block's edges
/// through the unit block's ±y walls. Those decide the pair, so nothing
/// undecided rides the finding, and a holding `Gap ≤ −0.01` over the
/// unit block's +x wall and the wide block's −x wall, which bound the
/// overlap, quiets it as it quiets the slab. Red, the finding loud
/// under its gap, where a `CensusUndecidable` stands beside a decided
/// pair.
#[test]
fn an_overlap_only_pierces_show_is_quiet_under_its_gap() {
    let doc = ProfileDoc::empty_derived("atrest-pierce-quiet", Tol::witness());
    let (doc, a) = block(doc, (0.0, 0.0), 0.5, 1.0);
    let (doc, b) = prism(
        doc,
        vec![(0.45, -1.0), (1.5, -1.0), (1.5, 1.0), (0.45, 1.0)],
        0.5,
    );
    let (doc, p) = place(doc, a);
    let (doc, q) = place_up(doc, b, 0.25);
    let loud = only_finding(&doc);
    assert!(
        !loud.evidence.is_empty()
            && loud.evidence.iter().all(|e| matches!(
                e,
                ValidationError::UnrecordedContact {
                    contact: CensusContact::EdgeFacePierce { .. },
                    ..
                }
            )),
        "the evidence is the pierces alone: {:?}",
        loud.evidence
    );
    assert!(loud.is_loud(), "no assertion is written, so it is loud");
    let (doc, assertion) = gap_at_copies(
        doc.clone(),
        (p, plus_x(&doc, a)),
        (q, minus_x(&doc, b)),
        AssertionRelation::AtMost,
        -0.01,
    );
    assert_eq!(
        only_finding(&doc).quiet,
        Some(assertion),
        "the gap over the overlap's opposed walls quiets it"
    );
}
