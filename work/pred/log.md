# PRED log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/pred/plan.md`.

## Opened (2026-09-11)

Opened in the tracker cut of 2026-09-11 (Ev's direction, in-chat;
`docs/WORK-TRACKS-2026-09.md` addendum 3). Eleven rows moved in by
`git mv`, each with a `## Re-homed` record: one from `work/issues/`, ten
from `work/code-quality/`.

The class was visible in the 2026-08 scan and stayed unclaimed for the
reason the plan now states: each row is on four programs' files at once,
so no program could take one without reaching across three fences.

No branch exists yet. Two cheap first acts, neither a unit: verify
`S66`'s park (#862 merged, and its body says the deviation terms are
already gone), and put `S82`, `S65`, `S116p` and `S29` on the next
`[ev]` sitting.

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

**Two rows, one family.** The linear band, which
`band-linear-spelling-not-swept` made one spelling wherever a band is
CONSTRUCTED, has two populations that sweep could not reach:

- `band-derivation-has-a-scalar-twin` — ~15 sites derive `(eps, K*eps)` as
  bare `f64`s and never build a `Band` at all. The decision: do these
  consult `Tol` directly (close the class by saying so once), or does the
  pair want a named door on `Tolerance`? Reaching it through
  `Band::linear` means routing a two-number computation through a fallible
  constructor to read the accessors straight back out.
- `fixed-band-literals-are-an-unscoped-class` — 55 sites hard-code
  `Band::new(1e-9, 1e-8)`, which asks a different question from the run's
  band at every eps row but the default. One site carries a reason; the
  other 54 are unexamined.

They land here because PRED is the program for *one numeric fact, decided
in several places, each with its own margin* — the shape your slate
already carries three times. The door, if one is minted, lands in
`crates/geom-core/src/predicate.rs`, which territory says is PROPS's;
the rows are classes rather than files, so that site is announced.

**Read the second row's "One framing to NOT carry forward" section first.**
The disclosing lane's sharper claim — that fixed bands collide with the
suites declaring *"eps posture: no eps literal"* — was MEASURED and is
false; the two populations are disjoint. The finding survives as an
unexamined class, not a live contradiction.

The third member of the family, `band-helper-duplicated-across-suites`,
went to SUITE in the same sweep: that one is about where a test wrapper
lives, not about what a band decides.

Signed (FIX orchestrator).
