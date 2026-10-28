//! **Placements**: every copy of a body the tour puts somewhere is a
//! `Node::place` of it, its bundle the mates and values the scene
//! means ([`Answer`]) — each number the scene writes, nothing read back
//! off an evaluation:
//!
//! - [`place_in_world`]: the copy in the world where it stands
//!   ([`stand`]: the identity bundle, no mate and the constant `0` on
//!   every freedom), so it follows every variable its body reads;
//! - [`in_world_by`]: the copy in the world moved by values on the
//!   world's axes, as written;
//! - [`against`]: the copy placed into another body's space by values
//!   on that space's axes — how a construction combined with another
//!   joins its space;
//! - [`on_axis`]: a revolve's copy mated by its own axis to a frame's
//!   `z`.
//!
//! A pose a split or a pattern reads is a frame read off a body's
//! corners ([`corner_frame`]) and moved by the turns and slides the
//! scene means ([`moved_frame`]).
//!
//! A placement is one edit, the insert of the placement holding its
//! whole bundle; [`Placed::edits`] is that edit, so a scene that
//! exports its edit log replays exactly what it applied.
//!
//! **The product** is what a document says it is about, and the scene
//! says it ([`set_product`]): one copy, or a `⊔` of several. Nothing
//! is its product until the scene sets it.

use pncad::document::pose::{PoseDef, PoseRole, RoleRead};
use pncad::document::{
    BodyRead, Coord, Doc, DocEdit, EvalOptions, Formula, FrameMove, Mate, Operand,
    Placed as Answer, ProfileProgram, RecipeNodeId, TargetAxis, apply, corner_frame, moved_frame,
    stand,
};
use pncad::geom_core::Tol;
use pncad::prelude::Dimension;

/// What a placement left: the document, the placement it minted, and
/// the edits it applied, in order.
pub struct Placed {
    pub doc: Doc<ProfileProgram>,
    pub placement: RecipeNodeId,
    pub edits: Vec<DocEdit<ProfileProgram>>,
}

/// What setting the product left: the document and the edits it
/// applied, in order.
pub struct Producted {
    pub doc: Doc<ProfileProgram>,
    pub edits: Vec<DocEdit<ProfileProgram>>,
}

/// **The product set to `bodies`**: the one body itself, or a `⊔` of
/// several in the order given — the "this is what the document makes"
/// gesture.
///
/// # Panics
///
/// If `bodies` is empty or an edit refuses.
pub fn set_product(doc: &Doc<ProfileProgram>, bodies: &[Operand], tol: Tol) -> Producted {
    let mut doc = doc.clone();
    let mut edits = Vec::new();
    let product = match bodies {
        [] => panic!("a product is at least one body"),
        [one] => one.clone(),
        several => {
            let insert = DocEdit::InsertNode {
                node: Box::new(pncad::document::Node::Assemble {
                    members: pncad::document::Bodies::Spelled(
                        several.iter().cloned().map(BodyRead::plain).collect(),
                    ),
                }),
                fresh: Vec::new(),
            };
            let applied = apply(&doc, &insert, tol)
                .unwrap_or_else(|err| panic!("the product's bodies assemble: {err}"));
            doc = applied.doc;
            edits.push(insert);
            Operand::Node(applied.record.minted.expect("an insert mints an id"))
        }
    };
    let set = DocEdit::SetProduct {
        product: Some(product),
    };
    doc = apply(&doc, &set, tol)
        .unwrap_or_else(|err| panic!("the product is set: {err}"))
        .doc;
    edits.push(set);
    Producted { doc, edits }
}

/// **The product said anew** as `bodies` ([`set_product`]): the `⊔` the
/// product read before is deleted first, so the product is set anew
/// rather than beside it.
///
/// # Panics
///
/// If `bodies` is empty or an edit refuses.
pub fn replace_product(doc: &Doc<ProfileProgram>, bodies: &[Operand], tol: Tol) -> Producted {
    let held = doc
        .product()
        .and_then(|var| doc.operation_of(var))
        .filter(|&at| matches!(doc.node(at), Some(pncad::document::Node::Assemble { .. })));
    let mut edits = Vec::new();
    let mut doc = doc.clone();
    if let Some(id) = held {
        let delete = DocEdit::DeleteNode { id };
        doc = apply(&doc, &delete, tol)
            .unwrap_or_else(|err| panic!("the old product's `⊔` deletes: {err}"))
            .doc;
        edits.push(delete);
    }
    let set = set_product(&doc, bodies, tol);
    edits.extend(set.edits);
    Producted {
        doc: set.doc,
        edits,
    }
}

/// One copy of `body` in the world where it stands ([`stand`]).
///
/// # Panics
///
/// If an edit refuses.
pub fn place_in_world(doc: &Doc<ProfileProgram>, body: impl Into<Operand>, tol: Tol) -> Placed {
    written(doc, body.into(), world(), stand(), tol)
}

/// One copy of `body` in the world placed by `values` on the world's
/// axes as written — the spins a chain of turns about the world's `x`,
/// `y` and `z` through the copy's origin, the slides where they carry
/// it — each freedom no value names written `0`.
///
/// # Panics
///
/// If an edit refuses.
pub fn in_world_by(
    doc: &Doc<ProfileProgram>,
    body: impl Into<Operand>,
    values: Vec<(Coord, Formula)>,
    tol: Tol,
) -> Placed {
    written(doc, body.into(), world(), Answer::unmated(values), tol)
}

/// A copy of `body` placed into `target`'s space by `values` on that
/// space's axes, as written ([`in_world_by`]'s rule). The placement's
/// output is the copy, in `target`'s space.
///
/// # Panics
///
/// If an edit refuses.
pub fn against(
    doc: &Doc<ProfileProgram>,
    target: impl Into<Operand>,
    body: impl Into<Operand>,
    values: Vec<(Coord, Formula)>,
    tol: Tol,
) -> Placed {
    let (target, body) = (target.into(), body.into());
    let space = space_of(doc, &target);
    written(doc, body, space, Answer::unmated(values), tol)
}

/// **The slides along the space's `x`, `y` and `z`** by `by`, each
/// that is not zero.
pub fn slides(by: [f64; 3]) -> Vec<(Coord, Formula)> {
    TargetAxis::ALL
        .into_iter()
        .zip(by)
        .filter(|&(_, by)| by != 0.0)
        .map(|(axis, by)| {
            let by = Formula::literal(by, Dimension::Length).expect("a finite length");
            (Coord::Slide(axis), by)
        })
        .collect()
}

/// `turn/den`, negated where `neg`: an exact fraction of a turn.
pub fn turn(den: u64, neg: bool) -> Formula {
    let t = Formula::turn_over(den).expect("a turn is an angle");
    if neg {
        Formula::neg(t).expect("an angle negates")
    } else {
        t
    }
}

/// **A pose read off `body`'s corners** ([`corner_frame`]: `z` along
/// its edge from `corner` to `up`, `x` toward its vertex at `toward`)
/// **and moved by `moves`** as written, about and along that frame's
/// own axes ([`moved_frame`]).
///
/// # Panics
///
/// If `body` has no line edge from `corner` to `up` or no vertex at
/// `toward`.
pub fn frame_on(
    doc: &Doc<ProfileProgram>,
    body: impl Into<Operand>,
    [corner, up, toward]: [[f64; 3]; 3],
    moves: &[FrameMove],
    tol: Tol,
) -> Operand {
    let body = body.into();
    let base = corner_frame(doc, &body, corner, up, toward, &EvalOptions::default(), tol)
        .unwrap_or_else(|refusal| panic!("a frame is read off {body:?}'s corners: {refusal}"));
    moved_frame(base, moves)
}

/// **A copy of the full revolve `revolve` standing on the `z` of
/// `frame`** — a frame read off the body it joins and stated by exact
/// moves ([`pncad::document::moved_frame`]), so a quarter turn is
/// `turn/4` and the axis is exactly where it is meant: an `Axis` mate
/// from the revolve's own axis, under the orientation `orient` that puts
/// that axis on the frame's `z`, slid along it by `slide` — the target
/// axis the orientation puts the revolve's axis on, and its coordinate
/// there — its spin about it the constant `0`.
///
/// # Panics
///
/// If an edit refuses.
pub fn on_axis(
    doc: &Doc<ProfileProgram>,
    revolve: RecipeNodeId,
    frame: Operand,
    slide: (TargetAxis, Formula),
    orient: pncad::document::Orientation,
    tol: Tol,
) -> Placed {
    let space = space_of(doc, &frame);
    let to = Operand::Pose(Box::new(PoseDef::Slide {
        pose: RoleRead::through(frame.clone(), PoseRole::Z),
        along: RoleRead::through(frame, PoseRole::Z),
        by: Formula::zero(Dimension::Length),
    }));
    let mate = Mate {
        on: Operand::Output {
            node: revolve,
            port: 1,
        },
        to,
        orient,
    };
    // The slide an axis mate leaves, written; its spin is the constant 0.
    let placed = Answer::of(doc, vec![mate], vec![(Coord::Slide(slide.0), slide.1)]);
    written(doc, Operand::output(revolve, 0), space, placed, tol)
}

/// The world's space, as a placement's target reads it.
fn world() -> Operand {
    Operand::Var(pncad::document::VarId::WORLD)
}

/// **The space `target` is in**, as a placement's target reads it: a
/// copy placed against a body joins that body's space.
fn space_of(doc: &Doc<ProfileProgram>, target: &Operand) -> Operand {
    Operand::Var(
        doc.operand_space(target)
            .unwrap_or_else(|| panic!("the target is a body in a space: {target:?}")),
    )
}

/// The placement of `body` into `space` written as one edit: the insert
/// of the placement holding what the door answered
/// ([`pncad::document::stand`] and the rest), its whole bundle.
fn written(
    doc: &Doc<ProfileProgram>,
    body: Operand,
    space: Operand,
    placed: Answer,
    tol: Tol,
) -> Placed {
    let edit = DocEdit::place(BodyRead::plain(body), space, placed.bundle());
    let applied =
        apply(doc, &edit, tol).unwrap_or_else(|err| panic!("the placement applies: {err}"));
    let placement = applied.record.minted.expect("an insert mints an id");
    let doc = applied.doc;
    let edits = vec![edit];
    Placed {
        doc,
        placement,
        edits,
    }
}

/// **A body where it stands in its space**, for a picture or a
/// measurement read off it: the façade's read-out door
/// ([`pncad::export::body_in_space`]), which composes the value's body
/// into its space's coordinates. A body computed in its space's
/// coordinates is itself.
pub fn in_its_space<S: crate::scalar::Scalar>(
    body: &pncad::document::Framed<S>,
    tol: Tol,
) -> pncad::topo::Body<S> {
    pncad::topo::Body::clone(
        &pncad::export::body_in_space(body, tol)
            .unwrap_or_else(|e| panic!("a body composes into its space: {e}")),
    )
}
