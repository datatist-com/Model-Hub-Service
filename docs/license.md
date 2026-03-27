# 许可证管理 API

所有许可证接口均为**公开接口**，无需认证。

---

## GET /api/v1/license

获取当前激活的许可证信息。

### 响应 `200`

存在激活许可证时：

```json
{
  "code": "OK",
  "message": "message.license.info.success",
  "data": {
    "status": "active",
    "projectName": "MyProject",
    "licenseKeyMasked": "ABCDEFGH****WXYZ",
    "expiresAt": "2025-01-15T00:00:00Z",
    "activatedAt": "2024-01-15T08:00:00Z"
  }
}
```

无许可证时：

```json
{
  "code": "OK",
  "message": "message.license.info.success",
  "data": {
    "status": "missing",
    "projectName": "",
    "licenseKeyMasked": "",
    "expiresAt": "",
    "activatedAt": ""
  }
}
```

### `status` 取值

| 值 | 含义 |
|----|------|
| `active` | 许可证有效 |
| `expired` | 许可证已过期 |
| `missing` | 未激活任何许可证 |

---

## POST /api/v1/license/verify

仅解密验证许可证密钥，不写入数据库。

### 请求体

```json
{
  "licenseKey": "<64字符 base64url 编码的许可证密钥>"
}
```

### 响应 `200`

```json
{
  "code": "OK",
  "message": "message.license.verify.success",
  "data": {
    "valid": true,
    "expired": false,
    "projectName": "MyProject",
    "expiresAt": "2025-01-15T00:00:00Z"
  }
}
```

### 错误

| 状态码 | 场景 | message key |
|--------|------|-------------|
| 400 | `licenseKey` 为空 | `error.license.key_required` |
| 400 | 密钥无法解密或版本不支持 | `error.license.*`（具体见 [i18n-keys.md](../i18n-keys.md)） |

---

## POST /api/v1/license/activate

激活许可证。若已有激活许可证，旧许可证状态自动变为 `replaced`。

### 请求体

```json
{
  "licenseKey": "<64字符 base64url 编码的许可证密钥>"
}
```

### 响应 `200`

返回格式与 `GET /api/v1/license` 相同，`status` 为 `active`，`message` 为 `message.license.activate.success`。

### 错误

| 状态码 | 场景 | message key |
|--------|------|-------------|
| 400 | `licenseKey` 为空 | `error.license.key_required` |
| 400 | 密钥无法解密或版本不支持 | `error.license.*`（具体见 [i18n-keys.md](../i18n-keys.md)） |
| 400 | 许可证已过期 | `error.license.expired`（含 `params.expiresAt`） |
