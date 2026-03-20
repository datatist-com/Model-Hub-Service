# Frontend-Driven Backend Docs

This directory documents backend API and database requirements inferred from the current frontend codebase.

## Scope

These docs are generated from frontend evidence in `src/`:
- routes and pages (`src/router/index.tsx`, `src/pages/*`)
- business components (`src/layouts/AppLayout.tsx`, `src/components/*`)
- local mock/domain constants (`src/constants/mockMaps.ts`)
- auth/session helpers (`src/auth/*`)
- form/table actions and workflow status in UI

Current repository has almost no real HTTP client/service implementation (no fetch/axios service layer for business APIs). Most contracts below are inferred from UI workflows and mock data structures.

## Confidence Levels

- Confirmed: explicitly represented in code structures, fields, or routing/action behavior.
- Strongly Inferred: not directly called via HTTP in code, but clearly required by page actions.
- Uncertain: requires backend confirmation.

## Document Index

- [API Overview](api-overview.md)
- [OpenAPI Draft](openapi.yaml)
- [API Examples](api-examples.md)
- API domains:
  - [Dashboard](api/dashboard.md)
  - [Auth](api/auth.md)
  - [Users and Profile](api/users-and-profile.md)
  - [License](api/license.md)
  - [Data Platform (Data Sources, Hive, DuckDB, Ingest)](api/data-platform.md)
  - [Features](api/features.md)
  - [Portraits and Targets](api/portraits-and-targets.md)
  - [Models and Scoring](api/models-and-scoring.md)
  - [Operations](api/operations.md)
  - [SQL and Logs](api/sql-and-logs.md)
- Database docs:
  - [Database Overview](database/overview.md)
  - [Tables and Fields](database/tables.md)
  - [Relationships](database/relationships.md)
  - [Enums and Statuses](database/enums-and-statuses.md)

## Backend Confirmation Checklist

Highest-priority confirmations:
- final endpoint paths and HTTP methods
- unified response envelope and error schema
- async job model (polling vs push)
- permission model granularity (role-only vs resource-level ACL)
- delete semantics (hard vs soft delete)
- pagination/filter/sort standard
