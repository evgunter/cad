//! **The parameter→field flow, executed against the birth record.**
//!
//! The declaration in `verbs::flow` is plain data with no consumer, so
//! nothing in a normal build would notice if it named a role family the
//! operation does not mint, or forgot a parameter entirely. These rows
//! are what make it a checked claim instead of a comment:
//!
//! 1. **Every scalar parameter has exactly one row, and every flow
//!    SOURCE is declared somewhere.** Computed over `ScalarParam::ALL`
//!    and `FlowSource::ALL`, so a verb the vocabulary gains is measured
//!    the moment its parameter is named — never a per-verb count copied
//!    into an assertion. The two censuses differ in shape because the
//!    two source kinds do: a verb scalar belongs to exactly one verb, so
//!    it is declared exactly once, while an operand-carried scalar is
//!    declared by every verb that sweeps that operand — the extrude and
//!    the revolve both name the profile edge's radius.
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

use geom_core::{Affine3, Vec3};
use sweep::Revolution;
use sweep::blend::naming::BlendNaming;
use topo::{Body, BooleanDeclarations, BooleanOp, SweepStrategy};
use verbs::{
    Arity, EdgeScalar, FlowSource, PairOut, RoleFamily, ScalarParam, Verb, VerbKind, VerbRecord,
};

use crate::fixture::{disc, offset_disc, tol, x_axis, z_plane};

/// Every scalar parameter in the vocabulary is named by exactly one
/// flow row, on the verb it belongs to.
#[test]
fn every_scalar_parameter_has_one_flow_row() {
    let mut rows: Vec<ScalarParam> = Vec::new();
    for kind in VerbKind::ALL {
        for flow in kind.param_flow() {
            let FlowSource::Param(param) = flow.source else {
                continue;
            };
            assert_eq!(
                param.verb(),
                *kind,
                "{:?}'s flow declares {:?}, which belongs to {:?}",
                kind,
                param,
                param.verb()
            );
            rows.push(param);
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

/// **Every source in the vocabulary is declared by some verb, and no
/// verb declares one twice.**
///
/// The "exactly once" census above cannot cover an operand-carried
/// source: the profile edge's radius belongs to the OPERAND, so every
/// verb that sweeps a profile declares it and the count is two, not
/// one. What is checkable — and what a dead vocabulary entry would
/// break — is coverage: a source nothing declares is a name with no
/// meaning, and a source declared twice by one verb would make a
/// consumer stamp one field from two rows.
#[test]
fn every_flow_source_is_declared_and_never_twice_by_one_verb() {
    let mut seen: BTreeSet<FlowSource> = BTreeSet::new();
    for kind in VerbKind::ALL {
        let mut here: BTreeSet<FlowSource> = BTreeSet::new();
        for flow in kind.param_flow() {
            assert!(
                here.insert(flow.source),
                "{kind:?} declares {:?} twice",
                flow.source
            );
        }
        seen.extend(here);
    }
    let all: BTreeSet<FlowSource> = FlowSource::ALL.iter().copied().collect();
    assert_eq!(
        seen, all,
        "the declared sources are not the source vocabulary"
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
                flow.source,
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
    let out = verb
        .run(operand, tol())
        .expect("the fixture is inside the door");
    let VerbRecord::Blend(naming) = out.record else {
        panic!("a blend run produced another family's record");
    };
    naming.expect("the surgery always keeps records")
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

/// **The boolean's flow has no rows, and that is a claim beside a real
/// record.** A crossing union is run and its birth record read back —
/// non-trivial (the seam survives into it) — so "no scalar parameter
/// lands in anything this record names" is a statement about an actual
/// result rather than an untested constant. The boolean HAS no scalar
/// parameters: its payload is the op selector and declared references
/// (arena keys), so there is no `ScalarParam` to write a row for — the
/// emptiness is one level up from the chamfer's (a scalar that reaches
/// no field) and the census above is what proves nothing was skipped.
#[test]
fn the_booleans_flow_is_empty_beside_a_real_record() {
    let a = sweep::test_support::cube(1.0, tol());
    let map = Affine3::translation(Vec3::new(0.5, 0.5, 0.5));
    let b = topo::transform_rigid(&a, &map, tol()).expect("a translation is rigid");
    let out = Verb::Boolean {
        op: BooleanOp::Union,
        declare: BooleanDeclarations::none(),
    }
    .run_pair(&a, &b, SweepStrategy::Realized, tol())
    .expect("the crossing union is inside the door");
    let PairOut::Out(out) = out else {
        panic!("the crossing union is a body");
    };
    let VerbRecord::Boolean { naming, .. } = out.record else {
        panic!("a boolean run produced another family's record");
    };
    assert!(
        !naming.seam_edges.is_empty(),
        "the crossing union minted no seam; the fixture no longer exercises the record"
    );

    for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
        assert!(
            VerbKind::Boolean(op).param_flow().is_empty(),
            "the boolean has no scalar parameters; a row appeared with nothing to declare"
        );
    }
}

/// **The split's flow has no rows, and that is a claim beside a real
/// record.** A cube is parted through its middle and the record read
/// back — non-trivial (a section face on each side) — so "no scalar
/// parameter lands in anything this record names" is a statement
/// about an actual result. The split HAS no scalar parameter: its
/// payload is a plane, a placement read off a datum, and the faces it
/// mints are planes with no stored scalar field for anything to land
/// in — so, like the boolean, the emptiness is one level up from the
/// chamfer's, and the census above is what proves nothing was skipped.
#[test]
fn the_splits_flow_is_empty_beside_a_real_record() {
    let cube = sweep::test_support::cube(1.0, tol());
    let out = Verb::Split {
        plane: z_plane(0.5),
    }
    .run_split(&cube, tol())
    .expect("the mid-plane cut is inside the door");
    let VerbRecord::Split(naming) = out.record else {
        panic!("a split run produced another family's record");
    };
    assert_eq!(
        naming.sections.len(),
        2,
        "the mid-plane cut minted a section face per side; the fixture no longer exercises the record"
    );
    assert!(
        VerbKind::Split.param_flow().is_empty(),
        "the split has no scalar parameters; a row appeared with nothing to declare"
    );
}

/// **The shell's thickness row is empty beside a real record**, and
/// the emptiness is the chamfer's shape rather than the boolean's: the
/// verb HAS a scalar parameter, so the row exists, and what it reaches
/// is nothing this declaration can name.
///
/// The record is produced here so the claim is made about a run: a cube
/// is hollowed, and the cavity twins it lists are the faces whose
/// carriers store `r − t` where anything stores the thickness at all.
/// That derived number is why the row is empty — VS-Q3 gives v1 no
/// source for a field that is the operand's own minus the parameter —
/// and a run is what makes "the record names no family the thickness
/// reaches" a statement rather than a constant.
#[test]
fn the_shells_flow_is_empty_beside_a_real_record() {
    let cube = sweep::test_support::cube(1.0, tol());
    let out = Verb::Shell {
        thickness: 0.1,
        open: Vec::new(),
    }
    .run_shell(&cube, tol())
    .expect("the sealed cube is inside the door");
    let VerbRecord::Shell(naming) = out.record else {
        panic!("a shell run produced another family's record");
    };
    assert_eq!(
        naming.inner.len(),
        6,
        "the sealed cube minted a cavity twin per face; the fixture no longer exercises the record"
    );

    let rows = VerbKind::Shell.param_flow();
    assert_eq!(
        rows.len(),
        1,
        "the thickness's row must be present, not absent"
    );
    assert_eq!(
        rows[0].source,
        FlowSource::Param(ScalarParam::ShellThickness)
    );
    assert!(
        rows[0].fields.is_empty(),
        "the thickness reaches only derived cavity fields, which v1 has no source for"
    );
}

/// **A profile-edge source may be declared only by a verb whose
/// operand IS a profile.**
///
/// Nothing in the types stops a one-body verb from writing a
/// `FlowSource::ProfileEdge` row, and the consequence would be silent
/// rather than typed: the row would attach nothing — a blend's record
/// has no swept walls, and `attach_swept` is reached only from the
/// profile lowering — while still flipping the GLOBAL predicate
/// `editor-core` reads to decide whether a carrier radius's spelling
/// enters a profile node's content key. Every profile in every
/// document would key differently for a row that reaches no field
/// anywhere. So the census is the guard: the source kind names the
/// operand, and the operand is the arity.
#[test]
fn only_profile_operand_verbs_declare_a_profile_edge_source() {
    for kind in VerbKind::ALL {
        for flow in kind.param_flow() {
            let FlowSource::ProfileEdge(scalar) = flow.source else {
                continue;
            };
            assert_eq!(
                kind.arity(),
                Arity::Profile,
                "{kind:?} declares the operand-carried {scalar:?} but its operand is \
                 {:?}, so nothing would ever attach it — and the key feed would widen anyway",
                kind.arity()
            );
        }
    }
}

/// **The sweeps' flow names a family their records really mint**, and
/// the walls it names are non-empty on both fixtures — the row that
/// fires if a sweep stopped exporting its per-loop wall lists, which is
/// the only thing a per-edge source can be attached through.
#[test]
fn the_sweeps_flow_names_the_wall_family_their_records_mint() {
    let extruded = Verb::Extrude { distance: 1.0 }
        .run_profile(&disc(0.5), tol())
        .expect("the disc extrudes");
    let VerbRecord::Extrude(built) = extruded else {
        panic!("an extrude run produced another family's record");
    };
    assert!(
        built.side_faces.iter().any(|loop_| !loop_.is_empty()),
        "the extruded disc minted no side walls"
    );

    let revolved = Verb::Revolve {
        axis: x_axis(),
        revolution: Revolution::Full,
    }
    .run_profile(&offset_disc(0.25, 1.0), tol())
    .expect("the offset disc revolves");
    let VerbRecord::Revolve(built) = revolved else {
        panic!("a revolve run produced another family's record");
    };
    assert!(
        built.walls.iter().flatten().any(Option::is_some),
        "the revolved disc minted no walls"
    );

    // Every family the two flows name is the wall family, and it is the
    // one both records just filled.
    for kind in [VerbKind::Extrude, VerbKind::Revolve] {
        for flow in kind.param_flow() {
            for field in flow.fields {
                assert_eq!(
                    field.family(),
                    RoleFamily::SweptWalls,
                    "{kind:?}'s flow names {field:?}, whose family no sweep record mints"
                );
            }
        }
    }
}

/// **The two extents' rows are empty beside real records.** The
/// distance and the angle are how far the sweep runs, and a sweep that
/// ran is what makes the emptiness a statement rather than a constant:
/// both fixtures above produced bodies, and neither verb's own scalar
/// reaches a field of them.
#[test]
fn the_sweep_extents_reach_no_field() {
    for (kind, param) in [
        (VerbKind::Extrude, ScalarParam::ExtrudeDistance),
        (VerbKind::Revolve, ScalarParam::RevolveAngle),
    ] {
        let row = kind
            .param_flow()
            .iter()
            .find(|row| row.source == FlowSource::Param(param))
            .expect("the extent's row must be present, not absent");
        assert!(
            row.fields.is_empty(),
            "{param:?} reaches no stored field: an extent is not a surface's data"
        );
        let edge = kind
            .param_flow()
            .iter()
            .find(|row| row.source == FlowSource::ProfileEdge(EdgeScalar::Radius))
            .expect("the profile edge's row must be present");
        assert!(
            !edge.fields.is_empty(),
            "{kind:?} declares the profile edge's radius and lands it nowhere"
        );
    }
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
