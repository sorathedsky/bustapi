use bytes::Bytes;
use futures::future::BoxFuture;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use pyo3::prelude::*;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

type BoxBody = http_body_util::Full<Bytes>;

#[pyclass]
#[derive(Clone)]
struct Route {
    path: String,
    handler: PyObject,
    method: String,
}

#[pyclass]
#[derive(Clone)]
struct App {
    routes: Vec<Route>,
}

struct AppService {
    routes: Arc<Mutex<Vec<Route>>>,
}

impl Service<Request<Incoming>> for AppService {
    type Response = Response<BoxBody>;
    type Error = hyper::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let routes = self.routes.clone();
        
        Box::pin(async move {
            let routes = routes.lock().await;
            let path = req.uri().path();
            let method = req.method().as_str();
            
            for route in routes.iter() {
                if route.path == path && route.method == method {
                    let response = Python::with_gil(|py| {
                        match route.handler.call0(py) {
                            Ok(result) => {
                                let json = result.extract::<String>(py).unwrap_or_else(|_| "{}".to_string());
                                Response::builder()
                                    .status(StatusCode::OK)
                                    .header("Content-Type", "application/json")
                                    .body(Full::new(Bytes::from(json)))
                                    .unwrap()
                            }
                            Err(_) => {
                                Response::builder()
                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                    .body(Full::new(Bytes::from("Internal Server Error")))
                                    .unwrap()
                            }
                        }
                    });
                    return Ok(response);
                }
            }
            
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::new(Bytes::from("Not Found")))
                .unwrap())
        })
    }
}

#[pymethods]
impl App {
    #[new]
    fn new() -> Self {
        App {
            routes: Vec::new(),
        }
    }

    fn get(&mut self, path: String, handler: Option<PyObject>) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Some(handler) = handler {
                self.routes.push(Route {
                    path,
                    handler: handler.clone(),
                    method: "GET".to_string(),
                });
                Ok(handler)
            } else {
                // Create a closure that will be called when the decorator is used
                let mut routes = self.routes.clone();
                let path = path.clone();
                let decorator = py.eval(
                    "lambda f: (lambda p=p, h=f: (routes.append(h), h)[1])()",
                    None,
                    Some(&[("p", path), ("routes", &routes)].into_py_dict(py)),
                )?;
                Ok(decorator.into())
            }
        })
    }

    fn post(&mut self, path: String, handler: Option<PyObject>) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Some(handler) = handler {
                self.routes.push(Route {
                    path,
                    handler: handler.clone(),
                    method: "POST".to_string(),
                });
                Ok(handler)
            } else {
                // Create a closure that will be called when the decorator is used
                let mut routes = self.routes.clone();
                let path = path.clone();
                let decorator = py.eval(
                    "lambda f: (lambda p=p, h=f: (routes.append(h), h)[1])()",
                    None,
                    Some(&[("p", path), ("routes", &routes)].into_py_dict(py)),
                )?;
                Ok(decorator.into())
            }
        })
    }

    fn run(&mut self, host: Option<String>, port: Option<u16>) -> PyResult<()> {
        let host = host.unwrap_or_else(|| "127.0.0.1".to_string());
        let port = port.unwrap_or(8000);
        let addr = format!("{}:{}", host, port).parse().unwrap();
        
        println!("BustAPI running on http://{}:{}", host, port);
        
        let routes = Arc::new(Mutex::new(self.routes.clone()));
        
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            serve(addr, routes).await.unwrap();
        });
        
        Ok(())
    }
}

async fn serve(addr: SocketAddr, routes: Arc<Mutex<Vec<Route>>>) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;
    
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        
        let service = AppService {
            routes: routes.clone(),
        };
        
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service)
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

#[pymodule]
fn bustapi(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<App>()?;
    Ok(())
}
