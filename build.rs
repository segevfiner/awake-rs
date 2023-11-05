use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "bin")]
    shadow_rs::new()?;

    #[cfg(feature = "bin")]
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        winresource::WindowsResource::new()
            .set_manifest_file("keepawake.exe.manifest")
            .compile()?;
    }

    Ok(())
}
