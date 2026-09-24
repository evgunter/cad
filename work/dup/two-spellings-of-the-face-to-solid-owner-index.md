---
id: two-spellings-of-the-face-to-solid-owner-index
kind: issue
title: SolidOwners::of and offset_together::Scope::walk are two spellings of the face-to-solid owner index
status: closed
opened: 2026-09-20
priority: P1
cost: D
refs: [the-guarded-shell-list-of-a-solid-is-spelled-thirteen-times, listing-a-solids-faces-is-spelled-four-times-in-topo-src]
closed: 2026-09-24
branch: dup/owner-index-divergence
---



## Finding

- **Where**: `crates/topo/src/separation.rs` (`SolidOwners::of`,
  ~:528) and `crates/topo/src/offset_together.rs` (`Scope::walk`,
  ~:819).
- **Importance**: medium
- **Confidence**: sure. Both were read.
- **Raised by**: the `faces_of` fold, 2026-09-20
  (`work/dup/listing-a-solids-faces-is-spelled-four-times-in-topo-src.md`).

Both walk `Solid::shells -> Shell::faces` and build a
`SecondaryMap<FaceKey, SolidKey>` plus a
`SecondaryMap<VertexKey, SolidKey>` from it — "which solid does this
entity belong to", indexed. `SolidOwners` is `pub` and documented as
*"the multi-solid companion to `SolidSeparation`"*; `Scope` is
`pub(crate)`, carries an `edges` map and a scope list besides, and is
the offset-together verb's working set.

They differ in **refusal posture**, which is the part a fold has to
decide rather than assume — and there is a **third posture in
`SolidOwners`' own file**, 124 lines above it:

| | `SolidOwners::of` (`separation.rs` ~:528) | `Scope::walk` (`offset_together.rs` ~:819) | `SolidSeparation::of` (`separation.rs` ~:409) |
| --- | --- | --- | --- |
| an unresolved shell | skipped (`let Some(..) else { continue }`) | refuses `None` (`?`), so `of_solids` answers `None` | refuses **typed** — `BooleanError::ClassificationInvariant { what: "solid separation: a solid names a shell the body lost" }` |
| scope | every solid of the body | the named solids only, extendable by a re-scope | every solid of the body |
| what it builds | a face→solid and vertex→solid index | the same, plus edges and a scope list | a per-solid list of face BOXES, then a hull |

**Three postures on one nest, two of them in one file.** The third was
missed on this row's first pass because the row's subject was stated as
"the pair that build the same index", and `SolidSeparation::of` builds
boxes rather than an index — literally true, and the wrong fence for a
row whose table IS the posture comparison.

So this is not a copy to delete: it is one index under two contracts,
and the question is whether the total-and-lenient reading and the
partial-and-refusing one are one door with a posture argument or two
doors that should cite each other. `Body::faces_of_solid`, minted by
the unit that found this, is the inner step of both and is the natural
seam if they do fold.

## Why this is filed on dup and not on shell

`scripts/work.py territory` puts `crates/topo/src/offset_together.rs`
on **shell**'s ground; `crates/topo/src/separation.rs` is unclaimed.
The row is filed on dup because its subject is the duplication class —
one index spelled twice — and because the decision it asks for is about
two doors' contracts rather than about offsetting. **A shell lane
opening `offset_together.rs` will not see this row**, so it is
cross-referenced from
`work/dup/listing-a-solids-faces-is-spelled-four-times-in-topo-src.md`;
a shell lane that would rather own it should move the file, per
`work/README.md`'s one-file-one-item rule.

## How it was found, and what the instrument could not see

The denominator-first census for "list a solid's faces" enumerated
every `Solid::shells`-shaped field read in every tracked file (139) and
classified each by whether it reached a `.faces` (41). These two were
the only pair among them that build the same INDEX; the other walks
consume the faces inline for a point cloud, a bounding box or an arena
count. The instrument cannot see an index assembled without a
`Solid::shells` read — one built by scanning the face arena and reading
each face's back-pointer would be invisible to it.


## Dispositioned, and not a member: `Scope::faces_in_scope`

`offset_together.rs` (~:858) is a third spelling of "these solids'
faces in arena order" — and it is **not** a member of this row or of
`listing-a-solids-faces-…`, for a reason its own rustdoc states: it
walks no body at all. It filters the `SecondaryMap` the scope already
built, whose key-slot iteration IS arena order, so it is a read of a
cached partition rather than a scan. Nothing there can fail, which is
why it returns a bare `Vec` where `Body::faces_of_solid` returns
`Option`.

It is listed here because it was neither member nor non-member on the
first pass, and because
`offset_together::scope_walks::an_out_of_scope_solids_corruption_does_not_refuse_the_scope_walk`
(and `shell10_r2_probes::r2_a_re_scope_up_holds_the_solid_it_was_aimed_at`)
assert `faces_in_scope()` equal to `Body::faces_of_solid` as a
SEQUENCE — so if this index and the door ever diverge, that row says
so. **Those two rows are the only ones that do.** `body.rs`'s
`faces_of_solid_restricts_the_face_arena_to_one_solid` was cited here
for that and does not mention `faces_in_scope` at all; after the fix
pass it carries no shell-walk comparison either.

## Measured 2026-09-20: TWO doors, and they already DISAGREE

The `Body::shells_of_solid` unit took this row as the second half of
its unit and did **not** fold it. The reason is a measurement, not a
judgement about contracts.

`Scope::walk` builds its vertex map by descending
face → loops → `loop_cycle` → half-edges, and it has an arm for
`LoopBoundary::Empty { vertex }`. `SolidOwners::of` builds its vertex
map by scanning the **half-edge arena** and looking each half-edge's
loop up in the face map. **A lone vertex in an empty loop has no
half-edge**, so it never enters `SolidOwners`' map at all.

Run in-crate against a bare `mvfs` body — one solid, one shell, one
face, one empty loop, one vertex:

| | answer |
| --- | --- |
| `crate::validate(&body)` | `Ok(())` — **tier 1 valid** |
| `SolidOwners::of(&body).face(m.face)` | `Some(solid)` |
| `SolidOwners::of(&body).vertex(m.vertex)` | **`None`** |
| `Scope::whole(&body).solid_of(m.face)` | `Some(solid)` |
| `Scope::whole(&body).holds_vertex(m.vertex)` | **`true`** |

So the two indices answer differently about the same entity of the
same body, and **one of the two says so in its own rustdoc while the
other's is false about it**:

- `Scope`'s doc (`offset_together.rs`, above the struct): *"The scope
  is total on the entities a shell owns, lone vertices included: an
  empty loop's vertex is reached through the loop's own face rather
  than through an orbit it has no half-edge for."* Accurate.
- `SolidOwners`' doc (`separation.rs`): *"a body that passes tier 1 has
  a total map."* **Measurably false** on the body above. The skeletal
  `mvfs` state is tier-1 valid — `validate::tests::tier_two_rejects_the_skeletal_mvfs_state`
  is the row that says tier TWO is what rejects it — so the sentence's
  own precondition is met and its claim is not.

**That settles the row's question.** These are not one door with a
posture argument: folding either onto the other would change what it
answers about a lone vertex, silently, with no row to catch it.

**And the falsification itself is unpinned.** The probe above was a
throwaway, run in-crate and deleted; **nothing in the tree reds if
either door's behaviour changes here**, in either direction. So the
divergence is recorded in this file and nowhere a build can see it,
which is the weakest possible state for a finding about a `pub` door:
the next lane to touch `SolidOwners` gets no signal at all. A row that
asserts the two answers against each other — whichever way the
decision goes — is the first thing a unit here owes, before any fold.

The `shells_of_solid` unit corrected `SolidOwners`' **mechanism**
prose, which was false independently of the decision: the paragraph
said the map is built *"from the STORED back-pointers — a half-edge
names its loop, a loop its face, a face its shell, a shell its solid"*,
and the FACE map is not built that way at all. It walks the forward
ownership lists, `body.solids() → solid.shells → shell.faces`; only
the vertex map follows back-pointers. That correction carries the
divergence as a stated non-totality and points here. **It does not
decide which answer is right**, which is this row's question and stays
open.

The shared spine is nonetheless smaller than the row assumed: the
outer step of `Scope::walk` — `get_solid(solid)?.shells` — folded onto
`Body::shells_of_solid` with the rest of that class.
`SolidOwners::of` and `SolidSeparation::of` were **not members** of
that class at all — they are in its "not members" table, with the
reason. The three-posture table above therefore compares three things
of which only one ever resolved a solid key.

### What is left for a unit here

1. The divergence above: decide which answer is right, pin it with a
   row, and correct the false totality sentence. **This is the part
   that has to happen first** — it is a live disagreement, not a
   duplication.
2. Only then, whether the face half of the two walks shares a home.

## Closed 2026-09-24 — `SolidOwners` made total; two doors, citing each other

Merge base `6db5b87f2`.

### 1. The divergence: `SolidOwners` was wrong, and is now total

**Reproduced first**, as a committed row rather than a throwaway: the
row below, run against the unfixed tree, reds on
`SolidOwners::of(&body).vertex(bare.vertex)` — `left: None, right:
Some(SolidKey(2v1))` — after its fixture's own
`crate::validate(&body) == Ok(())` assertion passed. So the measurement
this row recorded on 2026-09-20 holds at the merge base.

**Which answer is right, argued from the code's own contracts:**

- `validate.rs`'s module docs, tier-1 check 5 (*vertex anchoring*):
  *"every vertex is anchored by incident half-edges XOR by one empty
  loop"*, and a lone vertex held by no empty loop is an
  `OrphanEntity`. On a tier-1-valid body every lone vertex therefore
  has exactly one loop, that loop one face, that face one shell, that
  shell one solid. The body places it.
- `SolidOwners::of`'s own doc: a caller that gets `None` *"learns that
  this body does not place the entity"*. By the check above that
  sentence was false for every lone vertex, so the `None` was the
  door contradicting itself, not a posture.
- `SolidOwners`' struct doc: *"Which solid each face and each vertex of
  a body belongs to"* — no exception for lone vertices.
- `Scope`'s doc already claimed totality on lone vertices, and was
  right. Moving `Scope` toward `SolidOwners` would have made it wrong
  against check 5 as well.

So the vertex map now reads **both anchors check 5 allows** — the
half-edge arena pass it had, plus a loop-arena pass placing each
`LoopBoundary::Empty` vertex through its loop's face — and the struct
doc says *"A body that passes tier 1 has a total map"*, with the
reason.

**The false totality sentence was not in the tree at the merge base.**
The brief named *"a body that passes tier 1 has a total map"* as false;
the `shells_of_solid` unit had already replaced it with a stated
non-totality pointing here. This unit's doc change is the other
direction: it restores the totality sentence and makes it true.

### Every caller, and what it did with a `None`

Denominator: the type name. A caller must spell `SolidOwners` to get a
value — `of` is the only constructor, and `Default`/`Clone` need the
name or an existing value. `git grep -n SolidOwners` at the merge
base, no path argument: **22 hits** — 8 in code, 13 in this file, 1 in
`work/STATUS.md`. The 8 code hits:

| hit | what it is | `.vertex` / `.face` reads | what a `None` did | what changes |
| --- | --- | --- | --- | --- |
| `crates/topo/src/separation.rs` ×2 | the definition | — | — | — |
| `crates/topo/src/lib.rs` (`pub use separation::{…}`) | re-export | — | — | — |
| `crates/topo/tests/solid_separation.rs` ×3 (`//!` twice, `use`) | prose and import | — | — | — |
| `crates/topo/tests/solid_separation.rs` (`solid_owners_places_every_face_and_vertex`) | `SolidOwners::of` on two cubes | `.face` ×1, `.vertex` ×1 | `.expect("every vertex is placed")` — a `None` panics the row | nothing: two tier-2 cubes have no empty loop, so the new pass inserts nothing |
| `crates/editor-core/src/checks.rs` (`declared_pairs`) | the separation resident's declared-contact pairs | `.vertex` ×3, `.face` ×5 | `note` drops the record: an unresolved side contributes no pair, so a declaration naming a lone vertex suppressed **no** separation finding | such a record now contributes its pair and suppresses the finding it declares. On a body with no empty loop the map is identical to before, entry for entry |

**Second pass, aimed at the first's blind spot** (a call site whose
receiver was built elsewhere, so the call itself never spells the
type):
every line carrying a `.vertex(` call in every tracked `*.rs` and
`*.py` file, **28**, classified by receiver. Three lines have a
`SolidOwners` receiver — `checks.rs` ×2 and `solid_separation.rs` ×1,
the sites above. The other 25: `GraftKeys` (`product.rs` ×2,
`names/emit.rs`, `graft_disjoint.rs`), `VoidInserted` (`shell.rs`),
`KeyView` (`boolean/ops.rs` ×4), a `ProfileNaming` loop
(`eval/anchor.rs`), ladder-resolved entity keys (`eval/wire.rs` ×2),
anchors in two `editor-core` suites (×2), the blend's `links` /
`faces` / `ends` / `c` (`sweep/src/blend` ×8) and step-import's own
`self.vertex` (×3). **What neither pass can see**: a
name assembled by a macro.

**Reach, measured with a `panic!` in the new loop-arena pass (P6
below)**: nothing in `topo`'s lib or integration suite and nothing in
`editor-core`'s reaches it except the new row. `editor-core`'s run
had two failures, identical on the unplanted baseline —
`msolve8_levered_clash::c4_band_…_{empty,overflow}` panic with
*"first commit in this process: AlreadyInitialized"*, a
process-global tolerance commit that a single-process `cargo test`
trips; neither mentions the plant. **Not established**: whether a
gathered product body can carry an empty loop at all. If it cannot,
the `checks.rs` change is inert in practice.

### The pinning row, and the plants

`separation::owner_index::solid_owners_and_the_scope_walk_place_every_entity_alike`,
on one tier-1-valid body of three solids: a unit box; a bare `mvfs`
seed (lone vertex in an empty OUTER loop); and a seed grown by a
segment and a killed strut (lone vertex in an empty RING of a face
that also bounds a cycle — the `kemr` state). It asserts both lone
vertices placed in their own solids, every face and vertex placed by
`SolidOwners`, face answers equal to `Scope::whole`'s, and for every
vertex and every solid `owners.vertex(v) == Some(s)` exactly when
`Scope::of_solids(&[s]).holds_vertex(v)`.

The row asserts an equality, so both directions of divergence make it
harder to satisfy, and totality is asserted separately because an
equality alone passes two indices that both dropped a lone vertex.
Baseline, `cargo test -p topo --lib --tests`: **761 + 575 = 1336, all
green, 0 ignored.** Every plant restored byte-for-byte from saved
bytes; `git diff --stat HEAD` empty after each.

| plant | what | direction | red | where |
| --- | --- | --- | --- | --- |
| P1 | delete `SolidOwners`' loop-arena pass (the merge base's behaviour) | owner door regresses | **1** / 1336 | the row, at the outer-loop vertex assertion |
| P2 | `Scope::walk`'s `Empty` arm inserts nothing | scope door regresses | **1** / 1336 | the row, at the per-solid equality |
| P3 | both at once | both regress together — equality alone is satisfied | **1** / 1336 | the row, at the outer-loop vertex assertion |
| P4 | the loop-arena pass places a lone vertex in the body's FIRST solid | a different answer, not a null | **1** / 1336 | the row, at the outer-loop vertex assertion |
| P5 | `Scope::walk`'s `Empty` arm `panic!`s | reach control (item 19) | **1** / 1336 | the row |
| P6 | the loop-arena pass `panic!`s | reach control | **1** / 1336 in `topo`; **0** attributable in `editor-core` (181 + 4×1 + 1607, 2 baseline failures, above) | the row |

P5 is a finding in itself: **before this row, nothing in `topo`
reached `Scope`'s lone-vertex arm**, so the sentence in `Scope`'s doc
claiming totality on lone vertices was held true by nothing.

### 2. The face half: two doors, citing each other

They do not fold. `Scope`'s doc now names `SolidOwners` as the same
index and says why they stay two; `SolidOwners::of` points back at it.
The reasons:

- **They refuse differently, and both refusals are load-bearing.**
  `SolidOwners` indexes every solid and skips what does not resolve,
  so absence is its answer; `Scope` walks only the named solids and
  refuses (`None`, mapped to `Corrupt` by `scope_of_moves` and the
  two `shell.rs` callers) on an unresolved entity of one of them.
  `scope_walks::an_out_of_scope_solids_corruption_does_not_refuse_the_scope_walk`
  pins `Scope::whole` refusing a corrupt body. A shared walk would
  need a posture parameter to keep both, which is two doors with one
  name.
- **`Scope::walk` does not have a separable face half.** It descends
  face → loops → half-edges in the same pass, so a shared face walk
  would cost it a second walk of the same faces.
- **`Body::faces_of_solid` is not a home for either.** It reads the
  faces' back-pointers where both walks read the solids' shell lists,
  answers one solid per whole-arena scan (a whole-body index through
  it is solids × faces), and drops a face whose shell does not resolve
  where `Scope` refuses.

`SolidSeparation::of`, the third posture in the table above, builds
boxes rather than an index and was not touched.

### 3. `shell10_r2_probes` folded

Closed with `work/dup/shell10-r2-probes-restates-the-scope-walk-fixtures-verbatim.md`
in the same PR: the module's two rows moved into `scope_walks`, beside
the fixtures they had restated, and the module is gone.
