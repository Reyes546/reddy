use tide::Request;
use tide::prelude::*;

#[derive(Debug, Deserialize)]
struct ArdReq {
    rgb: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    
    let mut app = tide::new();
    app.at("/:identifier/rgb").post(set_rgb);
    app.at("/v1").nest({
        let mut api = tide::new();

        api.at("/").post(|_req: Request<()>| async move{
            Ok("Created")
        });

        api.at("/:id").post(|_req: Request<()>| async move{
            Ok("Created")
        });

        api
    });

    let _ = app.at("/www").serve_dir("./www");
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
 
async fn set_rgb(mut req: Request<()>) -> tide::Result {
    let ArdReq { rgb } = req.body_json().await?;
    Ok(format!("Hello, {}! ", rgb).into())
}

