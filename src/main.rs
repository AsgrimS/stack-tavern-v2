use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
   use axum::{
       body::Body as AxumBody,
       extract::State,
       http::Request,
       response::{IntoResponse, Response},
   };
   use axum::{headers::Cookie, TypedHeader};
   use leptos_axum::{generate_route_list, LeptosRoutes};

   use axum::middleware;
   use axum::{routing::post, Router};
   use dotenv::dotenv;
   use leptos::*;
   use stack_tavern_v2::api::auth::require_token;
   use stack_tavern_v2::api::auth::verify_token;
   use stack_tavern_v2::app::*;
   use stack_tavern_v2::fileserv::file_and_error_handler;
   use stack_tavern_v2::shared::dto::user::UserInfoDto;

   async fn leptos_handler_with_context(
       State(options): State<LeptosOptions>,
       TypedHeader(cookie): TypedHeader<Cookie>,
       req: Request<AxumBody>,
   ) -> Response {
       let mut user_info_dto: Option<UserInfoDto> = None;

       let access_token = cookie.get("access_token");

       if let Some(access_token) = access_token {
           user_info_dto = verify_token(access_token).await;
       };

       let handler = leptos_axum::render_app_to_stream_with_context(
           options,
           move || {
               provide_context(user_info_dto.clone());
           },
           || view! { <App/> },
       );
       handler(req).await.into_response()
   }

   #[tokio::main]
   async fn main() {
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
           .leptos_routes_with_handler(routes, leptos_handler_with_context)
           .fallback(file_and_error_handler)
           .with_state(leptos_options);

       log::info!("listening on http://{}", &addr);
       axum::Server::bind(&addr)
           .serve(app.into_make_service())
           .await
           .unwrap();
   }
}}

#[cfg(feature = "ssr")]
#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
