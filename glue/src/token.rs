pub struct HeaderToken {
    pub message: String,
}
#[cfg(feature = "actix")]
mod actix_impl {
    use super::HeaderToken;

    pub use actix_web::{dev::Payload, FromRequest as ActixFromRequest, HttpRequest};

    use futures::future::{err, ok, Ready};

    use crate::errors::{NanoServiceError, NanoServiceErrorStatus};

    impl ActixFromRequest for HeaderToken {
        type Error = NanoServiceError;
        type Future = Ready<Result<HeaderToken, NanoServiceError>>;

        fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
            let raw_data = match req.headers().get("token") {
                Some(data) => data,
                None => {
                    return err(NanoServiceError {
                        status: NanoServiceErrorStatus::Unauthorized,
                        message: "Token not in header under key 'token'".to_string(),
                    })
                }
            };
            let message = match raw_data.to_str() {
                Ok(token) => token.to_string(),
                Err(_) => {
                    return err(NanoServiceError {
                        status: NanoServiceErrorStatus::Unauthorized,
                        message: "token not a valid string".to_string(),
                    })
                }
            };

            return ok(HeaderToken { message });
        }
    }
}

#[cfg(feature = "actix")]
pub use actix_impl::ActixFromRequest;
