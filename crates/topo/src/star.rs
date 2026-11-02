//! **A vertex's neighbourhood cut by a surface** (DESIGN
//! `[one-way.places]`): the runs of its orbit on the side a verb moves,
//! where each run's null edge lands, the tree its struts hang as, and
//! the mint. A verb supplies its entries' sides ([`Member`]) and which
//! circle carries its chords; the star reads the orbit.
//!
//! # Sites
//!
//! A null edge moving one arc of a vertex's orbit to a new vertex (its
//! copy) lands at one [`Site`], which only this module builds:
//!
//! - **a fan**: the arc holds some, not all, of the orbit's real
//!   half-edges, and `mev(Fan { he1, he2 })` moves them, `he1` the
//!   arc's first and `he2` the orbit successor of its last
//!   ([`Body::run_site`]), read in the body as it stands, so earlier
//!   splices at the vertex are counted;
//! - **a strut whose tip stands for the arc**: the arc holds no real
//!   half-edge (subdivision duplicates of one sector alone), so nothing
//!   moves, and the strut `mev(Fan { c, c })` splices into the corner
//!   the arc lies in, before the sector's next real half-edge
//!   ([`Body::strut_before_next_real`]); or a strut a verb places
//!   itself ([`strut`]);
//! - **a strut whose tip stands for the rest**: the arc holds every
//!   real half-edge, so what it excludes lies inside the one corner it
//!   does not cross, between the arrival `prev(first)` and the
//!   departure `first`. Moving the whole arc would leave the old vertex
//!   bare, so the strut splices into that corner: the old vertex keeps
//!   the arc and the tip stands for the excluded side. Walked the other
//!   way round the orbit the arc is empty, and its strut is this one;
//! - **the lone first strut of a bare ring vertex** ([`hang_ring`]).
//!
//! # The mint
//!
//! [`Body::mev_null_run`] takes a site and the run's side and nothing
//! else. By the sense theorem (`boolean::join`'s module docs) the half
//! facing the run's start germ is UP exactly when the run is above/OUT,
//! so which half faces the start and which side the copy takes are one
//! fact: a fan's `he_plus` (old → copy) runs along the arc's first cut,
//! facing the start, and its copy takes the run's side; a strut's tip
//! takes the run's side exactly when it stands for the arc. The flip
//! for a strut whose tip stands for the rest happens here.

use geom_core::{Decide, Real};

use crate::body::{Body, WALKS_CLOSE};
use crate::entity::{EntityId, HalfEdgeKey, LoopKey, VertexKey};
use crate::euler::{EulerOpError, MevCreated, MevSite};
use crate::live::proven;
use crate::null::{NewVertexSide, NullEdge};

/// One entry of a verb's sector walk round a vertex, as the star reads
/// it.
pub(crate) trait Member {
    /// The orbit half-edge the entry's sector carries.
    fn he(&self) -> HalfEdgeKey;
    /// Whether the entry is a real edge of the orbit, not a subdivision
    /// duplicate.
    fn real(&self) -> bool;
    /// Whether the entry lies on the side the verb's runs move to their
    /// copies.
    fn moved(&self) -> bool;
}

/// A maximal cyclic run of moved entries, by entry index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Run {
    /// Index of the run's first entry.
    pub start: usize,
    /// Number of entries in the run (≥ 1).
    pub len: usize,
}

/// The maximal cyclic runs of moved entries, in order from the first
/// entry that is not moved (the anchor). A one-sided neighbourhood
/// (every entry moved, or none) has no runs: nothing to separate, no
/// null edge.
pub(crate) fn runs<E: Member>(entries: &[E]) -> Vec<Run> {
    let n = entries.len();
    let Some(anchor) = entries.iter().position(|e| !e.moved()) else {
        return Vec::new();
    };
    let mut runs = Vec::new();
    let mut k = 0;
    while k < n {
        let start = (anchor + 1 + k) % n;
        if entries[start].moved() {
            let mut len = 0;
            while k < n && entries[(anchor + 1 + k) % n].moved() {
                len += 1;
                k += 1;
            }
            runs.push(Run { start, len });
        } else {
            k += 1;
        }
    }
    runs
}

/// Which side a strut's tip stands for, the arc it was minted for or
/// the rest of the orbit; equally, which germ its `he_plus` faces, the
/// run's start or its end (module docs, "The mint").
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Tip {
    /// The tip stands for the arc: `he_plus` faces the run's start.
    Arc,
    /// The tip stands for the rest: `he_plus` faces the run's end.
    Rest,
}

impl Tip {
    /// The tip whose `he_plus` faces the run's start exactly when
    /// `plus_faces_start`.
    pub(crate) fn facing(plus_faces_start: bool) -> Self {
        if plus_faces_start {
            Self::Arc
        } else {
            Self::Rest
        }
    }
}

/// Where a run's null edge lands (module docs, "Sites"). Built only
/// here, so the mint never meets a site a verb spelled itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Site(RunSite);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RunSite {
    /// The arc `he1 .. he2` (exclusive) is a proper part of the orbit.
    Fan { he1: HalfEdgeKey, he2: HalfEdgeKey },
    /// The strut spliced into the corner before `corner`.
    Strut { corner: HalfEdgeKey, tip: Tip },
    /// The first strut of the bare vertex `r#loop` holds.
    Lone { r#loop: LoopKey, tip: Tip },
}

impl Site {
    /// The corner's departure half-edge where the site is a strut into
    /// a corner of the orbit.
    pub(crate) fn strut_corner(self) -> Option<HalfEdgeKey> {
        match self.0 {
            RunSite::Strut { corner, .. } => Some(corner),
            RunSite::Fan { .. } | RunSite::Lone { .. } => None,
        }
    }

    /// Whether the null edge dangles: a strut, whose tip holds no other
    /// edge.
    pub(crate) fn dangling(self) -> bool {
        !matches!(self.0, RunSite::Fan { .. })
    }

    /// Which side the minted copy stands for: a fan's copy takes the arc.
    pub(crate) fn tip(self) -> Tip {
        match self.0 {
            RunSite::Fan { .. } => Tip::Arc,
            RunSite::Strut { tip, .. } | RunSite::Lone { tip, .. } => tip,
        }
    }
}

/// The strut a verb places itself, into the corner before `corner`,
/// its tip standing for `tip`: the Boolean's vertex–vertex planner,
/// whose struts at a shared vertex splice among each other by their
/// germs, and whose facing the corner's geometry reads
/// (`boolean::insert::strut_faces_first`).
pub(crate) fn strut(corner: HalfEdgeKey, tip: Tip) -> Site {
    Site(RunSite::Strut { corner, tip })
}

/// **The site of the null edge moving the arc `run` of `entries`** at
/// `at`: a fan or the strut whose tip stands for the rest
/// ([`Body::run_site`]) where the arc holds a real half-edge, else the
/// strut whose tip stands for the arc, inside the arc's sector
/// ([`Body::strut_before_next_real`]), `what` naming the arc.
///
/// # Panics
///
/// Where the arc's real half-edges have no orbit successor, or the
/// duplicates' corner moved (both kernel bugs, stated at the callees).
#[track_caller]
pub(crate) fn arc_site<T: Real, E: Member>(
    body: &Body<T>,
    at: VertexKey,
    entries: &[E],
    run: Run,
    what: impl Fn() -> String,
) -> Site {
    let n = entries.len();
    let arc = (0..run.len).map(|j| &entries[(run.start + j) % n]);
    let mut real = arc.filter(|e| e.real());
    let first = real.next();
    match (first, real.next_back().or(first)) {
        (Some(first), Some(last)) => body.run_site(first.he(), last.he()).unwrap_or_else(|| {
            unreachable!(
                "{} at {at:?} has no fan end: its keys are the sector walk's, and {WALKS_CLOSE}",
                what()
            )
        }),
        _ => body.strut_before_next_real(
            (at, entries[run.start].he()),
            (run.start + run.len..run.start + run.len + n).map(|j| {
                let e = &entries[j % n];
                (e.he(), e.real())
            }),
            what,
        ),
    }
}

impl<T: Real> Body<T> {
    /// Where a null edge moving the orbit run `first ..= last` (walked
    /// clockwise, [`Body::orbit_step`]) to a new vertex lands: the fan
    /// ending at the orbit successor of `last`, read in the body as it
    /// stands, or, when that successor is `first` again, the strut
    /// before `first` whose tip stands for the rest; `None` on a stale
    /// key or a broken mate.
    pub(crate) fn run_site(&self, first: HalfEdgeKey, last: HalfEdgeKey) -> Option<Site> {
        let he2 = self.orbit_step(last)?;
        Some(Site(if he2 == first {
            RunSite::Strut {
                corner: first,
                tip: Tip::Rest,
            }
        } else {
            RunSite::Fan { he1: first, he2 }
        }))
    }

    /// **Where a run of duplicate entries alone hangs its strut**, its
    /// tip standing for the run: the corner after `corner`, a half-edge
    /// out of `at`, before its orbit successor, the first real entry of
    /// `past` (the entries past the run in orbit order, each its
    /// half-edge and whether it is a real edge), `what` naming the run.
    ///
    /// # Panics
    ///
    /// Where that entry is not `corner`'s orbit successor at `at`: an
    /// earlier mint moved the corner, which no verb's runs do.
    #[track_caller]
    pub(crate) fn strut_before_next_real(
        &self,
        (at, corner): (VertexKey, HalfEdgeKey),
        past: impl IntoIterator<Item = (HalfEdgeKey, bool)>,
        what: impl FnOnce() -> String,
    ) -> Site {
        let next_real = past
            .into_iter()
            .find_map(|(he, real)| real.then_some(he))
            .unwrap_or_else(|| unreachable!("an orbit has a real half-edge"));
        if self.proven_orbit_step(corner) != next_real
            || proven(&self.half_edges, next_real, EntityId::HalfEdge).start != at
        {
            unreachable!(
                "{}'s corner before {next_real:?} left {at:?} or gained a half: no other run's \
                 mint touches it",
                what()
            );
        }
        Site(RunSite::Strut {
            corner: next_real,
            tip: Tip::Arc,
        })
    }
}

impl<T: Decide> Body<T> {
    /// **One run's null edge**, minted at `site` on the vertex `at`,
    /// the run lying on `run_side`, with the sense its facing gives
    /// (module docs, "The mint"). Both read the run in the minting
    /// body's own orbit (the sense theorem's forward wedge, about that
    /// body's outward normal): the start germ is the one the run
    /// follows there, and a run ordered about the other body's opposed
    /// normal starts at its end. The mint side, the attribute and the
    /// halves come from these inputs alone, so the body's scaffold
    /// attribute and the caller's record are one datum.
    ///
    /// # Errors
    ///
    /// As [`Body::mev_null`](crate::construction::Construct::mev_null).
    pub(crate) fn mev_null_run(
        &mut self,
        site: Site,
        at: VertexKey,
        run_side: NewVertexSide,
    ) -> Result<NullRunMint, EulerOpError> {
        let mev = match site.0 {
            RunSite::Fan { he1, he2 } => MevSite::Fan { he1, he2 },
            RunSite::Strut { corner, .. } => MevSite::Fan {
                he1: corner,
                he2: corner,
            },
            RunSite::Lone { r#loop, .. } => MevSite::Lone { r#loop },
        };
        let arc = site.tip() == Tip::Arc;
        let side = match (run_side, arc) {
            (side, true) => side,
            (NewVertexSide::Above, false) => NewVertexSide::Below,
            (NewVertexSide::Below, false) => NewVertexSide::Above,
        };
        let created = self.mev_null(mev, side)?;
        let attr = match side {
            NewVertexSide::Below => NullEdge {
                below_end: created.vertex,
                above_end: at,
            },
            NewVertexSide::Above => NullEdge {
                below_end: at,
                above_end: created.vertex,
            },
        };
        let (halves, run_end) = if arc {
            ([created.he_plus, created.he_minus], created.vertex)
        } else {
            ([created.he_minus, created.he_plus], at)
        };
        Ok(NullRunMint {
            created,
            attr,
            halves,
            run_end,
        })
    }
}

/// What [`Body::mev_null_run`] minted.
#[derive(Clone, Copy, Debug)]
pub(crate) struct NullRunMint {
    pub(crate) created: MevCreated,
    /// The attribute the mint recorded.
    pub(crate) attr: NullEdge,
    /// `[start half, end half]`: the halves facing the run's start and
    /// end germs.
    pub(crate) halves: [HalfEdgeKey; 2],
    /// The end on the run's side: the copy, or the old vertex where a
    /// strut's tip stands for the rest.
    pub(crate) run_end: VertexKey,
}

/// **Walks `tree` in mint order**, handing each strut the node its
/// parent's mint returned (`None`: the strut hangs at the root), and
/// keeping the node its own mint returns for its children.
pub(crate) fn hang<N: Copy, E>(
    tree: &[RingStrut],
    mut mint: impl FnMut(&RingStrut, Option<N>) -> Result<N, E>,
) -> Result<(), E> {
    let mut nodes: Vec<Option<N>> = vec![None; tree.len()];
    for strut in tree {
        let parent = strut.parent.map(|p| {
            nodes[p].unwrap_or_else(|| {
                unreachable!("ring_tree mints each strut after the one it hangs at")
            })
        });
        nodes[strut.run] = Some(mint(strut, parent)?);
    }
    Ok(())
}

/// **A ring's struts, hung at the bare vertex `root`** that the empty
/// loop `ring_loop` holds, as `tree` says ([`ring_tree`]): every strut
/// on `run_side`, its node its tip. A strut hangs at its node just
/// before the half leaving it towards the root, after the node's
/// earlier struts; at the root, the first is the lone strut and each
/// later one hangs before that first strut's leaving half. `minted`
/// sees each in mint order, with the vertex it hangs at.
///
/// # Errors
///
/// The mint's, and `minted`'s.
pub(crate) fn hang_ring<T: Decide, E: From<EulerOpError>>(
    body: &mut Body<T>,
    (root, ring_loop): (VertexKey, LoopKey),
    tree: &[RingStrut],
    run_side: NewVertexSide,
    mut minted: impl FnMut(&RingStrut, VertexKey, NullRunMint) -> Result<(), E>,
) -> Result<(), E> {
    let mut root_anchor: Option<HalfEdgeKey> = None;
    hang(tree, |strut, parent: Option<(VertexKey, HalfEdgeKey)>| {
        let tip = Tip::facing(strut.plus_faces_start);
        let (at, site) = match (parent, root_anchor) {
            (Some((node, back)), _) => (node, RunSite::Strut { corner: back, tip }),
            (None, Some(corner)) => (root, RunSite::Strut { corner, tip }),
            (None, None) => (
                root,
                RunSite::Lone {
                    r#loop: ring_loop,
                    tip,
                },
            ),
        };
        let mint = body.mev_null_run(Site(site), at, run_side)?;
        if parent.is_none() {
            root_anchor.get_or_insert(mint.created.he_plus);
        }
        let node = (mint.created.vertex, mint.created.he_minus);
        minted(strut, at, mint)?;
        Ok(node)
    })
}

/// One strut of a pierce ring ([`ring_tree`]): its run, the run whose
/// strut's far end it hangs at (`None`: the ring vertex), and whether
/// the half leaving that node faces the run's start germ.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct RingStrut {
    pub run: usize,
    pub parent: Option<usize>,
    pub plus_faces_start: bool,
}

/// **A pierce ring's struts, as the tree of its corners**, from its
/// runs' germs in clockwise `order` about the pierced face's outward
/// normal (germ `2i` is run `i`'s start, `2i + 1` its end), in mint
/// order: each strut after the one it hangs at, and a node's struts
/// clockwise. A read of `order` alone, so it adds no predicate.
///
/// The runs are non-crossing chords of the vertex's link above the
/// face. The loop of struts at the point passes their germs in clockwise
/// order, so it walks round the plane tree with one node per region
/// above the face and one strut per chord (forced up to its root). A
/// stack walk reads it: a germ opens its run's chord at the far end of
/// the open chord it lies under, or at the root, the half leaving that
/// node facing it; the run's other germ closes it. Facings alternate
/// strictly by depth: a strut faces its start exactly when the root's
/// struts do and its depth is even.
///
/// The root is `root` where given (a region, numbered as a walk from
/// the corner before `order[0]` first enters them), otherwise the
/// region bordering the most chords, the lowest numbered of those
/// tied: where one borders every chord, the ring is a star. The walk
/// starts at the root's corner whose next germ is the lowest run's.
/// `None` where `order` pairs crossing chords, or `root` names no
/// region.
///
/// The split reads the same tree with the two circles swapped
/// (`splitting::insert`): its `order` is an ON vertex's cuts in orbit
/// order, its chords the corners of the section there, and its regions
/// the pieces of material round the vertex, rooted at the one the
/// vertex keeps.
pub(crate) fn ring_tree(order: &[usize], root: Option<usize>) -> Option<Vec<RingStrut>> {
    let m = order.len();
    let k = m / 2;
    // The region of each corner `j` (after `order[j]`), and how many
    // chords border each region.
    let (mut region_of, mut degree) = (vec![0; m], vec![0usize; k + 1]);
    let mut open: Vec<(usize, usize)> = Vec::new();
    let (mut here, mut regions) = (0, 1);
    for (j, &g) in order.iter().enumerate() {
        let run = g / 2;
        if open.last().is_some_and(|&(r, _)| r == run) {
            here = open.pop()?.1;
        } else if open.iter().any(|&(r, _)| r == run) {
            return None;
        } else {
            open.push((run, here));
            degree[here] += 1;
            here = regions;
            *degree.get_mut(here)? += 1;
            regions += 1;
        }
        region_of[j] = here;
    }
    if !open.is_empty() || regions != k + 1 {
        return None;
    }
    let root = match root {
        Some(r) => r,
        None => (0..=k).max_by_key(|&r| (degree[r], std::cmp::Reverse(r)))?,
    };
    let cut = (0..m)
        .filter(|&j| region_of[j] == root)
        .min_by_key(|&j| order[(j + 1) % m] / 2)?;
    let mut struts = Vec::with_capacity(k);
    let mut open: Vec<usize> = Vec::new();
    for t in 1..=m {
        let g = order[(cut + t) % m];
        let run = g / 2;
        if open.last() == Some(&run) {
            open.pop();
        } else {
            struts.push(RingStrut {
                run,
                parent: open.last().copied(),
                plus_faces_start: g.is_multiple_of(2),
            });
            open.push(run);
        }
    }
    Some(struts)
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static RING_ROOT: core::cell::Cell<Option<usize>> = const { core::cell::Cell::new(None) };
}

/// The ring root a test pinned on this thread ([`with_ring_root`]);
/// `None`, [`ring_tree`]'s own choice, outside the test arms.
pub(crate) fn ring_root() -> Option<usize> {
    #[cfg(any(test, feature = "test-support"))]
    {
        RING_ROOT.with(core::cell::Cell::get)
    }
    #[cfg(not(any(test, feature = "test-support")))]
    {
        None
    }
}

/// Runs `f` with every pierce ring on this thread rooted at region
/// `root` ([`ring_tree`]), for the rows that build a ring from each of
/// its roots. It applies to every pierce of two or more runs on the
/// thread while `f` runs, whichever boolean step reaches it.
#[cfg(any(test, feature = "test-support"))]
pub fn with_ring_root<R>(root: usize, f: impl FnOnce() -> R) -> R {
    let before = RING_ROOT.with(|c| c.replace(Some(root)));
    let out = f();
    RING_ROOT.with(|c| c.set(before));
    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// A run's fan end is its last member's orbit successor, and a run
    /// holding the whole orbit is the strut in the corner before its
    /// first member whose tip stands for the rest, from whichever member
    /// the run starts.
    #[test]
    fn run_site_reads_the_fan_end_and_names_the_whole_orbit() {
        let cube = crate::test_support_fixtures::geometric_cube::<f64>(geom_core::Tol::witness());
        let body = cube.body;
        let (_, v) = body.vertices().next().unwrap();
        let orbit = body.vertex_orbit(v.emanating.unwrap()).unwrap();
        let [h0, h1, h2] = orbit[..] else {
            panic!("a cube corner has valence 3: {orbit:?}");
        };
        let fan = |he1, he2| Some(Site(RunSite::Fan { he1, he2 }));
        assert_eq!(
            body.run_site(h0, h1),
            fan(h0, h2),
            "a two-edge run ends at the third"
        );
        assert_eq!(
            body.run_site(h2, h2),
            fan(h2, h0),
            "a one-edge run ends at its successor"
        );
        for (first, last) in [(h0, h2), (h1, h0), (h2, h1)] {
            assert_eq!(
                body.run_site(first, last),
                Some(strut(first, Tip::Rest)),
                "the run {first:?} ..= {last:?} holds the whole orbit: the strut before {first:?}"
            );
        }
        assert_eq!(body.run_site(h0, HalfEdgeKey::default()), None);
    }

    /// The runs of moved entries, anchored at the first entry not moved:
    /// none where one side is empty; one per maximal moved stretch,
    /// including one that wraps the array's seam, and several apart.
    #[test]
    fn runs_are_the_maximal_moved_stretches() {
        struct E(bool);
        impl Member for E {
            fn he(&self) -> HalfEdgeKey {
                HalfEdgeKey::default()
            }
            fn real(&self) -> bool {
                true
            }
            fn moved(&self) -> bool {
                self.0
            }
        }
        let read = |m: &[bool]| runs(&m.iter().map(|&b| E(b)).collect::<Vec<_>>());
        let run = |start, len| Run { start, len };
        assert_eq!(read(&[true, true, true]), vec![]);
        assert_eq!(read(&[false, false]), vec![]);
        assert_eq!(read(&[true, false, false, true]), vec![run(3, 2)]);
        assert_eq!(
            read(&[true, false, true, true, false, false]),
            vec![run(2, 2), run(0, 1)]
        );
        assert_eq!(read(&[true, false, true, false, true, false]).len(), 3);
    }

    fn ring_strut(run: usize, parent: Option<usize>, plus_faces_start: bool) -> RingStrut {
        RingStrut {
            run,
            parent,
            plus_faces_start,
        }
    }

    /// **A pierce ring's tree from its germs' clockwise order**, one
    /// row per shape, each in mint order:
    /// - three runs apart, each start germ first: a star of struts each
    ///   facing its start;
    /// - three runs nested under one (`meeting::comb`'s order): a star
    ///   in an order other than the runs', each facing its end;
    /// - a run between two others (`meeting::arch`'s order): a path,
    ///   run 2 hung off run 1's far end and facing the other way;
    /// - five runs, one over two each over one more (`meeting::branching_cone`'s
    ///   order): three struts at the ring vertex facing alike, and one
    ///   hung off each of two of them, facing the other way;
    /// - two runs whose chords cross: none.
    #[test]
    fn a_ring_tree_is_read_off_its_germs_order() {
        type Row<'a> = (&'a str, &'a [usize], Option<Vec<RingStrut>>);
        let rows: [Row; 5] = [
            (
                "apart",
                &[0, 1, 2, 3, 4, 5],
                Some(vec![
                    ring_strut(0, None, true),
                    ring_strut(1, None, true),
                    ring_strut(2, None, true),
                ]),
            ),
            (
                "nested under one",
                &[0, 5, 4, 3, 2, 1],
                Some(vec![
                    ring_strut(0, None, false),
                    ring_strut(2, None, false),
                    ring_strut(1, None, false),
                ]),
            ),
            (
                "one between others",
                &[0, 3, 4, 5, 2, 1],
                Some(vec![
                    ring_strut(0, None, false),
                    ring_strut(1, None, false),
                    ring_strut(2, Some(1), true),
                ]),
            ),
            (
                "branching",
                &[0, 1, 4, 3, 2, 5, 8, 7, 6, 9],
                Some(vec![
                    ring_strut(0, None, true),
                    ring_strut(2, None, true),
                    ring_strut(1, Some(2), false),
                    ring_strut(4, None, true),
                    ring_strut(3, Some(4), false),
                ]),
            ),
            ("crossing", &[0, 2, 1, 3], None),
        ];
        for (what, order, want) in rows {
            assert_eq!(ring_tree(order, None), want, "{what}");
        }
    }

    /// **Every root of a ring tree holds every run once, each after the
    /// strut it hangs at, each half facing the germ its node meets
    /// first**: from each region of the arch's and the branching cone's
    /// orders, the struts are the runs, a parent precedes its child, and
    /// a region outside the tree is none.
    #[test]
    fn every_root_of_a_ring_tree_mints_each_run_after_its_parent() {
        for order in [
            &[0usize, 3, 4, 5, 2, 1][..],
            &[0, 1, 4, 3, 2, 5, 8, 7, 6, 9],
        ] {
            let k = order.len() / 2;
            for root in 0..=k {
                let tree = ring_tree(order, Some(root)).unwrap();
                let mut runs: Vec<usize> = tree.iter().map(|s| s.run).collect();
                runs.sort_unstable();
                assert_eq!(runs, (0..k).collect::<Vec<_>>(), "{order:?} at {root}");
                for (i, s) in tree.iter().enumerate() {
                    if let Some(p) = s.parent {
                        assert!(
                            tree[..i].iter().any(|t| t.run == p),
                            "{order:?} at {root}: run {} before its parent {p}",
                            s.run
                        );
                    }
                }
            }
            assert_eq!(
                ring_tree(order, Some(k + 1)),
                None,
                "{order:?}: no region k + 1"
            );
        }
    }

    /// The ring tree of `order` built from its chord diagram's regions
    /// rather than by a walk: gaps between circle positions in one
    /// region when no chord separates them; the root the region the
    /// most chords border, lowest by its first gap from the one before
    /// `order[0]`; each chord's parent the chord whose far region is its
    /// near one; its facing the germ the clockwise walk round its near
    /// region meets first. Sorted by run.
    fn dual_tree(order: &[usize]) -> Vec<RingStrut> {
        let m = order.len();
        let k = m / 2;
        let mut pos = vec![0; m];
        for (p, &g) in order.iter().enumerate() {
            pos[g] = p;
        }
        let chord = |r: usize| {
            (
                pos[2 * r].min(pos[2 * r + 1]),
                pos[2 * r].max(pos[2 * r + 1]),
            )
        };
        // Gap `j` lies after position `j`; inside chord (a, b) when a <= j < b.
        let inside = |j: usize, r: usize| {
            let (a, b) = chord(r);
            a <= j && j < b
        };
        let gaps: Vec<usize> = std::iter::once(m - 1).chain(0..m - 1).collect();
        let mut region = vec![usize::MAX; m];
        let mut regions = 0;
        for &j in &gaps {
            if region[j] != usize::MAX {
                continue;
            }
            for &l in &gaps {
                if (0..k).all(|r| inside(j, r) == inside(l, r)) {
                    region[l] = regions;
                }
            }
            regions += 1;
        }
        let sides = |r: usize| {
            let (a, b) = chord(r);
            (region[(a + m - 1) % m], region[a], region[b])
        };
        let mut degree = vec![0; regions];
        for r in 0..k {
            let (out, inn, _) = sides(r);
            degree[out] += 1;
            degree[inn] += 1;
        }
        let root = (0..regions)
            .max_by_key(|&g| (degree[g], std::cmp::Reverse(g)))
            .unwrap();
        // Each chord's near (root-side) and far region: the far region of
        // a chord is reached through it alone, so from the root the near
        // region is the one nearer the root; read it by the regions'
        // depth from the root over the chords.
        let mut depth = vec![usize::MAX; regions];
        depth[root] = 0;
        for d in 0..regions {
            for r in 0..k {
                let (out, inn, _) = sides(r);
                for (x, y) in [(out, inn), (inn, out)] {
                    if depth[x] == d && depth[y] == usize::MAX {
                        depth[y] = d + 1;
                    }
                }
            }
        }
        let near_far = |r: usize| {
            let (out, inn, _) = sides(r);
            if depth[out] < depth[inn] {
                (out, inn)
            } else {
                (inn, out)
            }
        };
        let mut tree: Vec<RingStrut> = (0..k)
            .map(|r| {
                let (near, _) = near_far(r);
                let parent = (0..k).find(|&q| near_far(q).1 == near);
                let (a, b) = chord(r);
                // The walk round the near region reaches the endpoint
                // whose gap before it is the near region's.
                let first = if region[(a + m - 1) % m] == near {
                    a
                } else {
                    b
                };
                RingStrut {
                    run: r,
                    parent,
                    plus_faces_start: order[first].is_multiple_of(2),
                }
            })
            .collect();
        tree.sort_by_key(|s| s.run);
        tree
    }

    /// Every non-crossing perfect matching of positions `0..m`, as each
    /// position's mate.
    fn matchings(m: usize) -> Vec<Vec<usize>> {
        fn pairs(lo: usize, hi: usize) -> Vec<Vec<(usize, usize)>> {
            if lo >= hi {
                return vec![Vec::new()];
            }
            let mut out = Vec::new();
            for j in (lo + 1..hi).step_by(2) {
                for inner in pairs(lo + 1, j) {
                    for outer in pairs(j + 1, hi) {
                        let mut v = vec![(lo, j)];
                        v.extend(&inner);
                        v.extend(outer);
                        out.push(v);
                    }
                }
            }
            out
        }
        pairs(0, m)
            .into_iter()
            .map(|ps| {
                let mut mate = vec![0; m];
                for (a, b) in ps {
                    mate[a] = b;
                    mate[b] = a;
                }
                mate
            })
            .collect()
    }

    /// The germ order of every closed meander of `k` runs: an above and a
    /// below non-crossing matching whose union is one cycle, walked from
    /// position 0 above first, germ `g` at the position the walk reaches
    /// `g`-th.
    fn meanders(k: usize) -> Vec<Vec<usize>> {
        let m = 2 * k;
        let ms = matchings(m);
        let mut out = Vec::new();
        for above in &ms {
            for below in &ms {
                let mut order = vec![usize::MAX; m];
                let (mut p, mut g) = (0, 0);
                while order[p] == usize::MAX {
                    order[p] = g;
                    g += 1;
                    p = if g % 2 == 1 { above[p] } else { below[p] };
                }
                if g == m {
                    out.push(order);
                }
            }
        }
        out
    }

    /// **Every closed meander's ring tree is its chord diagram's dual
    /// tree, from every root** (k ≤ 7, the meander counts OEIS A005315):
    /// from the default root it is [`dual_tree`]'s, built from the
    /// regions rather than walked, and a star at exactly two meanders per
    /// k; from every region as root, the walk round the tree (each strut
    /// opened at the germ its half leaving its node faces, closed at the
    /// other) is a rotation of the germ order, the struts are minted in
    /// that walk's order, their facings alternate by depth, each root
    /// gives a different tree, and a region past the last gives none.
    #[test]
    fn every_meanders_ring_tree_is_its_dual_tree() {
        fn tour(t: &[RingStrut], parent: Option<usize>, out: &mut Vec<usize>) {
            for s in t.iter().filter(|s| s.parent == parent) {
                let open = 2 * s.run + usize::from(!s.plus_faces_start);
                out.push(open);
                tour(t, Some(s.run), out);
                out.push(open ^ 1);
            }
        }
        let counts = [0usize, 1, 2, 8, 42, 262, 1828, 13820];
        for (k, &count) in counts.iter().enumerate().skip(2) {
            let m = 2 * k;
            let orders = meanders(k);
            assert_eq!(orders.len(), count, "k={k}: meanders");
            let mut stars = 0;
            for order in orders {
                let tree = ring_tree(&order, None).unwrap_or_else(|| panic!("{order:?}: no tree"));
                let mut sorted = tree.clone();
                sorted.sort_by_key(|s| s.run);
                assert_eq!(sorted, dual_tree(&order), "k={k} {order:?}");
                stars += usize::from(tree.iter().all(|s| s.parent.is_none()));
                let mut shapes = Vec::new();
                for root in 0..=k {
                    let t = ring_tree(&order, Some(root))
                        .unwrap_or_else(|| panic!("{order:?}: no tree at {root}"));
                    let depth = |run: usize| {
                        let (mut d, mut r) = (0, run);
                        while let Some(p) = t.iter().find(|s| s.run == r).and_then(|s| s.parent) {
                            d += 1;
                            r = p;
                        }
                        d
                    };
                    for s in &t {
                        assert_eq!(
                            s.plus_faces_start,
                            t[0].plus_faces_start ^ (depth(s.run) % 2 == 1),
                            "k={k} {order:?} at {root}: facing by depth"
                        );
                    }
                    let mut walk = Vec::new();
                    tour(&t, None, &mut walk);
                    assert!(
                        (0..m).any(|r| (0..m).all(|i| walk[i] == order[(r + i) % m])),
                        "k={k} {order:?} at {root}: walk {walk:?}"
                    );
                    let opened: Vec<usize> = walk
                        .iter()
                        .enumerate()
                        .filter(|&(i, g)| !walk[..i].iter().any(|&h| h / 2 == g / 2))
                        .map(|(_, g)| g / 2)
                        .collect();
                    assert_eq!(
                        opened,
                        t.iter().map(|s| s.run).collect::<Vec<_>>(),
                        "k={k} {order:?} at {root}: mint order"
                    );
                    let mut shape: Vec<_> = t.iter().map(|s| (s.run, s.parent)).collect();
                    shape.sort_unstable();
                    assert!(
                        !shapes.contains(&shape),
                        "k={k} {order:?}: two roots, one tree"
                    );
                    shapes.push(shape);
                }
                assert_eq!(
                    ring_tree(&order, Some(k + 1)),
                    None,
                    "k={k} {order:?}: past the last"
                );
            }
            assert_eq!(stars, 2, "k={k}: stars");
        }
    }
}
