use std::{collections::BTreeMap, sync::Arc};

use axum::http::StatusCode;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

pub trait UserRepo: Send + Sync {
    fn get_by_id(&self, id: u64) -> Option<User>;
    fn paginate(&self, page: u64, rpp: u64) -> PaginationInfo<User>;
    fn save(&self, user: &User);
    fn get_current_id(&self) -> u64;
    fn remove(&self, id: u64) -> Result<(), StatusCode>;
}

#[derive(Deserialize, Debug)]
pub struct PaginationParam {
    pub page: u64,
    pub rpp: u64,
    pub foo: Option<String>,
}

#[derive(Clone)]
pub struct AppStateDyn {
    pub user_repo: Arc<dyn UserRepo>,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct User {
    pub id: u64,
    pub name: String,
}

#[derive(Deserialize)]
pub struct UserCreateParams {
    pub name: String,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct PaginationInfo<T>
where
    T: Default + Clone,
{
    pub data: Vec<T>,
    pub total: u64,
}

#[derive(Debug, Default, Clone)]
pub struct IncrementalTable<T>
where
    T: Default,
{
    pub auto_increment: u64,
    pub data: BTreeMap<u64, T>,
}

impl<T> IncrementalTable<T>
where
    T: Default + Clone,
{
    pub fn new(auto_increment: u64, data: BTreeMap<u64, T>) -> Self {
        Self {
            auto_increment,
            data,
        }
    }

    pub fn get_auto_increment(&self) -> u64 {
        self.auto_increment
    }

    pub fn insert(&mut self, data: T) -> T {
        self.data.insert(self.auto_increment, data.clone());
        self.auto_increment += 1;

        data
    }

    pub fn get(&self, index: &u64) -> Option<&T> {
        self.data.get(index)
    }
}

#[derive(Debug, Clone)]
pub struct InMemoryUserRepo {
    pub users: Arc<Mutex<IncrementalTable<User>>>,
}

impl UserRepo for InMemoryUserRepo {
    fn get_by_id(&self, id: u64) -> Option<User> {
        let users = self.users.lock();
        users.get(&id).cloned()
    }

    fn save(&self, user: &User) {
        let mut users = self.users.lock();
        users.insert(user.clone());
    }

    fn get_current_id(&self) -> u64 {
        let users = self.users.lock();
        users.get_auto_increment()
    }

    fn remove(&self, id: u64) -> Result<(), StatusCode> {
        let mut users = self.users.lock();

        let found_key = users
            .data
            .iter()
            .find(|(_key, val)| val.id.eq(&id))
            .map(|current| current.0.clone());

        if let Some(found_key) = found_key {
            users.data.remove(&found_key);
        }

        Ok(())
    }

    fn paginate(&self, page: u64, rpp: u64) -> PaginationInfo<User> {
        let users = self.users.lock();

        let mut pagination_info = PaginationInfo::<User>::default();
        pagination_info.total = users.data.keys().len() as u64;

        let start_index = page * rpp;

        for key in users
            .data
            .keys()
            .skip(start_index as usize)
            .take(rpp as usize)
        {
            let Some(item) = users.data.get(key) else {
                break;
            };

            pagination_info.data.push(item.clone());
        }

        pagination_info
    }
}

impl InMemoryUserRepo {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(IncrementalTable::new(1, BTreeMap::new()))),
        }
    }
}
