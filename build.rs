#[cfg(windows)]
fn build_windows() {
    let file = "src/platform/windows.cc";
    let file2 = "src/platform/windows_delete_test_cert.cc";
    cc::Build::new().file(file).file(file2).compile("windows");
    println!("cargo:rustc-link-lib=WtsApi32");
    println!("cargo:rerun-if-changed={}", file);
    println!("cargo:rerun-if-changed={}", file2);
}

#[cfg(target_os = "macos")]
fn build_mac() {
    let file = "src/platform/macos.mm";
    let mut b = cc::Build::new();
    if let Ok(os_version::OsVersion::MacOS(v)) = os_version::detect() {
        let v = v.version;
        if v.contains("10.14") {
            b.flag("-DNO_InputMonitoringAuthStatus=1");
        }
    }
    b.file(file).compile("macos");
    println!("cargo:rerun-if-changed={}", file);
}

fn install_android_deps() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os != "android" {
        return;
    }
    let mut target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    target_arch = match target_arch.as_str() {
        "x86_64" => "x64".to_owned(),
        "x86" => "x86".to_owned(),
        "aarch64" => "arm64".to_owned(),
        _ => "arm".to_owned(),
    };
    let target = format!("{}-android", target_arch);
    let vcpkg_root = std::env::var("VCPKG_ROOT").unwrap();
    let mut path: std::path::PathBuf = if let Ok(v) = std::env::var("VCPKG_INSTALLED_ROOT") {
        v.into()
    } else {
        let mut p: std::path::PathBuf = vcpkg_root.into();
        p.push("installed");
        p
    };
    path.push(&target);
    println!("cargo:rustc-link-search={}", path.join("lib").to_str().unwrap());
    println!("cargo:rustc-link-lib=ndk_compat");
    println!("cargo:rustc-link-lib=oboe");
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=OpenSLES");
}

#[cfg(windows)]
fn embed_rc() {
    embed_resource::compile("hasner_icon.rc", embed_resource::NONE);
}

fn main() {
    hbb_common::gen_version();
    install_android_deps();

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    #[cfg(windows)]
    {
        build_windows();
        embed_rc();
    }

    #[cfg(target_os = "macos")]
    {
        build_mac();
        println!("cargo:rustc-link-arg-bins=hasner_icon.res");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
