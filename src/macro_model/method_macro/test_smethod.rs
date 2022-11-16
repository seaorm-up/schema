#![cfg(test)]

use crate::*;
smethod!(
    format("create app CONTENT {}", instance, instance),
    App,
    create_app // [get_app][get_app]
);
// delete would return [] all ways
// create as test would return value, so it is no need aa get_app
smethod!(
    format("delete app where name='{}'", instance, instance.name),
    App,
    delete,
    [create_app][get_app]
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
