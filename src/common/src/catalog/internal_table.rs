// Copyright 2022 Singularity Data
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

use std::sync::LazyLock;

use regex::Regex;

pub const RW_TABLE_FUNCTION_NAME: &str = "rw_table";

pub fn generate_internal_table_name_with_type(
    mview_name: &str,
    fragment_id: u32,
    table_id: u32,
    table_type: &str,
) -> String {
    format!(
        "__internal_{}_{}_{}_{}",
        mview_name,
        fragment_id,
        table_type.to_lowercase(),
        table_id
    )
}

pub fn valid_table_name(table_name: &str) -> bool {
    static INTERNAL_TABLE_NAME: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"__internal_.*_\d+").unwrap());
    !INTERNAL_TABLE_NAME.is_match(table_name)
}
