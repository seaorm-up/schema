// smethod!(str,method_name,executor)
use crate::*;

#[feature(test)]
pub async fn new_db() {
    DbX::new("test".to_owned(), "test".to_owned(), "memory".to_owned())
        .await
        .ok();
}

#[macro_export]
macro_rules! smethod {
    (format($sql:tt, $instance:ident, $($arg:expr)*),$item:ident,$method:ident) => {
        paste!{
            #[cfg(test)]
            #[test]
            fn [<snap_ $method _str>]() {
                let $instance = instance();
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
    (format($sql:tt, $instance:ident, $($arg:expr)*),$item:ident,$method:ident,$check_method:ident) => {
        paste!{
            #[cfg(test)]
            #[test]
            fn [<snap_ $method _str>]() {
                let $instance = instance();
                assert_debug_snapshot!(format!($sql, $($arg)*));
            }

            #[cfg(test)]
            #[tokio::test]
            async fn [<snap_ $method>]() {
                new_db().await;
                let $instance = instance();

                set_snapshot_suffix!("before_hook");
                assert_debug_snapshot!($item::[<$check_method _execute>](&$instance).await);
                set_snapshot_suffix!("check_execute");
                assert_debug_snapshot!(
                    $item::[<$method _check>](&$instance).await
                );
                set_snapshot_suffix!("after_hook");
                assert_debug_snapshot!($item::[<$check_method _execute>](&$instance).await);
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

#[cfg(test)]
#[derive(SurrealDbObject, Debug, Clone)]
pub struct App {
    pub name: String,
}
#[cfg(test)]
mod test {
    use crate::*;
    fn instance() -> App {
        App {
            name: "adf".to_owned(),
        }
    }
    smethod!(
        format("create app CONTENT {}", instance, instance),
        App,
        create_app,
        get_app
    );

    smethod!(
        format(
            "select * from app where name = '{}'",
            instance,
            instance.name
        ),
        App,
        get_app
    );
}
