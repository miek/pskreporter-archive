#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

mod schema;

use chrono::NaiveDateTime;
use chrono::naive::serde::ts_seconds;
use diesel::insert_into;
use diesel::prelude::*;
use serde_xml_rs::from_reader;

use schema::reports;

#[derive(Debug, Deserialize)]
struct Value<T> {
    pub value: T,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "reports"]
#[serde(rename_all = "camelCase")]
struct ReceptionReport {
    pub receiver_callsign: String,
    pub receiver_locator: String,
    pub sender_callsign: String,
    pub sender_locator: String,
    pub frequency: i64,
    #[serde(with = "ts_seconds", rename = "flowStartSeconds")]
    pub timestamp: NaiveDateTime,
    pub mode: String,
    pub is_sender: i32,
    pub is_receiver: i32,
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
    let reception_reports: ReceptionReports = from_reader(s.as_bytes()).unwrap();
    println!("{:#?}", reception_reports);


    let url = "test.db";
    let conn = SqliteConnection::establish(&url).unwrap();

    use schema::reports::dsl::*;
    for report in reception_reports.reception_reports {
        insert_into(reports).values(&report).execute(&conn).unwrap();
    }
}
