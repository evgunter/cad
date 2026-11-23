---
id: ciw
kind: program
title: CIW — hosted CI, workflows and scripts, and what no gate reads
status: ready
opened: 2026-09-03
group: infra/ci
prefix: ciw/
tag: (CIW orchestrator)
ab_band: 1500-1599
paths: [.github/workflows/*, local-scripts/*, .claude/hooks/*, scripts/apt-install.sh, demos/*.sh, demos/*.py, scripts/check-*.py, scripts/criterion-emit.py, scripts/doc-gate.sh, scripts/check_admesh.sh, scripts/check_step.sh, scripts/k_probe_sweep.sh, scripts/tess_budget_cut.sh, scripts/tess_budget_sweep.sh, crates/pncad/src/prelude.rs, crates/topo/src/review_m1_pr5_internal.rs, crates/viewer/src/vocab.rs, scripts/gates/bounds-allowlist.sh, scripts/gates/interval-square-allowlist.sh, scripts/gates/bit-identity-consumer.sh, scripts/gates/reporting-margin-door.sh, scripts/gates/register-equal-allowlist.sh, scripts/gates/no-ambient-env.sh, crates/topo/tests/r1_lane1_bracket_read_census.rs, scripts/gates/*, crates/*/tests/*, crates/test-utils/*]
keep_out: [scripts/gates/* is GUARD's (work/ciw/program.md), tools/* is INSTR's (work/instr/program.md), scripts/ci-filter.py is S-TCOST's, scripts/work.py is the tracker's own and changes only with work/README.md, CI build knobs (profile/cache) stay S-TCOST's rule — measured in-unit or not at all, what a main push re-gates is an [ev] ruling, UNREAD (claims and references no gate reads) merged in on 2026-11-02 and kept its fences - a new gate is a new file under scripts/gates/* which is GUARD's directory - announce it on GUARD's log and build it on GUARD's harness (lib.sh) rather than a private reader, a checker that is an arm of scripts/work.py lint is META's ground - build it there by announced seam and announce it on META's log and the row stays here, crates/*/tests/* and crates/test-utils/* are S-TINT's and S-TCOST's, crates/pncad/src/prelude.rs and crates/topo/src/review_m1_pr5_internal.rs and crates/viewer/src/vocab.rs are claimed only because three rows' guards land there and are shared with whichever programs own those crates - a row here edits them only for the guard it adds, the reference rows read citations in work/ and crates/*/src and docs/doc-ledger/* and the façade censuses in crates/pncad/tests/all.rs and the Python binding census without claiming them, docs/doc-ledger/ notes are written by the program being swept at its exit and this program only adds a check that their recovery commands resolve - it does not rewrite notes beyond the one-line pointer fixes its own rows name, a stale citation found in another program's item file or plan is routed to that program and not fixed across the fence (META's one-file-one-item rule), run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled, BLIND merged in on 2026-11-20 (docs/doc-ledger/blind-guard-tint-and-vacuity-merge-into-ciw.md) - it held no P0 or P1 row, from BLIND: opened 2026-10-26 by GUARD's split - the rows moved by git mv with ids and bodies unchanged and GUARD stays open on scripts/gates/* whole with the gate harness, from BLIND: so every gate script here is shared ground with GUARD - a change to lib.sh's shared reader or selftest harness is GUARD's and is announced there rather than made here, from BLIND: crates/geom-core/tests/bounds_census.rs and crates/test-utils/src/source.rs are S-TINT's and S-TCOST's ground - S210's census half lands there by announced seam, from BLIND: the certified doors the bounds gate would have to see (crates/geom-brep/src/props/*, from BLIND: crates/profile/src, from BLIND: crates/mesh/src, from BLIND: crates/bvh/src) belong to PROPS and others - S210's and S41's reach into them is a row filed on the owner and never a diff from here, from BLIND: crates/profile/src/path/arc_fillet.rs and the ~49 profile sites G4 collapses are edited by announced seam as G4's own body says, from BLIND: .github/workflows/* is CIW's, from BLIND: run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled, GUARD merged in on 2026-11-20 (docs/doc-ledger/blind-guard-tint-and-vacuity-merge-into-ciw.md) - it held no P0 or P1 row, from GUARD: crates/geom-core/tests/bounds_census.rs and crates/test-utils/src/source.rs are S-TINT's and S-TCOST's ground - the bound-list row lands its shell side here and announces the Rust side on the seam, from GUARD: tools/* is INSTR's, from GUARD: .github/workflows/* and local-scripts/* and scripts/check-*.py and ruff.toml's checker are CIW's - wiring the gates' selftests into a job lands by announced seam, from GUARD: BLIND (work/blind/) shares six gate scripts here and owns what their matchers see while this program owns the reader and harness they run on, from GUARD: CIW (work/ciw/, from GUARD: which UNREAD merged into on 2026-11-02) adds the new gates for its no-gate-at-all rows under this directory on this program's harness and announces each one here, TINT merged in on 2026-11-20 (docs/doc-ledger/blind-guard-tint-and-vacuity-merge-into-ciw.md) - it held no P0 or P1 row, from TINT: cost is S-TCOST's question and no row here is justified by a cpu-second or a wall-second — a row that turns out to be a cost lever goes back by git mv, from TINT: scripts/ci-filter.py is S-TCOST's and so is the per-file gate mechanism it carries, from TINT: .github/workflows/* is CIW's, from TINT: scripts/gates/* is GUARD's and tools/* is INSTR's, from TINT: a fix that has to land in crates/*/src (a doctest moved out of tests/, from TINT: a helper that must be pub) is announced to the program that owns that crate before it lands, from TINT: no test is deleted for being slow and no fixed seed and no #[ignore] on a row that gates — S-TCOST's keep-outs bind here unchanged, from TINT: changes to memories/ are Ev's call and a rule this program finds wrong is an [ev] PR and never a lane's edit, VACUITY merged in on 2026-11-20 (docs/doc-ledger/blind-guard-tint-and-vacuity-merge-into-ciw.md) - it held no P0 or P1 row, from VACUITY: opened by TINT's 2026-09-20 priority-seam cut (Ev, from VACUITY: in chat) per work/README.md Track size - TINT measured 207.5 (97.5 once priced) budget points and was cut into tracks meant to run in PARALLEL (Ev, from VACUITY: in chat: for these high priority tracks it is ideal to have several components that can be worked on in parallel), from VACUITY: the siblings are TINT VACUITY FIXTURE HELPER and they share their parent's territory by design - shared ground is legitimate by the README's 2026-09-20 rule and what is owed is awareness while a lane is LIVE, from VACUITY: so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled rather than drawing a fence, from VACUITY: the fixtures these rows share live under crates/*/tests and crates/test-utils which S-TCOST globs - a unit here adds or moves rows there as ordinary tests and says so in the PR, from VACUITY: DUP owns the one-thing-spelled-n-times class on PRODUCTION code and these four own it on the test side]
---

**The workflow half**: `.github/workflows`, the jobs that gate a PR and
the nightly that holds the rest, and what a reader of a red run can
tell from it.

The per-PR gate is sized for latency (`ci.yml`); everything else runs in
`nightly.yml`. `work/ciw/latency-cut.md` carries the selection and its
evidence, and a change that adds work to the per-PR gate argues against
it.

Ev put this band low by name — *"it is low priority to improve tooling
in order to cause ci to fail less / main to be red less often"* — and
most of the slate is that. The rows that are not about redness are about
a PR that is silently UNGATED rather than red (a head with no merge ref
gets no run), and a red inherited from main that is not attributed to
the merge that caused it. The P3 rows came from BLIND and MIRROR when
those tracks left the tracker: instruments that pass while blind, which
is Ev's medium band (tooling that prevents a SILENT bug).

**The guards that do not exist yet** (UNREAD's half, merged in on
2026-11-02 when the budget came to count only P0, P1 and unbanded rows
and both tracks fell under it: `docs/doc-ledger/unread-merges-into-ciw.md`).
Every one of these rows is something the tree states that no gate, test or
CI job reads, so it goes stale without anything going red:

- **claims** — a measured number, a census's membership, a compile-fail
  proof, a table's reasons, a probe-gated pin, a formatting promise;
- **references and surfaces a human wrote** — a `file.rs:NNN` citation
  in `work/`, a test name in a doc comment, a `work/<program>/<id>`
  path in a crate comment that rots the day the row moves, a
  doc-ledger recovery command nobody runs, a public method neither
  façade census can see, authored lines folded into a merge commit.

It is the workflow half's question asked of the whole tree: *"what goes
red if this stops being true?"*, where *"nothing, and here is why"* is a
complete answer for one row. A row in either half that is an UNGATED
claim rather than a red one belongs to the same sitting, and two pairs
of rows are one finding seen from each half (`work/ciw/plan.md`).

Charter and unit order: `work/ciw/plan.md`; narrative in `work/ciw/log.md`.

## From BLIND (merged 2026-11-20)

BLIND — the kernel-discipline gates' matchers, and the spellings they miss

The kernel's discipline gates — `bounds-allowlist.sh`,
`interval-square-allowlist.sh`, `bit-identity-consumer.sh`,
`reporting-margin-door.sh`, `register-equal-allowlist.sh`,
`no-ambient-env.sh` and the bracket-read census — each police one rule
from `docs/DESIGN.md` or a ratified README clause, and every row here is
a spelling of the policed thing that the matcher misses (or, twice, an
innocent spelling it refuses). Cut out of GUARD on 2026-10-26 when GUARD
measured 43 points against 30. Charter and order: `work/ciw/plan.md`;
narrative in `docs/doc-ledger/blind-guard-tint-and-vacuity-merge-into-ciw.md`.

## From GUARD (merged 2026-11-20)

GUARD — the gate harness: the shared readers and the selftests

`scripts/gates/lib.sh` and the plumbing every gate shares: the Rust
reader (`gate_rust_code`), the test-only skip, the selftest harness, the
guards on a reader's pipeline stages, and the readers gates copy instead
of share. Every row is a way the harness lets a gate read nothing, read
wrongly, or prove itself on the wrong evidence, while the gate prints
OK. Since the 2026-10-26 split the matcher rows are BLIND's
(`work/blind/`) and the ungated-population rows CIW's
(`work/ciw/`, UNREAD's until it merged there on 2026-11-02). Charter and unit order: `work/ciw/plan.md`;
narrative in `docs/doc-ledger/blind-guard-tint-and-vacuity-merge-into-ciw.md`.

## From TINT (merged 2026-11-20)

S-TINT — test-suite integrity

**The instruments that cannot go red, after the 2026-09-20 cut**:
censuses, markers, rosters and hand-kept enumerations that pass on a
substring, on a term that matches nothing, or on a list nobody updated.

The source-scanning censuses hand-parse Rust and fail loud, so an
ordinary construct reds the gate rather than being read; `mesh7r1_probes`'
`R1-DOOR-ONLY` markers are grep-only, sentinels with no reader; the
hand-written-impl census's only per-impl sight anchor is a suppression
list that SHRINKS; a `Shared` ledger row is checked by one substring,
so a site that keeps the substring and loses the behaviour still
passes. And `test-utils` documents itself as dev-only while being
production source to every narrowing gate — the fact several of those
gates' own arguments rest on.

TINT measured **207.5 budget points unpriced and 97.5 once its rows
carried a cost** — the clearest reading yet of what the 2.5-point
default does to an unscored slate. It was cut on 2026-09-20 (Ev, in
chat) into four tracks: VACUITY (rows that cannot fail), FIXTURE (one
shape spelled n times), HELPER (one helper with several homes) and this
remainder. TINT keeps its band 3500-3599.

Charter and order: `work/ciw/plan.md`; narrative in `docs/doc-ledger/blind-guard-tint-and-vacuity-merge-into-ciw.md`.

## From VACUITY (merged 2026-11-20)

VACUITY — rows that cannot fail: assertions on their own codomain, probes that print, floors that count what they discard

**A row that cannot go red is not a test.** The sharpest half of
S-TINT's charter, as its own track: eleven tests in three mate/fix
suites that print probe answers and assert nothing; a phantom-turn row
that asserts nothing in three of four arms; sign and positivity asserts
riding a cancellation that is structurally zero, so they hold on their
own codomain; a fuzz row whose `continue` shrinks its own sample while
its coverage floor counts the discarded trials; an anti-vacuity floor
that survives deleting either reference it was built to protect.

Three rows are the other shape — a row that goes red for a reason that
is not a defect: a strict-gap floor gating hosted CI on a varying
seed, a suite that reds under a malformed ambient epsilon, and a
random-ray oracle that tripped once at eps 1e-6. The last of those is
**a test that actually failed** and is the one row here that may be
reporting a real pick defect rather than its own weakness; it is P1 and
should be read first.

This program claims no paths: the rows are spread across every crate's
suites, and each unit announces its fence in its own PR.

Charter and order: `work/ciw/plan.md`; narrative in `docs/doc-ledger/blind-guard-tint-and-vacuity-merge-into-ciw.md`.
