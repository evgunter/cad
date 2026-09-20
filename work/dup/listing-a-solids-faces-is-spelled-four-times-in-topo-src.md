---
id: listing-a-solids-faces-is-spelled-four-times-in-topo-src
kind: issue
title: Listing a solid's faces is spelled five times in topo/src, not four; three named faces_of, one a public door, one inline
status: closed
opened: 2026-09-19
closed: 2026-09-20
refs: [solid-of-face-has-eleven-hand-written-walks-outside-it, shell10-r2-probes-restates-the-scope-walk-fixtures-verbatim, the-face-to-solid-walk-is-spelled-per-test-file, two-spellings-of-the-face-to-solid-owner-index]
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

The arena-scan denominator is closed by construction for this class:
every member in arena order must iterate the face arena, so a member
this instrument cannot see does not exist in that spelling. The
shell-walk instrument covers the other spelling. Together they have one
blind spot that stays open — a member assembled by a macro — and it is
unfalsifiable by text.

### The hit list, every hit dispositioned

| site | disposition |
| --- | --- |
| `topo/src/shell10_r2_probes.rs` (`faces_of`, ~:39) | **folded** — helper deleted, 6 call sites read the door |
| `topo/src/offset_together.rs` (`scope_walks::faces_of`, ~:989) | **folded** — helper deleted, 9 call sites read the door |
| `topo/src/boolean/solid_contain.rs` (`SolidFaces::of`, ~:2411) | **folded** — its `get_solid` guard AND its filter are now one `ok_or` over the door; the `SurfaceSharedOutsideSolid` pass is untouched |
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

It refuses rather than returning a bare `Vec` because **the empty list
and the absent solid are different answers** — and that refusal is not
decoration: deleting the `get_solid` guard from the door reds
`bool4_material_containment::the_per_solid_door_answers_for_one_solid_of_the_arena`,
which is `SolidFaces::of`'s own `NoSuchSolid` row, plus the door's own.

### The mutations (PR's merge base `b29fe8bd1`; 730/566 green before, 731/566 after)

| planted in `Body::faces_of_solid` | topo lib | topo integration |
| --- | --- | --- |
| the solid filter deleted | **10 red** — and they are exactly the five folded sites plus the door's own row and a `SolidFaces::of` consumer in `census.rs` | **25 red** |
| the `get_solid` guard deleted | 1 red (the door's own row) | 1 red (`bool4_material_containment`) |
| arena order replaced by shell-walk order, reversed | 3 red (`faces_in_scope` sequence equality ×2, the door's own row) | 0 red |

No folded site is dark. The third mutation is the receipt for the
door's "arena order" sentence.

### X4 the fold minted, disclosed

Twelve copies of `.expect("a live solid")` across the two `topo/src`
test modules — the ceremony the `Option` costs at a call site that
holds a key `body.solids()` just yielded. It carries no logic and
nothing goes stale with it, and the ambient rate in `topo/src` is 362
`.expect(` and 2055 `.unwrap()`, so it is idiom rather than a copy of
a *thing*. Repeated calls within one row were bound once instead of
re-asked, which took it from fifteen to twelve. It collapses entirely
when the two modules' shared fixture family gets one home, which is
`work/dup/shell10-r2-probes-restates-the-scope-walk-fixtures-verbatim.md`'s
decision and not this unit's.

## Closed

Folded 2026-09-20. The five `topo/src` members read
`Body::faces_of_solid`; the three `sweep/tests` members are carried on
tint's row, which already named them and now has a door to fold them
onto. Nothing in this row is left undisclosed as prose: the owner-index
duplication has its own file, and so does the `.expect` residue's home.
