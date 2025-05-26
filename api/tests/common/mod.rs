use axum::Router;
use supercv::interface::routes::create_router;

pub fn test_app() -> Router {
    create_router()
}
