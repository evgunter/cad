# TINT-4 — a header roster welded to the rows it names

**Unit of S-TINT.** Row:
`work/tint/r2-m10-6-header-roster-omits-the-suites-heaviest-row.md`.
Branch: `tint/4-roster-weld`. Read
`docs/prompts/implementer-discipline.md` in full first. Deleted at merge
(`docs/DOC-LEDGER.md`); the item file survives.

**Base: branch from `origin/tint/3-aggregation-guard` if it exists,
otherwise `origin/tint/2-stand-down-channel`.** Both are green and
awaiting merge; branching from the tip avoids re-resolving the same
`docs/DOC-LEDGER.md` collision.

## Scope — ONE row and ONE adopting file. This is deliberate.

A feasibility probe was run before this spec existed, and it **broke the
class this unit was nearly cut against.** The slate grouped four rows as
wanting one check. They do not:

- `interrogate-ladder-header-claims-every-rung-and-pins-five` — claims
  coverage of an **enum's variants**, which a roster of test names
  cannot see. Routed out; its shape is TINT-1's exhaustive `match`.
- `test-headers-name-fns-that-exist-nowhere` — both its sites are
  **deliberate lineage** naming retired predecessors on purpose. A
  roster would forbid exactly that. **Closed as wrongly filed.**
- TOPO's `review-d18-probes-header-miscounts-its-own-rows` — TOPO's
  ground. The mechanism is verified to work there and has been offered
  as a pointer; adopting it is TOPO's edit, not yours.

**So: the macro, and `crates/editor-core/tests/r2_m10_6_probes_interval.rs`.
Do not generalise.** The probe measured 85 files whose `//!` header
backtick-names at least one of their own rows — those are
illustrations, not rosters, and pulling them in is the over-reach this
scope exists to prevent. Three files carry the completeness claim; the
other two are not yours this unit.

## The mechanism — executed, not proposed

Read the roster off **libtest's own `--list`**, via a `current_exe()`
re-exec with `--list --format=terse`. That is the harness's ground truth
about what the binary contains. **No Rust is parsed**, so this is not an
instance of `work/tint/source-scanning-censuses-are-a-tripwire-on-ordinary-rust`,
which is live on this slate and is the reason a scanner is refused. The
tree already re-execs its own test binary at eighteen sites (e.g.
`crates/editor-core/tests/m4_pr6_eps_diff.rs`,
`crates/geom-core/tests/eps_provenance.rs`), all of which *run* a child
row — strictly more than listing one.

The weld is one token with three consumers:

```rust
roster! {
    the_certifying_filter_changes_a_pre_m10_6_documents_drive: "what it is for",
    a_tolerance_study_end_to_end_through_the_public_doors: "346-660 s; the suite's critical path",
}
```

Each entry is an **ident**, and it feeds: `let _: fn() = $name;` so a
retired or misspelt name is a **compile error**; `stringify!($name)` so
the compared string cannot be mistyped; and the printed roster a human
reads. `module_path!()` supplies the module prefix mechanically — the
`file!()` move from `loud_skip_marker!`. Nothing about the file is
hand-typed twice.

The probe ran, on this toolchain, in a scratch crate: self-`--list` from
inside a `#[test]` (lists `#[ignore]`d and `mod`-nested rows); the same
under the pinned `cargo-nextest 0.9.140` (process-per-test is
irrelevant — the child is a fresh spawn); **F1** add a row and don't
roster it → RED naming the row; **F2** roster a name that exists
nowhere → `error[E0425]`, does not compile; **F3** `#[cfg(feature)]` on
an entry, holds both ways; **F4** the aggregated `tests/all.rs` layout;
**F5** a `#[cfg(test)] mod` in `src/`. Preconditions checked tree-wide:
zero `#[test] fn … -> Result` in `crates/`, no `harness = false`, no
cargo `runner`, no proc-macro crate and no build script.

## THE MEASUREMENT YOU TAKE FIRST

The probe could not run one thing, and it is the one that can kill this:

> **Time the self-`--list` inside `crates/editor-core/tests/all.rs` with
> `--features interval`.**

That is the largest test binary in the tree — every editor-core suite is
`mod`-ed into one `[[test]] name = "all"` target — and it is where the
adopting file lives. The probe measured 3.6 ms on a 10 MB / 42-row
binary; if this binary is slow to list, or if anything in its startup
does work before libtest parses argv, the per-file cost is not 3.6 ms
and the design is worth re-arguing. **Take that measurement, report the
number, and if it is bad, STOP and report rather than shipping it.**

Secondary, cheaper: confirm the prefix filter yields exactly the **seven**
rows of `r2_m10_6_probes_interval` and not its neighbours in that binary
— the probe found a sibling module whose name is a prefix-extension is
the case that catches a naive filter.

(This section exists because both earlier S-TINT specs named a mechanism
the orchestrator had not executed and both were wrong —
`work/tint/process-observations.md`, observation 2.)

## What it will NOT enforce — say this at the site and in the PR

**The mechanism welds NAMES and never PROSE**, and the row this unit
closes has both halves. Its defect 1 is that the roster names
`report_key_is_blind_to_the_dials_that_move_a_report` **in the opposite
sense to what the row asserts**. The name becomes a compile error if it
is retired; the *sentence* describing the key as blind is caught by
nothing and never will be by this design.

Write that down. A lane that ships "the roster cannot go stale" will be
wrong in precisely the way TINT-1's doc comment was wrong, and this
program has now done that twice.

Also unenforced: nothing requires a file to HAVE a roster, so a suite
that grows a header enumeration without adopting the macro is silent.

## Fences

- In: `crates/*/tests/**`, `crates/test-utils/**`.
- Out: every other `crates/*/src/**` (TOPO's `review_d18_probes.rs`
  explicitly), `scripts/**`, `.github/workflows/**`.
- Rows inside a `mod` need `pub` to be named from the roster site —
  minor and real; if that forces a visibility change anywhere outside
  the fence, stop and report.
- The `//!` header stops enumerating and points at the `roster!{}`
  block. That visibly changes the top of the file; it is a design choice
  and the PR should show the before/after rather than bury it.

## Prove it bites

Re-run F1 and F2 against the REAL adopting file, not the scratch crate:
add an eighth row without rostering it (red, naming it); misspell a
roster entry (compile error). Restore both, and put the output in the PR.

## Review

One style review by path, no A/B row. First question, as always here:
does the fix mint a fresh instance of what it closes? Press specifically
on whether the roster's prose column (`: "what it is for"`) is a
hand-written list wearing a new name — it is hand-written, it is
unchecked, and the honest answer may be that it is acceptable because
nothing computes with it. Say which.
