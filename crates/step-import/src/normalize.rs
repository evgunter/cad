//! **Reported structure normalizations** for periodic faces the kernel
//! cannot represent as stated (M7-2 Leg C; the sphere's edge-free face
//! is the sibling case, minted in [`crate::entities`] where the bound
//! is read).
//!
//! # Why a shell can arrive unrepresentable
//!
//! Open CASCADE never splits a periodic face: a full cylinder wall, a
//! full cone, and a whole torus each arrive as ONE `ADVANCED_FACE`
//! whose loop uses its seam edge twice — when the exporter kept the
//! seam at all. Most of that is fine — the importer's manifold
//! precondition and the seam adoption rung were built for exactly
//! that shape, and the cylinder and the truncated cone assemble and
//! certify as stated when the file states the seam. Three shapes do
//! not:
//!
//! - **The apex cone.** Its lateral face is `(seam, base circle,
//!   seam)`, and the seam's far end is the apex — a vertex with ONE
//!   incident edge. `topo`'s tier-2 validity calls a valence-1 vertex
//!   construction scaffolding (`ScaffoldingStrutVertex`), because in a
//!   finished solid it is: the strut tip of an unfinished Euler
//!   sequence. A body carrying one is not a closed solid by the
//!   kernel's own definition.
//! - **The whole torus.** One face wrapping the FULL period in BOTH
//!   chart directions is the fundamental-polygon square, whose four
//!   sides are two curves each used twice. The topology closes
//!   (Euler–Poincaré gives genus 1 and tier 2 passes), but the face is
//!   not a chart iso-rectangle, and the closed-form divergence
//!   contribution reads its own `Δu` off a rim that appears with both
//!   orientations — the volume comes back with the right magnitude and
//!   the wrong SIGN, which tier 3 catches as `NegativeVolume`.
//! - **The seamless band** (M7-5). A cylinder or torus lateral face
//!   stated as its two full-period rim bounds with NO seam generator
//!   at all. The kernel's face model is one outer loop plus rings,
//!   and a curved face carrying a ring has no volume construction
//!   (`RingOnCurvedFace`), so adopting the second rim as a ring would
//!   hand back a body that is not tier-3 measurable. Detection tags
//!   the shape at the face gate ([`crate::entities`]); [`band_seam`]
//!   re-mints it here.
//!
//! # What this module does about it
//!
//! The same D7 stage-3 repair the edge-free sphere takes: the LOCUS is
//! fully explained by the file's own surfaces and carriers and is
//! adopted unchanged — only the boundary-graph tessellation is
//! re-minted, into the splitting a natively built body carries (the
//! kernel's own cone is two lateral half-faces; its own torus is two
//! half-faces; its own revolved wall is ONE face whose loop uses its
//! seam edge twice). The census mapping is carried out as
//! [`crate::StructureNormalization`] data.
//!
//! What a mint may state that the file did not is bounded and named:
//! the apex cone's and whole torus's minted curves are each either a
//! sub-arc of a carrier the file states or that carrier's half-turn
//! rotation about the face's own axis; the band's seam generator is a
//! NEW carrier — the surface's own `u_ref` ruling (cylinder) or
//! `u_ref` meridian arc (torus), the same license under which the
//! edge-free sphere's re-mint ([`crate::entities`]) states its two
//! meridian circles from nothing but the surface's own fields. In
//! every case the minted curve lies ON the file's stated surface by
//! construction and is certified there by the same adoption gates as
//! any file-stated edge; no minted curve is a new shape.
//!
//! Anything that does not match these shapes exactly is left alone —
//! this is a bounded repair of three named cases, not a licence to
//! re-tessellate.
//!
//! # Orientation is derived, never minted
//!
//! A re-tessellation decides how its new faces are wound, which makes
//! it the exact place an inside-out solid could be quietly turned
//! right-side-out. The torus is where that bites: its face is the
//! fundamental-polygon square `[x, y, x', y']`, whose multiset of
//! oriented uses is INVARIANT under loop reversal, so the winding
//! lives only in the cyclic ORDER. [`full_torus`] reads that order —
//! against `u` and `v` directions sampled through the kernel's own
//! chart inverse — and cross-checks it against the face's
//! `same_sense`. The two consistent statements of a right-side-out
//! ring (`.T.` with a CCW loop, `.F.` with a CW one) both adopt to the
//! same body; the two inverted ones REFUSE typed. Nothing about the
//! file's material side is assumed, and nothing is repaired into
//! existence.
//!
//! The M6-6 rider extends the same derived-winding cross-check to the
//! degenerate-apex cone (inside [`apex_cone`], before its re-mint) and
//! — read-only — to two-rim cylinder/cone walls ([`wall_inversion`]):
//! since M6-6 the kernel's tier-3 curved sense gate (check 6,
//! `CurvedSenseInverted`) refuses the inside-out body such a file
//! would build, and import returns certified bodies, so the
//! orientation-inverted diagnosis fires HERE, pre-body, with the
//! whole-torus refusal's typed story.
//!
//! [`band_seam`] (M7-5) reads its winding the same way — each rim's
//! chart-u direction against `same_sense` — before it re-mints: an
//! inverted cylinder band refuses typed pre-body, and a torus band's
//! reading selects which of the two v-intervals between its rims the
//! face covers (its docs say why that is the honest maximum there).

use std::collections::BTreeMap;

use geom::Curve3;
use geom::Surface;
use geom_core::{Point3, Vec3};

use crate::entities::{EdgeSpec, EdgeUse, FaceSpec, LoopSpec, SolidSpec};
use crate::error::StepImportError;
use crate::{FaceCensus, NormalizationKind, StructureNormalization};

/// Rotates `w` a half turn about the unit direction `axis` — the exact
/// f64 identity `2(w·â)â − w`, with no trigonometry to round.
fn half_turn(w: Vec3<f64>, axis: Vec3<f64>) -> Vec3<f64> {
    // A half turn negates exact zeros into `−0.0`; [`crate::signed_zero`]
    // states why a minted field must not carry one.
    crate::signed_zero::plus_zero(axis * (2.0 * w.dot(axis)) - w)
}

/// A carrier rotated a half turn about `(origin, axis)` — the copy of
/// a seam curve half a period away. Only the two kinds the normalized
/// shapes use exist here; anything else answers `None` and the caller
/// leaves the face alone.
fn half_turn_curve(
    carrier: &Curve3<f64>,
    origin: Point3<f64>,
    axis: Vec3<f64>,
) -> Option<Curve3<f64>> {
    let point =
        |p: Point3<f64>| crate::signed_zero::plus_zero_point(origin + half_turn(p - origin, axis));
    match *carrier {
        Curve3::Line { origin: o, dir } => Some(Curve3::Line {
            origin: point(o),
            dir: half_turn(dir, axis),
        }),
        Curve3::Circle {
            center,
            axis: n,
            radius,
            u_ref,
        } => Some(Curve3::Circle {
            center: point(center),
            axis: half_turn(n, axis),
            radius,
            u_ref: half_turn(u_ref, axis),
        }),
        _ => None,
    }
}

/// The shell's vertex valences: how many edge ENDS meet each vertex (a
/// self-loop contributes two).
fn valences(solid: &SolidSpec) -> BTreeMap<u64, usize> {
    let mut out = BTreeMap::new();
    for spec in solid.edges.values() {
        *out.entry(spec.start).or_insert(0) += 1;
        *out.entry(spec.end).or_insert(0) += 1;
    }
    out
}

/// Replaces every use of `edge` in the shell, EXCEPT inside
/// `skip_faces`, by the two halves it was split into.
///
/// `first` runs from the split edge's own start to the new midpoint
/// vertex; `second` runs from the midpoint back to the end. A forward
/// use therefore walks `first` then `second`; a reversed use walks
/// `second` then `first`, each reversed.
fn expand_split_uses(
    solid: &mut SolidSpec,
    edge: u64,
    first: u64,
    second: u64,
    skip_faces: &[usize],
) {
    for (fi, face) in solid.faces.iter_mut().enumerate() {
        if skip_faces.contains(&fi) {
            continue;
        }
        for lp in &mut face.loops {
            let mut out = Vec::with_capacity(lp.uses.len() + 1);
            for u in &lp.uses {
                if u.edge == edge {
                    let (a, b) = if u.forward {
                        (first, second)
                    } else {
                        (second, first)
                    };
                    out.push(EdgeUse {
                        edge: a,
                        forward: u.forward,
                    });
                    out.push(EdgeUse {
                        edge: b,
                        forward: u.forward,
                    });
                } else {
                    out.push(*u);
                }
            }
            lp.uses = out;
        }
    }
}

/// Splits `edge` at its parameter midpoint, minting the midpoint
/// vertex and the two half-edges. Answers `(first, second, midpoint)`.
fn split_at_midpoint(
    solid: &mut SolidSpec,
    edge: u64,
    mint: &mut dyn FnMut() -> u64,
) -> (u64, u64, u64) {
    let spec = &solid.edges[&edge];
    let tm = (spec.t0 + spec.t1) / 2.0;
    split_at_param(solid, edge, tm, mint)
}

/// Splits `edge` at the interior carrier parameter `tm` (the caller
/// guarantees `t0 < tm < t1`), minting the new vertex and the two
/// halves. Answers `(first, second, new vertex)` — the
/// [`split_at_midpoint`] contract at an arbitrary parameter (M7-5:
/// the band mint splits rims at the seam azimuth, not the midpoint).
fn split_at_param(
    solid: &mut SolidSpec,
    edge: u64,
    tm: f64,
    mint: &mut dyn FnMut() -> u64,
) -> (u64, u64, u64) {
    let spec = &solid.edges[&edge];
    let (t0, t1, start, end) = (spec.t0, spec.t1, spec.start, spec.end);
    let carrier = spec.carrier.clone();
    let mid_v = mint();
    solid.vertices.insert(mid_v, carrier.eval(tm));
    let (first, second) = (mint(), mint());
    solid.edges.insert(
        first,
        EdgeSpec {
            start,
            end: mid_v,
            carrier: carrier.clone(),
            t0,
            t1: tm,
            // A minted edge: the importer states its own
            // start → end, so there is no sense to compose.
            reversed: false,
        },
    );
    solid.edges.insert(
        second,
        EdgeSpec {
            start: mid_v,
            end,
            carrier,
            t0: tm,
            t1,
            // A minted edge: the importer states its own
            // start → end, so there is no sense to compose.
            reversed: false,
        },
    );
    solid.edges.remove(&edge);
    (first, second, mid_v)
}

/// Whether traversing `carrier` in its increasing parameter raises the
/// chart coordinate selected by `pick` — the file's own winding, read
/// off the geometry rather than assumed from a flag.
///
/// Sampled a quarter and a fifth of the way along the interval: far
/// enough from both ends that the sample sits mid-quadrant, and clear
/// of the chart's own wrap (the seam at |u| = π and the inner equator
/// at |v| = π), where a finite difference would read a jump as a
/// direction. `None` when the samples do not invert onto the chart, or
/// when the step is too small to have a sign — an undecidable winding
/// is refused by the caller, never guessed.
fn chart_direction(
    surface: &Surface<f64>,
    carrier: &Curve3<f64>,
    t0: f64,
    t1: f64,
    pick: fn(geom_core::Point2<f64>) -> f64,
) -> Option<bool> {
    let span = t1 - t0;
    let at = |f: f64| crate::chart::uv_of(surface, carrier.eval(t0 + span * f)).map(pick);
    let (lo, hi) = (at(0.25)?, at(0.30)?);
    // The step covers 5% of a full period, i.e. ~0.31 rad; anything
    // materially smaller than that is not a reading.
    ((hi - lo).abs() > 0.1).then_some(hi > lo)
}

/// Runs the shell normalizations (module docs). `eps_in` is the
/// file's own declared uncertainty in kernel metres — the band mint's
/// budget for recognizing an existing rim vertex as already sitting
/// at the seam azimuth.
pub(crate) fn normalize_shell(
    solid: &mut SolidSpec,
    eps_in: f64,
    mint: &mut dyn FnMut() -> u64,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    band_seam(solid, eps_in, mint, sink)?;
    apex_cone(solid, mint, sink)?;
    full_torus(solid, mint, sink)?;
    wall_inversion(solid)
}

/// **The seamless periodic band** (M7-5): a cylinder/torus face whose
/// two bounds each wrap the chart's full u period with no seam
/// generator between them, tagged by detection at
/// [`crate::entities`]'s face gate. Re-minted as ONE single-loop face
/// whose loop walks one rim, the minted seam generator, the other
/// rim, and the generator again reversed — the seam edge used twice,
/// the shape a natively revolved wall carries.
///
/// The seam azimuth is the surface's own `u_ref` azimuth, ALWAYS
/// (spec D1): the seam chart image is defined spatially as the locus
/// in the closed u_ref half-plane and certification meters
/// SeamHalfplane/SeamSide against it, so re-charting `u_ref` to dodge
/// a split would mutate imported geometry beyond need. Where a rim
/// has no vertex at that azimuth, the rim is split there
/// ([`split_at_param`]) and the split propagates to EVERY face
/// sharing the rim ([`expand_split_uses`]) — load-bearing for shared
/// rims between adjacent bands.
///
/// The winding is derived, never assumed (spec D2, the module's
/// orientation doctrine): each rim's chart-u direction is read
/// through the kernel's own chart inverse and the two must OPPOSE
/// (every coherent band boundary's property). Against `same_sense`
/// they determine the face:
///
/// - **Cylinder** — `v` is not periodic, so exactly one region lies
///   between the rims and the winding is a pure cross-check: the CCW
///   boundary of the chart rectangle traverses its low-`v` rim in
///   `+u`, so a face whose derived winding disagrees with its stated
///   `same_sense` describes an inside-out solid and REFUSES typed
///   here, pre-body (the whole-torus posture; the kernel's tier-3
///   curved sense gate stays the backstop).
/// - **Torus** — `v` IS periodic, so the two rims bound TWO
///   complementary regions and the winding × sense pair is not a
///   redundant check but the file's only statement of WHICH: the
///   region runs v-increasing from the rim whose traversal is `+u`
///   (for `same_sense`) or `−u` (for a reversed face). Both ISO
///   encodings of one region decode identically; the encoding that
///   would be "inverted" is, face-locally, a legitimate statement of
///   the complementary region, so there is nothing left to refuse —
///   a file that misstates its region builds a body whose locus its
///   own neighbours contradict, which the kernel's tier-3 gates and
///   the oracle volume row catch. What DOES refuse here, typed, is
///   every unreadable winding: nothing is guessed.
///
/// The minted seam generator is the cylinder's `u_ref` ruling (a
/// Line segment) or the torus's `u_ref` meridian (a minor-circle
/// arc): NEW carriers, not sub-arcs of file-stated ones — the same
/// license as the edge-free sphere's minted meridians (module docs).
fn band_seam(
    solid: &mut SolidSpec,
    eps_in: f64,
    mint: &mut dyn FnMut() -> u64,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    for fi in 0..solid.faces.len() {
        if solid.faces[fi].band {
            mint_band(solid, fi, eps_in, mint, sink)?;
            solid.faces[fi].band = false;
        }
    }
    Ok(())
}

/// The vertex a use STARTS at.
fn use_start(solid: &SolidSpec, use_: EdgeUse) -> u64 {
    let spec = &solid.edges[&use_.edge];
    if use_.forward { spec.start } else { spec.end }
}

/// The boundary census of one face's loops as they stand: distinct
/// edges and distinct endpoint vertices.
fn face_boundary_census(solid: &SolidSpec, fi: usize) -> (usize, usize) {
    let mut edges = std::collections::BTreeSet::new();
    let mut vertices = std::collections::BTreeSet::new();
    for lp in &solid.faces[fi].loops {
        for u in &lp.uses {
            edges.insert(u.edge);
            let spec = &solid.edges[&u.edge];
            vertices.insert(spec.start);
            vertices.insert(spec.end);
        }
    }
    (edges.len(), vertices.len())
}

/// One tagged band face's re-mint ([`band_seam`] docs).
fn mint_band(
    solid: &mut SolidSpec,
    fi: usize,
    eps_in: f64,
    mint: &mut dyn FnMut() -> u64,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    let face_id = solid.faces[fi].id;
    let surface = solid.faces[fi].surface.clone();
    let sense = solid.faces[fi].sense;
    // Detection tagged only these two chart kinds (spec D3).
    let (s_axis, s_u_ref, axis_point, torus) = match surface {
        Surface::Cylinder {
            origin,
            axis,
            u_ref,
            ..
        } => (axis, u_ref, origin, None),
        Surface::Torus {
            axis,
            u_ref,
            center,
            major_radius,
            minor_radius,
        } => (
            axis,
            u_ref,
            center,
            Some((center, major_radius, minor_radius)),
        ),
        _ => {
            return Err(StepImportError::Topology {
                id: face_id,
                what: "internal: a face tagged as a periodic band on a chart the band \
                       detection does not recognize",
            });
        }
    };
    let (entry_edges, entry_vertices) = face_boundary_census(solid, fi);
    if solid.faces[fi].loops.len() != 2 {
        return Err(StepImportError::Topology {
            id: face_id,
            what: "internal: a face tagged as a periodic band without exactly two \
                   bounds",
        });
    }
    let undecidable = || StepImportError::Topology {
        id: face_id,
        what: "a seamless periodic band whose winding its own geometry cannot state \
               (a rim vertex that does not invert onto the chart, or coincident rim \
               levels): the orientation of the re-mint would be a guess, and D7 \
               forbids guessing it",
    };
    // ---- Each rim loop's derived chart-u direction and v level ----
    //
    // The direction read is STRUCTURAL, never sampled (R1 fix pass):
    // every rim carrier is required here to be a circle coaxial with
    // the surface within the file's own budget, and a coaxial circle
    // traversed in its increasing parameter raises the chart azimuth
    // exactly when its own axis points ALONG the surface axis — the
    // sign of n · axis, composed with the use's direction. A sampled
    // read (`chart_direction`) has two defects on this class the
    // review executed: a rim vertex in the (0.4π, 0.5π] azimuth
    // window puts the two samples astride the u = ±π wrap (direction
    // read INVERTED — a silent complement-region mint), and a rim
    // already split by a NEIGHBOUR band's mint can leave `uses[0]`
    // shorter than the sampler's step floor (a spurious undecidable
    // refusal on a valid solid). The structural read has neither
    // failure mode and is exact on the class the mint admits.
    //
    // The coaxiality that licenses it is GATED within the file's own
    // ε_in budget, not a loose classifier: a tilt moves rim points
    // off the iso-v circle by ~radius·|n × axis| and an off-axis
    // centre by its offset, so both are held to ε_in — a
    // Villarceau-tilted rim takes the typed band refusal here rather
    // than a generic certification message downstream.
    let mut rim = Vec::with_capacity(2);
    for li in 0..2 {
        let lp = &solid.faces[fi].loops[li];
        let mut raises = None;
        for (ui, u) in lp.uses.iter().enumerate() {
            let spec = &solid.edges[&u.edge];
            let coaxial = match spec.carrier {
                Curve3::Circle {
                    center,
                    axis: n,
                    radius,
                    ..
                } => {
                    let d = center - axis_point;
                    let off_axis = (d - s_axis * d.dot(s_axis)).norm();
                    let tilt = radius * n.cross(s_axis).norm();
                    if ui == 0 {
                        raises = Some(u.forward == (n.dot(s_axis) > 0.0));
                    }
                    tilt <= eps_in && off_axis <= eps_in
                }
                _ => false,
            };
            if !coaxial {
                return Err(StepImportError::Topology {
                    id: face_id,
                    what: "a seamless periodic band whose rim chain is not made of \
                           circles coaxial with the surface within the file's own \
                           interpretation budget — outside the band re-mint's \
                           minted class (its winding read, splits and seam endpoints \
                           are exact only on coaxial rims); extending the band \
                           re-mint to this rim shape is the recourse",
                });
            }
        }
        let raises = raises.ok_or_else(undecidable)?;
        let use0 = lp.uses[0];
        let p0 = solid.vertices[&use_start(solid, use0)];
        let v = crate::chart::uv_of(&surface, p0).ok_or_else(undecidable)?.y;
        rim.push((raises, v));
    }
    if rim[0].0 == rim[1].0 {
        return Err(StepImportError::Topology {
            id: face_id,
            what: "a seamless periodic band whose two rims traverse the chart's u in \
                   the SAME direction — no coherent band boundary does that, so the \
                   face's winding contradicts itself and the re-mint refuses rather \
                   than guessing",
        });
    }
    // The region's start rim: the loop the face's boundary traverses
    // in +u when `same_sense` (the CCW rectangle's low edge), in −u
    // when reversed — the region runs v-INCREASING from it (band_seam
    // docs; for the torus this SELECTS the v-interval, wrapping
    // allowed).
    let ls = usize::from(rim[0].0 != sense);
    let le = 1 - ls;
    if torus.is_none() {
        let (v_s, v_e) = (rim[ls].1, rim[le].1);
        if v_s == v_e {
            return Err(undecidable());
        }
        if v_s > v_e {
            return Err(StepImportError::Topology {
                id: face_id,
                what: "an ORIENTATION-INVERTED cylinder band face: its stated \
                       same_sense and the derived winding of its rims (their chart-u \
                       directions against their levels) disagree, so the solid it \
                       describes is inside out. The re-mint will not re-tessellate it \
                       right-side-out — that would launder the inversion — and the \
                       kernel's tier-3 curved sense gate (check 6) would refuse the \
                       built body; import returns certified bodies, so the refusal \
                       fires here, before any body exists",
            });
        }
    }
    // Note which rims already carry a vertex at the seam azimuth
    // BEFORE any surgery, then ensure both do.
    let mut splits = 0usize;
    let mut seam_v = [0u64; 2];
    for li in [ls, le] {
        seam_v[li] = seam_vertex_on_loop(solid, fi, li, &surface, eps_in, mint, &mut splits)?;
    }
    let (p_start, p_end) = (solid.vertices[&seam_v[ls]], solid.vertices[&seam_v[le]]);
    if (p_end - p_start).norm() <= eps_in {
        return Err(StepImportError::Topology {
            id: face_id,
            what: "a seamless periodic band whose two rims meet at the seam azimuth — \
                   the seam generator would be a point, so the band is degenerate and \
                   the re-mint refuses",
        });
    }
    // ---- The minted seam generator (band_seam docs) ---------------
    let gen_id = mint();
    let carrier = match torus {
        None => Curve3::Line {
            origin: p_start,
            dir: s_axis,
        },
        Some((center, major_radius, minor_radius)) => Curve3::Circle {
            center: crate::signed_zero::plus_zero_point(center + s_u_ref * major_radius),
            axis: crate::signed_zero::plus_zero(s_u_ref.cross(s_axis)),
            radius: minor_radius,
            u_ref: s_u_ref,
        },
    };
    let (t0, t1) = crate::geometry::endpoint_params(gen_id, &carrier, p_start, p_end, false)?;
    solid.edges.insert(
        gen_id,
        EdgeSpec {
            start: seam_v[ls],
            end: seam_v[le],
            carrier,
            t0,
            t1,
            // A minted edge: the importer states its own
            // start → end, so there is no sense to compose.
            reversed: false,
        },
    );
    // The mint's whole point is D1's spatial statement — this edge IS
    // the u_ref half-plane seam — so adoption must certify it as
    // the seam chart image or refuse; a silent downgrade to the
    // conventional mapped-curve rung would import a body whose "seam"
    // is off the half-plane (R1 fix pass, m2). Recorded here, enforced
    // in [`crate::adopt`].
    solid.band_seams.insert(gen_id);
    // ---- The single loop: [rim_s…, seam⁺, rim_e…, seam⁻] ----------
    //
    // The chart rectangle's boundary (CCW about the chart normal for
    // `same_sense`, its reversal otherwise): out of the start rim's
    // seam vertex round its whole period, up the seam generator, round
    // the end rim, and back down the generator — both rims keep their
    // FILE-stated uses verbatim (the derived winding above is what
    // certifies they already run the right way), merely rotated to
    // start at their seam vertex.
    let rotate = |solid: &SolidSpec, li: usize, at: u64| -> Result<Vec<EdgeUse>, StepImportError> {
        let uses = &solid.faces[fi].loops[li].uses;
        let k = uses.iter().position(|u| use_start(solid, *u) == at).ok_or(
            StepImportError::Topology {
                id: face_id,
                what: "internal: a band rim lost its seam vertex during the re-mint",
            },
        )?;
        let mut out = uses.clone();
        out.rotate_left(k);
        Ok(out)
    };
    let mut uses = rotate(solid, ls, seam_v[ls])?;
    uses.push(EdgeUse {
        edge: gen_id,
        forward: true,
    });
    uses.extend(rotate(solid, le, seam_v[le])?);
    uses.push(EdgeUse {
        edge: gen_id,
        forward: false,
    });
    solid.faces[fi].loops = vec![LoopSpec { outer: true, uses }];
    sink.push(StructureNormalization {
        face: face_id,
        kind: NormalizationKind::SeamlessPeriodicBand,
        file_census: FaceCensus {
            faces: 1,
            edges: entry_edges,
            vertices: entry_vertices,
        },
        // Each rim split replaces one edge by two (+1) and mints one
        // vertex; the seam generator is one more edge. The face count
        // is UNCHANGED — the band stays one face, now with a seam.
        kernel_census: FaceCensus {
            faces: 1,
            edges: entry_edges + splits + 1,
            vertices: entry_vertices + splits,
        },
    });
    Ok(())
}

/// The vertex of loop `li` of face `fi` at the surface's u_ref (seam)
/// azimuth — an existing rim vertex when one sits in the u_ref
/// half-plane within ε_in (spec D1: the file's own interpretation
/// budget), else minted by splitting the rim arc that crosses the
/// azimuth, propagating the split to every sharing face.
fn seam_vertex_on_loop(
    solid: &mut SolidSpec,
    fi: usize,
    li: usize,
    surface: &Surface<f64>,
    eps_in: f64,
    mint: &mut dyn FnMut() -> u64,
    splits: &mut usize,
) -> Result<u64, StepImportError> {
    use core::f64::consts::TAU;
    let face_id = solid.faces[fi].id;
    let s_u_ref = match *surface {
        Surface::Cylinder { u_ref, .. } | Surface::Torus { u_ref, .. } => u_ref,
        _ => {
            return Err(StepImportError::Topology {
                id: face_id,
                what: "internal: a band seam-vertex walk on a chart the band \
                       detection does not recognize",
            });
        }
    };
    // An existing vertex already in the half-plane: measured
    // SPATIALLY, as certification will meter it — the distance from
    // the vertex to the surface's own u = 0 locus at the vertex's v.
    for u in &solid.faces[fi].loops[li].uses {
        let vid = use_start(solid, *u);
        let p = solid.vertices[&vid];
        if let Some(uv) = crate::chart::uv_of(surface, p)
            && (surface.eval(0.0, uv.y) - p).norm() <= eps_in
        {
            return Ok(vid);
        }
    }
    // No vertex there: split the rim arc whose parameter interval
    // crosses the azimuth. The crossing point is EXACT — the rim is a
    // coaxial circle (the caller checked), so the u_ref half-plane
    // meets it at `rim center + rim radius · u_ref`, and the carrier
    // parameter of that point comes from the same angle convention as
    // [`crate::geometry::endpoint_params`].
    let mut crossing = None;
    for u in &solid.faces[fi].loops[li].uses {
        let spec = &solid.edges[&u.edge];
        let Curve3::Circle {
            center,
            axis: n,
            radius,
            u_ref: uc,
        } = spec.carrier
        else {
            // The caller checked every rim carrier is a coaxial
            // circle; anything else lands in the typed refusal below.
            continue;
        };
        let q = crate::signed_zero::plus_zero_point(center + s_u_ref * radius);
        let w = q - center;
        let vr = n.cross(uc);
        let mut t = w.dot(vr).atan2(w.dot(uc));
        while t <= spec.t0 {
            t += TAU;
        }
        if t < spec.t1 {
            crossing = Some((u.edge, t, q));
            break;
        }
    }
    if let Some((edge, t, q)) = crossing {
        let (first, second, mid_v) = split_at_param(solid, edge, t, mint);
        // The split vertex must be the EXACT crossing point `q` (rim
        // centre + radius · u_ref, no trigonometry), not the carrier's
        // trig evaluation at `t` — the evaluation carries up-to-a-ulp
        // dust off the half-plane, and the dust would shift the
        // re-derived parameters of a re-import (the writer round-trip
        // must be a fixed point). For the same reason both halves'
        // parameter intervals are re-derived from their endpoint
        // POSITIONS through the exact convention a re-import applies
        // (`geometry::endpoint_params`): a half whose minted interval
        // crossed the period boundary would otherwise carry a
        // parameter representative shifted by 2π from the one the
        // re-import derives — the same arc, but not bit-identical
        // under evaluation.
        solid.vertices.insert(mid_v, q);
        for half in [first, second] {
            let spec = &solid.edges[&half];
            let self_loop = spec.start == spec.end;
            let (ps, pe) = (solid.vertices[&spec.start], solid.vertices[&spec.end]);
            let (t0, t1) =
                crate::geometry::endpoint_params(half, &spec.carrier, ps, pe, self_loop)?;
            // The interval write is load-bearing (the comment above);
            // a lookup that could skip it would break the round-trip
            // fixed point silently. `half` cannot miss: both halves
            // were minted into `solid.edges` by `split_at_param`, and
            // nothing between that insert and this write removes an
            // edge — the same fact the read at the top of this loop
            // body already indexes on.
            let Some(spec) = solid.edges.get_mut(&half) else {
                unreachable!("a split half minted by `split_at_param` is in `solid.edges`")
            };
            spec.t0 = t0;
            spec.t1 = t1;
        }
        // Patch EVERY face — including this band's own loops (the rim
        // may be shared with the neighbouring face, or with another
        // band whose mint runs later).
        expand_split_uses(solid, edge, first, second, &[]);
        *splits += 1;
        return Ok(mid_v);
    }
    Err(StepImportError::Topology {
        id: face_id,
        what: "a seamless periodic band rim with no vertex at the seam azimuth and no \
               arc crossing it — the loop does not wrap the period it was detected \
               to, so the re-mint refuses rather than guessing",
    })
}

/// **The cylinder/cone wall orientation cross-check** (M6-6 rider, the
/// torus check's pattern, read-only): a lateral face bounded by two
/// rims whose stated `same_sense` disagrees with the winding of its
/// own loop describes an inside-out solid. Adoption would carry the
/// disagreement into a body verbatim (sense and traversal both copied)
/// and the kernel's tier-3 curved sense gate (check 6,
/// `CurvedSenseInverted`) would refuse that body — but import returns
/// certified bodies, so the honest surfacing is the whole-torus
/// refusal's: typed, PRE-BODY, with the orientation-inverted
/// diagnosis.
///
/// The winding is derived, never assumed, exactly as the torus reads
/// its fundamental polygon: each rim's chart-`u` direction is sampled
/// through the kernel's own chart inverse, and the CCW boundary of a
/// chart rectangle traverses its low-`v` rim in `+u` (the high-`v` one
/// in `−u`). `sense == ccw` is right-side-out; both ISO encodings
/// (`.T.` CCW / `.F.` CW) adopt, the two inverted ones refuse. Scope
/// is bounded to the shape the reading is exact on — a single-loop
/// cylinder/cone face with exactly two distinct rims, opposite rim
/// `u`-directions, distinct levels; anything else (one-rim caps,
/// unreadable charts, partial walls whose rims agree in direction) is
/// left for the kernel's own ladder — which catches the CIRCLE-rimmed
/// analytic class from below (executed on a three-rim wall). The
/// quadrature-owned conic-trimmed class is a RECORDED residual that
/// slips both layers: an ellipse-trimmed wall has no circle-rim pair
/// to read here, and the kernel arm exempts its boundary parse — such
/// flips import and certify green today (executed on the
/// tilted-section cylinder; closes with the ellipse-rim material-side
/// encoding).
fn wall_inversion(solid: &SolidSpec) -> Result<(), StepImportError> {
    for face in &solid.faces {
        let (Surface::Cylinder { axis, .. } | Surface::Cone { axis, .. }) = face.surface else {
            continue;
        };
        let [lp] = face.loops.as_slice() else {
            continue;
        };
        // The rims: circle uses whose carrier axis is the surface axis
        // (the torus check's `> 0.5` structural classifier).
        let mut rims: Vec<(EdgeUse, f64, bool)> = Vec::new(); // (use, v, raises u)
        let mut readable = true;
        for use_ in &lp.uses {
            let spec = &solid.edges[&use_.edge];
            let Curve3::Circle { axis: n_c, .. } = spec.carrier else {
                continue;
            };
            if n_c.dot(axis).abs() <= 0.5 {
                continue;
            }
            let Some(p) =
                chart_direction(&face.surface, &spec.carrier, spec.t0, spec.t1, |uv| uv.x)
            else {
                readable = false;
                break;
            };
            let Some(uv) = crate::chart::uv_of(&face.surface, spec.carrier.eval(spec.t0)) else {
                readable = false;
                break;
            };
            rims.push((*use_, uv.y, use_.forward == p));
        }
        let [(ra, va, ru_a), (rb, vb, ru_b)] = rims.as_slice() else {
            continue;
        };
        // Exact-shape gates: two DISTINCT rims at distinct levels,
        // traversed in opposite chart-u directions (every coherent
        // wall rectangle's property — a face outside it is not a clean
        // reading and is left to the kernel gate).
        if !readable || ra.edge == rb.edge || va == vb || ru_a == ru_b {
            continue;
        }
        let ccw = if va < vb { *ru_a } else { *ru_b };
        if face.sense != ccw {
            return Err(StepImportError::Topology {
                id: face.id,
                what: "an ORIENTATION-INVERTED cylinder/cone wall face: its stated same_sense \
                       and the winding of its own loop (the rims' chart directions against \
                       their levels) disagree, so the solid it describes is inside out. \
                       Adoption would copy both encodings verbatim and the kernel's tier-3 \
                       curved sense gate (check 6) would refuse the built body — import \
                       returns certified bodies, so the refusal fires here, before any \
                       body exists",
            });
        }
    }
    Ok(())
}

/// **The degenerate-apex cone** (module docs): a conical face whose
/// loop is `(seam, base circle, seam)` with the seam's far vertex of
/// valence 1. Re-minted as the kernel's own two lateral half-faces,
/// joined by a second generator half a turn round the axis.
///
/// Census: 1 lateral face / 2 edges / (apex + 1 base vertex) becomes 2
/// lateral faces / 4 edges / (apex + 2 base vertices). The base cap
/// face keeps its identity; only its single circular bound becomes two
/// half-circles.
fn apex_cone(
    solid: &mut SolidSpec,
    mint: &mut dyn FnMut() -> u64,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    let valence = valences(solid);
    // The one candidate shape, found by the defect it has: a valence-1
    // vertex. Anything else this scan meets is left alone.
    let Some((&apex_v, _)) = valence.iter().find(|&(_, &n)| n == 1) else {
        return Ok(());
    };
    let mut found = None;
    for (fi, face) in solid.faces.iter().enumerate() {
        let Surface::Cone { apex, axis, .. } = face.surface else {
            continue;
        };
        let [lp] = face.loops.as_slice() else {
            continue;
        };
        let [a, b, c] = lp.uses.as_slice() else {
            continue;
        };
        // (seam, circle, seam): the seam edge twice, the base circle
        // once, in either rotation of the cycle.
        let (seam, circle) = if a.edge == c.edge {
            (*a, *b)
        } else if a.edge == b.edge {
            (*a, *c)
        } else if b.edge == c.edge {
            (*b, *a)
        } else {
            continue;
        };
        let seam_spec = &solid.edges[&seam.edge];
        if (seam_spec.start != apex_v && seam_spec.end != apex_v)
            || !matches!(seam_spec.carrier, Curve3::Line { .. })
        {
            continue;
        }
        // The shape's second seam use traverses the seam the OTHER
        // way — a closed walk crosses its doubled edge once in each
        // direction. A loop that states the seam twice in the SAME
        // direction is not this shape (its walk does not close over
        // the seam), so detection declines rather than picking one
        // use twice and re-minting a guess.
        let Some(&seam_back) = lp
            .uses
            .iter()
            .find(|u| u.edge == seam.edge && u.forward != seam.forward)
        else {
            continue;
        };
        found = Some((fi, apex, axis, seam, circle, seam_back));
        break;
    }
    let Some((fi, apex, axis, seam_a, circle_use, seam_b)) = found else {
        // A valence-1 vertex that is NOT this shape is a defect the
        // kernel's own validity ladder must report, not something to
        // repair blind.
        return Ok(());
    };
    let face_id = fi;
    let (base_v, seam_carrier) = {
        let seam_spec = &solid.edges[&seam_a.edge];
        let base_v = if seam_spec.start == apex_v {
            seam_spec.end
        } else {
            seam_spec.start
        };
        (base_v, seam_spec.carrier.clone())
    };
    // ---- Orientation cross-check (M6-6 rider, the torus pattern) --
    //
    // The re-mint below copies the file's sense and traversal, so an
    // inverted description would survive it and build a body the
    // kernel's tier-3 curved sense gate (check 6, CurvedSenseInverted)
    // then refuses. Import returns certified bodies, so the honest
    // story is the torus's: refuse PRE-BODY, with the same
    // orientation-inverted diagnosis. The winding is derived, never
    // assumed: which way the base circle's traversal runs in chart `u`
    // (sampled through the kernel's own chart inverse), crossed with
    // which side of the apex (`v = 0`) the base rim sits on — the CCW
    // boundary of the lateral chart region traverses a `v > 0` rim in
    // `−u` and a `v < 0` rim in `+u`. `sense == ccw` is right-side-out
    // (both ISO encodings adopt); disagreement refuses. A reading the
    // geometry does not answer skips the check rather than guessing —
    // unlike the torus, this mint copies rather than re-orients, so an
    // unreadable winding leaves a body the kernel gate still judges
    // (true for this shape: a degenerate-apex cone's boundary is
    // circle-rimmed, squarely in the kernel arm's analytic class).
    {
        let face = &solid.faces[fi];
        let circle_spec = &solid.edges[&circle_use.edge];
        let p = chart_direction(
            &face.surface,
            &circle_spec.carrier,
            circle_spec.t0,
            circle_spec.t1,
            |uv| uv.x,
        );
        let v_b = crate::chart::uv_of(&face.surface, solid.vertices[&base_v]).map(|uv| uv.y);
        if let (Some(p), Some(v_b)) = (p, v_b)
            && v_b != 0.0
        {
            let ru = circle_use.forward == p;
            let ccw = ru != (v_b > 0.0);
            if face.sense != ccw {
                return Err(StepImportError::Topology {
                    id: face.id,
                    what: "an ORIENTATION-INVERTED degenerate-apex cone face: its stated \
                           same_sense and the winding of its own loop (base-rim chart \
                           direction against the rim's side of the apex) disagree, so \
                           the solid it describes is inside out. This importer will not \
                           re-tessellate it as stated — the kernel's tier-3 curved \
                           sense gate (check 6) refuses the inside-out face it would \
                           build — and import returns certified bodies, so the refusal \
                           fires here, before any body exists",
                });
            }
        }
    }
    let Some(second_seam) = half_turn_curve(&seam_carrier, apex, axis) else {
        return Ok(());
    };
    // The half of the base circle each new face carries.
    let (first_c, second_c, mid_v) = split_at_midpoint(solid, circle_use.edge, mint);
    // The minted generator, from the new base vertex to the apex.
    let gen_id = mint();
    let (t0, t1) = crate::geometry::endpoint_params(
        gen_id,
        &second_seam,
        solid.vertices[&mid_v],
        solid.vertices[&apex_v],
        false,
    )?;
    solid.edges.insert(
        gen_id,
        EdgeSpec {
            start: mid_v,
            end: apex_v,
            carrier: second_seam,
            t0,
            t1,
            // A minted edge: the importer states its own
            // start → end, so there is no sense to compose.
            reversed: false,
        },
    );
    // Every OTHER loop's use of the base circle becomes its two halves
    // (the cap face's single bound is the one that exists).
    expand_split_uses(solid, circle_use.edge, first_c, second_c, &[face_id]);

    // The two lateral half-faces. `seam_a` is the seam use that leaves
    // the apex (it precedes the circle in the cycle); `seam_b` is the
    // one that returns to it. Each new face walks out along one
    // generator, round its half of the base, and back along the other.
    let out_use = if solid_edge_leaves_apex(solid, seam_a, apex_v, base_v) {
        seam_a
    } else {
        seam_b
    };
    let in_use = EdgeUse {
        edge: out_use.edge,
        forward: !out_use.forward,
    };
    let template = &solid.faces[face_id];
    let (surface, sense, face_entity) = (template.surface.clone(), template.sense, template.id);
    let g = circle_use.forward;
    // Traversal order: under orientation `g` the base circle walks its
    // two halves in this order, which is the order the two new faces
    // take them in.
    let (h1, h2) = if g {
        (first_c, second_c)
    } else {
        (second_c, first_c)
    };
    let face_a = FaceSpec {
        id: face_entity,
        surface: surface.clone(),
        sense,
        loops: vec![LoopSpec {
            outer: true,
            uses: vec![
                out_use,
                EdgeUse {
                    edge: h1,
                    forward: g,
                },
                EdgeUse {
                    edge: gen_id,
                    forward: true,
                },
            ],
        }],
        band: false,
    };
    let face_b = FaceSpec {
        id: face_entity,
        surface,
        sense,
        loops: vec![LoopSpec {
            outer: true,
            uses: vec![
                EdgeUse {
                    edge: gen_id,
                    forward: false,
                },
                EdgeUse {
                    edge: h2,
                    forward: g,
                },
                in_use,
            ],
        }],
        band: false,
    };
    solid.faces[face_id] = face_a;
    solid.faces.insert(face_id + 1, face_b);
    sink.push(StructureNormalization {
        face: face_entity,
        kind: NormalizationKind::DegenerateApexCone,
        file_census: FaceCensus {
            faces: 1,
            edges: 2,
            vertices: 2,
        },
        kernel_census: FaceCensus {
            faces: 2,
            edges: 4,
            vertices: 3,
        },
    });
    Ok(())
}

/// Whether traversing `use_` walks from the apex towards the base.
fn solid_edge_leaves_apex(solid: &SolidSpec, use_: EdgeUse, apex_v: u64, _base_v: u64) -> bool {
    let spec = &solid.edges[&use_.edge];
    let from = if use_.forward { spec.start } else { spec.end };
    from == apex_v
}

/// **The whole torus in one face** (module docs): a toroidal face whose
/// single loop is the fundamental-polygon square — four uses over two
/// self-loop edges at one vertex. Re-minted as the kernel's own two
/// half-faces.
///
/// The split runs in **v**, the minor direction, because that is the
/// splitting a natively revolved torus carries: each half-face is a
/// full turn in u (its two rims are whole circles, one at the profile
/// vertex's latitude and one at the antipodal latitude) and half a turn
/// in v (the u-seam meridian, cut in two and used twice per face —
/// STEP's ordinary seam encoding). Splitting in u instead leaves each
/// half wrapping the full v-period, which is the same both-ways
/// ambiguity in the transpose.
///
/// Census: 1 face / 2 edges / 1 vertex becomes 2 / 4 / 2.
fn full_torus(
    solid: &mut SolidSpec,
    mint: &mut dyn FnMut() -> u64,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    let mut found = None;
    for (fi, face) in solid.faces.iter().enumerate() {
        let Surface::Torus { center, axis, .. } = face.surface else {
            continue;
        };
        let [lp] = face.loops.as_slice() else {
            continue;
        };
        let [a, b, c, d] = lp.uses.as_slice() else {
            continue;
        };
        // The fundamental polygon: A B A' B' with each edge used once
        // each way, both self-loops at the same vertex.
        if a.edge != c.edge || b.edge != d.edge || a.edge == b.edge {
            continue;
        }
        if a.forward == c.forward || b.forward == d.forward {
            continue;
        }
        let (ea, eb) = (&solid.edges[&a.edge], &solid.edges[&b.edge]);
        if ea.start != ea.end || eb.start != eb.end || ea.start != eb.start {
            continue;
        }
        // The RIM runs round the major direction: its own carrier axis
        // IS the torus axis. The other is the meridian, and cutting the
        // meridian is what halves the face.
        let axis_of = |c: &Curve3<f64>| match *c {
            Curve3::Circle { axis: n, .. } => Some(n),
            _ => None,
        };
        let (Some(na), Some(nb)) = (axis_of(&ea.carrier), axis_of(&eb.carrier)) else {
            continue;
        };
        let (rim, meridian, rim_axis) = if na.dot(axis).abs() > 0.5 && nb.dot(axis).abs() <= 0.5 {
            (*a, *b, na)
        } else if nb.dot(axis).abs() > 0.5 && na.dot(axis).abs() <= 0.5 {
            (*b, *a, nb)
        } else {
            continue;
        };
        found = Some((
            fi, face.id, face.sense, center, axis, rim, meridian, rim_axis,
        ));
        break;
    }
    let Some((fi, face_entity, sense, center, axis, rim, meridian, rim_axis)) = found else {
        return Ok(());
    };
    // ---- The file's winding, derived rather than assumed ----------
    //
    // The fundamental polygon `[x, y, x', y']` maps to ITSELF under
    // loop reversal — reversing the cycle and flipping every flag
    // permutes the four uses back into the same multiset. So the
    // per-use `forward` flags alone cannot say which way this face
    // turns; the winding lives ONLY in the cyclic ORDER, and a
    // reconstruction that reads the flags and not the order rebuilds an
    // inside-out torus as a right-side-out one. (That is exactly what
    // the first cut of this repair did, and why the order is now what
    // gets read.)
    //
    // `p` and `q` are geometric: which traversal of the rim raises u,
    // which traversal of the meridian raises v — sampled through the
    // kernel's own chart inverse. The chart's CCW boundary is, in
    // order, `(rim, +u) (meridian, +v) (rim, −u) (meridian, −v)`, so
    // the file's cycle is CCW exactly when the use FOLLOWING its `+u`
    // rim use is the meridian's `+v` one. Every reading that the
    // geometry does not answer refuses; none is guessed.
    let surface = solid.faces[fi].surface.clone();
    let undecidable = || StepImportError::Topology {
        id: face_entity,
        what: "a whole-torus face whose winding its own geometry cannot state (a rim or \
               meridian carrier that does not invert onto the torus chart, or a loop \
               whose uses do not alternate): the orientation of the re-tessellation \
               would be a guess, and D7 forbids guessing it",
    };
    let (p, q) = {
        let rim_spec = &solid.edges[&rim.edge];
        let p = chart_direction(
            &surface,
            &rim_spec.carrier,
            rim_spec.t0,
            rim_spec.t1,
            |uv| uv.x,
        )
        .ok_or_else(undecidable)?;
        let mer_spec = &solid.edges[&meridian.edge];
        let q = chart_direction(
            &surface,
            &mer_spec.carrier,
            mer_spec.t0,
            mer_spec.t1,
            |uv| uv.y,
        )
        .ok_or_else(undecidable)?;
        (p, q)
    };
    // The cyclic successor of the `+u` rim use.
    let ccw = {
        let uses = &solid.faces[fi].loops[0].uses;
        let plus_u = uses
            .iter()
            .position(|u| u.edge == rim.edge && u.forward == p)
            .ok_or_else(undecidable)?;
        let successor = uses[(plus_u + 1) % uses.len()];
        if successor.edge != meridian.edge {
            return Err(undecidable());
        }
        successor.forward == q
    };
    // The face's material side: the torus chart normal (∂u × ∂v) points
    // outward, so the face's own normal is that normal for
    // `same_sense` and its negation otherwise, and the loop must run
    // CCW about the FACE normal. Right-side-out is therefore
    // `sense == ccw`; the two consistent encodings of the same solid
    // are (`.T.`, CCW) and (`.F.`, CW), and both adopt.
    if sense != ccw {
        return Err(StepImportError::Topology {
            id: face_entity,
            what: "an ORIENTATION-INVERTED whole-torus face: its stated same_sense and the \
                   winding of its own fundamental-polygon loop disagree, so the solid it \
                   describes is inside out. This importer will not re-tessellate it \
                   right-side-out — that would launder the inversion into a positive \
                   volume — and it will not adopt it as stated: the kernel's tier-3 \
                   curved sense gate (check 6, CurvedSenseInverted) refuses an inside-out \
                   rim-bearing face, and import returns certified bodies, so the refusal \
                   fires here, before any body exists",
        });
    }

    // Halving the meridian mints the antipodal profile vertex.
    let (first_m, second_m, mid_v) = split_at_midpoint(solid, meridian.edge, mint);
    // The second rim: the u-circle through that vertex — same axis
    // direction as the stated rim (so u runs the same way), centred on
    // the torus axis at the vertex's own height, and anchored so the
    // vertex sits at its angle 0. Built from the vertex the file's own
    // meridian produced, so it cannot drift off the locus.
    let v1 = solid.vertices[&mid_v];
    let c1 = crate::signed_zero::plus_zero_point(center + axis * ((v1 - center).dot(axis)));
    let spoke = v1 - c1;
    let radius = spoke.norm();
    if !(radius.is_finite() && radius > 0.0) {
        return Ok(());
    }
    let rim_id = mint();
    let carrier = Curve3::Circle {
        center: c1,
        axis: rim_axis,
        radius,
        u_ref: crate::signed_zero::plus_zero(spoke * radius.recip()),
    };
    let Ok((t0, t1)) = crate::geometry::endpoint_params(rim_id, &carrier, v1, v1, true) else {
        return Ok(());
    };
    solid.edges.insert(
        rim_id,
        EdgeSpec {
            start: mid_v,
            end: mid_v,
            carrier,
            t0,
            t1,
            // A minted edge: the importer states its own
            // start → end, so there is no sense to compose.
            reversed: false,
        },
    );
    // The minted halves are wound `+u` first — CCW about the chart
    // normal, which for a torus points outward — and described with
    // `same_sense` true to match. Neither is copied from the file's
    // face, and both are DERIVED from the material side the check
    // above established:
    //
    // ISO 10303-42 lets one solid be stated two ways, `(.T., CCW)` and
    // `(.F., CW)`; they are the same material side and both reach here.
    // The kernel, however, derives a torus face's flux sign from its
    // TRAVERSAL alone (the tier-3 curved sense gate — M6-6 — now
    // cross-checks the bit against that traversal, but the flux still
    // reads the traversal), so a `(.F., CW)` re-tessellation would
    // come back with a negative volume for a right-side-out ring.
    // Minting the outward
    // convention explicitly is therefore the honest description of the
    // faces being minted, not a healing of the file's: the file's face
    // does not survive the re-tessellation, and its replacements get a
    // description that means what it says. The inverted encodings never
    // get here at all — they refused above.
    let rs = p;
    let sense = true;
    // Traversal order: under `q` the meridian walks its halves in this
    // order, so `h1` is the half touching the stated vertex and `h2`
    // the half touching the minted one.
    let (h1, h2) = if q {
        (first_m, second_m)
    } else {
        (second_m, first_m)
    };
    let u = |edge: u64, forward: bool| EdgeUse { edge, forward };
    // Each half-face is the chart rectangle between the two rims: the
    // stated rim below, its half of the meridian up one side and back
    // down the other (the seam, used twice), the minted rim above —
    // and the mirror image for the other half.
    let face_a = FaceSpec {
        id: face_entity,
        surface: surface.clone(),
        sense,
        loops: vec![LoopSpec {
            outer: true,
            uses: vec![u(rim.edge, rs), u(h1, q), u(rim_id, !rs), u(h1, !q)],
        }],
        band: false,
    };
    let face_b = FaceSpec {
        id: face_entity,
        surface,
        sense,
        loops: vec![LoopSpec {
            outer: true,
            uses: vec![u(rim_id, rs), u(h2, q), u(rim.edge, !rs), u(h2, !q)],
        }],
        band: false,
    };
    solid.faces[fi] = face_a;
    solid.faces.insert(fi + 1, face_b);
    sink.push(StructureNormalization {
        face: face_entity,
        kind: NormalizationKind::FullPeriodTorus,
        file_census: FaceCensus {
            faces: 1,
            edges: 2,
            vertices: 1,
        },
        kernel_census: FaceCensus {
            faces: 2,
            edges: 4,
            vertices: 2,
        },
    });
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use core::f64::consts::TAU;

    use super::*;

    /// A cone face whose loop states its seam twice in the SAME
    /// direction is not the degenerate-apex shape — that walk does not
    /// close over the seam — so detection declines and the shell is
    /// left, untouched, for the kernel's own validity ladder.
    #[test]
    fn apex_cone_declines_a_same_direction_doubled_seam() {
        let (apex_v, base_v) = (1, 2);
        let (seam_e, circle_e) = (10, 11);
        let apex = Point3::new(0.0, 0.0, 0.0);
        let base = Point3::new(1.0, 0.0, -1.0);
        let mut edges = BTreeMap::new();
        edges.insert(
            seam_e,
            EdgeSpec {
                start: apex_v,
                end: base_v,
                carrier: Curve3::Line {
                    origin: apex,
                    dir: Vec3::new(1.0, 0.0, -1.0) * 0.5_f64.sqrt(),
                },
                t0: 0.0,
                t1: 2.0_f64.sqrt(),
                reversed: false,
            },
        );
        edges.insert(
            circle_e,
            EdgeSpec {
                start: base_v,
                end: base_v,
                carrier: Curve3::Circle {
                    center: Point3::new(0.0, 0.0, -1.0),
                    axis: Vec3::new(0.0, 0.0, 1.0),
                    radius: 1.0,
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                },
                t0: 0.0,
                t1: TAU,
                reversed: false,
            },
        );
        let mut vertices = BTreeMap::new();
        vertices.insert(apex_v, apex);
        vertices.insert(base_v, base);
        let fwd = |edge| EdgeUse {
            edge,
            forward: true,
        };
        let mut solid = SolidSpec {
            id: 100,
            faces: vec![FaceSpec {
                id: 101,
                surface: Surface::Cone {
                    apex,
                    axis: Vec3::new(0.0, 0.0, -1.0),
                    half_angle: std::f64::consts::FRAC_PI_4,
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                },
                sense: true,
                loops: vec![LoopSpec {
                    outer: true,
                    uses: vec![fwd(seam_e), fwd(circle_e), fwd(seam_e)],
                }],
                band: false,
            }],
            edges,
            vertices,
            band_seams: Default::default(),
        };
        let mut minted = 0_u64;
        let mut sink = Vec::new();
        apex_cone(
            &mut solid,
            &mut || {
                minted += 1;
                1000 + minted
            },
            &mut sink,
        )
        .expect("a declined detection is not an error");
        assert_eq!(minted, 0, "a declined detection mints nothing");
        assert!(sink.is_empty(), "a declined detection reports nothing");
        assert_eq!(solid.faces.len(), 1, "the face is left as stated");
        assert_eq!(solid.edges.len(), 2, "no edge is split or minted");
    }
}
