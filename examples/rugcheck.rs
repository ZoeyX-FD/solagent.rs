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

use solagent::SolAgent;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let mint = "84VUXykQjNvPDm88oT5FRucXeNcrwdQGottJKjkAoqd1".into();

    let agent = Arc::new(SolAgent::new("", "", "openai_api_key"));
    let check = agent.fetch_summary_report(mint).await.unwrap();
    println!("Token check: {:?}", check);
}
