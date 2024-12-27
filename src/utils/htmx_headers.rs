#![allow(dead_code)]
use actix_web::http::header::HeaderName as H;

// Request headers
pub const HX_REQUEST: H = H::from_static("hx-request");
pub const HX_BOOSTED: H = H::from_static("hx-boosted");
pub const HX_CURRENT_URL: H = H::from_static("hx-current-url");
pub const HX_HISTORY_RESTORE_REQUEST: H = H::from_static("hx-history-restore-request");
pub const HX_PROMPT: H = H::from_static("hx-prompt");
pub const HX_TARGET: H = H::from_static("hx-target");
pub const HX_TRIGGER_NAME: H = H::from_static("hx-trigger-name");
pub const HX_TRIGGER: H = H::from_static("hx-trigger");

// Response headers
pub const HX_LOCATION: H = H::from_static("hx-location");
pub const HX_PUSH_URL: H = H::from_static("hx-push_url");
pub const HX_REDIRECT: H = H::from_static("hx-redirect");
pub const HX_REFRESH: H = H::from_static("hx-refresh");
pub const HX_REPLACE_URL: H = H::from_static("hx-replace-url");
pub const HX_RESWAP: H = H::from_static("hx-reswap");
pub const HX_RETARGET: H = H::from_static("hx-retarget");
pub const HX_RESELECT: H = H::from_static("hx-reselect");
pub const HX_TRIGGER_AFTER_SETTLE: H = H::from_static("hx-trigger-after-settle");
pub const HX_TRIGGER_AFTER_SWAP: H = H::from_static("hx-trigger-after-swap");

macro_rules! common_header {
    // list header, zero or more items
    ($(#[$attrs:meta])*($id:ident, $name:expr) => ($item:ty)*) => {
        $(#[$attrs])*
        #[derive(debug, clone, partialeq, eq)]
        pub struct $id(pub vec<$item>);

        impl actix_web::http::header::Header for $id {
            #[inline]
            fn name() -> actix_web::http::header::HeaderName {
                $name
            }

            #[inline]
            fn parse<M: actix_web::HttpMessage>(msg: &M) -> Result<Self, actix_web::error::ParseError> {
                let headers = msg.headers().get_all(Self::name());
                actix_web::http::header::from_comma_delimited(headers).map($id)
            }
        }

        impl ::core::fmt::Display for $id {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                actix_web::http::header::fmt_comma_delimited(f, &self.0[..])
            }
        }

        impl actix_web::http::header::TryIntoHeaderValue for $id {
            type Error = actix_web::http::header::InvalidHeaderValue;

            #[inline]
            fn try_into_value(self) -> Result<actix_web::http::header::HeaderValue, Self::Error> {
                use ::core::fmt::Write;
                let mut writer = actix_web::http::header::Writer::new();
                let _ = write!(&mut writer, "{}", self);
                actix_web::http::header::HeaderValue::from_maybe_shared(writer.take())
            }
        }
    };

    // List header, one or more items
    ($(#[$attrs:meta])*($id:ident, $name:expr) => ($item:ty)+) => {
        $(#[$attrs])*
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $id(pub Vec<$item>);

        impl actix_web::http::header::Header for $id {
            #[inline]
            fn name() -> actix_web::http::header::HeaderName {
                $name
            }

            #[inline]
            fn parse<M: actix_web::HttpMessage>(msg: &M) -> Result<Self, actix_web::error::ParseError>{
                let headers = msg.headers().get_all(Self::name());

                actix_web::http::header::from_comma_delimited(headers)
                    .and_then(|items| {
                        if items.is_empty() {
                            Err(actix_web::error::ParseError::Header)
                        } else {
                            Ok($id(items))
                        }
                    })
            }
        }

        impl ::core::fmt::Display for $id {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                actix_web::http::header::fmt_comma_delimited(f, &self.0[..])
            }
        }

        impl actix_web::http::header::TryIntoHeaderValue for $id {
            type Error = actix_web::http::header::InvalidHeaderValue;

            #[inline]
            fn try_into_value(self) -> Result<actix_web::http::header::HeaderValue, Self::Error> {
                use ::core::fmt::Write;
                let mut writer = actix_web::http::header::Writer::new();
                let _ = write!(&mut writer, "{}", self);
                actix_web::http::header::HeaderValue::from_maybe_shared(writer.take())
            }
        }
    };

    // Single value header
    ($(#[$attrs:meta])*($id:ident, $name:expr) => [$value:ty]) => {
        $(#[$attrs])*
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $id(pub $value);

        impl actix_web::http::header::Header for $id {
            #[inline]
            fn name() -> actix_web::http::header::HeaderName {
                $name
            }

            #[inline]
            fn parse<M: actix_web::HttpMessage>(msg: &M) -> Result<Self, actix_web::error::ParseError> {
                let header = msg.headers().get(Self::name());
                actix_web::http::header::from_one_raw_str(header).map($id)
            }
        }

        impl ::core::fmt::Display for $id {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.0, f)
            }
        }

        impl actix_web::http::header::TryIntoHeaderValue for $id {
            type Error = actix_web::http::header::InvalidHeaderValue;

            #[inline]
            fn try_into_value(self) -> Result<actix_web::http::header::HeaderValue, Self::Error> {
                self.0.try_into_value()
            }
        }
    };

    // List header, one or more items with "*" option
    ($(#[$attrs:meta])*($id:ident, $name:expr) => {Any / ($item:ty)+}) => {
        $(#[$attrs])*
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub enum $id {
            /// Any value is a match
            Any,

            /// Only the listed items are a match
            Items(Vec<$item>),
        }

        impl actix_web::http::header::Header for $id {
            #[inline]
            fn name() -> actix_web::http::header::HeaderName {
                $name
            }

            #[inline]
            fn parse<M: actix_web::HttpMessage>(msg: &M) -> Result<Self, actix_web::error::ParseError> {
                let is_any = msg
                    .headers()
                    .get(Self::name())
                    .and_then(|hdr| hdr.to_str().ok())
                    .map(|hdr| hdr.trim() == "*");

                if let Some(true) = is_any {
                    Ok($id::Any)
                } else {
                    let headers = msg.headers().get_all(Self::name());
                    Ok($id::Items(actix_web::http::header::from_comma_delimited(headers)?))
                }
            }
        }

        impl ::core::fmt::Display for $id {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match *self {
                    $id::Any => f.write_str("*"),
                    $id::Items(ref fields) =>
                        actix_web::http::header::fmt_comma_delimited(f, &fields[..])
                }
            }
        }

        impl actix_web::http::header::TryIntoHeaderValue for $id {
            type Error = actix_web::http::header::InvalidHeaderValue;

            #[inline]
            fn try_into_value(self) -> Result<actix_web::http::header::HeaderValue, Self::Error> {
                use ::core::fmt::Write;
                let mut writer = actix_web::http::header::Writer::new();
                let _ = write!(&mut writer, "{}", self);
                actix_web::http::header::HeaderValue::from_maybe_shared(writer.take())
            }
        }
    };
}

common_header! { (HxRequest, HX_REQUEST) => [String] }
