# python-cannot-set-options-structs — the four options doors (spec)

**Unit:** `work/port/python-cannot-set-options-structs.md`. **Program:**
PORT. **Branch:** `port/pyopts-four-doors`. **Deleted at merge** with a
`docs/DOC-LEDGER.md` row in this PR — not in a later one, and not
without the row.

Read `docs/prompts/implementer-discipline.md` in full first.

## Territory

`crates/pncad-py/*` is **LIB's**; `crates/stl/*` and
`crates/step-import/*` are **EXCH's** if you need to read them (you
should not need to edit them). PORT claims no paths and announces. Name
both programs in the PR body. LIB may take this row instead.

## The premises, checked 2026-09-15 — verify them yourself anyway

The row was written 2026-09-01. I re-read it against the tree today and
all of it holds, but **line numbers have moved and one citation in the
row is stale by position**: the tooth the row cites at
`py/value.rs:999` is now at `:2025`. Cite by name (§7); a bare
`file.rs:NNN` is not a citation.

- **The tooth is live.** `crates/pncad-py/src/py/value.rs` calls
  `import_step(text, &ImportOptions::default(), tol)`, so a Python
  caller cannot set `eps_in` on STEP import. That is the same asymmetry
  issue 730 named for export, on the import door.
- **The pattern exists and is exactly one struct deep.**
  `crates/pncad-py/src/surface_census.rs` gives `StepOptions` the full
  treatment — a roster with one entry per field, and above it a
  destructure with **no `..`**, which is the anchor. The other four
  structs have almost nothing in `pncad-py` by comparison.
- **The decay machinery the row tells you to reuse has landed.**
  `the_not_bound_roster_decays` is in `surface_census.rs` and
  `test_binding_census.py`'s `test_the_rosters_decay` is its Python
  half. A `NotBound` entry is an assertion, not an excuse — if you
  declare a field deliberately unbound, its decay is already checked.

## The work

Four doors, one established pattern, replicated:

| struct | door |
| --- | --- |
| `step_import::ImportOptions` | the STEP import door in `py/value.rs` — **the tooth** |
| `stl::AsciiOptions` | the STL ascii door |
| `stl::BinaryOptions` | the STL binary door |
| `EvalOptions` | the evaluation door |

The shape is PR 1493's `step_string`, and it is not yours to redesign:
one keyword per field; `None` forwards to the Rust default via
`unwrap_or(defaults.X)` and **never re-spells a constant**; a
`surface_census.rs` destructure anchor with no `..` per struct, plus its
roster.

The STL doors already expose `solid_name`/`header` as kwargs but carry
**no census anchor** tying those kwargs to the structs. So for those two
the visible work is small and the anchor is the whole point — do not
skip it because the door already looks right.

## What actually has to be true at the end

**A field added to any of the four structs must not be silent in
Python.** That is the unit's claim, and it is the one thing to verify
directly rather than by inspection:

Add a probe field to each struct in turn, build, and record what reds.
`E0063` at the door (a struct literal missing a field) and `E0027` at
the census (a destructure missing a field) are what PR 1493 measured.
**Put the measured result in the PR body, one line per struct** — four
probes, four outcomes. If any struct reds at only one of the two sites,
say so and say why; if one reds at neither, the anchor is not doing its
job and that is the finding, not a detail.

Remove the probes before you push, obviously. The measurement is the
evidence, not the diff.

## Scope

**Four doors, and stop.** If a fifth options struct turns up, file it
(§6) rather than growing this unit. If a door needs a kernel-side change
to be settable at all, that is EXCH's or LIB's ground — land the three
that work and file the fourth with what you found.

Do not re-spell a kernel default anywhere. If you find yourself typing a
number that already exists in a `Default` impl, stop: that is the defect
this pattern exists to prevent, and PR 1493's `unwrap_or(defaults.X)` is
the spelling that avoids it.

## Sweep (§5)

The class is "a kernel options struct crossing the FFI with no per-field
exposure and no census anchor". The row names four; **prove that is
all of them**. Grep for the shape — types reaching a `pncad-py` door as
a whole struct, and `::default()` calls inside `py/`. Hit list with a
disposition per hit in the PR body, and state what your pattern could
not match. A fifth instance found and filed is a better outcome than
four fixed and a clean-looking sweep.

## Verification

Hosted CI is the verification of record. Push, mark ready for review,
poll the run's jobs API in the foreground until it concludes, report in
the same turn. A green code-tier run shows twelve `test (…)` and five
`k-lint (gate, …)` jobs. The python suite runs for this diff and is the
row that matters most here. No `CI-Config:` trailer.

## Review

**Style review** (`docs/prompts/reviewer-style-lane.md`), and the plan's
Review-posture section says why this unit does not get the second arm
even though it changes four binding signatures: the census anchor is the
correctness arm, in the compiler. A human one would be reading past a
guard that cannot be fooled. That reasoning is only as good as the
anchor, which is why the four probe measurements above are not optional.
