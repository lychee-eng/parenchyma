#![cfg(test)]

extern crate parenchyma;
extern crate parenchyma_opencl;

mod opencl_framework_spec {
    use parenchyma::Framework;
    use parenchyma_opencl::OpenCL;

    #[test]
    fn it_can_init_opencl() {
        let opencl_platform = OpenCL::new();
        assert!(opencl_platform.available_platforms.len() > 0);
    }
}

