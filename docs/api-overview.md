# API Overview

## Current Frontend Integration State

- No dedicated API client/service layer was found in current frontend sources.
- Most pages use local mock data and local state updates.
- API contracts in this folder are inferred from:
  - list/detail tables
  - create/edit/delete forms and modals
  - status transitions and action buttons
  - route params and detail pages

## Suggested API Conventions

## Base
- Base path: `/api/v1`
- Auth: Bearer token for authenticated modules

## Response Envelope (recommended)

```json
{
  "code": "OK",
  "message": "success",
  "data": {},
  "requestId": "uuid"
}
```

Error envelope:

```json
{
  "code": "VALIDATION_ERROR",
  "message": "human readable",
  "details": {}
}
```

## Pagination and Filters

Recommended list query format:
- `page` (1-based)
- `pageSize`
- `sortBy`
- `sortOrder` (`asc` or `desc`)
- domain filters per page

Recommended list response:

```json
{
  "items": [],
  "page": 1,
  "pageSize": 20,
  "total": 123
}
```

## Auth and Session

Frontend evidence:
- token presence checks in auth guards
- role-based menu filtering
- profile password change and login/action history

## Status/Job Patterns

Several pages imply long-running tasks:
- model train/validate
- scoring generation
- portrait/target period compute
- ingest jobs

Recommended pattern:
- trigger endpoint returns `jobId`
- query endpoint returns `status`, `progress`, timestamps, and `errorMessage`

## Confidence Note

Each domain file explicitly tags endpoints as:
- Confirmed (frontend structure confirms fields/actions)
- Strongly Inferred (UI requires it)
- Uncertain (backend policy/detail pending)
