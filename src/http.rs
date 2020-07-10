
///Data representation of a HTTP Request.
#[allow(dead_code)]
pub struct Request {
    headers: Vec<(String, String)>,
    body: String,
    method: String,
    version: String,
    path: String,
    host: String
}

impl Request {
    
    //Static functions

    ///Returns a request with the provided host, path and method.
    /// 
    /// # Arguments
    /// * `host` - The host to send to (ex: "www.google.com")
    /// * `path` - The path of the request (ex: "/")
    /// * `method` - The method of the request usually "GET"
    /// 
    /// Automatically adds a "Host" entry to the headers with the specified host.
    pub fn new(host: &str, path: &str, method: &str) -> Request {
        
        let headers : Vec<(String, String)> = vec![(String::from("Host"), String::from(host))];

        Request {
            headers,
            body: String::new(),
            method: String::from(method),
            version: String::from("HTTP/1.1"),
            path: String::from(path),
            host: String::from(host),
        }
    }

    ///Returns a POST request with the specified host and path.
    /// 
    /// # Arguments
    /// * `host` - The host to send to (ex: "www.google.com")
    /// * `path` - The path of the request (ex: "/")
    /// 
    /// Automatically adds a "Host" entry to the headers with the specified host.
    pub fn post(host: &str, path: &str) -> Request {
        return Request::new(host, path, "POST");
    }

    ///Returns a GET request with the specified host and path.
    /// 
    /// # Arguments
    /// * `host` - The host to send to (ex: "www.google.com")
    /// * `path` - The path of the request (ex: "/")
    /// 
    /// Automatically adds a "Host" entry to the headers with the specified host.
    pub fn get(host: &str, path:&str) -> Request {
        return Request::new(host, path, "GET");
    }

    ///Returns a request parsed from the provided String.
    /// 
    /// #Arguments: 
    /// *`raw_request` - a string containing the raw http response.
    pub fn parse(raw_request: String) -> Request {
        
        let mut split_request = raw_request.split("\r\n");

        //Parse the status line.
        
        let status_str = split_request.next().unwrap();
        let mut split_status = status_str.split(" ");

        let method = split_status.next().unwrap();
        let path = split_status.next().unwrap();
        let version = split_status.next().unwrap();

        //Parse the headers.

        let mut headers : Vec<(String, String)> = vec![];

        let mut header_attr = split_request.next().unwrap();

        while header_attr != "" {
            let mut split_header_attr = header_attr.split(":");

            let attr = split_header_attr.next().unwrap().trim();
            let value = split_header_attr.next().unwrap().trim();

            headers.push((String::from(attr), String::from(value)));

            header_attr = split_request.next().unwrap();
        }

        
        //Parse the body.
        
        let body : String = split_request.collect();

        Request {
            headers,
            body,
            method: String::from(method),
            version: String::from(version),
            path: String::from(path),
            host: String::new()
        }
    }

    //method functions

    ///Sets the method that this request will use with the server.
    /// 
    /// # Arguments: 
    /// * `method` - A string slice containing the method "GET", "POST", "UPDATE", ... etc
    pub fn set_method(&mut self, method: &str) {
        self.method = String::from(method);
    }


    /// Sets the version string to use with the request
    /// 
    /// # Arguments: 
    /// * `version` - A string slice representing the version "HTTP/1.1" by default.
    pub fn set_version(&mut self, version: &str) {
        self.version = String::from(version);
    }

    /// Sets the body of the request.
    /// 
    /// # Arguments:
    /// * `body` - A string slice representing the body of the request.
    /// 
    /// Also adds a "Content-Length" entry to the headers of the request.
    pub fn set_body(&mut self, body: &str) {
        self.body = String::from(body);

        self.set_header_attr("Content-Length", format!("{}", self.body.len()).as_str());
    }


    /// Adds a header attribute to the request.
    /// 
    /// # Arguments:
    /// * `attrib` - A string slice representing the attribute.
    /// * `value` - A string slice representing the value.
    pub fn set_header_attr(&mut self, attrib: &str, value: &str) {
        self.headers.push((String::from(attrib), String::from(value)));
    }

    ///Compiles the request data structure into a HTTP request.
    /// 
    /// # Returns: 
    /// the HTTP request as a String.
    pub fn build(&self) -> String {

        let mut headers = String::new();

        for (attr, value) in self.headers.iter() {
            headers.push_str(format!("{}: {}\r\n", attr, value).as_str());
        }

        let request_meta = format!("{} {} {}", self.method, self.path, self.version);

        let built_request = format!("{}\r\n{}\r\n{}", request_meta, headers, self.body);

        return built_request;
    }

}

///Data representation of a HTTP Response.
#[allow(dead_code)]
pub struct Response {
    status: u32,
    reason_phrase: String,
    version: String,
    headers: Vec<(String, String)>,
    body: String 
}

impl Response {
    //Static methods.

    ///Returns a response with the specified status code.
    /// 
    /// # Arguments: 
    /// * `status` - a u32 representing the status of the response (ex: 200).
    /// 
    /// # Returns:
    /// A response with the specified status code 
    pub fn new(status: u32) -> Response {
        Response {
            status,
            version: String::from("HTTP/1.1"),
            reason_phrase: String::new(),
            headers: vec![],
            body: String::new()
        }
    }

    ///Returns a response parsed from the provided String.
    /// 
    /// #Arguments: 
    /// *`raw_response` - a string containing the raw http response.
    pub fn parse(raw_response: String) -> Response {
        return Response::new(200);
    }

    // Method functions.

    ///Sets the status of the response.
    /// 
    /// # Arguments: 
    /// * `status` - u32 representing the status of the response.
    pub fn set_status(&mut self, status : u32) {
        self.status = status;
    }

    ///Sets the reason phrase of the response
    /// 
    /// # Arguments: 
    /// * `reason_phrase` - string slice containing the reason phrase of the response.
    pub fn set_reason_phrase(&mut self, reason_phrase: &str) {
        self.reason_phrase = String::from(reason_phrase);
    }

    /// Adds a header attribute to the response.
    /// 
    /// # Arguments:
    /// * `attrib` - A string slice representing the attribute.
    /// * `value` - A string slice representing the value.
    pub fn set_header_attr(&mut self, attrib: &str, value: &str) {
        self.headers.push((String::from(attrib), String::from(value)));
    }

    /// Sets the body of the response.
    /// 
    /// # Arguments:
    /// * `body` - A string slice representing the body of the response.
    /// 
    /// Also adds a "Content-Length" entry to the headers of the response.
    pub fn set_body(&mut self, body: &str) {
        self.body = String::from(body);

        self.set_header_attr("Content-Length", format!("{}", self.body.len()).as_str());
    }

    ///Compiles the response data structure into a HTTP request.
    /// 
    /// # Returns: 
    /// the HTTP response as a String.
    pub fn build(&self) -> String {

        let mut headers = String::new();

        for (attr, value) in self.headers.iter() {
            headers.push_str(format!("{}: {}\r\n", attr, value).as_str());
        }

        let request_meta = format!("{} {} {}", self.version, self.status, self.reason_phrase);

        let built_request = format!("{}\r\n{}\r\n{}", request_meta, headers, self.body);

        return built_request;
    }
}