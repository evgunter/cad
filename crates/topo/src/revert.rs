//! `revert` — whole-body orientation reversal (M3 PR 1): the ch. 15
//! `revert(b)` that boolean difference needs (`A ∖ B ≡ A ∩ revert(B)`).
//!
//! A reverted body bounds the **complementary** volume: every loop's
//! half-edge cycle runs backwards, so by the interior-left rule the
//! outward side flips. The surgery is a pure per-entity map — no keys
//! are minted or killed, so the reverted body's arenas are key-for-key
//! those of the source (D9: a deterministic function of the input, and
//! a bitwise **involution** — `revert ∘ revert` is the identity,
//! pinned by test):
//!
//! - **Half-edges**: `start ← end(he)` (each half now runs the other
//!   way), `next ↔ prev` (cycles reverse).
//! - **Edges**: `he_plus ↔ he_minus`. This is what keeps every curve
//!   valid *unchanged*: the old minus half already traverses the
//!   carrier forward, and after reversal it is exactly the
//!   forward-running half — so the he_plus forward contract holds with
//!   zero curve mutation (bitwise involution for free).
//! - **Vertices**: `emanating ← mate(emanating)` (the old anchor no
//!   longer starts here; its mate does — and `mate ∘ mate = id` keeps
//!   the involution). Lone vertices (`None`) are untouched.
//! - **Surfaces**: every `Plane`'s `normal` is negated (`u_ref` and
//!   `origin` unchanged — the frame stays right-handed with `v_ref`
//!   flipping alongside), re-satisfying the convention that a face's
//!   outward normal is its plane's stored normal. Negation is a
//!   bitwise involution.
//! - **Chart images and pcurve rows on a plane**: the flipped `v_ref`
//!   is the plane's chart reflected, `(u, v) ↦ (u, −v)`, so every
//!   datum stated in that chart's coordinates is re-stated under the
//!   reflection with the frame — each [`geom_brep::EdgeDescription::Chart`]
//!   image on a plane ([`geom_brep::EdgeCurve::with_chart_v_mirrored`])
//!   and each stored [`geom_brep::PcurveCache`] row on a plane face
//!   ([`geom_brep::PcurveCache::mirrored_v`]). Certificates travel
//!   verbatim — why a certificate of the source is a certificate of
//!   the result is stated once, on [`geom_brep::Pcurve::mirror_v`].
//!   A `v` sign flip on the stored coefficients is a bitwise
//!   involution, exact in every image kind. Images and rows on the
//!   curved charts are untouched: those charts carry the reversal on
//!   the face's `sense` bit and their frames do not move.
//! - **Face senses** (M5 S12): every face whose surface is **not** a
//!   `Plane` has [`crate::entity::Face::sense`] flipped. This is the
//!   curved arm, and it is the *same* statement as the plane bullet —
//!   each face's outward normal is negated exactly once — written in
//!   whichever of the two encodings the surface class admits:
//!   - a `Plane` can carry its own reversal (the normal is stored, and
//!     negating it is exact), so the plane arm stays where M3 put it
//!     and stays bit-for-bit what the planar battery pins;
//!   - the analytic charts cannot (D1's S10 amendment: cylinder, cone
//!     and torus normals are ODD in the radius, the sphere's is EVEN
//!     and the `radius > 0` convention fixes it outward), and a NURBS
//!     chart's parameterization is not ours to rewrite — so those
//!     faces flip the S10 bit instead.
//!
//!   The two arms are **exclusive by surface kind**, so no face is
//!   flipped twice, and both are exact structure: a `bool` negation
//!   and an IEEE sign flip are each bitwise involutions, so `revert ∘
//!   revert` stays bit-identical at every scalar backend (D1: "exact
//!   structure, never a decide"). The sense flip is applied to every
//!   face on a non-plane surface — including one already carrying an
//!   honest `false` from S11's concave/inward constructors, which is
//!   the whole point: reverting a body with mixed senses must flip
//!   each of them, not stamp a constant.
//! - **Loop anchors**: every cycle's `first` moves to the half-edge
//!   that PRECEDES it in the source — its successor once the cycle
//!   runs the other way. The anchor is load-bearing on a minted
//!   periodic chart ([`crate::entity::LoopBoundary::Cycle`]'s `first`
//!   states the invariant: the one-branch walk can report a period
//!   wrap only at the joint before `first`), and a reversal that kept
//!   `first` would put that joint right after it — mid-chain, where
//!   tier 3's stored-row continuity pass reports a
//!   `LoopDiscontinuity`. Moved to the source predecessor, the same
//!   joint is the closure again. No row is touched: a row is a
//!   function of the carrier parameter, and a reversal does not touch
//!   it. One rule for every loop — a plane loop has no period, so its
//!   move is a no-op in meaning, and there is nothing a second rule
//!   would protect. `prev` and `next` swap under the map, so the moved
//!   anchor's `prev` in the result is its `next` in the source — the
//!   anchor it came from — and the move is an exact involution with
//!   nothing arithmetic in it.
//! - Loops' membership, faces, shells, solids, points, every curve not
//!   described in a plane's chart, provenance, and F9 null records are
//!   copied unchanged (a re-anchored `Cycle::first` still names a
//!   member of its cycle; outer/ring designation is a maintained
//!   designation and survives; null-entity sides refer to the
//!   splitting surface, not the body's orientation).
//!
//! **No longer planar-only (M5 S12).** Originally (F5) non-`Plane`
//! surfaces could not represent their orientation-reversed side at
//! all: D3's enum is closed and the analytic variants' chart normals
//! have a fixed parity, so there was nothing for this function to
//! write, and it refused `RevertError::UnsupportedSurface`. S10 closed
//! that by moving the reversal onto the FACE —
//! [`crate::entity::Face::sense`] — where flipping it is exact
//! structure; S11's constructors made the incoming bits honest (a
//! concave wall already reads `false`, so there is a real bit to
//! flip rather than a uniform lie); S12 (here) writes it. The refusal
//! is **retired**: the sense flip is uniform over every non-plane
//! surface class, so there is no per-class residue left inside this
//! operator. What remains gated is downstream and belongs to the
//! *boolean*, not to `revert` — a curved subtract still needs a join
//! lane for its seam, and the classes that lack one refuse typed at
//! their own doors naming their own blocker.
//!
//! Functional style (the plan's assumption, made concrete): `revert`
//! takes `&self` and returns a **new body value** — the operand is
//! untouched, both bodies remain usable (Problem 15.7's
//! both-results-free, inherited by ∖'s use of revert).
//!
//! **Validity class**: a reverted body is **tier-2 currency** — every
//! structural invariant survives the map, every EDGE certification
//! survives it (the plane-chart images are re-stated with their
//! frames; every other description is invariant), every stored
//! pcurve ROW is still a certified row of its edge (plane rows
//! re-stated, curved rows untouched), and every loop's one-branch
//! continuity survives. That last is an argument, not a measurement:
//! the reversed cycle's joints are the source's joints read from the
//! other side, so at each joint the stored-row continuity pass
//! (`crate::pcurves`, `validate_pcurves` pass 3) meters the source's
//! gap negated, at the lever read off the other side's `v` — and the
//! two sides' `v` agree within the band that same pass pins the
//! joint's height gap to. The closure moved with the anchor (the
//! anchor bullet), so the one joint the forward walk may have left a
//! period off is the closure still, where the pass does not read it;
//! and `loop_closes` is symmetric in start and end (its `±τ`
//! candidates, the sphere twin's `du ∓ π` and the torus's polar wrap
//! all cover both signs). What the map deliberately does NOT give is
//! tier 3: it bounds the complement, so the +V invariant fails
//! (`NegativeVolume`, pinned by test). That is correct, not a defect:
//! `revert(B)` is ∖'s transient operand, never an at-rest solid
//! handed across the API. `sweep`'s `revert_periodic_wrap` measures
//! exactly `NegativeVolume` on the two-arc sphere's cavity and a cone
//! through its apex (loops that wrap at closure), and on the drums,
//! the tori and a NURBS loft (loops that do not);
//! `revert_plane_charts` on the plane-mirror rows.
//!
//! Serves ch. 15 `setopfinish` (difference reverts `B`'s kept
//! component) and the `A ∖ B ≡ A ∩ revert(B)` oracle (M3 PR 5).

use core::fmt;
use std::collections::BTreeSet;

use geom::Surface;
use geom_core::Real;

use crate::body::Body;
use crate::entity::{HalfEdgeKey, LoopBoundary, LoopKey};
use crate::geometry::SurfaceKey;
use crate::null::CurveGeom;

/// A failed [`Body::revert`] precondition (closed enum, D3 style); the
/// source body is never touched (revert is `&self`).
///
/// **Retired variant — `UnsupportedSurface`** (M3 PR 1 → M5 S12).
/// Retired, not left unreachable: a closed enum that can no longer
/// produce one of its variants is a lie about the frontier. The record
/// is kept here, and the refusal pin it carried is re-pinned as a
/// CONSTRUCTION row (the S9 pattern) in
/// `crates/sweep/tests/m5_s12_curved_ops.rs` (the curved arm needs the
/// sweep constructors to build a curved body at all).
///
/// - **What it said**: a surface is not a `Plane`, so this operator has
///   no representation to write for the reversed side of a curved face.
/// - **Why it is gone**: the reversal moved onto the FACE.
///   [`Body::revert`] flips [`crate::entity::Face::sense`] on every face
///   carried by a non-plane surface — exact structure, uniform over
///   cylinder, cone, sphere, torus and NURBS alike — so no per-class
///   residue is left *inside* `revert`. What is still gated is
///   downstream and belongs to the boolean: a curved subtract needs a
///   JOIN lane for its seam, and the classes lacking one refuse typed at
///   their own doors, naming their own blocker.
/// - **The parity finding it carried** (M5 PR 9c, executed 2026-08-01;
///   scoped per kind by that review's F1), retained because it is the
///   reason no surface-side fix ever existed, and hence why D1's S10
///   amendment had to be *ratified* rather than coded around:
///   - **Cylinder, cone, torus**: the chart normal is ODD in the radius
///     (`∂u × ∂v = r·radial(u)` for the cylinder, analogously for the
///     other two), so it is OUTWARD for either sign — a negative radius
///     moves the point to `radial(u + π)` and the normal with it, and
///     negating `axis` merely reparameterizes `u ↦ −u`. Nothing to
///     write.
///   - **Sphere**: `∂u × ∂v = r²·cos v·n̂` is EVEN in the radius, so the
///     chart normal is outward exactly under the ratified `radius > 0`
///     convention. A negative-radius sphere is therefore a de facto
///     reversed sphere — REJECTED as a representation, not adopted: it
///     breaks that convention, and every consumer metering a sphere
///     residual by `2r` reads the sign backwards, this build's own
///     `point_in_solid` sphere arm included.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RevertError {
    /// A link the reversal map follows does not resolve — tier-1-invalid
    /// input, surfaced typed (D9: never a panic). The variant names the
    /// link, so the report names the thing that failed to resolve.
    Corrupt {
        /// Which link.
        link: RevertLink,
    },
}

/// The link of a tier-1-invalid body that [`Body::revert`]'s
/// precondition read could not follow.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RevertLink {
    /// `he`'s derived end (`start` of its `next`) does not resolve.
    End {
        /// The half-edge whose `next` chain is broken.
        he: HalfEdgeKey,
    },
    /// `he`, a vertex's emanating half-edge, has no mate.
    Mate {
        /// The emanating half-edge.
        he: HalfEdgeKey,
    },
    /// The loop's cycle anchor, or that anchor's `prev`, does not
    /// resolve.
    LoopAnchor {
        /// The loop.
        r#loop: LoopKey,
        /// The half-edge key that did not resolve.
        he: HalfEdgeKey,
    },
}

impl fmt::Display for RevertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self::Corrupt { link } = self;
        write!(f, "revert: ")?;
        match link {
            RevertLink::End { he } => write!(f, "half-edge {he:?}'s end does not resolve"),
            RevertLink::Mate { he } => write!(f, "emanating half-edge {he:?} has no mate"),
            RevertLink::LoopAnchor { r#loop, he } => write!(
                f,
                "loop {loop:?}'s anchor half-edge {he:?} (or its `prev`) does not resolve"
            ),
        }?;
        write!(f, " (malformed body)")
    }
}

impl std::error::Error for RevertError {}

impl<T: Real> Body<T> {
    /// The orientation-reversed body value (module docs: the per-entity
    /// map, the two exclusive orientation-reversal encodings, the
    /// involution/determinism contract). Every surface class is
    /// supported as of M5 S12. The source is untouched.
    ///
    /// # Errors
    ///
    /// [`RevertError::Corrupt`] on tier-1-invalid input the reversal map
    /// cannot follow — every link the map reads is resolved first, and
    /// the variant names the one that did not ([`RevertLink`]). All
    /// checks precede construction of the result.
    pub fn revert(&self) -> Result<Self, RevertError> {
        // ---- Preconditions (read-only). ----
        // Which surfaces carry their own reversal (`Plane`: the normal
        // is negated in the surface — the M3 encoding) and which push it
        // onto the face's `sense` bit (every other class — module docs).
        // Read from the SOURCE: revert is key-for-key, so the
        // classification is valid for the result too.
        let plane_surfaces: BTreeSet<SurfaceKey> = self
            .surfaces
            .iter()
            .filter(|(_, surface)| matches!(surface, Surface::Plane { .. }))
            .map(|(key, _)| key)
            .collect();
        // Resolve every half-edge's new start and every vertex's new
        // anchor from the SOURCE before building the result (the map
        // must read pre-reversal adjacency throughout).
        let mut new_starts = Vec::with_capacity(self.half_edges.len());
        for (he_key, _) in self.half_edges.iter() {
            let end = self.half_edge_end(he_key).ok_or(RevertError::Corrupt {
                link: RevertLink::End { he: he_key },
            })?;
            new_starts.push((he_key, end));
        }
        let mut new_anchors = Vec::new();
        for (vertex_key, vertex) in self.vertices.iter() {
            if let Some(emanating) = vertex.emanating {
                let mate = self.mate(emanating).ok_or(RevertError::Corrupt {
                    link: RevertLink::Mate { he: emanating },
                })?;
                new_anchors.push((vertex_key, mate));
            }
        }
        // Every cycle's anchor (module docs): `first` moves to its
        // SOURCE predecessor, read here before `next` and `prev` swap.
        // The predecessor is resolved, not just read: a `prev` naming a
        // dead key is the tier-1 corruption this door refuses typed.
        let mut new_firsts = Vec::new();
        for (loop_key, lp) in self.loops.iter() {
            let LoopBoundary::Cycle { first } = lp.boundary else {
                continue;
            };
            let anchor = |he| RevertError::Corrupt {
                link: RevertLink::LoopAnchor {
                    r#loop: loop_key,
                    he,
                },
            };
            let prev = self.get_half_edge(first).ok_or(anchor(first))?.prev;
            if self.get_half_edge(prev).is_none() {
                return Err(anchor(prev));
            }
            new_firsts.push((loop_key, prev));
        }

        // ---- The map (infallible from here on). ----
        let mut out = self.clone();
        // The three keyed loops below carry a value the plan phase
        // derived per key, so they look their key up; `out` is a clone
        // of `self` and cloning a slotmap preserves its keys, so every
        // lookup resolves and the map removes nothing. The edge loop
        // carries no such value and therefore does not look anything
        // up — it walks the arena directly, like the surface and face
        // loops below.
        for (he_key, start) in new_starts {
            let Some(he) = out.get_half_edge_mut(he_key) else {
                unreachable!("revert: `he_key` was iterated out of the arena `out` clones")
            };
            he.start = start;
            core::mem::swap(&mut he.next, &mut he.prev);
        }
        for (_, edge) in out.edges.iter_mut() {
            core::mem::swap(&mut edge.he_plus, &mut edge.he_minus);
        }
        for (vertex_key, anchor) in new_anchors {
            let Some(vertex) = out.get_vertex_mut(vertex_key) else {
                unreachable!("revert: `vertex_key` was iterated out of the arena `out` clones")
            };
            vertex.emanating = Some(anchor);
        }
        for (loop_key, first) in new_firsts {
            let Some(lp) = out.get_loop_mut(loop_key) else {
                unreachable!("revert: `loop_key` was iterated out of the arena `out` clones")
            };
            lp.boundary = LoopBoundary::Cycle { first };
        }
        for (_, surface) in out.surfaces.iter_mut() {
            if let Surface::Plane { normal, .. } = surface {
                *normal = -*normal;
            }
        }
        // The plane charts' images and rows go with their frames
        // (module docs): every datum stated in a plane's chart
        // coordinates, re-stated under the reflection the negation
        // above is. Chart images are walked by CURVE, so a carrier
        // shared by several edges is mirrored once; stored rows by
        // half-edge, resolved to their face through the SOURCE (the
        // topology is key-for-key, and `out`'s is mid-map). A row
        // whose half-edge no longer resolves is on no face, so it is
        // on no plane face and travels as found — the dead-key
        // exception `crate::pcurves`'s posture docs state for this
        // door.
        for (_, geom) in out.curves.iter_mut() {
            let CurveGeom::Certified(curve) = geom else {
                continue;
            };
            let on_plane = curve
                .description()
                .chart()
                .is_some_and(|c| plane_surfaces.contains(&c.surface));
            if on_plane {
                *curve = curve.with_chart_v_mirrored();
            }
        }
        for (he_key, row) in out.pcurves.iter_mut() {
            let on_plane = self
                .face_of_half_edge(he_key)
                .and_then(|f| self.get_face(f))
                .is_some_and(|face| plane_surfaces.contains(&face.surface));
            if on_plane {
                *row = row.mirrored_v();
            }
        }
        // The curved arm (M5 S12): the reversal a non-plane chart
        // cannot express goes on the FACE. Exclusive with the plane
        // negation above — a face is flipped in exactly one encoding —
        // and it is a `bool` negation, so it is exact structure at every
        // backend and a bitwise involution.
        for (_, face) in out.faces.iter_mut() {
            if !plane_surfaces.contains(&face.surface) {
                face.sense = !face.sense;
            }
        }
        // N6: `revert` flips every surface source's orientation tag
        // (`rev ∘ rev = id`) — the negated description is the SAME
        // recipe source seen from the other side. Only the `Recipe`
        // arm of the provenance row moves: the other three origins
        // carry no orientation, and a reversal neither stamps nor
        // clears. Curve and point records are untouched (their
        // descriptions are).
        //
        // The per-field ParamSource rows are untouched too, and that is
        // a decision rather than an omission: a token names the
        // EXPRESSION a stored scalar came from, and a radius is the
        // same number whichever side of the surface the material is on.
        // The channel carries no orientation to flip
        // (`crate::param_source`).
        for (_, origin) in out.surface_origins.iter_mut() {
            if let crate::GeomOrigin::Recipe(gs) = origin {
                *gs = gs.reverted();
            }
        }

        #[cfg(debug_assertions)]
        debug_assert_eq!(
            crate::validate::validate(&out),
            Ok(()),
            "revert postcondition: result is not tier-1 valid (kernel bug)",
        );
        Ok(out)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{RevertError, RevertLink};
    use crate::test_support_fixtures::declined_cube;
    use geom_core::Tol;

    /// **CONSTRUCTION row, flipped from the M3 refusal pin** (S9
    /// pattern; the retired `UnsupportedSurface` record is on
    /// [`RevertError`]). `declined_cube`'s faces all share the `mvfs`
    /// `Nurbs` placeholder surface — the exact shape that refused
    /// before S12 — and now revert: every face's `sense` flips, and
    /// nothing else about a non-plane surface moves. (The full
    /// involution/determinism/tier pins run on the geometric cube in
    /// `tests/m3_pr1_surgery.rs` and on real analytic surfaces in
    /// `crates/sweep/tests/m5_s12_curved_ops.rs`.)
    #[test]
    fn revert_flips_sense_on_non_plane_faces_instead_of_refusing() {
        let cube = declined_cube::<f64>(Tol::witness());
        let before: Vec<bool> = cube.body.faces().map(|(_, f)| f.sense).collect();
        assert!(before.iter().all(|s| *s), "mvfs/mef mint sense: true");
        let reverted = cube.body.revert().expect("S12: curved revert is wired");
        let after: Vec<bool> = reverted.faces().map(|(_, f)| f.sense).collect();
        assert!(after.iter().all(|s| !*s), "every non-plane face flipped");
        // The surfaces themselves are untouched (the reversal is on the
        // face, not the chart), and the involution is bitwise.
        assert_eq!(
            format!("{:?}", reverted.surfaces().collect::<Vec<_>>()),
            format!("{:?}", cube.body.surfaces().collect::<Vec<_>>()),
        );
        assert_eq!(
            format!("{:?}", reverted.revert().unwrap()),
            format!("{:?}", cube.body),
        );
    }

    /// **Every loop's anchor moves to its source predecessor.**
    /// `declined_cube`'s faces all share the `mvfs` placeholder surface
    /// (not a plane), and a one-face body whose face is a plane
    /// carries a two-half-edge cycle: in both the reverted `first` is
    /// the source's `prev(first)`, which — `next` and `prev` having
    /// swapped — is the source anchor's successor in the reversed
    /// cycle, so the source's anchor is now the cycle's last
    /// half-edge and the second reversal lands back on it bit for
    /// bit. One rule, no surface-kind partition (module docs, the
    /// anchor bullet). Neither fixture stores a pcurve row, so this
    /// row pins the structural move and the involution alone; the
    /// wrap the move exists for is measured in `sweep`'s
    /// `revert_periodic_wrap`.
    #[test]
    fn revert_re_anchors_every_loop_at_the_source_predecessor() {
        use crate::LoopBoundary;
        let assert_moved = |body: &crate::Body<f64>, reverted: &crate::Body<f64>| {
            let mut moved = 0;
            for (lk, lp) in body.loops() {
                let LoopBoundary::Cycle { first } = lp.boundary else {
                    panic!("every loop here is a cycle")
                };
                let LoopBoundary::Cycle { first: after } = reverted.get_loop(lk).unwrap().boundary
                else {
                    panic!("a cycle stays a cycle")
                };
                assert_eq!(
                    after,
                    body.get_half_edge(first).unwrap().prev,
                    "the reverted anchor is the source's predecessor"
                );
                assert_eq!(
                    reverted.get_half_edge(first).unwrap().next,
                    after,
                    "which is the source anchor's successor in the reversed cycle"
                );
                assert_eq!(
                    reverted.get_half_edge(after).unwrap().prev,
                    first,
                    "so the source's anchor closes the reversed cycle"
                );
                assert_ne!(after, first, "the cycle has more than one half-edge");
                moved += 1;
            }
            assert_eq!(
                format!("{:?}", reverted.revert().unwrap()),
                format!("{body:?}"),
                "bitwise involution"
            );
            moved
        };
        let cube = declined_cube::<f64>(Tol::witness());
        assert_eq!(assert_moved(&cube.body, &cube.body.revert().unwrap()), 6);

        let plane = lone_plane_face();
        assert_eq!(assert_moved(&plane, &plane.revert().unwrap()), 1);
    }

    /// One plane face bounded by a single line edge (a two-half-edge
    /// cycle), built through the Euler door alone.
    fn lone_plane_face() -> crate::Body<f64> {
        let mut plane = crate::Body::<f64>::new();
        let seed = plane.mvfs(geom_core::Point3::new(0.0, 0.0, 0.0)).unwrap();
        plane
            .set_face_surface(
                seed.face,
                crate::FaceSurface::New(geom::Surface::Plane {
                    origin: geom_core::Point3::new(0.0, 0.0, 0.0),
                    normal: geom_core::Vec3::unit_z(),
                    u_ref: geom_core::Vec3::unit_x(),
                }),
            )
            .unwrap();
        plane
            .mev_line(
                crate::MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                geom_core::Point3::new(1.0, 0.0, 0.0),
                Tol::witness(),
            )
            .unwrap();
        plane
    }

    /// **The anchor read resolves the link it follows.** The
    /// precondition phase reads each cycle's `prev(first)` and pushes
    /// it as the new anchor; a `prev` naming a dead key is tier-1
    /// corruption, and the door refuses it typed — naming the loop and
    /// the key — rather than returning an `Ok` body whose anchor names
    /// no half-edge (which only the debug postcondition would catch,
    /// and a release build not at all). A dead anchor itself is
    /// refused the same way.
    #[test]
    fn revert_refuses_a_dangling_anchor_prev_typed() {
        use crate::{HalfEdgeKey, LoopBoundary};
        let cube = declined_cube::<f64>(Tol::witness());
        let (lk, first) = cube
            .body
            .loops()
            .find_map(|(lk, lp)| match lp.boundary {
                LoopBoundary::Cycle { first } => Some((lk, first)),
                LoopBoundary::Empty { .. } => None,
            })
            .unwrap();
        let dead = HalfEdgeKey::default();

        let mut body = cube.body.clone();
        body.get_half_edge_mut(first).unwrap().prev = dead;
        assert_eq!(
            body.revert().err(),
            Some(RevertError::Corrupt {
                link: RevertLink::LoopAnchor {
                    r#loop: lk,
                    he: dead
                }
            }),
            "a dangling `prev` on the anchor refuses typed, naming the dead key"
        );

        let mut body = cube.body.clone();
        body.get_loop_mut(lk).unwrap().boundary = LoopBoundary::Cycle { first: dead };
        assert_eq!(
            body.revert().err(),
            Some(RevertError::Corrupt {
                link: RevertLink::LoopAnchor {
                    r#loop: lk,
                    he: dead
                }
            }),
            "a dead anchor refuses typed, naming the loop"
        );
    }

    /// **A plane `Chart` image with a `v` channel survives the map.**
    /// One plane face (`z = 0`, `u_ref = +x`) carrying a half-circle
    /// at rest in its chart, built through the Euler door alone: the
    /// image is `(cos t, sin t)`, and its `v` is what an unmirrored
    /// reversal used to leave pointing the wrong way. After `revert`
    /// the image is the stored one with `v` negated coefficient for
    /// coefficient; the SOURCE's curve re-certified against the
    /// reverted surfaces refuses `ChartResidual` at sample 1 (the
    /// merge-base shape of the defect, kept as the control), the
    /// reverted curve re-certified there yields the certificate it
    /// carries, byte for byte, and the involution restores the bits.
    #[test]
    fn revert_mirrors_a_plane_chart_image_and_its_certificate_survives() {
        use crate::{FaceSurface, MevSite};
        use geom::{Curve3, Surface};
        use geom_brep::certify::{CertCheck, CertifyError};
        use geom_brep::{EdgeCurveSpec, Pcurve};
        use geom_core::{Band, Point3, Vec3};

        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let circle = Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        let (t0, t1) = (0.0, core::f64::consts::PI);
        let (start, end) = (circle.eval(t0), circle.eval(t1));
        let plane = Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let mut body = crate::Body::<f64>::new();
        let seed = body.mvfs(start).unwrap();
        body.set_face_surface(seed.face, FaceSurface::New(plane))
            .unwrap();
        let chart = body.get_face(seed.face).unwrap().surface;
        let spec = EdgeCurveSpec::arc_of_circle(circle, t0, t1)
            .unwrap()
            .at_rest_in_chart(chart, false);
        body.mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            end,
            spec,
            tol,
        )
        .unwrap();
        let curve_of = |b: &crate::Body<f64>| {
            let (_, e) = b.edges().next().unwrap();
            b.get_curve_geom(e.curve)
                .unwrap()
                .certified()
                .unwrap()
                .clone()
        };
        let image_of =
            |c: &geom_brep::EdgeCurve<f64>| c.description().chart().unwrap().pcurve.clone();
        let source = curve_of(&body);
        let Pcurve::Harmonic { p0, pa, pb, pl } = image_of(&source) else {
            panic!("a circle in a plane chart is a harmonic image")
        };
        assert!(
            pb.y.abs() > 0.5,
            "the image has a v channel (sin t · 1): pb = {pb:?}"
        );

        let reverted = body.revert().unwrap();
        let mirrored = curve_of(&reverted);
        let Pcurve::Harmonic {
            p0: q0,
            pa: qa,
            pb: qb,
            pl: ql,
        } = image_of(&mirrored)
        else {
            panic!("the mirrored image keeps its kind")
        };
        for (before, after) in [(p0.x, q0.x), (pa.x, qa.x), (pb.x, qb.x), (pl.x, ql.x)] {
            assert_eq!(
                before.to_bits(),
                after.to_bits(),
                "u coefficients are untouched"
            );
        }
        for (before, after) in [(p0.y, q0.y), (pa.y, qa.y), (pb.y, qb.y), (pl.y, ql.y)] {
            assert_eq!(
                (-before).to_bits(),
                after.to_bits(),
                "v coefficients are negated"
            );
        }
        let surfaces = |k| reverted.get_surface(k).cloned();
        // The control: the unmirrored image against the reverted plane
        // is the defect, and the meter says so where it always did.
        assert!(
            matches!(
                source.recertify(start, end, surfaces, band),
                Err(CertifyError::ResidualExceeded {
                    check: CertCheck::ChartResidual,
                    sample: 1
                })
            ),
            "the source's image is wrong on the reverted plane"
        );
        let rerun = mirrored
            .recertify(start, end, surfaces, band)
            .expect("the mirrored image certifies on the reverted plane");
        assert_eq!(
            format!("{rerun:?}"),
            format!("{:?}", mirrored.certificate()),
            "the certificate that travelled verbatim is the fresh run's"
        );
        assert_eq!(
            format!("{:?}", reverted.revert().unwrap()),
            format!("{body:?}"),
            "bitwise involution"
        );
    }
}
