use strict;
use warnings;

sub read_file($) {
    my $file = shift;
    local $/ = undef;
    open my $fh, "<", $file or die "could not open $file: $!";
    <$fh>;
}

sub normalize_whitespaces($) {
    local $_ = shift;

    # Remove leading and trailing whitespace
    s/^\s*//;
    s/\s*$//;

    # Replace multiple whitespaces with one character
    s/\s+/ /g;

    return $_;
}

sub remove_brackets($$$) {
    my ($text, $opening, $closing) = @_;
    while ($text =~ /\Q$opening/) {
        unless (/\Q$closing/) {
            warn "Can't find closing bracket: $closing";
            last;
        }
        my $re = "\Q$opening" . "[^\Q$closing\E]*?\Q$closing";
        $text =~ s/$re//g;
    }
    return $text
}

sub parse_comma_separated_list($) {
    my @res;
    for (split /,/, shift) {
        # Remove whitespace
                s/\s//g;

        # Skip if empty (the last enum element can have a trailing comma)
        next if /^$/;

        # Remove the enum numeric value
        #
        # typedef enum type1 {
        #     ELEMENT_0 = 0,   ----> ELEMENT_0
        #     ELEMENT_1 = 1,   ----> ELEMENT_1
        # } type2;
        s/=.*//;

        # Add the enum element
        push @res, $_;
    }
    @res;
}

sub extract_enum_definitions($) {
    my @res;

    for (split /;/, shift) {
        # Not an enum
        next unless /\benum\b/;

        $_ = normalize_whitespaces($_);

        # typedef enum TYPE_1 { ... } TYPE_2;
        # typedef enum { ... } TYPE
        if (/^(typedef )?enum (\w*)\s*\{(.*)\}(.*)$/) {
            # Add the type names
            push @res, $2 if $2;
            push @res, parse_comma_separated_list($4);

            # Enum elements
            push @res, parse_comma_separated_list($3);
            next;
        }

        warn "Can't parse: $_";
    }

    return @res;
}

sub extract_definitions($) {
    local $_ = shift;
    my @res;

    # #define A B
    # #define A 
    push @res, m/#define (\w+)/g;

    # Remove preprocessor directives
    s/^\s*#.*$//gm;

    # Remove extern "C" {
    #
    # One of the next steps it to remove all {} brackets pairs,
    # so it is needed to preserve code in extern "C" blocks
    s/\s*extern\s*"C"\s*{//gs;

    # Remove chars, so that we don't have to care about " being inside '
    s/'.'//g;
    s/'\\.'//g;

    # Remove strings
    s/".*?"//g;

    # Remove comments
    s!//.*$!!gm;
    s!/\*.*?\*/!!gs;

    # Remove everything inside () and []
    $_ = remove_brackets($_, '(', ')');
    $_ = remove_brackets($_, '[', ']');

    push @res, extract_enum_definitions($_);

    # After parsing enums, we don't need curly brackets
    # Remove everything inside {} to remove function bodies and struct memebers
    $_ = remove_brackets($_, '{', '}');
    # There is a bug in removing {} pairs, so remove trainling }
    s/}.*//g;

    # Replace * with spaces - to handle pointer types
    s/\*/ /g;

    # Parse individual statements
    for (split /;/) {
        # Skip empty statements
        next if /^\s*$/;

        # Enums have already been processed
        next if /\benum\b/;

        # Remove variable values
        s/=.*//gs;

        # Remove type modifiers
        s/const//g;
        s/static//g;
        s/inline//g;

        # Template variables
        #
        # Remove spaces in the type, but leave space between the type and the variable
        # Input examples:
        # std::vector< std::queue<std::string>>foo
        # std::vector<std::queue<std::string>  > foo
        # Output:
        # std::vector<std::queue<std::string>> foo
        if (/>/) {
            my ($left, $right) = $_ =~ /(.*>)(.*)/;
            $left =~ s/\s*//g;
            $_ = "$left $right";
        }

        $_ = normalize_whitespaces($_);

        # Variable declarations / function declarations
        #
        # Semicolons have been removed
        # int a; ---> int a
        #
        # Round brackets and evenrything inside them have been removed
        # int a(char b, long c); --> int a
        if (/^[\w:<>]+ ([\w<>]+)$/) {
            push @res, $1;
            next;
        }

        # Structs in typedef
        #
        # Due to removing unnecessary stuff, the structs in typedef now look like this:
        # typedef struct A { ... } B;   ---> typedef struct A B
        # typedef struct { ... } B;     ---> typedef struct B
        if (/typedef struct (\w*)\s*(\w+)$/) {
            push @res, $1 if $1;
            push @res, $2;
            next;
        }

        # Simple typedef
        #
        # typedef A B
        if (/typedef ([\w:<>]+) (\w+)/) {
            push @res, $2;
            next;
        }

        warn "Can't parse: $_";
    }

    return \@res;
}

unless (caller) {
    print(join "\n", @{extract_definitions(read_file($ARGV[0]))}   );
}
