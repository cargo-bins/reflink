use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(any(target_os = "linux", target_os = "android"))] {
        mod linux;
        pub use linux::reflink;
    } else if #[cfg(any(target_os = "macos", target_os = "ios"))] {
        mod macos;
        pub use macos::reflink;
    } else {
        use super::reflink_not_supported as reflink;
    }
}
