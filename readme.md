# Rust IoT Platform

This is a high-performance IoT development platform built with Rust, designed to support multiple protocols and provide real-time data processing capabilities. The platform supports MQTT, WebSocket (WS), TCP, and CoAP protocols, making it highly flexible for various IoT application scenarios.

## Key Features

- **High Performance**: Written in Rust, leveraging Rust's memory safety and concurrency features to deliver an efficient IoT solution.
- **Multi-Protocol Support**: Supports MQTT, WebSocket (WS), TCP, and CoAP protocols, catering to a wide range of application requirements.
- **Real-Time Data Processing**: Built-in real-time data processing mechanisms ensure fast response and efficient data transmission.
- **Modular Design**: Clearly defined modules for easy extension and maintenance.

feature list : [Feature](./feature.md)

## Architecture Diagram

Below is the architecture diagram of the platform, illustrating how the various modules work together:

![](./readme/架构图.jpg)

## Folder Structure

- **[common](common)**: Contains common utility modules for the platform, such as logging, configuration management, etc.
- **[data_processing](data_processing)**: Modules for data processing, including data parsing, transformation, and other operations.
- **[iot_protocol](iot_protocol)**: Modules for interfacing with various IoT protocols, including MQTT, WS, TCP, and CoAP.
- **[notification](notification)**: Modules for message notifications, supporting push notifications to devices or users.
- **[api](api)**: Modules providing external APIs for integrating the platform with other systems.

## Supported Protocols

- **MQTT**: Supports the standard MQTT protocol, ideal for real-time messaging applications.
- **WebSocket (WS)**: Provides real-time bidirectional communication support for web clients.
- **TCP**: A general-purpose transport protocol for device-to-device communication.
- **CoAP**: A protocol designed for low-power devices, suitable for embedded applications.



## Contributing

We welcome PRs to improve the project. Any suggestions or issues can be raised in the [Issues](https://github.com/iot-ecology/rust-iot-platform/issues) section.

## License

[Apache 2.0 License](LICENSE)

