pub struct CleanupGroup {
    min: i32,
    max: i32,
}

impl CleanupGroup {
    pub fn new(min: i32, max: i32) -> Result<CleanupGroup, ()> {
        if min > max {
            return Err(());
        }
        Ok(CleanupGroup { min: min, max: max })
    }

    pub fn overlaps(&self, group: &CleanupGroup) -> bool {
        (self.min >= group.min && self.min <= group.max)
            || (self.max >= group.min && self.max <= group.max)
    }
    pub fn fully_contains(&self, group: &CleanupGroup) -> bool {
        self.min <= group.min && self.max >= group.max
    }
}

#[cfg(test)]
mod clean_group_tests {
    use super::*;
    #[test]
    fn test_fully_contained() {
        let group1 = CleanupGroup::new(2, 8).unwrap();
        let group2 = CleanupGroup::new(3, 7).unwrap();
        assert_eq!(group1.fully_contains(&group2), true);
        assert_eq!(group2.fully_contains(&group1), false);

        let group3 = CleanupGroup::new(6, 6).unwrap();
        let group4 = CleanupGroup::new(4, 6).unwrap();
        assert_eq!(group4.fully_contains(&group3), true);
        assert_eq!(group3.fully_contains(&group4), false);
    }

    #[test]
    fn test_overlap() {
        let group1 = CleanupGroup::new(5, 7).unwrap();
        let group2 = CleanupGroup::new(7, 9).unwrap();
        assert_eq!(group1.overlaps(&group2), true);
        assert_eq!(group2.overlaps(&group1), true);
        let group1 = CleanupGroup::new(2, 6).unwrap();
        let group2 = CleanupGroup::new(4, 8).unwrap();
        assert_eq!(group1.overlaps(&group2), true);
        assert_eq!(group2.overlaps(&group1), true);
    }
}
