//! **The parameter→field flow, executed against the birth record.**
//!
//! The declaration in `verbs::flow` is plain data with no consumer, so
//! nothing in a normal build would notice if it named a role family the
//! operation does not mint, or forgot a parameter entirely. These rows
//! are what make it a checked claim instead of a comment:
//!
//! 1. **Every scalar parameter has exactly one row.** Computed over
//!    `ScalarParam::ALL`, so a verb the vocabulary gains is measured
//!    the moment its parameter is named — never a per-verb count copied
//!    into an assertion.
//! 2. **Every field a row names belongs to the verb whose row it is**,
//!    and no field is claimed twice within a row.
//! 3. **Every role family a row names is one the birth record really
//!    mints** — run the verb on a fixture that exercises the family and
//!    read the record back. This is the row that would fire if the
//!    surgery stopped minting a family, or if a flow row were written
//!    from the docs rather than from the code.
//!
//! What these rows do NOT check, stated so the next reader does not
//! over-trust them: that the parameter's VALUE is what lands in the
//! named field. The record carries keys, not carriers, so reading the
//! stored radius back would mean re-deriving which surface each face
//! sits on — a geometry assertion the blend suites in `sweep` already
//! own (`m6_5_fillet_naming`, `blend_tworims`). What is checked here is
//! the correspondence's SHAPE: parameters covered, families real.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use geom_core::Tol;
use sweep::blend::naming::BlendNaming;
use topo::Body;
use verbs::{RoleFamily, ScalarParam, Verb, VerbKind};

fn tol() -> Tol {
    Tol::witness()
}

/// Every scalar parameter in the vocabulary is named by exactly one
/// flow row, on the verb it belongs to.
#[test]
fn every_scalar_parameter_has_one_flow_row() {
    let mut rows: Vec<ScalarParam> = Vec::new();
    for kind in VerbKind::ALL {
        for flow in kind.param_flow() {
            assert_eq!(
                flow.param.verb(),
                *kind,
                "{:?}'s flow declares {:?}, which belongs to {:?}",
                kind,
                flow.param,
                flow.param.verb()
            );
            rows.push(flow.param);
        }
    }
    rows.sort_unstable();
    let mut expected: Vec<ScalarParam> = ScalarParam::ALL.to_vec();
    expected.sort_unstable();
    assert_eq!(
        rows, expected,
        "the flow declarations do not cover the scalar-parameter census exactly once each"
    );
}

/// No row names the same field twice — a duplicate would make a
/// consumer attach the same source to one field twice and read as
/// evidence of two.
#[test]
fn no_flow_row_repeats_a_field() {
    for kind in VerbKind::ALL {
        for flow in kind.param_flow() {
            let distinct: BTreeSet<_> = flow.fields.iter().copied().collect();
            assert_eq!(
                distinct.len(),
                flow.fields.len(),
                "{:?}'s {:?} row repeats a field: {:?}",
                kind,
                flow.param,
                flow.fields
            );
        }
    }
}

/// Which role families a record actually filled.
fn minted(rec: &BlendNaming) -> BTreeSet<RoleFamily> {
    let mut out = BTreeSet::new();
    if !rec.blends.is_empty() {
        out.insert(RoleFamily::Blends);
    }
    if !rec.corners.is_empty() {
        out.insert(RoleFamily::Corners);
    }
    if !rec.bands.is_empty() {
        out.insert(RoleFamily::Bands);
    }
    out
}

fn record(verb: &Verb<f64>, operand: &Body<f64>) -> BlendNaming {
    verb.run(operand, tol())
        .expect("the fixture is inside the door")
        .naming
        .expect("the surgery always keeps records")
}

/// **The fillet's flow names only families it mints**, measured over
/// two fixtures because no single one carries all three: the cube's
/// open chains mint the blend bands and the corner spheres, the dome's
/// closed rim mints the torus band.
#[test]
fn the_fillets_flow_names_families_the_record_mints() {
    let cube = sweep::test_support::cube(1.0, tol());
    let cube_edges: Vec<_> = cube.edges().map(|(k, _)| k).collect();
    let open = record(
        &Verb::Fillet {
            edges: cube_edges,
            radius: 0.1,
        },
        &cube,
    );

    let dome = sweep::test_support::dome(1.0, tol());
    let rim = sweep::test_support::closed_plane_sphere_rim(&dome, 1.0);
    let closed = record(
        &Verb::Fillet {
            edges: vec![rim],
            radius: 0.05,
        },
        &dome,
    );

    let mut mints = minted(&open);
    mints.extend(minted(&closed));
    assert!(
        mints.contains(&RoleFamily::Bands),
        "the dome rim minted no band; the fixture no longer exercises the closed-chain family"
    );

    for flow in VerbKind::Fillet.param_flow() {
        for field in flow.fields {
            assert!(
                mints.contains(&field.family()),
                "the fillet's flow names {:?}, whose family {:?} no run mints",
                field,
                field.family()
            );
        }
    }
}

/// **The chamfer's flow is empty, and that is a claim about the run.**
/// The row exists (checked above), and the record it would have to name
/// a family in is produced here — so the row's emptiness is a statement
/// made beside a real result rather than an untested constant.
#[test]
fn the_chamfers_flow_is_empty_beside_a_real_record() {
    let cube = sweep::test_support::cube(1.0, tol());
    let edges: Vec<_> = cube.edges().map(|(k, _)| k).collect();
    let rec = record(
        &Verb::Chamfer {
            edges,
            distance: 0.1,
        },
        &cube,
    );
    assert!(
        !minted(&rec).is_empty(),
        "the chamfer minted nothing; the fixture no longer runs the op"
    );

    let rows = VerbKind::Chamfer.param_flow();
    assert_eq!(
        rows.len(),
        1,
        "the setback's row must be present, not absent"
    );
    assert!(
        rows[0].fields.is_empty(),
        "the setback reaches no stored field: the chamfer's carriers are planes"
    );
}

/// A built verb's flow is its kind's — the payload does not enter it.
#[test]
fn the_flow_is_a_function_of_the_verb_name_only() {
    let a = Verb::Fillet {
        edges: Vec::new(),
        radius: 1.0_f64,
    };
    let b = Verb::Fillet {
        edges: Vec::new(),
        radius: 2.0_f64,
    };
    assert_eq!(a.param_flow(), b.param_flow());
    assert_eq!(a.param_flow(), VerbKind::Fillet.param_flow());
}
