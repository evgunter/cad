# SYM-5 PR-2 review brief — r2 (rule E, the quotient's common factor)

You are reviewer **r2** of the v6 dual for unit SYM-5 of program
SYM (the E12 symbolic identity tier, `geom_core::sym`), PR #2589 on
`evgunter/cad`, **frozen head `480704dbb81c32ee2ff0135171531bc3516d61fa`**. Your review is one of two
run CONCURRENTLY on this head by two reviewers who must not see each
other's work. The unit's Phase 1 (PR #2568, merged) is the measurement
this PR builds on.

**Read first, in full:** `docs/prompts/reviewer-style-lane.md`
(binding; name the questions exercised; `sure`/`likely`/`unsure` on
every finding); `docs/SYM-5-SPEC.md` (the binding spec, with its
Amendment A1 and its stop condition); the PR body (long, and it is
the unit's argument — `mcp__github__pull_request_read` `get`; load
with ToolSearch `select:mcp__github__pull_request_read,mcp__github__actions_list,mcp__github__get_job_logs`);
`crates/geom-core/src/sym/quotient.rs` whole (the rule and its
soundness argument); the diff to `sym.rs` (the dial, `without_rule_e`,
the rule's place after the per-node A/B reduction) and to
`sym/trig.rs` (`sqrt_atom` normalising its argument);
`crates/editor-core/tests/m10_derived_frame_tilted_interval.rs`
whole; the re-baselined pins (`m10_bulge_interval.rs`,
`m10_10_pins_interval.rs`, `m10_sym_profile_interval.rs`'s ledger);
`sym.rs`'s header sections `# The form-level algebra (M10-10)`,
`# Freezing`, and the new rule E section; `sym/form.rs` (what a
`Form` and a `Poly` are now — SYM-4 landed under this PR);
`memories/review-and-dependency-policy.md`.

## Your lane (v6 item 5 isolation — READ side)

- Worktree `/home/user/lanes/sym-5-r2` at the frozen head, branch
  `sym/5-review-r2` for probe rows. Every command:
  `cd /home/user/lanes/sym-5-r2 && CARGO_TARGET_DIR=/home/user/sym-5-r2-target CARGO_INCREMENTAL=0 …`
  on ONE line. Wait until `/home/user/sym-5-r2-target/SEEDED`
  exists before any cargo command (foreground `ls` poll). Private
  scratch `/home/user/sym-5-r2-tmp/`.
- **Until your report is delivered you must not fetch, check out, or
  read the other reviewer's branch (`sym/5-review-*` other than yours),
  worktree, target, scratch or CI artifacts, nor the implementer's
  worktree (`/home/user/lanes/sym-5`) or scratch.** The PR, the branch
  `sym/5-unit-vector` and your own tree are what you read. Any glimpse
  is DISCLOSED in your report (a `pgrep -af` prints other lanes'
  command lines — use `pgrep -f` with your own path, or `ps` filtered
  to your own target).
- Push your probe branch early and often. Foreground rule: no
  background waiters; anything over 600 s runs `setsid`-detached with
  a recorded PID and is polled in the foreground; never end a turn
  with a detached job running. Four cores shared with one other
  review lane: one crate at a time; the ceilings rows are minutes per
  document — run them detached and polled, one at a time.
- Hosted run **34861725623** on the frozen head: read its job list
  yourself (twelve `test (…)`, five `k-lint (gate, …)`) before
  conditioning anything on it. Two earlier runs on this branch were
  red on earlier heads; say what they failed on if it bears on the
  head you review.

## Reviews here include end-to-end exercise

Build a document of your own that the unit did not measure — a
derived frame whose axes carry a parameter in a different way (a
rotation about a different axis; a `FaceFrame` on a face of a REVOLVED
body; two derived frames stacked), and a document with a genuinely
non-unit vector that a careless rule might "normalise" — drive them
through the public doors at `Sym<Interval>` with the rule on and
off, and report what happened, in scope and in ergonomics. Local runs
are unique-signal only: your probes, mutants, re-takes; the existing
suites ride the hosted gate.

## The claims to falsify (the dispatcher's hypotheses, not findings)

1. **Soundness of rule E.** Three steps in the early walk at every
   node: divide out the monomial both halves share; scale so the
   denominator's coefficient at its smallest monomial has magnitude
   one; fold `r·D'/D'` to the constant `r`. The PR argues every step
   is an equality of rational functions wherever the denominator is
   non-zero, and that a form's denominator is built from the
   numerators of the `Inv` nodes above it — reals the value channel
   DIVIDED by — so a box where it vanishes is one clause 1 already
   refused. **Hunt the box where that argument fails**: a denominator
   that vanishes at an interior point of a box clause 1 admits (the
   value channel's `Inv` of an enclosure straddling zero is `Trv`/
   poison — does clause 1 always see it, including through
   `Form::recip`'s poison and through a frozen node?); a shared
   MONOMIAL factor divided out where the monomial's indeterminate is
   zero at a point (an atom whose argument's form vanishes — `sqrt(0)`
   — and a form that is `x·a/(x·b)` at `x = 0`); a fold `r·D'/D' = r`
   when `D'` is the zero polynomial after scaling. Write the row that
   reds if any of these folds a non-identity; if you cannot find one,
   say what you tried.
2. **Nothing lost, by execution.** The PR says every pinned count
   moved UP or not at all and no ceiling on the five measured
   documents moves by a digit. Re-take: the M10-8/9/10 pins (they ride
   the gate — read the run), the walk ledger (the PLAIN walk's digests
   must be unchanged — rule E runs in the early walk only; the
   early/door digests moved and were re-baselined "with a reason":
   read the reason and confirm the plain chains are byte-identical to
   SYM-4's pin commit), and at least two of the five ceilings on your
   box with the dial on and off (the plate and the pad).
3. **The mechanism is what the chain says.** The refused residual's
   early form carries `sqrt(P(t)/P(t))` — the number ONE as an opaque
   atom — and two frozen products; the rule removes the common factor
   so the products fit the budget and the residual is the zero form.
   Render the tilted document's residual with `without_rule_e` and
   with the shipped set (`explain_depth 6`) and confirm the chain and
   the discharge; then check the PR's reading that candidates (a) and
   (c) "re-key and remove nothing" — is that right, or would (a)'s
   `U_i · sqrt(S) = a_i` have reached the same forms?
4. **The two interactions the lane found** — rule E must run AFTER the
   per-node A/B reduction or rule A loses its square; `trig::sqrt_atom`
   must normalise its argument through the rule or the two spellings
   of one arc mint two atoms. Is the ordering a principled fact (a
   rule that removes a factor rule A needs — is there a THIRD rule
   that needs what E removes?) or a fix for the two rows that failed?
   Plant the ordering the other way and name what reds.
5. **The dial ships ON with the pad at 10.9 s** (off 3.73 s) against
   the header's 1.6 s affordability line; the other four are under
   the line or cheaper (the link improves). The spec says "shipped on
   only if affordable on the five measured documents". M10-10 shipped
   its algebra with the pad at 10.1 s. Is the lane's balance the right
   call, and is the cost table right (re-take the pad and the link on
   your box, release, both dials)?
6. **The 5e-2 refusal is the PROPS defect one site over** (a clause-1
   `Invalid` at the boss's CAP plane), pinned by name as current
   behaviour. Confirm it is that and not the rule's; and that the
   parity row's `without_rule_e` half says what the rule is FOR (the
   derived boss refuses on `newell_plane_residual` under `Guided`
   while the twin certifies).
7. **The bulge pins moved for the right reason** (R1's boss
   `carrier_on_surface_2` 27 → 9 numeric, `carrier_matches_mapped_source`
   6 → 0, the D-tabs' `carrier_endpoint_start` 4 → 0) and the boss's
   ceiling moved `8.26e2 → 9.36e2·ε` — the move SYM-3 measured a
   512-bit ring making, now at 256. Re-take one.
8. **No step cap, "structurally"**: one pass over both halves, no
   product allocated, output at most the input's terms and degree.
   Read the code for a case where it is not one pass or grows.

## Style lane emphasis

Q1: `quotient.rs` beside `algebra.rs` (rules A/B) and `trig.rs` (rule
D) — is the "rule module" shape consistent, and does rule E's scaling
step duplicate a normalisation `Rat::from_parts` or `Form` already
does? Q3: can the theorem row and the three negative rows fail on
anything but the dial? Q5: the new header section's promises against
the code. Q6: three disclosed deviations (the dial's name and
mechanism (b) over (a); no step cap; green at ε/8 and 1e-3 but not
5e-2) — improvements or debts? Q8: read `quotient.rs` whole and
`sym.rs`'s early walk.

## The dispatcher's own exposure

Everything above is the dispatcher's belief about the tree; check it
before building on it, and report any correction as a finding. The
spec named three mechanisms and the lane took (b) with a reading of
why (a) is not it — that reading is a claim, not a finding.

## Report (≤150 lines, to the orchestrator; do not post on the PR)

Verdict (`MERGEABLE` / `MERGEABLE-AFTER-FIXES` / `NOT MERGEABLE`) with
MAJOR/MINOR/NOTE findings on the claims, each with `file:line`, a
confidence, and whether DEMONSTRATED BY EXECUTION or by inspection; a
`## Style` section; the CODE QUALITY REPORT with the fixed rubric —
counts of MAJOR / MINOR / NOTE; spec deviations as reported vs SILENT;
three ratings 1–5 with one line of evidence each (idiom/structure,
test quality, doc/comment honesty); the questions exercised; the gate
verification (run id, head SHA, job census); what you ran locally with
its numbers beside the PR's; the e2e exercise and what it showed;
any glimpse (disclose). Push your probe branch and name the commit.
