# PATHS log

## Opened at S-BOOL's exit (2026-09-16)

Opened by S-BOOL's orchestrator in the PR that proposes
`docs/S-BOOL-EXIT-WALK.md`, as `work/README.md`'s closing rule directs:
S-BOOL's eight open lattice items cohere into one track on one
territory (`crates/profile`), so the closing program opens the
successor and moves them here. Band 5000–5099 recorded in the ledger's
banding entry in the same commit. No unit dispatched; the first sitting
picks from `work/paths/plan.md` §The slate. Ev's sign-off on the exit
walk ratifies the opening.

## Rows arriving from FIX, 2026-09-20

Ev ruled in chat on 2026-09-20 that **FIX carries no design decisions**:
*"can you kick all the design decisions back to the track they actually
belong to, leaving fix design-free?"* FIX is the program for rows whose
fix is already written; three waves closed those and left a slate that
had drifted into decisions. Fourteen rows moved out by `git mv` to the
track owning the surface each decision is about, each carrying a
`## Re-homed` section stating the question, the routing basis, and what
was NOT decided for the receiver. Routing was taken from
`python3 scripts/work.py territory --files -` over every path the rows
cite, not from FIX's `keep_out` prose — two of that clause's fence
claims were stale and are corrected in the moved rows.

**One row: `arc-carrier-refusal-register-misses-two-format-arms`.**
`docs/PATHS-DESIGN.md` is yours by `paths`; the row was on FIX's slate
only because FIX took the two units that moved the arms the register
omits, and its own Fence section says so.

Its two halves separate cleanly. The cheap one: the *"Typed runtime
errors, from geometry"* register names neither `PathError::NonFiniteDirection`
(landed by PR 2359, never named) nor `PathError::UnderflowedDirection`
(PR 2415), both raised from inside that document's own surface — so it
wants the two arms plus the sentence distinguishing them from
`ZeroDirection` (not a coincidence at any tolerance; the recourse is
scale, not eps). The design half: whether that register is checked at all,
given `PathErrorKind` is a fieldless mirror and `pncad-py`'s
`TAG_INVENTORY` already pins the full arm set at the Python door — or
whether it is prose by design and should say so, so the next reader does
not take its completeness for a claim.

FIX keeps `fillet-leg-carrier-renders-raw-float-noise`
(`crates/profile/src/validate.rs`, your ground) because its fix IS
written — route the two `f64` fields through `path::num` — and FIX spans
fences by announcement for written fixes. Say if you would rather hold it
beside `validate-rs-hosts-a-quarter-of-the-fillet-subsystem-it-never-runs`.

Signed (FIX orchestrator).
