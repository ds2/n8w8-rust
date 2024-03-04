// Copyright (C) 2024 Dirk Strauss
//
// This file is part of Nachtwacht.
//
// Nachtwacht is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Nachtwacht is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Copyright (C) 2023 Dirk Strauss
//
// This file is part of Nachtwacht.
//
// Nachtwacht is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Nachtwacht is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use tokio_test::block_on;
//use futures::executor::block_on;
use nachtwacht_core::longhorn::{get_node_by_id, get_nodes};
use url::Url;

#[test_log::test]
#[ignore]
fn test_get_lh_nodes() {
    let lh_url = Url::parse("http://localhost:11080/v1").unwrap();
    let result = block_on(get_nodes(lh_url));
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.len() > 0);
    assert!(result.get(0).unwrap().disks.len() > 0);
    println!("Result: {:?}", result);
}

#[test_log::test]
#[ignore]
fn test_get_lh_node() {
    let lh_url = Url::parse("http://localhost:11080/v1").unwrap();
    let result = block_on(get_node_by_id(lh_url, "du131e"));
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.disks.len() > 0);
    println!("Result: {:?}", result);
}
