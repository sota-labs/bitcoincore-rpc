use reqwest::Client;
use serde_json::json;

/// The different authentication methods for the client.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Auth {
    None,
    UserPass(String, String),
}

impl Auth {
    /// Convert into the arguments that jsonrpc::Client needs.
    pub fn get_user_pass(self) -> (Option<String>, Option<String>) {
        match self {
            Auth::None => (None, None),
            Auth::UserPass(u, p) => (Some(u), Some(p)),
        }
    }
}

/// Client implements a JSON-RPC client for the Bitcoin Core daemon or compatible APIs.
#[allow(dead_code)]
#[derive(Clone)]
pub struct RpcClient {
    client: Client,
    url: String,
    user: Option<String>,
    pass: Option<String>,
}

#[allow(dead_code)]
impl RpcClient {
    pub fn new(url: String, auth: Auth) -> RpcClient {
        let (user, pass) = auth.get_user_pass();
        // Create a reqwest client
        let client = Client::new();
        RpcClient {
            client,
            url,
            user,
            pass,
        }
    }

    /// Call an `method` rpc with given `args` list
    async fn call<T: for<'a> serde::de::Deserialize<'a>>(
        &mut self,
        method: &str,
        args: &[serde_json::Value],
    ) -> anyhow::Result<T> {
        // Prepare RPC request data
        let params = args;
        let request_data = json!({
            "jsonrpc": "2.0",
            "id": "rusttest",
            "method": method,
            "params": params,
        });

        let mut req_builder = self.client.post(&self.url).header("content-type", "text/plain;");

        if let Some(user) = &self.user {
            req_builder = req_builder.basic_auth(user, self.pass.clone());
        }
        // Make the HTTP POST request
        let response = req_builder.json(&request_data).send().await?;
        // Parse the JSON response
        let json_response: T = response.json().await?;

        Ok(json_response)
    }

    // RpcApi
    /// Returns the numbers of block in the longest chain.
    // fn get_block_count(&self) -> Option<u64> {
    //     self.call("getblockcount", &[])
    // }

    async fn get_block_count(&mut self) -> Option<u64> {
        match self.call::<u64>("getblockcount", &[]).await {
            Ok(result) => Some(result),
            Err(_) => None, // You can handle the error case as needed
        }
    }
}
