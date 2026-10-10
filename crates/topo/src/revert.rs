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
//! - **Joint elements** ([`crate::joint`]): the element of the joint
//!   `p → he` is stored on `he`, and the reversed cycle runs `he → p`,
//!   so it moves onto `p` — the source predecessor — as its inverse:
//!   the same deck transformation read from the other side. A row's
//!   image is a function of the carrier parameter, which a reversal does
//!   not touch, so every image stays where it is. Inversion is exact
//!   integer arithmetic and its own inverse, so the involution holds
//!   bit for bit. No loop's `first` moves: no stored byte depends on
//!   it.
//! - Loops' membership and anchors, faces, shells, solids, points,
//!   every curve not described in a plane's chart, provenance, and F9
//!   null records are copied unchanged (outer/ring designation is a maintained
//!   designation and survives; null-entity sides refer to the
//!   splitting surface, not the body's orientation).
//!
//! **Every surface class reverts, and `revert` refuses nothing.** The
//! sense flip is uniform over every non-plane surface class, so no
//! per-class residue is left inside this operator. What remains gated
//! is downstream and belongs to the *boolean* — a curved subtract still
//! needs a join lane for its seam, and the classes that lack one refuse
//! typed at their own doors naming their own blocker. The operand is a
//! body some door left, at rest or mid-operation, so a link the map
//! cannot follow is a kernel bug and panics naming its record (D2
//! row 4).
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
//! continuity survives: each joint's element is the source's inverted,
//! which carries the same two image points the other way, and a loop's
//! winding is the source's inverted, which closes exactly when it does.
//! What the map deliberately does NOT give is
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

use std::collections::BTreeSet;

use geom::Surface;
use geom_core::Real;

use crate::body::Body;
use crate::entity::{EntityId, FaceKey, GeomRef, HalfEdgeKey};
use crate::geometry::CurveKey;
use crate::live::{OPERATORS_KEEP_LINKS, Proven, dangling_link, link};
use crate::null::CurveGeom;

impl<T: Real> Body<T> {
    /// The orientation-reversed body value (module docs: the per-entity
    /// map, the two exclusive orientation-reversal encodings, the
    /// involution/determinism contract). Every surface class is
    /// supported as of M5 S12. The source is untouched.
    ///
    /// # Panics
    ///
    /// Where a link the map follows does not resolve, naming the record
    /// that holds it, or resolves to a start or an anchor the reversed
    /// body could not keep (D2 row 4): a body no public door leaves.
    /// Every such read precedes construction of the result.
    #[track_caller]
    pub fn revert(&self) -> Self {
        // ---- The plan (read-only). ----
        // Which faces' surfaces carry their own reversal (`Plane`: the
        // normal is negated in the surface — the M3 encoding) and which
        // push it onto the face's `sense` bit (every other class — module
        // docs); which chart images and stored rows are stated in a
        // plane's chart. Read from the SOURCE: revert is key-for-key, so
        // the classification is valid for the result too. A row whose
        // half-edge no longer resolves is on no face, so it is on no
        // plane face and travels as found — the dead-key exception
        // `crate::pcurves`'s posture docs state for this door.
        let is_plane = |surface: &Surface<T>| matches!(surface, Surface::Plane { .. });
        let plane_faces: BTreeSet<FaceKey> = self
            .faces
            .iter()
            .filter(|&(face, data)| is_plane(self.face_surface_linked(face, data)))
            .map(|(face, _)| face)
            .collect();
        let plane_images: BTreeSet<CurveKey> = self
            .curves
            .iter()
            .filter(|&(key, geom)| {
                let CurveGeom::Certified(curve) = geom else {
                    return false;
                };
                curve.description().chart().is_some_and(|chart| {
                    is_plane(self.get_surface(chart.surface).unwrap_or_else(|| {
                        dangling_link(
                            GeomRef::Curve(key),
                            "chart image's surface",
                            GeomRef::Surface(chart.surface),
                        )
                    }))
                })
            })
            .map(|(key, _)| key)
            .collect();
        let plane_rows: BTreeSet<HalfEdgeKey> = self
            .pcurves
            .keys()
            .filter(|&he| {
                self.half_edges.contains_key(he) && plane_faces.contains(&self.face_of_linked(he))
            })
            .collect();
        // Resolve every half-edge's new start and every vertex's new
        // anchor from the SOURCE before building the result (the map
        // must read pre-reversal adjacency throughout).
        // Every new start and every new anchor is proven to hold in the
        // result: a live but foreign `next`, `prev` or `start` would
        // otherwise carry its fault onto the start or anchor read
        // through it.
        let mut new_starts = Vec::with_capacity(self.half_edges.len());
        for (he_key, _) in self.half_edges.iter() {
            let end = self.proven_half_edge_end(he_key);
            let proof = self.proven_mate(he_key, Proven);
            let (mate, mate_start) = (proof.mate, proof.mate_data.start);
            if mate_start != end {
                unreachable!(
                    "{he_key:?}'s end {end:?}, its start once reversed, is not its mate \
                     {mate:?}'s start {mate_start:?}: on a tier-1-valid body an edge's halves \
                     run between its two ends in opposite directions; {OPERATORS_KEEP_LINKS}"
                );
            }
            new_starts.push((he_key, end));
        }
        let mut new_anchors = Vec::new();
        for (vertex_key, vertex) in self.vertices.iter() {
            if let Some(emanating) = vertex.emanating {
                let holder = link(EntityId::Vertex(vertex_key), "emanating");
                let mate = self.proven_mate(emanating, holder).mate;
                let anchor_start = self.proven_half_edge_end(mate);
                if anchor_start != vertex_key {
                    unreachable!(
                        "{vertex_key:?}'s new anchor {mate:?} (its emanating half-edge's mate) \
                         would start at {anchor_start:?}: on a tier-1-valid body a vertex's \
                         emanating half-edge starts at it; {OPERATORS_KEEP_LINKS}"
                    );
                }
                new_anchors.push((vertex_key, mate));
            }
        }
        // Every joint element moves onto its source predecessor,
        // inverted (module docs): the element on `next(he)` is the one of
        // the joint `he → next(he)`, which the reversed cycle runs
        // `next(he) → he`. Read here, before `next` and `prev` swap; every
        // `next` resolves (the first pass proved it).
        let new_joints: Vec<(HalfEdgeKey, Option<crate::JointElement>)> = self
            .half_edges
            .iter()
            .map(|(he_key, he)| {
                (
                    he_key,
                    self.joint(he.next).map(crate::JointElement::inverse),
                )
            })
            .collect();

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
        out.joints.clear();
        for (he_key, element) in new_joints {
            out.write_joint(he_key, element);
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
        // shared by several edges is mirrored once.
        for (key, geom) in out.curves.iter_mut() {
            let CurveGeom::Certified(curve) = geom else {
                continue;
            };
            let on_plane = plane_images.contains(&key);
            // `None` is the one image with no reflected locus — a
            // spiric WALL image, which lives on a torus chart by
            // construction and therefore cannot be on a plane face.
            // Reached only by a row already on the wrong face, which
            // this door may not repair: it travels as found, the
            // dead-key exception's posture one case over, and the
            // at-rest pcurve validator is what reports it.
            if on_plane && let Some(mirrored) = curve.with_chart_v_mirrored() {
                *curve = mirrored;
            }
        }
        for (he_key, row) in out.pcurves.iter_mut() {
            if plane_rows.contains(&he_key)
                && let Some(mirrored) = row.mirrored_v()
            {
                *row = mirrored;
            }
        }
        // The curved arm (M5 S12): the reversal a non-plane chart
        // cannot express goes on the FACE. Exclusive with the plane
        // negation above — a face is flipped in exactly one encoding —
        // and it is a `bool` negation, so it is exact structure at every
        // backend and a bitwise involution.
        for (key, face) in out.faces.iter_mut() {
            if !plane_faces.contains(&key) {
                face.sense = !face.sense;
            }
        }

        #[cfg(debug_assertions)]
        debug_assert_eq!(
            crate::validate::validate(&out),
            Ok(()),
            "revert postcondition: result is not tier-1 valid (kernel bug)",
        );
        out
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use crate::test_support_fixtures::declined_cube;
    use geom_core::Tol;

    /// **A non-plane face reverts on its `sense` bit.**
    /// `declined_cube`'s faces all share the `mvfs` `Nurbs` placeholder
    /// surface, and revert: every face's `sense` flips, and
    /// nothing else about a non-plane surface moves. (The full
    /// involution/determinism/tier pins run on the geometric cube in
    /// `tests/m3_pr1_surgery.rs` and on real analytic surfaces in
    /// `crates/sweep/tests/m5_s12_curved_ops.rs`.)
    #[test]
    fn revert_flips_sense_on_non_plane_faces_instead_of_refusing() {
        let cube = declined_cube::<f64>(Tol::witness());
        let before: Vec<bool> = cube.body.faces().map(|(_, f)| f.sense).collect();
        assert!(
            before.iter().all(|s| *s),
            "the seed states sense: true and every mef derives it"
        );
        let reverted = cube.body.revert();
        let after: Vec<bool> = reverted.faces().map(|(_, f)| f.sense).collect();
        assert!(after.iter().all(|s| !*s), "every non-plane face flipped");
        // The surfaces themselves are untouched (the reversal is on the
        // face, not the chart), and the involution is bitwise.
        assert_eq!(
            format!("{:?}", reverted.surfaces().collect::<Vec<_>>()),
            format!("{:?}", cube.body.surfaces().collect::<Vec<_>>()),
        );
        assert_eq!(
            format!("{:?}", reverted.revert()),
            format!("{:?}", cube.body),
        );
    }

    /// **Every loop keeps its anchor.** `declined_cube`'s faces all
    /// share the `mvfs` placeholder surface (not a plane), and a one-face
    /// body whose face is a plane carries a two-half-edge cycle: in both
    /// every reverted `first` is the source's, and the involution is
    /// bitwise. No stored byte depends on `first` (module docs), so one
    /// rule holds for every loop.
    #[test]
    fn revert_keeps_every_loops_anchor() {
        let assert_kept = |body: &crate::Body<f64>, reverted: &crate::Body<f64>| {
            let mut kept = 0;
            for (lk, lp) in body.loops() {
                assert_eq!(
                    reverted.get_loop(lk).unwrap().boundary,
                    lp.boundary,
                    "the anchor stays"
                );
                kept += 1;
            }
            assert_eq!(
                format!("{:?}", reverted.revert()),
                format!("{body:?}"),
                "bitwise involution"
            );
            kept
        };
        let cube = declined_cube::<f64>(Tol::witness());
        assert_eq!(assert_kept(&cube.body, &cube.body.revert()), 6);

        let plane = lone_plane_face();
        assert_eq!(assert_kept(&plane, &plane.revert()), 1);
    }

    /// One plane face bounded by a single line edge (a two-half-edge
    /// cycle), built through the Euler door alone.
    fn lone_plane_face() -> crate::Body<f64> {
        let mut plane = crate::Body::<f64>::new();
        let seed = plane
            .mvfs(geom_core::Point3::new(0.0, 0.0, 0.0), true)
            .unwrap();
        plane
            .set_face_surface(
                seed.face,
                crate::FaceSurface::New {
                    surface: geom::Surface::Plane {
                        origin: geom_core::Point3::new(0.0, 0.0, 0.0),
                        normal: geom_core::Vec3::unit_z(),
                        u_ref: geom_core::Vec3::unit_x(),
                    },
                    sense: true,
                },
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

    /// **Each torn link panics naming its record.** One tear of
    /// `declined_cube` per link the map follows: every read precedes the
    /// build, so the panic is the plan's (debug or release), names the
    /// tier-1 premise, and names the keys every arena order of the tear
    /// shares — a tear that faults two half-edges panics at whichever
    /// comes first.
    #[test]
    fn revert_panics_naming_each_torn_link() {
        use crate::review_d18::ROW_FOUR;
        use crate::{EdgeKey, HalfEdgeKey};
        let cube = declined_cube::<f64>(Tol::witness());
        let body = &cube.body;
        let dead = HalfEdgeKey::default();
        let (v, vertex) = body.vertices().next().unwrap();
        let e = vertex.emanating.unwrap();
        let m = body.mate(e).unwrap();
        let edge = body.get_half_edge(e).unwrap().edge;
        let foreign = body
            .half_edges()
            .find(|(_, h)| h.start != v)
            .map(|(k, _)| k)
            .unwrap();
        let w = body.get_half_edge(foreign).unwrap().start;
        let unanchored = body
            .half_edges()
            .map(|(k, _)| k)
            .find(|&k| body.vertices().all(|(_, x)| x.emanating != Some(k)))
            .unwrap();
        let torn = |tear: &dyn Fn(&mut crate::Body<f64>)| {
            let mut torn = body.clone();
            tear(&mut torn);
            torn
        };
        let rows: [(&str, crate::Body<f64>, Vec<String>); 7] = [
            (
                "a dead `next`",
                torn(&|b| b.get_half_edge_mut(e).unwrap().next = dead),
                vec![format!("{e:?}'s next names"), format!("{dead:?}")],
            ),
            (
                "a half-edge on a dead edge",
                torn(&|b| b.get_half_edge_mut(unanchored).unwrap().edge = EdgeKey::default()),
                vec![
                    format!("{unanchored:?}"),
                    format!("{:?}", EdgeKey::default()),
                ],
            ),
            (
                "a dead emanating half-edge",
                torn(&|b| b.get_vertex_mut(v).unwrap().emanating = Some(dead)),
                vec![format!("{v:?}'s emanating names"), format!("{dead:?}")],
            ),
            (
                "a foreign `next`",
                torn(&|b| b.get_half_edge_mut(m).unwrap().next = foreign),
                vec![format!("{m:?}'s end"), format!("mate {e:?}'s start")],
            ),
            (
                "a foreign `start`, read as its predecessor's end and its mate's mate's start",
                torn(&|b| b.get_half_edge_mut(e).unwrap().start = w),
                vec!["its start once reversed".to_owned(), format!("{w:?}")],
            ),
            (
                "a dead mate slot",
                torn(&|b| {
                    let edge = b.get_edge_mut(edge).unwrap();
                    if edge.he_plus == m {
                        edge.he_plus = dead;
                    } else {
                        edge.he_minus = dead;
                    }
                }),
                vec![format!("{edge:?}")],
            ),
            (
                "an emanating half-edge that starts elsewhere",
                torn(&|b| b.get_vertex_mut(v).unwrap().emanating = Some(foreign)),
                vec![
                    format!("{v:?}'s new anchor {:?}", body.mate(foreign).unwrap()),
                    format!("would start at {w:?}"),
                ],
            ),
        ];
        for (cause, torn, fragments) in rows {
            let report = crate::surgery::tests::panic_message(std::panic::AssertUnwindSafe(|| {
                drop(torn.revert());
            }));
            for fragment in fragments.iter().map(String::as_str).chain([ROW_FOUR]) {
                assert!(
                    report.contains(fragment),
                    "{cause}: want {fragment:?} in: {report}"
                );
            }
        }
    }

    /// **A chart image's surface is a link.** A half-circle at rest in
    /// a plane face's chart, the face then moved onto a copy of its
    /// plane and the original dropped: every face resolves its chart,
    /// and the image names a surface that does not resolve. The panic
    /// names the curve and the surface. The read sweep cannot tell this
    /// link from the face's, since a sound image names its own face's
    /// surface and the face is read first.
    #[test]
    fn revert_panics_naming_a_chart_images_dangling_surface() {
        use crate::entity::GeomRef;
        use crate::review_d18::ROW_FOUR;
        use crate::{FaceSurface, MevSite};
        use geom::{Curve3, Surface};
        use geom_brep::EdgeCurveSpec;
        use geom_core::{Point3, Vec3};
        let tol = Tol::witness();
        let circle = Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        let plane = Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let mut body = crate::Body::<f64>::new();
        let seed = body.mvfs(circle.eval(0.0), true).unwrap();
        body.set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: plane.clone(),
                sense: true,
            },
        )
        .unwrap();
        let chart = body.get_face(seed.face).unwrap().surface;
        let end = circle.eval(core::f64::consts::PI);
        let spec = EdgeCurveSpec::arc_of_circle(circle, 0.0, core::f64::consts::PI)
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
        let curve = body.edges().next().unwrap().1.curve;
        let copy = body.surfaces.insert(plane);
        body.faces.get_mut(seed.face).unwrap().surface = copy;
        body.surfaces.remove(chart).unwrap();
        let report = crate::surgery::tests::panic_message(std::panic::AssertUnwindSafe(|| {
            drop(body.revert());
        }));
        for fragment in [
            format!("{}'s chart image's surface names", GeomRef::Curve(curve)),
            GeomRef::Surface(chart).to_string(),
            ROW_FOUR.to_owned(),
        ] {
            assert!(report.contains(&fragment), "want {fragment:?} in: {report}");
        }
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
        let seed = body.mvfs(start, true).unwrap();
        body.set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: plane,
                sense: true,
            },
        )
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

        let reverted = body.revert();
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
                    sample: 1,
                    ..
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
            format!("{:?}", reverted.revert()),
            format!("{body:?}"),
            "bitwise involution"
        );
    }
}
