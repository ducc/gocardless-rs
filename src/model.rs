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
    /// The account id of the given account in the financial institution
    pub resource_id: String,
    pub iban: Option<String>,
    /// This data element is used for payment accounts which have no IBAN
    pub bban: Option<String>,
    /// The BIC associated to the account
    pub bic: Option<String>,
    /// An alias to a payment account via a registered mobile phone number
    pub msisdn: Option<String>,
    /// Account currency
    pub currency: String,
    /// Name of the legal account owner. If there is more than one owner, then e.g. two names might be noted here. For a corporate account, the corporate name is used for this attribute
    pub owner_name: Option<String>,
    /// Address of the legal account owner
    pub owner_address_unstructured: Option<String>,
    /// Name of the account, as assigned by the financial institution
    pub name: Option<String>,
    /// Name of the account as defined by the end user within online channels
    pub display_name: Option<String>,
    /// Specifications that might be provided by the financial institution - characteristics of the account - characteristics of the relevant card
    pub details: Option<String>,
    /// Product Name of the Bank for this account, proprietary definition
    pub product: Option<String>,
    /// ExternalCashAccountType1Code from ISO 20022
    pub cash_account_type: Option<String>,
    /// Account status, if this field is None, then the account is available in the sense of the specification
    pub status: Option<AccountStatus>,
    /// This data attribute is a field, where an financial institution can name a cash account associated to pending card transactions
    pub linked_accounts: Option<String>,
    /// Specifies the usage of the account
    pub usage: Option<AccountUsage>,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccountStatus {
    #[default]
    #[serde(rename = "enabled")]
    /// Account is available
    Enabled,
    #[serde(rename = "deleted")]
    /// Account is terminated
    Deleted,
    #[serde(rename = "blocked")]
    /// Account is blocked e.g. for legal reasons
    Blocked,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccountUsage {
    #[default]
    #[serde(rename = "PRIV")]
    /// Private personal account
    Private,
    #[serde(rename = "ORGA")]
    /// Professional account
    Professional,
}
