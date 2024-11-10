use cornucopia::{CodegenSettings, Error};

// This script will generate a new cornucopia file every time your schema or queries change.
// In this example, we generate the module in our project, but
// we could also generate it elsewhere and embed the generated
// file with a `include_str` statement in your project.
fn main() -> Result<(), Error> {
    let queries_path = "src/app/adapter/outbound/for_storage/pg/query";
    let schema_files: Vec<String> =
        vec!["src/app/adapter/outbound/for_storage/pg/schema.sql".to_string()];
    let destination = "src/app/adapter/outbound/for_storage/pg/cornucopia.rs";
    let settings = CodegenSettings {
        is_async: true,
        derive_ser: false,
    };

    println!("cargo:rerun-if-changed={queries_path}");
    for schema_file in &schema_files {
        println!("cargo:rerun-if-changed={schema_file}");
    }
    let res = cornucopia::generate_managed(
        queries_path,
        schema_files,
        Some(destination),
        false,
        settings,
    );
    match res {
        Ok(_) => {
            println!("Generated Cornucopia code");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
