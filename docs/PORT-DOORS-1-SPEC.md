# PORT-DOORS-1 — one order and one list at the assembly doors (spec)

**Unit:** `work/port/PORT-DOORS-1.md`, carrying two findings:
`assembly-door-raises-only-the-head-of-each-refusal-list` and
`product-table-answers-a-tie-before-kind-the-operand-the-reverse`.
**Branch:** `port/doors-1-refusal-order`. **Deleted at merge** per
`docs/DOC-LEDGER.md`; the item files are the record that survives.

Read `docs/prompts/implementer-discipline.md` in full first. §3
(baselines are not targets) and §5 (sweeps) carry most of the weight
here.

**Both design calls are already made**, by the PORT orchestrator on
2026-09-15, each with its reasoning in its own row. Read both rows
before you start. You are not asked to re-litigate them — but if you
find the reasoning wrong against the tree, **say so and stop rather
than implementing something you think is wrong**; that is a finding,
and the orchestrator adjudicates it.

## Territory — none of this is PORT's

`crates/editor-core/src/assembly.rs` is **EDIT's**;
the `crates/pncad-py/*` façade half is **LIB's**. PORT claims no paths
and announces. Name both programs in the PR body. Either may take the
unit instead.

**Another lane is live in `crates/pncad-py/` right now** (S415, branch
`port/s415-boundary-residues`). Keep your façade changes to what part B
actually needs, and expect to merge-forward rather than assuming you are
alone in that crate. You will both append to `work/port/log.md`; that
conflict is ordinary and you resolve it by keeping both entries.

---

## Part A — widen the two mint refusals

`crates/editor-core/src/assembly.rs`, `assemble_gathered`.

Two sites take the head of a refusal list and drop the rest:
`carried_unminted.into_iter().next()` raising
`AssemblyError::CarriedMintRefusal`, and `unminted.into_iter().next()`
raising the `MintRefusal` conversion. Both carry a comment saying the
widening is a follow-up.

**Decided: widen.** The row states the argument; the short form is that
`AssemblyError::AtRest` already takes every finding and publishes that
as a guarantee in its doc-comment, so the enum has answered this
question and the two mint arms are the ones out of step.

**What the decision does not settle, and you decide:**

1. **Whether the widened arms reuse the `AtRest` shape** — a `Vec` on
   the arm, findings in gather order — **or take a channel of their
   own.** The code's own comment proposes "one second refusal channel
   on `AssemblyError` … would serve both". Reusing the existing shape
   is the orchestrator's weak preference, on the grounds that one enum
   answering one question one way is the whole point of the change; but
   the two mint lists and the at-rest findings are different payloads
   and you may find they do not fit one arm honestly. Argue for what
   you pick.
2. **Ordering.** `carried_unminted` is raised before `unminted`, and
   that precedence is load-bearing — the doc-comment says why (the
   inner document is the file the author must open). Widening must not
   quietly flatten the two lists into one, or that precedence becomes
   invisible. Keep the two refusals distinguishable.
3. **The façade.** `AssemblyError`'s mint arms travel out through
   `crates/pncad-py/`. A door that raised a scalar refusal and now
   raises a list is a **public error-channel change**, which is the
   reason this row was held for a decision. Decide what Python
   receives, and keep the bindings' never-strings contract
   (`crates/pncad-py/src/errors.rs`): a list of structured refusals
   must not arrive as a formatted message. If the honest façade change
   is larger than this unit, land the kernel half and **file the façade
   half on LIB's slate** (§6) rather than stringifying it to get done.

**Delete the two comments that record the widening as a follow-up.**
They are the pickup this unit is redeeming, and leaving them is a
comment asserting a plan that no longer exists (§4, and Q4 of the
reviewer brief).

**A test that can go red.** A row asserting the first refusal still
arrives proves nothing about the widening. The row that matters builds
a product with **two or more** unminted mates and asserts every one of
them is reported, in gather order. Same for `carried_unminted`. Without
those two rows this part is unverified.

---

## Part B — ask kind before tie in `resolve_face`

`crates/editor-core/src/assembly.rs`, `resolve_face`.

Today: `Entry::Unique` dispatches on the entity kind, `Entry::Tied`
answers `Ambiguous { width }` whatever the kind. `operand_answer`, its
sibling twenty lines below, answers `NotAFace { kind }` for a non-face
entry **unique or tied**, before it asks anything about rootedness.

**Decided: kind-first in `resolve_face` too.** So a tied entry whose
kind is not `Face` answers `NotAFace { kind }`, and only a tie **among
faces** answers `Ambiguous { width }`.

**The kind to report is the NAME's kind.** `operand_answer`'s
doc-comment already establishes this and says why: *"The kind is the
NAME's kind, which the table makes the entry's kind for every candidate
(`NameTable::insert`, `insert_tied`)."* Use the same source it uses
rather than reading a candidate's key, and the two sites stay one rule.
**Verify that claim holds** — it is the premise the whole change rests
on, and if `insert_tied` admits candidates of mixed kind then this part
needs re-thinking and you should stop and say so.

### What moves, and what does not

`crates/editor-core/tests/msolve5_read_below_a_root.rs` pins both
halves of today's divergence, and its doc-comments narrate it as though
it were a rule. Expect to re-take assertions and **rewrite those
doc-comments** — a comment that explains why two doors disagree is
exactly the comment this change is deleting.

The orchestrator's reading, which you verify rather than trust:

- `a_tied_edge_below_a_root_refuses_not_a_face` — its **second**
  assertion (the same tied edge read AT the pattern) currently expects
  `Ambiguous { width }` and should become `NotAFace { kind: Edge }`.
  Its doc-comment says *"The same tie AT the pattern is the product's
  own row and answers `Ambiguous`, as every tie the product holds
  does"* — that sentence is the divergence, stated as a rule, and it
  goes.
- `a_tied_face_below_a_root_refuses_read_below_a_root_and_at_the_root_ambiguous`
  — **unchanged.** A tie among faces is still `Ambiguous` under
  kind-first. The row said "the two control assertions" move; the
  orchestrator's reading is that only one does. Check it; if both move,
  say so and correct the row.
- `crates/editor-core/tests/display_contract.rs` pins the words of both
  refusals. Whether its `Ambiguous` row moves depends on whether its
  fixture is a face tie — **read it, do not assume.**

**These assertions are baselines, not targets** (§3). Do not shape the
change to keep a number; re-take what moved and say in the PR what
moved and why. "The output stayed identical" is not a justification
here and would in fact mean the change did not land.

### Sweep (§5)

Kind-vs-tie precedence is plausibly a class, not two instances. The
shape is a site that dispatches on `Entry::Tied` and produces a
**refusal**. Starting points, none of them verified as instances:
`crates/editor-core/src/resolve/mod.rs` (two tied arms in one match),
`crates/editor-core/src/appearance.rs` (a tied arm), and
`crates/editor-core/src/names/interrogate.rs` (two tied arms). Most
`Entry::Tied` sites merely carry entries and are not this shape — do
not report them as hits.

Put the hit list and its disposition in the PR body, one line per hit:
fixed, or not-this-unit and why. **State what your pattern could not
match.** A hit with no row and no fix gets filed (§6) on the owning
program's slate in this PR.

---

## Verification

Hosted CI is the verification of record. Push, mark ready for review,
then poll the run's jobs API in the foreground until it concludes and
report in the same turn — never end a turn on a pending run. A green
code-tier run shows twelve `test (…)` jobs and five `k-lint (gate, …)`
jobs. The python suite runs if you touch the façade. No `CI-Config:`
trailer — nothing reads one.

`crates/editor-core` is a heavily-tested crate and `demos/tour` and
`demos/wild` are ordinary consumers of the public API that
`--workspace` does not compile. If `AssemblyError` changes shape, check
them: `(cd demos/tour && cargo clippy --all-targets -- -D warnings)`
and the same in `demos/wild`.

## Review

**Style review** (`docs/prompts/reviewer-style-lane.md`), PORT's
default. Note for the reviewer, and for you: this diff is itself a fix
for a structural finding — two doors that spelled one rule twice — so
the standing trap applies. Check whether the fix mints a fresh instance
of the defect it closes: a widened refusal channel that becomes the
second way to say what `AtRest` already says, or a kind-first rule
spelled a third time rather than shared.
