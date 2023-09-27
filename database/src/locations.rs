pub struct DbExit {
    pub id: String,
    pub direction: String,
    pub to_room: i32,
}

pub struct DbRoom {
    pub id: i32,

    pub name: String,
    pub description: String,

    pub can_delete: bool,

    pub exits: Vec<DbExit>,
}

impl Default for DbRoom {
    fn default() -> Self {
        DbRoom {
            can_delete: true,
            description: String::from(""),
            exits: Vec::new(),
            id: 0,
            name: String::from(""),
        }
    }
}

pub struct LocationRepo;

impl LocationRepo {}
