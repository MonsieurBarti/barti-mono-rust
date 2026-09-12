use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

#[derive(OpenApi)]
#[openapi(
    info(title = "Barti Freight", version = "0.1.0"),
    servers((url = "/"))
)]
pub(crate) struct AppApi;

pub(crate) fn merge(loads: OpenApiRouter) -> OpenApiRouter {
    OpenApiRouter::with_openapi(AppApi::openapi()).merge(loads)
}

#[cfg(test)]
mod tests {
    use super::merge;
    use utoipa_axum::router::OpenApiRouter;

    #[test]
    fn merged_spec_has_loads_slice() {
        let json = merge(OpenApiRouter::with_openapi(loads::openapi()))
            .into_openapi()
            .to_pretty_json()
            .unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["info"]["title"], "Barti Freight");
        assert_eq!(v["info"]["version"], "0.1.0");
        assert_eq!(v["servers"][0]["url"], "/");
        assert!(v["paths"].get("/health").is_none());
        let post = &v["paths"]["/loads"]["post"];
        assert_eq!(post["operationId"], "create_load");
        assert_eq!(post["tags"][0], "loads");
        assert!(post["responses"].get("201").is_some());
        assert!(post["responses"].get("400").is_some());
        assert!(post["responses"].get("409").is_some());
    }
}
