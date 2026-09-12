//! **A deferral has to answer the question it defers.**
//!
//! The census's containment arm (arm 2) reports where one instance
//! sits relative to another. It used to SKIP any solid pair linked by
//! a contact record, justified by *"under the confirm pass's
//! examination"*. The confirm pass asks of each record whether that
//! record's own coincidence is geometrically real — vv a coincident
//! vertex pair, v-on-f a vertex in a face's region, `curves` a tangent
//! locus, `patches` a conformal overlap. None of the four is a
//! question about placement, and no variant of `ContactRecords` states
//! one, so the pass the arm deferred to never asked the arm's
//! question.
//!
//! The consequence was live: an instance sitting inside another's
//! material, with its contact declared TRUTHFULLY, validated clean —
//! and the same body with the contact UNDECLARED refused. Declaring a
//! true contact is what switched the examination off.
//!
//! These rows are all-planar, so they build in `topo`'s own suite. The
//! sibling narrowing in arm 1 — a v-on-f deferral that named the other
//! SOLID where the finding is about the other FACE — needs an
//! arc-bounded planar face and is pinned in
//! `sweep/tests/s49_census_jurisdiction.rs` beside the jurisdiction
//! rows it belongs with.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::Point3;
use geom_core::Tol;
use topo::{
    Body, CensusContact, ContactRecords, EntityId, FaceKey, LoopBoundary, ValidationError,
    VertexKey, VfContact, VvContact, validate_pseudomanifold,
};

/// A cube of side `side` with its minimum corner at `(dx, dy, dz)`.
fn cube(side: f64, dx: f64, dy: f64, dz: f64) -> Body<f64> {
    common::mapped_cube(|x, y, z| Point3::new(side * x + dx, side * y + dy, side * z + dz))
}

/// The pair as one two-instance arena.
fn assembly(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let mut out = a.clone();
    topo::graft_disjoint(&mut out, b, Tol::witness()).unwrap();
    out
}

/// Every boundary vertex of `f`, as (key, point).
fn face_verts(body: &Body<f64>, f: FaceKey) -> Vec<(VertexKey, Point3<f64>)> {
    let face = body.get_face(f).unwrap();
    let mut out = Vec::new();
    for &lk in core::iter::once(&face.outer).chain(&face.rings) {
        let l = body.get_loop(lk).unwrap();
        let LoopBoundary::Cycle { first } = l.boundary else {
            // A lone-vertex loop has no cycle to walk. These fixtures
            // are cubes, which have none; the skip is a shape
            // requirement of the walk, not a judgement that an empty
            // loop carries nothing, which is the reading that turns a
            // shape requirement into a silent skip.
            continue;
        };
        for he in body.loop_cycle(first).unwrap() {
            let v = body.get_half_edge(he).unwrap().start;
            let p = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            out.push((v, p));
        }
    }
    out
}

/// The one face of `body` all of whose boundary vertices satisfy
/// `pick` — the fixtures below are built so exactly one does.
fn the_face(body: &Body<f64>, pick: impl Fn(Point3<f64>) -> bool) -> FaceKey {
    let hits: Vec<FaceKey> = body
        .faces()
        .map(|(f, _)| f)
        .filter(|&f| {
            let vs = face_verts(body, f);
            vs.len() == 4 && vs.iter().all(|&(_, p)| pick(p))
        })
        .collect();
    assert_eq!(hits.len(), 1, "expected exactly one such face: {hits:?}");
    hits[0]
}

/// Does some `CensusUndecidable` name the two SOLIDS of a two-instance
/// arena with the given verdict? Arm 2 is the only arm that reports at
/// solid granularity, so such a finding IS the containment arm's — but
/// it mints THREE distinct verdicts (definite containment, in-band, and
/// an unclaimable container extent), and a row that accepts any of them
/// gets easier as the guarantee degrades. `want` is matched as a
/// substring of the refusal's `what`, so each row names the verdict it
/// means.
fn containment_refused(errors: &[ValidationError], want: &str) -> bool {
    errors.iter().any(|e| {
        matches!(
            e,
            ValidationError::CensusUndecidable {
                a: EntityId::Solid(_),
                b: EntityId::Solid(_),
                what,
            } if what.contains(want)
        )
    })
}

/// The verdict the embedded fixture must draw: its extents are in a
/// containment relation the arm cannot definitely separate. Not the
/// weaker "unclaimable extent" verdict, which would mean the arm never
/// compared the two solids at all.
const IN_BAND: &str = "not definitely separable from containment";

/// **The embedded-instance fixture.** A 1 m cube sitting wholly inside
/// a 4 m cube, flush at `z = 0`, with its four bottom corners declared
/// v-on-f on the big cube's bottom face. Every record is geometrically
/// true — those corners really do lie in that face's region — so the
/// confirm pass has nothing to say about any of them, which is what
/// makes this fixture different from a bogus bridge.
///
/// The inner cube is inside the outer's MATERIAL. The census cannot
/// decide that (interference is C6's class); arm 2 is what says so.
fn embedded() -> (Body<f64>, ContactRecords) {
    let body = assembly(&cube(4.0, 0.0, 0.0, 0.0), &cube(1.0, 1.0, 1.0, 0.0));
    // The 4 m cube's bottom face: the only face whose four corners all
    // sit at z = 0 and OUTSIDE the inner cube's footprint (the inner
    // cube's own bottom face is the other all-at-z-0 face).
    let big_bottom = the_face(&body, |p| p.z.abs() < 1e-12 && !(0.9..2.1).contains(&p.x));
    let mut records = ContactRecords::default();
    for (v, p) in body
        .faces()
        .map(|(f, _)| f)
        .flat_map(|f| face_verts(&body, f))
    {
        let inner_corner =
            p.z.abs() < 1e-12 && (0.9..2.1).contains(&p.x) && (0.9..2.1).contains(&p.y);
        let rec = VfContact {
            vertex: v,
            face: big_bottom,
        };
        if inner_corner && !records.b_on_a.contains(&rec) {
            records.b_on_a.push(rec);
        }
    }
    assert_eq!(
        records.b_on_a.len(),
        4,
        "the fixture's precondition: four declared inner corners"
    );
    (body, records)
}

/// **The regression row.** The embedded pair's containment question is
/// examined whether or not its contact is declared.
///
/// The second assertion is what stops it passing for the wrong reason.
/// A bridged pair can always be made loud by giving it a BOGUS record
/// — the confirm pass refuses that — and the only row this skip ever
/// had (`m9_2_census_door::r1_delta_probe_bridged_nested_pair_stays_loud`)
/// is exactly that shape. So this row requires that **no record here is
/// refuted**: no `StaleContactDeclaration`, no `ContactContradicted`.
/// The loudness it measures can only come from arm 2.
#[test]
fn an_embedded_instance_is_examined_though_its_contact_is_declared() {
    let (body, records) = embedded();
    let errors = validate_pseudomanifold(&body, &records, Tol::witness())
        .expect_err("an instance inside another's material must never clear");
    assert!(
        !errors.iter().any(|e| matches!(
            e,
            ValidationError::StaleContactDeclaration { .. }
                | ValidationError::ContactContradicted { .. }
        )),
        "the fixture's precondition: every declared contact confirms, so the \
         loudness cannot be borrowed from a refuted record: {errors:?}"
    );
    assert!(
        containment_refused(&errors, IN_BAND),
        "the containment arm must name the solid pair, with the verdict about \
         THESE two extents rather than a weaker one: {errors:?}"
    );
    // And the declaration did its own job: the four corner rests it
    // names are no longer reported as undeclared contacts, so the
    // refusal above is not the vertex sweeps' either.
    assert!(
        !errors.iter().any(|e| matches!(
            e,
            ValidationError::UndeclaredContact {
                contact: CensusContact::VertexOnFace { .. },
                ..
            }
        )),
        "the records back their own vertex-on-face events: {errors:?}"
    );
}

/// The same fixture UNDECLARED, which is what the arm always did. It
/// is here so the row above reads as a statement about the deferral
/// rather than about the fixture: the two verdicts must agree on the
/// containment question, and before the deferral was removed they did
/// not.
#[test]
fn the_embedded_pair_is_refused_undeclared_too() {
    let (body, _) = embedded();
    let errors = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness())
        .expect_err("the undeclared twin refuses");
    assert!(
        containment_refused(&errors, IN_BAND),
        "the containment arm names the solid pair, same verdict, with no records \
         at all: {errors:?}"
    );
}

/// **The other direction, so the rows above cannot pass by refusing
/// every declared assembly.** Two cubes stacked face to face, with the
/// interface declared at its four coincident corners: the extents are
/// definitely separable in `z`, arm 2 clears them, and the body
/// validates with no findings at all.
///
/// This is the acceptance class the old skip was protecting, and it
/// never needed the skip — it clears on the arm's own margin.
#[test]
fn a_declared_stacked_assembly_still_clears_at_the_containment_arm() {
    let body = assembly(&cube(1.0, 0.0, 0.0, 0.0), &cube(1.0, 0.0, 0.0, 1.0));
    let mut records = ContactRecords::default();
    for corner in [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)] {
        let hits: Vec<VertexKey> = body
            .vertices()
            .filter(|(_, v)| {
                let p = body.get_point(v.point).unwrap();
                (p.x - corner.0).abs() < 1e-12
                    && (p.y - corner.1).abs() < 1e-12
                    && (p.z - 1.0).abs() < 1e-12
            })
            .map(|(k, _)| k)
            .collect();
        let [a, b] = hits[..] else {
            panic!("exactly two coincident corners at {corner:?}: {hits:?}");
        };
        records.vv.push(VvContact { a, b });
    }
    assert_eq!(
        validate_pseudomanifold(&body, &records, Tol::witness()),
        Ok(()),
        "a declared stacked assembly clears on the containment arm's own margin"
    );
}
