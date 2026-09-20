# FIX — kernel and façade doors with the fix written (plan)

**STATUS: OPEN (2026-09-03).** Opened 2026-09-03 from
`docs/WORK-TRACKS-2026-09.md` (FIX section). Live state is
`work/fix/log.md`'s tail and the item files beside this plan, never
this file.

Branch prefix (the #396 convention): **`fix/`** — unit branches
`fix/<unit>-<slug>`, orchestrator branch `fix/orchestrator`.
Away-channel tag `(FIX orchestrator)`. A/B ordinal band
**FIX = 1700–1799**, claimed in `docs/MODEL-AB-LOG.md`'s banding entry.

## Charter

The items every program filed and none scheduled because the fix was
too small to cut a unit for: typed declines whose payload is named in
the body, `Display` impls the consumers already need, a one-line
finiteness gate, a rename. Each is E. This program exists so that they
land instead of accreting, and it spans fences by construction — one
item per PR, the fence named in the PR body, the owning program told on
the away channel.

## No design decisions (Ev, in chat, 2026-09-20)

*"Can you kick all the design decisions back to the track they actually
belong to, leaving fix design-free?"*

**A row whose blocking question is a design decision is not this
program's**, whatever ground it was found on. It goes to the track that
owns the surface the decision is about — by `git mv`, with a
`## Re-homed` section on the item stating the question, the routing
basis and what was NOT decided for the receiver, and a note on that
program's `log.md`. **Fourteen rows left this way on 2026-09-20**; the
sweep and its routing table are in `log.md`.

Three consequences worth stating, because each is a way the rule gets
broken by accident:

- **Route from the instrument, not from this directory's prose.**
  `python3 scripts/work.py territory --files -` names the owner. Two
  fence claims in `program.md`'s `keep_out` were stale when the sweep
  read them (`census.rs` as CURVED's, `mc.rs` as unowned), and both had
  been used to justify homing a row here.
- **Where FIX IS the owning track, the seat rules rather than
  re-homes.** The three paths in `paths:` are this program's, and a
  door decision on its own file is what the seat is for. A ruling like
  that goes on the item with its argument and with what the lane must
  still establish — not into a PR body, where it dies with the branch.
- **A written fix on another program's ground still belongs here.** The
  rule is about decisions, not fences. Announce and take it.

## Review posture

**No A/B row on any unit of this program** (Ev, in-chat, 2026-09-04):
the band 1700-1799 stays unclaimed and `docs/MODEL-AB-LOG.md` is not
touched. The exemption is absolute.

**Reviews are LIGHT OR ABSENT** (Ev, in-chat, 2026-09-11; restated
2026-09-20: everything here is orchestrator-review or style-only). The
units on this slate are small enough that a standing review lane costs
more than it returns; the orchestrator adjudicates from the diff, and a
style lane (`docs/prompts/reviewer-style-lane.md`) runs where a diff is
large enough to reward one. **No unit here carries a full adversarial
review; a unit that would need one is a unit cut wrong, and gets re-cut
or re-homed instead.**

**What the lane owes instead**, because nothing downstream will catch
it: the discipline moves INTO the brief. Every dispatch carries the
three instructions the log earned the hard way — *a row citing a clause
gets "read the clause, not the row's summary"*; *a row citing a site
list gets "re-derive it at your merge base"*; and *a unit that changes
a RENDERING owes an answer to "does any existing pin discriminate the
old rendering from the new one", where no is the pin being part of the
defect*. A lane running without a reviewer states every place it was
unsure explicitly, in the PR body and its report, rather than smoothing
it over. That is the trade: less review, more disclosed uncertainty.

## Unit order

Five rows, every one of them a written fix. Order is free; nothing here
blocks anything else here.

- `subject-refused-accepts-the-one-refusal-that-must-not-go-through-it`
  — **ruled (a)**: `Subject::refused` returns `Subject::NoBodyRoots`
  for the no-body arm, routing on
  `product::ProductErrorKind::means_no_body`, and the hand-routing
  comes out of both production callers. FIX's own file; the ruling and
  what the lane still establishes are on the item.
- `remap-name-misses-lose-the-id-they-caught-at-six-refactor-sites` —
  **ruled: carry the id** at the six `map_err(|_| …)` sites. FIX's own
  file. The open question is whether any of the three error kinds
  crosses to Python, which is a disclosure, not a quiet absorption.
- `quantity-fmt-error-display-row-is-a-verbatim-copy-of-assert-f6` —
  **ruled: fold** onto `test_utils::f6::assert_f6`, `test-utils` into
  `[dev-dependencies]`. Plant the sibling-arm leak and show it red
  first; announce the edge to S-TCOST and S-TINT.
- `fillet-leg-carrier-renders-raw-float-noise` — route
  `FilletLegCarrier::Arc`'s two `f64` fields through `path::num`, the
  one unit-bearing scalar rendering in `crates/profile/src/` that does
  not. PATHS's ground, by announcement.
- `recourse-chain-stops-at-the-second-hop-carriers` — the five
  second-hop carriers (`PcurveCertifyError`, `MeterError`,
  `PatchBoundError`, `FitError`, `SplineError`). The method is the
  parent unit's and is established: ground each repair in the module's
  or the variant's own docs, assert the DELEGATION where the carrier
  below has no enforcement row of its own, and leave the arms that are
  already right alone. Counts are read-by-eye at the row's merge base —
  re-derive. Crosses CHART, ENCL and PROPS by announcement.

## Exit shape

The slate empties; the walk convention applies. New one-PR findings on
ground no live program is working may be homed here while the program
is open — **new findings that need a decision first may not**, and go
to the owning track the day they are filed.

**Worth raising at the next handoff rather than deciding here:**
`work/door/` was opened on this program's own precedent and carries the
same charter in the same words — *"one-PR rows whose body already
contains the fix: no design question, no ruling, no census to build
first"*. Two programs with one charter is the shape this tracker
normally folds. FIX holds five rows; DOOR holds eleven.
