from bustapi import App
from typing import Optional

app = App()

# In-memory store for demo purposes
items = {}

@app.get("/items")
def list_items(skip: int = 0, limit: Optional[int] = 10):
    items_list = list(items.values())[skip:skip + limit]
    return {"items": items_list, "total": len(items)}

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

@app.post("/upload")
def upload_file(file: bytes):
    file_size = len(file)
    return {"filename": "uploaded_file", "size": file_size}

if __name__ == "__main__":
    app.run(host="127.0.0.1", port=8000)
