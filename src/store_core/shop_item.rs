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
}
