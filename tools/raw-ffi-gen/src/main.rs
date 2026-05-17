use anyhow::{anyhow, Context, Result};
use bindgen::{Builder, EnumVariation};
use regex::{escape as regex_escape, Regex};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Debug)]
struct AuditRow {
    symbol: String,
    kind: String,
    header: String,
}

#[derive(Clone, Debug)]
struct ModuleConfig {
    output: &'static str,
    existing_module: Option<&'static str>,
    headers: &'static [&'static str],
    header_prefixes: &'static [&'static str],
}

const MODULES: &[(&str, ModuleConfig)] = &[
    (
        "vdsp",
        ModuleConfig {
            output: "src/ffi/generated/vdsp_missing.rs",
            existing_module: Some("src/ffi/vdsp.rs"),
            headers: &["vecLib/vDSP.h"],
            header_prefixes: &["vecLib/vDSP.h"],
        },
    ),
    (
        "vforce",
        ModuleConfig {
            output: "src/ffi/generated/vforce_missing.rs",
            existing_module: Some("src/ffi/vforce.rs"),
            headers: &["vecLib/vForce.h"],
            header_prefixes: &["vecLib/vForce.h"],
        },
    ),
    (
        "lapack",
        ModuleConfig {
            output: "src/ffi/generated/lapack_missing.rs",
            existing_module: Some("src/ffi/lapack.rs"),
            headers: &["vecLib/clapack.h"],
            header_prefixes: &["vecLib/clapack.h"],
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
            header_prefixes: &[
                "vecLib/BNNS/bnns.h",
                "vecLib/BNNS/bnns_structures.h",
                "vecLib/BNNS/bnns_graph.h",
            ],
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
            header_prefixes: &[
                "vecLib/Sparse/Types.h",
                "vecLib/Sparse/BLAS.h",
                "vecLib/Sparse/Solve.h",
                "vecLib/Sparse/SolveImplementation.h",
                "vecLib/Sparse/SolveImplementationTyped.h",
            ],
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
            header_prefixes: &[
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
            header_prefixes: &[
                "vecLib/vBigNum.h",
                "vecLib/vBasicOps.h",
                "vecLib/vfp.h",
                "vecLib/thread_api.h",
                "vecLib/LinearAlgebra/object.h",
            ],
        },
    ),
];

fn main() -> Result<()> {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| anyhow!("failed to locate repo root from generator manifest"))?
        .to_path_buf();
    let audit_path = repo_root.join("COVERAGE_AUDIT.md");
    let audit = fs::read_to_string(&audit_path)
        .with_context(|| format!("failed to read {}", audit_path.display()))?;
    let rows = load_gap_rows(&audit)?;
    let sdk_path = macos_sdk_path()?;

    for (name, config) in MODULES {
        let relevant_rows = rows_for_module(&rows, config);
        generate_module(&repo_root, &sdk_path, name, config, &relevant_rows)?;
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

fn load_gap_rows(audit: &str) -> Result<Vec<AuditRow>> {
    let Some((_, tail)) = audit.split_once("## 🔴 GAPS") else {
        return Err(anyhow!("missing audit gaps section"));
    };
    let section_body = tail.split("\n## ").next().unwrap_or(tail);
    Ok(parse_table_rows(section_body))
}

fn parse_table_rows(section: &str) -> Vec<AuditRow> {
    section
        .lines()
        .filter(|line| line.starts_with('|'))
        .filter(|line| !line.starts_with("| ---"))
        .filter(|line| !line.contains("Symbol | Kind | Header |"))
        .filter_map(|line| {
            let parts: Vec<_> = line
                .trim_matches('|')
                .split('|')
                .map(str::trim)
                .collect();
            if parts.len() < 4 {
                return None;
            }
            Some(AuditRow {
                symbol: parts[0].to_owned(),
                kind: parts[1].to_owned(),
                header: parts[2].to_owned(),
            })
        })
        .collect()
}

fn rows_for_module(rows: &[AuditRow], config: &ModuleConfig) -> Vec<AuditRow> {
    rows.iter()
        .filter(|row| config.header_prefixes.iter().any(|prefix| row.header == *prefix))
        .cloned()
        .collect()
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
    rows: &[AuditRow],
) -> Result<()> {
    if rows.is_empty() {
        return Err(anyhow!("module {name} matched no gap rows"));
    }

    let mut functions = BTreeSet::new();
    let mut types = BTreeSet::new();
    let mut vars = BTreeSet::new();

    for row in rows {
        match row.kind.as_str() {
            "function" => {
                functions.insert(row.symbol.clone());
            }
            "typedef" => {
                types.insert(row.symbol.clone());
            }
            "const" => {
                vars.insert(row.symbol.clone());
            }
            _ => {}
        }
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
        .layout_tests(false)
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

    for symbol in &functions {
        builder = builder.allowlist_function(format!("^{}$", regex_escape(symbol)));
    }
    for symbol in &types {
        builder = builder.allowlist_type(format!("^{}$", regex_escape(symbol)));
    }
    for symbol in &vars {
        builder = builder.allowlist_var(format!("^{}$", regex_escape(symbol)));
    }

    let bindings = builder
        .generate()
        .with_context(|| format!("bindgen failed for module {name}"))?;
    let output = format!(
        "// @generated by tools/raw-ffi-gen; do not edit by hand.\n\n{}",
        bindings.to_string().replace(
            "unsafe extern \"C\" {",
            "#[link(name = \"Accelerate\", kind = \"framework\")]\nunsafe extern \"C\" {",
        )
    );

    let output_path = repo_root.join(config.output);
    fs::write(&output_path, output)
        .with_context(|| format!("failed to write {}", output_path.display()))?;

    Ok(())
}
