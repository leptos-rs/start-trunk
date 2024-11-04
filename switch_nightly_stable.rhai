let default_channel = variable::get("default_channel").to_upper();

switch default_channel {
    "STABLE" => {
        file::delete("Cargo.nightly.toml");
        file::rename("Cargo.stable.toml", "Cargo.toml");
    }
    "NIGHTLY" => {
        file::delete("Cargo.stable.toml");
        file::rename("Cargo.nightly.toml", "Cargo.toml");
    }
}