# 用户管理 API

所有用户管理接口均需要 **`platform_admin`** 角色认证。

---

## GET /api/v1/users

获取用户列表（分页）。

### 查询参数

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `page` | number | 否 | 页码，默认 1 |
| `pageSize` | number | 否 | 每页条数，默认 20，范围 1–200 |
| `sortBy` | string | 否 | 排序字段，默认 `createdAt` |
| `sortOrder` | string | 否 | 排序方向 `asc`/`desc`，默认 `desc` |
| `role` | string | 否 | 按角色筛选 |
| `status` | string | 否 | 按状态筛选 |
| `keyword` | string | 否 | 模糊搜索用户名或真实姓名 |

**`sortBy` 支持的字段**：`createdAt`、`updatedAt`、`username`、`role`、`status`、`realName`（同时支持 snake_case 写法）。

### 响应 `200`

```json
{
  "code": "OK",
  "message": "success",
  "data": {
    "items": [
      {
        "id": "uuid-string",
        "username": "zhangsan",
        "realName": "张三",
        "role": "model_developer",
        "status": "active",
        "language": null,
        "uiTheme": null,
        "createdAt": "2024-01-15T08:00:00Z"
      }
    ],
    "page": 1,
    "pageSize": 20,
    "total": 1
  }
}
```

### 错误

| 状态码 | 场景 |
|--------|------|
| 401 | 未认证 |
| 403 | 非管理员 |

---

## POST /api/v1/users

创建用户。

### 请求体

```json
{
  "username": "zhangsan",
  "password": "secure123",
  "realName": "张三",
  "role": "model_developer"
}
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `username` | string | 是 | 用户名（唯一） |
| `password` | string | 是 | 密码（Argon2 哈希存储） |
| `realName` | string | 否 | 真实姓名 |
| `role` | string | 是 | 角色 |

### 角色取值

| 值 | 含义 |
|----|------|
| `model_developer` | 模型开发者 |
| `model_operator` | 模型运营者 |
| `platform_admin` | 平台管理员 |

### 响应 `200`

返回创建的用户对象（格式同列表中的 `items` 元素）。

### 错误

| 状态码 | 场景 |
|--------|------|
| 400 | 用户名或密码为空 / 角色无效 |
| 401 | 未认证 |
| 403 | 非管理员 |
| 409 | 用户名已存在 |

---

## PUT /api/v1/users/{id}

更新用户信息。

### 路径参数

| 参数 | 说明 |
|------|------|
| `id` | 用户 ID |

### 请求体

所有字段均为可选，只传需要修改的字段：

```json
{
  "realName": "张三丰",
  "role": "platform_admin",
  "status": "frozen"
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `realName` | string | 真实姓名 |
| `role` | string | 角色（`model_developer` / `model_operator` / `platform_admin`） |
| `status` | string | 状态（`active` / `frozen`） |

### 响应 `200`

返回更新后的用户对象。

### 错误

| 状态码 | 场景 |
|--------|------|
| 400 | 角色或状态值无效 |
| 401 | 未认证 |
| 403 | 非管理员 |
| 404 | 用户不存在 |

---

## DELETE /api/v1/users/{id}

删除用户。

### 路径参数

| 参数 | 说明 |
|------|------|
| `id` | 用户 ID |

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
| 400 | 不允许删除自己 |
| 401 | 未认证 |
| 403 | 非管理员 |
| 404 | 用户不存在 |
