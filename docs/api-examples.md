# API Request/Response Examples (Inferred)

These examples align with current frontend fields and workflows.

## Auth

### POST /auth/login

Request:
```json
{
  "username": "admin",
  "password": "******"
}
```

Response:
```json
{
  "code": "OK",
  "message": "success",
  "data": {
    "accessToken": "jwt-token",
    "user": {
      "id": "u-001",
      "username": "admin",
      "role": "platform_admin",
      "language": "zh-CN",
      "uiTheme": "dark"
    }
  }
}
```

## Users

### GET /users?page=1&pageSize=20&role=model_developer

Response:
```json
{
  "code": "OK",
  "data": {
    "items": [
      {
        "id": "u-011",
        "username": "alice",
        "realName": "Alice",
        "role": "model_developer",
        "status": "active",
        "createdAt": "2026-03-01T10:00:00Z"
      }
    ],
    "page": 1,
    "pageSize": 20,
    "total": 1
  }
}
```

## License

### POST /license/activate

Request:
```json
{
  "licenseKey": "ABCD-XXXX-XXXX-1234"
}
```

Response:
```json
{
  "code": "OK",
  "data": {
    "status": "active",
    "licenseKeyMasked": "ABCD********1234",
    "licensee": "宁波建行",
    "activatedAt": "2026-03-17T05:00:00Z",
    "expiresAt": "2027-03-17T05:00:00Z"
  }
}
```

## Data Sources

### POST /data-sources

Request:
```json
{
  "name": "hive-prod",
  "type": "hive",
  "connectionAddress": "thrift://hive-host:10000"
}
```

Response:
```json
{
  "code": "OK",
  "data": {
    "id": "ds-001",
    "name": "hive-prod",
    "type": "hive",
    "connected": true,
    "objectCount": 124
  }
}
```

## Features

### POST /features

Request:
```json
{
  "sourceId": "ds-001",
  "database": "dwd",
  "tableName": "user_balance_m",
  "tableType": "monthly",
  "customerIdField": "customer_id",
  "timeField": "stat_month",
  "featureFields": ["balance", "avg_balance"]
}
```

Response:
```json
{
  "code": "OK",
  "data": {
    "id": "ft-001"
  }
}
```

## Portraits and Targets

### POST /portraits

Request:
```json
{
  "portraitName": "AUM资产客群画像",
  "dataSource": "computed",
  "sourceTables": ["dwd.user_balance_m"]
}
```

### POST /targets

Request:
```json
{
  "targetName": "信用卡激活预测",
  "dataSource": "computed",
  "targetType": "binary",
  "description": "预测未来一个月激活概率"
}
```

## Models and Scoring

### POST /models

Request:
```json
{
  "modelName": "画龙模型A",
  "portraitId": "up-001",
  "targetId": "tgt-002",
  "algorithmId": "hualong-a-202601"
}
```

### POST /models/{id}/train

Request:
```json
{
  "runMode": "sample",
  "featureMonths": "2025-01~2025-12",
  "labelMonth": "2026-01"
}
```

Response:
```json
{
  "code": "OK",
  "data": {
    "jobId": "job-train-001",
    "status": "running"
  }
}
```

## Operations

### POST /operations

Request:
```json
{
  "name": "2026-03 高价值人群名单",
  "modelId": "m-001",
  "scoreRule": "Top 10%",
  "conditions": [
    { "field": "is_vip", "operator": "=", "value": "true" },
    { "field": "aum", "operator": ">=", "value": "500000" }
  ],
  "abTest": {
    "enabled": true,
    "unit": "percent",
    "value": 20
  }
}
```

## SQL

### POST /sql/execute

Request:
```json
{
  "sourceId": "ds-001",
  "sql": "SELECT customer_id, score FROM model_scores LIMIT 20"
}
```

Response:
```json
{
  "code": "OK",
  "data": {
    "columns": ["customer_id", "score"],
    "rows": [
      { "customer_id": "c-001", "score": 0.98 }
    ],
    "rowCount": 1,
    "durationMs": 31,
    "status": "success"
  }
}
```

## Logs

### GET /logs?type=operation&page=1&pageSize=20

Response:
```json
{
  "code": "OK",
  "data": {
    "items": [
      {
        "id": "log-001",
        "timestamp": "2026-03-17T05:00:00Z",
        "type": "operation",
        "user": "admin",
        "action": "create_operation",
        "detail": "created operation op-009",
        "status": "success"
      }
    ],
    "page": 1,
    "pageSize": 20,
    "total": 1
  }
}
```
