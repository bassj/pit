extern crate difference;

#[cfg(test)]
mod tests {
    use difference::{Changeset};
    
    #[test]
    fn test_http_request() {
        use pit::{Request};

        let mut request = Request::post("www.tutorialspoint.com", "/cgi=bin/process.cgi");

        request.set_header_attr("Content-Type", "text/xml; charset=utf-8");
        request.set_header_attr("Accept-Language", "en-us");
        request.set_header_attr("User-Agent", "Mozilla/4.0 (compatible; MSIE5.01; Windows NT)");
        request.set_header_attr("Connection", "Keep-Alive");

        request.set_body("<?xml version=\"1.0\" encoding=\"utf-8\"?><string xmlns=\"http://clearforest.com/\">string</string>");

        let request_str = request.build();


        let t_request_str = String::from(
        "POST /cgi=bin/process.cgi HTTP/1.1\r
Host: www.tutorialspoint.com\r
Content-Type: text/xml; charset=utf-8\r
Accept-Language: en-us\r
User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)\r
Connection: Keep-Alive\r
Content-Length: 93\r
\r
<?xml version=\"1.0\" encoding=\"utf-8\"?><string xmlns=\"http://clearforest.com/\">string</string>");

        
        let changeset = Changeset::new(t_request_str.as_str(), request_str.as_str(), "\n");

        //println!("{}", changeset);

        assert_eq!(request_str, t_request_str);
    }

    #[test]
    fn test_http_response() {

    }
}