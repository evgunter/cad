import subprocess, sys, os
sys.path.insert(0,'/home/user/rv-mut')
W='/home/user/rv-wt'; F=W+'/crates/topo/src/boolean/sectors.rs'
SP='/tmp/claude-0/-home-user-cad/d9226f33-3eb0-52ea-907b-2e954ce9b07e/scratchpad/rv'
exec(open('/home/user/rv-mut/run.py').read().split("sel=sys.argv")[0])
env=dict(os.environ, CARGO_TARGET_DIR='/home/user/rv-wt-target')
for name in sys.argv[1:]:
    a,b=M[name]; open(F,'w').write(orig.replace(a,b))
    for s in ('1','4'):
        e=dict(env, RV_CASES=f'{SP}/cases_{s}.txt', RV_OUT=f'{SP}/mut_{s}.txt')
        subprocess.run(['cargo','nextest','run','-p','topo','--lib','rv_probe_cases'],cwd=W,env=e,capture_output=True)
        p=subprocess.run(['python3','cmp.py',s,f'mut_{s}.txt','0'],cwd=SP,capture_output=True,text=True)
        print(name, s, p.stdout.splitlines()[0], flush=True)
open(F,'w').write(orig)
