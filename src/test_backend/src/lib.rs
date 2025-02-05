use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::{memory_manager::{MemoryId, MemoryManager, VirtualMemory}, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
use ic_stable_structures::storable::Bound;


// Define the memory type
type Memory = VirtualMemory<DefaultMemoryImpl>;

// Define the SimpleStorage struct
#[derive(CandidType, Deserialize)]
pub struct SimpleStorage {
    data: String,
}

// Implement Storable trait for SimpleStorage
impl Storable for SimpleStorage {
    const BOUND : Bound = Bound::Unbounded;
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap()) // Serialize to bytes
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(&bytes.as_ref(), Self).unwrap() // Deserialize from bytes
    }
}

// Thread-local storage for memory manager
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // A stable BTreeMap to store the SimpleStorage for each Principal
    pub static SIMPLE_STORAGE_MAP: RefCell<StableBTreeMap<Principal, SimpleStorage, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
    ));
}

// Function to set data for a specific Principal
#[ic_cdk::update]
pub fn set_data_for_principal(principal: Principal, data: String) {
    SIMPLE_STORAGE_MAP.with(|map| {
        let mut map = map.borrow_mut();
        let simple_storage = SimpleStorage { data };
        map.insert(principal, simple_storage);  // Insert or update data for the principal
    });
}

// Function to retrieve data for a specific Principal
#[ic_cdk::query]
pub fn get_data_for_principal(principal: Principal) -> String {
    SIMPLE_STORAGE_MAP.with(|map| {
        let map = map.borrow();
        map.get(&principal).map_or_else(
            || "No data found".to_string(),  // Return a default message if data doesn't exist
            |storage| storage.data.clone()   // Retrieve stored data for the principal
        )
    })
}

ic_cdk::export_candid!();
