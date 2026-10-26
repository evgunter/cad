# GUARD — the gate harness: the shared readers and the selftests (plan)

Opened 2026-09-11 in the tracker cut of that date
(`docs/WORK-TRACKS-2026-09.md` addendum 3) on `scripts/gates/*` whole;
re-chartered on 2026-10-26, when GUARD measured 43 points against 30 and
split three ways (`work/guard/log.md`, that date). Live state is
`work/guard/log.md`'s tail and the item files beside this plan, never
this file.

Branch prefix (the #396 convention): **`guard/`** — unit branches
`guard/<unit>-<slug>`. The retired prefix `gates/` names the closed
GATES program's branches and is not reused. Away-channel tag
`(GUARD orchestrator)`. A/B ordinal band **GUARD = 3600–3699**, claimed
in `docs/MODEL-AB-LOG.md` in the opening commit.

## Charter

Every gate under `scripts/gates/` runs on `lib.sh`: one Rust reader,
one test-only skip, one selftest harness. **A defect there is a defect
in every gate at once, and it fails in the quiet direction** — a reader
that cuts a record wrongly or reads nothing, and a gate that prints OK
over it. The rows here are that harness: the reader that splits an
array type at its `;`, the `--skip-cfg-test` cut that drops a
production item sharing a test item's line, the selftest that can
assert a gate fired but not on which identity, the selftests no CI job
runs, the pipeline stages with no guard, and the readers gates copy
instead of share (the embedded Python TOML readers, the three
bound-list walkers).

The neighbours since the split:

- **BLIND** (`work/blind/`) holds the rows where one discipline gate's
  matcher misses a spelling. Its gate scripts are shared ground; what a
  matcher sees is BLIND's, the reader it runs on is GUARD's.
- **UNREAD** (`work/unread/`) holds the rows with no gate at all. Its
  new gates land in this directory on this harness, announced here.

## Territory and the seams

Territory: **`scripts/gates/*` whole**. Stated seams, each in
`program.md`'s `keep_out`:

- **The Rust side of the bound-list reader** — `crates/geom-core/tests/bounds_census.rs`
  and `crates/test-utils/src/source.rs` are S-TINT's and S-TCOST's. The
  row lands its shell half here and announces the Rust half.
- **CI wiring is CIW's.** `gate-selftests-run-in-no-ci-job` adds a step
  to `.github/workflows/*` by announced seam; the embedded-Python row's
  lint half reaches `scripts/check-python-lint.py`, also CIW's.

## The slate

| item | pri | cost | what it is |
| --- | --- | --- | --- |
| `gate-rust-reader-splits-an-array-type-at-its-semicolon` | P3 | D | `gate_rust_code --statements` cuts `[T; N]` at its `;` and tracks no `<>`; `viewer-vocab-declared-once.sh` carries a workaround reader |
| `gate-selftest-cannot-observe-the-identity-a-gate-names` | P3 | D | `gate_selftest_case` matches one substring; a `want` that asserts file:line and identifier is owed |
| `gate-selftests-run-in-no-ci-job` | P3 | E | no CI job runs any gate's `--selftest`; one sat red on main for three weeks |
| `gate-roster-and-probe-census-have-no-reader-guards` | P3 | D | `probe-suite-census.sh`'s process substitutions have no stage guard (the roster half is moot) |
| `embedded-python-readers-are-copied-between-gates-and-unlinted` | P3 | D | the heredoc TOML readers are copied between gates and outside `ruff.toml`'s population |
| `bound-list-readers-have-three-homes` | P3 | D | three bracket-depth bound-list walkers in two languages, none citing the others |
| `skip-cfg-test-drops-a-production-item-sharing-a-test-items-line` | P4 | E | `--skip-cfg-test` drops a production item on a test item's closing line |

## Order

The shared reader first (`gate-rust-reader-…`), because most gates call
`gate_rust_code` and every later row reads better against a reader that
parses items correctly; `skip-cfg-test-…` is the same function and can
ride it. Then the selftest affordance, which is what makes the rest of
the slate — and BLIND's widened matchers — checkable, and
`gate-selftests-run-in-no-ci-job`, which keeps the selftests honest
after. Then the stage guards and the two shared-reader consolidations;
`bound-list-readers-…`'s full fix is a shell-vs-Rust design call, and
its cheap half (each reader cites the others) can land first.

## Review posture

Infra-only, the CIW/S-TCOST posture: one style review per unit against
`docs/prompts/reviewer-style-lane.md`, no A/B row, the band claimed for
bookkeeping. A unit that changes what a gate ACCEPTS (as against what it
reports) takes a second correctness reviewer, because a widened gate that
is wrong is silent — and a change to `lib.sh`'s reader changes what every
gate accepts, so every reader unit takes one.
