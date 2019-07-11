table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    dashboards (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        default_dashboard -> Bool,
        settings -> Jsonb,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Text,
    }
}

table! {
    widgets (id) {
        id -> Int4,
        category_id -> Int4,
        name -> Varchar,
        component_key -> Varchar,
        icon -> Varchar,
    }
}

table! {
    datapoints (id) {
        id -> Int4,
        name -> Varchar,
        #[sql_name = "type"]
        data_type -> Int4,
    }
}

table! {
    ts_number (id) {
        id -> Int4,
        ts -> BigInt,
        val -> Float4,
        ack -> Bool,
        _from -> Int4,
        q -> Int4,
    }
}

joinable!(dashboards -> users (user_id));
joinable!(widgets -> categories (category_id));

allow_tables_to_appear_in_same_query!(categories, dashboards, posts, users, widgets,);
allow_tables_to_appear_in_same_query!(datapoints, ts_number,);
