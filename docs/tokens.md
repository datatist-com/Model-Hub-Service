# Token 管理 API

所有接口均需要 `platform_admin` 认证。

---

## GET /api/v1/tokens

查询所有有效 Token（status = active 且未过期）。

### 查询参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `page` | number | 否 | 页码，默认 1 |
| `pageSize` | number | 否 | 每页条数，默认 20 |

固定按 `created_at DESC`（最新优先）排序。

### 响应 `200`

```json
{
  "code": "OK",
  "message": "message.tokens.list.success",
  "data": {
    "items": [
      {
        "id": "uuid",
        "userId": "uuid",
        "username": "zhangsan",
        "maskedToken": "ABCDefgh****WXYZ",
        "ip": "192.168.1.100",
        "device": "macOS Chrome",
        "status": "active",
        "createdAt": "2024-03-26T10:30:00",
        "expiresAt": "2024-03-27T10:30:00"
      }
    ],
    "page": 1,
    "pageSize": 20,
    "total": 3
  }
}
```

> `maskedToken` 仅显示前 8 位 + `****` + 后 4 位，不可用于认证。

### 错误

| 状态码 | 场景 | message key |
|--------|------|-------------|
| 401 | 未认证 | `error.auth.token_invalid` |
| 403 | 非管理员 | `error.auth.admin_required` |

---

## DELETE /api/v1/tokens/{id}

注销指定 Token，使其立即失效。

### 路径参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | string | Token 记录的 UUID |

### 响应 `200`

```json
{
  "code": "OK",
  "message": "message.tokens.revoke.success",
  "data": {
    "success": true
  }
}
```

### 错误

| 状态码 | 场景 | message key |
|--------|------|-------------|
| 400 | Token 已注销 | `error.tokens.already_revoked` |
| 401 | 未认证 | `error.auth.token_invalid` |
| 403 | 非管理员 | `error.auth.admin_required` |
| 404 | Token 不存在 | `error.tokens.not_found` |
