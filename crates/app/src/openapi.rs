use utoipa::OpenApi;
use utoipa::openapi::path::PathItem;
use utoipa::openapi::{Content, OpenApi as OpenApiDoc, Ref, RefOr, Response, ResponseBuilder};
use utoipa_axum::router::OpenApiRouter;

#[derive(OpenApi)]
#[openapi(
    info(title = "Barti Freight", version = "0.1.0"),
    servers((url = "/"))
)]
pub(crate) struct AppApi;

pub(crate) fn merge(loads: OpenApiRouter) -> OpenApiRouter {
    let mut router = OpenApiRouter::with_openapi(AppApi::openapi()).merge(loads);
    inject_401_500(router.get_openapi_mut());
    router
}

fn problem_response(description: &'static str) -> RefOr<Response> {
    ResponseBuilder::new()
        .description(description)
        .content(
            "application/problem+json",
            Content::new(Some(Ref::from_schema_name("Problem"))),
        )
        .into()
}

fn operations_mut(
    item: &mut PathItem,
) -> impl Iterator<Item = &mut utoipa::openapi::path::Operation> {
    [
        item.get.as_mut(),
        item.put.as_mut(),
        item.post.as_mut(),
        item.delete.as_mut(),
        item.options.as_mut(),
        item.head.as_mut(),
        item.patch.as_mut(),
        item.trace.as_mut(),
    ]
    .into_iter()
    .flatten()
}

fn inject_401_500(openapi: &mut OpenApiDoc) {
    let unauthenticated = problem_response("UNAUTHENTICATED");
    let internal = problem_response("Internal server error");
    for item in openapi.paths.paths.values_mut() {
        for operation in operations_mut(item) {
            operation
                .responses
                .responses
                .entry("401".into())
                .or_insert_with(|| unauthenticated.clone());
            operation
                .responses
                .responses
                .entry("500".into())
                .or_insert_with(|| internal.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{inject_401_500, merge};
    use utoipa::openapi::path::{HttpMethod, Operation, PathItem};
    use utoipa::openapi::{Info, OpenApi, Paths, ResponseBuilder};
    use utoipa_axum::router::OpenApiRouter;

    const HTTP_METHODS: &[&str] = &[
        "get", "put", "post", "delete", "options", "head", "patch", "trace",
    ];

    fn cell_openapis() -> [(&'static str, OpenApi); 1] {
        [("loads", loads::openapi())]
    }

    fn spec_json(spec: OpenApi) -> serde_json::Value {
        serde_json::from_str(&spec.to_pretty_json().unwrap()).unwrap()
    }

    fn operations(spec: &serde_json::Value) -> Vec<(String, &'static str, serde_json::Value)> {
        let Some(paths) = spec["paths"].as_object() else {
            return Vec::new();
        };
        let mut ops = Vec::new();
        for (path, item) in paths {
            let Some(item) = item.as_object() else {
                continue;
            };
            for method in HTTP_METHODS {
                if let Some(op) = item.get(*method).filter(|op| op.is_object()) {
                    ops.push((path.clone(), *method, op.clone()));
                }
            }
        }
        ops
    }

    fn operation_tags(spec: &serde_json::Value) -> std::collections::BTreeSet<String> {
        let mut tags = std::collections::BTreeSet::new();
        if let Some(doc_tags) = spec["tags"].as_array() {
            for tag in doc_tags {
                if let Some(name) = tag["name"].as_str() {
                    tags.insert(name.to_owned());
                }
            }
        }
        for (_, _, op) in operations(spec) {
            if let Some(arr) = op["tags"].as_array() {
                for tag in arr {
                    if let Some(name) = tag.as_str() {
                        tags.insert(name.to_owned());
                    }
                }
            }
        }
        tags
    }

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
        let problem = serde_json::json!({
            "content": {
                "application/problem+json": {
                    "schema": { "$ref": "#/components/schemas/Problem" }
                }
            }
        });
        let mut unauthenticated = problem.clone();
        unauthenticated["description"] = "UNAUTHENTICATED".into();
        let mut internal = problem;
        internal["description"] = "Internal server error".into();
        assert_eq!(post["responses"]["401"], unauthenticated);
        assert_eq!(post["responses"]["500"], internal);
        assert!(v["components"]["schemas"].get("Problem").is_some());
        assert!(v["components"].get("responses").is_none());
    }

    #[test]
    fn inject_keeps_cell_401() {
        let mut operation = Operation::new();
        operation.responses.responses.insert(
            "401".into(),
            ResponseBuilder::new().description("cell-401").into(),
        );
        let mut paths = Paths::new();
        paths
            .paths
            .insert("/x".into(), PathItem::new(HttpMethod::Post, operation));
        let mut openapi = OpenApi::new(Info::new("t", "0"), paths);
        inject_401_500(&mut openapi);
        let post = openapi.paths.paths["/x"].post.as_ref().unwrap();
        let utoipa::openapi::RefOr::T(resp) = &post.responses.responses["401"] else {
            panic!("expected inline 401");
        };
        assert_eq!(resp.description, "cell-401");
        assert!(post.responses.responses.contains_key("500"));
    }

    #[test]
    fn coverage_tags_cell_operations_and_omits_health() {
        let merged = spec_json(merge(OpenApiRouter::with_openapi(loads::openapi())).into_openapi());
        let merged_tags = operation_tags(&merged);

        for (cell, spec) in cell_openapis() {
            let spec = spec_json(spec);
            let ops = operations(&spec);
            if ops.is_empty() {
                assert!(
                    !merged_tags.contains(cell),
                    "{cell} has no public routes so it must not contribute a tag"
                );
                continue;
            }
            for (path, method, op) in ops {
                let tagged = op["tags"]
                    .as_array()
                    .is_some_and(|tags| tags.iter().any(|tag| tag.as_str() == Some(cell)));
                assert!(tagged, "{method} {path} is not tagged {cell}");
                assert!(
                    merged["paths"][&path][method].is_object(),
                    "merged spec missing {method} {path} from {cell}"
                );
            }
        }

        let empty = spec_json(merge(OpenApiRouter::new()).into_openapi());
        assert!(
            operation_tags(&empty).is_empty(),
            "a cell with no public routes contributes no tag"
        );
        assert!(merged["paths"].get("/health").is_none());
    }

    #[test]
    fn problem_schema_is_identical_across_cells_before_merge() {
        let problems: Vec<serde_json::Value> = cell_openapis()
            .into_iter()
            .filter_map(|(_, spec)| {
                spec_json(spec)
                    .pointer("/components/schemas/Problem")
                    .cloned()
            })
            .collect();
        for pair in problems.windows(2) {
            assert_eq!(pair[0], pair[1]);
        }
    }
}
