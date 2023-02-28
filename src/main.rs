pub mod consts;
pub mod generator;
pub mod generators;
pub mod utils;

fn main() {
    generators::casaos::casaos_package().unwrap();
    generators::casaos_ui::casaos_ui_package().unwrap();
    generators::casaos_gateway::casaos_gateway_package().unwrap();
    generators::casaos_cli::casaos_cli_package().unwrap();
    generators::casaos_user_service::casaos_user_service_package().unwrap();
    generators::casaos_app_management::casaos_app_management_package().unwrap();
    generators::casaos_message_bus::casaos_message_bus_package().unwrap();
    generators::casaos_local_storage::casaos_local_storage_package().unwrap();
}
