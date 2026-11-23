//! **The evaluation digest** — the ONE home of the
//! feed every verb-migration suite pins its documents with
//! (`seat4_verb_lowering`, `seat7_sweep_lowering`,
//! `seat8_split_lowering`). The constants stay per suite; the feed
//! does not, because three verbatim copies that each claimed to be
//! "byte for byte the SEAT-4 feed" were held to it by nothing — an
//! edit to one would have left every suite green and the claim false.
//!
//! # What it covers, and how that set was chosen
//!
//! Not "everything observable": the channels the migrated lowerings can
//! move, enumerated off their own bodies. `wire_blend` writes exactly
//! three things — the name table the emitter returns, the body the
//! kernel verb returns, and (on the refusal path) a typed error.
//! `wire_swept` writes the same three. The pair lowerings
//! (`wire_subtract`, `wire_combine`) write the first two plus the
//! boolean VALUE's other halves — the result classification, the
//! surviving contacts, and the typed empty success — fed by
//! the `Boolean` arm, each with a pinned input on which it actually
//! VARIES (the contacts through `kiss_carry`, the empty token through
//! the disjoint intersect; both were measured fed-but-dead before those
//! inputs existed). `wire_split` writes the name table and TWO sides,
//! fed by the `Split` arm: each side
//! under its ROLE token — so a lowering that swapped the halves moves
//! the digest even where every arena is bit-identical — and an EMPTY
//! side as its own token, pinned live by an input that produces it.
//!
//! The refusal path is NOT covered, and that is a real hole, stated: a
//! refusal payload's spelling and the verdict logs are outside this
//! digest, so a change that only altered which `NodeErrorKind` came
//! back would pass it.
//!
//! # Why each half of the body feed is load-bearing, measured
//!
//! - **Point bits alone are not enough.** A unit cube filleted at
//!   radius `r` and a unit cube chamfered at setback `r` have the same
//!   twenty-four vertex positions to the bit, differing only in whether
//!   the faces between them are cylinders and spheres or planes. A
//!   points-only digest gave the two documents ONE identical number.
//! - **Face carriers alone are not enough either**: the edge curve
//!   carriers are fed beside them.
//!
//! **Where each body stands.** A value's body is in its own coordinates
//! under a frame (D10 `[placement.frame]`), so the body fed is the one
//! composed into its space through `in_space` ([`super::in_space`]): a
//! copy is pinned where it stands, as its rebuilt body was.
//!
//! Geometry enters through `Debug`, whose `f64` rendering
//! is the shortest round-tripping decimal: a bijection with the bits
//! for every finite value, `-0.0` included. Nothing rendered from a
//! classification band enters, so the constants are eps-independent.
//!
//! # What it leaves out: arena keys and mint ids
//!
//! **No arena key and no mint id is fed.** Each cell is read by its
//! geometry ([`KeyFree`]): a vertex by its point, an edge by its
//! directed ends and its curve, a face by its sense, its carrier and its
//! loops as canonically rotated cycles of signed edge readings, and a
//! key inside another entity's `Debug` by a hash of the reading of the
//! cell it names. A node is fed by its place in the evaluation order,
//! and a name by its `Debug` with every node id spelled the same way
//! ([`id_free`]). So a build-order change that renumbers an arena, or a
//! change to how ids are minted, moves no digest; a name that comes to
//! denote another cell, a cell that moves, and a body whose topology
//! changes each do.

use std::collections::BTreeMap;

use editor_core::{BooleanValue, EntityKey, Evaluation, SplitSide, ValuePayload};
use topo::{Body, EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, VertexKey};

/// FNV-1a 64 over `bytes`.
fn fnv(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x1000_0000_01b3);
    }
    h
}

/// **A body read with no arena key in it** — every cell by its
/// geometry (module docs), each key that appears inside another
/// entity's `Debug` replaced by a hash of the reading of the cell it
/// names.
pub struct KeyFree<'b> {
    body: &'b Body<f64>,
    /// Each key's `Debug`, and the word it is replaced by.
    words: Vec<(String, String)>,
}

impl<'b> KeyFree<'b> {
    pub fn new(body: &'b Body<f64>) -> Self {
        let mut read = Self {
            body,
            words: Vec::new(),
        };
        // Each arena in the order its readings need the ones before it:
        // a curve's description names surfaces, an edge reads its
        // curve, a half-edge its edge, a loop its half-edges, a face its
        // surface and loops.
        let surfaces: Vec<_> = body
            .surfaces()
            .map(|(k, s)| (format!("{k:?}"), read.word("S", format!("{s:?}"))))
            .collect();
        read.learn(surfaces);
        let points: Vec<_> = body
            .points()
            .map(|(k, p)| (format!("{k:?}"), read.word("P", point(*p))))
            .collect();
        read.learn(points);
        let curves: Vec<_> = body
            .curves()
            .map(|(k, c)| (format!("{k:?}"), read.word("C", format!("{c:?}"))))
            .collect();
        read.learn(curves);
        let vertices: Vec<_> = body
            .vertices()
            .map(|(k, _)| (format!("{k:?}"), read.word("V", read.vertex(k))))
            .collect();
        read.learn(vertices);
        let edges: Vec<_> = body
            .edges()
            .map(|(k, _)| (format!("{k:?}"), read.word("E", read.edge(k))))
            .collect();
        read.learn(edges);
        let half_edges: Vec<_> = body
            .half_edges()
            .map(|(k, _)| (format!("{k:?}"), read.word("H", read.half_edge(k))))
            .collect();
        read.learn(half_edges);
        let loops: Vec<_> = body
            .loops()
            .map(|(k, l)| (format!("{k:?}"), read.word("L", read.boundary(&l.boundary))))
            .collect();
        read.learn(loops);
        let faces: Vec<_> = body
            .faces()
            .map(|(k, _)| (format!("{k:?}"), read.word("F", read.face(k))))
            .collect();
        read.learn(faces);
        read
    }

    /// Adds `words`, keeping the longest key first, so no key is
    /// replaced inside a longer one.
    fn learn(&mut self, words: Vec<(String, String)>) {
        self.words.extend(words);
        self.words.sort_by_key(|(k, _)| core::cmp::Reverse(k.len()));
    }

    /// A cell's word: its kind, and a hash of its reading.
    fn word(&self, kind: &str, reading: String) -> String {
        format!("{kind}#{:016x}", fnv(self.resolved(reading).as_bytes()))
    }

    /// `text` with every key the body knows so far replaced by its word.
    fn unkeyed(&self, mut text: String) -> String {
        if !text.contains("Key(") {
            return text;
        }
        for (key, word) in &self.words {
            if text.contains(key.as_str()) {
                text = text.replace(key.as_str(), word);
            }
        }
        text
    }

    /// [`Self::unkeyed`], loud where a key is left: a reading that still
    /// named an arena slot would feed it.
    pub fn resolved(&self, text: String) -> String {
        let text = self.unkeyed(text);
        assert!(
            !text.contains("Key("),
            "a key-free reading still names an arena slot: {text}"
        );
        text
    }

    pub fn vertex(&self, v: VertexKey) -> String {
        point(super::point(self.body, v))
    }

    /// An edge by its `he_plus`-forward ends and its curve.
    pub fn edge(&self, e: EdgeKey) -> String {
        let [a, b] = super::ends(self.body, e);
        let curve = self
            .body
            .get_curve_geom(self.body.get_edge(e).expect("a live edge").curve)
            .expect("an edge has a curve");
        format!(
            "[{} -> {} on {}]",
            self.vertex(a),
            self.vertex(b),
            self.resolved(format!("{curve:?}"))
        )
    }

    /// A half-edge by its edge and the way it runs along it.
    fn half_edge(&self, he: HalfEdgeKey) -> String {
        let h = self.body.get_half_edge(he).expect("a live half-edge");
        let plus = self.body.get_edge(h.edge).expect("a live edge").he_plus == he;
        format!("{}{}", if plus { '+' } else { '-' }, self.edge(h.edge))
    }

    /// A loop's boundary: its cycle of signed edge readings, rotated to
    /// start where the joined cycle reads least, or the lone vertex of
    /// an empty ring.
    fn boundary(&self, boundary: &LoopBoundary) -> String {
        match *boundary {
            LoopBoundary::Empty { vertex } => format!("ring at {}", self.vertex(vertex)),
            LoopBoundary::Cycle { first } => {
                let cycle: Vec<String> = self
                    .body
                    .loop_cycle(first)
                    .expect("a closed cycle")
                    .into_iter()
                    .map(|he| self.half_edge(he))
                    .collect();
                (0..cycle.len())
                    .map(|i| [&cycle[i..], &cycle[..i]].concat().join(" "))
                    .min()
                    .unwrap_or_default()
            }
        }
    }

    /// A face by its sense, its carrier, its outer loop and its rings.
    pub fn face(&self, f: FaceKey) -> String {
        let record = self.body.get_face(f).expect("a live face");
        let surface = self.body.face_surface(f).expect("a face has a carrier");
        let loop_of = |lk| self.boundary(&self.body.get_loop(lk).expect("a live loop").boundary);
        let mut loops = self.body.face(f).expect("a live face").loops();
        let outer = loops
            .next()
            .map(|(lk, _, _)| loop_of(lk))
            .unwrap_or_default();
        let mut rings: Vec<String> = loops.map(|(lk, _, _)| loop_of(lk)).collect();
        rings.sort();
        format!(
            "{} {} outer {outer} rings {rings:?}",
            if record.sense { '+' } else { '-' },
            self.resolved(format!("{surface:?}"))
        )
    }

    /// The cell a name table entry's key denotes, or its absence.
    pub fn entity(&self, key: &EntityKey) -> String {
        // A key the body does not hold reads as that absence: the
        // table's entity is not a cell of the body it names it in.
        match *key {
            EntityKey::Body => "body".to_owned(),
            EntityKey::Face(f) if self.body.get_face(f).is_some() => {
                format!("face {}", self.face(f))
            }
            EntityKey::Edge(e) if self.body.get_edge(e).is_some() => {
                format!("edge {}", self.edge(e))
            }
            EntityKey::Vertex(v) if self.body.get_vertex(v).is_some() => {
                format!("vertex {}", self.vertex(v))
            }
            EntityKey::Face(_) => "a face the body does not hold".to_owned(),
            EntityKey::Edge(_) => "an edge the body does not hold".to_owned(),
            EntityKey::Vertex(_) => "a vertex the body does not hold".to_owned(),
        }
    }

    /// The whole body: its faces, its edges and its points, each sorted,
    /// then its entity census.
    pub fn body(&self) -> String {
        let sorted = |mut v: Vec<String>| {
            v.sort();
            v.join("\n")
        };
        let faces = sorted(self.body.faces().map(|(f, _)| self.face(f)).collect());
        let edges = sorted(self.body.edges().map(|(e, _)| self.edge(e)).collect());
        let points = sorted(self.body.points().map(|(_, p)| point(*p)).collect());
        format!(
            "{faces}\n{edges}\n{points}\nV{}E{}F{}",
            self.body.vertices().count(),
            self.body.edges().count(),
            self.body.faces().count()
        )
    }
}

/// A point by its coordinates' bits.
fn point(p: geom_core::Point3<f64>) -> String {
    let [x, y, z] = p.to_array().map(f64::to_bits);
    format!("({x:016x} {y:016x} {z:016x})")
}

/// `text` with every mint id spelled by what it is to `ev`: a node's
/// id by the node's place in the evaluation order, any other id by its
/// ordinal alone — never the chain digest that drew it.
pub fn id_free(ev: &Evaluation<f64>, text: &str) -> String {
    const OPEN: &str = "MintId { ordinal: ";
    let place: BTreeMap<String, usize> = ev
        .order
        .iter()
        .enumerate()
        .map(|(i, id)| (format!("{:?}", id.0), i))
        .collect();
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find(OPEN) {
        out.push_str(&rest[..at]);
        let end = rest[at..].find('}').map_or(rest.len(), |e| at + e + 1);
        let id = &rest[at..end];
        match place.get(id) {
            Some(i) => out.push_str(&format!("@node{i}")),
            None => {
                let ordinal = id[OPEN.len()..].split(',').next().unwrap_or("?");
                out.push_str(&format!("@id{ordinal}"));
            }
        }
        rest = &rest[end..];
    }
    out.push_str(rest);
    out
}

/// **One node's name tables, key-free and id-free**: each name
/// ([`id_free`]) with the cell it denotes read by its geometry
/// ([`KeyFree`]) in the output body it names it in, sorted.
pub fn name_rows(ev: &Evaluation<f64>, id: editor_core::RecipeNodeId) -> Vec<String> {
    let Some(value) = ev.value(id) else {
        return Vec::new();
    };
    let mut bodies: BTreeMap<u32, Option<Body<f64>>> = BTreeMap::new();
    for table in super::tables(ev, id) {
        for (_, entry) in table.iter() {
            let refs = match entry {
                editor_core::Entry::Unique(r) => core::slice::from_ref(r),
                editor_core::Entry::Tied(c) => c.as_slice(),
            };
            for r in refs {
                bodies.entry(r.body).or_insert_with(|| {
                    value
                        .payload
                        .output_body(r.body)
                        .ok()
                        .map(|b| b.body().clone())
                });
            }
        }
    }
    let reads: BTreeMap<u32, Option<KeyFree<'_>>> = bodies
        .iter()
        .map(|(i, b)| (*i, b.as_ref().map(KeyFree::new)))
        .collect();
    let read = |r: &editor_core::EntityRef| match &reads[&r.body] {
        Some(body) => format!("{}:{}", r.body, body.entity(&r.key)),
        None => format!("{}:no body", r.body),
    };
    let mut rows = Vec::new();
    for table in super::tables(ev, id) {
        for (name, entry) in table.iter() {
            let entry = match entry {
                editor_core::Entry::Unique(r) => format!("u {}", read(r)),
                editor_core::Entry::Tied(c) => {
                    let mut tied: Vec<String> = c.iter().map(read).collect();
                    tied.sort();
                    format!("t {tied:?}")
                }
            };
            rows.push(format!("{}={entry}", id_free(ev, &format!("{name:?}"))));
        }
    }
    rows.sort();
    rows
}

/// FNV-1a 64 over a document's evaluated name tables and values — see
/// the module docs for what is fed and why.
pub fn digest(ev: &Evaluation<f64>) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut feed = |bytes: &[u8]| {
        for b in bytes {
            h ^= u64::from(*b);
            h = h.wrapping_mul(0x1000_0000_01b3);
        }
    };
    for (place, id) in ev.order.iter().enumerate() {
        feed(format!("@node{place}").as_bytes());
        let Some(value) = ev.value(*id) else { continue };
        for row in name_rows(ev, *id) {
            feed(row.as_bytes());
        }
        feed(value.payload.kind_name().as_bytes());
        // EXHAUSTIVE, no wildcard: a payload kind added to the value
        // vocabulary is a visit here that says whether a migrated
        // lowering writes it. The kinds fed nothing beyond their name
        // are named for that reason — each is one no migrated lowering
        // produces, and a wildcard would silently unfeed the next one
        // that is.
        match &value.payload {
            ValuePayload::Body(body) => feed_body(&mut feed, &super::in_space(body)),
            ValuePayload::Boolean(bv) => match bv {
                BooleanValue::Body { body, kind } => {
                    let contacts =
                        KeyFree::new(body.body()).resolved(format!("{:?}", value.carry.contacts));
                    feed(format!("{kind:?}{}", id_free(ev, &contacts)).as_bytes());
                    feed_body(&mut feed, &super::in_space(body));
                }
                BooleanValue::Empty => feed(b"empty"),
            },
            ValuePayload::Split { above, below } => {
                for (role, side) in [("above", above), ("below", below)] {
                    feed(role.as_bytes());
                    match side {
                        SplitSide::Body(body) => feed_body(&mut feed, &super::in_space(body)),
                        SplitSide::Empty => feed(b"empty-side"),
                    }
                }
            }
            ValuePayload::Profile(_)
            | ValuePayload::Instances(_)
            | ValuePayload::Measure { .. }
            | ValuePayload::Assertion(_)
            | ValuePayload::Family => {}
        }
    }
    h
}

/// The body half of [`digest`]: the body's key-free reading
/// ([`KeyFree::body`]) — its faces with their carriers, senses and
/// loops, its edges with their curves, its points' bits and its entity
/// census.
pub fn feed_body(feed: &mut impl FnMut(&[u8]), body: &Body<f64>) {
    feed(KeyFree::new(body).body().as_bytes());
}
