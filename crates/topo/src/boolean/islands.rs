//! **Every island a solid of its own** — the boolean's output
//! convention for a material component that stands inside a cavity of
//! another.
//!
//! The combine door grafts every kept shell of both operands under
//! operand A's one solid, so a result whose material falls apart leaves
//! the pieces under that solid. Several `Outer` shells side by side
//! under one solid are the ruled assembly shape (tier 3's check 10
//! admits them on purpose). An `Outer` shell standing inside a `Void` of
//! the same solid is not: the material it bounds touches none of the
//! wall around that void, so it is a solid of its own. Tier 3 admits that
//! shape too (`+1 - 1 + 1 = 1`), so filing it is the boolean's job, at
//! the one point every result path passes before its gate.
//!
//! # The reading
//!
//! A solid with fewer than three shells, or without both a `Void` and a
//! second `Outer`, has no island, and nothing is read. Otherwise each
//! shell's role is read off its own closed-form signed volume
//! ([`selection_role`], the at-infinity read of the point-in-solid walk)
//! and each shell's ENCLOSERS (the other shells whose closed surface it
//! lies inside) off one vertex of it probed against each other shell
//! alone ([`point_in_solid_faces`] over [`SolidFaces::of_shell`], the
//! walk tier 3's check 10 reads the same nesting with). Shells do not
//! cross, so a shell's enclosers nest, and its PARENT is the encloser
//! with the most enclosers of its own.
//!
//! An island is an `Outer` whose parent is a `Void`. It moves into a new
//! solid ([`Body::move_shells_to_new_solid`]) with every `Void` whose
//! parent it is, outer shell first. An island inside a void of an island
//! moves on its own turn, so each material component ends up a solid of
//! its own. A shell whose parent is none stays, so the source solid
//! never empties. The move re-partitions ownership only: every face,
//! edge and vertex keeps its key, and the contact and naming records
//! keyed by them stand as they are.
//!
//! # Where the reading cannot answer
//!
//! A refused role read or a refused probe refuses the boolean
//! ([`BooleanError::Containment`]): a result whose solids cannot be told
//! apart is not handed back under a guessed grouping. A vertex that
//! lies ON another shell says nothing about nesting, so the next vertex
//! of the shell is tried. A shell every vertex of which touches another
//! shell leaves its solid as the graft filed it
//! (`work/fuse/island-filing-is-silent-where-every-vertex-touches.md`).

use geom_core::{Band, Decide, Tol};

use super::BooleanError;
use super::solid_contain::{SolidContainment, SolidFaces, point_in_solid_faces, selection_role};
use crate::body::Body;
use crate::entity::{ShellKey, SolidKey};
use crate::props::ShellRole;
use crate::validate::shell_vertices;

/// Moves every island of every solid of `body` into a solid of its own
/// (module docs).
///
/// # Errors
///
/// [`BooleanError::Containment`] where a role read or a nesting probe
/// refuses; [`BooleanError::Euler`] where the move refuses (a desync:
/// every precondition it checks holds by construction here).
pub(super) fn file_islands<T: Decide>(
    body: &mut Body<T>,
    band: Band,
    tol: Tol,
) -> Result<(), BooleanError> {
    let solids: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
    for solid in solids {
        for island in islands_of(body, solid, band, tol)? {
            body.move_shells_to_new_solid(&island)
                .map_err(BooleanError::Euler)?;
        }
    }
    Ok(())
}

/// The islands of `solid`, each as the shell list its new solid takes
/// (outer shell first, then its voids in the solid's order).
fn islands_of<T: Decide>(
    body: &Body<T>,
    solid: SolidKey,
    band: Band,
    tol: Tol,
) -> Result<Vec<Vec<ShellKey>>, BooleanError> {
    let shells = body
        .shells_of_solid(solid)
        .ok_or(BooleanError::JoinDesync {
            what: "island filing: a result solid does not resolve",
        })?;
    if shells.len() < 3 {
        return Ok(Vec::new());
    }
    let read = shells
        .iter()
        .map(|&shell| {
            let sel = SolidFaces::of_shell(body, shell).map_err(BooleanError::Containment)?;
            let role =
                selection_role(body, sel.faces(), band, tol).map_err(BooleanError::Containment)?;
            Ok((shell, role, sel))
        })
        .collect::<Result<Vec<_>, BooleanError>>()?;
    let count = |role| read.iter().filter(|(_, r, _)| *r == role).count();
    if count(ShellRole::Void) == 0 || count(ShellRole::Outer) < 2 {
        return Ok(Vec::new());
    }

    let mut enclosers: Vec<Vec<usize>> = Vec::with_capacity(read.len());
    for (i, &(shell, _, _)) in read.iter().enumerate() {
        let Some(found) = enclosers_of(body, shell, i, &read, band, tol)? else {
            return Ok(Vec::new());
        };
        enclosers.push(found);
    }
    let parent = |i: usize| {
        enclosers[i]
            .iter()
            .copied()
            .max_by_key(|&t| enclosers[t].len())
    };

    Ok(read
        .iter()
        .enumerate()
        .filter(|&(i, (_, role, _))| {
            *role == ShellRole::Outer && parent(i).is_some_and(|p| read[p].1 == ShellRole::Void)
        })
        .map(|(i, &(island, _, _))| {
            core::iter::once(island)
                .chain(
                    read.iter()
                        .enumerate()
                        .filter(|&(v, (_, role, _))| {
                            *role == ShellRole::Void && parent(v) == Some(i)
                        })
                        .map(|(_, &(void, _, _))| void),
                )
                .collect()
        })
        .collect())
}

/// The indices into `read` of the shells whose closed surface `shell`
/// (at index `i`) lies inside, read at its first vertex that touches no
/// other shell; `None` when every vertex touches one.
fn enclosers_of<T: Decide>(
    body: &Body<T>,
    shell: ShellKey,
    i: usize,
    read: &[(ShellKey, ShellRole, SolidFaces)],
    band: Band,
    tol: Tol,
) -> Result<Option<Vec<usize>>, BooleanError> {
    'witness: for witness in shell_vertices(body, shell) {
        let mut found = Vec::new();
        for (t, (_, role, sel)) in read.iter().enumerate() {
            if t == i {
                continue;
            }
            // A selection's material is its outer shell's inside, or a
            // void's outside: inside the closed surface is `In` for the
            // one and `Out` for the other.
            let inside = match point_in_solid_faces(body, sel, witness, band, tol)
                .map_err(BooleanError::Containment)?
            {
                SolidContainment::In => *role == ShellRole::Outer,
                SolidContainment::Out => *role == ShellRole::Void,
                SolidContainment::OnBoundary => continue 'witness,
            };
            if inside {
                found.push(t);
            }
        }
        return Ok(Some(found));
    }
    Ok(None)
}
