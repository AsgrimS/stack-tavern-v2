#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::middleware;
    use axum::{routing::post, Router};
    use dotenv::dotenv;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use stack_tavern_v2::api::auth::require_token;
    use stack_tavern_v2::app::*;
    use stack_tavern_v2::fileserv::file_and_error_handler;

    dotenv().ok();
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .route_layer(middleware::from_fn(require_token))
        .route("/api/public/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    log::info!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
