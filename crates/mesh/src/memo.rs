//! The per-face patch memo behind [`crate::tessellate_with`]: a face
//! whose inputs are bit-identical to one already meshed is placed from
//! the memo instead of run through its lane.
//!
//! # Why this is sound
//!
//! Per-face tessellation is a pure function of the face's inputs and
//! nothing else (crate docs, *Watertightness and the memo-key
//! contract*), and D9 makes "same bits ⇒ same patch" a theorem rather
//! than a hope. So a memo keyed by those bits can answer for the lane,
//! and the ONE defect this design can have is a key that omits an
//! input the lane reads. [`FaceInputs`] states the key as the list of
//! inputs each lane reads; `tests/index_memo.rs` in `viewer` is the
//! differential that proves it across edits, and `tests/patch_memo.rs`
//! here mutates each input singly.
//!
//! # The key, per lane
//!
//! Everything below is folded as BYTES: `f64`s by their bit pattern
//! (so `-0.0` and `0.0` differ, and NaN equals itself), every
//! sequence length-prefixed, every enum variant tagged. Arena keys —
//! `FaceKey`, `EdgeKey`, mesh vertex ids — are NOT in the key: they
//! are lineage-scoped, and a memo that outlives an evaluation must not
//! depend on them. Where a lane reads a mesh id it reads it for its
//! IDENTITY (which boundary points are the same point), and that
//! structure is folded as a relabeling: the face's boundary id
//! sequence numbered by first occurrence.
//!
//! Common to every lane:
//! - δ (the chordal tolerance; δ_s and the certificates' δ derive from
//!   it) and the ambient tolerance's ε and k (the lanes' `Eps` and the
//!   curved lane's shape band derive from those);
//! - the surface's KIND (the dispatch reads it);
//! - for each loop in face order (outer, then rings), for each edge in
//!   walk order: its traversal direction, its certified carrier
//!   (`Curve3`) and parameter interval, its chord points' positions,
//!   and the identity relabeling of its chord ids.
//!
//! The planar lane reads nothing more: its chart frame comes from the
//! boundary, not from the stored plane.
//!
//! The curved lane additionally reads the surface's fields (the chart),
//! the face's `sense` (the pole-to-pole band's azimuth choice), each
//! edge description's `seam` flag (`topo::chart_iso::classify_kind`
//! reads it before the carrier) and the identity structure of the
//! edges' split-lineage carriers (`geom_brep::props`' torus folding
//! asks whether two arcs are pieces of one original edge) — the last
//! folded as a relabeling, like the chord ids, since a root edge key
//! is an arena key.
//!
//! The trimmed lane additionally reads the surface's fields (an
//! `Approx` face through its fit, which is the geometry the lane
//! meshes), each edge's chord PARAMETERS (it evaluates pcurves on that
//! schedule) and each half-edge's stored pcurve.
//!
//! # Eviction and D9
//!
//! Eviction is generational: [`PatchMemo::end_picture`] drops every
//! entry not used since the previous call, so the memo holds one
//! picture's worth of patches and cannot grow. A caller that reuses a
//! whole body's mesh without re-tessellating it keeps that body's
//! faces alive with [`PatchMemo::keep`].
//!
//! The map is keyed by a fixed 128-bit FNV-1a digest of the key bytes
//! (two independent bases), never by `std`'s per-process hasher; a
//! lookup by content influences nothing, and the one iteration
//! (eviction) is order-independent. A digest collision would be a
//! wrong mesh, so every entry carries its full key bytes and a hit
//! compares them — a false hit is impossible by construction.
//!
//! The FNV constants here are one of the tree's many copies, and the
//! picture/open/close counter machinery is re-spelled by
//! `editor_core::PickMemo`; both are named for consolidation in
//! `work/perf/fnv-digest-and-memo-machinery-copies.md`.

use std::collections::HashMap;

use geom::{Curve3, NurbsCurve2, NurbsCurve3, NurbsSurface, Surface};
use geom_brep::{EdgeDescription, Pcurve};
use geom_core::spline::KnotVector;
use geom_core::{Point2, Point3, Tol, Vec2, Vec3};
use topo::{Body, FaceKey};

use crate::chords::ChordPass;
use crate::tessellate::{Lane, Patch, PatchVertex};
use crate::types::TessellateError;
use crate::walk::loop_half_edges;

/// A face's content digest: 128 bits of FNV-1a over its key bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PatchDigest([u64; 2]);

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
/// The second lane's basis: the golden-ratio constant XORed in, so
/// the two 64-bit digests start from independent states.
const HI_TWEAK: u64 = 0x9e37_79b9_7f4a_7c15;

impl PatchDigest {
    fn of(bytes: &[u8]) -> Self {
        let mut lo = FNV_OFFSET;
        let mut hi = FNV_OFFSET ^ HI_TWEAK;
        for &b in bytes {
            lo ^= u64::from(b);
            lo = lo.wrapping_mul(FNV_PRIME);
            hi ^= u64::from(b);
            hi = hi.wrapping_mul(FNV_PRIME);
        }
        Self([lo, hi])
    }
}

/// The digests of one tessellation's faces, in face-arena order — what
/// [`PatchMemo::keep`] takes to keep a reused body's faces alive.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PatchKeys(Vec<PatchDigest>);

impl PatchKeys {
    /// One digest per face.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether the tessellation had no faces.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The digests, in face-arena order.
    pub fn iter(&self) -> impl Iterator<Item = PatchDigest> + '_ {
        self.0.iter().copied()
    }

    /// The digest of the face at position `i`, if the tessellation
    /// had one.
    pub fn get(&self, i: usize) -> Option<PatchDigest> {
        self.0.get(i).copied()
    }

    pub(crate) fn push(&mut self, digest: PatchDigest) {
        self.0.push(digest);
    }
}

/// One memoised face: its key bytes (compared on every hit), its patch
/// and the picture it was last used in.
struct Entry {
    key: Vec<u8>,
    patch: StoredPatch,
    picture: u64,
}

/// The memo. Owned by whoever owns the previous picture; see the
/// module docs for the key and the eviction rule.
#[derive(Default)]
pub struct PatchMemo {
    entries: HashMap<PatchDigest, Entry>,
    /// The picture being built: entries used carry this stamp.
    picture: u64,
    /// Whether the last picture was closed and nothing has started the
    /// next: the counters below then describe the closed picture, and
    /// the next use zeroes them.
    closed: bool,
    hits: usize,
    misses: usize,
}

impl core::fmt::Debug for PatchMemo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PatchMemo")
            .field("entries", &self.entries.len())
            .field("picture", &self.picture)
            .field("hits", &self.hits)
            .field("misses", &self.misses)
            .finish()
    }
}

impl PatchMemo {
    /// An empty memo.
    pub fn new() -> Self {
        Self::default()
    }

    /// How many faces the memo holds.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the memo holds nothing.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The memo's heap footprint, approximately: every entry's key
    /// bytes, interior points and triangles. A measurement door, not a
    /// budget.
    pub fn bytes(&self) -> usize {
        self.entries
            .values()
            .map(|e| {
                e.key.len()
                    + e.patch.interior.len() * core::mem::size_of::<Point3<f64>>()
                    + e.patch.triangles.len() * core::mem::size_of::<[StoredVertex; 3]>()
            })
            .sum()
    }

    /// Faces answered from the memo in the current picture — or, once
    /// [`PatchMemo::end_picture`] has closed it and nothing has started
    /// the next, in that closed picture.
    pub fn hits(&self) -> usize {
        self.hits
    }

    /// Faces run through their lane, over the same picture
    /// [`PatchMemo::hits`] counts.
    pub fn misses(&self) -> usize {
        self.misses
    }

    /// The first use after a close starts the next picture's counts.
    fn open(&mut self) {
        if self.closed {
            self.closed = false;
            self.hits = 0;
            self.misses = 0;
        }
    }

    /// Mark `keys`' faces as part of the picture being built, so
    /// [`PatchMemo::end_picture`] keeps them: the caller reused a
    /// whole tessellation and its faces were never looked up.
    pub fn keep(&mut self, keys: &PatchKeys) {
        self.open();
        for digest in keys.iter() {
            if let Some(entry) = self.entries.get_mut(&digest) {
                entry.picture = self.picture;
            }
        }
    }

    /// Close the picture: drop every entry not used in it. The memo
    /// then holds exactly the faces of the picture just built, and its
    /// counts describe that picture until the next one starts.
    pub fn end_picture(&mut self) {
        let picture = self.picture;
        self.entries.retain(|_, entry| entry.picture == picture);
        self.picture += 1;
        self.closed = true;
    }

    fn insert(&mut self, digest: PatchDigest, key: Vec<u8>, patch: StoredPatch) {
        self.open();
        let picture = self.picture;
        self.entries.insert(
            digest,
            Entry {
                key,
                patch,
                picture,
            },
        );
    }

    /// The patch under `digest`, if the memo holds one whose FULL key
    /// bytes match — so a digest collision is a miss and not a wrong
    /// patch.
    fn stored(&self, digest: PatchDigest, key: &[u8]) -> Option<&StoredPatch> {
        match self.entries.get(&digest) {
            Some(entry) if entry.key == key => Some(&entry.patch),
            _ => None,
        }
    }

    /// The memo's PURE half of one face: its key, and the stored patch
    /// restored against this tessellation's ids if the memo holds one.
    ///
    /// `&self` only — no counter, no picture stamp, no insert — which
    /// is what lets it run inside `tessellate`'s per-face parallel map
    /// (D9 idiom 1). Everything it decides travels to the arena-order
    /// fold in a [`FaceMemo`], which [`PatchMemo::record`] applies.
    ///
    /// `inputs` is the face's key; its boundary id sequence is what the
    /// stored patch's shared corners are named against.
    pub(crate) fn lookup(&self, inputs: &FaceInputs) -> FaceLookup {
        let key = inputs.key();
        let digest = PatchDigest::of(&key);
        let boundary = inputs.boundary_ids();
        let hit = self.stored(digest, &key).map(|s| s.restore(&boundary));
        FaceLookup {
            digest,
            key,
            boundary,
            hit,
        }
    }

    /// The memo's MUTATING half of one face, applied in face-arena
    /// order: the digest into `keys`, the hit or miss counted, the
    /// entry's picture stamped, and a missed face's patch stored.
    ///
    /// Arena order is the contract, not a convenience —
    /// [`PatchMemo::hits`] and [`PatchMemo::misses`] are counts over an
    /// ordered walk and `PatchKeys` is a per-face sequence, so both are
    /// the numbers the serial loop produced only because this runs
    /// where that loop's body did.
    pub(crate) fn record(&mut self, face: FaceMemo, keys: &mut PatchKeys) {
        self.open();
        let picture = self.picture;
        keys.push(face.digest);
        match face.outcome {
            Outcome::Hit => {
                self.hits += 1;
                if let Some(entry) = self.entries.get_mut(&face.digest) {
                    entry.picture = picture;
                }
            }
            Outcome::Refused => self.misses += 1,
            Outcome::Miss { key, stored } => {
                self.misses += 1;
                match stored {
                    Some(stored) => self.insert(face.digest, key, stored),
                    // A lane named a shared id outside its own
                    // boundary. The patch it produced is right — it was
                    // just computed — but the key cannot name that id,
                    // so nothing is stored and the face is meshed
                    // afresh every picture. Debug builds say so: it is
                    // a key-completeness defect, not a state.
                    None => debug_assert!(
                        false,
                        "a lane emitted a shared mesh id outside its face's boundary walk"
                    ),
                }
            }
        }
    }
}

/// One face's read of the memo, taken from `&PatchMemo` alone
/// ([`PatchMemo::lookup`]).
pub(crate) struct FaceLookup {
    digest: PatchDigest,
    key: Vec<u8>,
    boundary: Vec<u32>,
    hit: Option<Patch>,
}

impl FaceLookup {
    /// The memo's answer for this face, if it held one. The lane runs
    /// exactly when this is `None`.
    pub(crate) fn take_hit(&mut self) -> Option<Patch> {
        self.hit.take()
    }

    /// Closes a face the memo answered.
    pub(crate) fn answered(self) -> FaceMemo {
        FaceMemo {
            digest: self.digest,
            outcome: Outcome::Hit,
        }
    }

    /// Closes a face whose lane refused. There is no patch to store,
    /// and the miss is still a miss: the serial loop counted it before
    /// it ran the lane, so the counters over a picture that failed are
    /// what they were.
    pub(crate) fn refused(self) -> FaceMemo {
        FaceMemo {
            digest: self.digest,
            outcome: Outcome::Refused,
        }
    }

    /// Closes a face whose lane ran, naming `patch`'s shared corners
    /// against the boundary walk so the entry is placeable in a mesh
    /// with different ids.
    ///
    /// The renaming happens HERE, in the map, rather than in the fold:
    /// it is per-face work over the face's own patch, and the fold's
    /// only business is the order.
    pub(crate) fn ran(self, patch: &Patch) -> FaceMemo {
        FaceMemo {
            digest: self.digest,
            outcome: Outcome::Miss {
                stored: StoredPatch::store(patch, &self.boundary),
                key: self.key,
            },
        }
    }
}

/// What the arena-order fold still owes the memo for one face
/// ([`PatchMemo::record`]).
pub(crate) struct FaceMemo {
    digest: PatchDigest,
    outcome: Outcome,
}

/// Where the face landed in the memo, decided in the map.
enum Outcome {
    /// The memo held it; the fold counts a hit and re-stamps the entry.
    Hit,
    /// It missed; the fold counts a miss and inserts `stored`, which is
    /// `None` for the key-completeness defect [`PatchMemo::record`]
    /// names.
    Miss {
        key: Vec<u8>,
        stored: Option<StoredPatch>,
    },
    /// It missed and its lane then refused: the miss is counted and
    /// nothing is stored.
    Refused,
}

/// One corner of a stored patch: a boundary point by its index in the
/// face's boundary id sequence, or an interior point by its index in
/// the patch's own interior — neither a mesh id.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StoredVertex {
    Boundary(u32),
    Local(u32),
}

/// A patch with its shared corners renamed to boundary positions, so
/// it can be placed in a mesh whose ids differ from the one it was
/// computed in.
struct StoredPatch {
    interior: Vec<Point3<f64>>,
    triangles: Vec<[StoredVertex; 3]>,
}

impl StoredPatch {
    /// `None` if the patch names a shared id that is not in `boundary`.
    fn store(patch: &Patch, boundary: &[u32]) -> Option<Self> {
        let mut index: HashMap<u32, u32> = HashMap::with_capacity(boundary.len());
        for (i, &id) in boundary.iter().enumerate() {
            #[allow(clippy::cast_possible_truncation)]
            index.entry(id).or_insert(i as u32);
        }
        let mut triangles = Vec::with_capacity(patch.triangles.len());
        for t in &patch.triangles {
            let mut out = [StoredVertex::Local(0); 3];
            for (k, v) in t.iter().enumerate() {
                out[k] = match *v {
                    PatchVertex::Shared(id) => StoredVertex::Boundary(*index.get(&id)?),
                    PatchVertex::Local(i) => StoredVertex::Local(i),
                };
            }
            triangles.push(out);
        }
        Some(Self {
            interior: patch.interior.clone(),
            triangles,
        })
    }

    fn restore(&self, boundary: &[u32]) -> Patch {
        Patch {
            interior: self.interior.clone(),
            triangles: self
                .triangles
                .iter()
                .map(|t| {
                    t.map(|v| match v {
                        StoredVertex::Boundary(i) => PatchVertex::Shared(boundary[i as usize]),
                        StoredVertex::Local(i) => PatchVertex::Local(i),
                    })
                })
                .collect(),
        }
    }
}

/// One edge of a face's boundary walk, as the lanes read it.
#[derive(Clone, Debug)]
pub(crate) struct EdgeInputs {
    /// Whether the walk traverses the edge `he_plus`-forward.
    pub(crate) forward: bool,
    /// The certified carrier and its parameter interval.
    pub(crate) carrier: Curve3<f64>,
    pub(crate) params: (f64, f64),
    /// The edge's chord-point mesh ids, `he_plus`-forward — folded only
    /// as an identity relabeling, never as ids.
    pub(crate) ids: Vec<u32>,
    /// The chord points, in the same order.
    pub(crate) positions: Vec<Point3<f64>>,
    /// The chord parameters, in the same order (the trimmed lane's
    /// pcurve schedule).
    pub(crate) chord_params: Vec<f64>,
    /// The half-edge's stored pcurve, if any (the trimmed lane's).
    pub(crate) pcurve: Option<Pcurve<f64>>,
    /// Whether the edge's description marks it a chart seam (the
    /// curved lane's classification reads this before the carrier).
    pub(crate) seam: bool,
    /// The edge's split-lineage root, as an identity: `None` where the
    /// lineage does not resolve. Folded as a relabeling, never as the
    /// key it is.
    pub(crate) lineage: Option<u32>,
}

/// Everything a face's lane reads — the memo key, as a value. The
/// module docs list which lane reads which field.
#[derive(Clone, Debug)]
pub(crate) struct FaceInputs {
    pub(crate) lane: Lane,
    pub(crate) chordal: f64,
    pub(crate) eps: f64,
    pub(crate) k: f64,
    pub(crate) surface: Surface<f64>,
    pub(crate) sense: bool,
    /// Outer loop first, then rings in face order; each in walk order.
    pub(crate) loops: Vec<Vec<EdgeInputs>>,
}

impl FaceInputs {
    /// Read the face's inputs off the body and the chord pass. Refuses
    /// exactly where the lanes refuse on the same reads (a missing
    /// entity, an empty loop, a null-scaffold edge).
    ///
    /// This clones every carrier, pcurve and chord position list of
    /// the face, hit or miss, once per picture: the memo's whole cost
    /// on a face it cannot answer, measured at about 2 % of a ring
    /// document's tessellation (the PR's table). A borrowing form
    /// would save the copies at the price of a lifetime on the key.
    #[allow(clippy::too_many_arguments)] // one parameter per named input the lanes take
    pub(crate) fn gather(
        body: &Body<f64>,
        fk: FaceKey,
        lane: Lane,
        surface: &Surface<f64>,
        chords: &ChordPass,
        positions: &[Point3<f64>],
        chordal: f64,
        tol: Tol,
    ) -> Result<Self, TessellateError> {
        let face = body
            .get_face(fk)
            .ok_or(TessellateError::MissingEntity { what: "face" })?;
        let mut loops = Vec::with_capacity(1 + face.rings.len());
        // Split-lineage roots relabeled by first occurrence over the
        // whole face, the way the chord ids are.
        let mut roots: HashMap<topo::EdgeKey, u32> = HashMap::new();
        for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
            let walk = loop_half_edges(body, lk, fk)?;
            let mut edges = Vec::with_capacity(walk.len());
            for (hek, ek, forward) in walk {
                let edge = body
                    .get_edge(ek)
                    .ok_or(TessellateError::MissingEntity { what: "edge" })?;
                let curve = body
                    .get_curve_geom(edge.curve)
                    .ok_or(TessellateError::MissingEntity { what: "edge curve" })?
                    .certified()
                    .ok_or(TessellateError::NullScaffoldEdge { edge: ek })?;
                let ids = chords
                    .ids
                    .get(&ek)
                    .ok_or(TessellateError::MissingEntity {
                        what: "edge chords",
                    })?
                    .clone();
                let chord_params = chords
                    .params
                    .get(&ek)
                    .ok_or(TessellateError::MissingEntity {
                        what: "edge chord parameters",
                    })?
                    .clone();
                let root = body.split_root(ek, |_| false).ok();
                #[allow(clippy::cast_possible_truncation)]
                let next = roots.len() as u32;
                let lineage = root.map(|r| *roots.entry(r).or_insert(next));
                edges.push(EdgeInputs {
                    forward,
                    carrier: curve.carrier().clone(),
                    params: curve.params(),
                    positions: ids.iter().map(|&id| positions[id as usize]).collect(),
                    ids,
                    chord_params,
                    pcurve: body.pcurve(hek).map(|cache| cache.pcurve().clone()),
                    seam: matches!(curve.description(), EdgeDescription::Chart(c) if c.seam),
                    lineage,
                });
            }
            loops.push(edges);
        }
        let ambient = tol.get();
        Ok(Self {
            lane,
            chordal,
            eps: ambient.eps,
            k: ambient.k,
            surface: surface.clone(),
            sense: face.sense,
            loops,
        })
    }

    /// The face's boundary id sequence: every loop's every edge's chord
    /// ids, `he_plus`-forward, in walk order. The stored patch's shared
    /// corners are indices into this.
    pub(crate) fn boundary_ids(&self) -> Vec<u32> {
        self.loops
            .iter()
            .flatten()
            .flat_map(|e| e.ids.iter().copied())
            .collect()
    }

    /// The key bytes: the module docs' list, per lane.
    pub(crate) fn key(&self) -> Vec<u8> {
        let mut w = KeyWriter::default();
        w.tag(b"pncad-patch-key-1");
        w.u8(match self.lane {
            Lane::Planar => 0,
            Lane::Curved => 1,
            Lane::Trimmed => 2,
        });
        w.f64(self.chordal);
        w.f64(self.eps);
        w.f64(self.k);
        let (curved, trimmed) = match self.lane {
            Lane::Planar => (false, false),
            Lane::Curved => (true, false),
            Lane::Trimmed => (false, true),
        };
        // The dispatch reads the surface's kind on every lane; the
        // curved and trimmed lanes read its fields.
        w.surface_kind(&self.surface);
        if curved || trimmed {
            w.surface(&self.surface);
        }
        if curved {
            w.bool(self.sense);
        }
        // The identity relabeling over the whole face's sequence.
        let mut label: HashMap<u32, u32> = HashMap::new();
        w.len(self.loops.len());
        for lp in &self.loops {
            w.len(lp.len());
            for e in lp {
                w.bool(e.forward);
                w.curve3(&e.carrier);
                w.f64(e.params.0);
                w.f64(e.params.1);
                w.len(e.ids.len());
                for &id in &e.ids {
                    #[allow(clippy::cast_possible_truncation)]
                    let next = label.len() as u32;
                    w.u32(*label.entry(id).or_insert(next));
                }
                w.len(e.positions.len());
                for p in &e.positions {
                    w.p3(*p);
                }
                if curved {
                    w.bool(e.seam);
                    match e.lineage {
                        None => w.u8(0),
                        Some(c) => {
                            w.u8(1);
                            w.u32(c);
                        }
                    }
                }
                if trimmed {
                    w.len(e.chord_params.len());
                    for &t in &e.chord_params {
                        w.f64(t);
                    }
                    match &e.pcurve {
                        None => w.u8(0),
                        Some(p) => {
                            w.u8(1);
                            w.pcurve(p);
                        }
                    }
                }
            }
        }
        w.0
    }
}

/// Bytes of a key: every scalar by bit pattern, every sequence
/// length-prefixed, every variant tagged.
#[derive(Default)]
struct KeyWriter(Vec<u8>);

impl KeyWriter {
    fn tag(&mut self, s: &[u8]) {
        self.len(s.len());
        self.0.extend_from_slice(s);
    }

    fn u8(&mut self, x: u8) {
        self.0.push(x);
    }

    fn bool(&mut self, x: bool) {
        self.u8(u8::from(x));
    }

    fn u32(&mut self, x: u32) {
        self.0.extend_from_slice(&x.to_le_bytes());
    }

    fn u64(&mut self, x: u64) {
        self.0.extend_from_slice(&x.to_le_bytes());
    }

    fn len(&mut self, n: usize) {
        self.u64(n as u64);
    }

    fn f64(&mut self, x: f64) {
        self.u64(x.to_bits());
    }

    fn p3(&mut self, p: Point3<f64>) {
        self.f64(p.x);
        self.f64(p.y);
        self.f64(p.z);
    }

    fn v3(&mut self, v: Vec3<f64>) {
        self.f64(v.x);
        self.f64(v.y);
        self.f64(v.z);
    }

    fn p2(&mut self, p: Point2<f64>) {
        self.f64(p.x);
        self.f64(p.y);
    }

    fn v2(&mut self, v: Vec2<f64>) {
        self.f64(v.x);
        self.f64(v.y);
    }

    fn knots(&mut self, k: &KnotVector) {
        self.len(k.degree());
        self.len(k.knots().len());
        for &t in k.knots() {
            self.f64(t);
        }
    }

    fn weights(&mut self, w: &[f64]) {
        self.len(w.len());
        for &x in w {
            self.f64(x);
        }
    }

    fn nurbs3(&mut self, c: &NurbsCurve3<f64>) {
        self.knots(c.knots());
        self.len(c.control().len());
        for &p in c.control() {
            self.p3(p);
        }
        self.weights(c.weights());
    }

    fn nurbs2(&mut self, c: &NurbsCurve2<f64>) {
        self.knots(c.knots());
        self.len(c.control().len());
        for &p in c.control() {
            self.p2(p);
        }
        self.weights(c.weights());
    }

    fn nurbs_surface(&mut self, s: &NurbsSurface<f64>) {
        self.knots(s.knots_u());
        self.knots(s.knots_v());
        self.len(s.control().len());
        for &p in s.control() {
            self.p3(p);
        }
        self.weights(s.weights());
    }

    fn surface_kind(&mut self, s: &Surface<f64>) {
        self.u8(match s {
            Surface::Plane { .. } => 0,
            Surface::Cylinder { .. } => 1,
            Surface::Cone { .. } => 2,
            Surface::Sphere { .. } => 3,
            Surface::Torus { .. } => 4,
            Surface::Nurbs(_) => 5,
            Surface::Approx(_) => 6,
        });
    }

    /// The surface's fields. Exhaustive on purpose: a new variant or
    /// field is a compile error here, not a silent gap in the key.
    fn surface(&mut self, s: &Surface<f64>) {
        self.surface_kind(s);
        match s {
            Surface::Plane {
                origin,
                normal,
                u_ref,
            } => {
                self.p3(*origin);
                self.v3(*normal);
                self.v3(*u_ref);
            }
            Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => {
                self.p3(*origin);
                self.v3(*axis);
                self.f64(*radius);
                self.v3(*u_ref);
            }
            Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => {
                self.p3(*apex);
                self.v3(*axis);
                self.f64(*half_angle);
                self.v3(*u_ref);
            }
            Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            } => {
                self.p3(*center);
                self.f64(*radius);
                self.v3(*axis);
                self.v3(*u_ref);
            }
            Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            } => {
                self.p3(*center);
                self.v3(*axis);
                self.f64(*major_radius);
                self.f64(*minor_radius);
                self.v3(*u_ref);
            }
            Surface::Nurbs(n) => self.nurbs_surface(n),
            // The lane meshes the fit; the description, window and
            // certificate it was fitted against are not read.
            Surface::Approx(a) => self.nurbs_surface(a.fit()),
        }
    }

    fn curve3(&mut self, c: &Curve3<f64>) {
        match c {
            Curve3::Line { origin, dir } => {
                self.u8(0);
                self.p3(*origin);
                self.v3(*dir);
            }
            Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => {
                self.u8(1);
                self.p3(*center);
                self.v3(*axis);
                self.f64(*radius);
                self.v3(*u_ref);
            }
            Curve3::Ellipse {
                center,
                axis,
                major,
                minor,
                u_ref,
            } => {
                self.u8(2);
                self.p3(*center);
                self.v3(*axis);
                self.f64(*major);
                self.f64(*minor);
                self.v3(*u_ref);
            }
            Curve3::Nurbs(n) => {
                self.u8(3);
                self.nurbs3(n);
            }
        }
    }

    fn pcurve(&mut self, p: &Pcurve<f64>) {
        match p {
            Pcurve::Harmonic { p0, pa, pb, pl } => {
                self.u8(0);
                self.p2(*p0);
                self.v2(*pa);
                self.v2(*pb);
                self.v2(*pl);
            }
            Pcurve::Fitted(c) => {
                self.u8(1);
                self.nurbs2(c);
            }
            Pcurve::General(c) => {
                self.u8(2);
                self.nurbs2(c);
            }
            Pcurve::IsoLine { p0, pl } => {
                self.u8(3);
                self.p2(*p0);
                self.v2(*pl);
            }
            Pcurve::IsoArc {
                p0,
                pd,
                t0,
                angle,
                breaks,
            } => {
                self.u8(4);
                self.p2(*p0);
                self.v2(*pd);
                self.f64(*t0);
                self.f64(*angle);
                self.knots(breaks);
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! The key's read list, per lane, as a table: for each input, does
    //! changing it — and nothing else — change the key? The body-level
    //! differential (`tests/patch_memo.rs`) drives the same question
    //! through the body's doors for the inputs a door can change; this
    //! is the whole list, including the inputs no door can isolate.

    use super::*;

    fn p(x: f64) -> Point3<f64> {
        Point3::new(x, 2.0 * x, -x)
    }

    fn edge(seed: f64, ids: [u32; 3]) -> EdgeInputs {
        EdgeInputs {
            forward: true,
            carrier: Curve3::Line {
                origin: p(seed),
                dir: Vec3::new(1.0, 0.0, seed),
            },
            params: (0.0, 1.0),
            ids: ids.to_vec(),
            positions: ids.iter().map(|&i| p(f64::from(i))).collect(),
            chord_params: vec![0.0, 0.5, 1.0],
            pcurve: Some(Pcurve::IsoLine {
                p0: Point2::new(seed, 0.0),
                pl: Vec2::new(0.0, 1.0),
            }),
            seam: false,
            lineage: Some(ids[0]),
        }
    }

    fn sample(lane: Lane) -> FaceInputs {
        FaceInputs {
            lane,
            chordal: 0.05,
            eps: 1e-9,
            k: 10.0,
            surface: Surface::Cylinder {
                origin: Point3::origin(),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: 1.0,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            sense: true,
            loops: vec![vec![edge(1.0, [10, 11, 12]), edge(2.0, [12, 13, 10])]],
        }
    }

    /// `(what moved, the move, misses on [planar, curved, trimmed])`.
    type Row = (&'static str, fn(&mut FaceInputs), [bool; 3]);

    const ROWS: &[Row] = &[
        ("δ", |f| f.chordal *= 2.0, [true, true, true]),
        ("ε", |f| f.eps *= 2.0, [true, true, true]),
        ("k", |f| f.k += 1.0, [true, true, true]),
        (
            "the surface's kind",
            |f| {
                f.surface = Surface::Sphere {
                    center: Point3::origin(),
                    radius: 1.0,
                    axis: Vec3::new(0.0, 0.0, 1.0),
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                }
            },
            [true, true, true],
        ),
        (
            "a surface parameter",
            |f| {
                if let Surface::Cylinder { radius, .. } = &mut f.surface {
                    *radius *= 2.0;
                }
            },
            [false, true, true],
        ),
        (
            "the face's sense",
            |f| f.sense = !f.sense,
            [false, true, false],
        ),
        (
            "an edge's direction",
            |f| f.loops[0][1].forward = false,
            [true, true, true],
        ),
        (
            "an edge's carrier",
            |f| {
                f.loops[0][1].carrier = Curve3::Line {
                    origin: p(9.0),
                    dir: Vec3::new(1.0, 0.0, 2.0),
                }
            },
            [true, true, true],
        ),
        (
            "an edge's parameter interval",
            |f| f.loops[0][1].params = (0.0, 2.0),
            [true, true, true],
        ),
        (
            "the chord ids, renumbered one-to-one",
            |f| {
                for e in &mut f.loops[0] {
                    for id in &mut e.ids {
                        *id += 100;
                    }
                }
            },
            [false, false, false],
        ),
        (
            "the chord ids' identity structure",
            |f| f.loops[0][1].ids = vec![12, 13, 14],
            [true, true, true],
        ),
        (
            "a chord position",
            |f| f.loops[0][1].positions[1] = p(-7.0),
            [true, true, true],
        ),
        (
            "a chord parameter",
            |f| f.loops[0][1].chord_params[1] = 0.25,
            [false, false, true],
        ),
        (
            "a pcurve",
            |f| {
                f.loops[0][1].pcurve = Some(Pcurve::IsoLine {
                    p0: Point2::new(3.0, 0.0),
                    pl: Vec2::new(0.0, 1.0),
                })
            },
            [false, false, true],
        ),
        (
            "a pcurve's presence",
            |f| f.loops[0][1].pcurve = None,
            [false, false, true],
        ),
        (
            "an edge's seam flag",
            |f| f.loops[0][1].seam = true,
            [false, true, false],
        ),
        (
            "the carriers' identity structure",
            |f| f.loops[0][1].lineage = f.loops[0][0].lineage,
            [false, true, false],
        ),
        (
            "a carrier lineage that stops resolving",
            |f| f.loops[0][1].lineage = None,
            [false, true, false],
        ),
    ];

    #[test]
    fn each_input_moves_the_key_of_exactly_the_lanes_that_read_it() {
        for (what, moved, expect) in ROWS {
            for (i, lane) in [Lane::Planar, Lane::Curved, Lane::Trimmed]
                .into_iter()
                .enumerate()
            {
                let base = sample(lane);
                let mut after = sample(lane);
                moved(&mut after);
                let missed = base.key() != after.key();
                assert_eq!(
                    missed,
                    expect[i],
                    "{what} moved: the {lane:?} lane {} miss",
                    if expect[i] { "should" } else { "should not" }
                );
            }
        }
    }

    #[test]
    fn the_key_folds_the_lane_and_a_renumbered_boundary_is_the_same_face() {
        assert_ne!(sample(Lane::Planar).key(), sample(Lane::Curved).key());
        assert_ne!(sample(Lane::Curved).key(), sample(Lane::Trimmed).key());
        let a = sample(Lane::Trimmed);
        let mut b = sample(Lane::Trimmed);
        for e in &mut b.loops[0] {
            for id in &mut e.ids {
                *id = 1000 - *id;
            }
        }
        assert_eq!(a.key(), b.key());
        assert_ne!(a.boundary_ids(), b.boundary_ids());
    }

    #[test]
    fn a_stored_patch_is_placed_against_the_new_boundary_ids() {
        let patch = Patch {
            interior: vec![p(0.5)],
            triangles: vec![
                [
                    PatchVertex::Shared(10),
                    PatchVertex::Shared(11),
                    PatchVertex::Local(0),
                ],
                [
                    PatchVertex::Shared(12),
                    PatchVertex::Shared(10),
                    PatchVertex::Local(0),
                ],
            ],
        };
        let stored = StoredPatch::store(&patch, &[10, 11, 12, 12, 13, 10])
            .expect("every id is on the boundary");
        let placed = stored.restore(&[20, 21, 22, 22, 23, 20]);
        assert_eq!(placed.interior.len(), 1);
        assert_eq!(
            placed.interior[0].x.to_bits(),
            patch.interior[0].x.to_bits()
        );
        assert_eq!(
            placed.triangles,
            vec![
                [
                    PatchVertex::Shared(20),
                    PatchVertex::Shared(21),
                    PatchVertex::Local(0)
                ],
                [
                    PatchVertex::Shared(22),
                    PatchVertex::Shared(20),
                    PatchVertex::Local(0)
                ],
            ]
        );
        assert!(
            StoredPatch::store(&patch, &[10, 11]).is_none(),
            "an id off the boundary cannot be stored"
        );
    }

    #[test]
    fn a_hit_compares_the_whole_key_not_only_the_digest() {
        let mut memo = PatchMemo::new();
        let digest = PatchDigest::of(b"a");
        memo.insert(
            digest,
            b"a".to_vec(),
            StoredPatch {
                interior: Vec::new(),
                triangles: Vec::new(),
            },
        );
        let mut keys = PatchKeys::default();
        assert!(memo.stored(digest, b"a").is_some());
        memo.record(
            FaceMemo {
                digest,
                outcome: Outcome::Hit,
            },
            &mut keys,
        );
        assert!(
            memo.stored(digest, b"b").is_none(),
            "same digest, other bytes: a miss"
        );
        memo.record(
            FaceMemo {
                digest,
                outcome: Outcome::Miss {
                    key: b"b".to_vec(),
                    stored: Some(StoredPatch {
                        interior: Vec::new(),
                        triangles: Vec::new(),
                    }),
                },
            },
            &mut keys,
        );
        assert_eq!((memo.hits(), memo.misses()), (1, 1));
        assert_eq!(keys.len(), 2, "one digest per face, in the fold's order");
        memo.end_picture();
        assert_eq!(memo.len(), 1);
        memo.end_picture();
        assert_eq!(memo.len(), 0, "unused across a picture: evicted");
    }
}
