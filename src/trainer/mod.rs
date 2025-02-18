use crate::models::Model;

pub trait Trainer {
    type Model: Model + Sync;
}
