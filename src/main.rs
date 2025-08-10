use tokio::net::TcpListener;

use axum::{
    extract::{Path, State}, http::StatusCode, routing::{get, patch}, Json, Router
};
use serde_json::json;

use sqlx::{postgres::PgPoolOptions, PgPool};

mod models;
use crate::models::{
    BotsRow, CreateBotReq, CreateBotRow, CreateVirtualMachineReq, CreateVirtualMachineRow, Frequencia, UpdateBotReq, UpdateVirtualMachineReq, VirtualMachineRow
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access the .env file.");

    let server_address: String = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:7878".to_owned());
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in the env file!");

    let db_pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(16)
        .connect(&database_url)
        .await
        .expect("Can't connect to the database!");

    let listener : TcpListener = TcpListener::bind(server_address)
        .await
        .expect("Could not create TCP Listener.");

    println!("Listening on {}", listener.local_addr().unwrap());

    let app: Router = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/api/healthcheck", get(|| async {(StatusCode::OK, json!({"sucess": true}).to_string())}))
        .route("/api/vms", get(get_vms).post(create_vm))
        .route("/api/vms/:vm_id", patch(update_vm).delete(delete_vm))
        .route("/api/bots", get(get_bots).post(create_bot))
        .route("/api/bots/:bot_id", patch(update_bot).delete(delete_bot))
        .with_state(db_pool);


    axum::serve(listener, app)
        .await
        .expect("Error serving the application");

}


async fn get_vms(
    State(pg_pool): State<PgPool>
) -> Result<(StatusCode, String), (StatusCode, String)>{
    let rows = sqlx::query_as::<_, VirtualMachineRow>("SELECT * FROM orchestrator.virtual_machines ORDER BY id ASC")
        .fetch_all(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"sucess": false, "message": e.to_string()}).to_string()
            )
        })?;

        Ok((
            StatusCode::OK,
            json!({"sucess": true, "data": rows}).to_string()
        ))
}

async fn create_vm(
    State(pg_pool): State<PgPool>,
    Json(virtual_machine): Json<CreateVirtualMachineReq>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let row: CreateVirtualMachineRow = sqlx::query_as!(
        CreateVirtualMachineRow,
        r#"INSERT INTO orchestrator.virtual_machines (nome_vm, endereco_ipv4_vm, flg_status_vm)
        VALUES ($1, $2, $3)
        RETURNING id"#,
        virtual_machine.nome_vm,
        virtual_machine.endereco_ipv4_vm,
        virtual_machine.flg_status_vm
    )
    .fetch_one(&pg_pool)
    .await 
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({ "sucess": false, "message": e.to_string() }).to_string(),
    ))?;

    Ok((
        StatusCode::CREATED,
        json!({ "sucess": true, "data": row }).to_string(),
    ))
}

async fn update_vm(
    State(pg_pool): State<PgPool>,
    Path(vm_id): Path<i32>,
    Json(virtual_machine): Json<UpdateVirtualMachineReq>
)  -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query!(
        "
        UPDATE orchestrator.virtual_machines SET
        nome_vm = $2, 
        endereco_ipv4_vm = $3,
        flg_status_vm = $4
        WHERE id = $1
        ",
        vm_id,
        virtual_machine.nome_vm,
        virtual_machine.endereco_ipv4_vm,
        virtual_machine.flg_status_vm
    ).execute(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "message": false, "message": e.to_string() }).to_string(),
        )
    })?;

    Ok((StatusCode::OK, json!({ "sucess": true }).to_string()))
}

async fn delete_vm(
    State(pg_pool): State<PgPool>,
    Path(vm_id): Path<i32>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query!("DELETE FROM orchestrator.virtual_machines where id = $1", vm_id)
        .execute(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "message": false, "message": e.to_string() }).to_string(),
            )
        })?;

        Ok((StatusCode::OK, json!({ "sucess": true }).to_string()))
}

async fn get_bots(
    State(pg_pool): State<PgPool>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as::<_, BotsRow>("SELECT * FROM orchestrator.bots ORDER BY id ASC")
        .fetch_all(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"sucess": false, "message": e.to_string()}).to_string()
            )
        })?;

        Ok((
            StatusCode::OK,
            json!({"sucess": true, "data": rows}).to_string()
        ))
}

async fn create_bot(
    State(pg_pool): State<PgPool>,
    Json(bot): Json<CreateBotReq>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let row : CreateBotRow = sqlx::query_as!(
        CreateBotRow,
        r#"INSERT INTO orchestrator.bots 
        (
            nome_automacao, flg_status_bot, frequencia_execucao,
            dia_execucao, hora_execucao, intervalo_execucao, tolerancia_execucao, 
            virtual_machine_id
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id
        "#,
        bot.nome_automacao,
        bot.flg_status_bot,
        bot.frequencia_execucao as Frequencia,
        bot.dia_execucao,
        bot.hora_execucao,
        bot.intervalo_execucao,
        bot.tolerancia_execucao,
        bot.virtual_machine_id
    )
    .fetch_one(&pg_pool)
    .await
        .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({ "sucess": false, "message": e.to_string() }).to_string(),
    ))?;

    Ok((
        StatusCode::CREATED,
        json!({ "sucess": true, "data": row }).to_string(),
    ))
}

async fn update_bot(
    State(pg_pool): State<PgPool>,
    Path(bot_id): Path<i32>,
    Json(bot): Json<UpdateBotReq>
)  -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query!(
        "
        UPDATE orchestrator.bots SET
        nome_automacao = $2,
        flg_status_bot = $3,
        frequencia_execucao = $4,
        dia_execucao = $5,
        hora_execucao = $6,
        intervalo_execucao = $7,
        tolerancia_execucao = $8,
        virtual_machine_id = $9
        WHERE id = $1
        ",
        bot_id,
        bot.nome_automacao,
        bot.flg_status_bot, 
        bot.frequencia_execucao as Option<Frequencia>,
        bot.dia_execucao,
        bot.hora_execucao,
        bot.intervalo_execucao,
        bot.tolerancia_execucao,
        bot.virtual_machine_id
    ).execute(&pg_pool)
    .await
        .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "message": false, "message": e.to_string() }).to_string(),
        )
    })?;

    Ok((StatusCode::OK, json!({ "sucess": true }).to_string()))
}

async fn delete_bot(
    State(pg_pool): State<PgPool>,
    Path(bot_id): Path<i32>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query!("DELETE FROM orchestrator.bots WHERE id = $1", bot_id)
        .execute(&pg_pool)
        .await
                .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "message": false, "message": e.to_string() }).to_string(),
            )
        })?;

        Ok((StatusCode::OK, json!({ "sucess": true }).to_string()))
}
