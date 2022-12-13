fn main() {
    /* do nothing */
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::process::{Command, Stdio};
    use glob::glob;
    use path_macro::path;
    use reqwest::blocking::Client;
    use std::fs;
    use std::path::{PathBuf, Path};
    use sxd_document::parser;
    use sxd_xpath::{Factory, Context, Value, XPath};

    const DEVICE_SERVICE: &str = "http://localhost:8088/onvif/device_service"; //TODO: make the URL root configurable
    const MEDIA_SERVICE: &str = "http://localhost:8088/onvif/media_service";

    fn resource_root() -> PathBuf {
        path!(PathBuf::from(env!("CARGO_MANIFEST_DIR")) / "resources" / "test")
    }

    // Build XPath and suitable Context
    fn build_soap_xpath(xpath_expression: &str) -> (XPath, Context) {
        let factory = Factory::new();
        let result = factory.build(xpath_expression)
            .expect("Build XPath")
            .ok_or(sxd_xpath::Error::NoXPath)
            .unwrap();

        let mut context = Context::new();
        context.set_namespace("s", "http://www.w3.org/2003/05/soap-envelope");
        context.set_namespace("trt", "http://www.onvif.org/ver10/media/wsdl");
        context.set_namespace("tt", "http://www.onvif.org/ver10/schema");

        (result, context)
    }

    // Assert that `xml_source` is valid XML, and that the Body element has the expected namespace
    fn validate_xml_namespace(xml_source: &str, expected_namespace: &str) {
        let xpath = "/s:Envelope/s:Body/*[1]";
        let package = parser::parse(xml_source).expect("Cannot parse XML");
        let document = package.as_document();

        // Perform the XPath query
        let value = {
            let (xpath, context) = build_soap_xpath(xpath);
            xpath.evaluate(&context, document.root()).unwrap()
        };

        // Unpack and check the result
        let nodeset = match value {
            Value::Nodeset(nodes) => nodes,
            _ => panic!("Expected only a Nodeset")
        };
        assert_eq!(nodeset.size(), 1);

        let elem = match nodeset.iter().next().expect("Cannot get first node") {
            sxd_xpath::nodeset::Node::Element(elem) => elem,
            _ => panic!("Expected only an Element"),
        };
        assert_eq!(elem.name().namespace_uri().expect("Cannot get namespace from XML"), expected_namespace);
    }

    fn post_file<P: AsRef<Path>>(file_name: P, url: &str) -> reqwest::blocking::Response {
        // Load the file
        let content = fs::read_to_string(&file_name).unwrap_or_else(|_| panic!("Cannot load file {:?}", file_name.as_ref()));

        // POST the content and return the result
        Client::builder()
            .build().unwrap()
            .post(url)
            .body(content)
            .send().unwrap_or_else(|_| panic!("Cannot POST to {url}"))
    }

    fn get_fault_subcode(xml_text: &str) -> String {
        let package = parser::parse(xml_text).expect("Cannot parse XML");
        let document = package.as_document();


        let (xpath, context) = build_soap_xpath("/s:Envelope/s:Body/s:Fault/s:Code/s:Subcode/s:Value/text()");
        let value = xpath.evaluate(&context, document.root()).unwrap();

        value.into_string()
    }

    #[test]
    fn test_endpoints_ok() {
        let file_pattern = path!(resource_root() / "onvif" / "**" / "*.xml");

        let entries: Vec<PathBuf> = glob(file_pattern.to_str().unwrap())
            .expect("Failed to resolve glob pattern")
            .map(|e| e.expect("Cannot read directory"))
            .collect();
        assert!(!entries.is_empty());

        // POST each entry to the correct endpoint and check the results
        let root = resource_root();
        for entry in entries.into_iter() {
            let parent = entry.parent().unwrap();
            let endpoint = parent.strip_prefix(&root).unwrap().to_str().unwrap();

            let url = format!("http://localhost:8088/{}", endpoint); //TODO: make URL root configurable
            let response = post_file(&entry, &url);
            assert!(response.status().is_success());

            let namespace = match endpoint {
                "onvif/device_service" => "http://www.onvif.org/ver10/device/wsdl",
                "onvif/imaging_service" => "http://www.onvif.org/ver20/imaging/wsdl",
                "onvif/media_service" => "http://www.onvif.org/ver10/media/wsdl",
                "onvif/event_service" => "http://www.onvif.org/ver10/events/wsdl",
                _ => panic!("Unknown endpoint {}", endpoint)
            };
            validate_xml_namespace(&response.text().expect("Response lacks body"), namespace);
        }
    }

    #[test]
    fn test_authentication() {
        let file_path = path!(resource_root() / "lacking_authentication.xml");

        let response = post_file(file_path, DEVICE_SERVICE);
        assert!(response.status().is_client_error());

        let body = response.text().expect("Cannot get content");
        let subcode = get_fault_subcode(&body);
        assert_eq!(subcode, "ter:NotAuthorized");
    }


    #[test]
    fn test_unknown_service() {
        let file_path = path!(resource_root() / "unknown_service.xml");
        let response = post_file(file_path, DEVICE_SERVICE);
        assert!(response.status().is_client_error());

        let body = response.text().expect("Cannot get content");
        let subcode = get_fault_subcode(&body);
        assert_eq!(subcode, "ter:ActionNotSupported");
    }

    #[test]
    fn test_not_xml() {
        let file_path = path!(resource_root() / "invalid_request.xml");
        let response = post_file(file_path, DEVICE_SERVICE);
        assert!(response.status().is_client_error());

        let body = response.text().expect("Cannot get content");
        let subcode = get_fault_subcode(&body);
        assert_eq!(subcode, "ter:WellFormed");
    }


    #[test]
    fn test_valid_stream_uri() {
        let file_path = path!(resource_root() / "onvif" / "media_service" / "GetStreamUri.xml");
        let response = post_file(file_path, MEDIA_SERVICE);
        assert!(response.status().is_success());

        let xml_text = response.text().expect("Cannot get content");

        let uri = {
            let package = parser::parse(&xml_text).expect("Cannot parse XML");
            let document = package.as_document();

            let (xpath, context) = build_soap_xpath("//trt:MediaUri/tt:Uri/text()");
            let value = xpath.evaluate(&context, document.root()).unwrap();

            value.into_string()
        };

        dbg!(&uri);

        let mut child = Command::new("ffmpeg").args([
            "-i", &uri,
            "-t", "00:00:01",
            "-f", "null", "/dev/null"
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Cannot start ffmpeg");

        let exit_code = child.wait()
            .expect("Cannot reap child process")
            .code()
            .expect("Cannot get exit status");

        assert_eq!(0, exit_code);

    }

}