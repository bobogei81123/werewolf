use crate::common::*;
use crate::user::{Identity, User};
use tokio::sync::RwLock;
use std::{
    collections::HashMap,
    sync::{atomic::{AtomicUsize, Ordering}, Arc, Mutex},
    cell::RefCell,
    borrow::BorrowMut,
};

struct Room {
    id: usize,
    players: Vec<Arc<Mutex<User>>>,
}

impl Room {
    fn new() -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            players: vec![],
        }
    }
}

impl From<&Room> for api::Room {
    fn from(room: &Room) -> Self {
        Self {
            id: room.id as u64,
            players: room.players.iter().map(|u| (&*u.borrow().into()).collect(),
        }
    }
}

pub struct Server {
    rooms: RwLock<HashMap<usize, Room>>,
    users: RwLock<HashMap<Identity, Arc<Mutex<User>>>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            rooms: RwLock::new(HashMap::new()),
            users: RwLock::new(HashMap::new()),
        }
    }

    pub fn new_user(&self, identity: Identity, name: String) {
        let user = User {
            name,
            identity,
        };
        self.users.write().borrow_mut().insert(identity, Arc::new(RefCell::new(user)));
    }

    pub fn rooms(&self) -> Vec<api::Room> {
        self.rooms.read().values().map(Into::into).collect()
    }

    pub fn create_room(&self, user: User) -> usize {
        let mut room = Room::new();
        let room_id = room.id;
        room.players.push(user);
        self.rooms.write().insert(room.id, room);

        room_id
    }

    pub fn join_room(&self, room_id: usize, user: User) -> HttpResult<()> {
        let mut rooms = self.rooms.write();

        let room = rooms
            .get_mut(&room_id)
            .context(format!("Room {} not found.", room_id))
            .or_not_found()?;

        //if room.players.contains(&user) {
            //return Err(anyhow!("User already in room.").bad_request());
        //} 
        room.players.push(user);
        Ok(())
    }
}
