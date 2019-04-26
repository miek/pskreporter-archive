#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

use serde_xml_rs::from_reader;

#[derive(Debug, Deserialize)]
struct Value<T> {
    pub value: T,
}

#[derive(Debug, Deserialize)]
struct ReceptionReport {
    pub receiverCallsign: String,
    pub receiverLocator: String,
    pub senderCallsign: String,
    pub senderLocator: String,
    pub frequency: u64,
    pub flowStartSeconds: u64,
    pub mode: String,
    pub isSender: u8,
    pub isReceiver: u8,
    pub senderRegion: String,
    pub senderDXCC: String,
    pub senderDXCCCode: String,
    pub senderDXCCLocator: String,
    pub sNR: i32,
}

#[derive(Debug, Deserialize)]
struct ReceptionReports {
    pub lastSequenceNumber: Value<u64>,
    pub maxFlowStartSeconds: Value<u64>,

    #[serde(rename = "receptionReport", default)]
    pub receptionReports: Vec<ReceptionReport>
}

fn main() {
    let s = r##"
        <?xml version="1.0"?>
        <receptionReports >
          <lastSequenceNumber value="7000505044"/>
          <maxFlowStartSeconds value="1556313633"/>
          <receptionReport receiverCallsign="SQ2OMK" receiverLocator="JO93ia" senderCallsign="MM0MZW" senderLocator="IO85jw" frequency="7075672" flowStartSeconds="1556312703" mode="FT8" isSender="1" isReceiver="0" senderRegion="Scotland" senderDXCC="Scotland" senderDXCCCode="GM" senderDXCCLocator="IO76" sNR="-13" />
        </receptionReports>
    "##;
    let reports: ReceptionReports = from_reader(s.as_bytes()).unwrap();
    println!("{:?}", reports);
}
