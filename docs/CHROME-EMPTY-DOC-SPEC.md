# CHROME-EMPTY-DOC — the empty-document rule, cited once and compile-checked where it gates

**Row** (read it in full first):
`work/chrome/viewer-states-the-empty-document-rule-in-four-places-and-
the-one-that-gates-cannot-red` (P1, E).

**Branch** `chrome/empty-document-gate`. **Never merge; I merge.**

**Standing discipline**: `docs/prompts/implementer-discipline.md`, in
full, before you start. It binds you alongside this spec.

## The defect, in one sentence

WIRE's PR 2629 gave the empty-document classification one home —
`ProductErrorKind::is_empty_document` in `crates/editor-core/src/
product.rs` — and the viewer keeps arguing the rule anyway, in three
places the citation did not reach; meanwhile the site that actually
DECIDES what the chrome badges is a `matches!`, the one construct here
that cannot go red when an eleventh `ProductError` arm lands.

Verified on `origin/main` before this dispatch: `product_badge` is in
`crates/viewer/src/frame.rs` and its filter still reads

```rust
!(fault.kind().is_empty_document()
    || matches!(fault, ProductError::RootFailed { .. }
                     | ProductError::RootPoisoned { .. }
                     | ProductError::UnknownNode { .. }))
```

and `ProductErrorKind` carries ten arms today.

## What the row asks for, and what it does NOT

**Keep the three-arm policy in the viewer.** The row is explicit and I
reaffirm it: the Features pane already badges those three at the node
with a typed cause, one of them deliberately quiet — that is a
decision about the CHROME, not a classification of the refusal.
Nothing here moves to `editor-core`, and the dependency keeps running
`viewer` → `editor-core`.

**What is wrong is the instrument.** The honest shape the row names is
*one cited rule plus one local policy, both compile-checked* — an
exhaustive `fn` over `ProductErrorKind`, living in the viewer beside
the badge it serves, so an eleventh arm reds the viewer the way it
already reds `editor-core` twice by name.

**Build that. The design call left to you** is where it lives and what
it returns — a `bool`, or a small enum saying *badged here / badged at
the node / not a fault*, which may read better since three different
answers are in play. Weigh it, choose, and write the reasoning in the
PR. If the tree makes my framing wrong, **say so instead of
implementing it**.

Two smaller things at the same site, for the same diff: the expression
reads one value at two levels (`fault.kind()` against
`matches!(fault, …)`), and `!(A || B)` is harder to read than the
`!matches!(…)` it replaced.

## The three restatements

| site | what to do with it |
| --- | --- |
| `frame.rs`, `product_badge`'s doc | *"Nothing here is wrong to report"* sits immediately after a citation of the rule it restates, and duplicates `is_empty_document`'s own *"deleting the last feature"* worked example. Cite, do not restate. |
| `frame.rs`, the in-module test's comment above the four quiet arms | Disclosed by 2629 and deliberately left, because rewriting another program's test comment is past what an announced seam is for. It is not past what THIS unit is for. |
| `crates/viewer/src/pickindex.rs` (~`:1038`, ~`:1079-1085`) | The same classification, the same two worked examples, independently written a third time. `pickindex.rs` reads no `ProductError`, which is why 2629's sweep dismissed it — true of the code and not of the sentence. |

Comment style is `docs/prompts/implementer-discipline.md` §4: state
the invariant, not the history. A comment that has to reconcile two
spellings of one rule is evidence the rule needs one home
(`docs/prompts/reviewer-style-lane.md` Q2) — which is this row.

## What the unit owes in tests

A row that goes red when the gate stops being compile-checked is not
writable (that is the compiler's job now) — but a row that pins the
POLICY is: which arms badge at the frame and which are left to the
node, asserted by name. `frame.rs`'s in-module tests already drive
`product_badge` over the quiet arms; extend rather than duplicate, and
say which mutation you used to prove your rows can go red.

## Explicitly OUT of this unit

`work/chrome/at-rest-badge-reports-an-empty-document-as-a-refusal` is
the BEHAVIOURAL half of the same cluster and lives in
`crates/viewer/src/session.rs`. **AUTHOR's live lane `author/profile-
frame` (AUTH-3) has `session.rs` in its scope right now**, so that row
waits for a later wave. Read it for context — it is worth reading —
and do not touch `session.rs`.

## Scope

**In**: `crates/viewer/src/frame.rs`, `crates/viewer/src/pickindex.rs`,
and `crates/viewer/tests/` if you add a row there.

**Out**: `crates/editor-core` entirely (read-only here), `session.rs`,
everything else.

**A live seam you must handle**: `pickindex.rs` is being changed right
now by `vgeom/p0-fields` (PR 3007, ~160 lines in `PickIndex`'s
edge-pick path). Your edit there is prose in the module/doc comments,
so a conflict is unlikely but possible. Merge `origin/main`
immediately before you open the PR and again whenever main moves, and
say in the PR that you did. Run `python3 scripts/work.py territory
--base main` on your branch and put what it printed in the PR —
`frame.rs` and `pickindex.rs` are claimed by several programs, which
is legitimate; what is owed is awareness.

A finding outside this fence gets a ROW under implementer-discipline
§6, in this same PR — not a fix, and not a line in the PR body.

## Verification

Hosted CI is the verification of record: expect **twelve `test (…)`**
and **five `k-lint (gate, …)`** jobs, confirm the run's head SHA is
your branch head, and read the STEP for any row whose job name does
not name it. Poll the run in the FOREGROUND until it concludes and
report in the same turn — never end a turn with a wait armed.

Locally, as an iteration tool only: `cargo fmt --all --check`,
`cargo clippy -p viewer --all-targets -- -D warnings` and again
`--features app`, `scripts/doc-gate.sh`, `python3 scripts/work.py
lint`.

`CARGO_TARGET_DIR=/home/user/chrome-empty-doc-target`, exported on
EVERY cargo invocation, **outside the worktree**. Scratch files in
`~/.local/share/cad-work/chrome-empty-doc/` — never the session
scratchpad, which is shared between lanes.

## Deliverable

A PR titled `CHROME: the empty-document rule is cited once, and the
gate that reads it reds on a new arm`, body carrying: the design call
on the gate function's shape and its reasoning, what happened to each
of the three restatements, the mutations proving your rows can go red,
the territory output, and any §6 rows you filed. Update the item
file's body with what you measured. **Report to me; do not merge.**
