**rGateway** is a high-performance, modular API Gateway written in Rust. Inspired by [Express Gateway](https://www.express-gateway.io/), it provides a declarative, extensible system for handling API requests in distributed environments, with first-class support for routing, authentication, rate limiting, and pluggable middleware.

## Features

- **Config-driven architecture**
    - Define routes, upstream targets, and middleware pipelines via YAML or JSON
    - Centralized, declarative configuration management

- **Dynamic routing**
    - Route matching by path, method, headers, and query parameters
    - Support for path parameters (`/users/:id`) and wildcard routes

- **Middleware pipelines**
    - Authentication (JWT, API Key, OAuth2)
    - Rate limiting
    - Request/response transformation
    - Request validation
    - Logging and tracing

- **Secure by design**
    - JWT validation with HMAC or RSA
    - IP allow/deny lists
    - CORS enforcement

- **Observability**
    - Structured logging (via `tracing`)
    - Planned integration with Prometheus and OpenTelemetry

- **Extensible plugin system**
    - Define custom middleware
    - Future support for dynamically loaded plugins (shared libraries or WASM)

- **Multi-service support**
    - Route requests to multiple upstream services
    - Planned support for simple load balancing and retries

## Configuration Example

```yaml
http:
  port: 8080

routes:
  - path: /api/users
    methods: [GET]
    target: http://user-service:3000
    pipeline: [auth, rate-limit]

  - path: /api/login
    methods: [POST]
    target: http://auth-service:4000
    pipeline: [body-parser]

pipelines:
  auth:
    - jwt:
        secret: "supersecret"
        algorithms: [HS256]

  rate-limit:
    - rate-limit:
        window: 60
        limit: 100
```

## Roadmap

### Core
- [x] Configuration loader
- [x] Basic routing with Axum
- [x] Static middleware pipeline
- [x] Proxy to upstream targets

### In Progress
- [ ] JWT authentication middleware
- [ ] Rate limiting
- [ ] Structured logging with request context
- [ ] Built-in metrics export

### Planned
- [ ] Dynamic plugin system
- [ ] Live config reload
- [ ] Admin API for config inspection
- [ ] Web dashboard (optional)
- [ ] WASM middleware support

## Getting Started

```bash
git clone https://github.com/your-org/rgateway
cd rgateway
cargo run
```

## Requirements

- Rust 1.75 or newer
- Cargo
- (Optional) Docker for deployment/testing

## Philosophy

Minimal, performant, secure by default. Built to handle real traffic at scale with predictable behavior and low overhead.

## License

MIT or Apache 2.0, at your choice.
