# VSEAM — the viewer's seams and session vocabulary (plan)

**STATUS: OPEN (2026-09-17).** Opened in VIEW's re-scope, on fourteen
rows that arrived by `git mv` with their bodies unchanged. Live state
is `work/vseam/log.md`'s tail and the item files beside this plan,
never this file.

Branch prefix (the #396 convention): **`vseam/`** — unit branches
`vseam/<unit>-<slug>`, orchestrator branch `vseam/orchestrator`.
Away-channel tag `(VSEAM orchestrator)`. A/B ordinal band
**VSEAM = 5400–5499**.

## Charter

**Every row here is about something the viewer HOLDS on behalf of the
document, and about the boundary that is supposed to own it.** An op it
can author, a spec it lowers into the kernel's vocabulary, a field
derived from the document, an answer cached across a generation, work
handed to a worker thread, a gesture carried across frames. In every
row the thing held is real and the holding has no named boundary:
nothing types it, nothing says when it goes stale or who resets it, and
nothing alarms when the document's own vocabulary moves underneath it.

Read off the rows, the four shapes that holding takes:

- **The op vocabulary** — what the viewer may author at all, and
  whether one op is one act. `no-persistent-setplacement-session-op` (a
  free move cannot be committed as a document edit),
  `session-save-is-two-acts`, `revolve-tool-unreachable-no-axisinplane-form`
  (no form authors the datum the tool needs, so the op is unreachable),
  `patternrulespec-is-a-partial-mirror-with-no-growth-alarm` (the
  viewer's spec mirrors two of the kernel enum's three arms and nothing
  tells it when the kernel grows).
- **Document-derived state** — what the session must forget when the
  document changes. `viewerapp-document-derived-state-has-no-boundary`
  (fields reset by hand at one door with nothing at the declaration
  naming the set), `new-document-owes-the-reframe-open-gets`,
  `projection-fault-has-no-sweeper` (a fault that can go permanently
  stale, and two new fields with no row).
- **The seam and its cache** — what crosses to a worker and what comes
  back. `evalservice-coalescing-rule-is-prose-no-implementor-is-held-to`
  (at-most-one-outstanding lives in module prose and no trait clause
  holds an implementor to it),
  `index-request-and-index-inputs-are-one-concept-twice` (the same five
  fields owned and borrowed, with nothing saying they are one concept),
  `ui-thread-work-after-the-index-seam`,
  `refused-a5-gate-eats-the-body-the-fit-then-regathers` (the landing's
  body is held, consumed by a refusing gate, and gathered again).
- **State carried across frames** —
  `the-two-drags-name-their-gestures-in-two-shapes`.

**The test that separates this program from its three siblings.** A
VSEAM row is about something that OUTLIVES the frame that made it, and
its fix names a boundary: a type, a door, a stated invariant, an
exhaustiveness alarm. That is false of VNEWS, whose rows live and die
inside one frame's news; false of VGEOM, whose rows are a value wrong
at one call rather than stale across many; and false of VDOC, whose
fixes change no viewer behaviour.

Applying it the other way, honestly: **one row on this slate does not
meet that test.** `adjacent-same-typed-arguments-are-the-same-swap` is
a swap hazard in a call vocabulary — roughly a dozen `fn` headers in
`crates/viewer/src`, the worst two `transform_node` and `add_transform`
relaying two `[Expr; 3]` positionally over two hops. It is not news,
not a number, and not a claim about the tree; it is here because its
two worst instances are this program's authoring doors and because the
other three charters are each false of it. **It was placed by
elimination and this sentence is the record of that** — a better home
may exist and the row should move to it rather than be built here out
of inertia.

## Ev's requests — high priority

Filed 2026-09-17 from Ev's own list of UI nits, and **ahead of the
order below**: Ev asked for these directly, so they are taken before
anything else on this slate. Each row carries Ev's note verbatim.

None open: both rows filed here landed on 2026-09-19 (PR 2858, PR 2862).

## Order

E-first, and the three sibling successors are file-disjoint from this
one except at the shared files `program.md`'s `keep_out` names, so this
program is dispatchable in parallel with VNEWS and VGEOM from its
opening day.

1. **The seam's two vocabulary rows first** —
   `index-request-and-index-inputs-are-one-concept-twice` and
   `evalservice-coalescing-rule-is-prose-no-implementor-is-held-to`.
   Both make a written rule structural rather than prose, and both are
   preconditions for reasoning about the third seam row below.
2. **`ui-thread-work-after-the-index-seam`** — three unbounded steps
   still inside the frame after the pick index moved off it. **Its own
   numbers must be re-derived before it is dispatched**: the VIEW
   register records that this row's `~5 µs per triangle` was a ratio
   across two different δ and that the honest figures are 60 ns and
   0.09× steady. The row's text still carries the contradiction.
   Crosses `pickindex.rs`, which is VGEOM's too; announce.
3. **Document-derived state, as one conversation** —
   `viewerapp-document-derived-state-has-no-boundary` decides the shape
   that `new-document-owes-the-reframe-open-gets` and
   `projection-fault-has-no-sweeper` then follow. Taking the two
   particular rows first re-mints the general one.
4. **The op vocabulary** — `session-save-is-two-acts` and
   `no-persistent-setplacement-session-op` are both additions to the
   replay/undo vocabulary and both take a correctness arm for the
   round trip. `revolve-tool-unreachable-no-axisinplane-form` is the
   reachability row and the register's rule applies to it directly:
   a reachability claim is about what a USER can cause, so enumerate
   the input modalities before the code paths.
5. **`patternrulespec-is-a-partial-mirror-with-no-growth-alarm`** —
   the alarm, not the third arm. The kernel enum is EDIT's ground and
   the alarm lives on this side.
6. **`refused-a5-gate-eats-the-body-the-fit-then-regathers`** — its
   defect site is `crates/editor-core/src/assembly.rs`, which is
   EDIT's. This is a hand-off to be filed and negotiated, not a diff
   from here; what this program owns is `DocSession::landed_body`'s
   side of it.
7. **`the-two-drags-name-their-gestures-in-two-shapes`** — **the wait
   is over and the row is closed** (PR #2965). It was written as
   waiting on `two-hand-written-copies-of-the-g1-gesture-machine`,
   which rewrites the machine both drags copy; that row's PR (#2672)
   merged on 2026-09-15 and `crates/viewer/src/g1.rs` has been on
   `main` since. The wait outlived its trigger by six days because the
   row carried `status: review` after its PR merged — see §Inbound.
8. **Held.** `pick-priority-filter-vocabulary` is **deferred**, ratified
   by `crates/viewer/GUI-DESIGN.md` GQ7; it is not work and stays
   deferred until that ruling moves.
   `adjacent-same-typed-arguments-are-the-same-swap` is not scheduled:
   see the charter's last paragraph, and settle its home before its
   shape.

## Inbound

**This section was wrong when it was written and is corrected here.**
It said *"Four rows in `review` are this program's and are still on
VIEW's slate, because their lanes are in flight and a rename mid-review
is a merge conflict for nothing"* — and then listed **five**. None of
the five lanes was in flight: every one of these PRs had already merged
when the cut (#2806) wrote that sentence, so the rows kept a `review`
status their PRs had retired, and this program recorded a wait on one
of them (Order item 7). All five are **closed** on `main` since
PR #2976, which closed them with their merge dates. They arrive here
with VIEW's exit walk as ordinary closed rows, not as in-flight work:
`the-two-seams-are-hand-maintained-twins` (#2666),
`the-picture-key-never-became-a-type` (#2670),
`two-hand-written-copies-of-the-g1-gesture-machine` (#2672),
`a-pick-over-a-stale-picture-answers-about-a-picture-nobody-can-see`
(#2662), `id-query-is-keyed-on-the-generation-not-on-the-picture`
(#2622). All five are this charter's subject exactly — a cached answer,
a key, a twin machine, a stale picture — and item 7 of the Order waited
on one of them until PR #2965 closed it.

`a-dead-seam-worker-reads-as-an-ordinary-idle-state` is also this
charter's subject and is **not** inbound: its PR #2762 was parked on a
ruling from Ev. **That ruling landed** — a crashed seam worker panics
(Ev, in-chat, 2026-09-17) — and #2762 merged carrying it, with almost
all of the badge vocabulary deleted rather than shipped, because a
crashed worker takes the process down and so cannot be a state the
chrome describes. The row is closed on VIEW's slate. Nothing here
touches it.

## The register

**`work/view/plan.md`'s rule register binds every lane dispatched from
this program, inherited BY REFERENCE and not copied.** Read it in full
before writing a dispatch.

The reason it is not copied is the register's own: a claim fixed in one
place and stale in another contradicts itself, and four copies of a
register that is re-derived every wave guarantee four divergent copies
within a week. The register is also evidence — every rule in it is a
named failure at a named PR — and a copy detached from the program that
paid for it reads as a rule without its receipt.

**What that costs, said plainly:** `work/view/plan.md` goes when VIEW's
directory goes at its exit walk, and this reference dangles that day.
The register's permanent home is
`work/view/the-lane-register-has-no-home-after-views-directory-goes`,
open on VIEW's slate, and it is a precondition of VIEW's exit walk
rather than a follow-up to it. This section re-points when it lands.

## Review posture

**Inherited from VIEW unchanged (Ev, in-chat, 2026-09-04, reaffirmed
2026-09-04 evening; `docs/MODEL-AB-LOG.md`'s roster line).** No A/B
duals, no row in `docs/MODEL-AB-LOG.md`; the band stays claimed and
empty. The default is a style review against
`docs/prompts/reviewer-style-lane.md`, with a correctness arm added
only where a unit's failure mode is a confident wrong answer rather
than a refusal. A new op entering the replay/undo vocabulary takes the
arm for its round trip; so does anything that changes what crosses the
seam.

## Exit shape

Every row above landed or ruled out, every boundary this program names
stated once in `crates/viewer/README.md` beside the code it governs,
and the `[ev]` GQ7 residue either ruled or still deferred with the
citation intact. The walk convention applies; residue re-homes per
`work/README.md`.
