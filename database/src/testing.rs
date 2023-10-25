use shared::prelude::*;

use crate::prelude::*;

pub fn create_bad_test_character(db_interface: &DbInterface, character_name: String) {
    let user = get_bad_dummy_user(db_interface);
    db_interface
        .characters
        .create_character(&character_name, 1, &user)
        .expect("Failed to create character");
}

pub fn create_test_character(db_interface: &DbInterface, character_name: String) {
    let user = get_dummy_user(db_interface);
    db_interface
        .characters
        .create_character(&character_name, 1, &user)
        .expect("Failed to create character");
}

pub fn get_bad_dummy_user(db_interface: &DbInterface) -> User {
    db_interface
        .users
        .get_by_id(2)
        .expect("Failed to unwrap result")
        .expect("Failed to unwrap value")
}

pub fn get_dummy_user(db_interface: &DbInterface) -> User {
    db_interface
        .users
        .get_by_id(1)
        .expect("Failed to unwrap result")
        .expect("Failed to unwrap value")
}
