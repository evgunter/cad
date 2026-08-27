#!/usr/bin/env python3
"""Fit the full model suite and dump posterior summaries + thinned draws."""
import csv
import json
import math
import os
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from models import (fit_count, fit_gauss, fit_logit, fit_ordered_logit,
                    diagnose, contrast)
from mcmc import summarize

HERE = os.path.dirname(os.path.abspath(__file__))
NA = ("NA", "", "-", "—", "None", None)


def na(v):
    return v is None or str(v).strip() in NA


def num(v):
    return None if na(v) else float(v)


def load():
    def rd(p):
        rows = []
        for r in csv.DictReader(open(os.path.join(HERE, p))):
            if r.get("row_id", "").startswith("#"):
                continue
            rows.append(r)
        return {r["row_id"]: r for r in rows}

    core = rd("core-rows.csv")
    cost = rd("labels/cost-parsed.csv")
    char = rd("labels/task-character.csv")
    fixc = rd("labels/fix-convergence.csv")

    # per-MAJOR severity -> per-row counts
    sev = {k: {"in_scope": 0, "unclear": 0, "total": 0} for k in core}
    with open(os.path.join(HERE, "labels/maj-severity.csv")) as f:
        for r in csv.DictReader(f):
            rid = r.get("row_id", "")
            if rid.startswith("#") or rid not in sev:
                continue
            sev[rid]["total"] += 1
            c = r["category"].strip()
            if c == "defect_in_scope":
                sev[rid]["in_scope"] += 1
            elif c == "unclear":
                sev[rid]["unclear"] += 1

    rows = []
    for rid, c in core.items():
        d = dict(c)
        d.update({("cost_" + k): v for k, v in cost.get(rid, {}).items()
                  if k != "row_id"})
        d.update({("ch_" + k): v for k, v in char.get(rid, {}).items()
                  if k != "row_id"})
        d.update({("fx_" + k): v for k, v in fixc.get(rid, {}).items()
                  if k != "row_id"})
        d["sev_in_scope"] = sev[rid]["in_scope"]
        d["sev_unclear"] = sev[rid]["unclear"]
        d["sev_total"] = sev[rid]["total"]
        rows.append(d)
    return rows


RANDOMIZED = ("v2", "v3", "v4")  # v2 pairs, v3 triples, v4 {o,o,o,f} blocks


def v2_quality(rows):
    """All blinded-lane rows under a randomized protocol (v2 or v3)."""
    return [r for r in rows
            if r["protocol"] in RANDOMIZED and r["blinded_lane"] == "1"]


def v2_all(rows):
    return [r for r in rows if r["protocol"] in RANDOMIZED]


def only(rows, proto):
    return [r for r in rows if r["protocol"] == proto]


def is_v3(r):
    return 1.0 if r["protocol"] == "v3" else 0.0


def opus(r):
    return 1.0 if r["arm"] == "opus" else 0.0


def dS(r):
    return 1.0 if r["difficulty"] == "S" else 0.0


def dL(r):
    return 1.0 if r["difficulty"] == "L" else 0.0


def design_main(r):
    """[intercept, opus, S, L]"""
    return [1.0, opus(r), dS(r), dL(r)]


RESULTS = {}


def record(key, title, chains, npar, names, n, contrasts, extra=None,
           note=None, rows_used=None):
    diag = diagnose(chains, npar, names)
    out = {"title": title, "n": n, "diagnostics": diag, "contrasts": {},
           "note": note, "rows_used": rows_used}
    for cname, (j, tf) in contrasts.items():
        summ, draws = contrast(chains, j, tf)
        thin = draws[:: max(1, len(draws) // 2000)]
        out["contrasts"][cname] = {"summary": summ, "draws": thin}
    if extra:
        out.update(extra)
    RESULTS[key] = out
    bad = [d for d in diag if d["rhat"] > 1.01 or d["ess"] < 400]
    flag = "  !! CHECK: %s" % [d["param"] for d in bad] if bad else ""
    key_c = list(contrasts)[0]
    s = out["contrasts"][key_c]["summary"]
    print("  %-34s n=%-3d %s med=%.3f [%.3f, %.3f] P(>ref)=%.2f%s"
          % (key, n, key_c, s["median"], s["q025"], s["q975"], s["p_gt0"], flag))


def count_model(key, title, rows, yfun, family="poisson", prior_scale=1.0):
    y, X, used = [], [], []
    for r in rows:
        v = yfun(r)
        if v is None:
            continue
        y.append(v)
        X.append(design_main(r))
        used.append(r["row_id"])
    psd = [1.5, 0.8 * prior_scale, 1.0, 1.0]
    chains, npar = fit_count(y, X, family=family, prior_sd=psd)
    names = ["intercept", "opus", "diff_S", "diff_L"]
    if family == "negbin":
        names.append("log_phi")
    record(key, title, chains, npar, names, len(y),
           {"rate_ratio_opus_over_fable": (1, math.exp),
            "log_rate_ratio": (1, None)},
           extra={"arm_counts": arm_counts(rows, yfun),
                  "family": family},
           rows_used=used)


def arm_counts(rows, yfun):
    out = {}
    for arm in ("fable", "opus"):
        vals = [yfun(r) for r in rows if r["arm"] == arm and yfun(r) is not None]
        out[arm] = {"n": len(vals),
                    "total": sum(vals) if vals else 0,
                    "mean": (sum(vals) / len(vals)) if vals else None}
    return out


def gauss_model(key, title, rows, yfun, prior_scale=1.0, log=False,
                extra_cols=None, extra_names=None):
    y, X, used = [], [], []
    for r in rows:
        v = yfun(r)
        if v is None:
            continue
        if log:
            if v <= 0:
                continue
            v = math.log(v)
        cols = design_main(r)
        if extra_cols:
            cols = cols + [f(r) for f in extra_cols]
        y.append(v)
        X.append(cols)
        used.append(r["row_id"])
    p = len(X[0])
    psd = [10.0, 2.0 * prior_scale] + [2.0] * (p - 2)
    chains, npar = fit_gauss(y, X, prior_sd=psd)
    names = ["intercept", "opus", "diff_S", "diff_L"] + (extra_names or []) \
        + ["log_sigma"]
    cs = {"opus_minus_fable": (1, None)}
    if log:
        cs = {"ratio_opus_over_fable": (1, math.exp),
              "log_ratio": (1, None)}
    record(key, title, chains, npar, names, len(y), cs,
           extra={"arm_means": arm_means(rows, yfun)}, rows_used=used)


def arm_means(rows, yfun):
    out = {}
    for arm in ("fable", "opus"):
        vals = [yfun(r) for r in rows if r["arm"] == arm and yfun(r) is not None]
        out[arm] = {"n": len(vals),
                    "mean": (sum(vals) / len(vals)) if vals else None,
                    "vals": vals}
    return out


def logit_model(key, title, rows, yfun, with_difficulty=True):
    y, X, used = [], [], []
    for r in rows:
        v = yfun(r)
        if v is None:
            continue
        y.append(v)
        X.append(design_main(r) if with_difficulty else [1.0, opus(r)])
        used.append(r["row_id"])
    chains, npar = fit_logit(y, X)
    names = (["intercept", "opus", "diff_S", "diff_L"] if with_difficulty
             else ["intercept", "opus"])
    record(key, title, chains, npar, names, len(y),
           {"odds_ratio_opus_over_fable": (1, math.exp),
            "log_odds_ratio": (1, None)},
           extra={"arm_means": arm_means(rows, yfun)}, rows_used=used)


# ---------------------------------------------------------------- outcomes

def y_maj(r):
    return num(r["maj"])


def y_min(r):
    return num(r["min"])


def y_note(r):
    return num(r["note"])


def y_total_findings(r):
    a, b, c = num(r["maj"]), num(r["min"]), num(r["note"])
    return None if (a is None or b is None or c is None) else a + b + c


def y_silent(r):
    return num(r["silent"])


def y_any_silent(r):
    v = num(r["silent"])
    return None if v is None else (1.0 if v > 0 else 0.0)


def y_idiom(r):
    return num(r["idiom"])


def y_tests(r):
    return num(r["tests"])


def y_docs(r):
    return num(r["docs"])


def y_rubric_mean(r):
    vs = [num(r["idiom"]), num(r["tests"]), num(r["docs"])]
    if any(v is None for v in vs):
        return None
    return sum(vs) / 3.0


def y_sev_lower(r):
    """consequential MAJ, unclear NOT counted"""
    if r["blinded_lane"] != "1" or na(r["maj"]):
        return None
    return float(r["sev_in_scope"])


def y_sev_upper(r):
    """consequential MAJ, unclear counted as in-scope"""
    if r["blinded_lane"] != "1" or na(r["maj"]):
        return None
    return float(r["sev_in_scope"] + r["sev_unclear"])


def y_tok_impl(r):
    return num(r.get("cost_tok_impl"))


def y_h_impl(r):
    return num(r.get("cost_h_impl"))


def y_h_any(r):
    v = num(r.get("cost_h_impl"))
    return v if v is not None else num(r.get("cost_h_total_stated"))


def y_tok_recorded_total(r):
    parts = [num(r.get("cost_tok_impl")), num(r.get("cost_tok_fix")),
             num(r.get("cost_tok_review"))]
    parts = [p for p in parts if p is not None]
    if parts:
        return sum(parts)
    return num(r.get("cost_tok_total_stated"))


def y_gate_red_any(r):
    v = num(r.get("fx_gate_red_rounds"))
    return None if v is None else (1.0 if v > 0 else 0.0)


def y_converged(r):
    v = r.get("fx_converged_clean")
    return None if na(v) else float(v)


def y_fix_size(r):
    return num(r.get("fx_fix_size"))


def y_self_caught(r):
    v = r.get("fx_self_caught")
    return None if na(v) else float(v)


def main():
    t0 = time.time()
    rows = load()
    Q = v2_quality(rows)
    A = v2_all(rows)
    print("v2 quality rows: %d   v2 all rows: %d" % (len(Q), len(A)))

    print("\n== review findings (raw counts, pre-registered metric) ==")
    count_model("maj_raw", "MAJOR findings per dispatch", Q, y_maj)
    count_model("maj_raw_nb", "MAJOR findings (negative binomial)", Q, y_maj,
                family="negbin")
    count_model("min_raw", "MINOR findings per dispatch", Q, y_min)
    count_model("note_raw", "NOTE findings per dispatch", Q, y_note)
    count_model("findings_total", "All findings per dispatch", Q,
                y_total_findings)

    print("\n== severity-recoded MAJORs (blinded hand-coding) ==")
    count_model("maj_conseq_lower", "Consequential MAJORs (unclear excluded)",
                Q, y_sev_lower)
    count_model("maj_conseq_upper", "Consequential MAJORs (unclear included)",
                Q, y_sev_upper)

    print("\n== silent deviations (protocol's worst-weighted metric) ==")
    count_model("silent_count", "Silent deviations per dispatch", Q, y_silent)
    logit_model("silent_any", "Any silent deviation", Q, y_any_silent)

    print("\n== rubric quality ratings (1-5) ==")
    gauss_model("rubric_mean", "Rubric mean (idiom/tests/docs)", Q,
                y_rubric_mean)
    gauss_model("rubric_idiom", "Idiom / structure rating", Q, y_idiom)
    gauss_model("rubric_tests", "Test quality rating", Q, y_tests)
    gauss_model("rubric_docs", "Doc / comment honesty rating", Q, y_docs)

    print("\n== cost ==")
    gauss_model("tok_impl", "Implementation tokens (log scale)", A,
                y_tok_impl, log=True)
    gauss_model("tok_total", "Total recorded tokens (log, phases conflated)",
                A, y_tok_recorded_total, log=True)
    gauss_model("h_impl", "Implementation wall-clock hours (log)", A,
                y_h_impl, log=True)
    gauss_model("h_any", "Any wall-clock hours (log, + contamination covar)",
                A, y_h_any, log=True,
                extra_cols=[lambda r: 1.0 if r.get("cost_contaminated") == "1"
                            else 0.0],
                extra_names=["gap_contaminated"])

    print("\n== fix-pass behaviour (debugging proxy) ==")
    ordered_fix(Q)
    logit_model("gate_red_any", "Any red gate round", Q, y_gate_red_any)
    logit_model("converged", "Fix pass converged clean", Q, y_converged)
    logit_model("self_caught", "Implementer self-caught an error", Q,
                y_self_caught, with_difficulty=False)

    print("\n== does the effect depend on S/M/L? (arm x difficulty) ==")
    interaction_difficulty(Q, y_maj, "maj_by_difficulty", "MAJORs by difficulty")
    interaction_difficulty(Q, y_rubric_mean, "rubric_by_difficulty",
                           "Rubric mean by difficulty", gauss=True)

    print("\n== debugger vs designer (character interaction) ==")
    character_models(Q)

    print("\n== protocol era: does v3 (2:1 triples) look like v2? ==")
    count_model("maj_v2_only", "MAJORs, protocol v2 rows only",
                only(Q, "v2"), y_maj)
    count_model("maj_v3_only", "MAJORs, protocol v3 rows only",
                only(Q, "v3"), y_maj)
    era_model(Q)

    print("\n== prior sensitivity on the headline ==")
    count_model("maj_prior_half", "MAJORs, prior half width", Q, y_maj,
                prior_scale=0.5)
    count_model("maj_prior_double", "MAJORs, prior double width", Q, y_maj,
                prior_scale=2.0)

    with open(os.path.join(HERE, "results.json"), "w") as f:
        json.dump(RESULTS, f)
    print("\nwrote results.json  (%.1fs)" % (time.time() - t0))


def ordered_fix(rows):
    y, X, used = [], [], []
    for r in rows:
        v = y_fix_size(r)
        if v is None:
            continue
        y.append(int(v))
        X.append([opus(r), dS(r), dL(r)])
        used.append(r["row_id"])
    chains, npar = fit_ordered_logit(y, X, n_cat=5)
    names = ["opus", "diff_S", "diff_L", "cut1", "d2", "d3", "d4"]
    record("fix_size", "Fix-pass size (ordered logit, 0=none..4=heavy)",
           chains, npar, names, len(y),
           {"odds_ratio_bigger_fix_opus": (0, math.exp),
            "log_odds": (0, None)},
           extra={"arm_means": arm_means(rows, y_fix_size)}, rows_used=used)


def era_model(rows):
    """MAJOR rate with an era main effect and an arm x era interaction.

    Guards against reading a protocol-era shift as an arm effect: v3
    allocates 2:1 toward opus, so if the project got easier or the reviews
    got softer over time, the pooled arm contrast would absorb it.
    """
    y, X, used = [], [], []
    for r in rows:
        v = y_maj(r)
        if v is None:
            continue
        o, e = opus(r), is_v3(r)
        y.append(v)
        X.append([1.0, o, e, o * e, dS(r), dL(r)])
        used.append(r["row_id"])
    chains, npar = fit_count(y, X, prior_sd=[1.5] + [1.0] * 5)
    record("maj_era", "MAJORs: arm x protocol era", chains, npar,
           ["intercept", "opus", "era_v3", "opus_x_v3", "diff_S", "diff_L"],
           len(y),
           {"arm_effect_in_v2": (1, math.exp),
            "era_main_effect": (2, math.exp),
            "arm_x_era": (3, math.exp)},
           rows_used=used)


def interaction_difficulty(rows, yfun, key, title, gauss=False):
    y, X, used = [], [], []
    for r in rows:
        v = yfun(r)
        if v is None:
            continue
        o = opus(r)
        y.append(v)
        X.append([1.0, dS(r), dL(r), o * (1 - dS(r)) * (1 - dL(r)),
                  o * dS(r), o * dL(r)])
        used.append(r["row_id"])
    names = ["intercept", "diff_S", "diff_L", "opus_in_M", "opus_in_S",
             "opus_in_L"]
    if gauss:
        chains, npar = fit_gauss(y, X, prior_sd=[10.0] + [2.0] * 5)
        names = names + ["log_sigma"]
        cs = {"opus_in_M": (3, None), "opus_in_S": (4, None),
              "opus_in_L": (5, None)}
    else:
        chains, npar = fit_count(y, X, prior_sd=[1.5] + [1.0] * 5)
        cs = {"opus_in_M": (3, math.exp), "opus_in_S": (4, math.exp),
              "opus_in_L": (5, math.exp)}
    record(key, title, chains, npar, names, len(y), cs, rows_used=used)


def character_models(rows):
    """arm x task-character, plus continuous design/debug load moderators."""
    def build_new(r):
        return 1.0 if r.get("ch_character") == "build_new" else 0.0

    def load_c(field):
        vals = [num(r.get(field)) for r in rows if not na(r.get(field))]
        mu = sum(vals) / len(vals)
        return lambda r: (num(r.get(field)) - mu) if not na(r.get(field)) else 0.0

    debug_c = load_c("ch_debug_load")
    design_c = load_c("ch_design_load")

    for key, title, yfun, gauss in [
        ("char_maj", "MAJORs: arm x build-new", y_maj, False),
        ("char_rubric", "Rubric mean: arm x build-new", y_rubric_mean, True),
    ]:
        y, X, used = [], [], []
        for r in rows:
            v = yfun(r)
            if v is None:
                continue
            o, b = opus(r), build_new(r)
            y.append(v)
            X.append([1.0, o, b, o * b, dS(r), dL(r)])
            used.append(r["row_id"])
        names = ["intercept", "opus", "build_new", "opus_x_build_new",
                 "diff_S", "diff_L"]
        if gauss:
            chains, npar = fit_gauss(y, X, prior_sd=[10.0] + [2.0] * 5)
            names += ["log_sigma"]
            cs = {"opus_x_build_new": (3, None), "opus_main": (1, None)}
        else:
            chains, npar = fit_count(y, X, prior_sd=[1.5] + [1.0] * 5)
            cs = {"opus_x_build_new": (3, math.exp), "opus_main": (1, math.exp)}
        record(key, title, chains, npar, names, len(y), cs, rows_used=used)

    for key, title, yfun, mod, mname, gauss in [
        ("mod_debug_maj", "MAJORs: arm x debug-load", y_maj, debug_c,
         "debug_load", False),
        ("mod_design_maj", "MAJORs: arm x design-load", y_maj, design_c,
         "design_load", False),
        ("mod_debug_rubric", "Rubric: arm x debug-load", y_rubric_mean,
         debug_c, "debug_load", True),
        ("mod_design_rubric", "Rubric: arm x design-load", y_rubric_mean,
         design_c, "design_load", True),
    ]:
        y, X, used = [], [], []
        for r in rows:
            v = yfun(r)
            if v is None:
                continue
            o, m = opus(r), mod(r)
            y.append(v)
            X.append([1.0, o, m, o * m, dS(r), dL(r)])
            used.append(r["row_id"])
        names = ["intercept", "opus", mname, "opus_x_" + mname, "diff_S",
                 "diff_L"]
        if gauss:
            chains, npar = fit_gauss(y, X, prior_sd=[10.0] + [2.0] * 5)
            names += ["log_sigma"]
            cs = {"interaction": (3, None), "opus_at_mean": (1, None)}
        else:
            chains, npar = fit_count(y, X, prior_sd=[1.5] + [1.0] * 5)
            cs = {"interaction": (3, math.exp), "opus_at_mean": (1, math.exp)}
        record(key, title, chains, npar, names, len(y), cs, rows_used=used)


if __name__ == "__main__":
    main()
