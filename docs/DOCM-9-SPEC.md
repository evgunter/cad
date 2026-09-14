# DOCM-9 — The certified locally-valid range: a query that proves instead of samples (spec)

**Program:** DOCM (`work/docm/plan.md`), unit `DOCM-9`
(`work/docm/DOCM-9.md`). **Ruling of record:** Ev, in chat 2026-09-13,
on `work/docm/certify-locally-valid-range-instead-of-sampling.md`
(the plan's open question 3): the sampling probe stays the
interactive answer; the certified range is an ON-DEMAND query whose
result arrives later and replaces the probe's. The finding is read in
full first — it names the three doors and what each is for.
**Track:** kernel change — the standard v6 unit; the last on DOCM's
slate. **Pre-draw fields, logged before the draw:** difficulty **M**,
task-class **STRUCTURAL**.

- **M** — one new module composing an existing driver; a query-side
  document rewrite through two existing edits; a verdict contract
  with four arms and its rows; no new node, no new persisted
  vocabulary, no change to the driver.
- **STRUCTURAL** — every numeric decision is the interval driver's
  (`drive.rs`, PROPS's lane, consumed read-only); this unit decides
  which leaves are adjacent and what each refusal class MEANS for a
  range.

## What exists, and what is missing

`viewer::bounds` answers "how far can this field move before
something new fails" by sampling and bisection, and says so
(`crates/viewer/src/bounds.rs`, the module doc's three limits).
The kernel can prove the stronger statement: `drive`
(`crates/editor-core/src/drive.rs`) subdivides a parameter box over
`evaluate::<Interval>`, CERTIFIES a leaf on exact verdict-vector
equality with the f64 witness, and REFUSES a leaf with a typed reason
— `FlipCrossing` (a decision differs inside the leaf, with the flip
evidence), `Budget` (depth, leaves or resolution exhausted),
`SliverTerminal`, `Bifurcation`, `Infeasible`, `MeasureRefused`.
Two of the finding's three doors exist: a document parameter is
boxed through `EvalOptions::param_box`, and the driver's refusal
classes are the indeterminate-versus-failure distinction the finding
asked for, read at the leaf. The third door — widening a NODE SLOT,
whose value is a bit-pinned `f64` literal with no name — does not.

## What the unit builds

**1. The field and its widening are the query's, never the
document's** (`crates/editor-core/src/range.rs`, new). A
`RangeField` is either a document parameter (`ParamName`) or a slot
(`RecipeNodeId`, `SlotId`). For a slot, the query derives a document
of its own: a clone of the input with ONE synthetic continuous
parameter whose nominal is the slot's literal value bit for bit
(`DocEdit::SetDocParam`) and the slot rewritten to name it
(`DocEdit::SetParam` with `Expr::param`), then boxed over the asked
interval as that parameter's bounded support (`Distribution::Band`,
whose analyzed axis IS the box — `analysis.rs` documents the bounded
forms so). The input document is never edited, persisted or keyed:
the derivation is a pure function of (document, field, interval)
whose result lives only for the query. A slot that is not a
continuous scalar (a `Count`, a structural slot, a slot already
driven by an expression the rewrite would shadow) refuses typed at
the door, naming the slot. A parameter field boxes directly with no
rewrite. `drive.rs` and `analysis.rs` are consumed through their
public API and not edited.

**2. The query** (`range.rs`): `certified_range(doc, field,
interval, config, tol) -> Result<CertifiedRange, RangeRefusal>`.
`interval` is the seed — the box to certify over, chosen by the
caller (the sampling probe's bracket is the natural seed; the query
does not choose it). `config` is the driver's `DriveConfig` (budget
is the caller's, since the query is on demand). The driver's
`ParamBoxVerdict` is read as a set of one-axis leaves; the query
sorts them and walks OUTWARD from the nominal on each side.

**3. The verdict contract**, one per side (`Side { lo, hi }` of a
`CertifiedRange`), four arms and no other:
- `Certified { to }` — every leaf from the nominal to `to` certified
  and `to` is the seed's edge: NOTHING in `[nominal, to]` changes any
  decision the witness made. The seed's edge is where the proof
  stops, never "unbounded".
- `NewFailure { certified_to, within, evidence }` — the first
  uncertified leaf outward is a `FlipCrossing` whose evidence shows a
  node's STANDING change (a node that was `Ok` at the witness is not
  in the leaf, or the reverse): the boundary the probe was looking
  for is inside `within`, with `[nominal, certified_to]` proven.
- `DecisionFlip { certified_to, within, evidence }` — the first
  uncertified leaf is a `FlipCrossing` with NO standing change: a
  recorded predicate decides differently inside `within` while every
  node still builds. This is the boundary of the CERTIFICATE, not
  necessarily of validity — the probe would call such values valid;
  the query says only that it cannot prove them and why. Never
  folded into `NewFailure`.
- `Indeterminate { certified_to, within, reason }` — the first
  uncertified leaf is `Budget`, `SliverTerminal`, `Bifurcation`,
  `Infeasible` or `MeasureRefused`: the driver could not decide
  `within`, which is NOT a failure and NOT a boundary; the recourse
  is the reason's (more budget, a coarser seed). Never reported as a
  bound.
A leaf's class decides its arm; the query invents no decision of its
own. The relation to the probe is stated at the type: a certified
range is a SUBSET of every locally-valid range (it proves more than
"nothing new fails"), and the two answer different questions, so a
consumer shows both or names which.

**4. Purity and identity.** The query's witness (the derived
document at f64) equals the input document's own f64 evaluation
node for node in every `RunStatus` and every published name (the
derived parameter enters the environment through the same
`from_f64` the literal did, so the build is bit-identical; the
content keys differ because the slot's expression differs — state
that, and that the memo is not shared). The input document's memo,
identity and edit log are untouched by a query.

**5. Doors for the consumers, filed not built.** The viewer's
on-demand action (a "certify" affordance on the bounds panel that
runs the query off the interaction path, seeded by the probe's
bracket, and replaces the reading when it returns) is CHROME's; the
Python door is LIB's. File both with the query's signature; build
neither.

## Acceptance

- **A1 — the proof agrees with the probe where the probe is right.**
  On a monotone field (an extrude height that meets a floor):
  `NewFailure` with `within` inside the probe's bracket on the failing
  side, `Certified { to }` at the seed's edge on the other; the
  certified interval is a subset of the probe's answer.
- **A2 — the proof is stronger than the probe.** A field whose
  variation flips a recorded predicate without failing any node:
  `DecisionFlip` where the probe reports valid; the evidence names the
  predicate (a row).
- **A3 — indeterminate is not a bound.** With a budget too small to
  reach the boundary: `Indeterminate { reason: Budget }` with
  `certified_to` strictly inside; raising the budget on the same
  document turns it into A1's answer (one row, two configs).
- **A4 — non-monotone validity is reported honestly.** A field with
  an island of validity beyond a failure: the query reports the first
  boundary and stops (its type says so); a seed placed on the island
  certifies the island. Two rows.
- **A5 — the query is pure and its witness is the document's.** The
  input document is unchanged (bit-identical serialization before and
  after); the derived witness matches the document's own f64
  evaluation in every node's standing and every published name.
- **A6 — a parameter field boxes directly** (no rewrite; the same
  four arms), and a non-continuous slot refuses typed at the door.
- **A7 — nothing else moved.** `drive.rs`, `analysis.rs`, the viewer
  and the persisted format are untouched; every existing row passes.

## Constraints, binding

- `docs/prompts/implementer-discipline.md` in full, by path. Hosted
  CI is the verification of record; poll it in the foreground. The
  interval lane IS implicated (the query runs `evaluate::<Interval>`):
  the full matrix runs on every PR; say in the PR that the interval
  rows gated.
- **Blinding: NO `Co-Authored-By` trailer in lane commits.**
- Merge-only; private `CARGO_TARGET_DIR` and scratch directory outside
  the worktree; `git status` before every `git add`; never `git add -A`.
- Comments state the invariant, not the history.
- Fence: `crates/editor-core/src/range.rs` (new) and its `lib.rs`
  export line, `crates/editor-core/tests/docm9_range.rs` (new), the
  two `work/` files item 5 names. NOTHING in `drive.rs`,
  `analysis.rs`, `distribution.rs`, `measure.rs`, `eval/*`, `doc.rs`,
  `edit.rs`, the viewer, `pncad*`, or the persisted format: the query
  is a consumer of doors that exist. If a door is missing, the stop
  clause fires; it is not built here.
- **Stop clause.** If the slot rewrite cannot make the f64 witness
  bit-identical to the document's own evaluation (a slot whose
  literal takes a path `Expr::param` does not), if `Distribution::Band`
  cannot express the asked interval exactly as the analyzed axis, or
  if the leaves of a one-axis drive are not a partition of the seed
  the query can walk (a gap, an overlap), STOP: write what you
  measured in the PR as a draft and end your turn.

## Out of scope

Several fields at once (the driver's box is n-dimensional; the query's
contract is one field, the probe's shape); replacing the probe;
choosing the seed; the viewer and Python doors (filed); any change to
what the driver certifies or refuses.

## Review

v6 dual on the frozen head, claims to falsify:

- **C1** The four arms are exhaustive over the driver's refusal
  classes and reached (A1–A4) on documents the implementer did not
  choose; `Indeterminate` is never rendered as a bound and
  `DecisionFlip` is never folded into `NewFailure` (mutate the
  classification and show the rows red).
- **C2** The query is pure (A5): the input document's serialization,
  memo and identity are untouched; the derived witness equals the
  document's f64 evaluation in standing and names (grep the diff for
  any write through the input).
- **C3** The slot rewrite is exact: the synthetic parameter's nominal
  is the literal's bits; a non-continuous or expression-driven slot
  refuses typed (build one of each).
- **C4** The certified interval is a subset of the probe's answer on
  every fixture where both run (A1), and strictly smaller on A2.
- **C5** The fence held: no hunk in `drive.rs`, `analysis.rs`, the
  viewer, `pncad*`, `eval/*`, `doc.rs`, `edit.rs`; the two consumer
  doors are filed with the query's signature and nothing built.
