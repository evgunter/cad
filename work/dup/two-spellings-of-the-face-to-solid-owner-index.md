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
decide rather than assume:

| | `SolidOwners::of` | `Scope::walk` |
| --- | --- | --- |
| an unresolved shell | skipped (`let Some(..) else { continue }`) | refuses (`?` on `Option`, so `of_solids` answers `None`) |
| scope | every solid of the body | the named solids only, extendable by a re-scope |
| vertices | derived from the half-edge arena, via `faces` | walked with the loops, in the same pass |
| edges | none | indexed |

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
