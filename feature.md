# Feature List

-   **Device Management**: Easily manage all device data and information. The device group feature allows devices to be grouped based on different scenarios, improving the efficiency of device management.

-   **Personnel Management**: Provides comprehensive role, department, and permission management functions to ensure that each user has access only within their authorized scope, enhancing system security.

-   **Lifecycle Management**: Involves the entire process from production to delivery of devices, ensuring smooth execution at each stage and enabling tracking of the device's status and progress.

-   **Protocol Management**: Supports multiple commonly used IoT protocols, ensuring efficient and secure communication between devices and the platform through detailed management and configuration.

-   **Data Management**: Provides flexible data processing and alert configuration capabilities. Users can set various signal processing and alert rules based on their needs, ensuring real-time system response.

-   **Notification Management**: Through Feishu and DingTalk bots, the platform can send important notifications and alerts in a timely manner, ensuring administrators can quickly respond to and handle abnormal situations.

-   **Forwarding Management**: Supports data forwarding to various mainstream databases, ensuring efficient storage and processing of device data to meet different storage and query requirements.

## Device Management
- **Device Details**: View and manage detailed information of individual devices, including device configurations, status, logs, and usage data.
- **Device Group**: Organize devices into groups based on functionality, location, or other custom criteria to enable bulk operations and centralized management.

## Personnel Management
- **User List**: Manage user information within the platform, including role assignment and access control.
- **Department List**: Organize users by department for better management and permission allocation.
- **Role List**: Define different roles, assign corresponding permissions to each role, and ensure users can only access platform features within their role's scope.
- **Info List**: Manage additional information related to users, devices, and events, providing detailed records and search functionality.

## Lifecycle Management
- **Product List**: Manage the lifecycle of IoT products, including creation, updates, and status tracking.
- **IoT Card Management**: Manage the allocation, status updates, and usage of IoT cards, ensuring stable device communication.
- **Device Production Management**: Track the device production process to ensure production progress and quality control.
- **Device Shipping Management**: Manage the shipping, distribution, and delivery status of devices, ensuring timely arrival.

## Protocol Management
- **MQTT Protocol**:
  - **MQTT Client Management**: Configure and manage MQTT clients, including device connections and message transmissions.
  - **MQTT Client Service Node Management**: Manage MQTT service nodes, controlling the connections and message routing between clients and servers.
  
- **HTTP Protocol**:
  - **HTTP Processor Management**: Manage HTTP request handling, including different API interfaces and request types.
  - **HTTP Server Management**: Configure and manage the HTTP server to handle requests from external systems.
  
- **TCP Protocol**:
  - **TCP Processor Management**: Manage TCP connections and data flow, including connection pools and data parsing.
  - **TCP Server Management**: Configure and manage the TCP server to receive and send device data.
  
- **CoAP Protocol**:
  - **CoAP Processor Management**: Configure and manage data processors for CoAP protocol, suitable for low-power devices.
  - **CoAP Server Management**: Manage the CoAP server, supporting efficient communication between devices.
  
- **WebSocket Protocol**:
  - **WebSocket Processor Management**: Manage WebSocket connection processors to support real-time, bidirectional communication.
  - **WebSocket Server Management**: Configure the WebSocket server to provide real-time data transmission services to clients.

## Data Management
- **Signal Management**: Manage signals from devices or sensors, including signal types, sources, and handling methods.
- **Signal Alert Configuration**: Set and manage alert rules for signals, triggering alerts when signals exceed predefined thresholds.
- **Computation Rules**: Define calculation logic and rules for processing and analyzing data from devices.
- **Computation Parameters**: Manage the parameters required for data processing, ensuring accuracy during the computation process.
- **Script Alerting**: Automate alerting using scripts, supporting custom alert behavior based on script logic.
- **Script Parameters**: Define and manage parameters used in scripts to allow flexible adjustments under different conditions.

## Notification Management
- **Feishu Bot**: Send real-time notifications via Feishu Bot, supporting customized notification content and triggering conditions.
- **DingTalk Bot**: Integrate DingTalk Bot to send messages and alerts, ensuring timely communication.

## Forwarding Management
- **Cassandra**: Configure data forwarding to Cassandra for large-scale data storage and fast queries.
- **ClickHouse**: Forward data to ClickHouse, suitable for real-time data analysis and large-scale data storage.
- **InfluxDB 2**: Support forwarding time-series data to InfluxDB 2 for efficient time-series data queries and storage.
- **MongoDB**: Forward data to MongoDB, supporting JSON-formatted data storage and flexible querying.
- **MySQL**: Forward data to MySQL, suitable for relational data storage, management, and querying.
