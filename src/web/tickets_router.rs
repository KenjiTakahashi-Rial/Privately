use axum::{
    extract::{Path, State},
    routing, Json, Router,
};

use crate::{
    ctx::Ctx,
    model::{CreateTicketParams, ModelController, Ticket},
    Result,
};

pub fn new(controller: ModelController) -> Router {
    Router::new()
        .route("/tickets", routing::get(list_tickets).post(create_ticket))
        .route("/tickets/:id", routing::delete(delete_ticket))
        .with_state(controller)
}

async fn create_ticket(
    State(controller): State<ModelController>,
    ctx: Ctx,
    Json(params): Json<CreateTicketParams>,
) -> Result<Json<Ticket>> {
    println!("create_ticket");
    let ticket = controller.create_ticket(ctx, params).await?;
    Ok(Json(ticket))
}

async fn list_tickets(
    State(controller): State<ModelController>,
    ctx: Ctx,
) -> Result<Json<Vec<Ticket>>> {
    println!("list_tickets");
    let tickets = controller.list_tickets(ctx).await?;
    Ok(Json(tickets))
}

async fn delete_ticket(
    State(controller): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("create_ticket");
    let ticket = controller.delete_ticket(ctx, id).await?;
    Ok(Json(ticket))
}