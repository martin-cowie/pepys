use super::*;

#[test]
fn test_ser() {
    let mut str0 = SystemDateTime::default();
    str0.date_time_type = SetDateTimeType::Ntp;
    str0.time_zone = Some(TimeZone { tz: "BST".to_string() });
    str0.utc_date_time = Some(DateTime {
        time: Time{hour:1, minute: 2, second: 3 },
        date: Date{ year: 1, month: 2, day: 3 }
    });

    let xml0 = yaserde::ser::to_string(&str0).unwrap();

    println!("Structure => {:?}", str0);
    println!("XML => {}", xml0);
}

#[test]
fn test_de_ok() {
    let xml_str = r#"<?xml version="1.0" encoding="utf-8"?>
        <foo:SystemDateAndTime xmlns:foo="http://www.onvif.org/ver10/schema" xmlns:X="Y">
            <foo:DateTimeType>NTP</foo:DateTimeType>
            <foo:DaylightSavings>true</foo:DaylightSavings>
            <foo:TimeZone>
                <foo:TZ>UTC-1</foo:TZ>
            </foo:TimeZone>
            <foo:UTCDateTime>
                <foo:Time>
                    <foo:Hour>12</foo:Hour>
                    <foo:Minute>7</foo:Minute>
                    <foo:Second>18</foo:Second>
                </foo:Time>
                <foo:Date>
                    <foo:Year>2022</foo:Year>
                    <foo:Month>4</foo:Month>
                    <foo:Day>2</foo:Day>
                </foo:Date>
            </foo:UTCDateTime>
            <foo:LocalDateTime>
                <foo:Time>
                    <foo:Hour>13</foo:Hour>
                    <foo:Minute>7</foo:Minute>
                    <foo:Second>18</foo:Second>
                </foo:Time>
                <foo:Date>
                    <foo:Year>2022</foo:Year>
                    <foo:Month>4</foo:Month>
                    <foo:Day>2</foo:Day>
                </foo:Date>
            </foo:LocalDateTime>
            <foo:Extension/>
        </foo:SystemDateAndTime>"#;

    let system_date_time: SystemDateTime = yaserde::de::from_str(xml_str).unwrap();
    assert_eq!(system_date_time.time_zone.unwrap().tz, "UTC-1");
}

#[test]
fn test_de_ok_real_data() {
    let xml_str = r#"<?xml version="1.0" encoding="utf-8"?>
        <tt:Something xmlns="http://www.onvif.org/ver10/device/wsdl" xmlns:tds="http://www.onvif.org/ver10/device/wsdl" xmlns:tt="http://www.onvif.org/ver10/schema">
            <tt:DateTimeType xmlns:tt="http://www.onvif.org/ver10/schema">NTP</tt:DateTimeType>
            <tt:DaylightSavings xmlns:tt="http://www.onvif.org/ver10/schema">true</tt:DaylightSavings>
            <tt:TimeZone xmlns:tt="http://www.onvif.org/ver10/schema">
                <tt:TZ>UTC-1</tt:TZ>
            </tt:TimeZone>
            <tt:UTCDateTime xmlns:tt="http://www.onvif.org/ver10/schema">
                <tt:Time>
                    <tt:Hour>12</tt:Hour>
                    <tt:Minute>7</tt:Minute>
                    <tt:Second>18</tt:Second>
                </tt:Time>
                <tt:Date>
                    <tt:Year>2022</tt:Year>
                    <tt:Month>4</tt:Month>
                    <tt:Day>2</tt:Day>
                </tt:Date>
            </tt:UTCDateTime>
            <tt:LocalDateTime xmlns:tt="http://www.onvif.org/ver10/schema">
                <tt:Time>
                    <tt:Hour>13</tt:Hour>
                    <tt:Minute>7</tt:Minute>
                    <tt:Second>18</tt:Second>
                </tt:Time>
                <tt:Date>
                    <tt:Year>2022</tt:Year>
                    <tt:Month>4</tt:Month>
                    <tt:Day>2</tt:Day>
                </tt:Date>
            </tt:LocalDateTime>
            <tt:Extension xmlns:tt="http://www.onvif.org/ver10/schema"/>
        </tt:Something>"#;

        let system_date_time: SystemDateTime = yaserde::de::from_str(xml_str).unwrap();
        assert_eq!(system_date_time.local_date_time.unwrap().time.second, 18);

}