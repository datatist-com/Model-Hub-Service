# SQL and Logs API

Frontend usage evidence:
- `src/pages/SqlConsolePage.tsx`
- `src/pages/LogViewerPage.tsx`
- `src/pages/ProfilePage.tsx`

## SQL Console

### POST /sql/execute
- Name: Execute SQL
- Purpose: run ad hoc SQL on selected data source
- Auth: Yes
- Request body (inferred):
  - `sourceId`
  - `sql`
- Response body (inferred):
  - `columns`
  - `rows`
  - `rowCount`
  - `durationMs`
  - `status`
- Error cases:
  - syntax error
  - timeout
  - permission denied
- Confidence: Strongly Inferred
- Priority: Core

### GET /sql/history
- Name: SQL history
- Purpose: list prior executions
- Query params: `sourceId`, `page`, `pageSize`
- Confidence: Strongly Inferred
- Priority: Secondary

### DELETE /sql/history/{id}
- Name: delete history row
- Purpose: cleanup history entries
- Confidence: Uncertain
- Priority: Optional

## Logs

### GET /logs
- Name: Log list
- Purpose: operation and login log viewer
- Query params (inferred):
  - `type` (`login`, `operation`)
  - `user`
  - `keyword`
  - `timeRange`
  - pagination/sort params
- Response fields:
  - `id`, `timestamp`, `type`, `user`, `ip`, `action`, `detail`, `status`
- Confidence: Strongly Inferred
- Priority: Core

### GET /profile/login-history
### GET /profile/action-history
- Name: Profile scoped logs
- Purpose: personal history panels in profile page
- Confidence: Strongly Inferred
- Priority: Secondary

## Notes
- Frontend indicates both system-wide logs and user-scoped profile logs.
- Backend should define retention and masking rules for sensitive log data.
