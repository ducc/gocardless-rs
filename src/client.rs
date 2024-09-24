use secrecy::{ExposeSecret, SecretString};
use serde_json::json;

use crate::model::*;

const URL_CREATE_TOKEN: &str = "https://bankaccountdata.gocardless.com/api/v2/token/new/";
const URL_GET_INSTITUTIONS: &str =
    "https://bankaccountdata.gocardless.com/api/v2/institutions/?country=gb"; // TODO: make country a variable
const URL_CREATE_END_USER_AGREEMENT: &str =
    "https://bankaccountdata.gocardless.com/api/v2/agreements/enduser/";
const URL_REQUISITIONS: &str = "https://bankaccountdata.gocardless.com/api/v2/requisitions/";

/// `Client` is a public struct that represents a client for making requests to the API.
///
/// Fields:
/// * `req_client`: A `reqwest::Client` instance used for making HTTP requests.
/// * `secret_id`: A `SecretString` that represents the client's secret ID.
/// * `secret_key`: A `SecretString` that represents the client's secret key.
/// * `created_token`: An `Option<CreateTokenResponse>` that represents the token created by the client. It is `None` if no token has been created yet.
///
/// The `Client` struct is used to interact with the API. It uses the `reqwest` crate for making HTTP requests and the `secrecy` crate for handling secret strings.
/// The `secret_id` and `secret_key` are used for authentication with the API.
/// The `created_token` field is used to store the token received from the API after successful authentication.
pub struct Client {
    req_client: reqwest::Client,
    secret_id: SecretString,
    secret_key: SecretString,
    created_token: Option<CreateTokenResponse>,
}

impl Client {
    /// `new` is an associated function that creates a new instance of the `Client` struct.
    ///
    /// # Arguments
    ///
    /// * `secret_id`: An implementor of the `Into<SecretString>` trait. This is converted into a `SecretString` that represents the client's secret ID.
    /// * `secret_key`: An implementor of the `Into<SecretString>` trait. This is converted into a `SecretString` that represents the client's secret key.
    ///
    /// # Returns
    ///
    /// This function returns a `Client` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let secret_id = "my_secret_id".to_string();
    /// let secret_key = "my_secret_key".to_string();
    /// let client = Client::new(secret_id, secret_key);
    /// ```
    ///
    /// # Async
    ///
    /// This function is async and should be awaited.
    pub async fn new(
        secret_id: impl Into<SecretString>,
        secret_key: impl Into<SecretString>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let req_client = reqwest::Client::new();

        let mut c = Client {
            req_client,
            secret_id: secret_id.into(),
            secret_key: secret_key.into(),
            created_token: None,
        };

        let created_token = c.create_token().await?;
        c.created_token = Some(created_token);

        Ok(c)
    }

    /// `create_token` is an async method that sends a POST request to the `URL_CREATE_TOKEN` endpoint to create a new token.
    ///
    /// # Returns
    ///
    /// This method returns a `Result` that is either a `CreateTokenResponse` on success or a `Box<dyn std::error::Error>` on failure.
    ///
    /// # Async
    ///
    /// This method is async and should be awaited.
    ///
    /// # Examples
    ///
    /// ```
    /// let secret_id = "my_secret_id".to_string();
    /// let secret_key = "my_secret_key".to_string();
    /// let mut client = Client::new(secret_id, secret_key).await?;
    /// let create_token_response = client.create_token().await?;
    /// ```
    ///
    /// This method is typically called within the `Client::new` method to automatically create a token when a new `Client` is created.
    pub async fn create_token(&self) -> Result<CreateTokenResponse, Box<dyn std::error::Error>> {
        let response: CreateTokenResponse = self
            .req_client
            .post(URL_CREATE_TOKEN)
            .body(
                json!({
                    "secret_id": self.secret_id.expose_secret(),
                    "secret_key": self.secret_key.expose_secret(),
                })
                .to_string(),
            )
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// `get_institutions` is an async method that sends a GET request to the `URL_GET_INSTITUTIONS` endpoint to retrieve a list of institutions.
    ///
    /// # Returns
    ///
    /// This method returns a `Result` that is either a `Vec<Institution>` on success or a `Box<dyn std::error::Error>` on failure.
    ///
    /// # Async
    ///
    /// This method is async and should be awaited.
    ///
    /// # Examples
    ///
    /// ```
    /// let secret_id = "my_secret_id".to_string();
    /// let secret_key = "my_secret_key".to_string();
    /// let mut client = Client::new(secret_id, secret_key).await?;
    /// let institutions = client.get_institutions().await?;
    /// ```
    ///
    /// This method requires that a token has been created and stored in the `created_token` field of the `Client` struct. If no token has been created, this method will return an error.
    pub async fn get_institutions(&self) -> Result<Vec<Institution>, Box<dyn std::error::Error>> {
        let access_token = self.created_token.clone().unwrap().access;

        let response: Vec<Institution> = self
            .req_client
            .get(URL_GET_INSTITUTIONS)
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// `create_end_user_agreement` is an async method that sends a POST request to the `URL_CREATE_END_USER_AGREEMENT` endpoint to create an end user agreement.
    ///
    /// # Arguments
    ///
    /// * `institution_id`: A reference to a string that represents the ID of the institution for which the end user agreement is being created.
    ///
    /// # Returns
    ///
    /// This method returns a `Result` that is either an `EndUserAgreement` on success or a `Box<dyn std::error::Error>` on failure.
    ///
    /// # Async
    ///
    /// This method is async and should be awaited.
    ///
    /// # Examples
    ///
    /// ```
    /// let secret_id = "my_secret_id".to_string();
    /// let secret_key = "my_secret_key".to_string();
    /// let mut client = Client::new(secret_id, secret_key).await?;
    /// let institution_id = "institution_id".to_string();
    /// let end_user_agreement = client.create_end_user_agreement(&institution_id).await?;
    /// ```
    ///
    /// This method requires that a token has been created and stored in the `created_token` field of the `Client` struct. If no token has been created, this method will return an error.
    pub async fn create_end_user_agreement(
        &self,
        institution_id: &str,
        max_historical_days: i32,
    ) -> Result<EndUserAgreement, Box<dyn std::error::Error>> {
        let access_token = self.created_token.clone().unwrap().access;

        let response = self
            .req_client
            .post(URL_CREATE_END_USER_AGREEMENT)
            .body(
                json!({
                    "institution_id": institution_id,
                    "max_historical_days": max_historical_days,
                    "access_valid_for_days": "30",
                    "access_scope": [
                        "balances",
                        "details",
                        "transactions"
                    ]
                })
                .to_string(),
            )
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .text()
            .await?;

        let agreement: EndUserAgreement = serde_json::from_str(&response)?;

        Ok(agreement)
    }

    /// `list_requisitions` is an async method that sends a GET request to the `URL_REQUISITIONS` endpoint to retrieve a list of requisitions.
    ///
    /// # Returns
    ///
    /// This method returns a `Result` that is either a `ListRequisitionsResponse` on success or a `Box<dyn std::error::Error>` on failure.
    ///
    /// # Async
    ///
    /// This method is async and should be awaited.
    ///
    /// # Examples
    ///
    /// ```
    /// let secret_id = "my_secret_id".to_string();
    /// let secret_key = "my_secret_key".to_string();
    /// let mut client = Client::new(secret_id, secret_key).await?;
    /// let requisitions = client.list_requisitions().await?;
    /// ```
    ///
    /// This method requires that a token has been created and stored in the `created_token` field of the `Client` struct. If no token has been created, this method will return an error.
    pub async fn list_requisitions(
        &self,
    ) -> Result<ListRequisitionsResponse, Box<dyn std::error::Error>> {
        let access_token = self.created_token.clone().unwrap().access;

        let response: ListRequisitionsResponse = self
            .req_client
            .get(URL_REQUISITIONS)
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// `create_requisition` is an async method that sends a POST request to the `URL_REQUISITIONS` endpoint to create a new requisition.
    ///
    /// # Arguments
    ///
    /// * `redirect`: A reference to a string that represents the URL to which the user will be redirected after completing the requisition.
    /// * `institution_id`: A reference to a string that represents the ID of the institution for which the requisition is being created.
    /// * `agreement_id`: A reference to a string that represents the ID of the end user agreement associated with the requisition.
    /// * `reference`: A reference to a string that represents a unique reference for the requisition.
    ///
    /// # Returns
    ///
    /// This method returns a `Result` that is either a `Requisition` on success or a `Box<dyn std::error::Error>` on failure.
    ///
    /// # Async
    ///
    /// This method is async and should be awaited.
    ///
    /// # Examples
    ///
    /// ```
    /// let secret_id = "my_secret_id".to_string();
    /// let secret_key = "my_secret_key".to_string();
    /// let mut client = Client::new(secret_id, secret_key).await?;
    /// let redirect = "http://localhost:3000/callback".to_string();
    /// let institution_id = "institution_id".to_string();
    /// let agreement_id = "agreement_id".to_string();
    /// let reference = "reference".to_string();
    /// let requisition = client.create_requisition(&redirect, &institution_id, Some(&agreement_id), Some(&reference)).await?;
    /// ```
    ///
    /// This method requires that a token has been created and stored in the `created_token` field of the `Client` struct. If no token has been created, this method will return an error.
    pub async fn create_requisition(
        &self,
        redirect: &str,
        institution_id: &str,
        agreement_id: Option<&str>,
        reference: Option<&str>,
    ) -> Result<Requisition, Box<dyn std::error::Error>> {
        let access_token = self.created_token.clone().unwrap().access;

        let mut request = json!({
            "redirect": redirect,
            "institution_id": institution_id,
            "user_language": "EN" // TODO: configurable
        });
        if let Some(reference) = reference {
            request["reference"] = json!(reference);
        }
        if let Some(agreement_id) = agreement_id {
            request["agreement"] = json!(agreement_id);
        }

        let response: Requisition = self
            .req_client
            .post(URL_REQUISITIONS)
            .body(request.to_string())
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// `list_transactions` is an async method that sends a GET request to the `https://bankaccountdata.gocardless.com/api/v2/accounts/{account_id}/transactions` endpoint to retrieve a list of transactions for a specific account.
    ///
    /// # Arguments
    ///
    /// * `account_id`: A reference to a string that represents the ID of the account for which the transactions are being retrieved.
    ///
    /// # Returns
    ///
    /// This method returns a `Result` that is either a `ListTransactionsResponse` on success or a `Box<dyn std::error::Error>` on failure.
    ///
    /// # Async
    ///
    /// This method is async and should be awaited.
    ///
    /// # Examples
    ///
    /// ```
    /// let secret_id = "my_secret_id".to_string();
    /// let secret_key = "my_secret_key".to_string();
    /// let mut client = Client::new(secret_id, secret_key).await?;
    /// let account_id = "account_id".to_string();
    /// let transactions = client.list_transactions(&account_id).await?;
    /// ```
    ///
    /// This method requires that a token has been created and stored in the `created_token` field of the `Client` struct. If no token has been created, this method will return an error.
    pub async fn list_transactions(
        &self,
        account_id: &str,
    ) -> Result<ListTransactionsResponse, Box<dyn std::error::Error>> {
        let access_token = self.created_token.clone().unwrap().access;

        let response = self
            .req_client
            .get(format!(
                "https://bankaccountdata.gocardless.com/api/v2/accounts/{}/transactions",
                account_id
            ))
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .text()
            .await?;

        let parsed: ListTransactionsResponse = serde_json::from_str(&response)?;

        Ok(parsed)
    }

    /// `list_balances` is an async method that sends a GET request to the `https://bankaccountdata.gocardless.com/api/v2/accounts/{account_id}/balances` endpoint to retrieve a list of balances for a specific account.
    ///
    /// # Arguments
    ///
    /// * `account_id`: A reference to a string that represents the ID of the account for which the balances are being retrieved.
    ///
    /// # Returns
    ///
    /// This method returns a `Result` that is either a `ListBalancesResponse` on success or a `Box<dyn std::error::Error>` on failure.
    ///
    /// # Async
    ///
    /// This method is async and should be awaited.
    ///
    /// # Examples
    ///
    /// ```
    /// let secret_id = "my_secret_id".to_string();
    /// let secret_key = "my_secret_key".to_string();
    /// let mut client = Client::new(secret_id, secret_key).await?;
    /// let account_id = "account_id".to_string();
    /// let balances = client.list_balances(&account_id).await?;
    /// ```
    ///
    /// This method requires that a token has been created and stored in the `created_token` field of the `Client` struct. If no token has been created, this method will return an error.
    pub async fn list_balances(
        &self,
        account_id: &str,
    ) -> Result<ListBalancesResponse, Box<dyn std::error::Error>> {
        let access_token = self.created_token.clone().unwrap().access;

        let response: ListBalancesResponse = self
            .req_client
            .get(format!(
                "https://bankaccountdata.gocardless.com/api/v2/accounts/{}/balances",
                account_id
            ))
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// `get_account_details` is an async method that sends a GET request to the `https://bankaccountdata.gocardless.com/api/v2/accounts/{account_id}/details` endpoint to retrieve the details of a specific account.
    ///
    /// # Arguments
    ///
    /// * `account_id`: A reference to a string that represents the ID of the account for which the details are being retrieved.
    ///
    /// # Returns
    ///
    /// This method returns a `Result` that is either an `AccountDetailsResponse` on success or a `Box<dyn std::error::Error>` on failure.
    ///
    /// # Async
    ///
    /// This method is async and should be awaited.
    ///
    /// # Examples
    ///
    /// ```
    /// let secret_id = "my_secret_id".to_string();
    /// let secret_key = "my_secret_key".to_string();
    /// let mut client = Client::new(secret_id, secret_key).await?;
    /// let account_id = "account_id".to_string();
    /// let account_details = client.get_account_details(&account_id).await?;
    /// ```
    ///
    /// This method requires that a token has been created and stored in the `created_token` field of the `Client` struct. If no token has been created, this method will return an error.
    pub async fn get_account_details(
        &self,
        account_id: &str,
    ) -> Result<AccountDetailsResponse, Box<dyn std::error::Error>> {
        let access_token = self.created_token.clone().unwrap().access;

        let response: AccountDetailsResponse = self
            .req_client
            .get(format!(
                "https://bankaccountdata.gocardless.com/api/v2/accounts/{}/details",
                account_id
            ))
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}
