---
id: SPEC-MEMORY
type: spec
status: in-progress
updated: '2026-09-21'
tags:
- spec
features:
- 4
---

# Spec: Semantic Memory Vault

## Related

- [[ADR-007]]
- [[T-053]]
- [[T-057]]

## Scope

Manages a local markdown vault parsed into an SQLite database with FTS5 and vector search (sqlite-vec) to provide long-term semantic memory for agents.

## Interfaces/schemas

```rust
pub fn search_hybrid(query: &str) -> Vec<Note> {
    // Combines FTS5 BM25 + sqlite-vec cosine similarity
}
```

## Data

Raw data resides in `.md` files. SQLite acts as a synchronized materialized view of the filesystem. Embeddings stored as blob vectors.

## Security

- Threat 1: Agent overwrites critical notes. Mitigation: Version history and write contention locking.
- Threat 2: SQL injection via search queries. Mitigation: Parameterized queries strictly enforced.
- Threat 3: Path traversal during file reads. Mitigation: Jail reads to vault directory.

## UI

Graph view visualization of note links. Split pane markdown editor.

## Acceptance tests

- Verify filesystem watcher updates SQLite within 1 second of file change.
- Verify vector search returns semantically relevant results.
- Verify hybrid search merges scores correctly.

## Open questions

- Which embedding model provides the best balance of speed and accuracy for local generation?
