//! **Certified separation** (GROUP-BOOLEAN-DESIGN, ratified A′): prove
//! that two solids cannot touch — or refuse honestly.
//!
//! Two doors, one box rule. [`Separation`] takes ONE prototype body and
//! N rigid placements of it and proves that no two placed copies can
//! touch, BEFORE a graft, in the prototype's frame. [`SolidSeparation`]
//! takes one body and proves that two of its solids cannot touch,
//! AFTER one, in the body's own frame. Both discharge the same
//! obligation for a different caller, and the second door's own header
//! says why it exists.
//!
//! The recipe layer's group boolean (`PlacedUnion`) fuses N copies of one
//! prototype into ONE body through the disjoint-graft door
//! ([`crate::graft_disjoint_all_keyed`]). That door asserts nothing about
//! its operands (its module docs spell out the gap: overlapping solids
//! pass the at-rest validator undetected, #382), so the caller must
//! establish disjointness. This module is that establishment, and it is
//! stronger than the graft's DECLARED-disjoint boundary: nothing is taken
//! on the author's word.
//!
//! # What is proved, and what is not
//!
//! The certificate is **sufficient, not necessary**. Every face of the
//! prototype gets a conservative axis-aligned box (the boolean sweep's own
//! [`crate::boolean::boxes::face_box`], padded); a pair of placements is
//! CERTIFIED SEPARATED when no box of one placement's copy can meet any
//! box of the other's. Boxes are supersets of what they enclose and an
//! affine image of a superset is a superset of the affine image, so a
//! box-level separation is a genuine separation of the solids.
//!
//! **That premise is the box module's contract, and it holds on every
//! scalar**: the per-kind extents range over a bracketed description
//! rather than sampling one endpoint of it. This door is the one where
//! a box-level non-overlap is a GRANT rather than a prune, so it is
//! the door that would pay for any gap, and the sentence above may be
//! read as unconditional.
//!
//! **The other direction costs this door too, and costs it more
//! often.** A box that is too BIG cannot make a wrong certificate —
//! it can only withhold one — but withholding is this door's whole
//! output. What remains is the looseness the RULES themselves state —
//! a whole ball for a sphere band, a full turn for an arc — not slack
//! in the code: each arm claims exactly its construction, which the
//! `boxes` module's ceiling rows pin.
//!
//! **This door needs no surface-kind gate of its own.** It shares the one
//! [`crate::boolean::boxes::FaceBoxRule`]; what differs is only what a
//! POISON box MEANS at each door, and that difference is what lets the
//! rule be shared. In the sweep, poison prunes nothing, so an unboxable
//! face reaches the exact predicates. Here, poison overlaps everything, so
//! an unboxable face makes every pair carrying it refuse typed and the
//! prototype is never certified. Both readings are the same box's
//! fail-loud direction, so this module does not care WHICH kinds are
//! boxable — only that an unboxable one is never silently claimed, which
//! the rule guarantees.
//!
//! # This door GRANTS, and it admits a `Dual` (D1, 2026-08-19)
//!
//! `Separation::of`, [`Separation::certify`] and `image` are
//! `T: Decide + Bounds` with **no** `geom_core::CertifiedEnclosure`, so
//! since the D1 ruling they are instantiable at `Dual64` and at
//! `Dual<Interval>`. Worth saying plainly because the direction is easy
//! to get backwards: box NON-overlap here is a **grant**, not a refusal —
//! `Ok(())` IS the certificate, and `crate::graft_disjoint_all_keyed`
//! re-checks nothing (#382), so this module is the whole establishment of
//! disjointness.
//!
//! What makes the dual instantiation sound is **delegation**, not the
//! refusal direction: every endpoint a `Dual<T>` box carries is its value
//! channel's, and the value channel of a `Dual<T>` build IS the plain-`T`
//! build bit-identically (D9). So a dual run's certificate is exactly the
//! base scalar's — the `f64` run's at `Dual<f64>`, the `Interval` run's
//! at `Dual<Interval>` — and there is no reading under which a dual gets
//! a certificate its base scalar would not. Whether this door should
//! nonetheless take `CertifiedEnclosure` is a **#643-completeness**
//! question (it grants, and the grant is believed), left open rather than
//! answered in passing; see `geom-core/src/real.rs`'s `Bounds` scope rule.
//!
//! The converse does not hold: two genuinely disjoint copies whose boxes
//! interpenetrate are REFUSED, not accepted. That is the fail-loud reading
//! of the design's "identical objects make non-overlap easier" — a refusal
//! that names the pair, refinable later by a sharper predicate, never a
//! silent maybe.
//!
//! # Why the prototype's tree, built once
//!
//! Every placement is the SAME body under a different rigid map, so there
//! is exactly one set of boxes to build and exactly one tree over them.
//! A pair `(i, j)` is tested in the prototype's OWN frame by mapping
//! placement `j`'s boxes back through `M_i⁻¹ ∘ M_j` and querying that one
//! tree — build once, `O(F log F)` per pair that survives the hull
//! prefilter, and most pairs do not survive it.
//!
//! # Conservatism under both scalars
//!
//! Map coefficients enter as `[lo(), hi()]` brackets (the allowlisted
//! [`geom_core::Bounds`] spatial-index seam, ratified 2026-07-29 — same
//! standing as the C10 tree this module queries), so the Interval lane's
//! enclosure widens the image box rather than narrowing it. Floating-point
//! slop in the image arithmetic is absorbed by an outward relative pad.
//! Both are conservative in the SAME direction as the face pad: they can
//! only make a certificate harder to obtain, never wrongly grant one. A
//! poisoned (NaN) coordinate produces a poison box, which overlaps
//! everything, so poison refuses — it never certifies.

use bvh::{Aabb, Bvh};
use geom_core::{Affine3, Band, Bounds, Decide, Tol};

use crate::body::Body;
use crate::boolean::BooleanError;
use crate::boolean::boxes::{face_box, sweep_pad};
use crate::entity::SolidKey;

/// A pair of placements the certificate could not separate: their padded
/// boxes meet, so the union of those two copies is not provably a
/// disjoint union.
///
/// Indices are into the placement list, ascending (`i < j`), and the pair
/// reported is the FIRST in that order — deterministic (D9), so the
/// refusal a document produces does not depend on iteration luck.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlacementsMeet {
    /// The lower placement index.
    pub i: usize,
    /// The higher placement index.
    pub j: usize,
}

/// The prototype's separation certificate machinery: its padded face
/// boxes, their hull, and the one tree over them.
///
/// Built ONCE per prototype ([`Separation::of`]) and queried per placement
/// pair ([`Separation::certify`]).
#[derive(Debug, Clone)]
pub struct Separation {
    /// The padded per-face boxes, in the prototype's face-arena order —
    /// the tree's input order, so a query's item index indexes here.
    boxes: Vec<Aabb>,
    /// The hull of every box: the whole prototype's conservative box, the
    /// pair prefilter's operand.
    hull: Aabb,
    /// The tree over `boxes`.
    tree: Bvh,
}

impl Separation {
    /// Builds the prototype's boxes and tree.
    ///
    /// # Errors
    ///
    /// [`BooleanError`] — the box builder's own corruption refusals (a
    /// face whose loop is unwalkable is not a body), and
    /// `ClassificationInvariant` when the ambient band is unusable.
    pub fn of<T: Decide + Bounds>(proto: &Body<T>, tol: Tol) -> Result<Self, BooleanError> {
        let band = Band::linear(tol).map_err(|_| BooleanError::ClassificationInvariant {
            what: "placement separation: the ambient tolerance band is unusable",
        })?;
        let pad = sweep_pad(band);
        let mut boxes = Vec::new();
        for (f, _) in proto.faces() {
            boxes.push(face_box(proto, f, pad)?);
        }
        // A face-less prototype encloses nothing; the hull of nothing is
        // the poison box, which overlaps everything — so a face-less
        // prototype can never be certified, and the caller's own
        // "graft source holds no solid" refusal is what actually fires.
        let hull = boxes
            .iter()
            .copied()
            .reduce(|a, b| a.hull(&b))
            .unwrap_or_else(Aabb::poison);
        let tree = Bvh::build(&boxes);
        Ok(Self { boxes, hull, tree })
    }

    /// Certifies that no two of `maps`'s placed copies can meet.
    ///
    /// `Ok(())` is the certificate. `Err(PlacementsMeet)` names the first
    /// pair (ascending `(i, j)`) the boxes could not separate — the typed
    /// refusal, NOT a claim that the copies actually overlap (module
    /// docs: sufficient, not necessary).
    ///
    /// Fewer than two placements is vacuously certified.
    ///
    /// # Errors
    ///
    /// [`PlacementsMeet`] — see above.
    pub fn certify<T: Decide + Bounds>(&self, maps: &[Affine3<T>]) -> Result<(), PlacementsMeet> {
        let hulls: Vec<Aabb> = maps.iter().map(|m| image(&self.hull, m)).collect();
        for j in 1..maps.len() {
            for i in 0..j {
                if !hulls[i].overlaps(&hulls[j]) {
                    continue;
                }
                // The hulls meet, so descend to faces — in the
                // PROTOTYPE's frame, where the one tree lives: carry j's
                // boxes back through `M_i⁻¹ ∘ M_j` and query.
                let rel = relative(&maps[i], &maps[j]);
                // A prototype with no face boxes has nothing to descend
                // to, so the hull overlap is the last word — and it says
                // "meets" (the hull of no boxes is poison, which overlaps
                // everything). Refusing here keeps the empty case from
                // reading as a vacuous certificate.
                if self.boxes.is_empty() {
                    return Err(PlacementsMeet { i, j });
                }
                for b in &self.boxes {
                    if !self.tree.overlapping(&image(b, &rel)).is_empty() {
                        return Err(PlacementsMeet { i, j });
                    }
                }
            }
        }
        Ok(())
    }
}

/// `M_i⁻¹ ∘ M_j` — placement `j` expressed in placement `i`'s frame.
///
/// Composed through the affine door's own `inverse` and `Mul`, so the
/// arithmetic is the kernel's, not a re-derivation. A non-invertible map
/// yields non-finite coefficients, which propagate to a poison image box
/// — poison overlaps everything, so it refuses (never certifies).
fn relative<T: Decide>(mi: &Affine3<T>, mj: &Affine3<T>) -> Affine3<T> {
    mi.inverse() * *mj
}

/// The conservative axis-aligned image of `b` under `m`: a box CONTAINING
/// `m(b)`, hence containing `m(x)` for every `x` the box contained.
///
/// The eight corners are mapped with bracketed coefficients (`[lo, hi]`
/// per entry — the Interval lane widens, never narrows) and the results
/// hulled, then every side is pushed outward by a relative pad that
/// dominates the three-term dot product's rounding. Non-finite input
/// yields the poison box, which overlaps everything.
fn image<T: Decide + Bounds>(b: &Aabb, m: &Affine3<T>) -> Aabb {
    let br = |v: T| (v.lo(), v.hi());
    let cols = [m.linear.c0, m.linear.c1, m.linear.c2];
    // Row r of the linear part, bracketed: column j's r-th component.
    let row = |r: usize| {
        [0usize, 1, 2].map(|j| {
            let c = cols[j];
            br(match r {
                0 => c.x,
                1 => c.y,
                _ => c.z,
            })
        })
    };
    let trans = [
        br(m.translation.x),
        br(m.translation.y),
        br(m.translation.z),
    ];
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    let xs = [b.min_x, b.max_x];
    let ys = [b.min_y, b.max_y];
    let zs = [b.min_z, b.max_z];
    for &x in &xs {
        for &y in &ys {
            for &z in &zs {
                let p = [x, y, z];
                for r in 0..3 {
                    let a = row(r);
                    let (mut l, mut h) = trans[r];
                    let mut mag = l.abs().max(h.abs());
                    for k in 0..3 {
                        // Bracket × exact scalar: the extremes are the
                        // two endpoint products, in either order.
                        let (u, v) = (a[k].0 * p[k], a[k].1 * p[k]);
                        l += u.min(v);
                        h += u.max(v);
                        mag = mag.max(u.abs()).max(v.abs());
                    }
                    mag = mag.max(l.abs()).max(h.abs());
                    // Outward rounding: four roundings of magnitude at
                    // most `mag` each, generously bounded. `MIN_POSITIVE`
                    // keeps a zero-magnitude row from padding by nothing.
                    let slop = 8.0 * f64::EPSILON * mag + f64::MIN_POSITIVE;
                    lo[r] = lo[r].min(l - slop);
                    hi[r] = hi[r].max(h + slop);
                }
            }
        }
    }
    if !lo.iter().chain(hi.iter()).all(|v| v.is_finite()) {
        return Aabb::poison();
    }
    Aabb {
        min_x: lo[0],
        min_y: lo[1],
        min_z: lo[2],
        max_x: hi[0],
        max_y: hi[1],
        max_z: hi[2],
    }
}

// ---------------------------------------------------------------------
// The SOLID-PAIR door
// ---------------------------------------------------------------------

/// A pair of SOLIDS the certificate could not separate: their padded
/// face boxes meet, so this body is not provably a disjoint union of
/// those two solids.
///
/// Keys are the body's own, and a producer of these orders them by the
/// body's solid-arena order (D9) so a report does not depend on
/// iteration luck.
///
/// **Not a claim that the solids overlap.** Like [`PlacementsMeet`],
/// this is the failure of a SUFFICIENT test: two solids that merely
/// touch, and two that interpenetrate, both land here, and so does a
/// disjoint pair whose boxes happen to interpenetrate. What it denies
/// is the certificate, and denying the certificate is the honest
/// output — the module's opening contract, read at solid granularity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolidsMeet {
    /// The earlier solid in arena order.
    pub a: SolidKey,
    /// The later solid in arena order.
    pub b: SolidKey,
}

/// One solid's separation machinery: its padded face boxes, their hull,
/// and the tree over them.
#[derive(Debug, Clone)]
struct SolidBoxes {
    /// The padded per-face boxes, in this solid's own face order.
    boxes: Vec<Aabb>,
    /// The hull of every box: the pair prefilter's operand.
    hull: Aabb,
    /// The tree over `boxes`.
    tree: Bvh,
}

/// **Certified separation of the solids of ONE body** — the
/// multi-solid sibling of [`Separation`], for a gather that has
/// already happened.
///
/// [`Separation`] answers "can these N placements of one prototype
/// touch", before a graft, in the prototype's frame. This answers "can
/// these two solids of one body touch", after it, in the body's own
/// frame — so there is no affine image step and no `Bounds` bracketing
/// to be conservative about: the boxes are compared where they were
/// built. Everything else is the same door, and deliberately so — the
/// same [`face_box`] rule, the same pad, the same reading of a poison
/// box (overlaps everything, so it refuses rather than certifies), the
/// same sufficient-not-necessary contract.
///
/// # What it inherits from [`Separation`], said rather than implied
///
/// The bound is `T: Decide + Bounds` with no
/// [`geom_core::CertifiedEnclosure`], which is the sibling door's
/// signature verbatim and carries the sibling's ratified answer with
/// it: `geom-core/src/real.rs`'s `Bounds` scope rule settled that
/// question for `separation` as **NO — the caller decides it, not the
/// door**. Its callers have decided differently from one another, and
/// the rule expects that: `editor_core::eval::wire` keeps the loose
/// lane (it is beneath `evaluate<T>`, which a tighter bound would
/// reach by propagation), while `editor_core::checks` is TIGHTENED to
/// `Decide + CertifiedBounds` because nothing generic calls it. The
/// rule's 2026-08-29 entry records that second caller.
///
/// So it admits a `Dual` exactly as [`Separation`] does, and for the
/// same reason: a `SolidsMeet` is a pair of arena keys, not a value in
/// `T`, and a box's non-overlap at a dual is its value channel's,
/// which is the base scalar's (D9).
///
/// # Why this exists
///
/// [`crate::graft_disjoint_all_keyed`] asserts nothing about its
/// operands, so every caller owes an establishment of disjointness.
/// The recipe layer's group boolean discharges it with [`Separation`].
/// The document layer's product gather — which grafts one body per
/// product ROOT — had no establishment at all, so a document whose
/// roots denote overlapping solids gathered them into a product that
/// counted the same material twice and said nothing. This is that
/// layer's establishment, and `editor_core`'s separation check is its
/// consumer.
#[derive(Debug, Clone)]
pub struct SolidSeparation {
    /// Per-solid machinery, in the body's solid-arena order.
    solids: Vec<(SolidKey, SolidBoxes)>,
    /// Position in `solids` for each key, so a pair query is a lookup
    /// rather than a scan.
    index: slotmap::SecondaryMap<SolidKey, usize>,
}

impl SolidSeparation {
    /// Builds every solid's boxes and tree, once.
    ///
    /// # Errors
    ///
    /// [`BooleanError`] — the box builder's own corruption refusals (a
    /// face whose loop is unwalkable is not a body), a shell or face
    /// the body cannot resolve, and `ClassificationInvariant` when the
    /// ambient band is unusable.
    pub fn of<T: Decide + Bounds>(body: &Body<T>, tol: Tol) -> Result<Self, BooleanError> {
        let band = Band::linear(tol).map_err(|_| BooleanError::ClassificationInvariant {
            what: "solid separation: the ambient tolerance band is unusable",
        })?;
        let pad = sweep_pad(band);
        let mut solids = Vec::new();
        let mut index = slotmap::SecondaryMap::new();
        for (key, solid) in body.solids() {
            let mut boxes = Vec::new();
            for &shell_key in &solid.shells {
                let shell =
                    body.get_shell(shell_key)
                        .ok_or(BooleanError::ClassificationInvariant {
                            what: "solid separation: a solid names a shell the body lost",
                        })?;
                for &face in &shell.faces {
                    boxes.push(face_box(body, face, pad)?);
                }
            }
            // A face-less solid encloses nothing, and the hull of
            // nothing is poison — which overlaps everything, so such a
            // solid is never certified against anything. That is the
            // fail-loud direction: a solid the box rule cannot describe
            // must not read as separated from its neighbours.
            let hull = boxes
                .iter()
                .copied()
                .reduce(|a, b| a.hull(&b))
                .unwrap_or_else(Aabb::poison);
            let tree = Bvh::build(&boxes);
            index.insert(key, solids.len());
            solids.push((key, SolidBoxes { boxes, hull, tree }));
        }
        Ok(Self { solids, index })
    }

    /// The solids this was built over, in arena order — the order a
    /// caller should enumerate pairs in to keep its report
    /// deterministic (D9).
    pub fn keys(&self) -> impl Iterator<Item = SolidKey> + '_ {
        self.solids.iter().map(|(k, _)| *k)
    }

    /// Certifies that solids `a` and `b` cannot touch.
    ///
    /// `Ok(())` is the certificate. `Err(SolidsMeet)` is the typed
    /// denial — see [`SolidsMeet`] for what it does and does not claim.
    /// The returned pair is ordered by arena position, so it does not
    /// depend on the order the arguments were passed in.
    ///
    /// A solid compared against itself denies — a solid is not disjoint
    /// from itself, and answering `Ok` there would let a caller's
    /// self-pair silently read as a certificate.
    ///
    /// **PRECONDITION: both keys are `body`'s.** This is the caller's
    /// obligation and the type system does not carry it — the graft
    /// door's arity rule, one dimension over. A key the index cannot
    /// resolve denies, but do not read that as detection: the index is
    /// a `SecondaryMap`, which resolves any key whose slot and version
    /// are live, and a key from a DIFFERENT body of the same shape has
    /// live slots here. Such a key silently addresses whichever solid
    /// occupies that slot, and the answer is about that solid, not
    /// about the caller's. The door cannot tell the difference and
    /// does not claim to; only a key past the end of this body's arena
    /// reliably lands in the denying arm.
    ///
    /// # Errors
    ///
    /// [`SolidsMeet`] — see above.
    pub fn certify(&self, a: SolidKey, b: SolidKey) -> Result<(), SolidsMeet> {
        let (Some(&ia), Some(&ib)) = (self.index.get(a), self.index.get(b)) else {
            return Err(SolidsMeet { a, b });
        };
        let (lo, hi) = if ia <= ib { (ia, ib) } else { (ib, ia) };
        let pair = SolidsMeet {
            a: self.solids[lo].0,
            b: self.solids[hi].0,
        };
        if lo == hi {
            return Err(pair);
        }
        let (x, y) = (&self.solids[lo].1, &self.solids[hi].1);
        if !x.hull.overlaps(&y.hull) {
            return Ok(());
        }
        // The hulls meet, so descend to faces. An empty box list makes
        // the hull poison, which the prefilter above could not have
        // cleared — so reaching here with no boxes is the poison case
        // and it denies, rather than falling through the loop as a
        // vacuous certificate.
        if x.boxes.is_empty() || y.boxes.is_empty() {
            return Err(pair);
        }
        for b in &x.boxes {
            if !y.tree.overlapping(b).is_empty() {
                return Err(pair);
            }
        }
        Ok(())
    }
}

/// Which solid each face and each vertex of a body belongs to.
///
/// The multi-solid companion to [`SolidSeparation`]: that door decides
/// whether two solids can touch, this one says which solid an entity
/// is part of, so a caller holding entity-keyed records (declared
/// contacts, a picked face) can ask its question at solid
/// granularity.
///
/// Built from the STORED back-pointers — a half-edge names its loop, a
/// loop its face, a face its shell, a shell its solid — every one of
/// which tier 1 validates. So this is a read of structure and never a
/// geometric decision: no tolerance enters, and a body that passes
/// tier 1 has a total map.
#[derive(Debug, Clone, Default)]
pub struct SolidOwners {
    faces: slotmap::SecondaryMap<crate::entity::FaceKey, SolidKey>,
    vertices: slotmap::SecondaryMap<crate::entity::VertexKey, SolidKey>,
}

impl SolidOwners {
    /// Builds the map in one pass over the shells and one over the
    /// half-edge arena.
    ///
    /// An entity whose back-pointer chain does not resolve is simply
    /// absent, never guessed at: the map is a lookup, and a caller that
    /// gets `None` learns that this body does not place the entity,
    /// which is a fact it can act on.
    pub fn of<T: geom_core::Real>(body: &Body<T>) -> Self {
        let mut faces = slotmap::SecondaryMap::new();
        let mut vertices = slotmap::SecondaryMap::new();
        for (solid_key, solid) in body.solids() {
            for &shell_key in &solid.shells {
                let Some(shell) = body.get_shell(shell_key) else {
                    continue;
                };
                for &face in &shell.faces {
                    faces.insert(face, solid_key);
                }
            }
        }
        for (_, he) in body.half_edges() {
            let Some(r#loop) = body.get_loop(he.parent_loop) else {
                continue;
            };
            let Some(&solid) = faces.get(r#loop.face) else {
                continue;
            };
            vertices.insert(he.start, solid);
        }
        Self { faces, vertices }
    }

    /// The solid this face belongs to.
    pub fn face(&self, face: crate::entity::FaceKey) -> Option<SolidKey> {
        self.faces.get(face).copied()
    }

    /// The solid this vertex belongs to.
    pub fn vertex(&self, vertex: crate::entity::VertexKey) -> Option<SolidKey> {
        self.vertices.get(vertex).copied()
    }
}
