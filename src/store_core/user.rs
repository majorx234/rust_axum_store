use uuid::Uuid;

pub struct User{
    id: Uuid,
    name: String,
}

impl User{
    pub fn new(name: String) -> Self {
        let id = Uuid::new_v4();
        User{
            id,
            name,
        }
    }
}
