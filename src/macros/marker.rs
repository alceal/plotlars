#[doc(hidden)]
#[macro_export]
macro_rules! marker {
    ( $( $x:ident ),* ) => {{
        let mut marker = plotly::common::Marker::new();
        $(
            marker = marker.$x($x.unwrap());
        )*
        marker
    }};
}
