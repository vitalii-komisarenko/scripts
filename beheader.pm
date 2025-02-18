use strict;
use warnings;
use File::Find;

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
    push @res, m/^\s*#define\s+(\w+)/gm;

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
        s/\bconst\b//g;
        s/\bstatic\b//g;
        s/\binline\b//g;
        s/\bextern\b//g;

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
        if (/typedef struct (\w+) (\w+)$/) {
            push @res, $1, $2;
            next;
        }

        if (/typedef struct (\w+)$/) {
            push @res, $1;
            next;
        }

        # Simple typedef
        #
        # typedef A B
        if (/typedef ([\w:<>]+) (\w+)/) {
            push @res, $2;
            next;
        }

        # Skip function calls
        #
        # Since () have been removed, it is just a single word
        # A(B, C, D);  -->  A
        next if /[\w:<>]+/;

        warn "Can't parse: $_";
    }

    return \@res;
}

sub extract_includes($) {
    local $_ = shift;
    my @paths;
    push @paths, m/^\s*#include\s+"<.+>"/gm;
    push @paths, m/^\s*#include\s+"(.+)"/gm;

    # For the sake of simplicity, leave only file names
    my @res = grep { s!.*/!!; } @paths;
    return \@res;
}

sub extract_usages($) {
    local $_ = shift;

    # Remove preprocessor directives
    s/^\s*#.*$//gm;

    # Return any word as a usage
    my @res = m/(\w+)/g;
    return \@res;
}

unless (caller) {
    my @paths;
    grep { find(sub {push @paths, $File::Find::name}, $_) } @ARGV;

    my %definitions;
    my %includes;
    my %usages;
    for (@paths) {
        next unless (/\.c$/ || /\.cpp$/ || /\.cc$/ || /\.h$/ || /\.hpp$/);

        my $filename = $_;
        $filename =~ s!.*/!!;

        $includes{$filename} = extract_includes(read_file($_));
        $definitions{$filename} = extract_definitions(read_file($_));
        $usages{$filename} = extract_usages(read_file($_));
    }

    while (my ($filename, $headers) = each %includes) {
        for my $header_filename (@$headers) {
            # Skip external headers - we have no info about them
            next unless exists $definitions{$header_filename};

            my $header_is_used = 0;
            for my $word (@{$usages{$filename}}) {
                for my $definition (@{$definitions{$header_filename}}) {
                    if ($word eq $definition) {
                        $header_is_used = 1;
                        last;
                    }
                }
            }
            unless ($header_is_used) {
                print "$filename does not need $header_filename\n";
                print "Header definitions: \n  ".join("\n  ", @{$definitions{$header_filename}}) . "  \n\n";
            }
        }
    }
}
