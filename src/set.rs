pub(super) struct Set([bool; 9], usize);

impl Default for Set {
    fn default() -> Self {
        Self([true; 9], 9)
    }
}

impl Set {
    /// vlaue is 1-9
    pub(super) fn set(&mut self, value: u8) {
        self.0 = [false; 9];
        self.1 = 0;
        self.add(value);
    }
    fn contains(&self, value: &usize) -> bool {
        self.0[*value]
    }
    /// vlaue is 1-9
    pub(super) fn remove(&mut self, value: u8) -> Option<()> {
        let index = value as usize - 1;
        if self.contains(&index) {
            self.0[index] = false;
            self.1 -= 1;
            Some(())
        } else {
            None
        }
    }

    /// vlaue is 1-9
    pub(super) fn add(&mut self, value: u8) -> Option<()> {
        let index = value as usize - 1;
        if !self.contains(&index) {
            self.0[index] = true;
            self.1 += 1;
            Some(())
        } else {
            None
        }
    }

    pub(super) fn size(&self) -> usize {
        self.1
    }

    /// retuen num is 1-9
    pub(super) fn get(&self) -> Vec<u8> {
        self.0
            .iter()
            .enumerate()
            .filter(|value| *value.1)
            .map(|(value, _)| value as u8 + 1)
            .collect()
    }

    /// retuen num is 1-9
    pub(super) fn find_only(&self) -> Option<u8> {
        if self.size() == 1 {
            self.0
                .iter()
                .enumerate()
                .find(|(_value, item)| **item)
                .map(|value| value.0 as u8 + 1)
        } else {
            None
        }
    }
}
