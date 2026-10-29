//! **Reported structure normalizations** for periodic faces the kernel
//! cannot represent as stated (M7-2 Leg C; the sphere's edge-free face
//! is the sibling case, minted in [`crate::entities`] where the bound
//! is read).
//!
//! # Why a shell can arrive unrepresentable
//!
//! Open CASCADE never splits a periodic face: a full cylinder wall, a
//! full cone, and a whole torus each arrive as ONE `ADVANCED_FACE`
//! whose loop uses its wrap edges twice — when the exporter kept them
//! at all. Most of that is fine — the importer's manifold precondition
//! and the wrap-edge adoption rung were built for exactly that shape,
//! and the cylinder, the truncated cone and the whole torus (one face
//! cut once each way, D1) assemble and certify as stated. Two shapes
//! do not:
//!
//! - **A chart singularity that is not a vertex.** A sphere's pole or a
//!   cone's apex inside a face, inside one of its edges, or at the end
//!   of a seam that is its only edge (Open CASCADE's full cone and its
//!   sphere cap). A chart singularity inside a face is a vertex of it,
//!   in the pole form ([`geom::pole_form`]); the valence-1 tip is
//!   also what `topo`'s tier 2 calls construction scaffolding
//!   (`ScaffoldingStrutVertex`). The [`singularity`] pass mints it.
//! - **The seamless band** (M7-5). A cylinder or torus lateral face
//!   stated as its two full-period rim bounds with NO seam generator
//!   at all. The kernel's face model is one outer loop plus rings,
//!   and a ring that winds the chart's period is a loop whose chart
//!   walk does not close on its start (`topo::loop_windings`), so
//!   adopting the second rim as a ring would hand back a face that
//!   does not lift to a disc of its chart. Detection tags
//!   the shape at the face gate ([`crate::entities`]); [`band_seam`]
//!   re-mints it here.
//!
//! # What this module does about it
//!
//! The same D7 stage-3 repair the edge-free sphere takes: the LOCUS is
//! fully explained by the file's own surfaces and carriers and is
//! adopted unchanged — only the boundary-graph tessellation is
//! re-minted, into the splitting a natively built body carries (the
//! kernel's own cone is two lateral half-faces meeting at the apex on
//! two half-turn generators; its own revolved wall is ONE face whose
//! loop uses its wrap edge twice). The census mapping
//! is carried out as [`crate::StructureNormalization`] data.
//!
//! What a mint may state that the file did not is bounded and named:
//! a singularity's minted curves are each a sub-arc of a carrier the
//! file states, that carrier's half-turn rotation about the face's own
//! axis, or the surface's own meridian at its `u_ref` azimuth or its
//! half turn; the band's seam generator is a
//! NEW carrier — the surface's own `u_ref` ruling (cylinder) or
//! `u_ref` meridian arc (torus), the same license under which the
//! edge-free sphere's re-mint ([`crate::entities`]) states its two
//! meridian circles from nothing but the surface's own fields. In
//! every case the minted curve lies ON the file's stated surface by
//! construction and is certified there by the same adoption gates as
//! any file-stated edge; no minted curve is a new shape.
//!
//! Anything that does not match these shapes exactly is left alone —
//! this is a bounded repair of named cases, not a licence to
//! re-tessellate.
//!
//! # Orientation is derived, never minted
//!
//! A re-tessellation decides how its new faces are wound, which makes
//! it the exact place an inside-out solid could be quietly turned
//! right-side-out. So a re-mint reads the file's winding — against `u`
//! and `v` directions sampled through the kernel's own chart inverse —
//! and cross-checks it against the face's `same_sense` before it mints
//! anything: the valence-1 tip inside [`singularity`], and —
//! read-only — two-rim cylinder/cone walls ([`wall_inversion`]). The
//! kernel's tier-3 curved sense gate (check 6, `CurvedSenseInverted`)
//! refuses the inside-out body such a file would build, and import
//! returns certified bodies, so the orientation-inverted diagnosis
//! fires HERE, pre-body, typed. A face adopted as stated, the whole
//! torus included, is read by that gate on the body: its chart Green
//! form reads the winding off the loop's cyclic order, which is where
//! a fundamental polygon's winding lives.
//!
//! [`band_seam`] (M7-5) reads its winding the same way — each rim's
//! chart-u direction against `same_sense` — before it re-mints: an
//! inverted cylinder band refuses typed pre-body, and a torus band's
//! reading selects which of the two v-intervals between its rims the
//! face covers (its docs say why that is the honest maximum there).

use std::collections::BTreeMap;

use geom::Curve3;
use geom::Surface;

use crate::entities::{EdgeSpec, EdgeUse, LoopSpec, SolidSpec};
use crate::error::StepImportError;
use crate::{FaceCensus, NormalizationKind, StructureNormalization};

mod singularity;

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

/// Splits `edge` at the interior carrier parameter `tm` (the caller
/// guarantees `t0 < tm < t1`), minting the new vertex and the two
/// halves. Answers `(first, second, new vertex)`, at an arbitrary
/// parameter (M7-5: the band mint splits rims at the seam azimuth, not
/// the midpoint).
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
/// chart u — the file's own winding, read off the geometry rather than
/// assumed from a flag.
///
/// Sampled a quarter and three tenths of the way along the interval,
/// far enough from both ends that the samples sit mid-quadrant, and the
/// step between them read modulo a turn, so the window's cut falling
/// between them is no direction. `None` when the samples do not invert
/// onto the chart, or when the step is too small to have a sign — an
/// undecidable winding is refused by the caller, never guessed.
fn chart_direction(
    surface: &Surface<f64>,
    carrier: &Curve3<f64>,
    t0: f64,
    t1: f64,
    chart_band: Option<geom_core::Band>,
) -> Option<bool> {
    let span = t1 - t0;
    let at = |f: f64| {
        crate::chart::uv_of(surface, carrier.eval(t0 + span * f), chart_band).map(|uv| uv.x)
    };
    let (lo, hi) = (at(0.25)?, at(0.30)?);
    // The step, brought into (−π, π] so a window's cut between the two
    // samples reads as the step it is. It covers 5% of a full period,
    // i.e. ~0.31 rad; anything materially smaller than that is not a
    // reading.
    let (sin, cos) = (hi - lo).sin_cos();
    let step = sin.atan2(cos);
    (step.abs() > 0.1).then_some(step > 0.0)
}

/// Runs the shell normalizations (module docs). `eps_in` is the
/// file's own declared uncertainty in kernel metres — the band mint's
/// budget for recognizing an existing rim vertex as already sitting
/// at the seam azimuth. `run_band` is the run's own linear band, which
/// the singularity pass reads a pole or apex against beside the file's.
pub(crate) fn normalize_shell(
    solid: &mut SolidSpec,
    eps_in: f64,
    chart_band: Option<geom_core::Band>,
    run_band: Option<geom_core::Band>,
    mint: &mut dyn FnMut() -> u64,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    crate::knot_split::split_c0_knots(solid, eps_in, mint, sink)?;
    band_seam(solid, eps_in, chart_band, mint, sink)?;
    // The walls' orientation is read first: an inverted cone wall holds
    // its apex on the side its bit claims, and is refused as inverted,
    // not cut.
    wall_inversion(solid, chart_band)?;
    singularity::singularity_vertices(solid, eps_in, chart_band, run_band, mint, sink)
}

/// **The seamless periodic band** (M7-5): a cylinder/torus face whose
/// two bounds each wrap the chart's full u period with no seam
/// generator between them, tagged by detection at
/// [`crate::entities`]'s face gate. Re-minted as ONE single-loop face
/// whose loop walks one rim, the minted seam generator, the other
/// rim, and the generator again reversed — the seam edge used twice,
/// the shape a natively revolved wall carries.
///
/// The seam azimuth is the surface's own `u_ref` azimuth: a wrap
/// edge may sit at any azimuth (D1), and cutting at `u_ref` keeps the
/// imported surface as stated, where re-charting `u_ref` to dodge a
/// split would mutate imported geometry beyond need. Where a rim
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
    chart_band: Option<geom_core::Band>,
    mint: &mut dyn FnMut() -> u64,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    for fi in 0..solid.faces.len() {
        if solid.faces[fi].band {
            mint_band(solid, fi, eps_in, chart_band, mint, sink)?;
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
    chart_band: Option<geom_core::Band>,
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
        let v = crate::chart::uv_of(&surface, p0, chart_band)
            .ok_or_else(undecidable)?
            .y;
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
        seam_v[li] = seam_vertex_on_loop(
            solid,
            fi,
            li,
            &surface,
            eps_in,
            chart_band,
            mint,
            &mut splits,
        )?;
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
    // The mint's whole point is D1's statement that this edge is the
    // band face's wrap edge, so adoption must certify it as one or
    // refuse; it is not offered the conventional mapped-curve rung.
    // Recorded here, enforced in [`crate::adopt`].
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
#[allow(clippy::too_many_arguments)] // one parameter per named quantity
fn seam_vertex_on_loop(
    solid: &mut SolidSpec,
    fi: usize,
    li: usize,
    surface: &Surface<f64>,
    eps_in: f64,
    chart_band: Option<geom_core::Band>,
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
        if let Some(uv) = crate::chart::uv_of(surface, p, chart_band)
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
            // ONE lookup serves both the reads and the load-bearing
            // interval write (the comment above) — `edges` and
            // `vertices` are disjoint fields, so the vertex reads
            // borrow beside the spec. The half cannot miss: both were
            // minted into `solid.edges` by `split_at_param` above.
            let Some(spec) = solid.edges.get_mut(&half) else {
                unreachable!("a split half minted by `split_at_param` is in `solid.edges`")
            };
            let self_loop = spec.start == spec.end;
            let (ps, pe) = (solid.vertices[&spec.start], solid.vertices[&spec.end]);
            let (t0, t1) =
                crate::geometry::endpoint_params(half, &spec.carrier, ps, pe, self_loop)?;
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
/// quadrature-owned conic-trimmed class is a RECORDED residual of both
/// orientation readings: an ellipse-trimmed wall has no circle-rim pair
/// to read here, and the kernel's curved arm exempts its boundary
/// parse. Tier 3 still refuses its flips through other arms — a lone
/// flip by the material wedge or check 7 and the pinch corners, a
/// whole-body inversion at the planar faces (`step-export`'s
/// `m6_6_sense_gate` rows); the readings themselves close with the
/// ellipse-rim material-side encoding.
fn wall_inversion(
    solid: &SolidSpec,
    chart_band: Option<geom_core::Band>,
) -> Result<(), StepImportError> {
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
                chart_direction(&face.surface, &spec.carrier, spec.t0, spec.t1, chart_band)
            else {
                readable = false;
                break;
            };
            let Some(uv) =
                crate::chart::uv_of(&face.surface, spec.carrier.eval(spec.t0), chart_band)
            else {
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use core::f64::consts::TAU;

    use super::*;
    use crate::entities::FaceSpec;
    use geom_core::{Point3, Vec3};

    /// A cone face whose loop states its seam twice in the SAME
    /// direction does not end that seam at a strut tip — the walk does
    /// not close over the seam — so the singularity pass declines and
    /// leaves the shell untouched. The import pipeline never presents this shape
    /// (the shell's manifold precondition refuses a twice-forward
    /// edge before normalize runs); the decline guards direct
    /// constructors of a `SolidSpec`.
    #[test]
    fn the_singularity_pass_declines_a_same_direction_doubled_seam() {
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
        singularity::singularity_vertices(
            &mut solid,
            1e-9,
            crate::chart::file_band(1e-9, geom_core::Tol::witness()),
            geom_core::Band::linear(geom_core::Tol::witness()).ok(),
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
