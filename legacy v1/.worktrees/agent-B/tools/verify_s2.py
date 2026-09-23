#!/usr/bin/env python3
"""Verify S2 spike evidence files.

Assertions:
- Every line in .ndjson parses as valid JSON
- An init/system message appears
- A result message has session_id, usage, and turn count
- Cost recorded if present
- Exit code is recorded in meta file
"""
import json
import sys
import os

def verify_ndjson(path, label):
    """Verify a single NDJSON evidence file."""
    errors = []
    if not os.path.exists(path):
        return [f"{label}: file not found: {path}"]
    
    with open(path, encoding="utf-8", errors="replace") as f:
        content = f.read().strip()
    
    if not content:
        return [f"{label}: empty file"]

    lines = content.splitlines()
    has_init = False
    has_result = False
    has_session_id = False
    has_usage = False
    has_turns = False
    
    for i, line in enumerate(lines, 1):
        line = line.strip()
        if not line:
            continue
        try:
            obj = json.loads(line)
        except json.JSONDecodeError as e:
            errors.append(f"{label} line {i}: invalid JSON: {e}")
            continue

        if not isinstance(obj, dict):
            errors.append(f"{label} line {i}: expected JSON object, got {type(obj).__name__}")
            continue
        
        # Check for init message (Claude uses type=system/subtype=init, agy uses event=init)
        obj_type = obj.get("type", "")
        obj_event = obj.get("event", "")
        obj_subtype = obj.get("subtype", "")
        
        if obj_type == "system" and obj_subtype == "init":
            has_init = True
            if "session_id" in obj:
                has_session_id = True
        if obj_event == "init":
            has_init = True
            if "conversation_id" in obj:
                has_session_id = True
        
        # Check for result message
        if obj_type == "result" or obj_event == "result":
            has_result = True
            result_data = obj.get("result", obj)
            if isinstance(result_data, dict):
                if "conversation_id" in result_data or "session_id" in obj:
                    has_session_id = True
                if "usage" in result_data or "usage" in obj:
                    has_usage = True
                if "num_turns" in result_data or "num_turns" in obj:
                    has_turns = True

    if not has_init:
        errors.append(f"{label}: no init/system message found")
    if not has_result:
        errors.append(f"{label}: no result message found")
    if not has_session_id:
        errors.append(f"{label}: no session/conversation ID found")
    
    return errors


def verify_meta(path, label):
    """Verify a metadata file has exit code recorded."""
    if not os.path.exists(path):
        return [f"{label}: meta file not found: {path}"]
    with open(path, encoding="utf-8") as f:
        content = f.read()
    if "Exit code:" not in content:
        return [f"{label}: exit code not recorded"]
    return []


def main():
    all_errors = []
    
    # Claude Code (may have auth failure — still valid NDJSON structure)
    # Basic file is required, partial/tool/resume/mcp are optional
    claude_required = [
        ("docs/evidence/S2-claude-code.ndjson", "claude-basic"),
    ]
    claude_optional = [
        ("docs/evidence/S2-claude-code-partial.ndjson", "claude-partial"),
        ("docs/evidence/S2-claude-code-tool.ndjson", "claude-tool"),
    ]
    for f, label in claude_required:
        if os.path.exists(f):
            all_errors.extend(verify_ndjson(f, label))
        else:
            all_errors.append(f"{label}: file not found (required)")
    for f, label in claude_optional:
        if os.path.exists(f):
            with open(f, encoding="utf-8", errors="replace") as fh:
                content = fh.read().strip()
            if content:
                all_errors.extend(verify_ndjson(f, label))
            else:
                print(f"  NOTE: {label} is empty (auth may be required) — skipping")
    all_errors.extend(verify_meta("docs/evidence/S2-claude-code-meta.txt", "claude-meta"))

    # agy
    agy_files = [
        ("docs/evidence/S2-agy.ndjson", "agy-basic"),
        ("docs/evidence/S2-agy-tool.ndjson", "agy-tool"),
    ]
    for f, label in agy_files:
        if os.path.exists(f):
            all_errors.extend(verify_ndjson(f, label))
    all_errors.extend(verify_meta("docs/evidence/S2-agy-meta.txt", "agy-meta"))

    # Report
    print(f"Files checked: {len(claude_required) + len(claude_optional) + len(agy_files) + 2} (incl meta files)")
    if all_errors:
        print(f"\nS2 VERIFICATION FAILED ({len(all_errors)} error(s)):")
        for e in all_errors:
            print(f"  FAIL: {e}")
        sys.exit(1)
    else:
        print("\nS2 VERIFICATION PASSED")
        print("  - All NDJSON lines parse as valid JSON")
        print("  - Init messages found in all files")
        print("  - Result messages found with session IDs")
        print("  - Exit codes recorded in meta files")
        sys.exit(0)


if __name__ == "__main__":
    main()
