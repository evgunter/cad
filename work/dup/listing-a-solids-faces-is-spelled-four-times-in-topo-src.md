---
id: listing-a-solids-faces-is-spelled-four-times-in-topo-src
kind: issue
title: Listing a solid's faces is spelled five times in topo/src, not four; three named faces_of, one a public door, one inline
status: closed
opened: 2026-09-19
closed: 2026-09-20
refs: [solid-of-face-has-eleven-hand-written-walks-outside-it, shell10-r2-probes-restates-the-scope-walk-fixtures-verbatim, the-face-to-solid-walk-is-spelled-per-test-file, two-spellings-of-the-face-to-solid-owner-index, the-guarded-shell-list-of-a-solid-is-spelled-thirteen-times, the-same-solid-two-shell-body-is-hand-built-three-times]
---


## Finding

- **Where**: `crates/topo/src/shell10_r2_probes.rs` (`faces_of`, ~:39),
  `crates/topo/src/offset_together.rs` (`faces_of`, ~:981),
  `crates/topo/src/boolean/solid_contain.rs` (`faces_of`, ~:3764) and
  the body of `crates/topo/src/boolean/solid_contain.rs`'s
  `SolidFaces::of` (~:2411) — which is `pub`, re-exported from
  `topo::lib`, and is the only one of the four with a guard.
- **Importance**: medium
- **Confidence**: sure. All four were read. The name census
  (`(fn|let)\s+(solid_of|faces_of|shell_of|owning_solid|solid_for)\w*`
  over every tracked file, no path argument) is what found the third
  `faces_of`; no structural regex in this program's censuses reached
  it, because its body is a call and not a walk.
- **Raised by**: the `solid_of_face` fold, 2026-09-19.

One job — *the faces of this solid, in face-arena order* — under three
functions of the same name plus a public door. **Two answers to what
happens when it cannot be done**, not three: the three helpers panic
and the door refuses typed. The third helper's panic differs only in
where it is raised, which is a fact about its body and not a fourth
posture:

| site | body | refusal |
| --- | --- | --- |
| `shell10_r2_probes::faces_of` | filter on `solid_of_face` | panics |
| `offset_together::scope_walks::faces_of` | the same filter, modulo indentation | panics |
| `boolean::solid_contain` tests' `faces_of` | `SolidFaces::of(..).unwrap()` | panics, one layer down |
| `SolidFaces::of` | the same filter, then a group-read guard | `PointInSolidError` |

**`SolidFaces::of` is mechanically substitutable and is still not the
home to want.** The third `faces_of` in the table is literally
`SolidFaces::of(body, solid).unwrap().faces().to_vec()`, so there is no
impossibility to claim: the other two could be re-pointed at it today.
What it costs is an obligation and a walk. It refuses typed
(`PointInSolidError`) when a group-read kind's surface key crosses the
selection boundary — a promise a caller who wants a bare face list
neither needs nor can discharge — and it walks the whole face arena
twice. So the unit is a decision about whether the plain list is a door
in its own right — `Body::faces_of_solid`, which `SolidFaces::of` would
then filter — not a re-point of three call sites at an existing name.

The first two bodies are identical modulo their leading indentation
(one is top-level in `shell10_r2_probes.rs`, the other sits inside
`offset_together`'s `scope_walks` module); their wider fixture family
is
`work/dup/shell10-r2-probes-restates-the-scope-walk-fixtures-verbatim.md`.

## Why this is filed on dup and not on curved

`scripts/work.py territory` puts
`crates/topo/src/boolean/solid_contain.rs` on **curved**'s ground.
The row is filed on dup because its subject is the duplication class —
four spellings of one job, of which only one site is curved's — and
because the decision it asks for (mint `Body::faces_of_solid`, or do
not) is a `topo::Body` door question, not a boolean-classification one.
**A curved lane opening `solid_contain.rs` will not see this row**, so
it is cross-referenced from
`work/dup/solid-of-face-has-eleven-hand-written-walks-outside-it.md`;
a curved lane that would rather own it should move the file, per
`work/README.md`'s one-file-one-item rule.


## Re-taken 2026-09-20 at `b29fe8bd1`; the class is FIVE in `topo/src`, eight tree-wide

Three instruments, every tracked file, **no path argument**. The row's
"four in `topo/src`" was an undercount by one, and its scope sentence
hid three more outside that crate.

| instrument | denominator | hits | what it cannot see |
| --- | --- | --- | --- |
| **name/signature census** — `fn …(… solid: SolidKey) -> Vec<FaceKey>` | — | 7 (6 members + `shell8_common::top_chart`) | a spelling with no name (the `props.rs` inline site) and one whose return type differs (`SolidFaces::of` returns `Result<Self, _>`) — it missed both |
| **denominator-first, arena scan** — every textual `.faces()` call, classified forward by a solid token in the window | **839 calls over 4501 tracked files** | 16 at window 4, 21 at window 8 | the shell-walk spelling entirely; a `.faces` FIELD read off a `Shell` |
| **denominator-first, shell walk** — every `Solid::shells`-shaped field read, classified by reaching a `.faces` | **139 reads** | 41 | a walk assembled through a helper that returns the shell list |

**"Closed by construction" was overstated when this row first said it.**
Every arena-order member must iterate the face arena, but the
instrument matches the textual `.faces()` CALL, and the arena can be
iterated two other ways it never sees:

- **`body.faces.iter()` — the arena FIELD, in-crate.** 31 such sites
  at `b29fe8bd1`; 5 carry a solid token, of which `census.rs` (~:3229,
  ~:3265) genuinely group by solid (into per-solid boxes, so not
  members of THIS class) and three are `SolidSpec::faces` in
  `step-import`, a different type. `boolean/combine.rs` (~:262) is
  another of the 31. This is a DIFFERENT blind spot from the `.faces`
  field read off a `Shell` the row stated, and it was unstated.
- **`topo::query::all_faces(body)` at the caller** — the whole arena
  with no `.faces()` in the caller's own text, so
  `all_faces(body).into_iter().filter(..)` is invisible. **59 call
  sites, of which 1 carries a solid token**:
  `editor-core/tests/seat7_sweep_lowering.rs` (~:619), an **over-fire**
  — `solid` there is a node id bound two lines above and the line is
  `all_faces(body).len()`. No live member hides there today, so nothing
  is missing from the fold; the denominator is still not closed, and
  this door's own rustdoc advertises `query::all_faces` as the thing it
  restricts, which makes that spelling the one a future caller reaches
  for first.

  **This row first published that as a ZERO**, and the zero was an
  artifact of a forward-only window: the hit is two lines ABOVE the
  call. A published zero that depends on which way the window looks is
  the tidy result this program keeps tripping on — one hit,
  dispositioned, is the honest shape.

The shell-walk instrument covers the other spelling. The blind spot
that cannot be closed at all is a member assembled by a macro, which is
unfalsifiable by text.

### The hit list, every hit dispositioned

| site | disposition |
| --- | --- |
| `topo/src/shell10_r2_probes.rs` (`faces_of`, ~:39) | **folded** — helper deleted, 6 call sites read the door |
| `topo/src/offset_together.rs` (`scope_walks::faces_of`, ~:989) | **folded** — helper deleted, 9 call sites read the door |
| `topo/src/boolean/solid_contain.rs` (`SolidFaces::of`, ~:2411) | **folded** — its `get_solid` guard AND its filter are now one `ok_or` over the door |
| `topo/src/boolean/solid_contain.rs` (~:2421, the `SurfaceSharedOutsideSolid` pass) | **folded — AND THIS ROW BUCKETED IT ONCE.** A SECOND unnamed inline arena scan under the same predicate, negated, three lines below the one above and in the same function. The arena-scan instrument fires on it as a hit DISTINCT from ~:2412; this row's first pass carried one line for the file and demoted this one to a parenthetical (*"the `SurfaceSharedOutsideSolid` pass is untouched"*). **That is the `props.rs` story one unit later, in the file the unit was editing** — and not a miss but a bucketing, which is exactly what method item 9 is about. It now reads the complement off `faces` itself (`!selected.contains(&k)`) instead of re-asking the back-pointers, so the two passes cannot disagree about where the selection boundary is |
| `topo/src/boolean/solid_contain.rs` (`faces_of`, ~:3764) | **folded** — the one-line wrapper deleted, its 4 call sites read the door and no longer pay the group-read guard to get a face key |
| `topo/src/props.rs` (~:3290) | **folded — THE ROW MISSED THIS ONE.** The same filter written INLINE, with no helper name, inside `face_list_door_tests`. The name census could not see it; the `solid_of_face` fold had rewritten the walk inside it three lines earlier and did not see that the whole expression was a fifth copy |
| `sweep/tests/shell8_common.rs` (~:67) | not this unit — sweep/tests is tcost/tint ground and a sibling lane was live in it; carried on `work/tint/the-face-to-solid-walk-is-spelled-per-test-file.md`, which already named these three |
| `sweep/tests/shell8_r1_probes.rs` (~:45) | the same |
| `sweep/tests/shell8_r2_probes.rs` (~:73) | the same |
| `topo/tests/bool4r1_probes.rs` (~:195) | **not a member.** It `find`s ONE face by geometry, with solid membership as one criterion among several; it never materialises the list |
| `sweep/tests/shell8_common.rs` (`top_chart`, ~:203) | **not a member** — a solid's faces filtered further by height; it CALLS `faces_of` |
| `sweep/tests/shell10_r1_probes.rs` (`all_faces`, ~:29) | **not a member** — no solid filter; an instrument-window over-fire |
| `sweep/tests/shell8_r2_probes.rs` (~:407) | **not a member** — a pairwise cross-solid comparison, not a list |
| `demos/tour/src/heatsink.rs` (~:473), `sweep/tests/shell5_r1_dump.rs` (~:81), `topo/src/review_m1_pr1.rs` (~:466), `topo/src/review_m1_pr2/degenerates_and_sequences.rs` (~:702), `topo/src/seqgen.rs` (~:724, ~:1598) | **not members** — window over-fires; a count assertion, a dump line, a name lookup, and two face-pair searches |
| `work/dup/solid-of-face-has-eleven-hand-written-walks-outside-it.md` (~:209) | **not code.** The arena-scan instrument reads every tracked file, so it fires on this program's own prose describing the shape. Listed because a hit dropped without a disposition is how a census loses things |
| the twelve `Solid::shells → Shell::faces` walks the third instrument found | **not members, and a different job.** None materialises "the faces of this solid": each consumes faces inline for something else (a point cloud, a bounding box, an arena count, an owner map). Two of them ARE a duplication of each other, one level out, and are filed: `work/dup/two-spellings-of-the-face-to-solid-owner-index.md` |

### The decision the row asked for, made

**Yes, the plain list is a door.** `Body::faces_of_solid(solid) ->
Option<Vec<FaceKey>>` sits beside `Body::solid_of_face`, its inverse.
`SolidFaces::of` is now that door plus its group-read guard, which is
what the row proposed.

It refuses rather than returning a bare `Vec`. **Two reasons no
mutation measures**, and they are the load-bearing ones:

- `SolidFaces::of` now discharges the **public**
  `PointInSolidError::NoSuchSolid` entirely through this guard. The
  refusal is not a private nicety; a door in the public error surface
  stands on it.
- `Option<Vec<K>>` is already the house form for a materialising read
  in this file — `Body::loop_cycle`, `Body::vertex_orbit`. A bare
  `Vec` would have made the new door the odd one out among its
  neighbours.

The argument this row published first — that the empty list and the
absent solid are different answers — is true and is **the weaker of
the three**: the two rows that red when the guard is deleted (the
door's own, and
`bool4_material_containment::the_per_solid_door_answers_for_one_solid_of_the_arena`)
are thin evidence on their own.

### The mutations (merge base `b29fe8bd1`; 730/566 green before, **732**/566 after — two rows added)

| planted | topo lib | topo integration |
| --- | --- | --- |
| the solid filter deleted | **10 red** — eight of them rows at the five folded sites (`props` ×1, `solid_contain::per_solid_entry_tests` ×2, `offset_together::scope_walks` ×3, `shell10_r2_probes` ×2), plus the door's own row and a `SolidFaces::of` consumer in `census.rs`. **Every folded site reds.** | **25 red** |
| the `get_solid` guard deleted | 1 red (the door's own row) | 1 red (`bool4_material_containment`) |
| the door answers the **plain** shell walk (not reversed) | **1 red** — the multi-shell row below | 0 red |
| `!selected.contains(&k)` inverted in the folded complement pass | 1 red (`a_group_read_key_shared_across_the_selection_refuses_typed`) | 0 red |

No folded site is dark, the sixth folded site included.

**The order guard was weaker than the first pass claimed, and the fix
is a fixture, not a sentence.** `pillow` is `ngon_pillow(2)`: one
solid, ONE shell. For a single-shell solid the shell walk and the
arena scan are the SAME SEQUENCE, so a shell-walking implementation
passed every row this unit first wrote — which is why the first
mutation had to REVERSE the order to get a red, and a reversal is not
the thing the doc claims. `faces_of_solid_answers_arena_order_where_the_shell_walk_would_not`
now puts a solid's two shells out of step with the arena (shell 1 holds
the first face, shell 2 the third and then the second — the state an
operator that moves a face between one solid's shells leaves behind),
and a **plain** shell walk reds on it. The `faces_in_scope` sequence
equalities do NOT red on it: their solids are single-shell too, so
they never guarded the order at all.

### X4 the fold minted, disclosed — EIGHTEEN sites, not twelve

**This section is the single home for the argument**; the sibling rows
point here rather than restating it, because a restated argument with
restated numbers is `orient-module-prose-accumulation` minted by the
unit disclosing that it had not minted one.

The `Option` taxes every former call site. Counting the *ceremony*
rather than one string gives **eighteen**:

| spelling | sites |
| --- | --- |
| `.expect("a live solid")` | 12 — `offset_together.rs` ×8, `shell10_r2_probes.rs` ×4 |
| `.expect("a solid the body yielded")` | 1 — `props.rs` (~:3292) |
| `.unwrap()` | 4 — `solid_contain.rs` (~:3779, ~:3780, ~:3820, ~:3821), **created by deleting the one-line `faces_of` wrapper that held the unwrap ONCE** |
| `.ok_or(NoSuchSolid)` | 1 — `SolidFaces::of`, which is the fold, not the tax |

**Twelve of the eighteen collapse** when the two `topo/src` test
modules' shared fixture family gets one home with a local adapter over
the door — `work/dup/shell10-r2-probes-restates-the-scope-walk-fixtures-verbatim.md`'s
decision, not this unit's. The four `solid_contain.rs` unwraps are a
different case and do not: that file had a wrapper and this unit
removed it, which is a real regression in that file traded for removing
a name that hid an unrelated guard.

It carries no logic and nothing goes stale with it. **At the merge
base `b29fe8bd1`** the ambient rate in `topo/src` is **351 `.expect(`
and 2045 `.unwrap()`** — the figures this row published first (362 and
2055) were taken AFTER the fold, so the denominator included the copies
it was the denominator for. Measured before, it is idiom rather than a
copy of a *thing*. Binding repeated calls within one row instead of
re-asking took the string count from fifteen to twelve.

## Reference walks: a class this fold must NOT sweep

A test that is a door's reference cannot read through that door, or it
asserts `door == door`. Three hand-written `solid.shells ->
shell.faces` walks in `topo/src` are members of this row's class by
shape and are **exempt by construction**:

| site | what it is the reference for |
| --- | --- |
| `body.rs`'s `faces_of_solid_answers_arena_order_where_the_shell_walk_would_not` | `Body::faces_of_solid` itself — the row asserts the two agree as a SET and disagree as a SEQUENCE |
| `instance.rs` (~:352–358) | the graft's own invariant: every shell's back-pointer names the solid that lists it, and no face is claimed by two solids. Reading it through a door built on those back-pointers would assume what it checks |
| `review_m0_pr7.rs` (~:281–292) | an arena-slot audit that must tolerate every key failing to resolve (`let Some(..) else { continue }` at three levels); a door that refuses cannot express it |

Recorded here rather than on
`work/dup/the-guarded-shell-list-of-a-solid-is-spelled-thirteen-times.md`,
where this exemption was first written: that row's subject is the shell
LIST, and these are shell→**faces** walks, so a lane sweeping the faces
class would read this row and never see it there.

## Closed

Folded 2026-09-20. **Six** `topo/src` members read
`Body::faces_of_solid` — the five this row first named plus the
negated second scan inside `SolidFaces::of` that its first pass
bucketed. The three `sweep/tests` members are carried on tint's row,
which already named them and now has a door to fold them onto.

Nothing here is left undisclosed as prose. Each residue has a file:

| residue | file |
| --- | --- |
| the face→solid OWNER INDEX, spelled three ways | `work/dup/two-spellings-of-the-face-to-solid-owner-index.md` |
| the guarded SHELL list of a solid, ×13 — the door on the other side of this one, which this unit's own instrument had in hand and did not ask about | `work/dup/the-guarded-shell-list-of-a-solid-is-spelled-thirteen-times.md` |
| the `.expect` ceremony's home | `work/dup/shell10-r2-probes-restates-the-scope-walk-fixtures-verbatim.md` |
| the same-solid two-shell body, hand-built ×3 — **an X4 this unit's own fix pass minted**, in the fixture rather than the walk, and self-declared in prose at both copy sites | `work/dup/the-same-solid-two-shell-body-is-hand-built-three-times.md` |
| the three `sweep/tests` copies | `work/tint/the-face-to-solid-walk-is-spelled-per-test-file.md` |
