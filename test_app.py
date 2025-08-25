from bustapi import App
import json

app = App()

@app.get("/")
def root():
    return json.dumps({"message": "Welcome to BustAPI!"})

@app.get("/test")
def test():
    return json.dumps({"message": "test"})

if __name__ == "__main__":
    app.run(port=8001)
