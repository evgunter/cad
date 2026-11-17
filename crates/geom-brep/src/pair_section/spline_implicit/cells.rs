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
    CarriedComponent, CarryError, CarryRefusal, ComponentShape, LoopTubeRefusal, PadBox, farthest,
    parts,
};
use super::{Bracketed, box_gap};
use crate::descent::{Descent, Work};
use crate::ssi::enclose::{Box3, NurbsBoxes, UvWindow};
use crate::ssi::{SsiBranch, SsiTube};

#[cfg(test)]
mod rows;

/// A chart rectangle `[u0, u1] × [v0, v1]`, as `(u0, u1, v0, v1)`.
type Rect = (f64, f64, f64, f64);

/// How many rungs wide a cell's image is read at first, at most. A
/// cell that is not cleared holds a point within the band of the
/// partner, up to its enclosure's slack, so its farthest point stands
/// within its diagonal of the section: at one rung wide, within two
/// rungs.
const FIRST_CELL_RUNGS: f64 = 1.0;

/// How many times the first reading halves a window, at most, toward
/// cells no wider than [`FIRST_CELL_RUNGS`] rungs. The cleared halves
/// are dropped as they are cut, so the cells kept lie along the arc: a
/// window `2¹²` rungs long is read to the rung.
const FIRST_CUTS: u32 = 12;

/// The carrier points a cell is read against, evenly over the arc's
/// whole span, its ends among them.
const CARRIER_SAMPLES: usize = 1025;

/// The stride of the first pass over the carrier's points for the one
/// nearest a box; the second reads every point within a stride of it.
const SAMPLE_STRIDE: usize = 16;

/// How many times the carrier point nearest a cell is settled between
/// its neighbours, by golden section.
const SETTLE_STEPS: u32 = 12;

/// **The work bound of [`apart`]'s refinement** (`crate::descent`'s
/// cumulative budget): the cells it halves, over every round and every
/// arc of one section.
const REFINE_CELLS: usize = 1 << 11;

/// One cell of an arc: a chart rectangle inside a cut window of its
/// tube, an enclosure of its image on the wall, the farthest that stands
/// from the carrier, the window it is a part of, and the node whose box
/// it is.
#[derive(Clone, Copy)]
struct Cell {
    rect: Rect,
    image: Box3,
    pad: f64,
    window: usize,
    node: Option<usize>,
}

/// What a cell is imaged, cleared and read by.
struct Reader<'a> {
    boxes: NurbsBoxes<'a, f64>,
    partner: Bracketed<'a>,
    /// The distance from the partner past which a cell is cleared: the
    /// band's escalation.
    cut: f64,
    carrier: geom::NurbsCurve3<Interval>,
    span: (f64, f64),
    samples: Vec<Point3<Interval>>,
}

fn unread(why: LoopTubeRefusal) -> CarryError {
    CarryError::Refused(CarryRefusal::TubeUnread(why))
}

impl Reader<'_> {
    /// Whether the partner's form, in metres over `image`, lies wholly
    /// past the cut: no point of `image` stands within the band of the
    /// partner. A refused enclosure clears nothing.
    fn clears(&self, image: Box3) -> bool {
        let form = self.partner.form;
        let q = form.framed(image);
        let metres = form.floor_metres(form.plus_lift(form.value(q)), q);
        metres.is_certified() && (metres.lo() > self.cut || metres.hi() < -self.cut)
    }

    /// The image of `rect` on the wall, met with `within`, an enclosure
    /// of a rectangle holding it; `None` where it is cleared.
    fn image(
        &self,
        (u0, u1, v0, v1): Rect,
        within: Option<Box3>,
    ) -> Result<Option<Box3>, CarryError> {
        let uv = UvWindow::new(u0, u1, v0, v1).ok_or_else(|| unread(LoopTubeRefusal::NotFinite))?;
        let image = self.boxes.rect_box(uv);
        let image = within.map_or(image, |w| image.meet_enclosure(w));
        Ok((!self.clears(image)).then_some(image))
    }

    /// The carrier's parameter at its `k`th point.
    fn param(&self, k: usize) -> f64 {
        let last = self.samples.len() - 1;
        #[allow(clippy::cast_precision_loss)]
        let share = k as f64 / last as f64;
        self.span.0 + (self.span.1 - self.span.0) * share
    }

    /// The carrier point nearest `b` in the farthest-point sense, by
    /// index, and how far `b` reaches from it: a strided pass, then
    /// every point within a stride of its best. `+∞` where none reads.
    fn nearest(&self, b: Box3) -> (usize, f64) {
        let last = self.samples.len() - 1;
        let better = |best: (usize, f64), k: usize| {
            let far = farthest(b, self.samples[k]);
            if far < best.1 { (k, far) } else { best }
        };
        let coarse = (0..=last)
            .step_by(SAMPLE_STRIDE)
            .chain([last])
            .fold((0, f64::INFINITY), better);
        let around = coarse.0.saturating_sub(SAMPLE_STRIDE)..=(coarse.0 + SAMPLE_STRIDE).min(last);
        around.fold(coarse, better)
    }

    /// A carrier point between the neighbours of the `k`th, settled
    /// toward the one `b` reaches least from, by golden section.
    fn settled(&self, b: Box3, k: usize) -> Point3<Interval> {
        let lift = <Interval as Real>::from_computed;
        let last = self.samples.len() - 1;
        let (mut lo, mut hi) = (
            self.param(k.saturating_sub(1)),
            self.param((k + 1).min(last)),
        );
        let at = |t: f64| {
            let p = self.carrier.eval(lift(t));
            (farthest(b, p), p)
        };
        let g = 0.5 * (5f64.sqrt() - 1.0);
        let mut best = (farthest(b, self.samples[k]), self.samples[k]);
        for _ in 0..SETTLE_STEPS {
            let (x1, x2) = (hi - g * (hi - lo), lo + g * (hi - lo));
            let (f1, f2) = (at(x1), at(x2));
            if f1.0 < f2.0 {
                hi = x2;
            } else {
                lo = x1;
            }
            for f in [f1, f2] {
                if f.0 < best.0 {
                    best = f;
                }
            }
        }
        best.1
    }

    /// **The farthest `image` stands from the carrier**, in metres: the
    /// image is halved into parts (`pair_section::parts`), and a part's
    /// every point is no farther from the carrier than from any one
    /// carrier point, the nearest read of the whole span's and one
    /// settled for the image.
    fn pad(&self, image: Box3) -> Result<f64, CarryError> {
        let settled = self.settled(image, self.nearest(image).0);
        parts(image).into_iter().try_fold(0.0f64, |acc, part| {
            let far = self.nearest(part).1.min(farthest(part, settled));
            if far.is_finite() {
                Ok(acc.max(far))
            } else {
                Err(unread(LoopTubeRefusal::of(part, &self.samples)))
            }
        })
    }

    /// `rect` halved along the chart axis that leaves the narrower
    /// images, each half with its image, the cleared halves dropped;
    /// `None` where neither axis has a midpoint strictly inside.
    fn halves(&self, rect: Rect, image: Box3) -> Result<Option<Vec<(Rect, Box3)>>, CarryError> {
        let (u0, u1, v0, v1) = rect;
        let (um, vm) = (geom::mid_param(u0, u1), geom::mid_param(v0, v1));
        let cut = |a: Rect, b: Rect| -> Result<Vec<(Rect, Box3)>, CarryError> {
            let mut out = Vec::with_capacity(2);
            for r in [a, b] {
                if let Some(i) = self.image(r, Some(image))? {
                    out.push((r, i));
                }
            }
            Ok(out)
        };
        let across_u = (u0 < um && um < u1)
            .then(|| cut((u0, um, v0, v1), (um, u1, v0, v1)))
            .transpose()?;
        let across_v = (v0 < vm && vm < v1)
            .then(|| cut((u0, u1, v0, vm), (u0, u1, vm, v1)))
            .transpose()?;
        let widest = |h: &[(Rect, Box3)]| h.iter().map(|(_, i)| i.width()).fold(0.0, f64::max);
        Ok(match (across_u, across_v) {
            (Some(u), Some(v)) => Some(if widest(&u) <= widest(&v) { u } else { v }),
            (one, other) => one.or(other),
        })
    }
}

/// **A fitted arc's cells** (module docs), with what reads them.
pub(super) struct Enclosure<'a> {
    reader: Reader<'a>,
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
        let lift = <Interval as Real>::from_computed;
        let lifted = carrier.map_scalar(lift);
        let boxes = NurbsBoxes::new(net);
        let domain = boxes.domain();
        let mut reader = Reader {
            boxes,
            partner,
            cut: band.escalate(),
            carrier: lifted,
            span: b.params,
            samples: Vec::new(),
        };
        reader.samples = (0..CARRIER_SAMPLES)
            .map(|k| {
                let t = if k + 1 == CARRIER_SAMPLES {
                    b.params.1
                } else {
                    #[allow(clippy::cast_precision_loss)]
                    let share = k as f64 / (CARRIER_SAMPLES - 1) as f64;
                    b.params.0 + (b.params.1 - b.params.0) * share
                };
                reader.carrier.eval(lift(t))
            })
            .collect();
        let last = windows.len() - 1;
        let mut cells = Vec::new();
        for (k, w) in windows.iter().enumerate() {
            let cut = crate::ssi::one_arc::meet(w.rect, domain).ok_or(CarryError::Desync(
                "a window of a certified chart tube lies off the wall's knot rectangle",
            ))?;
            let whole = (cut.u.0, cut.u.1, cut.v.0, cut.v.1);
            if w.node {
                let node = match k {
                    0 => ends[0],
                    _ if k == last => ends[1],
                    _ => None,
                };
                let uv = UvWindow::new(whole.0, whole.1, whole.2, whole.3)
                    .ok_or_else(|| unread(LoopTubeRefusal::NotFinite))?;
                let image = reader.boxes.rect_box(uv);
                cells.push(Cell {
                    rect: whole,
                    image,
                    pad: reader.pad(image)?,
                    window: k,
                    node,
                });
                continue;
            }
            let Some(image) = reader.image(whole, None)? else {
                continue;
            };
            let mut open = vec![(whole, image, 0u32)];
            while let Some((rect, image, depth)) = open.pop() {
                // A cell no wider than `FIRST_CELL_RUNGS` rungs, or one
                // cut as often as a window is, is read as it stands.
                let narrow = image.width().partial_cmp(&(FIRST_CELL_RUNGS * rung))
                    != Some(core::cmp::Ordering::Greater);
                let halves = if narrow || depth == FIRST_CUTS {
                    None
                } else {
                    reader.halves(rect, image)?
                };
                match halves {
                    Some(halves) => open.extend(halves.into_iter().map(|(r, i)| (r, i, depth + 1))),
                    None => cells.push(Cell {
                        rect,
                        image,
                        pad: reader.pad(image)?,
                        window: k,
                        node: None,
                    }),
                }
            }
        }
        Ok(Self {
            reader,
            spans: windows.iter().map(|w| w.ends).collect(),
            node_windows: windows.iter().map(|w| w.node).collect(),
            cells,
            ends,
            hull_sup: b.certificate.hull_sup,
            at: geom::mid_param(b.params.0, b.params.1),
        })
    }

    /// The cells with those marked in `meets` halved, a node's box
    /// excepted, and the cleared halves dropped; beside them the indices
    /// of the halves and of the cells marked in `near`, and whether any
    /// cell was halved.
    fn halved(
        &self,
        meets: &[bool],
        near: &[bool],
    ) -> Result<(Vec<Cell>, Vec<usize>, bool), CarryError> {
        let mut cells = Vec::with_capacity(self.cells.len() + meets.len());
        let mut active = Vec::new();
        let mut any = false;
        for (k, &c) in self.cells.iter().enumerate() {
            let halves = if meets[k] && c.node.is_none() {
                self.reader.halves(c.rect, c.image)?
            } else {
                None
            };
            match halves {
                Some(halves) => {
                    any = true;
                    for (rect, image) in halves {
                        active.push(cells.len());
                        cells.push(Cell {
                            rect,
                            image,
                            pad: self.reader.pad(image)?,
                            ..c
                        });
                    }
                }
                None => {
                    if near[k] {
                        active.push(cells.len());
                    }
                    cells.push(c);
                }
            }
        }
        Ok((cells, active, any))
    }

    /// A cell's pad as a site reads it: the farthest its image stands
    /// from the carrier, and no less than the carrier's limb-2 bound, so
    /// the carrier lies within it of each surface too.
    fn pad_of(&self, c: &Cell) -> f64 {
        c.pad.max(self.hull_sup)
    }

    /// **The component the arc carries** (`ClassCarrier::Fitted`): the
    /// branch's certified carrier over its span, its certificate, its
    /// witness `carrier(mid)`, its cells as the boxes a site is read in
    /// ([`CarriedComponent::boxes`]), its pad the widest of theirs, and
    /// its reach: the farthest the cells of a window whose span holds
    /// the witness's parameter stand from the witness, the least over
    /// such windows. It claims no winding: an open arc winds about
    /// neither surface.
    ///
    /// # Errors
    ///
    /// A desync where a window not a node's box holds no cell, though
    /// limb 3 puts a piece of the arc in it, or where no window's span
    /// holds the witness; [`CarryRefusal::PadShort`] where the limb-2
    /// bound is no number; [`CarryRefusal::TubeUnread`] where the
    /// witness does not read.
    pub(super) fn carried(
        &self,
        b: &SsiBranch,
        shape: ComponentShape,
    ) -> Result<CarriedComponent<f64>, CarryError> {
        let mut held = vec![false; self.spans.len()];
        for c in &self.cells {
            held[c.window] = true;
        }
        if held
            .iter()
            .zip(&self.node_windows)
            .any(|(&h, &node)| !h && !node)
        {
            return Err(CarryError::Desync(
                "a window of a rim arc's tube holds no cell the partner's form does not clear",
            ));
        }
        let pad = self
            .cells
            .iter()
            .map(|c| self.pad_of(c))
            .fold(0.0, f64::max);
        if self.hull_sup.is_nan() {
            return Err(CarryError::Refused(CarryRefusal::PadShort {
                pad,
                hull_sup: self.hull_sup,
            }));
        }
        let lift = <Interval as Real>::from_computed;
        let witness = self.reader.carrier.eval(lift(self.at));
        let holding: Vec<usize> = (0..self.spans.len())
            .filter(|&k| self.spans[k].0 <= self.at && self.at <= self.spans[k].1)
            .collect();
        if holding.is_empty() {
            return Err(CarryError::Desync(
                "no piece of a tube holds its witness, though the pieces tile its span",
            ));
        }
        let reach = holding
            .iter()
            .map(|&k| {
                self.cells
                    .iter()
                    .filter(|c| c.window == k)
                    .map(|c| farthest(c.image, witness))
                    .fold(
                        0.0f64,
                        |acc, r| if r.is_finite() { acc.max(r) } else { f64::NAN },
                    )
            })
            .filter(|r| r.is_finite())
            .fold(f64::INFINITY, f64::min);
        if !reach.is_finite() {
            // Every cell read against the carrier, so it is the witness
            // that does not.
            return Err(unread(LoopTubeRefusal::of(
                self.cells[0].image,
                core::slice::from_ref(&witness),
            )));
        }
        let boxes = self
            .cells
            .iter()
            .map(|c| {
                let side = |i: Interval| (i.lo(), i.hi());
                let [x, y, z] = [side(c.image.x), side(c.image.y), side(c.image.z)];
                PadBox {
                    lo: [x.0, y.0, z.0],
                    hi: [x.1, y.1, z.1],
                    pad: self.pad_of(c),
                    node: c.node,
                }
            })
            .collect();
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
        })
    }
}

/// How two cells of two arcs stand to each other.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Stand {
    /// Farther apart than the reach.
    Apart,
    /// Within the reach, one of them meeting the box of a node both arcs
    /// end at.
    AtNode,
    /// Within the reach, by this gap in metres: `NaN` where a box is
    /// refused, which holds nothing apart.
    Meet(f64),
}

/// How the cells imaged in `a` and `b` stand at `reach`, `node_boxes`
/// the boxes of the nodes both their arcs end at.
pub(super) fn stand(a: Box3, b: Box3, node_boxes: &[Box3], reach: f64) -> Stand {
    let gap = box_gap(a, b);
    if gap.partial_cmp(&reach) == Some(core::cmp::Ordering::Greater) {
        return Stand::Apart;
    }
    if [a, b]
        .iter()
        .any(|&r| node_boxes.iter().any(|&n| box_gap(r, n) <= 0.0))
    {
        return Stand::AtNode;
    }
    Stand::Meet(gap)
}

/// A listed component's cells: `None` for one that is no fitted arc.
pub(super) type Arc<'a> = Option<Result<Enclosure<'a>, CarryError>>;

/// **The section's arcs held apart, cell by cell, or carrying neither**
/// ([`CarryRefusal::PadsMeet`], per arc, `None` for an arc that stands
/// apart). A site is read on an arc where it stands in one of the arc's
/// cells, each widened by the band's escalation
/// ([`CarriedComponent::reads`]). A point read on two arcs that share no
/// node is a desync, and one read on two that share a node escalates on
/// its distance from it (`CarriedSection::site`), so each pair of arcs'
/// cells stand farther apart than twice that escalation, except where one
/// of the two meets the box of a node both arcs end at: there the arcs
/// meet, and a site escalates.
///
/// **Refined until that resolves**: the cells of two arcs that stand
/// within that reach of each other are halved, the cleared halves
/// dropped, round by round, until no pair is left, so an arc with no
/// close neighbour keeps its first reading. Short of that the loop stops
/// as `crate::descent` stops one: where two rounds running have not
/// narrowed the widest pad among the cells still meeting or no such cell
/// halves (a stall), or where the next round would pass
/// [`REFINE_CELLS`] (the wall). The arcs still meeting then carry
/// neither.
///
/// An arc whose cell does not read in a round carries that refusal, and
/// is held apart from none.
pub(super) fn apart(arcs: &mut [Arc<'_>], band: Band) -> Vec<Option<CarryRefusal>> {
    let reach = 2.0 * band.escalate();
    let mut active: Vec<Vec<usize>> = arcs
        .iter()
        .map(|a| match a {
            Some(Ok(e)) => (0..e.cells.len()).collect(),
            _ => Vec::new(),
        })
        .collect();
    let mut work = Work::new(REFINE_CELLS);
    let mut descent: Option<Descent<()>> = None;
    loop {
        let flags = |on: bool| -> Vec<Vec<bool>> {
            arcs.iter()
                .map(|a| match a {
                    Some(Ok(e)) => vec![on; e.cells.len()],
                    _ => Vec::new(),
                })
                .collect()
        };
        let (mut near, mut meets) = (flags(false), flags(false));
        // Each arc's nearest meeting: the gap, and the two cells' pads.
        let mut worst: Vec<Option<(f64, [f64; 2])>> = vec![None; arcs.len()];
        for (i, a) in arcs.iter().enumerate() {
            let Some(Ok(a)) = a else { continue };
            for (j, b) in arcs.iter().enumerate().skip(i + 1) {
                let Some(Ok(b)) = b else { continue };
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
                for &ka in &active[i] {
                    let ca = &a.cells[ka];
                    for &kb in &active[j] {
                        let cb = &b.cells[kb];
                        let gap = match stand(ca.image, cb.image, &node_boxes, reach) {
                            Stand::Apart => continue,
                            Stand::AtNode => None,
                            Stand::Meet(gap) => Some(gap),
                        };
                        near[i][ka] = true;
                        near[j][kb] = true;
                        let Some(gap) = gap else { continue };
                        meets[i][ka] = true;
                        meets[j][kb] = true;
                        let pads = [a.pad_of(ca), b.pad_of(cb)];
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
        let refusals = |worst: &[Option<(f64, [f64; 2])>]| {
            worst
                .iter()
                .map(|w| w.map(|(gap, pads)| CarryRefusal::PadsMeet { pads, gap }))
                .collect()
        };
        if std::env::var_os("RIMPAD_TRACE").is_some() && worst.iter().all(Option::is_none) {
            eprintln!(
                "RIMPAD apart: spent {} cells {:?} pads {:?}",
                work.spent(),
                arcs.iter()
                    .map(|a| a
                        .as_ref()
                        .and_then(|a| a.as_ref().ok())
                        .map_or(0, |e| e.cells.len()))
                    .collect::<Vec<_>>(),
                arcs.iter()
                    .map(
                        |a| a.as_ref().and_then(|a| a.as_ref().ok()).map_or(0.0, |e| e
                            .cells
                            .iter()
                            .map(|c| c.pad)
                            .fold(0.0, f64::max))
                    )
                    .collect::<Vec<_>>()
            );
        }
        if worst.iter().all(Option::is_none) {
            return refusals(&worst);
        }
        // The cells still meeting: the widest pad among them, and how
        // many of them halve.
        let (mut bound, mut cost) = (0.0f64, 0usize);
        for (a, m) in arcs.iter().zip(&meets) {
            let Some(Ok(a)) = a else { continue };
            for (c, _) in a.cells.iter().zip(m).filter(|&(_, &m)| m) {
                bound = bound.max(c.pad);
                cost += usize::from(c.node.is_none());
            }
        }
        if std::env::var_os("RIMPAD_TRACE").is_some() {
            eprintln!(
                "RIMPAD round: meeting {cost} bound {bound:e} spent {} cells {:?} worst {:?}",
                work.spent(),
                arcs.iter()
                    .map(|a| a
                        .as_ref()
                        .and_then(|a| a.as_ref().ok())
                        .map_or(0, |e| e.cells.len()))
                    .collect::<Vec<_>>(),
                worst
            );
        }
        let round = Descent::after(descent.take(), bound, ());
        let wall = (!work.admits(cost)).then_some(());
        if round.stop(cost == 0, wall).is_some() {
            return refusals(&worst);
        }
        descent = Some(round);
        work.spend(cost);
        let mut moved = false;
        for (i, arc) in arcs.iter_mut().enumerate() {
            let Some(Ok(e)) = arc else { continue };
            match e.halved(&meets[i], &near[i]) {
                Ok((cells, live, any)) => {
                    e.cells = cells;
                    active[i] = live;
                    moved |= any;
                }
                Err(refusal) => {
                    *arc = Some(Err(refusal));
                    active[i].clear();
                    moved = true;
                }
            }
        }
        if !moved {
            return refusals(&worst);
        }
    }
}
