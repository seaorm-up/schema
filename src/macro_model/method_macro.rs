#[macro_export]
macro_rules! smethod {
    (
        format($sql:tt, $instance:ident $(,$arg:expr)*),
        $item:ident,$method:ident
    ) => {
        paste!{
            #[cfg(test)]
            #[test]
            fn [<snap_ $method _str>]() {
                let $instance = instance();
                set_filters!(r"[0-9a-zA-Z]{20}\b", "[UID]",);
                assert_debug_snapshot!(format!($sql, $($arg)*));
            }

            impl $item {
                pub async fn [<$method _check>]($instance:&  $item) -> bool {
                    format!($sql, $($arg)*).check_exist().await
                }

                pub async fn [<$method _execute>]($instance:&  $item) -> Vec<$item> {
                    format!($sql, $($arg)*).execute_one::<$item>().await
                }
            }
        }
    };
// }
// macro_rules! cud {
    (
        format($sql:tt, $instance:ident, $($arg:expr)*),
        $item:ident,$method:ident,
        [$($before_check:ident)*]
        [$($after_check:ident)*]
    ) => {
        paste!{
            #[cfg(test)]
            #[test]
            fn [<snap_ $method _str>]() {
                let $instance = instance();
                set_filters!(r"[0-9a-zA-Z]{20}\b", "[UID]",);
                assert_debug_snapshot!(format!($sql, $($arg)*));
            }

            #[cfg(test)]
            #[tokio::test]
            async fn [<snap_ $method>]() {
                new_db().await;
                let $instance = instance();
                set_filters!(r"[0-9a-zA-Z]{20}\b", "[UID]",);
                set_snapshot_suffix!("before_hook");
                $(
                    assert_debug_snapshot!($item::[<$before_check _execute>](&$instance).await);
                )*
                set_snapshot_suffix!("check_execute");
                assert_debug_snapshot!(
                    $item::[<$method _check>](&$instance).await
                );
                set_snapshot_suffix!("after_hook");
                $(
                    assert_debug_snapshot!($item::[<$after_check _execute>](&$instance).await);
                )*
            }


            impl $item {
                pub async fn [<$method _check>]($instance:& $item) -> bool {
                    format!($sql, $($arg)*).check_exist().await
                }

                pub async fn [<$method _execute>]($instance:&  $item) -> Vec<$item> {
                    format!($sql, $($arg)*).execute_one::<$item>().await
                }
            }
        }
    };

}

mod test_smethod;
