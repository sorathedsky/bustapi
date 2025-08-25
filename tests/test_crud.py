import pytest
from bustapi import App

@pytest.fixture
def app():
    return App()

@pytest.fixture
def crud_app(app):
    items = {}
    
    @app.get("/items")
    def list_items():
        return {"items": list(items.values())}
    
    @app.post("/items")
    def create_item(item: dict):
        item_id = len(items) + 1
        items[item_id] = item
        return {"id": item_id, "item": item}
    
    @app.get("/items/{item_id}")
    def get_item(item_id: int):
        if item_id not in items:
            return {"error": "Item not found"}, 404
        return {"item": items[item_id]}
    
    return app

def test_create_item(crud_app):
    route = next(r for r in crud_app.routes if r.method == "POST" and r.path == "/items")
    assert route is not None

def test_get_items(crud_app):
    route = next(r for r in crud_app.routes if r.method == "GET" and r.path == "/items")
    assert route is not None

def test_get_item_by_id(crud_app):
    route = next(r for r in crud_app.routes if r.method == "GET" and r.path == "/items/{item_id}")
    assert route is not None
