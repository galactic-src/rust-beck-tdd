

class WasRun:
    def __init__(self, name):
        self.wasRun = None

    def test_method(self):
        self.wasRun = 1


test = WasRun("testMethod")
print(test.wasRun)
test.test_method()
print(test.wasRun)
