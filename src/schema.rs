table! {

    series {
        id -> Integer,
        title -> VarChar,
        poster_url -> Nullable<VarChar>,
    }
}

table! {
    info_uri {
       id -> Integer,
       series_id -> Integer,
       uri -> VarChar,
       primary -> Bool,
    }
}

joinable!(info_uri -> series (series_id));
