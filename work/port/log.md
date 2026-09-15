# PORT log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/port/plan.md`.

## Opened (2026-09-11)

Opened in the tracker cut of 2026-09-11 (Ev's direction, in-chat;
`docs/WORK-TRACKS-2026-09.md` addendum 3). Seven rows moved in by
`git mv`, each with a `## Re-homed` record: three from `work/issues/`,
four from `work/code-quality/`.

This is the track most likely to be dissolved by its own seams: every row
is on LIB's, EXCH's or DOCM's ground, and any of them may take a row at
any time. The plan says so — a row claimed away moves by `git mv` and
this program does not track it afterwards, which is code-quality's rule
applied to a program that is its descendant.

## First sitting: two rulings answered, two calls taken (2026-09-15)

The program had never dispatched anything — eight rows, all `open`, no
`port/` branch, no PR, and a log with one entry. This sitting did no
code; it cleared what stood in front of the code.

**Two rows were waiting on Ev and neither had been asked.** `S107`
carried `needs_ev: true` since 2026-08-20 and `msrv-floor-…` was called
an Ev question by the plan, but `git log --all --grep` finds no `[ev]`
PR was ever opened for either. They were flagged, not asked. Both were
put to Ev in chat and both were answered the same day:

- **`S107`: a defect**, on the general principle that *Python should
  always match Rust where it can*. The compatibility reading had no
  installed base to protect — `publish = false`, no name yet (Q9). The
  ruling closes `S107` carrying its verdict; the work it releases is
  `python-dimensionerror-names-the-quantity-check-not-the-dimension-check`,
  filed the same day, because a residue disclosed in prose is not
  scheduled (`work/README.md`). That row and
  `load-path-stringifies-structured-refusals` are one door and spec
  together: the first frees the name `DimensionError`, the second
  decides what the load door's refusals arrive as.
- **`msrv-floor-…`: a check that the two strings are equal, and call it
  a day.** That is the row's reading (2) made mechanical. It is a better
  answer than it looks: pinned equal, the channel *is* the floor, so
  every job that already compiles on the channel compiles at the floor —
  the promise becomes the one CI already proves, with no new build row.
  The row now names its cost (the floor can never lag the channel; the
  day it should, the gate is deleted deliberately and the original
  question returns with a consumer attached) and its shape: a
  `scripts/gates/` row, not a Rust `#[test]`, with the substitution
  disclosed for Ev to reject. Class drops **M → E**.

**The two assembly rows were the orchestrator's call and are decided.**
Ev declined to weigh in and left it here on a confidence test. Both
went the way the tree already pointed:

- Widen. `AssemblyError::AtRest` already carries every finding and
  publishes that as a guarantee; the two mint arms drop all but the head
  twenty lines away in the same function. Contracting would mean
  deleting a guarantee the type publishes to make three arms agree at
  the weaker answer. This was never really an open question — the door
  had answered it.
- Kind-first in `resolve_face`. MSOLVE-5's premise for `operand_answer`
  — a non-face never mints anywhere — is about the name, not the table,
  so it holds at the root too. It is also the more answerable refusal,
  which is this program's standing review question.

They dispatch as **one unit**, same file, same door.

**Review posture rewritten** (Ev, in-chat): style review is the default,
full review reserved for the hardest units. The section had asked for a
correctness arm on every unit changing a public refusal or a binding
signature — which is nearly every row here, so it was a full review on
almost everything. The one full review on this slate goes to
`load-path-stringifies-structured-refusals`: found by execution rather
than reading, carries a two-sweep class obligation, and a wrong fix
there is invisible from Rust.

**Charter corrected.** `program.md`'s `keep_out` and `plan.md` both
routed three rows to **DOCM**, which closed at sweep 14 (2026-09-13);
`assembly.rs`, `node.rs` and `persist/wire.rs` are **EDIT's** now, and
EDIT's own `keep_out` names this program back for the first two. The
slate table was also two rows short — `load-path-…` arrived at DOCM's
exit sweep after the table was written, and `S107`'s successor was
filed today.

**Filed outside the fence:** `work/edit/edit-charter-counts-a-row-that-went-to-port`.
EDIT's charter counts the load door's structured refusals among its
rows, but that row came to PORT in the same sweep; and EDIT's `keep_out`
names PORT for `assembly.rs` and `node.rs` but not for `persist/wire.rs`,
where PORT's row actually lands. Worth noting for the double-claim
rule's eventual promotion to an error: **PORT claims no paths at all**,
so it can never appear in an overlap pair, and its announcement surface
lives entirely in prose that nothing checks.

Nothing on this slate now waits on Ev. `S415` is still the opener.

## 2026-09-15 — S415, the three boundary residues (implementer lane)

All three closed on `port/s415-boundary-residues`. None of the files
are PORT's: `crates/step-import/*`, `crates/step-export/*` and
`crates/stl/*` are **EXCH's**, `crates/pncad-py/*` is **LIB's**. The
unit is announced to both in the PR body; either may take the row's
successors instead.

**Three of the row's premises were wrong**, two of them corrected in
the spec and one found here. The spec's two held: the printable-ASCII
band is **two formats** (Part 21, read and written) and not three, and
`no_minted_id` **does** have a kernel enum behind it. The third is the
spec's own: it said the two `no_minted_id` paths differ on the wire
because `declare_err` "carries the arm's fields" — it does not for
that arm, which passes `(None, EditPayload::NONE)` exactly as
`boundary_edit_err` does. The two were byte-identical to a Python
caller, and only the human message differed. That changed the verdict
from "a defect a caller can hit" to "a synonym nothing would have
reported", which is the shape the LIB filing below carries.

**Judgement calls taken**, since the spec left all three open:

- The scaffold guard is a **post-condition on the minted endpoint**,
  not a pre-condition on `p`: it states the rule the two sites share
  (a strut with coincident endpoints cannot certify) rather than
  today's arithmetic for reaching it, and it survives a change in the
  offset. Both sites share it outright, as two free functions in
  `assemble.rs` — `coincide` and `strut_endpoint`.
- `plant`'s coincidence check moved from **bitwise to value
  equality**. Bitwise is narrower than the hazard: `0.0` and `-0.0`
  are different bits and the same point, and the chord between them
  is still zero-length.
- **The Part 21 duplication stays.** A shared home would need a new
  crate or a dependency edge between the two STEP crates, and the
  kernel is no place for a text-format constant (`docs/DESIGN.md`'s
  `## Layering` section — prose, carrying no D-number, so it is cited
  by name rather than by an id it does not have). A leaf crate below
  both is the workspace's own precedent (`test-utils`), and a
  `step-import` -> `step-export` edge already exists as a
  DEV-dependency for the round-trip oracle — neither can carry a
  shipped constant as it stands, and both are heavy answers for one
  range. What
  changed is that it is disclosed at all three sites — the two STEP
  sites as a mirrored pair, the STL site as a coincidence deliberately
  not shared — and that both bounds are pinned by a test at each STEP
  site, which neither had.

**Filed outside the fence** (§6), both in the same PR:

- `work/exch/step-scaffold-strut-offset-is-absolute-in-a-unit-free-format`
  — the second half the row did not name: `1.0` is absolute in a
  format whose coordinates carry no unit contract. The degenerate
  half is closed here; the magnitude is a design call on EXCH's
  ground.
- `work/lib/boundary-minted-refusal-tags-are-pinned-nowhere-and-share-the-kernel-namespace`
  — the tag inventory re-derives itself by reading `src/tags.rs`, so
  the three words minted at raise sites (`name_serialize`,
  `not_utf8`, `wireframe`) are outside its reach: their values are
  pinned nowhere and nothing would have reported the `no_minted_id`
  collision.

The **M** estimate held: four crates, three judgement calls, two
filings, no ruling in front of any of it.

### Fix pass after the style review (2026-09-15)

Five must-fix, one sweep receipt, five rows filed. What moved:

- **The threshold was wrong and it was wrong three places deep** — the
  original row, the spec, and this lane's own `strut_endpoint` doc all
  said the offset vanishes at `|p.x| >= 2^53`. Checked: it is not a
  threshold on EITHER side. In `[2^53, 2^54)` the offset is exactly
  half a step, so round-half-to-even decides on the value's own
  mantissa parity — `2^53` loses it, `2^53 + 2` keeps it. From `2^54`
  out it is always lost. The negative side is the same picture shifted
  one binade, because a positive offset moves a negative coordinate
  TOWARD zero into the finer binade: `-2^53 + 1.0` is exact. The test
  now pins both bands on both parities and both edges, so the claim
  cannot be restated wrong without reddening.
- **The site-level row the spec asked for was written, and the branch
  turns out to be dead on the corpus.** Instrumenting the arm and
  running the whole `step-import` suite reaches it **zero** times: the
  corpus always takes the plain `mef` fan order. So the row drives the
  arm directly, and its ordinary-coordinate control proves the guard
  is not unconditional.
- **`mev_line` does refuse a coincident chord, and the message is the
  argument.** Probed: *"certification: the stored parameter interval
  is not forward … a degenerate zero-span interval is refused by the
  same gate"*. Loud, but in the certification gate's vocabulary about
  parameter intervals — which is why the guard's own `Topology` arm is
  worth having, and the PR says so with the real text rather than a
  guess about a NaN direction.
- **`D-layering` was an invented decision id** carried from the spec
  into this log. `docs/DESIGN.md`'s `## Layering` is prose with no
  D-number and is now cited by name. Two related claims were also
  looser than stated and are fixed in the code comments: a leaf crate
  below both is the workspace's own precedent (`test-utils`), and a
  `step-import` -> `step-export` edge already exists as a
  dev-dependency.
- **The spec's ledger entry was missing.** `docs/DOC-LEDGER.md` now
  carries the `## Per-merge deletion` row, with the recovery SHA and
  the four statements of the spec the unit corrects.

The **band class was not swept** in the first pass, which is the class
this unit is mostly about — the disclosure covered three of eleven
statements of `0x20..=0x7E` across the three crates. Swept now: the
eleven-site hit list is in the PR, and the per-crate restatements that
had no reason to spell the numbers now point at the site that decides.
Nine remain, each with a stated reason (three deciding matches, two
user-facing refusal texts, three public docs, one bounds argument).

**Filed outside the fence in the fix pass** — LIB:
`persist-err-projects-fourteen-arms-through-a-fifteen-slot-positional-tuple`,
`doc-module-header-promises-a-door-it-does-not-hold`; EXCH:
`signed-zero-module-hand-counts-the-bitwise-readers-it-exists-for`,
`three-crates-fold-signed-zero-with-one-body-and-three-unlinked-arguments`.
The 76 tag words restated in `py/doc.rs` prose went as **evidence on
the row already filed** rather than as a second row — same fence, same
blind spot. That row's proposed remedy was also rewritten: it had
offered a hand-maintained roster of words nothing derives as the
answer to "nothing derives this set", which is the trap, pre-committed
into the next unit's brief. It now states the two properties owed and
leaves the mechanism open.
