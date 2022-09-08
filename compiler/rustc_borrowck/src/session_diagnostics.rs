use rustc_errors::{IntoDiagnosticArg, MultiSpan};
use rustc_macros::{LintDiagnostic, SessionDiagnostic, SessionSubdiagnostic};
use rustc_middle::ty::Ty;
use rustc_span::{Span, Symbol};

use crate::diagnostics::RegionName;

#[derive(SessionDiagnostic)]
#[diag(borrowck::move_unsized, code = "E0161")]
pub(crate) struct MoveUnsized<'tcx> {
    pub ty: Ty<'tcx>,
    #[primary_span]
    #[label]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::higher_ranked_lifetime_error)]
pub(crate) struct HigherRankedLifetimeError {
    #[subdiagnostic]
    pub cause: Option<HigherRankedErrorCause>,
    #[primary_span]
    pub span: Span,
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum HigherRankedErrorCause {
    #[note(borrowck::could_not_prove)]
    CouldNotProve { predicate: String },
    #[note(borrowck::could_not_normalize)]
    CouldNotNormalize { value: String },
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::higher_ranked_subtype_error)]
pub(crate) struct HigherRankedSubtypeError {
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::generic_does_not_live_long_enough)]
pub(crate) struct GenericDoesNotLiveLongEnough {
    pub kind: String,
    #[primary_span]
    pub span: Span,
}

#[derive(LintDiagnostic)]
#[diag(borrowck::var_does_not_need_mut)]
pub(crate) struct VarNeedNotMut {
    #[suggestion_short(applicability = "machine-applicable", code = "")]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::const_not_used_in_type_alias)]
pub(crate) struct ConstNotUsedTraitAlias {
    pub ct: String,
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::var_cannot_escape_closure)]
#[note]
#[note(borrowck::cannot_escape)]
pub(crate) struct FnMutError {
    #[primary_span]
    pub span: Span,
    #[subdiagnostic]
    pub ty_err: FnMutReturnTypeErr,
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum VarHereDenote {
    #[label(borrowck::var_here_captured)]
    Captured {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::var_here_defined)]
    Defined {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::closure_inferred_mut)]
    FnMutInferred {
        #[primary_span]
        span: Span,
    },
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum FnMutReturnTypeErr {
    #[label(borrowck::returned_closure_escaped)]
    ReturnClosure {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::returned_async_block_escaped)]
    ReturnAsyncBlock {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::returned_ref_escaped)]
    ReturnRef {
        #[primary_span]
        span: Span,
    },
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::lifetime_constraints_error)]
pub(crate) struct LifetimeOutliveErr {
    #[primary_span]
    pub span: Span,
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum LifetimeReturnCategoryErr<'a> {
    #[label(borrowck::returned_lifetime_wrong)]
    WrongReturn {
        #[primary_span]
        span: Span,
        mir_def_name: &'a str,
        outlived_fr_name: RegionName,
        fr_name: &'a RegionName,
    },
    #[label(borrowck::returned_lifetime_short)]
    ShortReturn {
        #[primary_span]
        span: Span,
        category_desc: &'static str,
        free_region_name: &'a RegionName,
        outlived_fr_name: RegionName,
    },
}

impl IntoDiagnosticArg for &RegionName {
    fn into_diagnostic_arg(self) -> rustc_errors::DiagnosticArgValue<'static> {
        format!("{}", self).into_diagnostic_arg()
    }
}

impl IntoDiagnosticArg for RegionName {
    fn into_diagnostic_arg(self) -> rustc_errors::DiagnosticArgValue<'static> {
        format!("{}", self).into_diagnostic_arg()
    }
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum RequireStaticErr {
    #[note(borrowck::used_impl_require_static)]
    UsedImpl {
        #[primary_span]
        multi_span: MultiSpan,
    },
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum AddMoveErr {
    #[label(borrowck::data_moved_here)]
    Here {
        #[primary_span]
        binding_span: Span,
    },
    #[label(borrowck::and_data_moved_here)]
    AndHere {
        #[primary_span]
        binding_span: Span,
    },
    #[note(borrowck::moved_var_cannot_copy)]
    MovedNotCopy,
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum BorrowUsedHere {
    #[label(borrowck::used_here_by_closure)]
    ByClosure {
        #[primary_span]
        path_span: Span,
    },
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum BorrowUsedLater<'a> {
    #[label(borrowck::borrow_later_captured_by_trait_object)]
    TraitCapture {
        borrow_desc: &'a str,
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::borrow_later_captured_by_closure)]
    ClosureCapture {
        borrow_desc: &'a str,
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::borrow_later_used_by_call)]
    Call {
        borrow_desc: &'a str,
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::borrow_later_stored_here)]
    FakeLetRead {
        borrow_desc: &'a str,
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::borrow_later_used_here)]
    Other {
        borrow_desc: &'a str,
        #[primary_span]
        span: Span,
    },
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum BorrowUsedLaterInLoop<'a> {
    #[label(borrowck::trait_capture_borrow_in_later_iteration_loop)]
    TraitCapture {
        borrow_desc: &'a str,
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::closure_capture_borrow_in_later_iteration_loop)]
    ClosureCapture {
        borrow_desc: &'a str,
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::call_used_borrow_in_later_iteration_loop)]
    Call {
        borrow_desc: &'a str,
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::borrow_later_stored_here)]
    FakeLetRead {
        borrow_desc: &'a str,
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::used_borrow_in_later_iteration_loop)]
    Other {
        borrow_desc: &'a str,
        #[primary_span]
        span: Span,
    },
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum BorrowLaterBorrowUsedLaterInLoop<'a> {
    #[label(borrowck::bl_trait_capture_borrow_in_later_iteration_loop)]
    TraitCapture {
        borrow_desc: &'a str,
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::bl_closure_capture_borrow_in_later_iteration_loop)]
    ClosureCapture {
        borrow_desc: &'a str,
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::call_used_borrow_in_later_iteration_loop)]
    Call {
        borrow_desc: &'a str,
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::bl_borrow_later_stored_here)]
    FakeLetRead {
        borrow_desc: &'a str,
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::bl_used_borrow_in_later_iteration_loop)]
    Other {
        borrow_desc: &'a str,
        #[primary_span]
        span: Span,
    },
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum UsedLaterDropped<'a> {
    #[label(borrowck::drop_local_might_cause_borrow)]
    UsedHere {
        borrow_desc: &'a str,
        local_name: Symbol,
        type_desc: &'a str,
        dtor_desc: &'a str,
        #[primary_span]
        span: Span,
    },
    #[note(borrowck::var_dropped_in_wrong_order)]
    OppositeOrder,
    #[label(borrowck::temporary_access_to_borrow)]
    TemporaryCreatedHere {
        borrow_desc: &'a str,
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::drop_temporary_might_cause_borrow_use)]
    MightUsedHere {
        borrow_desc: &'a str,
        type_desc: &'a str,
        dtor_desc: &'a str,
        #[primary_span]
        span: Span,
    },
    #[suggestion_verbose(
        borrowck::consider_add_semicolon,
        applicability = "maybe-incorrect",
        code = ";"
    )]
    AddSemicolon {
        #[primary_span]
        span: Span,
    },
    #[multipart_suggestion(
        borrowck::consider_move_expression_end_of_block,
        applicability = "maybe-incorrect"
    )]
    MoveBlockEnd {
        #[suggestion_part(code = "let x = ")]
        lo_span: Span,
        #[suggestion_part(code = "; x")]
        hi_span: Span,
    },
    #[note(borrowck::consider_forcing_temporary_drop_sooner)]
    ManualDrop,
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum MustValidFor<'a> {
    #[label(borrowck::outlive_constraint_need_borrow_for)]
    Borrowed {
        category: &'a str,
        desc: &'a str,
        region_name: &'a RegionName,
        #[primary_span]
        span: Span,
    },
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum OnLifetimeBound<'a> {
    #[help(borrowck::consider_add_lifetime_bound)]
    Add { fr_name: &'a RegionName, outlived_fr_name: &'a RegionName },
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum ClosureCannotAgain {
    #[note(borrowck::closure_cannot_invoke_again)]
    Invoke {
        place: String,
        #[primary_span]
        span: Span,
    },
    #[note(borrowck::closure_cannot_move_again)]
    Move {
        place: String,
        #[primary_span]
        span: Span,
    },
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum ShowMutatingUpvar {
    #[label(borrowck::require_mutable_binding)]
    RequireMutableBinding {
        place: String,
        reason: String,
        #[primary_span]
        span: Span,
    },
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum CaptureCausedBy<'a> {
    #[label(borrowck::moved_by_call)]
    Call {
        place_name: &'a str,
        partially_str: &'a str,
        loop_message: &'a str,
        #[primary_span]
        span: Span,
    },
    #[note(borrowck::moved_fnonce_value)]
    FnOnceVal {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::moved_by_operator_use)]
    OperatorUse {
        place_name: &'a str,
        partially_str: &'a str,
        loop_message: &'a str,
        #[primary_span]
        span: Span,
    },
    #[note(borrowck::lhs_moved_by_operator_call)]
    OperatorCall {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::moved_by_implicit_call)]
    ImplicitCall {
        place_name: &'a str,
        partially_str: &'a str,
        loop_message: &'a str,
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::moved_by_method_call)]
    MethodCall {
        place_name: &'a str,
        partially_str: &'a str,
        loop_message: &'a str,
        #[primary_span]
        span: Span,
    },
    #[note(borrowck::function_takes_self_ownership)]
    FnTakeSelf {
        place_name: &'a str,
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::consider_borrow_content_of_type)]
    ConsiderManualBorrow {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::value_moved_here)]
    ValueHere {
        move_msg: &'a str,
        partially_str: &'a str,
        loop_message: &'a str,
        #[primary_span]
        span: Span,
    },
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum NotImplCopy<'a, 'tcx> {
    #[label(borrowck::type_not_impl_Copy)]
    Label {
        place_desc: &'a str,
        ty: Ty<'tcx>,
        move_prefix: &'a str,
        #[primary_span]
        span: Span,
    },
    #[note(borrowck::type_not_impl_Copy)]
    Note { place_desc: &'a str, ty: Ty<'tcx>, move_prefix: &'a str },
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum FnMutBumpFn<'a> {
    #[label(borrowck::cannot_act)]
    Cannot {
        act: &'a str,
        #[primary_span]
        sp: Span,
    },
    #[label(borrowck::expects_fnmut_not_fn)]
    AcceptFnMut {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::expects_fn_not_fnmut)]
    AcceptFn {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::empty_label)]
    EmptyLabel {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::in_this_closure)]
    Here {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck::return_fnmut)]
    ReturnFnMut {
        #[primary_span]
        span: Span,
    },
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum RegionNameLabels<'a> {
    #[label(borrowck::name_this_region)]
    NameRegion {
        #[primary_span]
        span: Span,
        rg_name: &'a RegionName,
    },
    #[label(borrowck::lifetime_appears_in_type)]
    LifetimeInType {
        #[primary_span]
        span: Span,
        type_name: &'a str,
        rg_name: &'a RegionName,
    },
    #[label(borrowck::return_type_has_lifetime)]
    LifetimeInReturned {
        #[primary_span]
        span: Span,
        mir_description: &'a str,
        type_name: &'a str,
        rg_name: &'a RegionName,
    },
    #[label(borrowck::lifetime_appears_in_type_of)]
    LifetimeInTypeOf {
        #[primary_span]
        span: Span,
        upvar_name: String,
        rg_name: &'a RegionName,
    },
    #[label(borrowck::return_type_is_type)]
    ReturnTypeIsTpye {
        #[primary_span]
        span: Span,
        mir_description: &'a str,
        type_name: &'a str,
    },
    #[label(borrowck::yield_type_is_type)]
    YieldTypeIsTpye {
        #[primary_span]
        span: Span,
        type_name: &'a str,
    },
    #[label(borrowck::lifetime_appears_here_in_impl)]
    LifetimeInImpl {
        #[primary_span]
        span: Span,
        rg_name: &'a RegionName,
        location: &'a str,
    },
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::type_parameter_not_used_in_trait_type_alias)]
pub(crate) struct UnusedTypeParameter<'tcx> {
    pub ty: Ty<'tcx>,
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::non_defining_opaque_type)]
pub(crate) struct OpaqueTypeNotDefine {
    #[subdiagnostic]
    pub cause: OpaqueTyDefineErrCause,
    #[primary_span]
    pub span: Span,
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum OpaqueTyDefineErrCause {
    #[label(borrowck::lifetime_not_used_in_trait_type_alias)]
    UnusedLifetime {
        #[primary_span]
        span: Span,
        r: String,
    },
    #[note(borrowck::used_non_generic_for_generic)]
    NonGenericUsed {
        #[primary_span]
        span: Span,
        descr: String,
        arg: String,
    },
    #[label(borrowck::cannot_use_static_lifetime_here)]
    UsedStaticLifetime {
        #[primary_span]
        span: Span,
    },
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum DefiningTypeNote<'a> {
    #[note(borrowck::define_type_with_closure_substs)]
    Closure { type_name: &'a str, subsets: &'a str },
    #[note(borrowck::define_type_with_generator_substs)]
    Generator { type_name: &'a str, subsets: &'a str },
    #[note(borrowck::define_type)]
    FnDef { type_name: &'a str },
    #[note(borrowck::define_const_type)]
    Const { type_name: &'a str },
    #[note(borrowck::define_inline_constant_type)]
    InlineConst { type_name: &'a str },
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::borrowed_temporary_value_dropped, code = "E0716")]
pub(crate) struct TemporaryDroppedErr {
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::thread_local_outlive_function, code = "E0712")]
pub(crate) struct ThreadLocalOutliveErr {
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::closure_borrowing_outlive_function, code = "E0373")]
pub(crate) struct ClosureVarOutliveErr<'a> {
    pub closure_kind: &'a str,
    pub borrowed_path: &'a str,
    #[primary_span]
    #[label]
    pub closure_span: Span,
    #[label(borrowck::path_label)]
    pub capture_span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::cannot_return_ref_to_local, code = "E0515")]
pub(crate) struct ReturnRefLocalErr<'a> {
    pub return_kind: &'a str,
    pub reference: &'a str,
    pub local: &'a str,
    #[primary_span]
    #[label]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::path_does_not_live_long_enough, code = "E0597")]
pub(crate) struct PathShortLive<'a> {
    pub path: &'a str,
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::cannot_borrow_across_destructor, code = "E0713")]
pub(crate) struct BorrowAcrossDestructor {
    #[primary_span]
    pub borrow_span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::cannot_borrow_across_generator_yield, code = "E0626")]
pub(crate) struct BorrowAcrossGeneratorYield {
    #[primary_span]
    pub span: Span,
    #[label]
    pub yield_span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::cannot_mutate_in_immutable_section, code = "E0510")]
pub(crate) struct MutateInImmute<'a> {
    pub action: &'a str,
    pub immutable_place: &'a str,
    pub immutable_section: &'a str,
    #[primary_span]
    #[label]
    pub mutate_span: Span,
    #[label(borrowck::immutable_value_label)]
    pub immutable_span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::cannot_act_moved_value, code = "E0382")]
pub(crate) struct ActMovedValueErr<'a> {
    pub verb: &'a str,
    pub optional_adverb_for_moved: &'a str,
    pub moved_path: String,
    #[primary_span]
    pub use_span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::cannot_move_out_of_interior_of_drop, code = "E0509")]
pub(crate) struct InteriorDropMoveErr<'a> {
    pub container_ty: Ty<'a>,
    #[primary_span]
    #[label]
    pub move_from_span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::cannot_move_out_of, code = "E0507")]
pub(crate) struct MovedOutErr<'a> {
    pub move_from_desc: &'a str,
    #[primary_span]
    pub move_from_span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::cannot_assign, code = "E0594")]
pub(crate) struct AssignErr<'a> {
    pub desc: &'a str,
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::cannot_assign_immutable_argument, code = "E0384")]
pub(crate) struct ImmuteArgAssign<'a> {
    pub desc: &'a str,
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::cannot_reassign_immutable_variable, code = "E0384")]
pub(crate) struct ImmuteVarReassign<'a> {
    pub desc: &'a str,
    #[primary_span]
    pub span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::cannot_assign_to_borrowed, code = "E0506")]
pub(crate) struct AssignBorrowErr<'a> {
    pub desc: &'a str,
    #[primary_span]
    #[label]
    pub span: Span,
    #[label(borrowck::borrow_here_label)]
    pub borrow_span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::cannot_move_out_of_interior_noncopy, code = "E0508")]
pub(crate) struct InteriorNoncopyMoveErr<'a, 'b> {
    pub ty: Ty<'a>,
    pub type_name: &'b str,
    #[primary_span]
    #[label]
    pub move_from_span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::cannot_reborrow_already_uniquely_borrowed, code = "E0501")]
pub(crate) struct UniquelyBorrowReborrowErr<'a> {
    pub container_name: &'a str,
    pub desc_new: &'a str,
    pub opt_via: &'a str,
    pub kind_new: &'a str,
    pub old_opt_via: &'a str,
    pub second_borrow_desc: &'a str,
    #[primary_span]
    #[label]
    pub new_loan_span: Span,
    #[label(borrowck::old_span_label)]
    pub old_loan_span: Span,
    #[label(borrowck::optional_label)]
    pub previous_end_span: Option<Span>,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::cannot_uniquely_borrow_by_one_closure, code = "E0501")]
pub(crate) struct ClosureUniquelyBorrowErr<'a> {
    pub container_name: &'a str,
    pub desc_new: &'a str,
    pub opt_via: &'a str,
    pub noun_old: &'a str,
    pub old_opt_via: &'a str,
    #[primary_span]
    #[label]
    pub new_loan_span: Span,
    #[label(borrowck::old_span_label)]
    pub old_loan_span: Span,
    #[label(borrowck::optional_label)]
    pub previous_end_span: Option<Span>,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::borrowed_data_escapes_closure, code = "E0521")]
pub(crate) struct BorrowEscapeClosure<'a> {
    pub escapes_from: &'a str,
    #[primary_span]
    pub escape_span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::cannot_uniquely_borrow_by_two_closures, code = "E0524")]
pub(crate) struct TwoClosuresUniquelyBorrowErr<'a> {
    pub desc: &'a str,
    #[subdiagnostic]
    pub case: ClosureConstructLabel,
    #[primary_span]
    pub new_loan_span: Span,
    #[label]
    pub old_load_end_span: Option<Span>,
    #[label(borrowck::new_span_label)]
    pub diff_span: Option<Span>,
}

#[derive(SessionSubdiagnostic)]
pub(crate) enum ClosureConstructLabel {
    #[label(borrowck::first_closure_constructed_here)]
    First {
        #[primary_span]
        old_loan_span: Span,
    },
    #[label(borrowck::closures_constructed_here)]
    Both {
        #[primary_span]
        old_loan_span: Span,
    },
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::cannot_use_when_mutably_borrowed, code = "E0503")]
pub(crate) struct UseMutBorrowErr<'a> {
    pub desc: &'a str,
    pub borrow_desc: &'a str,
    #[primary_span]
    #[label]
    pub span: Span,
    #[label(borrowck::borrow_span_label)]
    pub borrow_span: Span,
}

#[derive(SessionDiagnostic)]
#[diag(borrowck::cannot_move_when_borrowed, code = "E0505")]
pub(crate) struct MoveBorrowedErr<'a> {
    pub desc: &'a str,
    #[primary_span]
    pub span: Span,
}
