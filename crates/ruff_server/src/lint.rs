//! Access to the Ruff linting API for the LSP

use std::path::Path;

use ruff_diagnostics::Diagnostic;
use ruff_linter::{
    directives::{extract_directives, Flags},
    linter::{check_path, LinterResult, TokenSource},
    registry::AsRule,
    settings::{flags, LinterSettings},
    source_kind::SourceKind,
};
use ruff_python_ast::PySourceType;
use ruff_python_codegen::Stylist;
use ruff_python_index::Indexer;
use ruff_python_parser::lexer::LexResult;
use ruff_python_parser::AsMode;
use ruff_source_file::Locator;

use crate::{PositionEncoding, DIAGNOSTIC_NAME};

pub(crate) fn check(
    document: &crate::edit::Document,
    linter_settings: &LinterSettings,
    encoding: PositionEncoding,
) -> Vec<lsp_types::Diagnostic> {
    let contents = document.contents();

    let source_type = PySourceType::default();

    // TODO(jane): Support Jupyter Notebooks
    let source_kind = SourceKind::Python(contents.to_string());

    // Tokenize once.
    let tokens: Vec<LexResult> = ruff_python_parser::tokenize(contents, source_type.as_mode());

    // Map row and column locations to byte slices (lazily).
    let locator = Locator::new(contents);

    // Detect the current code style (lazily).
    let stylist = Stylist::from_tokens(&tokens, &locator);

    // Extra indices from the code.
    let indexer = Indexer::from_tokens(&tokens, &locator);

    // Extract the `# noqa` and `# isort: skip` directives from the source.
    let directives = extract_directives(&tokens, Flags::empty(), &locator, &indexer);

    // Generate checks.
    let LinterResult {
        data: (diagnostics, _imports),
        ..
    } = check_path(
        Path::new("<filename>"),
        None,
        &locator,
        &stylist,
        &indexer,
        &directives,
        linter_settings,
        flags::Noqa::Enabled,
        &source_kind,
        source_type,
        TokenSource::Tokens(tokens),
    );

    diagnostics
        .into_iter()
        .map(|diagnostic| to_lsp_diagnostic(diagnostic, document, encoding))
        .collect()
}

fn to_lsp_diagnostic(
    diagnostic: Diagnostic,
    document: &crate::edit::Document,
    encoding: PositionEncoding,
) -> lsp_types::Diagnostic {
    let Diagnostic {
        kind,
        range,
        fix: _fix,
        ..
    } = diagnostic;

    let rule = kind.rule();

    // TODO: impl when implementing code action fixes
    let data = None;

    lsp_types::Diagnostic {
        range: crate::edit::text_range_to_range(range, document, encoding),
        severity: Some(lsp_types::DiagnosticSeverity::ERROR),
        code: Some(lsp_types::NumberOrString::String(
            rule.noqa_code().to_string(),
        )),
        code_description: rule.url().and_then(|url| {
            Some(lsp_types::CodeDescription {
                href: lsp_types::Url::parse(&url).ok()?,
            })
        }),
        source: Some(DIAGNOSTIC_NAME.into()),
        message: kind.body,
        related_information: None,
        tags: None,
        data,
    }
}
