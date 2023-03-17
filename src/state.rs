use std::sync::Arc;

use sqlx::PgPool;
use tokio::sync::broadcast;

use crate::Message;

pub(crate) type AppState = Arc<InnerState>;

#[derive(Clone)]
pub(crate) struct InnerState {
    tx: broadcast::Sender<Message>,
    pool : PgPool
}

impl InnerState {
    pub(crate) fn tx(&self) -> &broadcast::Sender<Message> {
        &self.tx
    }

    pub(crate) fn pool(&self) -> &PgPool {
        &self.pool
    }
}

pub(crate) trait AppStateExt {
    fn new_state(pool : PgPool) -> Self;
}

impl AppStateExt for AppState {
    fn new_state(pool : PgPool) -> Self {
        let (tx, _rx) = broadcast::channel::<Message>(1);

        Arc::new(InnerState { tx, pool })
    }
}
