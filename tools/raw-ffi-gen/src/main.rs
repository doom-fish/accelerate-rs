use anyhow::{anyhow, Context, Result};
use bindgen::{Builder, EnumVariation};
use regex::{escape as regex_escape, Regex};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Debug)]
struct ModuleConfig {
    output: &'static str,
    existing_module: Option<&'static str>,
    headers: &'static [&'static str],
    apple_cf_reexports: &'static [&'static str],
}

#[derive(Debug, Default)]
struct Symbols {
    functions: BTreeSet<String>,
    types: BTreeSet<String>,
    vars: BTreeSet<String>,
}

const C_VECTOR_TYPES: &[&str] = &[
    "vUInt8", "vSInt8", "vUInt16", "vSInt16", "vUInt32", "vSInt32", "vUInt64", "vSInt64", "vFloat",
    "vDouble", "vBool32",
];

const MODULES: &[(&str, ModuleConfig)] = &[
    (
        "vdsp",
        ModuleConfig {
            output: "src/ffi/generated/vdsp_missing.rs",
            existing_module: Some("src/ffi/vdsp.rs"),
            headers: &["vecLib/vDSP.h"],
            apple_cf_reexports: &[],
        },
    ),
    (
        "vforce",
        ModuleConfig {
            output: "src/ffi/generated/vforce_missing.rs",
            existing_module: Some("src/ffi/vforce.rs"),
            headers: &["vecLib/vForce.h"],
            apple_cf_reexports: &[],
        },
    ),
    (
        "lapack",
        ModuleConfig {
            output: "src/ffi/generated/lapack_missing.rs",
            existing_module: Some("src/ffi/lapack.rs"),
            headers: &["vecLib/clapack.h"],
            apple_cf_reexports: &[],
        },
    ),
    (
        "bnns",
        ModuleConfig {
            output: "src/ffi/generated/bnns_missing.rs",
            existing_module: Some("src/ffi/bnns.rs"),
            headers: &[
                "vecLib/BNNS/bnns.h",
                "vecLib/BNNS/bnns_structures.h",
                "vecLib/BNNS/bnns_graph.h",
            ],
            apple_cf_reexports: &[],
        },
    ),
    (
        "sparse",
        ModuleConfig {
            output: "src/ffi/generated/sparse_missing.rs",
            existing_module: Some("src/ffi/sparse.rs"),
            headers: &[
                "vecLib/Sparse/Types.h",
                "vecLib/Sparse/BLAS.h",
                "vecLib/Sparse/Solve.h",
            ],
            apple_cf_reexports: &[],
        },
    ),
    (
        "vimage",
        ModuleConfig {
            output: "src/ffi/generated/vimage_missing.rs",
            existing_module: Some("src/ffi/vimage.rs"),
            headers: &[
                "vImage/BasicImageTypes.h",
                "vImage/Alpha.h",
                "vImage/Convolution.h",
                "vImage/Conversion.h",
                "vImage/Geometry.h",
                "vImage/Histogram.h",
                "vImage/Morphology.h",
                "vImage/Transform.h",
                "vImage/vImage_CVUtilities.h",
                "vImage/vImage_Types.h",
                "vImage/vImage_Utilities.h",
            ],
            apple_cf_reexports: &["Boolean", "CFTypeRef", "CFStringRef"],
        },
    ),
    (
        "veclib_extras",
        ModuleConfig {
            output: "src/ffi/generated/veclib_extras.rs",
            existing_module: None,
            headers: &[
                "vecLib/vBigNum.h",
                "vecLib/vBasicOps.h",
                "vecLib/vfp.h",
                "vecLib/thread_api.h",
                "vecLib/LinearAlgebra/object.h",
            ],
            apple_cf_reexports: &[],
        },
    ),
];

fn main() -> Result<()> {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| anyhow!("failed to locate repo root from generator manifest"))?
        .to_path_buf();
    let sdk_path = macos_sdk_path()?;

    for (name, config) in MODULES {
        let symbols = checked_in_symbols(&repo_root, config)?;
        generate_module(&repo_root, &sdk_path, name, config, &symbols)?;
    }

    Ok(())
}

fn macos_sdk_path() -> Result<String> {
    let output = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .context("failed to invoke xcrun")?;
    if !output.status.success() {
        return Err(anyhow!(
            "xcrun failed to resolve the macOS SDK path: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

fn checked_in_symbols(repo_root: &Path, config: &ModuleConfig) -> Result<Symbols> {
    let path = repo_root.join(config.output);
    let file =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    let function = Regex::new(r"\bpub fn ([A-Za-z_][A-Za-z0-9_]*)\s*\(")?;
    let item = Regex::new(r"\bpub (?:type|struct|union|enum) ([A-Za-z_][A-Za-z0-9_]*)")?;
    let var = Regex::new(r"\bpub (?:const|static(?: mut)?) ([A-Za-z_][A-Za-z0-9_]*)\s*:")?;
    let mut symbols = Symbols::default();
    symbols.functions.extend(
        function
            .captures_iter(&file)
            .map(|capture| capture[1].to_owned()),
    );
    symbols.types.extend(
        item.captures_iter(&file)
            .map(|capture| capture[1].to_owned()),
    );
    symbols.types.extend(
        config
            .apple_cf_reexports
            .iter()
            .map(|name| (*name).to_owned()),
    );
    symbols.vars.extend(
        var.captures_iter(&file)
            .map(|capture| capture[1].to_owned()),
    );
    Ok(symbols)
}

fn drop_by_value_vector_functions(bindings: &str) -> Result<(String, Vec<String>)> {
    let block = Regex::new(
        r#"(?s)#\[link\(name = "Accelerate", kind = "framework"\)\]\nunsafe extern "C" \{\n.*?\n\}\n"#,
    )?;
    let function = Regex::new(r"\bpub fn ([A-Za-z_][A-Za-z0-9_]*)\s*\(")?;
    let alternatives = C_VECTOR_TYPES
        .iter()
        .map(|name| regex_escape(name))
        .collect::<Vec<_>>()
        .join("|");
    let vector = Regex::new(&format!(r"(\*const |\*mut )?\b(?:{alternatives})\b"))?;
    let mut dropped = Vec::new();
    let mut kept = String::with_capacity(bindings.len());
    let mut last = 0;
    for found in block.find_iter(bindings) {
        kept.push_str(&bindings[last..found.start()]);
        let text = found.as_str();
        let by_value = vector
            .captures_iter(text)
            .any(|capture| capture.get(1).is_none());
        if by_value {
            dropped.extend(
                function
                    .captures_iter(text)
                    .map(|capture| capture[1].to_owned()),
            );
        } else {
            kept.push_str(text);
        }
        last = found.end();
    }
    kept.push_str(&bindings[last..]);
    Ok((kept, dropped))
}

fn reexport_apple_cf_types(bindings: &str, names: &[&str]) -> Result<String> {
    let mut output = bindings.to_owned();
    for name in names {
        let typedef = Regex::new(&format!(r"(?m)^pub type {} = [^;]*;$", regex_escape(name)))?;
        if !typedef.is_match(&output) {
            return Err(anyhow!("expected a generated typedef for {name}"));
        }
        output = typedef
            .replace(&output, format!("pub use apple_cf::raw::{name};").as_str())
            .into_owned();
    }
    Ok(output)
}

fn existing_type_names(repo_root: &Path, module_path: &str) -> Result<BTreeSet<String>> {
    let file = fs::read_to_string(repo_root.join(module_path))
        .with_context(|| format!("failed to read {module_path}"))?;
    let regex = Regex::new(r"\bpub\s+(?:type|struct|enum|union)\s+([A-Za-z_][A-Za-z0-9_]*)")?;
    Ok(regex
        .captures_iter(&file)
        .map(|capture| capture[1].to_owned())
        .collect())
}

fn manual_type_overrides(_module: &str) -> &'static [&'static str] {
    &[]
}

fn sdk_header_path(sdk_path: &str, header: &str) -> Result<String> {
    if let Some(rest) = header.strip_prefix("vecLib/") {
        return Ok(format!(
            "{sdk_path}/System/Library/Frameworks/Accelerate.framework/Frameworks/vecLib.framework/Headers/{rest}"
        ));
    }
    if let Some(rest) = header.strip_prefix("vImage/") {
        return Ok(format!(
            "{sdk_path}/System/Library/Frameworks/Accelerate.framework/Frameworks/vImage.framework/Headers/{rest}"
        ));
    }
    Err(anyhow!("unsupported header prefix for {header}"))
}

fn generate_module(
    repo_root: &Path,
    sdk_path: &str,
    name: &str,
    config: &ModuleConfig,
    symbols: &Symbols,
) -> Result<()> {
    if symbols.functions.is_empty() && symbols.types.is_empty() && symbols.vars.is_empty() {
        return Err(anyhow!("module {name} has no checked-in symbols"));
    }

    let wrapper_name = format!("{name}_wrapper.h");
    let wrapper_contents = config
        .headers
        .iter()
        .map(|header| sdk_header_path(sdk_path, header).map(|path| format!("#include \"{path}\"")))
        .collect::<Result<Vec<_>>>()?
        .join("\n");

    let mut builder = Builder::default()
        .header_contents(&wrapper_name, &wrapper_contents)
        .use_core()
        .size_t_is_usize(true)
        .layout_tests(true)
        .generate_comments(false)
        .derive_debug(true)
        .derive_default(true)
        .default_enum_style(EnumVariation::Consts)
        .clang_arg("-x")
        .clang_arg("c")
        .clang_arg("-isysroot")
        .clang_arg(sdk_path)
        .clang_arg("-D__clang_tapi__=1")
        .clang_arg("-F")
        .clang_arg(format!("{sdk_path}/System/Library/Frameworks"))
        .clang_arg("-F")
        .clang_arg(format!(
            "{sdk_path}/System/Library/Frameworks/Accelerate.framework/Frameworks"
        ));

    if let Some(existing_module) = config.existing_module {
        for type_name in existing_type_names(repo_root, existing_module)? {
            builder = builder.blocklist_type(format!("^{}$", regex_escape(&type_name)));
        }
    }
    for type_name in manual_type_overrides(name) {
        builder = builder.blocklist_type(format!("^{}$", regex_escape(type_name)));
    }

    for symbol in &symbols.functions {
        builder = builder.allowlist_function(format!("^{}$", regex_escape(symbol)));
    }
    for symbol in symbols
        .types
        .iter()
        .filter(|symbol| !C_VECTOR_TYPES.contains(&symbol.as_str()))
    {
        builder = builder.allowlist_type(format!("^{}$", regex_escape(symbol)));
    }
    for symbol in &symbols.vars {
        builder = builder.allowlist_var(format!("^{}$", regex_escape(symbol)));
    }

    let bindings = builder
        .generate()
        .with_context(|| format!("bindgen failed for module {name}"))?;
    let linked = bindings.to_string().replace(
        "unsafe extern \"C\" {",
        "#[link(name = \"Accelerate\", kind = \"framework\")]\nunsafe extern \"C\" {",
    );
    let (linked, dropped) = drop_by_value_vector_functions(&linked)?;
    if !dropped.is_empty() {
        println!(
            "{name}: omitted {} functions that pass C vector types by value: {}",
            dropped.len(),
            dropped.join(", ")
        );
    }
    let linked = reexport_apple_cf_types(&linked, config.apple_cf_reexports)?;
    let output = format!("// @generated by tools/raw-ffi-gen; do not edit by hand.\n\n{linked}");

    let output_path = repo_root.join(config.output);
    fs::write(&output_path, output)
        .with_context(|| format!("failed to write {}", output_path.display()))?;

    Ok(())
}
