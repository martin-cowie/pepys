#!/usr/bin/perl -w
use strict;
use warnings;
use File::Find;
use FindBin qw($Bin $Script);
use Term::ANSIColor qw(:constants);
use XML::Parser;

use XML::XPath;
use XML::XPath::XMLParser;

print "\$Bin = $Bin\n";
die "Cannot chdir($Bin)" unless chdir($Bin);

=pod1
=head1 Web Service Integration Tests
=cut

use constant VERBOSE => $ENV{'VERBOSE'} || 0;

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
        my ($rc, $output, $command) = postFile($url, $filename);

        if ($rc != 0) {
            print STDERR RED, "FAILED: $url $filename - $output\n", RESET;
        } elsif (!validate_xml($output)) {
            print STDERR RED, "FAILED: $url $filename - Invalid XML response\n", RESET;
        } else {
            print STDERR BOLD, "PASSED:", RESET, " $url $filename\n"
        }
    }
}

# POST all files found under directory `onvif` and check for a 200 response with valid XML
sub testServices {
    my %findOptions = (
        wanted => \&consider,
        no_chdir => 1
    );

    find( \%findOptions, "onvif");
}

# Test that both positive and negative authentication work
sub testAuthentication {
    my $url = "http://localhost:8088/onvif/device_service";
    my $filename = "lacking_authentication.xml";
    my ($rc, $output, $command) = postFile($url, $filename);
    if ($rc == 0) {
        print STDERR RED, "FAILED: $url $filename - $output\n", RESET;
        return;
    }

    # Parse and assert on XML content returned
    my $subCode = getFaultSubCode($output);
    if ($subCode eq "ter:NotAuthorized") {
        print STDERR BOLD, "PASSED:", RESET, " $url $filename\n"
    } else {
        print STDERR RED, "FAILED: $url $filename - Did not expect subCode == $subCode\n", RESET;
    }
}

sub testUnknownService {
    my $url = "http://localhost:8088/onvif/device_service";
    my $filename = "unknown_service.xml";

    my ($rc, $output, $command) = postFile($url, $filename);

    # Expect this call to fail
    if ($rc == 0) {
        print STDERR RED, "FAILED: $url $filename - $output\n", RESET;
        return;
    }

    # Parse and assert on XML content returned
    my $subCode = getFaultSubCode($output);
    if ($subCode eq "ter:ActionNotSupported") {
        print STDERR BOLD, "PASSED:", RESET, " $url $filename\n"
    } else {
        print STDERR RED, "FAILED: $url $filename - Did not expect subCode == $subCode\n", RESET;
    }

}

# Post non XML content to a valid endpoint.
sub testNotXML {
    my $url = "http://localhost:8088/onvif/device_service";
    my $filename = "invalid_request.xml";

    my ($rc, $output, $command) = postFile($url, $filename);

    # Expect this call to fail
    if ($rc == 0) {
        print STDERR RED, "FAILED: $url $filename - $output\n", RESET;
        return;
    }

    # Parse and assert on XML content returned
    my $subCode = getFaultSubCode($output);
    if ($subCode eq "ter:WellFormed") {
        print STDERR BOLD, "PASSED:", RESET, " $url $filename\n"
    } else {
        print STDERR RED, "FAILED: $url $filename - Did not expect subCode == $subCode\n", RESET;

    }
}

sub getFaultSubCode {
    my ($xml) = @_;

    my $xpath = "/s:Envelope/s:Body/s:Fault/s:Code/s:Subcode/s:Value/text()";
    my $xp = XML::XPath->new(xml => $xml);
    $xp->set_namespace("s", "http://www.w3.org/2003/05/soap-envelope");
    my $nodeset = $xp->find($xpath);
    return $nodeset->string_value();
}

sub postFile {
    my ($url, $filename) = @_;
    my $command = "curl --fail-with-body -s -X POST -d @./$filename $url";
    print STDERR FAINT, "$command\n", RESET if(VERBOSE);

    my $output = `$command`;
    my $rc = $?;

    return ($rc, $output, $command)
}


#====== main ======

testServices();
testAuthentication();
testUnknownService();
testNotXML();