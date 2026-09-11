# DOOR log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/door/plan.md`.

## Opened (2026-09-11)

Opened in the tracker cut of 2026-09-11 (Ev's direction, in-chat;
`docs/WORK-TRACKS-2026-09.md` addendum 3). Eleven rows moved in by
`git mv`, each with a `## Re-homed` record: five from `work/issues/`,
six from `work/code-quality/`.

This is the cut's cheapest track by a wide margin — seven `E` rows of the
nineteen the whole cut found — and that is the reason it exists as a
track rather than as eleven routings: the board had no way to say so.

No branch exists yet. First look: `S190`, whose park cites a closed
trigger.

## Read against the tree before the first dispatch (2026-09-11)

Orchestrator seated. Posture confirmed by Ev in-chat: **no A/B row**,
light style reviews by default, full correctness review only where a row
has real risk of being wrong. `plan.md`'s **Review posture** names the
four that qualify and why.

Every row was read against the tree before anything was dispatched,
which the charter's own test makes worth doing: a row asserting its fix
is written is a claim about the tree, and three of eleven did not
survive it.

**`S190` was already fixed — closed, not dispatched.** The plan called
it the first thing to look at, on its park citing a closed trigger; the
answer is that both halves landed and nothing was owed. The kernel half
gives `CensusUnsupported` a `CensusSubject` whose `FacePair` `PartialEq`
is **written rather than derived**, so the arena-order question the
finding was about is closed in the type. The consumption half resolves
through `assembly.rs`'s `by_pair`. And the row's load-bearing claim —
that no single-declaration fixture could tell a face-width lookup from a
pair lookup, so the defect had no test that could go red — is answered
by `a_face_in_two_declarations_answers_each_pair_to_its_own_mate`,
asserting in **both** arena orders. The row's file carries the citations.
Closing it also cleared the `work.py lint` warning it had been throwing
since #855 closed.

**`viewer-cannot-author-a-part-node` went to CHROME.** It is the row
that failed the charter test, and it failed it in the way worth
recording: it names four files, which reads like a diff, and does not
name what the `AddPart` op takes — which is the whole of the work. A
file list is not a fix. CHROME owns the territory and closed the sibling
gap (`placed-union-has-no-session-op`, PR 1762), which is both where it
belongs and the shape to copy.

**`viewer-pathverb-all-hand-written-seventeen` lost half its premise to
VIEW's `const ALL` unit.** `PathVerb` moved to `forms.rs` and is
declared through `vocabulary!`, so `ALL` is projected and the
hand-written seventeen the title names is gone. What survives is whole
and is the half that mattered: `PathVerb::of` is exhaustive over the
**viewer's own** `PathStep`, so the projection and the match hold the
viewer to itself and nothing relates either to `profile`'s nineteen
verbs. `CircleSplit` still has no viewer spelling. The row's cheaper fix
and its more expensive one have swapped places, which is the kind of
thing a stale premise does quietly.

**Two rows claimed from VIEW, before any code.** DOOR opened holding two
instances of a class whose head sat on VIEW's slate
(`hand-maintained-mirrors-of-a-kernel-enum-are-unforced`), and the fix
that closes either instance closes the head too — one publication of
`topo::BooleanOp::ALL` retires `forms::BOOLEAN_OPS` and the
`kernel_wire` copy together. Landing that from here while the head sat
elsewhere would have closed another program's row by side effect. Ev's
direction in-chat was to take the items and to take them **out of VIEW
first rather than after the work**, which is the order `work/README.md`
prescribes and the order followed here; VIEW was told.
`dimension-radio-row-is-an-unforced-mirror-of-a-kernel-enum` came with
the head on that row's own instruction (whoever takes either takes
both). Five stale `work/view/…` path citations were repointed, one of
them in `crates/viewer/src/forms.rs`.

**What did not come with them:** `forms::MATE_PRIMITIVES`, which the
head names and the site documents as riding it. It is **deliberately
partial** — `MatePrimitive` has a fourth variant the kernel represents
so it can refuse it — so it is a mirror no mechanism should force. The
class's PR argues that at the site rather than projecting it.

**The mirror class does not get one PR per row**, and this is the
program's one ruled departure from one-PR-one-row: two PRs, one per
kernel enum published, each naming the rows it closes. The posture
exists so a lane does not widen into a second row's file; here the
second row is the same file and the same diff, and splitting would mean
landing half a projection. Ruled in `plan.md`'s **Order** and nowhere
else — a lane does not mint a second exception for itself.

**`patherror-display-renders-float-noise` grew a correction before it
grew a sweep.** Ev's caution in-chat was to round at a principled point
rather than one that could obscure real geometry, and reading the helper
showed the caution was understated: `path::num` rounds at a *relative*
1e-9 while D4's ε is ~1e-9 m **absolute**, so at metre scale the
rounding point sits exactly on ε_precision and above it is coarser — and
these sentences mostly report margins *against* ε. Propagating that
constant to every arm first, then discovering this, was the expensive
order available.

**The rounding point needed both grids, and finding out why took the
call sites.** Ev's proposal was a fixed absolute quantum one decade
below ε, and above ε it is exactly right: the kernel cannot distinguish
finer, so further digits are noise a reader cannot act on. It fails
below ε, and it fails on the call sites that dominate this family —
`JunctionTangent` and its siblings report a `margin` that is *below the
threshold by construction*, since being below it is what makes the
junction tangent, so a 1e-10 grid renders the only number in the
sentence as `0 m`. The helper's own doc had already made that argument
correctly (*"never to the nearest nanometre and never to `0`"*) and then
undercut it with the constant it picked, which is a tidy example of a
justification outliving the number it defends. So: the finer of a 1e-10
absolute floor and a relative noise band, with the relative arm carrying
the sub-ε payloads.

**One call the orchestrator made on top of that**: the floor reads the
compile-time `DEFAULT_EPS`, never `Tolerance::eps()`. ε is a live
process value and a code-tier run gates {default, 1e-6, 1e-12}; reading
it would make every rendered refusal a function of process
configuration and every string assertion in the tree eps-sensitive
across three rows. That is the difference between this row being an `M`
and an `H`.

**No guard is owed on `run-on-whitespace`** (Ev, in-chat): fixing the
five literals is the row. A guard rides along only if it is trivial and
costless beside `error_display.rs`'s existing predicate; if it needs its
own corpus or its own pattern language it is neither, and the lane says
so rather than growing the row.

**The slate stands at eleven rows, eight `E` and three `M`** — the same
count it opened with, by coincidence and not by conservation. No branch
exists yet; dispatch follows this entry, viewer rows first.
