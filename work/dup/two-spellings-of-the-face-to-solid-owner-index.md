---
id: two-spellings-of-the-face-to-solid-owner-index
kind: issue
title: SolidOwners::of and offset_together::Scope::walk are two spellings of the face-to-solid owner index
status: open
opened: 2026-09-20
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
