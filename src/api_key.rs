use rocket::{
    form::ValueField, http::Status, request, request::FromRequest, request::Outcome, Request,
};
#[derive(Debug)]
pub struct ApiKey(pub String);

#[derive(Debug)]
pub enum ApiKeyError {
    WrongQueryFieldValue,
    NoQueryFields,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // Check if API key is present in header
        let keys: Vec<ValueField> = request.query_fields().collect();
        if keys.len() == 0 {
            return Outcome::Failure((Status::Unauthorized, ApiKeyError::NoQueryFields));
        }

        let query_value = keys[0].value.to_string();

        if query_value == "yes" {
            return Outcome::Success(ApiKey(query_value));
        } else {
            return Outcome::Failure((Status::BadRequest, ApiKeyError::WrongQueryFieldValue));
        }
    }
}
