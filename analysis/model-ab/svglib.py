#!/usr/bin/env python3
"""Tiny SVG chart builders: forest plots, posterior densities, dot strips.

Everything is emitted against CSS custom properties so light/dark theming
happens in one place. Marks follow the dataviz spec: 2px lines, >=8px
markers, recessive grid, 2px surface ring where marks overlap.
"""
import math

FABLE = "var(--series-1)"
OPUS = "var(--series-2)"


def esc(s):
    return (str(s).replace("&", "&amp;").replace("<", "&lt;")
            .replace(">", "&gt;"))


def _fmt(v, dec=2):
    if v is None or v != v:
        return "-"
    if abs(v) >= 100:
        return "%.0f" % v
    return ("%." + str(dec) + "f") % v


class Scale(object):
    def __init__(self, lo, hi, px0, px1, log=False):
        self.log = log
        self.lo = math.log(lo) if log else lo
        self.hi = math.log(hi) if log else hi
        self.px0, self.px1 = px0, px1

    def __call__(self, v):
        if self.log:
            v = math.log(max(v, 1e-12))
        t = (v - self.lo) / (self.hi - self.lo)
        t = min(max(t, -0.05), 1.05)
        return self.px0 + t * (self.px1 - self.px0)

    def clamped(self, v):
        if self.log:
            lv = math.log(max(v, 1e-12))
        else:
            lv = v
        return lv < self.lo or lv > self.hi


def forest(items, xlabel, ref=1.0, log=True, width=880, row_h=34,
           xlo=None, xhi=None, ticks=None, title=None):
    """items: list of dicts {label, med, lo, hi, color, n, sub}"""
    left = 300
    right = 66
    top = 46 if title else 26
    h = top + row_h * len(items) + 46
    x0, x1 = left, width - right
    lo = xlo if xlo else min(i["lo"] for i in items) * 0.85
    hi = xhi if xhi else max(i["hi"] for i in items) * 1.15
    if log:
        lo = max(lo, 0.02)
    sc = Scale(lo, hi, x0, x1, log=log)
    if ticks is None:
        ticks = ([0.125, 0.25, 0.5, 1, 2, 4, 8] if log
                 else [-2, -1, -0.5, 0, 0.5, 1, 2])
        ticks = [t for t in ticks if lo <= t <= hi]

    p = ['<svg viewBox="0 0 %d %d" width="100%%" role="img" '
         'class="chart" aria-label="%s">' % (width, h, esc(xlabel))]
    if title:
        p.append('<text x="0" y="18" class="ct">%s</text>' % esc(title))
    # gridlines
    for t in ticks:
        X = sc(t)
        p.append('<line x1="%.1f" y1="%d" x2="%.1f" y2="%d" class="grid"/>'
                 % (X, top - 8, X, top + row_h * len(items)))
        p.append('<text x="%.1f" y="%d" class="tick" text-anchor="middle">%s'
                 '</text>' % (X, top + row_h * len(items) + 18,
                              _fmt(t, 3).rstrip("0").rstrip(".")
                              if t != int(t) else int(t)))
    # reference line
    RX = sc(ref)
    p.append('<line x1="%.1f" y1="%d" x2="%.1f" y2="%d" class="refline"/>'
             % (RX, top - 8, RX, top + row_h * len(items)))

    for k, it in enumerate(items):
        y = top + row_h * k + row_h / 2.0
        col = it.get("color", FABLE)
        p.append('<text x="0" y="%.1f" class="rowlab">%s</text>'
                 % (y + 4, esc(it["label"])))
        if it.get("sub"):
            p.append('<text x="0" y="%.1f" class="rowsub">%s</text>'
                     % (y + 15, esc(it["sub"])))
        L, H = sc(it["lo"]), sc(it["hi"])
        p.append('<title>%s: %s [%s, %s]</title>'
                 % (esc(it["label"]), _fmt(it["med"]), _fmt(it["lo"]),
                    _fmt(it["hi"])))
        p.append('<g><title>%s  median %s, 95%% CrI [%s, %s]%s</title>'
                 % (esc(it["label"]), _fmt(it["med"]), _fmt(it["lo"]),
                    _fmt(it["hi"]),
                    ("  n=%s" % it["n"]) if it.get("n") else ""))
        p.append('<line x1="%.1f" y1="%.1f" x2="%.1f" y2="%.1f" '
                 'stroke="%s" stroke-width="2.5" stroke-linecap="round" '
                 'opacity="0.55"/>' % (L, y, H, y, col))
        # 80% inner band if given
        if it.get("lo80") is not None:
            p.append('<line x1="%.1f" y1="%.1f" x2="%.1f" y2="%.1f" '
                     'stroke="%s" stroke-width="6" stroke-linecap="round" '
                     'opacity="0.35"/>'
                     % (sc(it["lo80"]), y, sc(it["hi80"]), y, col))
        p.append('<circle cx="%.1f" cy="%.1f" r="5.5" fill="%s" '
                 'stroke="var(--surface-1)" stroke-width="2"/>'
                 % (sc(it["med"]), y, col))
        p.append('</g>')
        p.append('<text x="%d" y="%.1f" class="val">%s</text>'
                 % (width - right + 8, y + 4, _fmt(it["med"])))
    p.append('<text x="%.1f" y="%d" class="axlab" text-anchor="middle">%s'
             '</text>' % ((x0 + x1) / 2.0, h - 6, esc(xlabel)))
    p.append('</svg>')
    return "\n".join(p)


def _kde(samples, lo, hi, n=160):
    """Gaussian KDE, Silverman bandwidth. Pure python, samples are thinned."""
    m = len(samples)
    mu = sum(samples) / m
    sd = math.sqrt(sum((x - mu) ** 2 for x in samples) / max(m - 1, 1))
    if sd <= 0:
        sd = 1e-6
    bw = 1.06 * sd * (m ** (-0.2))
    xs = [lo + (hi - lo) * i / float(n - 1) for i in range(n)]
    ys = []
    inv = 1.0 / (bw * math.sqrt(2 * math.pi) * m)
    for x in xs:
        s = 0.0
        for v in samples:
            z = (x - v) / bw
            if -6 < z < 6:
                s += math.exp(-0.5 * z * z)
        ys.append(s * inv)
    return xs, ys


def density(series, xlabel, ref=0.0, width=880, height=250, xlo=None,
            xhi=None, title=None, log_axis=False, ticks=None):
    """series: list of {label, samples, color, ci}"""
    left, right, top, bot = 54, 24, 40 if title else 18, 44
    allv = [v for s in series for v in s["samples"]]
    lo = xlo if xlo is not None else min(allv)
    hi = xhi if xhi is not None else max(allv)
    pad = (hi - lo) * 0.04
    lo, hi = lo - pad, hi + pad
    sc = Scale(lo, hi, left, width - right)
    curves = []
    ymax = 0.0
    for s in series:
        xs, ys = _kde(s["samples"], lo, hi)
        ymax = max(ymax, max(ys))
        curves.append((s, xs, ys))
    ysc = Scale(0, ymax * 1.12, height - bot, top)

    p = ['<svg viewBox="0 0 %d %d" width="100%%" role="img" class="chart" '
         'aria-label="%s">' % (width, height, esc(xlabel))]
    if title:
        p.append('<text x="0" y="18" class="ct">%s</text>' % esc(title))
    if ticks is None:
        span = hi - lo
        step = 10 ** math.floor(math.log10(span / 4.0))
        for mult in (1, 2, 2.5, 5, 10):
            if span / (step * mult) <= 6:
                step = step * mult
                break
        t = math.ceil(lo / step) * step
        ticks = []
        while t <= hi:
            ticks.append(t)
            t += step
    for t in ticks:
        X = sc(t)
        p.append('<line x1="%.1f" y1="%d" x2="%.1f" y2="%.1f" class="grid"/>'
                 % (X, top, X, height - bot))
        lab = ("%g" % round(math.exp(t), 2)) if log_axis else ("%g" % round(t, 3))
        p.append('<text x="%.1f" y="%d" class="tick" text-anchor="middle">%s'
                 '</text>' % (X, height - bot + 17, lab))
    p.append('<line x1="%.1f" y1="%d" x2="%.1f" y2="%.1f" class="refline"/>'
             % (sc(ref), top, sc(ref), height - bot))
    for s, xs, ys in curves:
        col = s.get("color", FABLE)
        pts = " ".join("%.1f,%.1f" % (sc(x), ysc(y)) for x, y in zip(xs, ys))
        base = ysc(0)
        p.append('<polygon points="%.1f,%.1f %s %.1f,%.1f" fill="%s" '
                 'opacity="0.16"/>' % (sc(xs[0]), base, pts, sc(xs[-1]), base,
                                       col))
        p.append('<polyline points="%s" fill="none" stroke="%s" '
                 'stroke-width="2" stroke-linejoin="round"><title>%s</title>'
                 '</polyline>' % (pts, col, esc(s["label"])))
        if s.get("ci"):
            a, b = s["ci"]
            p.append('<line x1="%.1f" y1="%.1f" x2="%.1f" y2="%.1f" '
                     'stroke="%s" stroke-width="3" stroke-linecap="round" '
                     'opacity="0.7"/>' % (sc(a), base + 9, sc(b), base + 9, col))
    p.append('<line x1="%d" y1="%.1f" x2="%d" y2="%.1f" class="axis"/>'
             % (left, ysc(0), width - right, ysc(0)))
    p.append('<text x="%.1f" y="%d" class="axlab" text-anchor="middle">%s'
             '</text>' % ((left + width - right) / 2.0, height - 6, esc(xlabel)))
    p.append('</svg>')
    return "\n".join(p)


def dotstrip(groups, xlabel, width=880, row_h=44, xlo=None, xhi=None,
             title=None, jitter_seed=7):
    """groups: list of {label, points:[{v,arm,tip}]}  — one row per group."""
    import random
    rng = random.Random(jitter_seed)
    left, right = 150, 30
    top = 44 if title else 16
    h = top + row_h * len(groups) + 42
    allv = [pt["v"] for g in groups for pt in g["points"]]
    lo = xlo if xlo is not None else min(allv)
    hi = xhi if xhi is not None else max(allv)
    if hi == lo:
        hi = lo + 1
    pad = (hi - lo) * 0.06
    sc = Scale(lo - pad, hi + pad, left, width - right)
    p = ['<svg viewBox="0 0 %d %d" width="100%%" role="img" class="chart" '
         'aria-label="%s">' % (width, h, esc(xlabel))]
    if title:
        p.append('<text x="0" y="18" class="ct">%s</text>' % esc(title))
    step = max(1, int(round((hi - lo) / 6.0)))
    t = math.floor(lo)
    while t <= hi:
        X = sc(t)
        p.append('<line x1="%.1f" y1="%d" x2="%.1f" y2="%d" class="grid"/>'
                 % (X, top - 6, X, top + row_h * len(groups)))
        p.append('<text x="%.1f" y="%d" class="tick" text-anchor="middle">%g'
                 '</text>' % (X, top + row_h * len(groups) + 17, t))
        t += step
    for k, g in enumerate(groups):
        y = top + row_h * k + row_h / 2.0
        p.append('<text x="0" y="%.1f" class="rowlab">%s</text>'
                 % (y + 4, esc(g["label"])))
        if k:
            p.append('<line x1="%d" y1="%.1f" x2="%d" y2="%.1f" class="sep"/>'
                     % (left - 12, top + row_h * k, width - right,
                        top + row_h * k))
        for pt in g["points"]:
            col = OPUS if pt["arm"] == "opus" else FABLE
            jy = y + (rng.random() - 0.5) * (row_h * 0.46)
            p.append('<circle cx="%.1f" cy="%.1f" r="4.6" fill="%s" '
                     'fill-opacity="0.85" stroke="var(--surface-1)" '
                     'stroke-width="1.5"><title>%s</title></circle>'
                     % (sc(pt["v"]), jy, col, esc(pt["tip"])))
    p.append('<text x="%.1f" y="%d" class="axlab" text-anchor="middle">%s'
             '</text>' % ((left + width - right) / 2.0, h - 6, esc(xlabel)))
    p.append('</svg>')
    return "\n".join(p)


def legend(entries):
    out = ['<div class="legend">']
    for lab, col in entries:
        out.append('<span class="lg"><span class="sw" style="background:%s">'
                   '</span>%s</span>' % (col, esc(lab)))
    out.append("</div>")
    return "".join(out)


def slope(groups, xlabel, width=880, row_h=30, xlo=None, xhi=None,
          title=None, left=170):
    """Paired R1->R2 slope chart. groups: [{label, a, b}] one row each."""
    right = 90
    top = 44 if title else 16
    h = top + row_h * len(groups) + 44
    allv = [v for g in groups for v in (g["a"], g["b"])]
    lo = xlo if xlo is not None else min(allv)
    hi = xhi if xhi is not None else max(allv)
    if hi == lo:
        hi = lo + 1
    pad = (hi - lo) * 0.08
    sc = Scale(lo - pad, hi + pad, left, width - right)
    p = ['<svg viewBox="0 0 %d %d" width="100%%" role="img" class="chart" '
         'aria-label="%s">' % (width, h, esc(xlabel))]
    if title:
        p.append('<text x="0" y="18" class="ct">%s</text>' % esc(title))
    t = math.floor(lo)
    while t <= hi:
        X = sc(t)
        p.append('<line x1="%.1f" y1="%d" x2="%.1f" y2="%d" class="grid"/>'
                 % (X, top - 6, X, top + row_h * len(groups)))
        p.append('<text x="%.1f" y="%d" class="tick" text-anchor="middle">%g'
                 '</text>' % (X, top + row_h * len(groups) + 17, t))
        t += 1
    for k, g in enumerate(groups):
        y = top + row_h * k + row_h / 2.0
        p.append('<text x="0" y="%.1f" class="rowlab">%s</text>'
                 % (y + 4, esc(g["label"])))
        A, B = sc(g["a"]), sc(g["b"])
        same = abs(g["a"] - g["b"]) < 1e-9
        p.append('<g><title>%s: R1=%g  R2=%g%s</title>'
                 % (esc(g["label"]), g["a"], g["b"],
                    "  (agree)" if same else "  (differ)"))
        if not same:
            p.append('<line x1="%.1f" y1="%.1f" x2="%.1f" y2="%.1f" '
                     'stroke="var(--muted)" stroke-width="2" '
                     'stroke-linecap="round" opacity="0.55"/>'
                     % (A, y, B, y))
        p.append('<circle cx="%.1f" cy="%.1f" r="5" fill="%s" '
                 'stroke="var(--surface-1)" stroke-width="2"/>' % (A, y, FABLE))
        p.append('<circle cx="%.1f" cy="%.1f" r="5" fill="%s" '
                 'stroke="var(--surface-1)" stroke-width="2"/>' % (B, y, OPUS))
        p.append('</g>')
        p.append('<text x="%d" y="%.1f" class="val">%s</text>'
                 % (width - right + 10, y + 4,
                    "agree" if same else "%g→%g" % (g["a"], g["b"])))
    p.append('<text x="%.1f" y="%d" class="axlab" text-anchor="middle">%s'
             '</text>' % ((left + width - right) / 2.0, h - 6, esc(xlabel)))
    p.append('</svg>')
    return "\n".join(p)
