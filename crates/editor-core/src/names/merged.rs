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

use super::role::{EntityKind, NameRef, PieceRun, RoleSeg, StableName};

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

/// The one-piece walls a run wall of two or more pieces stands for —
/// its `Lateral`, `Band` or `BandPi` segment spelled once per piece —
/// read through its descent wrappers and re-wrapped by that same chain,
/// or `None` when the name, peeled to its foot, is not such a wall. A
/// run wall is not a merge: it holds its pieces' walls the way a merged
/// face holds its constituents, and is read the same way, but nothing
/// mints it as `Merged` and nothing flattens it.
pub(crate) fn run_constituents(name: &StableName) -> Option<Vec<StableName>> {
    type Side = fn(NameRef) -> RoleSeg;
    let mut wrappers: Vec<(Side, crate::node::RecipeNodeId)> = Vec::new();
    let mut at = name;
    let (seg, run): (fn(PieceRun) -> RoleSeg, &PieceRun) = loop {
        let (side, inner): (Side, &NameRef) = match at.path.as_slice() {
            [RoleSeg::Lateral(run)] => break (RoleSeg::Lateral, run),
            [RoleSeg::Band(run)] => break (RoleSeg::Band, run),
            [RoleSeg::BandPi(run)] => break (RoleSeg::BandPi, run),
            [RoleSeg::FromA(inner)] => (RoleSeg::FromA, inner),
            [RoleSeg::FromB(inner)] => (RoleSeg::FromB, inner),
            _ => return None,
        };
        wrappers.push((side, at.node));
        at = inner;
    };
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
                    path: vec![seg(PieceRun::one(*p))],
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

/// **N3's one constituents view**: the faces a set-holding row stands
/// for — a merged face's constituents or a run wall's one-piece walls,
/// each read through the row's descent wrappers — or `None` for a row
/// that holds no set.
pub(crate) fn constituents(name: &StableName) -> Option<Vec<StableName>> {
    constituents_through_wrappers(name).or_else(|| run_constituents(name))
}

/// True iff a row whose constituent set is `set` covers `name`: `name`
/// is a constituent or is held by one (a run wall the merge listed
/// holds its pieces' walls), or `name` is itself a set-holding face
/// ([`constituents`]) every one of whose constituents is so covered.
pub(crate) fn covers(set: &[StableName], name: &StableName) -> bool {
    let one = |n: &StableName| {
        set.contains(n)
            || set
                .iter()
                .any(|c| run_constituents(c).is_some_and(|held| held.contains(n)))
    };
    if one(name) {
        return true;
    }
    match constituents(name) {
        Some(cs) => !cs.is_empty() && cs.iter().all(one),
        None => false,
    }
}

/// True iff the live row `row` covers `name` by a set it holds: a
/// merged row by its flat constituent set, a run wall by its pieces'
/// walls (`Lateral([p0, p1])` covers `Lateral([p0])` and a run inside
/// its own).
pub(crate) fn row_covers(row: &StableName, name: &StableName) -> bool {
    let merged = row.path.iter().any(|seg| match seg {
        RoleSeg::Merged(set) => covers(set, name),
        _ => false,
    });
    merged || run_constituents(row).is_some_and(|set| covers(&set, name))
}

/// N3's offers for a name that no longer resolves, read off the live
/// `rows`: a merged name's constituents as spelled (the merge stopped
/// happening, so they are live again); for a run wall, the live walls
/// that cover its pieces (an edit broke the run); and every live row
/// that covers the name (it was merged away, or a station joined it
/// into a run). Deterministic: first-seen order, no repeats.
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
}
