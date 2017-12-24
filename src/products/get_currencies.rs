use hyper::Method;
use serde_json;

use rest_client::{deserialize_from_str, EndPointRequest, EndPointRequestHandler};
use url::Route;

#[derive(Default)]
pub struct GetCurrencies;

impl GetCurrencies {
    pub fn new() -> GetCurrencies {
        GetCurrencies::default()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Currency {
    id: String,
    name: String,
    #[serde(deserialize_with = "deserialize_from_str")] min_size: f64,
}

impl EndPointRequestHandler<Vec<Currency>> for GetCurrencies {
    fn create_request(&self) -> EndPointRequest {
        EndPointRequest {
            http_method: Method::Get,
            route: Route::new().add_segment(&"currencies"),
            body: String::new(),
        }
    }

    fn deserialize(&self, http_body: String) -> Vec<Currency> {
        serde_json::from_str(&http_body).unwrap()
    }
}

#[cfg(test)]
mod test {
    use hyper::Method;

    use super::{Currency, EndPointRequest, EndPointRequestHandler, GetCurrencies, Route};

    #[test]
    fn test_create_request() {
        let result = GetCurrencies::new().create_request();

        let expected = EndPointRequest {
            http_method: Method::Get,
            route: Route::new().add_segment(&"currencies"),
            body: String::new(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_deserialize() {
        let result = GetCurrencies::new().deserialize(String::from(
            "[{
    \"id\": \"BTC\",
    \"name\": \"Bitcoin\",
    \"min_size\": \"0.00000001\"
}, {
    \"id\": \"USD\",
    \"name\": \"United States Dollar\",
    \"min_size\": \"0.01000000\"
}]",
        ));
        let expected = vec![
            Currency {
                id: String::from("BTC"),
                name: String::from("Bitcoin"),
                min_size: 0.00000001,
            },
            Currency {
                id: String::from("USD"),
                name: String::from("United States Dollar"),
                min_size: 0.01,
            },
        ];

        assert_eq!(result, expected);
    }
}