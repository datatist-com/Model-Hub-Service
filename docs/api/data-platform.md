# Data Platform API (Data Sources, Hive, DuckDB, Ingest)

Frontend usage evidence:
- `src/pages/DataSourcesPage.tsx`
- `src/pages/HiveDatabasesPage.tsx`
- `src/pages/HiveTablesPage.tsx`
- `src/pages/DuckDBTablesPage.tsx`
- `src/pages/IngestJobsPage.tsx`

## Data Sources

### GET /data-sources
- Purpose: list configured data sources
- Response fields (inferred): `id`, `name`, `type`, `connected`, `objectCount`
- Confidence: Strongly Inferred
- Priority: Core

### POST /data-sources
- Purpose: create data source
- Request body (inferred): `name`, `type`, `connectionAddress` (for Hive), optional credentials
- Confidence: Strongly Inferred
- Priority: Core

### PUT /data-sources/{id}
- Purpose: update source config
- Confidence: Strongly Inferred
- Priority: Core

### DELETE /data-sources/{id}
- Purpose: delete source
- Confidence: Strongly Inferred
- Priority: Core

### POST /data-sources/{id}/test-connection
- Purpose: verify connectivity
- Confidence: Strongly Inferred
- Priority: Secondary

## Hive metadata

### GET /data-sources/{sourceId}/hive-databases
- Purpose: list Hive databases
- Query params: pagination/search optional
- Confidence: Strongly Inferred
- Priority: Core

### GET /data-sources/{sourceId}/hive-tables
- Purpose: list tables in a database
- Query params: `database`
- Confidence: Strongly Inferred
- Priority: Core

### GET /data-sources/{sourceId}/hive-tables/{table}/fields
- Purpose: table schema details
- Confidence: Strongly Inferred
- Priority: Core

### DELETE /data-sources/{sourceId}/hive-tables/{table}
- Purpose: delete hive table
- Confidence: Strongly Inferred
- Priority: Secondary

## DuckDB metadata and upload

### GET /data-sources/{sourceId}/duckdb-tables
- Purpose: list duckdb tables
- Confidence: Strongly Inferred
- Priority: Core

### GET /data-sources/{sourceId}/duckdb-tables/{table}/fields
- Purpose: table field details
- Confidence: Strongly Inferred
- Priority: Core

### POST /data-sources/{sourceId}/duckdb-ingest
- Purpose: upload file and ingest to table
- Request: multipart file upload + target table metadata
- Confidence: Strongly Inferred
- Priority: Core

### PUT /data-sources/{sourceId}/duckdb-tables/{table}
- Purpose: update table state (enable/disable)
- Confidence: Inferred
- Priority: Secondary

### DELETE /data-sources/{sourceId}/duckdb-tables/{table}
- Purpose: delete duckdb table
- Confidence: Strongly Inferred
- Priority: Secondary

## Ingest jobs

### GET /ingest/jobs
- Purpose: list ingest jobs and progress
- Filters: source/table/status/time
- Response fields (inferred): `jobId`, `tableName`, `status`, `progress`, `createdAt`, `errorMessage`
- Confidence: Strongly Inferred
- Priority: Core

### GET /ingest/jobs/{jobId}
- Purpose: job detail/polling
- Confidence: Inferred
- Priority: Secondary

## Notes
- UI indicates async job lifecycle for ingestion.
- Backend should standardize status transitions: `queued`, `running`, `succeeded`, `failed`.
