use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::{
    ctx::Ctx,
    error::{Result, ServerError},
};

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub author_id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct CreateTicketParams {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    // TODO: replace with a DB
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }

    pub async fn create_ticket(&self, ctx: Ctx, params: CreateTicketParams) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            author_id: ctx.user_id(),
            title: params.title,
        };
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self, _: Ctx) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();
        let tickets = store.iter().filter_map(|t| t.clone()).collect();
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, _: Ctx, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let ticket = store.get_mut(id as usize).and_then(|t| t.take());
        ticket.ok_or(ServerError::TicketIdNotFound { id })
    }
}
