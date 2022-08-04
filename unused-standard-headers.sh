#!/bin/bash

# Print unused standard headers.
# Print unused forward declarations.

# This script does not modify files.

declare -A symbolsPerHeader=(
    [array]="array to_array"
    [vector]="vector"
    [deque]="deque"
    [forward_list]="forward_list"
    [list]="list"
    [stack]="stack"
    [queue]="queue priority_queue"
    [set]="set multiset"
    [map]="map multimap"
    [unordered_set]="unordered_set unordered_multiset"
    [unordered_map]="unordered_map unordered_multimap"
    [string]="char_traits basic_string string u8string u16string u32string wstring getline stoi stol stoll stoul stoull stof stod stold to_string to_wstring"
    [chrono]="chrono" # chrono is a namespace
    [sstream]="basic_stringbuf basic_istringstream basic_ostringstream basic_stringstream stringbuf wstringbuf istringstream wistringstream ostringstream wostringstream stringstream wstringstream"
    [ios]="ios_base basic_ios ios wios fpos io_errc is_error_code_enum streamoff streamsize iostream_category make_error_code make_error_condition boolalpha noboolalpha showbase noshowbase showpoint noshowpoint showpos noshowpos skipws noskipws uppercase nouppercase unitbuf nounitbuf internal left right dec hex oct fixed scientific hexfloat defaultfloat"
    [ostream]="basic_ostream ostream wostream endl ends flush emit_on_flush noemit_on_flush flush_emit"
    [istream]="basic_istream istream wistream basic_iostream iostream wiostream ws"
    [streambuf]="basic_streambuf streambuf wstreambuf"
    [optional]="optional bad_optional_access nullopt_t make_optional"
    [thread]="this_thread thread jthread" # this_thread is a namespace
    [regex]="regex_match regex_search regex_replace regex_iterator regex_token_iterator basic_regex match_results sub_match regex_traits regex_error regex wregex cmatch wcmatch smatch wsmatch csub_match wcsub_match ssub_match wssub_match regex_constants"
    [concepts]="same_as derived_from convertible_to common_reference_with common_with integral signed_integral unsigned_integral floating_point assignable_from swappable swappable_with destructible constructible_from default_initializable move_constructible copy_constructible equality_comparable equality_comparable_with totally_ordered totally_ordered_with movable copyable semiregular regular invocable regular_invocable predicate relation equivalence_relation strict_weak_order" # ranges::swap ???
    [any]="any bad_any_cast make_any any_cast"
    [compare]="three_way_comparable three_way_comparable_with partial_ordering weak_ordering strong_ordering common_comparison_category compare_three_way_result compare_three_way strong_order weak_order partial_order compare_strong_order_fallback compare_weak_order_fallback compare_partial_order_fallback is_eq is_neq is_lt is_lteq is_gt is_gteq"
    [csetjmp]="jmp_buf setjmp longjmp"
    [csignal]="sig_atomic_t SIGABRT SIGFPE SIGILL SIGINT SIGSEGV SIGTERM SIG_DFL SIG_IGN SIG_ERR signal raise"
    [cstdarg]="va_list va_start va_arg va_copy va_end"
    [cstddef]="NULL offsetof size_t ptrdiff_t nullptr_t max_align_t byte"
    [cstdlib]="div_t ldiv_t lldiv_t size_t EXIT_SUCCESS EXIT_FAILURE MB_CUR_MAX NULL RAND_MAX abort exit quick_exit _Exit atexit at_quick_exit system getenv malloc aligned_alloc calloc realloc free atof atoi atol atoll strtol strtoll strtoul strtoull strtof strtod strtold mblen mbtowc wctomb mbstowcs wcstombs rand srand qsort bsearch abs labs llabs div ldiv lldiv"
    [ctime]="CLOCKS_PER_SEC NULL clock_t size_t time_t tm timespec clock time difftime timespec_get ctime asctime strftime wcsftime gmtime localtime mktime"
    [expected]="expected unexpected bad_expected_access unexpect_t unexpect"
    [functional]="placeholders function move_only_function mem_fn reference_wrapper unwrap_reference unwrap_ref_decay bad_function_call is_bind_expression is_placeholder plus minus multiplies divides  modulus negate equal_to  not_equal_to greater less greater_equal less_equal ranges compare_three_way logical_and logical_or logical_not bit_and bit_or bit_xor bit_not not_fn identity default_searcher boyer_moore_searcher boyer_moore_horspool_searcher bind_front bind_back bind ref cref invoke invoke_r unary_function binary_function binder1st binder2nd bind1st bind2nd pointer_to_unary_function pointer_to_binary_function ptr_fun mem_fun_t mem_fun1_t const_mem_fun_t const_mem_fun1_t mem_fun mem_fun_ref_t mem_fun1_ref_t const_mem_fun_ref_t const_mem_fun1_ref_t mem_fun_ref unary_negate binary_negate not1 not2"
    [initializer_list]="initializer_list"
    [source_location]="source_location"
    [type_traits]="integral_constant bool_constant true_type false_type is_void is_null_pointer is_integral is_floating_point is_array is_enum is_union is_class is_function is_pointer is_lvalue_reference is_rvalue_reference is_member_object_pointer is_member_function_pointer is_fundamental is_arithmetic is_scalar is_object is_compound is_reference is_member_pointer is_const is_volatile is_trivial is_trivially_copyable is_standard_layout is_pod is_literal_type has_unique_object_representations is_empty is_polymorphic is_abstract is_final is_aggregate is_signed is_unsigned is_bounded_array is_unbounded_array is_scoped_enum is_constructible is_trivially_constructible is_nothrow_constructible is_default_constructible is_trivially_default_constructible is_nothrow_default_constructible is_copy_constructible is_trivially_copy_constructible is_nothrow_copy_constructible is_move_constructible is_trivially_move_constructible is_nothrow_move_constructible is_assignable is_trivially_assignable is_nothrow_assignable is_copy_assignable is_trivially_copy_assignable is_nothrow_copy_assignable is_move_assignable is_trivially_move_assignable is_nothrow_move_assignable is_destructible is_trivially_destructible is_nothrow_destructible has_virtual_destructor is_swappable_with is_swappable is_nothrow_swappable_with is_nothrow_swappable alignment_of rank extent is_same is_base_of is_convertible is_nothrow_convertible is_layout_compatible is_pointer_interconvertible_base_of is_invocable is_invocable_r is_nothrow_invocable is_nothrow_invocable_r remove_cv remove_const remove_volatile add_cv add_const add_volatile remove_reference add_lvalue_reference add_rvalue_reference remove_pointer add_pointer make_signed make_unsigned remove_extent remove_all_extents aligned_storage aligned_union decay remove_cvref enable_if conditional common_type common_reference basic_common_reference underlying_type result_of invoke_result void_t type_identity conjunction disjunction  negation is_pointer_interconvertible_with_class is_corresponding_member is_constant_evaluated"
    [typeinfo]="type_info bad_typeid bad_cast"
    [memory_resource]="polymorphic_allocator memory_resource pool_options synchronized_pool_resource unsynchronized_pool_resource monotonic_buffer_resource new_delete_resource null_memory_resource get_default_resource set_default_resource"
    [new]="bad_alloc bad_array_new_length nothrow_t align_val_t destroying_delete_t new_handler nothrow destroying_delete hardware_destructive_interference_size hardware_constructive_interference_size get_new_handler set_new_handler launder"
    [scoped_allocator]="scoped_allocator_adaptor"
    [cfloat]="FLT_ROUNDS FLT_EVAL_METHOD FLT_HAS_SUBNORM DBL_HAS_SUBNORM LDBL_HAS_SUBNORM FLT_RADIX FLT_MANT_DIG DBL_MANT_DIG LDBL_MANT_DIG FLT_DECIMAL_DIG DBL_DECIMAL_DIG LDBL_DECIMAL_DIG DECIMAL_DIG FLT_DIG DBL_DIG LDBL_DIG FLT_MIN_EXP DBL_MIN_EXP LDBL_MIN_EXP FLT_MIN_10_EXP DBL_MIN_10_EXP LDBL_MIN_10_EXP FLT_MAX_EXP DBL_MAX_EXP LDBL_MAX_EXP FLT_MAX_10_EXP DBL_MAX_10_EXP LDBL_MAX_10_EXP FLT_MAX DBL_MAX LDBL_MAX FLT_EPSILON DBL_EPSILON LDBL_EPSILON FLT_MIN DBL_MIN LDBL_MIN FLT_TRUE_MIN DBL_TRUE_MIN LDBL_TRUE_MIN"
    [climits]="CHAR_BIT SCHAR_MIN SCHAR_MAX UCHAR_MAX CHAR_MIN CHAR_MAX MB_LEN_MAX SHRT_MIN SHRT_MAX USHRT_MAX INT_MIN INT_MAX UINT_MAX LONG_MIN LONG_MAX ULONG_MAX LLONG_MIN LLONG_MAX ULLONG_MAX"
    [cstdint]="int8_t int16_t int32_t int64_t int_fast8_t int_fast16_t int_fast32_t int_fast64_t int_least8_t int_least16_t int_least32_t int_least64_t intmax_t intptr_t uint8_t uint16_t uint32_t uint64_t uint_fast8_t uint_fast16_t uint_fast32_t uint_fast64_t uint_least8_t uint_least16_t uint_least32_t uint_least64_t uintmax_t uintptr_t INTN_MIN INTN_MAX UINTN_MAX INT_FASTN_MIN INT_FASTN_MAX UINT_FASTN_MAX INT_LEASTN_MIN INT_LEASTN_MAX UINT_LEASTN_MAX INTMAX_MIN INTMAX_MAX UINTMAX_MAX INTPTR_MIN INTPTR_MAX UINTPTR_MAX PTRDIFF_MIN PTRDIFF_MAX SIZE_MAX SIG_ATOMIC_MIN SIG_ATOMIC_MAX WCHAR_MIN WCHAR_MAX WINT_MIN WINT_MAX INTN_C UINTN_C INTMAX_C UINTMAX_C"
    [limits]="numeric_limits float_round_style float_denorm_style"
    [cassert]="assert"
    [cerrno]="errno E2BIG EACCES EADDRINUSE EADDRNOTAVAIL EAFNOSUPPORT EAGAIN EALREADY EBADF EBADMSG EBUSY ECANCELED ECHILD ECONNABORTED ECONNREFUSED ECONNRESET EDEADLK EDESTADDRREQ EDOM EEXIST EFAULT EFBIG EHOSTUNREACH EIDRM EILSEQ EINPROGRESS EINTR EINVAL EIO EISCONN EISDIR ELOOP EMFILE EMLINK EMSGSIZE ENAMETOOLONG ENETDOWN ENETRESET ENETUNREACH ENFILE ENOBUFS ENODATA ENODEV ENOENT ENOEXEC ENOLCK ENOLINK ENOMEM ENOMSG ENOPROTOOPT ENOSPC ENOSR ENOSTR ENOSYS ENOTCONN ENOTDIR ENOTEMPTY ENOTRECOVERABLE ENOTSOCK ENOTSUP ENOTTY ENXIO EOPNOTSUPP EOVERFLOW EOWNERDEAD EPERM EPIPE EPROTO EPROTONOSUPPORT EPROTOTYPE ERANGE EROFS ESPIPE ESRCH ETIME ETIMEDOUT ETXTBSY EWOULDBLOCK EXDEV"
    [exception]="exception nested_exception bad_exception unexpected_handler terminate_handler exception_ptr unexpected uncaught_exception uncaught_exceptions make_exception_ptr current_exception rethrow_exception throw_with_nested rethrow_if_nested terminate get_terminate set_terminate get_unexpected set_unexpected"
    [stacktrace]="stacktrace_entry basic_stacktrace stacktrace" # to_string operator<<
    [stdexcept]="logic_error invalid_argument domain_error length_error out_of_range runtime_error range_error overflow_error underflow_error"
    [cctype]="isalnum isalpha islower isupper isdigit isxdigit iscntrl isgraph isspace isblank isprint ispunct tolower toupper"
    [charconv]="chars_format from_chars to_chars"
    [cstring]="NULL size_t strcpy strncpy strcat strncat strxfrm strlen strcmp strncmp strcoll strchr strrchr strspn strcspn strpbrk strstr strtok memchr memcmp memset memcpy memmove strerror"
    [cuchar]="__STDC_UTF_16__ __STDC_UTF_32__ mbstate_t size_t mbrtoc16 c16rtomb mbrtoc32 c32rtomb mbrtoc8 c8rtomb"
    [cwchar]="NULL WEOF WCHAR_MIN WCHAR_MAX mbstate_t size_t wint_t tm wcscpy wcsncpy wcscat wcsncat wcsxfrm wcslen wcscmp wcsncmp wcscoll wcschr wcsrchr wcsspn wcscspn wcspbrk wcsstr wcstok wmemcpy wmemmove wmemcmp wmemchr wmemset mbsinit btowc wctob mbrlen mbrtowc wcrtomb mbsrtowcs wcsrtombs fgetwc getwc fgetws fputwc putwc fputws getwchar putwchar ungetwc fwide wscanf fwscanf swscanf vwscanf vfwscanf vswscanf wprintf fwprintf swprintf vwprintf vfwprintf vswprintf wcsftime wcstol wcstoll wcstoul wcstoull wcstof wcstod wcstold"
    [cwctype]="wctrans_t wctype_t wint_t WEOF iswalnum iswalpha iswlower iswupper iswdigit iswxdigit iswcntrl iswgraph iswspace iswblank iswprint iswpunct iswctype wctype towlower towupper towctrans wctrans"
    [format]="formatter basic_format_parse_context format_parse_context wformat_parse_context basic_format_context format_context wformat_context basic_format_arg basic_format_args format_args wformat_args format_error format format_to format_to_n formatted_size vformat vformat_to visit_format_arg make_format_args make_wformat_args"
    [span]="span dynamic_extent as_bytes as_writable_bytes"
)

symbolsPerHeader[iostream]+="cin wcin cout wcout cerr wcerr clog wclog ${symbolsPerHeader[ios]} ${symbolsPerHeader[streambuf]} ${symbolsPerHeader[istream]} ${symbolsPerHeader[ostream]}"
symbolsPerHeader[coroutine]+="${symbolsPerHeader[compare]} coroutine_traits coroutine_handle noop_coroutine_promise noop_coroutine_handle suspend_never suspend_always noop_coroutine"
symbolsPerHeader[bitset]+="${symbolsPerHeader[string]} ${symbolsPerHeader[iosfwd]} bitset"
symbolsPerHeader[tuple]+="${symbolsPerHeader[compare]} tuple tuple_size tuple_element tuple_size tuple_element uses_allocator ignore make_tuple tie forward_as_tuple tuple_cat get apply make_from_tuple"
symbolsPerHeader[typeindex]+="${symbolsPerHeader[compare]} type_index"
symbolsPerHeader[bad_cast]+="${symbolsPerHeader[compare]} ${symbolsPerHeader[initializer_list]} rel_ops swap exchange forward forward_like move move_if_noexcept as_const declval to_underlying cmp_equal cmp_not_equal cmp_less cmp_greater cmp_less_equal cmp_greater_equal in_range unreachable make_pair pair tuple_size tuple_element integer_sequence piecewise_construct_t piecewise_construct in_place in_place_type in_place_index in_place_t in_place_type_t in_place_index_t"
symbolsPerHeader[variant]+="${symbolsPerHeader[compare]} variant monostate bad_variant_access variant_size variant_size_v variant_alternative variant_alternative_t variant_npos visit holds_alternative get_if"
symbolsPerHeader[memory]+="${symbolsPerHeader[compare]} pointer_traits pointer_safety allocator allocator_traits allocation_result allocator_arg_t uses_allocator raw_storage_iterator unique_ptr shared_ptr weak_ptr auto_ptr owner_less enable_shared_from_this bad_weak_ptr default_delete out_ptr_t inout_ptr_t allocator_arg allocate_at_least to_address addressof align assume_aligned declare_reachable undeclare_reachable declare_no_pointers undeclare_no_pointers get_pointer_safety uninitialized_copy uninitialized_copy_n uninitialized_fill uninitialized_fill_n uninitialized_move uninitialized_move_n uninitialized_default_construct uninitialized_default_construct_n uninitialized_value_construct uninitialized_value_construct_n construct_at destroy_at destroy destroy_n get_temporary_buffer return_temporary_buffer make_unique make_unique_for_overwrite make_shared make_shared_for_overwrite allocate_shared allocate_shared_for_overwrite static_pointer_cast dynamic_pointer_cast const_pointer_cast reinterpret_pointer_cast get_deleter out_ptr inout_ptr ranges::uninitialized_copy ranges::uninitialized_copy_n ranges::uninitialized_fill ranges::uninitialized_fill_n ranges::uninitialized_move ranges::uninitialized_move_n ranges::uninitialized_default_construct ranges::uninitialized_default_construct_n ranges::uninitialized_value_construct ranges::uninitialized_value_construct_n ranges::construct_at ranges::destroy_at ranges::destroy ranges::destroy_n" # operator<<
symbolsPerHeader[cinttypes]+="${symbolsPerHeader[inttypes.h]} imaxdiv_t abs imaxabs div imaxdiv strtoimax strtoumax wcstoimax wcstoumax PRIdN PRIiN PRIoN PRIuN PRIxN PRIXN SCNdN SCNiN SCNoN SCNuN SCNxN PRIdLEASTN PRIiLEASTN PRIoLEASTN PRIuLEASTN PRIxLEASTN PRIXLEASTN SCNdLEASTN SCNiLEASTN SCNoLEASTN SCNuLEASTN SCNxLEASTN PRIdFASTN PRIiFASTN PRIoFASTN PRIuFASTN PRIxFASTN PRIXFASTN SCNdFASTN SCNiFASTN SCNoFASTN SCNuFASTN SCNxFASTN PRIdMAX PRIiMAX PRIoMAX PRIuMAX PRIxMAX PRIXMAX SCNdMAX SCNiMAX SCNoMAX SCNuMAX SCNxMAX PRIdPTR PRIiPTR PRIoPTR PRIuPTR PRIxPTR PRIXPTR SCNdPTR SCNiPTR SCNoPTR SCNuPTR SCNxPTR"
symbolsPerHeader[system_error]+="${symbolsPerHeader[compare]} error_category error_condition errc error_code system_error is_error_code_enum is_error_condition_enum generic_category system_category make_error_code make_error_condition"
symbolsPerHeader[string_view]+="${symbolsPerHeader[compare]} basic_string_view string_view u8string_view u16string_view u32string_view wstring_view sv" # operator""sv

function get_cpp_files() {
    for path in "$@"; do
        if [[ -f "$path" ]]; then
            echo "$path"
        elif [[ -d "$path" ]]; then
            find "$path" -name "*.c" -or -name "*.cpp" -or -name "*.tcc" -or -name "*.h" -or -name "*.hpp"
        else
            echo "Neither ordinary file nor directory: $path"
        fi
    done
}

function check_if_file_has_unused_forward_declarations() {
    local file="$1"

    # Find all forward declarations
    local forward_declarations=$(grep '^\s*class .*;$' "$file" | sed -e 's/.*class //' -e 's/;$//')

    for fwd_decl in $forward_declarations; do
        # Remove all declarations from the file - forward or not
        grep -v class "$file" |
            # If the class name is not used, report it
            grep -q "$fwd_decl" || echo "Useless forward declaration: $fwd_decl in file $file"
    done
}

function check_if_header_is_unused_in_file() {
    local header="$1"
    local file="$2"

    # If the header is not included, just exit
    grep -q "#include.*$header" "$file" || return

    grep_pattern="\\b${symbolsPerHeader[$header]// /\\b\\|\\b}\\b"

    # Remove all #include lines
    grep -v '#include' "$file" |
        # If no symbol from this header is found, report it
        grep -q "$grep_pattern" || echo "Useless header: $header in file $file"
}

if [[ -z "$@" ]]; then
    cpp_files=$(get_cpp_files .)
else
    cpp_files=$(get_cpp_files "$@")
fi

for file in $cpp_files; do
    check_if_file_has_unused_forward_declarations "$file"

    for header in "${!symbolsPerHeader[@]}"; do
        check_if_header_is_unused_in_file "$header" "$file"
    done
done
