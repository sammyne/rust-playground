import base64


class App:
    def __init__(self, name):
        self.name = name

    def hello_world(self, args):
        data = []
        for i, v in enumerate(args):
            vv = base64.b64encode(v["value"])
            print(f"{i}: {v['key']}-{vv}")
            data.append(vv)

        out = {
            "err": 0,
            "data": data,
        }

        return out


if __name__ == "__main__":
    app = App("hello world")
    print(f"{app.name}")

    args = [
        {"key": b"hello", "value": b"#1"},
        {"key": b"world", "value": b"#2"},
    ]
    (err, b64_values) = app.hello_world(args)
    print(f"err = {err}")
    for i, v in enumerate(b64_values):
        print(f"{i}: {v}")
