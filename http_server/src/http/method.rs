pub enum Method {
    GET(String),
    POST(u64),
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
    DELETE,
}