use ruff_formatter::PrintedRange;
use ruff_python_formatter::{format_module_source, format_range};
use ruff_text_size::TextRange;
use ruff_workspace::FormatterSettings;

use crate::edit::Document;

pub(crate) fn format(
    document: &Document,
    formatter_settings: &FormatterSettings,
) -> crate::Result<String> {
    // TODO: support Jupyter Notebook
    let format_options = formatter_settings
        .to_format_options(ruff_python_ast::PySourceType::Python, document.contents());
    let formatted = format_module_source(document.contents(), format_options)?;
    Ok(formatted.into_code())
}

pub(crate) fn range_format(
    document: &Document,
    formatter_settings: &FormatterSettings,
    range: TextRange,
) -> crate::Result<PrintedRange> {
    // TODO: support Jupyter Notebook
    let format_options = formatter_settings
        .to_format_options(ruff_python_ast::PySourceType::Python, document.contents());

    Ok(format_range(document.contents(), range, format_options)?)
}
