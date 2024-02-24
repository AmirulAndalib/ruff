//! A stateful LSP implementation that calls into the Ruff API.

use crate::server::client::{Notifier, Responder};
use crate::session::{Session, SessionSnapshot};

use lsp_server as server;
use lsp_types::notification::Notification as LSPNotification;
use lsp_types::request::Request as LSPRequest;

pub(super) trait Request {
    type RequestType: LSPRequest;
    const METHOD: &'static str = <<Self as Request>::RequestType as LSPRequest>::METHOD;

    fn cast(
        request: server::Request,
    ) -> std::result::Result<
        (
            lsp_server::RequestId,
            <<Self as Request>::RequestType as LSPRequest>::Params,
        ),
        server::ExtractError<server::Request>,
    > {
        request.extract(Self::METHOD)
    }

    fn respond<R>(
        id: lsp_server::RequestId,
        result: crate::server::Result<R>,
        responder: &Responder,
    ) where
        R: serde::Serialize,
    {
        if let Err(err) = responder.respond(id, result) {
            tracing::error!("Failed to send response: {err}");
        }
    }
}

pub(super) trait SyncRequest: Request {
    fn run(
        session: &mut Session,
        notifier: Notifier,
        params: <<Self as Request>::RequestType as LSPRequest>::Params,
    ) -> super::Result<<<Self as Request>::RequestType as LSPRequest>::Result>;
}

pub(super) trait BackgroundRequest: Request {
    fn document_url(
        params: &<<Self as Request>::RequestType as LSPRequest>::Params,
    ) -> &lsp_types::Url;

    fn run_with_snapshot(
        snapshot: SessionSnapshot,
        notifier: Notifier,
        params: <<Self as Request>::RequestType as LSPRequest>::Params,
    ) -> super::Result<<<Self as Request>::RequestType as LSPRequest>::Result>;
}

pub(super) trait Notification {
    type NotificationType: LSPNotification;
    const METHOD: &'static str =
        <<Self as Notification>::NotificationType as LSPNotification>::METHOD;

    fn cast(
        notification: server::Notification,
    ) -> std::result::Result<
        (
            String,
            <<Self as Notification>::NotificationType as LSPNotification>::Params,
        ),
        server::ExtractError<server::Notification>,
    > {
        Ok((
            Self::METHOD.to_string(),
            notification.extract(Self::METHOD)?,
        ))
    }

    fn respond(method: String, result: crate::server::Result<()>, _responder: &Responder) {
        if let Err(err) = result {
            tracing::error!("Background notification failed: {err}");
        } else {
            tracing::debug!("`{method}` notification handler finished successfully");
        }
    }
}

pub(super) trait SyncNotification: Notification {
    fn run(
        session: &mut Session,
        notifier: Notifier,
        params: <<Self as Notification>::NotificationType as LSPNotification>::Params,
    ) -> super::Result<()>;
}

pub(super) trait BackgroundNotification: Notification {
    fn document_url(
        params: &<<Self as Notification>::NotificationType as LSPNotification>::Params,
    ) -> &lsp_types::Url;

    fn run_with_snapshot(
        snapshot: SessionSnapshot,
        notifier: Notifier,
        params: <<Self as Notification>::NotificationType as LSPNotification>::Params,
    ) -> super::Result<()>;
}
