use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use specta::{functions::FunctionDataType, ts::TsExportError, ExportError, TypeDefs};

/// Building blocks for [`export`] and [`export_with_cfg`].
///
/// Specta-enabled library.
pub mod internal {
    use heck::ToLowerCamelCase;
    use indoc::formatdoc;

    use specta::{
        functions::FunctionDataType,
        ts::{self, TsExportError},
        TypeDefs,
    };

    const DO_NOT_EDIT: &str = "// This file was generated by [deno-specta](https://github.com/whymidnight/deno-specta). Do not edit this file manually.";

    /// Type definitions and constants that the generated functions rely on
    pub fn globals() -> String {
        formatdoc! {
            r#""#
        }
    }

    /// Renders a collection of [`FunctionDataType`] into a TypeScript string.
    pub fn render_functions(
        function_types: Vec<FunctionDataType>,
        cfg: &specta::ts::ExportConfiguration,
    ) -> Result<String, TsExportError> {
        function_types
            .into_iter()
            .map(|function| {
                let name = &function.name;
                let name_camel = function.name.to_lower_camel_case();

                let arg_defs = function
                    .args
                    .iter()
                    .map(|(name, typ)| {
                        ts::datatype(cfg, typ)
                            .map(|ty| format!("{}: {}", name.to_lower_camel_case(), ty))
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .join(", ");

                let ret_type = ts::datatype(cfg, &function.result)?;

                let arg_usages = function
                    .args
                    .iter()
                    .map(|(name, _)| name.to_lower_camel_case())
                    .collect::<Vec<_>>();

                let arg_usages = arg_usages
                    .is_empty()
                    .then(Default::default)
                    .unwrap_or_else(|| format!(", {{ {} }}", arg_usages.join(",")));

                Ok(formatdoc!(
                    r#"
                    export function {name_camel}({arg_defs}) {{
                        return invoke<{ret_type}>("{name}"{arg_usages})
                    }}"#
                ))
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|v| v.join("\n\n"))
    }

    /// Renders the output of [`globals`], [`render_functions`] and all dependant types into a TypeScript string.
    pub fn render(
        function_types: Vec<FunctionDataType>,
        type_map: TypeDefs,
        cfg: &specta::ts::ExportConfiguration,
    ) -> Result<String, TsExportError> {
        let globals = globals();

        let functions = render_functions(function_types, cfg)?;

        let dependant_types = type_map
            .values()
            .filter_map(|v| v.as_ref())
            .map(|v| ts::export_datatype(cfg, v))
            .collect::<Result<Vec<_>, _>>()
            .map(|v| v.join("\n"))?;

        Ok(formatdoc! {
            r#"
                {DO_NOT_EDIT}

                {globals}

                {functions}

                {dependant_types}
            "#
        })
    }
}

/// Exports the output of [`internal::render`] for a collection of [`FunctionDataType`] into a TypeScript file.
/// Allows for specifying a custom [`ExportConfiguration`](specta::ts::ExportConfiguration).
pub fn export_with_cfg(
    (function_types, type_map): (Vec<FunctionDataType>, TypeDefs),
    cfg: specta::ts::ExportConfiguration,
    export_path: impl AsRef<Path>,
) -> Result<(), TsExportError> {
    let export_path = PathBuf::from(export_path.as_ref());

    if let Some(export_dir) = export_path.parent() {
        fs::create_dir_all(export_dir)?;
    }

    let mut file = File::create(export_path)?;

    write!(
        file,
        "{}",
        internal::render(function_types, type_map, &cfg)?
    )?;

    Ok(())
}

/// Exports the output of [`internal::render`] for a collection of [`FunctionDataType`] into a TypeScript file.
pub fn export(
    macro_data: Result<(Vec<FunctionDataType>, TypeDefs), ExportError>,
    export_path: impl AsRef<Path>,
) -> Result<(), TsExportError> {
    export_with_cfg(macro_data?, Default::default(), export_path)
}
