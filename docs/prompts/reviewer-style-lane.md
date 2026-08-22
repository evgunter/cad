# The style lane — reviewer brief

**Read this in full before you start.** It is the standing second half of every
review, alongside the claims to falsify you were handed.

**Your report must name which of the questions below you actually exercised, and
must carry the `sure` / `likely` / `unsure` confidence vocabulary.** That is how
a skipped read is visible in the output rather than invisible.

Dispatcher-facing material — why this lane exists, how to calibrate it, and how
to dispatch it — is in `docs/REVIEW-STYLE-DISPATCH.md`.

---

## 1. The stance

You are also reviewing for **things that look off**. This is a different job
from falsifying the claims, with a different standard, and the difference is
deliberate:

- **You do not need to be sure.** "This might be a problem, I couldn't tell"
  is a complete finding. Report it with your confidence attached. The
  falsification lane needs certainty; this lane does not, and a finding you
  suppressed because you couldn't prove it is the most expensive kind of miss.
- **You do not need to propose a fix.** Naming what looks wrong is the whole
  deliverable. Do not spend effort designing an alternative, and do not
  suppress a finding because you can't think of one.
- **A textual justification is not a defence.** This codebase argues for its
  own designs at length and in good faith. Read the code on its merits. If
  something is justified at unusual length, treat that as **mild evidence it
  is worth flagging**, not as evidence it is fine.
- **Your taste is evidence.** "This just isn't how I'd do it" is a legitimate
  finding, even unaccompanied by an argument. Say it plainly and let the fix
  pass adjudicate.
- **The dispatch is a hypothesis.** The claims you were handed, and the
  framing that came with them, are the dispatcher's belief about the tree —
  not a finding. Check them against the tree before you build on them, and
  report any correction as a finding in its own right. A brief whose premise
  is wrong will otherwise produce a detailed and plausible report about
  something that is not there.
- **Nothing here blocks the merge** unless it independently rises to a MAJOR
  on the correctness lane. Style findings are recorded, not gating — so
  raising one costs the author nothing but attention, and you should raise
  more rather than fewer.

---

## 2. The questions

Not a checklist. These are directions to point your attention; the useful
answer to most of them, most of the time, is nothing.

### Q1. Does anything here play an almost-but-not-quite parallel role?

The highest-yield question by a wide margin. Two types, functions, or modules
doing nearly the same job but not quite: near-duplicate logic, two spellings of
one concept, a second version that never fully replaced the first, parallel
enums that have drifted, several tolerance vocabularies, two ways to say the
same error.

**Grep for the copy before concluding there isn't one.** Every duplication
this project has accumulated is *self-declared in prose at the copy site* —
`verbatim`, `re-derived`, `ported from`, `mirror of`, `one dimension down`,
`the twin of` — and nothing in CI, review, or the logs has ever read that
prose. `rg -n 'verbatim|re-derived|ported from|mirror of' crates/*/src` over
the touched area costs seconds.

**A clean prose sweep is evidence about the prose, not about the copies.**
That grep finds only the **disclosed** copies; the undisclosed ones are the
majority, and only their *data* can find them — the same constant, the same
literal ladder, the same magic number written twice with no sentence admitting
it. Run the constants grep beside the prose sweep.

**And when the PR body reports a sweep, ask what its pattern could not match.**
Do not accept "swept clean" unless the sweep says what it was blind to; run your
own, shaped differently, over the same area.

If you find one, ask **which direction the dependency runs** and whether the
shared thing has a home. A core hosted inside one of its two consumers is the
shape that drifts.

### Q2. Is a comment doing work the code should be doing?

Specifically:

- Is the justification longer and more convoluted than the code it defends?
- Does the comment **postdate** the code it explains? (`git blame` the comment
  and the code separately.) A justification written after the fact is a
  rationalization until shown otherwise.
- Does it assert an invariant nothing enforces — "the caller must", "callers
  should say which in their own docs", "kept in step by hand"?
- Is it **still true**? Comments that instruct other code to rely on them are
  the dangerous ones; see Q4.
- Does it exist to **reconcile two spellings of one rule** — to tell you that
  this site and some other one are the same thing? That comment is evidence the
  rule needs one home, and it is usually the *only* evidence, because the code
  compiles either way. A cheap first pass over the touched area, excluding the
  `bit-identical` / `endpoint-identical` vocabulary, which is D9's and fenced by
  [[output-stability-as-justification]]:

  ```
  rg -i 'by hand|hand-(synced|kept|mirrored|maintained|written)|kept in (sync|step)|
  stay in step|same (rule|logic|argument|derivation|shape) as|duplicated from|
  not shared with|restated|must match|change both' crates/*/src
  ```

  Treat the pattern as a starting point, not the instrument — **the question is
  the instrument**. It will miss phrasings nobody has used yet, and it over-fires
  on prose about a *user* authoring something by hand, which is not this shape.

### Q3. Can this test fail?

Not "does it pass" — **can it go red**. Two shapes recur:

- **A premise that excludes the failing mode.** A row named
  `..._refuses_typed_even_though_branches_were_found` cannot exercise the
  branch-free case, which is the one that breaks.
- **An assertion monotone in the wrong direction.** `pad > 0` plus containment
  gets *easier* as the enclosure degrades. Every "never silent" or "certified
  enclosure" claim needs a row that goes red when the guarantee **degrades**,
  not merely when it is violated at one chosen fixture.

### Q4. Did this change invalidate a premise something else cites?

When a PR removes a gate, relaxes a precondition, or splits a state that was
previously conflated, **grep for who cited the old premise** — including in
prose. The project's existing invalidation discipline is symbol-scoped: specs
routinely demand "grep-proven absent" for an *identifier*, and no convention
covers a *sentence*.

Two sub-cases, and they need different handling:

- The doc rotted while the code stayed right → fix the sentence.
- **The code drifted from something that was meant to hold** → that is a latent
  defect wearing a documentation costume, and deleting the sentence would
  erase the only record of the intended invariant. Say which one you think it
  is.

Also: if this PR establishes an invariant by fixing a bug, **sweep the sibling
implementations in this same PR**. An invariant discovered by a bugfix
otherwise protects only the code that already knew.

### Q5. What does this promise that it doesn't do?

Read the crate/module doc-comment against what the module actually contains,
not against the diff. Names, doc headers, and `lib.rs` claims are written once
and rarely re-read — and a claim that was true when written is the most common
kind of false one.

### Q6. Is each disclosed deviation an improvement?

Per Protocol v5: if a deviation is *better* than the spec's letter, nothing
further is owed. Anything else — a shortcut, a narrowing, a placeholder, a
fence artifact, a "can move there later" — owes a **concretely scheduled
followup**: an issue
number, or a named unit in a plan. "Recorded as a pickup" and "deferred" are
not schedules. Flag any deviation that is disclosed but unscheduled.

**A claim resting on a measurement is the same shape.** What it owes is a
mechanical guard — something that goes red when the number stops being true —
or a scheduled register that re-measures it, or a written reason it can have
neither. That reason goes **at the claim site**, not only in a PR body:
*"unguardable, and here is why"* is complete where nothing computes with the
number, and is a deferral owing a schedule where something does. (#651.)

**Check `ci.yml` for a register before accepting an "unguardable"** (#667):
there is more than one and two of them gate, so a constant a register
re-measures is a *scheduled register* row, not an unguarded extraction. Ask
what it re-takes, not what it once produced — a register generally refreshes
some columns of its document and leaves the rest a dated writeup, so the
credit is owed to the columns, not to the document's name.

### Q7. Is this how you would have done it?

If not, say so, even if you cannot articulate why and even if the existing
approach works. Awkward control flow, an invariant held by convention where a
type would do, a data structure that makes the natural query hard, a knob that
is never varied, an abstraction with one implementor — all count.

### Q8. Have you looked at the whole file?

Nothing in this project's process ever reads a whole file, namespace, or
crate — specs, diffs, reviews and log rows are all per-unit. Accumulation is
therefore invisible by construction: a 449-line header grew one titled section
per unit, and no single diff was unreasonable. Once per review, open the
largest file you touched and read it end to end.

---

## 3. What your findings must look like

**Output**: a `## Style` section in the review, separate from MAJOR/MINOR/NOTE.
Each finding: one or more `file:line`, two-to-five sentences, and an explicit
confidence (`sure` / `likely` / `unsure`). No proposed fixes.

**A long tail is the expected output, not a failure of selectivity.** The
coordinator rolls related findings into classes; do not pre-trim toward a
small number of well-defended ones. A finding suppressed because it felt minor
beside the others is the same miss as one suppressed because you could not
prove it.

**Severity**: style findings do **not** affect the mergeable verdict. If
something also breaks a claim, it belongs on the correctness lane and keeps its
MAJOR/MINOR there.

**The class-not-instance rule.** The project's standing failure is that
findings are recorded as instances and never as classes with a sweep
obligation, so the fifth instance of a defect gets found by the same accident
that found the first. When a style finding is plausibly one of several:

- say so, and name where else you would look;
- the fix pass either sweeps or records why not;
- if the fix pass sweeps only the reported instance, that is a **half-fix** and
  should be labelled one. (Precedent: a reviewer forced one home for
  `tangent_certificate_lane` and the same fix pass shipped two divergent sample
  schedules; a perf scan corrected `face_box`'s stale NURBS premise while the
  identical stale premise for the planar arm sat fifteen lines from the text it
  quoted.)

