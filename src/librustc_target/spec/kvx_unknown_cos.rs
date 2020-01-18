use crate::spec::{LinkerFlavor, PanicStrategy,
           Target, TargetOptions, TargetResult};

pub fn target() -> TargetResult {
    Ok(Target {
        data_layout: "e-S256-p:64:64-i1:8-i8:8-i16:16-i32:32-i64:64-v64:64-v128:128-v256:256-v512:512-v1024:1024-f16:16-f32:32-f64:64-a:0:64-m:e-n32:64".to_string(),
        llvm_target: "kvx".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "64".to_string(),
        target_c_int_width: "32".to_string(),
        target_os: "cos".to_string(),
        target_env: String::new(),
        target_vendor: "unknown".to_string(),
        arch: "kvx".to_string(),
        linker_flavor: LinkerFlavor::Gcc,

        options: TargetOptions {
	    cpu: "kv3".to_string(),
            max_atomic_width: Some(0),
            atomic_cas: false,
            panic_strategy: PanicStrategy::Abort,
            .. Default::default()
        },
    })
}
