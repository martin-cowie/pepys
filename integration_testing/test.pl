#!/usr/bin/perl -w
use strict;
use warnings;
use File::Find;
use Term::ANSIColor qw(:constants);
use XML::Parser;

=pod1
=head1 Web Service Integration Tests
=cut

use constant VERBOSE => 0;

### Check the validity of an XML document and its namespaces - xmllint does not.
sub validate_xml {
    my ($xml_source) = @_;

    eval {
        my $parser = new XML::Parser(Namespaces => 1);
        $parser->parse($xml_source);
        return 1;
    };

    return 0 if ($@);
    return 1;
}

sub consider {
    my ($filename, $dir) = ($File::Find::name, $File::Find::dir);

    if ($filename =~ /(.*)\.xml$/) {
        my $url = "http://localhost:8088/$dir";
        my $command = "curl --fail-with-body -s -X POST -d @./$filename $url";

        print STDERR FAINT, "$command\n", RESET if(VERBOSE);

        my $output = `$command`;
        my $rc = $?;
        if ($rc != 0) {
            print STDERR RED, "FAILED: $url $filename - $output\n", RESET;
        } elsif (!validate_xml($output)) {
            print STDERR RED, "FAILED: $url $filename - Invalid XML response\n", RESET;
        } else {
            print STDERR BOLD, "PASSED:", RESET, " $url $filename\n"
        }
    }
}

my %findOptions = (
    wanted => \&consider,
    no_chdir => 1
);

find( \%findOptions, "pepys");
