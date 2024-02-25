pub mod bing;
pub mod spotlight;
pub mod util;

#[cfg(test)]
mod tests {
    use crate::bing::ParsedID;

    #[test]
    fn test_parsed_id() {
        let id = "OHR.YosemiteFirefall_ROW8895162487_1920x1080.jpg";
        let parsed_id = ParsedID::from(id);

        assert_eq!(parsed_id.name, "YosemiteFirefall");
        assert_eq!(parsed_id.market, "ROW");
        assert_eq!(parsed_id.number, 8895162487);
        assert_eq!(parsed_id.width, 1920);
        assert_eq!(parsed_id.height, 1080);
        assert_eq!(parsed_id.extension, "jpg")
    }
}
