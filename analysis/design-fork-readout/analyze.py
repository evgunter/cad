"""Unblind the coded rows and fit the models -> results.json.

Inputs: key.csv (the only file that knows the models), labels/p1.csv and
labels/p2.csv (two independent blinded coding passes, same schema).

Every estimand is a proportion with a Beta(1, 1) prior (closed form),
except the time trend, a logistic regression fitted by 2-D grid
quadrature. Pooled results average the two passes' posteriors (an equal
mixture), so coder disagreement widens the interval instead of being
resolved by fiat.
"""
import csv, json, math, sys
from datetime import date
import numpy as np

HERE = sys.argv[1] if len(sys.argv) > 1 else "."
RNG = np.random.default_rng(20261031)
DRAWS = 200_000


def load(path):
    return {r["key"]: r for r in csv.DictReader(open(path))}


key = load(f"{HERE}/key.csv")
passes = {p: load(f"{HERE}/labels/{p}.csv") for p in ("p1", "p2")}
for p, rows in passes.items():
    missing = set(key) - set(rows)
    if missing:
        sys.exit(f"{p}: missing {sorted(missing)}")


def side(code, a_model):
    """Map an A/B code to a model name; other codes pass through."""
    if code == "A":
        return a_model
    if code == "B":
        return "fable" if a_model == "opus" else "opus"
    return code


def usable(k, strict):
    r = key[k]
    if not r["a_model"]:
        return False
    if strict and r["deviation"]:
        return False
    return True


# ---------- posterior helpers ----------

def beta_summary(s, f, prior=(1, 1)):
    a, b = s + prior[0], f + prior[1]
    x = RNG.beta(a, b, DRAWS)
    return x


def mix(*draws):
    return np.concatenate(draws)


def summarise(x, ref=0.5):
    lo, med, hi = np.quantile(x, [0.025, 0.5, 0.975])
    return {"median": float(med), "lo": float(lo), "hi": float(hi),
            "p_gt_ref": float((x > ref).mean()), "ref": ref}


def density(x, lo=0.0, hi=1.0, bins=120):
    h, edges = np.histogram(x, bins=bins, range=(lo, hi), density=True)
    # light smoothing for display
    k = np.array([1, 4, 6, 4, 1], float); k /= k.sum()
    h = np.convolve(h, k, mode="same")
    return {"x": [float((edges[i] + edges[i + 1]) / 2) for i in range(bins)],
            "y": [float(v) for v in h]}


# ---------- per-pass tallies ----------

def tallies(p, strict=False):
    rows = passes[p]
    out = {"h2h": [0, 0], "lean": [0, 0], "pos": [0, 0],
           "cats": {}, "hit": {"opus": [0, 0], "fable": [0, 0]},
           "mover": [0, 0], "frame": {"opus": [0, 0], "fable": [0, 0]},
           "ratified": {"opus": [0, 0], "fable": [0, 0]},
           "frame_disc": [0, 0], "ratified_disc": [0, 0],
           "conf": {"opus": [], "fable": []},
           "conf_disc": [0, 0],
           "agree": {}, "ev_vs": {}, "decided": 0, "n": 0,
           "series": []}
    for k in key:
        if not usable(k, strict):
            continue
        r, a = rows[k], key[k]["a_model"]
        out["n"] += 1
        out["agree"][r["agree_first"]] = out["agree"].get(r["agree_first"], 0) + 1
        for X in "AB":
            m = side(X, a)
            v = r[f"frame_{X}"]
            if v in "yn" and v:
                out["frame"][m][0 if v == "y" else 1] += 1
            v = r[f"ratified_{X}"]
            if v in "yn" and v:
                out["ratified"][m][0 if v == "y" else 1] += 1
            c = r[f"conf_{X}"]
            if c.strip():
                out["conf"][m].append(int(c))
        fo, ff = side("A", a), side("B", a)
        for fld, disc in (("frame", "frame_disc"), ("ratified", "ratified_disc")):
            va, vb = r[f"{fld}_A"], r[f"{fld}_B"]
            if {va, vb} == {"y", "n"}:
                who = "A" if va == "y" else "B"
                out[disc][0 if side(who, a) == "opus" else 1] += 1
        ca, cb = r["conf_A"].strip(), r["conf_B"].strip()
        if ca and cb and ca != cb:
            hi = "A" if int(ca) > int(cb) else "B"
            out["conf_disc"][0 if side(hi, a) == "opus" else 1] += 1
        mv = r["movement"]
        if mv in ("A", "B"):
            out["mover"][0 if side(mv, a) == "opus" else 1] += 1
        if r["decided"] != "y":
            continue
        out["decided"] += 1
        out["ev_vs"][r["ev_vs_to_ev"]] = out["ev_vs"].get(r["ev_vs_to_ev"], 0) + 1
        cm = r["coder_match"]
        cat = side(cm, a)
        out["cats"][cat] = out["cats"].get(cat, 0) + 1
        if cm in ("A", "B"):
            out["h2h"][0 if cat == "opus" else 1] += 1
            out["pos"][0 if cm == "A" else 1] += 1
            out["series"].append((key[k]["date"], 1 if cat == "opus" else 0))
        for m in ("opus", "fable"):
            hit = cat == m or cm == "both"
            out["hit"][m][0 if hit else 1] += 1
        ln = r["ev_lean"]
        if ln in ("A", "B"):
            out["lean"][0 if side(ln, a) == "opus" else 1] += 1
    return out


# ---------- time trend: logistic on date, grid quadrature ----------

def trend(series):
    d0 = date(2026, 9, 24)
    t = np.array([(date.fromisoformat(d) - d0).days for d, _ in series], float)
    y = np.array([v for _, v in series], float)
    tc = (t - t.mean()) / 7.0  # weeks, centred
    al = np.linspace(-4, 4, 401)
    be = np.linspace(-2, 2, 401)
    A, B = np.meshgrid(al, be, indexing="ij")
    eta = A[..., None] + B[..., None] * tc
    ll = (y * eta - np.logaddexp(0, eta)).sum(-1)
    lp = ll - 0.5 * (A / 1.5) ** 2 - 0.5 * (B / 1.0) ** 2
    w = np.exp(lp - lp.max()); w /= w.sum()
    idx = RNG.choice(w.size, size=40_000, p=w.ravel())
    sa, sb = A.ravel()[idx], B.ravel()[idx]
    grid_t = np.linspace(t.min(), t.max(), 40)
    gc = (grid_t - t.mean()) / 7.0
    p = 1 / (1 + np.exp(-(sa[:, None] + sb[:, None] * gc)))
    band = np.quantile(p, [0.025, 0.5, 0.975], axis=0)
    return {"slope": summarise(sb, 0.0),
            "or_per_week": summarise(np.exp(sb), 1.0),
            "curve": {"t": grid_t.tolist(),
                      "lo": band[0].tolist(), "med": band[1].tolist(),
                      "hi": band[2].tolist()},
            "points": [[float(a), float(b)] for a, b in zip(t, y)],
            "t_mean": float(t.mean())}


# ---------- coder reliability ----------

def kappa(f):
    ks = [k for k in key if key[k]["a_model"]]
    a = [passes["p1"][k][f] for k in ks]
    b = [passes["p2"][k][f] for k in ks]
    cats = sorted(set(a) | set(b))
    n = len(ks)
    po = sum(x == y for x, y in zip(a, b)) / n
    pe = sum((a.count(c) / n) * (b.count(c) / n) for c in cats)
    return {"agree": po, "kappa": (po - pe) / (1 - pe) if pe < 1 else 1.0,
            "n": n}


def logger_check():
    """The log's own match cell was written after unblinding. Compare it
    with the blinded coders: where they differ, whom does the log
    credit more?"""
    out = {}
    for p in passes:
        credit = {"opus": 0, "fable": 0}
        diffs = 0
        for k in key:
            if not usable(k, False):
                continue
            r, a = passes[p][k], key[k]["a_model"]
            if r["decided"] != "y":
                continue
            lg, cd = r["logged_match"], r["coder_match"]
            if lg == cd:
                continue
            diffs += 1
            for m in ("opus", "fable"):
                won_l = side(lg, a) == m or lg == "both"
                won_c = side(cd, a) == m or cd == "both"
                credit[m] += int(won_l) - int(won_c)
        out[p] = {"diffs": diffs, "net_credit": credit}
    return out


def logged_h2h(p):
    s = [0, 0]
    for k in key:
        if not usable(k, False):
            continue
        r, a = passes[p][k], key[k]["a_model"]
        if r["decided"] == "y" and r["logged_match"] in ("A", "B"):
            s[0 if side(r["logged_match"], a) == "opus" else 1] += 1
    return s


def main():
    T = {p: tallies(p) for p in passes}
    TS = {p: tallies(p, strict=True) for p in passes}
    res = {"n_rows": len(key), "per_pass": {}, "pooled": {}}
    for p in passes:
        t = T[p]
        res["per_pass"][p] = {k: t[k] for k in
                              ("h2h", "lean", "pos", "cats", "hit", "mover",
                               "frame", "ratified", "frame_disc",
                               "ratified_disc", "conf_disc", "agree", "ev_vs",
                               "decided", "n")}
        res["per_pass"][p]["conf_mean"] = {
            m: float(np.mean(v)) if v else None for m, v in t["conf"].items()}
        res["per_pass"][p]["strict_h2h"] = TS[p]["h2h"]
        res["per_pass"][p]["logged_h2h"] = logged_h2h(p)

    def pooled(get, ref=0.5):
        d = mix(*[beta_summary(*get(T[p])) for p in passes])
        return d

    P = res["pooled"]
    h2h = pooled(lambda t: t["h2h"])
    P["h2h"] = summarise(h2h); P["h2h_density"] = density(h2h)
    P["lean"] = summarise(pooled(lambda t: t["lean"]))
    P["position"] = summarise(pooled(lambda t: t["pos"]))
    P["strict_h2h"] = summarise(mix(*[beta_summary(*TS[p]["h2h"]) for p in passes]))
    P["logged_h2h"] = summarise(mix(*[beta_summary(*logged_h2h(p)) for p in passes]))
    P["mover_opus"] = summarise(pooled(lambda t: t["mover"]))
    P["frame_disc"] = summarise(pooled(lambda t: t["frame_disc"]))
    P["ratified_disc"] = summarise(pooled(lambda t: t["ratified_disc"]))
    P["conf_disc"] = summarise(pooled(lambda t: t["conf_disc"]))
    for m in ("opus", "fable"):
        P[f"hit_{m}"] = summarise(pooled(lambda t, m=m: t["hit"][m]))
        P[f"frame_{m}"] = summarise(pooled(lambda t, m=m: t["frame"][m]))
        P[f"ratified_{m}"] = summarise(pooled(lambda t, m=m: t["ratified"][m]))
    hd = mix(*[beta_summary(*T[p]["hit"]["opus"]) - beta_summary(*T[p]["hit"]["fable"])
               for p in passes])
    P["hit_diff"] = summarise(hd, 0.0)

    # outcome mix: Dirichlet(1,...) per pass, pooled
    cats = ["opus", "fable", "both", "mixed", "neither"]
    dd = []
    for p in passes:
        c = np.array([T[p]["cats"].get(x, 0) for x in cats], float) + 1
        dd.append(RNG.dirichlet(c, DRAWS))
    dd = np.concatenate(dd)
    P["cats"] = {x: summarise(dd[:, i], 0) for i, x in enumerate(cats)}

    # trend: fit per pass, report pass 1 and pass 2 slopes, plot pooled
    res["trend"] = {p: trend(T[p]["series"]) for p in passes}

    res["kappa"] = {f: kappa(f) for f in
                    ("coder_match", "logged_match", "agree_first",
                     "ev_lean", "movement", "decided")}
    res["logger"] = logger_check()
    # coder vs logger agreement within each pass
    res["coder_vs_logged"] = {}
    for p in passes:
        ks = [k for k in key if key[k]["a_model"] and passes[p][k]["decided"] == "y"]
        res["coder_vs_logged"][p] = sum(
            passes[p][k]["coder_match"] == passes[p][k]["logged_match"]
            for k in ks) / len(ks)
    json.dump(res, open(f"{HERE}/results.json", "w"), indent=1)
    print(json.dumps({k: v for k, v in P.items() if "density" not in k},
                     indent=1)[:4000])
    for p in passes:
        print(p, {k: res["per_pass"][p][k] for k in
                  ("h2h", "lean", "pos", "cats", "mover", "decided",
                   "strict_h2h", "logged_h2h", "conf_mean")})
    print("kappa", res["kappa"])
    print("logger", res["logger"])
    print("trend", {p: res["trend"][p]["or_per_week"] for p in passes})


if __name__ == "__main__":
    main()


# ---------- secondary cuts (appended; read results.json["extra"]) ----------

def extra():
    out = {}
    for p in passes:
        rows = passes[p]
        x = {"logged_vs_coder": {}, "conf_hit": {}, "challenger": [0, 0],
             "framer": [0, 0], "halves": {"early": [0, 0], "late": [0, 0]},
             "by_agree": {}}
        ks = [k for k in key if usable(k, False)]
        dates = sorted(key[k]["date"] for k in ks)
        mid = dates[len(dates) // 2]
        for k in ks:
            r, a = rows[k], key[k]["a_model"]
            if r["decided"] != "y":
                continue
            lg, cd = side(r["logged_match"], a), side(r["coder_match"], a)
            x["logged_vs_coder"][f"{lg}|{cd}"] = x["logged_vs_coder"].get(f"{lg}|{cd}", 0) + 1
            for X in "AB":
                m = side(X, a)
                c = r[f"conf_{X}"].strip() or "?"
                hit = cd == m or cd == "both"
                d = x["conf_hit"].setdefault(f"{m}:{c}", [0, 0])
                d[0 if hit else 1] += 1
            if cd in ("opus", "fable"):
                half = "early" if key[k]["date"] < mid else "late"
                x["halves"][half][0 if cd == "opus" else 1] += 1
                ag = r["agree_first"]
                x["by_agree"].setdefault(ag, [0, 0])[0 if cd == "opus" else 1] += 1
                for fld, slot in (("ratified", "challenger"), ("frame", "framer")):
                    va, vb = r[f"{fld}_A"], r[f"{fld}_B"]
                    if {va, vb} == {"y", "n"}:
                        who = side("A" if va == "y" else "B", a)
                        x[slot][0 if who == cd else 1] += 1
        out[p] = x
    pooled = {}
    for name in ("challenger", "framer"):
        pooled[name] = summarise(mix(*[beta_summary(*out[p][name]) for p in passes]))
    for h in ("early", "late"):
        pooled[h] = summarise(mix(*[beta_summary(*out[p]["halves"][h]) for p in passes]))
    pooled["mid_date"] = mid
    return {"per_pass": out, "pooled": pooled}


if __name__ == "__main__":
    res = json.load(open(f"{HERE}/results.json"))
    res["extra"] = extra()
    json.dump(res, open(f"{HERE}/results.json", "w"), indent=1)
    print(json.dumps(res["extra"], indent=1))
