from bustapi import App

app = App()

@app.get("/")
def read_root():
    return {"message": "Welcome to BustAPI!"}

@app.get("/hello/{name}")
def greet(name: str):
    return {"message": f"Hello, {name}!"}

@app.post("/items")
def create_item(item: dict):
    return {"item": item, "status": "created"}

if __name__ == "__main__":
    app.run(host="127.0.0.1", port=8000)
