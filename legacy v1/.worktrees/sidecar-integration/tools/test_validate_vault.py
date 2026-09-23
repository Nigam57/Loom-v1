import os
import sys
import pytest
from pathlib import Path

# Add tools directory to sys.path so we can import validate_vault
sys.path.insert(0, str(Path(__file__).parent))
import validate_vault

@pytest.fixture(autouse=True)
def reset_errors():
    """Reset the global errors list before each test."""
    validate_vault.errors = []
    validate_vault.warnings = []
    yield
    validate_vault.errors = []
    validate_vault.warnings = []

def test_missing_spec_type_is_error(tmp_path):
    """Negative test: a file in 04-Spec without type: spec fails."""
    spec_dir = tmp_path / "04-Spec"
    spec_dir.mkdir()
    
    bad_spec = spec_dir / "bad_spec.md"
    bad_spec.write_text("---\nid: BAD-SPEC\ntype: document\n---\n# Bad Spec\n")
    
    validate_vault.VAULT_ROOT = tmp_path
    
    with pytest.raises(SystemExit) as excinfo:
        validate_vault.validate_vault()
        
    assert excinfo.value.code == 1
    assert any("lacks type: spec" in e for e in validate_vault.errors)

def test_stub_spec_fails_sections(tmp_path):
    """Negative test: a spec missing required sections fails."""
    spec_dir = tmp_path / "04-Spec"
    spec_dir.mkdir()
    
    stub_spec = spec_dir / "stub.md"
    stub_spec.write_text("---\nid: STUB\ntype: spec\nfeatures: [1]\n---\n# Stub Spec\nJust a stub. [[ADR-001]] [[T-030]]")
    
    validate_vault.VAULT_ROOT = tmp_path
    
    with pytest.raises(SystemExit) as excinfo:
        validate_vault.validate_vault()
        
    assert excinfo.value.code == 1
    assert any("Missing required section: 'Scope'" in e for e in validate_vault.errors)

def test_spec_missing_links(tmp_path):
    """Negative test: a spec missing ADR or Task links fails."""
    spec_dir = tmp_path / "04-Spec"
    spec_dir.mkdir()
    
    bad_spec = spec_dir / "nolinks.md"
    content = "---\nid: NOLINKS\ntype: spec\nfeatures: [1]\n---\n"
    sections = ["Scope", "Interfaces/schemas", "Data", "Security", "UI", "Acceptance tests", "Open questions"]
    for s in sections:
        content += f"## {s}\nTest.\n"
    bad_spec.write_text(content)
    
    validate_vault.VAULT_ROOT = tmp_path
    
    with pytest.raises(SystemExit) as excinfo:
        validate_vault.validate_vault()
        
    assert excinfo.value.code == 1
    assert any("Missing link to an ADR" in e for e in validate_vault.errors)
    assert any("Missing link to tasks" in e for e in validate_vault.errors)

def test_roadmap_mismatch(tmp_path):
    """Negative test: exact roadmap rollup check fails if numbers don't match."""
    # Create tasks
    task_dir = tmp_path / "06-Backlog" / "tasks"
    task_dir.mkdir(parents=True)
    task1 = task_dir / "T-001.md"
    task1.write_text("---\nid: T-001\ntype: task\nfeatures: [1]\nestimate_hours:\n  optimistic: 2\n  pessimistic: 4\n---")
    
    # Create roadmap
    rm_dir = tmp_path / "05-Roadmap"
    rm_dir.mkdir()
    rm = rm_dir / "roadmap.md"
    # Claiming 3-5h but task is 2-4h
    rm.write_text("## MVP-Phase\nEstimate: 3-5h\n- [ ] T-001\n")
    
    validate_vault.VAULT_ROOT = tmp_path
    
    with pytest.raises(SystemExit) as excinfo:
        validate_vault.validate_vault()
        
    assert excinfo.value.code == 1
    assert any("does not match task sum 2-4h" in e for e in validate_vault.errors)

def test_dependency_cycle(tmp_path):
    """Negative test: cyclic task dependencies are detected."""
    task_dir = tmp_path / "06-Backlog" / "tasks"
    task_dir.mkdir(parents=True)
    
    t1 = task_dir / "T-001.md"
    t1.write_text("---\nid: T-001\ntype: task\nfeatures: [1]\ndepends_on: [T-002]\n---")
    
    t2 = task_dir / "T-002.md"
    t2.write_text("---\nid: T-002\ntype: task\nfeatures: [1]\ndepends_on: [T-001]\n---")
    
    validate_vault.VAULT_ROOT = tmp_path
    
    with pytest.raises(SystemExit) as excinfo:
        validate_vault.validate_vault()
        
    assert excinfo.value.code == 1
    assert any("Cycle detected" in e for e in validate_vault.errors)

def test_missing_task_for_roadmap(tmp_path):
    """Negative test: roadmap mentions a task that doesn't exist."""
    rm_dir = tmp_path / "05-Roadmap"
    rm_dir.mkdir(parents=True)
    rm = rm_dir / "roadmap.md"
    rm.write_text("## MVP-Phase\nEstimate: 2-4h\n- [ ] T-999\n")
    
    validate_vault.VAULT_ROOT = tmp_path
    
    with pytest.raises(SystemExit) as excinfo:
        validate_vault.validate_vault()
        
    assert excinfo.value.code == 1
    assert any("Referenced task T-999 has no task note" in e for e in validate_vault.errors)
