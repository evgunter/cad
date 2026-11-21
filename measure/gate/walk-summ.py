import glob,os,sys,json,statistics as st,collections
M=os.path.expanduser('~/.local/share/cad-work/gate/walk')
pop=json.load(open(M+'/population.json')); shape_of={t:sh for b,t,sh in pop['sample']}
part=sys.argv[1]
W=[];M4=[]
for f in glob.glob(f'{M}/log-{part}/*.log'):
    for l in open(f,errors='replace'):
        p=l.rstrip('\n').split('\t')
        if p[0]=='WALK':
            d={'test':p[1]}; d.update(kv.split('=',1) for kv in p[2:]); W.append(d)
        elif p[0]=='M4':
            d={'test':p[1]}; d.update(kv.split('=',1) for kv in p[2:]); M4.append(d)
def sp(x): a,b=x.split(':'); return int(a),float(b)
ref=[d for d in W if d['verdict']!='Crosses']
print('refusing asks',len(ref),'crossing asks',len(W)-len(ref))
if part=='dear':
    for d in sorted(ref,key=lambda d:-float(d['secs']))[:12]:
        a,b,cp,cq,bf=[sp(d[k]) for k in('a','b','c_pre','c_post','before')]
        print(f"{float(d['secs']):7.1f}s {d['test'][:55]:55s} {d['verdict'][:26]:26s} first@depth {d['first_depth']} after {d['splits_at_first']} splits; before {bf[0]} {bf[1]:.1f}s | a {a[0]} {a[1]:.1f}s | b {b[0]} {b[1]:.1f}s | c {cp[0]}/{cq[0]} {cp[1]:.1f}/{cq[1]:.1f}s | open {d['d_open']} | pairs refusing {d['refusing_pairs']} {d['kinds'][:80]}")
groups=collections.defaultdict(list)
for d in ref: groups[shape_of.get(d['test'],'dear' if part=='dear' else 'm3-only')].append(d)
print('== per shape (median / total)')
for g,v in groups.items():
    def col(k,i): xs=[sp(d[k])[i] for d in v]; return f"{st.median(xs):.3g}/{sum(xs):.4g}"
    print(g,len(v),'before',col('before',1),'s','a',col('a',1),'b',col('b',1),'c_pre',col('c_pre',1),'c_post',col('c_post',1),'| splits a',col('a',0),'b',col('b',0),'c',col('c_post',0),'| open med',st.median(int(d['d_open']) for d in v),'| first depth med',st.median(int(d['first_depth']) for d in v))
print('== M2: distinct refusing pairs per refusing ask', collections.Counter(int(d['refusing_pairs']) for d in ref).most_common(12))
kinds=collections.Counter(k for d in ref for k in eval(d['kinds'])); print('pair refusal kinds',kinds.most_common(10))
cr=[d for d in W if d['verdict']=='Crosses']
print('crossing asks also holding a refusing pair:',sum(int(d['refusing_pairs'])>0 for d in cr),'of',len(cr))
print('== M3 crossing location (asks that crossed after a refusal):',collections.Counter(d['crossing'] for d in cr if int(d['refusing_pairs'])>0))
if M4:
    r=[float(d['continued_secs'])/max(float(d['stopped_secs']),1e-6) for d in M4]
    print('== M4',len(M4),'crossing asks continued: stopped s %.2f continued s %.2f, median ratio %.2f, verdicts'%(sum(float(d['stopped_secs']) for d in M4),sum(float(d['continued_secs']) for d in M4),st.median(r)),collections.Counter(d['continued_verdict'] for d in M4))
