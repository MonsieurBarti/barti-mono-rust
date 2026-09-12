#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Violation {
    pub path: String,
    pub code: String,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::Violation;

    #[test]
    fn violation_holds_path_code_message() {
        let violation = Violation {
            path: "/shipperId".to_owned(),
            code: "required".to_owned(),
            message: "shipperId is required".to_owned(),
        };
        assert_eq!(violation.path, "/shipperId");
        assert_eq!(violation.code, "required");
        assert_eq!(violation.message, "shipperId is required");
    }
}
