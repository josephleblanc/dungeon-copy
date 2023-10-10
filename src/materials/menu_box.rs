use bevy::prelude::*;

#[derive(Clone)]
pub struct MenuBoxMaterials {
    pub top_left: Handle<Image>,
    pub top_center: Handle<Image>,
    pub top_right: Handle<Image>,
    pub mid_left: Handle<Image>,
    pub mid_center: Handle<Image>,
    pub mid_right: Handle<Image>,
    pub bottom_left: Handle<Image>,
    pub bottom_center: Handle<Image>,
    pub bottom_right: Handle<Image>,
}

impl MenuBoxMaterials {
    pub fn build_box(
        &self,
        tiles_wide: usize,
        tiles_tall: usize,
    ) -> Result<Vec<Vec<Handle<Image>>>, &'static str> {
        if tiles_wide < 3 {
            return Err("Menu box must be at least 3 tiles wide.");
        }
        if tiles_tall < 3 {
            return Err("Menu box must be at least 3 tiles tall.");
        }
        let mut menu_box: Vec<Vec<Handle<Image>>> = vec![];
        for row_index in 0..tiles_tall {
            let mut row: Vec<Handle<Image>> = Vec::new();
            for column_index in 0..tiles_wide {
                let image: Handle<Image> = {
                    if row_index == 0 {
                        if column_index == 0 {
                            self.top_left.clone()
                        } else if column_index + 1 == tiles_wide {
                            self.top_right.clone()
                        } else {
                            self.top_center.clone()
                        }
                    } else if row_index + 1 == tiles_tall {
                        if column_index == 0 {
                            self.bottom_left.clone()
                        } else if column_index + 1 == tiles_wide {
                            self.bottom_right.clone()
                        } else {
                            self.bottom_center.clone()
                        }
                    } else {
                        if column_index == 0 {
                            self.mid_left.clone()
                        } else if column_index + 1 == tiles_wide {
                            self.mid_right.clone()
                        } else {
                            self.mid_center.clone()
                        }
                    }
                };
                row.push(image);
            }
            menu_box.push(row);
        }
        Ok(menu_box)
    }
}
