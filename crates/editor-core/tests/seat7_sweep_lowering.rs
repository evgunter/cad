//! **The sweeps moved onto the verb substrate, the profile's own
//! radius reaches the walls they mint, and one declared parameter
//! reaches the cyl×cyl germ from a document.**
//!
//! Three claims, three groups of rows, in that order.
//!
//! # 1. Nothing observable moved (the SEAT-4/5 method)
//!
//! `Node::Extrude` and `Node::Revolve` now build a `verbs::Verb`, run
//! it through the profile door and read their birth record out of the
//! closed record channel. That is a re-plumbing, and a re-plumbing's
//! failure mode is a difference nobody looks for — so the wire format
//! is pinned by a byte-identical round trip over a document carrying
//! both sweeps, and each sweep-carrying corpus document's evaluation is
//! pinned to a committed digest that says WHICH document moved.
//!
//! What already covers this and what these rows add is the same
//! division SEAT-4 recorded: `m10_p_fence` digests every body point's
//! bits corpus-wide and `lib_g16_corpus_name_digests` digests every
//! name table, so either would catch a lowering that changed geometry
//! or names — and neither says which document did it, nor reaches the
//! provenance tables. **Those two goldens hold UNCHANGED across this
//! migration**, which is the corpus-wide half of the differential and
//! cost no re-blessing: the constants below were minted here and
//! re-taken on the extracted merge base with this file copied onto it.
//!
//! # 2. The per-edge flow, attached and read back
//!
//! The extrude's distance and the revolve's angle are extents and reach
//! no stored field — declared as empty rows, and asserted empty over
//! real bodies. What DOES reach a field is the operand profile's own
//! carrier radius: an extruded circle's wall is a cylinder whose stored
//! radius is that circle's, a revolved circle's wall is a torus whose
//! minor radius is, and both carry the lowered identity of the
//! expression the profile holds.
//!
//! Every row here reads the channel through the KERNEL's own evidence
//! door, never through a stored token's `Debug`: the claim under test
//! is what a boolean germ would see.
//!
//! # 3. The germ, end to end from a document
//!
//! The row `SEAT-6` could not write, and the reason its residue item
//! stayed open: two extruded circles at one declared `r`, spun off the
//! pinch, unioned — and the pinch refusal the kernel returns carries
//! `Declared`, computed with zero numerics from two walls a recipe
//! minted. The kernel-direct twin at bit-identical radii carries
//! `None`, permanently (P3).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::corpus;
use crate::fixture;

use corpus::{body_of, eval, failures};
use editor_core::{
    CancelToken, Datum, Dimension, DocEdit, DocParam, DocumentId, EvalOptions, Evaluation, Expr,
    LoopProgram, Node, ParamName, ProfileDoc, ProfileProgram, RecipeNodeId, SlotId, StepArg,
    evaluate, persist,
};
use fixture::{ang, axis_in_plane, insert, len, scl, square, step};
use geom_brep::RadiusEvidence;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use topo::{Body, BooleanError, FaceKey, SurfaceField};

fn tol() -> Tol {
    Tol::witness()
}

/// The declared radius every document below draws its circles at,
/// meters (dyadic).
const R: f64 = 1.0;
/// The second declared radius — a hole's, and the peg the outer wall
/// must NOT declare against (dyadic).
const Q: f64 = 0.25;
/// The extrusion half-height (dyadic).
const H: f64 = 1.2;
/// The spin that takes both seams off the pinch — the germ fixture's
/// own angle (`sweep`'s `verbs_germarms2`).
const PHI: f64 = PI / 4.0;

fn param(name: &str) -> Expr {
    Expr::param(ParamName::new(name), Dimension::Length)
}

/// A document declaring `r`.
fn doc_with_r(name: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(name), tol());
    step(
        doc,
        DocEdit::SetDocParam {
            name: ParamName::new("r"),
            value: DocParam::continuous(Dimension::Length, R),
        },
    )
    .0
}

/// A frame at `z` and a circle of radius `radius` drawn on it —
/// returns the doc, the frame node and the profile node.
fn circle_on_frame(
    doc: ProfileDoc,
    z: f64,
    radius: Expr,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Frame {
            origin: [len(0.0), len(0.0), len(z)],
            u: [scl(1.0), scl(0.0), scl(0.0)],
            v: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    );
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::Circle {
                centre: [len(0.0), len(0.0)],
                radius,
            }],
        }),
    );
    (doc, plane, profile)
}

/// A cylinder about `z` of radius `radius`, `z ∈ [−H, H]` — the
/// document spelling of the germ fixture's `cyl`.
fn cylinder(doc: ProfileDoc, radius: Expr) -> (ProfileDoc, RecipeNodeId) {
    let (doc, _, profile) = circle_on_frame(doc, -H, radius);
    insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(2.0 * H),
        },
    )
}

/// A rigid rotation of `input` about the origin.
fn spin(
    doc: ProfileDoc,
    input: RecipeNodeId,
    axis: [f64; 3],
    angle: f64,
) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Transform {
            input,
            translation: [len(0.0), len(0.0), len(0.0)],
            rotation_axis: axis.map(scl),
            rotation_angle: ang(angle),
        },
    )
}

// ------------------------------------------------------------------
// 1. Nothing observable moved
// ------------------------------------------------------------------

/// The two-sweep fixture: what to save, what to evaluate, and the two
/// sweep nodes' ids.
struct BothSweeps {
    snapshot: ProfileDoc,
    doc: ProfileDoc,
    edits: Vec<DocEdit<ProfileProgram>>,
    sweeps: [RecipeNodeId; 2],
}

/// **One document carrying both sweep nodes**: a square extruded, and a
/// square revolved about an axis written in its own frame — so a single
/// file exercises both wire spellings and both lowering paths.
fn both_sweeps() -> BothSweeps {
    let mut r = corpus::Recorder::new();
    let snapshot = r.doc.clone();
    let square_loop = LoopProgram::polygon(square(0.0, 0.0, 0.5)).unwrap();
    let frame = r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(1.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(1.0), scl(0.0)],
    }));
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane: frame,
        loops: vec![square_loop],
    }));
    let extruded = r.insert(Node::Extrude {
        profile,
        distance: len(1.0),
    });
    // The revolve's own profile: a square clear of the axis, drawn on
    // its own frame, spun about an axis written in that same frame.
    let rev_frame = r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(1.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(1.0), scl(0.0)],
    }));
    let rev_profile = r.insert(Node::Profile(ProfileProgram {
        plane: rev_frame,
        loops: vec![LoopProgram::polygon(square(0.0, -2.0, 0.5)).unwrap()],
    }));
    let axis = r.insert(axis_in_plane(rev_frame, (0.0, 0.0), (1.0, 0.0)));
    let revolved = r.insert(Node::Revolve {
        profile: rev_profile,
        axis,
        angle: ang(PI / 2.0),
    });
    BothSweeps {
        snapshot,
        doc: r.doc,
        edits: r.edits,
        sweeps: [extruded, revolved],
    }
}

/// **The wire format is untouched**: save → load → save reproduces the
/// bytes exactly, for a document carrying an extrude and a revolve.
///
/// Byte equality is the whole assertion. A schema bump, a field rename,
/// a reordered payload or a changed number format each break it, and
/// none of them would be visible in an evaluation digest.
#[test]
fn an_extrude_and_revolve_document_round_trips_byte_identical() {
    let fixture = both_sweeps();
    let first =
        persist::save(&fixture.snapshot, &fixture.edits, tol()).expect("the document saves");
    let loaded = persist::load(&first, tol()).expect("its own bytes load back");
    assert_eq!(loaded.edits, fixture.edits, "the edit log did not survive");
    assert!(
        loaded.doc.bit_eq(&fixture.doc),
        "the replayed document is not bit-identical to the authored one"
    );
    let second = persist::save(&loaded.snapshot, &loaded.edits, tol())
        .expect("the loaded document re-saves");
    assert_eq!(
        first, second,
        "an extrude+revolve document does not round-trip byte-identically"
    );
}

/// **Both sweep nodes evaluate**, in one document, through the one
/// generic lowering, each with a full name table under its own id.
#[test]
fn both_sweeps_evaluate_in_one_document() {
    let fixture = both_sweeps();
    let ev = eval::<f64>(&fixture.doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "the two-sweep document failed: {bad:?}");
    for id in fixture.sweeps {
        let value = ev.value(id).expect("the sweep node produced a value");
        assert!(
            value.name_table.iter().count() > 0,
            "node {id:?} produced an empty name table"
        );
    }
}

/// FNV-1a 64 over a document's evaluated name tables and values — the
/// SEAT-4 feed, byte for byte, so a red here is comparable with that
/// unit's rows.
///
/// The channels it covers were enumerated off `wire_blend`'s body
/// there; `wire_swept` writes the same four — the emitted table, the
/// body the kernel verb returned, the `stamp_minted` provenance on that
/// body, and (on the refusal path) a typed error — so the same feed is
/// the right one here. The refusal path is NOT covered, exactly as it
/// was not there.
///
/// **What it deliberately does NOT feed: the per-field parameter
/// sources.** They are the channel this unit ADDS, so feeding them
/// would move every constant below and make the differential against
/// the merge base impossible to state. They are pinned in their own
/// rows, through the kernel's evidence door, where a red says what
/// actually broke.
fn digest(ev: &editor_core::Evaluation<f64>) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut feed = |bytes: &[u8]| {
        for b in bytes {
            h ^= u64::from(*b);
            h = h.wrapping_mul(0x1000_0000_01b3);
        }
    };
    for id in &ev.order {
        feed(format!("#{id:?}").as_bytes());
        let Some(value) = ev.value(*id) else { continue };
        for (name, entry) in value.name_table.iter() {
            feed(format!("{name:?}={entry:?}").as_bytes());
        }
        feed(value.payload.kind_name().as_bytes());
        if let editor_core::ValuePayload::Body(body) = &value.payload {
            feed_body(&mut feed, body);
        }
    }
    h
}

/// The body half of [`digest`], byte-for-byte the SEAT-4 feed: points
/// with their provenance stamps, the curve and surface arenas with
/// theirs, the topology's attachment both ways, and the entity census.
fn feed_body(feed: &mut impl FnMut(&[u8]), body: &Body<f64>) {
    for (key, p) in body.points() {
        for c in [p.x, p.y, p.z] {
            feed(&c.to_bits().to_be_bytes());
        }
        feed(format!("{key:?}<-{:?}", body.point_source(key)).as_bytes());
    }
    for (key, curve) in body.curves() {
        feed(format!("{key:?}{curve:?}<-{:?}", body.curve_source(key)).as_bytes());
    }
    for (key, surface) in body.surfaces() {
        feed(format!("{key:?}{surface:?}<-{:?}", body.surface_source(key)).as_bytes());
    }
    for (key, face) in body.faces() {
        let surface = body
            .get_surface(face.surface)
            .expect("a face has a carrier");
        feed(format!("{key:?}{surface:?}").as_bytes());
    }
    for (key, edge) in body.edges() {
        let curve = body
            .get_curve_geom(edge.curve)
            .expect("an edge has a curve");
        feed(format!("{key:?}{curve:?}").as_bytes());
    }
    feed(
        format!(
            "V{}E{}F{}",
            body.vertices().count(),
            body.edges().count(),
            body.faces().count()
        )
        .as_bytes(),
    );
}

/// **The sweep-carrying corpus documents' evaluations are
/// bit-identical**, body and name table, one committed number each.
///
/// The registry is FULL of extrudes — every solid in it starts as one —
/// so this is the widest differential the verb migration has had. The
/// five rows are chosen to cover the shapes the lowering can differ on:
/// `die` and `corner_table` are polygon extrudes (no carrier radius, so
/// the per-edge flow attaches nothing and the digest must be untouched
/// by it), `cut_cylinder` and `boss_union` carry the two carrier loop
/// forms (`circle` and `circle_split` — where the flow DOES attach, and
/// the digest must STILL be untouched, since a field source is not in
/// this feed), and `kitchen_sink` is the registry's revolve.
///
/// They are goldens in the ordinary sense — when one moves the question
/// is whether the new behaviour is right, never how to restore the old
/// number.
#[test]
fn the_sweep_documents_evaluate_to_their_committed_digests() {
    let rows: [(&str, u64); 5] = [
        ("die", 0x328a_2ed2_c90b_2a5a),
        ("corner_table", 0x9dca_7d2f_110c_aeb1),
        ("cut_cylinder", 0xde55_26db_e3da_32c4),
        ("boss_union", 0x837b_32e6_3076_79f1),
        ("kitchen_sink", 0xb068_5374_fa2e_2b3e),
    ];
    let mut moved: Vec<String> = Vec::new();
    for (name, want) in rows {
        let doc = corpus::documents()
            .into_iter()
            .find(|d| d.name == name)
            .expect("the document is registered");
        let ev = eval::<f64>(&doc.doc);
        let bad = failures(&ev);
        assert!(bad.is_empty(), "{name} failed to evaluate: {bad:?}");
        let got = digest(&ev);
        println!("seat7 {name}: {got:#018x}");
        if got != want {
            moved.push(format!("{name}: {got:#018x} (want {want:#018x})"));
        }
    }
    assert!(
        moved.is_empty(),
        "these documents' evaluations moved — body or name table:\n{}",
        moved.join("\n")
    );
}

// ------------------------------------------------------------------
// 2. The per-edge flow
// ------------------------------------------------------------------

/// One face of `body` whose carrier is a cylinder.
fn a_cylinder_face(body: &Body<f64>) -> FaceKey {
    topo::query::all_faces(body)
        .into_iter()
        .find(|&f| {
            body.get_face(f)
                .and_then(|fd| body.get_surface(fd.surface))
                .is_some_and(|s| matches!(s, geom::Surface::Cylinder { .. }))
        })
        .expect("an extruded circle has cylindrical walls")
}

/// One face of `body` whose carrier is a torus.
fn a_torus_face(body: &Body<f64>) -> FaceKey {
    topo::query::all_faces(body)
        .into_iter()
        .find(|&f| {
            body.get_face(f)
                .and_then(|fd| body.get_surface(fd.surface))
                .is_some_and(|s| matches!(s, geom::Surface::Torus { .. }))
        })
        .expect("a revolved circle has toroidal walls")
}

/// Whether `face`'s carrier holds a source for `field`.
fn sourced(body: &Body<f64>, face: FaceKey, field: SurfaceField) -> bool {
    let surface = body.get_face(face).expect("a live face").surface;
    body.surface_field_source(surface, field).is_some()
}

/// The evidence between one cylinder wall of each body.
fn cyl_evidence(a: &Body<f64>, b: &Body<f64>) -> RadiusEvidence {
    topo::field_source_evidence(
        a,
        a_cylinder_face(a),
        b,
        a_cylinder_face(b),
        SurfaceField::CylinderRadius,
    )
}

/// **The acceptance row for the extrude's flow: one declared radius
/// reaching two bodies' walls is `Declared` at the germ's evidence
/// door.**
///
/// Two independent extrude nodes over two profiles, one shared document
/// parameter — and the evidence the kernel reads is `Declared`,
/// computed with zero numerics from the two walls' tokens. Nothing
/// compared a radius.
#[test]
fn one_shared_radius_declares_across_two_extruded_circles() {
    let doc = doc_with_r("seat7-extrude-flow");
    let (doc, a) = cylinder(doc, param("r"));
    let (doc, b) = cylinder(doc, param("r"));
    let ev = eval::<f64>(&doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "shared-r document:\n{}", bad.join("\n"));
    let (a, b) = (body_of(&ev, a), body_of(&ev, b));
    assert!(
        sourced(a, a_cylinder_face(a), SurfaceField::CylinderRadius),
        "the extruded circle's wall carries no radius source"
    );
    assert_eq!(
        cyl_evidence(a, b),
        RadiusEvidence::Declared,
        "two walls swept from one declared radius must be declared-equal"
    );
}

/// **A different expression is a different wall.** `r` and `r/2` are
/// two expressions, so the two walls' tokens differ and the evidence is
/// `None` — the answer the general rung is for.
#[test]
fn two_radii_spelled_differently_do_not_declare() {
    let doc = doc_with_r("seat7-two-radii");
    let (doc, a) = cylinder(doc, param("r"));
    let (doc, b) = cylinder(
        doc,
        Expr::div(param("r"), Expr::literal(2.0, Dimension::Scalar).unwrap()).unwrap(),
    );
    let ev = eval::<f64>(&doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "two-radii document:\n{}", bad.join("\n"));
    assert_eq!(
        cyl_evidence(body_of(&ev, a), body_of(&ev, b)),
        RadiusEvidence::None
    );
}

/// **The same geometry with no channel is `None`, permanently.** The
/// body is built by the kernel's own doors at the same radius — the
/// hand-built and imported posture — so the values coincide exactly and
/// the evidence is still `None` (P3).
#[test]
fn a_kernel_built_cylinder_has_no_channel() {
    let doc = doc_with_r("seat7-absence");
    let (doc, a) = cylinder(doc, param("r"));
    let ev = eval::<f64>(&doc);
    let evaluated = body_of(&ev, a);
    let raw = raw_cylinder(R, H);
    assert_eq!(
        cyl_evidence(evaluated, &raw),
        RadiusEvidence::None,
        "an unsourced wall must route the general rung whatever its radius reads"
    );
    assert_eq!(
        cyl_evidence(&raw, &raw),
        RadiusEvidence::None,
        "two unsourced walls agree on nothing — absence is not identity"
    );
}

/// **A polygon profile attaches nothing, because it carries no radius
/// to attach.**
///
/// The row is the per-edge source's own emptiness statement: a chain
/// loop holds no single carrier radius, so there is no expression to
/// lower and the walls it sweeps are planes besides. An attach that
/// stamped something here would be inventing an address.
#[test]
fn a_polygon_profile_attaches_nothing() {
    let doc = ProfileDoc::empty(DocumentId::derive("seat7-polygon"), tol());
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Frame {
            origin: [len(0.0), len(0.0), len(0.0)],
            u: [scl(1.0), scl(0.0), scl(0.0)],
            v: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    );
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::polygon(square(0.0, 0.0, 0.5)).unwrap()],
        }),
    );
    let (doc, cube) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let ev = eval::<f64>(&doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "polygon document:\n{}", bad.join("\n"));
    let body = body_of(&ev, cube);
    for face in topo::query::all_faces(body) {
        for &field in SurfaceField::ALL {
            assert!(
                !sourced(body, face, field),
                "a polygon extrusion stamped {field:?} on {face:?}"
            );
        }
    }
}

/// **The revolve's half of the same flow**: a revolved circle's walls
/// are tori, and their MINOR radius is the profile circle's — carrying
/// the declared expression's identity, while the major radius (the
/// distance from the axis, which the document holds nowhere as a
/// scalar) carries nothing.
///
/// The major-radius half is the row that would fire if the declared
/// field role ever widened to "every radius the carrier stores".
#[test]
fn a_revolved_circle_sources_its_minor_radius_only() {
    let doc = doc_with_r("seat7-revolve-flow");
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Frame {
            origin: [len(0.0), len(0.0), len(0.0)],
            u: [scl(1.0), scl(0.0), scl(0.0)],
            v: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    );
    // Negative y: the door's half-plane about the +x axis is
    // `(p − origin).perp_dot(dir) ≥ 0`, which is `−y`.
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::Circle {
                centre: [len(0.0), len(-3.0)],
                radius: param("r"),
            }],
        }),
    );
    let (doc, axis) = insert(doc, axis_in_plane(plane, (0.0, 0.0), (1.0, 0.0)));
    let (doc, torus) = insert(
        doc,
        Node::Revolve {
            profile,
            axis,
            angle: ang(2.0 * PI),
        },
    );
    let ev = eval::<f64>(&doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "revolve document:\n{}", bad.join("\n"));
    let body = body_of(&ev, torus);
    let wall = a_torus_face(body);
    assert!(
        sourced(body, wall, SurfaceField::TorusMinorRadius),
        "the revolved circle's minor radius is the profile's declared one"
    );
    assert!(
        !sourced(body, wall, SurfaceField::TorusMajorRadius),
        "the major radius is the distance from the axis, which no slot holds"
    );
}

/// **The extents attach nothing**, which is what their declaredly empty
/// rows say, asserted over bodies that really ran.
///
/// What this row can and cannot separate, stated: the distance and the
/// angle reach no field, and no carrier of a POLYGON extrusion stores
/// any scalar at all — so `a_polygon_profile_attaches_nothing` already
/// covers the outcome there. What this adds is the CIRCLE case, where
/// fields exist and are stamped: the walls carry exactly one source
/// each (the radius), and the caps — planes positioned by the distance
/// — carry none. An attach that read the extent's row as reaching the
/// walls would stamp twice and red here.
#[test]
fn the_extent_slots_reach_no_field() {
    let doc = doc_with_r("seat7-extent");
    let (doc, cyl) = cylinder(doc, param("r"));
    let ev = eval::<f64>(&doc);
    let body = body_of(&ev, cyl);
    for face in topo::query::all_faces(body) {
        let sourced_fields: Vec<SurfaceField> = SurfaceField::ALL
            .iter()
            .copied()
            .filter(|&f| sourced(body, face, f))
            .collect();
        let is_wall = body
            .get_face(face)
            .and_then(|fd| body.get_surface(fd.surface))
            .is_some_and(|s| matches!(s, geom::Surface::Cylinder { .. }));
        if is_wall {
            assert_eq!(
                sourced_fields,
                vec![SurfaceField::CylinderRadius],
                "a wall carries the radius and nothing else"
            );
        } else {
            assert!(
                sourced_fields.is_empty(),
                "a cap carries {sourced_fields:?}; the distance reaches no field"
            );
        }
    }
}

/// A profile of `loops` on a frame at `z`, extruded — returns the doc,
/// the profile node and the swept body node.
fn extruded(
    doc: ProfileDoc,
    z: f64,
    loops: Vec<LoopProgram>,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Frame {
            origin: [len(0.0), len(0.0), len(z)],
            u: [scl(1.0), scl(0.0), scl(0.0)],
            v: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    );
    let (doc, profile) = insert(doc, Node::Profile(ProfileProgram { plane, loops }));
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(2.0 * H),
        },
    );
    (doc, profile, body)
}

/// A circle at the origin of the given radius expression.
fn circle_loop(radius: Expr) -> LoopProgram {
    LoopProgram::Circle {
        centre: [len(0.0), len(0.0)],
        radius,
    }
}

/// Every cylindrical face of `body`, with the radius its carrier
/// stores — the test's own way of asking WHICH wall it is holding. The
/// claim under test is read through the evidence door below; this only
/// picks the face.
fn cylinder_walls(body: &Body<f64>) -> Vec<(FaceKey, f64)> {
    topo::query::all_faces(body)
        .into_iter()
        .filter_map(|f| {
            let s = body.get_surface(body.get_face(f)?.surface)?;
            match s {
                geom::Surface::Cylinder { radius, .. } => Some((f, *radius)),
                _ => None,
            }
        })
        .collect()
}

/// The evidence between two named cylinder faces.
fn wall_evidence(a: &Body<f64>, fa: FaceKey, b: &Body<f64>, fb: FaceKey) -> RadiusEvidence {
    topo::field_source_evidence(a, fa, b, fb, SurfaceField::CylinderRadius)
}

/// **A HOLED profile authored hole-first**: every loop's walls carry
/// that loop's own radius, and the canonical→program anchor is what
/// makes that true.
///
/// Canonicalization puts the OUTER loop first whatever the author
/// wrote, so a profile authored `[hole, outer]` is a transposition:
/// canonical loop 0 is program loop 1. The walls a sweep's record
/// exports are indexed by CANONICAL loop; the expressions live at
/// PROGRAM loops. Reading the program loops in canonical order — the
/// one-line mistake available at this site — swaps the two radii here
/// and stamps every outer wall with the hole's expression, which no
/// single-loop row can see and which the germ would then read as a
/// declaration between two bodies that share no parameter.
///
/// Two pegs make it visible through the evidence door alone: a
/// single-circle extrude at `r` and another at `q`. The annulus's
/// outer walls must declare against `r`'s peg and NOT against `q`'s,
/// and its hole walls the other way round.
#[test]
fn each_loop_of_a_hole_first_profile_carries_its_own_radius() {
    let doc = doc_with_r("seat7-hole-first");
    let (doc, _) = step(
        doc,
        DocEdit::SetDocParam {
            name: ParamName::new("q"),
            value: DocParam::continuous(Dimension::Length, Q),
        },
    );
    // Hole first, deliberately.
    let (doc, _, annulus) = extruded(
        doc,
        -H,
        vec![circle_loop(param("q")), circle_loop(param("r"))],
    );
    let (doc, _, peg_r) = extruded(doc, 10.0, vec![circle_loop(param("r"))]);
    let (doc, _, peg_q) = extruded(doc, 20.0, vec![circle_loop(param("q"))]);
    let ev = eval::<f64>(&doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "hole-first document:\n{}", bad.join("\n"));
    let (annulus, peg_r, peg_q) = (
        body_of(&ev, annulus),
        body_of(&ev, peg_r),
        body_of(&ev, peg_q),
    );
    let (face_r, _) = cylinder_walls(peg_r)[0];
    let (face_q, _) = cylinder_walls(peg_q)[0];
    let walls = cylinder_walls(annulus);
    assert_eq!(walls.len(), 4, "two loops, two semicircular walls each");
    for (wall, radius) in walls {
        let ((same, same_face), (other, other_face), which) = if radius == R {
            ((peg_r, face_r), (peg_q, face_q), "an outer")
        } else {
            assert_eq!(radius, Q, "a wall at neither declared radius");
            ((peg_q, face_q), (peg_r, face_r), "a hole")
        };
        assert_eq!(
            wall_evidence(annulus, wall, same, same_face),
            RadiusEvidence::Declared,
            "{which} wall must declare against the peg at its own parameter"
        );
        assert_eq!(
            wall_evidence(annulus, wall, other, other_face),
            RadiusEvidence::None,
            "{which} wall declared against the OTHER parameter's peg"
        );
    }
}

/// **The other carrier loop form, at a subdivision the anchor cannot
/// hide behind**: `circle_split` at n = 3 mints three walls, and all
/// three carry the one expression the loop is drawn at — pairwise, and
/// against a plain circle at the same parameter.
///
/// One radius per LOOP is the whole claim the per-edge source rests
/// on, and a split is where it would break if a wall were addressed by
/// anything finer than its loop.
#[test]
fn every_wall_of_a_split_carrier_carries_the_loops_one_radius() {
    let doc = doc_with_r("seat7-split");
    let (doc, _, split) = extruded(
        doc,
        -H,
        vec![LoopProgram::CircleSplit {
            centre: [len(0.0), len(0.0)],
            radius: param("r"),
            n: 3,
            phase: ang(0.3),
        }],
    );
    let (doc, _, plain) = extruded(doc, 10.0, vec![circle_loop(param("r"))]);
    let ev = eval::<f64>(&doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "split document:\n{}", bad.join("\n"));
    let (split, plain) = (body_of(&ev, split), body_of(&ev, plain));
    let walls = cylinder_walls(split);
    assert_eq!(walls.len(), 3, "n = 3 mints three walls");
    let (plain_face, _) = cylinder_walls(plain)[0];
    for &(w, _) in &walls {
        assert_eq!(
            wall_evidence(split, w, plain, plain_face),
            RadiusEvidence::Declared,
            "a split wall must declare against the plain circle at the same parameter"
        );
        for &(w2, _) in &walls {
            assert_eq!(
                wall_evidence(split, w, split, w2),
                RadiusEvidence::Declared,
                "two walls of one loop must declare against each other"
            );
        }
    }
}

/// One evaluation, optionally served from a prior one.
fn memo_eval(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        prior,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol(),
    )
}

/// **The memo never serves a sweep a token the document no longer
/// holds** — SEAT-6's stale-token row, for the source the OPERAND
/// carries.
///
/// SEAT-6 closed this class for a verb's own slot, and the key feed
/// for the profile's carrier radius (format v5) is the same fix at the
/// node that HOLDS the expression. What pins it there today is a key
/// INEQUALITY (`switch_program_key::resolved_values_feed_the_key`),
/// which is a fact about a hash and not about a served body. This is
/// the served-body row: A and B extrude circles at `r`, then A's
/// profile radius is re-spelled as the literal of the same value. The
/// geometry is bit-identical, so a key over resolved values alone
/// would hand A's profile — and with it A's extrude — straight back
/// out of the memo, carrying `r`'s token while the document says A is
/// a literal. The evidence would read `Declared` between two walls
/// that share no expression, and when `r` then moves, B re-runs at the
/// new radius while A stays at the old one: two radii under one token.
///
/// The third step runs that move, so the row fails on the value as
/// well as on the channel if the memo ever does serve the stale entry.
#[test]
fn the_memo_never_serves_a_stale_sweep_token() {
    let doc = doc_with_r("seat7-memo");
    let (doc, profile_a, a) = extruded(doc, -H, vec![circle_loop(param("r"))]);
    let (doc, _, b) = extruded(doc, 10.0, vec![circle_loop(param("r"))]);
    let ev1 = memo_eval(&doc, None);
    assert!(failures(&ev1).is_empty(), "{:?}", failures(&ev1));
    assert_eq!(
        cyl_evidence(body_of(&ev1, a), body_of(&ev1, b)),
        RadiusEvidence::Declared,
        "two circles at one parameter declare"
    );

    // A's carrier radius becomes the LITERAL of the same value.
    let (doc, _) = step(
        doc,
        DocEdit::SetParam {
            node: profile_a,
            slot: SlotId::Profile {
                loop_: 0,
                step: 0,
                arg: StepArg::Radius,
            },
            expr: len(R),
        },
    );
    let ev2 = memo_eval(&doc, Some(&ev1));
    assert!(failures(&ev2).is_empty(), "{:?}", failures(&ev2));
    assert!(
        ev2.reused > 0,
        "B's half of the document is memo-served, or the row proves nothing about the memo"
    );
    assert_eq!(
        cyl_evidence(body_of(&ev2, a), body_of(&ev2, b)),
        RadiusEvidence::None,
        "A is a literal in the document and B is `r`; a memo-served wall would still say `r`"
    );

    // Now move `r`. B re-runs at the new radius; A must not be left at
    // the old one under a token that claims `r`.
    let (doc, _) = step(
        doc,
        DocEdit::SetDocParam {
            name: ParamName::new("r"),
            value: DocParam::continuous(Dimension::Length, 2.0 * R),
        },
    );
    let ev3 = memo_eval(&doc, Some(&ev2));
    assert!(failures(&ev3).is_empty(), "{:?}", failures(&ev3));
    let (ba, bb) = (body_of(&ev3, a), body_of(&ev3, b));
    let (ra, rb) = (cylinder_walls(ba)[0].1, cylinder_walls(bb)[0].1);
    assert!(
        (ra - rb).abs() > 1e-9,
        "the fixture needs two radii after the move: {ra} vs {rb}"
    );
    assert_eq!(
        cyl_evidence(ba, bb),
        RadiusEvidence::None,
        "radii {ra} vs {rb} under one token would be a document-reachable contradiction"
    );
}

// ------------------------------------------------------------------
// 3. The germ, end to end
// ------------------------------------------------------------------

/// The kernel-direct twin of [`cylinder`]: the same profile, the same
/// extrude, at the same radius — and no recipe layer above them, so no
/// records anywhere.
fn raw_cylinder(r: f64, h: f64) -> Body<f64> {
    let lp = profile::circle(Point2::new(0.0, 0.0), r, tol()).unwrap();
    let plane = profile::SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, -h)));
    let sketch = profile::Profile::new(plane, vec![lp.into()])
        .validate(tol())
        .unwrap();
    sweep::extrude(&sketch, sweep::Extrusion::Distance(2.0 * h), tol())
        .unwrap()
        .body
}

/// A kernel-direct rigid spin, for the twin.
fn raw_spin(b: &Body<f64>, axis: Vec3<f64>, angle: f64) -> Body<f64> {
    topo::transform_rigid(
        b,
        &Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), axis, angle),
        tol(),
    )
    .unwrap()
}

/// The evidence a cyl×cyl pinch refusal carries, or a loud failure.
fn pinch_evidence(err: &BooleanError) -> RadiusEvidence {
    match err {
        BooleanError::GermFrameCylinderPinch { evidence, .. } => *evidence,
        other => panic!("the equal-radius pair refused with {other:?}, not the pinch"),
    }
}

/// **THE END-TO-END ROW** (VERB-SEAT-DESIGN §6's second acceptance,
/// and the item `seat6-germ-end-to-end-awaits-seat7` waited on): one
/// declared radius parameter, two extruded circles, one boolean — and
/// the germ reads `Declared`.
///
/// The pose is the germ fixture's own (`sweep`'s `verbs_germarms2`):
/// the classic Steinmetz pair never reaches the join at all, because
/// both operands' seams sit ON the pinch points and die a layer earlier
/// at the tangency door, so each operand is spun about its OWN axis —
/// a motion a cylinder of revolution is invariant under, which moves
/// the charts and not the surfaces.
///
/// What the union RETURNS is a refusal, and that is not a weakness of
/// the row: the equal-radius intersecting-axes locus is two ellipses
/// crossing at two valence-4 pinch vertices, which is not one conic and
/// therefore has no frame to hand over. The refusal carries the
/// evidence, so what is pinned is exactly the channel: `Declared` says
/// the kernel PROVED the configuration from the recipe's own
/// parameter, with no radius ever compared.
#[test]
fn one_declared_radius_reaches_the_germ_from_a_document() {
    let doc = doc_with_r("seat7-germ");
    let (doc, a) = cylinder(doc, param("r"));
    let (doc, b) = cylinder(doc, param("r"));
    // B's axis becomes +y; then each is spun about its own axis to take
    // the seams off the pinch.
    let (doc, b) = spin(doc, b, [1.0, 0.0, 0.0], PI / 2.0);
    let (doc, a) = spin(doc, a, [0.0, 0.0, 1.0], PHI);
    let (doc, b) = spin(doc, b, [0.0, 1.0, 0.0], PHI);
    let (doc, union) = insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a,
            b,
            declare: None,
        },
    );
    let ev = eval::<f64>(&doc);
    let evidence = match ev.nodes.get(&union) {
        Some(editor_core::NodeResult::Failed(e)) => match &e.kind {
            editor_core::NodeErrorKind::Boolean(err) => pinch_evidence(err),
            other => panic!("the union refused with {other:?}, not a boolean refusal"),
        },
        other => panic!("the union did not refuse: {other:?}"),
    };
    assert_eq!(
        evidence,
        RadiusEvidence::Declared,
        "one declared parameter must reach the germ as a declaration"
    );

    // The twin: the same two solids at bit-identical radii, built
    // through the kernel's own doors with no recipe above them.
    let raw_a = raw_spin(&raw_cylinder(R, H), Vec3::new(0.0, 0.0, 1.0), PHI);
    let raw_b = raw_spin(
        &raw_spin(&raw_cylinder(R, H), Vec3::new(1.0, 0.0, 0.0), PI / 2.0),
        Vec3::new(0.0, 1.0, 0.0),
        PHI,
    );
    let raw = topo::union(&raw_a, &raw_b, tol()).expect_err("this family has no join arm");
    assert_eq!(
        pinch_evidence(&raw),
        RadiusEvidence::None,
        "a hand-built pair declares nothing, whatever its radii read"
    );
}
