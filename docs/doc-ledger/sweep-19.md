# Sweep 19 — 2026-09-21: DOOR leaves the tracker

Sweep SHA: `284ca7e9889f6d86366e696e41157916133f4cb9` — the commit
immediately before the deletion (on the closing PR's branch, reachable
from `main` through that PR's merge commit; it is the state in which
DOOR's directory is complete and every row in it is closed), so every
path below is recoverable at `git show 284ca7e9889f:work/door/<FILE>`
and `git show 284ca7e9889f:docs/DOOR-EXIT-WALK.md`.

DOOR — the doors whose fix is already written — opened 2026-09-11 in the
eleven-program cut of that day (`docs/WORK-TRACKS-2026-09.md` addendum
3) and closed 2026-09-21 on Ev's in-chat ruling, quoted in the walk.
**Twenty-two rows closed from the directory and twenty-seven passed
through it**: eighteen issues, two units (`S190`, `D306`) and two
rulings. Five left rather than closed — `viewer-cannot-author-a-part-node`
on the program's first day (it failed the one charter test and is on
AUTHOR's slate today) and four in the design-free sweep of 2026-09-20
(`S114` and `patherror-display-renders-float-noise` to PROPS,
`all-census-idiom-forces-the-visit-not-the-update` to CENSUS,
`unit-symbol-proptest-generators-under-cover-with-no-file` to TINT).
The last wave merged four on their own green hosted heads: #2984
(`axis_datum`'s `expected:` comes from `phrase::DATUM_AXIS`), #2985 (the
placer field docs say what the placers accept and yield), #2986
(`PartFault::PartProduct` carries the class beside the sentence), #2989
(`VectorSlot::slots` deleted). Band **3800–3899** was claimed at the cut
and **never drew an ordinal** — the program ran style reviews with no
A/B row for its whole life, so the band is claimed and empty in
`docs/MODEL-AB-LOG.md` as FIX's and VIEW's are. The directory leaves
whole, with no row re-homed at the sweep: the slate was emptied first,
and every residue disclosed inside a closed row already has its own file
on the slate that owns the ground.

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `door` | DOOR — the doors whose fix is already written | 2026-09-21 | this entry; the walk at the sweep SHA; the code it left — `topo::BooleanOp::ALL` and `editor_core::Dimension::ALL` published with their hand-written mirrors retired, `grid_pitch`'s non-finite fallback replaced by a refusing door, `endpoint_params`' conic and LINE arms refusing where the angle is derived, `ders1_in_span` collapsing `loop_area`'s two basis passes, `PartFault::PartProduct` carrying `ProductErrorKind`, `eval::phrase::DATUM_AXIS` as the one home of the datum-axis phrase, and `VectorSlot::slots` deleted |

### What survived, and where

The program's output is the tree, two rule repairs and three practices,
not its directory.

- **The ruling that closed it.** Ev, 2026-09-21: *"can you close `door`?
  it just had its last item close."* It lands hours after FIX's, whose
  reason reads across and reads harder here (Ev, 2026-09-21: *"most of
  those were either mis-filed or should've been done as drive-by
  fixes"*): **DOOR claimed no paths at all**, so every row it ever held
  was on another program's ground by construction. The successor
  practice is not a successor program — file the row on the slate of the
  program whose ground it lands on the day it is found, take it in
  passing when you are already in that file, and let `work/issues/` be
  the last resort `work/README.md` says it is.
- **`docs/prompts/implementer-discipline.md` §6 now agrees with
  `work/README.md`.** DOOR's
  `lane-cross-program-filing-two-binding-docs-conflict` found the two
  binding documents giving a lane opposite instructions about filing on
  another program's slate; Ev ruled for the README on PR #2421 (*"the
  prompt is wrong and should be updated to agree with the README to have
  no reservations about filing directly to other programs"*), against
  the orchestrator's own proposal. §6 now directs a lane to file on the
  owner's slate in the same PR, without permission and without routing.
- **`CLAUDE.md`'s approval rule states a test instead of a list.**
  `readme-ratification-amendments-need-ev` began as a confession — #2387
  retired a ratified `crates/viewer/README.md` kind without Ev — and
  ended as a diagnosis: the git-workflow exception was a hand-written
  enumeration missing a member, naming open design questions and
  `memories/` where retiring a settled clause in a crate README design
  page is neither, and omitting `docs/prompts/` besides. The rule now
  reads *"the exception is text that binds future work rather than
  describing this change"*, with the four homes as what it covers today
  and a sentence saying a new home is covered the day it exists rather
  than the day the line is updated. The rule that governed DOOR had the
  defect DOOR spent that day closing.
- **Route from the instrument, never from a fence read in prose.** The
  same finding FIX's walk records, from a program with no territory at
  all. This one's `keep_out` asserted *"`crates/editor-core/*` is
  DOCM's"* and #2391 measured it false; **three rows named DOCM as their
  owner for a week after DOCM had left the tracker** (sweep 14), where
  `scripts/work.py territory` says `node.rs` and `eval/parts.rs` are
  EDIT's and `mate/member.rs` is MSOLVE's. The clause a reader would
  consult for the answer now leads with the instruction not to consult
  it.
- **Reading a guard is not running it.** #2387's fix for a hand-written
  complete list minted a hand-written census and its PR body asserted
  *"No new census"* two lines after admitting it created one. Executed,
  the hole was plain: add a variant, copy the arm the failing match asks
  for, and the row passes green with the variant absent from `ALL` —
  **the census forces the visit, not the update**, which is now CENSUS's
  `all-census-idiom-forces-the-visit-not-the-update`. Two lanes and an
  orchestrator had read that census and approved it; thirty seconds of
  `rustc` falsified it. Its sibling: **a negative result by grep is a
  claim about the pattern; a negative result by deletion is a fact about
  the tree** (#2989 deleted each sibling member of `VectorSlot` in turn
  and read the compiler).
- **After writing a sweep's blind-spot sentence, go and look for the
  shapes it names.** A regex-shaped sweep missed an instance inside its
  own PR's fence three units running, and in all three the missed shape
  was already written down — in a door's rustdoc, or in the sweep's own
  blind-spot sentence. The rows' own counts were stale nearly every time
  a lane re-derived them (`.slots()` 36 → 41; `Dimension::ALL`'s readers
  filed at five, measured eight of a population of ten; *"the third and
  last copy"* of the datum-axis phrase became a four-pattern
  measurement).
- **Every parallel lane gets its own `git worktree`, and a path a lane
  names is not thereby free.** Two orchestrator errors on 2026-09-12:
  two lanes dispatched into one checkout (shared index and scratchpad,
  one `git add -A` from an unrecoverable commit under merge-only rules),
  and a live lane's worktree deleted because the lane had mentioned it
  as a loose end, discarding six uncommitted edits.
