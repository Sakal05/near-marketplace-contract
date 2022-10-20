use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::collections::{ UnorderedMap };
use near_sdk::{ env, near_bindgen, AccountId, PanicOnDefault, Promise };
use near_sdk::serde::{ Serialize, Deserialize };

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Marketplace {
    listed_products: UnorderedMap<String, Product>,
}

#[near_bindgen]
impl Marketplace {
    #[init]
    pub fn init() -> Self {
        //We are returning Self with a new unordered map. 
        //While creating a new unordered map, we must pass the ID as Vec<u8> type so we are converting b"listed_products" which is a byte string, to Vec<u8>, using the to_vec() function. 
        //The prefix b is used to specify that we want a byte array of the string.
        Self {
            listed_products: UnorderedMap::new(b"listed_products".to_vec()),
        }
    }

    pub fn set_product(&mut self, payload: Payload) {
        let product = Product::from_payload(payload);
        self.listed_products.insert(&product.id, &product);
    }

    //function to check wheather the product id exists
    pub fn get_product(self, id: &String) -> Option<Product> {
        self.listed_products.get(id)
    }

    pub fn get_products(self) -> Vec<Product> {
        self.listed_products.values_as_vector().to_vec()
    }

    #[payable]
    pub fn buy_product(&mut self, product_id: &String) {
        match self.listed_products.get(product_id) {
            Some(ref mut product) => {
                let price = product.price.parse().unwrap();
                assert_eq!(
                    env::attached_deposit(),
                    price,
                    "attached deposit should be equal to the price of the product"
                );
                /* We then create a new Promise object and call the transfer method on it. This method takes the amount of tokens that the caller of the function has attached to the transaction and transfers them to the owner of the product. 
                We get the account of the owner of the product by accessing the owner property of the product we retrieved. */
                let owner = &product.owner.as_str();
                Promise::new(owner.parse().unwrap()).transfer(price);
                product.increment_sold_amount();
                self.listed_products.insert(&product.id, &product);
            }
            _ => {
                env::panic_str("product not found");
            }
        }
    }
}

#[near_bindgen]
#[derive(Serialize, Deserialize, PanicOnDefault)]
pub struct Payload {
    id: String,
    name: String,
    description: String,
    image: String,
    location: String,
    price: String,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Serialize, PanicOnDefault)]
pub struct Product {
    id: String,
    name: String,
    description: String,
    image: String,
    location: String,
    price: String,
    owner: AccountId,
    sold: u32,
}

#[near_bindgen]
impl Product {
    pub fn from_payload(payload: Payload) -> Self {
        Self {
            id: payload.id,
            description: payload.description,
            name: payload.name,
            location: payload.location,
            price: payload.price,
            sold: 0,
            image: payload.image,
            owner: env::signer_account_id(),
        }
    }

    pub fn increment_sold_amount(&mut self) {
        self.sold = self.sold + 1;
    }
}