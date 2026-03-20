# Operations API

Frontend usage evidence:
- `src/pages/OperationListOutputPage.tsx`
- `src/pages/OperationListCreatePage.tsx`
- `src/pages/OperationListDetailPage.tsx`

## GET /operations
- Name: List operations
- Purpose: operation/crowd output overview
- Response fields (inferred):
  - `id`, `name`, `modelName`, `scoreRule`, `conditionCount`, `abTest`, `status`
- Confidence: Strongly Inferred
- Priority: Core

## POST /operations
- Name: Create operation
- Purpose: save segmentation rules and AB test config
- Request body (inferred):
  - `name`, `modelId`, `scoreRule`
  - `conditions[]` (field/operator/value or expression form)
  - `abTest: { enabled, unit, value }`
- Confidence: Strongly Inferred
- Priority: Core

## GET /operations/{id}
- Name: Operation detail
- Purpose: load full definition and monthly outputs
- Confidence: Strongly Inferred
- Priority: Core

## PUT /operations/{id}
- Name: Update operation
- Purpose: edit operation definition
- Confidence: Inferred
- Priority: Secondary

## DELETE /operations/{id}
- Name: Delete operation
- Purpose: remove operation definition
- Confidence: Strongly Inferred
- Priority: Core

## POST /operations/{id}/outputs
- Name: Generate monthly output
- Purpose: produce operation output for selected month
- Request body: `month`
- Response body: output job metadata
- Confidence: Strongly Inferred
- Priority: Core

## GET /operations/{id}/outputs/{outputId}
- Name: Output detail rows
- Purpose: paged customer output data
- Confidence: Inferred
- Priority: Secondary

## POST /operations/{id}/ab-test-outputs
- Name: Generate AB test output
- Purpose: monthly AB test split output generation
- Confidence: Strongly Inferred
- Priority: Secondary

## Notes
- UI supports score rules such as top percentage/range and top N.
- Backend must validate rule grammar and condition expressions safely.
