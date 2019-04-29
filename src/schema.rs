table! {
    reports (id) {
        id -> Integer,
        receiver_callsign -> Text,
        receiver_locator -> Text,
        sender_callsign -> Text,
        sender_locator -> Text,
        frequency -> BigInt,
        timestamp -> Timestamp,
        mode -> Text,
        is_sender -> Integer,
        is_receiver -> Integer,
        sender_region -> Text,
        sender_dxcc -> Text,
        sender_dxcc_code -> Text,
        sender_dxcc_locator -> Text,
        snr -> Integer,
    }
}
