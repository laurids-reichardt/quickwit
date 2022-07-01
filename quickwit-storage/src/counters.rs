// Copyright (C) 2022 Quickwit, Inc.
//
// Quickwit is offered under the AGPL v3.0 and as commercial software.
// For commercial licensing, contact us at hello@quickwit.io.
//
// AGPL:
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use once_cell::sync::Lazy;
use quickwit_common::metrics::{new_counter, new_gauge, IntCounter, IntGauge};

use crate::Cache;

pub struct StorageCounters {
    pub fast_field_cache: CacheCounters,
    pub inflight_cache: CacheCounters,
    pub split_footer_cache: CacheCounters,
}

#[derive(Clone)]
pub struct CacheCounters {
    pub component_name: String,
    pub num_items: IntGauge,
    pub num_bytes: IntGauge,
    pub num_cache_hits_items: IntCounter,
    pub num_cache_hits_bytes: IntCounter,
    pub num_cache_miss_items: IntCounter,
}

impl CacheCounters {
    fn for_component(component_name: &str) -> Self {
        let prefix = format!("cache:{component_name}");
        CacheCounters {
            component_name: component_name.to_string(),
            num_items: new_gauge(
                &format!("{prefix}:num_items"),
                "Number of {component_name} items in cache",
            ),
            num_bytes: new_gauge(
                &format!("{prefix}:num_bytes"),
                "Number of {component_name} bytes in cache",
            ),
            num_cache_hits_items: new_counter(
                &format!("{prefix}:cache_hits_items"),
                "Number of {component_name} cache hits in items",
            ),
            num_cache_hits_bytes: new_counter(
                &format!("{prefix}:cache_hits_bytes"),
                "Number of {component_name} cache hits in bytes",
            ),
            num_cache_miss_items: new_counter(
                &format!("{prefix}:cache_miss_items"),
                "Number of {component_name} cache miss in items",
            ),
        }
    }
}

impl Default for StorageCounters {
    fn default() -> Self {
        StorageCounters {
            fast_field_cache: CacheCounters::for_component("fastfields"),
            inflight_cache: CacheCounters::for_component("inflight"),
            split_footer_cache: CacheCounters::for_component("splitfooter"),
        }
    }
}

pub static COUNTERS: Lazy<StorageCounters> = Lazy::new(StorageCounters::default);
