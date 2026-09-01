//! **The one-call hollow ring** — a holed profile, fully revolved.
//!
//! An annulus swept a full turn about an axis it does not touch: a
//! tube bent into a closed circle, hollow all the way round. Two
//! concentric circles go in; a **two-shell solid** comes out — the
//! outer torus and, inside it, a toroidal CAVITY that is the tube's
//! bore, inserted as a reversed shell through the boolean's shared
//! void-insertion door. One `revolve` call, no boolean, no assembly of
//! halves.
//!
//! This is the shape the Klein bottle scene wanted and could not have.
//! Its findings entry 7 is what this scene renders: the ring BUILDS,
//! and its remaining wall is the STEP export (below).
//!
//! Its entry 6 — that `tube_along_arc`, the door storing a torus's
//! INTENT parameters bit-exactly, was solid-only — **no longer
//! stands**: VERBS-TUBEWALL gave that door the hollow sibling
//! `tube_along_arc_hollow`, and `tubewall::hollowtorus` puts its full
//! period on the tour beside this ring. So the two panels are now the
//! SAME shape through two doors rather than a shape and a gap: this
//! one is the profile door's answer (a holed profile, fully revolved),
//! that one the parameter door's (an outer radius and a wall, stored),
//! and they come out with the same census, the same two shells and the
//! same closed forms. What is still said only once is the STEP wall,
//! which both of them hit — pinned on both, and on klein's wall 6.
//!
//! # Why it is drawn see-through
//!
//! The subject of this scene is a cavity, and a cavity is invisible in
//! an opaque render at every camera — the same reason the Klein
//! bottle's loop tubes are drawn at 45 (`SceneBody::transparent`'s
//! founding case). At 45 the bore's silhouette reads through the tube
//! wall and the ring is legible as hollow rather than as a plain
//! torus, which is the only difference between this panel and the
//! sheave's groove.
//!
//! # The hollowness evidence, printed rather than asserted by eye
//!
//! A translucent render is suggestive, not conclusive, so the scene
//! prints what the body actually is:
//!
//! - **two shells, one cavity** — `Revolved::cavities` names it, and
//!   the shell count is 2 in one solid;
//! - **the census and genus** the tour prints for every body: each
//!   shell is a torus, so the pair carries genus 2 across two shells;
//! - **mass properties against the closed forms**, which is where
//!   hollowness becomes a number: `V = 2π²·R·(rₒ² − rᵢ²)` and
//!   `A = 4π²·R·(rₒ + rᵢ)` are the torus forms, and the volume the
//!   kernel certifies is the SOLID torus's less the bore's. A body
//!   that had quietly built as a plain torus would miss the volume by
//!   the bore and the area by the inner wall, and the assertions
//!   below would fail rather than the picture looking slightly wrong.
//! - the tour's own mesh ribbon closes it: `check_mesh` is watertight
//!   over BOTH shells, and the mesh's signed volume tracks the exact
//!   one.
//!
//! # The standing gate this scene declares
//!
//! **The ring cannot leave as STEP.** The writer's outward/void shell
//! classifier has closed forms for planar faces only, so a multi-shell
//! CURVED solid refuses `CurvedShellClassification` — the known
//! standing gate of OFFSET-DESIGN O6's demo-gates list, pinned as
//! klein's wall 6. This scene is the first tour body to reach it while
//! being RENDERED, so it declares the frontier at the scene
//! (`SceneBody::step_at_frontier`), which runs the export every pass
//! and fails the tour if the refusal ever changes or stops. Its
//! manifest entry carries a null `step`, and the FreeCAD lane — whose
//! subject is OCC re-tessellating our STEP — skips the body and says
//! so rather than drawing it from the mesh.
//!
//! # Findings this scene records (the demo-purpose rule)
//!
//! 1. **The ring KEEPS its document, and this scene proves it rather
//!    than assuming it.** The obvious guess — that a hole cannot reach
//!    `Node::Revolve`, the way a chamfer cannot reach any node at all
//!    (`diechamfer` finding 1) — is FALSE, and it is false in a way
//!    only execution settles: `ProfileProgram::loops` is a list read
//!    outer-then-holes, `LoopProgram::Circle` is a loop form, and the
//!    revolve wires the holed profile through and names the cavity
//!    shells. So [`through_the_document`] builds this same ring as a
//!    three-node recipe and the scene ASSERTS the two doors agree —
//!    same shell count, same census, same volume. The rendered body
//!    comes off the plain sweep door because that is where the tour's
//!    other revolves are authored and where the verb lives; the
//!    document path is not a workaround for anything.
//!
//!    What this leaves standing is `diechamfer`'s finding on its own
//!    scene: the kernel verb takes arena KEYS, so a document's own
//!    selection cannot be handed to it, and THAT die has no document
//!    because the scene calls the verb directly. The two verbs are not
//!    in the same position, and saying they were would have been a
//!    finding invented from symmetry.
//! 2. **The cavity's props door is [`pncad::topo::classify_shells`]**
//!    (this finding used to record its absence; the checks unit built
//!    it). A consumer wanting the bore's own volume or area (a coolant
//!    capacity, a fill weight) asks per shell and reads the cavity's
//!    entry — the `Void`-role shell, its signed volume negative by the
//!    orientation convention. The closed forms below still stand as
//!    this scene's independent oracle for a torus.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use pncad::authoring::{p2, validated};
use pncad::geom_core::{Tol, Vec2};
use pncad::prelude::{
    CancelToken, Datum, Dimension, Doc, DocEdit, EvalOptions, Expr, LoopProgram, MM, Node,
    PI as HALF_TURN, ProfileProgram, RecipeNodeId, ValuePayload, WrittenAngle, WrittenLength,
    apply, evaluate,
};
// The prefix data lives with the unit TABLE, one hop away from the
// prelude — the scene converts its own constants with the same factor
// the table pairs with `mm`, so the two cannot drift.
use pncad::profile::{ProfileLoop, SketchPlane};
use pncad::quantity::MILLI;
use pncad::sweep::{Revolution, RevolveAxis, revolve};
use pncad::topo::Body;

use crate::{SceneBody, Stop, View};

// **This scene is authored in MILLIMETRES**, and its document says so.
//
// The `_MM` constants are what a person designing this ring would
// write; the canonical metres beside them are derived through the unit
// table's own factor, so the analytic oracle below (which works in m
// and m³) and the recipe cannot drift apart. The document's literals
// then REMEMBER `mm`, so the panel opens on `300`, `70`, `50` rather
// than on `0.3`, `0.07`, `0.05`.
//
// `heatsink` and `checks` are the other half of this exhibit: they
// author canonically, so the gallery carries a document written in a
// chosen unit and a document written in the kernel's own.

/// The ring's mean radius: axis to tube centre.
const R_MM: f64 = 300.0;
const R: f64 = R_MM * MILLI;
/// The tube's outer radius.
const RO_MM: f64 = 70.0;
const RO: f64 = RO_MM * MILLI;
/// The tube's bore radius. `RO - RI` is the 20 mm wall.
const RI_MM: f64 = 50.0;
const RI: f64 = RI_MM * MILLI;

/// One concentric circle of the section, centred on the tube axis.
fn section(radius: f64, tol: Tol) -> ProfileLoop<f64> {
    pncad::profile::circle(p2(R, 0.0), radius, tol)
        .expect("a positive section radius")
        .into()
}

/// **The same ring, through the DOCUMENT** — the three-node recipe a
/// consumer modelling in a `Doc` would write: one profile node whose
/// loop list is `[Circle(rₒ), Circle(rᵢ)]` (outer first, then holes),
/// an axis datum, and `Node::Revolve` at a full turn.
///
/// It exists to settle finding 1 by execution rather than by reading
/// signatures, so `stops` asserts its answer against the plain door's
/// body. If the recipe layer ever stops carrying a hole through to the
/// verb, this stops agreeing and the finding is rewritten from what
/// the assertion says — not the other way round.
fn through_the_document(tol: Tol) -> Body<f64> {
    let (doc, revolved) = document(tol);
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    match &ev.value(revolved).expect("the revolve evaluated").payload {
        ValuePayload::Body(b) => (**b).clone(),
        other => panic!("expected a body, got {other:?}"),
    }
}

/// This scene's recipe, as a document the GUI can open.
///
/// The same document `through_the_document` evaluates — the gallery
/// hands a reader exactly the recipe this scene's claim rests on.
pub fn gallery_document(tol: Tol) -> Doc<ProfileProgram> {
    document(tol).0
}

/// The ring's recipe and its revolve node.
fn document(tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId) {
    // Written in millimetres: the value crosses in canonical metres,
    // and the literal keeps the notation it was authored in.
    let mm = |v: f64| {
        Expr::written_length(WrittenLength::in_unit(v, MM)).expect("a length in millimetres")
    };
    let mut doc: Doc<ProfileProgram> = Doc::empty_derived("hollow-ring", tol);
    let insert = |doc: &mut Doc<ProfileProgram>, node| -> RecipeNodeId {
        let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("the edit applies");
        *doc = applied.doc;
        applied.record.minted.expect("insert mints an id")
    };
    let circle = |r_mm: f64| LoopProgram::Circle {
        centre: [mm(R_MM), mm(0.0)],
        radius: mm(r_mm),
    };
    let profile = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            plane: SketchPlane::xy(),
            // Outer first, then the holes: the list IS the hole
            // vocabulary, and nothing else here mentions one.
            loops: vec![circle(RO_MM), circle(RI_MM)],
        }),
    );
    let axis = insert(
        &mut doc,
        Node::Datum(Datum::Axis {
            origin: [mm(0.0), mm(0.0), mm(0.0)],
            direction: [
                Expr::literal(0.0, Dimension::Scalar).expect("a scalar"),
                Expr::literal(1.0, Dimension::Scalar).expect("a scalar"),
                Expr::literal(0.0, Dimension::Scalar).expect("a scalar"),
            ],
        }),
    );
    let revolved = insert(
        &mut doc,
        Node::Revolve {
            profile,
            axis,
            // A full turn, written as one: the half-turn row is a
            // NOTATION carried as a unit, so the recipe says `2 pi rad`
            // where it would otherwise say `6.283185307179586 rad`.
            angle: Expr::written_angle(WrittenAngle::in_unit(2.0, HALF_TURN)).expect("a full turn"),
        },
    );
    (doc, revolved)
}

pub fn stops(tol: Tol) -> Vec<Stop> {
    // The whole model: an annulus, and a full turn. The hole is a
    // second loop of the same profile — the ring is not assembled from
    // anything, and nothing here mentions a cavity.
    let annulus = validated(
        SketchPlane::xy(),
        vec![section(RO, tol), section(RI, tol)],
        tol,
    )
    .expect("the annulus validates: a hole strictly inside its outer");
    let ring = revolve(
        &annulus,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .expect("a holed profile fully revolves in one call");

    // Hollowness, stated three ways. First structurally: the hole came
    // back as its own shell, not as a tunnel through the outer one.
    assert_eq!(
        ring.body.shells().count(),
        2,
        "outer torus + toroidal cavity, in ONE solid"
    );
    assert_eq!(ring.cavities.len(), 1, "one cavity per hole loop");
    assert_ne!(ring.cavities[0], ring.shell, "the cavity is not the outer");
    assert_eq!(
        ring.body
            .get_shell(ring.cavities[0])
            .expect("the cavity shell")
            .solid,
        ring.solid,
        "the cavity belongs to the ring's own solid"
    );

    // Then numerically, against the torus closed forms — outer minus
    // bore. This is the assertion a body that had built as a plain
    // torus would fail.
    let props = pncad::topo::mass_properties(&ring.body, tol).expect("mass properties");
    let v_solid = 2.0 * PI * PI * R * RO * RO;
    let v_want = 2.0 * PI * PI * R * (RO * RO - RI * RI);
    let a_want = 4.0 * PI * PI * R * (RO + RI);
    assert!(
        ((props.volume - v_want) / v_want).abs() < 1e-12,
        "V = {} vs the closed form {v_want}",
        props.volume
    );
    assert!(
        ((props.surface_area - a_want) / a_want).abs() < 1e-12,
        "A = {} vs the closed form {a_want}",
        props.surface_area
    );

    // And per shell (finding 2 above, executed): the `Void`-role shell
    // IS the named cavity, and its signed volume is the bore's closed
    // form, negated — the bore's own capacity, asked for directly.
    let classes = pncad::topo::classify_shells(&ring.body, tol).expect("per-shell classification");
    let voids: Vec<_> = classes
        .iter()
        .filter(|c| c.role == pncad::topo::ShellRole::Void)
        .collect();
    assert_eq!(voids.len(), 1, "one cavity, one Void shell");
    assert_eq!(
        voids[0].shell, ring.cavities[0],
        "the Void shell is the named cavity"
    );
    let v_bore = 2.0 * PI * PI * R * RI * RI;
    assert!(
        ((voids[0].volume + v_bore) / v_bore).abs() < 1e-12,
        "bore volume {} vs the closed form -{v_bore}",
        voids[0].volume
    );
    assert_eq!(props.volume_pad, 0.0, "closed forms need no pad");

    let bore = v_solid - v_want;
    let (v, e, f) = (
        ring.body.vertices().count(),
        ring.body.edges().count(),
        ring.body.faces().count(),
    );
    // The census, pinned ABSOLUTELY and not only door-to-door: each
    // shell is a two-arc profile fully revolved — 2 half-tube walls,
    // 2 seam meridians, 2 full-period rims, 2 vertices — so the solid
    // carries twice that. A face reorder cannot move these numbers,
    // but a face appearing or vanishing fails here rather than only
    // shifting the tessellation baseline's per-scene total.
    assert_eq!((v, e, f), (4, 8, 4), "census");

    // Finding 1, settled by execution: the same ring through the
    // recipe layer, and the two doors agree entity for entity.
    let doc_ring = through_the_document(tol);
    assert_eq!(
        (
            doc_ring.shells().count(),
            doc_ring.vertices().count(),
            doc_ring.edges().count(),
            doc_ring.faces().count(),
        ),
        (2, v, e, f),
        "the recipe layer carries the hole through to the same verb"
    );
    let doc_props = pncad::topo::mass_properties(&doc_ring, tol).expect("recipe mass properties");
    assert_eq!(
        doc_props.volume, props.volume,
        "the two doors build the same solid, not merely the same census"
    );

    vec![Stop {
        name: "hollowring",
        caption: "THE ONE-CALL HOLLOW RING (a holed profile, fully revolved)".to_string(),
        // Montage cell RETIRED by the montage-v3 curation (Evan,
        // 2026-08-30), with `hollowelbow` and `hollowtorus` beside it.
        // A cavity is invisible in an opaque render at every camera and
        // the see-through render is only a partial answer — Evan's
        // ruling: "they just aren't that interesting-looking". This is
        // `voidbox`'s own precedent one door over: its panel was
        // retired at the #91 refresh because an opaque void is
        // indistinguishable from a cube, and `crate::cutaway` — which
        // shows an interior by splitting rather than by translucency —
        // is what replaced it (see `bool_bodies::voidbox_narration`). The hollowness evidence was never the pixels: two
        // shells, `Revolved::cavities`, and the torus closed forms are
        // printed, and they stay, as does the standalone render and the
        // STEP-frontier declaration.
        montage: false,
        story: "a tube bent into a closed circle, hollow the whole way round — two \
                concentric circles and a full turn, in ONE revolve call",
        ops: "revolve(annulus, axis, Revolution::Full): the outer circle sweeps the \
              torus, the hole sweeps its own solid of revolution and enters as a \
              REVERSED cavity shell through the shared void-insertion door — the \
              degenerate no-crossing arm, so no boolean crossing machinery runs",
        delta: 2e-3,
        note: Some(format!(
            "{v} vertices, {e} edges, {f} faces over TWO shells in one solid — the outer \
             torus and the bore, which `Revolved::cavities` names. Hollow as a number, not \
             as a picture: V = {:.6} m³ against the closed form 2π²R(rₒ²−rᵢ²), where the \
             SOLID torus of the same outer radius would be {v_solid:.6} — the bore is \
             {bore:.6} m³, {:.1}% of it — and A = {:.6} m² = 4π²R(rₒ+rᵢ), which counts the \
             inner wall. Both at zero enclosure pad. Drawn see-through at 45 for the same \
             reason the bottle's loop tubes are: a cavity cannot be read from an opaque \
             render at any camera. The shape's remaining wall is its STEP export — the \
             writer's outward/void classifier has closed forms for planar faces only, so \
             this multi-shell CURVED solid refuses CurvedShellClassification, declared at \
             the scene and probed every pass. The SAME ring built as a three-node recipe \
             — one profile whose loop list is [Circle(rₒ), Circle(rᵢ)], an axis datum, \
             Revolve(2π) — comes back with the same shells, the same {v}/{e}/{f} and a \
             bit-equal volume, so the hole reaches the verb through the document too",
            props.volume,
            100.0 * bore / v_solid,
            props.surface_area
        )),
        view: View {
            elev: 24.0,
            azim: -55.0,
            up: 'y',
        },
        bodies: vec![
            SceneBody::plain("hollowring", [0.42, 0.66, 0.74], ring.body)
                .transparent(45)
                .step_at_frontier(
                    |e| {
                        matches!(
                            e,
                            pncad::step_export::StepExportError::CurvedShellClassification { .. }
                        )
                    },
                    "say so in klein's findings entry 7 and retire the other two probes of \
                     this one gate — klein's WALL 6 (`klein::wall_probes`), which pins this \
                     exact refusal on this exact shape, and `tubewall::hollowtorus`, which \
                     pins it on the parameter door's hollow torus",
                ),
        ],
    }]
}
