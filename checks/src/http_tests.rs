// Copyright (C) 2024 Dirk Strauss
//
// This file is part of Nachtwacht.
//
// Nachtwacht is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Nachtwacht is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#[cfg(test)]
mod tests {
    use crate::http::{test_url, HttpTestResponseTrait};
    use nachtwacht_models::generated::n8w8::AuthBasicCredentials;
    use pretty_assertions::assert_eq;

    #[test]
    #[ignore]
    fn test_url_exists() {
        let url = url::Url::parse("https://www.google.com/").unwrap();
        assert_eq!(
            test_url(
                &url,
                20000,
                "GET",
                &AuthBasicCredentials {
                    username: "".to_string(),
                    password: "".to_string(),
                    special_fields: Default::default()
                }
            )
            .not_successful(),
            false
        );
    }
}
