use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateTokenResponse {
    pub access: String,
    #[serde(rename = "access_expires")]
    pub access_expires: i32,
    pub refresh: String,
    #[serde(rename = "refresh_expires")]
    pub refresh_expires: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Institution {
    pub id: String,
    pub name: String,
    pub bic: String,
    #[serde(rename = "transaction_total_days")]
    pub transaction_total_days: String,
    pub countries: Vec<String>,
    pub logo: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndUserAgreement {
    pub id: String,
    pub created: String,
    #[serde(rename = "institution_id")]
    pub institution_id: String,
    #[serde(rename = "max_historical_days")]
    pub max_historical_days: i64,
    #[serde(rename = "access_valid_for_days")]
    pub access_valid_for_days: i64,
    #[serde(rename = "access_scope")]
    pub access_scope: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRequisitionsResponse {
    pub count: i64,
    // pub next: Value,
    // pub previous: Value,
    pub results: Vec<Requisition>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Requisition {
    pub id: String,
    pub created: String,
    pub redirect: String,
    pub status: String,
    #[serde(rename = "institution_id")]
    pub institution_id: String,
    pub agreement: String,
    pub reference: String,
    pub accounts: Vec<String>,
    #[serde(rename = "user_language")]
    pub user_language: String,
    pub link: String,
    // pub ssn: Value,
    #[serde(rename = "account_selection")]
    pub account_selection: bool,
    #[serde(rename = "redirect_immediate")]
    pub redirect_immediate: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransactionsResponse {
    pub transactions: Transactions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transactions {
    pub booked: Vec<Transaction>,
    pub pending: Vec<Transaction>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub transaction_id: String,
    pub booking_date: String,
    pub value_date: String,
    pub booking_date_time: String,
    pub value_date_time: String,
    pub transaction_amount: TransactionAmount,
    pub creditor_name: Option<String>,
    pub remittance_information_unstructured: Option<String>,
    pub proprietary_bank_transaction_code: String,
    pub internal_transaction_id: Option<String>,
    pub debtor_name: Option<String>,
    pub creditor_account: Option<CreditorAccount>,
    #[serde(default)]
    pub currency_exchange: Vec<CurrencyExchange>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionAmount {
    pub amount: String,
    pub currency: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreditorAccount {
    pub bban: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyExchange {
    pub source_currency: String,
    pub exchange_rate: String,
    pub unit_currency: String,
    pub target_currency: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListBalancesResponse {
    pub balances: Vec<Balance>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub balance_amount: BalanceAmount,
    pub balance_type: String,
    pub reference_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceAmount {
    pub amount: String,
    pub currency: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountDetailsResponse {
    pub account: Account,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub resource_id: String,
    pub iban: String,
    pub bban: String,
    pub currency: String,
    pub owner_name: String,
    pub name: String,
    pub cash_account_type: String,
    pub status: String,
    pub masked_pan: String,
}
