#!/usr/bin/env python3
"""
Loom Vault Validator — validates docs/ Obsidian vault integrity.
Enforces strict rules on roadmap rollups, specs, dependencies, and evidence.
"""

import os
import re
import sys
import yaml
from pathlib import Path
from collections import Counter, defaultdict

VAULT_ROOT = Path(os.environ.get("LOOM_VAULT_ROOT", "docs"))
BANNED_STRINGS = ["definitively", "xterm-addon-webgl"]
REQUIRED_FRONTMATTER = {"id", "type", "status", "updated"}
VALID_TYPES = {"adr", "spike", "task", "spec", "research", "review", "guide", "decision", "milestone"}
VALID_STATUSES = {"draft", "proposed", "provisional", "confirmed", "in-progress", "done", "blocked", "superseded"}

errors = []
warnings = []

def error(file, msg):
    errors.append(f"ERROR [{file}]: {msg}")

def warn(file, msg):
    warnings.append(f"WARN  [{file}]: {msg}")

def parse_frontmatter(filepath):
    try:
        content = filepath.read_text(encoding="utf-8")
    except Exception as e:
        error(filepath.name, f"Cannot read: {e}")
        return None, ""
    
    if not content.startswith("---"):
        return None, content
    
    parts = content.split("---", 2)
    if len(parts) < 3:
        return None, content
    
    try:
        fm = yaml.safe_load(parts[1])
        return fm if isinstance(fm, dict) else None, parts[2]
    except yaml.YAMLError as e:
        error(filepath.name, f"Invalid YAML frontmatter: {e}")
        return None, content

def check_acyclic_deps(tasks):
    """Check for cycles in depends_on fields."""
    visited = set()
    path = set()
    
    def visit(task_id):
        if task_id in path:
            error("depends_on", f"Cycle detected involving {task_id}")
            return False
        if task_id in visited:
            return True
            
        path.add(task_id)
        for dep in tasks.get(task_id, {}).get("depends_on", []):
            if not visit(dep):
                return False
        path.remove(task_id)
        visited.add(task_id)
        return True
        
    for tid in tasks:
        visit(tid)

def validate_vault():
    if not VAULT_ROOT.exists():
        print(f"Vault root not found: {VAULT_ROOT}")
        sys.exit(1)

    all_files = {}
    tasks = {}
    specs = {}
    features = set()
    
    for md_file in sorted(VAULT_ROOT.rglob("*.md")):
        if "_archive" in str(md_file):
            continue
            
        fm, content = parse_frontmatter(md_file)
        all_files[md_file] = {"fm": fm, "content": content}
        
        # Check banned strings
        lower = content.lower()
        for bs in BANNED_STRINGS:
            if bs.lower() in lower:
                error(md_file.name, f"Banned string found: '{bs}'")
                
        # 04-Spec type check
        if "04-Spec" in md_file.parts:
            if not fm or fm.get("type") != "spec":
                error(md_file.name, "File in 04-Spec lacks type: spec")
            else:
                specs[md_file.name] = (fm, content)
                
        if fm:
            if fm.get("type") == "task":
                tasks[fm.get("id")] = fm
                if "features" in fm:
                    features.update(fm["features"])
            
            # Evidence link resolution
            links = re.findall(r'\[\[evidence/([^\]]+)\]\]', content)
            for link in links:
                ev_path = VAULT_ROOT / "evidence" / link
                if not ev_path.exists():
                    error(md_file.name, f"Evidence file does not exist: {link}")

    # Core features check
    for f in range(1, 7):
        if f not in features:
            error("tasks", f"Core feature {f} has no associated tasks")
            
    # Specs structural checks
    required_sections = ["Scope", "Interfaces/schemas", "Data", "Security", "UI", "Acceptance tests", "Open questions"]
    for spec_name, (fm, content) in specs.items():
        for section in required_sections:
            if not re.search(r'^#+\s+' + re.escape(section), content, re.MULTILINE | re.IGNORECASE):
                error(spec_name, f"Missing required section: '{section}'")
        if not re.search(r'\[\[ADR-\d+', content) and not re.search(r'\[\[ADR-\d+', str(fm)):
            error(spec_name, "Missing link to an ADR")
        if not re.search(r'\[\[T-\d+', content) and not re.search(r'\[\[T-\d+', str(fm)):
            error(spec_name, "Missing link to tasks")
            
    # Roadmap parsing
    roadmap_path = VAULT_ROOT / "05-Roadmap" / "roadmap.md"
    if roadmap_path.exists():
        roadmap_content = roadmap_path.read_text(encoding="utf-8")
        
        sections = re.split(r'^##\s+', roadmap_content, flags=re.MULTILINE)
        for section in sections[1:]:
            lines = section.splitlines()
            title = lines[0].strip()
            
            est_match = re.search(r'Estimate:\s*(\d+)-(\d+)h', section)
            if est_match:
                expected_opt, expected_pess = int(est_match.group(1)), int(est_match.group(2))
                
                # Sum task estimates
                task_ids = re.findall(r'- \[ \]\s+(T-\d+)', section)
                sum_opt = 0
                sum_pess = 0
                for tid in task_ids:
                    if tid not in tasks:
                        error("roadmap.md", f"Referenced task {tid} has no task note")
                        continue
                    est = tasks[tid].get("estimate_hours", {})
                    sum_opt += est.get("optimistic", 0)
                    sum_pess += est.get("pessimistic", 0)
                    
                if sum_opt != expected_opt or sum_pess != expected_pess:
                    error("roadmap.md", f"Section '{title}' estimate {expected_opt}-{expected_pess}h does not match task sum {sum_opt}-{sum_pess}h")
                    
    # Dependency cycles
    check_acyclic_deps(tasks)

    print(f"Loom Vault Validation Report")
    print(f"Files scanned: {len(all_files)}")
    print(f"Errors: {len(errors)}")
    for e in errors:
        print(f"  {e}")
        
    if errors:
        sys.exit(1)
    else:
        print("\nPASSED: Validation passed")
        sys.exit(0)

if __name__ == "__main__":
    if len(sys.argv) > 2 and sys.argv[1] == "--vault-root":
        VAULT_ROOT = Path(sys.argv[2])
    validate_vault()
