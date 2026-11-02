//! **The dispatch adds nothing**: the run doors and the op doors they
//! call produce the same body, the same birth record and the same
//! refusal.
//!
//! The point of the crate is that a caller can address an operation by
//! its NAME and get exactly what the door gives. These rows are what
//! makes that checkable: they run both paths on the same fixture and
//! compare bit-for-bit, so a dispatch that reordered arguments, dropped
//! a parameter or re-derived a band would red here rather than showing
//! up as drifted geometry three layers up.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::fmt::Write as _;
use sweep::ExtrudeSide;

use geom_core::{Affine3, Vec3};
use sweep::blend::build::{chamfer_edges, fillet_edges};
use sweep::{Extrusion, Revolution};
use topo::{
    AtRestBody, Body, BooleanInputs, BooleanOp, BooleanResult, SplitPart, SweepStrategy,
    boolean_op_with, split,
};
use verbs::{Blend, Boolean, Members, PairOut, Shell, Split, Sweep, VerbError, VerbRecord};

use crate::fixture::{disc, offset_disc, pinch_plane, pinch_prism, tol, x_axis, z_plane};
use sweep::test_support::finished;

/// The `f64` shell door, read at the ONE seam that answers it
/// (`topo::AtRestPolicy::shell_door`) rather than constructed here —
/// so a scalar's policy arm that stopped handing out the door reds
/// every shell row below instead of leaving them green against a door
/// the production path no longer reaches.
fn shell_door() -> topo::ShellDoor<f64> {
    <f64 as topo::AtRestPolicy>::shell_door().expect("f64 certifies, so it holds the shell door")
}

/// Every vertex point's BITS, plus the entity census — enough that a
/// carve differing anywhere in position or structure differs here.
fn dump(body: &Body<f64>) -> String {
    let mut s = String::new();
    let _ = writeln!(
        s,
        "V={} E={} F={}",
        body.vertices().count(),
        body.edges().count(),
        body.faces().count()
    );
    for (k, p) in body.vertex_points() {
        let _ = writeln!(
            s,
            "{k:?} {:016x} {:016x} {:016x}",
            p.x.to_bits(),
            p.y.to_bits(),
            p.z.to_bits()
        );
    }
    s
}

/// The blend channel of a run's record, or a loud failure.
fn blend_record(record: VerbRecord<f64>) -> Option<sweep::blend::naming::BlendNaming> {
    let VerbRecord::Blend(naming) = record else {
        panic!("a blend run produced another family's record: {record:?}");
    };
    naming
}

#[test]
fn the_fillet_dispatch_is_the_fillet_door() {
    let cube = sweep::test_support::cube(1.0, tol());
    let edges: Vec<_> = cube.edges().map(|(k, _)| k).collect();

    let door = fillet_edges(
        &sweep::test_support::at_rest(&cube, tol()),
        &edges,
        0.1,
        tol(),
    )
    .unwrap();
    let via = Blend::Fillet {
        edges: edges.clone(),
        radius: 0.1,
    }
    .run(&finished("the operand", cube.clone(), tol()), tol())
    .unwrap();

    assert_eq!(dump(&door.body), dump(&via.body));
    assert_eq!(
        format!("{:?}", door.naming),
        format!("{:?}", blend_record(via.record)),
        "the birth record is carried across, not rebuilt"
    );
}

#[test]
fn the_chamfer_dispatch_is_the_chamfer_door() {
    let cube = sweep::test_support::cube(1.0, tol());
    let edges: Vec<_> = cube.edges().map(|(k, _)| k).collect();

    let door = chamfer_edges(
        &sweep::test_support::at_rest(&cube, tol()),
        &edges,
        0.1,
        tol(),
    )
    .unwrap();
    let via = Blend::Chamfer {
        edges: edges.clone(),
        distance: 0.1,
    }
    .run(&finished("the operand", cube.clone(), tol()), tol())
    .unwrap();

    assert_eq!(dump(&door.body), dump(&via.body));
    assert_eq!(
        format!("{:?}", door.naming),
        format!("{:?}", blend_record(via.record))
    );
}

/// **A refusal crosses unaltered**, verb label included — the whole
/// reason `VerbError` wraps rather than re-classifies. The fixture is
/// the chamfer door's nonpositive-setback gate, which is the one
/// refusal reachable without building a degenerate body.
#[test]
fn a_refusal_crosses_the_dispatch_unaltered() {
    let cube = sweep::test_support::cube(1.0, tol());
    let edges: Vec<_> = cube.edges().map(|(k, _)| k).collect();

    let door = chamfer_edges(
        &sweep::test_support::at_rest(&cube, tol()),
        &edges,
        0.0,
        tol(),
    )
    .unwrap_err();
    let via = Blend::Chamfer {
        edges,
        distance: 0.0,
    }
    .run(&finished("the operand", cube.clone(), tol()), tol())
    .unwrap_err();

    let VerbError::Blend(carried) = via else {
        panic!("a blend refusal crossed as another family's: {via:?}");
    };
    assert_eq!(format!("{door:?}"), format!("{carried:?}"));
    assert_eq!(door.to_string(), carried.to_string());
}

/// The unit cube, finished — the boolean rows' first operand, and the
/// split rows' operand.
fn unit_cube() -> AtRestBody<f64> {
    sweep::test_support::finished(
        "the unit cube",
        sweep::test_support::cube(1.0, tol()),
        tol(),
    )
}

/// A unit cube translated by `d`, finished — the boolean rows' second
/// operand.
fn shifted_cube(d: Vec3<f64>) -> AtRestBody<f64> {
    let cube = sweep::test_support::cube(1.0, tol());
    let map = Affine3::translation(d);
    let moved = topo::transform_rigid(&cube, &geom_core::test_support::rigid(&map, tol()), tol())
        .expect("a translation is rigid");
    sweep::test_support::finished("the shifted cube", moved, tol())
}

#[test]
fn the_boolean_dispatch_is_the_boolean_door() {
    let a = unit_cube();
    // A proper crossing: overlap in every axis, no face-on-face rest.
    let b = shifted_cube(Vec3::new(0.5, 0.5, 0.5));

    let decls = BooleanInputs::default();
    let door = boolean_op_with(
        BooleanOp::Union,
        &a,
        &b,
        &decls,
        SweepStrategy::Realized,
        tol(),
    )
    .unwrap();
    let via = Boolean {
        op: BooleanOp::Union,
        inputs: Box::new(BooleanInputs::default()),
    }
    .run_pair(&a, &b, SweepStrategy::Realized, tol())
    .unwrap();

    let BooleanResult::Body(door) = door else {
        panic!("the fixture's union is a body");
    };
    let PairOut::Out(via) = via else {
        panic!("the dispatch's union is a body: {via:?}");
    };
    assert_eq!(dump(&door.body), dump(&via.body));
    let VerbRecord::Boolean {
        kind,
        contacts,
        naming,
        verdicts: _,
    } = via.record
    else {
        panic!("a boolean run produced another family's record");
    };
    assert_eq!(door.kind, kind, "the result classification is carried");
    assert_eq!(
        door.contacts, contacts,
        "the surviving contacts are carried"
    );
    assert_eq!(
        format!("{:?}", door.naming),
        format!("{naming:?}"),
        "the birth record is carried across, not rebuilt"
    );
}

/// **The typed empty success crosses as itself** (F8: ∅ is a value,
/// not an error) — disjoint operands intersected.
#[test]
fn an_empty_boolean_result_crosses_as_the_typed_empty() {
    let a = unit_cube();
    let b = shifted_cube(Vec3::new(3.0, 0.0, 0.0));

    let door = boolean_op_with(
        BooleanOp::Intersect,
        &a,
        &b,
        &BooleanInputs::default(),
        SweepStrategy::Realized,
        tol(),
    )
    .unwrap();
    assert!(matches!(door, BooleanResult::Empty));

    let via = Boolean {
        op: BooleanOp::Intersect,
        inputs: Box::new(BooleanInputs::default()),
    }
    .run_pair(&a, &b, SweepStrategy::Realized, tol())
    .unwrap();
    assert!(
        matches!(via, PairOut::Empty),
        "the dispatch turned the typed empty into {via:?}"
    );
}

/// **A boolean refusal crosses unaltered.** The fixture is two cubes
/// 2ε apart, inside the ambiguity band at every ε — the in-band
/// refusal, reached identically both ways.
#[test]
fn a_boolean_refusal_crosses_the_dispatch_unaltered() {
    let a = unit_cube();
    let b = shifted_cube(Vec3::new(1.0 + 2.0 * tol().eps(), 0.0, 0.0));

    let door = boolean_op_with(
        BooleanOp::Union,
        &a,
        &b,
        &BooleanInputs::default(),
        SweepStrategy::Realized,
        tol(),
    )
    .unwrap_err();
    let via = Boolean {
        op: BooleanOp::Union,
        inputs: Box::new(BooleanInputs::default()),
    }
    .run_pair(&a, &b, SweepStrategy::Realized, tol())
    .unwrap_err();

    let VerbError::Boolean(carried) = via else {
        panic!("a boolean refusal crossed as another family's: {via:?}");
    };
    assert_eq!(format!("{door:?}"), format!("{carried:?}"));
    assert_eq!(door.to_string(), carried.to_string());
}

/// Three members with ranks `ranks`, run through the n-ary dispatch and
/// through the kernel's door: (door, dispatch).
fn members_both_ways(
    bodies: [&AtRestBody<f64>; 3],
    ranks: [u32; 3],
) -> (topo::NaryRun<f64>, topo::NaryRun<f64>) {
    let inputs: Vec<topo::MemberInputs> = ranks
        .iter()
        .map(|&rank| topo::MemberInputs {
            rank,
            ..topo::MemberInputs::default()
        })
        .collect();
    let members: Vec<topo::NaryMember<'_, f64>> = bodies
        .iter()
        .zip(&inputs)
        .map(|(&body, inputs)| topo::NaryMember { body, inputs })
        .collect();
    let door = topo::nary_boolean(
        topo::NaryOp::Union,
        &members,
        SweepStrategy::Realized,
        tol(),
    );
    let via = Members {
        op: topo::NaryOp::Union,
        members: inputs.clone(),
        apart: topo::Apart::default(),
    }
    .run_members(&bodies, SweepStrategy::Realized, tol());
    (door, via)
}

/// **The n-ary dispatch is the n-ary door, members, ranks and all**:
/// three overlapping cubes ranked against their list order (2, 0, 1)
/// run both ways to the same run, judgement for judgement and step for
/// step, and the dispatch judges each pair with its lesser rank as
/// operand A, so a dispatch that dropped or reordered the members'
/// inputs differs.
#[test]
fn the_members_dispatch_is_the_nary_door() {
    let [a, b, c] = [0.0, 0.5, 1.0].map(|x| shifted_cube(Vec3::new(x, 0.0, 0.0)));
    let (door, via) = members_both_ways([&a, &b, &c], [2, 0, 1]);
    assert!(
        matches!(via.end, topo::NaryEnd::Built),
        "the fixture's union builds: {:?}",
        via.end
    );
    assert_eq!(
        via.judgements
            .iter()
            .map(|j| (j.a, j.b))
            .collect::<Vec<_>>(),
        [(1, 0), (2, 0), (1, 2)],
        "each pair is judged with its lesser rank as operand A"
    );
    assert_eq!(
        format!("{door:?}"),
        format!("{via:?}"),
        "the dispatch's run is the door's"
    );
}

/// **An n-ary refusal crosses unaltered**, naming where it was raised:
/// two cubes 2ε apart, in the ambiguity band, beside a third far off.
#[test]
fn a_members_refusal_crosses_the_dispatch_unaltered() {
    let a = unit_cube();
    let b = shifted_cube(Vec3::new(1.0 + 2.0 * tol().eps(), 0.0, 0.0));
    let far = shifted_cube(Vec3::new(10.0, 0.0, 0.0));
    let (door, via) = members_both_ways([&a, &b, &far], [0, 1, 2]);
    let topo::NaryEnd::Refused(carried) = &via.end else {
        panic!("an n-ary refusal crossed as something else: {:?}", via.end);
    };
    assert!(
        matches!(carried, topo::NaryRefusal::Judgement { a: 0, b: 1, .. })
            && via.judgements.is_empty()
            && via.steps.is_empty(),
        "the pair's judgement refused, first: {carried:?}"
    );
    assert_eq!(format!("{door:?}"), format!("{via:?}"));
}

/// The two sides' dumps, in order, with an empty side as its own
/// token — so a dispatch that swapped the sides, dropped one, or
/// turned the typed empty into a body differs here.
fn dump_sides(above: &SplitPart<f64>, below: &SplitPart<f64>) -> String {
    let side = |part: &SplitPart<f64>| match part {
        SplitPart::Body(b) => dump(b),
        SplitPart::Empty => "empty\n".to_owned(),
    };
    format!("above:\n{}below:\n{}", side(above), side(below))
}

#[test]
fn the_split_dispatch_is_the_split_door() {
    let cube = unit_cube();
    let plane = z_plane(0.5);

    let door = split(&cube, &plane, tol()).unwrap();
    let via = Split { plane }.run_split(&cube, tol()).unwrap();

    assert_eq!(
        dump_sides(&door.above, &door.below),
        dump_sides(&via.above, &via.below)
    );
    let VerbRecord::Split(naming) = via.record else {
        panic!(
            "a split run produced another family's record: {:?}",
            via.record
        );
    };
    assert!(
        !naming.sections.is_empty(),
        "the mid-plane cut minted no section face; the fixture no longer exercises the record"
    );
    assert_eq!(
        format!("{:?}", door.naming),
        format!("{naming:?}"),
        "the birth record is carried across, not rebuilt"
    );
}

/// Bitwise vertex lookup: how many vertices of `body` sit exactly at
/// `(x, y, z)`.
fn vertices_at(body: &Body<f64>, x: f64, y: f64, z: f64) -> usize {
    body.vertex_points()
        .filter(|(_, p)| {
            let p = *p;
            p.x == x && p.y == y && p.z == z
        })
        .count()
}

/// **The dispatch agrees with the door through a pinch below the
/// plane.**
///
/// On this operand the pinched pieces lie below the plane's normal, so
/// the kernel mints the pieces' tip copies on the Below side: two
/// coincident-but-distinct vertices at each tip on the pieces side, and
/// none on the slab side, whose top is one maximal face. A plane whose orientation the dispatch re-derived
/// would put the pieces on the other side.
///
/// The dispatch calls the door and nothing beneath it, so agreement
/// is by construction; it is pinned rather than assumed because the
/// one thing the dispatch could do to the plane — re-derive it, or
/// its orientation, before the door — is exactly what this operand
/// would expose while every other row stayed green.
#[test]
fn the_split_dispatch_agrees_with_the_door_through_a_pinch_below_the_plane() {
    let prism = sweep::test_support::finished("the pinch prism", pinch_prism(), tol());
    let plane = pinch_plane();

    let door = split(&prism, &plane, tol()).unwrap();
    let via = Split { plane }.run_split(&prism, tol()).unwrap();

    let (SplitPart::Body(slab), SplitPart::Body(pieces)) = (&via.above, &via.below) else {
        panic!("both sides of the pinch cut are bodies: {via:?}");
    };
    assert_eq!(
        pieces.shells().count(),
        3,
        "the pinch's three floor pieces did not reach the dispatch below the plane"
    );
    // The pieces' copies at each tip line; the slab's top is one face.
    for z in [0.0, 1.0] {
        assert_eq!(
            vertices_at(pieces, 4.0, 1.0, z),
            2,
            "the pieces side carries no tip copies at z = {z}"
        );
        assert_eq!(vertices_at(slab, 4.0, 1.0, z), 0);
    }
    assert_eq!(
        dump_sides(&door.above, &door.below),
        dump_sides(&via.above, &via.below)
    );
    let VerbRecord::Split(naming) = via.record else {
        panic!("a split run produced another family's record");
    };
    assert_eq!(format!("{:?}", door.naming), format!("{naming:?}"));
}

/// **An empty side crosses as the typed empty** — a plane clear of the
/// body puts everything below it, and the above side is the kernel's
/// `SplitPart::Empty` on both paths, never a phantom body and never
/// a refusal.
#[test]
fn an_empty_split_side_crosses_as_the_typed_empty() {
    let cube = unit_cube();
    let plane = z_plane(5.0);

    let door = split(&cube, &plane, tol()).unwrap();
    assert!(matches!(door.above, SplitPart::Empty));

    let via = Split { plane }.run_split(&cube, tol()).unwrap();
    assert!(
        matches!(via.above, SplitPart::Empty),
        "the dispatch turned the typed empty into {:?}",
        via.above
    );
    assert_eq!(
        dump_sides(&door.above, &door.below),
        dump_sides(&via.above, &via.below)
    );
}

/// **A split refusal crosses unaltered.** The fixture is an operand
/// with no solid at all — the split contract is one solid, and an
/// empty body is the refusal reachable without building a degenerate
/// section.
#[test]
fn a_split_refusal_crosses_the_dispatch_unaltered() {
    let empty = sweep::test_support::finished("the empty body", Body::<f64>::new(), tol());
    let plane = z_plane(0.5);

    let door = split(&empty, &plane, tol()).unwrap_err();
    let via = Split { plane }.run_split(&empty, tol()).unwrap_err();

    let VerbError::Split(carried) = via else {
        panic!("a split refusal crossed as another family's: {via:?}");
    };
    assert_eq!(format!("{door:?}"), format!("{carried:?}"));
    assert_eq!(door.to_string(), carried.to_string());
}

#[test]
fn the_extrude_dispatch_is_the_extrude_door() {
    let profile = disc(0.5);
    let door = sweep::extrude(
        &profile,
        Extrusion::Distance {
            depth: 1.0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap();
    let via = Sweep::Extrude {
        distance: 1.0,
        side: ExtrudeSide::Along,
    }
    .run_profile(&profile, tol())
    .unwrap();

    let VerbRecord::Extrude(via) = via else {
        panic!("an extrude run produced another family's record");
    };
    assert_eq!(dump(&door.body), dump(&via.body));
    assert_eq!(
        format!("{:?}", door.side_faces()),
        format!("{:?}", via.side_faces()),
        "the birth record is carried across, not rebuilt"
    );
    assert_eq!(format!("{:?}", door.top), format!("{:?}", via.top));
    assert_eq!(format!("{:?}", door.bottom), format!("{:?}", via.bottom));
    assert_eq!(
        format!("{:?}", door.strut_edges()),
        format!("{:?}", via.strut_edges())
    );
    assert_eq!(format!("{:?}", door.solid), format!("{:?}", via.solid));
    assert_eq!(format!("{:?}", door.shell), format!("{:?}", via.shell));
}

#[test]
fn the_revolve_dispatch_is_the_revolve_door() {
    // Negative y: the door's half-plane about the +x axis is
    // `(p − origin).perp_dot(dir) ≥ 0`, which is `−y`.
    let profile = offset_disc(0.25, 1.0);
    let door = sweep::revolve(&profile, x_axis(), Revolution::Full, tol()).unwrap();
    let via = Sweep::Revolve {
        axis: x_axis(),
        revolution: Revolution::Full,
    }
    .run_profile(&profile, tol())
    .unwrap();

    let VerbRecord::Revolve(via) = via else {
        panic!("a revolve run produced another family's record");
    };
    assert_eq!(dump(&door.body), dump(&via.body));
    assert_eq!(
        format!("{:?}", door.walls()),
        format!("{:?}", via.walls()),
        "the birth record is carried across, not rebuilt"
    );
    assert_eq!(format!("{:?}", door.rims), format!("{:?}", via.rims));
    assert_eq!(format!("{:?}", door.poles), format!("{:?}", via.poles));
    assert_eq!(format!("{:?}", door.kind), format!("{:?}", via.kind));
    assert_eq!(format!("{:?}", door.solid), format!("{:?}", via.solid));
    assert_eq!(format!("{:?}", door.shell), format!("{:?}", via.shell));
    assert_eq!(
        format!("{:?}", door.cavities),
        format!("{:?}", via.cavities)
    );
}

/// **A sweep refusal crosses unaltered**, both doors. The fixtures are
/// each door's own degenerate-extent gate — a zero distance and a zero
/// angle — which is the refusal reachable without building a
/// degenerate profile.
#[test]
fn a_sweep_refusal_crosses_the_dispatch_unaltered() {
    let profile = disc(0.5);
    let door = sweep::extrude(
        &profile,
        Extrusion::Distance {
            depth: 0.0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap_err();
    let via = Sweep::Extrude {
        distance: 0.0,
        side: ExtrudeSide::Along,
    }
    .run_profile(&profile, tol())
    .unwrap_err();
    let VerbError::Extrude(carried) = via else {
        panic!("an extrude refusal crossed as another family's: {via:?}");
    };
    assert_eq!(format!("{door:?}"), format!("{carried:?}"));
    assert_eq!(door.to_string(), carried.to_string());

    let off = offset_disc(0.25, 1.0);
    let door = sweep::revolve(&off, x_axis(), Revolution::Partial(0.0), tol()).unwrap_err();
    let via = Sweep::Revolve {
        axis: x_axis(),
        revolution: Revolution::Partial(0.0),
    }
    .run_profile(&off, tol())
    .unwrap_err();
    let VerbError::Revolve(carried) = via else {
        panic!("a revolve refusal crossed as another family's: {via:?}");
    };
    assert_eq!(format!("{door:?}"), format!("{carried:?}"));
    assert_eq!(door.to_string(), carried.to_string());
}

/// **The shell dispatch is the shell door**, in both of its spellings:
/// the sealed hollow and the opened one. `Shell` is one verb
/// for both, so the row that matters is that an EMPTY designation
/// reaches `topo::shell` — which is `shell_open`'s own contract — and a
/// nonempty one reaches the rim surgery, with the body and every field
/// of the birth record identical either way.
#[test]
fn the_shell_dispatch_is_the_shell_door() {
    let cube = sweep::test_support::cube(1.0, tol());
    let designated: Vec<topo::FaceKey> = cube.faces().map(|(k, _)| k).take(1).collect();

    for open in [Vec::new(), designated] {
        let door = if open.is_empty() {
            topo::shell(&finished("the operand", cube.clone(), tol()), 0.1, tol())
        } else {
            topo::shell_open(
                &finished("the operand", cube.clone(), tol()),
                0.1,
                &open,
                tol(),
            )
        }
        .unwrap_or_else(|e| panic!("the fixture is inside the door: {e}"));
        let via = Shell {
            thickness: 0.1,
            open: open.clone(),
        }
        .run_shell(
            &finished("the operand", cube.clone(), tol()),
            tol(),
            shell_door(),
        )
        .unwrap_or_else(|e| panic!("the dispatch refused what the door accepted: {e}"));

        assert_eq!(dump(&door.body), dump(&via.body));
        let VerbRecord::Shell(carried) = via.record else {
            panic!("a shell run produced another family's record");
        };
        // Field by field, as the revolve's row is: the whole-record
        // equality below would pass on a record rebuilt from the same
        // parts, and these say WHICH part moved if one stops.
        assert_eq!(door.naming.outer, carried.outer);
        assert_eq!(door.naming.inner, carried.inner);
        assert_eq!(door.naming.inner_edges, carried.inner_edges);
        assert_eq!(door.naming.inner_vertices, carried.inner_vertices);
        assert_eq!(door.naming.rims, carried.rims);
        assert_eq!(door.naming.dead, carried.dead);
        assert_eq!(
            door.naming, carried,
            "the birth record is carried across, not rebuilt"
        );
        assert_eq!(
            !open.is_empty(),
            !carried.rims.is_empty(),
            "an empty designation rims nothing and a nonempty one rims something"
        );
    }
}

/// **A shell refusal crosses unaltered.** The fixture is the door's own
/// thickness gate — a zero wall, refused before anything is built — and
/// it is the row that shows the scalar on `VerbError` doing its job: the
/// thickness the door echoed back into its refusal is what crosses,
/// unrendered.
#[test]
fn a_shell_refusal_crosses_the_dispatch_unaltered() {
    let cube = sweep::test_support::cube(1.0, tol());
    let door = topo::shell(
        &finished("the operand", cube.clone(), tol()),
        0.0_f64,
        tol(),
    )
    .unwrap_err();
    let via = Shell {
        thickness: 0.0,
        open: Vec::new(),
    }
    .run_shell(
        &finished("the operand", cube.clone(), tol()),
        tol(),
        shell_door(),
    )
    .unwrap_err();

    let VerbError::Shell(carried) = via else {
        panic!("a shell refusal crossed as another family's: {via:?}");
    };
    assert_eq!(format!("{door:?}"), format!("{carried:?}"));
    assert_eq!(door.to_string(), carried.to_string());
}
