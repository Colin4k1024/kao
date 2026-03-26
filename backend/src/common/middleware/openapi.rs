use axum::{
    extract::Path,
    http::Request,
    response::Response,
    routing::{get, MethodRouter},
    Router,
};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// OpenAPI specification for the Kao API
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Kao Admin Management System API",
        description = "Enterprise admin management system API documentation",
        contact(
            name = "Kao Team",
            url = "https://github.com/kao-admin/kao",
            email = "team@kao-admin.com"
        ),
        version = "1.0.0"
    ),
    servers(
        (url = "http://localhost:8080", description = "Development server"),
        (url = "https://api.kao-admin.com", description = "Production server")
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management"),
        (name = "roles", description = "Role management"),
        (name = "menus", description = "Menu management"),
        (name = "departments", description = "Department management"),
        (name = "dictionary", description = "Dictionary management"),
        (name = "config", description = "Configuration management"),
        (name = "jobs", description = "Scheduled job management"),
        (name = "monitoring", description = "System monitoring"),
    )
)]
pub struct ApiDoc;

/// Middleware for serving OpenAPI/Swagger documentation
pub fn setup_openapi_middleware(app: Router) -> Router {
    app.nest("/api-docs", SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.yaml", ApiDoc::openapi()))
}
