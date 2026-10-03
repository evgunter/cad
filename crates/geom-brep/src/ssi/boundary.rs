//! **The plane × NURBS lane's boundary pass**: the wall's knot
//! rectangle decided against the plane before any march (C3).
//!
//! The march meets the wall's domain boundary only as a box test on
//! marched states, so on its own it cannot tell a plane through a
//! corner from one flush with a side or from a branch shorter than its
//! step. This pass decides the four sides and the four corners once, at
//! ε, before seeding:
//!
//! - **A side** is plane × one boundary curve
//!   ([`crate::boundary_section`]). Either it lies within the band of
//!   the plane, or it meets the plane at isolated roots, each decided
//!   transversal along the side. A side within the band whose slope
//!   across it is one-signed over a strip beside it is a
//!   [`SsiBoundaryContact::Side`] region, or nothing where the strip is
//!   clear of the plane; otherwise its own roots decide it.
//! - **A corner** within the band of the plane is classified by the
//!   plane distance's two inward partials over a corner cell
//!   ([`NurbsBoxes::deriv_box`]), the cell walking the tube ladder.
//!   Both one-signed and of one sign: the locus leaves the domain
//!   there, and the corner is a [`SsiBoundaryContact::Corner`] region,
//!   or, where the corner's own distance has that sign too, nothing at
//!   all. Of opposite signs: a branch starts at the corner. A partial
//!   along a side that is not one-signed at any rung is the locus
//!   tangent to that side, a graze, which refuses naming the side.
//!
//! In the band the pass picks no side: a region asserts no topology,
//! only where the solution set lies. A region is reported exactly where
//! the locus is coincident with its corner or side: its certified zero
//! set, its **cover**, lies inside its cell, and is certified to lie
//! within ε of the corner or side. Otherwise the corner or side is no
//! region, and its roots are ordinary crossings, the curve between them
//! traced as any branch is. A reported region's cell holds no zero
//! beyond its cover, so a root in the cell is the region's; every other
//! root is kept. Whether a vertex lies on a face stays its consumer's
//! decision. Outside the domain the exact empty answer stands. Every
//! root the pass keeps becomes a branch end, settled onto both
//! surfaces, and the crossings are the only ends an open branch on this
//! lane has.

use geom::NurbsSurface;
use geom_core::interval::certification::Certification;
use geom_core::interval::{div_down, div_up, max_bound};
use geom_core::k_stats::decide;
use geom_core::{Band, Interval, Margin, Point3, Sign, Vec3};

use super::enclose::{ChartSpeeds, NurbsBoxes};
use super::exhaust::UvRect;
use super::march::MarchTol;
use super::section::{
    BandVerdict, BoundarySection, SectionRoot, boundary_roots, boundary_section, magnitude,
    settle_root, sign,
};
use super::system::{LocalSystem, ParametricPairR4};
use super::{ChartAxis, SsiError, TraceDecision};

/// How finely a side's strip is cut along the side to bound its reach:
/// at most `2^`this pieces (D9, a fixed rule).
const STRIP_PIECES_LOG2: u32 = 6;
use crate::dihedral::decide_reported;
use crate::nurbs_iso::{boundary_iso_u, boundary_iso_v};
use crate::recourse::Refused;

/// Which end of a parameter's domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartEnd {
    /// The domain's low end.
    Low,
    /// The domain's high end.
    High,
}

/// A side of a NURBS wall's knot rectangle: the iso-line where `fixed`
/// sits at its `end`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChartSide {
    /// The parameter held fixed along the side.
    pub fixed: ChartAxis,
    /// Which end of its domain it is held at.
    pub end: ChartEnd,
}

impl core::fmt::Display for ChartSide {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let axis = match self.fixed {
            ChartAxis::U => "u",
            ChartAxis::V => "v",
        };
        let end = match self.end {
            ChartEnd::Low => "low",
            ChartEnd::High => "high",
        };
        write!(f, "the wall's {axis} = {end} side")
    }
}

/// A corner of a NURBS wall's knot rectangle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChartCorner {
    /// Which end of the `u` domain.
    pub u: ChartEnd,
    /// Which end of the `v` domain.
    pub v: ChartEnd,
}

impl core::fmt::Display for ChartCorner {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let word = |e| match e {
            ChartEnd::Low => "low",
            ChartEnd::High => "high",
        };
        write!(
            f,
            "the wall's (u {}, v {}) corner",
            word(self.u),
            word(self.v)
        )
    }
}

/// Where a branch meets the wall's knot rectangle: a side and the
/// parameter along it (`v` on a `u` side, `u` on a `v` side).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundaryPoint {
    /// The side.
    pub side: ChartSide,
    /// The parameter along the side.
    pub t: f64,
}

/// **A certified region of the wall's boundary the plane meets in the
/// band** — not a contact the kernel decided: within the region the
/// solution set lies within `reach` of the corner or side, and at a
/// corner it is at most one arc. Whether that makes a vertex or an edge lie on a face is the
/// consumer's decision; outside the band no region is reported.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SsiBoundaryContact {
    /// A side of the wall lies within the band of the plane.
    Side {
        /// The side.
        side: ChartSide,
        /// How far from the side the locus may lie, in metres.
        reach: f64,
    },
    /// The plane passes within the band of a corner, and the locus
    /// leaves the domain there.
    Corner {
        /// The corner.
        corner: ChartCorner,
        /// How far from the corner the locus may lie, in metres.
        reach: f64,
    },
}

impl core::fmt::Display for SsiBoundaryContact {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Side { side, reach } => write!(
                f,
                "a region along {side}, which lies within the tolerance of the plane: the \
                 intersection there lies within {reach:e} m of the side"
            ),
            Self::Corner { corner, reach } => write!(
                f,
                "a region at {corner}, which lies within the tolerance of the plane: the \
                 intersection there is at most one arc within {reach:e} m of the corner"
            ),
        }
    }
}

/// A branch end the boundary pass certified: where on the boundary, and
/// the state settled onto both surfaces there.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Crossing {
    /// The boundary point.
    pub at: BoundaryPoint,
    /// The ℝ⁴ state `(plane u, plane v, wall u, wall v)`, settled.
    pub state: [f64; 4],
}

/// What the boundary pass decided.
#[derive(Clone, Debug)]
pub(crate) struct BoundaryPass {
    /// The regions it reports.
    pub contacts: Vec<SsiBoundaryContact>,
    /// The regions' certified cells, which the accounting banks.
    pub regions: Vec<UvRect>,
    /// Cells beside a side or a corner within the band of the plane that
    /// the plane is certified to miss, which the accounting excludes.
    pub clear: Vec<UvRect>,
    /// The branch ends, in side order and ascending along each side.
    pub crossings: Vec<Crossing>,
}

/// The plane, as the pass reads it.
#[derive(Clone, Copy)]
pub(crate) struct PassPlane {
    /// A point on it.
    pub origin: Point3<f64>,
    /// Its unit normal.
    pub normal: Vec3<f64>,
    /// Its chart's `u` axis.
    pub u_ref: Vec3<f64>,
}

/// The four sides, in the pass's fixed order (D9).
const SIDES: [ChartSide; 4] = [
    ChartSide {
        fixed: ChartAxis::U,
        end: ChartEnd::Low,
    },
    ChartSide {
        fixed: ChartAxis::U,
        end: ChartEnd::High,
    },
    ChartSide {
        fixed: ChartAxis::V,
        end: ChartEnd::Low,
    },
    ChartSide {
        fixed: ChartAxis::V,
        end: ChartEnd::High,
    },
];

/// The four corners, in the pass's fixed order (D9).
const CORNERS: [ChartCorner; 4] = [
    ChartCorner {
        u: ChartEnd::Low,
        v: ChartEnd::Low,
    },
    ChartCorner {
        u: ChartEnd::High,
        v: ChartEnd::Low,
    },
    ChartCorner {
        u: ChartEnd::Low,
        v: ChartEnd::High,
    },
    ChartCorner {
        u: ChartEnd::High,
        v: ChartEnd::High,
    },
];

/// The value of a domain at an end.
fn at(domain: (f64, f64), end: ChartEnd) -> f64 {
    match end {
        ChartEnd::Low => domain.0,
        ChartEnd::High => domain.1,
    }
}

/// The first `w` of `domain` in from its `end`, clamped to the domain.
fn cut(d: (f64, f64), end: ChartEnd, w: f64) -> (f64, f64) {
    match end {
        ChartEnd::Low => (d.0, (d.0 + w).min(d.1)),
        ChartEnd::High => ((d.1 - w).max(d.0), d.1),
    }
}

/// Whether the first `w` in from `end` of `domain` lies strictly inside
/// it: a cut that reaches the far end is not one whose far face can be
/// clear of zeros.
fn cut_inside(d: (f64, f64), w: f64) -> bool {
    w < d.1 - d.0
}

/// The inward sign of a domain end: `+1` at the low end, `−1` at the
/// high.
fn inward(end: ChartEnd) -> f64 {
    match end {
        ChartEnd::Low => 1.0,
        ChartEnd::High => -1.0,
    }
}

/// Everything the pass reads, minted once.
pub(crate) struct Pass<'a> {
    /// The wall.
    pub wall: &'a NurbsSurface<f64>,
    /// Its chart speeds.
    pub speeds: ChartSpeeds,
    /// The plane.
    pub plane: PassPlane,
    /// The ℝ⁴ system the crossings are settled on.
    pub sys: &'a ParametricPairR4<'a>,
    /// The march tolerance a settled state meets.
    pub tol: MarchTol,
    /// The section floor, in metres.
    pub floor: f64,
    /// The lever arm and the tube ladder's scale, in metres.
    pub extent: f64,
    /// The run band.
    pub band: Band,
}

/// A side's decided section, with what the corners need from it.
struct SideSection {
    side: ChartSide,
    section: BoundarySection,
}

/// What a side within the band of the plane is.
enum SideClass {
    /// The locus is coincident with it: a region of `reach` over
    /// `strip`, whose zero set lies strictly inside it.
    Region { strip: UvRect, reach: f64 },
    /// The plane misses the strip beside it (the exact empty answer).
    Clear { strip: UvRect },
    /// A strip holds the locus's cover, but no rung holds it within ε
    /// of the side: no region, and an arc there ends on the domain's
    /// sides, where the other sides' roots meet it.
    Apart,
    /// No rung holds the locus's cover inside its strip: the smallest
    /// certified reach.
    Unbounded { reach: f64 },
}

/// What a corner within the band of the plane is.
enum CornerClass {
    /// The locus leaves the domain at the corner and is coincident with
    /// it: a region of `reach` over `cell`, whose zero set lies strictly
    /// inside it.
    Contact { cell: UvRect, reach: f64 },
    /// The plane misses the corner cell (the exact empty answer).
    Empty { cell: UvRect },
    /// A branch starts at the corner.
    Start { cell: UvRect },
    /// No rung holds the locus within ε of the corner: the corner's
    /// roots are ordinary crossings, and the arc between them is traced.
    Through,
}

impl Pass<'_> {
    /// The wall's `(u, v)` domain.
    fn domain(&self) -> ((f64, f64), (f64, f64)) {
        (self.wall.knots_u().domain(), self.wall.knots_v().domain())
    }

    /// The side's curve: a row of a clamped wall's net, verbatim. Every
    /// `KnotVector` is clamped by construction, so no extraction beyond
    /// the copy is needed.
    fn curve(&self, side: ChartSide) -> Result<geom::NurbsCurve3<f64>, SsiError> {
        let high = side.end == ChartEnd::High;
        let row = match side.fixed {
            ChartAxis::U => boundary_iso_u(self.wall, high),
            ChartAxis::V => boundary_iso_v(self.wall, high),
        };
        row.map_err(|_| SsiError::UnsupportedCertificate {
            what: "a NURBS wall's boundary row is not valid spline structure",
        })
    }

    /// The certified speed along a side: the chart speed of the
    /// parameter that varies along it.
    fn along_speed(&self, side: ChartSide) -> geom_core::SupSpeed<f64> {
        match side.fixed {
            ChartAxis::U => self.speeds.v,
            ChartAxis::V => self.speeds.u,
        }
    }

    /// The plane distance's partials over a chart rectangle.
    fn partials(&self, r: UvRect) -> (Interval, Interval) {
        let boxes = NurbsBoxes::new(self.wall);
        let n = [
            self.plane.normal.x,
            self.plane.normal.y,
            self.plane.normal.z,
        ]
        .map(Interval::point);
        let dot = |b: super::enclose::Box3| n[0] * b.x + n[1] * b.y + n[2] * b.z;
        (
            dot(boxes.deriv_box(r.u.0, r.u.1, r.v.0, r.v.1, true)),
            dot(boxes.deriv_box(r.u.0, r.u.1, r.v.0, r.v.1, false)),
        )
    }

    /// The plane distance of a corner: a clamped wall's corner is its
    /// net's corner control point, so this encloses it exactly.
    fn corner_distance(&self, c: ChartCorner) -> Interval {
        let (nu, nv) = (
            self.wall.knots_u().control_count(),
            self.wall.knots_v().control_count(),
        );
        let iu = match c.u {
            ChartEnd::Low => 0,
            ChartEnd::High => nu - 1,
        };
        let iv = match c.v {
            ChartEnd::Low => 0,
            ChartEnd::High => nv - 1,
        };
        let Some(&p) = self.wall.control().get(iu * nv + iv) else {
            return Interval::refused();
        };
        let n = self.plane.normal;
        let o = self.plane.origin;
        Interval::point(n.x) * (Interval::point(p.x) - Interval::point(o.x))
            + Interval::point(n.y) * (Interval::point(p.y) - Interval::point(o.y))
            + Interval::point(n.z) * (Interval::point(p.z) - Interval::point(o.z))
    }

    /// The ladder's rungs: the tube ladder's radii, widest first, each
    /// crossed into chart pads per axis.
    fn rungs(&self) -> Vec<(f64, f64)> {
        super::certify::tube_ladder(self.extent, self.band)
            .map(|r| self.speeds.pad(r))
            .collect()
    }

    /// A strip along `side`, `pad` deep into the domain.
    fn strip(&self, side: ChartSide, pad: (f64, f64)) -> UvRect {
        let ((u0, u1), (v0, v1)) = self.domain();
        match side.fixed {
            ChartAxis::U => UvRect {
                u: cut((u0, u1), side.end, pad.0),
                v: (v0, v1),
            },
            ChartAxis::V => UvRect {
                u: (u0, u1),
                v: cut((v0, v1), side.end, pad.1),
            },
        }
    }

    /// A cell at `corner`, `pad` deep along each axis.
    fn corner_cell(&self, corner: ChartCorner, pad: (f64, f64)) -> UvRect {
        let ((u0, u1), (v0, v1)) = self.domain();
        UvRect {
            u: cut((u0, u1), corner.u, pad.0),
            v: cut((v0, v1), corner.v, pad.1),
        }
    }

    /// **A side within the band of the plane.** The first rung whose
    /// strip has the wall's slope across the side one-signed decides the
    /// side against the plane: the strip is clear where the side is on
    /// one side of the plane and the wall moves further that way inward
    /// (the exact empty answer), and otherwise the plane lies along it.
    ///
    /// Along it, a zero of the strip lies at most `sup / inf|φ⊥|` in from
    /// the side (`φ⊥` the slope across it), the strip's **cover**, and at
    /// most `sup · s⊥ / inf|φ⊥|` from it in metres, both read piecewise
    /// along the side ([`Pass::strip_reach`]). The region is reported at
    /// the widest rung, from the first one-signed rung down, whose cover
    /// lies strictly inside the strip (so the zero set meets the strip's
    /// far face nowhere, and an arc in the strip ends on the domain's
    /// sides) and whose distance from the side is at most ε; its reach
    /// is that distance plus ε.
    ///
    /// `Ok(None)` where no rung's slope across the side is one-signed:
    /// the strip certificate has nothing to stand on, and the side's own
    /// roots decide it.
    ///
    /// # Errors
    ///
    /// [`SsiError::BoundaryTangent`] where the slope across the side
    /// does not clear the band. Where a rung's cover lies inside its
    /// strip but none within ε of the side, the side is
    /// [`SideClass::Apart`]; where none lies inside its strip, it is
    /// [`SideClass::Unbounded`], and its own roots decide it.
    fn side_region(
        &self,
        side: ChartSide,
        sup: f64,
        side_of_plane: Option<bool>,
    ) -> Result<Option<SideClass>, SsiError> {
        let speed = match side.fixed {
            ChartAxis::U => self.speeds.u,
            ChartAxis::V => self.speeds.v,
        };
        let ((u0, u1), (v0, v1)) = self.domain();
        let across_domain = match side.fixed {
            ChartAxis::U => (u0, u1),
            ChartAxis::V => (v0, v1),
        };
        let rungs = self.rungs();
        let mut classified = false;
        let mut covered = false;
        let mut best_reach = f64::INFINITY;
        for pad in rungs {
            let strip = self.strip(side, pad);
            let (pu, pv) = self.partials(strip);
            let across = match side.fixed {
                ChartAxis::U => pu,
                ChartAxis::V => pv,
            };
            if !one_signed(across) {
                continue;
            }
            let inf = super::enclose::zero_free_lower_bound(across);
            if !classified {
                classified = true;
                // The side on one side of the plane, and the wall moving
                // further that way inward: the strip is clear of the plane.
                let rising_inward = sign(across) == Some(inward(side.end) > 0.0);
                if side_of_plane == Some(rising_inward) {
                    return Ok(Some(SideClass::Clear { strip }));
                }
                // The sine of the angle between the wall and the plane
                // across the side, levered as the march's transversality
                // is.
                let sine = div_down(inf, speed.get());
                let margin = Margin::levered(sine, self.extent);
                let verdict = match decide_reported("ssi_boundary_strip", margin, self.band) {
                    Ok(decided) => Refused::of(decided, self.band).map(BandVerdict::Refused),
                    Err(cause) => Some(BandVerdict::Undecided(cause)),
                };
                if let Some(verdict) = verdict {
                    return Err(SsiError::BoundaryTangent { side, verdict });
                }
            }
            let (depth, distance) = self.strip_reach(side, strip, sup);
            let pad_across = match side.fixed {
                ChartAxis::U => pad.0,
                ChartAxis::V => pad.1,
            };
            let reach = distance + self.band.zero();
            best_reach = best_reach.min(reach);
            let inside = depth < pad_across || !cut_inside(across_domain, pad_across);
            if inside && distance <= self.band.zero() {
                return Ok(Some(SideClass::Region { strip, reach }));
            }
            covered |= inside;
        }
        Ok(match (classified, covered) {
            (false, _) => None,
            (true, true) => Some(SideClass::Apart),
            (true, false) => Some(SideClass::Unbounded { reach: best_reach }),
        })
    }

    /// How deep in from `side` a zero of `strip` lies, in parameter, and
    /// how far from the side, in metres: the strip cut along the side
    /// into `2^j` pieces, `j` up to [`STRIP_PIECES_LOG2`], until the
    /// distance is within ε.
    /// A zero at `(t⊥, t)` in a piece is reached from the side at the
    /// same `t` inside the piece, where the wall's slope across is at
    /// least the piece's `inf|φ⊥|` and its speed across at most the
    /// piece's `s⊥`, so it lies at most `sup / inf|φ⊥|` in and
    /// `sup · s⊥ / inf|φ⊥|` from the side; the strip's bounds are the
    /// largest over its pieces. The rational wall's derivative boxes
    /// over a whole side pair its least slope with its greatest speed,
    /// which pieces part. `(∞, ∞)` where a piece's slope is not
    /// one-signed.
    fn strip_reach(&self, side: ChartSide, strip: UvRect, sup: f64) -> (f64, f64) {
        let boxes = NurbsBoxes::new(self.wall);
        let along_u = side.fixed == ChartAxis::U;
        let along = if along_u { strip.v } else { strip.u };
        let mut best = (f64::INFINITY, f64::INFINITY);
        for j in 0..=STRIP_PIECES_LOG2 {
            let n = 1u32 << j;
            let mut bound = (0.0_f64, 0.0_f64);
            for k in 0..n {
                let t = |i: u32| along.0 + (along.1 - along.0) * f64::from(i) / f64::from(n);
                let piece = if k + 1 == n {
                    (t(k), along.1)
                } else {
                    (t(k), t(k + 1))
                };
                let piece = if along_u {
                    UvRect {
                        u: strip.u,
                        v: piece,
                    }
                } else {
                    UvRect {
                        u: piece,
                        v: strip.v,
                    }
                };
                let d = boxes.deriv_box(piece.u.0, piece.u.1, piece.v.0, piece.v.1, along_u);
                let n_ = [
                    self.plane.normal.x,
                    self.plane.normal.y,
                    self.plane.normal.z,
                ]
                .map(Interval::point);
                let across = n_[0] * d.x + n_[1] * d.y + n_[2] * d.z;
                let inf = if one_signed(across) {
                    super::enclose::zero_free_lower_bound(across)
                } else {
                    0.0
                };
                let speed = boxes.speed_sup(piece.u.0, piece.u.1, piece.v.0, piece.v.1, along_u);
                if !(inf > 0.0 && speed.is_finite()) {
                    bound = (f64::INFINITY, f64::INFINITY);
                    break;
                }
                bound = (
                    max_bound(bound.0, div_up(sup, inf)),
                    max_bound(bound.1, div_up(sup, div_down(inf, speed))),
                );
            }
            if bound.1 < best.1 {
                best = bound;
            }
            if best.1 <= self.band.zero() {
                break;
            }
        }
        best
    }

    /// **A corner within the band of the plane**, classified by the
    /// plane distance's inward partials over the widest corner cell of
    /// the ladder where both are one-signed (module docs). Where the
    /// locus leaves the domain there, a zero of the cell at inward
    /// offsets `(du, dv)` has `inf|φ_u|·du + inf|φ_v|·dv ≤ |φ(corner)|`,
    /// so it lies in the **cover** `du ≤ |φ|/inf|φ_u|`,
    /// `dv ≤ |φ|/inf|φ_v|`, and at most
    /// `reach = |φ| · max(s_u/inf|φ_u|, s_v/inf|φ_v|)` from the corner
    /// (the largest of `s_u·du + s_v·dv` under the constraint, `s_u`,
    /// `s_v` the wall's speeds over the cell). The region is reported at
    /// the widest rung, from the classifying one down, whose cover lies
    /// strictly inside the cell and whose distance from the corner is at
    /// most ε; its reach is that distance plus ε.
    ///
    /// # Errors
    ///
    /// [`SsiError::BoundaryGraze`] naming the side along which the
    /// partial is never one-signed. Where no rung holds the locus within
    /// ε of the corner it is [`CornerClass::Through`].
    fn corner_class(&self, corner: ChartCorner, phi: Interval) -> Result<CornerClass, SsiError> {
        let rungs = self.rungs();
        let Some(&last_pad) = rungs.last() else {
            return Err(SsiError::TubeLadderEmpty {
                extent: self.extent,
                floor: super::certify::SSI_TUBE_RADIUS * self.band.zero(),
            });
        };
        let first = rungs.iter().position(|pad| {
            let (pu, pv) = self.partials(self.corner_cell(corner, *pad));
            one_signed(pu) && one_signed(pv)
        });
        let Some(first) = first else {
            let cell = self.corner_cell(corner, last_pad);
            let (pu, _) = self.partials(cell);
            // A partial along a side that never clears zero: the locus is
            // tangent to that side at the corner. `φ_u` runs along the
            // `v` side, `φ_v` along the `u` side.
            let side = if one_signed(pu) {
                ChartSide {
                    fixed: ChartAxis::U,
                    end: corner.u,
                }
            } else {
                ChartSide {
                    fixed: ChartAxis::V,
                    end: corner.v,
                }
            };
            let bracket = match side.fixed {
                ChartAxis::U => cell.v,
                ChartAxis::V => cell.u,
            };
            return Err(graze(side, bracket, self.extent, self.band));
        };
        let cell = self.corner_cell(corner, rungs[first]);
        let (pu, pv) = self.partials(cell);
        // The inward partials' signs.
        let su = sign(pu) == Some(inward(corner.u) > 0.0);
        let sv = sign(pv) == Some(inward(corner.v) > 0.0);
        if su != sv {
            return Ok(CornerClass::Start { cell });
        }
        // Both inward partials carry φ away from the corner's value with
        // sign `su`: a corner distance of that sign keeps the cell clear.
        if sign(phi) == Some(su) {
            return Ok(CornerClass::Empty { cell });
        }
        let ((u0, u1), (v0, v1)) = self.domain();
        let mag = magnitude(phi);
        let boxes = NurbsBoxes::new(self.wall);
        for &pad in &rungs[first..] {
            let cell = self.corner_cell(corner, pad);
            let (pu, pv) = self.partials(cell);
            if !(one_signed(pu) && one_signed(pv)) {
                continue;
            }
            // The wall's speeds over the cell, which bound the path from
            // the corner to a zero in it.
            let speed =
                |along_u: bool| boxes.speed_sup(cell.u.0, cell.u.1, cell.v.0, cell.v.1, along_u);
            let (s_u, s_v) = (speed(true), speed(false));
            let inf_u = super::enclose::zero_free_lower_bound(pu);
            let inf_v = super::enclose::zero_free_lower_bound(pv);
            let (du, dv) = (div_up(mag, inf_u), div_up(mag, inf_v));
            let (ru, rv) = (
                div_up(mag, div_down(inf_u, s_u)),
                div_up(mag, div_down(inf_v, s_v)),
            );
            let distance = max_bound(ru, rv);
            let inside = (du < pad.0 || !cut_inside((u0, u1), pad.0))
                && (dv < pad.1 || !cut_inside((v0, v1), pad.1));
            if inside && distance <= self.band.zero() {
                return Ok(CornerClass::Contact {
                    cell,
                    reach: distance + self.band.zero(),
                });
            }
        }
        Ok(CornerClass::Through)
    }

    /// The state at a boundary point settled onto both surfaces, or
    /// [`SsiError::EndNotOnLocus`].
    fn settle(&self, side: ChartSide, root: SectionRoot) -> Result<Crossing, SsiError> {
        let ((u0, u1), (v0, v1)) = self.domain();
        let curve = self.curve(side)?;
        let tol = self.tol.settling();
        let refuse = || SsiError::EndNotOnLocus {
            side,
            bracket: root.bracket,
        };
        let t = settle_root(
            &curve,
            self.plane.origin,
            self.plane.normal,
            root.bracket,
            tol,
        )
        .ok_or_else(refuse)?;
        if t.is_nan() {
            return Err(SsiError::UnsupportedCertificate {
                what: super::exhaust::SIDE_ENCLOSURE_REFUSED,
            });
        }
        let (u, v) = match side.fixed {
            ChartAxis::U => (at((u0, u1), side.end), t),
            ChartAxis::V => (t, at((v0, v1), side.end)),
        };
        let q = self.wall.eval(u, v) - self.plane.origin;
        let v_ref = self.plane.normal.cross(self.plane.u_ref);
        let state = [q.dot(self.plane.u_ref), q.dot(v_ref), u, v];
        if !self.sys.residual(&state).iter().all(|r| r.abs() <= tol) {
            return Err(refuse());
        }
        Ok(Crossing {
            at: BoundaryPoint { side, t },
            state,
        })
    }

    /// **The pass** (module docs).
    ///
    /// # Errors
    ///
    /// [`SsiError::BoundaryGraze`], [`SsiError::BoundaryTangent`],
    /// [`SsiError::EndNotOnLocus`], and the section's refusals.
    pub(crate) fn run(&self) -> Result<BoundaryPass, SsiError> {
        let ((u0, u1), (v0, v1)) = self.domain();
        let mut contacts = Vec::new();
        let mut regions: Vec<UvRect> = Vec::new();
        let mut clear: Vec<UvRect> = Vec::new();
        let mut sections = Vec::new();
        // The first side within the band that no region holds, and its
        // smallest certified reach.
        let mut unbounded: Option<(ChartSide, f64)> = None;
        for side in SIDES {
            let curve = self.curve(side)?;
            let section = boundary_section(
                &curve,
                self.plane.origin,
                self.plane.normal,
                self.along_speed(side),
                self.floor,
                self.extent,
                self.band,
            )
            .map_err(|e| e.on_side(side))?;
            let section = match section {
                BoundarySection::On { sup, side_of_plane } => {
                    match self.side_region(side, sup, side_of_plane)? {
                        Some(SideClass::Region { strip, reach }) => {
                            contacts.push(SsiBoundaryContact::Side { side, reach });
                            regions.push(strip);
                            section
                        }
                        Some(SideClass::Clear { strip }) => {
                            clear.push(strip);
                            section
                        }
                        // The locus lies inside a strip beside the side, so
                        // an arc there ends on the domain's sides; the side
                        // itself, within the band along its length, has no
                        // root to decide, and the corners on it are its.
                        Some(SideClass::Apart) => section,
                        // The side lies within the band of the plane, and
                        // no strip beside it holds the locus: no rung has
                        // the wall's slope across it one-signed (a side
                        // shorter than the band that the plane crosses is
                        // the case met), or none holds its cover inside
                        // the strip. The side's own roots decide it; a
                        // plane tangent to the wall along it refuses as
                        // their graze.
                        class => {
                            if let (None, Some(SideClass::Unbounded { reach })) = (unbounded, &class)
                            {
                                unbounded = Some((side, *reach));
                            }
                            boundary_roots(
                                &curve,
                                self.plane.origin,
                                self.plane.normal,
                                self.along_speed(side),
                                self.floor,
                                self.extent,
                                self.band,
                            )
                            .map_err(|e| e.on_side(side))?
                        }
                    }
                }
                roots => roots,
            };
            sections.push(SideSection { side, section });
        }
        let on = |s: ChartSide| {
            sections
                .iter()
                .any(|x| x.side == s && matches!(x.section, BoundarySection::On { .. }))
        };
        // The corners: each end root of a side lands at one.
        let mut starts: Vec<UvRect> = Vec::new();
        for corner in CORNERS {
            let through = [
                ChartSide {
                    fixed: ChartAxis::U,
                    end: corner.u,
                },
                ChartSide {
                    fixed: ChartAxis::V,
                    end: corner.v,
                },
            ];
            if through.iter().any(|s| on(*s)) {
                // A side region covers the corner.
                continue;
            }
            let phi = self.corner_distance(corner);
            let margin = Margin::of(magnitude(phi));
            match decide("ssi_boundary_corner", margin, self.band) {
                Ok(Sign::Positive) => continue,
                Ok(Sign::Zero | Sign::Negative) => {}
                Err(cause) if cause.margin.is_invalid() => {
                    return Err(TraceDecision::BoundarySection.escalated(cause));
                }
                Err(_) => {}
            }
            match self.corner_class(corner, phi)? {
                CornerClass::Empty { cell } => clear.push(cell),
                CornerClass::Contact { cell, reach } => {
                    contacts.push(SsiBoundaryContact::Corner { corner, reach });
                    regions.push(cell);
                }
                CornerClass::Start { cell } => starts.push(cell),
                CornerClass::Through => {}
            }
        }
        // The crossings: every root not inside a region; an end root at
        // a corner within the band only where a branch starts there, and
        // once per corner.
        let mut crossings: Vec<Crossing> = Vec::new();
        for SideSection { side, section } in &sections {
            let BoundarySection::Roots {
                interior,
                at_start,
                at_end,
            } = section
            else {
                continue;
            };
            let mut roots: Vec<SectionRoot> = Vec::new();
            for (end_root, end) in [(at_start, ChartEnd::Low), (at_end, ChartEnd::High)] {
                let Some(root) = end_root else { continue };
                let corner = match side.fixed {
                    ChartAxis::U => ChartCorner {
                        u: side.end,
                        v: end,
                    },
                    ChartAxis::V => ChartCorner {
                        u: end,
                        v: side.end,
                    },
                };
                let point = (at((u0, u1), corner.u), at((v0, v1), corner.v));
                let in_start = starts.iter().any(|c| holds(*c, point));
                // Inside a region's cell the root is the region's; in a
                // cell the plane is certified clear of, it is none.
                if regions.iter().chain(&clear).any(|c| holds(*c, point)) {
                    continue;
                }
                if !in_start {
                    // A root within the floor of a corner the plane is
                    // decided clear of: an ordinary crossing.
                    super::section::decide_crossing(*root, self.extent, self.band)
                        .map_err(|e| e.on_side(*side))?;
                }
                roots.push(*root);
            }
            roots.extend(interior.iter().copied());
            for root in roots {
                let c = self.settle(*side, root)?;
                let point = (c.state[2], c.state[3]);
                // A region's cell holds its zero set only within the
                // cover, so a root in the cell is the region's, wherever
                // its settling put it.
                if regions.iter().any(|r| holds(*r, point)) {
                    continue;
                }
                // One crossing per corner a branch starts at.
                let twin = starts.iter().any(|cell| {
                    holds(*cell, point)
                        && crossings
                            .iter()
                            .any(|o| holds(*cell, (o.state[2], o.state[3])))
                });
                if twin {
                    continue;
                }
                crossings.push(c);
            }
        }
        // A side within the band that no region holds, and no crossing
        // anywhere to trace from: nothing to trace and nothing to bank.
        if let (Some((side, reach)), true) = (unbounded, crossings.is_empty()) {
            return Err(SsiError::RegionUnbounded { side, reach });
        }
        Ok(BoundaryPass {
            contacts,
            regions,
            clear,
            crossings,
        })
    }
}

/// Whether a chart point lies in a rectangle.
fn holds(r: UvRect, (u, v): (f64, f64)) -> bool {
    u >= r.u.0 && u <= r.u.1 && v >= r.v.0 && v <= r.v.1
}

/// Whether an enclosure is certified and one-signed.
fn one_signed(i: Interval) -> bool {
    sign(i).is_some()
}

/// The graze refusal at `side` where the slope along it is not
/// one-signed: the crossing decision on a zero slope, which refuses.
fn graze(side: ChartSide, bracket: (f64, f64), arm: f64, band: Band) -> SsiError {
    let root = SectionRoot {
        bracket,
        slope: 0.0,
        rising: None,
    };
    match super::section::decide_crossing(root, arm, band) {
        Err(e) => e.on_side(side),
        // A zero margin decides zero at every band, which refuses.
        Ok(()) => unreachable!(
            "a zero slope decided positive at {side}: the crossing decision passes only a \
             margin clear of the band"
        ),
    }
}

/// The plane chart's window holds the wall: every state the ℝ⁴ trace
/// settles lies on the wall, inside its control hull (positive weights),
/// and on the plane, so its plane coordinates lie in the hull's
/// projection. A window that holds that projection never bounds a
/// march, which ends only at the wall's own boundary.
///
/// # Errors
///
/// [`SsiError::WindowShortOfWall`] when it does not.
pub(crate) fn window_holds_wall(
    plane: (Point3<f64>, Vec3<f64>, Vec3<f64>),
    wall: &NurbsSurface<f64>,
    half: f64,
) -> Result<(), SsiError> {
    let (p0, u_ref, v_ref) = plane;
    let lift = |v: Vec3<f64>| [v.x, v.y, v.z].map(Interval::point);
    let (u, v) = (lift(u_ref), lift(v_ref));
    let reach = wall
        .control()
        .iter()
        .flat_map(|p| {
            let q = [(p.x, p0.x), (p.y, p0.y), (p.z, p0.z)]
                .map(|(a, b)| Interval::point(a) - Interval::point(b));
            [u, v].map(|axis| {
                let c = axis[0] * q[0] + axis[1] * q[1] + axis[2] * q[2];
                magnitude(c)
            })
        })
        .fold(0.0, max_bound);
    if reach <= half {
        Ok(())
    } else {
        Err(SsiError::WindowShortOfWall {
            half_extent: half,
            reach,
        })
    }
}
