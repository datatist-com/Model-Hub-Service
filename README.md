# Model Hub Backend

基于 Rust + Actix-Web + SQLite 的模型管理平台后端服务。

## 功能

- **认证**：JWT 登录/登出/获取当前用户
- **用户管理**：CRUD（仅管理员）、角色与状态管理
- **个人设置**：修改密码

## 技术栈

- Rust + Actix-Web 4
- SQLite（sqlx，自动建库）
- JWT (jsonwebtoken)
- Argon2 密码哈希

## 快速开始

### 前置条件

- Rust 1.75+（`rustup` 安装）

### 编译运行

```bash
# 编译
cargo build --release

# 运行（默认 0.0.0.0:8080）
cargo run --release

# 自定义端口
cargo run --release -- --host 127.0.0.1 --port 3000
```

启动后会在当前目录自动创建 `model_hub.db` SQLite 数据库文件（如不存在）。

JWT Secret 每次启动自动生成。

### 默认管理员

首次启动时自动创建：

- 用户名：`admin`
- 密码：`123456`
- 真实姓名：`默认管理员`
- 角色：`platform_admin`

> 请在生产环境中立即修改默认密码。

## API 端点

所有接口前缀：`/api/v1`

### 认证

| 方法 | 路径 | 鉴权 | 说明 |
|------|------|------|------|
| POST | `/auth/login` | 无 | 登录 |
| POST | `/auth/logout` | Bearer | 登出 |
| GET | `/auth/current-user` | Bearer | 当前用户信息 |

### 用户管理（仅管理员）

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/users` | 分页列表（支持 role/status/keyword 过滤） |
| POST | `/users` | 创建用户 |
| PUT | `/users/{id}` | 更新用户 |
| DELETE | `/users/{id}` | 删除用户 |

### 个人设置

| 方法 | 路径 | 说明 |
|------|------|------|
| PUT | `/profile/password` | 修改密码 |

## 统一响应格式

```json
{
  "code": "OK",
  "message": "success",
  "data": { ... }
}
```

## 项目结构

```
src/
├── main.rs            # 启动入口
├── config.rs          # CLI 参数解析
├── db.rs              # SQLite 连接池 + migration
├── errors.rs          # 错误处理 + 统一响应
├── routes.rs          # 路由注册
├── middleware/
│   └── auth.rs        # JWT 鉴权提取器
├── models/
│   └── user.rs        # 用户模型 + DB 查询
└── handlers/
    ├── auth.rs        # 认证处理器
    ├── users.rs       # 用户 CRUD
    └── profile.rs     # 个人设置
```

## License

MIT
