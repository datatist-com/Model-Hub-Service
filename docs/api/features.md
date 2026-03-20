# Features API

Frontend usage evidence:
- `src/pages/FeatureManagementPage.tsx`
- `src/pages/FeatureFieldDetailPage.tsx`

## GET /features
- Name: List feature sources
- Purpose: render feature source table
- Response fields (inferred):
  - `id`, `sourceName`, `sourceType`, `database`, `tableName`
  - `tableType`, `customerIdField`, `timeField`, `featureFields`
- Confidence: Strongly Inferred
- Priority: Core

## POST /features
- Name: Create feature source config
- Request body (inferred):
  - `sourceId`, `database`, `tableName`
  - `tableType` (`monthly` or `flow`)
  - `customerIdField`, `timeField`
  - `featureFields: string[]`
- Confidence: Strongly Inferred
- Priority: Core

## PUT /features/{id}
- Name: Update feature source config
- Confidence: Strongly Inferred
- Priority: Core

## DELETE /features/{id}
- Name: Delete feature source
- Confidence: Strongly Inferred
- Priority: Core

## GET /features/{id}/fields
- Name: Derived feature detail
- Purpose: detail page with categories/statistics periods
- Response body (inferred):
  - source field groups
  - derived feature items:
    - `featureName`
    - `category` (`source`, `basic`, `yoy`, `mom`)
    - `statLabel`
    - `periodLabel`
- Confidence: Strongly Inferred
- Priority: Core

## GET /features/{id}/stats (optional)
- Name: Field stats for details
- Purpose: support expanded detail cards
- Confidence: Uncertain
- Priority: Optional

## Notes
- Feature derivation rules are visible in frontend logic and should ideally be backend-driven for consistency and traceability.
