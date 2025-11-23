use std::str::FromStr;

use bluer::{self, Address, Device};
use tokio;



struct BLEInterface {
    session: bluer::Session,
    adapter: bluer::Adapter,

    left_controller: Option<Device>,
    right_controller: Option<Device>
}
impl BLEInterface {

    pub async fn new() -> BLEInterface {

        let ble_session = match bluer::Session::new().await
        {
            Ok(s) => s,
            Err(_) => panic!(),
        };

        let ble_adapter = match ble_session.default_adapter().await
        {
            Ok(a) => a,
            Err(_) => panic!(),
        };


        BLEInterface
        {
            session: ble_session,
            adapter: ble_adapter,
            left_controller: None,
            right_controller: None
        }

    }

    pub async fn initialize(&self) -> Result<(), bluer::Error>
    {

        match self.adapter.set_powered(true).await
        {
            Err(_) => panic!(),
            _ => ()
        };

        let left_addr = Address::from_str("AA:BB:CC:DD:EE:01")?;
        let right_addr = Address::from_str("AA:BB:CC:DD:EE:02")?;

        let left_controller = self.adapter.device(left_addr)?;
        let right_controller = self.adapter.device(right_addr)?;

        left_controller.connect().await?;
        right_controller.connect().await?;


        Ok(())

    }

}
