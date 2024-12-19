let default_channel = variable::get("default_channel").to_upper();

switch default_channel {
    "STABLE" => {
        file::delete("Cargo.nightly.toml");
        file::rename("Cargo.stable.toml", "Cargo.toml");
        file::delete("src/components/counter_btn.nightly.rs");
        file::rename("src/components/counter_btn.stable.rs", "src/components/counter_btn.rs");
    }
    "NIGHTLY" => {
        file::delete("Cargo.stable.toml");
        file::rename("Cargo.nightly.toml", "Cargo.toml");
        file::delete("src/components/counter_btn.stable.rs");
        file::rename("src/components/counter_btn.nightly.rs", "src/components/counter_btn.rs");
    }
}