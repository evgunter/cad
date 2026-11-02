//! **The bit-identity fence.**
//!
//! A digest of the whole corpus's observable evaluation and nothing
//! else: every node's outcome in id order, with its id, and for every
//! body the bits of every point it carries, at `f64`, at `Interval`
//! and at `Probe`; and beside them the same corpus at `f64` with every
//! id masked ([`id_free_corpus_digest`]). Each is a golden number in
//! the ordinary sense: when one moves, the question is whether the new
//! behaviour is correct, not how to get the old number back.
//!
//! What each row says when it holds:
//!
//! - **The id-masked row** moves only when an outcome or a point moves.
//!   The others hash ids too, so a change to how the mint draws ids, or
//!   to the node count, moves them with no point moving; read this row
//!   beside them to tell the two apart.
//! - **The `Probe` row is the `f64` row's number** and must stay so: a
//!   transparent `f64` whose digest drifted would mean the telemetry
//!   scalar had started changing decisions. It is rostered into the
//!   K-telemetry sweep's executed floor, so it runs on that schedule.
//! - **The `f64` row is blind to curve parameters.** It hashes outcomes
//!   and points, so an unmoved `f64` digest says no outcome flipped and
//!   no point of any body moved; it is no claim of parameter-level bit
//!   identity.
//! - **A whole-corpus scalar says THAT a document moved, not which.**
//!   `lib_g16_corpus_name_digests` pins a digest per document, the finer
//!   reading.
//!
//! **A change that moves a number says in its PR what moved and why**,
//! measured by a dump of every node's outcome and every point's bits
//! taken with and without the change, at each scalar that moved. That
//! record of every past move lives in those PRs, not here.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;

use editor_core::{CancelToken, EvalOptions, NodeResult, ValuePayload, evaluate};
use geom_core::Tol;

/// A 128-bit FNV-1a over the evaluation's observable bits.
struct Digest {
    lo: u64,
    hi: u64,
}

impl Digest {
    fn new() -> Self {
        Self {
            lo: 0xcbf2_9ce4_8422_2325,
            hi: 0x9dc5_bb32_e0f7_1a49,
        }
    }

    fn u64(&mut self, x: u64) {
        for b in x.to_le_bytes() {
            self.lo = (self.lo ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3);
            self.hi = (self.hi ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    fn text(&mut self, s: &str) {
        for b in s.as_bytes() {
            self.u64(u64::from(*b));
        }
        self.u64(u64::MAX);
    }
}

/// One observable of the fence's walk, in walk order.
///
/// The walk is the fence's subject and has one spelling, [`walk`]. The
/// fence folds each observable into its [`Digest`]; the diff
/// instrument (`cert3r1_dump`) prints each as a line, so what it prints
/// is exactly what the digest hashes.
pub(crate) enum Seen<'a, T: geom_core::Real> {
    /// Arc-carrier fixture program `i` is about to be replayed.
    Fixture(usize),
    /// Fixture program `i` replayed to a loop of `vertices` vertices.
    FixtureLoop { i: usize, vertices: usize },
    /// Vertex `j` of fixture program `i`'s loop, and the bulge of the
    /// span leaving it.
    FixtureVertex {
        i: usize,
        j: usize,
        x: T,
        y: T,
        /// The leaving segment's stored sweep; zero for a line.
        sweep: T,
    },
    /// Fixture program `i` refused at this scalar.
    FixtureRefused(usize),
    /// Corpus document `name` is about to be evaluated.
    Document(&'a str),
    /// Node `id` of document `doc` and its outcome.
    Node {
        doc: &'a str,
        id: editor_core::MintId,
        outcome: Outcome,
    },
    /// Point `i` of the body node `id` carries, in arena order.
    Point {
        doc: &'a str,
        id: editor_core::MintId,
        i: usize,
        key: topo::PointKey,
        p: &'a geom_core::Point3<T>,
    },
}

/// A node's outcome as the fence observes it.
pub(crate) enum Outcome {
    Poisoned { through: editor_core::MintId },
    Failed,
    Ok { kind: &'static str },
}

/// **The fence's walk**: the arc-carrier fixture, then every corpus
/// document's evaluation at `T`, every node's outcome in id order and
/// every point of every body.
///
/// It uses no API the lift introduced, which is what lets this file
/// compile against a pre-lift tree.
pub(crate) fn walk<T: editor_core::EvalScalar>(mut seen: impl FnMut(Seen<'_, T>)) {
    // The arc-carrier fillet machinery, which no corpus document
    // reaches — see `fixture_walk`.
    fixture_walk::<T>(&mut seen);
    for doc in corpus::documents() {
        seen(Seen::Document(doc.name));
        let ev = evaluate::<T>(
            &doc.doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        for (id, result) in ev.nodes.iter() {
            let outcome = match result {
                NodeResult::Poisoned { through } => Outcome::Poisoned { through: through.0 },
                // The OUTCOME, not the rendered message. A refusal's
                // text embeds the classification band, so observing it
                // would make this whole fence eps-dependent — one
                // golden number per tolerance row, in a repo whose CI
                // deliberately samples three of them. What belongs in a
                // bit-identity fence is that the same nodes succeeded
                // and the same geometry came out; that a refusal is
                // also the SAME refusal is a claim about the lift
                // setting rather than about two trees, and it is
                // carried at full text by `m10_p_lift`'s pinned-vs-
                // guided comparison, which runs both sides in one
                // process at one eps and so can compare messages
                // honestly.
                NodeResult::Failed(_) => Outcome::Failed,
                NodeResult::Ok(v) => Outcome::Ok {
                    kind: v.payload.kind_name(),
                },
            };
            seen(Seen::Node {
                doc: doc.name,
                id: id.0,
                outcome,
            });
            if let NodeResult::Ok(v) = result
                && let ValuePayload::Body(b) = &v.payload
            {
                for (i, (key, p)) in b.body().points().enumerate() {
                    seen(Seen::Point {
                        doc: doc.name,
                        id: id.0,
                        i,
                        key,
                        p,
                    });
                }
            }
        }
    }
}

/// The corpus's evaluation, digested at one scalar.
///
/// `bits` maps the scalar's coordinate to its exact representation.
/// Feeding through the caller keeps this file free of any per-scalar
/// door, which is what lets it compile against a pre-lift tree.
fn corpus_digest<T, F, S>(bits: F, scalar: S) -> (u64, u64)
where
    T: editor_core::EvalScalar,
    F: Fn(&mut Digest, &geom_core::Point3<T>),
    S: Fn(&mut Digest, T),
{
    let mut d = Digest::new();
    walk::<T>(|seen| match seen {
        Seen::Fixture(i) => d.u64(i as u64),
        Seen::FixtureLoop { vertices, .. } => {
            d.text("ok");
            d.u64(vertices as u64);
        }
        Seen::FixtureVertex { x, y, sweep, .. } => {
            scalar(&mut d, x);
            scalar(&mut d, y);
            scalar(&mut d, sweep);
        }
        Seen::FixtureRefused(_) => d.text("refused"),
        Seen::Document(name) => d.text(name),
        Seen::Node { id, outcome, .. } => {
            d.u64(u64::from(id.ordinal()));
            d.u64(id.digest());
            match outcome {
                Outcome::Poisoned { through } => {
                    d.text("poisoned");
                    d.u64(u64::from(through.ordinal()));
                    d.u64(through.digest());
                }
                Outcome::Failed => d.text("failed"),
                Outcome::Ok { kind } => d.text(kind),
            }
        }
        Seen::Point { p, .. } => bits(&mut d, p),
    });
    (d.lo, d.hi)
}

/// **The arc-carrier fillet the corpus does not contain.**
///
/// Every Band-4 profile is authored from straight legs and closed
/// carriers, so a corpus digest — however wide — never once enters
/// `arc_fillet::resolve`, which is where the S8 selection ladder, the
/// derived-corner enumeration and the two angular gates live. That is
/// exactly the machinery this unit rewrote, so a fence that could not
/// see it was pinning the wrong half of the tree.
///
/// This fixture is the rocker eye and the vesica lens, built through
/// the typed surface and replayed at the walk's scalar. It uses no
/// API the lift introduced (`replay` is pre-lift). Its embedding goes
/// through `profile::Step::map_scalar`, which is younger than the lift,
/// so on a tree older than that door the embed is written out
/// by hand to carry this file across. A row that REFUSES is
/// observed as its refusal: the eye does not certify at `Interval`
/// (see `profile`'s `generic_replay` census for why), and "refuses
/// with this message" is as much a bit of behaviour to hold still as
/// "returns these coordinates".
fn fixture_walk<T: profile::ArcCarrierScalar>(seen: &mut impl FnMut(Seen<'_, T>)) {
    use geom_core::Point2;
    use profile::{ArcSweep, Center, Open, Start, Step};
    let tip = 0.75_f64.sqrt();
    // (1) the eye: circle x circle carriers crossing AT the entry
    // anchor. (2) the vesica lens: the two-survivor corner the S8
    // ladder actually ranks.
    let programs = [
        Open.arc_fillet_arc(
            Center {
                c: Point2::new(-0.5, 0.0),
                winding: ArcSweep::Ccw,
                p: Point2::new(0.0, -tip),
            },
            0.35,
            Center {
                c: Point2::new(0.5, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        ),
        Open.arc_fillet_arc(
            Center {
                c: Point2::new(-1.0, 0.0),
                winding: ArcSweep::Ccw,
                p: Point2::new(0.0, -3.0_f64.sqrt()),
            },
            0.5,
            Center {
                c: Point2::new(1.0, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        ),
    ];
    for (i, built) in programs.into_iter().enumerate() {
        seen(Seen::Fixture(i));
        let closed = built.expect("the arc-carrier fixture constructs at f64");
        let steps: Vec<Step<T>> = closed
            .program
            .iter()
            .map(|s| s.map_scalar(T::from_f64))
            .collect();
        match profile::replay(&steps, Tol::witness()) {
            Ok(lp) => {
                let lp = lp.as_loop();
                seen(Seen::FixtureLoop {
                    i,
                    vertices: lp.vertices().len(),
                });
                for (j, (v, s)) in lp.vertices().iter().zip(lp.segments()).enumerate() {
                    let sweep = match s {
                        profile::Segment::Line => T::zero(),
                        profile::Segment::Arc(arc) => arc.sweep,
                    };
                    seen(Seen::FixtureVertex {
                        i,
                        j,
                        x: v.x,
                        y: v.y,
                        sweep,
                    });
                }
            }
            // Same reason as the corpus arm above: the eye's interval
            // refusal names its band, and the band is the eps.
            Err(_) => seen(Seen::FixtureRefused(i)),
        }
    }
}

fn f64_bits(d: &mut Digest, p: &geom_core::Point3<f64>) {
    for c in p.to_array() {
        d.u64(c.to_bits());
    }
}

/// **The fence at `f64`.** Evaluating the corpus with the default
/// options produces exactly these bits.
#[test]
fn the_corpus_evaluation_is_bit_identical_at_f64() {
    let got = corpus_digest::<f64, _, _>(f64_bits, |d, v: f64| d.u64(v.to_bits()));
    println!("m10-p fence f64: {got:016x?}");
    assert_eq!(
        got,
        (0xf8de_8f0a_d8af_e469, 0x9410_e909_6d33_a235),
        "the corpus's f64 evaluation moved — see this file's header before \
         touching the number"
    );
}

/// The same fence at `Interval`, where every profile is elaborated at
/// the lane (its second pass).
#[test]
fn the_corpus_evaluation_is_bit_identical_at_interval() {
    use geom_core::{Bounds, Interval};
    let got = corpus_digest::<Interval, _, _>(
        |d, p| {
            for c in p.to_array() {
                d.u64(c.lo().to_bits());
                d.u64(c.hi().to_bits());
            }
        },
        |d, v: Interval| {
            d.u64(v.lo().to_bits());
            d.u64(v.hi().to_bits());
        },
    );
    println!("m10-p fence interval: {got:016x?}");
    assert_eq!(
        got,
        (0xd5c2_c30e_b9ff_bcd2, 0x5f21_e3d0_31a7_3b6e),
        "the corpus's Interval evaluation moved"
    );
}

/// And at `Probe`, the K-telemetry scalar.
#[cfg(feature = "probe")]
#[test]
fn the_corpus_evaluation_is_bit_identical_at_probe() {
    use geom_core::Probe;
    let got = corpus_digest::<Probe, _, _>(
        |d, p| {
            for c in p.to_array() {
                d.u64(c.0.to_bits());
            }
        },
        |d, v: Probe| d.u64(v.0.to_bits()),
    );
    println!("m10-p fence probe: {got:016x?}");
    // Probe is a transparent f64, so this is the f64 row's number and
    // must stay so: a Probe digest that drifted from it would mean the
    // telemetry scalar had started changing decisions.
    assert_eq!(
        got,
        (0xf8de_8f0a_d8af_e469, 0x9410_e909_6d33_a235),
        "the corpus's Probe evaluation moved"
    );
}

/// **The corpus's geometry, with every id masked**: per document, each
/// node's outcome and its points' bits, the nodes sorted by what they
/// hold rather than by id. Every other digest in this file feeds
/// `id.0`, so a change to how ids are minted moves it with no point
/// moving; this one moves only when an outcome or a point does.
fn id_free_corpus_digest() -> (u64, u64) {
    type Nodes = std::collections::BTreeMap<editor_core::MintId, (String, Vec<[u64; 3]>)>;
    let mut docs: Vec<(String, Nodes)> = Vec::new();
    let mut fixture = Digest::new();
    walk::<f64>(|seen| match seen {
        Seen::Fixture(i) => fixture.u64(i as u64),
        Seen::FixtureLoop { vertices, .. } => fixture.u64(vertices as u64),
        Seen::FixtureVertex { x, y, sweep, .. } => {
            for c in [x, y, sweep] {
                fixture.u64(c.to_bits());
            }
        }
        Seen::FixtureRefused(_) => fixture.text("refused"),
        Seen::Document(name) => docs.push((name.to_owned(), Nodes::new())),
        Seen::Node { id, outcome, .. } => {
            let outcome = match outcome {
                Outcome::Poisoned { .. } => "poisoned",
                Outcome::Failed => "failed",
                Outcome::Ok { kind } => kind,
            };
            docs.last_mut()
                .expect("a node is walked inside a document")
                .1
                .insert(id, (outcome.to_owned(), Vec::new()));
        }
        Seen::Point { id, p, .. } => docs
            .last_mut()
            .and_then(|(_, nodes)| nodes.get_mut(&id))
            .expect("a point is walked under its node")
            .1
            .push(p.to_array().map(f64::to_bits)),
    });
    let mut d = Digest::new();
    d.u64(fixture.lo);
    d.u64(fixture.hi);
    for (name, nodes) in docs {
        d.text(&name);
        let mut held: Vec<(String, Vec<[u64; 3]>)> = nodes
            .into_values()
            .map(|(outcome, mut points)| {
                points.sort_unstable();
                (outcome, points)
            })
            .collect();
        held.sort();
        for (outcome, points) in held {
            d.text(&outcome);
            d.u64(points.len() as u64);
            for c in points.into_iter().flatten() {
                d.u64(c);
            }
        }
    }
    (d.lo, d.hi)
}

/// **The fence with ids masked, at `f64`** ([`id_free_corpus_digest`]).
#[test]
fn the_corpus_geometry_is_bit_identical_with_ids_masked() {
    let got = id_free_corpus_digest();
    println!("m10-p fence id-free: {got:016x?}");
    assert_eq!(
        got,
        (0x9663_b4e2_023f_3622, 0xb57f_4c97_7804_d49e),
        "an outcome or a point of the corpus moved — every other row here also \
         moves with ids, and this one does not"
    );
}
