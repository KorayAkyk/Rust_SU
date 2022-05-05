/*
use tide::prelude::*;
use tide::Request;

#[derive(Debug, Deserialize)]
struct Number {
    value: i32,
}

fn factorielle(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        n * factorielle(n - 1)
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/fact/compute").post(return_facto);
    app.at("/fact/compute/:n").get(return_facto_get);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn return_facto(mut req: Request<()>) -> tide::Result {
    let Number { value } = req.body_json().await?;
    let result = factorielle(value);
    Ok(format!("Factorielle de {} = {}\n", value, result).into())
}

async fn return_facto_get(req: Request<()>) -> tide::Result {
    let n: i32 = req.param("n")?.parse().unwrap_or(0);
    let result = factorielle(n);
    Ok(format!("Factorielle de {} = {}\n", n, result).into())
}
*/
use serde::{Deserialize, Serialize};
use tide::prelude::*; // Pulls in the json! macro.
use tide::{Body, Request};

#[derive(Deserialize, Serialize)]
struct Cat {
    name: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());

    app.at("/submit").post(|mut req: Request<()>| async move {
        let cat: Cat = req.body_json().await?;
        println!("cat name: {}", cat.name);

        let cat = Cat {
            name: "chashu".into(),
        };

        Body::from_json(&cat)
    });

    app.at("/animals").get(|_| async {
        Ok(json!({
            "meta": { "count": 2 },
            "animals": [
                { "type": "cat", "name": "chashu" },
                { "type": "dog", "name": "waouf" },
                { "type": "cat", "name": "chashu" },
                { "type": "dog", "name": "waouf" },
                { "type": "cat", "name": "chashu" },
                { "type": "dog", "name": "waouf" },
                { "type": "cat", "name": "chashu" },
                { "type": "dog", "name": "waouf" },
            ]
        }))
    });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}