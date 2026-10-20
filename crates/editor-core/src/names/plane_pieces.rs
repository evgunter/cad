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
//! bounding a divider. Two pieces never share an edge (each half ends with
//! maximal faces), so nothing across a piece's edge is another piece.

use std::collections::{BTreeMap, BTreeSet};

use topo::{Body, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};

use super::emit::NamingError;

fn bug(what: &'static str) -> NamingError {
    NamingError::Emission { what }
}

/// The walls of each of `pieces`, in their order: the faces of `body`
/// across each piece's edges along a divider, each read by `wall`.
/// `islands` is the kernel's `(ring, island)` record, and `piece_of`
/// takes an island's section face to the piece of `body` holding it.
///
/// # Errors
///
/// [`NamingError::Emission`] where a piece or a loop does not walk, an
/// island's ring is no ring of a piece, a piece's outline lies in two
/// rings, or, among several pieces, one meets no divider; and `wall`'s.
pub(super) fn plane_walls<T: geom_core::Real, W: Ord>(
    body: &Body<T>,
    pieces: &[FaceKey],
    islands: &[(LoopKey, FaceKey)],
    piece_of: impl Fn(FaceKey) -> Option<FaceKey>,
    mut wall: impl FnMut(FaceKey) -> Result<W, NamingError>,
) -> Result<Vec<BTreeSet<W>>, NamingError> {
    let ix: BTreeMap<FaceKey, usize> = pieces.iter().enumerate().map(|(i, &f)| (f, i)).collect();
    // Region 0 is the exterior; each ring of a piece bounds its own.
    let mut ring_region: BTreeMap<LoopKey, (usize, FaceKey)> = BTreeMap::new();
    for &f in pieces {
        let face = body
            .get_face(f)
            .ok_or_else(|| bug("a plane piece is not live in its half"))?;
        for &r in &face.rings {
            let n = ring_region.len() + 1;
            ring_region.insert(r, (n, f));
        }
    }
    let mut outline_region: BTreeMap<FaceKey, usize> = BTreeMap::new();
    for &(ring, island) in islands {
        let Some(p) = piece_of(island).filter(|p| ix.contains_key(p)) else {
            continue;
        };
        let &(r, holder) = ring_region
            .get(&ring)
            .ok_or_else(|| bug("an island's ring is no ring of a plane piece"))?;
        if holder == p {
            continue;
        }
        if outline_region.insert(p, r).is_some_and(|was| was != r) {
            return Err(bug("a plane piece's outline lies in two rings"));
        }
    }
    let mut link: Vec<usize> = (0..=ring_region.len()).collect();
    let mut at: BTreeMap<VertexKey, usize> = BTreeMap::new();
    // (region, piece, the loop's half-edges)
    let mut sides: Vec<(usize, usize, Vec<HalfEdgeKey>)> = Vec::new();
    for (p, &f) in pieces.iter().enumerate() {
        let face = body
            .get_face(f)
            .ok_or_else(|| bug("a plane piece is not live in its half"))?;
        for l in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
            let region = if l == face.outer {
                outline_region.get(&f).copied().unwrap_or(0)
            } else {
                ring_region[&l].0
            };
            let hes = loop_half_edges(body, l)?;
            for &he in &hes {
                let v = body
                    .get_half_edge(he)
                    .ok_or_else(|| bug("a plane piece's half-edge is dangling"))?
                    .start;
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
            let across = face_across(body, he)?;
            if !ix.contains_key(&across) {
                walls[p].insert(wall(across)?);
            }
        }
    }
    if pieces.len() >= 2 && walls.iter().any(BTreeSet::is_empty) {
        return Err(bug("a plane piece among several meets no divider"));
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

fn loop_half_edges<T: geom_core::Real>(
    body: &Body<T>,
    l: LoopKey,
) -> Result<Vec<HalfEdgeKey>, NamingError> {
    match body
        .get_loop(l)
        .ok_or_else(|| bug("a plane piece's loop is dangling"))?
        .boundary
    {
        LoopBoundary::Empty { .. } => Ok(Vec::new()),
        LoopBoundary::Cycle { first } => body
            .loop_cycle(first)
            .ok_or_else(|| bug("a plane piece's loop does not walk")),
    }
}

fn face_across<T: geom_core::Real>(
    body: &Body<T>,
    he: HalfEdgeKey,
) -> Result<FaceKey, NamingError> {
    let mate = body
        .mate(he)
        .ok_or_else(|| bug("a plane piece's edge has no mate"))?;
    let mate_he = body
        .get_half_edge(mate)
        .ok_or_else(|| bug("a plane piece's edge mate is dangling"))?;
    Ok(body
        .get_loop(mate_he.parent_loop)
        .ok_or_else(|| bug("the loop across a plane piece's edge is dangling"))?
        .face)
}
