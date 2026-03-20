# Portraits and Targets API

Frontend usage evidence:
- `src/pages/UserPortraitPage.tsx`
- `src/pages/PortraitPeriodPage.tsx`
- `src/pages/TargetManagementPage.tsx`
- `src/pages/TargetPeriodPage.tsx`

## Portraits

### GET /portraits
- Purpose: list portrait definitions
- Response fields (inferred):
  - `id`, `portraitName`, `dataSource`, `sourceTables`
  - `userCount`, `featureCount`, `periodCount`
- Confidence: Strongly Inferred
- Priority: Core

### POST /portraits
### PUT /portraits/{id}
### DELETE /portraits/{id}
- Purpose: CRUD portrait definitions
- Confidence: Strongly Inferred
- Priority: Core

## Portrait Periods

### GET /portraits/{portraitId}/periods
- Purpose: list available portrait periods
- Response fields: `id`, `year`, `month`, `customerCount`, `featureCount`, `status`
- Confidence: Strongly Inferred
- Priority: Core

### POST /portraits/{portraitId}/periods
- Purpose: create or trigger period computation/import
- Request body (inferred): `year`, `month`, optional source/upload mode
- Confidence: Strongly Inferred
- Priority: Core

### POST /portraits/{portraitId}/periods/{periodId}/recalculate
- Purpose: recompute period values
- Confidence: Strongly Inferred
- Priority: Secondary

### DELETE /portraits/{portraitId}/periods/{periodId}
- Purpose: delete period data
- Confidence: Strongly Inferred
- Priority: Secondary

## Targets

### GET /targets
- Purpose: list target definitions
- Response fields (inferred):
  - `id`, `targetName`, `dataSource`, `sourceTables`, `targetType`
  - `description`, `periodCount`
- Confidence: Strongly Inferred
- Priority: Core

### POST /targets
### PUT /targets/{id}
### DELETE /targets/{id}
- Purpose: CRUD target definitions
- Confidence: Strongly Inferred
- Priority: Core

## Target Periods

### GET /targets/{targetId}/periods
- Purpose: list target period stats
- Response fields: `id`, `year`, `month`, `totalSamples`, `positiveSamples`, `negativeSamples`, `status`
- Confidence: Strongly Inferred
- Priority: Core

### POST /targets/{targetId}/periods
- Purpose: create/compute period
- Confidence: Strongly Inferred
- Priority: Core

### POST /targets/{targetId}/periods/{periodId}/recalculate
### DELETE /targets/{targetId}/periods/{periodId}
- Purpose: recompute/delete target period
- Confidence: Strongly Inferred
- Priority: Secondary

## Notes
- Data source mode (`computed` vs `imported`) is central in both portrait and target workflows.
- Backend should enforce uniqueness for `(portrait_id, year, month)` and `(target_id, year, month)`.
