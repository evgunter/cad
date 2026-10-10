//! **Reading a set-holding face's constituents** (N3): the questions
//! every consumer of a `Merged` row — or of a sweep's run wall
//! (`names/README.md`, N1 "Swept walls over a run") — asks, answered
//! once.
//!
//! A merged face's name is a FLAT set of face names; a merge over a
//! merged face lists the faces, never the merge. Two things follow
//! for a reader. A name that is itself a merged face — read THROUGH
//! its descent wrappers, the `From` an operation carries an input's
//! entity in by, since a face carried through untouched booleans or
//! into a union is still that face — stands for its
//! constituents re-wrapped by that same chain
//! ([`constituents_through_wrappers`]); and a merged row COVERS a name
//! when the name is one of its constituents, or when the name is a
//! merged face all of whose re-wrapped constituents are ([`covers`]).
//! The mint (`emit_topo`'s merge-group loop) uses the first to decide
//! what it publishes; the N3 offer sites and the union's look-through
//! use the second to read what was published. Neither flattens a
//! name: the set is flat because the mint made it so.

use super::role::{CapEnd, EntityKind, MeridianEnd, NameRef, PieceRun, RoleSeg, StableName};

/// The emission bug a nested merged face is — a `Merged` constituent
/// that is itself a merged face, through any wrapping — refused at
/// the mint (`emit_topo`) and again at the union's collapse
/// (`emit_union`): one rule, two doors.
pub(crate) const NESTED_MERGED: &str =
    "a boolean table carries a merged face whose constituent is itself a merged face";

/// One descent wrapper a set-holding name is read through: the `From`
/// an operation carries an input's entity in by, with the read it
/// names, and the shell's `Inner` (a cavity twin). A face carried
/// through any of them untouched is still that face, and a twin of a
/// set is the set of the twins, so its set is read through them and
/// each constituent re-wrapped by the same chain.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Wrap {
    From(crate::VarId),
    Inner,
}

impl Wrap {
    /// This wrapper around `inner`, as the segment minted by `node`.
    fn around(
        self,
        kind: EntityKind,
        node: crate::node::RecipeNodeId,
        inner: StableName,
    ) -> StableName {
        let inner = NameRef::new(inner);
        StableName {
            kind,
            node,
            path: vec![match self {
                Self::From(read) => RoleSeg::From { read, of: inner },
                Self::Inner => RoleSeg::Inner(inner),
            }],
        }
    }
}

/// One descent wrapper peeled off `name`, with the name inside, or
/// `None` when `name` is not one.
fn peel(name: &StableName) -> Option<(Wrap, &StableName)> {
    match name.path.as_slice() {
        [RoleSeg::From { read, of }] => Some((Wrap::From(*read), of)),
        [RoleSeg::Inner(inner)] => Some((Wrap::Inner, inner)),
        _ => None,
    }
}

/// `foot` re-wrapped by `wrappers`, which were peeled outermost first.
fn rewrap(
    kind: EntityKind,
    wrappers: &[(Wrap, crate::node::RecipeNodeId)],
    foot: StableName,
) -> StableName {
    wrappers
        .iter()
        .rev()
        .fold(foot, |inner, &(wrap, node)| wrap.around(kind, node, inner))
}

/// The constituents of a merged face, or of an edge set, read through
/// its descent wrappers ([`Wrap`]), each re-wrapped by that same chain —
/// or `None` when the name, peeled to its foot, is not a `Merged` set.
///
/// Only a bare wrapper chain is peeled: a foot that carries a tail
/// (`[Merged(cs), Fragment(q)]`) is a FRAGMENT of a merged face, a face
/// in its own right, and is left whole.
pub(crate) fn constituents_through_wrappers(name: &StableName) -> Option<Vec<StableName>> {
    let mut wrappers = Vec::new();
    let mut at = name;
    while let Some((wrap, inner)) = peel(at) {
        wrappers.push((wrap, at.node));
        at = inner;
    }
    let [RoleSeg::Merged(foot)] = at.path.as_slice() else {
        return None;
    };
    Some(
        foot.iter()
            .map(|c| rewrap(name.kind, &wrappers, c.clone()))
            .collect(),
    )
}

/// **The name of an edge minted by `node` that lies along the edges
/// `along`**: `Merged` of them, flat and in name order. An edge that is
/// itself a set, read through its descent wrappers, stands for its
/// constituents ([`constituents_through_wrappers`]), so a set of sets
/// lists edges, never sets (N3's flatness). The one builder every door
/// mints an edge set through: the pair boolean, the union, and the
/// joins the split, the shell and the blend end with
/// (`join_names::joined_name`).
pub(crate) fn edge_set(
    node: crate::node::RecipeNodeId,
    along: impl IntoIterator<Item = StableName>,
) -> StableName {
    let mut names = std::collections::BTreeSet::new();
    for n in along {
        match constituents_through_wrappers(&n) {
            Some(cs) => names.extend(cs),
            None => {
                names.insert(n);
            }
        }
    }
    super::canonical::minted(StableName {
        kind: super::role::EntityKind::Edge,
        node,
        path: vec![RoleSeg::Merged(names.into_iter().collect())],
    })
}

/// The run-holding role of a name's foot: which sweep role holds the
/// run (a rim or a meridian with its end), so two feet are the same role over
/// two runs exactly when their [`RunRole`]s are equal.
#[derive(Clone, Copy, PartialEq, Eq)]
enum RunRole {
    Lateral,
    RimEdge(CapEnd),
    Band,
    BandPi,
    Meridian(MeridianEnd),
    AxisEdge,
}

impl RunRole {
    /// The foot segment of this role over `run`.
    fn seg(self, run: PieceRun) -> RoleSeg {
        match self {
            Self::Lateral => RoleSeg::Lateral(run),
            Self::RimEdge(end) => RoleSeg::RimEdge(end, run),
            Self::Band => RoleSeg::Band(run),
            Self::BandPi => RoleSeg::BandPi(run),
            Self::Meridian(end) => RoleSeg::Meridian(end, run),
            Self::AxisEdge => RoleSeg::AxisEdge(run),
        }
    }
}

/// A foot that holds a run: its role and the run, or `None`.
fn run_foot(foot: &StableName) -> Option<(RunRole, &PieceRun)> {
    match foot.path.as_slice() {
        [RoleSeg::Lateral(run)] => Some((RunRole::Lateral, run)),
        [RoleSeg::RimEdge(end, run)] => Some((RunRole::RimEdge(*end), run)),
        [RoleSeg::Band(run)] => Some((RunRole::Band, run)),
        [RoleSeg::BandPi(run)] => Some((RunRole::BandPi, run)),
        [RoleSeg::Meridian(end, run)] => Some((RunRole::Meridian(*end), run)),
        [RoleSeg::AxisEdge(run)] => Some((RunRole::AxisEdge, run)),
        _ => None,
    }
}

/// The one-piece walls (or rim or meridian edges) a run of two or more
/// pieces stands for — its `Lateral`, `RimEdge(end, ·)`, `Band`,
/// `BandPi`, `Meridian(end, ·)` or `AxisEdge` segment spelled once per
/// piece — read through its descent wrappers
/// and re-wrapped by that same chain, or `None` when the name, peeled
/// to its foot, holds no such run. A run wall is not a merge: it holds
/// its pieces' walls the way a merged face holds its constituents, and
/// is read the same way, but nothing mints it as `Merged` and nothing
/// flattens it.
pub(crate) fn run_constituents(name: &StableName) -> Option<Vec<StableName>> {
    let mut wrappers = Vec::new();
    let mut at = name;
    while let Some((wrap, inner)) = peel(at) {
        wrappers.push((wrap, at.node));
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
                rewrap(at.kind, &wrappers, foot)
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
    use crate::names::role::{CapEnd, EntityKind, ProfileEdgeRef};
    use crate::node::RecipeNodeId;

    fn face(node: u64, path: Vec<RoleSeg>) -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId::new(0, node),
            path,
        }
    }

    fn cap(node: u64) -> StableName {
        face(node, vec![RoleSeg::Cap(CapEnd::End)])
    }

    fn from_a(node: u64, inner: StableName) -> StableName {
        face(node, vec![RoleSeg::From { read: crate::names::FOLD_A, of: inner.into() }])
    }

    fn from_b(node: u64, inner: StableName) -> StableName {
        face(node, vec![RoleSeg::From { read: crate::names::FOLD_B, of: inner.into() }])
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

    fn from_member(union: u64, member: u64, inner: StableName) -> StableName {
        face(
            union,
            vec![RoleSeg::From { read: crate::VarId::new(1, member),
                of: inner.into(),
            }],
        )
    }

    /// **A union over a union reads the inner merge through its
    /// `FromMember`** as a boolean over a boolean reads it through
    /// `FromA`: the inner merged face stands for its constituents, each
    /// keyed by the same member, and the outer flat row covers it. The
    /// member id is part of the wrapper: the same face keyed by another
    /// member is not covered.
    #[test]
    fn a_union_over_a_union_reads_the_inner_merge_through_from_member() {
        let inner = merged(
            7,
            vec![from_member(7, 2, cap(2)), from_member(7, 5, cap(5))],
        );
        assert_eq!(
            constituents_through_wrappers(&from_member(12, 7, inner.clone())).unwrap(),
            vec![
                from_member(12, 7, from_member(7, 2, cap(2))),
                from_member(12, 7, from_member(7, 5, cap(5))),
            ]
        );
        let outer_set = {
            let mut s = vec![
                from_member(12, 7, from_member(7, 2, cap(2))),
                from_member(12, 7, from_member(7, 5, cap(5))),
                from_member(12, 10, cap(10)),
            ];
            s.sort();
            s
        };
        assert!(covers(&outer_set, &from_member(12, 7, inner.clone())));
        assert!(!covers(&outer_set, &from_member(12, 8, inner)));
        // The edge set the union mints over a nested joined edge lists
        // its constituents.
        let rim = |n: StableName| StableName {
            kind: EntityKind::Edge,
            node: n.node,
            path: n.path.clone(),
        };
        let joined = rim(merged(
            7,
            vec![
                rim(from_member(7, 2, cap(2))),
                rim(from_member(7, 5, cap(5))),
            ],
        ));
        let set = edge_set(
            RecipeNodeId::new(0, 12),
            [
                rim(from_member(12, 7, joined)),
                rim(from_member(12, 10, cap(10))),
            ],
        );
        let [RoleSeg::Merged(listed)] = set.path.as_slice() else {
            panic!("an edge set is one Merged segment: {set:?}");
        };
        assert_eq!(listed.len(), 3, "flat through FromMember: {listed:?}");
        // A run wall a union carries covers its pieces' walls under the
        // same member.
        let run = from_member(12, 3, lateral(3, &[7, 8]));
        assert!(row_covers(&run, &from_member(12, 3, lateral(3, &[8]))));
        assert!(!row_covers(&run, &from_member(12, 4, lateral(3, &[8]))));
    }

    fn lateral(node: u64, steps: &[u64]) -> StableName {
        face(
            node,
            vec![RoleSeg::Lateral(
                PieceRun::new(
                    steps
                        .iter()
                        .map(|&s| ProfileEdgeRef::Piece {
                            step: crate::node::StepId::new(0, s),
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

    /// **A cap's rim holds its run as the wall does.** A rim named for
    /// one piece before the sweep carried the run as one edge (a
    /// station's rim piece) is offered the run's rim on the same cap,
    /// never the other cap's; the run rim covers each piece's rim.
    #[test]
    fn a_run_rim_is_offered_for_its_pieces_rims_on_its_own_cap() {
        let rim = |end: CapEnd, steps: &[u64]| {
            let run = PieceRun::new(
                steps
                    .iter()
                    .map(|&s| ProfileEdgeRef::Piece {
                        step: crate::node::StepId::new(0, s),
                        role: crate::names::PieceRole::Leg,
                    })
                    .collect(),
            )
            .unwrap();
            StableName {
                kind: EntityKind::Edge,
                node: RecipeNodeId::new(0, 3),
                path: vec![RoleSeg::RimEdge(end, run)],
            }
        };
        let run = rim(CapEnd::End, &[7, 8]);
        assert_eq!(
            run_constituents(&run).unwrap(),
            vec![rim(CapEnd::End, &[7]), rim(CapEnd::End, &[8])]
        );
        assert!(row_covers(&run, &rim(CapEnd::End, &[8])));
        assert!(
            !row_covers(&run, &rim(CapEnd::Start, &[8])),
            "the other cap"
        );
        let rows = [run.clone(), rim(CapEnd::Start, &[7, 8])];
        assert_eq!(offers(&rim(CapEnd::End, &[7]), rows.iter()), vec![run]);
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
                        step: crate::node::StepId::new(0, k),
                        role: crate::names::PieceRole::Leg,
                    })
                    .collect(),
            )
            .unwrap()
        };
        let meridian = |end: MeridianEnd, ks: &[u64]| StableName {
            kind: EntityKind::Edge,
            node: RecipeNodeId::new(0, 3),
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
                    step: crate::node::StepId::new(0, 1),
                    role: crate::names::PieceRole::Leg,
                },
                ProfileEdgeRef::Piece {
                    step: crate::node::StepId::new(0, 2),
                    role: crate::names::PieceRole::Leg,
                },
            ])],
        );
        assert!(constituents(&wall).is_none());
    }
}
