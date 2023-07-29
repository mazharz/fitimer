use std::collections::HashMap;

pub struct Keys {}

#[derive(Debug, Clone)]
pub struct Key {
    pub order: i32,
    pub key: char,
    pub description: String,
}

impl Keys {
    pub fn get() -> HashMap<String, Key> {
        HashMap::from([
            (
                String::from("toggle_enabled"),
                Key {
                    order: 0,
                    key: 't',
                    description: String::from("Toggle timer"),
                },
            ),
            (
                String::from("work"),
                Key {
                    order: 1,
                    key: 'w',
                    description: String::from("Change to work"),
                },
            ),
            (
                String::from("rest"),
                Key {
                    order: 2,
                    key: 'r',
                    description: String::from("Change to rest"),
                },
            ),
            (
                String::from("toggle_help_popup"),
                Key {
                    order: 3,
                    key: '?',
                    description: String::from("Toggle this popup"),
                },
            ),
            (
                String::from("quit"),
                Key {
                    order: 4,
                    key: 'q',
                    description: String::from("Quit"),
                },
            ),
        ])
    }
}
