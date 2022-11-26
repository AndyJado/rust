use rustc_errors::{IntoDiagnosticArg, MultiSpan};
use rustc_macros::{Diagnostic, LintDiagnostic, Subdiagnostic};
use rustc_middle::ty::Ty;
use rustc_span::Span;

use crate::diagnostics::RegionName;

#[derive(Diagnostic)]
#[diag(borrowck_move_unsized, code = "E0161")]
pub(crate) struct MoveUnsized<'tcx> {
    pub ty: Ty<'tcx>,
    #[primary_span]
    #[label]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(borrowck_higher_ranked_lifetime_error)]
pub(crate) struct HigherRankedLifetimeError {
    #[subdiagnostic]
    pub cause: Option<HigherRankedErrorCause>,
    #[primary_span]
    pub span: Span,
}

#[derive(Subdiagnostic)]
pub(crate) enum HigherRankedErrorCause {
    #[note(borrowck_could_not_prove)]
    CouldNotProve { predicate: String },
    #[note(borrowck_could_not_normalize)]
    CouldNotNormalize { value: String },
}

#[derive(Diagnostic)]
#[diag(borrowck_higher_ranked_subtype_error)]
pub(crate) struct HigherRankedSubtypeError {
    #[primary_span]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(borrowck_generic_does_not_live_long_enough)]
pub(crate) struct GenericDoesNotLiveLongEnough {
    pub kind: String,
    #[primary_span]
    pub span: Span,
}

#[derive(LintDiagnostic)]
#[diag(borrowck_var_does_not_need_mut)]
pub(crate) struct VarNeedNotMut {
    #[suggestion(style = "short", applicability = "machine-applicable", code = "")]
    pub span: Span,
}
#[derive(Diagnostic)]
#[diag(borrowck_var_cannot_escape_closure)]
#[note]
#[note(cannot_escape)]
pub(crate) struct FnMutError {
    #[primary_span]
    pub span: Span,
    #[subdiagnostic]
    pub ty_err: FnMutReturnTypeErr,
}

#[derive(Subdiagnostic)]
pub(crate) enum VarHereDenote {
    #[label(borrowck_var_here_captured)]
    Captured {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck_var_here_defined)]
    Defined {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck_closure_inferred_mut)]
    FnMutInferred {
        #[primary_span]
        span: Span,
    },
}

#[derive(Subdiagnostic)]
pub(crate) enum FnMutReturnTypeErr {
    #[label(borrowck_returned_closure_escaped)]
    ReturnClosure {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck_returned_async_block_escaped)]
    ReturnAsyncBlock {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck_returned_ref_escaped)]
    ReturnRef {
        #[primary_span]
        span: Span,
    },
}

#[derive(Diagnostic)]
#[diag(borrowck_lifetime_constraints_error)]
pub(crate) struct LifetimeOutliveErr {
    #[primary_span]
    pub span: Span,
}

#[derive(Subdiagnostic)]
pub(crate) enum LifetimeReturnCategoryErr<'a> {
    #[label(borrowck_returned_lifetime_wrong)]
    WrongReturn {
        #[primary_span]
        span: Span,
        mir_def_name: &'a str,
        outlived_fr_name: RegionName,
        fr_name: &'a RegionName,
    },
    #[label(borrowck_returned_lifetime_short)]
    ShortReturn {
        #[primary_span]
        span: Span,
        category: &'a str,
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

#[derive(Subdiagnostic)]
pub(crate) enum RequireStaticErr {
    #[note(borrowck_used_impl_require_static)]
    UsedImpl {
        #[primary_span]
        multi_span: MultiSpan,
    },
}

#[derive(Subdiagnostic)]
pub(crate) enum CaptureVarPathUseCause {
    #[label(borrowck_borrow_due_to_use_generator)]
    BorrowInGenerator {
        #[primary_span]
        path_span: Span,
    },
    #[label(borrowck_use_due_to_use_generator)]
    UseInGenerator {
        #[primary_span]
        path_span: Span,
    },
    #[label(borrowck_assign_due_to_use_generator)]
    AssignInGenerator {
        #[primary_span]
        path_span: Span,
    },
    #[label(borrowck_assign_part_due_to_use_generator)]
    AssignPartInGenerator {
        #[primary_span]
        path_span: Span,
    },
    #[label(borrowck_borrow_due_to_use_closure)]
    BorrowInClosure {
        #[primary_span]
        path_span: Span,
    },
    #[label(borrowck_use_due_to_use_closure)]
    UseInClosure {
        #[primary_span]
        path_span: Span,
    },
    #[label(borrowck_assign_due_to_use_closure)]
    AssignInClosure {
        #[primary_span]
        path_span: Span,
    },
    #[label(borrowck_assign_part_due_to_use_closure)]
    AssignPartInClosure {
        #[primary_span]
        path_span: Span,
    },
}

#[derive(Subdiagnostic)]
pub(crate) enum CaptureVarKind {
    #[label(borrowck_capture_immute)]
    Immut {
        #[primary_span]
        kind_span: Span,
    },
    #[label(borrowck_capture_mut)]
    Mut {
        #[primary_span]
        kind_span: Span,
    },
    #[label(borrowck_capture_move)]
    Move {
        #[primary_span]
        kind_span: Span,
    },
}

#[derive(Subdiagnostic)]
pub(crate) enum CaptureVarCause {
    #[label(borrowck_var_borrow_by_use_place_in_generator)]
    BorrowUsePlaceGenerator {
        is_single_var: bool,
        place: String,
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_var_borrow_by_use_place_in_closure)]
    BorrowUsePlaceClosure {
        is_single_var: bool,
        place: String,
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_var_borrow_by_use_in_generator)]
    BorrowUseInGenerator {
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_var_borrow_by_use_in_closure)]
    BorrowUseInClosure {
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_var_move_by_use_in_generator)]
    MoveUseInGenerator {
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_var_move_by_use_in_closure)]
    MoveUseInClosure {
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_var_first_borrow_by_use_place_in_generator)]
    FirstBorrowUsePlaceGenerator {
        place: String,
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_var_first_borrow_by_use_place_in_closure)]
    FirstBorrowUsePlaceClosure {
        place: String,
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_var_second_borrow_by_use_place_in_generator)]
    SecondBorrowUsePlaceGenerator {
        place: String,
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_var_second_borrow_by_use_place_in_closure)]
    SecondBorrowUsePlaceClosure {
        place: String,
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_var_mutable_borrow_by_use_place_in_closure)]
    MutableBorrowUsePlaceClosure {
        place: String,
        #[primary_span]
        var_span: Span,
    },
    #[label(borrowck_partial_var_move_by_use_in_generator)]
    PartialMoveUseInGenerator {
        #[primary_span]
        var_span: Span,
        is_partial: bool,
    },
    #[label(borrowck_partial_var_move_by_use_in_closure)]
    PartialMoveUseInClosure {
        #[primary_span]
        var_span: Span,
        is_partial: bool,
    },
}

#[derive(Diagnostic)]
#[diag(borrowck_cannot_move_when_borrowed, code = "E0505")]
pub(crate) struct MoveBorrow<'a> {
    pub place: &'a str,
    pub borrow_place: &'a str,
    pub value_place: &'a str,
    #[primary_span]
    #[label(move_label)]
    pub span: Span,
    #[label]
    pub borrow_span: Span,
}

#[derive(Subdiagnostic)]
pub(crate) enum CaptureReasonLabel<'a> {
    #[label(borrowck_moved_due_to_call)]
    Call {
        #[primary_span]
        fn_call_span: Span,
        place_name: &'a str,
        is_partial: bool,
        is_loop_message: bool,
    },
    #[label(borrowck_moved_due_to_usage_in_operator)]
    OperatorUse {
        #[primary_span]
        fn_call_span: Span,
        place_name: &'a str,
        is_partial: bool,
        is_loop_message: bool,
    },
    #[label(borrowck_moved_due_to_implicit_into_iter_call)]
    ImplicitCall {
        #[primary_span]
        fn_call_span: Span,
        place_name: &'a str,
        is_partial: bool,
        is_loop_message: bool,
    },
    #[label(borrowck_moved_due_to_method_call)]
    MethodCall {
        #[primary_span]
        fn_call_span: Span,
        place_name: &'a str,
        is_partial: bool,
        is_loop_message: bool,
    },
    #[label(borrowck_value_moved_here)]
    Desc {
        #[primary_span]
        move_span: Span,
        is_partial: bool,
        is_move_msg: bool,
        is_loop_message: bool,
    },
    #[label(borrowck_consider_borrow_type_contents)]
    BorrowContent {
        #[primary_span]
        var_span: Span,
    },
}

#[derive(Subdiagnostic)]
pub(crate) enum CaptureReasonNote {
    #[note(borrowck_moved_a_fn_once_in_call)]
    FnOnceMoveInCall {
        #[primary_span]
        var_span: Span,
    },
    #[note(borrowck_calling_operator_moves_lhs)]
    LhsMoveByOperator {
        #[primary_span]
        span: Span,
    },
    #[note(borrowck_func_take_self_moved_place)]
    FuncTakeSelf {
        place_name: String,
        #[primary_span]
        span: Span,
    },
}

#[derive(Subdiagnostic)]
pub(crate) enum CaptureReasonSuggest<'a, 'tcx> {
    #[suggestion(
        borrowck_suggest_iterate_over_slice,
        applicability = "maybe-incorrect",
        code = "&",
        style = "verbose"
    )]
    IterateSlice {
        ty: Ty<'tcx>,
        #[primary_span]
        span: Span,
    },
    #[suggestion(
        borrowck_suggest_create_freash_reborrow,
        applicability = "machine-applicable",
        code = "&mut *",
        style = "verbose"
    )]
    FreshReborrow {
        place_name: &'a str,
        #[primary_span]
        span: Span,
    },
}

#[derive(Subdiagnostic)]
pub(crate) enum CaptureArgLabel {
    #[label(borrowck_value_capture_here)]
    Capture {
        is_within: bool,
        #[primary_span]
        args_span: Span,
    },
    #[label(borrowck_move_out_place_here)]
    MoveOutPlace {
        place: String,
        #[primary_span]
        args_span: Span,
    },
}

#[derive(Subdiagnostic)]
pub(crate) enum OnClosureNote<'a> {
    #[note(borrowck_closure_invoked_twice)]
    InvokedTwice {
        place_name: &'a str,
        #[primary_span]
        span: Span,
    },
    #[note(borrowck_closure_moved_twice)]
    MovedTwice {
        place_name: &'a str,
        #[primary_span]
        span: Span,
    },
}

#[derive(Subdiagnostic)]
pub(crate) enum TypeNoCopy<'a, 'tcx> {
    #[label(borrowck_ty_no_impl_copy)]
    Label {
        is_partial_move: bool,
        ty: Ty<'tcx>,
        place: &'a str,
        #[primary_span]
        span: Span,
    },
    #[note(borrowck_ty_no_impl_copy)]
    Note { is_partial_move: bool, ty: Ty<'tcx>, place: &'a str },
}

#[derive(Subdiagnostic)]
pub(crate) enum AddMoveErr {
    #[label(borrowck_data_moved_here)]
    Here {
        #[primary_span]
        binding_span: Span,
    },
    #[label(borrowck_and_data_moved_here)]
    AndHere {
        #[primary_span]
        binding_span: Span,
    },
    #[note(borrowck_moved_var_cannot_copy)]
    MovedNotCopy,
}

#[derive(Subdiagnostic)]
pub(crate) enum OnLifetimeBound<'a> {
    #[help(borrowck_consider_add_lifetime_bound)]
    Add { fr_name: &'a RegionName, outlived_fr_name: &'a RegionName },
}

#[derive(Subdiagnostic)]
pub(crate) enum FnMutBumpFn<'a> {
    #[label(borrowck_cannot_act)]
    Cannot {
        act: &'a str,
        #[primary_span]
        sp: Span,
    },
    #[label(borrowck_expects_fnmut_not_fn)]
    AcceptFnMut {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck_expects_fn_not_fnmut)]
    AcceptFn {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck_empty_label)]
    EmptyLabel {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck_in_this_closure)]
    Here {
        #[primary_span]
        span: Span,
    },
    #[label(borrowck_return_fnmut)]
    ReturnFnMut {
        #[primary_span]
        span: Span,
    },
}

#[derive(Subdiagnostic)]
pub(crate) enum RegionNameLabels<'a> {
    #[label(borrowck_name_this_region)]
    NameRegion {
        #[primary_span]
        span: Span,
        rg_name: &'a RegionName,
    },
    #[label(borrowck_lifetime_appears_in_type)]
    LifetimeInType {
        #[primary_span]
        span: Span,
        type_name: &'a str,
        rg_name: &'a RegionName,
    },
    #[label(borrowck_lifetime_appears_in_type_of)]
    LifetimeInTypeOf {
        #[primary_span]
        span: Span,
        upvar_name: String,
        rg_name: &'a RegionName,
    },
    #[label(borrowck_yield_type_is_type)]
    YieldTypeIsTpye {
        #[primary_span]
        span: Span,
        type_name: &'a str,
    },
    #[label(borrowck_lifetime_appears_here_in_impl)]
    LifetimeInImpl {
        #[primary_span]
        span: Span,
        rg_name: &'a RegionName,
        location: &'a str,
    },
}

#[derive(Diagnostic)]
#[diag(borrowck_type_parameter_not_used_in_trait_type_alias)]
pub(crate) struct UnusedTypeParameter<'tcx> {
    pub ty: Ty<'tcx>,
    #[primary_span]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(borrowck_non_defining_opaque_type)]
pub(crate) struct OpaqueTypeNotDefine {
    #[primary_span]
    pub span: Span,
    #[subdiagnostic]
    pub cause: OpaqueTyDefineErrCause,
}

#[derive(Subdiagnostic)]
pub(crate) enum OpaqueTyDefineErrCause {
    #[note(borrowck_used_non_generic_for_generic)]
    NonGenericUsed {
        #[primary_span]
        span: Span,
        descr: &'static str,
        arg: String,
    },
    #[label(borrowck_cannot_use_static_lifetime_here)]
    UsedStaticLifetime {
        #[primary_span]
        span: Span,
    },
}

#[derive(Subdiagnostic)]
pub(crate) enum DefiningTypeNote<'a> {
    #[note(borrowck_define_type_with_closure_substs)]
    Closure { type_name: &'a str, subsets: &'a str },
    #[note(borrowck_define_type_with_generator_substs)]
    Generator { type_name: &'a str, subsets: &'a str },
    #[note(borrowck_define_type)]
    FnDef { type_name: &'a str },
    #[note(borrowck_define_const_type)]
    Const { type_name: &'a str },
    #[note(borrowck_define_inline_constant_type)]
    InlineConst { type_name: &'a str },
}

#[derive(Diagnostic)]
#[diag(borrowck_borrowed_temporary_value_dropped, code = "E0716")]
pub(crate) struct TemporaryDroppedErr {
    #[primary_span]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(borrowck_thread_local_outlive_function, code = "E0712")]
pub(crate) struct ThreadLocalOutliveErr {
    #[primary_span]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(borrowck_closure_borrowing_outlive_function, code = "E0373")]
pub(crate) struct ClosureVarOutliveErr<'a> {
    pub closure_kind: &'a str,
    pub borrowed_path: &'a str,
    #[primary_span]
    #[label]
    pub closure_span: Span,
    #[label(path_label)]
    pub capture_span: Span,
}

#[derive(Diagnostic)]
#[diag(borrowck_cannot_return_ref_to_local, code = "E0515")]
pub(crate) struct ReturnRefLocalErr<'a> {
    pub return_kind: &'a str,
    pub reference: &'a str,
    pub local: &'a str,
    #[primary_span]
    #[label]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(borrowck_path_does_not_live_long_enough, code = "E0597")]
pub(crate) struct PathShortLive<'a> {
    pub path: &'a str,
    #[primary_span]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(borrowck_cannot_borrow_across_destructor, code = "E0713")]
pub(crate) struct BorrowAcrossDestructor {
    #[primary_span]
    pub borrow_span: Span,
}

#[derive(Diagnostic)]
#[diag(borrowck_cannot_borrow_across_generator_yield, code = "E0626")]
pub(crate) struct BorrowAcrossGeneratorYield {
    #[primary_span]
    pub span: Span,
    #[label]
    pub yield_span: Span,
}

#[derive(Diagnostic)]
#[diag(borrowck_cannot_move_out_of_interior_of_drop, code = "E0509")]
pub(crate) struct InteriorDropMoveErr<'a> {
    pub container_ty: Ty<'a>,
    #[primary_span]
    #[label]
    pub move_from_span: Span,
}

#[derive(Diagnostic)]
#[diag(borrowck_cannot_assign_to_borrowed, code = "E0506")]
pub(crate) struct AssignBorrowErr<'a> {
    pub desc: &'a str,
    #[primary_span]
    #[label]
    pub span: Span,
    #[label(borrow_here_label)]
    pub borrow_span: Span,
}

#[derive(Diagnostic)]
#[diag(borrowck_cannot_uniquely_borrow_by_two_closures, code = "E0524")]
pub(crate) struct TwoClosuresUniquelyBorrowErr<'a> {
    pub desc: &'a str,
    #[primary_span]
    pub new_loan_span: Span,
    #[label]
    pub old_load_end_span: Option<Span>,
    #[label(new_span_label)]
    pub diff_span: Option<Span>,
    #[subdiagnostic]
    pub case: ClosureConstructLabel,
}

#[derive(Subdiagnostic)]
pub(crate) enum ClosureConstructLabel {
    #[label(borrowck_first_closure_constructed_here)]
    First {
        #[primary_span]
        old_loan_span: Span,
    },
    #[label(borrowck_closures_constructed_here)]
    Both {
        #[primary_span]
        old_loan_span: Span,
    },
}

#[derive(Diagnostic)]
#[diag(borrowck_cannot_use_when_mutably_borrowed, code = "E0503")]
pub(crate) struct UseMutBorrowErr<'a> {
    pub desc: &'a str,
    pub borrow_desc: &'a str,
    #[primary_span]
    #[label]
    pub span: Span,
    #[label(borrow_span_label)]
    pub borrow_span: Span,
}

#[derive(Diagnostic)]
pub(crate) enum MutBorrowMulti<'a> {
    #[diag(borrowck_cannot_mutably_borrow_multiply_same_span, code = "E0499")]
    SameSpan {
        new_place_name: &'a str,
        place: &'a str,
        old_place: &'a str,
        is_place_empty: bool,
        #[primary_span]
        new_loan_span: Span,
        #[label]
        old_load_end_span: Option<Span>,
        #[subdiagnostic(eager)]
        eager_label: MutMultiLoopLabel<'a>,
    },
    #[diag(borrowck_cannot_mutably_borrow_multiply, code = "E0499")]
    ChangedSpan {
        new_place_name: &'a str,
        place: &'a str,
        old_place: &'a str,
        is_place_empty: bool,
        is_old_place_empty: bool,
        #[primary_span]
        #[label(second_mut_borrow_label)]
        new_loan_span: Span,
        #[label]
        old_loan_span: Span,
        #[label(first_mut_end_label)]
        old_load_end_span: Option<Span>,
    },
}

#[derive(Subdiagnostic)]
#[label(borrowck_mutably_borrow_multiply_loop_label)]
pub(crate) struct MutMultiLoopLabel<'a> {
    pub new_place_name: &'a str,
    pub place: &'a str,
    pub is_place_empty: bool,
    #[primary_span]
    pub new_loan_span: Span,
}

#[derive(Diagnostic)]
#[diag(borrowck_cannot_uniquely_borrow_by_one_closure, code = "E0500")]
pub(crate) struct ClosureUniquelyBorrowErr<'a> {
    #[primary_span]
    #[label]
    pub new_loan_span: Span,
    pub is_generator: bool,
    pub desc_new: &'a str,
    pub opt_via: &'a str,
    #[label(occurs_label)]
    pub old_loan_span: Span,
    pub noun_old: &'a str,
    pub old_opt_via: &'a str,
    #[label(ends_label)]
    pub previous_end_span: Option<Span>,
}

#[derive(Diagnostic)]
#[diag(borrowck_cannot_reborrow_already_uniquely_borrowed, code = "E0501")]
pub(crate) struct ClosureReBorrowErr<'a> {
    #[primary_span]
    #[label]
    pub new_loan_span: Span,
    pub is_generator: &'a str,
    pub desc_new: &'a str,
    pub opt_via: &'a str,
    pub kind_new: &'a str,
    #[label(occurs_label)]
    pub old_loan_span: Span,
    pub old_opt_via: &'a str,
    #[label(ends_label)]
    pub previous_end_span: Option<Span>,
    pub second_borrow_desc: &'a str,
}
