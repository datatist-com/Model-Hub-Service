# Dashboard API

Frontend usage evidence:
- `src/pages/DashboardPage.tsx`

## GET /dashboard/summary
- Name: Dashboard summary metrics
- Purpose: cards/high-level model stats
- Auth: Yes
- Request params: optional time range
- Response body (inferred):
  - aggregate counts/status
  - model quality summary
  - hub/category summaries
- Confidence: Strongly Inferred
- Priority: Core

## GET /dashboard/auc-trends
- Name: AUC trend series
- Purpose: chart data for hubs over time
- Auth: Yes
- Request params (inferred):
  - `fromMonth`, `toMonth`
  - optional `hub`
- Response body (inferred):
  - `months: string[]`
  - `series: [{ hub, values[] }]`
- Confidence: Strongly Inferred
- Priority: Core

## Notes
- Dashboard currently uses mock arrays; backend endpoint should return chart-ready structure to avoid extra client-side transformation cost.
