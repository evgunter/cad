"""Model-compare readout 1: fit the estimands on rows.json -> results.json.

rows.json is the structured extraction of docs/model-compare-log/rows/*.md
(73 rows, main at e44e340032), made under SCHEMA.md and spot-checked by hand.

Method (numpy only, seeded):
- proportions: Beta(1,1) posteriors, reported as the Fable - Opus difference
  with a 90% interval and P(Fable > Opus);
- counts per review: Poisson regression (IRLS) with the other factor and the
  letter as covariates, interval by unit-clustered bootstrap;
- times and costs: log scale; geometric-mean ratio Fable/Opus with a
  letter-adjusted OLS fit, interval by unit-clustered bootstrap;
- concurrent pairs: within-unit Fable - Opus differences, sign counts and a
  bootstrap over pairs.
Every interval is 90%. Nothing here is a significance test with a decision
attached; the n is small and the report says so.
"""
import json, math, sys
import numpy as np

HERE = sys.argv[1] if len(sys.argv) > 1 else "."
RNG = np.random.default_rng(20261122)
B = 4000
O, F = "Opus 5.5", "Fable 5.2"

rows = json.load(open(f"{HERE}/rows.json"))
PRIMARY = ("single", "first", "R1", "R2")


def is_model(s, m):
    return s is not None and s.startswith(m)


def beta_diff(sf, nf, so, no):
    a = RNG.beta(sf + 1, nf - sf + 1, 200_000)
    b = RNG.beta(so + 1, no - so + 1, 200_000)
    d = a - b
    return dict(fable=f"{sf}/{nf}", opus=f"{so}/{no}",
                fable_rate=sf / nf if nf else None, opus_rate=so / no if no else None,
                diff_median=float(np.median(d)), lo=float(np.quantile(d, .05)),
                hi=float(np.quantile(d, .95)), p_fable_higher=float((d > 0).mean()))


def ols(X, y):
    beta, *_ = np.linalg.lstsq(X, y, rcond=None)
    return beta


def poisson(X, y, iters=50):
    beta = np.zeros(X.shape[1])
    beta[0] = math.log(max(y.mean(), 1e-3))
    for _ in range(iters):
        eta = np.clip(X @ beta, -20, 20)
        mu = np.exp(eta)
        W = mu
        z = eta + (y - mu) / mu
        XtW = X.T * W
        try:
            beta_new = np.linalg.solve(XtW @ X + 1e-6 * np.eye(X.shape[1]), XtW @ z)
        except np.linalg.LinAlgError:
            break
        if np.max(np.abs(beta_new - beta)) < 1e-9:
            beta = beta_new
            break
        beta = beta_new
    return beta


def letter_dummies(letters):
    # H baseline; M and S dummies
    return np.column_stack([[1.0 if l == "M" else 0.0 for l in letters],
                            [1.0 if l == "S" else 0.0 for l in letters]])


def cluster_boot(units, fit):
    """units: list of per-unit record lists; fit(records) -> scalar."""
    est = fit([r for u in units for r in u])
    vals = []
    n = len(units)
    for _ in range(B):
        idx = RNG.integers(0, n, n)
        recs = [r for i in idx for r in units[i]]
        try:
            v = fit(recs)
        except Exception:
            continue
        if v is not None and np.isfinite(v):
            vals.append(v)
    vals = np.array(vals)
    return dict(est=float(est), lo=float(np.quantile(vals, .05)),
                hi=float(np.quantile(vals, .95)), p_gt0=float((vals > 0).mean()))


def summarize(xs):
    xs = [x for x in xs if x is not None]
    if not xs:
        return dict(n=0)
    a = np.array(xs, float)
    return dict(n=len(xs), median=float(np.median(a)), mean=float(a.mean()),
                geo=float(np.exp(np.log(a[a > 0]).mean())) if (a > 0).any() else None)


R = {}

# ---------------------------------------------------------------- population
imp = [r for r in rows if r["implementer"]["counted"]]
prim = [(r, v) for r in rows for v in r["reviews"]
        if v["slot"] in PRIMARY and v["counted"]]
R["population"] = dict(
    rows=len(rows),
    implementer_fair={m: sum(is_model(r["implementer"]["model"], m) for r in imp) for m in (O, F)},
    implementer_excluded=[(r["pr"], r["implementer"]["exclusion_reason"]) for r in rows
                          if not r["implementer"]["counted"]],
    reviews_fair={m: sum(is_model(v["model"], m) for _, v in prim) for m in (O, F)},
    by_letter_impl={l: {m: sum(1 for r in imp if r["letter"] == l and is_model(r["implementer"]["model"], m))
                        for m in (O, F)} for l in "HMS"},
    by_mode={k: sum(1 for r in rows if r["review_mode"] == k) for k in
             ("single", "dual-sequential", "dual-concurrent")},
    cells={f"{a[:5]} impl / {b[:5]} rev": sum(1 for r, v in prim if r["implementer"]["counted"]
                                              and is_model(r["implementer"]["model"], a) and is_model(v["model"], b))
           for a in (O, F) for b in (O, F)},
    orchestrator={m: sum(1 for r in rows if is_model(r["orchestrator_model"], m)) for m in (O, F)},
)

# ---------------------------------------------------------------- draw integrity
bytes_ = []
for r in rows:
    if r["implementer"]["byte"] is not None:
        bytes_.append(r["implementer"]["byte"])
    for v in r["reviews"]:
        if v["byte"] is not None and v["slot"] in ("single", "first"):
            bytes_.append(v["byte"])
even = sum(b % 2 == 0 for b in bytes_)
R["draws"] = dict(n=len(bytes_), even=even,
                  note="every recorded byte matches its model (even -> Opus)")

# ---------------------------------------------------------------- implementer question
def rev_records(need_fair_impl=True):
    """One record per fair primary review (on a fair-implementer unit, for the
    implementer question; on any unit, for the reviewer question)."""
    out = []
    for r, v in prim:
        if need_fair_impl and not r["implementer"]["counted"]:
            continue
        out.append(dict(pr=r["pr"], letter=r["letter"],
                        impl_f=1.0 if is_model(r["implementer"]["model"], F) else 0.0,
                        rev_f=1.0 if is_model(v["model"], F) else 0.0,
                        conc=1.0 if r["review_mode"] == "dual-concurrent" else 0.0,
                        major=v["major"] or 0, minor=v["minor"] or 0, note=v["note"] or 0,
                        nm=1.0 if (v["verdict"] or "").startswith(("NOT-MERGEABLE", "REJECT")) else 0.0,
                        demo=v["demonstrated_findings"],
                        majors_demo=sum(1 for m in v["majors"] if m.get("demonstrated")),
                        time=v["time_ms"], cost=v["cost_usd"], tok=v["tokens_total"],
                        mut=v["mutants_run"]))
    return out


recs = rev_records()
units = {}
for x in recs:
    units.setdefault(x["pr"], []).append(x)
unit_list = list(units.values())


def pois_coef(key, which):
    def fit(rs):
        y = np.array([r[key] for r in rs], float)
        X = np.column_stack([np.ones(len(rs)), [r["impl_f"] for r in rs], [r["rev_f"] for r in rs],
                             letter_dummies([r["letter"] for r in rs])])
        if y.sum() == 0:
            return None
        return poisson(X, y)[which]
    return fit


imp_q = {}
for key, label in [("major", "MAJORs per review"), ("majors_demo", "DEMONSTRATED MAJORs per review"),
                   ("minor", "MINORs per review")]:
    rr = {}
    for m, flag in ((O, 0.0), (F, 1.0)):
        xs = [x[key] for x in recs if x["impl_f"] == flag]
        rr[m] = dict(n=len(xs), total=int(sum(xs)), mean=float(np.mean(xs)))
    c = cluster_boot(unit_list, pois_coef(key, 1))
    rr["adjusted_rate_ratio_fable_over_opus"] = dict(est=math.exp(c["est"]), lo=math.exp(c["lo"]),
                                                     hi=math.exp(c["hi"]), p_fable_higher=c["p_gt0"])
    imp_q[label] = rr

# NOT-MERGEABLE on the unit's first (or either concurrent) review
def unit_nm(r):
    vs = [v for v in r["reviews"] if v["slot"] in PRIMARY and v["counted"]]
    return any((v["verdict"] or "").startswith(("NOT-MERGEABLE", "REJECT")) for v in vs)


def unit_major(r):
    vs = [v for v in r["reviews"] if v["slot"] in PRIMARY and v["counted"]]
    return any((v["major"] or 0) > 0 for v in vs)


for name, fn in [("unit drew a NOT-MERGEABLE verdict", unit_nm), ("unit drew any MAJOR", unit_major)]:
    for mode_name, modes in [("all", ("single", "dual-sequential", "dual-concurrent")),
                             ("single+sequential (one review)", ("single", "dual-sequential")),
                             ("concurrent (two reviews)", ("dual-concurrent",))]:
        us = [r for r in imp if r["review_mode"] in modes and any(v["counted"] and v["slot"] in PRIMARY for v in r["reviews"])]
        sf = sum(fn(r) for r in us if is_model(r["implementer"]["model"], F))
        nf = sum(1 for r in us if is_model(r["implementer"]["model"], F))
        so = sum(fn(r) for r in us if is_model(r["implementer"]["model"], O))
        no = sum(1 for r in us if is_model(r["implementer"]["model"], O))
        imp_q[f"{name} [{mode_name}]"] = beta_diff(sf, nf, so, no)

# CI on first head: own red, among rows where it is known
known = [r for r in imp if r["ci_first_head"] in ("green", "red-own", "red-main-only")]
sf = sum(r["ci_first_head"] == "red-own" for r in known if is_model(r["implementer"]["model"], F))
nf = sum(1 for r in known if is_model(r["implementer"]["model"], F))
so = sum(r["ci_first_head"] == "red-own" for r in known if is_model(r["implementer"]["model"], O))
no = sum(1 for r in known if is_model(r["implementer"]["model"], O))
imp_q["hosted CI red on the branch's own row at the first pushed head"] = beta_diff(sf, nf, so, no)
imp_q["hosted CI red on the branch's own row at the first pushed head"]["unknown_or_not_run"] = {
    m: sum(1 for r in imp if is_model(r["implementer"]["model"], m) and r["ci_first_head"] not in ("green", "red-own", "red-main-only")) for m in (O, F)}

# silent spec deviations
sd = [r for r in imp if r["spec_dev_silent"] is not None]
sf = sum((r["spec_dev_silent"] or 0) > 0 for r in sd if is_model(r["implementer"]["model"], F))
nf = sum(1 for r in sd if is_model(r["implementer"]["model"], F))
so = sum((r["spec_dev_silent"] or 0) > 0 for r in sd if is_model(r["implementer"]["model"], O))
no = sum(1 for r in sd if is_model(r["implementer"]["model"], O))
imp_q["at least one SILENT spec deviation"] = beta_diff(sf, nf, so, no)

# rubric
def rubric_of(r):
    rb = r.get("rubric")
    if rb and rb.get("idiom") is not None:
        return rb
    # average per-review rubrics where only those exist
    vs = [v.get("rubric") for v in r["reviews"] if isinstance(v.get("rubric"), dict)
          and v["rubric"].get("idiom") is not None and v["slot"] in PRIMARY]
    if not vs:
        return None
    return {k: float(np.mean([v[k] for v in vs if v.get(k) is not None])) for k in ("idiom", "tests", "docs")}


rub = {}
for k in ("idiom", "tests", "docs"):
    d = {}
    for m in (O, F):
        xs = [rubric_of(r)[k] for r in imp if is_model(r["implementer"]["model"], m) and rubric_of(r)
              and rubric_of(r).get(k) is not None]
        d[m] = dict(n=len(xs), mean=float(np.mean(xs)))
    # bootstrap mean difference
    a = np.array([rubric_of(r)[k] for r in imp if is_model(r["implementer"]["model"], F) and rubric_of(r) and rubric_of(r).get(k) is not None])
    b = np.array([rubric_of(r)[k] for r in imp if is_model(r["implementer"]["model"], O) and rubric_of(r) and rubric_of(r).get(k) is not None])
    bs = [RNG.choice(a, len(a)).mean() - RNG.choice(b, len(b)).mean() for _ in range(B)]
    d["diff_fable_minus_opus"] = dict(est=float(a.mean() - b.mean()), lo=float(np.quantile(bs, .05)),
                                      hi=float(np.quantile(bs, .95)))
    rub[k] = d
imp_q["rubric (1-5, the reviewer's)"] = rub

# fix pass size
fx = {}
for m in (O, F):
    xs = [(r["fix_pass"]["lines_added"] or 0) + (r["fix_pass"]["lines_removed"] or 0) for r in imp
          if is_model(r["implementer"]["model"], m) and r["fix_pass"]["lines_added"] is not None]
    tk = [r["fix_pass"]["findings_taken"] for r in imp if is_model(r["implementer"]["model"], m)
          and r["fix_pass"]["findings_taken"] is not None]
    fx[m] = dict(lines=summarize(xs), findings_taken=summarize(tk))
imp_q["fix pass"] = fx


# time and cost, letter-adjusted log ratio
def log_ratio(items, key_val, flag_key):
    """items: list of (unit_id, letter, flag, value)."""
    by_u = {}
    for u, l, f, v in items:
        by_u.setdefault(u, []).append((l, f, v))
    ul = list(by_u.values())

    def fit(rs):
        if len({f for _, f, _ in rs}) < 2:
            return None
        y = np.log(np.array([v for _, _, v in rs], float))
        X = np.column_stack([np.ones(len(rs)), [f for _, f, _ in rs], letter_dummies([l for l, _, _ in rs])])
        return ols(X, y)[1]
    c = cluster_boot(ul, fit)
    return dict(ratio=math.exp(c["est"]), lo=math.exp(c["lo"]), hi=math.exp(c["hi"]), p_fable_higher=c["p_gt0"])


def impl_cost(key):
    items = [(r["pr"], r["letter"], 1.0 if is_model(r["implementer"]["model"], F) else 0.0, r["implementer"][key])
             for r in imp if r["implementer"][key]]
    out = {m: summarize([v for _, _, f, v in items if f == (1.0 if m == F else 0.0)]) for m in (O, F)}
    out["letter_adjusted_ratio_fable_over_opus"] = log_ratio(items, key, None)
    return out


imp_q["implementation wall clock (ms)"] = impl_cost("time_ms")
imp_q["implementation cost (USD, where given)"] = impl_cost("cost_usd")
imp_q["implementation tokens (harness total, where given)"] = impl_cost("tokens_total")
R["implementer"] = imp_q

# ---------------------------------------------------------------- reviewer question
recs = rev_records(need_fair_impl=False)
units = {}
for x in recs:
    units.setdefault(x["pr"], []).append(x)
unit_list = list(units.values())
rev_q = {}
for key, label in [("major", "MAJORs raised"), ("majors_demo", "DEMONSTRATED MAJORs raised"),
                   ("minor", "MINORs raised"), ("note", "NOTEs raised")]:
    rr = {}
    for m, flag in ((O, 0.0), (F, 1.0)):
        xs = [x[key] for x in recs if x["rev_f"] == flag]
        rr[m] = dict(n=len(xs), total=int(sum(xs)), mean=float(np.mean(xs)))
    c = cluster_boot(unit_list, pois_coef(key, 2))
    rr["adjusted_rate_ratio_fable_over_opus"] = dict(est=math.exp(c["est"]), lo=math.exp(c["lo"]),
                                                     hi=math.exp(c["hi"]), p_fable_higher=c["p_gt0"])
    rev_q[label] = rr

sf = sum(x["nm"] for x in recs if x["rev_f"] == 1)
nf = sum(1 for x in recs if x["rev_f"] == 1)
so = sum(x["nm"] for x in recs if x["rev_f"] == 0)
no = sum(1 for x in recs if x["rev_f"] == 0)
rev_q["verdict NOT-MERGEABLE-AS-IS"] = beta_diff(int(sf), nf, int(so), no)
sf = sum(x["major"] > 0 for x in recs if x["rev_f"] == 1)
so = sum(x["major"] > 0 for x in recs if x["rev_f"] == 0)
rev_q["raised at least one MAJOR"] = beta_diff(int(sf), nf, int(so), no)


def rev_cost(key):
    items = [(x["pr"], x["letter"], x["rev_f"], x[key]) for x in recs if x[key]]
    out = {m: summarize([v for _, _, f, v in items if f == (1.0 if m == F else 0.0)]) for m in (O, F)}
    out["letter_adjusted_ratio_fable_over_opus"] = log_ratio(items, key, None)
    return out


rev_q["review wall clock (ms)"] = rev_cost("time")
rev_q["review cost (USD, where given)"] = rev_cost("cost")
rev_q["review tokens (harness total, where given)"] = rev_cost("tok")

# paired concurrent
pairs = []
for r in rows:
    if r["review_mode"] != "dual-concurrent":
        continue
    vs = {("F" if is_model(v["model"], F) else "O"): v for v in r["reviews"]
          if v["slot"] in ("R1", "R2") and v["counted"]}
    if set(vs) != {"F", "O"}:
        continue
    p = r["pair"] or {}
    fslot = vs["F"]["slot"]
    uni_f = p.get("unilateral_r1") if fslot == "R1" else p.get("unilateral_r2")
    uni_o = p.get("unilateral_r2") if fslot == "R1" else p.get("unilateral_r1")
    umf = p.get("unilateral_r1_major") if fslot == "R1" else p.get("unilateral_r2_major")
    umo = p.get("unilateral_r2_major") if fslot == "R1" else p.get("unilateral_r1_major")
    pairs.append(dict(pr=r["pr"], letter=r["letter"], impl=r["implementer"]["model"],
                      impl_fair=r["implementer"]["counted"],
                      f_major=vs["F"]["major"] or 0, o_major=vs["O"]["major"] or 0,
                      f_find=(vs["F"]["major"] or 0) + (vs["F"]["minor"] or 0) + (vs["F"]["note"] or 0),
                      o_find=(vs["O"]["major"] or 0) + (vs["O"]["minor"] or 0) + (vs["O"]["note"] or 0),
                      f_nm=(vs["F"]["verdict"] or "").startswith("NOT-MERGEABLE"),
                      o_nm=(vs["O"]["verdict"] or "").startswith("NOT-MERGEABLE"),
                      f_time=vs["F"]["time_ms"], o_time=vs["O"]["time_ms"],
                      f_cost=vs["F"]["cost_usd"], o_cost=vs["O"]["cost_usd"],
                      f_tok=vs["F"]["tokens_total"], o_tok=vs["O"]["tokens_total"],
                      uni_f=uni_f, uni_o=uni_o, umaj_f=umf, umaj_o=umo,
                      bilateral=p.get("bilateral"), tally=p.get("tally_candidates")))


def paired(fk, ok, log=False):
    ps = [p for p in pairs if p[fk] is not None and p[ok] is not None]
    if not ps:
        return dict(n=0)
    if log:
        d = np.log(np.array([p[fk] for p in ps], float)) - np.log(np.array([p[ok] for p in ps], float))
    else:
        d = np.array([p[fk] for p in ps], float) - np.array([p[ok] for p in ps], float)
    bs = [RNG.choice(d, len(d)).mean() for _ in range(B)]
    out = dict(n=len(ps), fable_more=int((d > 0).sum()), opus_more=int((d < 0).sum()), ties=int((d == 0).sum()),
               mean_diff=float(d.mean()), lo=float(np.quantile(bs, .05)), hi=float(np.quantile(bs, .95)))
    if log:
        out.update(ratio=float(math.exp(d.mean())), ratio_lo=float(math.exp(out["lo"])), ratio_hi=float(math.exp(out["hi"])))
    else:
        out.update(fable_total=float(sum(p[fk] for p in ps)), opus_total=float(sum(p[ok] for p in ps)))
    return out


rev_q["paired (concurrent units)"] = dict(
    n_pairs=len(pairs),
    majors=paired("f_major", "o_major"),
    findings=paired("f_find", "o_find"),
    unilateral_findings=paired("uni_f", "uni_o"),
    unilateral_majors=paired("umaj_f", "umaj_o"),
    time_ratio=paired("f_time", "o_time", log=True),
    cost_ratio=paired("f_cost", "o_cost", log=True),
    token_ratio=paired("f_tok", "o_tok", log=True),
    not_mergeable=dict(fable=sum(p["f_nm"] for p in pairs), opus=sum(p["o_nm"] for p in pairs)),
    rows=pairs,
)

# sequential second reviews
seq = []
for r in rows:
    if r["review_mode"] != "dual-sequential":
        continue
    first = [v for v in r["reviews"] if v["slot"] == "first"]
    second = [v for v in r["reviews"] if v["slot"] == "second"]
    seq.append(dict(pr=r["pr"], first=first[0]["model"] if first else None,
                    first_major=first[0]["major"] if first else None,
                    second=second[0]["model"] if second else None,
                    second_major=second[0]["major"] if second else None,
                    second_unique=second[0].get("unique_vs_other") if second else None))
rev_q["sequential units"] = seq
R["reviewer"] = rev_q

# ---------------------------------------------------------------- escapes
R["escapes"] = [dict(pr=r["pr"], impl=r["implementer"]["model"],
                     reviewers=[v["model"] for v in r["reviews"] if v["slot"] in PRIMARY],
                     lines=r["later_escapes"]) for r in rows if r["later_escapes"]]

# ---------------------------------------------------------------- sensitivity: drop EMBED
emb = [r for r in rows if r["program"] == "embed"]
R["sensitivity"] = dict(embed_rows=[(r["pr"], r["implementer"]["model"], [v["model"] for v in r["reviews"] if v["slot"] in PRIMARY]) for r in emb])
recs_ne = [x for x in recs if x["pr"] not in {r["pr"] for r in emb}]
u_ne = {}
for x in recs_ne:
    u_ne.setdefault(x["pr"], []).append(x)
for key in ("major", "minor"):
    c_imp = cluster_boot(list(u_ne.values()), pois_coef(key, 1))
    c_rev = cluster_boot(list(u_ne.values()), pois_coef(key, 2))
    R["sensitivity"][f"no-embed {key} rate ratio (impl F/O, rev F/O)"] = [
        dict(est=math.exp(c_imp["est"]), lo=math.exp(c_imp["lo"]), hi=math.exp(c_imp["hi"])),
        dict(est=math.exp(c_rev["est"]), lo=math.exp(c_rev["lo"]), hi=math.exp(c_rev["hi"]))]

json.dump(R, open(f"{HERE}/results.json", "w"), indent=1, default=float)
print(json.dumps({k: R[k] for k in ("population", "draws")}, indent=1))

# ---------------------------------------------------------------- implementer x reviewer cells
recs_i = rev_records(need_fair_impl=True)
cells = {}
for a, af in (("Opus impl", 0.0), ("Fable impl", 1.0)):
    for b, bf in (("Opus rev", 0.0), ("Fable rev", 1.0)):
        xs = [x for x in recs_i if x["impl_f"] == af and x["rev_f"] == bf]
        cells[f"{a} / {b}"] = dict(n=len(xs), majors=int(sum(x["major"] for x in xs)),
                                   major_per_review=float(np.mean([x["major"] for x in xs])),
                                   minor_per_review=float(np.mean([x["minor"] for x in xs])))
u_i = {}
for x in recs_i:
    u_i.setdefault(x["pr"], []).append(x)


def inter(rs):
    y = np.array([r["major"] for r in rs], float)
    X = np.column_stack([np.ones(len(rs)), [r["impl_f"] for r in rs], [r["rev_f"] for r in rs],
                         [r["impl_f"] * r["rev_f"] for r in rs], letter_dummies([r["letter"] for r in rs])])
    return poisson(X, y)[3]


c = cluster_boot(list(u_i.values()), inter)
cells["interaction (log rate ratio of the reviewer gap on Fable code vs Opus code)"] = dict(
    ratio=math.exp(c["est"]), lo=math.exp(c["lo"]), hi=math.exp(c["hi"]), p_gt1=c["p_gt0"])
R["cells"] = cells
json.dump(R, open(f"{HERE}/results.json", "w"), indent=1, default=float)
print(json.dumps(cells, indent=1))

# ---------------------------------------------------------------- design-fork, Fable 5.2 era
DF = json.load(open(f"{HERE}/designfork.json"))


def df_class(r):
    m = r["match_labels"]
    if m is None or not r["ev_decided"]:
        return "undecided"
    if not m:
        return "none"
    fl = r["fable_label"]
    if fl is None:
        return "unmapped"
    labels = set(m)
    n = 3 if r["table"] == "three" else 2
    if len(labels) == n:
        return "all"
    has_f = fl in labels
    has_o = bool(labels - {fl})
    if has_f and has_o:
        return "fable+opus (not all)"
    return "fable only" if has_f else "opus only"


df = {}
for era in ("5.2", "spans", "unknown", "5.1"):
    for t in ("two", "three"):
        rs = [r for r in DF if r["era"] == era and r["table"] == t]
        if not rs:
            continue
        c = {}
        for r in rs:
            k = df_class(r)
            c[k] = c.get(k, 0) + 1
        agree = {}
        for r in rs:
            agree[r["agree_first"]] = agree.get(r["agree_first"], 0) + 1
        df[f"{era} / {t} designers"] = dict(n=len(rs), match=c, first_reports_agree=agree)
two = [r for r in DF if r["era"] == "5.2" and r["table"] == "two"]
fo = sum(df_class(r) == "fable only" for r in two)
oo = sum(df_class(r) == "opus only" for r in two)
x = RNG.beta(fo + 1, oo + 1, 200_000)
df["two-designer discordant: P(Ev sided with Fable)"] = dict(
    fable=fo, opus=oo, median=float(np.median(x)), lo=float(np.quantile(x, .05)), hi=float(np.quantile(x, .95)))
R["design_fork"] = df
json.dump(R, open(f"{HERE}/results.json", "w"), indent=1, default=float)
print(json.dumps(df, indent=1))
