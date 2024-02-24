use crate::server::{client::Notifier, Result};
use crate::session::SessionSnapshot;
use lsp_types::{self as types, request as req};
use types::{
    DocumentDiagnosticReportResult, FullDocumentDiagnosticReport,
    RelatedFullDocumentDiagnosticReport,
};

pub(crate) struct Diagnostic;

impl super::Request for Diagnostic {
    type RequestType = req::DocumentDiagnosticRequest;
}

impl super::BackgroundRequest for Diagnostic {
    super::define_document_url!(params: &types::DocumentDiagnosticParams);
    fn run_with_snapshot(
        snapshot: SessionSnapshot,
        _notifier: Notifier,
        _params: types::DocumentDiagnosticParams,
    ) -> Result<DocumentDiagnosticReportResult> {
        let diagnostics = crate::lint::check(
            snapshot.document(),
            &snapshot.configuration().linter,
            snapshot.encoding(),
        );

        Ok(DocumentDiagnosticReportResult::Report(
            types::DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
                related_documents: None,
                full_document_diagnostic_report: FullDocumentDiagnosticReport {
                    result_id: None,
                    items: diagnostics,
                },
            }),
        ))
    }
}
