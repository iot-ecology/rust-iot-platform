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

use quick_js::Context;
#[cfg(test)]
mod tests {
    use crate::js_test::test_js;

    #[test]
    fn test_js_functions() {
        test_js();
    }
}
pub async fn test_js() {
    let context = Context::new().unwrap();

    let js_code = r#"
        function main(nc) {
            var dataRows = [
                { "Name": "Temperature", "Value": "23" },
                { "Name": "Humidity", "Value": "30" },
                { "Name": "A", "Value": nc }
            ];
            var result = {
                "Time": Math.floor(Date.now() / 1000),
                "DataRows": dataRows,
                "IdentificationCode": "102",
                "DeviceUid": "102",
                "Nc": nc
            };
            return [result];
        }
    "#;

    context.eval(js_code).unwrap();

    let nc_value = "42";

    let value = context.call_function("main", [nc_value]).unwrap();

    let js_code_2 = r#"
        function main2(data) {
            return JSON.stringify(data);
        }"#;
    context.eval(js_code_2).unwrap();
    let value2 = context.call_function("main2", [value]).unwrap();
    println!("{:?}", value2.as_str().unwrap_or(""));
}
