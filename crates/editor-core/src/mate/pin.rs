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
    pub values: Vec<(Coord<Operand>, Formula)>,
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
        moves: Vec<(Coord<Operand>, Formula)>,
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
    pub fn unmated(moves: Vec<(Coord<Operand>, Formula)>) -> Self {
        Self::with_families(Vec::new(), &[], moves)
    }

    /// [`Self::of`], the mates' families given.
    fn with_families(
        mates: Vec<Mate<Formula>>,
        families: &[SubgroupFamily],
        moves: Vec<(Coord<Operand>, Formula)>,
    ) -> Self {
        let left = residual(families.iter().copied());
        let written: Vec<Coord<Operand>> = moves.iter().map(|(c, _)| c.clone()).collect();
        let mut values = moves;
        values.extend(unnamed(left, &written, None).into_iter().map(|coord| {
            let zero = Formula::zero(coord.dimension());
            (coord, zero)
        }));
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
    ) -> Vec<Coord<Operand>> {
        let families = mates
            .iter()
            .filter_map(|m| operand_kind(doc, &m.on).and_then(crate::VarKind::symmetry));
        unnamed(residual(families), &[] as &[Coord<Operand>], along)
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
pub(crate) fn unnamed<R: Clone + PartialEq>(
    left: SubgroupFamily,
    written: &[Coord<R>],
    along: Option<super::SlideAxes>,
) -> Vec<Coord<R>> {
    let mut out = Vec::new();
    let mut slides = slid(left, written, None).len();
    for axis in TargetAxis::ALL {
        let coord = Coord::slide(axis);
        let open = along.map_or(slides < super::solve::slides_left(left), |set| {
            set.contains(axis)
        });
        if open && !written.contains(&coord) {
            out.push(coord);
            slides += 1;
        }
    }
    if super::solve::chained(left) {
        let chain: Vec<&Coord<R>> = written.iter().filter(|c| !c.is_slide()).collect();
        let mut turns = chain
            .iter()
            .enumerate()
            .filter(|&(k, link)| k == 0 || chain[k - 1] != *link)
            .count();
        for axis in TargetAxis::ALL {
            if turns < 3 && !written.contains(&Coord::spin(axis)) {
                out.push(Coord::spin(axis));
                turns += 1;
            }
        }
    } else {
        let spin = Coord::SPIN;
        if leaves(left, &spin) && !written.contains(&spin) {
            out.push(spin);
        }
    }
    out
}

/// [`super::solve::leaves`] of a freedom in either form: a read is
/// left wherever its form is.
fn leaves<R>(left: SubgroupFamily, coord: &Coord<R>) -> bool {
    let form = match coord {
        Coord::Slide(axis) => Coord::Slide(*axis),
        Coord::Spin(axis) => Coord::Spin(*axis),
        Coord::SlideAlong(_) => {
            Coord::SlideAlong(crate::pose::RoleRead::plain(crate::VarId::WORLD))
        }
        Coord::SpinAbout(_) => Coord::SpinAbout(crate::pose::RoleRead::plain(crate::VarId::WORLD)),
    };
    super::solve::leaves(left, form)
}

/// **The written slides `left` keeps**, each once: those along the
/// target space's axes in `x`, `y`, `z` order (with `along`, those of
/// the set alone), then the mated one, then each read, in the order
/// written, up to as many as `left` frees.
fn slid<'a, R: PartialEq>(
    left: SubgroupFamily,
    written: &'a [Coord<R>],
    along: Option<super::SlideAxes>,
) -> Vec<&'a Coord<R>> {
    let mut out: Vec<&Coord<R>> = Vec::new();
    let axes = TargetAxis::ALL.into_iter().filter_map(|a| {
        written
            .iter()
            .find(|&c| *c == Coord::slide(a))
            .filter(|_| along.is_none_or(|set| set.contains(a)))
    });
    let rest = written
        .iter()
        .filter(|c| matches!(c, Coord::Slide(None)))
        .chain(written.iter().filter(|c| matches!(c, Coord::SlideAlong(_))));
    for c in axes.chain(rest) {
        if leaves(left, c) && !out.contains(&c) {
            out.push(c);
        }
    }
    out.truncate(super::solve::slides_left(left));
    out
}

/// **Which of a bundle's values a new residual keeps**, in the order
/// written: a value whose freedom `left` leaves, the first of each
/// freedom, the slides [`slid`] keeps (with `along`, the slides a
/// gesture read off the copy's geometry, those alone among the target
/// axes), and, where no direction is fixed, each spin of the chain that
/// turns from the one before it, up to three turns. What a mate added or
/// taken away leaves of the values ([`crate::DocEdit::AddConstraint`],
/// [`crate::DocEdit::RemoveConstraint`]); [`unnamed`] then writes the
/// constant `0` on the rest.
pub(crate) fn kept<R: PartialEq>(
    left: SubgroupFamily,
    written: &[Coord<R>],
    along: Option<super::SlideAxes>,
) -> Vec<bool> {
    let chained = super::solve::chained(left);
    let mut seen: Vec<&Coord<R>> = Vec::new();
    let mut last: Option<&Coord<R>> = None;
    let mut turns = 0;
    let slid = slid(left, written, along);
    written
        .iter()
        .map(|coord| {
            if !leaves(left, coord) {
                return false;
            }
            if chained && !coord.is_slide() {
                if last == Some(coord) || turns == 3 {
                    return false;
                }
                last = Some(coord);
                turns += 1;
                return true;
            }
            if seen.contains(&coord) || (coord.is_slide() && !slid.contains(&coord)) {
                return false;
            }
            seen.push(coord);
            true
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
