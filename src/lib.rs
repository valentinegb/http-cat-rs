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

#[cfg(feature = "http")]
impl TryFrom<http::StatusCode> for HttpCat {
    type Error = String;

    fn try_from(value: http::StatusCode) -> Result<Self, Self::Error> {
        match value {
            http::StatusCode::CONTINUE => Ok(HttpCat::Continue),
            http::StatusCode::SWITCHING_PROTOCOLS => Ok(HttpCat::SwitchingProtocols),
            http::StatusCode::PROCESSING => Ok(HttpCat::Processing),
            v if v.as_u16() == HttpCat::EarlyHints as u16 => Ok(HttpCat::EarlyHints),
            http::StatusCode::OK => Ok(HttpCat::Ok),
            http::StatusCode::CREATED => Ok(HttpCat::Created),
            http::StatusCode::ACCEPTED => Ok(HttpCat::Accepted),
            http::StatusCode::NON_AUTHORITATIVE_INFORMATION => {
                Ok(HttpCat::NonAuthoritativeInformation)
            }
            http::StatusCode::NO_CONTENT => Ok(HttpCat::NoContent),
            http::StatusCode::RESET_CONTENT => Ok(HttpCat::ResetContent),
            http::StatusCode::PARTIAL_CONTENT => Ok(HttpCat::PartialContent),
            http::StatusCode::MULTI_STATUS => Ok(HttpCat::MultiStatus),
            http::StatusCode::ALREADY_REPORTED => Ok(HttpCat::AlreadyReported),
            http::StatusCode::IM_USED => Ok(HttpCat::ImUsed),
            http::StatusCode::MULTIPLE_CHOICES => Ok(HttpCat::MultipleChoices),
            http::StatusCode::MOVED_PERMANENTLY => Ok(HttpCat::MovedPermanently),
            http::StatusCode::FOUND => Ok(HttpCat::Found),
            http::StatusCode::SEE_OTHER => Ok(HttpCat::SeeOther),
            http::StatusCode::NOT_MODIFIED => Ok(HttpCat::NotModified),
            http::StatusCode::USE_PROXY => Ok(HttpCat::UseProxy),
            http::StatusCode::TEMPORARY_REDIRECT => Ok(HttpCat::TemporaryRedirect),
            http::StatusCode::PERMANENT_REDIRECT => Ok(HttpCat::PermanentRedirect),
            http::StatusCode::BAD_REQUEST => Ok(HttpCat::BadRequest),
            http::StatusCode::UNAUTHORIZED => Ok(HttpCat::Unauthorized),
            http::StatusCode::PAYMENT_REQUIRED => Ok(HttpCat::PaymentRequired),
            http::StatusCode::FORBIDDEN => Ok(HttpCat::Forbidden),
            http::StatusCode::NOT_FOUND => Ok(HttpCat::NotFound),
            http::StatusCode::METHOD_NOT_ALLOWED => Ok(HttpCat::MethodNotAllowed),
            http::StatusCode::NOT_ACCEPTABLE => Ok(HttpCat::NotAcceptable),
            http::StatusCode::PROXY_AUTHENTICATION_REQUIRED => {
                Ok(HttpCat::ProxyAuthenticationRequired)
            }
            http::StatusCode::REQUEST_TIMEOUT => Ok(HttpCat::RequestTimeout),
            http::StatusCode::CONFLICT => Ok(HttpCat::Conflict),
            http::StatusCode::GONE => Ok(HttpCat::Gone),
            http::StatusCode::LENGTH_REQUIRED => Ok(HttpCat::LengthRequired),
            http::StatusCode::PRECONDITION_FAILED => Ok(HttpCat::PreconditionFailed),
            http::StatusCode::PAYLOAD_TOO_LARGE => Ok(HttpCat::PayloadTooLarge),
            v if v.as_u16() == HttpCat::RequestUriTooLong as u16 => Ok(HttpCat::RequestUriTooLong),
            http::StatusCode::UNSUPPORTED_MEDIA_TYPE => Ok(HttpCat::UnsupportedMediaType),
            v if v.as_u16() == HttpCat::RequestRangeNotSatisfiable as u16 => {
                Ok(HttpCat::RequestRangeNotSatisfiable)
            }
            http::StatusCode::EXPECTATION_FAILED => Ok(HttpCat::ExpectationFailed),
            http::StatusCode::IM_A_TEAPOT => Ok(HttpCat::ImATeapot),
            v if v.as_u16() == HttpCat::EnhanceYourCalm as u16 => Ok(HttpCat::EnhanceYourCalm),
            http::StatusCode::MISDIRECTED_REQUEST => Ok(HttpCat::MisdirectedRequest),
            http::StatusCode::UNPROCESSABLE_ENTITY => Ok(HttpCat::UnprocessableEntity),
            http::StatusCode::LOCKED => Ok(HttpCat::Locked),
            http::StatusCode::FAILED_DEPENDENCY => Ok(HttpCat::FailedDependency),
            v if v.as_u16() == HttpCat::TooEarly as u16 => Ok(HttpCat::TooEarly),
            http::StatusCode::UPGRADE_REQUIRED => Ok(HttpCat::UpgradeRequired),
            http::StatusCode::PRECONDITION_REQUIRED => Ok(HttpCat::PreconditionRequired),
            http::StatusCode::TOO_MANY_REQUESTS => Ok(HttpCat::TooManyRequests),
            http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE => {
                Ok(HttpCat::RequestHeaderFieldsTooLarge)
            }
            v if v.as_u16() == HttpCat::NoResponse as u16 => Ok(HttpCat::NoResponse),
            v if v.as_u16() == HttpCat::BlockedByWindowsParentalControls as u16 => {
                Ok(HttpCat::BlockedByWindowsParentalControls)
            }
            http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS => {
                Ok(HttpCat::UnavailableForLegalReasons)
            }
            v if v.as_u16() == HttpCat::HttpRequestSentToHttpsPort as u16 => {
                Ok(HttpCat::HttpRequestSentToHttpsPort)
            }
            v if v.as_u16() == HttpCat::TokenExpiredInvalid as u16 => {
                Ok(HttpCat::TokenExpiredInvalid)
            }
            v if v.as_u16() == HttpCat::ClientClosedRequest as u16 => {
                Ok(HttpCat::ClientClosedRequest)
            }
            http::StatusCode::INTERNAL_SERVER_ERROR => Ok(HttpCat::InternalServerError),
            http::StatusCode::NOT_IMPLEMENTED => Ok(HttpCat::NotImplemented),
            http::StatusCode::BAD_GATEWAY => Ok(HttpCat::BadGateway),
            http::StatusCode::SERVICE_UNAVAILABLE => Ok(HttpCat::ServiceUnavailable),
            http::StatusCode::GATEWAY_TIMEOUT => Ok(HttpCat::GatewayTimeout),
            http::StatusCode::VARIANT_ALSO_NEGOTIATES => Ok(HttpCat::VariantAlsoNegotiates),
            http::StatusCode::INSUFFICIENT_STORAGE => Ok(HttpCat::InsufficientStorage),
            http::StatusCode::LOOP_DETECTED => Ok(HttpCat::LoopDetected),
            v if v.as_u16() == HttpCat::BandwidthLimitExceeded as u16 => {
                Ok(HttpCat::BandwidthLimitExceeded)
            }
            http::StatusCode::NOT_EXTENDED => Ok(HttpCat::NotExtended),
            http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED => {
                Ok(HttpCat::NetworkAuthenticationRequired)
            }
            v if v.as_u16() == HttpCat::WebServerIsDown as u16 => Ok(HttpCat::WebServerIsDown),
            v if v.as_u16() == HttpCat::ConnectionTimedOut as u16 => {
                Ok(HttpCat::ConnectionTimedOut)
            }
            v if v.as_u16() == HttpCat::OriginIsUnreachable as u16 => {
                Ok(HttpCat::OriginIsUnreachable)
            }
            v if v.as_u16() == HttpCat::SslHandshakeFailed as u16 => {
                Ok(HttpCat::SslHandshakeFailed)
            }
            v if v.as_u16() == HttpCat::SiteFrozen as u16 => Ok(HttpCat::SiteFrozen),
            v if v.as_u16() == HttpCat::NetworkConnectTimeoutError as u16 => {
                Ok(HttpCat::NetworkConnectTimeoutError)
            }
            other => Err(format!("status code \"{other}\" is unimplemented")),
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Image(#[from] ImageError),
}
