use getset::{Getters, Setters};
use std::collections::HashSet;

#[derive(Getters, Setters)]
struct FilterList {
    #[getset(get = "pub", set = "pub")]
    allowlist: HashSet<String>,
    #[getset(get = "pub", set = "pub")]
    denylist: HashSet<String>,
}

impl FilterList {
    pub fn new() -> FilterList {
        FilterList {
            allowlist: HashSet::new(),
            denylist: HashSet::new(),
        }
    }

    pub fn push_allowlist(&mut self, item: String) -> () {
        self.allowlist.insert(item);
    }
    pub fn push_denylist(&mut self, item: String) -> () {
        self.denylist.insert(item);
    }

    pub fn get_items(&self) -> HashSet<String> {
        self.denylist
            .difference(&self.allowlist)
            .into_iter()
            .cloned()
            .collect()
    }

    pub fn clean(&mut self) {
        self.allowlist = HashSet::new();
        self.denylist = HashSet::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_items() {
        let mut map: FilterList = FilterList::new();
        map.set_allowlist(HashSet::from([
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
        ]));
        map.set_denylist(HashSet::from([
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
        ]));
        assert_eq!(map.get_items(), HashSet::from([String::from("4")]))
    }

    #[test]
    fn test_pushers() {
        let data: String = String::from("pomme");
        let mut map = FilterList::new();
        map.push_allowlist(data.clone());
        map.push_denylist(data.clone());
        assert_eq!(map.allowlist(), &HashSet::from([data.clone()]));
        assert_eq!(map.denylist(), &HashSet::from([data.clone()]));
    }

    #[test]
    fn test_clean() {
        let data: String = String::from("pomme");
        let mut map = FilterList::new();
        map.push_allowlist(data.clone());
        map.push_denylist(data.clone());
        map.clean();
        assert_eq!(map.allowlist(), &HashSet::<String>::new());
        assert_eq!(map.denylist(), &HashSet::<String>::new());
    }
}
