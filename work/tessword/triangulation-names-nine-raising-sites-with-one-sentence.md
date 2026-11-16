---
id: triangulation-names-nine-raising-sites-with-one-sentence
kind: issue
title: mesh: TessellateError::Triangulation is raised at nine sites across three lanes and says 'rejected a point insertion, corrupt geometry' for all of them
status: open
opened: 2026-11-14
priority: P3
cost: E
---

Filed by TESSLANE's turned-ball unit (`work/tesslane/a-sphere-face-tilted-by-an-ulp-against-its-copies-fails-the-cdt.md`), whose row opened on exactly this question: its refusal could not say whether spade's `insert` or `can_add_constraint` had fired, and finding out took instrumenting both.

`TessellateError::Triangulation { face }` (`crates/mesh/src/types.rs`, the variant and its `Display` arm) documents itself as "the CDT rejected a point insertion (non-finite or out-of-range UV coordinate — corrupt geometry)", and its `Display` says "a non-finite or out-of-range chart coordinate, i.e. corrupt geometry". It is raised at nine sites, of three different kinds:

- **spade refused a point** — `curved::tessellate_curved`'s `insert` closure (`map_err` on `insert`), `planar::triangulate_chart`'s insert feed, `trimmed::TrimmedFace::insert_points` (`bulk_load_cdt` failing). What the sentence says, and since every feed passes through `cdt::spade_point` only a NaN or a coordinate past `MAX_ALLOWED_VALUE` reaches it.
- **a boundary constraint would cross another** — `curved::tessellate_curved` (`can_add_constraint` false), `planar::triangulate_chart` and `trimmed::TrimmedFace::constrain_boundary` (`try_add_constraint` realising nothing). No point insertion was rejected.
- **the trimmed lane's own geometry rules** — `trimmed::TrimmedFace::chain_crease_lines` (`chain_creases` answering `None`; a crease passing through a boundary point), and the retry ladder's last attempt still splitting an offender (`TrimmedFace::attempt`). Neither is spade refusing anything.

So a refusal cannot say which fired, and two of the three kinds are described by a sentence about the third. `work/tesslane/a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta.md` is another valid body that reads as "corrupt" through it.

**Done when** each kind of raise is its own reason (a variant or a typed cause), the sentence says what that site decided, and a refusal names its site without instrumenting it.
