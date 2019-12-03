use {
    super::util::Marker
};

pub struct Player {
    pub name: String,
    pub marker: Marker,
}

impl Player {
    pub fn new(name: &str, marker: Marker) -> Self {
        Self {
            name: name.to_string(),
            marker
        }
    }
}
