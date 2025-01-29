use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct BonusesTypeInfo {
    pub id: i16,
    pub type_alias: String,
    pub type_title: String,
}

#[derive(Deserialize, Debug)]
pub struct BonusesTypesRsp {
    pub data: Vec<BonusesTypeInfo>,
}

pub enum BonusesType {
    Accrual,
    WriteOff,
    Custom(i16),
}

impl BonusesType {
    pub fn value(&self) -> i16 {
        match self {
            BonusesType::Accrual => 1,
            BonusesType::WriteOff => 2,
            BonusesType::Custom(value) => *value,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct BonusesHistoryEntryData {
    pub work_station_id: String,
    pub type_alias: String,
    pub type_title: String,
    pub date: i64,
    pub source: String,
    pub check_number: String,
    pub receipt_id: String,
    pub shop_id: String,
    #[serde(rename = "descriprion")]
    pub description: String,
    pub bonus: String,
}

#[derive(Deserialize, Debug)]
pub struct  BonusesHistoryEntry {
    pub month: i64,
    pub total_accrual: String,
    pub total_writeoff: String,
    pub data: Vec<BonusesHistoryEntryData>,
}

#[derive(Deserialize, Debug)]
pub struct  BonusesHistoryRsp {
    pub total_count: i64,
    pub page: u16,
    pub limit: u16,
    pub data: Vec<BonusesHistoryEntry>,
}
