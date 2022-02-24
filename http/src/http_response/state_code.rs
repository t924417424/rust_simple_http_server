#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HttpStateCode {
    StatusContinue,                      // RFC 7231, 6.2.1
    StatusSwitchingProtocols,            // RFC 7231, 6.2.2
    StatusProcessing,                    // RFC 2518, 10.1
    StatusEarlyHints,                    // RFC 8297
    StatusOK,                            // RFC 7231, 6.3.1
    StatusCreated,                       // RFC 7231, 6.3.2
    StatusAccepted,                      // RFC 7231, 6.3.3
    StatusNonAuthoritativeInformation,   // RFC 7231, 6.3.4
    StatusNoContent,                     // RFC 7231, 6.3.5
    StatusResetContent,                  // RFC 7231, 6.3.6
    StatusPartialContent,                // RFC 7233, 4.1
    StatusMultiStatus,                   // RFC 4918, 11.1
    StatusAlreadyReported,               // RFC 5842, 7.1
    StatusIMUsed,                        // RFC 3229, 10.4.1
    StatusMultipleChoices,               // RFC 7231, 6.4.1
    StatusMovedPermanently,              // RFC 7231, 6.4.2
    StatusFound,                         // RFC 7231, 6.4.3
    StatusSeeOther,                      // RFC 7231, 6.4.4
    StatusNotModified,                   // RFC 7232, 4.1
    StatusUseProxy,                      // RFC 7231, 6.4.5
    StatusTemporaryRedirect,             // RFC 7231, 6.4.7
    StatusPermanentRedirect,             // RFC 7538, 3
    StatusBadRequest,                    // RFC 7231, 6.5.1
    StatusUnauthorized,                  // RFC 7235, 3.1
    StatusPaymentRequired,               // RFC 7231, 6.5.2
    StatusForbidden,                     // RFC 7231, 6.5.3
    StatusNotFound,                      // RFC 7231, 6.5.4
    StatusMethodNotAllowed,              // RFC 7231, 6.5.5
    StatusNotAcceptable,                 // RFC 7231, 6.5.6
    StatusProxyAuthRequired,             // RFC 7235, 3.2
    StatusRequestTimeout,                // RFC 7231, 6.5.7
    StatusConflict,                      // RFC 7231, 6.5.8
    StatusGone,                          // RFC 7231, 6.5.9
    StatusLengthRequired,                // RFC 7231, 6.5.10
    StatusPreconditionFailed,            // RFC 7232, 4.2
    StatusRequestEntityTooLarge,         // RFC 7231, 6.5.11
    StatusRequestURITooLong,             // RFC 7231, 6.5.12
    StatusUnsupportedMediaType,          // RFC 7231, 6.5.13
    StatusRequestedRangeNotSatisfiable,  // RFC 7233, 4.4
    StatusExpectationFailed,             // RFC 7231, 6.5.14
    StatusTeapot,                        // RFC 7168, 2.3.3
    StatusMisdirectedRequest,            // RFC 7540, 9.1.2
    StatusUnprocessableEntity,           // RFC 4918, 11.2
    StatusLocked,                        // RFC 4918, 11.3
    StatusFailedDependency,              // RFC 4918, 11.4
    StatusTooEarly,                      // RFC 8470, 5.2.
    StatusUpgradeRequired,               // RFC 7231, 6.5.15
    StatusPreconditionRequired,          // RFC 6585, 3
    StatusTooManyRequests,               // RFC 6585, 4
    StatusRequestHeaderFieldsTooLarge,   // RFC 6585, 5
    StatusUnavailableForLegalReasons,    // RFC 7725, 3
    StatusInternalServerError,           // RFC 7231, 6.6.1
    StatusNotImplemented,                // RFC 7231, 6.6.2
    StatusBadGateway,                    // RFC 7231, 6.6.3
    StatusServiceUnavailable,            // RFC 7231, 6.6.4
    StatusGatewayTimeout,                // RFC 7231, 6.6.5
    StatusHTTPVersionNotSupported,       // RFC 7231, 6.6.6
    StatusVariantAlsoNegotiates,         // RFC 2295, 8.1
    StatusInsufficientStorage,           // RFC 4918, 11.5
    StatusLoopDetected,                  // RFC 5842, 7.2
    StatusNotExtended,                   // RFC 2774, 7
    StatusNetworkAuthenticationRequired, // RFC 6585, 6
}

impl From<HttpStateCode> for u16 {
    fn from(state_code: HttpStateCode) -> Self {
        match state_code {
            HttpStateCode::StatusContinue => 100,
            HttpStateCode::StatusSwitchingProtocols => 101,
            HttpStateCode::StatusProcessing => 102,
            HttpStateCode::StatusEarlyHints => 103,
            HttpStateCode::StatusOK => 200,
            HttpStateCode::StatusCreated => 201,
            HttpStateCode::StatusAccepted => 202,
            HttpStateCode::StatusNonAuthoritativeInformation => 203,
            HttpStateCode::StatusNoContent => 204,
            HttpStateCode::StatusResetContent => 205,
            HttpStateCode::StatusPartialContent => 206,
            HttpStateCode::StatusMultiStatus => 207,
            HttpStateCode::StatusAlreadyReported => 208,
            HttpStateCode::StatusIMUsed => 226,
            HttpStateCode::StatusMultipleChoices => 300,
            HttpStateCode::StatusMovedPermanently => 301,
            HttpStateCode::StatusFound => 302,
            HttpStateCode::StatusSeeOther => 303,
            HttpStateCode::StatusNotModified => 304,
            HttpStateCode::StatusUseProxy => 305,
            HttpStateCode::StatusTemporaryRedirect => 307,
            HttpStateCode::StatusPermanentRedirect => 308,
            HttpStateCode::StatusBadRequest => 400,
            HttpStateCode::StatusUnauthorized => 401,
            HttpStateCode::StatusPaymentRequired => 402,
            HttpStateCode::StatusForbidden => 403,
            HttpStateCode::StatusNotFound => 404,
            HttpStateCode::StatusMethodNotAllowed => 405,
            HttpStateCode::StatusNotAcceptable => 406,
            HttpStateCode::StatusProxyAuthRequired => 407,
            HttpStateCode::StatusRequestTimeout => 408,
            HttpStateCode::StatusConflict => 409,
            HttpStateCode::StatusGone => 410,
            HttpStateCode::StatusLengthRequired => 411,
            HttpStateCode::StatusPreconditionFailed => 412,
            HttpStateCode::StatusRequestEntityTooLarge => 413,
            HttpStateCode::StatusRequestURITooLong => 414,
            HttpStateCode::StatusUnsupportedMediaType => 415,
            HttpStateCode::StatusRequestedRangeNotSatisfiable => 416,
            HttpStateCode::StatusExpectationFailed => 417,
            HttpStateCode::StatusTeapot => 418,
            HttpStateCode::StatusMisdirectedRequest => 421,
            HttpStateCode::StatusUnprocessableEntity => 422,
            HttpStateCode::StatusLocked => 423,
            HttpStateCode::StatusFailedDependency => 424,
            HttpStateCode::StatusTooEarly => 425,
            HttpStateCode::StatusUpgradeRequired => 426,
            HttpStateCode::StatusPreconditionRequired => 428,
            HttpStateCode::StatusTooManyRequests => 429,
            HttpStateCode::StatusRequestHeaderFieldsTooLarge => 431,
            HttpStateCode::StatusUnavailableForLegalReasons => 451,
            HttpStateCode::StatusInternalServerError => 500,
            HttpStateCode::StatusNotImplemented => 501,
            HttpStateCode::StatusBadGateway => 502,
            HttpStateCode::StatusServiceUnavailable => 503,
            HttpStateCode::StatusGatewayTimeout => 504,
            HttpStateCode::StatusHTTPVersionNotSupported => 505,
            HttpStateCode::StatusVariantAlsoNegotiates => 506,
            HttpStateCode::StatusInsufficientStorage => 507,
            HttpStateCode::StatusLoopDetected => 508,
            HttpStateCode::StatusNotExtended => 510,
            HttpStateCode::StatusNetworkAuthenticationRequired => 511,
        }
    }
}

impl From<u16> for HttpStateCode {
    fn from(i: u16) -> Self {
        match i {
            100 => HttpStateCode::StatusContinue,
            101 => HttpStateCode::StatusSwitchingProtocols,
            102 => HttpStateCode::StatusProcessing,
            103 => HttpStateCode::StatusEarlyHints,
            200 => HttpStateCode::StatusOK,
            201 => HttpStateCode::StatusCreated,
            202 => HttpStateCode::StatusAccepted,
            203 => HttpStateCode::StatusNonAuthoritativeInformation,
            204 => HttpStateCode::StatusNoContent,
            205 => HttpStateCode::StatusResetContent,
            206 => HttpStateCode::StatusPartialContent,
            207 => HttpStateCode::StatusMultiStatus,
            208 => HttpStateCode::StatusAlreadyReported,
            226 => HttpStateCode::StatusIMUsed,
            300 => HttpStateCode::StatusMultipleChoices,
            301 => HttpStateCode::StatusMovedPermanently,
            302 => HttpStateCode::StatusFound,
            303 => HttpStateCode::StatusSeeOther,
            304 => HttpStateCode::StatusNotModified,
            305 => HttpStateCode::StatusUseProxy,
            307 => HttpStateCode::StatusTemporaryRedirect,
            308 => HttpStateCode::StatusPermanentRedirect,
            400 => HttpStateCode::StatusBadRequest,
            401 => HttpStateCode::StatusUnauthorized,
            402 => HttpStateCode::StatusPaymentRequired,
            403 => HttpStateCode::StatusForbidden,
            404 => HttpStateCode::StatusNotFound,
            405 => HttpStateCode::StatusMethodNotAllowed,
            406 => HttpStateCode::StatusNotAcceptable,
            407 => HttpStateCode::StatusProxyAuthRequired,
            408 => HttpStateCode::StatusRequestTimeout,
            409 => HttpStateCode::StatusConflict,
            410 => HttpStateCode::StatusGone,
            411 => HttpStateCode::StatusLengthRequired,
            412 => HttpStateCode::StatusPreconditionFailed,
            413 => HttpStateCode::StatusRequestEntityTooLarge,
            414 => HttpStateCode::StatusRequestURITooLong,
            415 => HttpStateCode::StatusUnsupportedMediaType,
            416 => HttpStateCode::StatusRequestedRangeNotSatisfiable,
            417 => HttpStateCode::StatusExpectationFailed,
            418 => HttpStateCode::StatusTeapot,
            421 => HttpStateCode::StatusMisdirectedRequest,
            422 => HttpStateCode::StatusUnprocessableEntity,
            423 => HttpStateCode::StatusLocked,
            424 => HttpStateCode::StatusFailedDependency,
            425 => HttpStateCode::StatusTooEarly,
            426 => HttpStateCode::StatusUpgradeRequired,
            428 => HttpStateCode::StatusPreconditionRequired,
            429 => HttpStateCode::StatusTooManyRequests,
            431 => HttpStateCode::StatusRequestHeaderFieldsTooLarge,
            451 => HttpStateCode::StatusUnavailableForLegalReasons,
            500 => HttpStateCode::StatusInternalServerError,
            501 => HttpStateCode::StatusNotImplemented,
            502 => HttpStateCode::StatusBadGateway,
            503 => HttpStateCode::StatusServiceUnavailable,
            504 => HttpStateCode::StatusGatewayTimeout,
            505 => HttpStateCode::StatusHTTPVersionNotSupported,
            506 => HttpStateCode::StatusVariantAlsoNegotiates,
            507 => HttpStateCode::StatusInsufficientStorage,
            508 => HttpStateCode::StatusLoopDetected,
            510 => HttpStateCode::StatusNotExtended,
            511 => HttpStateCode::StatusNetworkAuthenticationRequired,
            _ => HttpStateCode::StatusBadGateway,
        }
    }
}

impl From<HttpStateCode> for String {
    fn from(state_code: HttpStateCode) -> Self {
        match state_code {
            HttpStateCode::StatusContinue => "Continue".to_string(),
            HttpStateCode::StatusSwitchingProtocols => "Switching Protocols".to_string(),
            HttpStateCode::StatusProcessing => "Processing".to_string(),
            HttpStateCode::StatusEarlyHints => "Early Hints".to_string(),
            HttpStateCode::StatusOK => "OK".to_string(),
            HttpStateCode::StatusCreated => "Created".to_string(),
            HttpStateCode::StatusAccepted => "Accepted".to_string(),
            HttpStateCode::StatusNonAuthoritativeInformation => {
                "Non-Authoritative Information".to_string()
            }
            HttpStateCode::StatusNoContent => "No Content".to_string(),
            HttpStateCode::StatusResetContent => "Reset Content".to_string(),
            HttpStateCode::StatusPartialContent => "Partial Content".to_string(),
            HttpStateCode::StatusMultiStatus => "Multi-Status".to_string(),
            HttpStateCode::StatusAlreadyReported => "Already Reported".to_string(),
            HttpStateCode::StatusIMUsed => "IM Used".to_string(),
            HttpStateCode::StatusMultipleChoices => "Multiple Choices".to_string(),
            HttpStateCode::StatusMovedPermanently => "Moved Permanently".to_string(),
            HttpStateCode::StatusFound => "Found".to_string(),
            HttpStateCode::StatusSeeOther => "See Other".to_string(),
            HttpStateCode::StatusNotModified => "Not Modified".to_string(),
            HttpStateCode::StatusUseProxy => "Use Proxy".to_string(),
            HttpStateCode::StatusTemporaryRedirect => "Temporary Redirect".to_string(),
            HttpStateCode::StatusPermanentRedirect => "Permanent Redirect".to_string(),
            HttpStateCode::StatusBadRequest => "Bad Request".to_string(),
            HttpStateCode::StatusUnauthorized => "Unauthorized".to_string(),
            HttpStateCode::StatusPaymentRequired => "Payment Required".to_string(),
            HttpStateCode::StatusForbidden => "Forbidden".to_string(),
            HttpStateCode::StatusNotFound => "Not Found".to_string(),
            HttpStateCode::StatusMethodNotAllowed => "Method Not Allowed".to_string(),
            HttpStateCode::StatusNotAcceptable => "Not Acceptable".to_string(),
            HttpStateCode::StatusProxyAuthRequired => {
                "Proxy Authentication Required".to_string()
            }
            HttpStateCode::StatusRequestTimeout => "Request Timeout".to_string(),
            HttpStateCode::StatusConflict => "Conflict".to_string(),
            HttpStateCode::StatusGone => "Gone".to_string(),
            HttpStateCode::StatusLengthRequired => "Length Required".to_string(),
            HttpStateCode::StatusPreconditionFailed => "Precondition Failed".to_string(),
            HttpStateCode::StatusRequestEntityTooLarge => {
                "Request Entity Too Large".to_string()
            }

            HttpStateCode::StatusRequestURITooLong => "Request URI Too Long".to_string(),
            HttpStateCode::StatusUnsupportedMediaType => "Unsupported Media Type".to_string(),
            HttpStateCode::StatusRequestedRangeNotSatisfiable => {
                "Requested Range Not Satisfiable".to_string()
            }

            HttpStateCode::StatusExpectationFailed => "Expectation Failed".to_string(),
            HttpStateCode::StatusTeapot => "I'm a teapot".to_string(),
            HttpStateCode::StatusMisdirectedRequest => "Misdirected Request".to_string(),
            HttpStateCode::StatusUnprocessableEntity => "Unprocessable Entity".to_string(),
            HttpStateCode::StatusLocked => "Locked".to_string(),
            HttpStateCode::StatusFailedDependency => "Failed Dependency".to_string(),
            HttpStateCode::StatusTooEarly => "Too Early".to_string(),
            HttpStateCode::StatusUpgradeRequired => "Upgrade Required".to_string(),
            HttpStateCode::StatusPreconditionRequired => "Precondition Required".to_string(),
            HttpStateCode::StatusTooManyRequests => "Too Many Requests".to_string(),
            HttpStateCode::StatusRequestHeaderFieldsTooLarge => {
                "Request Header Fields Too Large".to_string()
            }

            HttpStateCode::StatusUnavailableForLegalReasons => {
                "Unavailable For Legal Reasons".to_string()
            }

            HttpStateCode::StatusInternalServerError => "Internal Server Error".to_string(),
            HttpStateCode::StatusNotImplemented => "Not Implemented".to_string(),
            HttpStateCode::StatusBadGateway => "Bad Gateway".to_string(),
            HttpStateCode::StatusServiceUnavailable => "Service Unavailable".to_string(),
            HttpStateCode::StatusGatewayTimeout => "Gateway Timeout".to_string(),
            HttpStateCode::StatusHTTPVersionNotSupported => {
                "HTTP Version Not Supported".to_string()
            }

            HttpStateCode::StatusVariantAlsoNegotiates => "Variant Also Negotiates".to_string(),
            HttpStateCode::StatusInsufficientStorage => "Insufficient Storage".to_string(),
            HttpStateCode::StatusLoopDetected => "Loop Detected".to_string(),
            HttpStateCode::StatusNotExtended => "Not Extended".to_string(),
            HttpStateCode::StatusNetworkAuthenticationRequired => {
                "Network Authentication Required".to_string()
            }
        }
    }
}
