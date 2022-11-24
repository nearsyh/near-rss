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
}
