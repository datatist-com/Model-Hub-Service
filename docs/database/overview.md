# Database Overview (Inferred)

This schema is inferred from frontend workflows and field shapes. It is not backend-confirmed DDL.

## Design Goals
- cover all visible UI CRUD and list/detail flows
- support async job/status patterns (train, scoring, ingest, compute)
- keep relations normalized where multi-entity links are clear
- preserve auditability for admin/security flows

## Core Entity Groups

- Identity and access:
  - `users`
  - optional `user_sessions`
  - optional `role_permissions`
- Data platform:
  - `data_sources`
  - `hive_databases`, `hive_tables`, `hive_table_fields`
  - `duckdb_tables`, `duckdb_table_fields`
  - `ingest_jobs`
- Feature and label engineering:
  - `feature_sources`, `feature_fields_derived`
  - `portraits`, `portrait_periods`
  - `targets`, `target_periods`
- Modeling and scoring:
  - `models`, `model_features`
  - `model_train_jobs`
  - `model_scoring_lists`, `model_scoring_rows`
- Operations:
  - `operations`, `operation_conditions`
  - `operation_outputs`, `operation_ab_outputs`
- Platform ops:
  - `license_info`
  - `audit_logs`
  - `sql_query_history`

## Confirmed vs Inferred

- Confirmed by code shape:
  - model fields: name, portrait, target, algorithm, auc, lift, publish state
  - portrait/target period structures (year/month + counts + status)
  - operation definition has score rules + conditions + AB config
  - license requires masked key + org + activated/expires timestamps
- Strongly inferred:
  - table names and endpoints to persist these objects
  - separate scoring list and scoring row tables for detail/export pages
- Uncertain:
  - exact auth/session storage model
  - hard-delete vs soft-delete
  - exact indexing strategy under production data size
