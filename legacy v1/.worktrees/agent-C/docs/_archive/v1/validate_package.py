import os
import re
import sys

base_dir = r'D:\Loom'

def check_files():
    report = open(os.path.join(base_dir, 'loom_research_report.md'), 'r', encoding='utf-8').read()
    spec = open(os.path.join(base_dir, 'loom_build_spec.md'), 'r', encoding='utf-8').read()
    backlog = open(os.path.join(base_dir, 'loom_backlog.md'), 'r', encoding='utf-8').read()
    trace = open(os.path.join(base_dir, 'traceability.md'), 'r', encoding='utf-8').read()

    # Check 1: Wrong strings
    all_text = report + spec + backlog + trace
    wrong_strings = ['xterm-addon-webgl', 'definitively']
    for ws in wrong_strings:
        if ws.lower() in all_text.lower():
            raise ValueError(f"Found wrong string: {ws}")
            
    # Check 2: MIT on Claude Squad
    if 'MIT' in report.split('Claude Squad')[1][:50]:
        raise ValueError('Found MIT near Claude Squad')

    # Check 3: ADR structural rules
    adrs = report.split('Total (5.0) |')[1:]
    for i, adr in enumerate(adrs):
        options = set(re.findall(r'\|\s*\*\*(\d+)\.', adr))
        if len(options) < 3:
            raise ValueError(f"ADR {i} has fewer than 3 options")
            
        if '*Change mind if:*' not in adr and '*Confirming Spike:*' not in adr:
            raise ValueError(f"ADR {i} lacks Change my mind / Spike fields")

    # Check 4: Sources
    sources = re.findall(r'- \[(P|S)\] .*? - (http.*)', report)
    if not sources:
        raise ValueError("Sources missing or badly formatted")

    # Check 5: Backlog Tasks and Math
    task_rows = re.findall(r'\|\s*T\d+\s*\|.*?\|.*?\|\s*(\d+)\s*\|.*?\|', backlog)
    if len(task_rows) < 30:
        raise ValueError("Fewer than 30 tasks found in backlog")
    
    total_hours = sum(int(h) for h in task_rows)
    if total_hours < 200 or total_hours > 300:
        raise ValueError(f"Backlog hours ({total_hours}) out of expected range 200-300.")
        
    print('Validation passed successfully.')

if __name__ == '__main__':
    try:
        check_files()
    except Exception as e:
        print(f"Validation failed: {e}")
        sys.exit(1)
