//! Boundary-loop walking: traversal extraction, chart inversion, and
//! periodic unwrapping — turning a curved face's loop into a UV polygon
//! whose sides are exactly straight (bitwise-constant u or v). That
//! held only for a side carried by ONE edge until #653 (this module's
//! header asserted the stronger thing for a milestone and
//! [`crate::curved`] had to discover it was false); it now holds for
//! every side, and the paragraph below says how.
//!
//! Every curved face this project's SWEEPS author is a swept UV
//! rectangle whose boundary edges are iso-curves (extrude/revolve
//! structure, PR 4/5): **rims** (circles around the surface axis,
//! v = const) and **meridians** (u = const:
//! struts/generators/profile copies/`Seam` edges). That is a fact
//! about authoring, **not** about input — an iso-bounded domain need
//! not be a rectangle (a keyway on a cylinder is bounded by lines and
//! circles and is a U), and this walk handles such a loop perfectly
//! well; it is [`crate::curved`]'s interior grid that needs the
//! rectangle, and that lane checks it (S28,
//! `TessellateError::UnsupportedCurvedDomain`). The walk classifies
//! each traversal structurally, assigns the constant coordinate once
//! per ISO SIDE — never per point, so no side wobbles from point to
//! point and the CDT sees no sliver of that kind — and unwraps the
//! periodic coordinate(s) by continuity (chord steps ≤ π/4 make branch
//! choice unambiguous away from poles).
//!
//! **"Once per edge" was not "once per side" (issue #653, closed
//! here).** A side that IS one edge always came out bitwise straight.
//! A side carried by two or more edges — every exporter emits one the
//! moment a vertex lands mid-side, and [`topo::Body::split_edge`]
//! mints one directly — used to have each sub-edge derive its own
//! column from [`mid_azimuth`] → [`Chart::u_of`], an `atan2` of a
//! DIFFERENT point of the same carrier. Those agree analytically, and
//! agree bitwise on axis-aligned dyadic fixtures, but under a general
//! rigid placement they differ by ulps — which is how 71 rows of a
//! 1524-row sweep returned `Ok` from `tessellate` and failed
//! `check_mesh`. [`iso_side_starts`] now groups consecutive traversals
//! into RUNS and gives each run ONE coordinate, so bitwise
//! straightness is this module's GUARANTEE rather than its intent.
//! [`crate::curved`]'s domain guard still bands its comparison in
//! metres (`curved::entries_off_bbox`) — as a backstop, no longer as
//! the thing that keeps valid parts from being refused.
//!
//! One structural exception to the continuity rule: a
//! rim-anchored loop's **final** meridian contains the loop's closing
//! vertex, so its column takes the branch nearest the first polygon
//! entry (`out[0].u`) rather than continuity — the right BRANCH by
//! construction for every wedge angle (continuity would pick the wrong
//! one for θ > 3π/2, where the complement 2π − θ < π/2 is closer).
//!
//! That final column is taken from `out[0].u` ITSELF, not merely from
//! the branch nearest it, so a rim-anchored loop closes **bitwise**:
//! the closing polygon side is exactly vertical by construction, at no
//! tolerance. It did not always. The column used to come from the
//! meridian carrier's MIDPOINT (`mid_azimuth`) — the same analytic
//! azimuth down a different float path — and the gap between the two
//! was closed by an ε-scaled snap. Imported geometry can make that gap
//! nonzero (it is source-coordinate rounding, not accumulation), which
//! is what made a tolerance look necessary. Taking the vertex's own
//! value makes the quantity identically zero instead, and the
//! comparison survives inside [`closing_column`] as a `debug_assert!`
//! measuring the INPUT's quality rather than a bar the mesh depends on.
//!
//! Pole handling (chart singularities; the surface's `normal` is never
//! sampled): a pole/apex is always an edge **endpoint** (valence 2). A
//! pole junction between two meridians emits *two* polygon entries —
//! one closing the incoming meridian's column, one opening the
//! outgoing column — both mapping to the single pole mesh vertex; the
//! collapsed side between them becomes the fan (see the curved-face
//! module — that the two entries yield ONE fan rather than two
//! overlapping ones is a property of the interior grid, held by
//! `curved::pole_columns`, not of this emission; issue #678).
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

use geom::Curve3;
use geom::Surface;
use geom_brep::EdgeGeometry;
use geom_core::{Point3, Vec3};
use topo::{Body, EdgeKey, FaceKey, LoopBoundary, LoopKey};

use crate::types::TessellateError;
use geom_core::Tol;

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

    /// The v counterpart of [`Self::radial`]: `|∂S/∂v|`, the length
    /// one unit of the chart's v coordinate displaces a point by.
    ///
    /// Constant per kind because v is either already a length
    /// (cylinder — axial metres; cone — slant metres, both
    /// `|∂S/∂v| = 1`) or an angle turning on a fixed radius (sphere —
    /// latitude on `r`; torus — minor angle on `r`). u needs the point
    /// because its lever arm is the *distance from the axis*, which
    /// varies over a cone and a sphere; v's does not, so this takes
    /// none.
    ///
    /// Together `(radial(p), v_lever())` convert a UV discrepancy into
    /// metres, which is the only honest unit to compare against ε — the
    /// argument `curved`'s domain guard makes, and the one
    /// [`gap_is_noise`] makes for the closure detector — one
    /// predicate, named there.
    pub(crate) fn v_lever(&self) -> f64 {
        match self.kind {
            ChartKind::Cylinder { .. } | ChartKind::Cone { .. } => 1.0,
            ChartKind::Sphere { r } => r,
            ChartKind::Torus { minor, .. } => minor,
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
    /// radius — one value per EDGE, which two sub-edges of ONE carrier
    /// circle already agreed on bitwise (same center, same radius) but
    /// two DISTINCT `Circle` carriers stating the same analytic circle
    /// (what an exporter emits for a split rim) did not. That was the
    /// module header's "once per edge is not once per side" caveat,
    /// issue #653, on the v axis; [`iso_side_starts`] closes it by
    /// giving the whole run the first edge's value, which is what makes
    /// the row bitwise straight whatever carries it.
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

/// **This crate's ε band, and its only spelling.** Is a UV discrepancy
/// of `gap` (radians for u and for a periodic v, metres otherwise), at
/// a point whose lever arm for that axis is `lever` metres, float noise
/// rather than a defect in the geometry?
///
/// `gap * lever` is the LENGTH the two values disagree by, and `eps` is
/// a length — the only unit the two are honestly comparable in. A
/// scale-free radian constant necessarily mis-ranks features, because a
/// fixed source-coordinate error subtends a larger angle on a smaller
/// one (see [`closing_column`]'s trace: 21 pm reads as 3.6e-9 rad on a
/// 0.117-inch hole). At `lever == 0` the coordinate carries no length
/// at all, so every gap is noise — the correct limit, reached without a
/// special case or a division. The comparison is a bare `<`, so a NaN
/// gap or lever is never noise: a poisoned coordinate stays a refusal
/// (`curved`) or an assertion failure (`closing_column`) rather than
/// being admitted.
///
/// The lever arms are [`Chart::radial`] (u — the point's own distance
/// from the axis, so a cone and a sphere get a varying one) and
/// [`Chart::v_lever`] (v — constant per kind).
///
/// # Consumers
///
/// - [`closing_column`]'s `debug_assert!` — the loop-closure DETECTOR.
///   It decides nothing: the closure is exact by construction and the
///   column is the closing vertex's own azimuth either way. What it
///   measures is INPUT quality, and it is this project's only detector
///   for a class of defective source coordinates (S22).
/// - [`crate::curved`]'s swept-rectangle domain guard
///   (`entries_off_bbox`) — decides whether a face is REFUSED. #648
///   compared exactly there, on the then-false premise that every iso
///   side is bitwise straight, and false-refused valid parts. #653
///   made the premise true, so the band is now a backstop that no
///   in-tree fixture needs; the row that witnesses it is synthetic.
/// - `loop_polygon`'s two ISO-SIDE CONTINUATION detectors (#653), one
///   per axis. Consecutive sub-edges of one side are each other's
///   float-path twins the way the closure's two paths were; the run's
///   first coordinate is used for all of them, and these
///   `debug_assert!`s report — and gate nothing on — how far the
///   discarded value was.
///
/// [`iso_side_starts`] is NOT in that list, and the distinction is
/// narrow enough to be worth stating rather than leaving to the
/// absence: it does not read THIS PREDICATE — which traversals share a
/// side is decided by kind and by a pole test, not by a gap-against-a-
/// lever band — but it does read **ε**, directly, and its read DECIDES
/// which `f64` the entries of a side carry. A reader taking the four
/// consumers above for the crate's ε ledger would be short by three:
/// this list is `gap_is_noise`'s callers and nothing wider.
/// [`crate::sizing::SizingTols`] says what an ε read may DO; the inventory of
/// where they are is computed by `mesh/tests/all.rs`'s
/// `the_eps_inventory_is_pinned`.
///
/// It is deliberately NOT named for the closure any more: it was
/// `closure_is_snappable` while a snap read it, and three of its four
/// consumers are not closures.
///
/// # What the three DETECTORS are, under D2
///
/// All three are `debug_assert!`s that gate nothing, and all three are
/// reachable by INPUT — a defective source file, not a kernel bug. By
/// the letter of D9's D2 addendum that is row 1/3 territory and
/// `debug_assert` is reserved for row 5, so **this is a deviation, and
/// it is named as one** rather than filed under "re-derivation". The
/// ledger for it, both ways, is written once at [`closing_column`] and
/// covers all three.
///
/// Two things bound the cost, and one does not:
///
/// - the bar is SPATIAL, not angular, everywhere. The only real file
///   that ever tripped a detector in this crate
///   (`nist_ftc_09_asme1_rd.stp`, 3.56e-9 rad) tripped a bare-radian
///   bar; in metres its gap is 1.6e6x inside that file's own ε and
///   nothing fires.
/// - none of them can make debug and release disagree about the MESH:
///   the coordinate is the same either way.
/// - what does NOT bound it: `crates/mesh` has no dev-dependency on
///   `step-import`, and no test in this repo tessellates the wild or
///   FreeCAD corpora. The input class these are about is untested by
///   construction, so "it fires nowhere" means nowhere anything looks.
///   A typed warning channel would dominate all three; there is none,
///   and building one is **issue #868** — the scheduled work that
///   would retire this deviation. Until it lands the deviation stands
///   as written, at the cost the two bullets above state.
pub(crate) fn gap_is_noise(gap: f64, lever: f64, eps: f64) -> bool {
    gap * lever < eps
}

/// The column of a rim-anchored loop's FINAL meridian: the closing
/// `anchor` (`out[0].u`) itself, bitwise.
///
/// `out[0]` lies on this meridian's plane by construction, so its
/// azimuth IS this column's — the exact value, whatever skew the
/// source geometry carries. Returning it verbatim makes the loop close
/// bitwise (`loop_polygon` asserts that after the walk) and the
/// closing polygon side exactly vertical before the CDT sees it as a
/// constraint.
///
/// The BRANCH rule is unchanged and is why `anchor` rather than
/// `prev_u` is the input at all: continuity toward `prev_u` picks the
/// wrong 2πk for wedge angles θ > 3π/2 (the 2π − θ < π/2 shortcut).
/// `unwrap_near(u_raw, anchor)` — what this used to return — selects
/// `anchor`'s branch and then differs from `anchor` only WITHIN it, so
/// dropping it cannot change which branch is taken. It drops exactly
/// the float difference an ε-scaled snap used to absorb after the
/// walk, and with it this crate's second structural read of ε (S22).
///
/// # The retained detector
///
/// `u_raw` is [`mid_azimuth`]: the same analytic azimuth reached from
/// the carrier MIDPOINT. Analytically `carrier == anchor`; a gap
/// between them is a statement about the INPUT, and this assertion is
/// the only place the project ever sees it. It gates nothing — the
/// column is `anchor` either way — so what it measures is **data
/// quality**, which is what it was always really about.
///
/// MEASURED, and the numbers live in ONE place: `SMELL-SCAN-2026-08`
/// §S22 carries the closure census (wild corpus and in-tree, per file,
/// with the instrumentation described). It is not restated here,
/// because the previous census in this exact spot went stale and wrong
/// — *"all 18 nonzero residues sit in one wild file"* survived into a
/// milestone doc after a second file joined them — and three
/// hand-synced copies of a number is the shape that produced that. The
/// shape, which is what a reader of this function needs: nonzero
/// closures are RARE and confined to imported files; the values repeat
/// at a single radius, so they are one geometric feature instanced
/// many times rather than accumulated error; and in-tree the only
/// nonzero closures are 1 ulp, from the one obliquely-placed fixture.
///
/// TRACED — the one file that matters, kept here because it is the
/// argument for the bar's UNIT and not a census figure. Those closures
/// are `nist_ftc_09_asme1_rd.stp` hole generators whose two endpoints
/// the FILE states non-co-azimuthally:
///
/// ```text
/// p0 = (-3.1330000001, +0.0896, -4.2499999992) inch
/// p1 = (-3.1330000000,  0.0,    -4.2500000000) inch
/// ```
///
/// The line runs along the hole axis and should hold x and z fixed;
/// the file's ~10-digit coordinates differ in the last one. That is
/// 21.4 pm PERPENDICULAR to the axis, subtending 7.2e-9 rad at
/// r = 2.9718e-3 m — and the gap is half that spread, because `anchor`
/// reads a vertex azimuth while `u_raw` reads the midpoint's.
///
/// THE BAR IS SPATIAL, NOT ANGULAR, and that trace is why: the angle
/// is displacement / radius, so a fixed coordinate-rounding error in
/// the source produces a LARGER angle on a SMALLER feature. A
/// scale-free radian constant (this once read `< 1e-9`) necessarily
/// mis-ranks small features — it flagged the 0.117-inch holes while
/// passing the same physical error elsewhere. In metres the invariant
/// quantity shows: 21 pm, 1.6e6x inside that file's own ε. The band
/// itself is [`gap_is_noise`].
///
/// # What the assertion costs, both ways
///
/// D9's D2 addendum reserves `debug_assert` for row 5 — a kernel bug
/// detectable only by re-derivation — and D9's footnote calls
/// debug_asserts unreachable by input. **This one is not.**
/// `import_step` + `tessellate` on `nist_ftc_09_asme1_rd.stp` reaches
/// it, on a file whose mesh is correct and bitwise identical in both
/// profiles. So the honest ledger is:
///
/// - FOR: debug and release can no longer DISAGREE about the mesh.
///   Under the old shape the assertion and the snap read the same bar,
///   so a residue over it disabled both — debug screamed while release
///   silently shipped an unsnapped, possibly self-crossing polygon
///   (#481). Now the polygon is the same in both profiles whatever the
///   assertion says.
/// - AGAINST, two of them. A debug build now PANICS on a file whose
///   mesh is fine; the residue is a data-quality remark, not a defect
///   in the output, and `tessellate` returning is a different question
///   from the mesh being right. And release now has NO detector at all
///   for the defective-source class: under the old shape a large
///   residue left the polygon unsnapped, which the CDT pre-check would
///   refuse typed. Nothing above 3.6e-9 rad has ever been seen, so the
///   practical risk is small — but it is a trade, not a free win.
///
/// A typed warning channel would dominate both; there is none, and
/// building one is **issue #868** — which is why the trade is where it
/// is, and what would retire it.
fn closing_column(u_raw: f64, anchor: f64, radius: f64, eps: f64) -> f64 {
    let carrier = unwrap_near(u_raw, anchor);
    let gap = (carrier - anchor).abs();
    debug_assert!(
        gap_is_noise(gap, radius, eps),
        "the closing meridian's carrier-midpoint azimuth {carrier} and its closing \
         vertex's {anchor} disagree by {gap} rad at radius {radius} m — {} m of arc, \
         over eps {eps}. The mesh does not depend on this (the column is the vertex's \
         own azimuth either way); it says the SOURCE states one line's endpoints \
         off-axis.",
        gap * radius
    );
    anchor
}

/// Which traversals OPEN an iso side, cyclically (issue #653).
///
/// An iso side of a curved face may be carried by SEVERAL edges — a
/// vertex dropped on it by [`topo::Body::split_edge`], a boolean, or an
/// exporter emitting two collinear `EDGE_CURVE`s (which is what every
/// exporter emits when a vertex lands on that edge). Each such edge
/// derives its own constant coordinate from its own mid-parameter
/// point, so the side is straight only to ulps under a general rigid
/// placement — analytically equal, bitwise equal only on axis-aligned
/// dyadic fixtures. Grouping the edges into RUNS and giving each run
/// ONE coordinate restores the bitwise-straight side, and with it the
/// premise `curved`'s interior grid and its domain guard rest on.
///
/// # The test is structural, not a band
///
/// Two consecutive traversals belong to one iso side iff they are the
/// same kind and their shared junction vertex is a REGULAR point of the
/// chart. That is not an approximation:
///
/// - A point off the axis has exactly ONE azimuth, so two meridians
///   meeting there are necessarily co-azimuthal — there is no such
///   thing as a meridian-meridian corner away from the axis.
/// - Two coaxial circles at different `v` are disjoint, so two rims
///   meeting anywhere are necessarily co-`v`.
///
/// The only way consecutive same-kind traversals can be genuinely
/// different sides is a CHART SINGULARITY at the junction — which is
/// exactly the pole-fan corner the walk already emits two entries for,
/// and exactly the π-apart wire-band case [`unwrap_tie`] exists for.
///
/// ## The premise that argument needs, which nothing here checks
///
/// Both bullets say "meridian" and "rim" meaning ISO-CURVES of this
/// chart. [`classify`] does not establish that: a `Line` carrier is a
/// meridian unconditionally, and a `Circle` is a rim iff
/// `|n · axis| > 0.5` — a cone of directions, not coaxiality. So the
/// real premise is **every boundary edge is an iso-curve of this
/// chart**, and it is inherited from upstream rather than verified.
/// Per kind:
///
/// - **cylinder, cone** — safe by geometry. A circle on either surface
///   whose axis is within 60° of the chart's IS coaxial.
/// - **torus** — leaks harmlessly. A Villarceau circle classifies as a
///   Rim whenever `minor/major < 0.866` and is not coaxial, but its
///   centre lies in the equatorial plane, so [`Chart::rim_v`] reads
///   `0.0` for every one of them and merging a pair lands on the value
///   they already shared.
/// - **sphere** — the real leak. EVERY plane section of a sphere is a
///   [`Curve3::Circle`], and `trimmed::has_trim_carrier` diverts only
///   `Ellipse` and `Nurbs`, so an obliquely-cut sphere face would
///   arrive here carrying two non-iso circles. Two consecutive such
///   arcs meeting off-axis get collapsed onto ONE coordinate.
///
/// That last one is a severity FLIP, not a wash, and it is worth being
/// explicit: per-edge, the two sub-arcs took two different wrong
/// coordinates and the polygon would very likely have failed
/// [`crate::curved`]'s domain guard as a typed
/// `UnsupportedCurvedDomain`; collapsed, it can BE its own bounding
/// rectangle and be admitted. The #653 sweep cannot see this — it
/// measures straightness and watertightness, not whether two sides
/// were correctly distinguished.
///
/// **Reachable today: no**, by two upstream gates. `topo::boolean`
/// refuses a tilted plane × sphere section typed (`SectionInvariant`;
/// an axis-aligned slab cut succeeds and re-charts, a 0.7 rad rotated
/// one refuses — executed). `import_step`'s tier-3
/// `props::curved::sphere_boundary` admits a circle only as a coaxial
/// rim or a great circle centred at the sphere centre — read, not
/// executed, and so the one door not directly witnessed. Recorded
/// rather than fixed: the cheap hardening, if this ever needs one, is
/// for `classify` to require the rim circle's centre ON the axis
/// (`|w − â(w·â)| < eps`) rather than only `|n · axis| > 0.5`.
///
/// # One test for every singularity — of the run-breaking DECISION
///
/// Every chart singularity lies ON THE AXIS — a sphere's poles, a
/// cone's apex — so `radial(junction) > eps` covers them with one
/// comparison rather than a match on [`ChartKind`], and in particular
/// without consulting [`Chart::poles`].
///
/// "One comparison for all of them" is a claim about the DECISION MADE
/// HERE and not about pole HANDLING downstream. [`Chart::poles`] is
/// empty for `Torus`, so `loop_polygon`'s `pole_v` returns `None` at a
/// toroidal axis point and the walk emits one ordinary entry instead
/// of the two-entry fan. Dormant — `revolve` refuses horn and spindle
/// at construction — but it is a different question from this one and
/// this function does not answer it.
///
/// **Not consulting `poles()` is deliberate but is NOT tested, because
/// this build cannot construct the case that would separate them.**
/// The one chart singularity `poles()` does not list is the axis point
/// of a horn or spindle torus (`major + minor·cos v` vanishing), and
/// `revolve` refuses both at construction
/// (`sweep::revolve`'s profile check — the sweep's own skip lines show
/// it). So a `poles()`-based test would behave identically on every
/// body this build can mint. The radial test is chosen because it is
/// the smaller thing to state, not because a live case demands it; if
/// horn tori ever become constructible, this line already covers them
/// and that is a bonus, not evidence. The unit row below exercises a
/// SPHERE chart, which is the only pole case there is here.
///
/// The bar is a LENGTH, as everywhere else in this module: within ε of
/// the axis an azimuth carries no distinguishable direction, so the
/// answer there is to break the run and keep the per-edge coordinate.
///
/// **In WHICH DIRECTION that is conservative, because the word alone
/// overstates it.** Breaking a run can never merge two genuinely
/// different sides, which is the failure that would corrupt a mesh
/// silently in a new way. It is NOT refusal-safe. If the bar breaks a
/// run at a LEGITIMATE boundary vertex — one that happens to sit
/// within ε of the axis — then `starts[k]` is `true` there and that
/// junction gets exactly main's per-edge assignment: the side stops
/// being bitwise straight, `curved::entries_off_bbox` is banded and
/// will not refuse it, and `tessellate` does not run `check_mesh`. The
/// fallback therefore **reinstates #653 for that face rather than
/// refusing it** — an unfixed residue, not a regression, since the
/// walk for that junction is byte-for-byte what shipped before #653.
///
/// Nothing has reached it: swept across cone slopes 1 … 1e-5, split
/// distances 1e-9 … 1e-3 from both ends of every line edge, ε in
/// {1e-9, 1e-7, 1e-3}, identity and oblique placements — no junction
/// ever landed at `0 < radial <= eps`. `split_edge`'s interiority gate
/// is metred against the same band, so it will not drop a vertex
/// within ~ε of an endpoint, and `revolve` refuses near-horn toroids
/// at a certified bar. The route with NO such gate, and so the
/// settling experiment nobody has run, is import: a STEP fixture
/// stating a cone face whose generator is two collinear `EDGE_CURVE`s
/// meeting within ε of the apex — the same door `split_oblique.step`
/// walks through.
///
/// # What this cannot change
///
/// On a loop whose every iso side is one edge, consecutive traversals
/// always differ in kind or meet at a pole, so EVERY entry is `true`
/// and every rule downstream is the one that ran before #653. That is
/// why unsplit bodies mesh bitwise as they did.
fn iso_side_starts(
    travs: &[Trav],
    chart: &Chart,
    positions: &[Point3<f64>],
    eps: f64,
) -> Vec<bool> {
    let m = travs.len();
    if m < 2 {
        // A single traversal has no predecessor to continue from, and
        // an empty loop has nothing to mark. Hoisted: it does not vary
        // with `k`.
        return vec![true; m];
    }
    (0..m)
        .map(|k| {
            let same_kind = matches!(
                (&travs[(k + m - 1) % m].kind, &travs[k].kind),
                (TravKind::Rim { .. }, TravKind::Rim { .. })
                    | (TravKind::Meridian { .. }, TravKind::Meridian { .. })
            );
            let junction = positions[travs[k].ids[0] as usize];
            !(same_kind && chart.radial(junction) > eps)
        })
        .collect()
}

/// Where the walk starts (index into `travs`), or `None` to leave the
/// cycle as it is.
///
/// A rim if the loop has one — that much predates #653 and is what
/// makes the closure rule apply to a MERIDIAN. The refinement is
/// "among rims, one that OPENS its row": a run that wrapped past index
/// 0 would have its first traversal marked a continuation with nothing
/// before it to continue from, and the `starts[0] = true` patch in
/// [`loop_polygon`] would then split that run in the middle of a side.
///
/// # The failing condition, named
///
/// This can only differ from the `position(Rim)` it replaces when the
/// loop's FIRST rim is a continuation — i.e. some rim row is carried by
/// several edges AND the cycle happens to begin inside it. On an
/// unsplit loop every rim opens its row and this returns the same index
/// the old form did, so nothing rotates differently. Rows:
/// `the_walk_anchors_on_a_rim_that_opens_its_row`.
///
/// The `or_else` fallback (any rim at all) is for the degenerate loop
/// whose rims form a single cyclic run with no opening — unreachable
/// through `traversals`, since such a loop has no meridian and so no
/// junction the run could break at, but stated rather than indexed
/// blind.
fn walk_anchor(travs: &[Trav], starts: &[bool]) -> Option<usize> {
    let has_rim = travs.iter().any(|t| matches!(t.kind, TravKind::Rim { .. }));
    if has_rim {
        (0..travs.len())
            .find(|&k| matches!(travs[k].kind, TravKind::Rim { .. }) && starts[k])
            .or_else(|| {
                travs
                    .iter()
                    .position(|t| matches!(t.kind, TravKind::Rim { .. }))
            })
    } else {
        // No rim: the walk did not rotate before #653 and still does
        // not on an unsplit loop, where `starts[0]` is already true.
        (0..travs.len()).find(|&k| starts[k])
    }
}

/// The index that OPENS the walk's last iso side — the side carrying
/// the loop's closing traversal, and so the side the closure rule and
/// the pole-band seed both belong to.
///
/// `m - 1` on an unsplit loop, where every traversal opens its own
/// side. It differs exactly when the LAST iso side is carried by
/// several edges, and that is the condition both of its consumers need:
///
/// - `loop_polygon`'s meridian arm applies [`closing_column`] at
///   `k == closing_side`, not at `k == m - 1`, so the rule belongs to
///   the side; the continuations after it repeat that column bitwise.
/// - the pole-to-pole band seeds `prev_u` with `band_u[closing_side]`.
///   Seeding with `band_u[m - 1]` — the last traversal's OWN value —
///   was wrong for a rimless band whose last iso side is carried by
///   several edges: the last traversal emits its RUN's column, so the
///   pole entry that closes the loop at `k == 0` carried a neighbour's
///   value instead. Measured on the ball, that put one walk 4.4e-16 m
///   off its own UV bounding box.
///
/// Rows: `the_closing_side_is_the_last_run_start_not_the_last_index`.
fn closing_side(starts: &[bool]) -> usize {
    (0..starts.len()).rev().find(|&k| starts[k]).unwrap_or(0)
}

/// Walks a curved face's loop into its UV polygon (module docs: the
/// classification, unwrapping, pole, and disambiguation rules).
///
/// The order of business, because the function is long and each step
/// depends on the one before:
///
/// 1. [`traversals`] — one entry per boundary edge, classified `Rim`
///    or `Meridian` with its raw constant coordinate.
/// 2. [`iso_side_starts`] — which traversals OPEN an iso side (#653).
///    Everything after this reads `starts`, not the edge list.
/// 3. [`walk_anchor`] + `rotate_left` — put index 0 on a side that
///    opens, preferring a rim so the closure rule applies to a
///    meridian. `starts[0]` is then forced `true`.
/// 4. [`closing_side`] — the opening index of the LAST iso side, which
///    is what the closure rule and the pole-band seed key on.
/// 5. `band_u` — for a rimless pole-to-pole band only, every column
///    precomputed from the loop's 3-D area vector.
/// 6. the emission loop — per traversal, take the run's coordinate (a
///    continuation repeats it bitwise, with a detector on the gap it
///    discards) and unwrap the other axis by continuity.
/// 7. the closure assertion — a revert detector, not a runtime guard.
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
    let m = travs.len();
    // ISO-SIDE RUNS (#653): which traversals open a side, and so take
    // a fresh constant coordinate rather than the running one.
    let mut starts = iso_side_starts(&travs, chart, positions, eps);
    let has_rim = travs.iter().any(|t| matches!(t.kind, TravKind::Rim { .. }));
    let anchor_at = walk_anchor(&travs, &starts);
    if let Some(start) = anchor_at {
        travs.rotate_left(start);
        starts.rotate_left(start);
    }
    // Traversal 0 opens the walk whatever the cycle looked like: it has
    // no predecessor to continue. (Only reachable as a `false` in the
    // degenerate case where the whole loop is one cyclic run and no
    // rotation could open it — then this is the pre-#653 behaviour for
    // that one junction, which is no worse than before.)
    if let Some(first) = starts.first_mut() {
        *first = true;
    }
    let closing_side = closing_side(&starts);
    let no_rim = !has_rim;
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

    let mut out: Vec<UvPoint> = Vec::new();
    // The value the FINAL traversal will emit, so the pole entry that
    // closes it at `k == 0` carries that column and not a neighbour's.
    // On a pole-to-pole band the last traversal takes its RUN's column,
    // which is the run start's band value — `m - 1` on an unsplit loop,
    // and `closing_side` in the failing case: a rimless pole-to-pole
    // band whose last iso side is carried by several edges. See
    // `closing_side`'s docs for the mechanism and the measurement.
    let mut prev_u = match &band_u {
        Some(us) => us[closing_side],
        None => f64::NAN,
    };
    let mut prev_v = f64::NAN;
    let mut prev_was_rim = false;
    for (k, cur) in travs.iter().enumerate() {
        let jid = cur.ids[0];
        let jpole = pole_v(jid);
        match cur.kind {
            TravKind::Rim { v_raw } => {
                let v_own = if chart.v_periodic() && k > 0 {
                    unwrap_near(v_raw, prev_v)
                } else {
                    v_raw
                };
                let v_edge = if starts[k] {
                    v_own
                } else {
                    // Same iso side as the previous traversal: ONE row
                    // for the whole side, bitwise (#653). The two
                    // values are the same analytic v down two float
                    // paths (two carrier circles' centres), so the
                    // difference is a statement about the INPUT — the
                    // detector below, in metres, is the only place the
                    // project sees it, and it gates nothing. Its D2
                    // classification is `gap_is_noise`'s, and the
                    // ledger for it is `closing_column`'s.
                    let gap = (v_own - prev_v).abs();
                    let lever = chart.v_lever();
                    debug_assert!(
                        gap_is_noise(gap, lever, eps),
                        "two edges of one rim row disagree by {gap} in v at a {lever} m/unit \
                         lever arm — {} m, over eps {eps}. The mesh does not depend on this \
                         (the row is the run's first edge's v either way); it says the two \
                         carrier circles are not the same circle.",
                        gap * lever
                    );
                    prev_v
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
                let ut = if starts[k] {
                    match &band_u {
                        Some(us) => us[k],
                        None => {
                            // `band_u.is_none()` ⟺ `!no_rim` (`no_rim`
                            // is computed AFTER the `rotate_left`), so
                            // this arm implies `travs[0]` is a `Rim`,
                            // which implies `k >= 1` — a meridian is
                            // never `travs[0]` here — and the rim arm
                            // above pushes UNCONDITIONALLY. So `out` is
                            // non-empty on every path that reaches this
                            // line.
                            //
                            // Stated as `unreachable!` rather than as a
                            // default: D2 addendum row 4, a kernel bug
                            // the code can observe in a branch. The
                            // default this replaces was `0.0` for the
                            // lever arm, which would have made
                            // `gap * 0 < eps` true for every gap and
                            // silently disabled the detector — a dead
                            // branch is still a place to state the
                            // invariant, and fail-open was the wrong way
                            // to state it.
                            let Some(first) = out.first() else {
                                unreachable!(
                                    "a rim-anchored loop reaches its meridians with a non-empty polygon"
                                )
                            };
                            let anchor = first.u;
                            if k == closing_side {
                                // Final ISO SIDE: its column contains
                                // the loop's closing vertex (`out[0]`
                                // lies on this meridian plane), so the
                                // column is that vertex's own azimuth,
                                // exactly — see `closing_column` for the
                                // branch rule it keeps and the detector
                                // it retains. `closing_side` is `m - 1`
                                // unless that side is carried by several
                                // edges, in which case the rule belongs
                                // to the SIDE and not to its last edge;
                                // the continuations below then repeat
                                // this column bitwise, so the closure
                                // assertion after the loop still holds.
                                // The lever arm is `out[0]`'s, the same
                                // point the assertion is about.
                                let radius = chart.radial(positions[first.id as usize]);
                                closing_column(u_raw, anchor, radius, eps)
                            } else {
                                unwrap_tie(u_raw, prev_u, anchor)
                            }
                        }
                    }
                } else {
                    // Same iso side as the previous traversal: ONE
                    // column for the whole side, bitwise (#653). This
                    // is the substitution the loop closure makes at the
                    // seam, one level over — the sub-edges are each
                    // other's float-path twins the way the closure's
                    // two paths are. The detector below measures the
                    // gap in metres and gates nothing; its D2
                    // classification is `gap_is_noise`'s, and the
                    // ledger for it is `closing_column`'s.
                    let own = unwrap_near(u_raw, prev_u);
                    let gap = (own - prev_u).abs();
                    let radius = chart.radial(positions[jid as usize]);
                    debug_assert!(
                        gap_is_noise(gap, radius, eps),
                        "two edges of one meridian column disagree by {gap} rad at radius \
                         {radius} m — {} m of arc, over eps {eps}. The mesh does not depend \
                         on this (the column is the run's first edge's either way); it says \
                         the SOURCE states one iso side's carriers off-axis.",
                        gap * radius
                    );
                    prev_u
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
    // CLOSURE, exact by construction. If the walk ends in a meridian,
    // `out[0]` is that column's junction — and `closing_column` took
    // the column from `out[0].u` itself, so the two agree bitwise and
    // the closing polygon side is exactly vertical.
    //
    // WHAT THIS REPLACES (S22, and the ε-vs-δ question it asked). The
    // column used to come from the carrier midpoint, so the closing
    // side's two ends were one analytic azimuth down two float paths;
    // the difference was SNAPPED away here when `residue * radius <
    // eps`. That snap was this crate's second structural read of ε,
    // and it made the mesh a function of (body, δ, ε) against `lib.rs`
    // and D9's claim of (body, δ). Whether the bar should have been ε
    // or δ is moot: taking the vertex's own azimuth one level up makes
    // the residue identically zero.
    //
    // WHAT THIS ASSERTION IS: a REVERT DETECTOR, not a runtime guard.
    // `closing_column` returns `anchor`, which is `out[0].u`, and
    // nothing mutates `out[0]` afterwards — so this compares `out[0].u`
    // with itself and CANNOT go red for any input, however defective.
    // What reds it is a SOURCE EDIT that puts the column back on a
    // derived value, and it does that job well: reverting
    // `closing_column` to `unwrap_near(u_raw, anchor)` reds this on
    // #648's obliquely-placed split frustum wedge at 1 ulp, through the
    // whole-body path rather than through a unit row written for it.
    // Keep it as the acceptance criterion made executable; do not read
    // it as evidence that the closure is being CHECKED at runtime.
    if !no_rim && matches!(travs[m - 1].kind, TravKind::Meridian { .. }) && !out.is_empty() {
        debug_assert_eq!(
            out[0].u, prev_u,
            "a rim-anchored loop must close BITWISE: the final meridian's column is \
             the closing vertex's own azimuth, so these are the same f64"
        );
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

    /// The bar is SPATIAL. The gap that falsified the old bare-radian
    /// form — `nist_ftc_09_asme1_rd.stp` closes at 3.56e-9 rad, over the
    /// old 1e-9 constant — reads as noise at any real lever arm, and
    /// only fails to at an absurd one.
    #[test]
    fn the_closure_bar_is_spatial_not_angular() {
        let eps = 3.38e-5;
        let gap = 3.56e-9;
        assert!(
            gap_is_noise(gap, 0.05, eps),
            "3.56e-9 rad at 50 mm displaces ~1.8e-10 m — far under eps"
        );
        assert!(
            !gap_is_noise(gap, 1e5, eps),
            "the same gap must NOT pass at a 100 km lever arm"
        );
    }

    /// Growing the lever arm tightens the angular bar proportionally —
    /// the property a bare radian constant did not have.
    #[test]
    fn the_closure_bar_tightens_as_the_lever_arm_grows() {
        let eps = 1e-5;
        let gap = 1e-6;
        assert!(gap_is_noise(gap, 1.0, eps), "1e-6 m < eps");
        assert!(
            !gap_is_noise(gap, 100.0, eps),
            "1e-4 m > eps — the same angle, ten thousand times the arc"
        );
    }

    /// On the axis the azimuth carries no length, so every gap is
    /// noise. Must be a plain comparison, not a NaN or a division.
    #[test]
    fn on_the_axis_every_gap_is_noise() {
        assert!(gap_is_noise(core::f64::consts::PI, 0.0, 1e-9));
    }

    /// **THE REGRESSION ROW for the closing column** (S22). It must
    /// return the anchor ITSELF, bitwise — not merely the 2πk branch
    /// nearest it. Reverting to `unwrap_near(u_raw, anchor)` lands in
    /// the same branch, so no branch-level or approximate test can see
    /// the difference; only an exact comparison can, and that exactness
    /// is the whole point (it is what makes the loop closure identically
    /// zero and the ε snap unnecessary).
    ///
    /// Both halves are pinned: the column is `anchor` to the last bit,
    /// AND the value the revert would produce sits in `anchor`'s own
    /// branch, so the wedge rule for θ > 3π/2 is untouched. The raws
    /// are a whole number of turns from the anchor plus a hair inside
    /// the branch — the shape imported geometry produces.
    ///
    /// `skew` is an ABSOLUTE radian offset, not a count of ulps: an
    /// earlier form of this row called it `ulps` and multiplied by
    /// `f64::EPSILON`, which is 2.22e-16 rad flat — a quarter of an ulp
    /// at `anchor = 5.9` and ~1e36 ulps at `anchor = 0.0`. The row does
    /// not depend on the count (its job is to be a real difference, and
    /// the non-vacuity counter below is what checks that), so the
    /// honest spelling is the offset itself.
    #[test]
    fn the_closing_column_is_the_anchor_bitwise_and_in_its_branch() {
        let mut skewed = 0_u32;
        const E: f64 = f64::EPSILON;
        for anchor in [0.0, 0.7, -2.4, core::f64::consts::PI, 5.9] {
            for turns in [-2.0_f64, -1.0, 0.0, 1.0, 2.0] {
                for skew in [0.0_f64, E, -E, 4096.0 * E, -4096.0 * E] {
                    let raw = anchor + turns * TAU + skew;
                    let carrier = unwrap_near(raw, anchor);
                    assert!(
                        (carrier - anchor).abs() < TAU / 2.0,
                        "the replaced form already selects the anchor's branch \
                         (anchor {anchor}, turns {turns}, skew {skew} rad)"
                    );
                    // eps/radius chosen so the detector stays quiet: the
                    // widest skew here is ~9.1e-13 rad, 9.1e-13 m of arc.
                    // The row that makes it fire is the next one.
                    let column = closing_column(raw, anchor, 1.0, 1e-9);
                    assert_eq!(
                        column, anchor,
                        "the closing column must be the anchor bitwise \
                         (anchor {anchor}, turns {turns}, skew {skew} rad)"
                    );
                    if carrier != anchor {
                        skewed += 1;
                    }
                }
            }
        }
        // Non-vacuity: the replaced form really does differ from the
        // anchor on this fixture, so the row above is a live comparison
        // rather than an accident of exact arithmetic.
        assert!(
            skewed >= 20,
            "fixture must exercise real skews; only {skewed} rows differed"
        );
    }

    /// **THE DETECTOR'S RED ROW.** Every other row in this file tests
    /// [`gap_is_noise`] as a predicate; none of them touched the
    /// `debug_assert!` that is its only production consumer, and the
    /// one row that did call [`closing_column`] picked its eps and
    /// radius *so the detector stays quiet*. Deleting that
    /// `debug_assert!` outright left the whole crate green — the same
    /// "the guard has no red row anywhere" shape S22 is about.
    ///
    /// This is that row: a gap far over the spatial bar must panic, and
    /// the message must carry the arc length, because the arc length is
    /// what a reader has to see to classify the file.
    ///
    /// That it was untested was not an oversight anyone could have
    /// caught by running the suite: the assertion cannot change a
    /// returned value (the column is `anchor` either way), so removing
    /// it could only red a row that observes the PANIC — and until this
    /// one there was no `#[should_panic]` anywhere in `crates/mesh`.
    ///
    /// One thing no row can pin: `gap_is_noise`'s first two arguments
    /// multiply, so swapping them at the call site is undetectable by
    /// construction, not merely untested. What IS pinned here is that
    /// the assertion exists, is wired to the predicate, and fires.
    ///
    /// Gated on `debug_assertions` because that is what compiles the
    /// assertion in; a `--release` test run would otherwise see the
    /// function return normally and fail the `should_panic`.
    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "m of arc")]
    fn the_closure_detector_fires_when_the_gap_clears_the_spatial_bar() {
        // 3.56e-9 rad — `nist_ftc_09`'s own residue, which is noise on
        // its 2.97 mm hole — read at a 100 km lever arm, where the same
        // angle is 3.56e-4 m of arc, five orders over eps.
        let anchor = 0.7_f64;
        let _ = closing_column(anchor + 3.56e-9, anchor, 1e5, 1e-9);
    }

    // ---- iso-side runs (#653) -----------------------------------

    fn trav(kind: TravKind, ids: &[u32]) -> Trav {
        Trav {
            ids: ids.to_vec(),
            kind,
        }
    }

    fn rim(ids: &[u32]) -> Trav {
        trav(TravKind::Rim { v_raw: 0.0 }, ids)
    }

    fn meridian(ids: &[u32]) -> Trav {
        trav(TravKind::Meridian { u_raw: 0.0 }, ids)
    }

    /// Positions for the rows below: id 0 is the chart's north pole,
    /// id 1 the south pole, and ids 2.. are ordinary surface points.
    fn unit_sphere_positions() -> Vec<Point3<f64>> {
        vec![
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(0.0, 0.0, -1.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.6, 0.8, 0.0),
        ]
    }

    /// A loop whose every iso side is ONE edge — the shape every
    /// unsplit body has — must open a side at every traversal, so
    /// nothing downstream of `starts` can behave differently from the
    /// pre-#653 walk.
    #[test]
    fn alternating_rims_and_meridians_open_a_side_every_time() {
        let c = z_chart(ChartKind::Sphere { r: 1.0 });
        let p = unit_sphere_positions();
        let travs = vec![
            rim(&[2, 3]),
            meridian(&[3, 4]),
            rim(&[4, 2]),
            meridian(&[2, 2]),
        ];
        assert_eq!(
            iso_side_starts(&travs, &c, &p, 1e-9),
            vec![true, true, true, true]
        );
    }

    /// Two meridians meeting AWAY from the axis are one iso side: a
    /// point off the axis has exactly one azimuth, so there is no such
    /// thing as a meridian-meridian corner there.
    #[test]
    fn consecutive_meridians_off_the_axis_are_one_side() {
        let c = z_chart(ChartKind::Sphere { r: 1.0 });
        let p = unit_sphere_positions();
        let travs = vec![rim(&[2, 3]), meridian(&[3, 4]), meridian(&[4, 2])];
        assert_eq!(
            iso_side_starts(&travs, &c, &p, 1e-9),
            vec![true, true, false]
        );
    }

    /// A POLE junction is a corner, not one side — the fan the walk
    /// emits two entries for, and `unwrap_tie`'s π-apart wire band.
    ///
    /// The chart here is a SPHERE, and that is the whole exercised
    /// class: the predicate is spatial, so a cone's apex takes the same
    /// path with no extra case, and a horn or spindle torus's axis
    /// point would too — but `revolve` refuses those at construction,
    /// so no row here or anywhere can exercise one. Read this as
    /// covering the sphere pole and the cone apex; the torus claim in
    /// `iso_side_starts`' docs is labelled unexercised there.
    #[test]
    fn a_pole_junction_always_breaks_the_side() {
        let c = z_chart(ChartKind::Sphere { r: 1.0 });
        let p = unit_sphere_positions();
        // A pole-to-pole band: two meridians, both junctions a pole.
        let travs = vec![meridian(&[1, 2, 0]), meridian(&[0, 3, 1])];
        assert_eq!(iso_side_starts(&travs, &c, &p, 1e-9), vec![true, true]);
        // ... and the same two meridians with a vertex dropped on the
        // FIRST one are one side across that vertex and two sides
        // across each pole.
        let split = vec![meridian(&[1, 2]), meridian(&[2, 0]), meridian(&[0, 3, 1])];
        assert_eq!(
            iso_side_starts(&split, &c, &p, 1e-9),
            vec![true, false, true]
        );
    }

    /// Rims run too: two coaxial circles at different `v` are
    /// disjoint, so two rims that meet are necessarily co-`v`.
    #[test]
    fn consecutive_rims_off_the_axis_are_one_row() {
        let c = z_chart(ChartKind::Cylinder { r: 1.0 });
        let p = unit_sphere_positions();
        let travs = vec![rim(&[2, 3]), rim(&[3, 4]), meridian(&[4, 2])];
        assert_eq!(
            iso_side_starts(&travs, &c, &p, 1e-9),
            vec![true, false, true]
        );
    }

    /// **The rim-anchor row** (#653). The walk must start at a rim
    /// that OPENS its row, not merely at the first rim.
    ///
    /// The failing condition, which is what makes this row a revert
    /// detector rather than a restatement: the loop's first rim is a
    /// CONTINUATION — a rim row carried by two edges, with the cycle
    /// beginning inside it. `travs[0]` is then the second half of a
    /// row, and the `position(Rim)` this replaces would anchor there
    /// and split the row in the middle. Reverting `walk_anchor` to
    /// `position(Rim)` reds the first assertion; the second and third
    /// pin that nothing else moved.
    #[test]
    fn the_walk_anchors_on_a_rim_that_opens_its_row() {
        let c = z_chart(ChartKind::Cylinder { r: 1.0 });
        let p = unit_sphere_positions();
        // A rim row carried by two edges, with the cycle starting in
        // the middle of it: traversal 0 continues traversal 2.
        let travs = vec![rim(&[2, 3]), meridian(&[3, 4]), rim(&[4, 2])];
        let starts = iso_side_starts(&travs, &c, &p, 1e-9);
        assert_eq!(starts, vec![false, true, true], "fixture precondition");
        assert_eq!(
            walk_anchor(&travs, &starts),
            Some(2),
            "the first rim (index 0) continues the last one; the walk must \
             anchor on index 2, which opens the row"
        );
        // Unsplit shape: every rim opens its row, so this is the
        // `position(Rim)` it replaces, index for index.
        let plain = vec![meridian(&[2, 3]), rim(&[3, 4]), meridian(&[4, 2])];
        let plain_starts = iso_side_starts(&plain, &c, &p, 1e-9);
        assert_eq!(walk_anchor(&plain, &plain_starts), Some(1));
        // Rimless: anchor on any side that opens.
        let band = vec![meridian(&[1, 2]), meridian(&[2, 0]), meridian(&[0, 3, 1])];
        let sphere = z_chart(ChartKind::Sphere { r: 1.0 });
        let band_starts = iso_side_starts(&band, &sphere, &p, 1e-9);
        assert_eq!(walk_anchor(&band, &band_starts), Some(0));
    }

    /// **The closing-side row** (#653). The closure rule and the
    /// pole-band `prev_u` seed both belong to the last iso SIDE, so
    /// `closing_side` must be that side's opening index, not `m - 1`.
    ///
    /// The failing condition: the last iso side is carried by several
    /// edges. Reverting either consumer to `m - 1` makes the closing
    /// meridian take a column its continuations do not repeat (the
    /// closure assertion in `loop_polygon`), or seeds the pole entry
    /// with a neighbour's band value (4.4e-16 m off-box on the ball).
    #[test]
    fn the_closing_side_is_the_last_run_start_not_the_last_index() {
        // Unsplit: every side is one edge, so it IS `m - 1`.
        assert_eq!(closing_side(&[true, true, true]), 2);
        // The last side is carried by two edges: its opening index is
        // 2, and index 3 continues it.
        assert_eq!(closing_side(&[true, true, true, false]), 2);
        // ... by three.
        assert_eq!(closing_side(&[true, true, false, false]), 1);
        // Degenerate: nothing opens (the whole loop is one cyclic run,
        // which `loop_polygon` patches to `true` at 0 before asking).
        assert_eq!(closing_side(&[false, false]), 0);
    }

    /// The junction test is CYCLIC — traversal 0's predecessor is the
    /// last one — which is what lets `loop_polygon` rotate the walk
    /// onto a side that opens rather than into the middle of one.
    #[test]
    fn the_side_test_wraps_around_the_loop() {
        let c = z_chart(ChartKind::Cylinder { r: 1.0 });
        let p = unit_sphere_positions();
        // ids[0] of traversal 0 is 2, shared with the last traversal.
        let travs = vec![meridian(&[2, 3]), rim(&[3, 4]), meridian(&[4, 2])];
        assert_eq!(
            iso_side_starts(&travs, &c, &p, 1e-9),
            vec![false, true, true]
        );
    }
}
