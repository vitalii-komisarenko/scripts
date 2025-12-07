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
    [iterator]="indirectly_readable indirectly_writable weakly_incrementable incrementable input_or_output_iterator sentinel_for sized_sentinel_for input_iterator output_iterator forward_iterator bidirectional_iterator random_access_iterator contiguous_iterator indirectly_unary_invocable indirectly_regular_unary_invocable indirect_unary_predicate indirect_binary_predicate indirect_equivalence_relation indirect_strict_weak_order indirectly_movable indirectly_movable_storable indirectly_copyable indirectly_copyable_storable indirectly_swappable indirectly_comparable permutable mergeable sortable indirect_result_t projected incrementable_traits indirectly_readable_traits iter_value_t iter_reference_t iter_const_reference_t iter_difference_t iter_rvalue_reference_t iter_common_reference_t iterator_traits input_iterator_tag output_iterator_tag forward_iterator_tag bidirectional_iterator_tag random_access_iterator_tag contiguous_iterator_tag iterator reverse_iterator move_iterator move_sentinel common_iterator default_sentinel_t counted_iterator unreachable_sentinel_t back_insert_iterator front_insert_iterator insert_iterator istream_iterator ostream_iterator istreambuf_iterator ostreambuf_iterator iter_move iter_swap unreachable_sentinel default_sentinel make_reverse_iterator make_move_iterator front_inserter back_inserter inserter advance distance next prev ranges::advance ranges::distance ranges::next ranges::prev"
    [ranges]="ranges::range ranges::borrowed_range ranges::sized_range ranges::view ranges::input_range ranges::output_range ranges::forward_range ranges::bidirectional_range ranges::random_access_range ranges::contiguous_range ranges::common_range ranges::viewable_range ranges::constant_range ranges::to ranges::iterator_t ranges::const_iterator_t ranges::sentinel_t ranges::range_difference_t ranges::range_size_t ranges::range_value_t ranges::range_reference_t ranges::range_const_reference_t ranges::range_rvalue_reference_t ranges::view_interface ranges::subrange ranges::dangling ranges::borrowed_iterator_t ranges::borrowed_subrange_t ranges::range_adaptor_closure ranges::empty_view views::empty ranges::single_view views::single ranges::iota_view views::iota ranges::basic_istream_view views::istream ranges::repeat_view views::repeat ranges::cartesian_product_view views::cartesian_product views::all_t views::all ranges::ref_view ranges::owning_view ranges::filter_view views::filter ranges::transform_view views::transform ranges::take_view views::take ranges::take_while_view views::take_while ranges::drop_view views::drop ranges::drop_while_view views::drop_while ranges::join_view views::join ranges::lazy_split_view views::lazy_split ranges::split_view views::split views::counted ranges::common_view views::common ranges::reverse_view views::reverse ranges::as_const_view views::as_const ranges::as_rvalue_view views::as_rvalue ranges::elements_view views::elements ranges::keys_view views::keys ranges::values_view views::values ranges::zip_view views::zip ranges::zip_transform_view views::zip_transform ranges::adjacent_view views::adjacent ranges::adjacent_transform_view views::adjacent_transform ranges::join_with_view views::join_with ranges::stride_view views::stride ranges::slide_view views::slide ranges::chunk_view views::chunk ranges::chunk_by_view views::chunk_by"
    [execution]="is_execution_policy sequenced_policy parallel_policy parallel_unsequenced_policy unsequenced_policy seq par par_unseq unseq"
    [bit]="endian bit_cast byteswap has_single_bit bit_ceil bit_floor bit_width rotl rotr countl_zero countl_one countr_zero countr_one popcount"
    [cfenv]="fenv_t fexcept_t feclearexcept fetestexcept feraiseexcept fegetexceptflag fesetexceptflag fegetround fesetround fegetenv fesetenv feholdexcept feupdateenv FE_ALL_EXCEPT FE_DIVBYZERO FE_INEXACT FE_INVALID FE_OVERFLOW FE_UNDERFLOW FE_DOWNWARD FE_TONEAREST FE_TOWARDZERO FE_UPWARD FE_DFL_ENV"
    [complex]="complex real imag arg norm conj proj polar if i il" # and lots of math functions taking complex arguments
    [numeric]="iota ranges::iota accumulate reduce transform_reduce inner_product adjacent_difference partial_sum inclusive_scan exclusive_scan transform_inclusive_scan transform_exclusive_scan gcd lcm midpoint"
    [cmath]="float_t double_t HUGE_VALF HUGE_VAL HUGE_VALL INFINITY NAN math_errhandling MATH_ERRNO MATH_ERREXCEPT FP_NORMAL FP_SUBNORMAL FP_ZERO FP_INFINITE FP_NAN fabs fabsf fabsl fmod fmodf fmodl remainder remainderf remainderl remquo remquof remquol fma fmaf fmal fmax fmaxf fmaxl fmin fminf fminl fdim fdimf fdiml nan nanf nanl lerp exp expf expl exp2 exp2f exp2l expm1 expm1f expm1l log logf logl log10 log10f log10l log2 log2f log2l log1p log1pf log1pl pow powf powl sqrt sqrtf sqrtl cbrt cbrtf cbrtl hypot hypotf hypotl sin sinf sinl cos cosf cosl tan tanf tanl asin asinf asinl acos acosf acosli atan atanf atanl atan2 atan2f atan2l sinh sinhf sinhl cosh coshf coshl tanh tanhf tanhl asinh asinhf asinhl acosh acoshf acoshl atanh atanhf atanhl erf erff erfl erfc erfcf erfcl tgamma tgammaf tgammal lgamma lgammaf lgammal ceil ceilf ceill floor floorf floorl trunc truncf truncl round roundf roundl lround lroundf lroundl llround llroundf llroundl nearbyint nearbyintf nearbyintl rint rintf rintl lrint lrintf lrintl llrint llrintf llrintl frexp frexpf frexpl ldexp ldexpf ldexpl modf modff modfl scalbn scalbnf scalbnl scalbln scalblnf scalblnl ilogb ilogbf ilogbl logb logbf logbl nextafter nextafterf nextafterl nexttoward nexttowardf nexttowardl copysign copysignf copysignl fpclassify isfinite isinf isnan isnormal signbit isgreater isgreaterequal isless islessequal islessgreater isunordered assoc_laguerre assoc_laguerref assoc_laguerrel assoc_legendre assoc_legendref assoc_legendrel beta betaf betal comp_ellint_1 comp_ellint_1f comp_ellint_1l comp_ellint_2 comp_ellint_2f comp_ellint_2l comp_ellint_3 comp_ellint_3f comp_ellint_3l cyl_bessel_i cyl_bessel_if cyl_bessel_il cyl_bessel_j cyl_bessel_jf cyl_bessel_jl cyl_bessel_k cyl_bessel_kf cyl_bessel_kl cyl_neumann cyl_neumannf cyl_neumannl ellint_1 ellint_1f ellint_1l ellint_2 ellint_2f ellint_2l ellint_3 ellint_3f ellint_3l expint expintf expintl hermite hermitef hermitel legendre legendref legendrel laguerre laguerref laguerrel riemann_zeta riemann_zetaf riemann_zetal sph_bessel sph_besself sph_bessell sph_legendre sph_legendref sph_legendrel sph_neumann sph_neumannf sph_neumannl"
    [ratio]="ratio ratio_add ratio_subtract ratio_multiply ratio_divide ratio_equal ratio_not_equal ratio_less ratio_less_equal ratio_greater ratio_greater_equal yocto zepto atto femto pico nano micro milli centi deci deca hecto kilo mega giga tera peta exa zetta yotta"
    [clocale]="lconv NULL LC_ALL LC_COLLATE LC_CTYPE LC_MONETARY LC_NUMERIC LC_TIME setlocale localeconv"
    [codecvt]="codecvt_utf8 codecvt_utf16 codecvt_utf8_utf16 codecvt_mode"
    [locale]="locale wstring_convert wbuffer_convert ctype_base codecvt_base messages_base time_base money_base ctype codecvt collate messages time_get time_put num_get num_put numpunct money_get money_put moneypunct ctype_byname codecvt_byname messages_byname collate_byname time_get_byname time_put_byname numpunct_byname moneypunct_byname use_facet has_facet"
    [cstdio]="FILE fpos_t size_t NULL stdin stdout stderr EOF FOPEN_MAX FILENAME_MAX BUFSIZ _IOFBF _IOLBF _IONBF SEEK_SET SEEK_CUR SEEK_END TMP_MAX L_tmpnam fopen freopen fclose fflush setbuf setvbuf fread fwrite fgetc getc fgets fputc putc fputs getchar gets putchar puts ungetc scanf fscanf sscanf vscanf vfscanf vsscanf printf fprintf sprintf snprintf vprintf vfprintf vsprintf vsnprintf ftell fgetpos fseek fsetpos rewind clearerr feof ferror perror remove rename tmpfile tmpnam"
    [fstream]="basic_filebuf basic_ifstream basic_ofstream basic_fstream filebuf wfilebuf ifstream wifstream ofstream wofstream fstream wfstream"
    [iomanip]="resetiosflags setiosflags setbase setfill setprecision setw get_money put_money get_time put_time quoted"
    [spanstream]="basic_spanbuf basic_ispanstream basic_ospanstream basic_spanstream spanbuf wspanbuf ispanstream wispanstream ospanstream wospanstream spanstream wspanstream"
    [strstream]="strstreambuf istrstream ostrstream strstream"
    [mutex]="mutex timed_mutex recursive_mutex recursive_timed_mutex lock_guard unique_lock scoped_lock defer_lock_t try_to_lock_t adopt_lock_t once_flag defer_lock try_to_lock adopt_lock try_lock lock call_once"
    [semaphore]="counting_semaphore binary_semaphore"
    [shared_mutex]="shared_mutex shared_timed_mutex shared_lock"
    [stop_token]="stop_token stop_source stop_callback nostopstate_t nostopstate"
    [latch]="latch"
    [future]="promise packaged_task future shared_future launch future_status future_error future_errc async future_category"
    [condition_variable]="condition_variable condition_variable_any cv_status notify_all_at_thread_exit"
    [barrier]="barrier"
    [atomic]="atomic atomic_ref atomic_flag memory_order atomic_bool atomic_char atomic_schar atomic_uchar atomic_short atomic_ushort atomic_int atomic_uint atomic_long atomic_ulong atomic_llong atomic_ullong atomic_char8_t atomic_char16_t atomic_char32_t atomic_wchar_t atomic_int8_t atomic_uint8_t atomic_int16_t atomic_uint16_t atomic_int32_t atomic_uint32_t atomic_int64_t atomic_uint64_t atomic_int_least8_t atomic_uint_least8_t atomic_int_least16_t atomic_uint_least16_t atomic_int_least32_t atomic_uint_least32_t atomic_int_least64_t atomic_uint_least64_t atomic_int_fast8_t atomic_uint_fast8_t atomic_int_fast16_t atomic_uint_fast16_t atomic_int_fast32_t atomic_uint_fast32_t atomic_int_fast64_t atomic_uint_fast64_t atomic_intptr_t atomic_uintptr_t atomic_size_t atomic_ptrdiff_t atomic_intmax_t atomic_uintmax_t atomic_signed_lock_free atomic_unsigned_lock_free atomic_is_lock_free atomic_store atomic_store_explicit atomic_load atomic_load_explicit atomic_exchange atomic_exchange_explicit atomic_compare_exchange_weak atomic_compare_exchange_weak_explicit atomic_compare_exchange_strong atomic_compare_exchange_strong_explicit atomic_fetch_add atomic_fetch_add_explicit atomic_fetch_sub atomic_fetch_sub_explicit atomic_fetch_and atomic_fetch_and_explicit atomic_fetch_or atomic_fetch_or_explicit atomic_fetch_xor atomic_fetch_xor_explicit atomic_wait atomic_wait_explicit atomic_notify_one atomic_notify_all atomic_flag_test atomic_flag_test_explicit atomic_flag_test_and_set atomic_flag_test_and_set_explicit atomic_flag_clear atomic_flag_clear_explicit atomic_flag_wait atomic_flag_wait_explicit atomic_flag_notify_one atomic_flag_notify_all atomic_init kill_dependency atomic_thread_fence atomic_signal_fence ATOMIC_VAR_INIT ATOMIC_FLAG_INIT"
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
symbolsPerHeader[algorithm]+="${symbolsPerHeader[initializer_list]} ranges::in_fun_result ranges::in_in_result ranges::in_out_result ranges::in_in_out_result ranges::in_out_out_result ranges::min_max_result ranges::in_found_result ranges::in_value_result ranges::out_value_result all_of any_of none_of for_each for_each_n count count_if mismatch find find_if find_if_not find_end find_first_of adjacent_find search search_n copy copy_if copy_n copy_backward move move_backward fill fill_n transform generate generate_n remove remove_if remove_copy remove_copy_if replace replace_if replace_copy replace_copy_if swap swap_ranges iter_swap reverse reverse_copy rotate rotate_copy shift_left shift_right random_shuffle shuffle sample unique unique_copy is_partitioned partition partition_copy stable_partition partition_point is_sorted is_sorted_until sort partial_sort partial_sort_copy stable_sort nth_element lower_bound upper_bound binary_search equal_range merge inplace_merge includes set_difference set_intersection set_symmetric_difference set_union is_heap is_heap_until make_heap push_heap pop_heap sort_heap max max_element min min_element minmax minmax_element clamp equal lexicographical_compare lexicographical_compare_three_way is_permutation next_permutation prev_permutation" # Hopefully ranges::XXXX functions correspond to functions mentioned here.
symbolsPerHeader[random]+="${symbolsPerHeader[initializer_list]} uniform_random_bit_generator linear_congruential_engine mersenne_twister_engine subtract_with_carry_engine discard_block_engine independent_bits_engine shuffle_order_engine minstd_rand0 minstd_rand mt19937 mt19937_64 ranlux24_base ranlux48_base ranlux24 ranlux48 knuth_b default_random_engine random_device uniform_int_distribution uniform_real_distribution bernoulli_distribution binomial_distribution negative_binomial_distribution geometric_distribution poisson_distribution exponential_distribution gamma_distribution weibull_distribution extreme_value_distribution normal_distribution lognormal_distribution chi_squared_distribution cauchy_distribution fisher_f_distribution student_t_distribution discrete_distribution piecewise_constant_distribution piecewise_linear_distribution seed_seq generate_canonical"
symbolsPerHeader[valarray]+="${symbolsPerHeader[initializer_list]} valarray slice slice_array gslice gslice_array mask_array indirect_array"
symbolsPerHeader[syncstream]+="${symbolsPerHeader[ostream]} basic_syncbuf basic_osyncstream syncbuf wsyncbuf osyncstream wosyncstream"
symbolsPerHeader[filesystem]+="${symbolsPerHeader[compare]} path filesystem_error directory_entry directory_iterator recursive_directory_iterator file_status space_info file_type perms perm_options copy_options directory_options file_time_type filesystem"

symbolsPerHeader['iso646.h']="and and_eq bitand bitor compl not not_eq or or_eq xor xor_eq"
symbolsPerHeader['stdbool.h']="bool true false _Bool"

symbolsPerHeader['assert.h']="${symbolsPerHeader[cassert]}"
symbolsPerHeader['ctype.h']="${symbolsPerHeader[cctype]}"
symbolsPerHeader['errno.h']="${symbolsPerHeader[cerrno]}"
symbolsPerHeader['float.h']="${symbolsPerHeader[cfloat]}"
symbolsPerHeader['limits.h']="${symbolsPerHeader[climits]}"
symbolsPerHeader['locale.h']="${symbolsPerHeader[clocale]}"
symbolsPerHeader['math.h']="${symbolsPerHeader[cmath]}"
symbolsPerHeader['setjmp.h']="${symbolsPerHeader[csetjmp]}"
symbolsPerHeader['signal.h']="${symbolsPerHeader[csignal]}"
symbolsPerHeader['stdarg.h']="${symbolsPerHeader[cstdarg]}"
symbolsPerHeader['stddef.h']="${symbolsPerHeader[cstddef]}"
symbolsPerHeader['stdint.h']="${symbolsPerHeader[cstdint]}"
symbolsPerHeader['stdio.h']="${symbolsPerHeader[cstdio]}"
symbolsPerHeader['stdlib.h']="${symbolsPerHeader[cstdlib]}"
symbolsPerHeader['string.h']="${symbolsPerHeader[cstring]}"
symbolsPerHeader['time.h']="${symbolsPerHeader[ctime]}"
symbolsPerHeader['wchar.h']="${symbolsPerHeader[cwchar]}"
symbolsPerHeader['wctype.h']="${symbolsPerHeader[cwctype]}"

function get_cpp_files() {
    for path in "$@"; do
        if [[ -f "$path" ]]; then
            echo "$path"
        elif [[ -d "$path" ]]; then
            find "$path" -name "*.c" -or -name "*.cpp" -or -name "*.tcc" -or -name "*.h" -or -name "*.hpp"
        else
            echo "Neither ordinary file nor directory: $path" 1>&2
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
    grep -q "#include\\s*.$header.\\s*$" "$file" || return

    grep_pattern="\\b${symbolsPerHeader[$header]// /\\b\\|\\b}\\b"

    # Remove all #include lines
    grep -v '#include' "$file" |
        # If no symbol from this header is found, report it
        # grep -q "$grep_pattern" || echo "Useless header: $header in file $file"
	grep -q "$grep_pattern" || echo "sed -i /#include\\\\s.$header.$/d $file"
}

if [[ -z "$@" ]]; then
    paths=.
else
    paths="$@"
fi

get_cpp_files $paths | while read -r file; do
    # check_if_file_has_unused_forward_declarations "$file"

    for header in "${!symbolsPerHeader[@]}"; do
        check_if_header_is_unused_in_file "$header" "$file"
    done
done
