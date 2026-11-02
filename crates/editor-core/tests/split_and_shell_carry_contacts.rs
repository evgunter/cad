//! **A split and a shell in the document carry their operand's contact
//! records** (`topo::SplitResult::carry_contacts`,
//! `topo::Shelled::carry_contacts`). A split's record whose cells lie
//! wholly on one side is that side's alone, and one on cells the cut
//! divides is each side's, along its own pieces; a reader of a side's
//! port reads that side's records, and so does the product. A shell's
//! thin solid holds the records of the walls it keeps.
//!
//! The operand is a union of two blocks touching along the vertical
//! line `x = 1, y = 1`, standing in the world: three records, the two
//! coincident edges and the vertex pair at each end of them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on, failure, run};
use crate::fixture::{self, insert, plane_on};
use editor_core::{Bodies, BodyRead, DocEdit, Evaluation, Node, Operand, ProfileDoc, RecipeNodeId};
use geom_core::Tol;

/// The document, its union and its split by the plane through `origin`
/// along `normal`.
fn split_by(origin: [f64; 3], normal: [f64; 3]) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    split_of(origin, normal, false)
}

/// [`split_by`], the blocks joined by a `⊔` where `assembled`.
fn split_of(
    origin: [f64; 3],
    normal: [f64; 3],
    assembled: bool,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("split-side-contacts", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let read = fixture::body_read(&doc, a.into());
    let (doc, a) = insert(doc, fixture::stood(BodyRead::plain(read)));
    let (doc, b) = block_on(doc, a, (1.0, 2.0), (1.0, 2.0), 0.0, 1.0);
    let members = Bodies::Spelled(vec![a.into(), b.into()]);
    let (doc, union) = insert(
        doc,
        if assembled {
            Node::Assemble { members }
        } else {
            Node::Union { members }
        },
    );
    let tool = plane_on(&doc, union, origin, normal);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: union.into(),
            tool,
        },
    );
    (doc, union, split)
}

/// The records `id`'s value carries.
fn held(ev: &Evaluation<f64>, id: RecipeNodeId, what: &str) -> usize {
    ev.value(id)
        .unwrap_or_else(|| panic!("{what}: {:?}", failure(ev, id)))
        .carry
        .contacts
        .rows()
        .len()
}

/// The records the union carries, and those a one-member union of each
/// side reads, above then below.
fn sides(origin: [f64; 3], normal: [f64; 3], assembled: bool) -> (usize, [usize; 2]) {
    let (doc, union, split) = split_of(origin, normal, assembled);
    let reader = |doc, port| {
        insert(
            doc,
            Node::Union {
                members: Bodies::Spelled(vec![BodyRead::plain(Operand::Output {
                    node: split,
                    port,
                })]),
            },
        )
    };
    let (doc, above) = reader(doc, 0);
    let (doc, below) = reader(doc, 1);
    let ev = run(&doc);
    (
        held(&ev, union, "the union"),
        [
            held(&ev, above, "the above side's reader"),
            held(&ev, below, "the below side's reader"),
        ],
    )
}

/// **A record wholly on one side is that side's**: the plane `x = 0.5`
/// leaves the touch at `x = 1` above it, so the above side holds all
/// three of the union's records and the below side none.
#[test]
fn a_record_wholly_on_one_side_is_that_sides() {
    let (operand, [above, below]) = sides([0.5, 0.5, 0.5], [1.0, 0.0, 0.0], false);
    assert_eq!(
        operand, 3,
        "the union records its edge pair and two vertex pairs"
    );
    assert_eq!(above, 3, "the above side holds every record");
    assert_eq!(below, 0, "the below side holds none");
}

/// **A record the cut divides is each side's**: the plane `z = 0.5`
/// divides the two touching edges, so each side holds its piece of the
/// edge pair, the vertex pair at its own end, and the vertex pair the
/// cut minted where it divided the two edges.
#[test]
fn a_record_the_cut_divides_is_each_sides() {
    let (operand, [above, below]) = sides([0.5, 0.5, 0.5], [0.0, 0.0, 1.0], false);
    assert_eq!(
        operand, 3,
        "the union records its edge pair and two vertex pairs"
    );
    assert_eq!(above, 3, "the above side holds its piece of the touch");
    assert_eq!(below, 3, "the below side holds its piece of the touch");
}

/// **The product reading a side gathers that side's records**: the
/// slot reads the split's above port, the gather's own reading of a
/// split's half.
#[test]
fn the_product_of_a_side_holds_its_records() {
    let (doc, _, split) = split_by([0.5, 0.5, 0.5], [1.0, 0.0, 0.0]);
    let doc = fixture::step(
        doc,
        DocEdit::SetProduct {
            product: Some(Operand::Output {
                node: split,
                port: 0,
            }),
        },
    )
    .0;
    let ev = run(&doc);
    let product = editor_core::product_recorded(&doc, &ev, Tol::witness())
        .unwrap_or_else(|e| panic!("the product gathers: {e}"));
    assert_eq!(
        product.contacts.rows().len(),
        3,
        "the above side's product holds the side's records"
    );
}

/// **A shell carries its operand's records onto the thin solid**: the
/// sealed hollow keeps the outer walls under their keys, so the touch
/// rides it whole.
#[test]
fn a_shell_carries_its_operands_records() {
    let (doc, union, _) = split_by([0.5, 0.5, 0.5], [1.0, 0.0, 0.0]);
    let (doc, shell) = insert(
        doc,
        Node::Shell {
            thickness: fixture::len(0.1),
            open: Operand::select(union, Vec::new()),
        },
    );
    let ev = run(&doc);
    assert_eq!(held(&ev, union, "the union"), 3, "the union's records");
    assert_eq!(
        held(&ev, shell, "the shell"),
        3,
        "the shell holds the touch"
    );
}

/// **A split of a `⊔` carries the touches its census kept**: the
/// blocks assembled, not fused, hold no records of their own, and the
/// `⊔`'s census keeps the touch between them as the union records it,
/// which the above side holds.
#[test]
fn a_split_of_an_assemble_carries_what_its_census_kept() {
    let (_, [above, below]) = sides([0.5, 0.5, 0.5], [1.0, 0.0, 0.0], true);
    assert_eq!(above, 3, "the above side holds the census's three records");
    assert_eq!(below, 0, "the below side holds none");
}

/// **A record the cut parts retires**: the plate under two blocks
/// whose footprints meet at one corner of its top, cut along `y = 1`
/// through that corner and the line the blocks touch along, puts each
/// of the union's two records' cells on different sides. The sides meet
/// there as they meet across the whole cut, a touch between two bodies
/// that neither side's records state, so both sides build and neither
/// holds a record. The carry alone refuses the cut
/// (`topo::CarryRefusal::Parted`).
#[test]
fn a_record_the_cut_parts_retires() {
    let doc = ProfileDoc::empty_derived("split-parts-a-record", Tol::witness());
    let (doc, members) = crate::union_pinch_member_order::pinch(doc);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(members.iter().map(|&m| m.into()).collect()),
        },
    );
    let tool = plane_on(&doc, union, [1.5, 1.0, 0.5], [0.0, 1.0, 0.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: union.into(),
            tool,
        },
    );
    let reader = |doc, port| {
        insert(
            doc,
            Node::Union {
                members: Bodies::Spelled(vec![BodyRead::plain(Operand::Output {
                    node: split,
                    port,
                })]),
            },
        )
    };
    let (doc, above) = reader(doc, 0);
    let (doc, below) = reader(doc, 1);
    let ev = run(&doc);
    assert_eq!(held(&ev, union, "the union"), 2, "the union's two records");
    assert_eq!(held(&ev, above, "the above side"), 0, "both records retire");
    assert_eq!(held(&ev, below, "the below side"), 0, "both records retire");
}

/// **A side's records cite the operand's records across a retired
/// one**: [`a_record_the_cut_parts_retires`]'s pinch with a third block
/// standing against the second's far corner, `(2, 3)`, holds five
/// records. The cut along `y = 1` parts records 2 and 4, the plate's
/// pinch; records 0, 1 and 3, the third block's touch, land above, and
/// the one after the retired record still cites operand record 3.
#[test]
fn a_sides_records_cite_the_operands_across_a_retired_one() {
    let doc = ProfileDoc::empty_derived("split-cites-across-a-retirement", Tol::witness());
    let (doc, mut members) = crate::union_pinch_member_order::pinch(doc);
    let (doc, third) =
        crate::blocks::block_on_of(doc, members[0], ((2.0, 2.5), (3.0, 4.0), 0.6, 1.0));
    members.push(third);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(members.iter().map(|&m| m.into()).collect()),
        },
    );
    let tool = plane_on(&doc, union, [1.5, 1.0, 0.5], [0.0, 1.0, 0.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: union.into(),
            tool,
        },
    );
    let ev = run(&doc);
    assert_eq!(held(&ev, union, "the union"), 5, "the union's five records");
    let sides = ev
        .value(split)
        .unwrap_or_else(|| panic!("the split: {:?}", failure(&ev, split)))
        .carry
        .members
        .clone()
        .expect("a split carries each side's own");
    let cited = |side: usize| -> Vec<u32> {
        sides[side]
            .contacts
            .cites()
            .flat_map(|cites| cites.iter().collect::<Vec<_>>())
            .map(|backing| match backing {
                topo::Backing::Carried { input: 0, record } => record,
                other => panic!("a side's record cites the operand's: {other:?}"),
            })
            .collect()
    };
    assert_eq!(
        cited(0),
        [0, 1, 3],
        "the above side cites operand records 0, 1 and 3"
    );
    assert!(cited(1).is_empty(), "the below side holds none");
}
