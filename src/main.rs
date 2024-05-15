use tide::Request;
use tide::prelude::*;
use port_scanner::scan_ports_addrs;

#[derive(Debug, Deserialize)]
struct ArdReq {
    rgb: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let app = setup_routes();
    app.listen("127.0.0.1:8080").await?;

    
    for open_addr in scan_ports_addrs(vec!["192.168.1.1:8000", "192.168.1.2:8000"]) {
        println!("IP {} has port {} open.", open_addr.ip(), open_addr.port());
    }

    Ok(())
}

fn setup_routes() -> tide::Server<()> {
    let mut app = tide::new();
    app.at("/:identifier/rgb").post(set_rgb);
    app.at("/v1").nest({
        let mut api = tide::new();

        api.at("/").post(|_req: Request<()>| async move{
            Ok("Created")
        });

        api.at("/:id/update").post(|_req: Request<()>| async move{
            Ok("BLA")
        });

        api
    });

    let _ = app.at("/www").serve_dir("./www");

    app
}
 
async fn set_rgb(mut _req: Request<()>) -> tide::Result {
    let ArdReq { rgb } = _req.body_json().await?;
    Ok(format!("Hello, {}! ", rgb).into())
}

/*async fn update(mut req: Request<()>) {

    
}*/

