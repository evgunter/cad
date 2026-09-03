# VERB-SEAT-DESIGN: the kernel query seat, one verb vocabulary, and lowered parameter identity

Status: **RATIFIED** (PR #1388, Ev's sign-off in-session,
2026-08-31; companion-table row at DESIGN.md). Originally a
design-conversation PR; the text below is the ratified record.
This doc proposes the mechanism for issue #1345's decided item (1)
and its cheap fix, the producer gap of issue #757, and the
parameter-identity channel of issue #1372 — one conversation because
the three share a load-bearing precedent (§0). Items (2)/(3) of
#1345 stay open; §5 names their trigger. Mechanics below are
measured (file:line), not assumed.

## 0. Grounding (committed; this doc does not re-litigate)

- **G1 layering / D5 / D3 / D9 / N1–N7.** Kernel verbs speak arena
  keys and return per-op birth records as plain data
  (`BlendNaming`, `BooleanNaming`, `SplitNaming`); names are
  derivation paths minted upstairs; the name table is a function of
  (recipe structure, structural params, verdict vector) only (N4);
  every dispatch enum is closed with no wildcard arms (D3).
- **The kernel is serde-free** (M4 PR 6 / F3): persistence lives in
  `editor-core` only. Schema bumps are clean breaks pre-publish
  (LQ7a).
- **The lowered-identity precedent, which this doc extends.**
  `GeomSource` (N6) lives in the KERNEL — `topo/src/source.rs`,
  stored as `SecondaryMap<SurfaceKey, GeomSource>` /
  `<CurveKey, …>` on `Body` (`body.rs:180-181`), read by the
  boolean's coincidence rungs (`plane_eq.rs:166`,
  `merge_faces.rs`). Its module docs state the layering ruling:
  the recipe vocabulary (typed node ids, expression paths) lives in
  `editor-core`; the kernel holds the **lowered pure-data forms**
  (`u64` node ids, structural expression addresses) and only ever
  compares them for identity. The records are **opt-in side
  tables** (`Option<&GeomSource>`), attached by `editor-core`
  (`set_surface_source`), absent for a kernel-direct caller. So the
  line the design actually draws is: the kernel may hold and
  compare lowered identity data beside its arenas; it never holds
  the typed recipe vocabulary (`Expr`, `RecipeNodeId`,
  `StableName`) and never persistence.
- **The measured per-verb cost baseline** (the `Node::Chamfer`
  merge, #1224): ~19 sites across 8 files in 4 crates. Three sites
  the older "nine arms" list names cost zero — diff (via
  `Node::bit_eq` over `slots()` + `payload_exprs()`), appearance
  (a `BTreeMap<StableName, …>`), edit validation (driven off
  `payload_names()`/`slots()`/`placement_rule_fault()`) — and they
  are free precisely because they consume `Node`'s structural
  traversal doors instead of matching variants. The scattered
  remainder: ~8 traversal doors in `node.rs` that are all
  projections of one fact (the payload's shape), the semantic
  registrations (`run_op` wiring, content tag, emitter, persist
  check), and the surface mirrors (schema bump, Python, viewer,
  prelude).
- **Demand evidence.** The tour's kernel-seat frictions
  (`bodies.rs` spacer, `diechamfer.rs`, `twopeg.rs`, `lily.rs`,
  `klein.rs` finding 8, `bud.rs`, `teapot.rs`): whole-body edge
  sets spelled as arena walks, selections re-derived as
  carrier-kind loops, one intended contact spelled as nine
  `FacePairDeclaration`s. Issue 757: `BooleanDeclarations`
  (`topo/src/boolean/mod.rs:314`) has a public consumer and no
  geometric producer; the ~55-line hand declarer is twinned in
  `demos/tour/src/booleans.rs` and `topo/tests/common/mod.rs`.
  Issue 1372: `cylinder_cylinder_section`'s `RadiusEvidence`
  parameter (geom-brep) has no production caller that could ever
  supply it, because nothing carries parameter identity from the
  recipe layer to boolean dispatch.

## 1. The kernel query seat

**S1 — a query module in `topo`, beside `Body`.** The selection
vocabulary's geometric half is a pure function of a body and moves
to the layer whose types it serves:

- Materializers: `all_edges(&Body<T>) -> Vec<EdgeKey>` and
  `all_faces(&Body<T>) -> Vec<FaceKey>`, deterministic arena order
  (D9). They retire the hand-rolled fold currently repeated across
  six `sweep` test files and `diechamfer.rs`.
- The EXACT atoms as pure predicates over `(&Body<T>, key)`:
  carrier kind of an edge, surface kind of a face, unordered
  adjacent-kind pair across an edge. `CurveKind` moves down beside
  `Curve3` (its own doc note at
  `editor-core/src/names/geompred.rs:77` records the move as
  additive); `SurfaceKind` stays in `geom-brep` and is reused
  (the one-fieldless-mirror rule).
- The DECIDED atom in **resolved** form: distance of an entity to a
  passed-in plane/axis/point value, compared through the same
  `sel_*`-named `k_stats` funnel sites the document door uses today
  (`k_stats` is `geom-core`, already below `topo`; the site moves,
  it is not twinned). Datum-node resolution — `RecipeNodeId` →
  `DatumValue` — stays in `editor-core`'s `prepare`.

**S2 — `select_where` becomes a wrapper.** The engine is already
factored for this: after `prepare`
(`editor-core/src/names/geompred.rs:571`) resolves recipe
references, `candidate_matches` (`geompred.rs:640`) consumes only
`&Body<T>`, an arena key, prepared atoms and a `Band`; its
`StableName` argument exists only for refusal payloads. That core
moves down; `editor-core` keeps everything name-flavored — the
structural `Selector`, the GS-Q4 tie trilean, refusal payload
assembly — and delegates the per-entity test. One implementation of
the geometry, two doors, the `ContactClass` re-export precedent
(SELECT-DESIGN §3(e)).

**S3 — the flush detector at the body seat.** `topo` gains
`find_flush_candidates(&Body<T>, &Body<T>, tol)` returning findings
in `FaceKey`s, implemented as the C4 verifier run in
candidate-generation mode — the verifier (`oriented_plane_eq`,
`topo/src/boolean/plane_eq.rs`) already lives there, so the
anti-twin rule ("the detector interprets nothing the verifier
doesn't") holds by construction. `declare_all` sugar produces the
`BooleanDeclarations` the op door takes; the no-fusion rule (GS-Q3)
applies at this seat exactly as at the document seat. The existing
name-level `find_flush_candidates`
(`editor-core/src/names/flush.rs:187`) becomes the derived wrapper
(keys → names through the table). One sentence of SELECT-DESIGN §3
is restated rather than silently contradicted: findings are names
at the document door, keys at the body door, one verifier under
both. This gives `BooleanDeclarations` its geometric producer and
deletes nothing that must stay (the demo fixture twins keep their
LB11 disposition until the library door exists to replace them).

**S4 — the blend verbs' `band` parameter is dropped.** `Tol` is a
zero-sized witness of the committed global tolerance
(`geom-core/src/tolerance.rs:679`), so `Band::linear(tol)` is a
function of the commit alone. The kernel already disagrees with
itself: `extrude`, `revolve`, `loft_body` and `tube_along_arc`
derive the band internally at operation entry (`extrude.rs:437`,
`revolve/mod.rs:680`, `loft.rs:269`, `tube.rs:340`); only
`fillet_edges`/`chamfer_edges` (`sweep/src/blend/build.rs:322`) and
`topo::shell`/`shell_open` expose the parameter, and every call
site in the tree passes exactly the linear derivation. The blend
and shell doors align with their siblings; a derived-scale band
remains constructible where an op genuinely needs one
(`Band::new`/`Band::angular_at` at the geometry layer, per the
existing convention).

§1 is executable ahead of §§2–3 and prejudges neither.

## 2. One verb vocabulary

**The desideratum** (#1345, decided in direction): adding a verb
costs one implementation, not a re-declaration scattered across the
document layer. **The mechanism proposed here:**

**V1 — a closed, kernel-side per-verb declaration.** A `Verb<T>`
enum beside the kernel ops (home: ledger VS-Q1), payload = the
verb's parameters reified as data — scalars at `T`, entity
references as arena keys, operand bodies NOT in the payload (they
are borrowed at run time; the declaration states operand arity and
kind). Each verb impl owns:

- `run(&self, operands, tol) -> Result<VerbOut<T>, VerbError>` —
  one dispatch site over the existing verb functions;
- its **birth-record shape** (the `BlendNaming` obligation
  generalized — this is RECIPE-DOORS D5's `ShellNaming` ask stated
  as a rule: a verb without a birth channel cannot join the enum);
- its **parameter→field flow**: which parameter lands in which
  field of which minted description, as data. This is §3's channel
  and the reason the declaration must sit beside the op — only the
  op knows its flow.

No serde, no `Expr`, no names — and no knowledge of Python,
persistence, memoization or display: the enum is the canonical
NAME and nothing more, within the §0 lowered line.

**V2 — commitments are exhaustive matches to stable tags, held by
their owners, so spellings cannot drift.** Every commitment a verb
has is an exhaustive match over the one canonical enum, mapping
variants to stable tags — and each match lives in the crate that
owns the commitment, looking AT the canonical name: the content-key
tag beside the memo machinery (`editor-core`), the wire spelling in
`persist`, the Python constructor in `pncad-py`, the tree label in
`viewer`. The kernel says nothing about any of them. The compiler
forces every commitment site to be visited on any variant change
(D3, no wildcard arms), so kernel and persisted spellings cannot
drift silently; the tag indirection means a kernel-side rename is a
compile-guided visit to each match, not a re-spelling of saved
files. (`eval/mod.rs`'s content-tag match is already this shape;
the `RimSide → RimSupport` emitter match at
`emit_blend.rs:192` already has the no-silent-drift property — what
it lacks is a canonical owner, which V1 supplies. Whether the twin
enum then collapses is ledger VS-Q5.)

**V3 — `editor-core` keeps the authoring vocabulary, loses the
restatements.** A verb's `Node` payload remains typed document data
— `Expr` per slot, frozen canonical `Vec<StableName>` selections,
`RecipeNodeId` inputs — because that IS the document's semantics
(what is an expression, what is a frozen reference, how resolution
refuses), not a restatement of the kernel. What goes away is the
scattering: the per-verb correspondence (slot ↔ parameter,
selection ↔ reference field, input ↔ operand) is declared once,
colocated with the verb's emitter, and the mechanical `node.rs`
projections and the `wire_*` lowering run generically off it —
resolve names to keys (the N5 ladder, unchanged), evaluate slots to
`T`, build `Verb<T>`, run, attach provenance (§3), emit names from
the birth record. Adding a verb then costs: the kernel `Verb` arm
(function + declaration), one `editor-core` verb module
(correspondence + emitter), the stable-tag rows, one schema bump,
and the surface mirrors (Python, viewer, prelude) — measured
against the chamfer baseline at the first door that lands on this
shape (§6).

**Honest counterarguments, recorded.** (a) The dispatch-arm cost
this replaces is compiler-guided and was priced and accepted
(GROUP-BOOLEAN "costs, eyes open"); the chamfer and tube doors
shipped through it routinely. The answer is that #1372 changes what
must be declared per verb — parameter flow is new, per-op knowledge
with no home today — so the per-verb declaration is now needed for
correctness, not only for cost. (b) A shared declaration couples
kernel refactors to schema-visible events; V2's stable tags reduce
that to a compile-guided visit, and pre-publish bumps are cheap,
but post-publish this is a discipline (a kernel variant reshape
forces a schema decision) — accepted deliberately, drift being the
alternative. (c) Migration touches the hottest editor-core code;
V4 sequences it.

**V4 — migration is per-verb and additive.** The enum lands with
two or three verbs (the blend pair first — signature-identical
twins, the richest birth record) while the remaining `Node` arms
keep their current wiring; each subsequent verb moves in its own
unit. No flag-day rewrite; the wire format changes only where a
migrated verb's spelling actually changes (target: it does not —
the persisted spelling is pinned by V2's tags before the first verb
moves).

## 3. Lowered parameter identity (issue 1372)

**P1 — the channel is lowered *expression* identity, per stored
field, in opt-in side records — `GeomSource` one level finer and
MORE opaque, not another mirror.** A `ParamSource` is attached
beside the geometry arenas for the stored scalar fields of minted
descriptions — per-kind field slots keyed like the existing
`surface_sources` table. To the kernel it is a fully **opaque
token**: `Eq`/`Ord` and nothing else — no fields the kernel can
read, no structure it can compose. The single spelling of
expression identity lives in `editor-core`, which lowers an
expression address to a deterministic token (D9: the interning is a
function of the recipe) and inverts it for diagnosis. This is
deliberately LESS structure than `GeomSource` carries:
`SourceExpr::Placed` exists in the kernel only because rigid
placement re-parameterizes a *description*, so whoever runs the op
must compose the record — a scalar field like a radius is
motion-invariant, so no kernel op ever composes or interprets its
source, and no second spelling of expression structure enters the
kernel. Identity is token equality, zero numerics;
equality-by-provenance, true by construction. Carrying expression
identity rather than a parameter handle answers the issue's offset
question by construction: both walls offset by the same declared
`t` lower to the same `r ± t` token and stay equal by syntax; `r`
vs `r ± t` differ. The scope caveat of `topo/src/source.rs` applies
verbatim: identity holds per evaluation against the current
document, never across unaudited document mutations.

**P2 — who attaches, who propagates, who consumes.**
`editor-core`'s lowering attaches sources at mint time, driven by
the verb's declared parameter→field flow (V1) and the slot's
expression address — the `set_surface_source` pattern. Kernel ops
never mint, compose or interpret sources; they carry the token
verbatim: survivors keep their records by key identity, rigid
placement carries them unchanged (the fields are motion-invariant),
kills drop records. A
kernel-computed derived value (e.g. the hollow tube's
`minor_radius − wall`) carries no source in v1 — identity ends
where `editor-core` did not evaluate the expression (ledger VS-Q3).
Consumers read positioned evidence: the cyl×cyl equal-radius germ's
`RadiusEvidence` gains its production caller (same field sources on
the two carriers ⇒ `Declared`); SPHSPH's structural-parallelism
option reads the same channel at its own position. One mechanism,
several typed positions (the issue's Q3).

**P3 — absence refuses, permanently.** Where no source exists —
imported geometry, hand-built bodies, kernel-derived fields — the
consuming family refuses typed, and that refusal is the permanent
fallback, not a gap (the issue's Q4; the current PR-2 behavior).
No numeric fallback: comparing stored radii would be measurement
masquerading as structure, the thing the contract forbids.

## 4. Question ledger

- **VS-Q1 — where does `Verb` live?** Recommendation: a new small
  crate above `sweep` and `topo`, below `editor-core` (the enum
  spans both crates' ops; `sweep` hosting boolean dispatch misnames
  the layer). Counterargument: crate count and the build-cost
  ledger (GENERICS-BUILD-COST); a `sweep`-hosted module is the
  cheap fallback if the crate is judged not worth its manifest.
- **VS-Q2 — `Node` payload shape.** Recommendation: keep typed
  per-verb `Node` variants (persistence spelling, typed edits,
  `deny_unknown_fields` all stand) with the correspondence declared
  once per verb (V3). Rejected: a uniform `Node::Op` slot-map
  variant — it trades compile-time exhaustiveness at the document
  layer for runtime arity checking, the silent-dispatch shape D3
  forbids. Deferred: macro-generating variants + projections from
  the correspondence, if the remaining arm noise still bites after
  two migrated verbs (the repo is deliberately macro-light).
- **VS-Q3 — kernel-derived fields.** v1: no source (P2).
  Alternative recorded: composite lowered sources minted by the
  kernel for its own arithmetic — rejected for now because it puts
  expression algebra below the line §0 draws, and no consumer needs
  it (the tube's inner wall has no equal-radius partner to
  declare).
- **VS-Q4 — `ParamSource` representation.** Recommendation: an
  opaque interned token, minted deterministically by `editor-core`
  from the lowered expression address, `Eq`-compared by the kernel
  and inverted upstairs for diagnosis (P1). Rejected: a
  SourceExpr-style structural address IN the kernel — a second
  spelling of expression structure below the line, with no kernel
  consumer for the structure (nothing composes a motion-invariant
  field). Rejected: a content digest — identity claims become
  hash-collision-shaped for no gain over interning.
- **VS-Q5 — does `RimSide`/`RimSupport` collapse onto V2's
  pattern?** Once a canonical owner exists, the persisted spelling
  can be a stable-tag match over the kernel enum and the twin
  nominal type retires. Not load-bearing either way for two
  variants; decide when the blend pair migrates (V4).
- **VS-Q6 — sequencing.** §1 first (its own unit(s), no dependency
  on §§2–3); then §2 with the blend pair; §3 rides the first §2
  verb whose consumer needs it (the cyl×cyl germ) or lands with the
  boolean's migration, whichever is cut first.

## 5. Out of scope, recorded

- **#1345 items (2)/(3) — call-minted anchors and the document as
  a log of invocations.** Deferred as one future conversation, not
  two: a name's value is persistence across re-evaluation, so
  anchors minted by seatless calls have meaning only where a
  replayable call log exists. The trigger is a real consumer (a
  scripting/replay story for direct callers). Nothing here
  forecloses it — the birth-record channel V1 requires per verb is
  the substrate that proposal would widen, and `Verb<T>` is the
  value such a log would store.
- **The #917 `OpGroup::Fillet` rename** — its own issue, its own
  scale.
- **Post-publish schema discipline** for V2's stable tags — folds
  into DESIGN.md's "Before publishing" list at ratification.

## 6. Acceptance sketch

- The next recipe door after §2 lands is costed against the chamfer
  baseline: kernel arm + one editor-core module + tag rows + bump +
  mirrors, with the `node.rs` projection matches unchanged by the
  addition.
- The cyl×cyl equal-radius germ reaches its closed form end to end
  from a document declaring one shared radius parameter, and
  refuses typed on the same geometry imported (P3).
- The spacer's frictions 1 and 3, diechamfer finding 2 and klein
  finding 8 retire at their sites (demo doctrine: workarounds
  deleted where re-authored); twopeg's nine and lily's six
  declarations collapse to detector + declare; the two ~55-line
  flush declarers are deleted with the library door in place.
- N4's invariant is untouched: name tables remain a function of
  (recipe structure, structural params, verdicts); no naming
  machinery moves below the G1 line.
