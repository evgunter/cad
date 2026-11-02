//! **A placement's bundle, as its caller wrote it** (the helpers
//! ruling, 2026-10-20): [`Placed::of`] is the one constructor of a
//! bundle from the mates and moves a caller wrote, with the constant
//! `0` on each freedom left unnamed. It reads no geometry and writes no
//! number its caller did not write, so a placement stands exactly where
//! its author means it: a quarter turn written `turn/4` is one, and
//! nothing is read back off an evaluation. A gesture that reads geometry
//! names the slides it means before calling it (ASSEMBLY.md A11,
//! `mate::slides_left_along`).
//!
//! [`stand`] is the identity bundle — no mate, and the constant `0` on
//! every freedom — which the place-part gesture writes. A frame a
//! caller means is written by the moves it means ([`moved_frame`]) over
//! a frame read off geometry ([`super::corner::corner_frame`]), or by a
//! mate and values.

use crate::node::RecipeNodeId;
use crate::pose::{PoseDef, PoseRole, RoleRead};
use crate::{Formula, Operand};

use super::coset::SubgroupFamily;
use super::{Coord, Mate, TargetAxis};

/// **A placement's constraints as one write** ([`Placed::of`],
/// [`stand`]): its mates, then its values, each a freedom and its
/// number. A bundle is written whole ([`Placed::bundle`]).
#[derive(Debug, Clone, PartialEq)]
pub struct Placed {
    /// The mates, each read off geometry.
    pub mates: Vec<Mate<Formula>>,
    /// The values, the caller's first in the order written, then the
    /// constant `0` on each freedom left unnamed.
    pub values: Vec<(Coord, Formula)>,
}

impl Placed {
    /// **The identity bundle** (D10 `[placement.values]`): no mate, and
    /// the constant `0` on the slides along the target space's `x`, `y`
    /// and `z` and on a spin about each, in that order. It reads
    /// nothing, so it is the same for every copy.
    #[must_use]
    pub fn identity() -> Self {
        Self::with_families(Vec::new(), &[], Vec::new())
    }

    /// **The bundle its caller wrote** (the helpers ruling): `mates`,
    /// then `moves` in the order written, then the constant `0` on each
    /// freedom the mates leave that no move names — a slide along each
    /// of the target space's (carried) `x`, `y` and `z` until the slides
    /// are as many as the mates leave, and the spin about the target's
    /// own direction, or, where no direction is fixed, the
    /// chain's missing turns about the target space's `x`, `y` and `z`
    /// appended until it turns three times. The freedoms the mates
    /// leave are read off their kinds in the recipe of `doc`, nothing
    /// evaluated; a mate whose kind does not resolve leaves what no mate
    /// would, and the door refuses the bundle typed.
    #[must_use]
    pub fn of<P>(
        doc: &crate::doc::Doc<P>,
        mates: Vec<Mate<Formula>>,
        moves: Vec<(Coord, Formula)>,
    ) -> Self {
        let families: Vec<SubgroupFamily> = mates
            .iter()
            .filter_map(|m| operand_kind(doc, &m.on).and_then(crate::VarKind::symmetry))
            .collect();
        Self::with_families(mates, &families, moves)
    }

    /// **A bundle with no mate**: `moves` in the order written, then the
    /// constant `0` on each freedom of the target space's axes they
    /// leave unnamed — [`Self::of`] with no mate, which reads no
    /// document.
    #[must_use]
    pub fn unmated(moves: Vec<(Coord, Formula)>) -> Self {
        Self::with_families(Vec::new(), &[], moves)
    }

    /// [`Self::of`], the mates' families given.
    fn with_families(
        mates: Vec<Mate<Formula>>,
        families: &[SubgroupFamily],
        moves: Vec<(Coord, Formula)>,
    ) -> Self {
        let left = residual(families.iter().copied());
        let written: Vec<Coord> = moves.iter().map(|(c, _)| *c).collect();
        let mut values = moves;
        values.extend(
            unnamed(left, &written, None)
                .into_iter()
                .map(|coord| (coord, Formula::zero(coord.dimension()))),
        );
        Self { mates, values }
    }

    /// **The freedoms a bundle of `mates` leaves**, each once, in the
    /// order [`Self::of`] writes them: the slides the mates leave (with
    /// `along`, the slides a gesture read off the copy's geometry,
    /// [`super::turn::slides_left_along`]), then the spin about the
    /// target's own direction, or the chain of turns about the target
    /// space's axes where no direction is fixed. The mates' kinds are
    /// read off the recipe of `doc`, nothing evaluated. What an editor
    /// lists as the values a draft's mates leave.
    #[must_use]
    pub fn freedoms<P>(
        doc: &crate::doc::Doc<P>,
        mates: &[Mate<Formula>],
        along: Option<super::SlideAxes>,
    ) -> Vec<Coord> {
        let families = mates
            .iter()
            .filter_map(|m| operand_kind(doc, &m.on).and_then(crate::VarKind::symmetry));
        unnamed(residual(families), &[], along)
    }

    /// **This bundle as one write** ([`crate::DocEdit::place`],
    /// [`crate::DocEdit::SetConstraints`]): each mate and then each
    /// value under the next placeholder id
    /// ([`crate::node::ConstraintId::placeholder`]).
    #[must_use]
    pub fn bundle(self) -> Vec<super::Constraint<Formula>> {
        use crate::node::ConstraintId;
        let mates = self.mates.into_iter().map(super::ConstraintKind::Mate);
        let values = self
            .values
            .into_iter()
            .map(|(coord, by)| super::ConstraintKind::Value(super::Value { coord, by }));
        (1..)
            .zip(mates.chain(values))
            .map(|(i, kind)| super::Constraint {
                id: ConstraintId::placeholder(i),
                kind,
            })
            .collect()
    }

    /// **The edit that writes this bundle onto `placement`** in place of
    /// the one it holds ([`crate::DocEdit::SetConstraints`]).
    #[must_use]
    pub fn write(self, placement: RecipeNodeId) -> crate::DocEdit<crate::ProfileProgram> {
        crate::DocEdit::SetConstraints {
            placement,
            constraints: self.bundle(),
            fresh: Vec::new(),
        }
    }
}

/// **The family a set of mates leaves**: their families met in
/// general position ([`SubgroupFamily::meet`]).
pub(crate) fn residual(families: impl IntoIterator<Item = SubgroupFamily>) -> SubgroupFamily {
    families
        .into_iter()
        .fold(SubgroupFamily::Se3, SubgroupFamily::meet)
}

/// **The freedoms of `left` that `written` leaves unnamed**, in
/// canonical order: a slide along each of `x`, `y` and `z` no value
/// writes until the slides are as many as the residual frees, and the
/// spin about the target's own direction where no value writes it; or,
/// where no direction is fixed, the chain's missing turns about `x`,
/// `y` and `z`, appended until it turns three times. With `along`, the
/// slides a gesture read off the copy's geometry
/// ([`super::turn::slides_left_along`]), the slides written are those
/// alone: each of them no value writes.
pub(crate) fn unnamed(
    left: SubgroupFamily,
    written: &[Coord],
    along: Option<super::SlideAxes>,
) -> Vec<Coord> {
    let mut out = Vec::new();
    let mut slides = TargetAxis::ALL
        .into_iter()
        .filter(|&a| written.contains(&Coord::Slide(a)))
        .count();
    for axis in TargetAxis::ALL {
        let coord = Coord::Slide(axis);
        let open = along.map_or(slides < super::solve::slides_left(left), |set| {
            set.contains(axis)
        });
        if open && !written.contains(&coord) {
            out.push(coord);
            slides += 1;
        }
    }
    if super::solve::chained(left) {
        let chain: Vec<TargetAxis> = written
            .iter()
            .filter_map(|c| match c {
                Coord::Spin(axis) => *axis,
                Coord::Slide(_) => None,
            })
            .collect();
        let mut turns = chain
            .iter()
            .enumerate()
            .filter(|&(k, axis)| k == 0 || chain[k - 1] != *axis)
            .count();
        for axis in TargetAxis::ALL {
            if turns < 3 && !chain.contains(&axis) {
                out.push(Coord::Spin(Some(axis)));
                turns += 1;
            }
        }
    } else {
        let spin = Coord::SPIN;
        if super::solve::leaves(left, spin) && !written.contains(&spin) {
            out.push(spin);
        }
    }
    out
}

/// **Which of a bundle's values a new residual keeps**, in the order
/// written: a value whose freedom `left` leaves, the first of each
/// freedom, the slides along the first axes in `x`, `y`, `z` order up to
/// as many as `left` frees (with `along`, the slides a gesture read off
/// the copy's geometry, those alone), and, where no direction is fixed,
/// each spin of the chain that turns from the one before it, up to
/// three turns. What a mate added or taken away leaves of the values
/// ([`crate::DocEdit::AddConstraint`], [`crate::DocEdit::RemoveConstraint`]);
/// [`unnamed`] then writes the constant `0` on the rest.
pub(crate) fn kept(
    left: SubgroupFamily,
    written: &[Coord],
    along: Option<super::SlideAxes>,
) -> Vec<bool> {
    let chained = super::solve::chained(left);
    let mut seen: Vec<Coord> = Vec::new();
    let mut last: Option<TargetAxis> = None;
    let mut turns = 0;
    // The slides kept are the first the residual leaves in axis order,
    // as [`unnamed`] writes them.
    let slid: Vec<TargetAxis> = TargetAxis::ALL
        .into_iter()
        .filter(|&a| written.contains(&Coord::Slide(a)))
        .filter(|&a| along.is_none_or(|set| set.contains(a)))
        .take(super::solve::slides_left(left))
        .collect();
    written
        .iter()
        .map(|&coord| {
            if !super::solve::leaves(left, coord) {
                return false;
            }
            match coord {
                Coord::Spin(Some(axis)) if chained => {
                    if last == Some(axis) || turns == 3 {
                        return false;
                    }
                    last = Some(axis);
                    turns += 1;
                    true
                }
                _ if seen.contains(&coord) => false,
                Coord::Slide(a) if !slid.contains(&a) => false,
                _ => {
                    seen.push(coord);
                    true
                }
            }
        })
        .collect()
}

/// **The kind a pose operand reads**, off the recipe: a variable's
/// kind, a node output's (a revolve's axis port), a definition's own,
/// or, for a move, a flip or a carried pose, the kind of the read it
/// keeps through its role. `None` where it reads nothing live.
pub fn operand_kind<P>(doc: &crate::doc::Doc<P>, operand: &Operand) -> Option<crate::VarKind> {
    match operand {
        Operand::Var(var) => doc.var(*var).map(crate::Var::kind),
        Operand::Output { node, port } => doc
            .output(*node, *port)
            .and_then(|var| doc.var(var))
            .map(crate::Var::kind),
        Operand::Pose(def) => def.kind().or_else(|| {
            let kept = def.kept()?;
            match kept.role {
                Some(role) => Some(role.kind()),
                None => operand_kind(doc, &kept.read),
            }
        }),
        _ => None,
    }
}

/// **A copy standing where its body stands** (D10 `[placement.values]`:
/// "a placement whose values are all zero and which has no mate is the
/// identity"): the identity bundle, no mate and the constant `0` on
/// every freedom — a slide along each of the target space's axes and a
/// spin about each — so the copy's own coordinates are the target
/// space's. It reads nothing of the copy, so every copy stands alike,
/// at every value of its variables, and whether or not the copy
/// evaluates. The place-part gesture writes it.
#[must_use]
pub fn stand() -> Placed {
    Placed::identity()
}

/// **A move a caller states** about or along one of a frame's own
/// axes, by an angle or a length as written: how a frame meant exactly
/// is written, a quarter turn as `turn/4` (D10 `[variables.literals]`),
/// which a rotation reads exactly.
#[derive(Debug, Clone, PartialEq)]
pub enum FrameMove {
    /// A spin about the axis by the angle, right-handed.
    Spin(super::TargetAxis, Formula),
    /// A slide along the axis by the length.
    Slide(super::TargetAxis, Formula),
}

/// **`base`, a frame read, moved by `moves` as written**, in order,
/// each about or along `base`'s own axes where `base` stands (not the
/// moved frame's), so a chain of spins composes on the left, the first
/// written the first made. The caller states each number; a
/// frame read off geometry at a known pose and moved by exact turns is
/// a frame exactly where the caller means it.
#[must_use]
pub fn moved_frame(base: Operand, moves: &[FrameMove]) -> Operand {
    let axis = |along: super::TargetAxis| {
        RoleRead::through(
            base.clone(),
            match along {
                super::TargetAxis::X => PoseRole::X,
                super::TargetAxis::Y => PoseRole::Y,
                super::TargetAxis::Z => PoseRole::Z,
            },
        )
    };
    moves.iter().fold(base.clone(), |pose, m| {
        Operand::Pose(Box::new(match m {
            FrameMove::Spin(about, by) => PoseDef::Spin {
                pose: RoleRead::plain(pose),
                about: axis(*about),
                by: by.clone(),
            },
            FrameMove::Slide(along, by) => PoseDef::Slide {
                pose: RoleRead::plain(pose),
                along: axis(*along),
                by: by.clone(),
            },
        }))
    })
}
