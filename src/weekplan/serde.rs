#[cfg(test)]
mod test {
    use crate::weekplan::*;

    #[test]
    fn weekday() {
        let day_in = Weekday::Monday;
        let json_out = serde_json::to_string(&day_in).unwrap();

        let json_in = r#""Monday""#;
        let day_out: Weekday = serde_json::from_str(&json_in).unwrap();

        assert_eq!(day_in, day_out);
        assert_eq!(json_in, json_out);
    }

    #[test]
    fn time() {
        let time_in = Time::new(8, 30).unwrap();
        let json_out = serde_json::to_string(&time_in).unwrap();

        let json_in = r#"{"hour":8,"minute":30}"#;
        let time_out: Time = serde_json::from_str(&r#"{"hour": 8, "minute": 30}"#).unwrap();

        assert_eq!(time_in, time_out);
        assert_eq!(json_in, json_out);
    }
}
