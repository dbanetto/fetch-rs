diesel::table! {
    series {
        id -> Integer,
        title -> VarChar,
        poster_url -> Nullable<VarChar>,
    }
}

diesel::table! {
    info_blob {
       id -> Integer,
       series_id -> Integer,
       blob -> Jsonb,
       info_type -> VarChar,
    }
}

diesel::joinable!(info_blob -> series (series_id));
