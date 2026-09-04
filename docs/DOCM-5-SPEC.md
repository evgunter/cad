# DOCM-5 — The check registry's subject: one gather per landing (spec)

**Program:** DOCM (`work/docm/plan.md`), unit `DOCM-5`
(`work/docm/DOCM-5.md`). **Ruling of record:** the plan's "questions
still open" item 2, ruled: `run_checks` computes the product once and
hands residents a subject; `assemble` takes a pre-gathered product;
the `product.rs` Dual arms are edited by announced seam to M10. The
finding it answers is `work/docm/check-registry-gathers-product-twice.md`
— read it in full; its table of the three gathers per landing and its
"what makes this more than a dedup" section are the premise.
**Track:** kernel change — the standard v6 unit (binding spec, drawn
implementer arm, cross-model dual review, union fix pass,
record-at-merge; §Review below).
**Pre-draw fields, logged before the draw:** difficulty **M**, task-class
**STRUCTURAL**.

- **M** — three public doors change shape behind kept wrappers
  (`run_checks`, `assemble`, `DocSession::land`), one crate boundary
  (viewer → editor-core), a measurement re-taken with a Q6 disposition,
  and every existing row of the registry and the assembly gate must
  hold unchanged. No new node, no new naming.
- **STRUCTURAL** — data movement: what is gathered once instead of
  three times. No numeric decision anywhere; the residents' predicates
  and the at-rest gate are untouched.

## What the unit builds

**1. The subject** (`checks.rs`). A resident no longer derives its own
subject. `run_checks` becomes a wrapper over a new door

```rust
pub fn run_checks_on<P, T>(doc, ev, subject: Subject<'_, T>, cfg, tol) -> Result<ChecksReport, ChecksError>
pub enum Subject<'a, T: Decide> { Product(&'a Product<T>), NoBodyRoots }
```

— the wrapper gathers exactly once through `product_recorded`, maps
`ProductError::NoBodyRoots` to `Subject::NoBodyRoots`, maps every
other gather refusal to `ChecksError::Product` (which is now what its
name says: the registry could not derive its subject — the one
registry-level refusal, no longer a per-resident special case), and
calls the door. Every resident takes the `Subject` (the separation
resident reads `Product`; connectedness reads what it reads today);
under `NoBodyRoots` a resident that needs a body has no subject and
contributes no finding, exactly today's behaviour, stated at the
enum. The five existing `run_checks` callers compile unchanged;
`pncad-py`'s wrapper is untouched.

**2. `assemble` over a gathered product** (`assembly.rs`). A new door
`assemble_gathered<P, T>(doc, product: Product<T>, tol) -> Result<Assembly<T>, AssemblyError>`
holds everything `assemble` does after its gather — the mint-refusal
raise, the A5 gate, the attribution — and `assemble` becomes
`product_recorded` followed by `assemble_gathered`, with no logic of
its own (the reviewer greps for a second copy of any line). The
canonical door is the one that takes the product; the wrapper exists
for the callers that have no product in hand. `AssemblyError::Product`
stays the wrapper's arm. Nothing about A5's verdicts changes.

**3. One gather per landing** (`viewer/src/session.rs`, `land` and
`at_rest_of`). `land` gathers ONCE and feeds the three consumers from
it, in this order: the product fault (`landed_fault` is the gather's
own `Err`, as now), the registry (`run_checks_on` over
`Subject::Product(&product)` — or `NoBodyRoots` — so a gather refusal
leaves `landed_checks` as `None`, "not checked", exactly today's
observable), then the A5 badge (`assemble_gathered` CONSUMES the
product, last, for assembly-shaped documents only; part documents keep
`None`). No `Clone` on `Product` and no `Arc`: the order above is what
makes one gather enough, and the doc at the site says so.

**4. The count, witnessed.** A gather counter that exists only under
`debug_assertions` (`product.rs`, the shape the bit-identity debug-only
machinery already uses; `cfg(debug_assertions)`-gated so release
builds carry nothing), read by the landing rows: ONE increment per
`DocSession::land`, for a part document and for an assembly-shaped
one. If you find a witness that needs no counter, use it and say so.

**5. The measurement, re-taken** (`checks.rs`'s "Cost" doc, ~:695–712,
and `product.rs`'s claim site). With the gather outside the registry
the two terms are separable: measure the registry over the heatsink
fin pattern (`docs/PERF-PLAN.md`'s discipline) with the subject in
hand, and the gather alone, at the same solid/face counts the doc
names; state both numbers where the withdrawn sentence stood; give the
claim its Q6 disposition — a row in the existing rebuild-latency
baseline machinery (`tests/baseline/rebuild-latency.json`, if the
registry's cost fits its shape) or, if not, the written reason at the
site and a scheduled re-measure named in the PR body.

**6. `eval/parts.rs`'s gather** at the instantiation seam is a
different document's product and is NOT this unit's; leave it and say
so.

## Acceptance

- **A1 — one gather.** The counter (item 4) reads exactly 1 after
  `land` on a part document and on an assembly-shaped document, and the
  landing carries all three results (fault `None`, a report, and for
  the assembly a badge) from that one gather.
- **A2 — the wrapper is the door.** For every corpus document,
  `run_checks(doc, ev, cfg, tol)` and `run_checks_on(doc, ev,
  Subject::Product(&product_recorded(..)), cfg, tol)` yield equal
  `ChecksReport`s; the empty document yields `Subject::NoBodyRoots`
  and a clean report through both; a document whose gather refuses
  (a naming collision across roots — `dsc_checks` has the fixture)
  yields `ChecksError::Product` through the wrapper and is never
  offered to the door.
- **A3 — `assemble` has no logic of its own.** For every assembly
  fixture in `asm_r2b_assembly.rs`, `mate1_member_vocab.rs` and the
  mate suites, `assemble(doc, ev, tol)` and
  `assemble_gathered(doc, product_recorded(..)?, tol)` agree exactly
  (Ok bodies `bit_eq`, refusals equal); every existing assembly and
  registry row passes unchanged.
- **A4 — refusal semantics preserved at the landing.** A gather
  refusal lands with `landed_fault = Some`, `landed_checks = None`,
  `landed_at_rest` as today; `NoBodyRoots` lands with no fault, a clean
  report, no badge.
- **A5 — the measurement.** Both terms measured and stated at the
  claim site with a Q6 disposition (item 5); the PR body carries the
  numbers and the method.
- **A6 — the Python mirror is untouched** beyond what an exhaustive
  mirror forces (expect nothing: no new error arm, no new node).

## Constraints, binding

- `docs/prompts/implementer-discipline.md` in full, by path. Hosted
  CI is the verification of record; poll it in the foreground; never
  end a turn with background work active.
- **Blinding: NO `Co-Authored-By` trailer in lane commits.**
- Merge-only: no rebase, no force-push, no squash. Push early and
  often.
- Private `CARGO_TARGET_DIR` and private scratch directory, both
  outside the worktree. Read `git status` before every `git add`;
  never `git add -A`.
- Comments state the invariant, not the history. The PR description
  carries the argument.
- Fence: `crates/editor-core/src/checks.rs`, `assembly.rs`,
  `product.rs` EXCEPT its `Dual` arms (M10's — if a change there is
  forced, STOP and say so; the orchestrator announces the seam),
  `crates/viewer/src/session.rs` (`land`, `at_rest_of` and what they
  call), tests. Nothing in `eval/`, `resolve/`, `crates/topo/*`,
  `crates/pncad-py` beyond forced rows.
- Do not change any resident's predicate, the A5 gate's verdicts, the
  order of findings, or `ChecksReport`'s shape; do not add a `Clone`
  to `Product` or wrap it in an `Arc` — if the order in item 3 cannot
  be made to work, STOP and say why.
- **Stop clause.** If a consumer turns out to need the product AFTER
  `assemble_gathered` consumes it, or a resident cannot be expressed
  over `Subject` without re-gathering, STOP: write what you measured
  (file:line, the shape) in the PR as a draft and end your turn.

## Out of scope

`eval/parts.rs`'s seam gather; the `Bvh` over per-solid hulls the cost
doc mentions (unbuilt, unmeasured as the bottleneck); widening
`assemble`'s single-refusal raise to every `unminted` row (its own
follow-up, noted in `assembly.rs`); the instantiation seam's identity
channel (`instantiation-seam-drops-mate-identity`, the next unit).

## Review

v6 dual on the frozen head, claims to falsify (the reviewers get these
verbatim plus `docs/prompts/reviewer-style-lane.md` by path):

- **C1** One gather per landing (A1) on a document the implementer did
  not choose, part and assembly alike; the counter is debug-only and
  release builds carry none of it.
- **C2** `run_checks` and `run_checks_on` are report-equal on every
  corpus document (A2); `NoBodyRoots` and a real gather refusal go
  where the spec says; no resident re-gathers (grep the residents for
  `product_recorded`).
- **C3** `assemble` has no logic of its own (A3); the A5 verdicts and
  attributions are unchanged on every fixture; nothing clones or
  `Arc`s the product.
- **C4** The landing's observables are unchanged (A4): fault, report,
  badge, in the same cases as before.
- **C5** The measurement (A5) is real: re-run it; the two terms are
  stated where the withdrawn sentence stood; the Q6 disposition is a
  guard or a scheduled register, not a promise.
- **C6** Nothing under `product.rs`'s Dual arms, `eval/`, `resolve/`
  or `crates/topo/*` moved; the Python mirror is untouched (A6).
