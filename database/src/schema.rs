table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    dashboard_folders (id) {
        id -> Int4,
        parent_id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        icon -> Varchar,
    }
}

table! {
    dashboards (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        default_dashboard -> Bool,
        settings -> Jsonb,
        icon -> Varchar,
        dashboard_folder_id -> Int4,
    }
}

table! {
    datapoints (id) {
        id -> Int4,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Int4,
    }
}

table! {
    sources (id) {
        id -> Int4,
        name -> Nullable<Text>,
    }
}

table! {
    ts_bool (id, ts) {
        id -> Int4,
        ts -> Int8,
        val -> Nullable<Bool>,
        ack -> Nullable<Bool>,
        _from -> Nullable<Int4>,
        q -> Nullable<Int4>,
    }
}

table! {
    ts_number (id, ts) {
        id -> Int4,
        ts -> Int8,
        val -> Float4,
        ack -> Nullable<Bool>,
        _from -> Nullable<Int4>,
        q -> Nullable<Int4>,
    }
}

table! {
    ts_string (id, ts) {
        id -> Int4,
        ts -> Int8,
        val -> Nullable<Text>,
        ack -> Nullable<Bool>,
        _from -> Nullable<Int4>,
        q -> Nullable<Int4>,
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

joinable!(dashboards -> users (user_id));
joinable!(dashboard_folders -> users (user_id));
joinable!(widgets -> categories (category_id));
joinable!(dashboards -> dashboard_folders (dashboard_folder_id));

allow_tables_to_appear_in_same_query!(
    categories,
    dashboard_folders,
    dashboards,
    datapoints,
    sources,
    ts_bool,
    ts_number,
    ts_string,
    users,
    widgets,
);
