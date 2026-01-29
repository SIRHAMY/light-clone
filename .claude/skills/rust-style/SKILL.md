---
name: rust-style
description: Rust coding style guide and architecture patterns. Use when writing, reviewing, or modifying Rust code (.rs files), creating new Rust projects, or answering questions about Rust conventions in this codebase.
---

# Rust Style Guide

Our approach to Rust: treat it like a high-level language to get most benefits with few downsides.

## Philosophy

Write simple, readable code: types-first, immutable, functional-ish. Get the performance, memory safety, and portability benefits of Rust (80-90%) without fighting the borrow checker.

## Core Principles

### Immutable Data and Pure Functions

These rarely have issues with the borrow checker or lifetimes.

```rust
// Good: pure function, easy to test and reason about
fn calculate_total(items: &[Item]) -> Decimal {
    items.iter().map(|i| i.price).sum()
}

// Good: immutable by default
let user = fetch_user(id)?;
let updated = User { name: new_name, ..user };
save_user(&updated)?;
```

### Clone Liberally

Favor clarity over micro-optimization. Clone often - just be mindful of large objects.

```rust
// Good: clear ownership, no lifetime complexity
fn process_order(order: Order) -> ProcessedOrder {
    // order is owned, can be transformed freely
}

// Avoid: fighting the borrow checker for marginal gains
fn process_order<'a>(order: &'a Order) -> ProcessedOrder<'a> {
    // lifetime annotations everywhere
}
```

### Arc for Shared State

Use `Arc<dyn Trait>` for sharing services. This avoids lifetime annotations and works well with async.

```rust
// Good: Arc provides 'static lifetime, works everywhere
pub struct AppState {
    pub user_service: Arc<dyn UserService>,
    pub order_service: Arc<dyn OrderService>,
}

// Services reference each other via traits
impl OrderServiceImpl {
    pub fn new(user_service: Arc<dyn UserService>) -> Self {
        Self { user_service }
    }
}
```

## Domain Driven Design

### Traits as Interfaces

Each service/repository is defined by a trait. Good for dependency injection and testing.

```rust
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>>;
    async fn save(&self, user: &User) -> Result<()>;
}

// Implementation
pub struct PgUserRepository {
    pool: PgPool,
}

#[async_trait]
impl UserRepository for PgUserRepository {
    // ...
}
```

### Services at Context Root

Services are assembled at the application root and passed down via `Arc<dyn Trait>`.

```rust
// In main.rs or app setup
let user_repo: Arc<dyn UserRepository> = Arc::new(PgUserRepository::new(pool.clone()));
let user_service: Arc<dyn UserService> = Arc::new(UserServiceImpl::new(user_repo));

let state = AppState { user_service };
```

### Functional Domain Objects

Domain logic follows the Impure-Pure-Impure sandwich pattern:

```
Load (impure) -> Transform (pure) -> Save (impure)
```

```rust
// Load
let order = order_repo.find_by_id(order_id).await?;

// Pure transform - easy to test
let updated = order.apply_discount(discount_code)?;

// Save
order_repo.save(&updated).await?;
```

Commands return new versions, not mutations:

```rust
impl Order {
    // Returns new Order, doesn't mutate self
    pub fn apply_discount(self, code: DiscountCode) -> Result<Order> {
        let discount = code.calculate_discount(&self)?;
        Ok(Order {
            total: self.total - discount,
            applied_discounts: self.applied_discounts.with(code),
            ..self
        })
    }
}
```

Each request owns its data - no shared mutable state.

## Testing Strategy

| Layer | Approach |
|-------|----------|
| Pure functions | Unit test thoroughly (data in, data out) |
| Services | Integration test with real DB or mocks at trait boundary |
| External services | Mock only (email, payments, third-party APIs) |

```rust
// Unit test pure logic
#[test]
fn apply_discount_reduces_total() {
    let order = Order::new(items, Decimal::new(100, 0));
    let result = order.apply_discount(ten_percent_code()).unwrap();
    assert_eq!(result.total, Decimal::new(90, 0));
}

// Integration test with mock at trait boundary
#[tokio::test]
async fn create_order_notifies_user() {
    let mock_notifier = Arc::new(MockNotifier::new());
    let service = OrderService::new(mock_notifier.clone());

    service.create_order(order_data).await.unwrap();

    assert!(mock_notifier.was_called_with(expected_notification));
}
```

## What This Avoids

| Pain Point | How We Avoid It |
|------------|-----------------|
| Lifetime annotations | `Arc` everywhere for shared state |
| Generic explosion | `dyn Trait` instead of monomorphization |
| Borrow checker fights | Functional transforms, clone liberally |
| Async + lifetime pain | `Arc` is `'static`, works with async |

## What This Keeps

| Benefit | How |
|---------|-----|
| Type safety | Strong types, discriminated unions |
| Exhaustive matching | Enums for state machines |
| No GC | Predictable performance (2-3x faster than C#/F#, 10x faster than TS) |
| Explicit errors | `Result` types, no exceptions |
| Testable code | Pure functions, trait-based DI |

## Vertical Slice Architecture

Organize code by feature (vertical slice) rather than by technical layer (horizontal). Each slice is a self-contained module with everything needed for that feature.

### Structure

```
src/
├── main.rs                    # Entry point, assembles slices
├── core/                      # Cross-cutting concerns
│   ├── mod.rs
│   ├── config.rs
│   ├── error.rs
│   ├── db.rs
│   └── views/
│       └── layout.rs
├── health/                    # Health check slice
│   ├── mod.rs
│   ├── health_routes.rs
│   └── health_service.rs
├── users/                     # Example feature slice
│   ├── mod.rs
│   ├── users_routes.rs
│   ├── users_service.rs
│   ├── users_data.rs
│   ├── users_types.rs
│   └── users_views.rs
```

### Slice Contents

Each slice may contain (only include what's needed):

| File | Purpose |
|------|---------|
| `*_routes.rs` | HTTP handlers, route definitions |
| `*_service.rs` | Business logic, orchestration |
| `*_data.rs` | Database queries, repository functions |
| `*_types.rs` | Slice-specific types, DTOs |
| `*_views.rs` | Maud HTML templates |

### Benefits

- **Cohesion**: Related code lives together
- **Independence**: Slices can be developed/tested in isolation
- **Discoverability**: Find all user-related code in `src/users/`
- **Minimal coupling**: Slices depend on `core/`, not each other

### Guidelines

- Start simple - a slice might just be `*_routes.rs` initially
- Extract `*_service.rs` when business logic grows beyond route handlers
- Extract `*_data.rs` when queries become complex or reusable
- Shared infrastructure (config, errors, DB pool, layouts) goes in `core/`
