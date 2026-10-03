#!/usr/bin/env python3
"""Hand classification of the non-fixture failures: classify.py <dir>...

The rules are by producing suite (read off each test's source; see
report.md section 4), plus the declared-elsewhere join for the pinch
union rows.
"""
import collections, sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from analyze import load, load_declared, reduce, suite_of
from tables import fixture

RULES = {
    # (i) a real undeclared touching tier 3 passes
    "topo::loop_reparenting_pcurve_rows": "i: Euler-built doubled face (mfkrh, equal plane, new key)",
    "sweep::shell5_r2_probes": "i: shell op builds crossing walls (SHELL-4 clearance, #1055)",
    "step-import::probe_dup": "i: twin coincident solids from a STEP file (import gate already 3')",
    # (ii) census refusal on geometry it cannot decide
    "sweep::shell7_dump": "ii: shell result, nested/adjacent curved solids, backstop undecidable",
    "sweep::shell7_r1_diff": "ii: shell result, nested/adjacent curved solids, backstop undecidable",
    "sweep::shell7_r2_probes": "ii: shell result, nested/adjacent curved solids, backstop undecidable",
    "sweep::shell8_dump": "ii: shell result, nested/adjacent curved solids, backstop undecidable",
    "sweep::shell8_r1_probes": "ii: shell result, nested/adjacent curved solids, backstop undecidable",
    "sweep::shell8_r2_probes": "ii: shell result, nested/adjacent curved solids, backstop undecidable",
    "sweep::shell9_r1_probes": "ii: shell result, nested/adjacent curved solids, backstop undecidable",
    "sweep::shell9_r2_dump": "ii: shell result, nested/adjacent curved solids, backstop undecidable",
    "sweep::shell10_r1_probes": "ii: shell result, nested/adjacent curved solids, backstop undecidable",
    "sweep::shell10_r2_probes": "ii: shell result, nested/adjacent curved solids, backstop undecidable",
    "sweep::sf2a_r1": "ii: in-band near-coincidence (profile door definite, census escalates)",
    "sweep::blend4_r1_probes": "ii: in-band near-coincidence on a blend result",
    "sweep::review_blend_k_rk_probes": "i+ii: ignored K-probe, prism thinner than the band at K~1.1",
    # (iii) carries contacts elsewhere
    "sweep::reach_continuation": "iii: boolean result with records; caller gates the bare body at tier 3",
    "sweep::join1_r2_probes": "iii: boolean result with records; caller gates the bare body at tier 3",
    "sweep::pi_seam_and_kiss_through_the_boolean": "iii: boolean result with records; caller gates the bare body at tier 3",
    "sweep::m9_3_wall_door": "iii: boolean result with records; caller gates the bare body at tier 3",
    "topo::m3_pr6_tier3prime": "iii: boolean result with records; caller gates the bare body at tier 3",
    "step-import::review_r1_tier_gate_probes": "iii: imported kiss with declarations",
    "editor-core::docm5_subject": "iii: assembly product; touching declared at the mate layer or refused by design",
    "editor-core::node_labels": "iii: assembly product; touching declared at the mate layer or refused by design",
    "editor-core::part_depth_bound": "iii: assembly product; touching declared at the mate layer or refused by design",
    "editor-core::refusal_concision_chains": "iii: assembly product; touching declared at the mate layer or refused by design",
    # contact-subject tests the module-name filter missed
    "sweep::contfp_reads_arcs_on_their_carriers": "fixture: census test (brick in a lune, corners on ellipses)",
    "sweep::m5_pr9_boss_union": "fixture: touching curved assemblies, declared/undeclared rows",
    "topo::m9_2_census_door": "fixture: overlapping instances",
    "topo::m9_2b_r2_probes": "fixture: cross-instance conformal touch",
    "topo::m9_c1_r1_probes": "fixture: seat rows, wrong/undeclared pairs",
    "topo::m9_c1_rest_face_rung": "fixture: seat rows, wrong/undeclared pairs",
    "topo::bool4r1_probes": "fixture: material containment probes",
}

def classify(suite, declared):
    if suite == "editor-core::union_pinch_member_order":
        return ("iii: pinch union minted WITH records" if declared
                else "i: pinch union whose published record dropped the contact (work/wire/a-boolean-drops-its-operands-own-contact-records.md)")
    return RULES.get(suite, "UNCLASSIFIED")

for d in sys.argv[1:]:
    rows = load(d)
    by, origins = reduce(rows)
    decl = load_declared(d)
    tab = collections.Counter()
    eps = rows[0]["eps"]
    for fp, r in by.items():
        if r["t3_ok"] and not r["p_ok"] and not fixture(origins[fp]):
            s = suite_of(sorted(origins[fp])[0])
            tab[(classify(s, fp in decl), "1" if r["solids"] == 1 else "M")] += 1
    print(f"\n### eps = {eps:g}\n\n| class | 1-solid | multi-solid |\n|---|---|---|")
    for c in sorted({k for k, _ in tab}):
        print(f"| {c} | {tab[(c, '1')]} | {tab[(c, 'M')]} |")
