use crate::server::schedule::Task;
use lsp_server as server;

mod notifications;
mod requests;
mod traits;

use notifications as notification;
use requests as request;
use traits::{BackgroundRequest, SyncNotification};

use super::Result;

macro_rules! handle_task {
    ($class: ty, $id: ident, $params: ident, $handle:ty) => {
        handle_task!($class, $id, $params, $handle { use background })
    };
    ($class: ty, $id: ident, $params: ident, $handle:ty { use local }) => {
        Task::local(move |session, notifier, responder| {
            let result = <$handle>::run(session, notifier, $params);
            <$handle as $class>::respond($id, result, &responder);
        })
    };
    ($class: ty, $id: ident, $params: ident, $handle:ty { use $schedule:ident }) => {
        Task::$schedule(move |session| {
            let Some(snapshot) = session.take_snapshot(<$handle>::document_url(&$params)) else { return Box::new(|_, _| {}) };
            Box::new(move |notifier, responder| {
                let result = <$handle>::run_with_snapshot(snapshot, notifier, $params);
                <$handle as $class>::respond($id, result, &responder);
            })
        })
    };
}

macro_rules! select_task {
    ($req:ident as $class:ty => { $($handle:ty$({ $($conf:tt)* })?),* $(,)? }) => {
        (move || {
            let __req;
            {
                $(
                    let $req = match <$handle as $class>::cast($req) {
                        Ok((id, params)) => {
                            return Ok(handle_task!($class, id, params, $handle $({$($conf)*})?));
                        },
                        Err(lsp_server::ExtractError::MethodMismatch(req)) => req,
                        Err(json_err @ lsp_server::ExtractError::JsonError { .. }) => {
                            let err: anyhow::Error = json_err.into();
                            return Err(anyhow::anyhow!("JSON parsing failure:\n{err}")).with_failure_code(lsp_server::ErrorCode::ParseError);
                        }
                    };
                )*
                __req = $req;
            };
            Err(anyhow::anyhow!("No route found for {:?}", __req)).with_failure_code(lsp_server::ErrorCode::MethodNotFound)
        })()
    };
}

macro_rules! define_document_url {
    ($params:ident: &$p:ty) => {
        fn document_url($params: &$p) -> &lsp_types::Url {
            &$params.text_document.uri
        }
    };
}

use define_document_url;

pub(in crate::server) fn request<'a>(req: server::Request) -> Task<'a> {
    let id = req.id.clone();
    select_task! {
        req as traits::Request => {
            request::CodeAction { use low_latency_thread },
            request::Diagnostic { use low_latency_thread },
            request::Format { use fmt_thread },
            request::FormatRange { use fmt_thread },
        }
    }
    .unwrap_or_else(|err| {
        tracing::error!("Encountered error when routing request: {err}");
        let result: Result<()> = Err(err);
        Task::immediate(id, result)
    })
}

pub(in crate::server) fn notification<'a>(notif: server::Notification) -> Task<'a> {
    select_task! {
        notif as traits::Notification => {
            notification::Cancel { use local },
            notification::DidOpen { use local },
            notification::DidChange { use local },
            notification::DidChangeConfiguration { use local },
            notification::DidChangeWorkspace { use local },
            notification::DidClose { use local },
        }
    }
    .unwrap_or_else(|err| {
        tracing::error!("Encountered error when routing notification: {err}");
        Task::nothing()
    })
}

pub(crate) struct Error {
    pub(crate) code: lsp_server::ErrorCode,
    pub(crate) error: anyhow::Error,
}

trait LSPResult<T> {
    fn with_failure_code(self, code: lsp_server::ErrorCode) -> super::Result<T>;
}

impl<T> LSPResult<T> for anyhow::Result<T> {
    fn with_failure_code(self, code: server::ErrorCode) -> super::Result<T> {
        self.map_err(|err| Error::new(err, code))
    }
}

impl Error {
    pub(crate) fn new(err: anyhow::Error, code: lsp_server::ErrorCode) -> Self {
        Self { code, error: err }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(f)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(f)
    }
}
