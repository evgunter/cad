---
id: a-family-copys-overlap-carries-no-pair
kind: issue
title: An interference a family's copy bounds carries no opposed pair, though a family Gap over the faces each member names could quiet it
status: open
opened: 2026-11-22
priority: P3
cost: M
---


`checks::at_rest::reading` (`crates/editor-core/src/checks/at_rest.rs`) drops an interference finding's `OpposedPair` where either copy's read reaches an index (`doc.var_reach(copy.var)`). The pair's faces are named as one member names them (`FaceSite.face`, a `Member { at, of }` name), and `gap_assertions`'s `face_of` reads a selection of a family read through `member_name`, which wraps the name again. So a `Gap` over the pair as carried quiets nothing, and the finding used to refuse at `Error` with a recourse nothing satisfied.

The quieting rule does take a family `Gap` over the faces each member names (the selection of the plain name off the family read). For fins sunk into one base, `Gap(select(fins, foot), select(base, top)) ≤ −δ` quiets each member's overlap where it holds at every member. A pair carrying that authored form could be offered, by the same route `at_rest::quieting` takes for a coincidence: unwrap the member name, keep it only where `face_of` reads it back as the member's face, and only where every member overlaps. The bound is the author's, so "holds at every member" needs care.

Until then such a finding is loud and refuses at no severity. Pinned by `intent_s5_c_quietable::an_overlap_a_familys_copy_bounds_carries_no_pair` ("fins sunk into their base").
