# 个人设置 API

所有接口需要**已认证用户**（任意角色）。

---

## PUT /api/v1/profile/password

修改当前登录用户的密码。

### 请求体

```json
{
  "currentPassword": "old123456",
  "newPassword": "newSecure789"
}
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `currentPassword` | string | 是 | 当前密码 |
| `newPassword` | string | 是 | 新密码（最少 6 位） |

### 响应 `200`

```json
{
  "code": "OK",
  "message": "success",
  "data": {
    "success": true
  }
}
```

### 错误

| 状态码 | 场景 |
|--------|------|
| 400 | 新密码少于 6 位 |
| 401 | 未认证 / 当前密码错误 |
| 404 | 用户不存在 |
