pub mod bing;
pub mod spotlight;
pub mod util;

#[cfg(test)]
mod tests {
    use crate::bing::ParsedID;

    #[test]
    fn test_parsed_id() {
        let cases = vec![
            (
                "OHR.YosemiteFirefall_ROW8895162487_1920x1080.jpg",
                ParsedID {
                    name: "YosemiteFirefall".to_string(),
                    market: "ROW".to_string(),
                    number: 8895162487,
                    width: 1920,
                    height: 1080,
                    extension: "jpg".to_string(),
                    ..ParsedID::default()
                },
            ),
            (
                "OHR.HalfDomeYosemite_EN-US4890007214_UHD.jpg",
                ParsedID {
                    name: "HalfDomeYosemite".to_string(),
                    market: "EN-US".to_string(),
                    number: 4890007214,
                    uhd: true,
                    extension: "jpg".to_string(),
                    ..ParsedID::default()
                },
            )
        ];
        
        for (id, expected) in cases {
            let parsed_id = ParsedID::from(id);
            assert_eq!(parsed_id, expected);
        }
    }
}
