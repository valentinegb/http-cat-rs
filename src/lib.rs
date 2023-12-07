//! The simplest way to use `http_cat` is like so:
//!
//! ```
//! use http_cat::HttpCat;
//!
//! #[tokio::main]
//! async fn main() {
//!     HttpCat::ImATeapot.get().await.unwrap().save("418.jpeg").unwrap();
//! }
//! ```
//!
//! This is the resulting image:
//!
//! [View on GitHub](https://github.com/valentinegb/http-cat-rs/blob/master/README.md)
//!
//! You can also optionally enable the [`http`](https://docs.rs/http) and/or
//! [`rocket`](https://docs.rs/rocket) features to convert those libraries' own
//! status structures to `HttpCat` with `::try_from()`.
//!
//! ```toml
//! [dependencies]
//! http_cat = { version = "0.1.0", features = ["rocket"] }
//! ```

use std::io::Cursor;

use image::{DynamicImage, ImageError, ImageFormat};
use thiserror::Error;

#[repr(u16)]
#[derive(Debug, PartialEq, Eq)]
pub enum HttpCat {
    Continue = 100,
    SwitchingProtocols,
    Processing,
    EarlyHints,
    Ok = 200,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    ImUsed = 226,
    MultipleChoices = 300,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
    PermanentRedirect,
    BadRequest = 400,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    RequestUriTooLong,
    UnsupportedMediaType,
    RequestRangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    EnhanceYourCalm = 420,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired = 428,
    TooManyRequests,
    RequestHeaderFieldsTooLarge = 431,
    NoResponse = 444,
    BlockedByWindowsParentalControls = 450,
    UnavailableForLegalReasons,
    HttpRequestSentToHttpsPort = 497,
    TokenExpiredInvalid,
    ClientClosedRequest,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    VariantAlsoNegotiates = 506,
    InsufficientStorage,
    LoopDetected,
    BandwidthLimitExceeded,
    NotExtended,
    NetworkAuthenticationRequired,
    WebServerIsDown = 521,
    ConnectionTimedOut,
    OriginIsUnreachable,
    SslHandshakeFailed = 525,
    SiteFrozen = 530,
    NetworkConnectTimeoutError = 599,
}

impl HttpCat {
    pub async fn get(self) -> Result<DynamicImage, Error> {
        let mut image = image::io::Reader::new(Cursor::new(
            reqwest::get(format!("https://http.cat/{}", self as u16))
                .await?
                .bytes()
                .await?,
        ));

        image.set_format(ImageFormat::Jpeg);

        Ok(image.decode()?)
    }
}

impl TryFrom<u16> for HttpCat {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            v if HttpCat::Continue as u16 == v => Ok(HttpCat::Continue),
            v if HttpCat::SwitchingProtocols as u16 == v => Ok(HttpCat::SwitchingProtocols),
            v if HttpCat::Processing as u16 == v => Ok(HttpCat::Processing),
            v if HttpCat::EarlyHints as u16 == v => Ok(HttpCat::EarlyHints),
            v if HttpCat::Ok as u16 == v => Ok(HttpCat::Ok),
            v if HttpCat::Created as u16 == v => Ok(HttpCat::Created),
            v if HttpCat::Accepted as u16 == v => Ok(HttpCat::Accepted),
            v if HttpCat::NonAuthoritativeInformation as u16 == v => {
                Ok(HttpCat::NonAuthoritativeInformation)
            }
            v if HttpCat::NoContent as u16 == v => Ok(HttpCat::NoContent),
            v if HttpCat::ResetContent as u16 == v => Ok(HttpCat::ResetContent),
            v if HttpCat::PartialContent as u16 == v => Ok(HttpCat::PartialContent),
            v if HttpCat::MultiStatus as u16 == v => Ok(HttpCat::MultiStatus),
            v if HttpCat::AlreadyReported as u16 == v => Ok(HttpCat::AlreadyReported),
            v if HttpCat::ImUsed as u16 == v => Ok(HttpCat::ImUsed),
            v if HttpCat::MultipleChoices as u16 == v => Ok(HttpCat::MultipleChoices),
            v if HttpCat::MovedPermanently as u16 == v => Ok(HttpCat::MovedPermanently),
            v if HttpCat::Found as u16 == v => Ok(HttpCat::Found),
            v if HttpCat::SeeOther as u16 == v => Ok(HttpCat::SeeOther),
            v if HttpCat::NotModified as u16 == v => Ok(HttpCat::NotModified),
            v if HttpCat::UseProxy as u16 == v => Ok(HttpCat::UseProxy),
            v if HttpCat::TemporaryRedirect as u16 == v => Ok(HttpCat::TemporaryRedirect),
            v if HttpCat::PermanentRedirect as u16 == v => Ok(HttpCat::PermanentRedirect),
            v if HttpCat::BadRequest as u16 == v => Ok(HttpCat::BadRequest),
            v if HttpCat::Unauthorized as u16 == v => Ok(HttpCat::Unauthorized),
            v if HttpCat::PaymentRequired as u16 == v => Ok(HttpCat::PaymentRequired),
            v if HttpCat::Forbidden as u16 == v => Ok(HttpCat::Forbidden),
            v if HttpCat::NotFound as u16 == v => Ok(HttpCat::NotFound),
            v if HttpCat::MethodNotAllowed as u16 == v => Ok(HttpCat::MethodNotAllowed),
            v if HttpCat::NotAcceptable as u16 == v => Ok(HttpCat::NotAcceptable),
            v if HttpCat::ProxyAuthenticationRequired as u16 == v => {
                Ok(HttpCat::ProxyAuthenticationRequired)
            }
            v if HttpCat::RequestTimeout as u16 == v => Ok(HttpCat::RequestTimeout),
            v if HttpCat::Conflict as u16 == v => Ok(HttpCat::Conflict),
            v if HttpCat::Gone as u16 == v => Ok(HttpCat::Gone),
            v if HttpCat::LengthRequired as u16 == v => Ok(HttpCat::LengthRequired),
            v if HttpCat::PreconditionFailed as u16 == v => Ok(HttpCat::PreconditionFailed),
            v if HttpCat::PayloadTooLarge as u16 == v => Ok(HttpCat::PayloadTooLarge),
            v if HttpCat::RequestUriTooLong as u16 == v => Ok(HttpCat::RequestUriTooLong),
            v if HttpCat::UnsupportedMediaType as u16 == v => Ok(HttpCat::UnsupportedMediaType),
            v if HttpCat::RequestRangeNotSatisfiable as u16 == v => {
                Ok(HttpCat::RequestRangeNotSatisfiable)
            }
            v if HttpCat::ExpectationFailed as u16 == v => Ok(HttpCat::ExpectationFailed),
            v if HttpCat::ImATeapot as u16 == v => Ok(HttpCat::ImATeapot),
            v if HttpCat::EnhanceYourCalm as u16 == v => Ok(HttpCat::EnhanceYourCalm),
            v if HttpCat::MisdirectedRequest as u16 == v => Ok(HttpCat::MisdirectedRequest),
            v if HttpCat::UnprocessableEntity as u16 == v => Ok(HttpCat::UnprocessableEntity),
            v if HttpCat::Locked as u16 == v => Ok(HttpCat::Locked),
            v if HttpCat::FailedDependency as u16 == v => Ok(HttpCat::FailedDependency),
            v if HttpCat::TooEarly as u16 == v => Ok(HttpCat::TooEarly),
            v if HttpCat::UpgradeRequired as u16 == v => Ok(HttpCat::UpgradeRequired),
            v if HttpCat::PreconditionRequired as u16 == v => Ok(HttpCat::PreconditionRequired),
            v if HttpCat::TooManyRequests as u16 == v => Ok(HttpCat::TooManyRequests),
            v if HttpCat::RequestHeaderFieldsTooLarge as u16 == v => {
                Ok(HttpCat::RequestHeaderFieldsTooLarge)
            }
            v if HttpCat::NoResponse as u16 == v => Ok(HttpCat::NoResponse),
            v if HttpCat::BlockedByWindowsParentalControls as u16 == v => {
                Ok(HttpCat::BlockedByWindowsParentalControls)
            }
            v if HttpCat::UnavailableForLegalReasons as u16 == v => {
                Ok(HttpCat::UnavailableForLegalReasons)
            }
            v if HttpCat::HttpRequestSentToHttpsPort as u16 == v => {
                Ok(HttpCat::HttpRequestSentToHttpsPort)
            }
            v if HttpCat::TokenExpiredInvalid as u16 == v => Ok(HttpCat::TokenExpiredInvalid),
            v if HttpCat::ClientClosedRequest as u16 == v => Ok(HttpCat::ClientClosedRequest),
            v if HttpCat::InternalServerError as u16 == v => Ok(HttpCat::InternalServerError),
            v if HttpCat::NotImplemented as u16 == v => Ok(HttpCat::NotImplemented),
            v if HttpCat::BadGateway as u16 == v => Ok(HttpCat::BadGateway),
            v if HttpCat::ServiceUnavailable as u16 == v => Ok(HttpCat::ServiceUnavailable),
            v if HttpCat::GatewayTimeout as u16 == v => Ok(HttpCat::GatewayTimeout),
            v if HttpCat::VariantAlsoNegotiates as u16 == v => Ok(HttpCat::VariantAlsoNegotiates),
            v if HttpCat::InsufficientStorage as u16 == v => Ok(HttpCat::InsufficientStorage),
            v if HttpCat::LoopDetected as u16 == v => Ok(HttpCat::LoopDetected),
            v if HttpCat::BandwidthLimitExceeded as u16 == v => Ok(HttpCat::BandwidthLimitExceeded),
            v if HttpCat::NotExtended as u16 == v => Ok(HttpCat::NotExtended),
            v if HttpCat::NetworkAuthenticationRequired as u16 == v => {
                Ok(HttpCat::NetworkAuthenticationRequired)
            }
            v if HttpCat::WebServerIsDown as u16 == v => Ok(HttpCat::WebServerIsDown),
            v if HttpCat::ConnectionTimedOut as u16 == v => Ok(HttpCat::ConnectionTimedOut),
            v if HttpCat::OriginIsUnreachable as u16 == v => Ok(HttpCat::OriginIsUnreachable),
            v if HttpCat::SslHandshakeFailed as u16 == v => Ok(HttpCat::SslHandshakeFailed),
            v if HttpCat::SiteFrozen as u16 == v => Ok(HttpCat::SiteFrozen),
            v if HttpCat::NetworkConnectTimeoutError as u16 == v => {
                Ok(HttpCat::NetworkConnectTimeoutError)
            }
            other => Err(format!("status code \"{other}\" is unimplemented")),
        }
    }
}

#[cfg(feature = "http")]
impl TryFrom<http::StatusCode> for HttpCat {
    type Error = <Self as TryFrom<u16>>::Error;

    fn try_from(value: http::StatusCode) -> Result<Self, Self::Error> {
        HttpCat::try_from(value.as_u16())
    }
}

#[cfg(feature = "rocket")]
impl TryFrom<rocket::http::Status> for HttpCat {
    type Error = <Self as TryFrom<u16>>::Error;

    fn try_from(value: rocket::http::Status) -> Result<Self, Self::Error> {
        HttpCat::try_from(value.code)
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Image(#[from] ImageError),
}
