use rouille::Response;

pub fn serve() -> Response {
    rouille::Response::text("OK")
}
