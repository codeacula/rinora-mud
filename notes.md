            world.send_event(TextEvent::from_str(
                command.entity,
                "Character names can only contain the letters A-Z, and only one word. Please try again.",
            ));
