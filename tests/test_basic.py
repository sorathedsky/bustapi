import pytest
from bustapi import App

def test_app_creation():
    app = App()
    assert app is not None
    assert hasattr(app, 'routes')
    assert len(app.routes) == 0

def test_route_registration():
    app = App()
    
    @app.get("/test")
    def test_route():
        return {"message": "test"}
    
    assert len(app.routes) == 1
    route = app.routes[0]
    assert route.path == "/test"
    assert route.method == "GET"

def test_multiple_routes():
    app = App()
    
    @app.get("/get")
    def get_route():
        return {"method": "GET"}
    
    @app.post("/post")
    def post_route():
        return {"method": "POST"}
    
    assert len(app.routes) == 2
    methods = {route.method for route in app.routes}
    assert methods == {"GET", "POST"}

def test_path_parameters():
    app = App()
    
    @app.get("/items/{item_id}")
    def get_item(item_id: int):
        return {"item_id": item_id}
    
    route = app.routes[0]
    assert "{item_id}" in route.path
