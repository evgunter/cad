# `scripts/gates/` — the invariants CI holds that Rust cannot express

Each file here is one gate with one home. `.github/workflows/ci.yml`'s
`discipline` job runs it as a named step and `local-scripts/ci-local.sh`'s
`discipline` row runs the same file; `gate-roster.sh` derives the roster from
this directory and reds if either half runs a different set, so neither the
gate logic nor the gate list is maintained twice. Every gate takes `--root DIR`
and `--selftest`, and both halves run the self-test before the real pass. The
shared plumbing — the Rust reader, the test-only module resolver, the self-test
harness — is `lib.sh`, and what each gate's own matcher can and cannot see is
that gate's header.

## Why these rules are greps, and not lints or types

The question is `S13`'s, and it is about four gates. The disciplines this
design leans on hardest are text-matching CI steps, and this page says which of
them are text-matching because text is the right instrument and which are not.
It is an evaluation, not a ruling: what it recommends is marked as a
recommendation, and ratifying or overruling it is Ev's.

| Gate | The rule |
| --- | --- |
| `bounds-allowlist.sh` | which files may name a compound `Bounds` bound |
| `evalscalar-allowlist.sh` | the same rule over the name `EvalScalar` |
| `no-extra-real-bounds.sh` | no second bound on a type parameter carrying `Real` |
| `interval-square-allowlist.sh` | no `x * x` where the enclosure may straddle zero |

**The premise "greps instead of types" is false for three of the four.** The
type-level encoding is real and does most of the work: `Real` is
comparison-free by construction, `Bounds`/`Decide`/`SpanLocate` are separate
subtraits, the `CertifiedEnclosure`-bounded surface is uninstantiable at a
dual, and the lane splits refuse rather than widen. What the gates enforce is
the **residue** — *which files may NAME a bound* — and Rust cannot express that
across crate boundaries at all: sealing controls who may **implement** a trait,
never who may **name** it. No lint of any shape changes that; a lint could only
enforce the same naming rule with a different reader.

### The four alternatives, against the four gates

**`clippy::disallowed_*` with a `clippy.toml`.** Free — clippy already runs on
every code-tier row, so there is no new toolchain and no new build. It is also
the wrong shape for all four rules: `disallowed_types`, `disallowed_methods`
and `disallowed_macros` each name a *path* that may not appear, and none of the
four rules is "this path may not appear". `Bounds` may be named; it may not be
named *beside another bound*, *outside a ratified file*. `x * x` may be
written; it may not be written *where the enclosure straddles zero*. A
configurable path ban expresses neither condition.

**A proc-macro.** It sees only the token stream of the item it is attached to,
so enforcing anything with it means annotating every generic item in the
kernel. A rule that holds only where someone remembered to opt in is not a
rule, and the omission is invisible — exactly the failure mode the gates exist
to close. Rejected on the mechanism, before any cost.

**A `syn`-based binary under `tools/`.** No toolchain pin, and it builds in the
workspace like `tools/k-lint` and `tools/tess-lint` do. What it buys over the
grep is a **parser instead of a lexer**: bracket depth, `where` clauses,
multi-line generic lists and rustfmt's wrapping become structure rather than
patterns. That is real, and it is also the part `lib.sh`'s statement view
already buys — the reader cuts at `{`, `}` and `;`, which is where a generic
list and its `where` clause end, so the formatter-produced spellings are
already matched — the ruling recorded in `scripts/gates/lib.sh`'s reader block,
under "THREE RECORD SHAPES". What it does **not** buy is name resolution:
`syn` parses one file at a time and resolves nothing, so the alias case below
stays open and `include!`d text and macro bodies stay invisible, because both
are resolved by the compiler and not by a parser. It costs a build in the
`discipline` job, which today runs no cargo at all.

**`dylint`.** The only candidate that closes anything the grep cannot. A
`LateLintPass` runs after name resolution and type checking with the HIR and
the `ty` layer in hand, so it sees:

- **the compound bound reached through an alias.** `profile/src/path/arc_fillet.rs`
  declares `trait ArcCarrierScalar: Decide + Bounds`; the declaration fires and
  is ratified, and every use site writes `T: ArcCarrierScalar` — character for
  character the sole bracket bound the gate pins as must-NOT-fire. No widening
  of a text matcher reaches those uses without redding correct code elsewhere
  (`bounds-allowlist.sh`'s KNOWN GAP 3 states this, and the same case is
  `no-extra-real-bounds.sh`'s first blind spot). A late pass reads the bound
  list after resolution and the alias is just another supertrait.
- **`include!`d text and `macro_rules!` bodies.** Both are ordinary text to
  `lib.sh`'s reader, which is a lexer and says so: a forbidden spelling
  assembled from token fragments is invisible, and one written literally inside
  a body nothing invokes reds a gate. A lint sees the expansion.

Its costs are three, and the third is the one that decides:

1. **A nightly toolchain.** `dylint` drivers link against `rustc_private` and
   are pinned to an exact nightly; the repo pins a stable toolchain in
   `rust-toolchain.toml`. The pin becomes a second toolchain to bump, and a
   driver that stops building on a bump takes the gate down with it.
2. **A build in the `discipline` job.** That job invokes no cargo at all: it
   reads text and matches it. What the job costs, and what it cost before the
   gates moved onto a shared reader, is `work/gates/D109.md`(e) — a reading with
   its own home, not restated here. A dylint row makes it a compile job.
3. **The CI-half parity rule.** Both halves of CI run every gate, and
   `gate-roster.sh` holds that they run the same set by deriving the roster
   from this directory. A lint runs where cargo runs; a shell gate runs
   anywhere. Moving one rule into a lint means one rule is no longer in the
   roster, and the property `gate-roster.sh` exists to hold stops being a
   property of the whole set.

### Which greps are the right tool

**`interval-square-allowlist.sh` is, and not by default.** The rule guards a
**whole-program** property: whether *this* enclosure can straddle zero is a
fact about upstream callers that refactors change silently. No type states it,
and no lint decides it either — a late pass would know the type is an interval
and still not know the sign regime. What the gate holds instead is a
conservative textual ban with a ratified allowlist, and it caught a real bug
class (`memories/interval-square-poison.md`; four live bugs came from this one
class). Its open residue is a **wider matcher**, not a different instrument, and
that residue is now re-derived on every run rather than argued in prose: the
five spellings the matcher structurally cannot see are counted each pass and
checked against a register of dispositioned sites.

**`no-ambient-env.sh` is**, for the same reason in a simpler form: the subject
is the appearance of a text, and the ban has a measured receipt.

**`bounds-allowlist.sh` and `evalscalar-allowlist.sh` are, for the rule they
actually enforce.** That rule is *which files may name a bound*, which is not a
type question and not a resolution question — it is a question about file
paths, and a file path is exactly what a grep reads. `evalscalar-allowlist.sh`
is what the encoded alternative costs: the trait is `pub`, so without that step
any file in any crate acquires a compound `Bounds` bound invisibly to the gate
beside it. Encoding the rule in the type needs MORE grep, not less.

**`no-extra-real-bounds.sh` is**, on the same reading. Its four spellings are
formatting variants of one construct, and a statement view matches all four.

**Where a grep is not the right tool is one case, not four:** the compound bound
reached through a NAME. It is the only place a lint buys a fact the text cannot
supply, it is shared by `bounds-allowlist.sh` and `no-extra-real-bounds.sh`, and
it is disclosed in both.

### The recommendation, and what is a finding rather than a ruling

Everything above is the evaluation `S13` commissioned, and it stands on its own:
what each alternative catches and what it costs are facts about the tools and
this tree. Weighing that one gap against those three costs is a JUDGEMENT, and
this page records it as a recommendation rather than as settled:

> **This page recommends that the four gates stay greps and that the alias gap
> stay registered where it is disclosed, rather than buying a `dylint` row for
> it.** Ratification is Ev's — the [ev] PR that lists this page in
> `docs/DESIGN.md`'s companion table is where that decision is asked for.

Until it is ratified, the recommendation is what a reader should weigh, not a
rule they should apply.

## What a gate proves, and what it does not

`gate-roster.sh` is itself a grep over YAML, so it proves **wiring, not
execution**: a step disabled by an `if:` condition keeps its `run:` line and
satisfies the check while Actions skips it. Closing that needs a workflow
evaluator; the hole is named in that script's header instead.
