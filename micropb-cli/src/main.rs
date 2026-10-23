mod cli;

use clap::Parser;
use cli::{Cli, ContainerOpt, EncodeDecodeOpt};
use micropb_gen::{EncodeDecode, Generator};
use std::fmt;
use std::path::Path;

fn print_warning(message: fmt::Arguments<'_>) {
    eprintln!("warning: {message}");
}

fn ignore_warning(_: fmt::Arguments<'_>) {}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let warning_callback = if cli.quiet {
        ignore_warning
    } else {
        print_warning
    };
    let mut generator = Generator::with_warning_callback(warning_callback);

    match cli.container {
        Some(ContainerOpt::Heapless) => {
            generator.use_container_heapless();
        }
        Some(ContainerOpt::HeaplessV08) => {
            generator.use_container_heapless_v0_8();
        }
        Some(ContainerOpt::Arrayvec) => {
            generator.use_container_arrayvec();
        }
        Some(ContainerOpt::Alloc) => {
            generator.use_container_alloc();
        }
        Some(ContainerOpt::Std) => {
            generator.use_container_std();
        }
        None => {}
    }

    let encode_decode = if cli.encode_only {
        EncodeDecode::EncodeOnly
    } else if cli.decode_only {
        EncodeDecode::DecodeOnly
    } else {
        match cli.encode_decode {
            EncodeDecodeOpt::Both => EncodeDecode::Both,
            EncodeDecodeOpt::EncodeOnly => EncodeDecode::EncodeOnly,
            EncodeDecodeOpt::DecodeOnly => EncodeDecode::DecodeOnly,
        }
    };
    generator.encode_decode(encode_decode);

    if let Some(value) = cli.calculate_max_size {
        generator.calculate_max_size(value);
    }
    if let Some(value) = cli.retain_enum_prefix {
        generator.retain_enum_prefix(value);
    }
    if let Some(value) = cli.format {
        generator.format(value);
    }
    if let Some(path) = cli.fdset_path {
        generator.file_descriptor_set_path(path);
    }
    if let Some(value) = cli.suffixed_package_names {
        generator.suffixed_package_names(value);
    }
    if let Some(value) = cli.single_oneof_msg_as_enum {
        generator.single_oneof_msg_as_enum(value);
    }
    if let Some(value) = cli.comments_to_docs {
        generator.comments_to_docs(value);
    }
    if let Some(value) = cli.encode_cache {
        generator.encode_cache(value);
    }
    if let Some(value) = cli.cache_extern_types {
        generator.cache_extern_types(value);
    }

    for include in cli.includes {
        let mut argument = std::ffi::OsString::from("-I");
        argument.push(include);
        generator.add_protoc_arg(argument);
    }
    for argument in cli.protoc_args {
        generator.add_protoc_arg(argument);
    }
    for mapping in cli.extern_type_paths {
        let (proto_path, rust_path) = mapping.split_once('=').ok_or_else(|| {
            format!("invalid --extern-type-path {mapping:?}: expected PROTO_PATH=RUST_PATH")
        })?;
        if proto_path.is_empty() || rust_path.is_empty() {
            return Err(format!(
                "invalid --extern-type-path {mapping:?}: both paths must be non-empty"
            )
            .into());
        }
        generator.extern_type_path(proto_path, rust_path);
    }

    for config in cli.config_files {
        let (file, package) = config
            .rsplit_once(':')
            .filter(|(_, package)| !package.is_empty())
            .map_or((config.as_str(), "."), |(file, package)| (file, package));
        generator.parse_config_file(Path::new(file), package)?;
    }

    if let Some(fdset) = cli.fdset_in {
        generator.compile_fdset_file(fdset, cli.output)?;
    } else if cli.find_config_files {
        generator.compile_protos_with_config_files(&cli.protos, cli.output)?;
    } else {
        generator.compile_protos(&cli.protos, cli.output)?;
    }

    Ok(())
}

fn main() {
    if let Err(error) = run(Cli::parse()) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
