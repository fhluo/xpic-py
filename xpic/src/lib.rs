pub mod bing;
pub mod spotlight;
pub mod util;

#[cfg(test)]
mod tests {
    use crate::bing::ImageDetail;

    #[test]
    fn test_parsed_id() {
        let cases = vec![
            (
                "OHR.YosemiteFirefall_ROW8895162487_1920x1080.jpg",
                ImageDetail {
                    name: "YosemiteFirefall".to_string(),
                    market: "ROW".to_string(),
                    number: 8895162487,
                    width: 1920,
                    height: 1080,
                    extension: "jpg".to_string(),
                    ..Default::default()
                },
            ),
            (
                "OHR.HalfDomeYosemite_EN-US4890007214_UHD.jpg",
                ImageDetail {
                    name: "HalfDomeYosemite".to_string(),
                    market: "EN-US".to_string(),
                    number: 4890007214,
                    uhd: true,
                    extension: "jpg".to_string(),
                    ..Default::default()
                },
            ),
        ];

        for (id, expected) in cases {
            let parsed_id = ImageDetail::try_from(id);
            assert_eq!(parsed_id, Ok(expected));
        }
    }
}
