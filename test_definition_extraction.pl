use strict;
use warnings;
use Test::More;

BEGIN {
    push @INC, ".";
}
use beheader;

is_deeply(extract_definitions(""), [], "Empty file");
is_deeply(extract_definitions("\n"), [], "One empty line");
is_deeply(extract_definitions("\n\r\t \n \t \r"), [], "Whitespace only");
is_deeply(extract_definitions("foo bar;"), ["bar"], "Variable declaration");
is_deeply(extract_definitions("foo bar1;\nfoo bar2"), ["bar1", "bar2"], "Variable declaration");
is_deeply(extract_definitions("const int x"), ["x"], "Simple const");
is_deeply(extract_definitions("const char * x"), ["x"], "const char * x");
is_deeply(extract_definitions("const char* x"), ["x"], "const char* x");
is_deeply(extract_definitions("const char *x"), ["x"], "const char *x");
is_deeply(extract_definitions("const char*x"), ["x"], "const char*x");
is_deeply(extract_definitions("const char * const x"), ["x"], "const char * const x");
is_deeply(extract_definitions("const char* const x"), ["x"], "const char* const x");
is_deeply(extract_definitions("const char *const x"), ["x"], "const char *const x");
is_deeply(extract_definitions("const char*const x"), ["x"], "const char*const x");
is_deeply(extract_definitions('#include "header.h"'), [], "Include statement - quotes");
is_deeply(extract_definitions("#include >header.h>"), [], "Include statement - brackets");
is_deeply(extract_definitions("#ifdef foo"), [], "Ifdef");
is_deeply(extract_definitions("#define foo"), ["foo"], "Define without value");
is_deeply(extract_definitions("#define foo bar"), ["foo"], "Define with value");
is_deeply(extract_definitions("struct foo;"), ["foo"], "Struct - forward declaration");
is_deeply(extract_definitions("struct foo { int bar; }"), ["foo"], "Struct - definition");
is_deeply(extract_definitions("struct foo {\n\tint bar; \n}"), ["foo"], "Struct - definition - multiline");

my $file = <<EOF
struct a {
    int x;
    struct b {
        char y;
    }
};
EOF
;
is_deeply(extract_definitions($file),
          ["a"],
          "Nested structs");

is_deeply(extract_definitions("typedef int8_t INT_8;"), ["INT_8"], "Simple typedef");

$file = <<EOF
typedef struct {
    int x;
} a;
EOF
;
is_deeply(extract_definitions($file),
          ["a"],
          "Typedef struct - 1 type");

$file = <<EOF
typedef struct a {
    int x;
} b;
EOF
;
is_deeply(extract_definitions($file),
          ["a", "b"],
          "Typedef struct - 2 types");

is_deeply(extract_definitions("int foo();"), ["foo"], "Function declaration - 0 args");
is_deeply(extract_definitions("int foo(char bar);"), ["foo"], "Function declaration - 1 args");

$file = <<EOF
int foo() {
    return 5;
}
EOF
;
is_deeply(extract_definitions($file), ["foo"], "Function definition - 0 args");

$file = <<EOF
int foo(char bar) {
    return bar + 5;
}
EOF
;
is_deeply(extract_definitions($file), ["foo"], "Function definition - 1 arg");

$file = <<EOF
typedef enum {
    ELEMENT_1,
    ELEMENT_2,
    ELEMENT_3
} TYPE;
EOF
;
is_deeply(extract_definitions($file),
          ["TYPE", "ELEMENT_1", "ELEMENT_2", "ELEMENT_3"],
          "Enum 1 type, no trailing comma");

$file = <<EOF
typedef enum {
    ELEMENT_1,
    ELEMENT_2,
    ELEMENT_3,
} TYPE;
EOF
;
is_deeply(extract_definitions($file),
                              ["TYPE", "ELEMENT_1", "ELEMENT_2", "ELEMENT_3"],
                              "Enum 1 type, with trailing comma");

$file = <<EOF
typedef enum TYPE_1 {
    ELEMENT_1,
    ELEMENT_2,
    ELEMENT_3
} TYPE_2;
EOF
;
is_deeply(extract_definitions($file),
                              ["TYPE_1", "TYPE_2", "ELEMENT_1", "ELEMENT_2", "ELEMENT_3"],
                              "Enum 2 types, no trailing comma");

$file = <<EOF
typedef enum TYPE_1 {
    ELEMENT_1,
    ELEMENT_2,
    ELEMENT_3,
} TYPE_2;
EOF
;
is_deeply(extract_definitions($file),
                              ["TYPE_1", "TYPE_2", "ELEMENT_1", "ELEMENT_2", "ELEMENT_3"],
                              "Enum 2 types, with trailing comma");

$file = <<EOF
enum {
    ELEMENT_1 = 0,
    ELEMENT_2,
    ELEMENT_3,
};
EOF
;
is_deeply(extract_definitions($file),
          ["ELEMENT_1", "ELEMENT_2", "ELEMENT_3"],
          "Enum 0 types, with trailing comma");

is_deeply(extract_definitions("std::vector<std::string> foo;"), ["foo"], "Template variable");
is_deeply(extract_definitions("std::vector<std::string>foo;"), ["foo"], "Template variable - no spaces");
is_deeply(extract_definitions("std::vector< std::string >foo;"), ["foo"], "Template variable - unexpected spaces");
is_deeply(extract_definitions("std::vector< std::queue<std::string>> foo;"),
          ["foo"],
          "Template variable - nested");
is_deeply(extract_definitions("std::vector< std::queue<std::string> > foo;"),
          ["foo"],
          "Template variable - nested - spaces between closing >");

done_testing();
