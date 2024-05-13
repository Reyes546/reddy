use tide::Request;
use tide::prelude::*;
use std::env;


#[derive(Debug, Deserialize)]
struct ArdReq {
    rgb: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {

    let path: std::path::PathBuf = env::current_dir()?;
    println!("The current directory is {}", path.display());
    let mut app = tide::new();
    app.at("/:identifier/rgb").post(set_rgb);
    let _ = app.at("/www").serve_dir("./www");
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
 
async fn set_rgb(mut req: Request<()>) -> tide::Result {
    let ArdReq { rgb } = req.body_json().await?;
    Ok(format!("Hello, {}! ", rgb).into())
}

