//! **The value-channel digest** — the ONE feed a cross-scalar
//! differential reads, so that "bit-identical to the `f64` run" means
//! the same thing in every suite that claims it.
//!
//! # What a value channel is
//!
//! A scalar the kernel evaluates at either IS an `f64` with
//! decoration (`Dual64`'s tangent, `Probe`'s recording) or CARRIES an
//! `f64` interval. [`ValueChannelBits`] is that scalar's own value
//! channel as EXACT BITS, and a differential quantifies over it: at
//! the interval pair the bits are `repr_bits`, decoration included, so
//! a decoration drift cannot hide behind equal endpoints.
//!
//! **Bits, never `==`.** `NaN != NaN` and `-0.0 == 0.0` both lie about
//! a wrapper: a channel that could not tell `-0.0` from `0.0` is not
//! checking one. Every scalar enters through `to_bits`/`repr_bits`
//! and every comparison is over the resulting words.
//!
//! # What it reads, and what it does not
//!
//! For every node in evaluation order: the result's arm
//! (`Ok`/`Failed`/`Poisoned`), the payload's arm, and the payload's
//! stored geometry read through the scalar's own value channel — every
//! body point, every datum frame, every profile loop's vertices and
//! bulges, plus arena counts.
//!
//! It does NOT read curve carriers, surface geometry, or pcurves, and
//! it does not read a `Failed` node's error or a `Poisoned` node's
//! ancestor past the arm tag. A consumer that needs those asserts them
//! beside this digest rather than inside it (`m4_pr8_k_probe`'s
//! differential does), and the adopted review digests sample the
//! carriers (`r1_dual_probes`, `r2_m10_di_probes`).
//!
//! # Four entry points over one feed
//!
//! [`value_digest`] folds the whole evaluation into one word — what a
//! row comparing two runs wholesale wants. [`value_digest_nodes`]
//! gives one word PER NODE over the same feed, so a differential that
//! diverges can name the first node it diverged at instead of
//! reporting that two numbers differ. Both call one `feed_node`, so
//! the two shapes cannot drift apart. [`body_digest`] reads ONE body,
//! for a row comparing product bodies with no evaluation to walk, and
//! [`props_digest`] reads one mass-properties certificate.
//! [`fold_words`] folds words a caller collected into one, so a row
//! printing a single number for its run does not hand-roll a hash.
//!
//! **Everything but [`Digest::u64`] is private**, and that is the
//! module's whole defence. The primitives assemble a feed; a caller
//! holding them writes a fifth shape one file over, which is what this
//! module was extracted to stop. `u64` stays public only because a
//! [`ValueChannelBits`] impl for a scalar declared elsewhere — `Probe`,
//! which exists only under the `probe` feature — has to be written
//! where that scalar is visible.
//!
//! # What it does NOT read, per entry point
//!
//! [`props_digest`] reads all four fields of the certificate. The
//! evaluation walks read the payload's STORED geometry only: no curve
//! carriers, no surface geometry, no pcurves, and nothing outside the
//! payload — a `NodeValue`'s name table, verdict and escalation logs,
//! keys, contacts, carried declarations and frame placement are
//! scalar-free or `f64`-only channels a caller compares directly.

use editor_core::{
    AssertionVerdict, BooleanValue, DatumValue, Dimension, Evaluation, NodeResult, RecipeNodeId,
    SplitSide, ValuePayload,
};
use geom_core::{Decide, Dual64};
use topo::{Body, MassProperties};

/// A scalar's VALUE CHANNEL as exact bits — what "bit-identical to the
/// base scalar's run" quantifies over. At the interval pair this is
/// `repr_bits`, decoration included, so a decoration drift cannot hide
/// behind equal endpoints.
pub trait ValueChannelBits: Copy {
    /// Feed this scalar's value channel, exactly.
    fn feed(self, d: &mut Digest);
}

impl ValueChannelBits for f64 {
    fn feed(self, d: &mut Digest) {
        d.u64(self.to_bits());
    }
}

impl ValueChannelBits for Dual64 {
    fn feed(self, d: &mut Digest) {
        d.u64(self.value.to_bits());
    }
}

impl ValueChannelBits for geom_core::Interval {
    fn feed(self, d: &mut Digest) {
        let (lo, hi, dec) = self.repr_bits();
        d.u64(lo);
        d.u64(hi);
        d.u64(u64::from(dec));
    }
}

impl ValueChannelBits for geom_core::DualInterval {
    fn feed(self, d: &mut Digest) {
        self.value.feed(d);
    }
}

/// FNV-1a 64 over the evaluation's value-channel bits (module docs),
/// with the count of words actually fed beside it.
///
/// **The count is not bookkeeping.** A differential whose two arms
/// both digest an empty feed agrees, and agreeing over nothing is the
/// failure mode a new instrument is most likely to ship with. A caller
/// asserts [`Digest::fed`] is positive and the vacuous pass is gone.
pub struct Digest {
    hash: u64,
    fed: usize,
}

impl Digest {
    /// A fresh digest at the FNV-1a offset basis, having fed nothing.
    fn new() -> Self {
        Self {
            hash: 0xcbf2_9ce4_8422_2325,
            fed: 0,
        }
    }

    /// The hash of everything fed so far.
    fn hash(&self) -> u64 {
        self.hash
    }

    /// How many words have been fed.
    fn fed(&self) -> usize {
        self.fed
    }

    /// Feed one exact word. The ONE primitive outside this module: a
    /// `ValueChannelBits` impl for a scalar the kernel gains later is
    /// written wherever that scalar is visible, and needs this. Nothing
    /// else is exported, because a caller holding the geometry
    /// primitives can assemble a FOURTH shape of the feed, which is the
    /// drift this module exists to remove.
    pub fn u64(&mut self, x: u64) {
        self.fed += 1;
        for b in x.to_le_bytes() {
            self.hash ^= u64::from(b);
            self.hash = self.hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    /// Feed one scalar through its own value channel.
    fn scalar<T: ValueChannelBits>(&mut self, x: T) {
        x.feed(self);
    }

    /// Feed a point's three coordinates.
    fn point3<T: Decide + ValueChannelBits>(&mut self, p: geom_core::Point3<T>) {
        self.scalar(p.x);
        self.scalar(p.y);
        self.scalar(p.z);
    }

    /// Feed a vector's three components.
    fn vec3<T: Decide + ValueChannelBits>(&mut self, v: geom_core::Vec3<T>) {
        self.scalar(v.x);
        self.scalar(v.y);
        self.scalar(v.z);
    }

    /// Feed a body's arena counts and every stored point.
    fn body<T: Decide + ValueChannelBits>(&mut self, body: &Body<T>) {
        self.u64(body.solids().count() as u64);
        self.u64(body.faces().count() as u64);
        self.u64(body.edges().count() as u64);
        self.u64(body.vertices().count() as u64);
        for (_k, p) in body.points() {
            self.point3(*p);
        }
    }
}

/// One node's record: its id, its result arm, and its payload read
/// through the scalar's value channel. The single feed both entry
/// points below share.
fn feed_node<T: Decide + ValueChannelBits>(d: &mut Digest, ev: &Evaluation<T>, id: RecipeNodeId) {
    d.u64(id.0);
    match ev.result(id) {
        None => d.u64(0),
        Some(NodeResult::Failed(_)) => d.u64(1),
        Some(NodeResult::Poisoned { .. }) => d.u64(2),
        Some(NodeResult::Ok(v)) => {
            d.u64(3);
            match &v.payload {
                ValuePayload::Datum(DatumValue::Plane { origin, normal }) => {
                    d.u64(10);
                    d.point3(*origin);
                    d.vec3(normal.get());
                }
                ValuePayload::Datum(DatumValue::Axis { origin, dir }) => {
                    d.u64(11);
                    d.point3(*origin);
                    d.vec3(dir.get());
                }
                ValuePayload::Datum(DatumValue::Point { position }) => {
                    d.u64(12);
                    d.point3(*position);
                }
                ValuePayload::Datum(DatumValue::Frame(f)) => {
                    d.u64(23);
                    d.point3(f.origin());
                    d.vec3(f.u().get());
                    d.vec3(f.v().get());
                }
                // Tag 24, appended: an in-plane axis is its own
                // payload, and BOTH its spellings are digested —
                // the sketch pair is what a revolve consumes, so a
                // drift there that the world lift happened to hide
                // must still move the digest.
                ValuePayload::Datum(DatumValue::AxisInPlane {
                    plane_origin,
                    plane_dir,
                    origin,
                    dir,
                }) => {
                    d.u64(24);
                    d.scalar(plane_origin.x);
                    d.scalar(plane_origin.y);
                    d.scalar(plane_dir.x);
                    d.scalar(plane_dir.y);
                    d.point3(*origin);
                    d.vec3(dir.get());
                }
                ValuePayload::Profile(p) => {
                    d.u64(13);
                    for lp in p.validated.loops() {
                        d.u64(lp.vertices().len() as u64);
                        for (v, s) in lp.vertices().iter().zip(lp.segments()) {
                            d.scalar(v.x);
                            d.scalar(v.y);
                            d.scalar(s.bulge);
                        }
                    }
                }
                ValuePayload::Body(b) => {
                    d.u64(14);
                    d.body(b);
                }
                ValuePayload::Boolean(BooleanValue::Empty) => d.u64(15),
                ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
                    d.u64(16);
                    d.body(body);
                }
                ValuePayload::Split { above, below } => {
                    d.u64(17);
                    for side in [above, below] {
                        match side {
                            SplitSide::Empty => d.u64(0),
                            SplitSide::Body(b) => {
                                d.u64(1);
                                d.body(b);
                            }
                        }
                    }
                }
                ValuePayload::Instances(bodies) => {
                    d.u64(18);
                    d.u64(bodies.len() as u64);
                    for b in bodies {
                        d.body(b);
                    }
                }
                ValuePayload::Declarations(pairs) => {
                    d.u64(19);
                    d.u64(pairs.len() as u64);
                }
                ValuePayload::Mate(_) => d.u64(20),
                // The measured quantity IS a lane value, so it is
                // digested through the same value-channel bracket
                // every coordinate takes.
                // A measure with no value at this scalar digests as
                // the ABSENCE, at its own tag: two passes that both
                // failed to measure agree, and neither agrees with
                // a pass that measured something.
                ValuePayload::MeasureUnavailable { .. } => d.u64(24),
                ValuePayload::Measure { value, dim } => {
                    d.u64(21);
                    d.u64(match dim {
                        Dimension::Length => 1,
                        Dimension::Angle => 2,
                        Dimension::Count => 3,
                        Dimension::Scalar => 4,
                    });
                    d.scalar(*value);
                }
                ValuePayload::Assertion(verdict) => {
                    d.u64(22);
                    d.u64(match verdict.holds() {
                        Some(true) => 1,
                        Some(false) => 2,
                        None => 3,
                    });
                    match verdict {
                        AssertionVerdict::Holds { measured, bound }
                        | AssertionVerdict::Violated { measured, bound } => {
                            d.scalar(*measured);
                            d.scalar(*bound);
                        }
                        AssertionVerdict::Unevaluated { .. } => {}
                    }
                }
            }
        }
    }
}

/// One body's arena counts and stored points, as a word and the count
/// of words fed. The shape a row comparing two runs' PRODUCT bodies
/// wants, where there is no evaluation to walk.
pub fn body_digest<T: Decide + ValueChannelBits>(body: &Body<T>) -> (u64, usize) {
    let mut d = Digest::new();
    d.body(body);
    (d.hash(), d.fed())
}

/// One mass-properties certificate, as a word and the count of words
/// fed: the two measured quantities through the lane's own value
/// channel, the two certified pads as the raw `f64` words they are at
/// every scalar.
///
/// It lives here and not in its caller because "compare a
/// `MassProperties` by bits" is written in about thirty places in this
/// tree, each with its own answer to whether the PADS are part of the
/// comparison — and a spelling that omits them compares less than it
/// says it does.
pub fn props_digest<T: Decide + ValueChannelBits>(m: &MassProperties<T>) -> (u64, usize) {
    let mut d = Digest::new();
    d.scalar(m.volume);
    d.scalar(m.surface_area);
    d.u64(m.volume_pad.to_bits());
    d.u64(m.area_pad.to_bits());
    (d.hash(), d.fed())
}

/// Fold a sequence of digest words into one — for a row that collected
/// per-node or per-certificate words and wants a single number to print
/// beside its census.
pub fn fold_words(words: impl IntoIterator<Item = u64>) -> u64 {
    let mut d = Digest::new();
    for w in words {
        d.u64(w);
    }
    d.hash()
}

/// The whole evaluation's value channel as one word — what a row
/// comparing two runs wholesale reads.
pub fn value_digest<T: Decide + ValueChannelBits>(ev: &Evaluation<T>) -> u64 {
    let mut d = Digest::new();
    for &id in &ev.order {
        feed_node(&mut d, ev, id);
    }
    d.hash()
}

/// The same feed, one word PER NODE, in evaluation order, with the
/// words fed for that node beside it.
///
/// A differential over this can name the FIRST node two arms disagreed
/// at; one over [`value_digest`] can only report that two numbers
/// differ, which for a corpus of thirty documents is a starting point
/// and not a finding.
pub fn value_digest_nodes<T: Decide + ValueChannelBits>(
    ev: &Evaluation<T>,
) -> Vec<(RecipeNodeId, u64, usize)> {
    ev.order
        .iter()
        .map(|&id| {
            let mut d = Digest::new();
            feed_node(&mut d, ev, id);
            (id, d.hash(), d.fed())
        })
        .collect()
}
