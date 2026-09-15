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
