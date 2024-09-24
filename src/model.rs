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
    pub status: RequisitionStatus,
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

#[derive(Default, Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum RequisitionStatus {
    #[default]
    #[serde(rename = "CR")]
    Created,
    #[serde(rename = "GC")]
    GivingConsent,
    #[serde(rename = "UA")]
    UndergoingAuthentication,
    #[serde(rename = "RJ")]
    Rejected,
    #[serde(rename = "SA")]
    SelectingAccounts,
    #[serde(rename = "GA")]
    GrantingAccess,
    #[serde(rename = "LN")]
    Linked,
    #[serde(rename = "EX")]
    Expired,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransactionsResponse {
    #[serde(default)]
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
    pub value_date: Option<String>,
    pub booking_date_time: String,
    pub value_date_time: Option<String>,
    pub transaction_amount: TransactionAmount,
    pub creditor_name: Option<String>,
    pub remittance_information_unstructured: Option<String>,
    pub proprietary_bank_transaction_code: Option<String>,
    pub internal_transaction_id: Option<String>,
    pub debtor_name: Option<String>,
    pub creditor_account: Option<CreditorAccount>,
    // TODO: this field is either an array of objects or just a single object.
    //       perhaps there is a way in serde to default to array of just 1 object?
    // pub currency_exchange: Vec<CurrencyExchange>,
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
    #[serde(default)]
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
    #[serde(default)]
    pub account: Option<Account>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub resource_id: String,
    pub iban: Option<String>,
    pub bban: Option<String>,
    pub currency: String,
    pub owner_name: Option<String>,
    pub name: Option<String>,
    pub cash_account_type: String,
    pub status: String,
    pub masked_pan: Option<String>,
}
