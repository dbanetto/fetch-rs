table! {
    series {
        id -> Integer,
        title -> VarChar,
        poster_url -> Nullable<VarChar>,
    }
}

table! {
    info_blob {
       id -> Integer,
       series_id -> Integer,
       blob -> Jsonb,
       info_type -> VarChar,
    }
}

joinable!(info_blob -> series (series_id));
