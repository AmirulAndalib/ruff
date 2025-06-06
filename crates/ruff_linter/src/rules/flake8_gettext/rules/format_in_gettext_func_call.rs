use ruff_python_ast::{self as ast, Expr};

use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_text_size::Ranged;

use crate::Violation;
use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for `str.format` calls in `gettext` function calls.
///
/// ## Why is this bad?
/// In the `gettext` API, the `gettext` function (often aliased to `_`) returns
/// a translation of its input argument by looking it up in a translation
/// catalog.
///
/// Calling `gettext` with a formatted string as its argument can cause
/// unexpected behavior. Since the formatted string is resolved before the
/// function call, the translation catalog will look up the formatted string,
/// rather than the `str.format`-style template.
///
/// Instead, format the value returned by the function call, rather than
/// its argument.
///
/// ## Example
/// ```python
/// from gettext import gettext as _
///
/// name = "Maria"
/// _("Hello, {}!".format(name))  # Looks for "Hello, Maria!".
/// ```
///
/// Use instead:
/// ```python
/// from gettext import gettext as _
///
/// name = "Maria"
/// _("Hello, %s!") % name  # Looks for "Hello, %s!".
/// ```
///
/// ## References
/// - [Python documentation: `gettext` — Multilingual internationalization services](https://docs.python.org/3/library/gettext.html)
#[derive(ViolationMetadata)]
pub(crate) struct FormatInGetTextFuncCall;

impl Violation for FormatInGetTextFuncCall {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`format` method argument is resolved before function call; consider `_(\"string %s\") % arg`".to_string()
    }
}

/// INT002
pub(crate) fn format_in_gettext_func_call(checker: &Checker, args: &[Expr]) {
    if let Some(first) = args.first() {
        if let Expr::Call(ast::ExprCall { func, .. }) = &first {
            if let Expr::Attribute(ast::ExprAttribute { attr, .. }) = func.as_ref() {
                if attr == "format" {
                    checker.report_diagnostic(FormatInGetTextFuncCall {}, first.range());
                }
            }
        }
    }
}
