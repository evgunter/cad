---
id: two-verb-seats-do-not-compose
kind: issue
title: The kernel seat and the recipe seat expose the same verbs twice and do not compose - one verb vocabulary decided, call-minted anchors open
status: open
opened: 2026-08-31
github: 1345
refs: [757, 1372, 1388, 1224]
---

## From GitHub issue 1345

opened 2026-08-31, 2 comments.

The kernel and the recipe layer expose the same verbs twice — `chamfer_edges`
beside `Node::Chamfer`, both re-exported through one prelude — and the two
spellings do not compose. This issue records what the split actually costs,
what is already decided about fixing it, and what is still open.

Raised by Ev while curating the demo montage (`Node::Chamfer` vs the
spacer's plain-body `chamfer_edges`): *"it feels a bit odd that we have two
parallel doors like that."*

## They are not parallel — and that is the problem

`sweep::chamfer_edges` is the `sweep` layer; `Node::Chamfer` is
`editor-core`, above it, calling it. The crate graph is clean. What is not
clean is that **the two doors speak different identity vocabularies and
nothing converts between them**:

- `chamfer_edges` takes arena `EdgeKey`s — body-scoped, non-persistent.
  `Node::Chamfer` takes selectors over `StableName`s. A document's selection
  cannot be handed to the kernel verb, and a kernel body's edge set cannot
  be named.
- `BooleanDeclarations` is the sharp case: a **kernel-level** type, public in
  the prelude, whose only ergonomic producer lives in `editor-core`
  (**#757**). `demos/tour/src/twopeg.rs` states it at the site — *"the
  document layer has selection (`GeoSelect`); the kernel-level `Body` does
  not, and a declared contact is a kernel-level object."*

So the lower layer owns types whose only real producer is upstairs. The
dependency graph does not invert; the usability does.

## Why names cannot live at the kernel seat today

This is an identity question, not a layering one, and the ratified record
says so:

- **D5**: every entity carries a **birth record** — the operator plus its
  argument keys — from creation (realized at M1). D5 explicitly stops short
  of the naming problem: *"recording identity at birth is cheap … this does
  not solve the topological naming problem."*
- **D8**: *"recipe node IDs are the substrate for D5 naming."*

A `StableName` is anchored to a *recipe node id*. A body built by direct
kernel calls has provenance but no anchors, because nothing recorded that
the calls happened. That is the entire split.

## The measured cost, today

`docs/GROUP-BOOLEAN-DESIGN.md` priced adding one node under "Costs, eyes
open":

> New arms at every `Node` dispatch site (inputs/slots/run_op/content_key/
> appearance/diff/edit validation/Python constructor), compiler-guided per D3.

Nine arms for one verb, every time, because the recipe layer re-declares a
vocabulary the kernel already has. Plus the demo-side evidence the tour
keeps recording: the spacer's three frictions (no whole-body edge selector
at the plain-body door; the verb takes arena keys so a document's selection
cannot reach it; it wants a `Band` derived from the `Tol` beside it), and
twopeg's nine hand-derived `FacePairDeclaration`s for one contact the author
could name in four words.

## Decided

**(1) One verb vocabulary, defined once at the kernel layer.** A closed
`Verb` enum beside the kernel ops — inputs, params, `run`, content key,
provenance minting — with `editor-core` holding expressions, edits and
persistence *over* it rather than re-declaring it. Adding a verb should cost
one impl, not nine arms. Closed rather than a trait with closures on
purpose: content-keyed memoization and interval replay need the params
reified as data, and D3's no-wildcard-arms discipline wants the exhaustive
match anyway. Realistically this collapses nine dispatch sites to about
three, not zero.

**The principle for the document layer**: a `Doc` is *not a second model of
the same thing*. Its content is persistence, expressions, re-evaluation and
edit semantics — not identity, and not a parallel verb set.

Both of the above are Ev's, on this thread.

## Open — mechanism not settled

**(2) Anchors minted by the call, not by the document.** If every verb
invocation minted an invocation id and role paths — the birth record widened
from "which op, which arguments" to "which *call*, which role" — then names
exist at both seats, selectors are meaningful over a raw `Body`, and the
document stops *adding* identity and merely *keeps* it. Direct kernel use
would get names and discard them; a document persists them.

**(3) The document as a log of invocations** — falls out of (2): a `Doc`
becomes a persisted, replayable, editable sequence of calls.

Neither mechanism is decided. The honest objections, stated so they are not
rediscovered:

- (2) walks straight into the problem D5 deliberately declined, in the
  hottest code in the kernel.
- It commits a B-rep kernel to the parametric layer's world view even for an
  embedder who wants neither. Possible mitigation: mint anchors into an
  **opt-in side table** rather than into the entities — D5's birth record
  widened, not a new mechanism.
- What decides whether (2) is ever *needed* is which door the product
  drives. The viewer sits on `editor-core` (GUI-DESIGN G1), so the document
  seat is the product's seat and the kernel seat serves tests, probes,
  demos and library users. While that holds, the split costs demo friction
  and dispatch arms and nothing else.

## Now, and independent of all of the above

**The cheap fix**: the *geometric* half of the selection vocabulary —
`SurfaceKind`/`CurveKind`/`GeomPred`, `all_edges` — needs no names and no
document. It is a pure function of a `Body`, and it currently lives above
the layer whose types it would serve. Moving that subset down to
`topo`/`sweep` and having `editor-core` consume it would:

- retire the spacer's frictions 1 and 3,
- make **#757**'s missing producer a selection plus a contact class
  (`declare_flush(a, b, band)`),
- collapse twopeg's nine hand-derived declarations,

and it prejudges none of (1)–(3). Worth doing before them.

**Standing rule proposed alongside it**: a convenience the recipe layer
needs over a body is built at the **body** layer and used from above.
Otherwise the upper door keeps growing affordances the lower one lacks, and
the lower seat's users hand-roll them — which is the code the demos keep
finding and filing.

## Status

A design conversation, not yet ratified into `docs/DESIGN.md`. (1) and the
`Doc`-is-not-a-second-model principle are wanted; (2) and (3) need a design
round of their own. Filed so the cheap fix does not become the whole answer.

## Comments

**2026-08-31** — orchestrator:

A code-measured second read of this issue, taken from the desiderata rather than the sketched mechanisms. Conclusion up front: **the cheap fix is right and is even cheaper than stated; the decided kernel-`Verb`-enum mechanism for (1) cannot deliver its own goal as literally written, though the goal is reachable another way; (2)/(3) are one decision, not two, and can be deferred with a clear trigger.**

## Corrections to the measured picture

**The cost accounting is off in both directions.** Measured against the actual `Node::Chamfer` merge (#1224), a fillet-shaped verb touched **~19 sites across 8 files in 4 crates**, not 9 — but three of the nine sites named here (via GROUP-BOOLEAN's list) cost **zero**: `diff.rs` contains no `Node` match at all (it compares via `Node::bit_eq` over `slots()` + `payload_exprs()`), appearance is a `BTreeMap` that never sees a variant, and the chamfer's `edit.rs` delta was comment-only (validation runs off `payload_names()` / `slots()` / `placement_rule_fault()`). The load-bearing lesson: **the sites that are already free are exactly the ones written against Node's structural-traversal doors instead of matching variants.** What remains scattered is (a) ~8 traversal doors in `node.rs` (`inputs`/`slots`/`expr`/`expr_mut`/`payload_names`/`rebind_payload_names`/`name_free_node`/`placement_rule_fault`) — all projections of *one fact*, the payload's shape; (b) real per-verb registrations (run_op wiring, content-key tag, emitter, persist check); (c) surface mirrors (schema bump, Python constructor/`.pyi`/tags, viewer label, prelude).

**The selection engine is already factored the way the cheap fix wants.** `geompred::prepare` (`editor-core/src/names/geompred.rs:571`) resolves all recipe references up front into `Prepared<'a, T>` (resolved `DatumValue`s, evaluated `T`s); after that, `candidate_matches` (`geompred.rs:640`) takes only `&Body`, an arena key, prepared atoms, and a `Band` — its `StableName` argument exists solely for refusal payloads. `SurfaceKind` already lives kernel-side (`geom-brep/src/intersect.rs:89`), and `CurveKind`'s own doc comment blesses moving it down beside `Curve3` as additive. So the cheap fix is mostly a **re-homing of code already structured for it**.

(Also: `GeoSelect` doesn't exist as a type — it's prose shorthand in the demo comments; the real pair is `select`/`select_where`.)

## The cheap fix — recommended seams

- **A query module in `topo`** (the lowest layer with a `Body`): `all_edges`/`all_faces` materializers in deterministic arena order; the three EXACT atoms (`CurveKind`, `SurfaceKind`, `AdjacentKinds`) as pure predicates over `(&Body, key)`; the DECIDED atom in *resolved* form — distance to a passed-in plane/axis/point value, through the same `sel_*` funnel names (`k_stats` is geom-core, so the funnel is already below; no twin site). `CurveKind` moves down; `SurfaceKind` stays and is reused.
- **editor-core keeps everything name-flavored**: structural `Selector`, datum-node resolution in `prepare`, the GS-Q4 tie trilean, refusal payloads. `select_where` becomes a wrapper whose per-entity test *is* the kernel predicate — nearly the current call graph. One implementation, two doors: the `ContactClass` precedent (SELECT-DESIGN §3(e), defined lowest, re-exported upward).
- **#757**: a `topo`-level `find_flush_candidates(&Body, &Body, tol)` returning findings in `FaceKey`s, implemented as the C4 verifier run in candidate-generation mode — the verifier (`oriented_plane_eq`, `topo/src/boolean/plane_eq.rs`) is already in `topo`, so the anti-twin rule is *easier* to honor down there. The existing name-level `find_flush_candidates` becomes the derived wrapper (keys→names through the table). One honest amendment: SELECT-DESIGN §3's "`pair: (StableName, StableName)` — names, never keys (G1)" should be restated as "names at the document door, keys at the body door, one verifier under both," not silently contradicted. This retires twopeg's nine declarations and lily's six via `declare` sugar over key-level findings.
- **The `Band` friction is already ruled upstairs**: `wire_chamfer` calls `band(tol)`, and the tests already use `Band::linear(tol)` — the document layer treats the derivation as canonical. Every tour caller derives it identically, so "does this argument ever carry information?" is a small kernel API conversation (default/drop it, or at minimum promote the one-argument constructor to the prelude).

This retires spacer frictions 1 and 3, diechamfer finding 2, klein finding 8, the bud/teapot by-description scans, and #757 — and prejudges nothing about (1)–(3).

## On (1): the desideratum yes, the kernel-enum mechanism no

Two structural facts cut against "a closed `Verb` enum beside the kernel ops, with editor-core holding persistence over it":

1. **The kernel is serde-free by ratified decision** (M4 PR 6/F3: the kernel crates gain no serde dependency). A `Verb` enum the document persists forces a persisted mirror in editor-core plus a conversion — and the repo has already ruled this twin *deliberate* at exactly this seam: `sweep::blend::naming::RimSide` vs `editor_core::names::RimSupport` (`blend/naming.rs:73-84` explains why they must not merge). A kernel `Verb` enum doesn't eliminate the second declaration; it standardizes it one level down and keeps both.
2. **The vocabularies genuinely differ, and the difference is the document's whole job**: `Expr` vs `T`, `StableName` vs `EdgeKey`, node-id inputs vs borrowed operands. The kernel's verb vocabulary already exists, defined once, as the functions — the duplication is entirely an editor-core scattering problem, so the fix belongs in editor-core.

What actually reaches "one impl per verb": extend the pattern that already made diff/appearance/edit-validation free — per verb, one colocated module (or one payload-shape declaration a small macro projects) from which the ~8 `node.rs` traversal doors derive, plus the irreducible registrations (kernel fn — exists; emitter; content tag; schema bump; Python/viewer/prelude mirrors). That lands at this issue's own "about three real sites" prediction with **no schema migration, no generic-vocabulary machinery, and D3's compile-time exhaustiveness intact**. Worth noting the arms are compiler-forced and several are genuine per-verb decisions (name-free? placement rule?) — D3 working as designed, and RECIPE-DOORS shipped Chamfer and the two Tubes through it routinely. So I'd rank this below the cheap fix and treat it as a refactor conversation with the chamfer merge as its measured baseline.

## On (2)/(3): one decision, deferred, with the trigger named

Two observations make deferral comfortable. First, **the substrate already exists as data**: every naming-capable verb returns a birth-record struct (`BlendNaming`, `BooleanNaming`, `SplitNaming`) keyed in arena keys, and editor-core's emitters are precisely the fold `(node id, record) → names` — so if (2) is ever wanted, its mechanism is mostly "emitters accept a caller-supplied anchor id," and nothing above forecloses it. Second, **the hard part is semantic, not mechanical**: `StableName.node` is a `RecipeNodeId` whose resolution ladder starts at `ladder::live(name, doc)` — a name without a document has no live rung, and a caller with no recipe has no re-evaluation for the name to survive. Names buy persistence across rebuilds; a seatless caller has none. So (2) has value only if (3) exists — **they are one design round**, triggered by a real consumer (a scripting/REPL/replay story for direct callers), not by symmetry. Meanwhile the cheap fix gives raw-body callers the thing every recorded friction actually reaches for: geometric selection over keys.

The standing rule at the end of the issue reads ratifiable as written — it's already the operative precedent (`ContactClass`, and now this).

---
_Generated by [Claude Code](https://claude.ai/code)_

**2026-08-31** — orchestrator:

Follow-up to my comment above, after Ev's pushback in-session. Three corrections/revisions, and the design round is now open as PR #1388 (`docs/VERB-SEAT-DESIGN.md`).

**1. The Band point, restated correctly.** `Tol` is a zero-sized witness (`geom-core/src/tolerance.rs:679`) — it carries the *right to read* the committed global ε/K, not a value — so `Band::linear(tol)` is a function of the commit alone. The sharp version of the finding: the kernel already disagrees with itself. `extrude`, `revolve`, `loft_body` and `tube_along_arc` derive the band internally at operation entry (`extrude.rs:437`, `revolve/mod.rs:680`, `loft.rs:269`, `tube.rs:340`); only the blend pair and `shell` expose a `band` parameter, and every call site in the tree passes exactly the linear derivation. The clean fix is to drop the parameter and align the odd doors with their siblings — not (as I first framed it) merely to publicize the one-argument constructor.

**2. The `RimSide`/`RimSupport` argument, re-grounded.** The serde-mechanics half of my claim was overstated: `editor-core` could persist a kernel enum field via serde `with`-modules without any kernel serde dependency, so "merging would drag serde into the kernel" is not literally forced. What holds is the change-rate fact — persisted spellings and content-key tags are versioned commitments, kernel enums are refactor-free — but the *twin* is not the only structure that honors it, and its exhaustive emitter match already prevents silent drift. Given a preference for **no drift at all**, the better structure is one canonical enum with each commitment an exhaustive match to stable tags: the compiler forces every commitment site to be visited on any variant change, and the tag indirection keeps a rename from re-spelling saved files. That structure is what PR #1388's V2 proposes.

**3. Issue #1372 changes the balance on decided item (1), in its favor — my earlier "defer the Verb enum" position is revised.** Two things I got wrong or under-weighted:

- The kernel-purity objection was defending a line the design does not draw. `GeomSource` (N6) lives in the KERNEL — `topo/src/source.rs`, `SecondaryMap`s on `Body` (`body.rs:180-181`), read by the boolean's coincidence rungs (`plane_eq.rs:166`) — as *lowered pure-data* identity (`u64` node ids, structural expression addresses), opt-in, compare-only, attached by `editor-core`. The actual line: the kernel may hold and compare lowered identity data; it never holds the typed recipe vocabulary or persistence. A kernel `Verb` declaration sits inside that line. (It is also, verbatim, this issue's own "opt-in side table — D5's birth record widened" sketch for item (2): that mechanism ships today.)
- Issue #1372 needs each verb to declare, as data, how its parameters flow into the fields of what it mints — the radius source must be attached when the cylinder is minted, and only the op knows its flow. That is a new per-verb obligation with no home today, and its natural home is exactly the per-verb declaration this issue's decided text lists ("inputs, params, run, content key, **provenance minting**"). "Params reified as data" now has the concrete kernel-seat consumer my earlier comment said was missing.

What survives from the earlier comment unchanged: the cheap fix (first, independent, and cheaper than this issue estimates — the selection engine is already factored kernel-shaped); the corrected cost accounting (~19 sites measured on the chamfer merge, with diff/appearance/edit-validation free *because* they consume traversal doors); and the thin residual duplication that should stay — `editor-core`'s authoring payloads (`Expr` per slot, frozen `StableName` selections) are the document's semantics, not a restatement. Items (2)/(3) stay deferred as one future conversation with the trigger named in the doc's §5.

---
_Generated by [Claude Code](https://claude.ai/code)_

## Home

SEAT: this issue is the origin of `docs/VERB-SEAT-DESIGN.md` (PR #1388), which SEAT's charter executes — one verb vocabulary (§2) and the lowered parameter-identity channel (§3) — and its cheap fix is the kernel query seat (§1) in `crates/topo/src/query.rs`, SEAT territory.
