# verbs: the kernel verb vocabulary seat

`verbs` is where the kernel's operations are named. One closed `Verb<T>`
enum reifies an operation's parameters as plain data — scalars at `T`,
entity references as arena keys, operands never in the payload — beside
the run dispatch over the op crates' own doors and the parameter→field
flow that only the operation itself knows. The crate sits above `sweep`
and `topo`, whose ops it names, and below `editor-core`, its only
consumer; the layering line it draws is `GeomSource`'s
(`topo/src/source.rs`): lowered pure data may sit beside the arenas and
be compared for identity, the recipe vocabulary — `Expr`, `StableName`,
`RecipeNodeId`, serde — may not, and `tests/layer_guard.rs` enforces
that rather than a comment asserting it. Two further seats of the same
design live in `topo`: the query seat (`topo::query`), the geometric
half of the selection vocabulary as pure functions of a `Body`, and the
flush detector (`topo::flush`), which gives `BooleanDeclarations` its
geometric producer. The third strand is lowered parameter identity: an
opaque per-field token minted in `editor-core`, carried verbatim by
kernel ops and compared for equality alone, so "these two radii are the
same" is a fact about the recipe and never a measurement. Names at the
document door, keys at the body door, one implementation under both.

## Where in the code

| Clause | Lives in |
|---|---|
| S1 kernel query seat | `crates/topo/src/query.rs`; exported at `crates/pncad/src/prelude.rs` |
| S2 `select_where` as a wrapper | `crates/editor-core/src/names/geompred.rs` |
| S3 flush detector at the body seat | `crates/topo/src/flush.rs`, verifier at `crates/topo/src/boolean/rest.rs` (`carrier_pair_relation`); the name-level wrapper at `crates/editor-core/src/names/flush.rs` |
| S4 band derived at op entry | `crates/sweep/src/blend/build.rs`, `crates/topo/src/shell.rs` |
| V1 the closed kernel-side declaration | `crates/verbs/src/verb.rs` (`Verb`, `VerbKind`, `Arity`), `run.rs` (the doors, `VerbOut`/`PairOut`/`SplitOut`/`VerbRecord`/`VerbError`), `flow.rs` (`ParamFlow`) |
| V2 owner-held stable-tag commitments | `crates/editor-core/src/eval/mod.rs` (`verb_content_tag`) |
| V3 per-verb correspondence | `crates/editor-core/src/verbs/{mod,blend,boolean,sweep,split}.rs`; the generic lowerings in `crates/editor-core/src/eval/wire.rs` |
| V4 additive migration | `crates/verbs/src/lib.rs` (what is on the seat and what is not) |
| P1 the lowered token | `crates/topo/src/param_source.rs` (the opaque token, the side tables); `crates/editor-core/src/param_source.rs` (the single spelling: `lower`, `invert`, the scope) |
| P2 attach / propagate / consume | `crates/editor-core/src/param_source.rs` (`attach_blend`, `attach_swept`); `crates/topo/src/param_source.rs` (`field_source_evidence`); the consumer at `crates/topo/src/boolean/join.rs` (`germ_section_frame`) with `RadiusEvidence` in `crates/geom-brep/src/intersect.rs` |
| P3 absence refuses | `crates/editor-core/tests/seat6_param_source.rs` |
| The censuses and guards | `crates/verbs/tests/{layer_guard,run_door,param_flow}.rs`; `VerbKind::ALL`, `Arity::ALL`, `ScalarParam::ALL`, `FlowSource::ALL` |

## 0. Grounding (the premises, not re-litigated here)

- **G1 layering / D3 / D5 / D8 / D9 / N1–N7.** Kernel verbs speak arena
  keys and hand back per-op birth records as plain data (`BlendNaming`,
  `BooleanNaming`, `SplitNaming`, `ShellNaming`, the sweeps' bundles);
  names are derivation paths minted upstairs; the name table is a
  function of (recipe structure, structural params, verdict vector) only
  (N4); the recipe is data (D8); every dispatch enum here is closed with
  no wildcard arm (D3), so a variant added anywhere breaks each
  commitment site at compile time.
- **The kernel is serde-free** (F3): persistence lives in `editor-core`;
  `scripts/gates/kernel-serde-free.sh` covers this crate and
  `tests/layer_guard.rs` is the in-crate half failing with it.
- **The lowered-identity precedent this seat extends.** `GeomSource`
  (N6) lives in the kernel — `topo/src/source.rs`, stored as
  `SecondaryMap<SurfaceKey, GeomSource>` / `<CurveKey, …>` on `Body` and
  read by the boolean's coincidence rungs — as opt-in side tables
  attached by `editor-core`, absent for a kernel-direct caller. The
  line: the kernel may hold and compare lowered identity data beside its
  arenas; it never holds the typed recipe vocabulary, nor persistence.
- **The per-verb cost baseline** is the `Node::Chamfer` merge (#1224):
  ~19 sites across 8 files in 4 crates, of which diff, appearance and
  edit validation cost zero because they consume `Node`'s structural
  traversal doors instead of matching variants. Every door migrated
  after §2 is costed against it (§6).
- **The K census.** The one decided selector site here is
  `sel_datum_distance` in `topo::query`, with `datum_unit_norm` under it
  as the datum vocabulary's own decision; both carry K-REPORT rows.
- **The demand this answers.** Whole-body edge sets once spelled as
  arena walks and one intended contact as many `FacePairDeclaration`s;
  `BooleanDeclarations` with a public consumer and no geometric producer
  (#757); `RadiusEvidence` with no caller that could supply it (#1372),
  nothing carrying parameter identity down to boolean dispatch.

## 1. The kernel query seat

**S1 — the query module lives in `topo`, beside `Body`.** The selection
vocabulary's geometric half is a pure function of a body and sits at the
layer whose types it serves (`topo/src/query.rs`):

- Materializers `all_edges(&Body<T>) -> Vec<EdgeKey>` and
  `all_faces(&Body<T>) -> Vec<FaceKey>` answer in deterministic arena
  order (D9), retiring the hand-rolled folds the demos and sweep tests
  repeated.
- The EXACT atoms are pure predicates over `(&Body<T>, key)`:
  `edge_carrier_matches`, `face_surface_matches`, `edge_adjacent_matches`
  and the kind reads under them. They read a carrier's enum TAG, go
  through no funnel and carry no margin, and answer an honest NO on a
  missing carrier or a dangling key. `CurveKind` is defined here — the
  mirror lives where it is used, beside the predicates that read it —
  while `SurfaceKind` stays the workspace's one fieldless surface mirror
  in `geom-brep` and is reused.
- The DECIDED atom is resolved: `datum_distance_sign` measures an
  entity's point against a passed-in `DatumValue` through the
  `SEL_DATUM_DISTANCE` funnel site in `geom-core`'s `k_stats`, with an
  honest `Margin` door and a typed indeterminate in band. Datum-node
  resolution — `RecipeNodeId` → `DatumValue` — stays in `editor-core`'s
  `prepare`. Two further doors here are deliberately not selection
  questions and say so: `is_finite_length` (whether it belongs at this
  seat is an open question filed against the seat's owner) and
  `decide_unit_direction`, the workspace's one `Margin::norm3`
  decide-then-normalize body, reached under two ratified funnel names by
  the datum constructor and the evaluation layer's direction door.
- **`rim_of(&Body<T>, EdgeKey) -> Result<Vec<EdgeKey>, RimError>`** is a
  fourth EXACT door: the rim an arc belongs to, whole
  (FILLET-RIM, retired into `docs/DOC-LEDGER.md`). It reads stored tags
  and stored carrier fields bit for bit, no funnel and no margin — but it
  returns a SET, so it refuses typed at every point a predicate would
  answer NO: an empty set and a partial set are both answers a caller
  would act on, and a rim handed back short is a fillet request that
  stalls at a seam vertex. It adds no vocabulary beyond `RimError`.

**S2 — `select_where` is a wrapper.** `editor-core`'s `geompred` keeps
everything name-flavored — the `GeomPred` atom vocabulary whose datum is
a recipe reference, `prepare`, the tie trilean, the `SelectRefusal`
payloads that name the candidate — and `candidate_matches` delegates
atom by atom to `topo::query`, re-exporting the kind mirrors and
kind-set comparands upward. One implementation of the geometry, two
doors, the `ContactClass` layering precedent (SELECT-DESIGN §3(e)).

**S3 — the flush detector runs at the body seat.**
`topo::flush::find_flush_candidates(&Body<T>, &Body<T>, tol)` returns
findings in `FaceKey`s, and `declare`/`declare_all` turn them into the
`BooleanDeclarations` the op door takes. The anti-twin rule holds by
identity rather than by care: the detector has no predicate triple of
its own but calls `carrier_pair_relation` (`topo/src/boolean/rest.rs`)
in `declared: false` mode — the same function verify-at-use calls in
`declared: true` mode — so a pair the detector calls flush cannot be a
pair the declared rung then contradicts, and detection's decisions land
at the verifier's own funnel sites. Findings are DEFINITE; a pair whose
margin lands in band is neither reported nor dropped but refuses
`FlushRefusal::PairInBand` naming it. Scope covers the curved rungs as
well as the planar one, which widens the failure surface as much as the
answer: a bore whose radius misses its peg's by less than the band makes
the whole query refuse. `FlushFinding<P>` is one generic in the pair
vocabulary — names at the document door, keys at the body door, one
verifier under both — with `editor-core/src/names/flush.rs` the derived
wrapper. Both ~55-line hand declarers are gone.

**S4 — the blend and shell doors take no `band`.** `Tol` is a
zero-sized witness of the committed global tolerance, so `Band::linear`
is a function of the commit alone, and every door derives its band at
operation entry: `fillet_edges` and `chamfer_edges`
(`sweep/src/blend/build.rs`) beside `extrude`, `revolve` and
`loft_body` (`tube_along_arc` forwards its `Tol` to the revolve), and
`shell`/`shell_open` (`topo/src/shell.rs`), where `shell` reaches
`shell_open` so both derive once. The shell doors
carry no second epsilon either: the `Tol` witness travels into
`geom-brep`'s production fit doors, which read `eps()` beside the
classification. A derived-scale band stays constructible where an op
needs one (`Band::new`, `Band::angular_at`, at the geometry layer).

§1 is independent of §§2–3 and prejudges neither.

## 2. One verb vocabulary

**V1 — the per-verb declaration is closed and kernel-side.** `Verb<T>`
(`verbs/src/verb.rs`) holds an operation's parameters as data: `Fillet`
and `Chamfer` (edge keys and a scalar), `Extrude` (a signed distance),
`Revolve` (a sketch-plane axis and a classified `Revolution`), `Boolean`
(the regularized op and the declared coincidence intents in arena keys),
`Split` (the parting plane) and `Shell` (a thickness and the faces to
open — one variant, since an empty designation IS the sealed hollow).
Operand bodies are not in the payload: they are borrowed at the run
doors, and what an operand IS, and how many, is declared data. The enum
is `T: Real` (the revolve's axis is geometry) and carries no equality of
any kind, deliberately: deciding whether two coordinates are the same is
a classification band's question, never `==`'s.

The vocabulary's payload-free projection is `VerbKind`, and it names
NINE verbs (`VerbKind::ALL`): the boolean's union, intersect and
subtract are three kernel operations sharing one payload shape, and
every commitment keyed on the vocabulary keeps them apart, so each is
its own name. `VerbKind::arity` declares which door answers a verb;
`Verb::kind` is the one place the projection is written, exhaustive.

Each verb owns:

- **its run door.** `Verb::run` (one body in, `VerbOut`), `run_pair`
  (two bodies in, `PairOut` — the typed empty is part of the boolean's
  contract, F8), `run_profile` (one borrowed `ValidatedProfile`, the
  door's whole bundle back as the record), `run_split` (`SplitOut`, two
  sides each a body or the typed empty) and `run_shell`. Every check,
  refusal and minted entity is the op door's; these dispatch and re-wrap.
- **its birth-record shape.** `VerbOut` carries the operation's own
  record in `VerbRecord`, one variant per record family, the kernel's
  own types moved across by value and never restated. A verb without a
  birth channel cannot join the enum: the record is what lets the
  document layer name what the operation created.
- **its parameter→field flow** (`verbs/src/flow.rs`), as data — §3's
  channel, and the reason the declaration sits beside the op: only the
  op knows where its parameters end up.

**`Arity` is the door vocabulary, under a historical name.** Its five
rows — `One`, `Two`, `Profile`, `Split`, `Shell` — were operand counts
once; the type's own docs record that they now have three axes (the
operand, the out-type, and the bound the door can run at) and that no
row is a claim about any one alone. `One` and `Split` take the same
operand and are two doors because their out-types differ; `One` and
`Shell` agree at both ends and are two doors because the shell's op door
demands certification rights (`Decide + PropsQuadLane + CertifiedBounds`)
no `Dual` scalar has — which is why `run_shell` lives in a second `impl`
block, so the mixed pass instantiated at `Dual` still compiles against
`run`. The name is kept for continuity — it crosses the document layer's
refusal payload (`NodeErrorKind::VerbArity`, re-exported through
`pncad`) — at a rename price measured at approximately nothing, so
keeping it is a decision someone may revisit, not a defect.
`VerbError::Arity` speaks the rows in the doors' own names;
`tests/run_door.rs` asserts `Arity::ALL` and the door matrix name one
set.

**V2 — commitments are exhaustive matches to stable tags, held by their
owners.** Every commitment a verb has is an exhaustive match over the
one canonical vocabulary, living in the crate that owns it and looking
at the canonical name: the content-key tag beside the memo machinery
(`editor-core`'s `verb_content_tag`), the wire spelling on `Node`'s
serde derives, the Python constructor, the viewer's tree label. The
kernel says nothing about any of them. The tags are the numbers that
were already there — 17 fillet, 24 chamfer, 5 extrude, 6 revolve, 8/9/10
the boolean's three ops, 7 split — pinned digit for digit by
`verb_content_tags_are_the_committed_numbers`, so a kernel-side rename
is a compile-guided visit and not a re-spelling of saved files. **A verb
in the vocabulary need not be one the document can author, and the
commitment states that rather than skipping it:** `verb_content_tag` is
an `Option<u8>` answering `None` for the shell, which has no `Node`, so
the censuses stay total over `VerbKind::ALL` while measuring only the
rows really in the tag space. `document_verb_tag` is the narrow door for
the verbs a `Node` builds and is loud on the kernel-only answer: a
silently wrong content tag is how a memo serves another node's
geometry.

**V3 — `editor-core` keeps the authoring vocabulary and states the
correspondence once.** A verb's `Node` payload remains typed document
data — `Expr` per slot, a frozen canonical `Vec<StableName>` selection,
`RecipeNodeId` inputs — because that IS the document's semantics. What
is declared once per verb, in `editor-core/src/verbs/`, is the
CORRESPONDENCE: which `SlotId` feeds which verb parameter, which payload
selection feeds the key list, which emitter mints the names, which arm
of the record channel the result arrives in. Four exist —
`BlendVerb`, `PairVerb`, `ProfileVerb`, `SplitVerb` — each per-instance
data (function pointers and literals, no match over a verb vocabulary
anywhere in those files), and the generic lowerings in `eval/wire.rs`
run off them: `wire_blend`, `wire_boolean`, `wire_swept`, `wire_split`.
Document semantics stay upstairs where they are authored — the boolean's
`resolve_declarations` over the operands' name tables, the revolve's
axis frame check and angle classification, the split's not-a-plane
refusal. `read_record` is the one rule for taking a family's record out
of the closed channel: the correspondence's own exhaustive projection,
a foreign family refused typed.

**V4 — migration is per-verb and additive, and complete for every verb
the document authors.** The seat carries the blend pair, the two sweeps,
the boolean's three ops, the split and the shell; every other door —
transform, pattern, loft, sweep along a path, measure — still runs as it
did, reached by `editor-core`'s lowering calling its op crate directly.
`crates/verbs/src/lib.rs` says so in its own header, since reading the
enum as "every operation a recipe door can invoke" would misjudge the
next unit's cost. No wire format moves in a migration: each unit pins
its spelling and digests first.

**The counterarguments, still recorded.** The dispatch-arm cost this
replaces is compiler-guided and was priced and accepted; what changed is
that #1372 makes parameter flow per-op knowledge with no other home, so
the declaration is needed for correctness, not only for cost. And a
shared declaration couples kernel refactors to schema-visible events:
V2's tags reduce that to a compile-guided visit, but post-publish it is
a discipline, accepted deliberately with drift as the alternative.

## 3. Lowered parameter identity

**P1 — the channel is lowered *expression* identity, per stored field,
in opt-in side records.** A `ParamSource` (`topo/src/param_source.rs`)
sits beside the geometry arenas for the stored scalar fields of minted
descriptions, keyed per kind like the `surface_sources` table. To the
kernel it is a fully opaque token: `Eq`/`Ord`/`Hash` and nothing else —
no readable payload, no constructor that builds one out of another, no
arithmetic, and a `Debug` printing the length alone, so a body dump is
not a door out of the payload. It carries deliberately LESS structure
than `GeomSource`: `SourceExpr::Placed` exists in the kernel only
because rigid placement re-parameterizes a *description*, while a stored
scalar field is motion-invariant, so no kernel op composes or interprets
one and no second spelling of expression structure enters the kernel.
Identity is token equality, zero numerics, equality by provenance: both
walls offset by the same declared `t` lower to the same `r ± t` token
and stay equal by syntax, while `r` and `r ± t` differ. The scope caveat
of `topo/src/source.rs` applies verbatim — identity holds per evaluation
against the current document, never across unaudited mutations.

The single spelling of expression identity lives in
`editor-core/src/param_source.rs`: `lower` produces the token, `invert`
reads one back to a slot address for diagnosis, and the encoding is
bit-semantic, matching `Expr::bit_eq` (`0.0` and `-0.0` are different
expressions; the display unit a literal was authored in is no part of
identity, D7). The token names the parameter TABLE as well as the
expression — `ParamScope::Root(DocumentId)`, or `ParamScope::Part`'s
`DocRef` with its pin — because two documents that each call their
radius `r` meet inside one evaluation whenever a part is instantiated,
and a host may instantiate one document at two pins.

**P2 — who attaches, who propagates, who consumes.** `editor-core`'s
lowering attaches sources at mint time, driven by the verb's declared
`param_flow` and the slot's expression address, one door per source
kind: `attach_blend` for a verb's own scalar, `attach_swept` for a
scalar the operand profile carries per edge. Kernel ops never mint,
compose or interpret a source: survivors keep their records by key
identity (the maps ride the clone every op starts from), rigid placement
carries them verbatim (`transform_rigid` clears the `GeomSource` records
because it rewrites description bits, and not these, because it cannot
change a radius), and kills drop them. The flow's rows name where a
value comes from: `FlowSource::Param` for a verb's own scalar,
`FlowSource::ProfileEdge(EdgeScalar::Radius)` for the profile circle's
radius that becomes the swept wall's stored radius — the row the
equal-radius germ reads. An empty `fields` list is a statement and not
an omission (the chamfer's setback positions planes and is stored in
none of them; the sweeps' extent and angle reach no stored field; the
shell's thickness becomes `r − t`, the identity of neither `r` nor `t`),
and a verb with no scalar parameter at all — the boolean, the split —
has an empty flow one level up, which keeps the census true. The memo is part of the contract:
`feed_content_key` writes a flow-bearing slot's lowered expression into
the content key beside its value, so a value-preserving expression edit
cannot memo-hit and hand back a body whose field rows name an expression
the document no longer holds. The first production consumer is
`germ_section_frame` (`topo/src/boolean/join.rs`), reading
`field_source_evidence` over the two germ faces' radius fields:
`RadiusEvidence::Declared` exactly when the recipe layer evaluated one
expression into both.

**P3 — absence refuses, permanently.** Where no source exists — imported
geometry, hand-built bodies, kernel-derived fields — the consuming
family answers `None` and routes its general rung permanently. There is
no numeric arm and will not be one: comparing stored radii would be
measurement masquerading as structure. The row is pinned by
`the_same_geometry_without_the_channel_refuses`
(`editor-core/tests/seat6_param_source.rs`).

## 4. The question ledger, answered

- **VS-Q1 — where `Verb` lives.** `Verb<T>` lives in `crates/verbs`, its
  own crate above `sweep` and `topo` and below `editor-core`: the
  vocabulary spans crates — the blend pair and the sweeps are `sweep`'s,
  the boolean and the split `topo`'s — so hosting the enum in either op
  crate would make one name the other's ops. The build-cost ledger
  (GENERICS-BUILD-COST) was the counterargument and a `sweep`-hosted
  module the cheap fallback; the crate earns its manifest.
- **VS-Q2 — `Node` payload shape.** Typed per-verb `Node` variants are
  kept, with the correspondence declared once per verb (V3): persistence
  spelling, typed edits and `deny_unknown_fields` all stand. A uniform
  `Node::Op` slot-map variant is rejected — it trades compile-time
  exhaustiveness at the document layer for runtime arity checking, the
  silent-dispatch shape D3 forbids. The macro deferral never triggered:
  after six migrations the residual arm noise is the routed
  `verb_refused` arm and four record projections, per verb.
- **VS-Q3 — kernel-derived fields carry no source.** Identity ends where
  `editor-core` did not evaluate the expression; the shell's `r − t`
  cavity twins are the latest instance, their flow row present and
  empty. Composite lowered sources minted by the kernel for its own
  arithmetic stay recorded and rejected: expression algebra below the
  line §0 draws, with no consumer for it.
- **VS-Q4 — `ParamSource` representation.** The token is a canonical
  injective ENCODING of the lowered expression — `Arc<[u8]>`, a scope
  prefix, then a tag byte per AST node, operands in child order,
  literals as `f64` bits, parameters by name — minted deterministically
  by `editor-core`, `Eq`-compared by the kernel, inverted upstairs for
  diagnosis. It is not an index into an interning table: an interner's
  ids are facts about a run, and the memo serves bodies minted by an
  earlier run beside siblings re-minted under fresh state, so equal
  indices from two tables would read as a false `Declared`. Being
  injective, it is not the rejected content digest either — token
  equality is expression equality outright. A SourceExpr-style
  structural address in the kernel stays rejected: a second spelling of
  expression structure below the line, with nothing to compose a
  motion-invariant field.
- **VS-Q5 — `RimSide`/`RimSupport` keeps its twin.** The indirection is
  not load-bearing for two variants, and `RimSide` belongs to the BIRTH
  RECORD rather than to a verb's payload, so a collapse is a change to
  the naming seam and not to the verb vocabulary. The disposition is
  recorded where the twin lives (`sweep/src/blend/naming.rs`), naming
  `verbs::Verb` as the canonical owner a collapse would target, the
  persisted spelling becoming a stable-tag match over it.
- **VS-Q6 — sequencing.** Followed: §1 first in its own units, §2 next
  with the blend pair, then the remaining verbs one unit each, §3 riding
  the verb whose consumer needed it.

## 5. Out of scope, recorded

- **#1345 items (2)/(3) — call-minted anchors and the document as a log
  of invocations.** Deferred as one future conversation, not two: a
  name's value is persistence across re-evaluation, so anchors minted by
  seatless calls have meaning only where a replayable call log exists,
  and the trigger is a real consumer. Nothing here forecloses it — the
  birth-record channel V1 requires per verb is the substrate that
  proposal would widen, and `Verb<T>` is what such a log would store.
- **The #917 `OpGroup::Fillet` rename** — its own issue, its own scale.
- **Post-publish schema discipline for V2's stable tags** — carried in
  DESIGN.md's "Before publishing" list.

## 6. What was accepted

- **Costing.** Every recipe door migrated after §2 is costed against the
  chamfer baseline — kernel arm, one `editor-core` module, tag rows,
  bump, mirrors — with `node.rs`'s projection matches untouched. The
  baseline is amended once, at the shell: a door's impl bound is part of
  its signature.
- **The germ, end to end.** One declared radius parameter reaches
  `cylinder_cylinder_section`'s closed form from a document; the same
  geometry without the channel refuses typed (P3), the literal twin
  answering `None`.
- **The demo frictions.** The spacer's kernel-seat frictions and the
  diechamfer and klein findings retire at their sites (demo doctrine:
  workarounds deleted where re-authored); twopeg's and the lily's hand
  declarations assemble through detector plus declare.
- **N4 is untouched.** Name tables remain a function of (recipe
  structure, structural params, verdicts), no naming machinery sits
  below the G1 line, and every migration pins its wire format and
  digests.
