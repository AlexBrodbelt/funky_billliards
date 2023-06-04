use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(menu);
    }
}

pub fn menu() {
    println!("You are on the main menu.")    
}