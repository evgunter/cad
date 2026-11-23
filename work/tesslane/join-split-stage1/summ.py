import sys, re, glob, collections
d = sys.argv[1]
splits = collections.defaultdict(lambda: [0,0,0,0,0,0,0.0,0,0,0])  # n, exceeded, unres_cert, unres_wind, fold, fold_parent, worst, apex, disagree, nocand
other = collections.Counter()
calls = {}
worst_foot = collections.defaultdict(float)
for f in glob.glob(d + '/*.log'):
    for line in open(f):
        kv = dict(re.findall(r'(\w+)=(\S+)', line))
        tag = line.split()[0]
        if tag == 'SPLIT':
            r = splits[kv['pair']]
            r[0] += 1
            r[1] += int(kv['exceeded'] != '0'); r[2] += int(kv['unres_cert'] != '0'); r[3] += int(kv['unres_wind'] != '0')
            r[4] += int(kv['fold'] != '0'); r[5] += int(kv['fold_parent'] != '0')
            r[6] = max(r[6], float(kv['worst_half_d']))
            r[7] += int(kv['apex'] == 'true'); r[8] += int(kv['disagree'] == 'true')
            fg = float(kv['footgap_d'])
            if fg == fg: worst_foot[kv['pair']] = max(worst_foot[kv['pair']], fg)
        elif tag in ('NOCAND', 'NOINSIDE'):
            other[(tag, kv['pair'])] += 1
        elif tag == 'JOIN':
            other[('JOIN-left>0', '')] += int(kv['left'] != '0')
            other[('JOIN-coincident', '')] += int(kv['coincident_new'])
            other[('JOIN-meshes', '')] += 1
        elif tag in ('FOLDCURE',):
            other[(tag, '')] += 1
        elif tag.startswith('CHORDFLOOR'):
            other[('CHORDFLOOR', '')] += 1
print('pair | splits | exceeded | unres_cert | unres_winding | fold(face) | fold(vs parent) | worst half/δ | apex | disagree | worst footgap/δ')
for k, r in sorted(splits.items(), key=lambda x: -x[1][0]):
    print(f'{k} | {r[0]} | {r[1]} | {r[2]} | {r[3]} | {r[4]} | {r[5]} | {r[6]:.3f} | {r[7]} | {r[8]} | {worst_foot[k]:.3e}')
for k, v in sorted(other.items()):
    print(k, v)
