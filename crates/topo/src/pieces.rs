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
//! A solid with one shell, or with at most one `Outer`, is already one
//! piece, and nothing is probed (one sign walk per shell decides the
//! count). A solid whose shells are all `Outer` is split one solid per
//! shell with no probe at all: touching pieces are distinct solids, so
//! nothing about where they stand matters. Otherwise each `Void` is
//! read where it stands ([`witness_insides`], check 10's own witness
//! loop): the shells whose closed surface it lies inside nest, so the
//! innermost of them is the one inside all the others, and that shell
//! is the `Void`'s owner. An encloser's own enclosers are read only
//! when a `Void` has more than one, so a shell nothing asks about is
//! never probed.
//!
//! # Refusals
//!
//! Where ownership cannot be read the sort refuses ([`PieceSortError`])
//! naming the shell, before any shell moves: a role the sign walk does
//! not decide, a witness the walk refuses or one every vertex of which
//! touches another shell, enclosers that do not nest (two at one depth:
//! shells that cross), and a `Void` whose innermost encloser is not an
//! `Outer` or that nothing encloses (no piece surrounds it).
//!
//! # The move
//!
//! The first `Outer` of the solid, in its own order, keeps the solid;
//! every other `Outer` moves into a new solid
//! ([`Body::move_shells_to_new_solid`]) with its voids, outer shell
//! first. Ownership moves and nothing else: every face, edge and vertex
//! keeps its key, so contact and lineage records keyed by them stand.

use geom_core::{Band, Decide, Tol};

use crate::body::Body;
use crate::boolean::PointInSolidError;
use crate::entity::{ShellKey, SolidKey};
use crate::props::{QuadLane, ShellRole};
use crate::validate::{Insides, ShellRead, witness_insides};

/// Why the sort could not read which piece a shell belongs to
/// (closed enum, D4 ¶3). Every arm refuses before any shell moves.
#[derive(Clone, Debug)]
pub enum PieceSortError {
    /// The shell's role (`Outer` or `Void`) is not decided: its sign
    /// walk refused, or its sign is still in band when the schedule
    /// runs out.
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
    /// The point-in-solid walk refused at a witness of the shell.
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
    /// The move refused (a desync: every precondition it checks holds
    /// by construction here).
    Move(crate::euler::EulerOpError),
}

// The shell rides in `Debug`; the message names it in words.
impl core::fmt::Display for PieceSortError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::RoleUnread { .. } => write!(
                f,
                "whether a shell of the result bounds material or a cavity could not be \
                 decided, so the piece it belongs to is unknown"
            ),
            Self::WitnessTouching { .. } => write!(
                f,
                "every corner of a shell of the result touches another shell, so the piece \
                 it belongs to could not be read"
            ),
            Self::Probe { source, .. } => write!(
                f,
                "the piece a shell of the result belongs to could not be read: {source}"
            ),
            Self::Crossing { .. } => write!(
                f,
                "two shells of the result cross, and no piece of material is bounded by \
                 crossing shells"
            ),
            Self::NoOwner { .. } => {
                write!(f, "no piece of material surrounds a cavity of the result")
            }
            Self::Move(e) => write!(f, "moving a piece into its own solid failed: {e}"),
        }
    }
}

impl std::error::Error for PieceSortError {}

/// Sorts every solid of `body` into pieces (module docs). `quad` is the
/// lane the shells' roles are read through, as check 10 reads them.
///
/// # Errors
///
/// [`PieceSortError`], before any shell of the refusing solid moves.
pub(crate) fn sort_into_pieces<T: Decide>(
    body: &mut Body<T>,
    band: Band,
    tol: Tol,
    quad: Option<QuadLane<T>>,
) -> Result<(), PieceSortError> {
    let solids: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
    for solid in solids {
        for piece in pieces_of(body, solid, band, tol, quad)? {
            body.move_shells_to_new_solid(&piece)
                .map_err(PieceSortError::Move)?;
        }
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
) -> Result<Vec<Vec<ShellKey>>, PieceSortError> {
    let Some(shells) = body.shells_of_solid(solid) else {
        return Ok(Vec::new());
    };
    if shells.len() < 2 {
        return Ok(Vec::new());
    }
    let reads = shells
        .iter()
        .map(|&shell| {
            ShellRead::of(body, shell, band, tol, quad).ok_or(PieceSortError::RoleUnread { shell })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let outers: Vec<usize> = (0..reads.len())
        .filter(|&i| reads[i].role == ShellRole::Outer)
        .collect();
    if outers.len() < 2 {
        return Ok(Vec::new());
    }

    // Owner of each void, read lazily (module docs).
    let mut enclosers: Vec<Option<Vec<usize>>> = vec![None; reads.len()];
    let mut owner: Vec<Option<usize>> = vec![None; reads.len()];
    for v in (0..reads.len()).filter(|&i| reads[i].role == ShellRole::Void) {
        let around = read_enclosers(body, v, &reads, &mut enclosers, band, tol)?;
        let innermost = match around[..] {
            [] => return Err(PieceSortError::NoOwner { shell: reads[v].shell }),
            [only] => only,
            _ => {
                // Nesting: the innermost encloser lies inside every other
                // one, so it has exactly `around.len() - 1` of them among
                // `around`, and each depth occurs once.
                let mut depths = Vec::with_capacity(around.len());
                for &t in &around {
                    let theirs = read_enclosers(body, t, &reads, &mut enclosers, band, tol)?;
                    depths.push((theirs.iter().filter(|u| around.contains(u)).count(), t));
                }
                depths.sort_unstable();
                if depths.iter().enumerate().any(|(d, &(depth, _))| depth != d) {
                    return Err(PieceSortError::Crossing { shell: reads[v].shell });
                }
                depths[depths.len() - 1].1
            }
        };
        if reads[innermost].role != ShellRole::Outer {
            return Err(PieceSortError::NoOwner { shell: reads[v].shell });
        }
        owner[v] = Some(innermost);
    }

    Ok(outers[1..]
        .iter()
        .map(|&o| {
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

/// The shells whose closed surface `reads[i]` lies inside, memoised in
/// `memo`.
fn read_enclosers<T: Decide>(
    body: &Body<T>,
    i: usize,
    reads: &[ShellRead],
    memo: &mut [Option<Vec<usize>>],
    band: Band,
    tol: Tol,
) -> Result<Vec<usize>, PieceSortError> {
    if let Some(known) = &memo[i] {
        return Ok(known.clone());
    }
    let shell = reads[i].shell;
    let around: Vec<usize> = match witness_insides(body, i, reads, band, tol) {
        Insides::Read(inside) => (0..reads.len()).filter(|&t| inside[t]).collect(),
        Insides::Touching => return Err(PieceSortError::WitnessTouching { shell }),
        Insides::Refused(source) => return Err(PieceSortError::Probe { shell, source }),
    };
    memo[i] = Some(around.clone());
    Ok(around)
}
