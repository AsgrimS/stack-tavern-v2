use cfg_if::cfg_if;
cfg_if! { if #[cfg(feature = "ssr")] {
    use axum::{
        response::{Response, IntoResponse},
        routing::get,
        extract::{Path, State, RawQuery},
        http::{Request, header::HeaderMap },
        body::Body as AxumBody,
        Router,
        middleware
    };
    use dotenv::dotenv;
    use leptos::*;
    use stack_tavern_v2::app::*;
    use stack_tavern_v2::db::initialize_pool;
    use stack_tavern_v2::fileserv::file_and_error_handler;
    use stack_tavern_v2::state::AppState;
    use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};


    async fn server_fn_handler(State(app_state): State<AppState>, path: Path<String>, headers: HeaderMap, raw_query: RawQuery,
    request: Request<AxumBody>) -> impl IntoResponse {

        logging::log!("{:?}", path);

        handle_server_fns_with_context(path, headers, raw_query, move || {
            provide_context(app_state.pool.clone());
        }, request).await
    }


    async fn leptos_routes_handler(State(app_state): State<AppState>, req: Request<AxumBody>) -> Response{
            let handler = leptos_axum::render_route_with_context(app_state.leptos_options.clone(),
            app_state.routes.clone(),
            move || {
                provide_context(app_state.pool.clone());
            },
            App
        );
        handler(req).await.into_response()
    }

   #[tokio::main]
   async fn main() {
       simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");
       dotenv().ok();

       // Setting get_configuration(None) means we'll be using cargo-leptos's env values
       // For deployment these variables are:
       // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
       // Alternately a file can be specified such as Some("Cargo.toml")
       // The file would need to be included with the executable when moved to deployment
       let conf = get_configuration(None).await.unwrap();
       let leptos_options = conf.leptos_options;
       let addr = leptos_options.site_addr;
       let routes = generate_route_list(App);
       let app_state = AppState {
            leptos_options,
            pool: initialize_pool().await.clone(),
            routes: routes.clone(),
       };

       // build our application with a route
       let app = Router::new()
            .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
            .leptos_routes_with_handler(routes, get(leptos_routes_handler))
            .fallback(file_and_error_handler)
            // .route_layer(middleware::from_fn(require_token))
            .with_state(app_state);

       // run our app with hyper
       // `axum::Server` is a re-export of `hyper::Server`
       log::info!("listening on http://{}", &addr);
       axum::Server::bind(&addr)
           .serve(app.into_make_service())
           .await
           .unwrap();
   }
}}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
