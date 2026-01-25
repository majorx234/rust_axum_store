use uuid::Uuid;

pub struct ShoppingCart{
    shop_item_list: Vec<Uuid>,
}

impl ShoppingCart{
    pub fn new() -> Self {
        ShoppingCart{
            shop_item_list: Vec::new()
        }
    }
}
