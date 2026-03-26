# Model Hub Backend — API 文档

## 概述

- **框架**: Rust / Actix-Web 4
- **数据库**: SQLite (sqlx 0.8)
- **认证方式**: 数据库持久化 Token（32 位随机字母数字字符串）
- **基础路径**: `/api/v1`

## 通用响应格式

所有接口返回统一信封：

```json
{
  "code": "OK",
  "message": "success",
  "data": { ... }
}
```

错误响应：

```json
{
  "code": "BAD_REQUEST",
  "message": "具体错误信息"
}
```

### 错误码

| HTTP 状态码 | code | 含义 |
|-------------|------|------|
| 400 | `BAD_REQUEST` | 请求参数错误 |
| 401 | `UNAUTHORIZED` | 未认证 / Token 无效或过期 |
| 403 | `FORBIDDEN` | 权限不足（账号冻结 / 非管理员） |
| 404 | `NOT_FOUND` | 资源不存在 |
| 409 | `CONFLICT` | 资源冲突（如用户名重复） |
| 500 | `INTERNAL_ERROR` | 服务端内部错误 |

## 认证机制

需要认证的接口通过以下方式传递 Token（按优先级）：

1. `Authorization` 请求头：`Bearer <token>` 或直接传 `<token>`
2. `X-Token` 请求头
3. URL 查询参数：`?token=<token>`

Token 有效期 24 小时，每次调用 `GET /api/v1/auth/token` 会自动续期 24 小时。

## 默认管理员

首次启动时自动创建种子管理员：
- 用户名：`admin`
- 密码：`123456`
- 角色：`platform_admin`

## API 分类

| 分类 | 文档 | 说明 |
|------|------|------|
| 许可证管理 | [license.md](license.md) | 许可证验证、激活、查询（公开） |
| 认证 | [auth.md](auth.md) | 登录、登出、Token 续期 |
| 用户管理 | [users.md](users.md) | 用户 CRUD（管理员） |
| 个人设置 | [profile.md](profile.md) | 修改密码（已认证用户） |
| 日志 | [logs.md](logs.md) | 登录日志、操作日志 |

## 分页参数

支持分页的列表接口均接受以下查询参数：

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `page` | number | 1 | 页码，最小 1 |
| `pageSize` | number | 20 | 每页条数，范围 1–200 |
| `sortBy` | string | `createdAt` | 排序字段（支持 camelCase 和 snake_case） |
| `sortOrder` | string | `desc` | 排序方向：`asc` / `desc` |

分页响应格式：

```json
{
  "code": "OK",
  "message": "success",
  "data": {
    "items": [...],
    "page": 1,
    "pageSize": 20,
    "total": 100
  }
}
```

## CORS

服务端允许所有来源跨域访问，支持方法：GET / POST / PUT / DELETE / OPTIONS。
允许的请求头：`Authorization`、`Content-Type`、`Accept`、`X-Token`。

## 启动参数

```bash
model-hub-backend [--host 0.0.0.0] [--port 8080]
```

| 参数 | 环境变量 | 默认值 | 说明 |
|------|----------|--------|------|
| `--host` | — | `0.0.0.0` | 监听地址 |
| `--port` | — | `8080` | 监听端口 |
| — | `DATABASE_URL` | `sqlite:model_hub.db` | SQLite 数据库路径 |
