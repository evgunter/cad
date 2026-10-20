//! **A face inside an edge's material wedge refuses at tier 3′.**
//!
//! Where more than two sheets meet on a line, each creased material
//! sector between two consecutive sheets is one edge, and the sectors
//! are disjoint, so no face passes through an edge's material wedge
//! (`crates/topo/README.md` tier 3). Tier 3′ reads an edge lying in a
//! face of its own solid for sides.
//!
//! - **The keel in the floor.** The slab `[0, 4]² × [0, 1]` less a
//!   V-trench whose keel rests on the floor `z = 0`, wall to wall, built
//!   as one prism: the keel is one edge between the two walls lying
//!   inside the floor, its wedge about 323° of material where the
//!   material is two wedges with the floor between them. With the
//!   records its contacts need, tier 3′ refuses it with the keel's
//!   `FaceThroughEdgeWedge` alone.
//! - **The Boolean's slit form passes.** The same slab less the trench,
//!   closed at both ends, open at one and wall to wall, and the slab
//!   less a glued tip with the tool's records carried: the Boolean parts
//!   each keel into the slit form (`topo::boolean::slit`), which tier 3′
//!   passes.
//! - **A ridge resting on its own floor.** The C-shaped block less a slot,
//!   united with a ridge hanging from the roof whose keel rests on the
//!   floor: one solid, the keel lying in the floor with its record, its
//!   wedge outside the floor's material. Tier 3′ passes it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::Pose;
use geom_core::Tol;
use topo::readback::{face_pose, vertex_point};
use topo::{
    AtRestBody, Body, BooleanInputs, BooleanOp, BooleanResult, Cell, CensusContact, Cited, Cites,
    Contact, ContactRecords, ValidationError, subtract, union, validate_pseudomanifold,
};

fn t() -> Tol {
    Tol::witness()
}

/// The prism over `profile` across `span`, its frame mapped by `map`.
fn prism(
    what: &str,
    profile: &[(f64, f64)],
    span: (f64, f64),
    map: &dyn Fn(f64, f64, f64) -> [f64; 3],
) -> AtRestBody<f64> {
    use topo::test_support::{FaceGeometry, describe_as_intersections, finished, prism_ops};
    let tol = t();
    let pose = Pose::rest();
    let mut b = Body::<f64>::new();
    prism_ops(
        &mut b,
        profile,
        span,
        |a, c, h| pose.at(map(a, c, h)),
        FaceGeometry::Certified,
        tol,
    );
    describe_as_intersections(&mut b, tol);
    finished(what, b, tol)
}

/// A prism along x over the (y, z) profile.
fn along_x(what: &str, profile: &[(f64, f64)], span: (f64, f64)) -> AtRestBody<f64> {
    prism(what, profile, span, &|y, z, x| [x, y, z])
}

/// The result body of a boolean that builds one.
fn built(what: &str, r: Result<BooleanResult<f64>, topo::BooleanError>) -> topo::BooleanBody<f64> {
    match r {
        Ok(BooleanResult::Body(b)) => b,
        other => panic!("{what}: builds, got {other:?}"),
    }
}

/// Tier 3′ refuses `body` once, with `FaceThroughEdgeWedge` for the
/// edge along x at `(y, z) = keel` and the face `z = keel.1` it lies in.
fn keel_refuses(what: &str, body: &Body<f64>, records: &ContactRecords, (y, z): (f64, f64)) {
    let errors = validate_pseudomanifold(body, records, t())
        .expect_err(&format!("{what}: tier 3′ refuses the keel in the floor"));
    for e in &errors {
        let ValidationError::FaceThroughEdgeWedge {
            edge,
            face,
            witness,
        } = e
        else {
            panic!("{what}: only the keel's wedge refuses, got {e:?}");
        };
        let plus = body
            .get_half_edge(body.get_edge(*edge).unwrap().he_plus)
            .unwrap();
        let ends = [plus.start, body.get_half_edge(plus.next).unwrap().start]
            .map(|v| vertex_point(body, v).unwrap());
        assert!(
            ends.iter()
                .all(|p| (p.y - y).abs() < 1e-12 && (p.z - z).abs() < 1e-12),
            "{what}: the edge is the keel, got {ends:?} at {witness}"
        );
        let floor = face_pose(body, *face).unwrap();
        assert!(
            floor.axis.z.abs() > 1.0 - 1e-12 && (floor.origin.z - z).abs() < 1e-12,
            "{what}: the face is the floor, got {floor:?}"
        );
    }
    assert_eq!(errors.len(), 1, "{what}: one keel, one refusal: {errors:?}");
}

/// The records the census names `body`'s contacts unrecorded by, each
/// a vertex on an edge or an edge lying in a face.
fn recorded(what: &str, body: &Body<f64>) -> ContactRecords {
    let errors = validate_pseudomanifold(body, &ContactRecords::default(), t())
        .expect_err(&format!("{what}: the keel's contacts are unrecorded"));
    ContactRecords::from_iter(errors.iter().map(|e| {
        let (x, y) = match e {
            ValidationError::UnrecordedContact {
                contact: CensusContact::VertexOnEdge { vertex, edge },
                ..
            } => (Cell::Vertex(*vertex), Cell::Edge(*edge)),
            ValidationError::UnrecordedContact {
                contact: CensusContact::EdgeFaceOverlap { edge, face },
                ..
            } => (Cell::Edge(*edge), Cell::Face(*face)),
            _ => panic!("{what}: only the keel's contacts stand, got {e:?}"),
        };
        Cited::new(Contact::new(body, x, y).unwrap(), Cites::decided(0))
    }))
}

#[test]
fn the_keel_in_the_floor_refuses_at_tier_3_prime() {
    let what = "the keel in the floor";
    let slab = along_x(
        what,
        &[
            (0.0, 0.0),
            (4.0, 0.0),
            (4.0, 1.0),
            (2.3, 1.0),
            (2.0, 0.0),
            (1.7, 1.0),
            (0.0, 1.0),
        ],
        (0.0, 4.0),
    );
    let records = recorded(what, &slab);
    assert_eq!(
        records.edge_face().count(),
        1,
        "{what}: the keel lies in the floor: {records:?}"
    );
    keel_refuses(what, &slab, &records, (2.0, 0.0));
}

#[test]
fn the_booleans_slit_keel_passes_tier_3_prime() {
    let tol = t();
    let slab = prism(
        "the slab",
        &[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)],
        (0.0, 1.0),
        &|x, y, z| [x, y, z],
    );
    for (what, span) in [
        ("both ends closed", (1.5, 2.5)),
        ("one end open", (1.5, 4.5)),
        ("wall to wall", (-0.5, 4.5)),
    ] {
        let trench = along_x("a trench", &[(2.0, 0.0), (2.6, 2.0), (1.4, 2.0)], span);
        let r = built(what, subtract(&slab, &trench, tol));
        assert_eq!(
            validate_pseudomanifold(&r.body, &r.contacts, tol),
            Ok(()),
            "{what}: the slit keel's wedges lie on either side of the floor"
        );
    }
}

#[test]
fn a_ridge_resting_on_its_own_solids_floor_passes_tier_3_prime() {
    let tol = t();
    let block = prism(
        "the block",
        &[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)],
        (0.0, 3.0),
        &|x, y, z| [x, y, z],
    );
    let slot = along_x(
        "the slot",
        &[(-1.0, 1.0), (3.0, 1.0), (3.0, 2.5), (-1.0, 2.5)],
        (-1.0, 5.0),
    );
    let c = built("the block less the slot", subtract(&block, &slot, tol));
    let ridge = along_x("a ridge", &[(1.5, 1.0), (2.1, 2.5), (0.9, 2.5)], (1.0, 3.0));
    let r = built("the ridge on the floor", union(&c.body, &ridge, tol));
    assert_eq!(r.body.solids().count(), 1, "one solid");
    assert_eq!(
        r.contacts.edge_face().count(),
        1,
        "the keel lies in the floor on record: {:?}",
        r.contacts
    );
    assert_eq!(
        validate_pseudomanifold(&r.body, &r.contacts, tol),
        Ok(()),
        "the keel's wedge lies outside the floor's material"
    );
}

#[test]
fn a_glued_tip_subtracted_with_its_records_passes_tier_3_prime() {
    let tol = t();
    let base = prism(
        "the tool's slab",
        &[(0.0, -1.0), (4.0, -1.0), (4.0, 3.0), (0.0, 3.0)],
        (0.0, 1.0),
        &|x, y, z| [x, y, z],
    );
    let tip = along_x("the tip", &[(1.0, 1.0), (1.6, 3.0), (0.4, 3.0)], (1.5, 3.5));
    let tool = built("the glued tip", union(&base, &tip, tol));
    assert_eq!(tool.body.solids().count(), 2, "the tip rests on the slab");
    assert_eq!(
        validate_pseudomanifold(&tool.body, &tool.contacts, tol),
        Ok(()),
        "the tip's keel lies on the slab's top from outside its wedge"
    );
    let inputs = BooleanInputs {
        records_b: tool.contacts.carried(),
        ..BooleanInputs::default()
    };
    for (what, from, to) in [
        ("a slab", (-1.0, -2.0, 0.0), (5.0, 4.0, 1.5)),
        ("a box round the tool", (-1.0, -2.0, -1.0), (5.0, 4.0, 4.0)),
    ] {
        let a = prism(
            what,
            &[
                (from.0, from.1),
                (to.0, from.1),
                (to.0, to.1),
                (from.0, to.1),
            ],
            (from.2, to.2),
            &|x, y, z| [x, y, z],
        );
        let r = built(
            what,
            topo::boolean_op_with(
                BooleanOp::Subtract,
                &a,
                &tool.body,
                &inputs,
                topo::SweepStrategy::Realized,
                tol,
            ),
        );
        assert_eq!(
            validate_pseudomanifold(&r.body, &r.contacts, tol),
            Ok(()),
            "{what}: the slit keel's wedges lie on either side of the pocket's floor"
        );
    }
}
