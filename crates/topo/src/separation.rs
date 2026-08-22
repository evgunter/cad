//! **Certified separation of repeated placements** (GROUP-BOOLEAN-DESIGN,
//! ratified A′): given ONE prototype body and N rigid placements of it,
//! prove that no two placed copies can touch — or refuse honestly.
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
//! **That premise is the box module's contract, and it is not fully
//! held today — issue #862.** Under an `Interval` scalar the slab and
//! conic arms read single bracket endpoints, so there are loci
//! `face_box` does not enclose; the `f64` lane is unaffected. This
//! door is the one where a box-level non-overlap is a GRANT rather
//! than a prune, so it is the door that pays for it, and the sentence
//! above must not be read as unconditional while #862 is open.
//!
//! **The other direction costs this door too, and costs it more
//! often.** A box that is too BIG cannot make a wrong certificate —
//! it can only withhold one — but withholding is this door's whole
//! output. `face_box`'s cylinder arm is over-wide by a full radius
//! along its own axis and its conic arm's amplitude is not even a
//! function of the locus (both #862), so genuinely separated
//! placements are refused here for reasons that are not about the
//! placements.
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
