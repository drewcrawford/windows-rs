// Remove when upstream metadata generator supports other targets
#![cfg(all(windows, target_pointer_width = "64"))]

windows::include_bindings!();
