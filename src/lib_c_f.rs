use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::collections::{ UnorderedMap };
use near_sdk::{ env, near_bindgen, AccountId, PanicOnDefault, Promise };
use near_sdk::serde::{ Serialize, Deserialize };

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Marketplace {
    listed_project: UnorderedMap<String, Project>,
}

#[near_bindgen]
impl Marketplace {
    #[init]
    pub fn init() -> Self {
        
        Self {
            listed_projects: UnorderedMap::new(b"listed_projects".to_vec()),
        }
    }

    pub fn set_product(&mut self, payload: Payload) {
        let project = Project::from_payload(payload);
        self.listed_projects.insert(&project.id, &project);
    }

    //function to check wheather the product id exists
    pub fn get_project(self, id: &String) -> Option<Project> {
        self.listed_products.get(id)
    }

    pub fn get_projects(self) -> Vec<Project> {
        self.listed_projects.values_as_vector().to_vec()
    }

    #[payable]
    pub fn donate_project(&mut self, project_id: &String) {
        match self.listed_projects.get(project_id) {
            Some(ref mut project) => {
                let donation = project.donation.parse().unwrap();
                // Get who is calling the method and how much $NEAR they attached
                let donor: AccountId = env::predecessor_account_id();
                let donation: Balance = env::attached_deposit();

                let mut total_donation = self.donations.get(&donor).unwrap_or(0);

                // assert_eq!(
                //     env::attached_deposit(),
                //     donation,
                //     "attached deposit should be equal to the price of the product"
                // );

                /* We then create a new Promise object and call the transfer method on it. This method takes the amount of tokens that the caller of the function has attached to the transaction and transfers them to the owner of the product. 
                We get the account of the owner of the product by accessing the owner property of the product we retrieved. */
                let owner = &project.owner.as_str();
                Promise::new(owner.parse().unwrap()).transfer(donation);
                project.increment_number_donor();
                project.increment_total_donation();
                self.listed_projects.insert(&project.id, &project);
            }
            _ => {
                env::panic_str("Project not found");
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
    target_investment: String,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Serialize, PanicOnDefault)]
pub struct Project {
    id: String,
    name: String,
    description: String,
    image: String,  
    location: String, //location of the project
    target_investment: String,  //targeted amount of money that project's owner are aiming for
    
    owner: AccountId,   //owner of each project
    total_donor: u32, //increment by number of donors
    donation: u32,
    total_donation: u32, //total amount of money from donation
}

#[near_bindgen]
impl Project {
    pub fn from_payload(payload: Payload) -> Self {
        Self {
            id: payload.id,
            description: payload.description,
            name: payload.name,
            location: payload.location,
            target_investment: payload.target_investment,
            image: payload.image,

            owner: env::signer_account_id(),
            donation: env::attached_deposit(),
            total_donor: 0,
            total_donation: 0
        }
    }

    //function to calculate the total number of donor
    pub fn increment_number_donor(&mut self) {
        self.total_donor = self.total_donor + 1;
    }

    //function to calcuate total amount of donation
    pub fn increment_total_donation(&mut self,) {
        self.total_donation = self.total_donation + self.donation;
    }
}