
use fastly::http::{StatusCode};
use fastly::{Error, Request, Response};

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // Pattern match on the path...
    match req.get_path() {
        // If request is to the `/pass` path...
        "/pass" => {
            let bereq = Request::get("https://httpbin.org/headers")
                .with_pass(true);

            let beresp = bereq.send("httpbin")?;
            Ok(beresp)
        }
        "/no-pass" => {

            // Create a new request.
            let bereq = Request::get("https://httpbin.org/headers")
                .with_pass(false);

            // Forward the request to a backend.
            let beresp = bereq.send("httpbin")?;

            // Send a default synthetic response.
            Ok(beresp)
        }

        // Catch all other requests and return a 404.
        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain("The page you requested could not be found\n")),
    }
}
