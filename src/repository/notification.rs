use std::sync::RwLock;

use lazy_static::lazy_static;

pub struct NotificationRepository;

use crate::model::notification::{self, Notification};

lazy_static! {
    static ref NOTIFICATIONS: RwLock<Vec<Notification>> = RwLock::new(vec![]);
}

impl NotificationRepository{
    
}