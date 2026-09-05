//! **The dispatch adds nothing**: the run doors and the op doors they
//! call produce the same body, the same birth record and the same
//! refusal.
//!
//! The point of the crate is that a caller can address an operation by
//! its NAME and get exactly what the door gives. These rows are what
//! makes that checkable: they run both paths on the same fixture and
//! compare bit-for-bit, so a dispatch that reordered arguments, dropped
//! a parameter or re-derived a band would red here rather than showing
//! up as drifted geometry three layers up. The door rows at the end
//! are the run doors' one own decision — refusing a verb at every door
//! but the one it declares — exercised in both directions so no
//! mismatch arm is untested code.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::fmt::Write as _;

use geom_core::{Affine3, Vec3};
use sweep::blend::build::{chamfer_edges, fillet_edges};
use sweep::{Extrusion, Revolution};
use topo::{
    Body, BooleanDeclarations, BooleanOp, BooleanResult, SplitPart, SweepStrategy, boolean_op_with,
    split,
};
use verbs::{Arity, PairOut, Verb, VerbError, VerbKind, VerbRecord};

use crate::fixture::{disc, offset_disc, pinch_plane, pinch_prism, tol, x_axis, z_plane};

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
    for (k, _) in body.vertices() {
        let p = body
            .get_vertex(k)
            .and_then(|v| body.get_point(v.point))
            .unwrap();
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

    let door = fillet_edges(&cube, &edges, 0.1, tol()).unwrap();
    let via = Verb::Fillet {
        edges: edges.clone(),
        radius: 0.1,
    }
    .run(&cube, tol())
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

    let door = chamfer_edges(&cube, &edges, 0.1, tol()).unwrap();
    let via = Verb::Chamfer {
        edges: edges.clone(),
        distance: 0.1,
    }
    .run(&cube, tol())
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

    let door = chamfer_edges(&cube, &edges, 0.0, tol()).unwrap_err();
    let via = Verb::Chamfer {
        edges,
        distance: 0.0,
    }
    .run(&cube, tol())
    .unwrap_err();

    let VerbError::Blend(carried) = via else {
        panic!("a blend refusal crossed as another family's: {via:?}");
    };
    assert_eq!(format!("{door:?}"), format!("{carried:?}"));
    assert_eq!(door.to_string(), carried.to_string());
}

/// A unit cube translated by `d` — the boolean rows' second operand.
fn shifted_cube(d: Vec3<f64>) -> Body<f64> {
    let cube = sweep::test_support::cube(1.0, tol());
    let map = Affine3::translation(d);
    topo::transform_rigid(&cube, &map, tol()).expect("a translation is rigid")
}

#[test]
fn the_boolean_dispatch_is_the_boolean_door() {
    let a = sweep::test_support::cube(1.0, tol());
    // A proper crossing: overlap in every axis, no face-on-face rest.
    let b = shifted_cube(Vec3::new(0.5, 0.5, 0.5));

    let decls = BooleanDeclarations::none();
    let door = boolean_op_with(
        BooleanOp::Union,
        &a,
        &b,
        &decls,
        SweepStrategy::Realized,
        tol(),
    )
    .unwrap();
    let via = Verb::Boolean {
        op: BooleanOp::Union,
        declare: BooleanDeclarations::none(),
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
    let a = sweep::test_support::cube(1.0, tol());
    let b = shifted_cube(Vec3::new(3.0, 0.0, 0.0));

    let door = boolean_op_with(
        BooleanOp::Intersect,
        &a,
        &b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol(),
    )
    .unwrap();
    assert!(matches!(door, BooleanResult::Empty));

    let via = Verb::Boolean {
        op: BooleanOp::Intersect,
        declare: BooleanDeclarations::none(),
    }
    .run_pair(&a, &b, SweepStrategy::Realized, tol())
    .unwrap();
    assert!(
        matches!(via, PairOut::Empty),
        "the dispatch turned the typed empty into {via:?}"
    );
}

/// **A boolean refusal crosses unaltered.** The fixture is two cubes
/// resting face on face with nothing declared — the undeclared-
/// coincidence refusal, reached identically both ways.
#[test]
fn a_boolean_refusal_crosses_the_dispatch_unaltered() {
    let a = sweep::test_support::cube(1.0, tol());
    let b = shifted_cube(Vec3::new(1.0, 0.0, 0.0));

    let door = boolean_op_with(
        BooleanOp::Union,
        &a,
        &b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol(),
    )
    .unwrap_err();
    let via = Verb::Boolean {
        op: BooleanOp::Union,
        declare: BooleanDeclarations::none(),
    }
    .run_pair(&a, &b, SweepStrategy::Realized, tol())
    .unwrap_err();

    let VerbError::Boolean(carried) = via else {
        panic!("a boolean refusal crossed as another family's: {via:?}");
    };
    assert_eq!(format!("{door:?}"), format!("{carried:?}"));
    assert_eq!(door.to_string(), carried.to_string());
}

/// One representative verb per vocabulary row, payload minimal.
fn sample(kind: VerbKind) -> Verb<f64> {
    match kind {
        VerbKind::Fillet => Verb::Fillet {
            edges: Vec::new(),
            radius: 0.1,
        },
        VerbKind::Chamfer => Verb::Chamfer {
            edges: Vec::new(),
            distance: 0.1,
        },
        VerbKind::Extrude => Verb::Extrude { distance: 1.0 },
        VerbKind::Revolve => Verb::Revolve {
            axis: x_axis(),
            revolution: Revolution::Full,
        },
        VerbKind::Boolean(op) => Verb::Boolean {
            op,
            declare: BooleanDeclarations::none(),
        },
        VerbKind::Split => Verb::Split {
            plane: z_plane(0.5),
        },
    }
}

/// **The door matrix**: `verb` run through EVERY door, each under the
/// row that names it, with the refusal (if any) beside it. This list
/// is the one place the doors are enumerated by hand — the two rows
/// below read it, one for the refusals and one for the census.
fn every_door(
    verb: &Verb<f64>,
    a: &Body<f64>,
    b: &Body<f64>,
    disc: &profile::ValidatedProfile<f64>,
) -> Vec<(Arity, Option<VerbError>)> {
    vec![
        (Arity::One, verb.run(a, tol()).err()),
        (
            Arity::Two,
            verb.run_pair(a, b, SweepStrategy::Realized, tol()).err(),
        ),
        (Arity::Profile, verb.run_profile(disc, tol()).err()),
        (Arity::Split, verb.run_split(a, tol()).err()),
    ]
}

/// **Every door row has a door, and every door a row.** `Arity` is
/// compile-forced nowhere outside its own census: a variant planted
/// on it builds clean and every dispatch row stays green, because the
/// only thing tying rows to doors was the hand-written door matrix.
/// This row ties them: the set of rows the matrix runs is exactly
/// [`Arity::ALL`], and every verb's declared row is one the matrix
/// runs. A fifth row without a fifth door reds here; a fifth door
/// without a row cannot be written (the matrix keys on the enum).
#[test]
fn every_arity_row_has_a_door_and_every_door_a_row() {
    let a = sweep::test_support::cube(1.0, tol());
    let b = shifted_cube(Vec3::new(0.5, 0.5, 0.5));
    let disc = disc(0.5);
    let doors: std::collections::BTreeSet<Arity> =
        every_door(&sample(VerbKind::Fillet), &a, &b, &disc)
            .into_iter()
            .map(|(door, _)| door)
            .collect();
    let rows: std::collections::BTreeSet<Arity> = Arity::ALL.iter().copied().collect();
    assert_eq!(
        doors, rows,
        "the door matrix and Arity::ALL name different sets — a row has no door, or a door no row"
    );
    for kind in VerbKind::ALL {
        assert!(
            doors.contains(&kind.arity()),
            "{kind:?} declares {:?}, which no door in the matrix answers",
            kind.arity()
        );
    }
}

/// **Each door refuses exactly the verbs that do not declare it**,
/// over the whole vocabulary — the typed mismatch refusal exercised
/// in both directions, so no door's cross-door arms are untested. The
/// declared door's behavior is the dispatch rows above; what is
/// pinned here is that every OTHER door answers `Arity` with the
/// right verb and the right door, and never runs the op — proven by
/// the returned variant itself: `Arity` is minted only at the doors'
/// own mismatch arms, before any op door is reached, so its arrival
/// IS the non-execution. The split door is the row where this matters
/// most: a split's operand is one body like a blend's, so the ONE
/// door is the door a wiring bug would most plausibly hand it to, and
/// the refusal must name the split door rather than an operand count
/// that agrees.
#[test]
fn each_door_refuses_the_undeclared_arity() {
    let a = sweep::test_support::cube(1.0, tol());
    let b = shifted_cube(Vec3::new(0.5, 0.5, 0.5));
    let disc = disc(0.5);

    for kind in VerbKind::ALL {
        let verb = sample(*kind);
        // Every door but this verb's own, so no shape is left untried:
        // the refusal must come back from each of them, naming the verb
        // and the door that refused.
        let refusals: Vec<(Arity, VerbError)> = every_door(&verb, &a, &b, &disc)
            .into_iter()
            .filter(|(door, _)| *door != kind.arity())
            .map(|(door, err)| {
                (
                    door,
                    err.unwrap_or_else(|| panic!("{kind:?} ran through the {door:?} door")),
                )
            })
            .collect();
        assert_eq!(refusals.len(), 3, "there are four doors, not four-plus");
        for (door, err) in refusals {
            let VerbError::Arity { verb: who, given } = err else {
                panic!("{kind:?} at the {door:?} door refused with {err:?}, not Arity");
            };
            assert_eq!(who, *kind);
            assert_eq!(given, door);
        }
    }
}

/// **The door refusal's sentence, pinned byte for byte.**
///
/// It is the one refusal string this door owns rather than forwards.
/// It is written in the doors' own NAMES and nothing else — no reading
/// of what a row means enters it — because it moved in two consecutive
/// units while it was written in the vocabulary of the moment ("N
/// operand(s)", then "declares a … operand"), and a door added to the
/// enum must not move it again. What a row added to `Arity` changes
/// is only which names can appear. Pinned exactly, in both directions
/// of the split's own mismatch, because a split's operand is one body
/// and the One door is where a wiring bug would most plausibly hand
/// it.
#[test]
fn the_arity_refusal_names_the_declared_operand_and_the_door() {
    let err = Verb::Fillet {
        edges: Vec::new(),
        radius: 0.1_f64,
    }
    .run_profile(&disc(0.5), tol())
    .expect_err("a fillet is not a profile verb");
    assert_eq!(
        err.to_string(),
        "the Fillet verb was run through the Profile door; the door that answers it is One"
    );

    let cube = sweep::test_support::cube(1.0, tol());
    let err = Verb::Extrude { distance: 1.0_f64 }
        .run(&cube, tol())
        .expect_err("an extrude is not a one-body verb");
    assert_eq!(
        err.to_string(),
        "the Extrude verb was run through the One door; the door that answers it is Profile"
    );

    let err = Verb::Split {
        plane: z_plane(0.5),
    }
    .run(&cube, tol())
    .expect_err("a split hands back two sides, which the one-body door cannot");
    assert_eq!(
        err.to_string(),
        "the Split verb was run through the One door; the door that answers it is Split"
    );
    let err = Verb::Fillet {
        edges: Vec::new(),
        radius: 0.1_f64,
    }
    .run_split(&cube, tol())
    .expect_err("a fillet hands back one body, not two sides");
    assert_eq!(
        err.to_string(),
        "the Fillet verb was run through the Split door; the door that answers it is One"
    );
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
    let cube = sweep::test_support::cube(1.0, tol());
    let plane = z_plane(0.5);

    let door = split(&cube, &plane, tol()).unwrap();
    let via = Verb::Split { plane }.run_split(&cube, tol()).unwrap();

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
    body.vertices()
        .filter(|(_, v)| {
            let p = *body.get_point(v.point).unwrap();
            p.x == x && p.y == y && p.z == z
        })
        .count()
}

/// **The dispatch agrees with the door THROUGH the D7 pinch lane.**
///
/// The kernel door reruns a one-sided pinch mirrored and swaps the
/// sides back, so on this operand what `topo::split` returns is the
/// mirrored run's decomposition re-labelled — not the direct run's,
/// which refuses. The premise — that the lane is TAKEN on this
/// operand — is asserted, not assumed, through the one observable
/// only the lane can produce: the kernel mints vertex copies for the
/// ABOVE side's pinch runs only, so a below-side pinch that succeeds
/// with two coincident-but-distinct copies at each tip on the pieces
/// side, and one vertex on the slab side, got those copies from the
/// mirrored run. A direct run of this orientation has no way to mint
/// them and refuses instead (the kernel's own suite pins that half).
///
/// The dispatch calls the door and nothing beneath it, so agreement
/// is by construction; it is pinned rather than assumed because the
/// one thing the dispatch could do to the plane — re-derive it, or
/// its orientation, before the door — is exactly what this operand
/// would expose while every other row stayed green. (The direct
/// pipeline is private to `topo::splitting`; nothing here can reach
/// it, so that is not the failure this row guards.)
#[test]
fn the_split_dispatch_agrees_with_the_door_through_the_pinch_lane() {
    let prism = pinch_prism();
    let plane = pinch_plane();

    let door = split(&prism, &plane, tol()).unwrap();
    let via = Verb::Split { plane }.run_split(&prism, tol()).unwrap();

    let (SplitPart::Body(slab), SplitPart::Body(pieces)) = (&via.above, &via.below) else {
        panic!("both sides of the pinch cut are bodies: {via:?}");
    };
    assert_eq!(
        pieces.shells().count(),
        3,
        "the pinch lane's three floor pieces did not reach the dispatch"
    );
    // The lane's signature: copies minted for the pieces at each tip
    // line, the slab keeping the one original vertex.
    for z in [0.0, 1.0] {
        assert_eq!(
            vertices_at(pieces, 4.0, 1.0, z),
            2,
            "the pieces side carries no tip copies at z = {z}: the mirror lane was not taken"
        );
        assert_eq!(vertices_at(slab, 4.0, 1.0, z), 1);
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
    let cube = sweep::test_support::cube(1.0, tol());
    let plane = z_plane(5.0);

    let door = split(&cube, &plane, tol()).unwrap();
    assert!(matches!(door.above, SplitPart::Empty));

    let via = Verb::Split { plane }.run_split(&cube, tol()).unwrap();
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
    let empty = Body::<f64>::new();
    let plane = z_plane(0.5);

    let door = split(&empty, &plane, tol()).unwrap_err();
    let via = Verb::Split { plane }.run_split(&empty, tol()).unwrap_err();

    let VerbError::Split(carried) = via else {
        panic!("a split refusal crossed as another family's: {via:?}");
    };
    assert_eq!(format!("{door:?}"), format!("{carried:?}"));
    assert_eq!(door.to_string(), carried.to_string());
}

#[test]
fn the_extrude_dispatch_is_the_extrude_door() {
    let profile = disc(0.5);
    let door = sweep::extrude(&profile, Extrusion::Distance(1.0), tol()).unwrap();
    let via = Verb::Extrude { distance: 1.0 }
        .run_profile(&profile, tol())
        .unwrap();

    let VerbRecord::Extrude(via) = via else {
        panic!("an extrude run produced another family's record");
    };
    assert_eq!(dump(&door.body), dump(&via.body));
    assert_eq!(
        format!("{:?}", door.side_faces),
        format!("{:?}", via.side_faces),
        "the birth record is carried across, not rebuilt"
    );
    assert_eq!(format!("{:?}", door.top), format!("{:?}", via.top));
    assert_eq!(format!("{:?}", door.bottom), format!("{:?}", via.bottom));
    assert_eq!(
        format!("{:?}", door.strut_edges),
        format!("{:?}", via.strut_edges)
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
    let via = Verb::Revolve {
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
        format!("{:?}", door.walls),
        format!("{:?}", via.walls),
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
    let door = sweep::extrude(&profile, Extrusion::Distance(0.0), tol()).unwrap_err();
    let via = Verb::Extrude { distance: 0.0 }
        .run_profile(&profile, tol())
        .unwrap_err();
    let VerbError::Extrude(carried) = via else {
        panic!("an extrude refusal crossed as another family's: {via:?}");
    };
    assert_eq!(format!("{door:?}"), format!("{carried:?}"));
    assert_eq!(door.to_string(), carried.to_string());

    let off = offset_disc(0.25, 1.0);
    let door = sweep::revolve(&off, x_axis(), Revolution::Partial(0.0), tol()).unwrap_err();
    let via = Verb::Revolve {
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
