use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "enum_frequencia", rename_all = "lowercase")]
pub enum Frequencia {
    Diaria, Semanal, Mensal, Trimestral, Intervalo, Demanda
}

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct VirtualMachineRow {
    pub id : i32, 
    pub nome_vm: String, 
    pub endereco_ipv4_vm: Option<String>,
    pub flg_status_vm: bool
}

#[derive(Deserialize, Debug, sqlx::FromRow)]
pub struct CreateVirtualMachineReq {
    pub nome_vm: String, 
    pub endereco_ipv4_vm: Option<String>,
    pub flg_status_vm: bool
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct CreateVirtualMachineRow {
    pub id : i32
}

#[derive(Deserialize, Debug)]
pub struct UpdateVirtualMachineReq {
    pub nome_vm: Option<String>, 
    pub endereco_ipv4_vm: Option<String>,
    pub flg_status_vm: Option<bool>
}


#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct BotsRow {
    pub id: i32,
    pub nome_automacao: String,
    pub flg_status_bot: bool,
    pub frequencia_execucao: Frequencia,
    pub dia_execucao: Option<String>,
    pub hora_execucao: Option<String>,
    pub intervalo_execucao: Option<i32>,
    pub tolerancia_execucao: Option<i32>,
    pub virtual_machine_id: i32
}

#[derive(Deserialize, Debug, sqlx::FromRow)]
pub struct CreateBotReq {
    pub nome_automacao: String,
    pub flg_status_bot: bool,
    pub frequencia_execucao: Frequencia,
    pub dia_execucao: Option<String>,
    pub hora_execucao: Option<String>,
    pub intervalo_execucao: Option<i32>,
    pub tolerancia_execucao: Option<i32>,
    pub virtual_machine_id: i32
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct CreateBotRow {
    pub id : i32
}

#[derive(Deserialize, Debug)]
pub struct UpdateBotReq {
    pub nome_automacao: Option<String>,
    pub flg_status_bot: Option<bool>,
    pub frequencia_execucao: Option<Frequencia>,
    pub dia_execucao: Option<String>,
    pub hora_execucao: Option<String>,
    pub intervalo_execucao: Option<i32>,
    pub tolerancia_execucao: Option<i32>,
    pub virtual_machine_id: Option<i32>
}