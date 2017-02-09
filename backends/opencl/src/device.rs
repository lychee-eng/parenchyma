use parenchyma::Processor;
use std::borrow::Cow;
use super::{cl, core};

#[derive(Clone, Debug)]
pub struct OpenCLDevice {
    pub(super) id: core::DeviceId,
    /// maximum compute units
    pub compute_units: u32,
    /// The name of the device
    pub name: String,
    /// The maximum work-group size.
    pub workgroup_size: usize,

    pub processor: Processor,
}

impl<'d> From<&'d cl::Device> for OpenCLDevice {

    fn from(cl_device: &cl::Device) -> Self {

        let id = *cl_device.as_core();
        let compute_units = {
            match cl_device.info(cl::enums::DeviceInfo::MaxComputeUnits) {
                cl::enums::DeviceInfoResult::MaxComputeUnits(n) => n,
                _ => unreachable!()
            }
        };
        let name = cl_device.name();
        let workgroup_size = cl_device.max_wg_size().unwrap();
        let processor = {
            match cl_device.info(cl::enums::DeviceInfo::Type) {
                cl::enums::DeviceInfoResult::Type(device_type) => {

                    match device_type {
                        core::DEVICE_TYPE_CPU => {
                            Processor::Cpu
                        },
                        core::DEVICE_TYPE_GPU => {
                            Processor::Gpu
                        },
                        core::DEVICE_TYPE_ACCELERATOR => {
                            Processor::Accelerator
                        },
                        core::DEVICE_TYPE_CUSTOM => {
                            Processor::Other(Cow::from(format!("{:?}", device_type)))
                        },
                        _ => {
                            unreachable!()
                        }
                    }
                },
                _ => unreachable!()
            }
        };

        OpenCLDevice {
            id: id,
            compute_units: compute_units,
            name: name,
            workgroup_size: workgroup_size,
            processor: processor,
        }
    }
}