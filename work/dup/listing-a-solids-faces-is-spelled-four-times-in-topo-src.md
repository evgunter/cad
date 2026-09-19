---
id: listing-a-solids-faces-is-spelled-four-times-in-topo-src
kind: issue
title: Listing a solid's faces is spelled four times in topo/src, three of them named faces_of
status: open
opened: 2026-09-19
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
