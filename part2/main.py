

class WasRun:
    def __init__(self, name):
        self.wasRun = None
        self.name = name

    def run(self):
        method = getattr(self, self.name)
        method()

    def test_method(self):
        self.wasRun = 1


test = WasRun("test_method")
print(test.wasRun)
test.run()
print(test.wasRun)
