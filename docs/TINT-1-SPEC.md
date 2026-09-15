# TINT-1 — the `assert_f6` ban lists stop being hand-written mirrors

**Unit of S-TINT.** Row: `work/tint/assert-f6-dump-lists-are-hand-written-mirrors-of-error-enums`.
Branch: `tint/1-assert-f6-dumps`. Read
`docs/prompts/implementer-discipline.md` in full before you start; it
binds you alongside this spec.

This spec is deleted at merge (`docs/DOC-LEDGER.md`); the item file is
the record that survives.

## The defect, verified

`crates/editor-core/tests/display_contract.rs` holds
`fn assert_f6<E: Debug + Display>(err, wants, dumps)`. Its `dumps`
argument is the set of variant identifiers that must NOT appear in the
rendering — the F6 claim being that an error renders as a sentence and
never as its own `Debug` dump, and that the variant identifier is the
dump's fingerprint.

**All seven `dumps` lists are written by hand, and three of the seven
have already fallen behind their enums:**

| enum | declared in | variants | banned | missing |
|---|---|---|---|---|
| `ParseError` | `editor-core/src/parse.rs` | 11 | 10 | **`Dimension`** |
| `SelectRefusal` | `editor-core/src/names/geompred.rs` | 8 | 8 entries, one of which is `"Angle"` (deliberate) | **`Band`** |
| `DeclareError` | `editor-core/src/names/flush.rs` | 3 | 2 | **`Edit`** |
| `InterrogateError` | `editor-core/src/names/interrogate.rs` | 10 | 10 | — |
| `NodePickError` | `editor-core/src/resolve/pick.rs` | 5 | 5 | — |
| `ResolveIndeterminate` | `editor-core/src/resolve/mod.rs` | 3 | 3 | — |
| `ResolveFault` | `editor-core/src/part.rs` | 3 | 3 | — |

Re-derived 2026-09-15 and independently confirmed at the orchestrator's
seat for `ParseError`. Two of the three holes (`SelectRefusal::Band(BandError)`,
`DeclareError::Edit(EditError)`) are payload-carrying wrappers, so
neither the wrapper identifier nor anything about the inner error's
rendering is banned.

## The design decision, and it is made — do not re-open it

The row weighed three shapes: an `ALL` constant per enum, a census row
per enum, or reading the variant identifiers out of the source through
`test_utils::source`. **Take none of them. Use the compiler.**

Write, per enum, an exhaustive `match` from a value to its variant
identifier, with **no wildcard arm**:

```rust
fn parse_error_variant(e: &ParseError) -> &'static str {
    match e {
        ParseError::UnexpectedChar { .. } => "UnexpectedChar",
        // … one arm per variant, no `_ =>` …
        ParseError::Dimension { .. } => "Dimension",
    }
}
```

Derive each `dumps` list from that mapping. **A variant added tomorrow
makes this file fail to COMPILE**, which is a guard that cannot go stale,
needs no source parsing, adds no dependency, and changes nothing in
`src/`.

**Why not the source-read census, explicitly.** This slate carries
`work/tint/source-scanning-censuses-are-a-tripwire-on-ordinary-rust`:
the existing source-scanning censuses hand-parse Rust and fail loud, so
an ordinary-but-unusual signature reds another program's test with a byte
offset for a message. An eighth source scanner would **mint a fresh
instance of a defect this program is holding a row on** — precisely the
trap `docs/prompts/reviewer-style-lane.md` §1 warns about, where a lane
closing a hand-written list adds a hand-written census. The compiler
already performs exhaustiveness checking; asking a hand-rolled parser to
re-derive what rustc knows is the worse instrument.

## The second half, which is what makes the guard bite

Completing the ban lists alone **changes nothing**. `assert_f6` only
inspects renderings of the `cases` each row supplies, and for the three
missing variants there is no case either — so a longer `dumps` list bans
identifiers that are never rendered.

So the unit owes, per enum, **case coverage**: every variant has at least
one `cases` entry that constructs it and renders it. Enforce it with the
same mapping — collect the identifiers the cases cover, compare against
the full set the `match` defines, and assert equality with a message
naming the uncovered variants. That assertion is the one that goes red
when someone adds a variant and no case, and it is the deliverable.

**Write the three missing cases** (`ParseError::Dimension`,
`SelectRefusal::Band`, `DeclareError::Edit`) with `wants` that state what
each rendering must contain, in the style of their neighbours. If any of
the three turns out to render as its own `Debug` dump — which is the
thing the suite exists to catch and has not been able to — **that is a
found bug, not an obstacle**: report it, and see *Fences* below before
touching `src/`.

## `SelectRefusal` is `#[non_exhaustive]` — a stated exception, not a gap

Six of the seven enums are ordinary and matchable from the test crate.
`SelectRefusal` carries `#[non_exhaustive]`, so a `match` in
`editor-core/tests/` requires a wildcard arm and rustc will **not**
enforce exhaustiveness there. A wildcard that panics does not help: it
fires only if a case constructs the new variant, which is the same
vacuity this unit is closing.

**Do this**: give `SelectRefusal` the same shape as the other six, with
the wildcard arm it is forced to have, and a comment at the site stating
plainly that this one enum's coverage is **not** compiler-enforced and
why — the attribute, by name. Do not pretend otherwise and do not weaken
the other six to match it.

**Do NOT fix it by reaching into `src/`.** A `match` inside `editor-core`
would be exhaustive, so the real home for `SelectRefusal`'s mapping is a
unit test beside the enum — but `crates/editor-core/src/names/geompred.rs`
is **WIRE's territory**. Record the residue in the item file as its own
finding (`work/README.md`: disclosing a residue is not scheduling it —
give it its own file at the moment you disclose it), naming WIRE as the
owner and this spec as where it came from. The orchestrator announces it;
you do not edit that file.

## Fences

- In: `crates/editor-core/tests/**`, `crates/test-utils/**`.
- **Out: every `crates/*/src/**`.** If the work turns out to need a `src`
  change — a helper that must be `pub`, or a `Display` arm that really
  does dump — stop and report it. The owning program is told before it
  lands (S-TINT's `keep_out`).
- Out: `scripts/**` (S-TCOST's), `.github/workflows/**` (CIW's).
- **Never weaken an assertion to get green.** If a rendering fails
  `assert_f6` once it is finally exercised, the rendering is the subject.
- This diff lands in `crates/*/tests/`, which S-TCOST double-claims by
  design. Say so in the PR body: the row is justified by a guard that
  cannot fail, never by a second of wall or CPU (`work/tint/plan.md`
  §*The fence with S-TCOST*).

## Sweep obligation

The row's own blind spot is stated and you inherit it: `grep -rn assert_f6`
finds the helper and its callers **only** in `display_contract.rs`, so an
eighth caller elsewhere would be invisible to that pattern. Re-run a
differently-shaped sweep (the `dumps:`/ban-list SHAPE, not the helper
name) and put the hit list and its disposition in the PR body, one line
per hit. Also say what your sweep could not match.

`crates/editor-core/tests/display_contract.rs`'s own header says
*"`HitTestError`'s own contract test lives with the hit-test suite,
beside the behaviour it renders."* Check that one too, and dispose of it
explicitly — it is a caller of this class living somewhere the sweep above
would not look.

## Verification

Hosted CI is the verification of record. A green code-tier run means
green at all six lane/eps points and all five k-lint unifications; if you
cannot see twelve `test (…)` jobs and five `k-lint (gate, …)` jobs,
something narrowed it and you should find out what. Poll the run's jobs
API in the foreground until it concludes and report in the same turn —
never end a turn with a CI wait or a background build outstanding.

Before pushing, run the fast local checks a contributor runs
(`cargo fmt --check`, `cargo clippy -p editor-core --all-targets`,
`cargo nextest run -p editor-core` scoped to this suite) with your own
`CARGO_TARGET_DIR` **outside** the worktree. Prove the guard bites:
**plant a variant** (or a `Display` arm that dumps) locally, show the new
assertion goes red, and restore it. A guard is not a guard until a
mutation says so — put that mutation and its output in the PR body.

## Review

One style review against `docs/prompts/reviewer-style-lane.md` by path,
plus this unit's own claims: that no assertion was weakened, that the
compiler really is the census for the six (a reviewer should try adding a
variant), that the `SelectRefusal` exception is stated rather than
papered over, and that the fix has not minted a fresh hand-written list
anywhere. No A/B row and no A/B protocol.

## State sync

Rides this unit's own PR, last, after the review is delivered: the item
file's re-derivation section gains the outcome, the `work/tint/log.md`
entry is appended, and any residue filed gets its own file in the same
commit.
