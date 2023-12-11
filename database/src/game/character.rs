impl DbCharacter {
    pub fn to_game_character(&self) -> CharacterBundle {
        CharacterBundle {
            being: Being {
                pronouns: Pronouns(self.pronouns),
            },
            description: Description(self.description.clone()),
            display_name: DisplayName(self.name.clone()),
            health: Health {
                current: self.current_hp,
                max: 0,
            },
            mana: Mana {
                current: self.current_mp,
                max: 0,
            },
            info: Character {
                character_id: self.id,
                user_id: self.user_id,
            },
            location: Location {
                entity: Entity::PLACEHOLDER,
                location_id: self.current_room_id,
            },
        }
    }
}
