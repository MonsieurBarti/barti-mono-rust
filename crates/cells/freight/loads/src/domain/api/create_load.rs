use garde::Validate;
use kernel::Violation;
use serde::{Deserialize, Serialize};
use std::future::Future;
use utoipa::ToSchema;

const PORT: &str = "createLoad";

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct CreateLoadInput {
    #[garde(length(min = 1))]
    pub(crate) shipper_id: String,
    #[garde(length(min = 2, max = 2), dive)]
    pub(crate) stops: Vec<StopInput>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct StopInput {
    #[garde(skip)]
    pub(crate) kind: StopKindPl,
    #[garde(custom(valid_date))]
    pub(crate) date: String,
    #[garde(inner(length(min = 1)))]
    pub(crate) name: Option<String>,
    #[garde(dive)]
    pub(crate) address: AddressInput,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) enum StopKindPl {
    Pickup,
    Delivery,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct AddressInput {
    #[garde(length(min = 1))]
    pub(crate) line1: String,
    #[garde(inner(length(min = 1)))]
    pub(crate) line2: Option<String>,
    #[garde(length(min = 1))]
    pub(crate) city: String,
    #[garde(length(min = 1))]
    pub(crate) region: String,
    #[garde(length(min = 1))]
    pub(crate) postal_code: String,
    #[garde(custom(iso_alpha2))]
    pub(crate) country: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LoadResource {
    pub(crate) id: String,
    pub(crate) shipper_id: String,
    pub(crate) stops: Vec<StopResource>,
    pub(crate) created_at: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StopResource {
    pub(crate) id: String,
    pub(crate) kind: StopKindPl,
    pub(crate) date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
    pub(crate) address: AddressResource,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AddressResource {
    pub(crate) line1: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) line2: Option<String>,
    pub(crate) city: String,
    pub(crate) region: String,
    pub(crate) postal_code: String,
    pub(crate) country: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ViolationDto {
    pub(crate) path: String,
    pub(crate) code: String,
    pub(crate) message: String,
}

impl From<Violation> for ViolationDto {
    fn from(violation: Violation) -> Self {
        Self {
            path: violation.path,
            code: violation.code,
            message: violation.message,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", content = "context", rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum CreateLoadError {
    ValidationFailed { violations: Vec<ViolationDto> },
    LoadConflict,
    LoadInvalid,
}

pub(crate) trait CreateLoad {
    fn create_load(
        &self,
        actor_id: String,
        idempotency_key: String,
        input: CreateLoadInput,
    ) -> impl Future<Output = Result<LoadResource, CreateLoadError>> + Send;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
enum StoredOutcome {
    Ok(LoadResource),
    Err(CreateLoadError),
}

pub(crate) fn decode(input: CreateLoadInput) -> Result<CreateLoadInput, CreateLoadError> {
    if let Err(report) = input.validate() {
        return Err(map_garde(report));
    }
    let pickup_count = input
        .stops
        .iter()
        .filter(|stop| stop.kind == StopKindPl::Pickup)
        .count();
    let delivery_count = input
        .stops
        .iter()
        .filter(|stop| stop.kind == StopKindPl::Delivery)
        .count();
    if pickup_count != 1 || delivery_count != 1 {
        return Err(violation(
            "stops",
            "duplicate",
            "must contain one pickup and one delivery",
        ));
    }
    let pickup = input
        .stops
        .iter()
        .find(|stop| stop.kind == StopKindPl::Pickup)
        .expect("counted one pickup");
    let delivery = input
        .stops
        .iter()
        .find(|stop| stop.kind == StopKindPl::Delivery)
        .expect("counted one delivery");
    if delivery.date < pickup.date {
        return Err(violation(
            "stops",
            "order",
            "delivery date must not be before pickup date",
        ));
    }
    match &delivery.name {
        Some(name) if !name.trim().is_empty() => {}
        _ => {
            return Err(violation(
                "stops.name",
                "required",
                "delivery requires a consignee name",
            ));
        }
    }
    Ok(input)
}

pub(crate) fn fingerprint(input: &CreateLoadInput) -> String {
    let json = serde_json::to_string(&FingerprintBody { port: PORT, input })
        .expect("create-load fingerprint is serializable");
    fnv1a64_hex(json.as_bytes())
}

fn fnv1a64_hex(bytes: &[u8]) -> String {
    const OFFSET: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;
    let mut hash = OFFSET;
    for &b in bytes {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(PRIME);
    }
    format!("{hash:016x}")
}

pub(crate) fn encode_outcome(result: &Result<LoadResource, CreateLoadError>) -> String {
    let stored = match result {
        Ok(resource) => StoredOutcome::Ok(resource.clone()),
        Err(error) => StoredOutcome::Err(error.clone()),
    };
    serde_json::to_string(&stored).expect("create-load outcome is serializable")
}

pub(crate) fn decode_outcome(raw: &str) -> Result<Result<LoadResource, CreateLoadError>, ()> {
    match serde_json::from_str::<StoredOutcome>(raw) {
        Ok(StoredOutcome::Ok(resource)) => Ok(Ok(resource)),
        Ok(StoredOutcome::Err(error)) => Ok(Err(error)),
        Err(_) => Err(()),
    }
}

#[derive(Serialize)]
struct FingerprintBody<'a> {
    port: &'static str,
    input: &'a CreateLoadInput,
}

fn map_garde(report: garde::Report) -> CreateLoadError {
    let violations = report
        .iter()
        .map(|(path, error)| {
            let message = error.to_string();
            let code = if message.contains("length") {
                "length"
            } else if message == "must be YYYY-MM-DD" {
                "date"
            } else if message == "must be two uppercase letters" {
                "country"
            } else {
                "invalid"
            };
            ViolationDto::from(Violation {
                path: path.to_string(),
                code: code.to_owned(),
                message,
            })
        })
        .collect();
    CreateLoadError::ValidationFailed { violations }
}

fn violation(path: &str, code: &str, message: &str) -> CreateLoadError {
    CreateLoadError::ValidationFailed {
        violations: vec![ViolationDto::from(Violation {
            path: path.to_owned(),
            code: code.to_owned(),
            message: message.to_owned(),
        })],
    }
}

fn valid_date(value: &str, _: &()) -> garde::Result {
    if is_ymd(value) {
        Ok(())
    } else {
        Err(garde::Error::new("must be YYYY-MM-DD"))
    }
}

fn iso_alpha2(value: &str, _: &()) -> garde::Result {
    if value.len() == 2 && value.bytes().all(|b| b.is_ascii_uppercase()) {
        Ok(())
    } else {
        Err(garde::Error::new("must be two uppercase letters"))
    }
}

fn is_ymd(date: &str) -> bool {
    let bytes = date.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }
    if !bytes
        .iter()
        .enumerate()
        .all(|(i, b)| i == 4 || i == 7 || b.is_ascii_digit())
    {
        return false;
    }
    let Ok(year) = date[0..4].parse::<i32>() else {
        return false;
    };
    let Ok(month) = date[5..7].parse::<u8>() else {
        return false;
    };
    let Ok(day) = date[8..10].parse::<u8>() else {
        return false;
    };
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap(year) => 29,
        2 => 28,
        _ => return false,
    };
    day >= 1 && day <= max_day
}

fn is_leap(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::{
        AddressResource, CreateLoadError, CreateLoadInput, LoadResource, StopKindPl, StopResource,
        decode, fingerprint,
    };

    fn valid_json() -> serde_json::Value {
        serde_json::json!({
            "shipperId": "shipper-1",
            "stops": [
                {
                    "kind": "pickup",
                    "date": "2026-09-20",
                    "address": {
                        "line1": "1 Dock",
                        "city": "Dallas",
                        "region": "TX",
                        "postalCode": "75201",
                        "country": "US"
                    }
                },
                {
                    "kind": "delivery",
                    "date": "2026-09-21",
                    "name": "Consignee",
                    "address": {
                        "line1": "9 Warehouse",
                        "city": "Austin",
                        "region": "TX",
                        "postalCode": "78701",
                        "country": "US"
                    }
                }
            ]
        })
    }

    fn input_from(value: serde_json::Value) -> CreateLoadInput {
        serde_json::from_value(value).expect("fixture json")
    }

    fn decode_err(value: serde_json::Value) -> CreateLoadError {
        decode(input_from(value)).expect_err("expected codec rejection")
    }

    #[test]
    fn decode_accepts_the_first_post_body() {
        let decoded = decode(input_from(valid_json())).expect("valid body");
        assert_eq!(decoded.shipper_id, "shipper-1");
        assert_eq!(decoded.stops.len(), 2);
        assert_eq!(decoded.stops[0].kind, StopKindPl::Pickup);
        assert_eq!(decoded.stops[1].kind, StopKindPl::Delivery);
        assert_eq!(decoded.stops[1].name.as_deref(), Some("Consignee"));
    }

    #[test]
    fn decode_rejects_empty_shipper_id() {
        let mut body = valid_json();
        body["shipperId"] = serde_json::json!("");
        assert!(matches!(
            decode_err(body),
            CreateLoadError::ValidationFailed { .. }
        ));
    }

    #[test]
    fn decode_rejects_one_stop() {
        let mut body = valid_json();
        body["stops"].as_array_mut().unwrap().pop();
        assert!(matches!(
            decode_err(body),
            CreateLoadError::ValidationFailed { .. }
        ));
    }

    #[test]
    fn decode_rejects_two_pickups() {
        let mut body = valid_json();
        body["stops"][1]["kind"] = serde_json::json!("pickup");
        body["stops"][1]["name"] = serde_json::Value::Null;
        match decode_err(body) {
            CreateLoadError::ValidationFailed { violations } => {
                assert_eq!(violations[0].code, "duplicate");
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn decode_rejects_delivery_before_pickup() {
        let mut body = valid_json();
        body["stops"][1]["date"] = serde_json::json!("2026-09-19");
        match decode_err(body) {
            CreateLoadError::ValidationFailed { violations } => {
                assert_eq!(violations[0].code, "order");
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn decode_rejects_missing_delivery_name() {
        let mut body = valid_json();
        body["stops"][1].as_object_mut().unwrap().remove("name");
        match decode_err(body) {
            CreateLoadError::ValidationFailed { violations } => {
                assert_eq!(violations[0].code, "required");
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn decode_rejects_unknown_field() {
        let mut body = valid_json();
        body["equipment"] = serde_json::json!("van");
        assert!(serde_json::from_value::<CreateLoadInput>(body).is_err());
    }

    #[test]
    fn decode_rejects_bad_country() {
        let mut body = valid_json();
        body["stops"][0]["address"]["country"] = serde_json::json!("usa");
        assert!(matches!(
            decode_err(body),
            CreateLoadError::ValidationFailed { .. }
        ));
    }

    #[test]
    fn decode_rejects_invalid_calendar_date() {
        let mut body = valid_json();
        body["stops"][0]["date"] = serde_json::json!("2026-02-31");
        assert!(matches!(
            decode_err(body),
            CreateLoadError::ValidationFailed { .. }
        ));
    }

    #[test]
    fn decode_rejects_signed_month() {
        let mut body = valid_json();
        body["stops"][0]["date"] = serde_json::json!("2026-+1-05");
        assert!(matches!(
            decode_err(body),
            CreateLoadError::ValidationFailed { .. }
        ));
    }

    #[test]
    fn decode_rejects_signed_year() {
        let mut body = valid_json();
        body["stops"][0]["date"] = serde_json::json!("-100-01-01");
        assert!(matches!(
            decode_err(body),
            CreateLoadError::ValidationFailed { .. }
        ));
    }

    #[test]
    fn output_omits_empty_optionals() {
        let json = serde_json::to_value(LoadResource {
            id: "01900000-0000-7000-8000-000000000001".to_owned(),
            shipper_id: "shipper-1".to_owned(),
            created_at: "2023-11-14T22:13:20.000Z".to_owned(),
            stops: vec![
                StopResource {
                    id: "01900000-0000-7000-8000-000000000002".to_owned(),
                    kind: StopKindPl::Pickup,
                    date: "2026-09-20".to_owned(),
                    name: None,
                    address: AddressResource {
                        line1: "1 Dock".to_owned(),
                        line2: None,
                        city: "Dallas".to_owned(),
                        region: "TX".to_owned(),
                        postal_code: "75201".to_owned(),
                        country: "US".to_owned(),
                    },
                },
                StopResource {
                    id: "01900000-0000-7000-8000-000000000003".to_owned(),
                    kind: StopKindPl::Delivery,
                    date: "2026-09-21".to_owned(),
                    name: Some("Consignee".to_owned()),
                    address: AddressResource {
                        line1: "9 Warehouse".to_owned(),
                        line2: None,
                        city: "Austin".to_owned(),
                        region: "TX".to_owned(),
                        postal_code: "78701".to_owned(),
                        country: "US".to_owned(),
                    },
                },
            ],
        })
        .unwrap();
        assert!(json["stops"][0].get("name").is_none());
        assert!(json["stops"][0]["address"].get("line2").is_none());
        assert_eq!(json["stops"][1]["name"], "Consignee");
        assert!(json.get("actorId").is_none());
        assert_eq!(json["createdAt"], "2023-11-14T22:13:20.000Z");
    }

    #[test]
    fn fingerprint_is_stable_for_the_same_decoded_body() {
        let input = decode(input_from(valid_json())).unwrap();
        let hash = fingerprint(&input);
        assert_eq!(hash, fingerprint(&input));
        assert_eq!(hash.len(), 16);
        assert!(hash.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f')));
        let mut other = valid_json();
        other["shipperId"] = serde_json::json!("shipper-2");
        let other = decode(input_from(other)).unwrap();
        assert_ne!(hash, fingerprint(&other));
    }
}
