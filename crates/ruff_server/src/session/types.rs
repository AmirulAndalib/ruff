#[derive(Debug, Clone, PartialEq, Eq)]
enum WhenToRun {
    OnType,
    OnSave,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
enum LogLevel {
    #[default]
    Error,
    Warning,
    Info,
    Debug,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct Settings {
    log_level: LogLevel,
    code_action: CodeActionSettings,
    organize_imports: bool,
    fix_all: bool,
    linter: LinterSettings,
    formatter: FormatterSettings,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct CodeActionSettings;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct LinterSettings;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct FormatterSettings;
