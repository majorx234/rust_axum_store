use uuid::Uuid;

pub struct ShopItem{
    id: Uuid,
    name: String,
    price: u32,
}

impl ShopItem{
    pub fn new(name: String, price: u32) -> Self {
        let id = Uuid::new_v4();
        ShopItem{
            id,
            name,
            price
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id.clone()
    }

    pub fn get_prize(&self) -> u32 {
        self.price
    }
}
