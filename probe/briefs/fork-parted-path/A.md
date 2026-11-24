DESIGNER REPORT: a-parted-path-with-no-turn-is-reported-as-an-empty-flip-crossing

## For Ev

**Recommendation (likely).** A leaf certifies when its CHOICES are the witness's, choice for choice. Its checks are its own business: each one has to come back definite and passing over the leaf, and they are not compared with the witness's. When the choices differ, there are three outcomes:
(a) **a turn**: the two runs take the same path, then one predicate is decided with the other sign. The leaf is refused as `FlipCrossing`, and the turn is named. The type requires it to be named, so an empty flip cannot be built.
(b) **a parting at a pruning query** (the runs handed on different candidates): bisect, the way an undecided leaf does.
(c) **any other parting**: the kernel disagrees with itself. That is a kernel defect, so it fails loud. It is never counted as a mass of the document.

Terms. A *verdict* is one decision the kernel logged: a predicate's name, its definite sign, and its *role*. A *choice* is a decision where another sign changes what gets built or named. A *check* is one where another sign can only make the node refuse. A node's *verdict vector* is its logged verdicts in decision order. The *witness* is the nominal run at f64. A *leaf* is a sub-box of the parameter box, replayed over interval arithmetic. A *mark* is a record of which candidates a pruning query passed on. Neither the mark nor the query is a verdict. *Refused mass* is the probability mass of refused leaves, counted per reason.

**Premise check (sure).** The three states have different meanings and should not be handled as one.
1. *The choices part, with no turn.* The kernel is deterministic, so two runs that logged the same decisions so far make the same next decision. If one run then decides a different predicate, or one log ends early, something the log does not record pushed the runs apart. There are two possible causes:
   - an unrecorded branch. Examples: a heuristic pick (`k_stats::choose`), a hand-written box filter with no mark, a reader that is not logged. These can differ only because the interval box is wide, and a narrower box can fix that.
   - one lane logging work the other lane does not run. That is the plate defect, and no box width fixes it.
   A parting at a mark is the designed case: an interval box covers more space, so its query can pass on candidates the point run pruned away.
2. *The choices match and the check sequences differ.* Neither run turned anything that shapes the body. The difference is which validations ran, and each of them passed in its own run.
3. *The choices match and a check has another passing sign.* This can come from real geometry: a "not zero" check passes on both + and −. The body is the same as the witness's.
DESIGN `[verdicts]` already says that predicate flips are the only places where topology changes. So none of the three states can be the finding that E6's no-flips rule refuses. Today they are counted as `FlipCrossing` mass, which claims the leaf was proven different. That claim is false for all three. The range search already works around this: it reads an empty flip as `FlipUnfounded`, "a defect of the leaf protocol to name".

**What the vector is and what equal vectors claim (likely).** The certificate claims two things:
- the leaf is the witness's build. That is what equal choices show, provided every branch that steers the build is logged or marked.
- the build stands. That follows from the leaf's own definite, passing checks and from every node being Ok, which the driver already checks before it compares anything.
Comparing checks with the witness does not test the document. It tests whether the two lanes of the kernel agree. E12 already settled that kind of question: a question about the kernel goes to a test instrument, and no receipt column counts it. DESIGN `[simple.checks]` says the same. That instrument here is cheap: replay at a zero-width box on the witness point. Any difference in the full log there, checks included, is a kernel bug by construction. Today, putting checks in the gate turns a kernel-parity bug into quiet refused mass (one test went red, in one place) and refuses state 3 under a false name.

**Ratified text I would change (E6).** The bullet "No-flips v1 (adopted round 3; Ev's proposal)" is your text. Its policy stays: no enumerating branches and no analysing the far side. What changes is its premise, that "definite on a different vector" means a flip. That premise is false in the code. Proposed wording, short:
- certified bullet: "Every predicate definite, every node built, and the leaf's choices matching the witness's (its checks are its own) → certified".
- no-flips bullet: "a leaf definite on the witness's path until one choice turns (same predicate, other sign) is refused mass (`FlipCrossing`, the turn named)… A path that parts at a pruning query's candidates is not yet comparable and bisects; one that parts any other way is a kernel defect."
I could not trace who wrote the `FlipEvidence::is_empty` doc ("empty… is honest"): the checkout is shallow. It goes either way.

**Final state.**
- `FlipCrossing` holds a non-empty evidence by type: at least one choice turn or structure flip. `FlipEvidence::is_empty`, the goldened " unnamed", the range's `Ground::Unfounded` / `Unproven::FlipUnfounded` with its viewer string and Python tag, and the viewer's `Floor(FlipCrossing)` arm all go.
- A parting at a mark is `Bisect { deciding: node, unsettled: Unsettled::Parted { site } }`. It splits on what the node reads (E6's rule), becomes `ReadsNoVariable` when the node reads no variable, and becomes `Budget` at the depth limit.
- Any other parting is DESIGN `[failure.taxonomy]` row 4 (`unreachable!`, naming the node and step). That needs every steering branch to be logged or marked first: the `choose` picks, the unmarked box filters the doctail row lists, and readers that steer but are not logged. Until those rows close, these partings bisect as in (b), with the step named.
- The masses mean: certified = proven to be the witness's build. FlipCrossing = proven to leave it at a named decision. Sliver/Budget/ReadsNoVariable = not proven either way. A parting adds no mass column, and a kernel disagreement is not a mass at all.
- What the consumers read: the drive receipt, the range search (`flip()` loses its filter and keeps its debug assert that a turn at the current value is a defect), the viewer and the Python tags (lose `flip_unfounded`). The tour reads only `Budget` and does not change.
- It is reversible: the gate is one comparison in `classify_replay`.

**The alternative, and why it ranks second.** Keep checks in the gate, at the strictest setting, and name a turned check as a flip. That refuses state-3 leaves whose body is the witness's, and the range search would draw a boundary there where nothing changes. Its one gain is defence against a role stamped wrongly at a call site. That is a kernel bug, and the zero-width parity test catches it better.

**What it costs (likely).** A genuinely new intersection reached only through a pruning query, a pair the witness pruned, was counted as FlipCrossing today by luck. It would now bisect down to `Budget`. That is honest, since it is not proven, but it names less and costs depth-many replays.

**The ruling already made (unlogged lifted premise pass, F48): right, and at the right layer (sure).** DESIGN `[verdicts.consumers]` says "a lane choice records nothing". It should be read as the rule for every lane-specific pass, not as a one-off. With checks out of the gate, it no longer carries any weight for certification, only for log parity and K.

## For the orchestrator

- **The brief's frame.** "A narrower box may re-align" is true for mark partings only. Kernel lane disparity never re-aligns. The item merges the three kinds of `Parting` (Mark, Step, Ended), and they mean different things.
- **Assumed, not verified:** that some check accepts two signs, so that state 3 can come from geometry. I did not find a call site. If no check ever does, state 3 is kernel-only, and the gate change can rest on states 1–2 alone. Also not verified: that mark partings ever reach a differing vector on today's corpus. The `VerdictVector` doc says extra candidates "decide nothing", which may be untrue.
- **Off the question (not filed):** `classify_replay`'s step (i) sends every typed node failure to bisect (`Unsettled::Failed`), including a check that refuses definitely over the leaf. A leaf that is definitely invalid ends as `Budget` after depth-many replays. E6 has no class for "definitely invalid over the leaf" (`Infeasible` is solver-only). It is worth its own row if you agree.
- **Sequencing:** build the zero-width parity test first. It is the instrument that would have caught PR 7344's escape on every curved row, and the `unreachable!` arm rests on the doctail marking rows closing.
- **Process:** this changes the ratified E6 bullets, so it is an `[ev]` PR.
