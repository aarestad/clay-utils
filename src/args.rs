use clay_core::Context;
use ocl;
use std::ops::Deref;

pub fn parse<T: Deref<Target = str>, I: Iterator<Item = T>>(args: I) -> clay_core::Result<Context> {
    let args = args.collect::<Vec<_>>();
    let platform = if args.len() > 1 {
        let platform_list = ocl::Platform::list();
        println!("{:?}", platform_list);
        let index = args[1].parse::<usize>().map_err(|e| e.to_string())?;
        assert!(platform_list.len() > index);
        platform_list[index]
    } else {
        ocl::Platform::default()
    };
    let device = ocl::Device::first(platform)?;
    let context = Context::new(platform, device)?;
    Ok(context)
}
