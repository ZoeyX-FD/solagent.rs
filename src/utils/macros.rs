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

#[doc(hidden)]
#[macro_export]
macro_rules! parameters_json_schema {
    ($($name:ident: $type:ty),* $(,)?) => {{
        let mut properties = serde_json::Map::new();
        $(
            let property = match stringify!($type) {
                "String" => json!({
                    "type": "string"
                }),
                "i32" | "i64" | "u64" | "u32" => json!({
                    "type": "number"
                }),
                "bool" => json!({
                    "type": "boolean"
                }),

                s if s.starts_with("Vec<") && s.ends_with(">") => {

                    let inner_type_str = &s[4..s.len() - 1];
                    let inner_type = match inner_type_str {
                        "String" => json!({
                            "type": "string"
                        }),
                        "i32" | "i64" | "u64" | "u32" => json!({
                            "type": "number"
                        }),
                        "bool" => json!({
                            "type": "boolean"
                        }),
                        _ => json!({
                            "type": "object"
                        }),
                    };
                    json!({
                        "type": "array",
                        "items": inner_type
                    })
                }
                _ => {
                    json!({
                        "type": "object"
                    })
                }
            };
            properties.insert(stringify!($name).to_string(), property);
        )*
        json!({
            "type": "object",
            "properties": properties,
        })
    }};
}
