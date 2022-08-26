use std::num::NonZeroU32;

use crate::cgu_reuse_tracker::CguReuse;
use crate::{self as rustc_session};
use rustc_errors::MultiSpan;
use rustc_macros::SessionDiagnostic;
use rustc_span::{Span, Symbol};

#[derive(SessionDiagnostic)]
#[diag(session::incorrect_cgu_reuse_type)]
pub struct IncorrectCguReuseType<'a> {
    #[primary_span]
    pub span: Span,
    pub cgu_user_name: &'a str,
    pub actual_reuse: CguReuse,
    pub expected_reuse: CguReuse,
    pub at_least: u8,
}

#[derive(SessionDiagnostic)]
#[diag(session::cgu_not_recorded)]
pub struct CguNotRecorded<'a> {
    pub cgu_user_name: &'a str,
    pub cgu_name: &'a str,
}

#[derive(SessionDiagnostic)]
#[diag(session::feature_gate_error, code = "E0658")]
pub struct FeatureGateError<'a> {
    #[primary_span]
    pub span: MultiSpan,
    pub explain: &'a str,
}

#[derive(SessionSubdiagnostic)]
#[note(session::feature_diagnostic_for_issue)]
pub struct FeatureDiagnosticForIssue {
    pub n: NonZeroU32,
}

#[derive(SessionSubdiagnostic)]
#[help(session::feature_diagnostic_help)]
pub struct FeatureDiagnosticHelp {
    pub feature: Symbol,
}

#[derive(SessionDiagnostic)]
#[diag(session::target_data_layout_parse_error)]
pub struct TargetDataLayoutParseError {
    pub err: String,
}

#[derive(SessionDiagnostic)]
#[diag(session::not_circumvent_feature)]
pub struct NotCircumventFeature;

#[derive(SessionDiagnostic)]
#[diag(session::linker_plugin_lto_windows_not_supported)]
pub struct LinkerPluginToWindowsNotSupported;

#[derive(SessionDiagnostic)]
#[diag(session::profile_use_file_does_not_exist)]
pub struct ProfileUseFileDoesNotExist<'a> {
    pub path: &'a std::path::Path,
}

#[derive(SessionDiagnostic)]
#[diag(session::profile_sample_use_file_does_not_exist)]
pub struct ProfileSampleUseFileDoesNotExist<'a> {
    pub path: &'a std::path::Path,
}

#[derive(SessionDiagnostic)]
#[diag(session::target_requires_unwind_tables)]
pub struct TargetRequiresUnwindTables;

#[derive(SessionDiagnostic)]
#[diag(session::sanitizer_not_supported)]
pub struct SanitizerNotSupported {
    pub us: String,
}

#[derive(SessionDiagnostic)]
#[diag(session::sanitizers_not_supported)]
pub struct SanitizersNotSupported {
    pub us: String,
}

#[derive(SessionDiagnostic)]
#[diag(session::cannot_mix_and_match_sanitizers)]
pub struct CannotMixAndMatchSanitizers {
    pub first: String,
    pub second: String,
}

#[derive(SessionDiagnostic)]
#[diag(session::cannot_enable_crt_static_linux)]
pub struct CannotEnableCrtStaticLinux;

#[derive(SessionDiagnostic)]
#[diag(session::sanitizer_cfi_enabled)]
pub struct SanitizerCfiEnabled;

#[derive(SessionDiagnostic)]
#[diag(session::unstable_virtual_function_elimination)]
pub struct UnstableVirtualFunctionElimination;

#[derive(SessionDiagnostic)]
#[diag(session::unsupported_dwarf_version)]
pub struct UnsupportedDwarfVersion {
    pub dwarf_version: u32,
}
