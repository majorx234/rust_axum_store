use uuid::Uuid;
use chrono;

#[derive(Debug)]
pub struct User{
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
}

impl User{
    pub fn new(name: String) -> Self {
        let id = Uuid::new_v4();
        User{
            id,
            name,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}
