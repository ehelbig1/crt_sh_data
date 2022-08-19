use chrono::NaiveDateTime;
use serde::Deserialize;

pub type Certs = Vec<Cert>;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Cert {
    pub issuer_ca_id: usize,
    pub issuer_name: String,
    pub common_name: String,
    pub name_value: String,
    pub id: usize,
    pub entry_timestamp: NaiveDateTime,
    pub not_before: NaiveDateTime,
    pub not_after: NaiveDateTime,
    pub serial_number: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn successfully_deserializes_json() {
        let json = r#"[{
            "issuer_ca_id": 185756,
            "issuer_name": "C=US, O=DigiCert Inc, CN=DigiCert TLS RSA SHA256 2020 CA1",
            "common_name": "www.example.org",
            "name_value": "example.com\nwww.example.com",
            "id": 5813209289,
            "entry_timestamp": "2021-12-17T11:32:05.977",
            "not_before": "2021-12-10T00:00:00",
            "not_after": "2022-12-09T23:59:59",
            "serial_number": "025216e1c4998e2632aa5d1da985b43c"
        }]"#;

        let expect = vec![Cert {
            issuer_ca_id: 185756,
            issuer_name: String::from("C=US, O=DigiCert Inc, CN=DigiCert TLS RSA SHA256 2020 CA1"),
            common_name: String::from("www.example.org"),
            name_value: String::from("example.com\nwww.example.com"),
            id: 5813209289,
            entry_timestamp: "2021-12-17T11:32:05.977".parse().unwrap(),
            not_before: "2021-12-10T00:00:00".parse().unwrap(),
            not_after: "2022-12-09T23:59:59".parse().unwrap(),
            serial_number: String::from("025216e1c4998e2632aa5d1da985b43c"),
        }];

        let got: Certs = serde_json::from_str(json).unwrap();

        assert_eq!(got, expect)
    }
}