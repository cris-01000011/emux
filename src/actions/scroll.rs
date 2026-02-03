use rand::Rng;

#[derive(Default)]
pub struct Scroll {
    pub selected: usize,
    pub visible: usize,
    pub start: usize,
    pub end: usize,
    pub total: usize,
}

impl Scroll {
    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
            return;
        }

        if self.start > 0 {
            self.start = self.start.saturating_sub(1);
            self.end = self.end.saturating_sub(1);
        }
    }

    pub fn move_down(&mut self) {
        if self.end == self.total && self.selected == self.visible - 1 {
            return;
        }

        if self.total < self.visible {
            match self.selected >= self.total - 1 {
                true => return,
                false => self.selected += 1,
            }
            return;
        }

        match self.selected == self.visible - 1 {
            true => {
                self.start += 1;
                self.end += 1
            }
            false => self.selected += 1,
        }
    }

    pub fn select_first(&mut self) {
        self.start = 0;
        self.selected = 0;
        self.end = self.visible;
    }

    pub fn select_last(&mut self) {
        self.start = self.total - self.visible;
        self.selected = self.visible - 1;
        self.end = self.total;
    }

    pub fn select_random(&mut self) {
        let len = self.total;

        if self.total == 0 {
            return;
        }

        let random = rand::thread_rng().gen_range(0..len);

        if random < self.visible {
            self.start = 0;
            self.end = self.visible;
            self.selected = random;
            return;
        }

        if random > self.total - self.visible {
            self.start = self.total - self.visible;
            self.end = self.total;
            self.selected = random - self.start;
            return;
        }

        self.start = random - self.visible;
        self.end = random;
        self.selected = self.visible / 2;
    }

    pub fn index_in_list(&self) -> usize {
        self.start + self.selected
    }
}
