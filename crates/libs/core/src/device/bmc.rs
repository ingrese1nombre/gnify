use gnify::{source::BMC, Record};

use super::{Device, DeviceStatus, DeviceView};

mod postgres;

pub struct ListDevices {
    pub status: Option<DeviceStatus>
}

impl BMC for ListDevices {
    type Output = Vec<DeviceView>;
}

pub struct WriteDevice {
    pub record: Record<Device>
}