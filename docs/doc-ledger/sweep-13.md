# Sweep 13 — 2026-09-13: M10 leaves the tracker

Sweep SHA: `bfafe4ad10286e1624f23a1100b5cb9c2fe1d7d6` — the commit
immediately before the deletion (on the closing PR's branch, reachable
from `main` through that PR's merge commit; it is the state in which
M10's directory is complete, `program.md` reads `status: closed`, and
every row in it is closed), so every path below is recoverable at
`git show bfafe4ad10286e1624f23a1100b5cb9c2fe1d7d6:work/m10/<FILE>` and
`git show bfafe4ad10286e1624f23a1100b5cb9c2fe1d7d6:docs/M10-EXIT-WALK.md`.

M10 — the error-propagation MVP — opened 2026-08-29 and closed
2026-09-13 on Ev's ratification of `docs/M10-EXIT-WALK.md` (PR #1700,
in chat: approve the walk and do the exit sweep; the orchestrator that
re-cut the walk had stopped and a successor session carried both).
**Thirteen units**, every one merged on its own green hosted head:
M10-D (#1146), M10-DI (#1154), M10-1 (#1147), M10-P (#1174), M10-2
(#1213), M10-3 (#1231), M10-4 (#1627), M10-5 (#1638), M10-6 (#1685),
M10-7 (#1725), M10-8 (#1828), M10-9 (#2048), M10-10 (#2100). Per the
sweep-5 rule the directory leaves whole — `program.md`, `plan.md`,
`log.md`, the seven unit rows still in it (`M10-4` … `M10-10`) and
twelve closed issue rows — with the twenty-two OPEN rows re-homed
first, below.

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `m10` | M10 — the error-propagation MVP | 2026-09-13 | this entry; the walk at the sweep SHA; the design at `docs/ERROR-DESIGN.md` (E1–E12, ratified) and `docs/DUAL-DESIGN.md` (DL1–DL6); the A/B record at ordinals 500–511 in `docs/MODEL-AB-LOG.md` |

### What survived, and where

The program's output is the tree, not its directory:

- **The design.** `docs/ERROR-DESIGN.md` E1–E12 and its E3 amendments
  (E12 ratified #1712), and `docs/DUAL-DESIGN.md` DL1–DL6 (#1146,
  which closed the D1 hedge: a dual is tangent transport and never
  certifies). Both are PROPS' files from this sweep.
- **The code.** Distributions, the `Measure` sink and report-only
  `Assertion`s, the E6 subdivision driver, the `Dual` sensitivities
  and the certified-worst-case stackup, the E7 clearance
  trichotomy, the E10/E11 reporting rows, and the E12 symbolic
  identity tier (`geom_core::sym`) with its registered-identity door.
- **The gates.** Three E10 CI rows, live and read by STEP conclusion
  and by LOG: assertion gating over every registered document, the
  goldened ε-keyed accounting, and the driver K population (rule 1 —
  an in-band indeterminate — measured 0 at every ε row, not
  demotable by any caller). The K addenda are in `docs/K-REPORT.md`,
  INSTR's file.
- **The demo.** `demos/tour/src/tolerance.rs`, two stops through
  public doors, with its own test inside `ci.yml`'s tour step
  asserting the numbers the captions print.
- **A successor program.** `work/sym/` — SYM, the E12 symbolic
  identity tier — opened in this sweep on Ev's call, holding fourteen
  of the twenty-two rows and the band 4700–4799.

### The numbers the walk carried, because nothing else now does

Criterion 11 — Ev's condition for closing the program, "a macroscopic
box certifies: the two-hole plate's real study returns certified
leaves bounded by genuine flips, not by ε". At the LEAF: the plate's
real study (±0.05 mm on the spacing, σ = 0.01 mm on the radii) driven
whole at 1,024 leaves certifies **431** leaves and refuses 593 on
budget — **89.07 %** of the mass, a certified worst-case hull
`[0.419, 0.845]` mm against the 0.5 mm floor — and every refused leaf,
refined to any depth by both reviewers, is bounded by the document's
own `assert_bound`: the real flip, which enters the box at 0.625 of
the study. Identical to the digit at three ε rows. At the CEILING: the
widest box that certifies WHOLE is **0.263** of the study (0.237 at
ε = 1e-6), bounded by dependency widening of the assertion's own
affine margin, which subdivision resolves — the class now at
`work/sym/real-margin-dependency-widening`.

**Verified at the sweep by execution**, not read off the walk: the
tour's own gating row (`the_two_stops_say_what_their_captions_say`,
the row that puts the cell inside `ci.yml`) ran green on the sweep
head at the default ε — `Receipt { certified: 193, refused: 319,
splits: 511 }`, holds 0.8337 of the tolerance mass, violated 0.0002,
unresolved 0.1661, in 320.6 s — which is criterion 8's and honesty
row 18's arithmetic to the digit.

### Residue re-homed before the deletion

Twenty-two open rows. The moves are earlier commits of the closing PR,
so the deleting commit finds the directory holding only closed work.
Every id is unchanged and every row carries a note in its own body
saying why it landed where it did. **Ev's call at the sweep** (in
chat, 2026-09-13): "the 14 to a successor program, the others to
either another successor program or a preexisting program" — the
fourteen rows standing on one territory being what `work/README.md`
calls a successor's opening slate rather than residue.

| item | to | why |
| --- | --- | --- |
| `real-margin-dependency-widening` | `work/sym/` | the numeric channel's half of E12's division of labour — what stands between the plate's 0.263 ceiling and the flip at 0.625 |
| `plate-ceiling-is-now-the-scaffold-pushforward` | `work/sym/` | one predicate bounds all five measured documents; the fix half is a PCURVE/D3 question and `geom-brep/src/certify.rs` is in no program's paths — TRIM is PCURVE's successor |
| `rule-d-reaches-the-unit-bulge-only` | `work/sym/` | rule D is the tier's, and this is the next ceiling class after M10-10 |
| `interval-self-dot-straddles-before-rule-a` | `work/sym/` | rule A's reach; the FIX is `powi(2)` in PROPS' `linalg/vec.rs` and is announced there — PROPS' linalg lane may take it [the row later moved to `work/decide/` and was CLOSED on DECIDE-1's measurement (#3001) with no `linalg` change: the fix had landed at M2 PR 4 for every norm] |
| `param-box-certification-of-implicit-quantities` | `work/sym/` | the tier's frontier: an iterated quantity has no expression in the parameters. Came to M10 from S-CERT; follows the tier |
| `declared-tangency-needs-the-registered-identity-door` | `work/sym/` | the door's live consumer; open rather than parked, and it waits on BLEND's fillet row |
| `the-span-identity-is-not-a-theorem-of-the-floats` | `work/sym/` | the limit of what a registration means; `register_equal` is in PROPS' `real.rs`, reached by announced seam |
| `the-witness-slack-is-eps-independent` | `work/sym/` | the same door and the same seam: `WITNESS_REL` does not move with the run's ε |
| `sym-registration-flattens-two-axes` | `work/sym/` | the door's shape and its cost, on the record |
| `symbolic-tier-costs-95-percent-of-the-m10-3-drive` | `work/sym/` | the work it asks for is a profile INSIDE the normal form. **The measurement stays S-TCOST's** and is named as such in the row |
| `derived-frame-placement-freezes-on-the-symbolic-lane` | `work/sym/` | DOCM-1's review found it; every freeze is a budget refusal in `sym::form_in`, so the row follows the mechanism and names DOCM |
| `symbolic-tier-census` | `work/sym/` | the tier's own 107-row reference, which `sym.rs`'s module docs cite |
| `sym-rs-is-one-file-with-a-347-line-header` | `work/sym/` | this program's file and nothing else's |
| `registered-is-spelled-five-times-and-pinned-once` | `work/sym/` | four of the five spellings are the tier's; the fifth is INSTR's k-lint column and is named |
| `certified-hull-padding-is-the-leaf-width-not-the-lane` | `work/props/` | the hull is `stackup.rs`'s, PROPS' already; the tier is the cause and not the site |
| `coincidence-zone-priced-budget-at-the-floor` | `work/props/` | a `RefusalReason` arm in `drive::classify_replay`; already `refs` PROPS' `k-stats-escalation-channel-and-redo` |
| `min-clearance-refusal-stringly-twin` | `work/props/` | the twin is in `measure.rs`; the layering question it waits on is the 1055 valve's seam |
| `mc-lanes-draws-are-not-reproducible-from-outside-the-crate` | `work/props/` | `mc.rs` is the advisory half of the analysis lane; the `pncad` re-export half is LIB's and is named |
| `fillet-tangency-is-not-the-constructors-node` | `work/blend/` | the profile fillet door: `build_seg` re-derives the centre the joint classifier needs. BLEND's title names that door and it inherited FILLET's residue |
| `revolve-carriers-state-only-the-rim` | `work/blend/` | `sweep/src/revolve/*` is BLEND's paths and what is owed is a constructor change |
| `symbolic-tier-and-clearance-engine` | `work/shell/` | `min_separation` is concrete at `Interval` in SHELL's `clearance.rs`; SHELL-3 is the same question from the other end |
| `pncad-py-eval-err-variants-outside-the-tag-inventory` | `work/census/` | CENSUS's class exactly — a vocabulary spelled by hand and the census that cannot see one spelling; `pncad-py` is LIB's and is announced |

**The territory moved with the rows.** PROPS takes the analysis lane
its `keep_out` had named since 2026-09-06 — `analysis.rs`,
`distribution.rs`, `drive.rs`, `measure.rs`, `mc.rs`, the `m10*` and
`e4_dual*` suites, `ERROR-DESIGN.md` and `DUAL-DESIGN.md`. SYM takes
`geom-core/src/sym.rs`, `sym/*` and `geom-core/tests/m10_*`, which is
a double claim inside PROPS' `geom-core/src/*` glob, written on BOTH
sides in the opening commit as `work/README.md` requires.
`crates/bvh/src/*`, which PROPS' clause had parked on M10, is in no
program's paths now and PROPS' `keep_out` says so.

### The inbound pointers this sweep rewrote

Every live `work/m10/…` citation in the tree, in three classes:

- **a row this sweep re-homed** — repointed at its new directory, the
  id unchanged: `geom-core/src/sym.rs`, `real.rs`, `k_stats.rs` and
  `Cargo.toml`, the `sweep` carriers, `editor-core`'s clearance and
  measure and its m10 suites, `demos/tour/src/tolerance.rs`,
  `docs/ERROR-DESIGN.md`, `docs/K-REPORT.md`,
  `scripts/gates/register-equal-allowlist.sh`,
  `crates/pncad-py/tests/test_binding_census.py`, and eight tracker
  rows;
- **a row re-homed at an EARLIER sweep**, so the citation was already
  stale: `signed-penetration-depth` (CURVED),
  `clearance-window-tightening-needs-chart-boundary` (TRIM),
  `contribution-bounds-via-dual-interval` (PROPS). Two of those are
  the pointers `docs/TRIM-3-SPEC.md` item 8 asks TRIM-3 to correct as
  a rider; **that rider is now a no-op** and the spec was left alone
  rather than edited under a dispatched lane;
- **a row closed WITH the program**, or M10's own plan or log — named
  as M10's closed row with this ledger entry as the way back. Eleven
  sites in `crates/` say so now (`first-refusal-at-twice-the-ceiling-
  is-an-order-artefact` ×5, `plate-rim-residual-needs-the-wide-
  coefficient-ring` ×3, `plate-ceiling-is-now-the-arc-span-identity`,
  `rule-d-multiple-reader-wraps-on-a-huge-dyadic-coefficient` and
  `M10-8`), and eleven more across seven tracker rows.

One citation was FALSE rather than stale and was fixed:
`work/props/three-per-node-verdict-shapes` said `drive.rs` is edited
by an announced seam because it is M10 territory. It is PROPS' own
from this sweep, so the seam is discharged.

**Eight header `refs:` broke on the unit rows** and `lint` caught
every one — this is the case CITE's sweep did not have. `M10-4`,
`M10-5`, `M10-7`, `M10-8` and `M10-10` were named by eight surviving
rows across BOOL, PROPS, SHELL and SYM, and each id was replaced with
that unit's **PR number** (1627, 1638, 1725, 1828, 2100), which is an
int the tracker does not resolve and which is where the unit's
documentation actually lives. Nothing else in those headers moved.

**Left as history, deliberately**: the provenance lines in programs'
logs ("`X` from `work/m10/`"), `docs/MODEL-AB-LOG.md`'s unit rows, and
the nine per-merge deletion notes in `docs/doc-ledger/` that cite
`work/m10/log.md` or `work/m10/M10-*.md` as a unit's statement of
record. Each states what was true when it was written, and every one
of those paths is recoverable at this entry's sweep SHA.

### Honesty notes

- **The walk was ratified by a session that did not write it.** Ev's
  word was to approve #1700 and sweep; this session read the walk,
  verified the one claim it could verify in reasonable time by
  execution (the tour's gating row, above), and did not re-run the
  1,024-leaf study or the hosted K rows. Criterion 11's 431/89.07 %
  and the 0.263 ceiling are the walk's numbers, carried forward here
  on the evidence rows it cites, not re-measured at the sweep.
- **Fourteen rows against eight is not a tidy split, and the fourteen
  are not finished work.** They are what four units on the symbolic
  tier left standing: three documents that do not certify their
  studies at any affordable dial, a door with two unbuilt registrants,
  a tier at 95 % of the interval drive that nobody has profiled
  inside, and one file of 3,898 lines. A program opened on that slate
  starts in debt, which is the honest shape of it.
- **Two rows SYM's door needs are not SYM's.** The fillet's declared
  tangency and the revolve carriers' span identity are constructor
  changes in `crates/profile` and `crates/sweep`, so they went to
  BLEND; SYM holds the consumer of the first and cannot finish it
  alone. The pairing is stated in both directions (`work/sym/log.md`,
  `work/blend/log.md`) rather than left for whoever picks one up.
- **`work/m10/plan.md` is not quoted anywhere.** The walk quoted its
  criterion rows verbatim and the walk is recoverable at the sweep
  SHA; the plan itself, with its substrate inventory and its lane
  order, is recoverable there too and is nowhere else.
- **The A/B band 500–599 stays claimed and closed**, on the VIEW and
  S-TCOST precedent: thirteen dual reviews at ordinals 500–511 (M10-D
  was a design pass), samples #39, #40, #43, #49, #50, #114, #115,
  #118, #124, #145, #150 and #173, with three symmetric tally pairs.
  Nothing renumbers.
