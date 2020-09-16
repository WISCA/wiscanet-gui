table! {
    applications (id) {
        id -> Nullable<Integer>,
        name -> Text,
        op_mode -> Text,
        mac_mode -> Text,
        lang -> Text,
        matlab_dir -> Text,
        matlab_func -> Text,
        matlab_log -> Text,
        num_samples -> Integer,
        sample_rate -> Float,
        freq -> Float,
        bw -> Float,
    }
}

table! {
    edgenodes (id) {
        id -> Nullable<Integer>,
        name -> Text,
        ipaddr -> Text,
        radio_type -> Text,
        radio_address -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    applications,
    edgenodes,
);
