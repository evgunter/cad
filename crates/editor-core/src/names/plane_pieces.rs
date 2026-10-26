//! **N2's `Borders` over a Split's tool plane**: the faces a half holds of
//! the plane are told apart by the operand faces across their edges along
//! a divider (`names/README.md`, "A Split's section faces").
//!
//! An obstacle is a connected part of the plane the half's pieces do not
//! hold, connected through its closure. Each loop of a piece bounds one
//! such part: a ring the part inside it, which it shares with the outline
//! of every piece the kernel placed in that ring (`SplitNaming::islands`);
//! any other outline the exterior. Parts whose loops meet at a vertex are
//! one obstacle. An obstacle that borders two or more pieces divides the
//! plane, and a piece's walls are the faces across its edges on the loops
//! bounding a divider. An edge between two pieces, which a curved group
//! the merge left unglued can leave (`SplitNaming::merge_skipped`), has
//! material on both sides and borders no obstacle, so it cites nothing.

use std::collections::{BTreeMap, BTreeSet};

use topo::{Body, FaceKey, HalfEdgeKey, LoopKey, VertexKey};

use super::emit::{NamingError, loop_half_edges};

/// The walls of each of `pieces`, in their order: the faces of `body`
/// across each piece's edges along a divider, each read by `wall`.
/// `islands` is the kernel's `(ring, island)` record over the faces
/// holding part of the plane, both halves' rows together.
///
/// # Panics
///
/// Where the kernel's record disagrees with `pieces`, where a piece
/// among several meets no divider, or where a piece's own topology does
/// not walk: `pieces` are faces of `body`'s half that the split door
/// recorded, in a body every door keeps tier-1-valid, so each is a
/// kernel bug (the `unreachable!`s say why).
///
/// # Errors
///
/// `wall`'s.
pub(super) fn plane_walls<T: geom_core::Real, W: Ord>(
    body: &Body<T>,
    pieces: &[FaceKey],
    islands: &[(LoopKey, FaceKey)],
    mut wall: impl FnMut(FaceKey) -> Result<W, NamingError>,
) -> Result<Vec<BTreeSet<W>>, NamingError> {
    let ix: BTreeMap<FaceKey, usize> = pieces.iter().enumerate().map(|(i, &f)| (f, i)).collect();
    // Region 0 is the exterior; each ring of a piece bounds its own.
    let mut ring_region: BTreeMap<LoopKey, usize> = BTreeMap::new();
    for &f in pieces {
        let face = piece_face(body, f);
        for &r in &face.rings {
            let n = ring_region.len() + 1;
            ring_region.insert(r, n);
        }
    }
    let mut outline_region: BTreeMap<FaceKey, usize> = BTreeMap::new();
    for &(ring, island) in islands.iter().filter(|(_, i)| ix.contains_key(i)) {
        let Some(&r) = ring_region.get(&ring) else {
            unreachable!(
                "the kernel records each island against a ring of a face its side's plane \
                 pieces hold (`SplitNaming::plane_pieces`), the faces named here"
            );
        };
        if outline_region.insert(island, r).is_some() {
            unreachable!(
                "the nesting rule places each outline in one innermost ring, so the kernel \
                 records it once"
            );
        }
    }
    let mut link: Vec<usize> = (0..=ring_region.len()).collect();
    let mut at: BTreeMap<VertexKey, usize> = BTreeMap::new();
    // (region, piece, the loop's half-edges)
    let mut sides: Vec<(usize, usize, Vec<HalfEdgeKey>)> = Vec::new();
    for (p, &f) in pieces.iter().enumerate() {
        let face = piece_face(body, f);
        for l in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
            let region = if l == face.outer {
                outline_region.get(&f).copied().unwrap_or(0)
            } else {
                ring_region[&l]
            };
            let hes = loop_half_edges(body, l);
            for &he in &hes {
                let Some(half) = body.get_half_edge(he) else {
                    unreachable!(
                        "plane piece {f:?}'s loop {l:?} walks to half-edge {he:?}, which is not \
                         live, though `loop_cycle` walks a tier-1-valid body's live half-edges"
                    )
                };
                let v = half.start;
                match at.get(&v) {
                    Some(&other) => union(&mut link, other, region),
                    None => {
                        at.insert(v, region);
                    }
                }
            }
            sides.push((region, p, hes));
        }
    }
    let mut touched: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for &(region, p, _) in &sides {
        touched
            .entry(root(&mut link, region))
            .or_default()
            .insert(p);
    }
    let mut walls: Vec<BTreeSet<W>> = pieces.iter().map(|_| BTreeSet::new()).collect();
    for (region, p, hes) in sides {
        if touched[&root(&mut link, region)].len() < 2 {
            continue;
        }
        for he in hes {
            let across = face_across(body, he);
            if !ix.contains_key(&across) {
                walls[p].insert(wall(across)?);
            }
        }
    }
    if pieces.len() >= 2 && walls.iter().any(BTreeSet::is_empty) {
        unreachable!(
            "every outline lies in the exterior or in a ring of another piece, so the pieces \
             and the parts they border are connected, and with two pieces each borders a part \
             another borders, along an edge whose other side is an operand face"
        );
    }
    Ok(walls)
}

fn root(link: &mut [usize], mut i: usize) -> usize {
    while link[i] != i {
        link[i] = link[link[i]];
        i = link[i];
    }
    i
}

fn union(link: &mut [usize], a: usize, b: usize) {
    let (a, b) = (root(link, a), root(link, b));
    link[a.max(b)] = a.min(b);
}

/// A plane piece's face, which the split door recorded in `body`.
#[track_caller]
fn piece_face<T: geom_core::Real>(body: &Body<T>, f: FaceKey) -> &topo::Face {
    let Some(face) = body.get_face(f) else {
        unreachable!(
            "plane piece {f:?} is not live in its half, though the split door records the \
             pieces of the half it hands over"
        )
    };
    face
}

/// The face across half-edge `he` of a plane piece: its mate's loop's.
#[track_caller]
fn face_across<T: geom_core::Real>(body: &Body<T>, he: HalfEdgeKey) -> FaceKey {
    let Some(mate) = body.mate(he) else {
        unreachable!("a plane piece's half-edge {he:?} has no mate, though a tier-1 edge has two")
    };
    let Some(mate_he) = body.get_half_edge(mate) else {
        unreachable!("half-edge {he:?}'s mate {mate:?} is not live in a tier-1-valid body")
    };
    let Some(lp) = body.get_loop(mate_he.parent_loop) else {
        unreachable!(
            "half-edge {mate:?}'s loop {:?} is not live in a tier-1-valid body",
            mate_he.parent_loop
        )
    };
    lp.face
}
