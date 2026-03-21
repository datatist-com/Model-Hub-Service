# Tables and Fields (Inferred)

Field types are practical guesses (`uuid/string/int/decimal/json/timestamp`). Backend may adjust.

## users
- Purpose: account, role, profile preferences
- PK: `id`
- Fields:
  - `id` (string/uuid, required)
  - `username` (string, required, unique)
  - `password_hash` (string, required)
  - `real_name` (string, nullable)
  - `role` (enum, required)
  - `status` (enum, required, default active)
  - `language` (string, nullable)
  - `ui_theme` (enum, nullable)
  - `created_at`, `updated_at` (timestamp)

## tokens
- Purpose: session tokens — DB-backed auth replacing stateless JWT
- PK: `id`
- Fields:
  - `id` (string/uuid, required)
  - `user_id` (fk users.id, ON DELETE CASCADE, required)
  - `token` (string 32 chars alphanumeric, unique, required)
  - `ip` (string, nullable) — client IP at login time; resolved from
    X-Forwarded-For / X-Real-IP when direct peer is a private address
    (10/8, 172.16/12, 192.168/16), otherwise peer socket address
  - `device` (string, nullable) — OS + Browser parsed from User-Agent
    e.g. "macOS Chrome", "iPhone Safari", "Windows Edge"
  - `status` (enum: `active` | `revoked`, default `active`)
  - `created_at` (timestamp, default now)
  - `expires_at` (timestamp, required) — set to now+24h on create;
    extended to now+24h on every `GET /auth/token` call
- Indexes: `token` (unique lookup), `user_id`

- Purpose: hive/duckdb source registry
- PK: `id`
- Fields:
  - `id`, `name`, `type`, `connection_address`
  - `connected` (bool)
  - `object_count` (int)
  - `created_by` (fk users.id)
  - `created_at`, `updated_at`

## feature_sources
- Purpose: feature source definitions from tables
- PK: `id`
- Fields:
  - `id`, `source_id` (fk data_sources.id)
  - `database_name`, `table_name`, `table_type`
  - `customer_id_field`, `time_field`
  - `feature_fields` (json array)
  - `created_by`, `created_at`, `updated_at`

## portraits
- Purpose: portrait definitions
- PK: `id`
- Fields:
  - `id`, `portrait_name`
  - `data_source_mode` (`computed`/`imported`)
  - `source_tables` (json array, nullable)
  - `user_count`, `feature_count`, `period_count`
  - `created_by`, `created_at`, `updated_at`

## portrait_periods
- Purpose: per-month portrait snapshots
- PK: `id`
- Fields:
  - `id`, `portrait_id` (fk portraits.id)
  - `year`, `month`
  - `customer_count`, `feature_count`
  - `status`
  - `computed_at`, `created_at`, `updated_at`
- Unique suggestion:
  - (`portrait_id`, `year`, `month`)

## targets
- Purpose: target definitions for modeling
- PK: `id`
- Fields:
  - `id`, `target_name`
  - `data_source_mode`
  - `source_tables` (json)
  - `target_type` (`binary`/`continuous`)
  - `description`
  - `period_count`
  - `created_by`, `created_at`, `updated_at`

## target_periods
- Purpose: target sample stats per month
- PK: `id`
- Fields:
  - `id`, `target_id` (fk targets.id)
  - `year`, `month`
  - `total_samples`, `positive_samples`, `negative_samples`
  - `status`
  - `computed_at`, `created_at`, `updated_at`
- Unique suggestion:
  - (`target_id`, `year`, `month`)

## models
- Purpose: model registry and publish state
- PK: `id`
- Fields:
  - `id`, `model_name`
  - `portrait_id` (fk portraits.id)
  - `target_id` (fk targets.id)
  - `algorithm`
  - `auc`, `lift_top10`
  - `published` (bool)
  - `created_by`, `created_at`, `updated_at`, `published_at`

## model_features
- Purpose: feature importance rows
- PK: `id`
- Fields:
  - `id`, `model_id` (fk models.id)
  - `feature_name`, `weight`, `rank`

## model_train_jobs
- Purpose: training/validation job records
- PK: `id`
- Fields:
  - `id`, `model_id` (fk models.id)
  - `job_type` (`train`/`validate`)
  - `run_mode` (`sample`/`full`)
  - `feature_months`, `label_month`
  - `status`, `auc`, `lift_top10`
  - `started_at`, `finished_at`, `error_message`

## model_scoring_lists
- Purpose: scoring batch by month
- PK: `id`
- Fields:
  - `id`, `model_id` (fk models.id)
  - `prediction_month`
  - `total_count`
  - `status`
  - `created_by`, `created_at`, `generated_at`

## model_scoring_rows
- Purpose: customer-level scores
- PK: `id`
- Fields:
  - `id`, `list_id` (fk model_scoring_lists.id)
  - `customer_id`, `score`, `rank`
  - `created_at`

## operations
- Purpose: operation/crowd definitions
- PK: `id`
- Fields:
  - `id`, `name`, `model_id` (fk models.id)
  - `score_rule`
  - `ab_test_enabled`, `ab_test_unit`, `ab_test_value`
  - `status`
  - `created_by`, `created_at`, `updated_at`

## operation_conditions
- Purpose: normalized condition rows
- PK: `id`
- Fields:
  - `id`, `operation_id` (fk operations.id)
  - `field_name`, `operator`, `value`, `value_type`
  - `sequence_no`

## operation_outputs
- Purpose: monthly operation outputs
- PK: `id`
- Fields:
  - `id`, `operation_id` (fk operations.id)
  - `month`, `record_count`, `status`
  - `generated_at`, `created_at`

## operation_ab_outputs
- Purpose: monthly AB output records
- PK: `id`
- Fields:
  - `id`, `operation_id` (fk operations.id)
  - `month`, `record_count`, `status`
  - `generated_at`, `created_at`

## ingest_jobs
- Purpose: file ingest asynchronous jobs
- PK: `id`
- Fields:
  - `id`, `source_id` (fk data_sources.id)
  - `table_name`, `file_name`, `file_size`
  - `status`, `progress`
  - `created_by`, `created_at`, `started_at`, `finished_at`, `error_message`

## sql_query_history
- Purpose: SQL console history
- PK: `id`
- Fields:
  - `id`, `source_id` (fk data_sources.id)
  - `sql_text`
  - `status`, `rows_returned`, `duration_ms`, `error_message`
  - `executed_by`, `executed_at`

## licenses
- Purpose: instance-level license activation history; only one row has `status = 'active'`
- PK: `id`
- Fields:
  - `id` (string/uuid, required)
  - `token` (string, the raw license key as submitted)
  - `project_name` (string — decoded from license payload, UTF-16LE)
  - `expires_at` (timestamp ISO-8601 UTC — decoded from license payload)
  - `status` (enum: `active` | `expired` | `replaced`, default `active`)
  - `activated_at` (timestamp, default now)
  - `created_at` (timestamp, default now)
- Indexes: `status`
- Notes:
  - On new activation the previous `active` row is set to `replaced`.
  - License crypto: AES-256-GCM, 12-byte nonce, 25-byte payload, 11-byte tag (embedded key).
  - Payload: 1-byte version (must be 2) + 4-byte BE u32 epoch + 20-byte UTF-16LE project name.

## audit_logs
- Purpose: platform audit and login/operation logs
- PK: `id`
- Fields:
  - `id`, `log_type`, `timestamp`
  - `user_id`, `username`, `ip_address`, `location`
  - `module`, `action`, `detail`
  - `result`, `error_message`
