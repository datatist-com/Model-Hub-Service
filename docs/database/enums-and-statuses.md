# Enums and Statuses (Inferred)

## users.role
- `model_developer`
- `model_operator`
- `platform_admin`

## users.status
- `active`
- `frozen`

## data_sources.type
- `hive`
- `duckdb`

## feature_sources.table_type
- `monthly`
- `flow`

## portraits.data_source_mode
- `computed`
- `imported`

## targets.data_source_mode
- `computed`
- `imported`

## targets.target_type
- `binary`
- `continuous`

## period status (portrait_periods, target_periods)
- `ready`
- `computing`
- optional `failed`

## model_train_jobs.job_type
- `train`
- `validate`

## model_train_jobs.run_mode
- `sample`
- `full`

## async job status (common pattern)
- `queued`
- `running`
- `succeeded`
- `failed`

## model_scoring_lists.status
- `generating`
- `generated`
- `failed`

## operations.status
- `active`
- `archived`

## operation output status
- `pending`
- `generated`
- `failed`

## operations AB unit
- `percent`
- `count`

## license_info.status
- `missing`
- `active`
- `expired`

## audit_logs.log_type
- `login`
- `operation`
- `data_modification`

## audit_logs.result
- `success`
- `failed`

## Notes
- Enum names above are inferred from current UI strings/status tags.
- Backend can preserve wire compatibility while storing different internal enum names via mapping layer.
