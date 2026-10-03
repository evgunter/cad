//! **The result sort: every piece of material a solid of its own**
//! (`docs/DESIGN.md`, "A solid is one piece of material").
//!
//! A solid is one `Outer` shell and the `Void` shells of the cavities in
//! its material. A verb that builds its result under one solid (the
//! boolean's combine door grafts every kept shell under operand A's
//! solid) hands that solid here, and each `Outer` leaves with the voids
//! its material surrounds.
//!
//! # The reading
//!
//! Each shell's role is read off its own sign walk ([`shell_role`]
//! through [`ShellRead`], the reader check 10 uses). A solid with one
//! shell, or with at most one decided `Outer`, is left as it is and
//! nothing is probed. A shell whose role stays undecided is silent
//! there, as check 10 leaves it uncounted: beside at most one decided
//! `Outer` it cannot make a second piece that the count would see.
//!
//! In a solid with two or more decided `Outer`s, every shell is read
//! where it stands ([`witness_insides`], check 10's own witness loop).
//! The shells whose closed surface a shell lies inside nest, so the
//! innermost of them is the one inside all the others:
//!
//! - a `Void`'s innermost encloser is its owner, and must be an `Outer`;
//! - an `Outer`'s innermost encloser, if any, must be a `Void`: an
//!   island stands in a cavity. An `Outer` straight inside another
//!   `Outer` is overlapping material, which no piece is bounded by.
//!
//! An encloser's own enclosers are read only when a shell has more than
//! one, so a shell nothing asks about is probed once. The boolean hands
//! over its sweep's certified face boxes, and two shells whose boxes are
//! apart cannot nest and are never probed against each other (`Screen`),
//! so its pieces side by side cost no probe and record no decision;
//! `split` and `shell` hand none, and every pair of theirs is probed.
//! A probe reads a ray that crosses nothing off
//! the shell's role, already decided ([`ShellRead`]), rather than off a
//! closed-form volume a curved face may not certify.
//!
//! # Refusals
//!
//! Where ownership cannot be read the sort refuses ([`PieceSortError`])
//! naming the shell, and no shell of the body moves: every solid's
//! pieces are read before the first move.
//!
//! # The move
//!
//! The first `Outer` of the solid, in its own order, keeps the solid;
//! every other `Outer` moves into a new solid
//! ([`Body::move_shells_to_new_solid`]) with its voids, outer shell
//! first. Ownership moves and nothing else: every face, edge and vertex
//! keeps its key, so contact and lineage records keyed by them stand.
//!
//! [`shell_role`]: crate::validate::shell_role

use geom_core::{Band, Decide, Tol};

use crate::body::Body;
use crate::boolean::PointInSolidError;
use crate::entity::{FaceKey, ShellKey, SolidKey};
use crate::props::{QuadLane, ShellRole};
use crate::validate::{Insides, ShellRead, witness_insides};

/// Why the sort could not read which piece a shell belongs to
/// (closed enum, D4 ¶3). Every arm refuses before any shell of the body
/// moves.
#[derive(Clone, Debug)]
pub enum PieceSortError {
    /// The shell's role (`Outer` or `Void`) is not decided — its sign
    /// walk refused, or its sign is still in band when the schedule
    /// runs out — in a solid with two decided `Outer` shells, so which
    /// piece it belongs to matters and is unknown.
    RoleUnread {
        /// The shell.
        shell: ShellKey,
    },
    /// Every vertex of the shell lies on another shell, so no witness
    /// of it says where it stands.
    WitnessTouching {
        /// The shell.
        shell: ShellKey,
    },
    /// The point-in-solid walk refused at a witness of the shell. No
    /// row reaches it: on the shells a verb builds today, a walk that
    /// refuses here has refused that verb earlier, in its own probes.
    Probe {
        /// The shell.
        shell: ShellKey,
        /// The walk's refusal.
        source: PointInSolidError,
    },
    /// Two shells around this one stand at one depth, so they do not
    /// nest: shells that cross, which no piece is bounded by.
    Crossing {
        /// The shell whose surroundings do not nest.
        shell: ShellKey,
    },
    /// No piece's material surrounds this `Void`: nothing encloses it,
    /// or the innermost shell around it is another `Void`.
    NoOwner {
        /// The cavity.
        shell: ShellKey,
    },
    /// This `Outer` stands straight inside another `Outer`, with no
    /// cavity between: overlapping material, which no piece is bounded
    /// by.
    Overlapping {
        /// The inner `Outer`.
        shell: ShellKey,
    },
    /// The move refused (a desync: every precondition it checks holds
    /// by construction here).
    Move(crate::euler::EulerOpError),
}

// The shell rides in `Debug`; the message names it in words. The sort
// reads operands as well as results (`shell` sorts the body it is
// handed), so a shape no verb builds may have been read from a file.
// Every arm is one sentence and then its one ending.
impl core::fmt::Display for PieceSortError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let (what, ending) = match self {
            Self::RoleUnread { .. } => (
                "whether a shell of the body bounds material or a cavity could not be \
                 decided, so the piece it belongs to is unknown"
                    .to_owned(),
                geom_core::KERNEL_LIMIT_RECOURSE,
            ),
            Self::WitnessTouching { .. } => (
                "every corner of a shell of the body touches another shell, so the piece \
                 it belongs to could not be read"
                    .to_owned(),
                geom_core::NOT_YET_ENDING,
            ),
            Self::Probe { source, .. } => {
                return write!(
                    f,
                    "the piece a shell of the body belongs to could not be read: {source}"
                );
            }
            Self::Crossing { .. } => (
                "two shells of the body cross, and no piece of material is bounded by \
                 crossing shells"
                    .to_owned(),
                geom_core::KERNEL_OR_FILE_DEFECT_ENDING,
            ),
            Self::NoOwner { .. } => (
                "no piece of material surrounds a cavity of the body".to_owned(),
                geom_core::KERNEL_OR_FILE_DEFECT_ENDING,
            ),
            Self::Overlapping { .. } => (
                "a shell of the body stands inside another piece's material with no cavity \
                 between, so their material overlaps"
                    .to_owned(),
                geom_core::KERNEL_OR_FILE_DEFECT_ENDING,
            ),
            Self::Move(e) => (
                format!("moving a piece into its own solid failed: {e}"),
                geom_core::KERNEL_DEFECT_ENDING,
            ),
        };
        write!(f, "{what}. {ending}")
    }
}

impl std::error::Error for PieceSortError {}

/// Sorts every solid of `body` into pieces (module docs). `quad` is the
/// lane the shells' roles are read through, as check 10 reads them.
/// `face_box`, when given, is a certified padded box of a face: two
/// shells whose faces' boxes hull apart are never probed against each
/// other ([`Screen`]). The boolean passes its sweep's boxes; a caller
/// without them passes `None` and every pair is probed.
///
/// # Errors
///
/// [`PieceSortError`], before any shell of the body moves.
pub(crate) fn sort_into_pieces<T: Decide>(
    body: &mut Body<T>,
    band: Band,
    tol: Tol,
    quad: Option<QuadLane<T>>,
    face_box: Option<FaceBox<'_, T>>,
) -> Result<(), PieceSortError> {
    let solids: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
    let mut moves = Vec::new();
    for solid in solids {
        moves.extend(pieces_of(body, solid, band, tol, quad, face_box)?);
    }
    for piece in moves {
        body.move_shells_to_new_solid(&piece)
            .map_err(PieceSortError::Move)?;
    }
    Ok(())
}

/// The pieces of `solid` that leave it, each as the shell list its new
/// solid takes (outer shell first, then its voids in the solid's order).
fn pieces_of<T: Decide>(
    body: &Body<T>,
    solid: SolidKey,
    band: Band,
    tol: Tol,
    quad: Option<QuadLane<T>>,
    face_box: Option<FaceBox<'_, T>>,
) -> Result<Vec<Vec<ShellKey>>, PieceSortError> {
    let Some(shells) = body.shells_of_solid(solid) else {
        return Ok(Vec::new());
    };
    if shells.len() < 2 {
        return Ok(Vec::new());
    }
    let read: Vec<Option<ShellRead>> = shells
        .iter()
        .map(|&shell| ShellRead::of(body, shell, band, tol, quad))
        .collect();
    let decided_outers = read
        .iter()
        .flatten()
        .filter(|r| r.role == ShellRole::Outer)
        .count();
    if decided_outers < 2 {
        return Ok(Vec::new());
    }
    let reads = read
        .into_iter()
        .zip(shells)
        .map(|(r, &shell)| r.ok_or(PieceSortError::RoleUnread { shell }))
        .collect::<Result<Vec<_>, _>>()?;

    let screen = Screen::of(body, &reads, face_box);
    let mut enclosers: Vec<Option<Vec<usize>>> = vec![None; reads.len()];
    let mut owner: Vec<Option<usize>> = vec![None; reads.len()];
    for i in 0..reads.len() {
        let shell = reads[i].shell;
        let inner = innermost(body, i, &reads, &screen, &mut enclosers, band, tol)?;
        match (reads[i].role, inner.map(|t| reads[t].role)) {
            (ShellRole::Void, Some(ShellRole::Outer)) => owner[i] = inner,
            (ShellRole::Void, _) => return Err(PieceSortError::NoOwner { shell }),
            (ShellRole::Outer, Some(ShellRole::Outer)) => {
                return Err(PieceSortError::Overlapping { shell });
            }
            (ShellRole::Outer, _) => {}
        }
    }

    Ok((0..reads.len())
        .filter(|&i| reads[i].role == ShellRole::Outer)
        .skip(1)
        .map(|o| {
            core::iter::once(reads[o].shell)
                .chain(
                    (0..reads.len())
                        .filter(|&v| owner[v] == Some(o))
                        .map(|v| reads[v].shell),
                )
                .collect()
        })
        .collect())
}

/// The innermost of the shells `reads[i]` lies inside, or `None` when it
/// lies inside none. The enclosers of a shell nest, so the innermost has
/// exactly `around.len() - 1` of them among `around`, and each depth
/// occurs once; two at one depth are shells that cross.
fn innermost<T: Decide>(
    body: &Body<T>,
    i: usize,
    reads: &[ShellRead],
    screen: &Screen,
    memo: &mut [Option<Vec<usize>>],
    band: Band,
    tol: Tol,
) -> Result<Option<usize>, PieceSortError> {
    let around = read_enclosers(body, i, reads, screen, memo, band, tol)?;
    match around[..] {
        [] => Ok(None),
        [only] => Ok(Some(only)),
        _ => {
            let mut depths = Vec::with_capacity(around.len());
            for &t in &around {
                let theirs = read_enclosers(body, t, reads, screen, memo, band, tol)?;
                depths.push((theirs.iter().filter(|u| around.contains(u)).count(), t));
            }
            depths.sort_unstable();
            if depths.iter().enumerate().any(|(d, &(depth, _))| depth != d) {
                return Err(PieceSortError::Crossing {
                    shell: reads[i].shell,
                });
            }
            Ok(depths.last().map(|&(_, t)| t))
        }
    }
}

/// The shells whose closed surface `reads[i]` lies inside, memoised in
/// `memo`.
fn read_enclosers<T: Decide>(
    body: &Body<T>,
    i: usize,
    reads: &[ShellRead],
    screen: &Screen,
    memo: &mut [Option<Vec<usize>>],
    band: Band,
    tol: Tol,
) -> Result<Vec<usize>, PieceSortError> {
    if let Some(known) = &memo[i] {
        return Ok(known.clone());
    }
    let shell = reads[i].shell;
    let around: Vec<usize> =
        match witness_insides(body, i, reads, &|t| screen.may_meet(i, t), band, tol) {
            Insides::Read(inside) => (0..reads.len()).filter(|&t| inside[t]).collect(),
            Insides::Touching => return Err(PieceSortError::WitnessTouching { shell }),
            Insides::Refused(source) => return Err(PieceSortError::Probe { shell, source }),
        };
    memo[i] = Some(around.clone());
    Ok(around)
}

/// A certified padded box of a face, `None` where it does not build:
/// the boolean's sweep boxes ([`crate::boolean::boxes::face_box`]).
pub(crate) type FaceBox<'a, T> = &'a dyn Fn(&Body<T>, FaceKey) -> Option<bvh::Aabb>;

/// Each shell's box — the hull of its faces' [`FaceBox`]es — so two
/// shells whose boxes are certified apart are never probed against each
/// other: one cannot lie inside the other. A shell whose box does not
/// build, or every shell when no `FaceBox` is given, is screened out of
/// nothing.
struct Screen(Vec<Option<bvh::Aabb>>);

impl Screen {
    fn of<T: Decide>(
        body: &Body<T>,
        reads: &[ShellRead],
        face_box: Option<FaceBox<'_, T>>,
    ) -> Self {
        Self(
            reads
                .iter()
                .map(|r| {
                    let face_box = face_box?;
                    r.sel
                        .faces()
                        .iter()
                        .try_fold(None, |hull: Option<bvh::Aabb>, &f| {
                            let b = face_box(body, f)?;
                            Some(Some(hull.map_or(b, |h| h.hull(&b))))
                        })?
                })
                .collect(),
        )
    }

    fn may_meet(&self, i: usize, t: usize) -> bool {
        match (&self.0[i], &self.0[t]) {
            (Some(a), Some(b)) => a.overlaps(b),
            _ => true,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::{Band, Tol};

    use super::{PieceSortError, sort_into_pieces};
    use crate::body::Body;
    use crate::test_support_fixtures::brick;

    type Range = (f64, f64);

    /// Every `(box, inside_out)` filed under one solid: an outward box is
    /// an `Outer` shell, an inside-out one a `Void`.
    fn one_solid(boxes: &[([Range; 3], bool)]) -> Body<f64> {
        let tol = Tol::witness();
        let mut body = Body::<f64>::new();
        for &([x, y, z], inside_out) in boxes {
            let b = brick(x, y, z, tol);
            let b = if inside_out { b.revert().unwrap() } else { b };
            crate::graft_disjoint_all_keyed(&mut body, &b).unwrap();
        }
        body.merge_all_solids().unwrap();
        body
    }

    fn sort(body: &mut Body<f64>) -> Result<(), PieceSortError> {
        let tol = Tol::witness();
        sort_into_pieces(body, Band::linear(tol).unwrap(), tol, None, None)
    }

    /// **A cavity no piece surrounds refuses**: two side-by-side cubes
    /// and an inside-out cube outside both.
    #[test]
    fn a_void_outside_every_outer_has_no_owner() {
        let mut body = one_solid(&[
            ([(0.0, 1.0), (0.0, 1.0), (0.0, 1.0)], false),
            ([(3.0, 4.0), (0.0, 1.0), (0.0, 1.0)], false),
            ([(6.0, 7.0), (0.0, 1.0), (0.0, 1.0)], true),
        ]);
        let cavity = body.shells().nth(2).unwrap().0;
        assert!(
            matches!(sort(&mut body), Err(PieceSortError::NoOwner { shell }) if shell == cavity)
        );
        assert_eq!(body.solids().count(), 1, "nothing moved");
    }

    /// **Shells that cross refuse rather than guess**: a cube and a bar
    /// through it, neither's corners inside the other, and a cavity
    /// inside both — two shells around it at one depth.
    #[test]
    fn a_void_inside_two_crossing_outers_refuses_as_crossing() {
        let mut body = one_solid(&[
            ([(0.0, 2.0), (0.0, 2.0), (0.0, 2.0)], false),
            ([(0.5, 1.5), (-1.0, 3.0), (0.5, 1.5)], false),
            ([(0.8, 1.2), (0.8, 1.2), (0.8, 1.2)], true),
        ]);
        let cavity = body.shells().nth(2).unwrap().0;
        assert!(
            matches!(sort(&mut body), Err(PieceSortError::Crossing { shell }) if shell == cavity)
        );
        assert_eq!(body.solids().count(), 1, "nothing moved");
    }

    /// **Side by side**: three cubes and no cavity split one solid per
    /// cube, the first keeping the original solid.
    #[test]
    fn outers_alone_split_one_solid_each() {
        let mut body = one_solid(&[
            ([(0.0, 1.0), (0.0, 1.0), (0.0, 1.0)], false),
            ([(3.0, 4.0), (0.0, 1.0), (0.0, 1.0)], false),
            ([(6.0, 7.0), (0.0, 1.0), (0.0, 1.0)], false),
        ]);
        let keeper = body.solids().next().unwrap().0;
        let order: Vec<_> = body.shells().map(|(k, _)| k).collect();
        sort(&mut body).unwrap();
        assert_eq!(body.shells_of_solid(keeper).unwrap(), &order[..1]);
        let mut filed: Vec<_> = body.solids().map(|(_, s)| s.shells.clone()).collect();
        filed.sort();
        assert_eq!(filed, order.iter().map(|&s| vec![s]).collect::<Vec<_>>());
    }

    /// **An `Outer` straight inside an `Outer` is overlapping material**:
    /// two pieces' worth of shells, one inside the other with no cavity
    /// between, refuse rather than split into two solids that overlap.
    #[test]
    fn an_outer_inside_an_outer_refuses_as_overlapping() {
        let mut body = one_solid(&[
            ([(0.0, 6.0), (0.0, 6.0), (0.0, 6.0)], false),
            ([(2.0, 4.0), (2.0, 4.0), (2.0, 4.0)], false),
        ]);
        let inner = body.shells().nth(1).unwrap().0;
        assert!(matches!(
            sort(&mut body),
            Err(PieceSortError::Overlapping { shell }) if shell == inner
        ));
        assert_eq!(body.solids().count(), 1, "nothing moved");
    }

    /// **An undecided role beside two decided `Outer`s refuses**: a
    /// sheet `(1 + K)·ε` thick, whose signed volume stays in band.
    #[test]
    fn an_in_band_shell_beside_two_pieces_refuses_role_unread() {
        let tol = Tol::witness();
        let t = (1.0 + tol.k()) * tol.eps();
        let mut body = one_solid(&[
            ([(0.0, 1.0), (0.0, 1.0), (0.0, 1.0)], false),
            ([(3.0, 4.0), (0.0, 1.0), (0.0, 1.0)], false),
            ([(6.0, 7.0), (0.0, 1.0), (0.0, t)], false),
        ]);
        let sheet = body.shells().nth(2).unwrap().0;
        assert!(matches!(
            sort(&mut body),
            Err(PieceSortError::RoleUnread { shell }) if shell == sheet
        ));
    }

    /// **A shell every corner of which touches another refuses**: a
    /// cavity whose corners all lie on the outer cube around it, beside
    /// a second cube.
    #[test]
    fn a_shell_touching_at_every_corner_refuses_witness_touching() {
        let mut body = one_solid(&[
            ([(0.0, 2.0), (0.0, 2.0), (0.0, 2.0)], false),
            ([(5.0, 6.0), (0.0, 1.0), (0.0, 1.0)], false),
            ([(0.0, 2.0), (0.0, 2.0), (0.0, 1.0)], true),
        ]);
        assert!(matches!(
            sort(&mut body),
            Err(PieceSortError::WitnessTouching { .. })
        ));
    }
}
