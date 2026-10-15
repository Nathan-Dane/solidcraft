#!/usr/bin/env python3
"""Where every icon stands: one self-contained HTML page (and inventory.csv), from the repo alone.

  python3 assets/icons/ui/design/tools/icon_status.py            # writes target/icon-status.html
  python3 assets/icons/ui/design/tools/icon_status.py --csv      # also rewrites design/inventory.csv
  python3 assets/icons/ui/design/tools/icon_status.py --out F    # page somewhere else
  python3 assets/icons/ui/design/tools/icon_status.py --summary  # counts only, no files

Standard library only; no build, no app. Open the page in any browser.

Icon ids come from the command registry (`.icon("id")` in crates/engine/src/cmd), the names the
UI passes to `icons::paint` and `glyph_icon`, plus every master and alias. Each id is:
  done         a master exists and is not listed in status.toml
  stand-in     a master exists but status.toml marks it as a placeholder drawing to be redrawn
  placeholder  no master (and no alias): the app draws the standard placeholder
The page shows each in the light and dark palette with its accent, at 48 and 20 px.
"""
import sys, os, re, glob, html, json, collections, tomllib

HERE = os.path.dirname(os.path.abspath(__file__))
D = os.path.dirname(HERE)                       # .../assets/icons/ui/design
ROOT = os.path.abspath(os.path.join(D, '..', '..', '..', '..'))
args = sys.argv[1:]
out_path = args[args.index('--out') + 1] if '--out' in args else os.path.join(ROOT, 'target', 'icon-status.html')

pal = tomllib.load(open(D + '/palette.toml', 'rb'))
roles = pal['roles']; accents = pal.get('accents', {})
alias_raw = tomllib.load(open(D + '/aliases.toml', 'rb')).get('alias', {})
stand_in = set(tomllib.load(open(D + '/status.toml', 'rb')).get('stand_in', []))
masters = {}
for f in sorted(glob.glob(D + '/svg/*/*.svg')):
    fam = os.path.basename(os.path.dirname(f)); n = os.path.basename(f)[:-4]
    if fam != 'small': masters[n] = (fam, f)

# --- where icon ids are used -----------------------------------------------------------------
uses = collections.defaultdict(list)   # id -> ["command id" | "ui"]
for f in glob.glob(ROOT + '/crates/engine/src/**/*.rs', recursive=True):
    t = open(f).read()
    ms = list(re.finditer(r'CommandSpec::new\(\s*"([^"]+)"\s*,\s*"([^"]*)"', t))
    for i, m in enumerate(ms):
        end = ms[i + 1].start() if i + 1 < len(ms) else len(t)
        chunk = '\n'.join(t[m.end():end].split('\n')[:14])
        ic = re.search(r'\.icon\(\s*"([^"]+)"', chunk)
        if ic: uses[ic.group(1)].append(m.group(1))
for f in glob.glob(ROOT + '/crates/ui-egui/src/*.rs'):
    t = open(f).read()
    for call in re.finditer(r'icons::paint[a-z_]*\((.*?);', t, re.S):
        for name in re.findall(r'"([a-z][a-z0-9_]*)"', call.group(1)):
            uses[name].append('ui:' + os.path.basename(f))
mk = ROOT + '/assets/icons/ui/source/markers.rs'
if os.path.exists(mk):
    for name in re.findall(r'=> "(c_[a-z_]+)"', open(mk).read()): uses[name].append('ui:sketch constraint glyph')
n_commands = len({c for v in uses.values() for c in v if not c.startswith('ui:')})

# --- status -----------------------------------------------------------------------------------
def alias_of(n):
    v = alias_raw.get(n)
    if v is None: return n, ''
    return (v['master'], v.get('accent', '')) if isinstance(v, dict) else (v, '')
ids = sorted(set(uses) | set(masters) | set(alias_raw))
rows = []
for n in ids:
    m, acc = alias_of(n)
    if m in masters and n != 'placeholder':
        status = 'stand-in' if m in stand_in else 'done'
    else:
        status, m = 'placeholder', 'placeholder'
    cmds = [u for u in uses.get(n, []) if not u.startswith('ui:')]
    rows.append(dict(id=n, status=status, master=m, accent=acc, commands=len(cmds), example=cmds[0] if cmds else '',
                     ui=sorted({u[3:] for u in uses.get(n, []) if u.startswith('ui:')}), unused=not uses.get(n)))
cnt = collections.Counter(r['status'] for r in rows if r['id'] != 'placeholder')
cmd_cnt = collections.Counter()
for r in rows:
    if r['id'] != 'placeholder': cmd_cnt[r['status']] += r['commands']
summary = '%d icon ids: %d done, %d stand-in, %d placeholder (%d commands: %d / %d / %d).' % (
    sum(cnt.values()), cnt['done'], cnt['stand-in'], cnt['placeholder'], sum(cmd_cnt.values()), cmd_cnt['done'], cmd_cnt['stand-in'], cmd_cnt['placeholder'])
print(summary)
if '--summary' in args: sys.exit(0)

if '--csv' in args:
    with open(D + '/inventory.csv', 'w') as f:
        f.write('icon,status,master,accent,commands,example_command,used_by_ui\n')
        for r in rows:
            if r['id'] == 'placeholder': continue
            f.write('%s,%s,%s,%s,%d,%s,"%s"\n' % (r['id'], r['status'], r['master'] if r['status'] != 'placeholder' else '', r['accent'], r['commands'], r['example'], ';'.join(r['ui'])))
    print('wrote', os.path.relpath(D + '/inventory.csv', ROOT))

# --- the page ---------------------------------------------------------------------------------
def colours(theme, accent):
    m = {r['light'].lower(): r[theme] for r in roles.values()}
    for rn, v in accents.get(accent, {}).items():
        m[roles[rn]['light'].lower()] = v[theme]
    return m, roles['line'][theme]
def themed(svg, theme, accent):
    m, line = colours(theme, accent)
    svg = re.sub(r'#[0-9a-fA-F]{6}', lambda x: m.get(x.group(0).lower(), x.group(0)), svg)
    return svg.replace('currentColor', line)
def sized(svg, px): return svg.replace('<svg ', '<svg width="%d" height="%d" ' % (px, px), 1)

cards = []
for r in rows:
    if r['id'] == 'placeholder': continue
    svg = open(masters[r['master']][1] if r['master'] in masters else masters['placeholder'][1]).read()
    th = {t: themed(svg, t, r['accent']) for t in ('light', 'dark')}
    use = ('%d command%s' % (r['commands'], '' if r['commands'] == 1 else 's')) if r['commands'] else ''
    if r['ui']: use += (', ' if use else '') + 'UI: ' + ', '.join(r['ui'])
    if r['unused']: use = 'no app use yet'
    src = ('= ' + r['master']) if r['master'] != r['id'] and r['status'] != 'placeholder' else ''
    cards.append('<div class="card %s" data-s="%s" data-k="%s">%s<div class="meta"><b>%s</b><span class="badge %s">%s</span><br>%s%s<br><small>%s</small></div></div>' % (
        r['status'], r['status'], html.escape(r['id'] + ' ' + r['example'] + ' ' + r['accent']),
        '<div class="pair">' + ''.join('<div class="%s">%s<span>%s</span></div>' % (t, sized(th[t], 48), sized(th[t], 20)) for t in ('light', 'dark')) + '</div>',
        html.escape(r['id']), r['status'].replace('-', ''), r['status'],
        html.escape(src), (' <i class="acc">' + r['accent'] + ' accent</i>') if r['accent'] else '', html.escape(use + ((' · e.g. ' + r['example']) if r['example'] else ''))))
page = '''<!doctype html><meta charset=utf-8><title>SolidCraft icon status</title>
<style>
body{font:13px system-ui,sans-serif;margin:0;background:#f4f6f9;color:#1d2733}
header{position:sticky;top:0;background:#fff;border-bottom:1px solid #d5dbe3;padding:10px 16px;z-index:2}
h1{font-size:16px;margin:0 0 4px}.sum{color:#4a5a6c;margin-bottom:8px}
button,input{font:inherit;padding:4px 10px;margin-right:6px;border:1px solid #b8c3cf;border-radius:6px;background:#fff}
button.on{background:#236ead;color:#fff;border-color:#236ead}
main{display:grid;grid-template-columns:repeat(auto-fill,minmax(190px,1fr));gap:10px;padding:14px 16px}
.card{background:#fff;border:1px solid #d5dbe3;border-radius:8px;overflow:hidden}
.card.placeholder{border-color:#e0a0a0}.card.stand-in{border-color:#e0c070}
.pair{display:grid;grid-template-columns:1fr 1fr}.pair>div{display:flex;align-items:center;justify-content:center;gap:8px;padding:12px 4px}
.light{background:#fff}.dark{background:#1e2530}
.meta{padding:6px 10px 8px;line-height:1.5}.meta small{color:#6b7a8a}
.badge{margin-left:6px;padding:0 6px;border-radius:9px;font-size:11px;color:#fff}
.badge.done{background:#2e8b57}.badge.standin{background:#c58a00}.badge.placeholder{background:#c0392b}
.acc{color:#a0500a}
</style>
<header><h1>SolidCraft icon status</h1><div class=sum>%s Each card: light, then dark, at 48 and 20 px.</div>
<button data-f=all class=on>All</button><button data-f=done>Done</button><button data-f=stand-in>Stand-in</button><button data-f=placeholder>Placeholder</button>
<input id=q placeholder="filter by id or command"></header><main>%s</main>
<script>let f='all';const q=document.getElementById('q');
function run(){document.querySelectorAll('.card').forEach(c=>{c.style.display=(f=='all'||c.dataset.s==f)&&c.dataset.k.includes(q.value.toLowerCase())?'':'none'})}
document.querySelectorAll('button').forEach(b=>b.onclick=()=>{f=b.dataset.f;document.querySelectorAll('button').forEach(x=>x.classList.toggle('on',x==b));run()});q.oninput=run</script>''' % (html.escape(summary), ''.join(cards))
os.makedirs(os.path.dirname(out_path), exist_ok=True)
open(out_path, 'w').write(page)
print('wrote', os.path.relpath(out_path, ROOT), '(%d KB)' % (len(page) // 1024))
