use std::io::Cursor;

use image::{DynamicImage, ImageError, ImageFormat};
use thiserror::Error;

#[repr(u16)]
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

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Image(#[from] ImageError),
}
