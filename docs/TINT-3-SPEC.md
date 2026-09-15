# TINT-3 — fourteen byte-identical aggregation guards

**Unit of S-TINT.** Row: `work/tint/D382.md`. Branch:
`tint/3-aggregation-guard`. Read
`docs/prompts/implementer-discipline.md` in full first; it binds you
alongside this spec. Deleted at merge (`docs/DOC-LEDGER.md`); the item
file is the record that survives.

**Base: branch from `origin/tint/2-stand-down-channel`, not from
`main`.** TINT-2 is merged in all but name and its `loud_skip_marker!`
is the worked precedent for this unit's mechanism; branching from it
avoids a conflict and gives you the pattern to copy.

## The defect, re-derived 2026-09-15

`grep -rn 'fn every_suite_file_is_aggregated' --include='*.rs'` returns
**fourteen**, one per crate, each in that crate's `tests/all.rs`:
`bvh`, `editor-core`, `geom`, `geom-brep`, `geom-core`, `mesh`,
`profile`, `step-export`, `step-import`, `stl`, `sweep`, `topo`,
`verbs`, `viewer`. (The row's title and body say thirteen, four times; a
copy was added since it was written.)

**All fourteen bodies hash identical**, from `fn` line to closing brace,
doc comment included. Each is five lines:

```rust
fn every_suite_file_is_aggregated() {
    let tests = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("tests");
    let violations = test_utils::source::aggregation_violations(&tests, include_str!("all.rs"));
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}
```

`D61` already homed the walk, the three checks and the argument for each
in `test_utils::source::aggregation_violations`. **What is left is a
call-site count and nothing else.**

## The mechanism, and why it must be a macro

`env!("CARGO_MANIFEST_DIR")` and `include_str!("all.rs")` **have to
expand at the call site**. Moved into a function in `test-utils` they
resolve against `test-utils` itself and the guard silently checks the
wrong crate — which would be this program's own defect, a check that
passes because it cannot see its subject. So the collapse is a
`#[macro_export]` macro whose invocation sits in each `tests/all.rs`.

**This shape is executed, not hypothesised.** TINT-2 landed
`test_utils::loud_skip_marker!` on exactly this reasoning — a no-op-ish
macro in `test-utils` carrying one spelling to nine call sites, with
`file!()` resolving at the invocation site. Read it before you write
anything. `crates/test-utils/src/lib.rs`'s `gated_to!` is the older
example of the same idiom.

## THE MEASUREMENT YOU TAKE FIRST, BEFORE WRITING THE FIX

**This spec's mechanism may not work, and you are to find out before
building on it, not after.** The question:

> Does `include_str!("all.rs")` inside a macro body resolve relative to
> the **invoking** file, or to the file where the macro is **defined**?

If it resolves at the definition site, every crate's guard would read
`test-utils`' own `all.rs` and the unit's whole shape is wrong. Same
question for `env!("CARGO_MANIFEST_DIR")`, which is an environment
lookup at expansion time and is the likelier of the two to surprise you.

Build a two-crate scratch probe outside the worktree and answer both.
**Report the answer in the PR body with the output.** If either resolves
at the definition site, STOP and report — do not force the shape, and do
not paper over it by passing the path in as an argument unless you can
say why that is not just the hand-written thing again.

(This instruction exists because both previous S-TINT specs named a
mechanism the orchestrator had not executed, and both were wrong — see
`work/tint/process-observations.md`, observation 2.)

## What the unit owes

- One macro, fourteen invocations, the fourteen bodies gone.
- **State what it does not enforce**, at the macro and in the PR body.
  At minimum: nothing checks that a crate HAS the guard at all, so a
  fifteenth crate added tomorrow with no invocation is silent — the same
  hole the fourteen copies have today, neither closed nor widened by
  this unit. Say so rather than implying the macro fixed it.
- **The row's rider claim is now false as written** and must be
  corrected rather than repeated: *"`test-utils/tests/` is the only
  member with no `all.rs` and no `autotests = false`"*. `crates/pncad-py/`
  is a second. It survives narrowed to members with **Rust** tests.
  Also: `crates/pncad/` has `all.rs` + `autotests = false` and no guard
  copy, deliberately, documented in its own header and kept by
  `bvh/tests/aggregator_headers.rs`'s
  `a_non_aggregating_tests_directory_holds_one_suite_file` — that is not
  a hole, and the unit must not "fix" it.

## Prove it bites

The aggregation invariant is real and gated
(`scripts/gates/test-aggregation.sh`). Plant a suite file that `all.rs`
does not aggregate, in one crate, show the macro-built guard goes red
there, restore it. Put the output in the PR body. A guard that moved
homes without being re-proved has not been verified.

## Fences

- In: `crates/*/tests/**`, `crates/test-utils/**`.
- Out: every `crates/*/src/**` outside `test-utils`; `scripts/**`
  (S-TCOST's and Track K's); `.github/workflows/**` (CIW's). The
  aggregation gate script is **not** yours to change — if the macro
  changes what the gate should look for, say so and file it.
- A retired or added suite file leaves `tests/all.rs` in the same
  commit; that rule is unchanged by this unit.

## Review

One style review by path against `docs/prompts/reviewer-style-lane.md`,
no A/B row. Its first question is this program's standing one: **does
the fix mint a fresh instance of what it closes?** Two units running
have failed that, so expect it to be asked hard — in particular, whether
the macro's arguments (if it takes any) are a hand-written list wearing
a new name.
