# 日志 API

---

## 登录日志

### GET /api/v1/logs/login/mine

查询当前登录用户自己的登录历史。**需要认证（任意角色）。**

#### 查询参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `page` | number | 否 | 页码，默认 1 |
| `pageSize` | number | 否 | 每页条数，默认 20 |

固定按 `created_at DESC`（最新优先）排序，无筛选参数。

#### 响应 `200`

```json
{
  "code": "OK",
  "message": "success",
  "data": {
    "items": [
      {
        "id": "uuid",
        "userId": "uuid",
        "username": "zhangsan",
        "ip": "192.168.1.100",
        "device": "macOS Chrome",
        "result": "success",
        "createdAt": "2024-03-26T10:30:00"
      }
    ],
    "page": 1,
    "pageSize": 20,
    "total": 5
  }
}
```

#### `result` 取值

| 值 | 含义 |
|----|------|
| `success` | 登录成功 |
| `failed` | 登录失败（密码错误 / 账号冻结等） |

---

### GET /api/v1/logs/login

查询所有用户的登录历史。**需要 `platform_admin` 认证。**

#### 查询参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `page` | number | 否 | 页码，默认 1 |
| `pageSize` | number | 否 | 每页条数，默认 20 |

固定按 `created_at DESC`（最新优先）排序，无筛选参数。

#### 响应 `200`

格式同 `/logs/login/mine`。

#### 错误

| 状态码 | 场景 |
|--------|------|
| 401 | 未认证 |
| 403 | 非管理员 |

---

## 操作日志

> 登录操作不记录在操作日志中（仅记录在登录日志中）。

### GET /api/v1/logs/operations/mine

查询当前登录用户自己的操作历史。**需要认证（任意角色）。**

#### 查询参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `page` | number | 否 | 页码，默认 1 |
| `pageSize` | number | 否 | 每页条数，默认 20 |

固定按 `created_at DESC`（最新优先）排序，无筛选参数。

#### 响应 `200`

```json
{
  "code": "OK",
  "message": "success",
  "data": {
    "items": [
      {
        "id": "uuid",
        "userId": "uuid",
        "username": "admin",
        "module": "users",
        "action": "create_user",
        "targetId": "target-user-uuid",
        "detail": "{\"i18n_key\":\"operation.users.create_user\",\"params\":{\"username\":\"zhangsan\"}}",
        "ip": "192.168.1.100",
        "createdAt": "2024-03-26T10:30:00"
      }
    ],
    "page": 1,
    "pageSize": 20,
    "total": 12
  }
}
```

#### `detail` 字段

JSON 字符串，格式为 `{"i18n_key": "...", "params": {...}}`，供前端国际化渲染使用。  
详见 [i18n-keys.md](i18n-keys.md)。

#### `module` 取值

| 值 | 含义 |
|----|------|
| `auth` | 认证相关（仅 logout） |
| `users` | 用户管理 |
| `profile` | 个人设置 |

#### `action` 取值

| 值 | 含义 |
|----|------|
| `logout` | 登出 |
| `create_user` | 创建用户 |
| `update_user` | 更新用户 |
| `delete_user` | 删除用户 |
| `change_password` | 修改密码 |

---

### GET /api/v1/logs/operations

查询所有用户的操作历史。**需要 `platform_admin` 认证。**

#### 查询参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `page` | number | 否 | 页码，默认 1 |
| `pageSize` | number | 否 | 每页条数，默认 20 |

固定按 `created_at DESC`（最新优先）排序，无筛选参数。

#### 响应 `200`

格式同 `/logs/operations/mine`。

#### 错误

| 状态码 | 场景 |
|--------|------|
| 401 | 未认证 |
| 403 | 非管理员 |
