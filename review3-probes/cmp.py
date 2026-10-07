import sys, collections
def load(p):
    cells={}; miss=collections.Counter(); wrong=collections.Counter()
    for l in open(p):
        l=l.rstrip('\n')
        if l.startswith(('SKIP','SUMMARY')): continue
        if l.startswith(('MISSING ','WRONG ','DOUBLED ')):
            tag,rest=l.split(' ',1); cell=rest.split(' ')[0] if False else None
            # cell is up to the op field: label|probe|pose|op
            parts=rest.split('|'); op=parts[3].split(' ')[0]; cell='|'.join(parts[:3]+[op])
            (miss if tag=='MISSING' else wrong)[cell]+=1; continue
        parts=l.split('|')
        cell='|'.join(parts[:4]); cells[cell]='|'.join(parts[4:])
    return cells,miss,wrong
m,mm,mw=load(sys.argv[1]); h,hm,hw=load(sys.argv[2])
cat=collections.Counter(); ex=collections.defaultdict(list)
for c in m:
    if c not in h: cat['main only (scene not built on head?)']+=1; continue
    a,b=m[c],h[c]
    ka='BODY' if a.startswith('BODY') else a.split(' ')[0]+' '+a.split(' ')[1].split('{')[0].split('(')[0] if ' ' in a else a
    kb='BODY' if b.startswith('BODY') else b.split(' ')[0]+' '+b.split(' ')[1].split('{')[0].split('(')[0] if ' ' in b else b
    if a==b: k='identical'
    elif ka=='BODY' and kb=='BODY':
        k='rows differ' + (' (head more missing)' if hm[c]>mm[c] else '') + (' (head wrong)' if hw[c] else '') + (' main wrong' if mw[c] else '')
    else: k=f'{ka} -> {kb}'
    cat[k]+=1; ex[k].append(c)
for k,v in sorted(cat.items(), key=lambda x:-x[1]):
    print(v,k, '| e.g.', ex[k][:2])
print('--- rows differ, no main wrong:')
c2=collections.Counter()
for c in ex['rows differ']:
    c2[(c.split('|')[0], 'main missing %d head missing %d'%(mm[c],hm[c]))]+=1
for k,v in c2.items(): print(v,k)
