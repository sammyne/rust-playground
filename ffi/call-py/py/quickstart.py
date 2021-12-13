import base64

def base64_encode(b):
  """
  @see https://docs.rs/pyo3/0.15.1/pyo3/prelude/struct.Python.html#examples-2
  """
  #for i, v in enumerate(b):
  #  print(f"{i}: {v}")
  return base64.b64encode(b)

def hello():
  print("hello world")

def print_list_of_dict(args):
  print("\nprint a list of dict")
  for i, v in enumerate(args):
    print(f"{i}: {v['key']}-{base64.b64encode(v['value'])}")
  print()

if __name__ == "__main__":
  b = bytes("Hello Rust!".encode("utf-8"))
  print(f"{base64_encode(b)}")

  v = [
    {"key": "hello", "value": "#1 Hello Rust!".encode('utf-8')},
    {"key": "world", "value": "#2 Hello Rust!".encode('utf-8')},
  ]
  print_list_of_dict(v)
