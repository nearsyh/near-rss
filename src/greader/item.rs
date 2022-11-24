use crate::database::items::Item;

impl Item {
    pub fn categories(&self) -> Vec<String> {
        let mut ret = vec![];
        ret.push("user/-/state/com.google/reading-list".to_owned());
        if self.read {
            ret.push("user/-/state/com.google/read".to_owned());
        } else {
            ret.push("user/-/state/com.google/fresh".to_owned());
        }
        if self.starred {
            ret.push("user/-/state/com.google/starred".to_owned());
        }
        ret
    }

    pub fn id_str_to_i64(id: &str) -> i64 {
        if id.starts_with("tag:google.com,2005:reader/item/") {
            let id_hex = id.strip_prefix("tag:google.com,2005:reader/item/").unwrap();
            i64::from_str_radix(id_hex, 16).unwrap_or(-1)
        } else {
            id.parse::<i64>().unwrap_or(-1)
        }
    }
}
