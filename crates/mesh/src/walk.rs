//! Boundary-loop walking: traversal extraction, chart inversion, and
//! periodic unwrapping — turning a curved face's loop into a UV polygon
//! whose sides are exactly straight (bitwise-constant u or v).
//!
//! Every curved M2 face is a swept UV rectangle whose boundary edges
//! are iso-curves (extrude/revolve structure, PR 4/5): **rims** (circles
//! around the surface axis, v = const) and **meridians** (u = const:
//! struts/generators/profile copies/`Seam` edges). The walk classifies
//! each traversal structurally, assigns the constant coordinate once
//! per edge (never per point — so rectangle sides are bitwise straight
//! and the CDT sees no sliver-generating wobble), and unwraps the
//! periodic coordinate(s) by continuity (chord steps ≤ π/4 make branch
//! choice unambiguous away from poles). One structural exception: a
//! rim-anchored loop's **final** meridian contains the loop's closing
//! vertex, so its column takes the branch nearest the first polygon
//! entry (`out[0].u`) rather than continuity — the right BRANCH by
//! construction for every wedge angle (continuity would pick the wrong
//! one for θ > 3π/2, where the complement 2π − θ < π/2 is closer).
//!
//! Branch choice being exact does not make the closure exact to the
//! last bit, and the difference took a defect to notice: the branch is
//! discrete, while the residue within it is a float quantity that
//! imported geometry can make nonzero. Measured, it usually IS zero —
//! 266 of 315 closures across the tour and the wild corpus close
//! bitwise, and the tour's worst is 9 ulps — but see `loop_polygon`'s
//! closure block for the exception class, its traced cause, and why
//! the snap bar is a length rather than an angle.
//!
//! Pole handling (chart singularities; the surface's `normal` is never
//! sampled): a pole/apex is always an edge **endpoint** (valence 2). A
//! pole junction between two meridians emits *two* polygon entries —
//! one closing the incoming meridian's column, one opening the
//! outgoing column — both mapping to the single pole mesh vertex; the
//! collapsed side between them becomes the fan (see the curved-face module).
//! Two documented value-level resolutions (display-layer, backstopped
//! by the per-triangle certificates and the mesh validator):
//!
//! - **Tie at a pole junction** (the two meridians sit exactly π
//!   apart, e.g. a wire-case cone band): the unwrap candidates are
//!   equidistant; the branch nearest the polygon's first entry wins
//!   (closure consistency).
//! - **Pole-to-pole bands** (no rim in the loop — the sphere bands of
//!   a ball): continuity gives no anchor at all, so the loop's 3-D
//!   area vector disambiguates: it points into the face's azimuth
//!   half (interior-left + the face's OUTWARD normal; verified
//!   derivation in the PR log), and each meridian takes the branch
//!   nearest `atan2(A·v_ref, A·u_ref)`.
//!
//!   That derivation is stated in the outward frame while `u_ref` /
//!   `v_ref` live in the surface's CHART frame, so since M5 S10 the
//!   area vector is multiplied by the face's `sense_sign` before the
//!   `atan2` — the one orientation-sense read in this crate. The old
//!   "assumes outward-oriented shells (true of every M2 body)" caveat
//!   is thereby discharged rather than restated: a reversed face
//!   stores its loop the other way round, so `A` flips and an
//!   unmultiplied azimuth would be π off, selecting the complementary
//!   branch and meshing the wrong half of the sphere. This is the
//!   direct analogue of `geom_brep::props::curved`'s rimless-sphere
//!   `s_f` — the same face kind, the same missing bit, the same fix.
//!   Every face this build mints has `sense: true`, so the multiply is
//!   `· 1.0` and bitwise inert today.

use std::collections::HashMap;

use geom_brep::EdgeGeometry;
use geom_core::{Point3, Vec3};
use geom_curves::Curve3;
use geom_surfaces::Surface;
use topo::{Body, EdgeKey, FaceKey, LoopBoundary, LoopKey};

use crate::types::TessellateError;

/// A curved surface's chart data for inversion (everything but the
/// plane, which takes the planar path).
pub(crate) struct Chart {
    /// The revolution/extrusion axis.
    pub axis: Vec3<f64>,
    /// The seam direction (u = 0).
    pub u_ref: Vec3<f64>,
    /// `axis × u_ref` (the frame convention).
    pub v_ref: Vec3<f64>,
    /// A point on the axis (apex/center/origin).
    pub anchor: Point3<f64>,
    /// Kind-specific inversion data.
    pub kind: ChartKind,
}

/// The kind payload of a [`Chart`].
pub(crate) enum ChartKind {
    /// Cylinder of radius `r`; v = axial meters.
    Cylinder {
        /// The radius.
        r: f64,
    },
    /// Cone; v = slant meters from the apex.
    Cone {
        /// The half-angle α.
        half_angle: f64,
    },
    /// Sphere of radius `r`; v = latitude.
    Sphere {
        /// The radius.
        r: f64,
    },
    /// Torus; v = minor angle (periodic).
    Torus {
        /// The major radius R.
        major: f64,
        /// The minor radius r.
        minor: f64,
    },
}

impl Chart {
    /// Builds the chart for a curved surface (`None` for planes;
    /// `Nurbs` is refused upstream).
    pub(crate) fn of(surface: &Surface<f64>) -> Option<Chart> {
        match *surface {
            Surface::Plane { .. } | Surface::Nurbs(_) => None,
            Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => Some(Chart {
                axis,
                u_ref,
                v_ref: axis.cross(u_ref),
                anchor: origin,
                kind: ChartKind::Cylinder { r: radius },
            }),
            Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => Some(Chart {
                axis,
                u_ref,
                v_ref: axis.cross(u_ref),
                anchor: apex,
                kind: ChartKind::Cone { half_angle },
            }),
            Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            } => Some(Chart {
                axis,
                u_ref,
                v_ref: axis.cross(u_ref),
                anchor: center,
                kind: ChartKind::Sphere { r: radius },
            }),
            Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            } => Some(Chart {
                axis,
                u_ref,
                v_ref: axis.cross(u_ref),
                anchor: center,
                kind: ChartKind::Torus {
                    major: major_radius,
                    minor: minor_radius,
                },
            }),
        }
    }

    /// Raw azimuth of a point, in (−π, π].
    pub(crate) fn azimuth(&self, p: Point3<f64>) -> f64 {
        let w = p - self.anchor;
        w.dot(self.v_ref).atan2(w.dot(self.u_ref))
    }

    /// Distance of a point from the chart axis — the lever arm that
    /// converts an angular u-discrepancy into a spatial one (arc
    /// length `r·δu`). `(u_ref, v_ref, axis)` is orthonormal by
    /// construction (`v_ref = axis × u_ref`), so this is just the
    /// in-plane component's length.
    ///
    /// Kind-free on purpose: it reads the POINT, not the surface
    /// parameters, so the cone — whose radius varies along the loop as
    /// `v·sin α` rather than sitting in `ChartKind` — needs no special
    /// case, and a point on the axis correctly reports 0.
    pub(crate) fn radial(&self, p: Point3<f64>) -> f64 {
        let w = p - self.anchor;
        w.dot(self.u_ref).hypot(w.dot(self.v_ref))
    }

    /// Raw chart u of a point, in (−π, π] — the azimuth, **except on a
    /// cone's mirror nappe** (v < 0, i.e. below the apex along the
    /// axis), where `S(u, v) = apex + axis·(v cos α) + radial(u)·(v sin α)`
    /// has a *negative* radial coefficient: the point at chart u sits
    /// at spatial azimuth u + π, so inversion subtracts the π back.
    /// (Revolve places every surface with the shared +a₃ axis, so
    /// downward-opening cone walls live on the mirror nappe — PR 5.)
    pub(crate) fn u_of(&self, p: Point3<f64>) -> f64 {
        let az = self.azimuth(p);
        if matches!(self.kind, ChartKind::Cone { .. }) && (p - self.anchor).dot(self.axis) < 0.0 {
            if az > 0.0 {
                az - core::f64::consts::PI
            } else {
                az + core::f64::consts::PI
            }
        } else {
            az
        }
    }

    /// The non-periodic-or-raw v coordinate of a point.
    pub(crate) fn v_of(&self, p: Point3<f64>) -> f64 {
        let w = p - self.anchor;
        let h = w.dot(self.axis);
        match self.kind {
            ChartKind::Cylinder { .. } => h,
            ChartKind::Cone { half_angle } => h / half_angle.cos(),
            ChartKind::Sphere { r } => (h / r).clamp(-1.0, 1.0).asin(),
            ChartKind::Torus { major, .. } => {
                let rho = (w - self.axis * h).norm();
                h.atan2(rho - major)
            }
        }
    }

    /// Whether v is a periodic coordinate (torus minor angle).
    pub(crate) fn v_periodic(&self) -> bool {
        matches!(self.kind, ChartKind::Torus { .. })
    }

    /// The chart's pole points with their v values (sphere poles, cone
    /// apex; empty otherwise).
    pub(crate) fn poles(&self) -> Vec<(Point3<f64>, f64)> {
        match self.kind {
            ChartKind::Sphere { r } => vec![
                (self.anchor + self.axis * r, core::f64::consts::FRAC_PI_2),
                (self.anchor - self.axis * r, -core::f64::consts::FRAC_PI_2),
            ],
            ChartKind::Cone { .. } => vec![(self.anchor, 0.0)],
            ChartKind::Cylinder { .. } | ChartKind::Torus { .. } => Vec::new(),
        }
    }

    /// The (raw) v of a rim from its carrier circle's center and
    /// radius — assigned once per edge so rim rows are bitwise
    /// straight.
    pub(crate) fn rim_v(&self, center: Point3<f64>, radius: f64) -> f64 {
        let h = (center - self.anchor).dot(self.axis);
        match self.kind {
            ChartKind::Cylinder { .. } => h,
            ChartKind::Cone { half_angle } => h / half_angle.cos(),
            ChartKind::Sphere { .. } => h.atan2(radius),
            ChartKind::Torus { major, .. } => h.atan2(radius - major),
        }
    }
}

/// One directed boundary traversal: an edge's chord ids in the loop's
/// walking direction, plus its iso classification.
pub(crate) struct Trav {
    /// Chord ids, traversal order (endpoints included).
    pub ids: Vec<u32>,
    /// Rim (`v = const`) or meridian (`u = const`) data.
    pub kind: TravKind,
}

/// The iso classification of a boundary edge on a curved face.
pub(crate) enum TravKind {
    /// A circle around the surface axis; carries the raw row v.
    Rim {
        /// Raw v (unwrapped at emission for periodic v).
        v_raw: f64,
    },
    /// A u = const boundary; carries the raw column azimuth.
    Meridian {
        /// Raw u ∈ (−π, π] (exactly 0.0 for `Seam` edges).
        u_raw: f64,
    },
}

/// The loop's half-edge traversal list `(edge, forward)` in `next`
/// order.
pub(crate) fn loop_edges(
    body: &Body<f64>,
    lk: LoopKey,
    face: FaceKey,
) -> Result<Vec<(EdgeKey, bool)>, TessellateError> {
    let lp = body
        .get_loop(lk)
        .ok_or(TessellateError::MissingEntity { what: "loop" })?;
    let LoopBoundary::Cycle { first } = lp.boundary else {
        return Err(TessellateError::EmptyLoop { face });
    };
    let cycle = body
        .loop_cycle(first)
        .ok_or(TessellateError::MissingEntity { what: "loop cycle" })?;
    let mut out = Vec::with_capacity(cycle.len());
    for hek in cycle {
        let he = body
            .get_half_edge(hek)
            .ok_or(TessellateError::MissingEntity { what: "half-edge" })?;
        let edge = body
            .get_edge(he.edge)
            .ok_or(TessellateError::MissingEntity { what: "edge" })?;
        out.push((he.edge, edge.he_plus == hek));
    }
    Ok(out)
}

/// Classifies and directs every traversal of a curved face's loop.
pub(crate) fn traversals(
    body: &Body<f64>,
    chart: &Chart,
    chords: &HashMap<EdgeKey, Vec<u32>>,
    face: FaceKey,
    lk: LoopKey,
) -> Result<Vec<Trav>, TessellateError> {
    let mut out = Vec::new();
    for (ek, forward) in loop_edges(body, lk, face)? {
        let edge = body
            .get_edge(ek)
            .ok_or(TessellateError::MissingEntity { what: "edge" })?;
        let curve = body
            .get_curve_geom(edge.curve)
            .ok_or(TessellateError::MissingEntity { what: "edge curve" })?
            .certified()
            .ok_or(TessellateError::NullScaffoldEdge { edge: ek })?;
        let mut ids = chords
            .get(&ek)
            .ok_or(TessellateError::MissingEntity {
                what: "edge chords",
            })?
            .clone();
        if !forward {
            ids.reverse();
        }
        let kind = classify(chart, curve, ek)?;
        out.push(Trav { ids, kind });
    }
    Ok(out)
}

/// Rim-vs-meridian classification (module docs): `Seam` descriptions
/// and line carriers are meridians; circle carriers split on axis
/// alignment (structurally either parallel — a rim — or perpendicular
/// — a meridian; 0.5 splits the two classes with maximal margin).
///
/// A meridian's column u always comes from the mid-point chart
/// inversion — **never** from the edge kind: a `Seam` edge is the
/// surface's `u_ref`-half-plane meridian, whose chart u is 0 on
/// ordinary kinds but π on a cone's mirror nappe (the kernel defines
/// the seam spatially via `u_ref`; `u_of` carries the nappe
/// correction).
fn classify(
    chart: &Chart,
    curve: &geom_brep::EdgeCurve<f64>,
    _ek: EdgeKey,
) -> Result<TravKind, TessellateError> {
    if matches!(curve.description(), EdgeGeometry::Seam { .. }) {
        return Ok(TravKind::Meridian {
            u_raw: mid_azimuth(chart, curve),
        });
    }
    match *curve.carrier() {
        Curve3::Line { .. } => Ok(TravKind::Meridian {
            u_raw: mid_azimuth(chart, curve),
        }),
        Curve3::Circle {
            center,
            axis,
            radius,
            ..
        } => {
            if axis.dot(chart.axis).abs() > 0.5 {
                Ok(TravKind::Rim {
                    v_raw: chart.rim_v(center, radius),
                })
            } else {
                Ok(TravKind::Meridian {
                    u_raw: mid_azimuth(chart, curve),
                })
            }
        }
        // RETIRED refusal (M5 PR 11): a conic/B-spline trim carrier
        // routes the whole face to the pcurve-driven trimmed lane
        // BEFORE this walk runs (`crate::trimmed::has_trim_carrier`),
        // so these arms are the router's backstop, not a frontier —
        // reaching one is a dispatch defect, surfaced typed.
        Curve3::Ellipse { .. } | Curve3::Nurbs(_) => Err(TessellateError::MissingEntity {
            what: "non-iso trim carrier reached the iso-rectangle walk (router defect)",
        }),
    }
}

/// The azimuth of the edge's mid-parameter carrier point — a
/// representative interior point, never an apex/pole endpoint.
fn mid_azimuth(chart: &Chart, curve: &geom_brep::EdgeCurve<f64>) -> f64 {
    let (t0, t1) = curve.params();
    chart.u_of(curve.carrier().eval(t0 + (t1 - t0) * 0.5))
}

/// One UV polygon entry of a curved face's boundary walk.
#[derive(Clone, Copy, Debug)]
pub(crate) struct UvPoint {
    /// Unwrapped azimuth.
    pub u: f64,
    /// (Unwrapped, for torus) v.
    pub v: f64,
    /// The mesh vertex this UV location maps to (several entries may
    /// share an id: seam double-traversals, pole corners).
    pub id: u32,
    /// Whether this entry is a pole/apex corner.
    pub pole: bool,
}

const TAU: f64 = core::f64::consts::TAU;

/// `raw + 2πk` nearest `prev`.
fn unwrap_near(raw: f64, prev: f64) -> f64 {
    raw + TAU * ((prev - raw) / TAU).round()
}

/// [`unwrap_near`] with half-period tie resolution toward `anchor`
/// (module docs: the wire-band pole junction, meridians exactly π
/// apart).
fn unwrap_tie(raw: f64, prev: f64, anchor: f64) -> f64 {
    let k = (prev - raw) / TAU;
    if (k - k.round()).abs() < 0.25 {
        raw + TAU * k.round()
    } else {
        let c1 = raw + TAU * k.floor();
        let c2 = raw + TAU * k.ceil();
        if (c1 - anchor).abs() <= (c2 - anchor).abs() {
            c1
        } else {
            c2
        }
    }
}

/// Walks a curved face's loop into its UV polygon (module docs: the
/// classification, unwrapping, pole, and disambiguation rules).
/// The loop-closure bar: may a residue of `residue` radians at
/// `radius` metres from the chart axis be snapped onto its column?
///
/// One predicate so the assertion and the snap cannot drift apart.
/// They did, in the form this replaces: both read a bare `residue <
/// 1e-9`, so exceeding it disabled BOTH — the debug build screamed
/// while release silently declined to snap and shipped the unsnapped
/// polygon on.
///
/// `residue * radius` is the arc length the snap moves the polygon
/// side by, and `eps` is the length this module already measures
/// against. At `radius == 0` the azimuth carries no length at all, so
/// every residue is snappable — the correct limit, and reached here
/// without a special case or a division.
fn closure_is_snappable(residue: f64, radius: f64, eps: f64) -> bool {
    residue * radius < eps
}

pub(crate) fn loop_polygon(
    body: &Body<f64>,
    chart: &Chart,
    chords: &HashMap<EdgeKey, Vec<u32>>,
    positions: &[Point3<f64>],
    face: FaceKey,
    lk: LoopKey,
    eps: f64,
) -> Result<Vec<UvPoint>, TessellateError> {
    let mut travs = traversals(body, chart, chords, face, lk)?;
    // Anchor the walk at a rim if the loop has one.
    if let Some(start) = travs
        .iter()
        .position(|t| matches!(t.kind, TravKind::Rim { .. }))
    {
        travs.rotate_left(start);
    }
    let no_rim = !travs.iter().any(|t| matches!(t.kind, TravKind::Rim { .. }));
    // The face's S10 orientation sense as a `±1` (module docs, the
    // pole-to-pole band). Read once here; consumed at exactly one site
    // below.
    let sense_sign: f64 = body
        .get_face(face)
        .ok_or(TessellateError::MissingEntity { what: "face" })?
        .sense_sign();
    let poles = chart.poles();
    let pole_v = |id: u32| -> Option<f64> {
        let p = positions[id as usize];
        poles
            .iter()
            .find(|(pp, _)| (p - *pp).norm() <= eps)
            .map(|&(_, pv)| pv)
    };
    // Pole-to-pole bands: precompute every column from the loop's 3-D
    // area vector (module docs).
    let band_u: Option<Vec<f64>> = if no_rim {
        let mut area = Vec3::new(0.0, 0.0, 0.0);
        let pts: Vec<Point3<f64>> = travs
            .iter()
            .flat_map(|t| t.ids[..t.ids.len() - 1].iter())
            .map(|&id| positions[id as usize])
            .collect();
        for (i, p) in pts.iter().enumerate() {
            let q = pts[(i + 1) % pts.len()];
            area = area + (*p - Point3::origin()).cross(q - Point3::origin());
        }
        // CATEGORY A (S10). `area` is the loop's 3-D vector area, so it
        // points along the face's OUTWARD normal side — but it is read
        // here as a direction in the CHART frame (`u_ref`/`v_ref`), to
        // pick which azimuth half the band occupies. Those two frames
        // differ by exactly `sense_sign`: a reversed face stores its
        // loop the other way round, `area` flips, and the raw `atan2`
        // would land π off — selecting the complementary meridian
        // branch and meshing the wrong half of the sphere. Multiplying
        // recovers the chart-frame azimuth for either sense. This is
        // NOT the double-count hazard that forbids a multiply in
        // `planar`/`curved`: nothing downstream re-derives this sign
        // from the winding — `mid_az` only chooses a `2πk` branch, and
        // the polygon's own winding (which does flip with the sense) is
        // consumed separately by `curved`'s `flip`.
        let chart_area = area * sense_sign;
        let mid_az = chart_area
            .dot(chart.v_ref)
            .atan2(chart_area.dot(chart.u_ref));
        Some(
            travs
                .iter()
                .map(|t| match t.kind {
                    TravKind::Meridian { u_raw } => unwrap_near(u_raw, mid_az),
                    TravKind::Rim { .. } => f64::NAN,
                })
                .collect(),
        )
    } else {
        None
    };

    let m = travs.len();
    let mut out: Vec<UvPoint> = Vec::new();
    let mut prev_u = match &band_u {
        Some(us) => us[m - 1],
        None => f64::NAN,
    };
    let mut prev_v = f64::NAN;
    let mut prev_was_rim = false;
    for (k, cur) in travs.iter().enumerate() {
        let jid = cur.ids[0];
        let jpole = pole_v(jid);
        match cur.kind {
            TravKind::Rim { v_raw } => {
                let v_edge = if chart.v_periodic() && k > 0 {
                    unwrap_near(v_raw, prev_v)
                } else {
                    v_raw
                };
                let ju = if k == 0 {
                    chart.u_of(positions[jid as usize])
                } else if prev_was_rim {
                    unwrap_near(chart.u_of(positions[jid as usize]), prev_u)
                } else {
                    prev_u
                };
                out.push(UvPoint {
                    u: ju,
                    v: v_edge,
                    id: jid,
                    pole: false,
                });
                prev_u = ju;
                for &id in &cur.ids[1..cur.ids.len() - 1] {
                    let u = unwrap_near(chart.u_of(positions[id as usize]), prev_u);
                    out.push(UvPoint {
                        u,
                        v: v_edge,
                        id,
                        pole: false,
                    });
                    prev_u = u;
                }
                prev_v = v_edge;
                prev_was_rim = true;
            }
            TravKind::Meridian { u_raw } => {
                let anchor = out.first().map_or(prev_u, |e| e.u);
                let ut = match &band_u {
                    Some(us) => us[k],
                    // Final traversal: its column contains the loop's
                    // closing vertex (`out[0]` lies on this meridian
                    // plane), so the branch nearest the closing anchor
                    // is exact by construction — continuity toward
                    // `prev_u` would pick the wrong branch for wedge
                    // angles θ > 3π/2 (the 2π − θ < π/2 shortcut).
                    None if k == m - 1 => unwrap_near(u_raw, anchor),
                    None => unwrap_tie(u_raw, prev_u, anchor),
                };
                if let Some(vp) = jpole {
                    // Close the incoming column, open the outgoing one.
                    out.push(UvPoint {
                        u: prev_u,
                        v: vp,
                        id: jid,
                        pole: true,
                    });
                    out.push(UvPoint {
                        u: ut,
                        v: vp,
                        id: jid,
                        pole: true,
                    });
                    prev_v = vp;
                } else {
                    let jv = if prev_was_rim {
                        prev_v
                    } else {
                        let v = chart.v_of(positions[jid as usize]);
                        if chart.v_periodic() && k > 0 {
                            unwrap_near(v, prev_v)
                        } else {
                            v
                        }
                    };
                    out.push(UvPoint {
                        u: ut,
                        v: jv,
                        id: jid,
                        pole: false,
                    });
                    prev_v = jv;
                }
                prev_u = ut;
                for &id in &cur.ids[1..cur.ids.len() - 1] {
                    let v_raw_pt = chart.v_of(positions[id as usize]);
                    let v = if chart.v_periodic() {
                        unwrap_near(v_raw_pt, prev_v)
                    } else {
                        v_raw_pt
                    };
                    out.push(UvPoint {
                        u: ut,
                        v,
                        id,
                        pole: false,
                    });
                    prev_v = v;
                }
                prev_was_rim = false;
            }
        }
    }
    // Closure: if the walk ends in a meridian, the first entry is that
    // column's junction. `out[0].u` and the final column are the same
    // analytic azimuth reached through two float paths (vertex atan2 vs
    // carrier-midpoint atan2), so they disagree only by accumulated
    // rounding. Snap onto the column so the polygon side is bitwise
    // straight; a residue too large to snap SAFELY is the structural
    // defect, and in release the unsnapped self-crossing polygon is
    // refused by the CDT constraint pre-check (typed `Triangulation`
    // error).
    //
    // THE BAR IS SPATIAL, NOT ANGULAR. This used to read `residue <
    // 1e-9` — a bare radian constant, unrelated to `eps` and to the
    // model's size, calibrated on an observed ≲1e-12 rad across the
    // scenes that existed then. `nist_ftc_09_asme1_rd.stp` closes at
    // 3.56e-9 rad and tripped it, having silently skipped the snap in
    // release for as long as that lane ran with assertions compiled
    // out (assert and snap read the same constant, so exceeding it
    // disabled both — the shape `closure_is_snappable` now forecloses).
    //
    // MEASURED (M8 census, 315 governed closures over the tour and the
    // wild corpus; instrumentation was temporary, the numbers are not):
    //
    //   tour, 202 closures — 171 bitwise EXACT, worst 4.0e-15 rad (9
    //     ulps at u ≈ π), worst arc 4.7e-16 m.
    //   wild, 113 closures — 95 bitwise exact. SEVEN of the eight
    //     importable files are exact on every closure. All 18 nonzero
    //     residues are in `nist_ftc_09`, and 16 of those sit at just
    //     two values (3.5640e-9 and 8.4502e-9 rad), eight apiece, all
    //     at one radius: 2.9718e-3 m = exactly 0.117 inch.
    //
    // So "exact by construction" is the RULE, not an approximation —
    // and the exception is not accumulated error. Discrete values
    // repeated eight times at a single radius is one geometric feature
    // instanced eight times; walk length has nothing to do with it.
    //
    // THE MECHANISM, traced. Those closures are hole generators whose
    // two endpoints the FILE states non-co-azimuthally. One of them:
    //
    //   p0 = (-3.1330000001, +0.0896, -4.2499999992) inch
    //   p1 = (-3.1330000000,  0.0,    -4.2500000000) inch
    //
    // The line runs along the hole's axis and should hold x and z
    // fixed; the file's ~10-decimal-digit coordinates differ in the
    // last one. That is 21.4 pm of displacement PERPENDICULAR to the
    // axis, which at r = 2.9718e-3 m subtends 7.2e-9 rad — and the
    // residue is half that spread (3.564e-9), because `out[0].u` reads
    // a vertex azimuth while the column reads the carrier MIDPOINT's.
    //
    // That is the whole case for measuring in metres. The angle is
    // `displacement / radius`, so a fixed coordinate-rounding error in
    // the source produces a LARGER angle on a SMALLER feature: a
    // scale-free angular bar necessarily mis-ranks small features, and
    // flags the 0.117-inch holes while passing the same physical error
    // elsewhere. The spatial bar sees the invariant quantity — 21 pm,
    // 1.6e6x inside this file's own ε — whatever the radius.
    //
    // TWO ROUTES TO ACTUAL EXACTNESS, neither taken here, both real:
    //   (a) adoption-side — re-mint such a line onto a single azimuth
    //       at import, the normalization class `StructureNormalization`
    //       already exists for. The move is 21 pm. Caveat: vertices are
    //       shared, so an azimuth snap for one cylinder perturbs the
    //       vertex its other faces see (still far inside ε).
    //   (b) kernel-side — have the FINAL meridian take its column from
    //       the closing vertex rather than the carrier midpoint, which
    //       is exact whatever the skew. `mid_azimuth` exists to dodge
    //       apex/pole endpoints, but `out[0]` is by construction a
    //       non-degenerate entry. Touches the anchor-branch choice
    //       (tuned for wedge angles > 3π/2), so it is a design
    //       conversation, not a tidy-up.
    //
    // u is an azimuth, so a residue is only as big as its lever arm:
    // `r·δu` is the arc length it displaces the polygon side by, and
    // `eps` is the length the rest of this function already measures
    // against (the junction merge above). Comparing the two in metres
    // is the dimensionally honest test; comparing radians to a
    // hard-coded angle was not a test of anything. At r → 0 the
    // azimuth carries no length at all and `eps/r → ∞` accepts freely,
    // which is the correct limit rather than a special case.
    if !no_rim && matches!(travs[m - 1].kind, TravKind::Meridian { .. }) && !out.is_empty() {
        let residue = (out[0].u - prev_u).abs();
        let radius = chart.radial(positions[out[0].id as usize]);
        debug_assert!(
            closure_is_snappable(residue, radius, eps),
            "loop closure residue {residue} rad at radius {radius} m displaces the \
             side by {} m, over eps {eps}",
            residue * radius
        );
        if closure_is_snappable(residue, radius, eps) {
            out[0].u = prev_u;
        }
    }
    Ok(out)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn cone_mirror_nappe_u_of() {
        let chart = Chart {
            axis: Vec3::new(0.0, 1.0, 0.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
            v_ref: Vec3::new(0.0, 1.0, 0.0).cross(Vec3::new(1.0, 0.0, 0.0)),
            anchor: Point3::new(0.0, 1.0, 0.0),
            kind: ChartKind::Cone {
                half_angle: core::f64::consts::FRAC_PI_4,
            },
        };
        let u = chart.u_of(Point3::new(0.5, 0.5, 0.0));
        assert!(
            (u - core::f64::consts::PI).abs() < 1e-12,
            "expected pi, got {u}"
        );
    }

    /// A chart about +z, anchored at the origin.
    fn z_chart(kind: ChartKind) -> Chart {
        let axis = Vec3::new(0.0, 0.0, 1.0);
        let u_ref = Vec3::new(1.0, 0.0, 0.0);
        Chart {
            axis,
            u_ref,
            v_ref: axis.cross(u_ref),
            anchor: Point3::new(0.0, 0.0, 0.0),
            kind,
        }
    }

    /// The lever arm is the distance from the axis, and sliding a point
    /// ALONG the axis does not change it.
    #[test]
    fn radial_is_the_distance_from_the_axis() {
        let c = z_chart(ChartKind::Cylinder { r: 2.0 });
        for z in [-10.0, 0.0, 7.5] {
            let d = c.radial(Point3::new(3.0, 4.0, z));
            assert!((d - 5.0).abs() < 1e-15, "expected 5, got {d} at z = {z}");
        }
    }

    /// `radial` reads the POINT, not `ChartKind` — which is what makes
    /// the cone work without a special case. A cone's radius is not in
    /// its kind payload at all (only the half-angle is); it varies along
    /// the loop, so one chart must yield different lever arms at
    /// different points. On a 45° cone from the origin, radius = height.
    #[test]
    fn radial_varies_along_a_cone_whose_kind_carries_no_radius() {
        let c = z_chart(ChartKind::Cone {
            half_angle: core::f64::consts::FRAC_PI_4,
        });
        for h in [3.0, 9.0] {
            let d = c.radial(Point3::new(0.0, h, h));
            assert!((d - h).abs() < 1e-15, "expected {h}, got {d}");
        }
    }

    /// The bar is SPATIAL. The residue that falsified the old bare-radian
    /// form — `nist_ftc_09_asme1_rd.stp` closes at 3.56e-9 rad, over the
    /// old 1e-9 constant — snaps at any real lever arm, and only fails to
    /// at an absurd one.
    #[test]
    fn the_closure_bar_is_spatial_not_angular() {
        let eps = 3.38e-5;
        let residue = 3.56e-9;
        assert!(
            closure_is_snappable(residue, 0.05, eps),
            "3.56e-9 rad at 50 mm displaces ~1.8e-10 m — far under eps"
        );
        assert!(
            !closure_is_snappable(residue, 1e5, eps),
            "the same residue must NOT pass at a 100 km lever arm"
        );
    }

    /// Growing the lever arm tightens the angular bar proportionally —
    /// the property a bare radian constant did not have.
    #[test]
    fn the_closure_bar_tightens_as_the_lever_arm_grows() {
        let eps = 1e-5;
        let residue = 1e-6;
        assert!(closure_is_snappable(residue, 1.0, eps), "1e-6 m < eps");
        assert!(
            !closure_is_snappable(residue, 100.0, eps),
            "1e-4 m > eps — the same angle, ten thousand times the arc"
        );
    }

    /// On the axis the azimuth carries no length, so every residue
    /// snaps. Must be a plain comparison, not a NaN or a division.
    #[test]
    fn on_the_axis_every_residue_snaps() {
        assert!(closure_is_snappable(core::f64::consts::PI, 0.0, 1e-9));
    }
}
