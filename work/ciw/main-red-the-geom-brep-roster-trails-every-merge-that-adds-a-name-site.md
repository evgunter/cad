---
id: main-red-the-geom-brep-roster-trails-every-merge-that-adds-a-name-site
kind: issue
title: main red: geom-brep's predicate roster trails every merge that adds a name site, because it runs only in the slow set
status: open
opened: 2026-11-23
---


`crates/geom-brep/tests/recourse_roster.rs` (`every_decided_name_is_read_and_every_site_is_listed`) is in the slow set. So the PR gate runs it only for a PR that names it, and a PR that adds a `decide*` site through a const, a closed name table, a forwarder or a carrier call merges without it. Main then stays red on it until someone edits the roster.

Found by `sectcone/certified-foot` (PR 7408). That PR edits the roster for its own consts, so its gate ran the row, and over 2026-11-22/23 each CI round met sites from newer merges that the roster did not list:

- `intersect.rs` consts `EXTERNAL`, `INTERNAL`, `PC_PARALLEL_GAP` (from `b1082d54ad`, PR 7354);
- `ssi/certify.rs` `analytic_limbs`: `arm.sup` and `arm.value`;
- `ssi/enclose.rs`'s three `Elevation::gate` calls (PRs 7350 and 7358);
- `pair_section/ruling.rs` `TANGENCY`, and `pair_section/cones.rs`'s `ConeDecision` sites (PR 7357);
- `pcurve_cache/spline_incidence.rs`'s forwarder, moved to `span_incidence_as`;
- `carrier_resolution.rs` `a_far_centred_section_stands_at_the_scale_its_form_is_stored_at: &cut`;
- `pcurve_cache/closed_image_rows.rs` `a_needle_cone_section_passes_with_its_own_beta: &plane`.

PR 7408 declares every one of them, but the next merge that adds a site reds the row again.

The fix is the gate's, not a roster's. Two options:
- select `recourse_roster` (and the sibling crates' rosters) whenever a diff touches a `decide*` spelling under `crates/geom-brep/src`;
- or move the walk out of the slow set: it takes about 2–3 s on CI (`FAIL [ 2.781s]`), above the 0.5 s line, so this would need a cheaper census.

Main has since rewritten the roster (`b8ca8a4118`'s line): it no longer lists indirect sites. What remains is the const-carrier check. A merge that adds a `decide*` through an undeclared const still reds the roster unseen, since it runs only in the slow set.
