---
id: ADR-007
type: adr
status: confirmed
updated: 2026-09-20
tags: [architecture, memory, database]
spike: S7
---

# ADR-007: Semantic Memory

## Decision
SQLite FTS5 + sqlite-vec for embeddings.

## Context
Loom needs a local-first semantic memory system for agents to store and retrieve context across sessions. Must be embedded (no server), support full-text search and vector similarity.

## Options Evaluated

| Option | Score | Notes |
|--------|-------|-------|
| **SQLite FTS5 + sqlite-vec** | **4.40** | Embedded, native, FTS + vector in one DB |
| Mem0 SDK | 3.30 | External dependency, cloud-optional |
| Letta/Graphiti | 2.85 | Requires Docker/WSL2 [P] |
| Cognee | 2.70 | External dependency |

## Evidence
- SQLite 3.49.1 available on this machine (confirmed in spike runner)
- FTS5 sync triggers defined and validated in spec
- sqlite-vec v0.1.3 loads as extension [P] https://github.com/asg017/sqlite-vec (2026-09-19)

## Schema
```sql
CREATE TABLE memory_nodes (
    id INTEGER PRIMARY KEY,
    title TEXT,
    content TEXT,
    path TEXT UNIQUE,
    provenance TEXT,
    staleness_timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    embedding_model TEXT
);

CREATE VIRTUAL TABLE memory_fts USING fts5(
    title, content, content='memory_nodes', content_rowid='id'
);

CREATE VIRTUAL TABLE vectors USING vec0(embedding float[384]);

CREATE TABLE graph_edges (
    source_id INTEGER REFERENCES memory_nodes(id),
    target_id INTEGER REFERENCES memory_nodes(id),
    relationship TEXT
);
```

## Counter-Argument
Must write embedding/RAG logic from scratch. No built-in chunking or retrieval strategies.

## Change My Mind If
An embedded vector DB with wider adoption replaces sqlite-vec.

## Related
- [[ADR-001-framework]]
