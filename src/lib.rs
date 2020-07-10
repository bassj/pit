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

    pub fn post(host: &str, path: &str) -> Request {
        return Request::new(host, path, "POST");
    }

    pub fn get(host: &str, path:&str) -> Request {
        return Request::new(host, path, "GET");
    }


    //method functions

    pub fn set_method(&mut self, method: &str) {
        self.method = String::from(method);
    }

    pub fn set_version(&mut self, version: &str) {
        self.version = String::from(version);
    }

    pub fn set_body(&mut self, body: &str) {
        self.body = String::from(body);

        self.set_header_attr("Content-Length", format!("{}", self.body.len()).as_str());
    }

    pub fn set_header_attr(&mut self, attrib: &str, value: &str) {
        self.headers.push((String::from(attrib), String::from(value)));
    }

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

    pub fn new(status: u32) -> Response {
        Response {
            status,
            version: String::from("HTTP/1.1"),
            reason_phrase: String::new(),
            headers: vec![],
            body: String::new()
        }
    }

    // Method functions.

    pub fn set_status(&mut self, status : u32) {
        self.status = status;
    }

    pub fn set_reason_phrase(&mut self, reason_phrase: &str) {
        self.reason_phrase = String::from(reason_phrase);
    }

    pub fn set_header_attr(&mut self, attrib: &str, value: &str) {
        self.headers.push((String::from(attrib), String::from(value)));
    }

    pub fn set_body(&mut self, body: &str) {
        self.body = String::from(body);

        self.set_header_attr("Content-Length", format!("{}", self.body.len()).as_str());
    }

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