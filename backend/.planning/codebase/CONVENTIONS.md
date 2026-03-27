# Coding Conventions

**Analysis Date:** 2026-03-27

## Naming Patterns

**Files:**
- Module files: `mod.rs`
- Models: `model.rs` (singular: `{name}/model.rs`)
- Repositories: `repo.rs` (e.g., `src/features/users/repo.rs`)
- Services: `service.rs`
- Routes: `routes.rs`
- Handlers: `handlers.rs` (API layer)

**Functions:**
- Public: `camelCase` (e.g., `list_users`, `create_user`)
- Private: `snake_case` (e.g., `compute_hash`, `validate_input`)
- Async: `snake_case` with `async` keyword (typically just underscore)

**Variables:**
- Public: `snake_case` (e.g., `user_id`, `page_size`)
- Private: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `MAX_PAGE_SIZE`)

**Types:**
- Structs: `PascalCase` (e.g., `UserService`, `AppState`)
- Enums: `PascalCase` (e.g., `AppError`)
- Traits: `PascalCase` with `Trait` suffix (e.g., `ResponseExt`)

## Code Style

**Formatting:**
- Tool: rustfmt (standard)
- Indentation: 4 spaces

**Linting:**
- Tool: Clippy
- Warnings treated as errors in CI

## Import Organization

**Order:**
1. std library imports
2. Third-party imports (axum, sqlx, serde, etc.)
3. crate imports (src modules)
4. super imports (parent module)

**Path Aliases:**
- `crate::common::` for shared utilities
- `crate::features::` for feature modules
- `super::` for module-relative imports

## Error Handling

**Patterns:**
- Return `Result<T, AppError>` for all public async functions
- Use `?` operator for propagation
- Convert errors via `From` traits in `src/common/error.rs`

**Patterns:**
```rust
pub async fn get_user(id: Uuid) -> Result<User, AppError> {
    let user = repo::find_user(db, id).await?;
    Ok(user)
}
```

## Logging

**Framework:** tracing crate

**Patterns:**
- Use `log::error!`, `log::warn!`, `log::info!`, `log::debug!`
- Log structure: `{message} {field=value}`
- Example: `log::error!("Failed to get online users: {}", e)`

## Comments

**When to Comment:**
- Complex business logic
- Non-obvious decisions
- Performance considerations
- Edge cases

**JSDoc/TSDoc:**
- Not applicable for Rust

## Function Design

**Size:** Prefer shorter functions < 50 lines

**Parameters:** Max 5 parameters, use structs for groups

**Return Values:** Always use `Result<T, AppError>` for fallible operations

## Module Design

**Exports:**
- Use `pub use` for re-exports
- Keep internal structure private
- Create mod.rs for module entry points

**Barrel Files:**
- `src/common/mod.rs`: Exports common utilities
- `src/features/mod.rs`: Exports all features
- `src/models/mod.rs`: Exports domain models

---

*Convention analysis: 2026-03-27*
