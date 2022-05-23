#[cfg(test)]
mod tests {
    use crate::http::{test_url, HttpTestResponseTrait};
    use nachtwacht_models::n8w8::AuthBasicCredentials;

    #[test]
    fn test_url_exists() {
        let url = url::Url::parse("https://www.google.com/").unwrap();
        assert_eq!(
            test_url(
                &url,
                5000,
                "GET",
                &AuthBasicCredentials {
                    username: "".to_string(),
                    password: "".to_string(),
                    unknown_fields: Default::default(),
                    cached_size: Default::default()
                }
            )
            .not_successful(),
            false
        );
    }
}
