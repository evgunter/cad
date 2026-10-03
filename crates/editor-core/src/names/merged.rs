//! **Reading a set-holding face's constituents** (N3): the questions
//! every consumer of a `Merged` row — or of a sweep's run wall
//! (`names/README.md`, N1 "Swept walls over a run") — asks, answered
//! once.
//!
//! A merged face's name is a FLAT set of face names; a merge over a
//! merged face lists the faces, never the merge. Two things follow
//! for a reader. A name that is itself a merged face — read THROUGH
//! its `FromA`/`FromB` descent wrappers, since a face carried through
//! untouched booleans is still that face — stands for its
//! constituents re-wrapped by that same chain
//! ([`constituents_through_wrappers`]); and a merged row COVERS a name
//! when the name is one of its constituents, or when the name is a
//! merged face all of whose re-wrapped constituents are ([`covers`]).
//! The mint (`emit_topo`'s merge-group loop) uses the first to decide
//! what it publishes; the N3 offer sites and the union's look-through
//! use the second to read what was published. Neither flattens a
//! name: the set is flat because the mint made it so.

use super::role::{EntityKind, MeridianEnd, NameRef, PieceRun, RoleSeg, StableName};

/// The emission bug a nested merged face is — a `Merged` constituent
/// that is itself a merged face, through any wrapping — refused at
/// the mint (`emit_topo`) and again at the union's collapse
/// (`emit_union`): one rule, two doors.
pub(crate) const NESTED_MERGED: &str =
    "a boolean table carries a merged face whose constituent is itself a merged face";

/// The constituents of a merged face read through its descent
/// wrappers, each re-wrapped by that same chain — or `None` when the
/// name, peeled to its foot, is not a merged face.
///
/// Only a bare `FromA`/`FromB` chain is peeled: a foot that carries a
/// tail (`[Merged(cs), Fragment(q)]`) is a FRAGMENT of a merged face,
/// a face in its own right, and is left whole.
pub(crate) fn constituents_through_wrappers(name: &StableName) -> Option<Vec<StableName>> {
    // The wrappers peeled, outermost first, each with its level's node;
    // the foot's constituents are re-wrapped innermost first.
    type Side = fn(NameRef) -> RoleSeg;
    let mut wrappers: Vec<(Side, crate::node::RecipeNodeId)> = Vec::new();
    let mut at = name;
    let foot = loop {
        let (side, inner): (Side, &NameRef) = match at.path.as_slice() {
            [RoleSeg::Merged(cs)] => break cs,
            [RoleSeg::FromA(inner)] => (RoleSeg::FromA, inner),
            [RoleSeg::FromB(inner)] => (RoleSeg::FromB, inner),
            _ => return None,
        };
        wrappers.push((side, at.node));
        at = inner;
    };
    Some(
        foot.iter()
            .map(|c| {
                wrappers
                    .iter()
                    .rev()
                    .fold(c.clone(), |inner, &(side, node)| StableName {
                        kind: EntityKind::Face,
                        node,
                        path: vec![side(NameRef::new(inner))],
                    })
            })
            .collect(),
    )
}

/// The run-holding role of a name's foot: which sweep role holds the
/// run (a meridian with its end), so two feet are the same role over
/// two runs exactly when their [`RunRole`]s are equal.
#[derive(Clone, Copy, PartialEq, Eq)]
enum RunRole {
    Lateral,
    Band,
    BandPi,
    Meridian(MeridianEnd),
}

impl RunRole {
    /// The foot segment of this role over `run`.
    fn seg(self, run: PieceRun) -> RoleSeg {
        match self {
            Self::Lateral => RoleSeg::Lateral(run),
            Self::Band => RoleSeg::Band(run),
            Self::BandPi => RoleSeg::BandPi(run),
            Self::Meridian(end) => RoleSeg::Meridian(end, run),
        }
    }
}

/// A foot that holds a run: its role and the run, or `None`.
fn run_foot(foot: &StableName) -> Option<(RunRole, &PieceRun)> {
    match foot.path.as_slice() {
        [RoleSeg::Lateral(run)] => Some((RunRole::Lateral, run)),
        [RoleSeg::Band(run)] => Some((RunRole::Band, run)),
        [RoleSeg::BandPi(run)] => Some((RunRole::BandPi, run)),
        [RoleSeg::Meridian(end, run)] => Some((RunRole::Meridian(*end), run)),
        _ => None,
    }
}

/// One descent wrapper (`FromA`/`FromB`) peeled off `name`: which side,
/// and the name inside, or `None` when `name` is not one.
fn peel(name: &StableName) -> Option<(bool, &StableName)> {
    match name.path.as_slice() {
        [RoleSeg::FromA(inner)] => Some((true, inner)),
        [RoleSeg::FromB(inner)] => Some((false, inner)),
        _ => None,
    }
}

/// The one-piece walls (or meridian edges) a run of two or more pieces
/// stands for — its `Lateral`, `Band`, `BandPi` or `Meridian(end, ·)`
/// segment spelled once per piece — read through its descent wrappers
/// and re-wrapped by that same chain, or `None` when the name, peeled
/// to its foot, holds no such run. A run wall is not a merge: it holds
/// its pieces' walls the way a merged face holds its constituents, and
/// is read the same way, but nothing mints it as `Merged` and nothing
/// flattens it.
pub(crate) fn run_constituents(name: &StableName) -> Option<Vec<StableName>> {
    type Side = fn(NameRef) -> RoleSeg;
    let mut wrappers: Vec<(Side, crate::node::RecipeNodeId)> = Vec::new();
    let mut at = name;
    while let Some((a, inner)) = peel(at) {
        let side: Side = if a { RoleSeg::FromA } else { RoleSeg::FromB };
        wrappers.push((side, at.node));
        at = inner;
    }
    let (role, run) = run_foot(at)?;
    if run.pieces().len() < 2 {
        return None;
    }
    Some(
        run.pieces()
            .iter()
            .map(|p| {
                let foot = StableName {
                    kind: at.kind,
                    node: at.node,
                    path: vec![role.seg(PieceRun::one(*p))],
                };
                wrappers
                    .iter()
                    .rev()
                    .fold(foot, |inner, &(side, node)| StableName {
                        kind: at.kind,
                        node,
                        path: vec![side(NameRef::new(inner))],
                    })
            })
            .collect(),
    )
}

/// True iff `row`, peeled of its descent wrappers, is a run of two or
/// more pieces — the rows [`run_holds`] can answer true for a name
/// other than themselves.
fn holds_run(row: &StableName) -> bool {
    let mut at = row;
    while let Some((_, inner)) = peel(at) {
        at = inner;
    }
    run_foot(at).is_some_and(|(_, run)| run.pieces().len() >= 2)
}

/// True iff the run row `row` (two or more pieces) holds `name`: the
/// two read through the SAME descent chain to feet of one role on one
/// node, and every piece of `name`'s run is one of `row`'s. A one-piece
/// wall is held by the run that contains its piece, a sub-run by the
/// run around it, and a run by itself. Reads in place: no name is
/// built.
fn run_holds(row: &StableName, name: &StableName) -> bool {
    let (mut r, mut n) = (row, name);
    loop {
        if r.kind != n.kind {
            return false;
        }
        match (peel(r), peel(n)) {
            (Some((ra, ri)), Some((na, ni))) if ra == na && r.node == n.node => {
                (r, n) = (ri, ni);
            }
            (None, None) => break,
            _ => return false,
        }
    }
    let (Some((rr, row_run)), Some((nr, name_run))) = (run_foot(r), run_foot(n)) else {
        return false;
    };
    rr == nr
        && r.node == n.node
        && row_run.pieces().len() >= 2
        && name_run.pieces().iter().all(|p| row_run.holds(p))
}

/// **N3's one constituents view**: the names a set-holding row stands
/// for — a merged face's constituents, or a run's one-piece walls or
/// meridians, each read through the row's descent wrappers — or `None`
/// for a row that holds no set. A loft wall (`LoftWall`) holds one
/// locator per SECTION of one wall, not a set of walls, so it has no
/// constituents here (`names/README.md`, N1).
pub(crate) fn constituents(name: &StableName) -> Option<Vec<StableName>> {
    constituents_through_wrappers(name).or_else(|| run_constituents(name))
}

/// True iff a merged row whose constituent set is `set` covers `name`:
/// `name` is a constituent or is held by one (a run wall the merge
/// listed holds its pieces' walls), or `name` is itself a set-holding
/// face ([`constituents`]) every one of whose constituents is so
/// covered. `set` is sorted, as every minted set is (name order), so
/// membership is a binary search; the run members (the only ones that
/// can hold a name other than themselves) are picked out once, so a
/// set-holding `name` costs `O(|set| + k·(log|set| + runs))`, not
/// `O(k·|set|)`, and the run reading allocates no name.
pub(crate) fn covers(set: &[StableName], name: &StableName) -> bool {
    let runs: Vec<&StableName> = set.iter().filter(|c| holds_run(c)).collect();
    let one = |n: &StableName| set.binary_search(n).is_ok() || runs.iter().any(|c| run_holds(c, n));
    if one(name) {
        return true;
    }
    match constituents(name) {
        Some(cs) => !cs.is_empty() && cs.iter().all(one),
        None => false,
    }
}

/// True iff the live row `row` covers `name` by a set it holds: a
/// merged row by its flat constituent set, a run row by its pieces
/// (`Lateral([p0, p1])` covers `Lateral([p0])`, a run inside its own,
/// and itself).
pub(crate) fn row_covers(row: &StableName, name: &StableName) -> bool {
    run_holds(row, name)
        || row.path.iter().any(|seg| match seg {
            RoleSeg::Merged(set) => covers(set, name),
            _ => false,
        })
}

/// N3's offers for a name that no longer resolves, read off the live
/// `rows`: a merged name's constituents as spelled (the merge stopped
/// happening, so they are live again); for a run, the live rows that
/// cover one of its pieces (an edit broke the run); and every live row
/// that covers the name (it was merged away, or a station joined it
/// into a run). Deterministic: first-seen order, no repeats. A broken
/// run's pieces are built once; the row reads ([`row_covers`]) build
/// nothing.
pub(crate) fn offers<'r>(
    name: &StableName,
    rows: impl Iterator<Item = &'r StableName> + Clone,
) -> Vec<StableName> {
    let mut out: Vec<StableName> = Vec::new();
    let push = |n: &StableName, out: &mut Vec<StableName>| {
        if !out.contains(n) {
            out.push(n.clone());
        }
    };
    for seg in &name.path {
        if let RoleSeg::Merged(constituents) = seg {
            for c in constituents {
                push(c, &mut out);
            }
        }
    }
    if let Some(pieces) = run_constituents(name) {
        for piece in &pieces {
            for row in rows.clone() {
                if row == piece || row_covers(row, piece) {
                    push(row, &mut out);
                }
            }
        }
    }
    for row in rows {
        if row_covers(row, name) {
            push(row, &mut out);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use crate::names::role::{CapEnd, ProfileEdgeRef};
    use crate::node::RecipeNodeId;

    fn face(node: u64, path: Vec<RoleSeg>) -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(node),
            path,
        }
    }

    fn cap(node: u64) -> StableName {
        face(node, vec![RoleSeg::Cap(CapEnd::End)])
    }

    fn from_a(node: u64, inner: StableName) -> StableName {
        face(node, vec![RoleSeg::FromA(inner.into())])
    }

    fn from_b(node: u64, inner: StableName) -> StableName {
        face(node, vec![RoleSeg::FromB(inner.into())])
    }

    fn merged(node: u64, mut set: Vec<StableName>) -> StableName {
        set.sort();
        face(node, vec![RoleSeg::Merged(set)])
    }

    /// The outer row of a boolean over a boolean: the inner merge's
    /// constituents re-wrapped, plus the partner. It covers each
    /// constituent, the consumed inner merged face wrapped once, and
    /// the same face carried through one more untouched step — and
    /// not a face it never listed.
    #[test]
    fn a_flat_row_covers_its_constituents_and_the_merged_faces_they_came_from() {
        let inner = merged(7, vec![from_a(7, cap(2)), from_b(7, cap(5))]);
        let outer_set = {
            let mut s = vec![
                from_a(12, from_a(7, cap(2))),
                from_a(12, from_b(7, cap(5))),
                from_b(12, cap(10)),
            ];
            s.sort();
            s
        };
        assert!(covers(&outer_set, &from_a(12, from_a(7, cap(2)))));
        assert!(covers(&outer_set, &from_b(12, cap(10))));
        assert!(covers(&outer_set, &from_a(12, inner.clone())));
        assert!(!covers(&outer_set, &from_b(12, inner.clone())));
        assert!(!covers(&outer_set, &from_a(12, from_a(7, cap(3)))));
        // Carried through one more untouched step, as operand B.
        let carried_set = {
            let mut s = vec![
                from_a(17, from_b(12, from_a(7, cap(2)))),
                from_a(17, from_b(12, from_b(7, cap(5)))),
                from_b(17, cap(15)),
            ];
            s.sort();
            s
        };
        assert!(covers(&carried_set, &from_a(17, from_b(12, inner))));
        // A fragment of a merged face is a face, not a merge: it is
        // covered only as a constituent.
        let fragment = face(
            12,
            vec![
                RoleSeg::Merged(vec![from_a(12, cap(2))]),
                RoleSeg::Fragment(crate::names::role::Qualifier::OrderAlong { rank: 0, of: 2 }),
            ],
        );
        assert!(constituents_through_wrappers(&fragment).is_none());
        assert!(!covers(&outer_set, &fragment));
    }

    fn lateral(node: u64, steps: &[u64]) -> StableName {
        face(
            node,
            vec![RoleSeg::Lateral(
                PieceRun::new(
                    steps
                        .iter()
                        .map(|&s| ProfileEdgeRef::Piece {
                            step: crate::node::StepId(s),
                            role: crate::names::PieceRole::Leg,
                        })
                        .collect(),
                )
                .unwrap(),
            )],
        )
    }

    /// **A run wall covers its pieces' walls and the runs inside it**,
    /// read through descent wrappers, and is not a merge.
    #[test]
    fn a_run_wall_covers_its_pieces_walls_through_its_wrappers() {
        let run = lateral(3, &[7, 8, 9]);
        assert!(constituents_through_wrappers(&run).is_none(), "not a merge");
        assert_eq!(
            run_constituents(&run).unwrap(),
            vec![lateral(3, &[7]), lateral(3, &[8]), lateral(3, &[9])]
        );
        assert!(row_covers(&run, &lateral(3, &[8])));
        assert!(
            row_covers(&run, &lateral(3, &[7, 8])),
            "a shorter run inside it"
        );
        assert!(!row_covers(&run, &lateral(3, &[6])));
        assert!(!row_covers(&run, &lateral(4, &[8])), "another node's wall");
        assert!(
            run_constituents(&lateral(3, &[7])).is_none(),
            "one piece holds no set"
        );
        let carried = from_a(12, run.clone());
        assert!(row_covers(&carried, &from_a(12, lateral(3, &[9]))));
        assert!(!row_covers(&carried, &from_b(12, lateral(3, &[9]))));
        // A merge that consumed the run wall covers its pieces' walls.
        let set = {
            let mut s = vec![from_a(12, run.clone()), from_b(12, cap(5))];
            s.sort();
            s
        };
        assert!(covers(&set, &from_a(12, lateral(3, &[7]))));
    }

    /// **N3's offers over a run** (`names/README.md`, N1 "Swept walls
    /// over a run"): a selection made before a station was inserted is
    /// offered the run wall that now covers it, and a run an edit broke
    /// offers its pieces' live walls.
    #[test]
    fn a_station_offers_the_run_wall_and_a_broken_run_offers_its_pieces_walls() {
        let run = lateral(3, &[7, 8]);
        let before = lateral(3, &[7]);
        let rows = [run.clone(), lateral(3, &[9]), cap(3)];
        assert_eq!(offers(&before, rows.iter()), vec![run.clone()]);
        let broken = [lateral(3, &[7]), lateral(3, &[8]), cap(3)];
        assert_eq!(
            offers(&run, broken.iter()),
            vec![lateral(3, &[7]), lateral(3, &[8])]
        );
        // A run broken into a piece and a shorter run: both walls.
        let wider = lateral(3, &[7, 8, 9]);
        let split = [lateral(3, &[7]), lateral(3, &[8, 9])];
        assert_eq!(offers(&wider, split.iter()), split.to_vec());
    }

    /// **A meridian over a run is read like a run wall**: the seam
    /// meridian a full revolve keeps over a run of two pieces covers
    /// each piece's own meridian, and a broken run offers them — while
    /// a meridian at another END is a different role and is not
    /// covered.
    #[test]
    fn a_run_meridian_covers_its_pieces_meridians_at_its_own_end() {
        let pieces = |ks: &[u64]| {
            PieceRun::new(
                ks.iter()
                    .map(|&k| ProfileEdgeRef::Piece {
                        step: crate::node::StepId(k),
                        role: crate::names::PieceRole::Leg,
                    })
                    .collect(),
            )
            .unwrap()
        };
        let meridian = |end: MeridianEnd, ks: &[u64]| StableName {
            kind: EntityKind::Edge,
            node: RecipeNodeId(3),
            path: vec![RoleSeg::Meridian(end, pieces(ks))],
        };
        let run = meridian(MeridianEnd::Seam, &[7, 8]);
        assert_eq!(
            run_constituents(&run).unwrap(),
            vec![
                meridian(MeridianEnd::Seam, &[7]),
                meridian(MeridianEnd::Seam, &[8])
            ]
        );
        assert!(row_covers(&run, &meridian(MeridianEnd::Seam, &[8])));
        assert!(!row_covers(&run, &meridian(MeridianEnd::Pi, &[8])));
        let rows = [run.clone()];
        assert_eq!(
            offers(&meridian(MeridianEnd::Seam, &[7]), rows.iter()),
            vec![run.clone()]
        );
    }

    /// **A loft wall holds no set**: its locators are one per section
    /// of one wall, so the constituents view has nothing for it.
    #[test]
    fn a_loft_wall_has_no_constituents() {
        let wall = face(
            3,
            vec![RoleSeg::LoftWall(vec![
                ProfileEdgeRef::Piece {
                    step: crate::node::StepId(1),
                    role: crate::names::PieceRole::Leg,
                },
                ProfileEdgeRef::Piece {
                    step: crate::node::StepId(2),
                    role: crate::names::PieceRole::Leg,
                },
            ])],
        );
        assert!(constituents(&wall).is_none());
    }
}
