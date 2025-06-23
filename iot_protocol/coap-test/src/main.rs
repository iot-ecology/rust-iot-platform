/*
Copyright [2025] [Zen HuiFer]

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use coap::request::RequestBuilder;
use coap::UdpCoAPClient;
use coap_lite::RequestType::Get;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let client = UdpCoAPClient::new_udp(("127.0.0.1", 5683)).await.unwrap();

    // 全部都是字符串
    let auth_data = json!({
        "username": "admin",
        "password": "admin",
        "device_id": "8"
    });

    // 将 JSON 对象转换为字符串
    let auth_data_string = auth_data.to_string();

    let request = RequestBuilder::new("/auth", Get)
        .data(Some(auth_data_string.as_str().into()))
        .build();
    let response = client.send(request).await;
    match response {
        Ok(resp) => {
            println!("resp1 {}", String::from_utf8(resp.message.payload).unwrap());
        }
        Err(err) => {
            println!("err1 {}", err);
        }
    }

    sleep(Duration::from_secs(2)).await;

    // let client1 = UdpCoAPClient::new_udp(("127.0.0.1", 5683)).await.unwrap();
    let request1 = RequestBuilder::new("/data", Get)
        .data(Some(b"aslkfj".to_vec()))
        .build();
    let result = client.send(request1).await;

    match result {
        Ok(resp) => {
            println!("resp2 {}", String::from_utf8(resp.message.payload).unwrap());
        }
        Err(err) => {
            println!("err2 {}", err);
        }
    }
}
