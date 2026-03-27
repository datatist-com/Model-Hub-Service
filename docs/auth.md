# 认证 API

---

## POST /api/v1/auth/login

用户登录，获取访问 Token。**无需认证。**

### 请求体

```json
{
  "username": "admin",
  "password": "123456"
}
```

### 响应 `200`

```json
{
  "code": "OK",
  "message": "message.auth.login.success",
  "params": { "username": "admin" },
  "data": {
    "accessToken": "aBcDeFgH1234567890AbCdEfGh123456",
    "user": {
      "id": "uuid-string",
      "username": "admin",
      "realName": "管理员",
      "role": "platform_admin",
      "language": null,
      "uiTheme": null
    }
  }
}
```

### 错误

| 状态码 | message key | 场景 |
|--------|-------------|------|
| 400 | `error.auth.credentials_required` | 用户名或密码为空 |
| 401 | `error.auth.invalid_credentials` | 用户名不存在或密码错误 |
| 403 | `error.auth.account_frozen` | 账号已冻结 |

### 说明

- Token 为 32 位随机字母数字字符串，有效期 24 小时。
- 服务端记录客户端 IP 和设备信息（从 User-Agent 解析）。
- IP 解析逻辑：当直连 IP 为私有地址（10/8、172.16/12、192.168/16、loopback）时，信任 `X-Forwarded-For` / `X-Real-IP` 头。

---

## POST /api/v1/auth/logout

登出，吊销当前 Token。**需要认证。**

### 请求头

```
Authorization: Bearer <token>
```

### 响应 `200`

```json
{
  "code": "OK",
  "message": "message.auth.logout.success",
  "data": {
    "success": true
  }
}
```

### 错误

| 状态码 | message key | 场景 |
|--------|-------------|------|
| 401 | `error.auth.token_invalid` | Token 缺失或无效 |

---

## GET /api/v1/auth/token

验证当前 Token 并自动续期 24 小时，返回最新用户信息。**需要认证。**

### 请求头

```
Authorization: Bearer <token>
```

### 响应 `200`

```json
{
  "code": "OK",
  "message": "message.auth.token.success",
  "data": {
    "accessToken": "aBcDeFgH1234567890AbCdEfGh123456",
    "user": {
      "id": "uuid-string",
      "username": "admin",
      "realName": "管理员",
      "role": "platform_admin",
      "language": null,
      "uiTheme": null
    }
  }
}
```

### 说明

- 每次调用此接口，Token 的过期时间会重置为当前时间 + 24 小时。
- 返回的 `accessToken` 是同一个字符串（不会更换），仅更新过期时间。
- 前端可在应用初始化时调用此接口，既验证 Token 有效性，又获取最新用户信息。

### 错误

| 状态码 | message key | 场景 |
|--------|-------------|------|
| 401 | `error.auth.token_invalid` | Token 缺失、无效或已过期 |
| 403 | `error.auth.account_frozen` | 账号已冻结 |
| 404 | `error.users.not_found` | 用户不存在 |
