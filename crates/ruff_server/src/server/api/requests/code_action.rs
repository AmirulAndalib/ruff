use crate::server::{client::Notifier, Result};
use crate::session::SessionSnapshot;
use lsp_types::{self as types, request as req};

pub(crate) struct CodeAction;

impl super::Request for CodeAction {
    type RequestType = req::CodeActionRequest;
}

impl super::BackgroundRequest for CodeAction {
    super::define_document_url!(params: &types::CodeActionParams);
    fn run_with_snapshot(
        _session: SessionSnapshot,
        _notifier: Notifier,
        _params: types::CodeActionParams,
    ) -> Result<Option<types::CodeActionResponse>> {
        Ok(Some(vec![types::CodeActionOrCommand::CodeAction(
            types::CodeAction {
                title: "Code Action".into(),
                ..Default::default()
            },
        )]))
    }
}
