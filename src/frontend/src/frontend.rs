// Copyright 2023 Greptime Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use meta_client::MetaClientOptions;
use serde::{Deserialize, Serialize};
use servers::http::HttpOptions;
use servers::Mode;

use crate::grpc::GrpcOptions;
use crate::influxdb::InfluxdbOptions;
use crate::mysql::MysqlOptions;
use crate::opentsdb::OpentsdbOptions;
use crate::postgres::PostgresOptions;
use crate::prom::PromOptions;
use crate::prometheus::PrometheusOptions;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct FrontendOptions {
    pub mode: Mode,
    pub http_options: Option<HttpOptions>,
    pub grpc_options: Option<GrpcOptions>,
    pub mysql_options: Option<MysqlOptions>,
    pub postgres_options: Option<PostgresOptions>,
    pub opentsdb_options: Option<OpentsdbOptions>,
    pub influxdb_options: Option<InfluxdbOptions>,
    pub prometheus_options: Option<PrometheusOptions>,
    pub prom_options: Option<PromOptions>,
    pub meta_client_options: Option<MetaClientOptions>,
}

impl Default for FrontendOptions {
    fn default() -> Self {
        Self {
            mode: Mode::Standalone,
            http_options: Some(HttpOptions::default()),
            grpc_options: Some(GrpcOptions::default()),
            mysql_options: Some(MysqlOptions::default()),
            postgres_options: Some(PostgresOptions::default()),
            opentsdb_options: Some(OpentsdbOptions::default()),
            influxdb_options: Some(InfluxdbOptions::default()),
            prometheus_options: Some(PrometheusOptions::default()),
            prom_options: Some(PromOptions::default()),
            meta_client_options: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toml() {
        let opts = FrontendOptions::default();
        let toml_string = toml::to_string(&opts).unwrap();
        let _parsed: FrontendOptions = toml::from_str(&toml_string).unwrap();
    }
}
