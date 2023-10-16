#[macro_export]
macro_rules! and_then {
    (|$ident:ident $(,)?| $body:expr) => {
        $ident.and_then(|$ident| $body)
    };
    (move |$ident:ident $(,)?| $body:expr) => {
        $ident.and_then(move |$ident| $body)
    };
    (|$first:ident, $($rest:ident),+ $(,)? | $body:expr) => {
        $first.and_then(|$first| and_then!(|$($rest),+| $body))
    };
    (move |$first:ident, $($rest:ident),+ $(,)? | $body:expr) => {
        $first.and_then(move |$first| and_then!(move |$($rest),+| $body))
    };
}
