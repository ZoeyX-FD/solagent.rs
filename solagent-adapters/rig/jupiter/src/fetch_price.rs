// Copyright 2025 zTgx
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde::{Deserialize, Serialize};
use solagent_core::rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use solagent_parameters::parameters;
use solagent_plugin_jupiter::fetch_price;

#[derive(Debug, Deserialize)]
pub struct FetchPriceArgs {
    token_address: String,
}

#[derive(Deserialize, Serialize)]
pub struct FetchPriceOutput {
    pub price: String,
}

#[derive(Debug, thiserror::Error)]
#[error("FetchPrice error")]
pub struct FetchPriceError;

#[derive(Default)]
pub struct FetchPrice;
impl FetchPrice {
    pub fn new() -> Self {
        FetchPrice {}
    }
}

impl Tool for FetchPrice {
    const NAME: &'static str = "fetch_price";

    type Error = FetchPriceError;
    type Args = FetchPriceArgs;
    type Output = FetchPriceOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "fetch_price".to_string(),
            description: r#"
            Fetch the current price of a Solana token in USDC using Jupiter API.

            input: {
                token_address: "So11111111111111111111111111111111111111112",
            },

            "#
            .to_string(),
            parameters: parameters!(
                token_address: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let token_address = args.token_address;
        let price = fetch_price(&token_address).await.expect("fetch_price");

        Ok(FetchPriceOutput { price })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for FetchPrice {
    type InitError = InitError;
    type Context = ();
    type State = ();

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(FetchPrice {})
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Fetch the current price of a Solana token in USDC using Jupiter API.".into()]
    }

    fn context(&self) -> Self::Context {}
}
