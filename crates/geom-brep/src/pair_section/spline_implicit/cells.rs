//! **A fitted arc's cells**: where on the wall a point of the arc can
//! stand, and so the region a site is read on the arc in and the region
//! two arcs are held apart by ([`apart`]).
//!
//! Limb 3 holds the section in the windows of the arc's chart tube one
//! arc, a piece of it in each, each window cut to the wall's knot
//! rectangle (`ssi::one_arc`), and the reading's accounting banks every
//! other cell of the rectangle. So every point of the arc is the wall's
//! image of a chart point in some cut window. A chart cell whose image
//! the partner's form puts wholly past the band holds none of them, nor
//! any site: it is cleared. The arc's cells are the cut windows less
//! their cleared parts ([`Enclosure`]), a node's own box kept whole.
//!
//! - **The pad** of a cell is the farthest its image stands from the
//!   carrier, read against the whole carrier, and the arc's the widest
//!   of its cells'.
//! - **The first reading** halves each window until a cell's image is no
//!   wider than [`FIRST_CELL_RUNGS`] rungs, [`FIRST_CUTS`] times at most.
//! - **Refinement** ([`apart`]) halves the cells of two arcs that stand
//!   within reach of each other until the section's arcs read apart, or
//!   stops as `crate::descent` stops a refining loop.

use geom::NurbsSurface;
use geom_core::{Band, Bounds as _, Interval, Point3, Real};

use super::super::{
    CarriedComponent, CarryError, CarryRefusal, ComponentShape, LoopTubeRefusal, NoCarrier, PadBox,
    PadsStop, TubeRun, farthest, parts, witness_reach,
};
use super::Bracketed;
use crate::descent::{Descent, Stop, Work};
use crate::ssi::enclose::{Box3, NurbsBoxes};
use crate::ssi::exhaust::UvRect;
use crate::ssi::{SsiBranch, SsiTube};

#[cfg(test)]
mod rows;

/// How many rungs wide a cell's image is read at first, at most. A cell
/// that is not cleared holds a point of the wall its enclosure does not
/// put past the band of the partner; that is a point near the section,
/// not on it, and how near is the enclosure's slack. Its farthest point
/// stands within its diagonal of that point, so at one rung wide the
/// first reading's pad is about a rung and a half of the section, plus
/// that slack and half the spacing of the carrier's points
/// ([`SAMPLES_PER_RUNG`]). The carry rows hold it within two rungs as a
/// measurement, not a bound.
const FIRST_CELL_RUNGS: f64 = 1.0;

/// How many times the first reading halves a window, at most, toward
/// cells no wider than [`FIRST_CELL_RUNGS`] rungs. The cleared halves
/// are dropped as they are cut, so the cells kept lie along the arc: a
/// window `2¹²` rungs long is read to the rung.
const FIRST_CUTS: u32 = 12;

/// How many carrier points a rung of the carrier's length is read
/// against: they stand evenly over the arc's whole span, its ends among
/// them, a quarter rung apart where the carrier's speed is even.
const SAMPLES_PER_RUNG: f64 = 4.0;

/// The fewest and the most carrier points an arc is read against.
const SAMPLES: (usize, usize) = (17, 1025);

/// The strides the walk to the carrier point nearest a box takes, widest
/// first.
const SAMPLE_STRIDES: [usize; 4] = [64, 16, 4, 1];

/// **How many rounds make one cycle of [`apart`]'s halvings**: a cell is
/// halved along the chart axis it is longer along, so two rounds halve
/// it along both. The stall is read once a cycle: a round that halves
/// along the arcs clears nothing, and says nothing.
const CYCLE: u32 = 2;

/// **The work bound of [`apart`]'s refinement, per cell of the section's
/// first reading** (`crate::descent`'s cumulative budget, in cells
/// halved over every round and arc). A section whose meeting cells are
/// all kept, round after round, stalls after two idle cycles, four
/// rounds, having halved `1 + 2 + 4 + 8 = 15` times the cells it began
/// with. Sixteen a cell lets that stall speak before the bound, so the
/// bound ends only a refinement that is still narrowing, and it grows
/// with the section: a long arc of many cells is given the rounds a
/// short one is.
const REFINE_PER_CELL: usize = 16;

/// The least [`apart`]'s work bound is, in cells halved: a section of a
/// few cells narrows a round at a time for as long as one of a hundred
/// and twenty-eight does at [`REFINE_PER_CELL`].
const REFINE_FLOOR: usize = 1 << 11;

/// One cell of an arc: a chart rectangle inside a cut window of its
/// tube, an enclosure of its image on the wall, the farthest that stands
/// from the carrier, the window it is a part of, and the node whose box
/// it is.
#[derive(Clone, Copy)]
struct Cell {
    rect: UvRect,
    image: Box3,
    pad: f64,
    /// The carrier point its pad was read nearest, by index.
    near: usize,
    window: usize,
    node: Option<usize>,
}

/// `rect` halved along the chart axis it is longer along, measured in
/// `pads` (the tube's, a rung over each axis's chart speed, so in metres
/// at the wall's fastest); `None` where neither axis has a midpoint
/// strictly inside.
fn halves_of(rect: UvRect, pads: (f64, f64)) -> Option<[UvRect; 2]> {
    let ((u0, u1), (v0, v1)) = (rect.u, rect.v);
    let (um, vm) = (geom::mid_param(u0, u1), geom::mid_param(v0, v1));
    let across_u = [(u0, um), (um, u1)].map(|u| UvRect { u, v: rect.v });
    let across_v = [(v0, vm), (vm, v1)].map(|v| UvRect { u: rect.u, v });
    match (u0 < um && um < u1, v0 < vm && vm < v1) {
        (true, true) if (u1 - u0) / pads.0 >= (v1 - v0) / pads.1 => Some(across_u),
        (true, false) => Some(across_u),
        (_, true) => Some(across_v),
        (false, false) => None,
    }
}

/// What a cell is imaged, cleared and read by.
struct Reader<'a> {
    boxes: NurbsBoxes<'a, f64>,
    partner: Bracketed<'a>,
    /// The band's escalation: with the partner's lift, the distance from
    /// the partner past which a cell is cleared.
    escalate: f64,
    samples: Vec<Point3<Interval>>,
    /// The tube's pads along `u` and `v`, in chart units.
    pads: (f64, f64),
}

fn unread(why: LoopTubeRefusal) -> CarryError {
    CarryError::Refused(CarryRefusal::TubeUnread(why))
}

impl<'a> Reader<'a> {
    /// The reader of the branch `b`'s cells on `net` against `partner`
    /// at `band`, its tube's rung `rung` and pads `pads`, with the
    /// witness `carrier(mid)` as an enclosure: the carrier's points stand
    /// evenly over the branch's span,
    /// [`SAMPLES_PER_RUNG`] a rung of the control polygon's length, which
    /// is no shorter than the carrier.
    fn new(
        b: &SsiBranch,
        carrier: &geom::NurbsCurve3<f64>,
        net: &'a NurbsSurface<f64>,
        partner: Bracketed<'a>,
        (rung, pads): (f64, (f64, f64)),
        band: Band,
    ) -> (Self, Point3<Interval>) {
        let lift = <Interval as Real>::from_computed;
        let lifted = carrier.map_scalar(lift);
        let length: f64 = carrier
            .control()
            .windows(2)
            .map(|p| (p[1] - p[0]).norm())
            .sum();
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let count =
            ((SAMPLES_PER_RUNG * length / rung).ceil() as usize + 1).clamp(SAMPLES.0, SAMPLES.1);
        let samples = (0..count)
            .map(|k| {
                let t = if k + 1 == count {
                    b.params.1
                } else {
                    #[allow(clippy::cast_precision_loss)]
                    let share = k as f64 / (count - 1) as f64;
                    b.params.0 + (b.params.1 - b.params.0) * share
                };
                lifted.eval(lift(t))
            })
            .collect();
        let reader = Self {
            boxes: NurbsBoxes::new(net),
            partner,
            escalate: band.escalate(),
            samples,
            pads,
        };
        let witness = lifted.eval(lift(geom::mid_param(b.params.0, b.params.1)));
        (reader, witness)
    }

    /// Whether the partner's form, in metres over `image`, lies wholly
    /// past the band: no point of `image` stands within the band's
    /// escalation of the partner. The form's own `f` is read, the band
    /// padded by its lift (`ImplicitForm::plus_lift`'s rule for a
    /// distance). A refused enclosure, or a lift that is no number,
    /// clears nothing.
    fn clears(&self, image: Box3) -> bool {
        let form = self.partner.form;
        let q = form.framed(image);
        let metres = form.floor_metres(form.value(q), q);
        let cut = self.escalate + form.lift().abs().hi();
        metres.is_certified() && (metres.lo() > cut || metres.hi() < -cut)
    }

    /// An enclosure of `rect`'s image on the wall, met with `within`, an
    /// enclosure of a rectangle holding it.
    fn enclosed(&self, rect: UvRect, within: Option<Box3>) -> Result<Box3, CarryError> {
        let uv = rect
            .window()
            .ok_or_else(|| unread(LoopTubeRefusal::NotFinite))?;
        let image = self.boxes.rect_box(uv);
        Ok(within.map_or(image, |w| image.meet_enclosure(w)))
    }

    /// [`Self::enclosed`], `None` where the image is cleared.
    fn image(&self, rect: UvRect, within: Option<Box3>) -> Result<Option<Box3>, CarryError> {
        let image = self.enclosed(rect, within)?;
        Ok((!self.clears(image)).then_some(image))
    }

    /// A carrier point near `b` in the farthest-point sense, by index,
    /// and how far `b` reaches from it: walked from the `hint`th while a
    /// neighbour at a stride reads nearer. Any carrier point bounds the
    /// reach, so the walk's end need not be the nearest. `NaN` where the
    /// hint does not read.
    fn nearest(&self, b: Box3, hint: usize) -> (usize, f64) {
        let last = self.samples.len() - 1;
        let far = |k: usize| farthest(b, self.samples[k]);
        let mut best = (hint.min(last), far(hint.min(last)));
        for stride in SAMPLE_STRIDES {
            loop {
                let from = best.0;
                let sides = [
                    from.checked_sub(stride),
                    Some(from + stride).filter(|&k| k <= last),
                ];
                for k in sides.into_iter().flatten() {
                    let f = far(k);
                    if f < best.1 {
                        best = (k, f);
                    }
                }
                if best.0 == from {
                    break;
                }
            }
        }
        best
    }

    /// The cell of `rect` imaged in `image`, a part of `window`. Its pad
    /// is **the farthest `image` stands from the carrier**, in metres:
    /// the image is halved into parts (`pair_section::parts`), and a
    /// part's every point is no farther from the carrier than from any
    /// one carrier point, here one walked to from the `hint`th.
    fn cell(
        &self,
        (rect, image): (UvRect, Box3),
        hint: usize,
        window: usize,
        node: Option<usize>,
    ) -> Result<Cell, CarryError> {
        let (near, _) = self.nearest(image, hint);
        let pad = parts(image).into_iter().try_fold(0.0f64, |acc, part| {
            let far = self.nearest(part, near).1;
            if far.is_finite() {
                Ok(acc.max(far))
            } else {
                Err(unread(LoopTubeRefusal::of(part, &self.samples)))
            }
        })?;
        Ok(Cell {
            rect,
            image,
            pad,
            near,
            window,
            node,
        })
    }

    /// `rect`'s halves ([`halves_of`]), each with its image, the cleared
    /// ones dropped; `None` where it has none.
    fn halves(
        &self,
        (rect, image): (UvRect, Box3),
    ) -> Result<Option<Vec<(UvRect, Box3)>>, CarryError> {
        let Some(cut) = halves_of(rect, self.pads) else {
            return Ok(None);
        };
        let mut out = Vec::with_capacity(2);
        for r in cut {
            if let Some(i) = self.image(r, Some(image))? {
                out.push((r, i));
            }
        }
        Ok(Some(out))
    }
}

/// **A fitted arc's cells** (module docs), with what reads them.
pub(super) struct Enclosure<'a> {
    reader: Reader<'a>,
    /// The witness `carrier(mid)`, enclosed.
    witness: Point3<Interval>,
    /// Each window's span of the carrier, in the windows' order.
    spans: Vec<(f64, f64)>,
    /// Which windows are a node's box.
    node_windows: Vec<bool>,
    /// The cells, in their windows' order.
    cells: Vec<Cell>,
    /// The nodes the arc's ends stand at, start first.
    ends: [Option<usize>; 2],
    hull_sup: f64,
    /// The witness's parameter.
    at: f64,
}

impl<'a> Enclosure<'a> {
    /// **The first reading of the branch `b`'s cells** on the wall `net`
    /// against `partner`, its ends at the section's nodes `ends` (start
    /// first): each window of its certificate's chart tube
    /// (`ssi::certify::chart_tube`) cut to the knot rectangle and halved
    /// to cells no wider than [`FIRST_CELL_RUNGS`] rungs, the cleared
    /// ones dropped. A node's box is one cell, never cleared.
    ///
    /// # Errors
    ///
    /// A desync where the certificate is not a chart tube, the branch
    /// carries no wall pcurve, its carrier is not a spline, its node
    /// ends are not its certificate's, or a window lies off the knot
    /// rectangle; [`CarryRefusal::TubeUnread`] where the windows or a
    /// cell do not read.
    pub(super) fn first(
        b: &SsiBranch,
        net: &'a NurbsSurface<f64>,
        partner: Bracketed<'a>,
        ends: [Option<usize>; 2],
        band: Band,
    ) -> Result<Self, CarryError> {
        let (
            SsiTube::Chart {
                pad_u,
                pad_v,
                rung,
                nodes,
            },
            Some(pcurve),
        ) = (b.certificate.tube, b.pcurve_b.as_ref())
        else {
            return Err(CarryError::Desync(
                "a spline × implicit branch holds no chart tube on the wall's pcurve",
            ));
        };
        let geom::Curve3::Nurbs(carrier) = &b.carrier else {
            return Err(CarryError::Desync(
                "a spline × implicit branch's carrier is not a spline",
            ));
        };
        // The listing's node ends and the certificate's are one fact, read
        // twice: the branch's tips, and the node boxes limb 3 proved.
        if [0, 1]
            .iter()
            .any(|&e| ends[e].is_some() != nodes[e].is_some())
        {
            return Err(CarryError::Desync(
                "a spline × implicit arc's node ends are not its certificate's node boxes",
            ));
        }
        let windows = crate::ssi::certify::chart_tube(pcurve, (pad_u, pad_v), nodes)
            .ok_or_else(|| unread(LoopTubeRefusal::Uncertified))?;
        let (reader, witness) = Reader::new(b, carrier, net, partner, (rung, (pad_u, pad_v)), band);
        let domain = reader.boxes.domain();
        let last_sample = reader.samples.len() - 1;
        let last = windows.len() - 1;
        let mut cells = Vec::new();
        for (k, w) in windows.iter().enumerate() {
            let whole = crate::ssi::one_arc::meet(w.rect, domain).ok_or(CarryError::Desync(
                "a window of a certified chart tube lies off the wall's knot rectangle",
            ))?;
            // The carrier point at the middle of the window's span.
            #[allow(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                clippy::cast_precision_loss
            )]
            let hint = (((w.mid - b.params.0) / (b.params.1 - b.params.0)).clamp(0.0, 1.0)
                * last_sample as f64) as usize;
            if w.node {
                let node = match k {
                    0 => ends[0],
                    _ if k == last => ends[1],
                    _ => None,
                };
                let image = reader.enclosed(whole, None)?;
                cells.push(reader.cell((whole, image), hint, k, node)?);
                continue;
            }
            let Some(image) = reader.image(whole, None)? else {
                continue;
            };
            let mut open = vec![((whole, image), 0u32)];
            while let Some((cell, depth)) = open.pop() {
                // A cell no wider than `FIRST_CELL_RUNGS` rungs, or one
                // cut as often as a window is, is read as it stands.
                let narrow = cell.1.width().partial_cmp(&(FIRST_CELL_RUNGS * rung))
                    != Some(core::cmp::Ordering::Greater);
                let halves = if narrow || depth == FIRST_CUTS {
                    None
                } else {
                    reader.halves(cell)?
                };
                match halves {
                    Some(halves) => open.extend(halves.into_iter().map(|h| (h, depth + 1))),
                    None => cells.push(reader.cell(cell, hint, k, None)?),
                }
            }
        }
        Ok(Self {
            reader,
            witness,
            spans: windows.iter().map(|w| w.ends).collect(),
            node_windows: windows.iter().map(|w| w.node).collect(),
            cells,
            ends,
            hull_sup: b.certificate.hull_sup,
            at: geom::mid_param(b.params.0, b.params.1),
        })
    }

    /// Whether `c` can be halved: it is no node's box, kept whole, and
    /// its rectangle has a midpoint strictly inside along an axis.
    fn halves(&self, c: &Cell) -> bool {
        c.node.is_none() && halves_of(c.rect, self.reader.pads).is_some()
    }

    /// The cells with those marked in `meets` that can be halved halved,
    /// the cleared halves dropped; beside them the indices of the halves
    /// and of the cells marked in `near`, which stay in the next round's
    /// scan whether or not they were halved.
    fn halved(&self, meets: &[bool], near: &[bool]) -> Result<(Vec<Cell>, Vec<usize>), CarryError> {
        let mut cells = Vec::with_capacity(self.cells.len() + meets.len());
        let mut active = Vec::new();
        for (k, c) in self.cells.iter().enumerate() {
            let halves = if meets[k] && self.halves(c) {
                self.reader.halves((c.rect, c.image))?
            } else {
                None
            };
            match halves {
                Some(halves) => {
                    for half in halves {
                        active.push(cells.len());
                        cells.push(self.reader.cell(half, c.near, c.window, c.node)?);
                    }
                }
                None => {
                    if near[k] {
                        active.push(cells.len());
                    }
                    cells.push(*c);
                }
            }
        }
        Ok((cells, active))
    }

    /// A cell's pad as a site reads it: the farthest its image stands
    /// from the carrier, and no less than the carrier's limb-2 bound, so
    /// the carrier lies within it of each surface too
    /// ([`CarriedComponent::pad`], closeness).
    fn site_pad(&self, c: &Cell) -> f64 {
        c.pad.max(self.hull_sup)
    }

    /// **The component the arc carries** (`ClassCarrier::Fitted`): the
    /// branch's certified carrier over its span, its certificate, its
    /// witness `carrier(mid)`, its cells as the boxes a site is read in
    /// ([`CarriedComponent::boxes`]) and, hulled window by window, the
    /// runs its foot is read over ([`CarriedComponent::runs`]), its pad
    /// the widest of theirs, and its reach: the farthest the cells of a
    /// window whose span holds the witness's parameter stand from the
    /// witness, the least over such windows (`pair_section::witness_reach`).
    /// It claims no winding: an open arc winds about neither surface.
    ///
    /// # Errors
    ///
    /// A desync where a window not a node's box holds no cell, though
    /// limb 3 puts a piece of the arc in it, or where no window's span
    /// holds the witness; [`CarryRefusal::PadShort`] where the limb-2
    /// bound is no number; [`CarryRefusal::TubeUnread`] where the
    /// witness or a cell's image does not read.
    pub(super) fn carried(
        &self,
        b: &SsiBranch,
        shape: ComponentShape,
    ) -> Result<CarriedComponent<f64>, CarryError> {
        let of_window = |k: usize| -> Vec<Box3> {
            self.cells
                .iter()
                .filter(|c| c.window == k)
                .map(|c| c.image)
                .collect()
        };
        let windows: Vec<Vec<Box3>> = (0..self.spans.len()).map(of_window).collect();
        if windows
            .iter()
            .zip(&self.node_windows)
            .any(|(cells, &node)| cells.is_empty() && !node)
        {
            return Err(CarryError::Desync(
                "a window of a rim arc's tube holds no cell the partner's form does not clear",
            ));
        }
        let pad = self
            .cells
            .iter()
            .map(|c| self.site_pad(c))
            .fold(0.0, f64::max);
        if self.hull_sup.is_nan() {
            return Err(CarryError::Refused(CarryRefusal::PadShort {
                pad,
                hull_sup: self.hull_sup,
            }));
        }
        let holding: Vec<&[Box3]> = (0..self.spans.len())
            .filter(|&k| self.spans[k].0 <= self.at && self.at <= self.spans[k].1)
            .map(|k| windows[k].as_slice())
            .collect();
        let reach = witness_reach(self.witness, &holding)?;
        let not_finite = || unread(LoopTubeRefusal::NotFinite);
        let boxes = self
            .cells
            .iter()
            .map(|c| PadBox::of(c.image, self.site_pad(c), c.node).ok_or_else(not_finite))
            .collect::<Result<Vec<_>, CarryError>>()?;
        // Each window's cells hulled, with the carrier's span over the
        // window: where on the carrier a site in them lies.
        let runs = windows
            .iter()
            .zip(&self.spans)
            .filter_map(|(cells, &span)| {
                let hull = cells.iter().copied().reduce(Box3::hull)?;
                Some(TubeRun::of(hull, span).ok_or_else(not_finite))
            })
            .collect::<Result<Vec<_>, CarryError>>()?;
        Ok(CarriedComponent {
            carrier: b.carrier.clone(),
            span: b.params,
            shape,
            certificate: Some(b.certificate),
            winds: [false, false],
            pad,
            reach,
            witness: b.witness,
            boxes,
            runs,
        })
    }
}

/// Whether the enclosures `a` and `b`, each widened by `by` along every
/// axis, share no point: along some axis the one's widened end falls
/// short of the other's. It is the complement of two boxes both holding
/// one site (`pair_section::box_holds`, the same sums), so cells apart
/// by it are read apart. A refused enclosure is apart from nothing.
fn widened_apart(a: Box3, b: Box3, by: f64) -> bool {
    let apart = |s: Interval, t: Interval| {
        s.is_certified()
            && t.is_certified()
            && (s.hi() + by < t.lo() - by || t.hi() + by < s.lo() - by)
    };
    apart(a.x, b.x) || apart(a.y, b.y) || apart(a.z, b.z)
}

/// How far `a` and `b` stand apart along the axis they stand farthest
/// apart on, zero where they share a point; `NaN` where either is
/// refused.
fn axis_gap(a: Box3, b: Box3) -> f64 {
    let gap = |s: Interval, t: Interval| {
        if s.is_certified() && t.is_certified() {
            (s.lo() - t.hi()).max(t.lo() - s.hi()).max(0.0)
        } else {
            f64::NAN
        }
    };
    [gap(a.x, b.x), gap(a.y, b.y), gap(a.z, b.z)]
        .into_iter()
        .fold(0.0, |acc, g| if g.is_nan() { g } else { acc.max(g) })
}

/// How two cells of two arcs stand to each other.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Stand {
    /// Widened by the band's escalation, they share no point.
    Apart,
    /// They share one, and one of them meets the box of a node both arcs
    /// end at.
    AtNode,
    /// They share one, standing this far apart along the axis they stand
    /// farthest apart on, in metres: `NaN` where a box is refused, which
    /// holds nothing apart.
    Meet(f64),
}

/// How the cells imaged in `a` and `b` stand, each widened by `by` (the
/// band's escalation, as a site's read widens them), `node_boxes` the
/// boxes of the nodes both their arcs end at.
pub(super) fn stand(a: Box3, b: Box3, node_boxes: &[Box3], by: f64) -> Stand {
    if widened_apart(a, b, by) {
        return Stand::Apart;
    }
    if [a, b]
        .iter()
        .any(|&r| node_boxes.iter().any(|&n| !widened_apart(r, n, 0.0)))
    {
        return Stand::AtNode;
    }
    Stand::Meet(axis_gap(a, b))
}

/// **A listed component as the carry reads it**: a fitted arc's cells,
/// or their refusal, or why the component has no carrier.
pub(super) enum Listed<'a> {
    /// A rim arc to mint.
    Arc(Result<Box<Enclosure<'a>>, CarryError>),
    /// A component that carries none.
    Bare(NoCarrier),
}

impl<'a> Listed<'a> {
    /// A rim arc with its first reading, or that reading's refusal.
    pub(super) fn arc(first: Result<Enclosure<'a>, CarryError>) -> Self {
        Self::Arc(first.map(Box::new))
    }

    /// The arc's cells, where it is an arc whose cells read.
    fn cells(&self) -> Option<&Enclosure<'a>> {
        match self {
            Self::Arc(Ok(e)) => Some(e.as_ref()),
            _ => None,
        }
    }
}

/// An arc's cells ordered along `x`, so that a cell of another arc is
/// stood only against those not already apart from it along `x`.
struct Swept {
    /// The cells, by index, in the order of their images' least `x`.
    order: Vec<usize>,
    /// Each one's least `x`, in that order: empty where an image is
    /// refused, and every cell is then stood against.
    least: Vec<f64>,
    /// The widest image along `x`.
    widest: f64,
}

impl Swept {
    fn along_x(e: &Enclosure<'_>, mut order: Vec<usize>) -> Self {
        let x = |k: usize| e.cells[k].image.x;
        if !order.iter().all(|&k| x(k).is_certified()) {
            return Self {
                order,
                least: Vec::new(),
                widest: 0.0,
            };
        }
        order.sort_by(|&p, &q| x(p).lo().total_cmp(&x(q).lo()));
        Self {
            least: order.iter().map(|&k| x(k).lo()).collect(),
            widest: order
                .iter()
                .map(|&k| x(k).hi() - x(k).lo())
                .fold(0.0, f64::max),
            order,
        }
    }

    /// The cells that may share a point with `c`, each widened by `by`:
    /// all but those whose image ends short of `c`'s along `x`, or
    /// starts past it, by more than twice `by` and a margin for the
    /// sums' rounding. [`stand`] decides each.
    fn near(&self, c: Box3, by: f64) -> &[usize] {
        if self.least.is_empty() || !c.x.is_certified() {
            return &self.order;
        }
        let reach = 2.0 * by * (1.0 + 1e-9) + 1e-9 * c.x.lo().abs().max(c.x.hi().abs());
        let from = c.x.lo() - self.widest - reach;
        let to = c.x.hi() + reach;
        let lo = self.least.partition_point(|&x| x < from);
        let hi = self.least.partition_point(|&x| x <= to);
        &self.order[lo..hi]
    }
}

/// One round's scan of the section's active cells.
struct Scan {
    /// Per arc and cell: within reach of another arc's cell, at a shared
    /// node or not.
    near: Vec<Vec<bool>>,
    /// Per arc and cell: meeting another arc's cell.
    meets: Vec<Vec<bool>>,
    /// Each arc's nearest meeting: the gap, and the two cells' pads.
    worst: Vec<Option<(f64, [f64; 2])>>,
}

impl Scan {
    /// How every two arcs' `active` cells stand, each widened by `by`.
    fn of(arcs: &[Listed<'_>], active: &[Vec<usize>], by: f64) -> Self {
        let flags = || -> Vec<Vec<bool>> {
            arcs.iter()
                .map(|a| vec![false; a.cells().map_or(0, |e| e.cells.len())])
                .collect()
        };
        let (mut near, mut meets) = (flags(), flags());
        let mut worst: Vec<Option<(f64, [f64; 2])>> = vec![None; arcs.len()];
        for (i, a) in arcs.iter().enumerate() {
            let Some(a) = a.cells() else { continue };
            for (j, b) in arcs.iter().enumerate().skip(i + 1) {
                let Some(b) = b.cells() else { continue };
                let shared =
                    |n: Option<usize>| n.is_some() && a.ends.contains(&n) && b.ends.contains(&n);
                // The boxes of the nodes both arcs end at: a cell meeting
                // one is the node's, where the two arcs meet.
                let node_boxes: Vec<Box3> = a
                    .cells
                    .iter()
                    .chain(&b.cells)
                    .filter(|c| shared(c.node))
                    .map(|c| c.image)
                    .collect();
                // Only a cell within the reach of the other arc's cells'
                // hull can stand within it of one of them.
                let hull = |e: &Enclosure<'_>, live: &[usize]| {
                    live.iter().map(|&k| e.cells[k].image).reduce(Box3::hull)
                };
                let (Some(hull_a), Some(hull_b)) = (hull(a, &active[i]), hull(b, &active[j]))
                else {
                    continue;
                };
                let within = |e: &Enclosure<'_>, live: &[usize], other: Box3| -> Vec<usize> {
                    live.iter()
                        .copied()
                        .filter(|&k| !widened_apart(e.cells[k].image, other, by))
                        .collect()
                };
                let live_a = within(a, &active[i], hull_b);
                let live_b = Swept::along_x(b, within(b, &active[j], hull_a));
                for &ka in &live_a {
                    let ca = &a.cells[ka];
                    for &kb in live_b.near(ca.image, by) {
                        let cb = &b.cells[kb];
                        let gap = match stand(ca.image, cb.image, &node_boxes, by) {
                            Stand::Apart => continue,
                            Stand::AtNode => None,
                            Stand::Meet(gap) => Some(gap),
                        };
                        near[i][ka] = true;
                        near[j][kb] = true;
                        let Some(gap) = gap else { continue };
                        meets[i][ka] = true;
                        meets[j][kb] = true;
                        let pads = [a.site_pad(ca), b.site_pad(cb)];
                        for k in [i, j] {
                            if worst[k].is_none_or(|(g, _)| {
                                gap.partial_cmp(&g) != Some(core::cmp::Ordering::Greater)
                            }) {
                                worst[k] = Some((gap, pads));
                            }
                        }
                    }
                }
            }
        }
        Self { near, meets, worst }
    }

    /// How many meeting cells can be halved.
    fn halvable(&self, arcs: &[Listed<'_>]) -> usize {
        arcs.iter()
            .zip(&self.meets)
            .filter_map(|(a, m)| Some((a.cells()?, m)))
            .map(|(a, m)| {
                a.cells
                    .iter()
                    .zip(m)
                    .filter(|&(c, &m)| m && a.halves(c))
                    .count()
            })
            .sum()
    }
}

/// **The section's arcs held apart, cell by cell, or carrying neither**
/// ([`CarryRefusal::PadsMeet`], per component, `None` for one that
/// stands apart or is no arc). A site is read on an arc where it stands
/// in one of the arc's cells, each widened by the band's escalation
/// along every axis ([`CarriedComponent::reads`]). A point read on two
/// arcs that share no node is a desync, and one read on two that share
/// a node escalates on its distance from it (`CarriedSection::site`), so
/// every two arcs' cells, widened so, share no point ([`stand`]), except
/// where one of the two meets the box of a node both arcs end at: there
/// the arcs meet, and a site escalates.
///
/// **Refined until that resolves**: the cells of two arcs that meet are
/// halved, the cleared halves dropped, round by round, until no pair is
/// left, so an arc with no close neighbour keeps its first reading.
/// Short of that the loop stops through `crate::descent`
/// ([`crate::descent::Descent::stop`]), and the arcs still meeting carry
/// neither, the refusal naming the stop:
///
/// - **the stall** ([`PadsStop::Stall`]): the pair has stopped
///   narrowing. What is watched is the share of the halves still
///   meeting, the meeting cells that can be halved over `2^round`, read
///   once a [`CYCLE`]. It falls whenever a half is cleared or parts from
///   the other arc, and holds where every half is kept. Two cycles
///   running that do not lower it stall, as does a round in which no
///   meeting cell can be halved (a node's box is kept whole, and is not
///   counted);
/// - **the work bound** ([`PadsStop::Work`]): the next round would pass
///   [`REFINE_PER_CELL`] halvings a cell of the section's first reading
///   ([`REFINE_FLOOR`] at least).
///
/// An arc whose cell does not read in a round carries that refusal and
/// is held apart from none. The carry takes no section one of whose
/// components refuses (`PairSection::carry` collects every component's
/// result), so no arc is carried on an apartness that arc was left out
/// of.
pub(super) fn apart(arcs: &mut [Listed<'_>], band: Band) -> Vec<Option<CarryRefusal>> {
    let by = band.escalate();
    let count = |a: &Listed<'_>| a.cells().map_or(0, |e| e.cells.len());
    let mut active: Vec<Vec<usize>> = arcs.iter().map(|a| (0..count(a)).collect()).collect();
    let first: usize = arcs.iter().map(count).sum();
    let mut work = Work::new(REFINE_FLOOR.max(REFINE_PER_CELL.saturating_mul(first)));
    let mut descent: Option<Descent<()>> = None;
    for round in 0u32.. {
        let scan = Scan::of(arcs, &active, by);
        if scan.worst.iter().all(Option::is_none) {
            break;
        }
        let cost = scan.halvable(arcs);
        if round % CYCLE == 0 {
            #[allow(clippy::cast_precision_loss)]
            let share = cost as f64 * 0.5f64.powi(i32::try_from(round).unwrap_or(i32::MAX));
            descent = Some(Descent::after(descent.take(), share, ()));
        }
        let wall = (!work.admits(cost)).then_some(PadsStop::Work);
        let stop = descent.as_ref().and_then(|d| d.stop(cost == 0, wall));
        if let Some(stop) = stop {
            let stop = match stop {
                Stop::Stall => PadsStop::Stall,
                Stop::Wall(wall) => wall,
            };
            return scan
                .worst
                .iter()
                .map(|w| w.map(|(gap, pads)| CarryRefusal::PadsMeet { pads, gap, stop }))
                .collect();
        }
        work.spend(cost);
        for (i, arc) in arcs.iter_mut().enumerate() {
            let Listed::Arc(Ok(e)) = arc else { continue };
            match e.halved(&scan.meets[i], &scan.near[i]) {
                Ok((cells, live)) => {
                    e.cells = cells;
                    active[i] = live;
                }
                Err(refusal) => {
                    *arc = Listed::Arc(Err(refusal));
                    active[i].clear();
                }
            }
        }
    }
    vec![None; arcs.len()]
}
