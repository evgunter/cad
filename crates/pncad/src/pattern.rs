//! **Patterns are index variables** (D10 `[repetition]`): the programs
//! a pattern writes, and the display-only reading of one back.
//!
//! A pattern is one placement whose values read an index `k = index(N)`:
//! the placement is evaluated once per value of `k`, and its copy is a
//! family of `N` members. The writers here author that program — the
//! index's declaration and the placement — and nothing else; the
//! document's doors decide it like any other edit. A row slides each
//! member `scalar(k)·pitch` along a target axis or a direction read
//! off a body; a ring spins each member `scalar(k)·step` about an
//! `Axis` mate's line or about an axis read in the target; a grid is two
//! indices, one within the other.
//!
//! [`recognise`] reads a placement whose values are multiples of the
//! indices it reaches back as the row, grid or ring it is, for a form
//! to show. It reads the recipe as written and never decides anything: a
//! placement it does not recognise is a placement like any other.

use editor_core::pose::RoleRead;
use editor_core::{
    BodyRead, Coord, Dimension, DocEdit, Formula, Mate, Node, Operand, Placed, ProfileDoc,
    ProfileProgram, RecipeNodeId, SlideAxes, TargetAxis, VarDecl, VarId, VarName,
};

/// An authored node.
pub type AuthoredNode = editor_core::AuthoredNode;

/// **The declaration of `name = index(count)`**, `within` another index
/// or none (D10 `[repetition]`).
#[must_use]
pub fn index_decl(
    name: VarName,
    count: Formula,
    within: Option<&VarName>,
) -> DocEdit<ProfileProgram> {
    DocEdit::DeclareVar {
        name,
        def: VarDecl::Index {
            count,
            within: within.map(|w| Formula::named(w.clone(), Dimension::Count)),
        },
    }
}

/// `scalar(index)`, the index read as a number.
///
/// # Panics
///
/// Never: an index named at `Count` converts.
#[must_use]
pub fn scalar_of(index: &VarName) -> Formula {
    Formula::count_to_scalar(Formula::named(index.clone(), Dimension::Count))
        .unwrap_or_else(|_| unreachable!("a count converts to a scalar"))
}

/// `scalar(index)·by`.
fn times(index: &VarName, by: Formula) -> Result<Formula, editor_core::DimensionError> {
    Formula::mul(scalar_of(index), by)
}

/// **The direction a row runs along** (D10 `[placement.values]`): one
/// of the target space's axes, or a `Direction` or an `Axis` read off a
/// body in the target space, through a role (an edge's line, a
/// carrier's axis, a face's normal, a frame's axis).
#[derive(Clone, Debug, PartialEq)]
pub enum RowDirection {
    /// The target space's axis.
    Axis(TargetAxis),
    /// A direction read off a body in the target space.
    Read(RoleRead<Operand>),
}

impl RowDirection {
    /// The slide along it.
    fn coord(&self) -> Coord<Operand> {
        match self {
            Self::Axis(axis) => Coord::slide(*axis),
            Self::Read(read) => Coord::SlideAlong(read.clone()),
        }
    }
}

/// **A row**: `source` copied into `target`, member `index` slid
/// `scalar(index)·pitch` along each `(direction, pitch)`, with no mate;
/// every other freedom is the constant `0`. `across` names the target
/// axes the other slides run along, which with the row's directions are
/// independent (a row along a read direction names the two its
/// direction is not in the plane of, [`editor_core::mate::slides_across`]);
/// `None` takes `x`, `y` and `z` in order.
///
/// # Errors
///
/// A pitch that is not a length.
pub fn linear_pattern(
    index: &VarName,
    source: BodyRead<Formula>,
    target: Operand,
    steps: &[(RowDirection, Formula)],
    across: Option<SlideAxes>,
) -> Result<AuthoredNode, editor_core::DimensionError> {
    let moves = steps
        .iter()
        .map(|(along, pitch)| Ok((along.coord(), times(index, pitch.clone())?)))
        .collect::<Result<Vec<_>, editor_core::DimensionError>>()?;
    Ok(Node::place(
        source,
        target,
        Placed::unmated_across(moves, across).bundle(),
    ))
}

/// **A grid**: [`linear_pattern`] over two indices, `inner` declared
/// within `outer` ([`index_decl`]), member `(outer, inner)` slid along
/// both; with two directions that are not target axes, a parallelogram
/// lattice.
///
/// # Errors
///
/// A pitch that is not a length.
pub fn grid(
    (outer, along_outer): (&VarName, (RowDirection, Formula)),
    (inner, along_inner): (&VarName, (RowDirection, Formula)),
    source: BodyRead<Formula>,
    target: Operand,
    across: Option<SlideAxes>,
) -> Result<AuthoredNode, editor_core::DimensionError> {
    let moves = vec![
        (along_outer.0.coord(), times(outer, along_outer.1)?),
        (along_inner.0.coord(), times(inner, along_inner.1)?),
    ];
    Ok(Node::place(
        source,
        target,
        Placed::unmated_across(moves, across).bundle(),
    ))
}

/// **How far apart a ring's members turn.**
#[derive(Clone, Debug, PartialEq)]
pub enum RingStep {
    /// A full ring of `count` members: `turn / scalar(count)`.
    Full(Formula),
    /// The angle the author wrote.
    By(Formula),
}

/// **The axis a ring turns about** (D10 `[placement.values]`).
#[derive(Clone, Debug, PartialEq)]
pub enum RingAxis {
    /// One `Axis` mate equating `on`, an axis read off the copy, with
    /// `to`, an axis in the target: each member rides the line, spun
    /// about it and slid `slide` along the mated direction.
    Mated {
        /// The copy's axis.
        on: Operand,
        /// The target's axis.
        to: Operand,
        /// How far along the mated direction every member stands.
        slide: Formula,
    },
    /// An `Axis` read off a body in the target space: each member spun
    /// about that line, with no mate, from where the source stands.
    About(RoleRead<Operand>),
    /// The target space's axis, through its origin: each member spun
    /// about it, with no mate, from where the source stands.
    Target(TargetAxis),
}

/// **A ring**: `source` copied into `target`, member `index` spun
/// `scalar(index)·step` about `axis`, every other freedom the constant
/// `0`.
///
/// # Errors
///
/// A step that is not an angle, or a count that is not a count.
pub fn circular_pattern(
    doc: &ProfileDoc,
    index: &VarName,
    source: BodyRead<Formula>,
    target: Operand,
    axis: RingAxis,
    step: RingStep,
) -> Result<AuthoredNode, editor_core::DimensionError> {
    let step = match step {
        RingStep::Full(count) => Formula::div(Formula::turn(), Formula::count_to_scalar(count)?)?,
        RingStep::By(angle) => angle,
    };
    let spun = times(index, step)?;
    let placed = match axis {
        RingAxis::Mated { on, to, slide } => Placed::of(
            doc,
            vec![Mate {
                on,
                to,
                orient: editor_core::Orientation::IDENTITY,
            }],
            vec![(Coord::SPIN, spun), (Coord::SLIDE, slide)],
        ),
        RingAxis::About(about) => Placed::unmated(vec![(Coord::SpinAbout(about), spun)]),
        RingAxis::Target(axis) => Placed::unmated(vec![(Coord::spin(axis), spun)]),
    };
    Ok(Node::place(source, target, placed.bundle()))
}

/// **A bolt circle**: a full ring of `count` copies of `source` about
/// `axis` ([`circular_pattern`]).
///
/// # Errors
///
/// As [`circular_pattern`].
pub fn bolt_circle(
    doc: &ProfileDoc,
    index: &VarName,
    count: Formula,
    source: BodyRead<Formula>,
    target: Operand,
    axis: RingAxis,
) -> Result<AuthoredNode, editor_core::DimensionError> {
    circular_pattern(doc, index, source, target, axis, RingStep::Full(count))
}

/// **A placement read back as the pattern it is**, for display.
#[derive(Clone, Debug, PartialEq)]
pub enum Recognised {
    /// A row: each slide a multiple of the index, no mate.
    Linear {
        /// The index.
        index: VarId,
        /// Each slide's direction and its step per member.
        steps: Vec<(RowDirection, Formula)>,
    },
    /// A grid: two indices, the inner within the outer, each slide a
    /// multiple of one of them, no mate.
    Grid {
        /// The outer index and its slides' directions and steps per
        /// member.
        outer: (VarId, Vec<(RowDirection, Formula)>),
        /// The inner index and its slides' directions and steps per
        /// member.
        inner: (VarId, Vec<(RowDirection, Formula)>),
    },
    /// A ring: its spin a multiple of the index, about one mate's axis
    /// or about an axis read in the target.
    Circular {
        /// The index.
        index: VarId,
        /// The turn per member.
        step: Formula,
    },
}

/// `formula` as `scalar(index)·step`, the step returned, where it is
/// that product in either order and `step` reads no index.
fn step_of(doc: &ProfileDoc, formula: &Formula, index: VarId) -> Option<Formula> {
    let k = Formula::count_to_scalar(Formula::var(index, Dimension::Count)).ok()?;
    [(0u8, 1u8), (1, 0)].into_iter().find_map(|(a, b)| {
        let (factor, step) = (formula.child(a)?, formula.child(b)?);
        let mut reads = Vec::new();
        step.var_reads(&mut reads);
        let free = reads
            .iter()
            .all(|&(var, _)| doc.var(var).and_then(|v| v.def().index()).is_none());
        (factor.bit_eq(&k) && free && Formula::mul(k.clone(), step.clone()).ok()?.bit_eq(formula))
            .then(|| step.clone())
    })
}

/// **What pattern `placement` is**, where it is one placement whose
/// values are each zero or a multiple of one index it reaches: a row
/// where it reaches one index, has no mate and every such value is a
/// slide; a grid where it reaches two, has no mate, and slides along
/// each; a ring where it reaches one, has one mate and its spin is the
/// multiple. `None` for any other placement.
#[must_use]
pub fn recognise(doc: &ProfileDoc, placement: RecipeNodeId) -> Option<Recognised> {
    let Some(Node::Place { constraints, .. }) = doc.node(placement) else {
        return None;
    };
    let reached = doc.node_reach(placement);
    let chain = reached.chain()?;
    let mates = constraints.iter().filter(|c| c.mate().is_some()).count();
    let mut steps: Vec<Vec<(RowDirection, Formula)>> = vec![Vec::new(); chain.len()];
    let mut spin = None;
    for value in constraints.iter().filter_map(|c| c.value()) {
        let written = doc.written(&editor_core::Expr::var(value.by, value.coord.dimension()));
        if written.is_zero() {
            continue;
        }
        let Some((at, step)) = chain
            .iter()
            .enumerate()
            .find_map(|(at, &index)| Some((at, step_of(doc, &written, index)?)))
        else {
            // A ring's slide along its mated direction is a constant.
            if mates == 1 && value.coord == Coord::SLIDE {
                continue;
            }
            return None;
        };
        match value.coord {
            Coord::Slide(Some(axis)) if mates == 0 => {
                steps[at].push((RowDirection::Axis(axis), step));
            }
            Coord::SlideAlong(read) if mates == 0 => steps[at].push((
                RowDirection::Read(RoleRead {
                    read: Operand::Var(read.read),
                    role: read.role,
                }),
                step,
            )),
            Coord::Spin(None) if mates == 1 && spin.is_none() => spin = Some(step),
            Coord::SpinAbout(_) | Coord::Spin(Some(_)) if mates == 0 && spin.is_none() => {
                spin = Some(step);
            }
            _ => return None,
        }
    }
    match (chain, mates, spin) {
        (&[index], 0, None) if !steps[0].is_empty() => Some(Recognised::Linear {
            index,
            steps: steps.swap_remove(0),
        }),
        (&[outer, inner], 0, None) if steps.iter().all(|s| !s.is_empty()) => {
            let inner_steps = steps.swap_remove(1);
            Some(Recognised::Grid {
                outer: (outer, steps.swap_remove(0)),
                inner: (inner, inner_steps),
            })
        }
        (&[index], 0 | 1, Some(step)) if steps[0].is_empty() => {
            Some(Recognised::Circular { index, step })
        }
        _ => None,
    }
}
