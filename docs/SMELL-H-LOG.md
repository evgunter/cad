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
| **2** | **H-b** (H2/S99–S103+S116(b)), **H-c** (H3/S85 + H4/S89) | Both wait on wave 1. `H-c`'s `S89` sits on `from_certified`, which `H-a` rewrites; `H-b`'s naming work wants `H-d`'s landed doc changes under it. |
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
| **H-a** | H1 | **S86** | `geom-core/src/ring_interval.rs` | **adversarial** + style | — |
| **H-b** | H2 (+H8) | **S99**–**S103**, **S116(b)** | `geom/src/{net,scalar_lift,surfaces,azimuth}.rs`, `curves/nurbs.rs`, `surfaces/nurbs.rs`, `geom-brep/src/nurbs_iso.rs` | **adversarial** + style | — |
| **H-c** | H3 + H4 | **S85**, **S89** | `geom-core/src/real.rs`, `ring_interval.rs`, `geom-brep/src/ssi/enclose.rs`, `topo/src/props.rs`, `geom-core/tests/decoration_seam.rs` | style | — |
| **H-d** | H6 | **S88** (`geom` half only) | `geom/src/projection.rs`, `curves/projection.rs`, `surfaces/projection.rs`, `curves/boxes.rs`, `bvh/src/aabb.rs` | style | — |
| **H-e** | H7 | **D109(a)** | `geom-core/src/linalg/{vec,mat}.rs`, `scripts/gates/`, whatever goldens move | **adversarial** + style | — |
| **H-f** | H5 | **C7** (+ S44 residue, S55), **S33** | `geom-core/src/real.rs`, `ring_interval.rs`, `geom/src/{curves,surfaces}.rs`, +11 | style; **`Dual` sub-lane adversarial** | — |

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
- **H-e** — it **moves `f64` bytes and re-cuts goldens** at
  `mat.rs::rotation_about`. The sequencing is non-negotiable (convert,
  re-cut, *then* widen the matcher), and getting it backwards reds two
  ratified sites — an outcome S63 has already realised twice.

## Incidents

*(none yet)*
