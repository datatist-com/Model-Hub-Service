# Models and Scoring API

Frontend usage evidence:
- `src/pages/ModelManagementPage.tsx`
- `src/pages/ModelDetailPage.tsx`
- `src/pages/modelDetail/ModelLiftSection.tsx`
- `src/pages/ModelTrainPage.tsx`
- `src/pages/ScoringGenerationPage.tsx`
- `src/pages/ModelScoringListPage.tsx`
- `src/pages/ModelListDetailPage.tsx`

## Models

### GET /models
- Purpose: model list/grid
- Response fields (inferred):
  - `id`, `modelName`, `portrait`, `target`, `algorithm`
  - `auc`, `liftTop10`, `published`
- Confidence: Strongly Inferred
- Priority: Core

### POST /models
### DELETE /models/{id}
- Purpose: create/delete model
- Notes: delete flow implies password confirmation/audit
- Confidence: Strongly Inferred
- Priority: Core

### GET /models/{id}
- Purpose: model detail page
- Includes: core metadata + feature importance list
- Confidence: Strongly Inferred
- Priority: Core

### POST /models/{id}/publish
### POST /models/{id}/unpublish
- Purpose: publication state transitions
- Confidence: Strongly Inferred
- Priority: Core

## Lift and Feature Analytics

### GET /models/{id}/lift
- Purpose: lift curve/tabs data
- Query params: `granularity=decile|percentile|permille`
- Response fields: `rank`, `liftValue`, `cumLiftValue`
- Confidence: Strongly Inferred
- Priority: Core

### GET /models/{id}/features
- Purpose: feature ranking with weights
- Confidence: Strongly Inferred
- Priority: Secondary

## Training

### GET /models/{id}/training-records
- Purpose: show training/validation records
- Response fields: record list with status/auc/lift/time windows
- Confidence: Strongly Inferred
- Priority: Core

### POST /models/{id}/train
### POST /models/{id}/validate
- Purpose: trigger train/validate jobs
- Request body (inferred): training mode + feature/y-table months
- Response body: `jobId`, `status`
- Confidence: Strongly Inferred
- Priority: Core

## Scoring Lists

### GET /models/{id}/scoring-lists
- Purpose: list scoring outputs by prediction month
- Confidence: Strongly Inferred
- Priority: Core

### POST /models/{id}/scoring-lists
- Purpose: create scoring output list
- Request body: `predictionMonth`
- Confidence: Strongly Inferred
- Priority: Core

### GET /models/{id}/scoring-lists/{listId}
- Purpose: score detail list with pagination/sort
- Response fields: customer score rows
- Confidence: Strongly Inferred
- Priority: Core

### GET /models/{id}/scoring-lists/{listId}/export
- Purpose: export scoring output
- Confidence: Inferred
- Priority: Secondary

## Notes
- Model/scoring pages clearly imply async processing jobs and status polling.
- Backfill generation and rerun rules need backend confirmation.
