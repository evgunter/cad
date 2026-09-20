---
id: sixty-one-tag-words-are-minted-by-two-or-more-maps-and-seven-are-read
kind: issue
title: Most of the tags.rs words minted by two or more maps have never been read against the scoping rule
status: open
opened: 2026-09-15
---


Filed by CENSUS-PY-GETTERS' fix pass (2026-09-15), from its own
review. The unit's PR stated a rule in `crates/pncad-py/src/tags.rs`'s
header — **a tag word is scoped to the map that mints it**, so a word
two maps both speak is a coincidence and not a collision — and offered
as evidence that `join` already collided with itself twice in-file.

The rule was DECIDED over the seven words one unit was dispatched at
and ASSERTED over the file. The population it covers was **61 words
minted by two or more maps** when this row was opened and is **62 on
`census/py-raise-literals`**, measured two ways that agree: a
per-function sweep of `tags.rs`'s literals, and the same computation
over `TAG_INVENTORY`'s committed rows (98 at the opening, 102 on that
branch). Eight of the 62 have been read by anybody — `face`, `edge`,
`vertex`, `empty`, `join`, `split`, `no_at_rest_record`, and `validate`,
which CENSUS-PY-RAISE-LITERALS dispositioned as a coincidence at
`SHARED_TAG_WORDS`'s own site when `validation_refusal_tag` landed.
**54 have not** — the same 54, because the unit that grew the
population also read the word it added.

**No count is written into the prose this row is about any more.**
`tags.rs`'s header carried `61` and went stale in the diff that moved
the population to 62; it now names `SHARED_TAG_WORDS` instead, which
is derived from `TAG_INVENTORY` and is therefore the measurement.
**This row's ID keeps its number** and its body no longer does: the id
is cited by `docs/DOC-LEDGER.md`'s CENSUS-PY-GETTERS entry as the
residue that unit filed, by `work/STATUS.md`, and by three sibling
rows, and a rename would falsify a ledger line that records what was
filed under that name. Read the id as a name, not as a measurement.

## What the fix pass left behind

`crates/pncad-py/src/tests.rs`'s
`every_word_two_tag_maps_share_is_on_the_committed_roster` holds that
population to a roster of `(word, number of maps)` derived from
`TAG_INVENTORY`, so a word that starts colliding, stops colliding, or
picks up a third minting map reds and names itself. It does NOT
disposition anything: the roster is the observation, and the question
"one fact or coincidence" is still owed per pair. The row is that
question.

## The measured population

Words minted by two or more maps, with the maps that mint them. Taken
from `TAG_INVENTORY` as of `census/py-raise-literals`; the test above
is what keeps it from decaying silently, and it is what to re-derive
from rather than this list.

- band (16): blend_error_tag boolean_error_tag checks_error_tag distribution_kind_tag extrude_error_tag frame_error_tag loft_error_tag node_error_tag path_error_tag profile_error_tag revolve_error_tag shell_classify_error_tag shell_error_tag transform_error_tag tube_error_tag validation_error_tag
- escalated (10): blend_error_tag boolean_error_tag check_evidence_tag naming_error_tag node_error_tag path_error_tag profile_error_tag shell_classify_error_tag shell_error_tag tube_error_tag
- pcurve (5): loft_error_tag revolve_error_tag shell_error_tag transform_error_tag validation_error_tag
- unknown_node (5): edit_error_tag eval_reason_tag export_error_tag inline_error_tag product_error_tag
- node_failed (4): eval_reason_tag export_error_tag hit_test_error_tag interrogate_error_tag
- non_finite (4): distribution_fault_tag expr_dimension_error_tag fmt_quantity_error_tag persist_error_tag
- unknown_param (4): eval_error_tag param_box_error_tag parse_error_tag seed_error_tag
- cap_plane (3): extrude_error_tag loft_error_tag revolve_error_tag
- corrupt (3): shell_error_tag transform_error_tag unexaminable_tag
- face (3): entity_id_tag entity_kind_tag shell_error_tag
- join (3): boolean_error_tag cluster_maintenance_tag split_op_error_tag
- node_not_evaluated (3): eval_reason_tag hit_test_error_tag interrogate_error_tag
- op (3): blend_error_tag extrude_error_tag revolve_error_tag
- pcurves (3): boolean_error_tag split_op_error_tag step_import_error_tag
- structure (3): profile_error_tag skin_error_tag step_import_error_tag
- vertex_vertex (3): census_contact_tag ring_contact_tag stale_declaration_tag
- ambiguous (2): interrogate_error_tag resolve_error_tag
- approx_lane_unsupported (2): transform_error_tag validation_error_tag
- assertion_dimension (2): edit_error_tag node_error_tag
- certify (2): blend_error_tag transform_error_tag
- contact_contradicted (2): boolean_error_tag validation_error_tag
- cosurface_escalated (2): extrude_error_tag revolve_error_tag
- dangling_geometry (2): readback_error_tag validation_error_tag
- dimension (2): edit_error_tag parse_error_tag
- edge (2): entity_id_tag entity_kind_tag
- empty (2): band_error_tag subgroup_tag
- empty_boolean (2): eval_reason_tag export_error_tag
- empty_placement_list (2): edit_error_tag placement_rule_fault_tag
- euler (2): boolean_error_tag loft_error_tag
- evaluation_of_another_document (2): checks_error_tag product_error_tag
- improper_placement (2): edit_error_tag placement_rule_fault_tag
- indeterminate (2): resolution_status_tag structure_refusal_tag
- instance (2): slot_id_tag step_import_error_tag
- io (2): stl_error_tag workspace_error_tag
- measure_malformed (2): edit_error_tag node_error_tag
- no_at_rest_record (2): class_admission_tag mint_refusal_tag
- no_such_body (2): interrogate_error_tag node_pick_error_tag
- node_poisoned (2): hit_test_error_tag interrogate_error_tag
- non_finite_direction (2): node_error_tag path_error_tag
- non_finite_placement (2): edit_error_tag placement_rule_fault_tag
- not_a_body (2): export_error_tag node_pick_error_tag
- null_scaffold_edge (2): tessellate_error_tag unexaminable_tag
- placement_rule_mismatch (2): edit_error_tag placement_rule_fault_tag
- poisoned (2): eval_reason_tag export_error_tag
- profile (2): node_error_tag slot_id_tag
- revolve (2): node_error_tag tube_error_tag
- shell (2): entity_id_tag node_error_tag
- skin (2): loft_error_tag node_error_tag
- sliver_join (2): extrude_error_tag revolve_error_tag
- sliver_rim (2): extrude_error_tag revolve_error_tag
- split (2): cluster_maintenance_tag node_error_tag
- tolerance_conflict (2): node_error_tag persist_error_tag
- transition (2): program_refusal_tag replay_error_tag
- undeclared_contact (2): node_error_tag validation_error_tag
- underflowed_direction (2): node_error_tag path_error_tag
- unnamed (2): hit_test_error_tag naming_error_tag
- unreadable (2): persist_error_tag select_refusal_tag
- validate (2): program_refusal_tag validation_refusal_tag — READ: a
  profile program's own validator against the Python method name
  `Body.validate`, two vocabularies sharing an English word and nothing
  else. Coincidence, decided at `SHARED_TAG_WORDS`.
- vertex (2): entity_id_tag entity_kind_tag
- vertex_on_edge (2): census_contact_tag ring_contact_tag
- vertex_on_face (2): census_contact_tag stale_declaration_tag
- wrong_kind (2): eval_reason_tag interrogate_error_tag

## The shape of the work

Most of these are plainly coincidence — `band` in sixteen refusal maps
is the English word for a numeric interval, not one fact. The ones
worth a reader's judgement are where two maps reason about the same
SUBJECT:

- `vertex_vertex` and `vertex_on_face` across `census_contact_tag`,
  `ring_contact_tag` and `stale_declaration_tag` — three maps over
  contact granularities, which
  `ring-contact-and-census-contact-share-two-words-by-prose-alone`
  carries.
- the evaluation door's words (`unknown_node`, `node_failed`,
  `poisoned`, `empty_boolean`, `wrong_kind`, `node_not_evaluated`)
  shared with `export_error_tag`, `interrogate_error_tag` and
  `hit_test_error_tag` — five doors reporting the same underlying
  node state, where a caller plausibly reads one fact off two of them.
- `face`/`edge`/`vertex` across `entity_id_tag` and `entity_kind_tag`
  are the pair CENSUS-PY-GETTERS pinned; `face`'s third minter
  (`shell_error_tag`) is coincidence.

A lane taking this row does not have to disposition all 54. What it
owes is that the header's rule stop claiming more than has been read,
which the fix pass did, and that each pair it DOES read gets a pin or
a sentence saying the collision is deliberate.

Territory: `crates/pncad-py/*` is LIB's fence and CENSUS's `keep_out`
announces its pncad-py rows there.

## Four more dispositioned, by EDIT's `edit/param-ref-one-convention` (PR #2819)

The four param-ref words — `slot_unknown_doc_param`,
`slot_doc_param_dimension`, `payload_unknown_doc_param`,
`payload_doc_param_dimension` — are now minted at BOTH doors, and
are dispositioned as ONE fact under the convention stated on
`editor_core::EditError`'s enum doc.

Population 63 → 67. Read-count 8 → 12.
