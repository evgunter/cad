//! **LIB-TUBE — the `Tube` and `HollowTube` recipe nodes**
//! (RECIPE-DOORS D4 as revised by the #1205 split ruling).
//!
//! Two node kinds over the kernel's two public doors, one each. The
//! documents under test are `corpus/tube_ring.rs` (solid, full ring)
//! and `corpus/hollow_tube_elbow.rs` (hollow, windowed) — opposite
//! corners of the vocabulary's two-by-two; the other two corners are
//! built here.
//!
//! # The oracles, and which question each answers
//!
//! **Volume and area** come from Pappus, stated by the tour's own
//! `tubewall` scene and re-derived here rather than copied as numbers:
//!
//! ```text
//! solid ring     V = 2π²Rr²             A = 4π²Rr
//! hollow ring    V = 2π²R(rₒ² − rᵢ²)    A = 4π²R(rₒ + rᵢ)
//! solid elbow    V = θ·R·πr²            A = θ·R·2πr + 2πr²
//! hollow elbow   V = θ·R·π(rₒ² − rᵢ²)   A = θ·R·2π(rₒ+rᵢ) + 2π(rₒ²−rᵢ²)
//! ```
//!
//! All carry π, so each is metered at a stated relative tolerance and
//! none is a corpus `MassPin` (which is asserted with `==`).
//!
//! **The differential** — solid minus hollow over the same spine and
//! window — is the bore's own Pappus form, and it is what
//! DISCRIMINATES the two node kinds inside one document. A volume row
//! on either node alone cannot: it would pass just as well if
//! `Node::HollowTube`'s arm quietly called the solid door with a
//! different radius.
//!
//! **The storage contract** is the row the volumes cannot reach at
//! all, and it is what audit rows 26/27 are actually about. A tube
//! body stores the caller's `minor_radius` VERBATIM in every outer
//! half-wall and `minor_radius − wall` — one IEEE subtraction of the
//! caller's own two numbers — in every inner one. Volumes agree to
//! 1e-12 whether or not that holds; only the stored bits see it, so
//! the rows here read `Surface::Torus`'s `minor_radius` and compare
//! `to_bits()`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use core::f64::consts::PI;

use corpus::{body_of, eval, failures, hollow_tube_elbow, tube_ring};
use editor_core::{
    Axis3, Datum, Dimension, DocEdit, Expr, Node, NodeErrorKind, NodeResult, ProfileDoc,
    ProfileProgram, RecipeNodeId, SlotId, TubeWindow, apply, load, save,
};
use fixture::len;
use geom_core::Tol;
use topo::{Body, Surface};

/// Relative agreement demanded of a closed form. The tour's tube stops
/// meter the same forms over the same doors at the same figure, so a
/// row here needing a looser one would be saying the recipe layer lost
/// precision the kernel had.
const REL: f64 = 1e-12;

fn scalar(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("finite")
}

fn angle(v: f64) -> Expr {
    Expr::literal(v, Dimension::Angle).expect("finite")
}

fn close(got: f64, want: f64, what: &str) {
    let rel = ((got - want) / want).abs();
    assert!(rel < REL, "{what}: got {got}, closed form {want} (rel {rel})");
}

fn push(d: &ProfileDoc, e: &DocEdit<ProfileProgram>) -> ProfileDoc {
    apply(d, e, Tol::witness()).expect("edit applies").doc
}

/// A document holding one datum axis plus whatever `build` hangs off
/// it — the two-node shape every tube recipe has.
fn spine_doc(
    axis_dir: [f64; 3],
    build: impl FnOnce(RecipeNodeId) -> Node<ProfileProgram>,
) -> (ProfileDoc, RecipeNodeId) {
    let mut doc = ProfileDoc::empty_derived("lib_tube_node", Tol::witness());
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Datum(Datum::Axis {
                origin: [len(0.0), len(0.0), len(0.0)],
                direction: axis_dir.map(scalar),
            }),
        },
    );
    let spine = *doc.order().last().expect("the datum is there");
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: build(spine),
        },
    );
    let tube = *doc.order().last().expect("the tube is there");
    (doc, tube)
}

/// Every stored minor radius on `body`, as raw bits — the storage
/// contract's whole evidence.
///
/// Read off `Surface::Torus` rather than off any recipe-side record:
/// the claim is about what the BODY holds, so a reader that consulted
/// the node's own expressions would be asserting the input against
/// itself.
fn stored_minor_bits(body: &Body<f64>) -> Vec<u64> {
    let mut bits: Vec<u64> = body
        .faces()
        .filter_map(|(_, face)| match body.get_surface(face.surface) {
            Some(Surface::Torus { minor_radius, .. }) => Some(minor_radius.to_bits()),
            _ => None,
        })
        .collect();
    bits.sort_unstable();
    bits
}

/// The two radii a hollow body must hold, as sorted bits.
fn both_radii(outer: f64, inner: f64) -> Vec<u64> {
    let mut w = vec![
        outer.to_bits(),
        outer.to_bits(),
        inner.to_bits(),
        inner.to_bits(),
    ];
    w.sort_unstable();
    w
}

fn solid_node(u_ref: [f64; 3], major: f64, window: TubeWindow, minor: f64) -> Node<ProfileProgram> {
    Node::Tube {
        spine: RecipeNodeId(0),
        u_ref: u_ref.map(scalar),
        major_radius: len(major),
        window,
        minor_radius: len(minor),
    }
}

fn hollow_node(
    u_ref: [f64; 3],
    major: f64,
    window: TubeWindow,
    minor: f64,
    wall: f64,
) -> Node<ProfileProgram> {
    Node::HollowTube {
        spine: RecipeNodeId(0),
        u_ref: u_ref.map(scalar),
        major_radius: len(major),
        window,
        minor_radius: len(minor),
        wall: len(wall),
    }
}

fn arc(t0: f64, t1: f64) -> TubeWindow {
    TubeWindow::Arc {
        t0: angle(t0),
        t1: angle(t1),
    }
}

// ---------------------------------------------------------------
// 1. The vocabulary as data
// ---------------------------------------------------------------

/// **Both kinds' slot lists, and what the hollow kind has that the
/// solid kind does not.**
///
/// The shared head is asserted AS a shared head — the hollow list is
/// the solid list plus the wall — because "shared kinds for the common
/// parameters" is a claim about the two together, and checking each
/// against its own literal would pass even if they had drifted apart.
#[test]
fn the_two_kinds_share_every_slot_but_the_wall() {
    let u = [1.0, 0.0, 0.0];
    let solid = solid_node(u, 2.0, arc(0.0, 1.5), 0.5);
    let hollow = hollow_node(u, 2.0, arc(0.0, 1.5), 0.5, 0.125);
    let (s, h) = (solid.slots(), hollow.slots());
    assert_eq!(
        h,
        [s.clone(), vec![SlotId::TubeWall]].concat(),
        "the hollow kind's slots are the solid kind's plus the wall"
    );
    assert!(!s.contains(&SlotId::TubeWall), "a solid tube has no wall");
    assert!(hollow.expr(SlotId::TubeWall).is_some());
    assert!(
        solid.expr(SlotId::TubeWall).is_none(),
        "the wall slot must not be readable on the solid kind"
    );

    // A full ring carries no window slots — the variant decides the
    // slot list, which is why it is structural payload, not a slot.
    let full = solid_node(u, 2.0, TubeWindow::Full, 0.5);
    assert!(!full.slots().contains(&SlotId::TubeWindowStart));
    assert!(full.expr(SlotId::TubeWindowStart).is_none());
    assert_eq!(full.slots().len() + 2, s.len());

    // One DAG edge each: the spine. A tube has no profile operand.
    assert_eq!(solid.inputs(), vec![RecipeNodeId(0)]);
    assert_eq!(hollow.inputs(), vec![RecipeNodeId(0)]);
    // And no payload names: a tube references no stable name, so a
    // `Rebind` cannot reach one.
    assert!(solid.payload_names().is_empty());
    assert!(hollow.payload_names().is_empty());
}

/// **The slot vocabulary's own table**: dimensions, structurality and
/// the prose labels a panel renders.
#[test]
fn the_tube_slots_carry_their_dimensions_and_labels() {
    for (slot, dim, label) in [
        (
            SlotId::TubeMajorRadius,
            Dimension::Length,
            "tube major radius",
        ),
        (
            SlotId::TubeMinorRadius,
            Dimension::Length,
            "tube minor radius",
        ),
        (SlotId::TubeWall, Dimension::Length, "tube wall"),
        (
            SlotId::TubeWindowStart,
            Dimension::Angle,
            "tube window start",
        ),
        (SlotId::TubeWindowEnd, Dimension::Angle, "tube window end"),
    ] {
        assert_eq!(slot.dimension(), dim, "{label}");
        assert_eq!(slot.label(), label);
        assert!(!slot.is_structural(), "{label} is a continuous parameter");
        assert!(slot.component().is_none(), "{label} is not a vector part");
    }
    // The reference direction rides the EXISTING vector family, so it
    // reads as a direction everywhere a direction is read.
    assert_eq!(SlotId::Direction(Axis3::X).dimension(), Dimension::Scalar);
    let u = [1.0, 0.0, 0.0];
    assert!(
        solid_node(u, 2.0, TubeWindow::Full, 0.5)
            .expr(SlotId::Direction(Axis3::X))
            .is_some()
    );
}

// ---------------------------------------------------------------
// 2. The closed forms
// ---------------------------------------------------------------

/// **The solid ring torus** — the registered `tube_ring` document
/// evaluates green and meters `V = 2π²Rr²`, `A = 4π²Rr`.
#[test]
fn the_solid_ring_meters_its_closed_form() {
    let d = tube_ring::document();
    let ev = eval::<f64>(&d.doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let body = body_of(&ev, d.result.expect("a result node"));
    let props = topo::mass_properties(body, Tol::witness()).expect("mass properties");
    let (r, minor) = (tube_ring::R, tube_ring::MINOR);
    close(props.volume, 2.0 * PI * PI * r * minor * minor, "ring V");
    close(props.surface_area, 4.0 * PI * PI * r * minor, "ring A");
    assert_eq!(props.volume_pad, 0.0, "a closed form needs no pad");
    assert_eq!(body.shells().count(), 1, "a solid torus is one shell");

    // The storage contract: every stored minor radius is the authored
    // one, bit for bit. This is the number the door exists to keep.
    // TWO half-walls, not four: a full ring's tube circle is the
    // two-arc traversal the door constructs, and closing the period
    // adds no third surface. (Measured, not assumed — the first draft
    // of this row said four and this is what the body actually holds.)
    assert_eq!(
        stored_minor_bits(body),
        vec![minor.to_bits(); 2],
        "two half-walls, each storing the authored minor radius {minor} verbatim"
    );
}

/// **The hollow elbow** — the registered `hollow_tube_elbow` document
/// meters the annular Pappus forms, and its stored radii are the
/// caller's own two numbers.
#[test]
fn the_hollow_elbow_meters_its_closed_form_and_stores_both_radii() {
    let d = hollow_tube_elbow::document();
    let ev = eval::<f64>(&d.doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let body = body_of(&ev, d.result.expect("a result node"));
    let props = topo::mass_properties(body, Tol::witness()).expect("mass properties");

    let (r, outer, wall) = (
        hollow_tube_elbow::R,
        hollow_tube_elbow::OUTER,
        hollow_tube_elbow::WALL,
    );
    let inner = hollow_tube_elbow::inner(wall);
    let theta = hollow_tube_elbow::T1 - hollow_tube_elbow::T0;
    let area = PI * (outer * outer - inner * inner);
    close(props.volume, theta * r * area, "elbow V");
    close(
        props.surface_area,
        theta * r * 2.0 * PI * (outer + inner) + 2.0 * area,
        "elbow A",
    );

    assert_eq!(body.shells().count(), 1, "an open elbow encloses nothing");
    assert_eq!(
        body.faces().count(),
        6,
        "two half-walls per circle plus two annular caps"
    );

    // THE STORAGE CONTRACT, which no volume row can see.
    assert_eq!(
        stored_minor_bits(body),
        both_radii(outer, inner),
        "the inner walls must hold {outer} - {wall} = {inner} as ONE subtraction of the \
         caller's own numbers — a drift here is the profile -> bulge -> radius \
         reconstruction this door retired, coming back"
    );
}

/// **The full hollow ring** — the corner no corpus document occupies,
/// and the only one that produces a CAVITY.
///
/// `V = 2π²R(rₒ² − rᵢ²)` is the `hollowring` scene's own statement.
#[test]
fn the_full_hollow_ring_is_a_torus_shell_with_a_cavity() {
    let (outer, wall, r) = (0.5, 0.125, 2.0);
    let inner = outer - wall;
    let (doc, tube) = spine_doc([0.0, 0.0, 1.0], |spine| Node::HollowTube {
        spine,
        u_ref: [scalar(1.0), scalar(0.0), scalar(0.0)],
        major_radius: len(r),
        window: TubeWindow::Full,
        minor_radius: len(outer),
        wall: len(wall),
    });
    let ev = eval::<f64>(&doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let body = body_of(&ev, tube);
    let props = topo::mass_properties(body, Tol::witness()).expect("mass properties");
    close(
        props.volume,
        2.0 * PI * PI * r * (outer * outer - inner * inner),
        "hollow ring V",
    );
    close(
        props.surface_area,
        4.0 * PI * PI * r * (outer + inner),
        "hollow ring A",
    );
    assert_eq!(
        body.shells().count(),
        2,
        "a torus shell is an outer shell plus its cavity"
    );
    assert_eq!(
        stored_minor_bits(body),
        both_radii(outer, inner),
        "the full ring stores the same two radii as the windowed elbow"
    );
}

/// **The solid elbow** — the fourth corner, and the one that shows the
/// window's two angles reaching the door as angles.
#[test]
fn the_solid_elbow_meters_its_pappus_form() {
    let (r, minor, t0, t1) = (2.0, 0.5, 0.25, 1.75);
    let (doc, tube) = spine_doc([0.0, 1.0, 0.0], |spine| Node::Tube {
        spine,
        u_ref: [scalar(1.0), scalar(0.0), scalar(0.0)],
        major_radius: len(r),
        window: arc(t0, t1),
        minor_radius: len(minor),
    });
    let ev = eval::<f64>(&doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let body = body_of(&ev, tube);
    let props = topo::mass_properties(body, Tol::witness()).expect("mass properties");
    let theta = t1 - t0;
    close(props.volume, theta * r * PI * minor * minor, "solid elbow V");
    close(
        props.surface_area,
        theta * r * 2.0 * PI * minor + 2.0 * PI * minor * minor,
        "solid elbow A",
    );
    assert_eq!(
        stored_minor_bits(body),
        vec![minor.to_bits(); 2],
        "two half-walls, each storing the authored minor radius"
    );
}

/// **The differential that discriminates the two kinds** (the D3
/// shape G16's fix pass proved out).
///
/// One document, one spine, one window, two nodes — a `Tube` and a
/// `HollowTube` with identical radii — and the difference of their
/// volumes is the BORE's own Pappus form. A volume row on either node
/// alone would pass if the hollow arm secretly called the solid door;
/// this one cannot, because the bore is exactly what the second door
/// removes and the first does not.
#[test]
fn solid_minus_hollow_is_the_bore_within_one_document() {
    let (outer, wall, r) = (0.5, 0.125, 2.0);
    let inner = outer - wall;
    let (t0, t1) = (0.0, 1.5);

    let mut doc = ProfileDoc::empty_derived("tube_differential", Tol::witness());
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Datum(Datum::Axis {
                origin: [len(0.0), len(0.0), len(0.0)],
                direction: [scalar(0.0), scalar(1.0), scalar(0.0)],
            }),
        },
    );
    let spine = *doc.order().last().expect("datum");
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Tube {
                spine,
                u_ref: [scalar(1.0), scalar(0.0), scalar(0.0)],
                major_radius: len(r),
                window: arc(t0, t1),
                minor_radius: len(outer),
            },
        },
    );
    let solid = *doc.order().last().expect("solid");
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::HollowTube {
                spine,
                u_ref: [scalar(1.0), scalar(0.0), scalar(0.0)],
                major_radius: len(r),
                window: arc(t0, t1),
                minor_radius: len(outer),
                wall: len(wall),
            },
        },
    );
    let hollow = *doc.order().last().expect("hollow");

    let ev = eval::<f64>(&doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let vol = |id| {
        topo::mass_properties(body_of(&ev, id), Tol::witness())
            .expect("mass properties")
            .volume
    };
    close((t1 - t0) * r * PI * inner * inner, vol(solid) - vol(hollow), "the bore");

    // The two nodes take DIFFERENT content keys, so a memo cannot
    // serve one for the other — the append-a-tag rule, executed
    // rather than asserted about the source.
    assert_ne!(
        ev.value(solid).expect("solid value").content_key,
        ev.value(hollow).expect("hollow value").content_key,
        "a solid tube and a hollow tube of the same radii must not share a content key"
    );
    // And they store DIFFERENT radii sets, which is the same claim one
    // layer nearer the geometry.
    assert_eq!(
        stored_minor_bits(body_of(&ev, solid)),
        vec![outer.to_bits(); 2]
    );
    assert_eq!(
        stored_minor_bits(body_of(&ev, hollow)),
        both_radii(outer, inner)
    );
}

/// **The window variant is content**, not merely a different slot
/// count: re-authoring a full ring as an arc moves the key.
#[test]
fn the_window_variant_feeds_the_content_key() {
    let mk = |window: TubeWindow| {
        let (doc, tube) = spine_doc([0.0, 0.0, 1.0], |spine| Node::Tube {
            spine,
            u_ref: [scalar(1.0), scalar(0.0), scalar(0.0)],
            major_radius: len(2.0),
            window,
            minor_radius: len(0.5),
        });
        eval::<f64>(&doc).value(tube).expect("a value").content_key
    };
    assert_ne!(
        mk(TubeWindow::Full),
        mk(arc(0.0, 1.5)),
        "a full ring and an arc are different bodies and must key apart"
    );
}

// ---------------------------------------------------------------
// 3. The refusal families
// ---------------------------------------------------------------

/// The typed tube refusal a node produced, or `None` if it built.
///
/// Reached through the recipe NODE, never by calling the kernel door:
/// the claim is that the node wires these refusals, and a direct
/// kernel call would prove only that the kernel still has them.
fn tube_refusal(node: Node<ProfileProgram>, axis: [f64; 3]) -> Option<String> {
    let (doc, tube) = spine_doc(axis, |spine| match node {
        Node::Tube {
            u_ref,
            major_radius,
            window,
            minor_radius,
            ..
        } => Node::Tube {
            spine,
            u_ref,
            major_radius,
            window,
            minor_radius,
        },
        Node::HollowTube {
            u_ref,
            major_radius,
            window,
            minor_radius,
            wall,
            ..
        } => Node::HollowTube {
            spine,
            u_ref,
            major_radius,
            window,
            minor_radius,
            wall,
        },
        other => other,
    });
    let ev = eval::<f64>(&doc);
    match ev.nodes.get(&tube) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::Tube(t) => Some(t.to_string()),
            other => panic!("a tube node must refuse NodeErrorKind::Tube, got {other:?}"),
        },
        _ => None,
    }
}

/// **The frame, window and convention refusals, reachable through BOTH
/// kinds** — every arm on the shared half of the kernel's enum.
#[test]
fn the_shared_refusals_are_reachable_from_both_kinds() {
    let z = [0.0, 0.0, 1.0];
    let u = [1.0, 0.0, 0.0];
    for (what, s, h) in [
        (
            "non-unit u_ref",
            solid_node([2.0, 0.0, 0.0], 2.0, TubeWindow::Full, 0.5),
            hollow_node([2.0, 0.0, 0.0], 2.0, TubeWindow::Full, 0.5, 0.125),
        ),
        (
            "u_ref not perpendicular to the axis",
            solid_node([0.0, 0.0, 1.0], 2.0, TubeWindow::Full, 0.5),
            hollow_node([0.0, 0.0, 1.0], 2.0, TubeWindow::Full, 0.5, 0.125),
        ),
        (
            "a reversed window",
            solid_node(u, 2.0, arc(1.5, 0.5), 0.5),
            hollow_node(u, 2.0, arc(1.5, 0.5), 0.5, 0.125),
        ),
        (
            "a window reaching one full period",
            solid_node(u, 2.0, arc(0.0, 7.0), 0.5),
            hollow_node(u, 2.0, arc(0.0, 7.0), 0.5, 0.125),
        ),
        (
            "the ring-torus convention R > r",
            solid_node(u, 0.25, TubeWindow::Full, 0.5),
            hollow_node(u, 0.25, TubeWindow::Full, 0.5, 0.125),
        ),
    ] {
        let sm =
            tube_refusal(s, z).unwrap_or_else(|| panic!("{what} must refuse through Node::Tube"));
        let hm = tube_refusal(h, z)
            .unwrap_or_else(|| panic!("{what} must refuse through Node::HollowTube"));
        // A shared arm names "tube door", never one of the two: the
        // kernel refuses to guess which caller it was, and the recipe
        // layer must not invent an answer either.
        assert!(sm.contains("tube door"), "{what} (solid): {sm}");
        assert!(
            !hm.contains("tube_along_arc_hollow"),
            "{what} is reachable through both doors, so its message must not claim \
             the hollow one: {hm}"
        );
    }
}

/// **The three wall arms, reachable ONLY through `HollowTube`.**
///
/// Each is a distinct verdict, and the third is the one the first two
/// cannot see: at a large outer radius a thickness well above ε still
/// falls under that radius's own ulp, so `minor_radius − wall` rounds
/// back onto `minor_radius` and the two circles would be stored as
/// one. That arm is what makes the storage contract's bit-equality
/// meaningful rather than vacuous — without it a "hollow" body could
/// store one radius twice and every volume row would still pass.
#[test]
fn the_three_wall_arms_are_reachable_and_only_through_the_hollow_kind() {
    let z = [0.0, 0.0, 1.0];
    let u = [1.0, 0.0, 0.0];

    let nonpositive = tube_refusal(hollow_node(u, 2.0, TubeWindow::Full, 0.5, 0.0), z)
        .expect("a zero wall is not a wall");
    assert!(nonpositive.contains("tube_wall"), "{nonpositive}");

    let eats_the_bore = tube_refusal(hollow_node(u, 2.0, TubeWindow::Full, 0.5, 0.5), z)
        .expect("a wall equal to the outer radius leaves no bore");
    assert!(eats_the_bore.contains("tube_wall_bore"), "{eats_the_bore}");

    // The realized-gap arm: an outer radius whose own ulp exceeds a
    // wall that is itself comfortably above ε.
    let collapsed = tube_refusal(hollow_node(u, 1e14, TubeWindow::Full, 1e12, 1e-6), z)
        .expect("a wall under the outer radius's own ulp collapses the stored gap");
    assert!(collapsed.contains("tube_wall_gap"), "{collapsed}");

    // All three name the HOLLOW door outright — the solid door cannot
    // produce them, and the message says so.
    for msg in [&nonpositive, &eats_the_bore, &collapsed] {
        assert!(
            msg.contains("tube_along_arc_hollow"),
            "a wall refusal must name the door only it can come from: {msg}"
        );
    }

    // The three are DISTINCT verdicts, not one arm reached three ways.
    assert_ne!(nonpositive, eats_the_bore);
    assert_ne!(eats_the_bore, collapsed);
    assert_ne!(nonpositive, collapsed);

    // And the solid kind has no wall to be wrong about: the same
    // spine and radii build, because there is no wall in its
    // vocabulary at all.
    assert!(
        tube_refusal(solid_node(u, 2.0, TubeWindow::Full, 0.5), z).is_none(),
        "the solid door has no wall arm to reach"
    );
}

/// **A tube's spine must be a datum AXIS**: anything else is the
/// ordinary wrong-operand refusal, and the kernel door is never
/// reached.
#[test]
fn a_spine_that_is_not_an_axis_refuses_at_the_operand() {
    let mut doc = ProfileDoc::empty_derived("tube_operand", Tol::witness());
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Datum(Datum::Point {
                position: [len(0.0), len(0.0), len(0.0)],
            }),
        },
    );
    let point = *doc.order().last().expect("datum point");
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Tube {
                spine: point,
                u_ref: [scalar(1.0), scalar(0.0), scalar(0.0)],
                major_radius: len(2.0),
                window: TubeWindow::Full,
                minor_radius: len(0.5),
            },
        },
    );
    let tube = *doc.order().last().expect("tube");
    let ev = eval::<f64>(&doc);
    match ev.nodes.get(&tube) {
        Some(NodeResult::Failed(e)) => assert!(
            matches!(
                e.kind,
                NodeErrorKind::WrongOperand {
                    expected: "datum axis",
                    ..
                }
            ),
            "{:?}",
            e.kind
        ),
        other => panic!("a point spine must refuse, got {other:?}"),
    }
}

// ---------------------------------------------------------------
// 4. Naming — the revolve template, applied wholesale
// ---------------------------------------------------------------

/// **A tube body carries a FULL name table**, minted by the revolve
/// emitter with no translation and no new role vocabulary.
///
/// The measurement deliverable 2 asked for, executed rather than
/// argued: `name_revolve` reads only `Revolved<T>`'s own maps, both
/// tube doors return one built by the same `full`/`partial`
/// machinery, and the table that comes back is TOTAL over the body
/// (the emitter's own `check_total` runs inside it, so an entity it
/// missed is a refusal rather than a silent gap). Were a
/// tube-specific translation needed, its absence would show here.
#[test]
fn both_kinds_mint_a_total_name_table_through_the_revolve_emitter() {
    for (what, d) in [
        ("solid ring", tube_ring::document()),
        ("hollow elbow", hollow_tube_elbow::document()),
    ] {
        let ev = eval::<f64>(&d.doc);
        assert!(failures(&ev).is_empty(), "{what}: {:?}", failures(&ev));
        let id = d.result.expect("a result node");
        let value = ev.value(id).expect("a value");
        let body = body_of(&ev, id);
        let entities =
            body.faces().count() + body.edges().count() + body.vertices().count() + 1;
        assert_eq!(
            value.name_table.len(),
            entities,
            "{what}: every face, edge and vertex plus the body itself is named — an \
             emitter that skipped one would leave a silent naming dead end"
        );
    }
}

// ---------------------------------------------------------------
// 5. Persistence and the edit door
// ---------------------------------------------------------------

/// **Both kinds round-trip through the v17 wire**, window variants and
/// the wall included.
#[test]
fn both_kinds_round_trip_through_persistence() {
    for d in [tube_ring::document(), hollow_tube_elbow::document()] {
        // The snapshot ALREADY holds the corpus edits, so the log
        // saved beside it is empty: handing `d.edits` here would
        // replay them onto the state they produced and double the
        // recipe (measured: four nodes back from a two-node document).
        let text = save(&d.doc, &[], Tol::witness()).expect("saves");
        assert_eq!(text.lines().next(), Some("schema: 17"));
        let back = load(&text, Tol::witness()).expect("loads");
        assert_eq!(
            back.doc.order(),
            d.doc.order(),
            "{}: the recipe survives the wire",
            d.name
        );
        for &id in d.doc.order() {
            assert_eq!(
                back.doc.node(id),
                d.doc.node(id),
                "{}: node {id:?} survives the wire",
                d.name
            );
        }
    }
}

/// **The wall slot is editable, and the edit moves the stored INNER
/// radius and nothing else** — the corpus bump, metered at the bits
/// rather than at the volume, because the bits are what audit rows
/// 26/27 claim.
#[test]
fn bumping_the_wall_moves_the_stored_inner_radius() {
    let d = hollow_tube_elbow::document();
    let before = hollow_tube_elbow::inner(hollow_tube_elbow::WALL);
    let after = hollow_tube_elbow::inner(hollow_tube_elbow::WALL_BUMPED);
    assert_ne!(before.to_bits(), after.to_bits());

    let bumped = apply(&d.doc, &d.bump, Tol::witness())
        .expect("the bump applies")
        .doc;
    let ev = eval::<f64>(&bumped);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let body = body_of(&ev, d.result.expect("a result node"));
    assert_eq!(
        stored_minor_bits(body),
        both_radii(hollow_tube_elbow::OUTER, after),
        "a wall edit moves the inner radius and leaves the outer one exactly alone"
    );
}
