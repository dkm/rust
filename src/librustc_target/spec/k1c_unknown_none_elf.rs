use crate::spec::{LinkerFlavor, 
           Target, TargetOptions, TargetResult};

pub fn target() -> TargetResult {
    Ok(Target {
        data_layout: "e-m:e-p:32:32-i64:64-n32-S128".to_string(),
        llvm_target: "k1c".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "64".to_string(),
        target_c_int_width: "32".to_string(),
        target_os: "none".to_string(),
        target_env: String::new(),
        target_vendor: "unknown".to_string(),
        arch: "k1c".to_string(),
        linker_flavor: LinkerFlavor::Gcc,

        options: TargetOptions {
            .. Default::default()
        },
    })
}
