#!/usr/bin/perl -w
use strict;
use warnings;
use File::Find;
use Term::ANSIColor qw(:constants);

=pod1
=head1 Web Service Integration Tests
=cut

sub consider {
    my ($filename, $dir) = ($File::Find::name, $File::Find::dir);

    if ($filename =~ /(.*)\.xml$/) {
        my $url = "http://localhost:8088/$dir";
        my $command = "curl --fail-with-body -s -X POST -d @./$filename $url";

        my $output = `$command`;
        my $rc = $?;
        if ($rc != 0) {

            print STDERR RED, "FAILED: $url $filename - $output\n", RESET;
        } else {
            print STDERR "PASSED: $url $filename\n"
        }
    }
}

my %findOptions = (
    wanted => \&consider,
    no_chdir => 1
);

find( \%findOptions, "pepys");
