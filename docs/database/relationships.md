# Relationships (Inferred)

## One-to-Many

- users -> data_sources
- users -> feature_sources
- users -> portraits
- users -> targets
- users -> models
- users -> operations
- users -> ingest_jobs
- users -> sql_query_history
- users -> audit_logs

- data_sources -> feature_sources
- data_sources -> ingest_jobs
- data_sources -> sql_query_history

- portraits -> portrait_periods
- targets -> target_periods

- models -> model_features
- models -> model_train_jobs
- models -> model_scoring_lists

- model_scoring_lists -> model_scoring_rows

- operations -> operation_conditions
- operations -> operation_outputs
- operations -> operation_ab_outputs

## Many-to-One

- models -> portraits
- models -> targets
- operations -> models

## Potential Many-to-Many (if future scope expands)

Current UI can be implemented without these, but future-proof options:
- model <-> feature_sources (junction table `model_feature_sources`) if model may use multiple feature sources explicitly.
- role <-> permission (`role_permissions`) if authorization grows beyond fixed role keys in frontend.

## Referential Integrity Suggestions

- Use foreign keys for all parent-child entities above.
- For high-write async tables (jobs/logs), allow optional deferred FK checks if needed for throughput.
- Keep deletion policy explicit:
  - recommended default: `RESTRICT` on core entities (`models`, `portraits`, `targets`)
  - optional soft-delete pattern if backend wants recovery/audit semantics.

## Uncertainties Requiring Confirmation

- Whether deleting `data_sources` should cascade to `feature_sources` and downstream modeling artifacts.
- Whether `operations` and scoring outputs should remain immutable snapshots even after model deletion.
- Whether `license_info` is single-tenant single-row or tenant-scoped multi-row.
