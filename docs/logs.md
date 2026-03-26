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
| `sortBy` | string | 否 | 排序字段，默认 `createdAt` |
| `sortOrder` | string | 否 | `asc`/`desc`，默认 `desc` |
| `result` | string | 否 | 按结果筛选：`success` / `failed` |

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
        "detail": null,
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
| `sortBy` | string | 否 | 排序字段，默认 `createdAt` |
| `sortOrder` | string | 否 | `asc`/`desc`，默认 `desc` |
| `userId` | string | 否 | 按用户 ID 筛选 |
| `result` | string | 否 | 按结果筛选：`success` / `failed` |

#### 响应 `200`

格式同 `/logs/login/mine`。

#### 错误

| 状态码 | 场景 |
|--------|------|
| 401 | 未认证 |
| 403 | 非管理员 |

---

## 操作日志

### GET /api/v1/logs/operations/mine

查询当前登录用户自己的操作历史。**需要认证（任意角色）。**

#### 查询参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `page` | number | 否 | 页码，默认 1 |
| `pageSize` | number | 否 | 每页条数，默认 20 |
| `sortBy` | string | 否 | 排序字段，默认 `createdAt` |
| `sortOrder` | string | 否 | `asc`/`desc`，默认 `desc` |
| `module` | string | 否 | 按模块筛选 |
| `action` | string | 否 | 按操作筛选 |

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
        "detail": "Created user zhangsan",
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

#### `module` 取值

| 值 | 含义 |
|----|------|
| `auth` | 认证相关 |
| `users` | 用户管理 |
| `profile` | 个人设置 |

#### `action` 取值

| 值 | 含义 |
|----|------|
| `login` | 登录 |
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
| `sortBy` | string | 否 | 排序字段，默认 `createdAt` |
| `sortOrder` | string | 否 | `asc`/`desc`，默认 `desc` |
| `userId` | string | 否 | 按用户 ID 筛选 |
| `module` | string | 否 | 按模块筛选 |
| `action` | string | 否 | 按操作筛选 |

#### 响应 `200`

格式同 `/logs/operations/mine`。

#### 错误

| 状态码 | 场景 |
|--------|------|
| 401 | 未认证 |
| 403 | 非管理员 |
