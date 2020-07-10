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

        println!("{}", changeset);

        assert_eq!(request_str, t_request_str);

        let reconst_req = Request::parse(t_request_str);

        assert_eq!(reconst_req, request);
    }

    #[test]
    fn test_http_response() {
        use pit::{Response};

        let mut response = Response::new(200);

        response.set_reason_phrase("OK");

        response.set_header_attr("Date", "Mon, 27 Jul 2009 12:28:53 GMT");
        response.set_header_attr("Server", "Apache/2.2.14 (Win32)");
        response.set_header_attr("Last-Modified", "Wed, 22 Jul 2009 19:15:56 GMT");
        response.set_header_attr("Connection", "Closed");
        response.set_header_attr("Content-Type", "text/html");

        response.set_body("<html><body><h1>Hello World!</h1></body></html>");

        let response_str = response.build();

        let t_response_str = String::from(
"HTTP/1.1 200 OK\r
Date: Mon, 27 Jul 2009 12:28:53 GMT\r
Server: Apache/2.2.14 (Win32)\r
Last-Modified: Wed, 22 Jul 2009 19:15:56 GMT\r
Connection: Closed\r
Content-Type: text/html\r
Content-Length: 47\r
\r
<html><body><h1>Hello World!</h1></body></html>"
    );

        let changeset = Changeset::new(t_response_str.as_str(), response_str.as_str(), "\n");

        println!("{}", changeset);

        assert_eq!(response_str, t_response_str);

        let reconst_resp = Response::parse(t_response_str);

        //assert_eq!(reconst_resp, response);
    }
}