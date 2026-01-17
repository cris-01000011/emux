use crate::app::App;

impl App {
    fn update_scroll_offset(&mut self, max_visible_items: usize) {
        if self.in_list {
            if self.selected < self.roms_scroll_offset {
                self.roms_scroll_offset = self.selected;
            } else if self.selected >= self.roms_scroll_offset + max_visible_items.saturating_sub(2)
            {
                // Start scrolling 2 items before the end (ante-penultimate)
                self.roms_scroll_offset = self
                    .selected
                    .saturating_sub(max_visible_items.saturating_sub(2) - 1);
            }
        } else {
            if self.selected < self.scroll_offset {
                self.scroll_offset = self.selected;
            } else if self.selected >= self.scroll_offset + max_visible_items.saturating_sub(2) {
                // Start scrolling 2 items before the end (ante-penultimate)
                self.scroll_offset = self
                    .selected
                    .saturating_sub(max_visible_items.saturating_sub(2) - 1);
            }
        }
    }

    pub fn update_scroll_for_height(&mut self, visible_height: usize) {
        let scroll_threshold_items = 5;

        let near_end = self.roms.len().saturating_sub(scroll_threshold_items);

        let threshold = if self.selected >= near_end {
            0
        } else {
            scroll_threshold_items
        };

        let max_visible_items = visible_height.saturating_sub(threshold);

        self.update_scroll_offset(max_visible_items);
    }
}
