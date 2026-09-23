import subprocess
import os

results = []

def run(name, cmd, cwd=None):
    results.append(f'--- {name} ---')
    try:
        r = subprocess.run(cmd, shell=True, capture_output=True, text=True, cwd=cwd)
        results.append(f'STDOUT:\n{r.stdout}\nSTDERR:\n{r.stderr}\nCODE: {r.returncode}')
    except Exception as e:
        results.append(f'ERROR: {e}')

# S1: ConPTY
run('S1_ConPTY', 'powershell -c "Start-Process cmd -ArgumentList \'/c echo Hello PTY\' -NoNewWindow -Wait"')

# S2: Headless JSON
run('S2_Headless', 'claude -p --output-format stream-json --version')

# S4: Git Worktree
os.makedirs('s4_test', exist_ok=True)
run('S4_Git_Init', 'git init', cwd='s4_test')
run('S4_Git_Commit', 'git config user.name test && git config user.email test@test.com && echo "init" > file.txt && git add file.txt && git commit -m "init"', cwd='s4_test')
run('S4_Git_Worktree', 'git worktree add ../s4_tree -b test-branch', cwd='s4_test')
run('S4_Git_Worktree_Check', 'dir ..\\s4_tree', cwd='s4_test')

# S7: SQLite-vec
run('S7_SQLite_Vec', 'python -c "import sqlite3; print(sqlite3.sqlite_version)"')

with open(r'D:\Loom\spikes\raw_results.txt', 'w') as f:
    f.write('\n'.join(results))

print('Spikes executed')
