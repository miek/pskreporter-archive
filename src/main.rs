#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use serde_xml_rs::from_reader;

#[derive(Debug, Deserialize)]
struct Value<T> {
    pub value: T,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReceptionReport {
    pub receiver_callsign: String,
    pub receiver_locator: String,
    pub sender_callsign: String,
    pub sender_locator: String,
    pub frequency: u64,
    #[serde(with = "ts_seconds", rename = "flowStartSeconds")]
    pub timestamp: DateTime<Utc>,
    pub mode: String,
    pub is_sender: u8,
    pub is_receiver: u8,
    pub sender_region: String,
    #[serde(rename = "senderDXCC")]
    pub sender_dxcc: String,
    #[serde(rename = "senderDXCCCode")]
    pub sender_dxcc_code: String,
    #[serde(rename = "senderDXCCLocator")]
    pub sender_dxcc_locator: String,
    #[serde(rename = "sNR")]
    pub snr: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReceptionReports {
    pub last_sequence_number: Value<u64>,
    pub max_flow_start_seconds: Value<u64>,

    #[serde(rename = "receptionReport", default)]
    pub reception_reports: Vec<ReceptionReport>
}

fn main() {
    let s = r##"
        <?xml version="1.0"?>
        <receptionReports >
          <lastSequenceNumber value="7000505044"/>
          <maxFlowStartSeconds value="1556313633"/>
          <receptionReport receiverCallsign="SQ2OMK" receiverLocator="JO93ia" senderCallsign="MM0MZW" senderLocator="IO85jw" frequency="7075672" flowStartSeconds="1556312703" mode="FT8" isSender="1" isReceiver="0" senderRegion="Scotland" senderDXCC="Scotland" senderDXCCCode="GM" senderDXCCLocator="IO76" sNR="-13" />
        <receptionReport receiverCallsign="DF9CY" receiverLocator="JO54al83" senderCallsign="MM0MZW" senderLocator="IO85jw" frequency="7075670" flowStartSeconds="1556312700" mode="FT8" isSender="1" isReceiver="0" senderRegion="Scotland" senderDXCC="Scotland" senderDXCCCode="GM" senderDXCCLocator="IO76" sNR="-13" />
        </receptionReports>
    "##;
    let reports: ReceptionReports = from_reader(s.as_bytes()).unwrap();
    println!("{:#?}", reports);
}
