# SMELL-SCAN Track H — orchestrator log

**Constituted 2026-08-21.** Track H is the second scan's *certification
substrate* track: `crates/geom-core/` and `crates/geom/` — the scalar,
interval and certification layer that everything else measures *with*.
**§H of `docs/SMELL-SCAN-2026-08.md` remains the schedule** — this file
is the execution record: rulings, lane state, review outcomes and
incidents. **Live status is here and in §H, never in `memories/`.**

**This track runs entirely outside the model A/B experiment.** No
Fable/Opus pairing, no ordinal, no row in `docs/MODEL-AB-LOG.md` —
**nothing on this track reads or edits that file.** The experiment is
paused on a model limit (Evan, 2026-08-21); the cheapest guarantee that
the pause stays clean is that this track never touches it. A lane that
believes it needs to is wrong and should ask.

**Branch prefix:** `smellh/` for units; the orchestrator sits on
`smellh/orchestrator`. Away-channel role tag: `(TRACK H orchestrator)`.

**Number block:** **`D140`–`D159` and `S210`–`S229`**, published in §H on
constitution rather than claimed in a lane message. **Re-derive after
every merge** — per Track G's `G-R13`, a block cannot protect against a
number arriving from another track, only re-checking can.

**Neighbours.** Track I (`props/`, `mesh/`, `census.rs`) and Track G
(`demos/`, `profile/`, `interval-transcendentals/`, `sweep/src/` outside
`fillet/`) have live ground. **The Track I edge is a dependency, not a
file overlap**: `geom-brep` and `mesh` build on `geom`/`geom-core`, so a
Track H change to a public signature ripples into Track I's builds.
**Publish such changes in the PR body**; neither track gates the other.

---

## Review policy for this track

Not the full orchestrator protocol. Per Evan, 2026-08-21, and inherited
unchanged from Track F:

- **Style review on every unit** — `docs/prompts/reviewer-style-lane.md`,
  dispatched **by path** (read it once; never paste it), with the
  per-lane emphasis a dispatch owes (`docs/REVIEW-STYLE-DISPATCH.md`).
  On top of the standing brief, every Track H style review answers two
  questions the brief does not:
  1. Is the finding's **original** stylistic problem now *completely*
     gone — not narrowed, not relocated, not half-closed in a way that
     reads as closed (§C13)?
  2. Was it closed in the **best** way available, or merely in a way
     that compiles?
- **Adversarial review only where a wrong answer is reachable** — the
  criterion is Evan's (`SMELL-C-LOG` C-R12): *complex enough that there
  is a significant chance the change introduces a regression CI will not
  catch*. That is narrower than "this code is load-bearing". §H's
  constitution names **H5's `Dual` sub-lane** at minimum; this track adds
  **H1**, **H2** and **H7**, each for a reason recorded in its roster row.

**Why this track needs the criterion argued rather than inherited.**
Track F's subject was guards, so its units' failure mode was a guard
firing on something true. **Track H's subject is the certification
substrate**, and its units' failure mode is one layer worse: a door here
that starts refusing — or keeps laundering — propagates into every
consumer that measures with it, and `geom-brep`/`mesh` are *another
track's* builds. A row is adversarial here when the change can move a
**certificate**, not merely when it touches an important file.

## What a lane does with what it finds

Three destinations, and a lane picks by the finding's kind, not by its size:

- **A new style finding** → recorded in `docs/SMELL-SCAN-2026-08.md`, in
  the lane's own PR, as a new numbered finding or as a member of an
  existing roll-up.
- **A finding about the kernel's logic** → a **GitHub issue**, signed,
  never a smell-doc row. Track H fixes style in the substrate; a logic
  defect is someone else's lane and needs a register that executes.
  Precedents live: **#723** (wrong certified volume) and **#862** (the
  cylinder box's logic half) are exactly this routing, and both were
  struck out of a style row rather than absorbed into one.
- **An important design question** → a **PR asking Evan**, per
  `memories/git-workflow.md` — the doc edited to state the question,
  updated in place with the answer. Never a comment on a merged PR.

## Recording convention

**The landing PR carries its own record**, so the concurrent
orchestrators never read a document that is behind the tree. Each unit
makes two edits to `docs/SMELL-SCAN-2026-08.md` in its own PR:

1. the finding's heading becomes `## SNN. FIXED by #NNN — …`, and its
   **original problem statement is replaced** by the record of what was
   done. Version control keeps the original; leaving it in place makes a
   closed finding read as open.
2. the unit's **row leaves §H's table**, per the *live rows only* rule —
   **but a row that carries evidence the finding's record does not is
   relocated, not deleted** (Track F's F-R11). Schedule rows accumulate
   re-derivations, counts and narratives that were never folded back into
   the finding; striking such a row silently destroys the best evidence
   for the thing being closed. Before deleting a row, read it for
   anything the finding's own text does not say, and move that into the
   record.

**Conflicts in that file are expected and survivable**, and there are
live orchestrators on Tracks G and I editing it. Resolve by merging
`origin/main`; **merging before CI is green is permitted when a previous
run on the same head was green and the only conflict was in the smell
doc** (Evan, 2026-08-21).

**Do not write the Record-edits section until the record edits are in
your diff** (Track F's F-a incident). A section describing work is not
evidence the work happened, and the sections least likely to be checked
are the procedural ones. The orchestrator reads the PR's **file list**
against its body before dispatching the review.

---

## Rulings

### H-R1. `S110(f)` is already closed, so `H8` is one member, not two

**§H's `H8` cites `S110(f)` and `S116(b)`. `S110(f)` was CLOSED by #790**
— the `d8_knot_queries_adversarial.rs` row now asserts *where* the spans
were placed, with a receipt in both failure directions. The citation
entered §H from the frozen table's `C-g/C-l/C-q` row, which was written
before #790 landed and was transcribed rather than re-derived when Track F
constituted this track.

**Which makes it the very error §H's own constitution warns about**, one
paragraph below: *a block cannot protect against a number arriving from
another track, only re-checking can*. The same is true of a **citation**.
Track H re-derives every row's members against the tree before dispatching
it, not against §H's transcription of them.

### H-R2. `H8` dissolves into `H2`; there is no separate roll-up lane

With `S110(f)` closed, `H8`'s whole content is **`S116(b)`** — three
modules named `projection`, two named `boxes`, two named `nurbs` in
`crates/geom/src/`, and `azimuth::frame`'s two-`Vec3<T>` return that a
transposed destructure compiles straight through.

**That is the same crate-merge residue `H2` already is.** `S116(b)`'s
`azimuth` half is literally `S102`'s subject — `surfaces.rs:26-30`'s *"The
shared helper"* bullet spelling the `radial`/`tangential` formula without
naming `crate::azimuth`. Two lanes editing `geom/src/surfaces.rs` and the
`azimuth` header for one merge's naming residue is a conflict manufactured
by the schedule, not by the code.

**So `H2` takes `S99`–`S103` **and** `S116(b)`**, and `H8` leaves the
table. The frozen table's *"S99–S103 are one merge's residue and want ONE
lane, not five rows"* is the argument; this ruling only observes that
`S116(b)` is a sixth member of that same residue.

### H-R3. The doors tighten to `CertifiedBounds`; the passes keep their lanes

**Evan, 2026-08-21, answering #867: *"tightening to `CertifiedBounds`
works at least for now."*** *"At least for now"* is part of the ruling
and is recorded as such — it closes the seams, it does not settle
whether any of them should ever be differentiable.

**The mechanism was already in the tree and nobody was using it where it
counts.** #643 split `Bounds` from `CertifiedEnclosure`;
`CertifiedEnclosure` is implemented for exactly `f64`, `Interval`,
`RingInterval` and `Probe` and **never for `Dual`**; and `real.rs:800`
gives the pair a sole-bound spelling, `pub trait CertifiedBounds: Bounds
+ CertifiedEnclosure {}`, blanket-implemented. So a seam that wants duals
out does not need anything built — it needs **a bound that does not type
check**.

**What the ruling does not do: it does not delete the four lane traits.**
The conversation opened on whether it should, and the evidence went the
other way.

> `CertifiedBounds` refuses at the **function**. A lane trait refuses at
> a **sub-operation inside a function that has non-certifying work to
> do**. No bound on a whole function can say *"this arm needs
> certification, the rest does not."*

All four lane traits gate **mixed passes** — `validate_geometric` /
`validate_pseudomanifold` (`PropsQuadLane`), `validate_pcurves`
(`PcurveFittedLane`), `census_and_certify` (`ChartRegionLane`),
`Body<T>`'s euler impl (`EdgeNurbsLane`) — and the decisive site is
`topo/tests/geometric_cube.rs:236`, which calls `validate_geometric` at
`Dual64` and asserts it **succeeds**: the quadrature arm declines
internally through the refusing impl while the rest genuinely validates,
after which every certificate's value channel is compared **bitwise** to
the `f64` build. `sweep/tests/m5_pr11_quad_props.rs`'s
`dual_lane_keeps_the_closed_form_refusal` is the same shape deliberately.
Bounding those passes on `CertifiedBounds` would not harden them; it
would delete `Body<Dual64>`'s ability to go through a validation pass.

**Why the steelman read the other way.** S44's *"What this does NOT
settle"* priced the four lane traits by asking *does deleting break the
guarantee* — and answered no for three of them, since their doors carry
`CertifiedEnclosure` already. That is true and is the wrong question.
The question that decides it is *does deleting remove a capability*, and
the answer is yes at all four. **A redundant guarantee and a redundant
trait are not the same finding**, and C7's collapse was scoped against
the first.

### H-R4. The door inventory, and Track H takes two sites outside its ground

**A door is a function whose whole job certifies.** Three are currently
`Decide + Bounds` and admit a dual:

| Door | Where | Owner |
|---|---|---|
| `project` / `project_seed` / `project_from_seed` | `geom/src/projection.rs`, `curves/projection.rs`, `surfaces/projection.rs` | **Track H** |
| `chart_region_overlap` | `topo/src/chart_region.rs:355` — `pub`, re-exported at `lib.rs:287`; an external caller instantiates it at a dual **without `ChartRegionLane` being consulted at all** | unowned |
| `fillet_edges`, `ring_clearance`, `run_battery` | `sweep/src/fillet/{build,surgery,battery}.rs` — S90's seam | unowned |

**The `geom` doors are not merely undocumented at a dual, they are
wrong there** — lane H-d filed **#874**: foot and orthogonality come back
as partials at a *frozen* `f64` foot parameter. The tightening is that
issue's structural half.

**Track H takes all three, including the two outside its scope.**
`topo/src/chart_region.rs` belongs to no track (Track I's `topo` ground
is `census.rs` only), and `sweep/src/fillet/` is explicitly outside Track
G's *"`sweep/src/` outside `fillet/`"*. **The reason to take them
together is that the ruling is one**: split across three lanes it would
be argued three times, and the second and third arguments would be
transcriptions — which is how `H8` acquired a stale citation (H-R1).
Published here and in the lane's PR so Tracks G and I can object.

**Verified before scheduling, because it would have been the blocker:**
`chart_region_overlap` is **not** called from `census_and_certify` — its
only callers are `chart_region_r2_probes.rs` and one `sweep` test — so
tightening the door does not propagate a `CertifiedBounds` bound up into
the census pass and does not disturb H-R3. **The same check is owed at
the `geom` doors and is the lane's first task**, since a door called from
inside a mixed pass cannot be tightened without breaking the pass.

### H-R5. `S210` is a Track H row, scheduled AFTER `H-g` — and a hypothesis it should test first

Lane H-d closed `S88` and minted **`S210`** for the part it could not
close: **nothing watches this class.** `scripts/gates/bounds-allowlist.sh`
**cannot** see a sole `T: Bounds` bound — that is its planted
*must-not-fire* case, because firing would red every certification file in
`geom` and `geom-brep`. So the sole bound is the form the rule
**prescribes**, and no instrument watches the form the rule prescribes.
H-d costed two closures — a whole-tree trait walk plus grep, accurate only
at its merge base; or a **resolved**-bound-set query via a `rustc` driver
or `rust-analyzer`, which stays true — and **minted no schedule row**,
handing placement here. That was right: the class spans Track G's ground,
Track C's abandoned ground, Track I's, and `bvh/`, and no lane should
place a row across four tracks from inside one.

**It is Track H's**, because the rule it enforces has exactly two homes and
both are in this track's ground: `real.rs`'s `Bounds` scope rule and
`scripts/gates/bounds-allowlist.sh`. **It is scheduled after `H-g`**,
because H-g changes the population it would measure: a door tightened to
`CertifiedBounds` is structurally safe and drops out of the class, so
building the instrument first would census a set that is about to shrink.

**A hypothesis the taker should test before paying for a `rustc` driver —
recorded as a hypothesis, not a finding, because I have not checked it.**
H-d's distinction is *terminal* versus *fed-back* bracket reads. There may
be a cheaper type-level proxy: the defect in `project` is not that it
decides with a bracket, it is that it returns a **`T`-valued output whose
tangent is wrong** — the foot, computed through a frozen `f64` parameter.
By contrast `project_seed` returns `f64`, the box constructors return
non-generic boxes, and a door that merely *decides* is safe by delegation
(a dual takes the value channel's branch, which is the base scalar's).
**If that holds, the signal is "sole `T: Bounds` **and** `T` in the return
type", which a grep can approximate and a trait walk can confirm** — far
short of resolved bound sets.

**RETRACTED the same day — #875's style reviewer killed it, which is what
it was recorded for.** The signal has good precision and useless recall:
it fires on all six true positives (`NurbsCurve{2,3}` and `NurbsSurface`'s
`project`/`project_from_seed`, all returning `Projection<T>`) and stays
silent on the five box constructors — **and it is silent on
`projection::mid<T: Bounds>(v: T) -> f64`, the freeze site, the function
this entire finding is about.** Also silent on `project_seed`, on
`fillet_select.rs:169`'s `nearest_joint<T: Bounds>(…) -> usize`, and on
`ssi/certify.rs`'s `exact<T: Bounds>(v: T) -> Option<f64>`.

**The mechanism is the reverse of what I guessed.** The defect is a door
that **strips** `T` — returns an `f64` or an index — whose result *the
caller* then feeds back. A door returning `T` is only where the damage
becomes **visible**. So my signal finds symptoms and misses causes, and
**its canonical false negative sits inside the very census that would have
been used to validate it** — which is how a plausible screen gets adopted.

**The better screen was already in the tree**, and it is `real.rs:461-463`'s
**payloads versus selections** split: *"sole `T: Bounds`, returns a non-`T`
scalar or index, and the call site uses the result as an **input** rather
than as a payload."* Whoever takes `S210` starts there and not from
scratch.

**Kept rather than deleted, because the retraction is the useful part.**
The hypothesis was cheap, wrong, and killed by the first person who
looked — which is the outcome recording it as a hypothesis was for. Had it
gone into a brief as an instruction it would have arrived carrying the
orchestrator's authority and been one commit from a ratified doc, which is
the failure `docs/REVIEW-STYLE-DISPATCH.md` §3 names and which this track
has now demonstrated on itself.

**`S211` needs no ruling** — H-d minted and fixed it in the same PR: two
`geom` box modules called themselves *"allowlisted `Bounds` seams"* while
being on no list and unable to be on one. Its third member,
`bvh/src/lib.rs:56-61`, is in no track's ground and is named in the record
rather than taken.

### H-R7. The eval seam: a door beneath `evaluate` is not tightenable alone

**Found by H-g, 2026-08-21, after CI falsified its own caller analysis.**
`sweep::fillet::build::fillet_edges` **is** reachable from a mixed pass:

```
evaluate<T: Decide + ContentBits + Bounds + Send + Sync + PropsQuadLane>   (pub)
  → run_op<same>  →  wire.rs:110 dispatch  →  wire_fillet<T: Decide + Bounds>
      → fillet_edges
```

So tightening the door drags `CertifiedBounds` onto `wire_fillet`,
`run_op` and **`evaluate`'s public bound** — tightening a **pass**, over
**one** node kind, foreclosing E4's dual-through-`evaluate` path for **all**
of them. That is the thing H-R3 forbids, reached from the direction H-R3
licenses.

**`real.rs:470-477` named this caller and we both read past it.** It says
*"the one production caller (`editor_core::eval`'s fillet wiring)"* and
then argues from **reachability** (`ContentBits` has no `Dual` impl).
H-R3's question is about the **bound**, and reachability does not answer
it. **The paragraph H-g deleted this morning as discharged was right about
the eval seam and wrong about nothing** — the bound discharges the
obligation at the **public surface** and does not discharge it at the eval
seam, and *the paragraph never distinguished the two*. That is the finding,
and it is worth more than either half.

**It is not a fillet quirk.** H-g answered `real.rs:394`'s
#643-completeness question **NO** on this rule and found the same shape at
`Separation::{of,certify,image}` — non-generic returns, so every read is
terminal, **and** their one production caller is `wire_placed_union`
beneath the same `evaluate`. **Two of two doors examined beneath the eval
seam behave this way.**

**The standing rule, and it amends H-R4's inventory.** A door's
tightenability has a second question the inventory did not ask:

1. Is the door's own bracket read terminal or fed back? *(H-R4)*
2. **Is the door reachable from `evaluate` — or from any pass whose bound
   is a lane trait rather than a scalar capability?** If yes, tightening
   the door is a decision about the **pass**, and belongs to whoever owns
   the pass's scalar policy.

`chart_region_overlap` and `geom`'s `::project`/`::project_from_seed`
**pass both tests** — re-checked without truncation, the projection doors'
only non-test callers already carry `CertifiedEnclosure`
(`edge_nurbs.rs:283`, `ssi/certify.rs:433`), so neither moves. **Those
proceed. The fillet seam is with Evan.**

**Method note, recorded because the failure is generic.** H-g's first
caller analysis piped a repo-wide grep through `| head -30` and the
thirtieth line was not the last; it reported *"no in-repo caller moves"*
on a truncated list, and **CI caught it rather than the lane**. A
truncating pager on an enumeration that is the evidence for a completeness
claim is now named in every Track H brief. The lane self-reported it
before anything else, which is why this row is a ruling and not an
incident.

---

## Sequencing

**Three waves, ordered by file collision rather than by importance.**
`H5` (`C7` + `S33`) touches `real.rs`, `ring_interval.rs`, `curves.rs` and
`surfaces.rs` — **535 refs across 15 files** — which is every file the
other rows edit. It goes last, for the reason `C-n` goes last wherever it
lands: it conflicts with every open lane, and that is a property of the
work, not of the schedule.

| Wave | Lanes | Why together |
|---|---|---|
| **1** | **H-a** (H1/S86), **H-d** (H6/S88), **H-e** (H7/D109(a)) | Disjoint files: `ring_interval.rs`, `geom`'s projection doors, `geom-core/src/linalg/`. `H1` is marked *take it first* by the frozen table — one file, hours of work, and a certification door returning a certificate for garbage. |
| **2** | **H-b** (H2/S99–S103+S116(b)), **H-c** (H3/S85 + H4/S89) | Both wait on wave 1. **The original reason given here was wrong** — it said `H-c`'s `S89` sits on `from_certified` *"which `H-a` rewrites"*, and #880's reviewer established that **`H-a` never touched `from_certified`**: its doc, its prose caller census and its three wrappers are all untouched. The real reason is H-R6's: `H-a`'s sweep took `real.rs`, `ssi/enclose.rs` and `tests/decoration_seam.rs`, **all three rostered to `H-c`**. Same conclusion, different fact, and the difference matters because a lane told the wrong reason checks the wrong file. `H-b`'s naming work wants `H-d`'s landed doc changes under it. |
| **3** | **H-f** (H5/C7+S33), 2–3 sub-lanes | Collides with all of the above. Its `Dual`-arithmetic sub-lane is adversarial per C-R12 and §H. |

**`S90` is Evan-only and is not a lane** — the largest D1 residue is the
only one without a schedule, and `real.rs:470-477` records it as prose
pointed at from `scripts/gates/bounds-allowlist.sh:27-31`. It is
*decided-and-open*, which the first scan's own closing rule says is
exactly the state a finding may not rest in. **Asked as a
design-conversation PR at constitution**, not held until a neighbour
stalls on it — `H-c` reads the same doc block and `H-f` inherits the
seam.

---

## Lane roster

| Lane | Row | Findings | Files | Review | State |
|---|---|---|---|---|---|
| **H-a** | H1 | **S86**, **S212** minted | `geom-core/src/{ring_interval,real,k_stats}.rs`, `geom-brep/src/ssi/enclose.rs`, `scripts/gates/probe-suite-census.sh`, +3 test files | **adversarial** + style | **LANDED #880** 2026-08-21, after one fix pass |
| **H-b** | H2 (+H8) | **S99**–**S103**, **S116(b)** | `geom/src/{net,scalar_lift,surfaces,azimuth}.rs`, `curves/nurbs.rs`, `surfaces/nurbs.rs`, `geom-brep/src/nurbs_iso.rs` | **adversarial** + style | — |
| **H-c** | H3 + H4 | **S85**, **S89** | `geom-core/src/real.rs`, `ring_interval.rs`, `geom-brep/src/ssi/enclose.rs`, `topo/src/props.rs`, `geom-core/tests/decoration_seam.rs` | style | — |
| **H-d** | H6 | **S88** (`geom` half), **S210**, **S211** minted | `geom/src/{projection,curves/*,surfaces/*}.rs`, `geom-core/src/dual.rs`, `geom/tests/dual_foot_tangent.rs` | style | **LANDED #875** 2026-08-21, after one fix pass |
| **H-e** | H7 | **D109(a)** | `geom-core/src/linalg/{vec,mat}.rs`, `scripts/gates/`, whatever goldens move | **adversarial** + style | **dispatched** 2026-08-21, lane `smellh-e`, branch `smellh/h-e` |
| **H-f** | H5 | **C7** (+ S44 residue, S55), **S33** | `geom-core/src/real.rs`, `ring_interval.rs`, `geom/src/{curves,surfaces}.rs`, +11 | style; **`Dual` sub-lane adversarial** | — |
| **H-g** | **H-R4** (new) | **S90**'s implementation, **#874**'s structural half | `geom/src/{projection,curves/projection,surfaces/projection}.rs`, `topo/src/chart_region.rs`, `sweep/src/fillet/{build,surgery,battery}.rs`, `geom-core/src/real.rs`, `scripts/gates/bounds-allowlist.sh` | **adversarial** + style | **dispatched** 2026-08-21, lane `smellh-g`, branch `smellh/h-g` |

**Why each adversarial row is adversarial**, since the criterion is
narrower than "load-bearing":

- **H-a** — the fix makes `certified_bracket` refuse. Every `T:
  CertifiedEnclosure` consumer that today receives `Some((NaN, NaN))` and
  survives by accident (`ssi/enclose.rs:211`'s `pad_interval` is named in
  the finding as exactly that) receives `None` afterwards. **A door that
  starts refusing is not the same test as a door that stops laundering**,
  and CI covers the second better than the first.
- **H-b** — `S99`'s widening changes what `net::is_placeholder` answers at
  **~25 consumer sites**, and the described-net-with-poisoned-`x` case it
  newly catches is by construction one nothing currently constructs. The
  behaviour change is the point; its blast radius is the risk.
- **H-g** — it **evicts `Dual` at the public API of a library crate**, on a
  ruling whose own scope note is *"at least for now"*. The failure mode
  is not a wrong answer but a tightened **pass**: H-R3 turns on the
  doors/passes distinction, and the edit that respects it and the edit
  that breaks it differ by one bound. Its first task is a
  caller-propagation proof, not a change.
- **H-e** — it **moves `f64` bytes and re-cuts goldens** at
  `mat.rs::rotation_about`. The sequencing is non-negotiable (convert,
  re-cut, *then* widen the matcher), and getting it backwards reds two
  ratified sites — an outcome S63 has already realised twice.

## Standing rules inherited from other tracks

Landed on `main` 2026-08-21 by Track E (#869). **Lanes read the standing
header from `main`**, which is why these are recorded here rather than
pasted into a brief — and why this log lands at every pipeline seam
rather than at session end.

- **A restart loses the inbox and keeps the worktree.** Two container
  restarts have happened on this project in one day. A lane that wakes
  holding uncommitted work it cannot account for must **revert first,
  ask second, and not conclude**: the restart drops the *delivery
  record* of orchestrator messages while the work done for them survives
  on disk, so the tree can hold a half-completed transition between two
  instructions whose record is gone. *"I cannot find the authority for
  this change"* is evidence about the records, and after a restart the
  records are the unreliable half. One lane reported itself for
  fabricating a ruling from Evan on exactly this; it had not.
  **`H-e` carries extra exposure here** — a re-cut golden reads as
  somebody's intention even when it is a mid-transition artifact — and
  its dispatch says so.
- **A fence's track is derivable from the branch prefix.** Every branch
  carries its track as a prefix, verified across all 81 `smell*`
  branches. So a head on **another** track is a fence and the lane
  stops; a head on **its own** is the orchestrator's to sequence, and
  the question is only *which lands first*. A lane does not need to ask
  who owns a head it bumped into. `smellh/` is this track's.

## Open with Evan

- **#867 — `S90`.** Does the twice-done enumeration discharge the fillet
  seam's lapsed guard, or is the lane owed? Three answers stated in the
  PR; the doc is edited in place and will be updated in place with the
  answer. **Nothing waits on it** — wave 1 does not read this seam — but
  `H-c` edits the same `real.rs` doc block and `H-f` inherits the seam,
  so the answer is wanted before wave 2 opens. `real.rs:394`'s
  *"#643-completeness question"* is in the same position and takes the
  same answer.

## Neighbours, live as of 2026-08-21

- **Track I** — `docs/SMELL-I-LOG.md`, constituted the same day (#866).
  The edge is a **dependency, not a file overlap**: `geom-brep` and
  `mesh` build on `geom`/`geom-core`, so every Track H public-signature
  change is published in the lane's PR body. Every Track H brief carries
  that instruction.
- **Track G** — `docs/SMELL-G-LOG.md`. Ground is `demos/`, `profile/`,
  `interval-transcendentals/`, `sweep/src/` outside `fillet/`. **`S88`'s
  `profile` half is Track G's `G4` and is explicitly out of `H-d`'s
  scope**; `H-d` hands its `profile` hits over as a receipt rather than
  taking them.

Both edit `docs/SMELL-SCAN-2026-08.md`. Conflicts there are expected;
see the recording convention above for how they resolve and when a merge
may precede a green run.

### H-R6. `S86` under-counted: the laundering was at three implementors, not one

**H-a swept rather than fixed the named site**, and the finding was one
of four. `certified_bracket` returned a `Some` without consulting a poison
state at **`RingInterval`** (the finding), at **`f64`** — whose doc carried
*the same* rejected-downstream-by-accident argument S86 condemns — and at
**`Probe`**. **`Interval` was already correct** and is now swept rather
than trusted. The trait's own Implementors list had asserted the
laundering of three of them.

**The postcondition is the durable half**: a `Some` never carries a NaN
end, stated at the trait so a generic `T: CertifiedEnclosure` consumer can
rely on it — which is precisely what S86 said no consumer was told.

**Not taken, on a ratified line:** `±∞` still certifies at `f64`. D4's Q1
residue ratifies *"∞ is not `f64` poison"* and `[−∞, ∞]` is a sound
bracket; `from_bounds` is what stops it one step later, and a test row now
pins **which** of the two stops **what**.

**Roster correction, and it lands on H-c.** H-a's row read
`ring_interval.rs` alone; the sweep made it **four source files across two
crates**. `real.rs`, `ssi/enclose.rs` and `tests/decoration_seam.rs` are
all rostered to **H-c** (wave 2) as well. Wave 2 waits on wave 1 by
construction, so this is **a re-merge for H-c, not a fence** — but H-c's
brief must say so, because a lane that finds three of its files already
edited by an unannounced hand reads it as a rogue actor, which is the
lane-takeover courtesy `memories/agent-lane-operations.md` exists to
prevent.

### H-R8. Two orchestrator hypotheses, two falsifications, both by reviewers

**Recorded as a pattern rather than twice as an apology.** This track has
now put two hypotheses of its own into dispatches, and reviewers have
killed both on first contact:

- **H-R5's screen** — *"sole `T: Bounds` **and** `T` in the return type"* —
  silent on `projection::mid`, the freeze site the finding is about.
  Retracted above.
- **The `S89` predicate worry** — that #880's poison checks at three
  implementors would mint three spellings of one predicate, S89's defect
  one finding over. **False.** Each door consults the type's own existing
  one-home test (`f64::is_nan`, `RingInterval::is_poison`,
  `Interval::is_certified`, `Probe` delegating). The reviewer's words:
  *"I built on the hypothesis and it did not survive contact."*

**What the second one cost, which is the part worth keeping.** The
hypothesis was not merely wrong, it **pointed away from the finding that
matters**: the defect *is* present, in the **prose** register (five
restatements of one rule, a hand-maintained implementor census) and not
the predicate register — and chasing the predicate register led away from
**S5**, the postcondition hand-rolled in three other doors, which is the
row the reviewer would actually schedule. **A wrong hypothesis in a brief
does not just fail; it spends the reviewer's attention.**

**Both were flagged as unverified when issued, and both were killed
because a reviewer was told to kill them.** That is the mechanism working
as designed — `docs/REVIEW-STYLE-DISPATCH.md` §3 says a dispatcher's
unchecked causal story arrives carrying the dispatcher's authority. The
standing practice for this track: **an orchestrator hypothesis ships with
an explicit instruction to falsify it, or it does not ship.** A third one
should also be cheaper to test than to believe.

### H-R9. A lane contested a review finding with a receipt and was right

**#875's reviewer, finding 15, claimed `nurbs_curve_aabb`'s census row was
inaccurate** — that it *"funnels through `Brk::{add, sub, mul,
sqrt_nonneg}`, which is `f64` interval arithmetic after the bracket
read"*, so the row's *"bracket reads only — no arithmetic"* was wrong.

**H-d contested it and the tree is with the lane.** The whole body is one
line:

```rust
pub fn nurbs_curve_aabb<T: Bounds>(curve: &NurbsCurve3<T>) -> Aabb {
    Aabb::from_points(curve.control().iter().copied()).unwrap_or_else(Aabb::poison)
}
```

`grep -c Brk` over the function is **0**; `Brk` belongs to
`circle_arc_aabb`/`ellipse_arc_aabb`. Verified independently at
`curves/boxes.rs:383` before merging. The lane named the path in both the
module doc and the census row so it is not re-raised.

**Recorded because the direction is the rare one.** This track has spent
the day on reviewers correcting lanes and on a reviewer correcting the
orchestrator twice. **A lane refusing a finding, with a one-line receipt,
is the same mechanism running the other way** — and the reviewer brief
invites exactly this by telling reviewers they need not be sure. A style
finding is *"recorded, not gating"*, which only works if a lane may decline
one on evidence. **The cost of getting this wrong is asymmetric**: a lane
that accepts every finding to be agreeable edits code on a false premise,
and the false premise is what survives in the record.

### H-R10. Two fix-pass moves worth copying, from #880

**Both are the same shape: fix the mechanism, not the instance** — and
both were the lane's own upgrade on what the review asked for.

**1. A stale census was rewritten so it cannot go stale again.** The
review named **two** places still asserting `S86` open; the lane found
**four** (adding §A2's prioritisation essay and the frozen table's intro).
Rather than correct four sentences, it rewrote the survey into past tense
and added: *"this list is not maintained and is not evidence a finding is
still open — read each finding's own lead."* **A present-tense
hand-kept census of other findings' states drifts the first time anything
below it closes**, and the next lane to close a row here would have had to
find it again. That is the S110/S89 family — a hand-maintained tally with
nothing deriving it — killed at the source.

**2. A one-sided guard was fixed in code rather than narrowed in prose.**
`every_refused_ring_crosses_as_poison` counted only `refused`, so a door
that refused *everything* left that row green while every other row went
red — in the row that is specifically about what refusal does. The
tempting fix is to correct the header's *"both halves carry non-vacuity
assertions"* to exclude it. **The lane instead made the row assert the
certified side crosses un-poisoned and moved it to the two-sided
counter**, so the claim became true rather than the claim's scope becoming
smaller. It is the only code delta in the fix pass, and it is the right
one.

**And the sweep distinction that decided mint-versus-sweep on `S5`**, which
is worth having as vocabulary: *the sweep for the **defect** found no
siblings — all three sibling doors already answer correctly — while the
sweep for the **invariant** found three restatements.* Two different
sweeps; only the second has hits. So the row is not *"the bug has siblings
I am declining to fix"* but *"a rule stated in four voices, one normative,
with nothing connecting them"*. Minted **`S212`** — the lane had to
renumber off `S210` mid-flight because #875 took it, which is G-R13's rule
realised **inside one track**: a published block protects against other
tracks, not against a sibling lane.

### H-R11. Two things to carry, from H-e's close

**1. `S163(d)`'s census was stale-dated before this track touched it — and it is still open.** H-e's own diff falsified two citations, and repointing them
turned up that one, `interval-square-allowlist.sh:153`, was **already wrong at
its merge base**, where the site was `:161`. H-e named the site rather than
renumbering it, and left the member open — the right call, since `D109(d)`'s
*"14 of 71 `gate_error` call sites are reached by no self-test case"* is a
**traced** count and this does not touch it. **But a census whose line numbers
were wrong before anyone edited the file is a census nobody has re-derived**,
and `D109(d)` is the row that will be read as authoritative when someone
staffs it. Recorded here so whoever takes (d) re-derives the sites by name
before trusting the count.

**2. A pattern-match liveness check matches its own command line. This cost
two false readings today, in opposite directions.** The orchestrator's
`ps aux | grep "cad-work/smellf"` reported ten live processes in lanes it was
about to delete — the hits were a backgrounded `du` whose *arguments* were
every lane path, and, but for a second look, twelve lanes would have been
kept for a phantom. H-e's `pgrep` reported *"DOC-GATE RUNNING"* against its
own command line and nearly reported phantom background work at close.

`memories/agent-lane-operations.md` already says **kill** by recorded PIDs and
never by pattern-matching a lane name. **This is the detection case and it is
the opposite failure**: pattern-matching does not over-kill here, it reports
**liveness that is not there** — which stalls a sweep instead of ending one.
The fix is the same shape and costs nothing: exclude the checking process
(`grep -v grep`, `pgrep -f` with a pattern that cannot match its own
invocation), and **when a liveness check is load-bearing for a destructive
action, confirm with a second signal** — a lock holder via `fuser`, a
`target/` mtime, a commit timestamp. If this recurs outside Track H it should
graduate into the memory; one track's log is the right home for it until then.

### H-R12. `#889` is a correctness defect in `H-b`'s file — flagged, not a gate

**Filed 2026-08-21 by another track**: *horn/spindle tori are rejected on
one of the two doors that can mint them*, at
**`crates/geom/src/surfaces.rs`** and `step-import/src/entities.rs`. The
first is Track H's ground and is **`H-b`'s** file (`S102` edits
`surfaces.rs:26-30`'s *"The shared helper"* bullet).

**It does not gate `H-b`, and the distinction from the `C-m` precedent is
the reason.** Evan struck `C-m` from Track I's schedule behind **#723**
because that row *consolidates the four quadrature engines in the very
file with the wrong certified volume* — consolidating first would either
bake the defect into the shared engine or move it somewhere the
reproduction no longer points. **The coupling here is nothing like
that**: `H-b`'s work on `surfaces.rs` is one doc bullet that names a
shared helper without naming `crate::azimuth`, and it does not touch the
torus classification the issue is about.

**So the rule that applies is the weaker one: `H-b`'s brief carries the
pointer.** A lane editing a file with an open correctness issue should
know before it starts, so that (i) it does not "tidy" the defective path
and make the reproduction stale, and (ii) if it happens to understand the
defect while it is in there, it files what it learns on **#889** rather
than into a style row. **A style pass does not fix a correctness defect** —
that ruling is Evan's and it holds here exactly as it did for `C-m`.

**Recorded rather than left to a merge conflict**, because the failure
mode is silent: `H-b` would have discovered it only if the issue had
touched the same lines, and it does not.

### H-R13. The repo has ~39 `compile_fail` rows and not one verifies what it claims

**Two halves, found six hours apart by two different lanes, and neither
half is visible from the other.**

- **`crates/*/tests/` — never collected.** A doctest in a `tests/` target
  is not run at all. Established by H-g, confirmed twice by #886's
  reviewer in a way that **excludes the `autotests = false` confound**: a
  planted `assert!(false)` doctest and a planted always-green
  `compile_fail` in the aggregated `geom` target were both uncollected,
  *and* the same in `crates/quantity/tests/` with autotests **enabled**
  was also uncollected, while `cargo test -p quantity --test zz_doc_probe`
  built and ran the target. `cargo test --doc` collects **lib targets
  only**. **11 such rows**, all in `geom-core/tests/` — this is `S214`.
- **`crates/*/src/` — collected, but the error code is inert.** #886's
  reviewer planted two rows annotated `compile_fail,E0277`, one failing
  with **E0425**, one with **E0308**: `4 passed; 0 failed`, no warning.
  **The annotation is documentation, not a check. 28 such rows.**

**So the two halves compose into one statement**: ~39 `compile_fail` rows,
of which 11 never execute and 28 execute but cannot tell the intended
error from a typo. **Every one of them is a negative proof about the type
system, and negative proofs rot in the direction nothing else catches.**

**And there is a floor under how well this can be fixed, which is the part
worth keeping.** For an **inherent method**, no error code can pin an
eviction: `Dual64` failing the bound and a misspelled method name both
emit **E0599**. So at `::project`/`::project_from_seed` the idiom is not
merely unenforced — *no annotation exists that would distinguish the
proof from a typo*. Only the free-function case (`chart_region_overlap`,
real `E0277` naming `required by a bound in chart_region_overlap`) can be
pinned by code at all. **A fix that rewrites annotations buys less than it
appears to**, and whoever takes this should decide what the rows are
*for* before deciding how to spell them.

**Why this track found it and the instruments track did not.** Track F's
subject was guards, and it swept for guards that *fire wrongly*. This is
a guard that **does not run** and a guard that **runs and accepts
anything** — the same family as `S110`, and reached here from the
opposite direction: a lane needed a compile-fail proof for its own change,
and asked whether the proof worked. **The question that found it is
"does my own evidence hold", not "is there a defect".**

### H-R14. `S213` was minted twice inside this track — the orchestrator allocates from here on

**Both lanes re-derived against `main`, both were right to, and neither
could have seen the other.** `smellh/h-g-doors` (#886) minted `S213` for
`real.rs`'s false `EdgeNurbsLane` credit; `smellh/h-e` (#885's fix pass)
minted `S213` for the blind oblique-rotation oracle. **Neither branch was
merged**, so `main` showed `S210`–`S212` taken and `S213` free to both.

**The published block does exactly what §H says it does, and no more.**
It protects Track H from *other tracks*. Between two open branches
*inside* Track H it is worth nothing — which is G-R13's rule, and which
this track had already seen once today: **H-a renumbered off `S210` only
because #875 had already landed.** The check that saved H-a cannot fire
when nothing has landed, and I left two lanes drawing from one pool with
that check as the only safeguard.

**Resolution — by first mint, not by landing order.** `S213` stays with
**#886**: minted ~2h earlier and already through an adversarial review, so
moving it would invalidate a reviewed artifact to spare an unreviewed one.
*(Track G's `D114` used landing order because both rows were already on
`main` and merged-row-wins could not decide; here neither has landed, so
the tiebreak is available and the cheaper one is correct.)* H-e's becomes
**`S215`**; `S214` is the orchestrator's.

**The mechanism, effective now: the orchestrator allocates, lanes ask.**
No Track H lane draws from `S210`–`S229` / `D140`–`D159` itself again.
Each reservation is published in **this file**, which sits on `main` far
more often than any lane branch — so a reservation is visible to a lane
that has merged `main`, which is precisely what a block on a branch is
not.

**Standing allocations**

| number | subject | holder |
|---|---|---|
| `S210` | the sole-`T: Bounds` class has a rule and no instrument | H-d, **landed #875** |
| `S211` | two `geom` box modules claim a gate entry they cannot have | H-d, **landed #875** |
| `S212` | the certified door's postcondition has one home, three doors re-derive it | H-a, **landed #880** |
| `S213` | `real.rs` credits M7-8 with a technique it does not use | H-g, **#886 open** |
| `S214` | eleven `compile_fail` doctests in `tests/` are never collected | orchestrator, **unmerged** |
| `S215` | the only oblique-axis bit-exact rotation test cannot fail | H-e, **#885 open** |
| `S216` | the `compile_fail` error code is inert — 28 rows in `src/` | H-g, **to mint** |

**Why this is a ruling and not an incident.** Nothing shipped wrong, both
lanes followed their briefs exactly, and the collision was caught before
either landed. **The defect is in the allocation design, which is mine** —
and the general shape is worth more than the fix: *a check that reads a
shared branch cannot see reservations held on unshared ones, so a
reservation scheme is only as good as the frequency with which its
register reaches the shared branch.*

## Incidents

### A closed track left a lane with a dirty tree, and only the cleaner's refusal saw it (2026-08-21)

**Disk hit the 15G WARN.** Track F is closed and its twelve lanes were
stale, so `local-scripts/clean-lanes.sh` was pointed at nine of them.
**It refused one** — `smellf-a`, on branch `smellf/f5-door-registries`,
whose HEAD was pushed and in sync but whose **working tree carried 1,483
uncommitted lines**: an sccache CI action, a 381-line `ci.yml` change,
`docs/CI-MINUTES-2026-08.md`, two rebuild-latency perf artefacts, and
**392 lines of `docs/SMELL-G-LOG.md`** — another track's log, on a
Track F branch.

**Nothing was lost.** The sccache work is on `main` (14 hits in
`ci.yml`, and `install-sccache/action.yml` is byte-identical to
`origin/main`), so the tree held a **stale snapshot of a main that had
already absorbed it**, not unpushed work. The eight clean lanes were
deleted; **`smellf-a` was left in place.**

**Three things worth keeping.**

1. **The refusal is the only instrument that looks.** Nothing else in
   the pipeline reads a terminated lane's working tree. A closed track
   publishes its rows, its log and its incidents — and says nothing
   about the state of the clones it abandons.
2. **"Pushed and in sync" describes the branch, not the lane.**
   `git status -sb` showed the branch clean against its remote while
   1,483 lines sat uncommitted beside it. A liveness or handoff check
   that reads the branch answers a different question than the one being
   asked.
3. **It was not deleted, and that is the ruling.** Establishing that the
   content had landed took four commands; establishing that *nothing
   else* in it was unique would have taken considerably more, and the
   reclaim was 3G out of a budget that the other eight lanes had already
   returned 13G to. **Deleting a tree you cannot fully account for to
   reclaim space you do not need is the trade the restart rule warns
   about**, one actor removed: there, a lane must not conclude about
   work whose record is missing; here, an orchestrator must not delete
   it. Whoever next sweeps `cad-work/` inherits this note rather than a
   mystery.
