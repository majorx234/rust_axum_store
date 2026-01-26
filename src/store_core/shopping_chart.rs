use uuid::Uuid;

use super::shop_item::ShopItem;

pub struct ShoppingCart{
    shop_item_list: Vec<Uuid>,
}

impl ShoppingCart{
    pub fn new() -> Self {
        ShoppingCart{
            shop_item_list: Vec::new()
        }
    }
    pub fn add_item(&mut self, item: ShopItem){
        self.shop_item_list.push(item.get_id());
    }
}

impl Default for ShoppingCart{
    fn default() -> Self {
        Self::new()
    }
}
